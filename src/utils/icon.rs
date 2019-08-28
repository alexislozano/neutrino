pub trait Icon {
    fn data(&self) -> String;
    fn extension(&self) -> String; 
}

include!(concat!(env!("OUT_DIR"), "/icons/enum.rs"));

include!(concat!(env!("OUT_DIR"), "/icons/impl.rs"));
