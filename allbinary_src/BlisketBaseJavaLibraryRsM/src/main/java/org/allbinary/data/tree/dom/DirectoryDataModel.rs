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
// package org.allbinary.data.tree.dom
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::directory::Directory::Directory;
use crate::org::allbinary::logic::visual::transform::TransformInterface::TransformInterface;
use crate::org::allbinary::logic::visual::transform::data::TransformDocumentFactory::TransformDocumentFactory;
use crate::org::allbinary::logic::visual::transform::data::TransformDocumentInterface::TransformDocumentInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::DomNodeInterface::DomNodeInterface;
use super::FileDomDataModel::FileDomDataModel;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DirectoryDataModel {
    pub fileBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub transformDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>,
    pub NO_TYPE: i32,
}

impl DomNodeInterface for DirectoryDataModel {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl TransformInterface for DirectoryDataModel {
    fn getTypeId(&mut self) -> i32 { Self::getTypeId(self) }
    fn setTransformDocumentInterface(&mut self, mut transformDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>) { Self::setTransformDocumentInterface(self, transformDocumentInterface) }
    fn getTransformDocumentInterface(&mut self) -> Option<Rc<RefCell<dyn TransformDocumentInterface>>> { Self::getTransformDocumentInterface(self) }
    fn view(&mut self) -> String { Self::view(self) }
}
impl crate::org::allbinary::logic::visual::transform::info::CompositeTransformInfoInterface::CompositeTransformInfoInterface for DirectoryDataModel {
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { Self::getTransformInfoInterface(self) }
}
impl crate::org::allbinary::data::tree::dom::document::mapping::DomDocumentMappingInterface::DomDocumentMappingInterface for DirectoryDataModel {
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toXmlDoc(self) }
}

impl DirectoryDataModel {
    

    pub fn new_struct(mut file: Option<Rc<RefCell<AbFile>>>) -> Self {
        let mut this = Self {
            fileBasicArrayList: None,
            transformDocumentInterface: None,
            NO_TYPE: 0,
        };
        this.NO_TYPE = 0;
        this.fileBasicArrayList = jcall!(Directory::getInstance()).search(file.clone());
        this.transformDocumentInterface = (TransformDocumentFactory::getInstance()).clone() /* as TransformDocumentInterface */;
        this
    }

    pub fn new(mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(file))))
    }
    

    pub fn getTypeId(&mut self) -> i32 {
        return self.NO_TYPE.clone();
    }
    

    // throws Exception
    pub fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        return None;
    }
    

    pub fn setTransformDocumentInterface(&mut self, mut transformDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>) {
        self.transformDocumentInterface = transformDocumentInterface.clone();
    }
    

    pub fn getTransformDocumentInterface(&mut self) -> Option<Rc<RefCell<dyn TransformDocumentInterface>>> {
        return self.transformDocumentInterface.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut dirNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(String::from("dir"));
        let mut size: i32 = jcall!(self.fileBasicArrayList).size();
        let mut nextFile: Option<Rc<RefCell<AbFile>>> = None;
        let mut fileNode: Option<Rc<RefCell<Node>>> = None;
        for index in (0..size) {
            nextFile = { let java_object = (jcall!(self.fileBasicArrayList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<AbFile>>>>().expect("ClassCastException").clone()) };
            fileNode = jcall!(FileDomDataModel::new(nextFile.clone())).toXmlNode(document.clone());
            jcall!(dirNode).appendChild(fileNode.clone());
        }
        return dirNode.clone();
    }
    

    // throws Exception
    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return jcall!(self.getTransformDocumentInterface()).getDoc();
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        let mut node: Option<Rc<RefCell<Node>>> = { let java_arguments = (jcall!(self.getTransformDocumentInterface()).getDoc(),); self.toXmlNode(java_arguments.0) };
        jcall!(jcall!(self.getTransformDocumentInterface()).getBaseNode()).appendChild(node.clone());
        return XmlDocumentHelper::toString(jcall!(self.getTransformDocumentInterface()).getDoc());
    }
}

