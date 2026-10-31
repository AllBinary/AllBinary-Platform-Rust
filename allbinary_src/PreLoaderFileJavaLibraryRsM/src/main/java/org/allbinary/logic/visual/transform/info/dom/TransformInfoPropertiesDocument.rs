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
// package org.allbinary.logic.visual.transform.info.dom
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentFileHelper::DomDocumentFileHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoData::TransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoProperties::TransformInfoProperties;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoPropertiesFactory::TransformInfoPropertiesFactory;
use crate::org::allbinary::logic::visual::transform::info::TransformInfosData::TransformInfosData;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoPropertiesDocument {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub document: Option<Rc<RefCell<Document>>>,
}


impl TransformInfoPropertiesDocument {
    

    pub fn new_struct(mut filePath: String) -> Self {
        let mut this = Self {
            logUtil: None,
            document: None,
        };
        this.logUtil = LogUtil::getInstance();
        let mut xmlFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(filePath.clone());
        this.document = DomDocumentFileHelper::createDocument(xmlFile.clone());
        this
    }

    pub fn new(mut filePath: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(filePath))))
    }
    

    pub fn toTransformInfoPropertiesHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut transformInfoPropertiesHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            let mut transformInfosNode: Option<Rc<RefCell<Node>>> = jcall!({ let java_receiver = self.document.clone(); let java_arguments = (jcall!(TransformInfosData::getInstance()).NAME.clone(),); let java_result = jcall!(java_receiver).getElementsByTagName(java_arguments.0); java_result }).item(0);
            let mut transformInfosChildNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(transformInfosNode).getChildNodes();
            let mut transformInfoNodeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = DomSearchHelper::getAllNodes(jcall!(TransformInfoData::getInstance()).NAME.clone(), transformInfosChildNodeList.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Size: "), jcall!(transformInfoNodeVector).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toTransformInfoPropertiesHashMap()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut size: i32 = jcall!(transformInfoNodeVector).size();
            let mut node: Option<Rc<RefCell<Node>>> = None;
            let mut transformInfoProperties: Option<Rc<RefCell<TransformInfoProperties>>> = None;
            for index in (0..size) {
                node = (jcall!(transformInfoNodeVector).get(index.clone())).clone() /* as Node */;
                transformInfoProperties = jcall!(TransformInfoPropertiesFactory::getInstance()).getInstance1(node.clone());
                jcall!(transformInfoPropertiesHashMap).put(jcall!(transformInfoProperties).getName(), transformInfoProperties.clone());
            }
            return transformInfoPropertiesHashMap.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toTransformInfoPropertiesHashMap()"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
}

