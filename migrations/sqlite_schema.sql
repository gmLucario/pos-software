-- SQLite Schema for POS Application
-- Replaces PostgreSQL schema with SQLite-compatible types

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
    abbreviation TEXT NOT NULL
);

-- Core Tables

CREATE TABLE IF NOT EXISTS product (
    id TEXT PRIMARY KEY,  -- UUID as TEXT
    barcode TEXT UNIQUE,  -- Can be NULL for products without barcodes
    full_name TEXT NOT NULL,
    user_price TEXT NOT NULL,  -- Price as Decimal stored as TEXT (e.g., "10.50")
    cost_price TEXT,           -- Cost for profit calculation as Decimal
    min_amount REAL DEFAULT 0, -- Minimum stock alert (quantity)
    current_amount REAL DEFAULT 0, -- Current inventory (quantity)
    unit_measurement_id INTEGER NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),

    FOREIGN KEY (unit_measurement_id) REFERENCES unit_measurement(id)
);

CREATE INDEX IF NOT EXISTS idx_product_barcode ON product(barcode);
CREATE INDEX IF NOT EXISTS idx_product_name ON product(full_name);
CREATE INDEX IF NOT EXISTS idx_product_created ON product(created_at);

CREATE TABLE IF NOT EXISTS sale (
    id TEXT PRIMARY KEY,  -- UUID as TEXT
    total_amount TEXT NOT NULL,  -- Total as Decimal stored as TEXT
    paid_amount TEXT NOT NULL,   -- Paid amount as Decimal stored as TEXT
    change_amount TEXT DEFAULT '0',  -- Change as Decimal stored as TEXT
    is_loan INTEGER DEFAULT 0, -- Boolean: 0 = fully paid, 1 = loan/partial payment
    sold_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_sale_date ON sale(sold_at);
CREATE INDEX IF NOT EXISTS idx_sale_is_loan ON sale(is_loan);

CREATE TABLE IF NOT EXISTS operation (
    id TEXT PRIMARY KEY,  -- UUID as TEXT
    sale_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    product_name TEXT NOT NULL, -- Denormalized for receipt generation
    quantity REAL NOT NULL,  -- Quantity (can be decimal for kg, lt)
    unit_price TEXT NOT NULL,  -- Price as Decimal stored as TEXT
    subtotal TEXT NOT NULL,    -- Subtotal as Decimal stored as TEXT
    recorded_at TEXT DEFAULT (datetime('now')),

    FOREIGN KEY (sale_id) REFERENCES sale(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES product(id)
);

CREATE INDEX IF NOT EXISTS idx_operation_sale ON operation(sale_id);
CREATE INDEX IF NOT EXISTS idx_operation_product ON operation(product_id);
CREATE INDEX IF NOT EXISTS idx_operation_date ON operation(recorded_at);

CREATE TABLE IF NOT EXISTS loan (
    id TEXT PRIMARY KEY,  -- References sale.id
    total_debt TEXT NOT NULL,  -- Total debt as Decimal stored as TEXT
    paid_amount TEXT DEFAULT '0',  -- Paid amount as Decimal stored as TEXT
    remaining_amount TEXT NOT NULL,  -- Remaining as Decimal stored as TEXT
    debtor_name TEXT NOT NULL,
    debtor_phone TEXT,  -- Phone number as identifier
    status_id INTEGER NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),

    FOREIGN KEY (id) REFERENCES sale(id),
    FOREIGN KEY (status_id) REFERENCES status_loan(id)
);

CREATE INDEX IF NOT EXISTS idx_loan_debtor_name ON loan(debtor_name);
CREATE INDEX IF NOT EXISTS idx_loan_debtor_phone ON loan(debtor_phone);
CREATE INDEX IF NOT EXISTS idx_loan_status ON loan(status_id);

CREATE TABLE IF NOT EXISTS loan_payment (
    id TEXT PRIMARY KEY,  -- UUID as TEXT
    loan_id TEXT NOT NULL,
    amount TEXT NOT NULL,  -- Payment amount as Decimal stored as TEXT
    payment_date TEXT DEFAULT (datetime('now')),
    notes TEXT,

    FOREIGN KEY (loan_id) REFERENCES loan(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_loan_payment_loan ON loan_payment(loan_id);
CREATE INDEX IF NOT EXISTS idx_loan_payment_date ON loan_payment(payment_date);

-- Insert default catalog data

INSERT OR IGNORE INTO item_condition (id, description) VALUES
    (1, 'Good'),
    (2, 'Damaged'),
    (3, 'Expired');

INSERT OR IGNORE INTO status_loan (id, description) VALUES
    (1, 'Active'),
    (2, 'Partially Paid'),
    (3, 'Fully Paid'),
    (4, 'Cancelled');

INSERT OR IGNORE INTO unit_measurement (id, description, abbreviation) VALUES
    (1, 'Kilogram', 'kg'),
    (2, 'Liter', 'lt'),
    (3, 'Unit', 'unit'),
    (4, 'Piece', 'pcs'),
    (5, 'Box', 'box'),
    (6, 'Can', 'can'),
    (7, 'Bottle', 'bottle');
