// SPDX-FileCopyrightText: 2021-2023 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::markdown;

#[derive(Clone, Debug)]
pub enum PaymentOption {
    Custom(String),
    Liberapay(String)
}

impl PaymentOption {
    pub fn init_custom(markdown_text: &str) -> PaymentOption {
        PaymentOption::Custom(markdown::to_html(markdown_text))
    }
    
    pub fn init_liberapay(account_name: &str) -> PaymentOption {
        PaymentOption::Liberapay(account_name.to_string())
    }
}