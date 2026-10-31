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
// use crate::java::util::Set::Set;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::DownloadableItem::DownloadableItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DownloadableItemView {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub downloadableItem: Option<Rc<RefCell<DownloadableItem>>>,
}

impl DomNodeInterface for DownloadableItemView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl DownloadableItemView {
    

    pub fn new_struct(mut downloadableItem: Option<Rc<RefCell<DownloadableItem>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            downloadableItem: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.downloadableItem = downloadableItem.clone();
        this
    }

    pub fn new(mut downloadableItem: Option<Rc<RefCell<DownloadableItem>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(downloadableItem))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRODUCTSEARCHLOGGING.clone()) {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"));
        }
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(self.downloadableItem).toHashMap();
        let mut totalTime: i64 = ((jcall!(self.downloadableItem).getValidTime()).unwrap() as i64);
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut year: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_YEAR.clone() }.clone());
        let mut month: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_MONTH.clone() }.clone());
        let mut day: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_DAY_OF_MONTH.clone() }.clone());
        let mut hour: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_HOUR.clone() }.clone());
        let mut minute: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_MINUTE.clone() }.clone());
        let mut second: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_SECOND.clone() }.clone());
        jcall!(calendar).setTimeInMillis(jcall!(calendar).getTimeInMillis() + totalTime);
        let mut yearDelta: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_YEAR.clone() }.clone());
        let mut monthDelta: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_MONTH.clone() }.clone());
        let mut dayDelta: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_DAY_OF_MONTH.clone() }.clone());
        let mut hourDelta: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_HOUR.clone() }.clone());
        let mut minuteDelta: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_MINUTE.clone() }.clone());
        let mut secondDelta: i32 = jcall!(calendar).get(unsafe { crate::java::util::Calendar::Calendar_SECOND.clone() }.clone());
        jcall!(hashMap).put(DownloadItemData_VALID_TIME_YEARS.with(Clone::clone).clone(), (yearDelta - year).to_string());
        jcall!(hashMap).put(DownloadItemData_VALID_TIME_MONTHS.with(Clone::clone).clone(), (monthDelta - month).to_string());
        jcall!(hashMap).put(DownloadItemData_VALID_TIME_DAYS.with(Clone::clone).clone(), (dayDelta - day).to_string());
        jcall!(hashMap).put(DownloadItemData_VALID_TIME_HOURS.with(Clone::clone).clone(), (hourDelta - hour).to_string());
        jcall!(hashMap).put(DownloadItemData_VALID_TIME_MINUTES.with(Clone::clone).clone(), (minuteDelta - minute).to_string());
        jcall!(hashMap).put(DownloadItemData_VALID_TIME_SECONDS.with(Clone::clone).clone(), (secondDelta - second).to_string());
        let mut keySet: Option<Rc<RefCell<dyn Set>>> = jcall!(hashMap).keySet();
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(DownloadItemData_NAME.with(Clone::clone).clone());
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut nameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keySet).toArray();
        let mut size: i32 = (nameArray.len() as i32).clone();
        for index in (0..size) {
            let mut name: String = (nameArray[(index) as usize]).clone() /* as String */;
            let mut value: String = (jcall!(hashMap).get(name.clone())).clone() /* as String */;
            value = jcall!(stringUtil).getNonNull(value.clone());
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), name.clone(), value.clone()));
        }
        return node.clone();
    }
    

    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn view(&mut self) -> String {
        return None;
    }
}

