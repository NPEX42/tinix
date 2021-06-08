pub fn test_runner(_tests: &[&dyn Testable]) {
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
    }
}
