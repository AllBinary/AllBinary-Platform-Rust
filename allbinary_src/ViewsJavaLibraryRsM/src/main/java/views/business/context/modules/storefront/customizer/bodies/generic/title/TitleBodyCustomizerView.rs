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
// package views.business.context.modules.storefront.customizer.bodies.generic.title
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::page::PageData::PageData;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::StoreTransformer::StoreTransformer;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::template::customizer::bodies::title::TitleBodyValidation::TitleBodyValidation;
use crate::org::allbinary::logic::visual::transform::template::util::TransformTemplateCustomizerUtil::TransformTemplateCustomizerUtil;
use crate::org::apache::commons::lang3::StringEscapeUtils::StringEscapeUtils;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TitleBodyCustomizerView {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub titleBody: Option<Rc<RefCell<TitleBodyValidation>>>,
}

thread_local! { pub static TitleBodyCustomizerView_NAME: String = String::from("None"); }
impl DomNodeInterface for TitleBodyCustomizerView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl TitleBodyCustomizerView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            titleBody: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    // throws Exception
    pub fn addDomNodeInterfaces(&mut self) {
        self.addDomNodeInterface((self.titleBody).clone() /* as DomNodeInterface */);
        self.addDomNodeInterface((self).clone() /* as DomNodeInterface */);
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut pageName: String = jcall!(TransformTemplateCustomizerUtil::getInstance()).getPageNameHack(jcall!(self.getTransformInfoInterface()).getName(), jcall!(self.getWeblisketSession()).getStoreName());
        if jcall!(StringValidationUtil::getInstance()).isEmpty(pageName.clone()) {
            pageName = TitleBodyCustomizerView_NAME.with(Clone::clone).clone();
        }
        return ModDomHelper::createNameValueNodes(document.clone(), jcall!(PageData::getInstance()).NAME.clone(), pageName.clone());
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = XmlDocumentHelper::toString(self.getDoc());
            let mut result: String = jcall!(StoreTransformer::new(self.abeClientInformation.clone(), self.getTransformInfoInterface())).translate(success.clone());
            return StringEscapeUtils::unescapeHtml3(result.clone());
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
impl std::ops::Deref for TitleBodyCustomizerView {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TitleBodyCustomizerView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait TitleBodyCustomizerViewDyn: crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn {
    fn as_TitleBodyCustomizerView(&self) -> &TitleBodyCustomizerView;
    fn as_TitleBodyCustomizerView_mut(&mut self) -> &mut TitleBodyCustomizerView;
    fn addDomNodeInterfaces(&mut self) { TitleBodyCustomizerView::addDomNodeInterfaces(self.as_TitleBodyCustomizerView_mut()) }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { TitleBodyCustomizerView::toXmlNode(self.as_TitleBodyCustomizerView_mut(), document) }
}
impl TitleBodyCustomizerViewDyn for TitleBodyCustomizerView {
    fn as_TitleBodyCustomizerView(&self) -> &TitleBodyCustomizerView { self }
    fn as_TitleBodyCustomizerView_mut(&mut self) -> &mut TitleBodyCustomizerView { self }
}

impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for TitleBodyCustomizerView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for TitleBodyCustomizerView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for TitleBodyCustomizerView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}


