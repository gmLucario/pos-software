# Migration Action Plan: Iced → Dioxus + PostgreSQL → SQLite

## Executive Summary

This document outlines the complete migration strategy for transitioning the POS (Point of Sale) software from:
- **UI Framework**: Iced → Dioxus (for cross-platform desktop apps)
- **Database**: PostgreSQL → SQLite
- **Architecture**: Restructured with clear separation of concerns

## Current State Analysis

### Current Technology Stack
- **UI**: Iced v0.9.0 (unstable, causing dependency issues)
- **Database**: PostgreSQL with UUID extensions
- **Runtime**: async-std
- **Data Types**: MONEY type, NUMERIC for decimals

### Current Project Structure
```
src/
├── config.rs
├── constants.rs
├── controllers/      # Business logic layer
├── db/              # Database connection
├── domain.rs
├── errors.rs
├── events.rs
├── helpers.rs
├── kinds.rs
├── models/          # Database models
├── repo/            # Data access layer
├── schemas/         # DTOs/View models
├── setup.rs
└── views/           # UI layer (Iced)
```

### Database Schema (Current)
- **Catalog Tables**: `item_condition`, `status_loan`, `unit_measurement`
- **Core Tables**: `product`, `catalog`, `operation`, `sale`, `sale_operation`, `loan`, `loan_payment`
- **Features**: UUID primary keys, MONEY type, timestamps with timezone

---

## Target State

### New Technology Stack
- **UI**: Dioxus (stable, cross-platform desktop support)
- **Database**: SQLite (embedded, no external dependencies)
- **Runtime**: Tokio (better Dioxus compatibility)
- **Data Types**: TEXT for IDs (UUID as string), REAL for money, NUMERIC for decimals

### New Project Structure
```
src/
├── main.rs
├── lib.rs
│
├── models/          # Database entities (SQLite schema representation)
│   ├── mod.rs
│   ├── product.rs
│   ├── catalog.rs
│   ├── sale.rs
│   ├── operation.rs
│   ├── loan.rs
│   └── loan_payment.rs
│
├── repo/            # Repository pattern (data access contracts/interfaces)
│   ├── mod.rs
│   ├── traits.rs         # Repository trait definitions
│   ├── product_repo.rs
│   ├── catalog_repo.rs
│   ├── sale_repo.rs
│   ├── loan_repo.rs
│   └── sqlite_impl.rs    # SQLite implementations
│
├── api/             # Core business logic (use cases)
│   ├── mod.rs
│   ├── inventory.rs      # Inventory management operations
│   ├── sales.rs          # Sales operations
│   └── loans.rs          # Loan tracking operations
│
├── handlers/        # Bridge between views and API
│   ├── mod.rs
│   ├── inventory_handler.rs
│   ├── sales_handler.rs
│   └── loans_handler.rs
│
├── views/           # Dioxus components
│   ├── mod.rs
│   ├── app.rs            # Root component
│   ├── inventory/
│   │   ├── mod.rs
│   │   ├── product_list.rs
│   │   └── product_form.rs
│   ├── sales/
│   │   ├── mod.rs
│   │   ├── sale_view.rs
│   │   └── payment_form.rs
│   └── loans/
│       ├── mod.rs
│       ├── loan_list.rs
│       └── payment_tracker.rs
│
├── utils/           # Shared utilities
│   ├── mod.rs
│   ├── validation.rs
│   ├── formatting.rs
│   ├── pdf_generator.rs  # For sale receipts
│   └── barcode.rs
│
├── config.rs        # App configuration
└── error.rs         # Error types
```

---

## Phase 1: Project Setup & Dependencies

### 1.1 Update Cargo.toml

**Remove:**
```toml
iced = { version = "0.9.0", features = [ "tokio" ] }
iced_aw = { version = "0.5.2", ... }
async-std = { version = "1.12.0", features = ["attributes"] }
sqlx = { version = "0.6.3", features = [ "runtime-async-std-native-tls", "postgres", ... ] }
```

**Add:**
```toml
[dependencies]
# UI Framework
dioxus = "0.5"
dioxus-desktop = { version = "0.5", features = ["devtools"] }

# Database
sqlx = { version = "0.8", features = ["runtime-tokio-native-tls", "sqlite", "uuid", "chrono"] }
tokio = { version = "1", features = ["full"] }

# Utilities
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"

# PDF Generation
printpdf = "0.7"

# Barcode handling (optional)
barcoders = "2.0"

# Validation
validator = { version = "0.18", features = ["derive"] }

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"
```

