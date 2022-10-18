pub const GET_PRODUCTS_TO_BUY: &str = r#"
select
    product.full_name as product_name,
    product.min_amount - sum("catalog".current_amount)  as amount_to_buy,
    unit_measurement.description as unit_measurement
from "catalog"
left join product on (
    product.id = catalog.product_id 
)
left join unit_measurement on (
	product.unit_measurement_id = unit_measurement.id
)
where
    "catalog".current_amount > 0.0
group by 
    product.full_name,
    product.min_amount,
    unit_measurement.description
having count(1) < product.min_amount
order by amount_to_buy asc;
"#;

pub const GET_PRODUCT_CATALOG_INFO: &str = r#"
select
    product.barcode,
    product.full_name as product_name,
    product.user_price,
    product.min_amount,
    "catalog"."cost",
    unit_measurement_id,
    "catalog".current_amount
from product
left join "catalog" on (
    product.id = "catalog".product_id
)
where
    product.barcode  = $1
    and "catalog".priced_at <= now()
order by "catalog"."cost" desc
limit 1;
"#;

pub const INSERT_PRODUCT_CATALOG: &str = r#"
call sp_save_product_catalog($1, $2, $3, $4, $5, $6, $7);
"#;
