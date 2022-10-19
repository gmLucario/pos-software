CREATE OR REPLACE PROCEDURE sp_save_product_catalog(
   input_barcode VARCHAR(100),
   input_full_name VARCHAR(100),
   input_user_price MONEY,
   input_min_amount NUMERIC(5,3),
   input_unit_measurement_id SMALLINT,
   input_cost MONEY,
   input_amount_product NUMERIC(5, 3)
)
language plpgsql    
AS $$
BEGIN
DECLARE
    id_product product.id%TYPE;
    BEGIN
        INSERT INTO product 
            (
                barcode, full_name, user_price, min_amount, unit_measurement_id  
            )
        VALUES (input_barcode, input_full_name, input_user_price, input_min_amount, input_unit_measurement_id)
        ON conflict (barcode) do
        UPDATE SET
            full_name = input_full_name,
            user_price = input_user_price,
            min_amount = input_min_amount,
            unit_measurement_id = input_unit_measurement_id
        returning id INTO id_product;

	INSERT INTO "catalog"
	    (
	        product_id,
	        "cost",
	        current_amount
	    )
	 VALUES (id_product, input_cost, input_amount_product);
    END;

    COMMIT;
END;$$