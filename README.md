# POS Software - Point of Sale System

A desktop application built with Rust, designed for small retail stores. Features inventory management, sales processing, and customer loan tracking with a clean, intuitive interface.

## Features

### 🛒 Sales Management
- **Real-time Sales Processing**: Quick product scanning and cart management
- **Cash Payment Method**: Cash sales with automatic change calculation
- **Loan Sales**: Create and track customer credit/loan purchases
- **Receipt Generation**: Detailed sale receipts with itemized products
- **Sales History**: Search and view past transactions by date range or customer

### 📦 Inventory Management
- **Product Catalog**: Complete product database with pricing and stock tracking
- **Unit-based Inventory**: Support for both unit-based and quantity-based products
- **Stock Tracking**: Real-time inventory updates with each sale
- **Product Search**: Fast product lookup with barcode support
- **Add/Edit Products**: Easy product management interface

### 💰 Customer Loans
- **Loan Creation**: Create loan records directly from sales
- **Payment Tracking**: Record partial or full loan payments with notes
- **Payment History**: View complete payment timeline with dates and amounts
- **Loan Dashboard**: Overview of active loans with totals and statistics
- **Automatic Calculations**: Real-time remaining balance updates

### 📊 Business Intelligence
- **Sales Statistics**: Daily, weekly, and monthly sales summaries
- **Revenue Tracking**: Total earnings and payment analysis
- **Loan Analytics**: Outstanding debt and payment trends
- **Inventory Insights**: Low stock alerts and product performance

## Technology Stack

- **Language**: Rust 2021 Edition
- **UI Framework**: Dioxus 0.7 (Desktop)
- **Database**: SQLite with sqlx
- **Async Runtime**: Tokio
- **Date/Time**: Chrono with timezone support (America/Mexico_City)
- **Decimal Math**: rust_decimal for precise currency calculations

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd pos-software
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

The application will automatically:
- Create the SQLite database file (`data/pos.db`)
- Run all necessary migrations
- Launch the desktop interface

### Database Setup

The database is automatically initialized on first run. The schema includes:
- `product` - Product catalog with pricing and stock
- `sale` - Sales transactions with payment details
- `operation` - Individual line items for each sale
- `loan` - Customer loan records
- `loan_payment` - Payment history for loans
- `status_loan` - Loan status tracking

## Development

### Building

Development build:
```bash
cargo build
```

Release build (optimized):
```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Code Documentation

Generate and open the documentation:
```bash
cargo doc --no-deps --open
```

### Windows (Cross-compile from macOS)

To build a single executable (no DLLs required) for Windows from macOS, we use `cargo-xwin` with the MSVC toolchain.

1.  **Install prerequisites:**
    ```bash
    brew install llvm
    cargo install cargo-xwin
    ```

2.  **Build for Windows:**
    
    Since LLVM is keg-only on macOS, you need to add it to your PATH for the build:

    ```bash
    export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
    cargo xwin build --target x86_64-pc-windows-msvc --release
    ```

    The executable will be located at:
    `target/x86_64-pc-windows-msvc/release/pos-app.exe`

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Dioxus](https://dioxuslabs.com/) - A portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust
- Database powered by [SQLx](https://github.com/launchbadge/sqlx) - The Rust SQL Toolkit
- Currency calculations with [rust_decimal](https://github.com/paupino/rust-decimal)

---

**Made with ❤️ and Rust**
