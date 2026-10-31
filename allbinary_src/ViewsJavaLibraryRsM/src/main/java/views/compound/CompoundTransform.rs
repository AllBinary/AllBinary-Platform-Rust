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
// package views.compound
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::InputStream::InputStream;
// use crate::javax::xml::transform::URIResolver::URIResolver;
use crate::org::allbinary::data::tree::dom::BasicUriResolver::BasicUriResolver;
use crate::org::allbinary::data::tree::dom::StoreUriResolver::StoreUriResolver;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use crate::org::allbinary::logic::visual::transform::AbTransformer::AbTransformer;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::views::compound::objectConfig::CompoundContextTransformInfoObjectConfig::CompoundContextTransformInfoObjectConfig;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompoundTransform {
    pub base: AbTransformer,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
}


impl CompoundTransform {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: AbTransformer::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            abeClientInformation: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        // super(...) is in the struct literal above
        { let java_arguments = ((StoreUriResolver::new(this.getTransformInfoInterface(), (this.getURIResolver()).clone() /* as BasicUriResolver */)).clone() /* as URIResolver */,); this.setURIResolver(java_arguments.0) };
        let mut objectConfig: Option<Rc<RefCell<CompoundContextTransformInfoObjectConfig>>> = CompoundContextTransformInfoObjectConfig::new3(this.abeClientInformation.clone(), this.getTransformInfoInterface(), jcall!(jcall!(this.getTransformInfoInterface()).getObjectConfigInterface()).toXmlDoc());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("\nObjectConfig: \n"), jcall!(objectConfig).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut templateInputStream: Option<Rc<RefCell<InputStream>>> = jcall!(objectConfig).createInputStream();
        this.setInputStream(templateInputStream.clone());
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
}
impl std::ops::Deref for CompoundTransform {
    type Target = AbTransformer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CompoundTransform {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::AbTransformer::AbTransformerDyn for CompoundTransform {
    fn as_AbTransformer(&self) -> &crate::org::allbinary::logic::visual::transform::AbTransformer::AbTransformer { &self.base }
    fn as_AbTransformer_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::AbTransformer::AbTransformer { &mut self.base }
}


