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
// package org.allbinary.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::AndroidGameStatistics::AndroidGameStatistics;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//ActualPlatform
pub struct GameStatisticsFactory {
    pub base: AndroidGameStatistics,
    pub stringBuffer: Option<Rc<RefCell<StringMaker>>>,
}

thread_local! { pub static GameStatisticsFactory_SINGLETON: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatisticsDyn>>> = GameStatisticsFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatisticsDyn>>); }

impl GameStatisticsFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AndroidGameStatistics::new_struct(),
            stringBuffer: None,
        };
        this.stringBuffer = StringMaker::new();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatisticsDyn>>> {
        return GameStatisticsFactory_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn add(&mut self, mut string: String) {
        if jcall!(self.stringBuffer).length() > 12000 {
            { let java_receiver = self.stringBuffer.clone(); let java_arguments = (0, jcall!(self.stringBuffer).length(),); jcall!(java_receiver).delete(java_arguments.0, java_arguments.1); };
            jcall!(self.stringBuffer).append(String::from("Old Stats Cleared"));
        }
        jcall!(self.stringBuffer).append(string.clone());
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(self.base.toString()).append(jcall!(self.stringBuffer).toString()).toString();
    }
}
impl std::ops::Deref for GameStatisticsFactory {
    type Target = AndroidGameStatistics;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameStatisticsFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatisticsDyn for GameStatisticsFactory {
    fn as_AndroidGameStatistics(&self) -> &crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatistics { &self.base }
    fn as_AndroidGameStatistics_mut(&mut self) -> &mut crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatistics { &mut self.base }
}
impl crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatisticsDyn for GameStatisticsFactory {
    fn as_BaseGameStatistics(&self) -> &crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatistics { &self.base.base }
    fn as_BaseGameStatistics_mut(&mut self) -> &mut crate::org::allbinary::canvas::BaseGameStatistics::BaseGameStatistics { &mut self.base.base }
    fn add(&mut self, mut string: String) { Self::add(self, string) }
    fn process(&mut self) { crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatistics::process(crate::org::allbinary::canvas::AndroidGameStatistics::AndroidGameStatisticsDyn::as_AndroidGameStatistics_mut(self)) }
    fn toString(&mut self) -> String { Self::toString(self) }
}


