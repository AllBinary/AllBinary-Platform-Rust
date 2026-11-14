//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
// 
// By agreeing to this license you and any business entity you represent are
// legally bound to the AllBinary Open License Version 1 legal agreement.
// 
// You may obtain the AllBinary Open License Version 1 legal agreement from
// AllBinary or the root directory of AllBinary's AllBinary Platform repository.
// 
// Created By: Travis Berthelot
//  
// Generated Code Do Not Modify
// package org.allbinary.time
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::Calendar::Calendar;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TimeUtil {
}

thread_local! { pub static TimeUtil_instance: Option<Rc<RefCell<TimeUtil>>> = TimeUtil::new(); }
thread_local! { pub static TimeUtil_calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance(); }

impl TimeUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<TimeUtil>>> {
        return TimeUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn setCalendar(&mut self, mut calendar: Option<Rc<RefCell<Calendar>>>, mut yearString: String, mut monthString: String, mut dayString: String, mut hourString: String) {
        let mut year: i32 = ((Some((yearString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        let mut month: i32 = ((Some((monthString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        let mut day: i32 = ((Some((dayString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        let mut hour: i32 = ((Some((hourString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        jcall!(calendar).set5(year.clone(), month.clone(), day.clone(), hour.clone(), 0);
    }
    

    pub fn setCalendarWithTime(&mut self, mut calendar: Option<Rc<RefCell<Calendar>>>, mut yearString: String, mut monthString: String, mut dayString: String, mut hourString: String, mut minuteString: String, mut secondString: String) {
        let mut year: i32 = ((Some((yearString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        let mut month: i32 = ((Some((monthString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        let mut day: i32 = ((Some((dayString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        let mut hour: i32 = ((Some((hourString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        let mut minute: i32 = ((Some((minuteString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        let mut second: i32 = ((Some((secondString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        jcall!(calendar).set6(year.clone(), month.clone(), day.clone(), hour.clone(), minute.clone(), second.clone());
    }
    

    pub fn setExpirationCalendar(&mut self, mut calendar: Option<Rc<RefCell<Calendar>>>, mut yearsString: String, mut monthsString: String, mut daysString: String, mut hoursString: String, mut minutesString: String, mut secondsString: String) {
        let mut year: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_YEAR.clone() }.clone()) + ((Some((yearsString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        let mut month: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_MONTH.clone() }.clone()) + ((Some((monthsString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        let mut day: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_DAY_OF_MONTH.clone() }.clone()) + ((Some((daysString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        let mut hour: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_HOUR.clone() }.clone()) + ((Some((hoursString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        let mut minute: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_MINUTE.clone() }.clone()) + ((Some((minutesString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        let mut second: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_SECOND.clone() }.clone()) + ((Some((secondsString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32);
        jcall!(calendar).set6(year.clone(), month.clone(), day.clone(), hour.clone(), minute.clone(), second.clone());
    }
    

    pub fn getTotalTime(&mut self, mut yearString: String, mut monthString: String, mut dayString: String, mut hourString: String, mut minuteString: String, mut secondString: String) -> i64 {
        let mut time: i64 = jcall!(TimeUtil_calendar.with(Clone::clone)).getTimeInMillis();
        { let java_arguments = (TimeUtil_calendar.with(Clone::clone).clone(), yearString.clone(), monthString.clone(), dayString.clone(), hourString.clone(), minuteString.clone(), secondString.clone(),); self.setExpirationCalendar(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5, java_arguments.6) };
        let mut timeDelta: i64 = jcall!(TimeUtil_calendar.with(Clone::clone)).getTimeInMillis();
        return timeDelta - time;
    }
}

