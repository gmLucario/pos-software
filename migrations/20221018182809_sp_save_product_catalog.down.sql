-- Add down migration script here
DROP PROCEDURE IF EXISTS sp_save_product_catalog
(
   input_barcode VARCHAR(100),
   input_full_name VARCHAR(100),
   input_user_price MONEY,
   input_min_amount NUMERIC(8, 3),
   input_unit_measurement_id SMALLINT,
   input_cost MONEY,
   input_amount_product NUMERIC(8, 3)
);
