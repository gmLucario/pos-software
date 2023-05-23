//! Handle logic to link [`crate::views::catalog`]
//! module with the [`crate::repo::product_repo`]
use std::{collections::BTreeMap, str::FromStr};

use iced::Command;
use sqlx::types::BigDecimal;

use crate::{
    events::AppEvent,
    kinds::View,
    models::catalog::{LoadProduct, ProductAmount, ProductInfo},
    result::AppResult,
    schemas::catalog::CatalogProductForm,
};

/// Controller links [`crate::views::catalog`] module with the [`crate::repo::product_repo`]
#[derive(Default)]
pub struct Catalog {
    /// BTreeMap (ordered Hashmap) of products to be added in the catalog
    ///
    /// key: `product barcode`
    /// value: [`crate::models::catalog::LoadProduct`]
    pub products_to_add: BTreeMap<String, LoadProduct>,
    /// Form info about a product to create a new catalog record
    pub form: CatalogProductForm,
    /// Products available in stock
    pub stock_products: Vec<ProductAmount>,
    /// Product name to filter the `stock_products` list
    pub product_name: String,
    /// Query results page
    pub products_catalog_page: i64,
}

impl Catalog {
    /// Fill catalog form fields based on a `barcode` result from
    /// [`crate::repo::product_repo::get_product_info_catalog`]
    pub fn process_product_info_form_data(
        &mut self,
        result: AppResult<Option<ProductInfo>>,
    ) -> Command<AppEvent> {
        if let Ok(product_info) = result {
            self.form = CatalogProductForm::from(product_info)
        }

        Command::none()
    }

    /// Reset variables that stores user input
    pub fn reset_form_values(&mut self) {
        self.form = CatalogProductForm::default();
    }

    /// Reset inputs products amount depends type of units measurement
    pub fn reset_product_amounts(&mut self) {
        self.form.amount = "1".to_string();
        self.form.min_amount = "1".to_string();
    }

    /// Add/update a record in the list of products to be added in the catalog
    pub fn process_new_product_into_list_to_be_added(
        &mut self,
        is_edit: bool,
    ) -> Command<AppEvent> {
        self.insert_or_update_from_form(is_edit);

        self.reset_values_form();

        Command::perform(async {}, |_| {
            AppEvent::ChangeView(View::CatalogProductsToBeAdded)
        })
    }

    /// Insert a new item in `products_to_add` list
    fn insert_or_update_from_form(&mut self, is_edit: bool) {
        let mut amount = BigDecimal::from_str(&self.form.amount).unwrap_or_default();
        amount = match self.products_to_add.get(&self.form.barcode) {
            Some(product) => {
                if is_edit {
                    amount
                } else {
                    amount + &product.current_amount
                }
            }
            None => amount,
        };

        let barcode = self.form.barcode.to_string();

        self.products_to_add
            .entry(barcode)
            .and_modify(|element| {
                element.current_amount = amount;
            })
            .or_insert_with(|| LoadProduct::from(&self.form));
    }

    /// Reset form field values
    pub fn reset_values_form(&mut self) {
        self.form = CatalogProductForm::default()
    }

    /// To lowercase the trimed product name value
    pub fn product_name_to_lowercase(&mut self) -> String {
        self.product_name.trim().to_lowercase()
    }
}
