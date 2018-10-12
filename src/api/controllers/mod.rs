use super::Router;

mod users;
mod tasks;

pub trait Engage {
    fn engage(self) -> Self;
}

impl Engage for Router {
    fn engage(mut self) -> Self {
        users::engage(&mut self);
        tasks::engage(&mut self);
        self
    }
}
