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
// package org.allbinary.input.automation.actions.script.condition.processors.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Iterator::Iterator;
// use crate::java::util::Set::Set;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessor;
use crate::org::allbinary::input::automation::robot::InputRobotFactory::InputRobotFactory;
use crate::org::allbinary::input::automation::robot::InputRobotInterface::InputRobotInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ProfileActionScriptInputInterface::ProfileActionScriptInputInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicProfileActionScriptInput {
    pub base: BasicProfileActionScriptProcessor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub inputRobotInterface: Option<Rc<RefCell<dyn InputRobotInterface>>>,
    pub time: i32,
}

// impl ProfileActionScriptInputInterface for BasicProfileActionScriptInput with the methods of the trait
impl crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface for BasicProfileActionScriptInput {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl BasicProfileActionScriptInput {
    

    pub fn new_struct2(mut label: String, mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptProcessor::new_struct2(label.clone(), node.clone()),
            logUtil: None,
            inputRobotInterface: None,
            time: 0,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(format!("{}{}", jcall!(this.commonStrings).START, label), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        let mut actionNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(unsafe { GenericProfileActionScriptInputData_TYPE.clone() }.clone(), jcall!(node).getChildNodes());
        let mut inputTypeString: String = DomNodeHelper::getTextNodeValue(actionNode.clone());
        let mut timeNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNodeNoThrow(unsafe { GenericProfileActionScriptInputData_DELAY.clone() }.clone(), jcall!(node).getChildNodes());
        if timeNode.is_some() {
            let mut delayString: String = DomNodeHelper::getTextNodeValue(timeNode.clone());
            { let java_arguments = ((((delayString.clone()).to_string()).unwrap() as i32),); this.setTime(java_arguments.0) };
        } else {
            this.setTime(0);
        }
        { let java_arguments = (jcall!(InputRobotFactory::getInstance()).get1(inputTypeString.clone()),); this.setInputRobotInterface(java_arguments.0) };
        this
    }

    pub fn new2(mut label: String, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(label, node))))
    }
    

    pub fn new_struct1(mut label: String) -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptProcessor::new_struct1(label.clone()),
            logUtil: None,
            inputRobotInterface: None,
            time: 0,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(format!("{}{}", jcall!(this.commonStrings).START, label), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        let mut inputRobotFactory: Option<Rc<RefCell<InputRobotFactory>>> = InputRobotFactory::getInstance();
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = (jcall!(inputRobotFactory).get()).clone() /* as ABHashtable */;
        let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(hashtable).keySet();
        let mut iterator: Option<Rc<RefCell<dyn Iterator>>> = jcall!(set).iterator();
        { let java_arguments = (jcall!(inputRobotFactory).get1((jcall!(iterator).next()).clone() /* as String */),); this.setInputRobotInterface(java_arguments.0) };
        this.setTime(0);
        this
    }

    pub fn new1(mut label: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(label))))
    }
    

    pub fn getTime(&mut self) -> i32 {
        return self.time.clone();
    }
    

    pub fn setTime(&mut self, mut time: i32) {
        self.time = time.clone();
    }
    

    pub fn BasicProfileActionScriptInput_toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(unsafe { GenericProfileActionScriptInputData_TYPE.clone() }.clone(), jcall!(self.getInputRobotInterface()).getName());
        jcall!(hashMap).put(unsafe { GenericProfileActionScriptInputData_DELAY.clone() }.clone(), (self.getTime()).to_string());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNodeWithValueNodes(document.clone(), unsafe { GenericProfileActionScriptInputData_NAME.clone() }.clone(), self.BasicProfileActionScriptInput_toHashMap());
        return node.clone();
    }
    

    pub fn setInputRobotInterface(&mut self, mut inputRobotInterface: Option<Rc<RefCell<dyn InputRobotInterface>>>) {
        self.inputRobotInterface = inputRobotInterface.clone();
    }
    

    pub fn getInputRobotInterface(&mut self) -> Option<Rc<RefCell<dyn InputRobotInterface>>> {
        return self.inputRobotInterface.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.base.toString());
        jcall!(stringBuffer).append(String::from(" Input Type: "));
        if self.getInputRobotInterface().is_some() {
            jcall!(stringBuffer).append(jcall!(self.getInputRobotInterface()).getName());
        } else {
            jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).NULL_STRING.clone());
        }
        jcall!(stringBuffer).append(String::from(" Time: "));
        jcall!(stringBuffer).appendlong(((self.getTime()) as i64));
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for BasicProfileActionScriptInput {
    type Target = BasicProfileActionScriptProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicProfileActionScriptInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasicProfileActionScriptInputDyn: crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessorDyn {
    fn as_BasicProfileActionScriptInput(&self) -> &BasicProfileActionScriptInput;
    fn as_BasicProfileActionScriptInput_mut(&mut self) -> &mut BasicProfileActionScriptInput;
    fn getTime(&mut self) -> i32 { BasicProfileActionScriptInput::getTime(self.as_BasicProfileActionScriptInput_mut()) }
    fn setTime(&mut self, mut time: i32) { BasicProfileActionScriptInput::setTime(self.as_BasicProfileActionScriptInput_mut(), time) }
    fn BasicProfileActionScriptInput_toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { BasicProfileActionScriptInput::BasicProfileActionScriptInput_toHashMap(self.as_BasicProfileActionScriptInput_mut()) }
    fn setInputRobotInterface(&mut self, mut inputRobotInterface: Option<Rc<RefCell<dyn InputRobotInterface>>>) { BasicProfileActionScriptInput::setInputRobotInterface(self.as_BasicProfileActionScriptInput_mut(), inputRobotInterface) }
    fn getInputRobotInterface(&mut self) -> Option<Rc<RefCell<dyn InputRobotInterface>>> { BasicProfileActionScriptInput::getInputRobotInterface(self.as_BasicProfileActionScriptInput_mut()) }
    fn toString(&mut self) -> String { BasicProfileActionScriptInput::toString(self.as_BasicProfileActionScriptInput_mut()) }
}
impl BasicProfileActionScriptInputDyn for BasicProfileActionScriptInput {
    fn as_BasicProfileActionScriptInput(&self) -> &BasicProfileActionScriptInput { self }
    fn as_BasicProfileActionScriptInput_mut(&mut self) -> &mut BasicProfileActionScriptInput { self }
}

impl crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessorDyn for BasicProfileActionScriptInput {
    fn as_BasicProfileActionScriptProcessor(&self) -> &crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessor { &self.base }
    fn as_BasicProfileActionScriptProcessor_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessor { &mut self.base }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItemDyn for BasicProfileActionScriptInput {
    fn as_ProfileActionScriptItem(&self) -> &crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &self.base.base }
    fn as_ProfileActionScriptItem_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &mut self.base.base }
}


