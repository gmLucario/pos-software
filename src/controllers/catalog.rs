//! Handle logic to link [`crate::views::catalog`]
//! module with the [`crate::data::product_repo::ProductRepo`]

use std::collections::HashMap;

use iced::{
    keyboard::{Event, KeyCode},
    Command,
};
use iced_native::Event::Keyboard;
use sqlx::{pool::Pool, postgres::Postgres};

use crate::{
    constants::CHARS_SAVED_AS_BARCODE, data::product_repo::ProductRepo, kinds::AppEvents,
    schemas::catalog::LoadProduct,
};

/// Controller links [`crate::views::catalog`] module with the [`crate::data::product_repo::ProductRepo`]
#[derive(Default)]
pub struct Catalog {
    /// Hashmap of products to be added in the catalog
    ///
    /// key: `product barcode`
    /// value: [`crate::schemas::catalog::LoadProduct`]
    pub products_to_add: HashMap<String, LoadProduct>,
    /// product info to create a new catalog record
    pub load_product: LoadProduct,
}

/// Handles user input events and link the [`crate::views::catalog`] view with
/// the [`crate::data::product_repo::ProductRepo`] repository
impl Catalog {
    /// Initialize the controller
    pub fn new() -> Self {
        Self {
            products_to_add: HashMap::new(),
            load_product: LoadProduct::default(),
        }
    }

    /// Insert `c` into the product `barcode` if `c` is `is_alphanumeric`
    fn process_char_event(&mut self, c: &char) {
        if c.is_alphanumeric() {
            self.load_product.barcode.push(*c);
        }

        if self.load_product.barcode.len() > CHARS_SAVED_AS_BARCODE {
            self.load_product.barcode.clear();
        }
    }

    /// Process `Keyboard` events to save the `barcode`
    pub fn process_barcode_input(
        &mut self,
        event: iced_native::Event,
        db_connection: &'static Pool<Postgres>,
    ) -> Command<AppEvents> {
        match event {
            Keyboard(Event::CharacterReceived(c)) => self.process_char_event(&c),
            Keyboard(Event::KeyPressed {
                key_code: KeyCode::Enter,
                ..
            }) => {
                if !self.load_product.barcode.is_empty() {
                    return Command::perform(
                        ProductRepo::get_product_info_catalog(
                            db_connection,
                            self.load_product.barcode.to_string(),
                        ),
                        AppEvents::CatalogProductInfoRequested,
                    );
                }
            }
            _ => (),
        }
        Command::none()
    }

    /// Reset variables that stores user input
    pub fn reset_values(&mut self) {
        self.load_product = LoadProduct::default();
    }

    /// Reset inputs products amount depends type of units measurement
    pub fn reset_product_amounts(&mut self) {
        self.load_product.amount = "1".to_string();
        self.load_product.min_amount = "1".to_string();
    }
}
