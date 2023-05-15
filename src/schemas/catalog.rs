//! Data structures to get user inputs related to catalog view

use validator::Validate;

use crate::{
    constants::TO_DECIMAL_DIGITS,
    kinds::UnitsMeasurement,
    models::catalog::{LoadProduct, ProductInfo},
};

/// Represents user input to load a new product to the catalog
#[derive(Default, Debug, Clone, Validate)]
pub struct CatalogProductForm {
    /// `barcode` of the product
    #[validate(length(min = 1))]
    pub barcode: String,
    /// full name of the product
    #[validate(length(min = 1))]
    pub product_name: String,
    /// Price to be charged to the user
    #[validate(length(min = 1))]
    pub user_price: String,
    /// Amount/Quantity of the product
    #[validate(length(min = 1))]
    pub amount: String,
    /// Unit of measurement of the product: kg, lts or pieces
    pub unit_measurement: UnitsMeasurement,
    /// Min amount the catalog must have
    #[validate(length(min = 1))]
    pub min_amount: String,
    /// Product price
    #[validate(length(min = 1))]
    pub cost: String,
}

impl From<Option<ProductInfo>> for CatalogProductForm {
    /// From model to schema
    fn from(model: Option<ProductInfo>) -> Self {
        match model {
            Some(product_info) => {
                let unit_measurement = UnitsMeasurement::from(product_info.unit_measurement_id);

                Self {
                    amount: "1".to_string(),
                    unit_measurement,
                    barcode: product_info.barcode,
                    product_name: product_info.product_name,
                    user_price: product_info
                        .user_price
                        .to_bigdecimal(TO_DECIMAL_DIGITS)
                        .to_string(),
                    min_amount: product_info.min_amount.to_string(),
                    cost: product_info
                        .cost
                        .to_bigdecimal(TO_DECIMAL_DIGITS)
                        .to_string(),
                }
            }
            None => Self::default(),
        }
    }
}

impl From<Option<&LoadProduct>> for CatalogProductForm {
    fn from(optional_product: Option<&LoadProduct>) -> Self {
        match optional_product {
            Some(product) => Self {
                barcode: product.barcode.to_string(),
                amount: product.current_amount.to_string(),
                unit_measurement: product.unit_measurement_id.into(),
                product_name: product.product_name.to_string(),
                user_price: product
                    .user_price
                    .to_bigdecimal(TO_DECIMAL_DIGITS)
                    .to_string(),
                min_amount: product.min_amount.to_string(),
                cost: product.cost.to_bigdecimal(TO_DECIMAL_DIGITS).to_string(),
            },
            None => Self::default(),
        }
    }
}
