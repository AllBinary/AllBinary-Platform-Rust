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
// package org.allbinary.logic.visual.transform.info.objectConfig.generator
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::InputOutputTypeData::InputOutputTypeData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TransformInfoObjectConfigGeneratorFactoryInterface::TransformInfoObjectConfigGeneratorFactoryInterface;
use super::StoreFileGenerator::StoreFileGenerator;
use super::BasicGenerator::BasicGenerator;
use super::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGenerator;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoObjectConfigGeneratorFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static TransformInfoObjectConfigGeneratorFactory_instance: Option<Rc<RefCell<TransformInfoObjectConfigGeneratorFactory>>> = TransformInfoObjectConfigGeneratorFactory::new(); }
impl TransformInfoObjectConfigGeneratorFactoryInterface for TransformInfoObjectConfigGeneratorFactory {
    fn getInstance(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGeneratorDyn>>> { Self::getInstance1(self, transformInfoInterface) }
}

impl TransformInfoObjectConfigGeneratorFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<TransformInfoObjectConfigGeneratorFactory>>> {
        return TransformInfoObjectConfigGeneratorFactory_instance.with(Clone::clone).clone();
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
    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGeneratorDyn>>> {
        let mut r#type: String = (jcall!(jcall!(transformInfoInterface).getObjectConfigInterface()).getOutputTypeName()).clone() /* as String */;
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Generating Instance for type: "), r#type), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone());
        }
        if false || (r#type.cmp(&jcall!(InputOutputTypeData::getInstance()).RESPONSE.clone()) as i32) == 0 || jcall!(transformInfoInterface).isChild() {
            return BasicGenerator::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGeneratorDyn>>);
        } else if (r#type.cmp(&jcall!(InputOutputTypeData::getInstance()).FILE.clone()) as i32) == 0 {
            return StoreFileGenerator::new(transformInfoInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGeneratorDyn>>);
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Such TransformInfoObjectConfigGenerator Type"))));
        }
    }
}

