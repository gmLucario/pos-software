//! Data structures to get user inputs related to sale view

use std::{collections::HashMap, str::FromStr};

use sqlx::{postgres::types::PgMoney, types::BigDecimal};

use crate::{
    constants::{PGMONEY_DECIMALS, TO_DECIMAL_DIGITS},
    kinds::UnitsMeasurement,
    models,
};

/// Represents input values to define a new product to be sold
#[derive(Default, Debug, Clone)]
pub struct ProductToAdd {
    /// `barcode` of the product
    pub barcode: String,
    /// Full name of the product
    pub product_name: String,
    /// Price to be charged to the client
    pub price: String,
    /// Unit of measurement of the product: kg, lts or pieces
    pub unit_measurement: UnitsMeasurement,
    /// Amount/quantity of the product
    pub amount: String,
    /// Amount of the product in the stock
    pub total_amount: BigDecimal,
}

impl ProductToAdd {
    /// Reset the struct field values
    pub fn reset_values(&mut self) {
        *self = Self::default();
    }
}

impl From<models::sale::SaleProductInfo> for ProductToAdd {
    /// Cast from the model to the schema
    fn from(model: models::sale::SaleProductInfo) -> Self {
        let unit_measurement = UnitsMeasurement::from(model.unit_measurement_id);

        Self {
            barcode: model.barcode,
            product_name: model.product_name,
            price: model.price.to_bigdecimal(TO_DECIMAL_DIGITS).to_string(),
            unit_measurement,
            amount: model.amount.to_string(),
            total_amount: model.total_amount,
        }
    }
}

/// Represents values to be shown in the sale list
#[derive(Debug, Clone)]
pub struct ProductList {
    /// full name of the product
    pub product_name: String,
    /// amount/quantity of the pruduct
    pub amount: BigDecimal,
    /// price to be charged per unit
    pub price: PgMoney,
}

impl From<&ProductToAdd> for ProductList {
    /// Cast from user input values schema to list item schema
    fn from(schema: &ProductToAdd) -> Self {
        let amount = BigDecimal::from_str(&schema.amount).unwrap();
        let price = BigDecimal::from_str(&schema.price).unwrap();
        let price = PgMoney::from_bigdecimal(price * &amount, PGMONEY_DECIMALS).unwrap();

        Self {
            product_name: schema.product_name.to_string(),
            amount,
            price,
        }
    }
}

/// Represents the main sale info
#[derive(Debug, Clone)]
pub struct SaleInfo {
    /// Products to be sold
    pub products: HashMap<String, ProductList>,
    /// Total money client will pay
    pub total_pay: PgMoney,
    /// Money client payed
    pub client_pay: String,
    /// Client full name if it was a loan
    pub client_name: String,
    /// Money to give client back
    pub payback_money: PgMoney,
}

impl Default for SaleInfo {
    fn default() -> Self {
        Self {
            products: Default::default(),
            total_pay: PgMoney(0),
            client_pay: Default::default(),
            client_name: Default::default(),
            payback_money: PgMoney(0),
        }
    }
}
