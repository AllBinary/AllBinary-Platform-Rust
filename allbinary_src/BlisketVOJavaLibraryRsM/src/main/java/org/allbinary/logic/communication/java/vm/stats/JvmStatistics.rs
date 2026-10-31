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
use crate::java::lang::Thread::*;
use crate::java::lang::Runtime::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct JvmStatistics {
    pub runtime: Option<Rc<RefCell<Runtime>>>,
}


impl JvmStatistics {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            runtime: None,
        };
        this.runtime = Runtime::getRuntime();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getFreeMemoryBytes(&mut self) -> i64 {
        return jcall!(self.runtime).freeMemory();
    }
    

    pub fn getFreeMemoryKilobytes(&mut self) -> i64 {
        return self.getFreeMemoryBytes() / 1024_i64;
    }
    

    pub fn getFreeMemoryMegabytes(&mut self) -> i64 {
        return self.getFreeMemoryKilobytes() / 1024_i64;
    }
    

    pub fn getMaxMemoryBytes(&mut self) -> i64 {
        return jcall!(self.runtime).maxMemory();
    }
    

    pub fn getMaxMemoryKilobytes(&mut self) -> i64 {
        return self.getMaxMemoryBytes() / 1024_i64;
    }
    

    pub fn getMaxMemoryMegabytes(&mut self) -> i64 {
        return self.getMaxMemoryKilobytes() / 1024_i64;
    }
    

    pub fn getTotalMemoryBytes(&mut self) -> i64 {
        return jcall!(self.runtime).totalMemory();
    }
    

    pub fn getTotalMemoryKilobytes(&mut self) -> i64 {
        return self.getTotalMemoryBytes() / 1024_i64;
    }
    

    pub fn getTotalMemoryMegabytes(&mut self) -> i64 {
        return self.getTotalMemoryKilobytes() / 1024_i64;
    }
    

    pub fn getFreeMemoryBytesLong(&mut self) -> Option<i64> {
        return Some((self.getFreeMemoryBytes()) as i64);
    }
    

    pub fn getFreeMemoryKilobytesLong(&mut self) -> Option<i64> {
        return Some((self.getFreeMemoryKilobytes()) as i64);
    }
    

    pub fn getFreeMemoryMegabytesLong(&mut self) -> Option<i64> {
        return Some((self.getFreeMemoryMegabytes()) as i64);
    }
    

    pub fn getMaxMemoryBytesLong(&mut self) -> Option<i64> {
        return Some((self.getMaxMemoryBytes()) as i64);
    }
    

    pub fn getMaxMemoryKilobytesLong(&mut self) -> Option<i64> {
        return Some((self.getMaxMemoryKilobytes()) as i64);
    }
    

    pub fn getMaxMemoryMegabytesLong(&mut self) -> Option<i64> {
        return Some((self.getMaxMemoryMegabytes()) as i64);
    }
    

    pub fn getTotalMemoryBytesLong(&mut self) -> Option<i64> {
        return Some((self.getTotalMemoryBytes()) as i64);
    }
    

    pub fn getTotalMemoryKilobytesLong(&mut self) -> Option<i64> {
        return Some((self.getTotalMemoryKilobytes()) as i64);
    }
    

    pub fn getTotalMemoryMegabytesLong(&mut self) -> Option<i64> {
        return Some((self.getTotalMemoryMegabytes()) as i64);
    }
    

    pub fn getFreeMemoryBytesString(&mut self) -> String {
        return jcall!(self.getFreeMemoryBytesLong()).toString();
    }
    

    pub fn getFreeMemoryKilobytesString(&mut self) -> String {
        return jcall!(self.getFreeMemoryKilobytesLong()).toString();
    }
    

    pub fn getFreeMemoryMegabytesString(&mut self) -> String {
        return jcall!(self.getFreeMemoryMegabytesLong()).toString();
    }
    

    pub fn getMaxMemoryBytesString(&mut self) -> String {
        return jcall!(self.getMaxMemoryBytesLong()).toString();
    }
    

    pub fn getMaxMemoryKilobytesString(&mut self) -> String {
        return jcall!(self.getMaxMemoryKilobytesLong()).toString();
    }
    

    pub fn getMaxMemoryMegabytesString(&mut self) -> String {
        return jcall!(self.getMaxMemoryMegabytesLong()).toString();
    }
    

    pub fn getTotalMemoryBytesString(&mut self) -> String {
        return jcall!(self.getTotalMemoryBytesLong()).toString();
    }
    

    pub fn getTotalMemoryKilobytesString(&mut self) -> String {
        return jcall!(self.getTotalMemoryKilobytesLong()).toString();
    }
    

    pub fn getTotalMemoryMegabytesString(&mut self) -> String {
        return jcall!(self.getTotalMemoryMegabytesLong()).toString();
    }
    

    pub fn getNumberOfThreads(&mut self) -> i64 {
        return Thread::activeCount();
    }
    

    pub fn getNumberOfThreadsLong(&mut self) -> Option<i64> {
        return Some((self.getNumberOfThreads()) as i64);
    }
    

    pub fn getNumberOfThreadsString(&mut self) -> String {
        return jcall!(self.getNumberOfThreadsLong()).toString();
    }
    

    pub fn collectGarbage(&mut self) {
        jcall!(self.runtime).gc();
    }
}
pub trait JvmStatisticsDyn {
    fn as_JvmStatistics(&self) -> &JvmStatistics;
    fn as_JvmStatistics_mut(&mut self) -> &mut JvmStatistics;
    fn getFreeMemoryBytes(&mut self) -> i64 { JvmStatistics::getFreeMemoryBytes(self.as_JvmStatistics_mut()) }
    fn getFreeMemoryKilobytes(&mut self) -> i64 { JvmStatistics::getFreeMemoryKilobytes(self.as_JvmStatistics_mut()) }
    fn getFreeMemoryMegabytes(&mut self) -> i64 { JvmStatistics::getFreeMemoryMegabytes(self.as_JvmStatistics_mut()) }
    fn getMaxMemoryBytes(&mut self) -> i64 { JvmStatistics::getMaxMemoryBytes(self.as_JvmStatistics_mut()) }
    fn getMaxMemoryKilobytes(&mut self) -> i64 { JvmStatistics::getMaxMemoryKilobytes(self.as_JvmStatistics_mut()) }
    fn getMaxMemoryMegabytes(&mut self) -> i64 { JvmStatistics::getMaxMemoryMegabytes(self.as_JvmStatistics_mut()) }
    fn getTotalMemoryBytes(&mut self) -> i64 { JvmStatistics::getTotalMemoryBytes(self.as_JvmStatistics_mut()) }
    fn getTotalMemoryKilobytes(&mut self) -> i64 { JvmStatistics::getTotalMemoryKilobytes(self.as_JvmStatistics_mut()) }
    fn getTotalMemoryMegabytes(&mut self) -> i64 { JvmStatistics::getTotalMemoryMegabytes(self.as_JvmStatistics_mut()) }
    fn getFreeMemoryBytesLong(&mut self) -> Option<i64> { JvmStatistics::getFreeMemoryBytesLong(self.as_JvmStatistics_mut()) }
    fn getFreeMemoryKilobytesLong(&mut self) -> Option<i64> { JvmStatistics::getFreeMemoryKilobytesLong(self.as_JvmStatistics_mut()) }
    fn getFreeMemoryMegabytesLong(&mut self) -> Option<i64> { JvmStatistics::getFreeMemoryMegabytesLong(self.as_JvmStatistics_mut()) }
    fn getMaxMemoryBytesLong(&mut self) -> Option<i64> { JvmStatistics::getMaxMemoryBytesLong(self.as_JvmStatistics_mut()) }
    fn getMaxMemoryKilobytesLong(&mut self) -> Option<i64> { JvmStatistics::getMaxMemoryKilobytesLong(self.as_JvmStatistics_mut()) }
    fn getMaxMemoryMegabytesLong(&mut self) -> Option<i64> { JvmStatistics::getMaxMemoryMegabytesLong(self.as_JvmStatistics_mut()) }
    fn getTotalMemoryBytesLong(&mut self) -> Option<i64> { JvmStatistics::getTotalMemoryBytesLong(self.as_JvmStatistics_mut()) }
    fn getTotalMemoryKilobytesLong(&mut self) -> Option<i64> { JvmStatistics::getTotalMemoryKilobytesLong(self.as_JvmStatistics_mut()) }
    fn getTotalMemoryMegabytesLong(&mut self) -> Option<i64> { JvmStatistics::getTotalMemoryMegabytesLong(self.as_JvmStatistics_mut()) }
    fn getFreeMemoryBytesString(&mut self) -> String { JvmStatistics::getFreeMemoryBytesString(self.as_JvmStatistics_mut()) }
    fn getFreeMemoryKilobytesString(&mut self) -> String { JvmStatistics::getFreeMemoryKilobytesString(self.as_JvmStatistics_mut()) }
    fn getFreeMemoryMegabytesString(&mut self) -> String { JvmStatistics::getFreeMemoryMegabytesString(self.as_JvmStatistics_mut()) }
    fn getMaxMemoryBytesString(&mut self) -> String { JvmStatistics::getMaxMemoryBytesString(self.as_JvmStatistics_mut()) }
    fn getMaxMemoryKilobytesString(&mut self) -> String { JvmStatistics::getMaxMemoryKilobytesString(self.as_JvmStatistics_mut()) }
    fn getMaxMemoryMegabytesString(&mut self) -> String { JvmStatistics::getMaxMemoryMegabytesString(self.as_JvmStatistics_mut()) }
    fn getTotalMemoryBytesString(&mut self) -> String { JvmStatistics::getTotalMemoryBytesString(self.as_JvmStatistics_mut()) }
    fn getTotalMemoryKilobytesString(&mut self) -> String { JvmStatistics::getTotalMemoryKilobytesString(self.as_JvmStatistics_mut()) }
    fn getTotalMemoryMegabytesString(&mut self) -> String { JvmStatistics::getTotalMemoryMegabytesString(self.as_JvmStatistics_mut()) }
    fn getNumberOfThreads(&mut self) -> i64 { JvmStatistics::getNumberOfThreads(self.as_JvmStatistics_mut()) }
    fn getNumberOfThreadsLong(&mut self) -> Option<i64> { JvmStatistics::getNumberOfThreadsLong(self.as_JvmStatistics_mut()) }
    fn getNumberOfThreadsString(&mut self) -> String { JvmStatistics::getNumberOfThreadsString(self.as_JvmStatistics_mut()) }
    fn collectGarbage(&mut self) { JvmStatistics::collectGarbage(self.as_JvmStatistics_mut()) }
}
impl JvmStatisticsDyn for JvmStatistics {
    fn as_JvmStatistics(&self) -> &JvmStatistics { self }
    fn as_JvmStatistics_mut(&mut self) -> &mut JvmStatistics { self }
}


