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
use super::*;
use crate::java::util::Calendar::Calendar;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::data::generator::ProductIdGenerator::ProductIdGenerator;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::time::TimeUtil::TimeUtil;
use crate::java::lang::Exception::*;

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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
        let mut EMPTY_STRING: String = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        this.id = { let java_value = (jcall!(ProductIdGenerator::new()).getNext()).clone(); java_value };
        this.basicItemId = id.clone();
        this.enabled = { let java_value = ({ let java_field = jcall!(EntryData::getInstance()).YES.clone(); java_field }).clone(); java_value }.clone();
        this.system = String::from("Java");
        this.platform = String::from("Android");
        this.specialName = EMPTY_STRING.clone();
        this.version = EMPTY_STRING.clone();
        this.changes = EMPTY_STRING.clone();
        this.licenseFile = EMPTY_STRING.clone();
        this.file = EMPTY_STRING.clone();
        this.size = { let java_value = (Some((0) as i64)).clone(); java_value };
        this.validTime = { let java_value = (Some((0) as i64)).clone(); java_value };
        this.retries = EMPTY_STRING.clone();
        this
    }

    pub fn new1(mut id: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(id))))
    }
    

    pub fn new_struct1_HashMap(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
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
        this.id = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_ID.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.basicItemId = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(BasicItemData::getInstance()).ID.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.enabled = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).ENABLE.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.system = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_SYSTEM.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.platform = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_PLATFORM.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.specialName = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_SPECIAL_NAME.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.version = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_VERSION.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.changes = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_CHANGES.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.licenseFile = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_LICENSE_FILE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.file = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_FILE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        let mut value: String = { let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_SIZE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(value.clone()) {
            this.size = { let java_value = (Some((value.clone()).trim().parse::<i64>().expect("NumberFormatException"))).clone(); java_value };
        } else {
            this.size = Some((0) as i64);
        }
        let mut validTime: String = { let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_VALID_TIME.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(validTime.clone()) {
            this.validTime = { let java_value = (Some((validTime.clone()).trim().parse::<i64>().expect("NumberFormatException"))).clone(); java_value };
        } else {
            let mut totalYears: String = { let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_VALID_TIME_YEARS.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut totalMonths: String = { let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_VALID_TIME_MONTHS.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut totalDays: String = { let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_VALID_TIME_DAYS.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut totalHours: String = { let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_VALID_TIME_HOURS.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut totalMinutes: String = { let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_VALID_TIME_MINUTES.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut totalSeconds: String = { let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_VALID_TIME_SECONDS.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut totalTime: i64 = jcall!(TimeUtil::getInstance()).getTotalTime(totalYears.clone(), totalMonths.clone(), totalDays.clone(), totalHours.clone(), totalMinutes.clone(), totalSeconds.clone());
            this.validTime = Some((totalTime.clone()) as i64);
        }
        this.retries = { let java_value = ({ let java_object = (jcall!(hashMap).get(crate::java_into_object(DownloadItemData_RETRIES.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this
    }

    pub fn new1_HashMap(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_HashMap(hashMap))))
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = (Some((jcall!(calendar).getTimeInMillis()) as i64)).expect("NullPointerException").to_string();
        let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
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
        jcall!(values).add({ let java_argument = crate::java_into_object((self.getSize()).expect("NullPointerException").to_string()); java_argument });
        jcall!(values).add({ let java_argument = crate::java_into_object((self.getValidTime()).expect("NullPointerException").to_string()); java_argument });
        jcall!(values).add(crate::java_into_object(self.getRetries()));
        jcall!(values).add(crate::java_into_object(time.clone()));
        jcall!(values).add(crate::java_into_object(time.clone()));
        return values.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut values: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(values).put(crate::java_into_object(DownloadItemData_ID.with(Clone::clone).clone()), crate::java_into_object(self.getId()));
        jcall!(values).put(crate::java_into_object({ let java_field = jcall!(BasicItemData::getInstance()).ID.clone(); java_field }.clone()), crate::java_into_object(self.basicItemId.clone()));
        jcall!(values).put(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).ENABLE.clone(); java_field }.clone()), crate::java_into_object(self.enabled.clone()));
        jcall!(values).put(crate::java_into_object(DownloadItemData_SYSTEM.with(Clone::clone).clone()), crate::java_into_object(self.getSystem()));
        jcall!(values).put(crate::java_into_object(DownloadItemData_PLATFORM.with(Clone::clone).clone()), crate::java_into_object(self.getPlatform()));
        jcall!(values).put(crate::java_into_object(DownloadItemData_SPECIAL_NAME.with(Clone::clone).clone()), crate::java_into_object(self.getSpecialName()));
        jcall!(values).put(crate::java_into_object(DownloadItemData_VERSION.with(Clone::clone).clone()), crate::java_into_object(self.getVersion()));
        jcall!(values).put(crate::java_into_object(DownloadItemData_CHANGES.with(Clone::clone).clone()), crate::java_into_object(self.getChanges()));
        jcall!(values).put(crate::java_into_object(DownloadItemData_LICENSE_FILE.with(Clone::clone).clone()), crate::java_into_object(self.getLicenseFile()));
        jcall!(values).put(crate::java_into_object(DownloadItemData_FILE.with(Clone::clone).clone()), crate::java_into_object(self.getFile()));
        jcall!(values).put(crate::java_into_object(DownloadItemData_SIZE.with(Clone::clone).clone()), { let java_argument = crate::java_into_object((self.getSize()).expect("NullPointerException").to_string()); java_argument });
        jcall!(values).put(crate::java_into_object(DownloadItemData_VALID_TIME.with(Clone::clone).clone()), { let java_argument = crate::java_into_object((self.getValidTime()).expect("NullPointerException").to_string()); java_argument });
        jcall!(values).put(crate::java_into_object(DownloadItemData_RETRIES.with(Clone::clone).clone()), crate::java_into_object(self.getRetries()));
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = (Some((jcall!(calendar).getTimeInMillis()) as i64)).expect("NullPointerException").to_string().clone();
        jcall!(values).put(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).LASTMODIFIED.clone(); java_field }.clone()), crate::java_into_object(time.clone()));
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

