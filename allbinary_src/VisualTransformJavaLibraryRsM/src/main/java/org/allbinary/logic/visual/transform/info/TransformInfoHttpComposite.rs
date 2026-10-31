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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TransformInfoHttpInterface::TransformInfoHttpInterface;
use super::TransformInfoInterface::TransformInfoInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoHttpComposite {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>>,
}


impl TransformInfoHttpComposite {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            transformInfoInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this.transformInfoInterface = (transformInfoInterface).clone() /* as TransformInfoHttpInterface */;
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    // throws Exception
    pub fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        return self.transformInfoInterface.clone().map(|reference| reference as Rc<RefCell<dyn TransformInfoInterface>>);
    }
    

    pub fn getWeblisketSession(&mut self) -> Option<Rc<RefCell<WeblisketSession>>> {
        return WeblisketSession::new2(self.getPropertiesHashMap(), self.getPageContext());
    }
    

    pub fn getPageContext(&mut self) -> Option<Rc<RefCell<PageContext>>> {
        return jcall!(self.transformInfoInterface).getPageContext();
    }
    

    pub fn getPropertiesHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return jcall!(self.transformInfoInterface).getPropertiesHashMap();
    }
}
pub trait TransformInfoHttpCompositeDyn {
    fn as_TransformInfoHttpComposite(&self) -> &TransformInfoHttpComposite;
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut TransformInfoHttpComposite;
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { TransformInfoHttpComposite::getTransformInfoInterface(self.as_TransformInfoHttpComposite_mut()) }
    fn getWeblisketSession(&mut self) -> Option<Rc<RefCell<WeblisketSession>>> { TransformInfoHttpComposite::getWeblisketSession(self.as_TransformInfoHttpComposite_mut()) }
    fn getPageContext(&mut self) -> Option<Rc<RefCell<PageContext>>> { TransformInfoHttpComposite::getPageContext(self.as_TransformInfoHttpComposite_mut()) }
    fn getPropertiesHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { TransformInfoHttpComposite::getPropertiesHashMap(self.as_TransformInfoHttpComposite_mut()) }
}
impl TransformInfoHttpCompositeDyn for TransformInfoHttpComposite {
    fn as_TransformInfoHttpComposite(&self) -> &TransformInfoHttpComposite { self }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut TransformInfoHttpComposite { self }
}


