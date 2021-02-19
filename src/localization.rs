pub struct Localization {
    pub language: String,
    pub writing_direction: WritingDirection
}

pub enum WritingDirection {
    Ltr,
    Rtl
}

impl Localization {
    pub fn defaults() -> Localization {
        Localization {
            language: String::from("en"),
            writing_direction: WritingDirection::Ltr
        }
    }
}