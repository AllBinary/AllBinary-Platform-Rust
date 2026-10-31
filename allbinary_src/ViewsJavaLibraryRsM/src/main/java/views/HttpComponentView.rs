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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use crate::org::allbinary::logic::visual::transform::BasicTransformer::BasicTransformer;
use crate::org::allbinary::logic::visual::transform::TransformInterface::TransformInterface;
use crate::org::allbinary::logic::visual::transform::data::TransformDocumentInterface::TransformDocumentInterface;
use crate::org::allbinary::logic::visual::transform::data::TransformHttpRequestDocumentFactory::TransformHttpRequestDocumentFactory;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HttpComponentView {
    pub base: TransformInfoHttpComposite,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub domNodeInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub transformDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>,
    pub NO_TYPE: i32,
}

// impl TransformInterface for HttpComponentView with the methods of the trait

impl HttpComponentView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: TransformInfoHttpComposite::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            abeClientInformation: None,
            domNodeInterfaceVector: None,
            transformDocumentInterface: None,
            NO_TYPE: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        this.NO_TYPE = 0;
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this.domNodeInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        { let java_arguments = (TransformHttpRequestDocumentFactory::getInstance(this.getPageContext(), this.getWeblisketSession()),); this.setTransformDocumentInterface(java_arguments.0) };
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn getTypeId(&mut self) -> i32 {
        return self.NO_TYPE.clone();
    }
    

    pub fn getTransformDocumentInterface(&mut self) -> Option<Rc<RefCell<dyn TransformDocumentInterface>>> {
        return self.transformDocumentInterface.clone();
    }
    

    pub fn setTransformDocumentInterface(&mut self, mut transformDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>) {
        self.transformDocumentInterface = transformDocumentInterface.clone();
    }
    

    pub fn addDomNodeInterface(&mut self, mut domNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>>) {
        jcall!(self.domNodeInterfaceVector).add(crate::java_into_object(domNodeInterface.clone()));
    }
    

    // throws Exception
    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut size: i32 = jcall!(self.domNodeInterfaceVector).size();
            for index in (0..size) {
                let mut domNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = { let java_object = jcall!(self.domNodeInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn DomNodeInterface>>>>().expect("ClassCastException").clone()) };
                jcall!(jcall!(self.transformDocumentInterface).getBaseNode()).appendChild(jcall!(domNodeInterface).toXmlNode(jcall!(self.transformDocumentInterface).getDoc()));
            }
            return jcall!(self.getTransformDocumentInterface()).getDoc();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlDoc()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn getDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        let mut document: Option<Rc<RefCell<Document>>> = jcall!(self.getTransformInfoInterface()).getDataDocument();
        let mut node: Option<Rc<RefCell<Node>>> = DomNodeHelper::getFirstChildElement(document.clone());
        if node.is_some() {
            let mut dataNode: Option<Rc<RefCell<Node>>> = jcall!(jcall!(self.getTransformDocumentInterface()).getDoc()).importNode(node.clone(), true);
            if dataNode.is_some() {
                jcall!(jcall!(self.getTransformDocumentInterface()).getBaseNode()).appendChild(dataNode.clone());
            }
        }
        return jcall!(self.getTransformDocumentInterface()).getDoc();
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.toXmlDoc();
            let mut success: String = XmlDocumentHelper::toString(self.getDoc());
            let mut result: String = jcall!(BasicTransformer::new(self.abeClientInformation.clone(), self.getTransformInfoInterface()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::BasicTransformer::BasicTransformerDyn>>)).translate(success.clone());
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}
impl std::ops::Deref for HttpComponentView {
    type Target = TransformInfoHttpComposite;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HttpComponentView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait HttpComponentViewDyn: crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn {
    fn as_HttpComponentView(&self) -> &HttpComponentView;
    fn as_HttpComponentView_mut(&mut self) -> &mut HttpComponentView;
    fn getTypeId(&mut self) -> i32 { HttpComponentView::getTypeId(self.as_HttpComponentView_mut()) }
    fn getTransformDocumentInterface(&mut self) -> Option<Rc<RefCell<dyn TransformDocumentInterface>>> { HttpComponentView::getTransformDocumentInterface(self.as_HttpComponentView_mut()) }
    fn setTransformDocumentInterface(&mut self, mut transformDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>) { HttpComponentView::setTransformDocumentInterface(self.as_HttpComponentView_mut(), transformDocumentInterface) }
    fn addDomNodeInterface(&mut self, mut domNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>>) { HttpComponentView::addDomNodeInterface(self.as_HttpComponentView_mut(), domNodeInterface) }
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { HttpComponentView::toXmlDoc(self.as_HttpComponentView_mut()) }
    fn getDoc(&mut self) -> Option<Rc<RefCell<Document>>> { HttpComponentView::getDoc(self.as_HttpComponentView_mut()) }
    fn view(&mut self) -> String { HttpComponentView::view(self.as_HttpComponentView_mut()) }
}
impl HttpComponentViewDyn for HttpComponentView {
    fn as_HttpComponentView(&self) -> &HttpComponentView { self }
    fn as_HttpComponentView_mut(&mut self) -> &mut HttpComponentView { self }
}

impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for HttpComponentView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base }
}


