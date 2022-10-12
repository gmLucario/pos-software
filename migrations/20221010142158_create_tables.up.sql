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

-- Models

-- ///////
-- Product
-- ///////

CREATE TABLE IF NOT EXISTS product (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    barcode VARCHAR(100),
    full_name VARCHAR(100),
    user_price MONEY,
    min_amount SMALLINT,
    created_at TIMESTAMP DEFAULT (NOW() AT TIME ZONE 'America/Mexico_City')
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
    priced_at TIMESTAMP DEFAULT (NOW() AT TIME ZONE 'America/Mexico_City'),
    cost MONEY,
    condition_id SMALLINT,

  UNIQUE (product_id, priced_at),
  CONSTRAINT fk_catalog_product_id
    FOREIGN KEY(product_id)
        REFERENCES product(id),
  CONSTRAINT fk_catalog_condition_id
    FOREIGN KEY(condition_id)
        REFERENCES item_condition(id)
);

CREATE INDEX catalog_priced_at_idx ON catalog (priced_at);
CREATE INDEX catalog_condition_idx ON catalog (condition_id);


-- ///////
-- Sale
-- ///////

CREATE TABLE IF NOT EXISTS sale (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    total_price MONEY,
    saled_at TIMESTAMP DEFAULT (NOW() AT TIME ZONE 'America/Mexico_City')
);

CREATE INDEX sale_saled_at_idx ON sale (saled_at);


-- ///////
-- SaleProduct
-- ///////

CREATE TABLE IF NOT EXISTS sale_product (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sale_id UUID,
    product_id UUID,
    amount SMALLINT,
    earning MONEY,

    CONSTRAINT fk_sale_id
        FOREIGN KEY(sale_id)
            REFERENCES sale(id),
    CONSTRAINT fk_sale_product_id
        FOREIGN KEY(product_id)
            REFERENCES product(id)            
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
    id UUID PRIMARY KEY,
    loan_id UUID,
    money_amount MONEY,
    payed_at TIMESTAMP DEFAULT (NOW() AT TIME ZONE 'America/Mexico_City'),

    CONSTRAINT fk_loan_id
        FOREIGN KEY(loan_id)
            REFERENCES loan(id)    
);
