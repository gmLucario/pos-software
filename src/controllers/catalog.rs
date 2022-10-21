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

#[derive(Default)]
pub struct Catalog {
    // widgets states
    pub scroll_list_state: scrollable::State,
    pub pick_list_state: pick_list::State<UnitsMeasurement>,

    // Text Input states
    pub barcode_input_state: text_input::State,
    pub full_name_input_state: text_input::State,
    pub user_price_input_state: text_input::State,
    pub cost_input_state: text_input::State,
    pub amount_input_state: text_input::State,
    pub min_amount_input_state: text_input::State,
    pub unit_measurement_input_state: text_input::State,

    // Btns states
    pub save_record_state: button::State,
    pub cancel_record_state: button::State,
    pub save_list_records_state: button::State,

    // Data
    pub products_to_add: HashMap<String, LoadProduct>,
    pub load_product: LoadProduct,
}

impl Catalog {
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

    fn process_char_event(&mut self, c: &char) {
        if c.is_alphanumeric() {
            self.load_product.barcode.push(*c);
        }

        if self.load_product.barcode.len() > CHARS_SAVED_AS_BARCODE {
            self.load_product.barcode.clear();
        }
    }

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

    pub fn reset_values(&mut self) {
        self.load_product = LoadProduct::default();
    }
}
