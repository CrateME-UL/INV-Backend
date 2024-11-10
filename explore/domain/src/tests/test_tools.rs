use std::cell::RefCell;
use std::fmt::Debug;

pub struct Mock<T, R> {
    expected_args: RefCell<Option<T>>,
    return_value: RefCell<Option<R>>,
}

impl<T: Clone + PartialEq + Debug, R: Clone + Debug> Mock<T, R> {
    pub fn new() -> Self {
        Self {
            expected_args: RefCell::new(None),
            return_value: RefCell::new(None),
        }
    }

    pub fn when(&self, args: T) -> &Self {
        *self.expected_args.borrow_mut() = Some(args);
        self
    }

    pub fn then_return(&self, value: R) {
        *self.return_value.borrow_mut() = Some(value);
    }

    pub fn call(&self, args: T) -> R {
        if let Some(expected) = &*self.expected_args.borrow() {
            if *expected == args {
                self.return_value
                    .borrow()
                    .clone()
                    .expect("Return value not set")
            } else {
                panic!(
                    "Unexpected argument: expected {:?}, got {:?}",
                    expected, args
                );
            }
        } else {
            panic!("No expected argument set. Please use `when` to set expected arguments.");
        }
    }
}

pub trait Dummy: Default {
    fn dummy() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }
}
pub trait Stub<T>: Default {
    fn stub(&self) -> T
    where
        T: Default,
    {
        T::default()
    }
}
