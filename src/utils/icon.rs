/// # Trait that any of the icons have to implement
pub trait Icon {
    /// Get the data
    fn data(&self) -> String;

    /// Get the extension
    fn extension(&self) -> String; 
}

include!(concat!(env!("OUT_DIR"), "/icons/enum.rs"));

include!(concat!(env!("OUT_DIR"), "/icons/impl.rs"));