### 1.2 Create SQLite Migration

Create: `migrations/sqlite_schema.sql`

```sql
-- Catalog/Reference Tables

CREATE TABLE IF NOT EXISTS item_condition (
    id INTEGER PRIMARY KEY,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS status_loan (
    id INTEGER PRIMARY KEY,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS unit_measurement (
    id INTEGER PRIMARY KEY,
    description TEXT NOT NULL,
    abbreviation TEXT -- e.g., 'kg', 'lt', 'unit'
);

-- Core Tables

CREATE TABLE IF NOT EXISTS product (
    id TEXT PRIMARY KEY,  -- UUID as TEXT
    barcode TEXT UNIQUE,  -- Can be NULL for products without barcodes
    full_name TEXT NOT NULL,
    user_price REAL NOT NULL,  -- Price in decimal (e.g., 10.50)
    cost_price REAL,           -- Cost for profit calculation
    min_amount REAL DEFAULT 0, -- Minimum stock alert
    current_amount REAL DEFAULT 0, -- Current inventory
    unit_measurement_id INTEGER NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),

    FOREIGN KEY (unit_measurement_id) REFERENCES unit_measurement(id)
);

CREATE INDEX idx_product_barcode ON product(barcode);
CREATE INDEX idx_product_name ON product(full_name);
CREATE INDEX idx_product_created ON product(created_at);

CREATE TABLE IF NOT EXISTS sale (
    id TEXT PRIMARY KEY,  -- UUID as TEXT
    total_amount REAL NOT NULL,
    paid_amount REAL NOT NULL,
    change_amount REAL DEFAULT 0,
    is_loan INTEGER DEFAULT 0, -- Boolean: 0 = fully paid, 1 = loan/partial payment
    sold_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX idx_sale_date ON sale(sold_at);
CREATE INDEX idx_sale_is_loan ON sale(is_loan);

CREATE TABLE IF NOT EXISTS operation (
    id TEXT PRIMARY KEY,  -- UUID as TEXT
    sale_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    product_name TEXT NOT NULL, -- Denormalized for receipt generation
    quantity REAL NOT NULL,
    unit_price REAL NOT NULL,
    subtotal REAL NOT NULL,
    recorded_at TEXT DEFAULT (datetime('now')),

    FOREIGN KEY (sale_id) REFERENCES sale(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES product(id)
);

CREATE INDEX idx_operation_sale ON operation(sale_id);
CREATE INDEX idx_operation_product ON operation(product_id);
CREATE INDEX idx_operation_date ON operation(recorded_at);

CREATE TABLE IF NOT EXISTS loan (
    id TEXT PRIMARY KEY,  -- References sale.id
    total_debt REAL NOT NULL,
    paid_amount REAL DEFAULT 0,
    remaining_amount REAL NOT NULL,
    debtor_name TEXT NOT NULL,
    debtor_phone TEXT,  -- Phone number as identifier
    status_id INTEGER NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),

    FOREIGN KEY (id) REFERENCES sale(id),
    FOREIGN KEY (status_id) REFERENCES status_loan(id)
);

CREATE INDEX idx_loan_debtor_name ON loan(debtor_name);
CREATE INDEX idx_loan_debtor_phone ON loan(debtor_phone);
CREATE INDEX idx_loan_status ON loan(status_id);

CREATE TABLE IF NOT EXISTS loan_payment (
    id TEXT PRIMARY KEY,  -- UUID as TEXT
    loan_id TEXT NOT NULL,
    amount REAL NOT NULL,
    payment_date TEXT DEFAULT (datetime('now')),
    notes TEXT,

    FOREIGN KEY (loan_id) REFERENCES loan(id) ON DELETE CASCADE
);

CREATE INDEX idx_loan_payment_loan ON loan_payment(loan_id);
CREATE INDEX idx_loan_payment_date ON loan_payment(payment_date);

-- Insert default catalog data

INSERT INTO item_condition (id, description) VALUES
    (1, 'Good'),
    (2, 'Damaged'),
    (3, 'Expired');

INSERT INTO status_loan (id, description) VALUES
    (1, 'Active'),
    (2, 'Partially Paid'),
    (3, 'Fully Paid'),
    (4, 'Cancelled');

INSERT INTO unit_measurement (id, description, abbreviation) VALUES
    (1, 'Kilogram', 'kg'),
    (2, 'Liter', 'lt'),
    (3, 'Unit', 'unit'),
    (4, 'Piece', 'pcs'),
    (5, 'Box', 'box'),
    (6, 'Can', 'can'),
    (7, 'Bottle', 'bottle');
```

