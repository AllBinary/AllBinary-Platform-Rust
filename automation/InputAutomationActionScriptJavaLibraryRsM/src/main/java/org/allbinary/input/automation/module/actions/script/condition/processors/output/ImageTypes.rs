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
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::input::media::image::InputImageType::InputImageType;
use crate::org::allbinary::logic::StdUtil::StdUtil;
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

pub struct ImageTypes {
    pub vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl ImageTypes {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            vector: None,
        };
        this.vector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            vector: None,
        };
        this.vector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        for index in (0..jcall!(nodeList).getLength()) {
            let mut childNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { ImageActionScriptOutputData_TYPE.clone() }.clone()) == 0 {
                let mut nextImageTypeString: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                jcall!(this.getVector()).add(InputImageType::getInstance(nextImageTypeString.clone()));
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("ImageTypes Unknown Node"))));
            }
        }
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn valueOf(mut anyType: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) -> Option<Rc<RefCell<ImageTypes>>> {
        let mut imageTypes: Option<Rc<RefCell<ImageTypes>>> = ImageTypes::new();
        for index in (0..(anyType.len() as i32)) {
            let mut imageType: Option<Rc<RefCell<InputImageType>>> = InputImageType::getInstance((anyType[(index) as usize]).clone() /* as String */);
            jcall!(jcall!(imageTypes).getVector()).add(imageType.clone());
        }
        return imageTypes.clone();
    }
    

    pub fn getVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.vector.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut newNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(unsafe { ImageActionScriptOutputData_TYPES.clone() }.clone());
        let mut size: i32 = jcall!(self.vector).size();
        for index in (0..size) {
            let mut imageType: Option<Rc<RefCell<InputImageType>>> = { let java_object = (jcall!(self.vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<InputImageType>>>>().expect("ClassCastException").clone()) };
            jcall!(newNode).appendChild(ModDomHelper::createTextNode(document.clone(), unsafe { ImageActionScriptOutputData_TYPE.clone() }.clone(), jcall!(imageType).getName()));
        }
        return newNode.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("ImageTypes: "));
        let mut size: i32 = jcall!(self.vector).size();
        for index in (0..size) {
            let mut imageType: Option<Rc<RefCell<InputImageType>>> = { let java_object = (jcall!(self.vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<InputImageType>>>>().expect("ClassCastException").clone()) };
            jcall!(stringBuffer).append(jcall!(imageType).getName());
            if index < size - 1 {
                jcall!(stringBuffer).append(String::from(", "));
            }
        }
        return jcall!(stringBuffer).toString();
    }
}

