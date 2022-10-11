pub const GET_PRODUCTS_TO_BUY: &str = r#"
select
    product.full_name as product_name,
    product.min_amount - COUNT(1) as amount_to_buy
from "catalog"
left join product on (
    product.id = catalog.product_id 
)
where
    "catalog".condition_id = 1
group by 
    product.full_name, product.min_amount
having COUNT(1) < product.min_amount
order by amount_to_buy asc;
"#;
