pub trait SignalContainer {
    fn new() -> Self where Self: Sized;

    fn reset(&mut self);
}
