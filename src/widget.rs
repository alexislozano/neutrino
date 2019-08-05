use crate::utils::Event;

pub trait Widget {
    fn eval(&self) -> String;

    fn trigger(&mut self, event: &Event);
}