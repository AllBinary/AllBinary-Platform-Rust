// Generated Code Do Not Modify
// package org.allbinary.time
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Calendar;
use super::*;
// use crate::java::util::Calendar::Calendar;
// use crate::java::util::TimeZone::TimeZone;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TimeTypeUtil {
    pub NIGHT: String,
    pub DAY: String,
}

thread_local! { pub static TimeTypeUtil_instance: Option<Rc<RefCell<TimeTypeUtil>>> = TimeTypeUtil::new(); }

impl TimeTypeUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NIGHT: String::new(),
            DAY: String::new(),
        };
        this.NIGHT = String::from("Night");
        this.DAY = String::from("Day");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<TimeTypeUtil>>> {
        return TimeTypeUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn isNight(&mut self) -> bool {
        let mut hoursOfDay: i32 = self.getHourOfDay();
        return hoursOfDay < 6 || hoursOfDay > 18;
    }
    

    pub fn getHourOfDay(&mut self) -> i32 {
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        return jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_HOUR_OF_DAY.clone() }.clone());
    }
    

    pub fn getNightOrDay(&mut self) -> String {
        return (if self.isNight() { self.NIGHT } else { self.DAY });
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getHourOfDay1(&mut self, mut timeInMillis: i64) -> i32 {
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        jcall!(calendar).setTimeInMillis(timeInMillis.clone());
        return jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_HOUR_OF_DAY.clone() }.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn isNight1(&mut self, mut timeInMillis: i64) -> bool {
        let mut hoursOfDay: i32 = self.getHourOfDay1(timeInMillis.clone());
        return hoursOfDay < 6 || hoursOfDay > 18;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getNightOrDay1(&mut self, mut timeInMillis: i64) -> String {
        return (if self.isNight1(timeInMillis.clone()) { self.NIGHT } else { self.DAY });
    }
}

