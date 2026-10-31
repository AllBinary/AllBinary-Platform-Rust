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
// package org.allbinary.business.context
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionInterface::WeblisketSessionInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbContext {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub propertiesHashMap: Option<Rc<RefCell<HashMap>>>,
    pub pageContext: Option<Rc<RefCell<PageContext>>>,
}


impl AbContext {
    

    pub fn new_struct(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            propertiesHashMap: None,
            pageContext: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Properties HashMap: "), jcall!(propertiesHashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this.propertiesHashMap = propertiesHashMap.clone();
        this.pageContext = pageContext.clone();
        this
    }

    pub fn new(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(propertiesHashMap, pageContext))))
    }
    

    pub fn getWeblisketSession(&mut self) -> Option<Rc<RefCell<dyn WeblisketSessionInterface>>> {
        return WeblisketSession::new2(self.propertiesHashMap.clone(), self.pageContext.clone()).map(|reference| reference as Rc<RefCell<dyn WeblisketSessionInterface>>);
    }
    

    pub fn getPageContext(&mut self) -> Option<Rc<RefCell<PageContext>>> {
        return self.pageContext.clone();
    }
    

    pub fn getPropertiesHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return self.propertiesHashMap.clone();
    }
}
pub trait AbContextDyn {
    fn as_AbContext(&self) -> &AbContext;
    fn as_AbContext_mut(&mut self) -> &mut AbContext;
    fn getWeblisketSession(&mut self) -> Option<Rc<RefCell<dyn WeblisketSessionInterface>>> { AbContext::getWeblisketSession(self.as_AbContext_mut()) }
    fn getPageContext(&mut self) -> Option<Rc<RefCell<PageContext>>> { AbContext::getPageContext(self.as_AbContext_mut()) }
    fn getPropertiesHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { AbContext::getPropertiesHashMap(self.as_AbContext_mut()) }
}
impl AbContextDyn for AbContext {
    fn as_AbContext(&self) -> &AbContext { self }
    fn as_AbContext_mut(&mut self) -> &mut AbContext { self }
}


