use std::collections::HashMap;

use iced::{button, keyboard::Event, keyboard::KeyCode, scrollable, text_input, Command};
use iced_native::Event::Keyboard;
use sqlx::{pool::Pool, postgres::Postgres};

use crate::{
    constants::CHARS_SAVED_AS_BARCODE, data::product_repo::ProductRepo, kinds::AppEvents,
    models::sale::SaleProductInfo, schemas::sale::ProductToAdd,
};

#[derive(Default)]
pub struct Sale {
    // States
    pub amount_input_state: text_input::State,
    pub cancel_new_record_btn_state: button::State,
    pub ok_new_record_btn_state: button::State,
    pub scroll_list_state: scrollable::State,

    // Data
    pub product_to_add: ProductToAdd,
    pub products: HashMap<String, SaleProductInfo>,
}

impl Sale {
    fn process_char_event(&mut self, c: &char) {
        if c.is_alphanumeric() {
            self.product_to_add.barcode.push(*c);
        }

        if self.product_to_add.barcode.len() > CHARS_SAVED_AS_BARCODE {
            self.product_to_add.barcode.clear();
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
                if !self.product_to_add.barcode.is_empty() {
                    return Command::perform(
                        ProductRepo::get_product_info_sale(
                            db_connection,
                            self.product_to_add.barcode.to_string(),
                        ),
                        AppEvents::SaleProductInfoRequested,
                    );
                }
            }
            _ => (),
        }
        Command::none()
    }
}
