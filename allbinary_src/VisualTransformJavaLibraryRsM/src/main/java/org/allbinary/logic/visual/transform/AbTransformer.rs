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
// package org.allbinary.logic.visual.transform
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::InputStream::InputStream;
// use crate::java::io::StringBufferInputStream::StringBufferInputStream;
// use crate::javax::xml::transform::URIResolver::URIResolver;
// use crate::javax::xml::transform::stream::StreamSource::StreamSource;
use crate::org::allbinary::data::tree::dom::XslHelper::XslHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGenerator;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGeneratorFactory::TransformInfoObjectConfigGeneratorFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::BasicTransformerInterface::BasicTransformerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbTransformer {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>,
    pub inputStream: Option<Rc<RefCell<InputStream>>>,
    pub uriResolver: Option<Rc<RefCell<URIResolver>>>,
}

impl BasicTransformerInterface for AbTransformer {
    fn setInputStream(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>) { Self::setInputStream(self, inputStream) }
    fn setURIResolver(&mut self, mut uriResolver: Option<Rc<RefCell<URIResolver>>>) { Self::setURIResolver(self, uriResolver) }
    fn getStreamSource(&mut self) -> Option<Rc<RefCell<StreamSource>>> { Self::getStreamSource(self) }
    fn getInputStream(&mut self) -> Option<Rc<RefCell<InputStream>>> { Self::getInputStream(self) }
    fn getURIResolver(&mut self) -> Option<Rc<RefCell<URIResolver>>> { Self::getURIResolver(self) }
    fn translate(&mut self, mut xmlDocumentStr: String) -> String { Self::translate(self, xmlDocumentStr) }
}
impl crate::org::allbinary::logic::visual::transform::info::CompositeTransformInfoInterface::CompositeTransformInfoInterface for AbTransformer {
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { Self::getTransformInfoInterface(self) }
}

impl AbTransformer {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            transformInfoInterface: None,
            inputStream: None,
            uriResolver: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.transformInfoInterface = transformInfoInterface.clone();
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        return self.transformInfoInterface.clone();
    }
    

    pub fn setInputStream(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>) {
        self.inputStream = inputStream.clone();
    }
    

    pub fn setURIResolver(&mut self, mut uriResolver: Option<Rc<RefCell<URIResolver>>>) {
        self.uriResolver = uriResolver.clone();
    }
    

    pub fn getInputStream(&mut self) -> Option<Rc<RefCell<InputStream>>> {
        return self.inputStream.clone();
    }
    

    pub fn getURIResolver(&mut self) -> Option<Rc<RefCell<URIResolver>>> {
        return self.uriResolver.clone();
    }
    

    pub fn getStreamSource(&mut self) -> Option<Rc<RefCell<StreamSource>>> {
        return StreamSource::new(self.getInputStream());
    }
    

    // throws Exception
    pub fn translate(&mut self, mut xmlDocumentStr: String) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).XMLLOGGING.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("xml: \n"), xmlDocumentStr), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("translate(String xmlDocumentStr)"));
            }
            if self.getURIResolver().is_none() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No URIResolver"))));
            }
            let mut result: String = jcall!(XslHelper::getInstance()).translate3(self.getURIResolver(), StreamSource::new(self.getInputStream()), StreamSource::new(StringBufferInputStream::new(xmlDocumentStr.clone())));
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).XSLLOGGING.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("translated xml: "), result), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("translate(String xmlDocumentStr)"));
            }
            let mut transformInfoObjectConfigGeneratorInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGeneratorDyn>>> = jcall!(TransformInfoObjectConfigGeneratorFactory::getInstance()).getInstance1(self.getTransformInfoInterface());
            return jcall!(transformInfoObjectConfigGeneratorInterface).process(result.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(self.transformInfoInterface).log(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("translate(document)"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
    }
}
pub trait AbTransformerDyn {
    fn as_AbTransformer(&self) -> &AbTransformer;
    fn as_AbTransformer_mut(&mut self) -> &mut AbTransformer;
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { AbTransformer::getTransformInfoInterface(self.as_AbTransformer_mut()) }
    fn setInputStream(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>) { AbTransformer::setInputStream(self.as_AbTransformer_mut(), inputStream) }
    fn setURIResolver(&mut self, mut uriResolver: Option<Rc<RefCell<URIResolver>>>) { AbTransformer::setURIResolver(self.as_AbTransformer_mut(), uriResolver) }
    fn getInputStream(&mut self) -> Option<Rc<RefCell<InputStream>>> { AbTransformer::getInputStream(self.as_AbTransformer_mut()) }
    fn getURIResolver(&mut self) -> Option<Rc<RefCell<URIResolver>>> { AbTransformer::getURIResolver(self.as_AbTransformer_mut()) }
    fn getStreamSource(&mut self) -> Option<Rc<RefCell<StreamSource>>> { AbTransformer::getStreamSource(self.as_AbTransformer_mut()) }
    fn translate(&mut self, mut xmlDocumentStr: String) -> String { AbTransformer::translate(self.as_AbTransformer_mut(), xmlDocumentStr) }
}
impl AbTransformerDyn for AbTransformer {
    fn as_AbTransformer(&self) -> &AbTransformer { self }
    fn as_AbTransformer_mut(&mut self) -> &mut AbTransformer { self }
}


