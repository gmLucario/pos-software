-- Tables

DROP TABLE IF EXISTS item_condition CASCADE;
DROP TABLE IF EXISTS product CASCADE;
DROP TABLE IF EXISTS catalog CASCADE;
DROP TABLE IF EXISTS sale CASCADE;
DROP TABLE IF EXISTS sale_product;
DROP TABLE IF EXISTS loan CASCADE;
DROP TABLE IF EXISTS loan_payment;


-- Indexes

DROP INDEX IF EXISTS item_condition_description_idx;
DROP INDEX IF EXISTS status_loan_description_idx;
DROP INDEX IF EXISTS product_full_name_idx;
DROP INDEX IF EXISTS product_created_at_idx;
DROP INDEX IF EXISTS product_barcode_idx;
DROP INDEX IF EXISTS catalog_priced_at_idx;
DROP INDEX IF EXISTS catalog_condition_idx;
DROP INDEX IF EXISTS sale_saled_at_idx;
DROP INDEX IF EXISTS loan_name_debtor_idx;
