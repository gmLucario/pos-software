//! Sql statements to be used by the data repositories

/// Retrieve products with its quantity are less than the minimum required
///
/// These query will give us products to refill the stock
///
/// # Columns
/// | column | description |
/// |---------|---------|
/// | `product_name`     | full name of the product to buy |
/// | `amount_to_buy`    | quantity to buy |
/// | `unit_measurement` | unit measurement of `amount_to_buy` |
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

/// Get minimum info, about a product by its `barcode` to the help the
/// user to fill the catalog form
///
/// # Columns
/// | column | description |
/// |---------|---------|
/// | `barcode`             | required barcode |
/// | `product_name`        | full name of the product |
/// | `user_price`          | price to be charged to the customer |
/// | `min_amount`          | min quantity at the stock |
/// | `cost`                | price that the store payed to own the product |
/// | `unit_measurement_id` | unit type of the quantity |
pub const GET_PRODUCT_CATALOG_INFO: &str = r#"
select
    product.barcode,
    product.full_name as product_name,
    product.user_price,
    product.min_amount,
    "catalog"."cost",
    unit_measurement_id
from product
left join "catalog" on (
    product.id = "catalog".product_id
)
where
    product.barcode  = $1
    and "catalog".priced_at <= now()
order by "catalog".priced_at desc
limit 1;
"#;

/// Call the store procedure (sp) that insert a new product
/// record in the `catalog` table
///
/// The sp update or insert the product info and create a new
/// record in the `catalog` table
///
/// # Columns
/// | param | description |
/// |---------|---------|
/// | `$1` | product `barcode` to be created or updated |
/// | `$2` | full_name of the product |
/// | `$3` | price to be charged to the customer |
/// | `$4` | minimum quantity of the product at the stock |
/// | `$5` | unit measurement type id |
/// | `$6` | price that the store payed to own the product |
/// | `$7` | quantity of the product to be store |
pub const INSERT_PRODUCT_CATALOG: &str = r#"
call sp_save_product_catalog($1, $2, $3, $4, $5, $6, $7);
"#;

/// Get product info to populate sale form product
pub const GET_SALE_PRODUCT_INFO: &str = r#"
select
    product.barcode,
	product.full_name as product_name,
	product.user_price as price,
	product.unit_measurement_id,
	1::numeric(5,3) as amount,
	product.unit_measurement_id,
	sum("catalog".current_amount) as total_amount
from product
left join "catalog" on (
	product.id = "catalog".product_id
)
where product.barcode = $1
group by
	product.barcode,
	product.full_name,
	product.user_price,
	product.unit_measurement_id;
"#;

/// Create new record in `sale` table
///
/// Based on the `client_payment` create a new sale record
pub const INSERT_NEW_SALE: &str = r#"
insert into sale (client_payment) values ($1) returning id;
"#;

/// Get the product id that match the `barcode` provided
pub const GET_PRODUCT_ID_BY_BARCODE: &str = r#"
select product.id from product where barcode = $1;
"#;

/// Retrieve the `catalog` records that sum the required amount of
/// product to be sold
///
/// # About
/// ## CatalogSum
/// Per each `product_id` that matches the `$1` provided, generate a accumulationsum records:
/// `
/// [1, 3, 2] -> accumulation_sum = [1, 4, 6]
/// `
/// ## General Query
/// Use the `CatalogSum` query to get `catalog` items that
/// sum the quantity required to be sold
pub const GET_PRODUCTS_CATALOG_UPDATE_SALE: &str = r#"
with CatalogSum as (
	select
		"catalog".id as catalog_id,
		"catalog".priced_at,
		sum(current_amount) over (partition by product_id order by "catalog".priced_at asc) as product_amount
	from "catalog"
	where
		"catalog".product_id  = $1
)
select 
	"catalog".id as catalog_id,
	"catalog".current_amount as amount,
	"catalog"."cost" as "cost"
from "catalog"
where
	"catalog".priced_at <= (
		select 
			CatalogSum.priced_at
		from CatalogSum
		where CatalogSum.product_amount >= $2
		order by 
			CatalogSum.product_amount asc
		limit 1
	)
	and "catalog".product_id  = $1;
"#;

/// Update `current_amount` value of `catalog.id` item
pub const UPDATE_CATALOG_AMOUNT: &str = r#"
update "catalog"
set
	current_amount = $2
where id = $1;
"#;

/// Delete a `catalog` item by its `id`
pub const DELETE_CATALOG_RECORD: &str = r#"
delete from "catalog"
where id = $1;
"#;

/// Insert a new `operation` record in `operation` table
///
/// when a product is saled each n units of that are
/// saved as an `operation`
pub const CREATE_OPERATION_FROM_CATALOG: &str = r#"
insert into operation (
	product_id,
	amount_product,
	"cost",
	earning,
	condition_id
)
	select
		p.id as product_id,
		(c.current_amount - $2) as amount_product,
		p.user_price as "cost",
		(c.current_amount - $2) * (p.user_price - c."cost") as earning,
		1 as condition_id
	from product p
	left join "catalog" c on (
		c.product_id = p.id
	)
	where c.id = $1
returning id;
"#;

/// Link an `operation` record with a `sale`
pub const INSERT_NEW_SALE_OPERATION: &str = r#"
insert into sale_operation (
	sale_id,
	operation_id
)
values (
	$1,
	$2
);
"#;

/// Insert a new item in `loan` table
///
/// The `loan.id` value must be an exiting `sale.id`
pub const INSERT_NEW_LOAN: &str = r#"
insert into loan (
	id,
	price,
	name_debtor,
	status_loan
)
values (
	$1,
	$2,
	$3,
	3
);
"#;