---

## Phase 2: Data Layer (Models + Repository)

### 2.1 Define Models

Create model structs matching SQLite schema:

**Files to create:**
- `src/models/mod.rs` - Export all models
- `src/models/product.rs` - Product entity
- `src/models/sale.rs` - Sale entity
- `src/models/operation.rs` - Operation (sale items)
- `src/models/loan.rs` - Loan entity
- `src/models/loan_payment.rs` - Payment tracking
- `src/models/catalogs.rs` - Reference data (units, statuses)

### 2.2 Implement Repository Pattern

**Files to create:**
- `src/repo/mod.rs`
- `src/repo/traits.rs` - Define repository traits:
  - `ProductRepository`
  - `SaleRepository`
  - `LoanRepository`
- `src/repo/product_repo.rs` - Product CRUD + search by barcode/name
- `src/repo/sale_repo.rs` - Sale CRUD + operations
- `src/repo/loan_repo.rs` - Loan tracking + payments
- `src/repo/sqlite_impl.rs` - SQLite connection pool management

**Key Repository Methods:**

**ProductRepository:**
- `create(product)` → Result<Product>
- `update(id, product)` → Result<Product>
- `get_by_id(id)` → Result<Option<Product>>
- `get_by_barcode(barcode)` → Result<Option<Product>>
- `search_by_name(query)` → Result<Vec<Product>>
- `list_all()` → Result<Vec<Product>>
- `delete(id)` → Result<()>
- `update_stock(id, quantity_delta)` → Result<()>
- `get_low_stock_items()` → Result<Vec<Product>>

**SaleRepository:**
- `create_sale(sale, operations)` → Result<Sale>
- `get_by_id(id)` → Result<Option<Sale>>
- `get_operations(sale_id)` → Result<Vec<Operation>>
- `list_sales(from_date, to_date)` → Result<Vec<Sale>>
- `get_daily_summary(date)` → Result<SalesSummary>

**LoanRepository:**
- `create_loan(sale_id, loan)` → Result<Loan>
- `add_payment(loan_id, payment)` → Result<LoanPayment>
- `get_loan(id)` → Result<Option<Loan>>
- `get_payments(loan_id)` → Result<Vec<LoanPayment>>
- `list_active_loans()` → Result<Vec<Loan>>
- `search_by_debtor(phone_or_name)` → Result<Vec<Loan>>
- `update_status(loan_id, status)` → Result<()>

---

## Phase 3: Business Logic Layer (API)

### 3.1 Inventory API

**File:** `src/api/inventory.rs`

**Functions:**
- `add_product(product_data)` - Create new product with validation
- `update_product(id, updates)` - Update product details
- `delete_product(id)` - Remove product
- `search_product(query)` - Search by name or barcode
- `adjust_stock(product_id, quantity, reason)` - Manual stock adjustment
- `get_inventory_report()` - Current stock levels
- `get_low_stock_alerts()` - Products below min_amount

### 3.2 Sales API

**File:** `src/api/sales.rs`

**Functions:**
- `create_sale(cart_items, payment_info)` - Process complete sale
  - Validate stock availability
  - Update inventory
  - Generate sale record
  - Create PDF receipt
  - Handle partial payment (create loan if needed)
- `void_sale(sale_id)` - Cancel sale and restore inventory
- `get_sale_details(sale_id)` - Retrieve sale with items
- `get_sales_report(date_range)` - Sales analytics
- `generate_receipt_pdf(sale_id)` - Create PDF ticket

### 3.3 Loans API

**File:** `src/api/loans.rs`

**Functions:**
- `create_loan_from_sale(sale_id, debtor_info)` - Create loan record
- `add_payment(loan_id, amount)` - Record payment
  - Update loan balance
  - Update status if fully paid
- `get_loan_details(loan_id)` - Loan with payment history
- `search_debtor_loans(phone)` - Find by phone number
- `get_overdue_loans()` - Loans past due date (if tracking)
- `get_loan_summary()` - Total outstanding debt

---

## Phase 4: Handlers Layer

### 4.1 Purpose
Bridge between Dioxus views (UI) and API layer. Handle:
- User input validation
- State management
- Error formatting for UI
- Async operation coordination

