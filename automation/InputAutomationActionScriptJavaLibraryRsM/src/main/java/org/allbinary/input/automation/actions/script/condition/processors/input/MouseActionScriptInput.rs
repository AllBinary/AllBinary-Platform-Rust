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
// use crate::java::awt::*;
// use crate::java::awt::event::InputEvent::InputEvent;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MouseActionScriptInput {
    pub base: BasicProfileActionScriptInput,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub point: Option<Rc<RefCell<Point>>>,
    pub buttons: i32,
    pub mouseActionScriptInputJPanel: Option<Rc<RefCell<MouseActionScriptInputJPanel>>>,
}

thread_local! { pub static MouseActionScriptInput_NAME: String = String::from("Mouse"); }
// impl MouseActionScriptInputInterface for MouseActionScriptInput with the methods of the trait
impl crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface for MouseActionScriptInput {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::input::automation::actions::script::condition::processors::ProfileActionScriptProcessorInterface::ProfileActionScriptProcessorInterface for MouseActionScriptInput {
}
impl crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItemInterface::ProfileActionScriptItemInterface for MouseActionScriptInput {
    fn process(&mut self, mut frame: Option<i64>) { Self::process(self, frame) }
}

impl MouseActionScriptInput {
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptInput::new_struct2(MouseActionScriptInput_NAME.with(Clone::clone).clone(), node.clone()),
            logUtil: None,
            point: None,
            buttons: 0,
            mouseActionScriptInputJPanel: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.point = Point::new();
        let mut actionNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(unsafe { MouseActionScriptInputData_NAME.clone() }.clone(), jcall!(node).getChildNodes());
        if actionNode.is_some() {
            let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(actionNode).getChildNodes();
            for index in (0..jcall!(nodeList).getLength()) {
                let mut childNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
                if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { MouseActionScriptInputData_BUTTONS.clone() }.clone()) == 0 {
                    let mut buttons: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    { let java_arguments = ((((buttons.clone()).to_string()).unwrap() as i32),); this.setButtonClicks(java_arguments.0) };
                } else if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { MouseActionScriptInputData_MOVE.clone() }.clone()) == 0 {
                    let mut mouseXNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(unsafe { MouseActionScriptInputData_MOVE_X.clone() }.clone(), jcall!(childNode).getChildNodes());
                    let mut mouseXString: String = DomNodeHelper::getTextNodeValue(mouseXNode.clone());
                    let mut mouseXInteger: Option<i32> = (mouseXString.clone()).to_string();
                    let mut mouseYNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(unsafe { MouseActionScriptInputData_MOVE_Y.clone() }.clone(), jcall!(childNode).getChildNodes());
                    let mut mouseYString: String = DomNodeHelper::getTextNodeValue(mouseYNode.clone());
                    let mut mouseYInteger: Option<i32> = (mouseYString.clone()).to_string();
                    let mut newPoint: Option<Rc<RefCell<Point>>> = Point::new(((mouseXInteger).unwrap() as i32), ((mouseYInteger).unwrap() as i32));
                    this.setPoint(newPoint.clone());
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Action Script Input Unknown Node"))));
                }
            }
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Action Script Input Node Null"))));
        }
        this.setAllowsChildren(false);
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct1(node))));
            this.mouseActionScriptInputJPanel = MouseActionScriptInputJPanel::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptInput::new_struct1(MouseActionScriptInput_NAME.with(Clone::clone).clone()),
            logUtil: None,
            point: None,
            buttons: 0,
            mouseActionScriptInputJPanel: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.point = Point::new();
        this.setAllowsChildren(false);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct())));
            this.mouseActionScriptInputJPanel = MouseActionScriptInputJPanel::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn setPoint(&mut self, mut point: Option<Rc<RefCell<Point>>>) {
        self.point = point.clone();
    }
    

    pub fn getPoint(&mut self) -> Option<Rc<RefCell<Point>>> {
        return self.point.clone();
    }
    

    pub fn getButtonClicks(&mut self) -> i32 {
        return self.buttons.clone();
    }
    

    pub fn setButtonClicks(&mut self, mut buttons: i32) {
        self.buttons = buttons.clone();
    }
    

    pub fn showDialog(&mut self) {
        jcall!(jcall!(self.mouseActionScriptInputJPanel).getMouseActionJDialog()).setVisible(true);
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(unsafe { MouseActionScriptInputData_BUTTONS.clone() }.clone(), (self.getButtonClicks()).to_string());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = self.base.toXmlNode(document.clone());
        let mut mouseNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createNodeWithValueNodes(document.clone(), unsafe { MouseActionScriptInputData_NAME.clone() }.clone(), self.toHashMap());
        jcall!(node).appendChild(mouseNode.clone());
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(unsafe { MouseActionScriptInputData_MOVE_X.clone() }.clone(), (jcall!(self.getPoint()).x.clone()).to_string());
        jcall!(hashMap).put(unsafe { MouseActionScriptInputData_MOVE_Y.clone() }.clone(), (jcall!(self.getPoint()).y.clone()).to_string());
        jcall!(mouseNode).appendChild(ModDomHelper::createNodeWithValueNodes(document.clone(), unsafe { MouseActionScriptInputData_MOVE.clone() }.clone(), hashMap.clone()));
        return node.clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut frame: Option<i64>) {
        MouseInputAutomationProcessor::MouseInputAutomationProcessor::process(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn log(&mut self) {
        let mut buttonStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(buttonStringBuffer).append(String::from("1: "));
        let mut booleanFactory: Option<Rc<RefCell<BooleanFactory>>> = BooleanFactory::getInstance();
        if (self.getButtonClicks() & unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON1_MASK.clone() }) != 0 {
            jcall!(buttonStringBuffer).append(jcall!(booleanFactory).TRUE_STRING.clone());
        } else {
            jcall!(buttonStringBuffer).append(jcall!(booleanFactory).FALSE_STRING.clone());
        }
        jcall!(buttonStringBuffer).append(String::from(" 2: "));
        if (self.getButtonClicks() & unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON2_MASK.clone() }) != 0 {
            jcall!(buttonStringBuffer).append(jcall!(booleanFactory).TRUE_STRING.clone());
        } else {
            jcall!(buttonStringBuffer).append(jcall!(booleanFactory).FALSE_STRING.clone());
        }
        jcall!(buttonStringBuffer).append(String::from(" 3: "));
        if (self.getButtonClicks() & unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON3_MASK.clone() }) != 0 {
            jcall!(buttonStringBuffer).append(jcall!(booleanFactory).TRUE_STRING.clone());
        } else {
            jcall!(buttonStringBuffer).append(jcall!(booleanFactory).FALSE_STRING.clone());
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Input Type: "), jcall!(self.getInputRobotInterface()).getName()), String::from(" Point: ")), self.getPoint()), String::from(" Buttons Clicked: ")), jcall!(buttonStringBuffer).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("log"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
}
impl std::ops::Deref for MouseActionScriptInput {
    type Target = BasicProfileActionScriptInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MouseActionScriptInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::actions::script::condition::processors::input::BasicProfileActionScriptInput::BasicProfileActionScriptInputDyn for MouseActionScriptInput {
    fn as_BasicProfileActionScriptInput(&self) -> &crate::org::allbinary::input::automation::actions::script::condition::processors::input::BasicProfileActionScriptInput::BasicProfileActionScriptInput { &self.base }
    fn as_BasicProfileActionScriptInput_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::condition::processors::input::BasicProfileActionScriptInput::BasicProfileActionScriptInput { &mut self.base }
}
impl crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessorDyn for MouseActionScriptInput {
    fn as_BasicProfileActionScriptProcessor(&self) -> &crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessor { &self.base.base }
    fn as_BasicProfileActionScriptProcessor_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessor { &mut self.base.base }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItemDyn for MouseActionScriptInput {
    fn as_ProfileActionScriptItem(&self) -> &crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &self.base.base.base }
    fn as_ProfileActionScriptItem_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &mut self.base.base.base }
    fn process(&mut self, mut frame: Option<i64>) { Self::process(self, frame) }
    fn showDialog(&mut self) { Self::showDialog(self) }
}


