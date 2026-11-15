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
// package org.allbinary.logic.communication.java.vm.stats
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct JvmStatisticsData {
}

pub static JvmStatisticsData_JVM_STATISTICS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("JVM_STATISTICS_"));
pub static JvmStatisticsData_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| format!("{}{}", JvmStatisticsData_JVM_STATISTICS.with(Clone::clone), String::from("NAME")));
pub static JvmStatisticsData_TOTAL_MEMORY_BYTES: crate::JavaStatic<String> = crate::JavaStatic::new(|| format!("{}{}", JvmStatisticsData_JVM_STATISTICS.with(Clone::clone), String::from("TOTAL_MEMORY_BYTES")));
pub static JvmStatisticsData_TOTAL_MEMORY_KILOBYTES: crate::JavaStatic<String> = crate::JavaStatic::new(|| format!("{}{}", JvmStatisticsData_JVM_STATISTICS.with(Clone::clone), String::from("TOTAL_MEMORY_KILOBYTES")));
pub static JvmStatisticsData_TOTAL_MEMORY_MEGABYTES: crate::JavaStatic<String> = crate::JavaStatic::new(|| format!("{}{}", JvmStatisticsData_JVM_STATISTICS.with(Clone::clone), String::from("TOTAL_MEMORY_MEGABYTES")));
pub static JvmStatisticsData_FREE_MEMORY_BYTES: crate::JavaStatic<String> = crate::JavaStatic::new(|| format!("{}{}", JvmStatisticsData_JVM_STATISTICS.with(Clone::clone), String::from("FREE_MEMORY_BYTES")));
pub static JvmStatisticsData_FREE_MEMORY_KILOBYTES: crate::JavaStatic<String> = crate::JavaStatic::new(|| format!("{}{}", JvmStatisticsData_JVM_STATISTICS.with(Clone::clone), String::from("FREE_MEMORY_KILOBYTES")));
pub static JvmStatisticsData_FREE_MEMORY_MEGABYTES: crate::JavaStatic<String> = crate::JavaStatic::new(|| format!("{}{}", JvmStatisticsData_JVM_STATISTICS.with(Clone::clone), String::from("FREE_MEMORY_MEGABYTES")));
pub static JvmStatisticsData_MAX_MEMORY_BYTES: crate::JavaStatic<String> = crate::JavaStatic::new(|| format!("{}{}", JvmStatisticsData_JVM_STATISTICS.with(Clone::clone), String::from("MAX_MEMORY_BYTES")));
pub static JvmStatisticsData_MAX_MEMORY_KILOBYTES: crate::JavaStatic<String> = crate::JavaStatic::new(|| format!("{}{}", JvmStatisticsData_JVM_STATISTICS.with(Clone::clone), String::from("MAX_MEMORY_KILOBYTES")));
pub static JvmStatisticsData_MAX_MEMORY_MEGABYTES: crate::JavaStatic<String> = crate::JavaStatic::new(|| format!("{}{}", JvmStatisticsData_JVM_STATISTICS.with(Clone::clone), String::from("MAX_MEMORY_MEGABYTES")));
pub static JvmStatisticsData_NUMBER_OF_THREADS: crate::JavaStatic<String> = crate::JavaStatic::new(|| format!("{}{}", JvmStatisticsData_JVM_STATISTICS.with(Clone::clone), String::from("NUMBER_OF_THREADS")));

impl JvmStatisticsData {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

