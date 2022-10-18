-- Add down migration script here
DROP PROCEDURE IF EXISTS sp_save_product_catalog
(
   barcode VARCHAR(100),
   full_name VARCHAR(100),
   user_price MONEY,
   min_amount NUMERIC(5,3),
   unit_measurement_id SMALLINT,
   cost MONEY,
   amount_product NUMERIC(5, 3)
); 