### 4.2 Files to Create
- `src/handlers/mod.rs`
- `src/handlers/inventory_handler.rs`
- `src/handlers/sales_handler.rs`
- `src/handlers/loans_handler.rs`

### 4.3 Example Handler Pattern

```rust
pub struct InventoryHandler {
    repo: Arc<dyn ProductRepository>,
}

impl InventoryHandler {
    pub async fn handle_add_product(&self, form_data: ProductForm) -> Result<Product, String> {
        // Validate input
        form_data.validate().map_err(|e| format!("Validation error: {}", e))?;

        // Call API layer
        let product = api::inventory::add_product(form_data, &self.repo)
            .await
            .map_err(|e| format!("Failed to add product: {}", e))?;

        Ok(product)
    }
}
```

---

## Phase 5: Views Layer (Dioxus UI)

### 5.1 Application Structure

**Main App Component:** `src/views/app.rs`
- Navigation menu
- Route handling
- Global state management

### 5.2 Inventory Module

**Files:**
- `src/views/inventory/mod.rs`
- `src/views/inventory/product_list.rs` - Display all products, search
- `src/views/inventory/product_form.rs` - Add/Edit product form
- `src/views/inventory/stock_adjustment.rs` - Adjust inventory

**Features:**
- Product table with search/filter
- Barcode scanner input
- Unit measurement selector
- Low stock indicators

### 5.3 Sales Module

**Files:**
- `src/views/sales/mod.rs`
- `src/views/sales/sale_view.rs` - Main POS interface
- `src/views/sales/cart.rs` - Shopping cart component
- `src/views/sales/payment_form.rs` - Payment processing
- `src/views/sales/receipt.rs` - Receipt preview

**Features:**
- Product search/scan
- Cart management (add/remove/quantity)
- Running total display
- Payment input (full/partial)
- PDF receipt generation trigger

### 5.4 Loans Module

**Files:**
- `src/views/loans/mod.rs`
- `src/views/loans/loan_list.rs` - Active loans list
- `src/views/loans/loan_detail.rs` - Payment history
- `src/views/loans/payment_form.rs` - Record payment

**Features:**
- Search by debtor name/phone
- Outstanding balance display
- Payment history timeline
- Quick payment input

---

## Phase 6: Utilities

### 6.1 PDF Generator

**File:** `src/utils/pdf_generator.rs`

**Function:**
```rust
pub fn generate_sale_receipt(
    sale: &Sale,
    operations: &[Operation],
    debtor_info: Option<&str>,
) -> Result<Vec<u8>, Error>
```

**Content:**
- Business header (name, address)
- Sale date/time
- Itemized list (product, qty, price, subtotal)
- Total amount
- Paid amount
- Change or remaining debt
- Debtor info (if loan)

### 6.2 Other Utilities

**Files:**
- `src/utils/validation.rs` - Common validators (barcode, phone, money)
- `src/utils/formatting.rs` - Money formatting, date formatting
- `src/utils/barcode.rs` - Barcode parsing/validation
- `src/utils/db.rs` - Database initialization

---

## Phase 7: Configuration & Error Handling

### 7.1 Configuration

**File:** `src/config.rs`

```rust
pub struct AppConfig {
    pub database_path: String,
    pub business_name: String,
    pub business_address: String,
    pub tax_rate: f64,
}
```

### 7.2 Error Handling

**File:** `src/error.rs`

Define application errors:
- `DatabaseError`
- `ValidationError`
- `InsufficientStockError`
- `NotFoundError`
- `PdfGenerationError`

---

## Implementation Steps (Detailed)

### Step 1: Clean Slate
- [ ] Backup current project
- [ ] Create new branch `migrate-to-dioxus`
- [ ] Remove all old `src/` files (keep migrations for reference)
- [ ] Remove `crates/` directory
- [ ] Update `Cargo.toml` with new dependencies

### Step 2: Database Setup
- [ ] Create `migrations/sqlite_schema.sql`
- [ ] Create `src/utils/db.rs` for database initialization
- [ ] Write database connection pool setup

### Step 3: Models Layer
- [ ] Create `src/models/mod.rs`
- [ ] Implement all model structs (product, sale, operation, loan, etc.)
- [ ] Add serde derives for serialization

### Step 4: Repository Layer
- [ ] Define repository traits in `src/repo/traits.rs`
- [ ] Implement `ProductRepository`
- [ ] Implement `SaleRepository`
- [ ] Implement `LoanRepository`
- [ ] Write unit tests for repositories

