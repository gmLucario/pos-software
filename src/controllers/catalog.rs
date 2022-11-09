//! Handle logic to link [`crate::views::catalog`]
//! module with the [`crate::data::product_repo::ProductRepo`]

use std::collections::HashMap;

use iced::{
    button,
    keyboard::{Event, KeyCode},
    pick_list, scrollable, text_input, Command,
};
use iced_native::Event::Keyboard;
use sqlx::{pool::Pool, postgres::Postgres};

use crate::{
    constants::CHARS_SAVED_AS_BARCODE,
    data::product_repo::ProductRepo,
    kinds::{AppEvents, UnitsMeasurement},
    schemas::catalog::LoadProduct,
};

/// Controller links [`crate::views::catalog`] module with the [`crate::data::product_repo::ProductRepo`]
#[derive(Default)]
pub struct Catalog {
    // widgets states
    /// Iced Scroll state
    pub scroll_list_state: scrollable::State,
    /// Iced pick list state
    pub pick_list_state: pick_list::State<UnitsMeasurement>,

    // Text Input states
    /// Iced text_input state for barcode input
    pub barcode_input_state: text_input::State,
    /// Iced text_input state for product name input
    pub full_name_input_state: text_input::State,
    /// Iced text_input state for barcode price for user input
    pub user_price_input_state: text_input::State,
    /// Iced text_input state for product cost input
    pub cost_input_state: text_input::State,
    /// Iced text_input state for product amount/quantity input
    pub amount_input_state: text_input::State,
    /// Iced text_input state for minimum product amount/quantity input
    pub min_amount_input_state: text_input::State,
    /// Iced text_input state for type of unit measurement input
    pub unit_measurement_input_state: text_input::State,

    // Btns states
    /// Iced button state for button save product to `products_to_add`
    pub save_record_state: button::State,
    /// Iced button state for button cancel form product to be 
    /// added in `products_to_add`
    pub cancel_record_state: button::State,
    /// Iced button state for button triggers the saving process
    /// of all the products in the catalog
    pub save_list_records_state: button::State,

    // Data
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
            scroll_list_state: scrollable::State::new(),
            pick_list_state: pick_list::State::new(),

            barcode_input_state: text_input::State::new(),
            full_name_input_state: text_input::State::new(),
            user_price_input_state: text_input::State::new(),
            cost_input_state: text_input::State::new(),
            amount_input_state: text_input::State::new(),
            min_amount_input_state: text_input::State::new(),
            unit_measurement_input_state: text_input::State::new(),

            save_record_state: button::State::new(),
            cancel_record_state: button::State::new(),
            save_list_records_state: button::State::new(),

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
}
