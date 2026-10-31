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
// package views
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use crate::org::allbinary::logic::visual::transform::TransformInfoCustomUriTransformer::TransformInfoCustomUriTransformer;
use crate::org::allbinary::logic::visual::transform::TransformInterface::TransformInterface;
use crate::org::allbinary::logic::visual::transform::data::TransformDocumentFactory::TransformDocumentFactory;
use crate::org::allbinary::logic::visual::transform::data::TransformDocumentInterface::TransformDocumentInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomUriTransformView {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>,
    pub transformDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>,
    pub NO_TYPE: i32,
}

impl TransformInterface for CustomUriTransformView {
    fn getTypeId(&mut self) -> i32 { Self::getTypeId(self) }
    fn setTransformDocumentInterface(&mut self, mut transformDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>) { Self::setTransformDocumentInterface(self, transformDocumentInterface) }
    fn getTransformDocumentInterface(&mut self) -> Option<Rc<RefCell<dyn TransformDocumentInterface>>> { Self::getTransformDocumentInterface(self) }
    fn view(&mut self) -> String { Self::view(self) }
}
impl crate::org::allbinary::logic::visual::transform::info::CompositeTransformInfoInterface::CompositeTransformInfoInterface for CustomUriTransformView {
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { Self::getTransformInfoInterface(self) }
}
impl crate::org::allbinary::data::tree::dom::document::mapping::DomDocumentMappingInterface::DomDocumentMappingInterface for CustomUriTransformView {
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toXmlDoc(self) }
}

impl CustomUriTransformView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            abeClientInformation: None,
            transformInfoInterface: None,
            transformDocumentInterface: None,
            NO_TYPE: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        this.NO_TYPE = 0;
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this.setTransformInfoInterface(transformInfoInterface.clone());
        { let java_arguments = (TransformDocumentFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn TransformDocumentInterface>>),); this.setTransformDocumentInterface(java_arguments.0) };
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn getTypeId(&mut self) -> i32 {
        return self.NO_TYPE.clone();
    }
    

    pub fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        return self.transformInfoInterface.clone();
    }
    

    pub fn setTransformInfoInterface(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) {
        self.transformInfoInterface = transformInfoInterface.clone();
    }
    

    pub fn getTransformDocumentInterface(&mut self) -> Option<Rc<RefCell<dyn TransformDocumentInterface>>> {
        return self.transformDocumentInterface.clone();
    }
    

    pub fn setTransformDocumentInterface(&mut self, mut transformDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>) {
        self.transformDocumentInterface = transformDocumentInterface.clone();
    }
    

    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return jcall!(self.transformDocumentInterface).getDoc();
    }
    

    // throws Exception
    pub fn getDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        let mut document: Option<Rc<RefCell<Document>>> = jcall!(self.getTransformInfoInterface()).getDataDocument();
        let mut dataNode: Option<Rc<RefCell<Node>>> = jcall!(jcall!(self.getTransformDocumentInterface()).getDoc()).importNode(jcall!(document).getFirstChild(), true);
        if dataNode.is_some() {
            jcall!(jcall!(self.getTransformDocumentInterface()).getBaseNode()).appendChild(dataNode.clone());
        }
        return jcall!(self.getTransformDocumentInterface()).getDoc();
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = XmlDocumentHelper::toString(self.getDoc());
            let mut result: String = jcall!(TransformInfoCustomUriTransformer::new(self.abeClientInformation.clone(), self.getTransformInfoInterface())).translate(success.clone());
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}