### Step 5: API Layer
- [ ] Implement `src/api/inventory.rs`
- [ ] Implement `src/api/sales.rs`
- [ ] Implement `src/api/loans.rs`
- [ ] Add business logic tests

### Step 6: Handlers Layer
- [ ] Create handler structs
- [ ] Implement inventory handlers
- [ ] Implement sales handlers
- [ ] Implement loan handlers

### Step 7: Utilities
- [ ] Implement PDF generator
- [ ] Create validation utilities
- [ ] Create formatting utilities
- [ ] Implement barcode utilities

### Step 8: Views - Basic App Shell
- [ ] Create `src/main.rs` with Dioxus desktop setup
- [ ] Create root `App` component
- [ ] Implement navigation menu
- [ ] Set up routing (if using dioxus-router)

### Step 9: Views - Inventory Module
- [ ] Create product list view
- [ ] Create product form (add/edit)
- [ ] Implement search functionality
- [ ] Add stock adjustment UI

### Step 10: Views - Sales Module
- [ ] Create main POS interface
- [ ] Implement cart component
- [ ] Create payment form
- [ ] Integrate PDF generation
- [ ] Add receipt preview

### Step 11: Views - Loans Module
- [ ] Create loans list view
- [ ] Implement loan detail view
- [ ] Create payment form
- [ ] Add debtor search

### Step 12: Integration & Testing
- [ ] End-to-end testing of complete sale flow
- [ ] Test inventory updates
- [ ] Test loan creation and payments
- [ ] Test PDF generation
- [ ] Performance testing

### Step 13: Polish & Deployment
- [ ] Add error handling and user feedback
- [ ] Improve UI/UX
- [ ] Add loading states
- [ ] Create app icon
- [ ] Build for Windows and macOS
- [ ] Create installation instructions

---

## Key Decisions & Rationale

### Why Dioxus?
- **Stability**: More stable than Iced, better for desktop apps
- **React-like**: Familiar component model
- **Cross-platform**: Single codebase for Windows, macOS, Linux
- **Active development**: Regular updates, good community

### Why SQLite?
- **Embedded**: No external database server needed
- **Portable**: Single file database
- **Reliable**: Battle-tested, ACID compliant
- **Sufficient**: Perfect for single-user POS system

### Architecture Benefits
- **Separation of concerns**: Clear boundaries between layers
- **Testability**: Each layer can be tested independently
- **Maintainability**: Easy to locate and modify code
- **Scalability**: Easy to add new features

### Database Schema Improvements
- **Simplified inventory**: Merged `catalog` into `product` table
- **Denormalization**: Store product name in operations for receipts
- **Loan tracking**: Direct relationship sale → loan
- **Phone numbers**: Added debtor_phone for easy lookup

---

## Risk Mitigation

### Data Migration
- **Risk**: Losing existing data during migration
- **Mitigation**: Create PostgreSQL → SQLite export script if data exists

### Learning Curve
- **Risk**: Team unfamiliar with Dioxus
- **Mitigation**: Start with simple components, refer to Dioxus documentation

### Performance
- **Risk**: SQLite slower than PostgreSQL for complex queries
- **Mitigation**: Add indexes, denormalize where needed, profile queries

---

## Timeline Estimate

- **Phase 1-2** (Setup + Data Layer): 2-3 days
- **Phase 3-4** (API + Handlers): 2-3 days
- **Phase 5** (Views): 4-5 days
- **Phase 6-7** (Utils + Config): 1-2 days
- **Testing & Polish**: 2-3 days

**Total**: 11-16 days of focused development

---

## Success Criteria

- [ ] Application builds without errors
- [ ] All three modules (inventory, sales, loans) functional
- [ ] PDF receipts generate correctly
- [ ] Inventory updates on sales
- [ ] Loan tracking and payments work
- [ ] Barcode scanning works
- [ ] Application runs on Windows and macOS
- [ ] No critical bugs in core workflows

---

## Next Steps

1. **Review and approve this plan**
2. **Set up development environment**
3. **Begin with Phase 1: Clean slate and dependency setup**
4. **Implement layer by layer, testing as you go**

---

## Notes

- Keep the old PostgreSQL code in a separate branch for reference
- Document any deviations from this plan
- Update this document as new requirements emerge
- Consider adding automated tests from the start

---

**Document Version**: 1.0
**Last Updated**: 2025-11-30
**Author**: Migration Planning Team
