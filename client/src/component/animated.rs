/// A component with frame-dependent state (e.g. cursor blink, animations).
/// Callers must invoke `tick()` exactly once per frame for correct behavior.
/// Failing to call `tick()` will cause animations to freeze.
pub trait Animated {
    fn tick(&mut self);
}
