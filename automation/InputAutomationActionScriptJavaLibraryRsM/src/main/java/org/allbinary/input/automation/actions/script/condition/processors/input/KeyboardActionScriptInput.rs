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
// use crate::java::awt::event::KeyEvent::KeyEvent;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::input::KeySingletonFactory::KeySingletonFactory;
use crate::org::allbinary::input::KeyUtil::KeyUtil;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::BasicProfileActionScriptInput::BasicProfileActionScriptInput;
use super::KeyboardActionScriptInputInterface::KeyboardActionScriptInputInterface;
use super::KeyboardActionScriptInputJPanel::KeyboardActionScriptInputJPanel;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct KeyboardActionScriptInput {
    pub base: BasicProfileActionScriptInput,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub keyArray: Vec<Option<i32>>,
    pub keyboardActionScriptInputJPanel: Option<Rc<RefCell<KeyboardActionScriptInputJPanel>>>,
    pub press: bool,
    pub release: bool,
    pub delayBetweenKeys: i32,
}

thread_local! { pub static KeyboardActionScriptInput_NAME: String = String::from("Keyboard"); }
// impl KeyboardActionScriptInputInterface for KeyboardActionScriptInput with the methods of the trait
impl crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface for KeyboardActionScriptInput {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::input::automation::actions::script::condition::processors::ProfileActionScriptProcessorInterface::ProfileActionScriptProcessorInterface for KeyboardActionScriptInput {
}
impl crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItemInterface::ProfileActionScriptItemInterface for KeyboardActionScriptInput {
    fn process(&mut self, mut frame: Option<i64>) { Self::process(self, frame) }
}

impl KeyboardActionScriptInput {
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptInput::new_struct2(KeyboardActionScriptInput_NAME.with(Clone::clone).clone(), node.clone()),
            logUtil: None,
            keyArray: Vec::new(),
            keyboardActionScriptInputJPanel: None,
            press: false,
            release: false,
            delayBetweenKeys: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.keyArray = vec![KeySingletonFactory::getInstance(unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_0.clone() }.clone())];
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        let mut actionNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(unsafe { KeyboardActionScriptInputData_NAME.clone() }.clone(), jcall!(node).getChildNodes());
        if actionNode.is_some() {
            let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(actionNode).getChildNodes();
            let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            for index in (0..jcall!(nodeList).getLength()) {
                let mut childNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
                if jcall!(jcall!(childNode).getNodeName()).startsWith(unsafe { KeyboardActionScriptInputData_KEY.clone() }.clone()) {
                    let mut keyString: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    jcall!(vector).add(crate::java_into_object((keyString.clone()).to_string()));
                } else if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { KeyboardActionScriptInputData_DELAY_BETWEEN_KEYS.clone() }.clone()) == 0 {
                    let mut time: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    { let java_arguments = ((((time.clone()).to_string()).unwrap() as i32),); this.setDelayBetweenKeys(java_arguments.0) };
                } else if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { KeyboardActionScriptInputData_PRESS.clone() }.clone()) == 0 {
                    let mut value: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    { let java_arguments = ((value.clone()).to_string(),); this.setPress(java_arguments.0) };
                } else if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { KeyboardActionScriptInputData_RELEASE.clone() }.clone()) == 0 {
                    let mut value: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    { let java_arguments = ((value.clone()).to_string(),); this.setRelease(java_arguments.0) };
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Action Script Input Unknown Node"))));
                }
            }
            { let java_arguments = ((jcall!(vector).toArrayType(vec![None; (jcall!(vector).size()) as usize])).clone() /* as  */,); this.setKeyArray(java_arguments.0) };
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Action Script Input Node Null"))));
        }
        this.setAllowsChildren(false);
        if !this.isPress() && !this.isRelease() {
            this.setNormal();
        }
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct1(node))));
            this.keyboardActionScriptInputJPanel = KeyboardActionScriptInputJPanel::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptInput::new_struct1(KeyboardActionScriptInput_NAME.with(Clone::clone).clone()),
            logUtil: None,
            keyArray: Vec::new(),
            keyboardActionScriptInputJPanel: None,
            press: false,
            release: false,
            delayBetweenKeys: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.keyArray = vec![KeySingletonFactory::getInstance(unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_0.clone() }.clone())];
        // super(...) is in the struct literal above
        if !this.isPress() && !this.isRelease() {
            this.setNormal();
        }
        this.setAllowsChildren(false);
        this.setTime(120);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct())));
            this.keyboardActionScriptInputJPanel = KeyboardActionScriptInputJPanel::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn getDelayBetweenKeys(&mut self) -> i32 {
        return self.delayBetweenKeys.clone();
    }
    

    pub fn setDelayBetweenKeys(&mut self, mut delayBetweenKeys: i32) {
        self.delayBetweenKeys = delayBetweenKeys.clone();
    }
    

    pub fn isPress(&mut self) -> bool {
        return self.press.clone();
    }
    

    pub fn setPress(&mut self, mut press: bool) {
        self.press = press.clone();
    }
    

    pub fn isRelease(&mut self) -> bool {
        return self.release.clone();
    }
    

    pub fn setRelease(&mut self, mut release: bool) {
        self.release = release.clone();
    }
    

    pub fn isNormal(&mut self) -> bool {
        return self.isRelease() && self.isPress();
    }
    

    pub fn setNormal(&mut self) {
        self.setPress(true);
        self.setRelease(true);
    }
    

    pub fn getKeyArray(&mut self) -> Vec<Option<i32>> {
        return self.keyArray.clone();
    }
    

    pub fn setKeyArray(&mut self, mut keyArray: Vec<Option<i32>>) {
        if keyArray.is_some() {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).START, (keyArray.len() as i32)), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setKeyArray"),); let java_result = jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); java_result };
        }
        self.keyArray = keyArray.clone();
    }
    

    pub fn showDialog(&mut self) {
        jcall!(jcall!(self.keyboardActionScriptInputJPanel).getKeyActionJDialog()).setVisible(true);
    }
    

    pub fn getText(&mut self) -> String {
        return self.getText1(self.getKeyArray());
    }
    

    pub fn setText(&mut self, mut text: String) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).START, text), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setText"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut integerArray: Vec<Option<i32>> = self.integerArrayValue(text.clone());
        self.setKeyArray(integerArray.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getText1(mut integerArray: Vec<Option<i32>>) -> String {
        let mut index: i32 = 0;
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        while index < (integerArray.len() as i32) {
            let mut nextInteger: Option<i32> = integerArray[(index) as usize].clone();
            if KeyUtil::isNormallyDisplayed(nextInteger.clone()) {
                jcall!(stringBuffer).append(KeyEvent::getKeyText(nextInteger.clone()));
            } else {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("&#"), nextInteger), String::from(";")));
            }
            index += 1;
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn integerArrayValue(&mut self, mut text: String) -> Vec<Option<i32>> {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).START, text), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("integerArrayValue"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut index: i32 = 0;
        while index < (text.chars().count() as i32) {
            let mut aChar: char = text.chars().nth((index.clone()) as usize).unwrap();
            if aChar == '&' {
                if text.chars().nth((index + 1) as usize).unwrap() == '#' {
                    let mut endIndex: i32 = jcall!(text).indexOf(';', index + 1);
                    if endIndex != -1 {
                        let mut nextCharString: String = text[(index + 2) as usize..(endIndex) as usize].to_string();
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", jcall!(CommonLabels::getInstance()).NEXT, String::from(" Char String: ")), nextCharString), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("integerArrayValue"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                        jcall!(vector).add(crate::java_into_object((nextCharString[(index) as usize..((nextCharString.chars().count() as i32)) as usize].to_string()).to_string()));
                        index = index + (nextCharString.chars().count() as i32) + 3;
                        continue;
                    }
                }
            }
            jcall!(vector).add(crate::java_into_object(jcall!(KeySingletonFactory::getHashtable()).get((aChar.clone()).to_string())));
            index += 1;
        }
        let mut integerArray: Vec<Option<i32>> = (jcall!(vector).toArrayType(vec![None; (jcall!(vector).size()) as usize])).clone() /* as  */;
        return integerArray.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        for index in (0..(self.getKeyArray().len() as i32)) {
            jcall!(hashMap).put(format!("{}{}", unsafe { KeyboardActionScriptInputData_KEY.clone() }, index), (self.getKeyArray()[(index) as usize].clone()).to_string());
        }
        jcall!(hashMap).put(unsafe { KeyboardActionScriptInputData_DELAY_BETWEEN_KEYS.clone() }.clone(), (self.getDelayBetweenKeys()).to_string());
        jcall!(hashMap).put(unsafe { KeyboardActionScriptInputData_PRESS.clone() }.clone(), (self.isPress()).to_string());
        jcall!(hashMap).put(unsafe { KeyboardActionScriptInputData_RELEASE.clone() }.clone(), (self.isRelease()).to_string());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = self.base.toXmlNode(document.clone());
        jcall!(node).appendChild(ModDomHelper::createNodeWithValueNodes(document.clone(), unsafe { KeyboardActionScriptInputData_NAME.clone() }.clone(), self.toHashMap()));
        return node.clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut frame: Option<i64>) {
        KeyboardInputAutomationProcessor::KeyboardInputAutomationProcessor::process(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn log(&mut self) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (self.toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("log"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.base.toString());
        jcall!(stringBuffer).append(String::from(" Text: "));
        jcall!(stringBuffer).append(self.getText());
        jcall!(stringBuffer).append(String::from(" isPress: "));
        jcall!(stringBuffer).appendboolean(self.isPress());
        jcall!(stringBuffer).append(String::from(" isRelease: "));
        jcall!(stringBuffer).appendboolean(self.isRelease());
        jcall!(stringBuffer).append(String::from(" Between Key Delay: "));
        jcall!(stringBuffer).appendint(self.getDelayBetweenKeys());
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for KeyboardActionScriptInput {
    type Target = BasicProfileActionScriptInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for KeyboardActionScriptInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait KeyboardActionScriptInputDyn: crate::org::allbinary::input::automation::actions::script::condition::processors::input::BasicProfileActionScriptInput::BasicProfileActionScriptInputDyn {
    fn as_KeyboardActionScriptInput(&self) -> &KeyboardActionScriptInput;
    fn as_KeyboardActionScriptInput_mut(&mut self) -> &mut KeyboardActionScriptInput;
    fn getDelayBetweenKeys(&mut self) -> i32 { KeyboardActionScriptInput::getDelayBetweenKeys(self.as_KeyboardActionScriptInput_mut()) }
    fn setDelayBetweenKeys(&mut self, mut delayBetweenKeys: i32) { KeyboardActionScriptInput::setDelayBetweenKeys(self.as_KeyboardActionScriptInput_mut(), delayBetweenKeys) }
    fn isPress(&mut self) -> bool { KeyboardActionScriptInput::isPress(self.as_KeyboardActionScriptInput_mut()) }
    fn setPress(&mut self, mut press: bool) { KeyboardActionScriptInput::setPress(self.as_KeyboardActionScriptInput_mut(), press) }
    fn isRelease(&mut self) -> bool { KeyboardActionScriptInput::isRelease(self.as_KeyboardActionScriptInput_mut()) }
    fn setRelease(&mut self, mut release: bool) { KeyboardActionScriptInput::setRelease(self.as_KeyboardActionScriptInput_mut(), release) }
    fn isNormal(&mut self) -> bool { KeyboardActionScriptInput::isNormal(self.as_KeyboardActionScriptInput_mut()) }
    fn setNormal(&mut self) { KeyboardActionScriptInput::setNormal(self.as_KeyboardActionScriptInput_mut()) }
    fn getKeyArray(&mut self) -> Vec<Option<i32>> { KeyboardActionScriptInput::getKeyArray(self.as_KeyboardActionScriptInput_mut()) }
    fn setKeyArray(&mut self, mut keyArray: Vec<Option<i32>>) { KeyboardActionScriptInput::setKeyArray(self.as_KeyboardActionScriptInput_mut(), keyArray) }
    fn getText(&mut self) -> String { KeyboardActionScriptInput::getText(self.as_KeyboardActionScriptInput_mut()) }
    fn setText(&mut self, mut text: String) { KeyboardActionScriptInput::setText(self.as_KeyboardActionScriptInput_mut(), text) }
    fn integerArrayValue(&mut self, mut text: String) -> Vec<Option<i32>> { KeyboardActionScriptInput::integerArrayValue(self.as_KeyboardActionScriptInput_mut(), text) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { KeyboardActionScriptInput::toHashMap(self.as_KeyboardActionScriptInput_mut()) }
    fn log(&mut self) { KeyboardActionScriptInput::log(self.as_KeyboardActionScriptInput_mut()) }
}
impl KeyboardActionScriptInputDyn for KeyboardActionScriptInput {
    fn as_KeyboardActionScriptInput(&self) -> &KeyboardActionScriptInput { self }
    fn as_KeyboardActionScriptInput_mut(&mut self) -> &mut KeyboardActionScriptInput { self }
}

impl crate::org::allbinary::input::automation::actions::script::condition::processors::input::BasicProfileActionScriptInput::BasicProfileActionScriptInputDyn for KeyboardActionScriptInput {
    fn as_BasicProfileActionScriptInput(&self) -> &crate::org::allbinary::input::automation::actions::script::condition::processors::input::BasicProfileActionScriptInput::BasicProfileActionScriptInput { &self.base }
    fn as_BasicProfileActionScriptInput_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::condition::processors::input::BasicProfileActionScriptInput::BasicProfileActionScriptInput { &mut self.base }
    fn toString(&mut self) -> String { Self::toString(self) }
}
impl crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessorDyn for KeyboardActionScriptInput {
    fn as_BasicProfileActionScriptProcessor(&self) -> &crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessor { &self.base.base }
    fn as_BasicProfileActionScriptProcessor_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessor { &mut self.base.base }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItemDyn for KeyboardActionScriptInput {
    fn as_ProfileActionScriptItem(&self) -> &crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &self.base.base.base }
    fn as_ProfileActionScriptItem_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &mut self.base.base.base }
    fn process(&mut self, mut frame: Option<i64>) { Self::process(self, frame) }
    fn showDialog(&mut self) { Self::showDialog(self) }
}


