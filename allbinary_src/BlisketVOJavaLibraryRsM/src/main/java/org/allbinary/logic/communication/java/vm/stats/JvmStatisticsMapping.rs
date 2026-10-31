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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use super::JvmStatistics::JvmStatistics;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct JvmStatisticsMapping {
    pub base: JvmStatistics,
}

impl TableMappingInterface for JvmStatisticsMapping {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}

impl JvmStatisticsMapping {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JvmStatistics::new_struct(),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(JvmStatisticsData_FREE_MEMORY_BYTES.with(Clone::clone).clone(), self.getFreeMemoryBytesString());
        jcall!(hashMap).put(JvmStatisticsData_FREE_MEMORY_KILOBYTES.with(Clone::clone).clone(), self.getFreeMemoryKilobytesString());
        jcall!(hashMap).put(JvmStatisticsData_FREE_MEMORY_MEGABYTES.with(Clone::clone).clone(), self.getFreeMemoryMegabytesString());
        jcall!(hashMap).put(JvmStatisticsData_MAX_MEMORY_BYTES.with(Clone::clone).clone(), self.getMaxMemoryBytesString());
        jcall!(hashMap).put(JvmStatisticsData_MAX_MEMORY_KILOBYTES.with(Clone::clone).clone(), self.getMaxMemoryKilobytesString());
        jcall!(hashMap).put(JvmStatisticsData_MAX_MEMORY_MEGABYTES.with(Clone::clone).clone(), self.getMaxMemoryMegabytesString());
        jcall!(hashMap).put(JvmStatisticsData_NUMBER_OF_THREADS.with(Clone::clone).clone(), self.getNumberOfThreadsString());
        jcall!(hashMap).put(JvmStatisticsData_TOTAL_MEMORY_BYTES.with(Clone::clone).clone(), self.getTotalMemoryBytesString());
        jcall!(hashMap).put(JvmStatisticsData_TOTAL_MEMORY_KILOBYTES.with(Clone::clone).clone(), self.getTotalMemoryKilobytesString());
        jcall!(hashMap).put(JvmStatisticsData_TOTAL_MEMORY_MEGABYTES.with(Clone::clone).clone(), self.getTotalMemoryMegabytesString());
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return JvmStatisticsData_NAME.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(vector).add(crate::java_into_object(self.getFreeMemoryBytesString()));
        jcall!(vector).add(crate::java_into_object(self.getFreeMemoryKilobytesString()));
        jcall!(vector).add(crate::java_into_object(self.getFreeMemoryMegabytesString()));
        jcall!(vector).add(crate::java_into_object(self.getMaxMemoryBytesString()));
        jcall!(vector).add(crate::java_into_object(self.getMaxMemoryKilobytesString()));
        jcall!(vector).add(crate::java_into_object(self.getMaxMemoryMegabytesString()));
        jcall!(vector).add(crate::java_into_object(self.getNumberOfThreadsString()));
        jcall!(vector).add(crate::java_into_object(self.getTotalMemoryBytesString()));
        jcall!(vector).add(crate::java_into_object(self.getTotalMemoryKilobytesString()));
        jcall!(vector).add(crate::java_into_object(self.getTotalMemoryMegabytesString()));
        return vector.clone();
    }
}
impl std::ops::Deref for JvmStatisticsMapping {
    type Target = JvmStatistics;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for JvmStatisticsMapping {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait JvmStatisticsMappingDyn: crate::org::allbinary::logic::communication::java::vm::stats::JvmStatistics::JvmStatisticsDyn {
    fn as_JvmStatisticsMapping(&self) -> &JvmStatisticsMapping;
    fn as_JvmStatisticsMapping_mut(&mut self) -> &mut JvmStatisticsMapping;
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { JvmStatisticsMapping::toHashMap(self.as_JvmStatisticsMapping_mut()) }
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { JvmStatisticsMapping::getKey(self.as_JvmStatisticsMapping_mut()) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { JvmStatisticsMapping::toVector(self.as_JvmStatisticsMapping_mut()) }
}
impl JvmStatisticsMappingDyn for JvmStatisticsMapping {
    fn as_JvmStatisticsMapping(&self) -> &JvmStatisticsMapping { self }
    fn as_JvmStatisticsMapping_mut(&mut self) -> &mut JvmStatisticsMapping { self }
}

impl crate::org::allbinary::logic::communication::java::vm::stats::JvmStatistics::JvmStatisticsDyn for JvmStatisticsMapping {
    fn as_JvmStatistics(&self) -> &crate::org::allbinary::logic::communication::java::vm::stats::JvmStatistics::JvmStatistics { &self.base }
    fn as_JvmStatistics_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::java::vm::stats::JvmStatistics::JvmStatistics { &mut self.base }
}


