use std::collections::HashMap;

use iced::{button, pick_list, scrollable, text_input};

use crate::{kinds::UnitsMeasurement, schemas::catalog::LoadProduct};

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
    pub listen_barcode_device: bool,
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

            listen_barcode_device: false,
            products_to_add: HashMap::new(),
            load_product: LoadProduct::default(),
        }
    }

    pub fn reset_values(&mut self) {
        self.listen_barcode_device = true;
        self.load_product = LoadProduct::default();
    }
}
