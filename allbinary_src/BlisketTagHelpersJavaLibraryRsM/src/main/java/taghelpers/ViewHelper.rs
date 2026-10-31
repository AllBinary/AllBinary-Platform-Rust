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
// package taghelpers
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use crate::org::allbinary::logic::visual::transform::TransformFactory::TransformFactory;
use crate::org::allbinary::logic::visual::transform::TransformInterface::TransformInterface;
use crate::org::allbinary::logic::visual::transform::data::TransformDocumentInterface::TransformDocumentInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ViewHelper {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub componentInterface: Option<Rc<RefCell<dyn TransformInterface>>>,
    pub NO_TYPE: i32,
}

impl TransformInterface for ViewHelper {
    fn getTypeId(&mut self) -> i32 { Self::getTypeId(self) }
    fn setTransformDocumentInterface(&mut self, mut viewDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>) { Self::setTransformDocumentInterface(self, viewDocumentInterface) }
    fn getTransformDocumentInterface(&mut self) -> Option<Rc<RefCell<dyn TransformDocumentInterface>>> { Self::getTransformDocumentInterface(self) }
    fn view(&mut self) -> String { Self::view(self) }
}
impl crate::org::allbinary::logic::visual::transform::info::CompositeTransformInfoInterface::CompositeTransformInfoInterface for ViewHelper {
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { Self::getTransformInfoInterface(self) }
}
impl crate::org::allbinary::data::tree::dom::document::mapping::DomDocumentMappingInterface::DomDocumentMappingInterface for ViewHelper {
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toXmlDoc(self) }
}

impl ViewHelper {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            abeClientInformation: None,
            componentInterface: None,
            NO_TYPE: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        this.NO_TYPE = 0;
        this.componentInterface = jcall!(TransformFactory::getInstance()).getInstance3(this.abeClientInformation.clone(), hashMap.clone(), pageContext.clone());
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap, pageContext))))
    }
    

    pub fn getTypeId(&mut self) -> i32 {
        return self.NO_TYPE.clone();
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return jcall!(self.componentInterface).view();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to View: ");
            if self.componentInterface.is_some() {
                error += &std::any::type_name_of_val(&self.componentInterface).to_string();
            } else {
                error += &String::from("Unknown");
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Hmm: "), jcall!(jcall!(self.componentInterface).getTransformDocumentInterface()).getDoc()))));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to View: ");
            if self.componentInterface.is_some() {
                error += &std::any::type_name_of_val(&self.componentInterface).to_string();
            } else {
                error += &String::from("Unknown");
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    pub fn getViewObject(&mut self) -> Option<Rc<RefCell<dyn TransformInterface>>> {
        return self.componentInterface.clone();
    }
    

    // throws Exception
    pub fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        return jcall!(self.componentInterface).getTransformInfoInterface();
    }
    

    pub fn getTransformDocumentInterface(&mut self) -> Option<Rc<RefCell<dyn TransformDocumentInterface>>> {
        return None;
    }
    

    pub fn setTransformDocumentInterface(&mut self, mut viewDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>) {
        return;
    }
}
pub trait ViewHelperDyn {
    fn as_ViewHelper(&self) -> &ViewHelper;
    fn as_ViewHelper_mut(&mut self) -> &mut ViewHelper;
    fn getTypeId(&mut self) -> i32 { ViewHelper::getTypeId(self.as_ViewHelper_mut()) }
    fn view(&mut self) -> String { ViewHelper::view(self.as_ViewHelper_mut()) }
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { ViewHelper::toXmlDoc(self.as_ViewHelper_mut()) }
    fn getViewObject(&mut self) -> Option<Rc<RefCell<dyn TransformInterface>>> { ViewHelper::getViewObject(self.as_ViewHelper_mut()) }
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { ViewHelper::getTransformInfoInterface(self.as_ViewHelper_mut()) }
    fn getTransformDocumentInterface(&mut self) -> Option<Rc<RefCell<dyn TransformDocumentInterface>>> { ViewHelper::getTransformDocumentInterface(self.as_ViewHelper_mut()) }
    fn setTransformDocumentInterface(&mut self, mut viewDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>) { ViewHelper::setTransformDocumentInterface(self.as_ViewHelper_mut(), viewDocumentInterface) }
}
impl ViewHelperDyn for ViewHelper {
    fn as_ViewHelper(&self) -> &ViewHelper { self }
    fn as_ViewHelper_mut(&mut self) -> &mut ViewHelper { self }
}


