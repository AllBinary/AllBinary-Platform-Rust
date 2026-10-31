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
// package org.allbinary.input.automation.actions.script.condition
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::BasicProfileActionScriptCondition::BasicProfileActionScriptCondition;
use super::TimeIntervalActionScriptConditionInterface::TimeIntervalActionScriptConditionInterface;
use super::TimeIntervalActionScriptConditionJPanel::TimeIntervalActionScriptConditionJPanel;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TimeIntervalActionScriptCondition {
    pub base: BasicProfileActionScriptCondition,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub timeIntervalActionScriptConditionJPanel: Option<Rc<RefCell<TimeIntervalActionScriptConditionJPanel>>>,
    pub timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
}

thread_local! { pub static TimeIntervalActionScriptCondition_NAME: String = String::from("Time Interval"); }
impl TimeIntervalActionScriptConditionInterface for TimeIntervalActionScriptCondition {
    fn getTimeDelayHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> { Self::getTimeDelayHelper(self) }
    fn log(&mut self) { Self::log(self) }
}

impl TimeIntervalActionScriptCondition {
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptCondition::new_struct2(TimeIntervalActionScriptCondition_NAME.with(Clone::clone).clone(), node.clone()),
            logUtil: None,
            timeIntervalActionScriptConditionJPanel: None,
            timeHelper: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        let mut actionNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(unsafe { TimeIntervalActionScriptConditionData_NAME.clone() }.clone(), jcall!(node).getChildNodes());
        if actionNode.is_some() {
            let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(actionNode).getChildNodes();
            for index in (0..jcall!(nodeList).getLength()) {
                let mut childNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
                if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { TimeIntervalActionScriptConditionData_TIME.clone() }.clone()) == 0 {
                    let mut interval: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    { let java_arguments = (TimeDelayHelper::new((interval.clone()).to_string()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>),); this.setTimeDelayHelper(java_arguments.0) };
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Time Interval Action Script Condition Unknown Node"))));
                }
            }
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Time Interval Action Script Condition Node Null"))));
        }
        this.init();
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptCondition::new_struct1(TimeIntervalActionScriptCondition_NAME.with(Clone::clone).clone()),
            logUtil: None,
            timeIntervalActionScriptConditionJPanel: None,
            timeHelper: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.setTimeDelayHelper(TimeDelayHelper::new(0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>));
        this.init();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        self.timeIntervalActionScriptConditionJPanel = TimeIntervalActionScriptConditionJPanel::new(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn showDialog(&mut self) {
        jcall!(jcall!(self.timeIntervalActionScriptConditionJPanel).getTimeIntervalActionJDialog()).setVisible(true);
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(unsafe { TimeIntervalActionScriptConditionData_TIME.clone() }.clone(), (jcall!(self.timeHelper).as_TimeDelayHelper_mut().delay.clone()).to_string());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = self.base.toXmlNode(document.clone());
        jcall!(node).appendChild(ModDomHelper::createNodeWithValueNodes(document.clone(), unsafe { TimeIntervalActionScriptConditionData_NAME.clone() }.clone(), self.toHashMap()));
        return node.clone();
    }
    

    pub fn log(&mut self) {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Time Interval: "), jcall!(self.timeHelper).as_TimeDelayHelper_mut().delay), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("log"));
    }
    

    pub fn getTimeDelayHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> {
        return self.timeHelper.clone();
    }
    

    pub fn setTimeDelayHelper(&mut self, mut timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>) {
        self.timeHelper = timeHelper.clone();
    }
    

    // throws Exception
    pub fn shouldProcess(&mut self, mut frame: Option<i64>) -> bool {
        let mut timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> = self.getTimeDelayHelper();
        if jcall!(timeHelper).isTimeTNT() {
            jcall!(timeHelper).setStartTimeTNT();
            return self.base.shouldProcess(frame.clone());
        }
        return false;
    }
}
impl std::ops::Deref for TimeIntervalActionScriptCondition {
    type Target = BasicProfileActionScriptCondition;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TimeIntervalActionScriptCondition {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::actions::script::condition::BasicProfileActionScriptCondition::BasicProfileActionScriptConditionDyn for TimeIntervalActionScriptCondition {
    fn as_BasicProfileActionScriptCondition(&self) -> &crate::org::allbinary::input::automation::actions::script::condition::BasicProfileActionScriptCondition::BasicProfileActionScriptCondition { &self.base }
    fn as_BasicProfileActionScriptCondition_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::condition::BasicProfileActionScriptCondition::BasicProfileActionScriptCondition { &mut self.base }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
    fn shouldProcess(&mut self, mut frame: Option<i64>) -> bool { Self::shouldProcess(self, frame) }
}
impl crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItemDyn for TimeIntervalActionScriptCondition {
    fn as_ProfileActionScriptItem(&self) -> &crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &self.base.base }
    fn as_ProfileActionScriptItem_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &mut self.base.base }
    fn init(&mut self) { Self::init(self) }
    fn showDialog(&mut self) { Self::showDialog(self) }
}


