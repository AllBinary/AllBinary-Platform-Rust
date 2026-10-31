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
// package org.allbinary.business.user.commerce.inventory.item.download
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Calendar;
use std::collections::HashMap;
use super::*;
// use crate::java::util::Calendar::Calendar;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::data::generator::ProductIdGenerator::ProductIdGenerator;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::time::TimeUtil::TimeUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DownloadableItem {
    pub id: String,
    pub basicItemId: String,
    pub enabled: String,
    pub system: String,
    pub platform: String,
    pub specialName: String,
    pub version: String,
    pub changes: String,
    pub licenseFile: String,
    pub file: String,
    pub size: Option<i64>,
    pub validTime: Option<i64>,
    pub retries: String,
}


impl DownloadableItem {
    

    pub fn new_struct1(mut id: String) -> Self {
        let mut this = Self {
            id: String::new(),
            basicItemId: String::new(),
            enabled: String::new(),
            system: String::new(),
            platform: String::new(),
            specialName: String::new(),
            version: String::new(),
            changes: String::new(),
            licenseFile: String::new(),
            file: String::new(),
            size: None,
            validTime: None,
            retries: String::new(),
        };
        let mut EMPTY_STRING: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.id = jcall!(ProductIdGenerator::new()).getNext();
        this.basicItemId = id.clone();
        this.enabled = jcall!(EntryData::getInstance()).YES.clone();
        this.system = String::from("Java");
        this.platform = String::from("Android");
        this.specialName = EMPTY_STRING.clone();
        this.version = EMPTY_STRING.clone();
        this.changes = EMPTY_STRING.clone();
        this.licenseFile = EMPTY_STRING.clone();
        this.file = EMPTY_STRING.clone();
        this.size = (0).to_string();
        this.validTime = (0).to_string();
        this.retries = EMPTY_STRING.clone();
        this
    }

