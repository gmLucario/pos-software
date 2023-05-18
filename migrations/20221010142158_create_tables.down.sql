-- Tables

DROP TABLE IF EXISTS item_condition CASCADE;
DROP TABLE IF EXISTS status_loan CASCADE;
DROP TABLE IF EXISTS unit_measurement CASCADE;
DROP TABLE IF EXISTS product CASCADE;
DROP TABLE IF EXISTS "catalog" CASCADE;
DROP TABLE IF EXISTS sale CASCADE;
DROP TABLE IF EXISTS sale_operation;
DROP TABLE IF EXISTS loan CASCADE;
DROP TABLE IF EXISTS loan_payment;
DROP TABLE IF EXISTS operation;


-- Indexes

DROP INDEX IF EXISTS item_condition_description_idx;
DROP INDEX IF EXISTS status_loan_description_idx;
DROP INDEX IF EXISTS product_full_name_idx;
DROP INDEX IF EXISTS product_created_at_idx;
DROP INDEX IF EXISTS product_barcode_idx;
DROP INDEX IF EXISTS catalog_priced_at_idx;
DROP INDEX IF EXISTS sale_sold_at_idx;
DROP INDEX IF EXISTS loan_name_debtor_idx;
DROP INDEX IF EXISTS unit_measurement_description_idx;
DROP INDEX IF EXISTS operation_recorded_at_idx;
DROP INDEX IF EXISTS operation_condition_idx;
