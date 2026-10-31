// Generated Code Do Not Modify
// package org.allbinary.time
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Calendar;
use super::*;
// use crate::java::util::Calendar::Calendar;
use crate::org::junit::jupiter::api::Assertions::Assertions;
use crate::org::junit::jupiter::api::DisplayName::DisplayName;
use crate::org::junit::jupiter::api::Test::Test;
use super::TimeTypeUtil::TimeTypeUtil;
use super::TestableTimeTypeUtil::TestableTimeTypeUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TimeTypeTest {
}


impl TimeTypeTest {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn millisForHourLocal(mut hour: i32) -> i64 {
        let mut cal: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        jcall!(cal).set(unsafe { crate::java::util::Calendar::Calendar_HOUR_OF_DAY.clone() }.clone(), hour.clone());
        jcall!(cal).set(unsafe { crate::java::util::Calendar::Calendar_MINUTE.clone() }.clone(), 0);
        jcall!(cal).set(unsafe { crate::java::util::Calendar::Calendar_SECOND.clone() }.clone(), 0);
        jcall!(cal).set(unsafe { crate::java::util::Calendar::Calendar_MILLISECOND.clone() }.clone(), 0);
        return jcall!(cal).getTimeInMillis();
    }
    

    pub fn isNight_returnsTrue_for_hours_less_than_6(&mut self) {
        /* Otherwise - statement - ForEachStmt */
    }
    

    pub fn isNight_returnsTrue_for_hours_greater_than_18(&mut self) {
        /* Otherwise - statement - ForEachStmt */
    }
    

    pub fn isNight_returnsFalse_for_boundary_hours_6_and_18(&mut self) {
        /* Otherwise - statement - ForEachStmt */
    }
    

    pub fn getNightOrDay_returns_expected_label(&mut self) {
        let mut nightUtil: Option<Rc<RefCell<TimeTypeUtil>>> = TestableTimeTypeUtil::new(0);
        Assertions::assertEquals(jcall!(nightUtil).NIGHT.clone(), jcall!(nightUtil).getNightOrDay());
        let mut dayUtil: Option<Rc<RefCell<TimeTypeUtil>>> = TestableTimeTypeUtil::new(12);
        Assertions::assertEquals(jcall!(dayUtil).DAY.clone(), jcall!(dayUtil).getNightOrDay());
    }
    

    pub fn getHourOfDay_long_converts_epoch_to_local_hour(&mut self) {
        let mut util: Option<Rc<RefCell<TimeTypeUtil>>> = TimeTypeUtil::new();
        /* Otherwise - statement - ForEachStmt */
    }
    

    pub fn isNight_long_expected_ranges(&mut self) {
        let mut util: Option<Rc<RefCell<TimeTypeUtil>>> = TimeTypeUtil::new();
        /* Otherwise - statement - ForEachStmt */
        /* Otherwise - statement - ForEachStmt */
    }
    

    pub fn getNightOrDay_long_consistency(&mut self) {
        let mut util: Option<Rc<RefCell<TimeTypeUtil>>> = TimeTypeUtil::new();
        let mut nightMillis: i64 = TimeTypeTest::millisForHourLocal(0);
        let mut dayMillis: i64 = TimeTypeTest::millisForHourLocal(12);
        Assertions::assertEquals(jcall!(util).NIGHT.clone(), jcall!(util).getNightOrDay(nightMillis.clone()));
        Assertions::assertEquals(jcall!(util).DAY.clone(), jcall!(util).getNightOrDay(dayMillis.clone()));
    }
    

    pub fn boundaries_day_for_long_based_api(&mut self) {
        let mut util: Option<Rc<RefCell<TimeTypeUtil>>> = TimeTypeUtil::new();
        /* Otherwise - statement - ForEachStmt */
    }
    

    pub fn constants_accessible_and_stable(&mut self) {
        let mut util: Option<Rc<RefCell<TimeTypeUtil>>> = TimeTypeUtil::new();
        Assertions::assertEquals(String::from("Night"), jcall!(util).NIGHT.clone());
        Assertions::assertEquals(String::from("Day"), jcall!(util).DAY.clone());
    }
    

    pub fn getInstance_returns_same_singleton(&mut self) {
        let mut a: Option<Rc<RefCell<TimeTypeUtil>>> = TimeTypeUtil::getInstance();
        let mut b: Option<Rc<RefCell<TimeTypeUtil>>> = TimeTypeUtil::getInstance();
        Assertions::assertSame(a.clone(), b.clone());
    }
}

pub struct TestableTimeTypeUtil {
    pub base: TimeTypeUtil,
    pub hour: i32,
}


impl TestableTimeTypeUtil {
        

        pub fn new_struct(mut hour: i32) -> Self {
            let mut this = Self {
            base: TimeTypeUtil::new_struct(),
            hour: 0,
            };
            this.hour = hour.clone();
            this
        }

        pub fn new(mut hour: i32) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(hour))))
        }
        

        pub fn getHourOfDay(&mut self) -> i32 {
            return self.hour.clone();
        }
}
impl std::ops::Deref for TestableTimeTypeUtil {
    type Target = TimeTypeUtil;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TestableTimeTypeUtil {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



