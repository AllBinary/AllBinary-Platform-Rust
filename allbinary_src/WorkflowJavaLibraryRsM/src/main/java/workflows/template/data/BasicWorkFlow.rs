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
// package workflows.template.data
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
// use crate::javax::servlet::jsp::tagext::TagSupport::TagSupport;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::ValidationComponentInterface::ValidationComponentInterface;
use crate::org::allbinary::logic::control::workflow::WorkFlowData::WorkFlowData;
use crate::org::allbinary::logic::control::workflow::WorkFlowInterface::WorkFlowInterface;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use crate::org::allbinary::logic::visual::transform::TransformFactory::TransformFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicWorkFlow {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub validationDomNodeInterface: Option<Rc<RefCell<dyn ValidationComponentInterface>>>,
    pub propertiesHashMap: Option<Rc<RefCell<HashMap>>>,
    pub pageContext: Option<Rc<RefCell<PageContext>>>,
}

impl WorkFlowInterface for BasicWorkFlow {
    fn getName(&mut self) -> String { Self::getName(self) }
    fn process(&mut self) -> Option<i32> { Self::process(self) }
}

impl BasicWorkFlow {
    

    pub fn new_struct(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            abeClientInformation: None,
            validationDomNodeInterface: None,
            propertiesHashMap: None,
            pageContext: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).WORKFLOW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Properties: "), jcall!(propertiesHashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("BasicWorkFlow()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this.propertiesHashMap = propertiesHashMap.clone();
        this.pageContext = pageContext.clone();
        this.validationDomNodeInterface = (jcall!(TransformFactory::getInstance()).getInstance3(this.abeClientInformation.clone(), propertiesHashMap.clone(), pageContext.clone())).clone() /* as ValidationComponentInterface */;
        this
    }

    pub fn new(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(propertiesHashMap, pageContext))))
    }
    

    // throws Exception
    pub fn getName(&mut self) -> String {
        return (jcall!(self.getPropertiesHashMap()).get(jcall!(WorkFlowData::getInstance()).NAME.clone())).clone() /* as String */;
    }
    

    pub fn getPropertiesHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return self.propertiesHashMap.clone();
    }
    

    pub fn getPageContext(&mut self) -> Option<Rc<RefCell<PageContext>>> {
        return self.pageContext.clone();
    }
    

    // throws Exception
    pub fn process(&mut self) -> Option<i32> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).WORKFLOW.clone()) {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone());
        }
        if jcall!(self.validationDomNodeInterface).isValid().unwrap() {
            jcall!(jcall!(self.pageContext).getOut()).print(jcall!(self.validationDomNodeInterface).view());
            return (unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_EVAL_BODY_INCLUDE.clone() }.clone()).to_string();
        } else {
            jcall!(jcall!(self.pageContext).getOut()).print(jcall!(self.validationDomNodeInterface).validationInfo());
            return (unsafe { crate::javax::servlet::jsp::tagext::TagSupport::TagSupport_SKIP_BODY.clone() }.clone()).to_string();
        }
    }
}
pub trait BasicWorkFlowDyn {
    fn as_BasicWorkFlow(&self) -> &BasicWorkFlow;
    fn as_BasicWorkFlow_mut(&mut self) -> &mut BasicWorkFlow;
    fn getName(&mut self) -> String { BasicWorkFlow::getName(self.as_BasicWorkFlow_mut()) }
    fn getPropertiesHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { BasicWorkFlow::getPropertiesHashMap(self.as_BasicWorkFlow_mut()) }
    fn getPageContext(&mut self) -> Option<Rc<RefCell<PageContext>>> { BasicWorkFlow::getPageContext(self.as_BasicWorkFlow_mut()) }
    fn process(&mut self) -> Option<i32> { BasicWorkFlow::process(self.as_BasicWorkFlow_mut()) }
}
impl BasicWorkFlowDyn for BasicWorkFlow {
    fn as_BasicWorkFlow(&self) -> &BasicWorkFlow { self }
    fn as_BasicWorkFlow_mut(&mut self) -> &mut BasicWorkFlow { self }
}


