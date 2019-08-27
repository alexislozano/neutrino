pub trait Icon {
    fn data(&self) -> String;
    fn extension(&self) -> String; 
}

pub enum BreezeIcon {
    Plus,
    Down,
}

impl Icon for BreezeIcon {
    fn data(&self) -> String {
        match self {
            BreezeIcon::Plus => include_str!(concat!(env!("OUT_DIR"), "/icons/breeze/plus.data")).to_string(),
            BreezeIcon::Down => include_str!(concat!(env!("OUT_DIR"), "/icons/breeze/down.data")).to_string(),
        }
    }

    fn extension(&self) -> String {
        match self {
            BreezeIcon::Plus => include_str!(concat!(env!("OUT_DIR"), "/icons/breeze/plus.extension")).to_string(),
            BreezeIcon::Down => include_str!(concat!(env!("OUT_DIR"), "/icons/breeze/down.extension")).to_string(),
        }
    }
}

pub enum DefaultIcon {
    Plus,
}

impl Icon for DefaultIcon {
    fn data(&self) -> String {
        match self {
            DefaultIcon::Plus => include_str!(concat!(env!("OUT_DIR"), "/icons/default/plus.data")).to_string(),
        }
    }

    fn extension(&self) -> String {
        match self {
            DefaultIcon::Plus => include_str!(concat!(env!("OUT_DIR"), "/icons/default/plus.extension")).to_string(),
        }
    }
}