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
// package views.business.context.modules.storefront.customizer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::generator::TransformGeneratorUtil::TransformGeneratorUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use super::CustomizerUtil::CustomizerUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreCustomizerComponentUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static StoreCustomizerComponentUtil_instance: Option<Rc<RefCell<StoreCustomizerComponentUtil>>> = StoreCustomizerComponentUtil::new(); }

impl StoreCustomizerComponentUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<StoreCustomizerComponentUtil>>> {
        return StoreCustomizerComponentUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn generate(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> String {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Generating Modified Views For: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateModifiedViews()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut customizerUtil: Option<Rc<RefCell<CustomizerUtil>>> = CustomizerUtil::getInstance();
        let mut allCustomizedViews: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(customizerUtil).getTransformInfoObjectConfigGroupComponentNodes(transformInfoInterface.clone());
        let mut size: i32 = jcall!(allCustomizedViews).size();
        for index in (0..size) {
            let mut nextTransformInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>> = { let java_object = (jcall!(allCustomizedViews).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>>>().expect("ClassCastException").clone()) };
            let mut allViewsToBeModified: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(customizerUtil).getTransformInfoObjectConfigComponentNodesToCustomize(transformInfoInterface.clone(), nextTransformInfoInterface.clone().map(|reference| reference as Rc<RefCell<dyn TransformInfoInterface>>));
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View or component that was modified by a Customizer: "), jcall!(nextTransformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateModifiedViews()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            self.generate3(abeClientInformation.clone(), transformInfoInterface.clone(), allViewsToBeModified.clone());
        }
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn generate3(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut allViewsToBeModifiedVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut customizerUtil: Option<Rc<RefCell<CustomizerUtil>>> = CustomizerUtil::getInstance();
        let mut size: i32 = jcall!(allViewsToBeModifiedVector).size();
        for index in (0..size) {
            let mut nextTransformInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>> = { let java_object = (jcall!(allViewsToBeModifiedVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>>>().expect("ClassCastException").clone()) };
            let mut specifiedTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>> = jcall!(customizerUtil).getTransformInfoInterfaceToCustomize(transformInfoInterface.clone(), nextTransformInfoInterface.clone().map(|reference| reference as Rc<RefCell<dyn TransformInfoInterface>>));
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Retrieved: "), jcall!(specifiedTransformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateModifiedViews()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            jcall!(TransformGeneratorUtil::getInstance()).generate(abeClientInformation.clone(), nextTransformInfoInterface.clone(), specifiedTransformInfoInterface.clone());
        }
    }
}

