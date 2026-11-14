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
// package org.allbinary.logic.visual.transform.info
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::w3c::dom::Attr::Attr;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::NamedNodeMap::NamedNodeMap;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Exception::*;
use super::TransformInfoDomNode::TransformInfoDomNode;
use super::TransformInfoInterface::TransformInfoInterface;
use super::TransformInfoData::TransformInfoData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoDomNodeView {
    pub base: TransformInfoDomNode,
}

impl DomNodeInterface for TransformInfoDomNodeView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl TransformInfoDomNodeView {
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut mapping: String) -> Self {
        let mut this = Self {
            base: TransformInfoDomNode::new_struct2(transformInfoInterface.clone(), mapping.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut mapping: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, mapping))))
    }
    

    pub fn new_struct1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: TransformInfoDomNode::new_struct1_TransformInfoInterface(transformInfoInterface.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(transformInfoInterface))))
    }
    

    pub fn new_struct2_String_String(mut name: String, mut mappedName: String) -> Self {
        let mut this = Self {
            base: TransformInfoDomNode::new_struct2_String_String(name.clone(), mappedName.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new2_String_String(mut name: String, mut mappedName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2_String_String(name, mappedName))))
    }
    

    pub fn new_struct1_String(mut name: String) -> Self {
        let mut this = Self {
            base: TransformInfoDomNode::new_struct2_String_String(name.clone(), None),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1_String(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_String(name))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut transformInfoData: Option<Rc<RefCell<TransformInfoData>>> = TransformInfoData::getInstance();
        let mut subViewNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement({ let java_field = jcall!(transformInfoData).NAME.clone(); java_field }.clone());
        let mut viewNameAttr: Option<Rc<RefCell<Attr>>> = jcall!(document).createAttribute({ let java_field = jcall!(transformInfoData).NAME.clone(); java_field }.clone());
        jcall!(viewNameAttr).setValue({ let java_argument = jcall!(self.getTransformInfoInterface()).getName(); java_argument });
        let mut componentNodeAttributes: Option<Rc<RefCell<NamedNodeMap>>> = jcall!(subViewNode).getAttributes();
        jcall!(componentNodeAttributes).setNamedItem(viewNameAttr.clone());
        if true {
            let mut mappedNameAttr: Option<Rc<RefCell<Attr>>> = jcall!(document).createAttribute({ let java_field = jcall!(transformInfoData).MAPPED.clone(); java_field }.clone());
            jcall!(mappedNameAttr).setValue(self.getMappedName());
            jcall!(componentNodeAttributes).setNamedItem(mappedNameAttr.clone());
        }
        return subViewNode.clone();
    }
}
impl std::ops::Deref for TransformInfoDomNodeView {
    type Target = TransformInfoDomNode;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TransformInfoDomNodeView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNodeDyn for TransformInfoDomNodeView {
    fn as_TransformInfoDomNode(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNode { &self.base }
    fn as_TransformInfoDomNode_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNode::TransformInfoDomNode { &mut self.base }
}


