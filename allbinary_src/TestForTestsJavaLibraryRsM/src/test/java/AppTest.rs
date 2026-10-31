// Generated Code Do Not Modify
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::junit::jupiter::api::Assertions::Assertions;
use crate::org::junit::jupiter::api::DisplayName::DisplayName;
use crate::org::junit::jupiter::api::Test::Test;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AppTest {
}


impl AppTest {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn testAddition(&mut self) {
        Assertions::assertEquals(2, 1 + 1);
    }
    

    pub fn testStringEquality(&mut self) {
        let mut expected: String = String::from("Hello World!");
        let mut actual: String = String::from("Hello World!");
        Assertions::assertEquals(expected.clone(), actual.clone());
    }
    

    pub fn testBooleanAssertions(&mut self) {
        Assertions::assertTrue(true);
        Assertions::assertFalse(false);
    }
}

