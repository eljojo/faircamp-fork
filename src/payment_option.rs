use crate::util;

#[derive(Clone, Debug)]
pub enum PaymentOption {
    Custom(String),
    Liberapay(String)
}

impl PaymentOption {
    pub fn init_custom(markdown: &str) -> PaymentOption {
        PaymentOption::Custom(util::markdown_to_html(markdown))
    }
    
    pub fn init_liberapay(account_name: &str) -> PaymentOption {
        PaymentOption::Liberapay(account_name.to_string())
    }
}