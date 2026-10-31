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
// package org.allbinary.business.user.modules.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::time::created::TimeCreated::TimeCreated;
use crate::org::allbinary::business::time::modified::TimeLastModified::TimeLastModified;
use crate::org::allbinary::logic::communication::smtp::configuration::user::UserEmailConfiguration::UserEmailConfiguration;
use crate::org::allbinary::logic::communication::smtp::configuration::user::UserEmailConfigurationInterface::UserEmailConfigurationInterface;
use super::UserConfigurationInterface::UserConfigurationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserConfiguration {
    pub userEmailConfigurationInterface: Option<Rc<RefCell<dyn UserEmailConfigurationInterface>>>,
    pub timeCreated: Option<Rc<RefCell<TimeCreated>>>,
    pub timeLastModified: Option<Rc<RefCell<TimeLastModified>>>,
}

impl UserConfigurationInterface for UserConfiguration {
    fn getUserEmailConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn UserEmailConfigurationInterface>>> { Self::getUserEmailConfigurationInterface(self) }
    fn setUserEmailConfigurationInterface(&mut self, mut userEmailConfigurationInterface: Option<Rc<RefCell<dyn UserEmailConfigurationInterface>>>) { Self::setUserEmailConfigurationInterface(self, userEmailConfigurationInterface) }
    fn getTimeCreated(&mut self) -> Option<Rc<RefCell<TimeCreated>>> { Self::getTimeCreated(self) }
    fn getTimeLastModified(&mut self) -> Option<Rc<RefCell<TimeLastModified>>> { Self::getTimeLastModified(self) }
    fn setTimeCreated(&mut self, mut timeCreated: Option<Rc<RefCell<TimeCreated>>>) { Self::setTimeCreated(self, timeCreated) }
    fn setTimeLastModified(&mut self, mut timeLastModified: Option<Rc<RefCell<TimeLastModified>>>) { Self::setTimeLastModified(self, timeLastModified) }
}

impl UserConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            userEmailConfigurationInterface: None,
            timeCreated: None,
            timeLastModified: None,
        };
        this.userEmailConfigurationInterface = (UserEmailConfiguration::new()).clone() /* as UserEmailConfigurationInterface */;
        this.timeCreated = TimeCreated::new1(0);
        this.timeLastModified = TimeLastModified::new1(0);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            userEmailConfigurationInterface: None,
            timeCreated: None,
            timeLastModified: None,
        };
        this.userEmailConfigurationInterface = (UserEmailConfiguration::new1(hashMap.clone())).clone() /* as UserEmailConfigurationInterface */;
        this.timeCreated = TimeCreated::new1((jcall!(hashMap).get(jcall!(EntryData::getInstance()).TIMECREATED.clone())).clone() /* as String */);
        this.timeLastModified = TimeLastModified::new1((jcall!(hashMap).get(jcall!(EntryData::getInstance()).LASTMODIFIED.clone())).clone() /* as String */);
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn getUserEmailConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn UserEmailConfigurationInterface>>> {
        return self.userEmailConfigurationInterface.clone();
    }
    

    pub fn setUserEmailConfigurationInterface(&mut self, mut userEmailConfigurationInterface: Option<Rc<RefCell<dyn UserEmailConfigurationInterface>>>) {
        self.userEmailConfigurationInterface = userEmailConfigurationInterface.clone();
    }
    

    pub fn getTimeCreated(&mut self) -> Option<Rc<RefCell<TimeCreated>>> {
        return self.timeCreated.clone();
    }
    

    pub fn getTimeLastModified(&mut self) -> Option<Rc<RefCell<TimeLastModified>>> {
        return self.timeLastModified.clone();
    }
    

    pub fn setTimeCreated(&mut self, mut timeCreated: Option<Rc<RefCell<TimeCreated>>>) {
        self.timeCreated = timeCreated.clone();
    }
    

    pub fn setTimeLastModified(&mut self, mut timeLastModified: Option<Rc<RefCell<TimeLastModified>>>) {
        self.timeLastModified = timeLastModified.clone();
    }
}

