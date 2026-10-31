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
// package org.allbinary.input.automation.module.actions.script.condition.processors.output
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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::BasicProfileActionScriptOutput::BasicProfileActionScriptOutput;
use super::ImageActionScriptOutputInterface::ImageActionScriptOutputInterface;
use super::ImageTypes::ImageTypes;
use super::ImageActionScriptOutputJPanel::ImageActionScriptOutputJPanel;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageActionScriptOutput {
    pub base: BasicProfileActionScriptOutput,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub saved: bool,
    pub display: bool,
    pub imageTypes: Option<Rc<RefCell<ImageTypes>>>,
    pub actionScriptOutputJPanel: Option<Rc<RefCell<ImageActionScriptOutputJPanel>>>,
}

thread_local! { pub static ImageActionScriptOutput_NAME: String = String::from("Captures"); }
// impl ImageActionScriptOutputInterface for ImageActionScriptOutput with the methods of the trait
impl crate::org::allbinary::input::automation::actions::script::condition::processors::ProfileActionScriptProcessorInterface::ProfileActionScriptProcessorInterface for ImageActionScriptOutput {
}
impl crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface for ImageActionScriptOutput {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItemInterface::ProfileActionScriptItemInterface for ImageActionScriptOutput {
    fn process(&mut self, mut frame: Option<i64>) { Self::process(self, frame) }
}

impl ImageActionScriptOutput {
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptOutput::new_struct2(ImageActionScriptOutput_NAME.with(Clone::clone).clone(), node.clone()),
            logUtil: None,
            saved: false,
            display: false,
            imageTypes: None,
            actionScriptOutputJPanel: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.saved = false;
        this.display = false;
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        let mut actionNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(unsafe { ImageActionScriptOutputData_NAME.clone() }.clone(), jcall!(node).getChildNodes());
        if actionNode.is_some() {
            let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(actionNode).getChildNodes();
            for index in (0..jcall!(nodeList).getLength()) {
                let mut childNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
                if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { ImageActionScriptOutputData_SAVE.clone() }.clone()) == 0 {
                    let mut booleanString: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    { let java_arguments = (Some(booleanString.clone()).unwrap(),); this.setSaved(java_arguments.0) };
                } else if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { ImageActionScriptOutputData_DISPLAY.clone() }.clone()) == 0 {
                    let mut booleanString: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    { let java_arguments = (Some(booleanString.clone()).unwrap(),); this.setSaved(java_arguments.0) };
                } else if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { ImageActionScriptOutputData_TYPES.clone() }.clone()) == 0 {
                    this.setImageTypes(ImageTypes::new1(childNode.clone()));
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Action Script Output Unknown Node"))));
                }
            }
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Action Script Output Node Null"))));
        }
        this.setAllowsChildren(false);
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct1(node))));
            this.actionScriptOutputJPanel = ImageActionScriptOutputJPanel::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptOutput::new_struct1(ImageActionScriptOutput_NAME.with(Clone::clone).clone()),
            logUtil: None,
            saved: false,
            display: false,
            imageTypes: None,
            actionScriptOutputJPanel: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.saved = false;
        this.display = false;
        // super(...) is in the struct literal above
        this.setImageTypes(ImageTypes::new());
        this.setAllowsChildren(false);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct())));
            this.actionScriptOutputJPanel = ImageActionScriptOutputJPanel::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn getActionScriptOutputJPanel(&mut self) -> Option<Rc<RefCell<ImageActionScriptOutputJPanel>>> {
        return self.actionScriptOutputJPanel.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(unsafe { ImageActionScriptOutputData_DISPLAY.clone() }.clone(), (self.isDisplay()).to_string());
        jcall!(hashMap).put(unsafe { ImageActionScriptOutputData_SAVE.clone() }.clone(), (self.isSaved()).to_string());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = self.base.toXmlNode(document.clone());
        let mut newNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createNodeWithValueNodes(document.clone(), unsafe { ImageActionScriptOutputData_NAME.clone() }.clone(), self.toHashMap());
        jcall!(newNode).appendChild(jcall!(self.getImageTypes()).toXmlNode(document.clone()));
        jcall!(node).appendChild(newNode.clone());
        return node.clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut frame: Option<i64>) {
        ImageActionScriptOutputProcessor::ImageActionScriptOutputProcessor::process(None /* Java this: a &mut self method has no Rc of itself */, frame.clone());
    }
    

    pub fn showDialog(&mut self) {
        jcall!(jcall!(self.actionScriptOutputJPanel).getCapturedImageActionJDialog()).setVisible(true);
    }
    

    pub fn isSaved(&mut self) -> bool {
        return self.saved.clone();
    }
    

    pub fn setSaved(&mut self, mut saved: bool) {
        self.saved = saved.clone();
    }
    

    pub fn isDisplay(&mut self) -> bool {
        return self.display.clone();
    }
    

    pub fn setDisplay(&mut self, mut display: bool) {
        self.display = display.clone();
    }
    

    pub fn future_toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from(" Is Save: "));
        jcall!(stringBuffer).appendboolean(self.isSaved());
        jcall!(stringBuffer).append(String::from(" Is Display: "));
        jcall!(stringBuffer).appendboolean(self.isDisplay());
        jcall!(stringBuffer).append(String::from(" "));
        jcall!(stringBuffer).append(jcall!(self.getImageTypes()).toString());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn log(&mut self) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (self.future_toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("log"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn getImageTypes(&mut self) -> Option<Rc<RefCell<ImageTypes>>> {
        return self.imageTypes.clone();
    }
    

    pub fn setImageTypes(&mut self, mut imageTypes: Option<Rc<RefCell<ImageTypes>>>) {
        self.imageTypes = imageTypes.clone();
    }
}
impl std::ops::Deref for ImageActionScriptOutput {
    type Target = BasicProfileActionScriptOutput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageActionScriptOutput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::module::actions::script::condition::processors::output::BasicProfileActionScriptOutput::BasicProfileActionScriptOutputDyn for ImageActionScriptOutput {
    fn as_BasicProfileActionScriptOutput(&self) -> &crate::org::allbinary::input::automation::module::actions::script::condition::processors::output::BasicProfileActionScriptOutput::BasicProfileActionScriptOutput { &self.base }
    fn as_BasicProfileActionScriptOutput_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::actions::script::condition::processors::output::BasicProfileActionScriptOutput::BasicProfileActionScriptOutput { &mut self.base }
}
impl crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessorDyn for ImageActionScriptOutput {
    fn as_BasicProfileActionScriptProcessor(&self) -> &crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessor { &self.base.base }
    fn as_BasicProfileActionScriptProcessor_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessor { &mut self.base.base }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItemDyn for ImageActionScriptOutput {
    fn as_ProfileActionScriptItem(&self) -> &crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &self.base.base.base }
    fn as_ProfileActionScriptItem_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &mut self.base.base.base }
    fn process(&mut self, mut frame: Option<i64>) { Self::process(self, frame) }
    fn showDialog(&mut self) { Self::showDialog(self) }
}


