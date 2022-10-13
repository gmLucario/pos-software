use iced::scrollable;

use crate::models::catalog::ProductsToBuy;

#[derive(Default)]
pub struct ToBuy {
    pub products: Vec<ProductsToBuy>,
    pub scroll_list_state: scrollable::State,
}

impl ToBuy {
    pub fn new() -> Self {
        Self {
            products: vec![],
            scroll_list_state: scrollable::State::new(),
        }
    }
}
