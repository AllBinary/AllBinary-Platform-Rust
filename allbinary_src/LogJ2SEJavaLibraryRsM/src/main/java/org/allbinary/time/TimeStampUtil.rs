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
use crate::Calendar;
use crate::Date;
use super::*;
// use crate::java::text::SimpleDateFormat::SimpleDateFormat;
// use crate::java::util::Calendar::Calendar;
// use crate::java::util::Date::Date;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//ActualPlatform
pub struct TimeStampUtil {
    pub simpleDataFormat: String,
}

thread_local! { pub static TimeStampUtil_instance: Option<Rc<RefCell<TimeStampUtil>>> = TimeStampUtil::new(); }

impl TimeStampUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<TimeStampUtil>>> {
        return TimeStampUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            simpleDataFormat: String::new(),
        };
        this.simpleDataFormat = String::from("yyyy-MM-dd HH:mm:ss.SSS");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getAsString(&mut self) -> String {
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut date: Option<Rc<RefCell<Date>>> = jcall!(calendar).getTime();
        return { let java_millis = jcall!(date.clone()).0.duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as i64).unwrap_or(0); let java_days = java_millis.div_euclid(86_400_000); let java_time = java_millis.rem_euclid(86_400_000); let java_z = java_days + 719_468; let java_era = java_z.div_euclid(146_097); let java_doe = java_z - java_era * 146_097; let java_yoe = (java_doe - java_doe / 1460 + java_doe / 36_524 - java_doe / 146_096) / 365; let java_doy = java_doe - (365 * java_yoe + java_yoe / 4 - java_yoe / 100); let java_mp = (5 * java_doy + 2) / 153; let java_day = java_doy - (153 * java_mp + 2) / 5 + 1; let java_month = if java_mp < 10 { java_mp + 3 } else { java_mp - 9 }; let java_year = java_yoe + java_era * 400 + if java_month <= 2 { 1 } else { 0 }; (self.simpleDataFormat).replace("yyyy", &format!("{:04}", java_year)).replace("MM", &format!("{:02}", java_month)).replace("dd", &format!("{:02}", java_day)).replace("HH", &format!("{:02}", java_time / 3_600_000)).replace("mm", &format!("{:02}", java_time / 60_000 % 60)).replace("ss", &format!("{:02}", java_time / 1000 % 60)).replace("SSS", &format!("{:03}", java_time % 1000)) };
    }
}

