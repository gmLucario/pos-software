CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Catalogs

-- ///////
-- ItemCondition
-- ///////

CREATE TABLE IF NOT EXISTS item_condition (
    id SMALLINT PRIMARY KEY,
    description VARCHAR(100)
);

CREATE INDEX item_condition_description_idx ON item_condition (description);

-- ///////
-- StatusLoan
-- ///////

CREATE TABLE IF NOT EXISTS status_loan (
    id SMALLINT PRIMARY KEY,
    description VARCHAR(100)
);

CREATE INDEX status_loan_description_idx ON status_loan (description);

-- ///////
-- Unit
-- ///////

CREATE TABLE IF NOT EXISTS unit_measurement (
    id SMALLINT PRIMARY KEY,
    description VARCHAR(100)
);

CREATE INDEX unit_measurement_description_idx ON unit_measurement (description);

-- Models

-- ///////
-- Product
-- ///////

CREATE TABLE IF NOT EXISTS product (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    barcode VARCHAR(100) UNIQUE,
    full_name VARCHAR(100),
    user_price MONEY,
    min_amount NUMERIC(8, 3),
    unit_measurement_id SMALLINT,
    created_at TIMESTAMP DEFAULT (NOW() AT TIME ZONE 'America/Mexico_City'),

    CONSTRAINT fk_product_unit_measurement_id
        FOREIGN KEY(unit_measurement_id)
            REFERENCES unit_measurement(id)
);

CREATE INDEX product_full_name_idx ON product (full_name);
CREATE INDEX product_created_at_idx ON product (created_at);
CREATE INDEX product_barcode_idx ON product (barcode);


-- ///////
-- Catalog
-- ///////

CREATE TABLE IF NOT EXISTS catalog (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID,
    cost MONEY,
    current_amount NUMERIC(8, 3),
    priced_at TIMESTAMP DEFAULT (NOW() AT TIME ZONE 'America/Mexico_City'),

  CONSTRAINT fk_catalog_product_id
    FOREIGN KEY(product_id)
        REFERENCES product(id)
);

CREATE INDEX catalog_priced_at_idx ON catalog (priced_at);

-- ///////
-- Operation
-- ///////
CREATE TABLE IF NOT EXISTS operation (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID,
    amount_product NUMERIC(8, 3),
    user_price MONEY,
    earning MONEY,
    condition_id SMALLINT,
    recorded_at TIMESTAMP DEFAULT (NOW() AT TIME ZONE 'America/Mexico_City'),

  CONSTRAINT fk_operation_product_id
    FOREIGN KEY(product_id)
        REFERENCES product(id),
  CONSTRAINT fk_operation_condition_id
    FOREIGN KEY(condition_id)
        REFERENCES item_condition(id)
);

CREATE INDEX operation_recorded_at_idx ON operation (recorded_at);
CREATE INDEX operation_condition_idx ON operation (condition_id);

-- ///////
-- Sale
-- ///////

CREATE TABLE IF NOT EXISTS sale (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    client_payment MONEY,
    sold_at TIMESTAMP DEFAULT (NOW() AT TIME ZONE 'America/Mexico_City')
);

CREATE INDEX sale_sold_at_idx ON sale (sold_at);


-- ///////
-- SaleProduct
-- ///////

CREATE TABLE IF NOT EXISTS sale_operation (
    sale_id UUID,
    operation_id UUID,

    CONSTRAINT fk_sale_id
        FOREIGN KEY(sale_id)
            REFERENCES sale(id),
    CONSTRAINT fk_sale_operation_id
        FOREIGN KEY(operation_id)
            REFERENCES operation(id),
    PRIMARY KEY (sale_id, operation_id)
);

-- ///////
-- Loan
-- ///////

CREATE TABLE IF NOT EXISTS loan (
    id UUID PRIMARY KEY REFERENCES sale,
    price MONEY,
    name_debtor VARCHAR(100),
    status_loan SMALLINT,
    
    CONSTRAINT fk_loan_status_loan
        FOREIGN KEY(status_loan)
            REFERENCES status_loan(id)
);

CREATE INDEX loan_name_debtor_idx ON loan (name_debtor);


-- ///////
-- LoanPayments
-- ///////

CREATE TABLE IF NOT EXISTS loan_payment (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loan_id UUID,
    money_amount MONEY,
    payed_at TIMESTAMP DEFAULT (NOW() AT TIME ZONE 'America/Mexico_City'),

    CONSTRAINT fk_loan_id
        FOREIGN KEY(loan_id)
            REFERENCES loan(id)
);