    pub fn new1(mut id: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(id))))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            id: String::new(),
            basicItemId: String::new(),
            enabled: String::new(),
            system: String::new(),
            platform: String::new(),
            specialName: String::new(),
            version: String::new(),
            changes: String::new(),
            licenseFile: String::new(),
            file: String::new(),
            size: None,
            validTime: None,
            retries: String::new(),
        };
        this.id = (jcall!(hashMap).get(DownloadItemData_ID.with(Clone::clone).clone())).clone() /* as String */;
        this.basicItemId = (jcall!(hashMap).get(jcall!(BasicItemData::getInstance()).ID.clone())).clone() /* as String */;
        this.enabled = (jcall!(hashMap).get(jcall!(EntryData::getInstance()).ENABLE.clone())).clone() /* as String */;
        this.system = (jcall!(hashMap).get(DownloadItemData_SYSTEM.with(Clone::clone).clone())).clone() /* as String */;
        this.platform = (jcall!(hashMap).get(DownloadItemData_PLATFORM.with(Clone::clone).clone())).clone() /* as String */;
        this.specialName = (jcall!(hashMap).get(DownloadItemData_SPECIAL_NAME.with(Clone::clone).clone())).clone() /* as String */;
        this.version = (jcall!(hashMap).get(DownloadItemData_VERSION.with(Clone::clone).clone())).clone() /* as String */;
        this.changes = (jcall!(hashMap).get(DownloadItemData_CHANGES.with(Clone::clone).clone())).clone() /* as String */;
        this.licenseFile = (jcall!(hashMap).get(DownloadItemData_LICENSE_FILE.with(Clone::clone).clone())).clone() /* as String */;
        this.file = (jcall!(hashMap).get(DownloadItemData_FILE.with(Clone::clone).clone())).clone() /* as String */;
        let mut value: String = (jcall!(hashMap).get(DownloadItemData_SIZE.with(Clone::clone).clone())).clone() /* as String */;
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(value.clone()) {
            this.size = (value.clone()).to_string();
        } else {
            this.size = Some((0) as i64);
        }
        let mut validTime: String = (jcall!(hashMap).get(DownloadItemData_VALID_TIME.with(Clone::clone).clone())).clone() /* as String */;
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(validTime.clone()) {
            this.validTime = (validTime.clone()).to_string();
        } else {
            let mut totalYears: String = (jcall!(hashMap).get(DownloadItemData_VALID_TIME_YEARS.with(Clone::clone).clone())).clone() /* as String */;
            let mut totalMonths: String = (jcall!(hashMap).get(DownloadItemData_VALID_TIME_MONTHS.with(Clone::clone).clone())).clone() /* as String */;
            let mut totalDays: String = (jcall!(hashMap).get(DownloadItemData_VALID_TIME_DAYS.with(Clone::clone).clone())).clone() /* as String */;
            let mut totalHours: String = (jcall!(hashMap).get(DownloadItemData_VALID_TIME_HOURS.with(Clone::clone).clone())).clone() /* as String */;
            let mut totalMinutes: String = (jcall!(hashMap).get(DownloadItemData_VALID_TIME_MINUTES.with(Clone::clone).clone())).clone() /* as String */;
            let mut totalSeconds: String = (jcall!(hashMap).get(DownloadItemData_VALID_TIME_SECONDS.with(Clone::clone).clone())).clone() /* as String */;
            let mut totalTime: i64 = jcall!(TimeUtil::getInstance()).getTotalTime(totalYears.clone(), totalMonths.clone(), totalDays.clone(), totalHours.clone(), totalMinutes.clone(), totalSeconds.clone());
            this.validTime = Some((totalTime.clone()) as i64);
        }
        this.retries = (jcall!(hashMap).get(DownloadItemData_RETRIES.with(Clone::clone).clone())).clone() /* as String */;
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString();
        let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(values).add(crate::java_into_object(self.getId()));
        jcall!(values).add(crate::java_into_object(self.basicItemId.clone()));
        jcall!(values).add(crate::java_into_object(self.enabled.clone()));
        jcall!(values).add(crate::java_into_object(self.getSpecialName()));
        jcall!(values).add(crate::java_into_object(self.getVersion()));
        jcall!(values).add(crate::java_into_object(self.getChanges()));
        jcall!(values).add(crate::java_into_object(self.getSystem()));
        jcall!(values).add(crate::java_into_object(self.getPlatform()));
        jcall!(values).add(crate::java_into_object(self.getLicenseFile()));
        jcall!(values).add(crate::java_into_object(self.getFile()));
        jcall!(values).add(crate::java_into_object(jcall!(self.getSize()).toString()));
        jcall!(values).add(crate::java_into_object(jcall!(self.getValidTime()).toString()));
        jcall!(values).add(crate::java_into_object(self.getRetries()));
        jcall!(values).add(crate::java_into_object(time.clone()));
        jcall!(values).add(crate::java_into_object(time.clone()));
        return values.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut values: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(values).as_BasicArrayList_mut().put(DownloadItemData_ID.with(Clone::clone).clone(), self.getId());
        jcall!(values).as_BasicArrayList_mut().put(jcall!(BasicItemData::getInstance()).ID.clone(), self.basicItemId.clone());
        jcall!(values).as_BasicArrayList_mut().put(jcall!(EntryData::getInstance()).ENABLE.clone(), self.enabled.clone());
        jcall!(values).as_BasicArrayList_mut().put(DownloadItemData_SYSTEM.with(Clone::clone).clone(), self.getSystem());
        jcall!(values).as_BasicArrayList_mut().put(DownloadItemData_PLATFORM.with(Clone::clone).clone(), self.getPlatform());
        jcall!(values).as_BasicArrayList_mut().put(DownloadItemData_SPECIAL_NAME.with(Clone::clone).clone(), self.getSpecialName());
        jcall!(values).as_BasicArrayList_mut().put(DownloadItemData_VERSION.with(Clone::clone).clone(), self.getVersion());
        jcall!(values).as_BasicArrayList_mut().put(DownloadItemData_CHANGES.with(Clone::clone).clone(), self.getChanges());
        jcall!(values).as_BasicArrayList_mut().put(DownloadItemData_LICENSE_FILE.with(Clone::clone).clone(), self.getLicenseFile());
        jcall!(values).as_BasicArrayList_mut().put(DownloadItemData_FILE.with(Clone::clone).clone(), self.getFile());
        jcall!(values).as_BasicArrayList_mut().put(DownloadItemData_SIZE.with(Clone::clone).clone(), jcall!(self.getSize()).toString());
        jcall!(values).as_BasicArrayList_mut().put(DownloadItemData_VALID_TIME.with(Clone::clone).clone(), jcall!(self.getValidTime()).toString());
        jcall!(values).as_BasicArrayList_mut().put(DownloadItemData_RETRIES.with(Clone::clone).clone(), self.getRetries());
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
        jcall!(values).as_BasicArrayList_mut().put(jcall!(EntryData::getInstance()).LASTMODIFIED.clone(), time.clone());
        return values.clone();
    }
    

    pub fn getId(&mut self) -> String {
        return self.id.clone();
    }
    

    pub fn getSystem(&mut self) -> String {
        return self.system.clone();
    }
    

    pub fn getPlatform(&mut self) -> String {
        return self.platform.clone();
    }
    

    pub fn getSpecialName(&mut self) -> String {
        return self.specialName.clone();
    }
    

    pub fn getVersion(&mut self) -> String {
        return self.version.clone();
    }
    

    pub fn getChanges(&mut self) -> String {
        return self.changes.clone();
    }
    

    pub fn getLicenseFile(&mut self) -> String {
        return self.licenseFile.clone();
    }
    

    pub fn getFile(&mut self) -> String {
        return self.file.clone();
    }
    

    pub fn getSize(&mut self) -> Option<i64> {
        return self.size.clone();
    }
    

    pub fn getValidTime(&mut self) -> Option<i64> {
        return self.validTime.clone();
    }
    

    pub fn getRetries(&mut self) -> String {
        return self.retries.clone();
    }
}

