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
// package views.business.context.modules.storefront.customizer.includes.style.theme
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::category::CategoryFactoryInterface::CategoryFactoryInterface;
use crate::org::allbinary::business::category::CategoryInterface::CategoryInterface;
use crate::org::allbinary::business::category::store::theme::StoreThemeCategoryFactory::StoreThemeCategoryFactory;
use crate::org::allbinary::business::category::store::theme::StoreThemeCategoryInterface::StoreThemeCategoryInterface;
use crate::org::allbinary::business::category::store::theme::StoreThemeCategoryPathValidationView::StoreThemeCategoryPathValidationView;
use crate::org::allbinary::data::tree::category::CategoryLoaderFactory::CategoryLoaderFactory;
use crate::org::allbinary::data::tree::category::CategoryLoaderInterface::CategoryLoaderInterface;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::ValidationComponentInterface::ValidationComponentInterface;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ThemeCustomizerView::ThemeCustomizerView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EditThemeValidationView {
    pub base: ThemeCustomizerView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

// impl ValidationComponentInterface for EditThemeValidationView with the methods of the trait

impl EditThemeValidationView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: ThemeCustomizerView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut isValid: Option<bool> = true;
            let mut categoryFactoryInterface: Option<Rc<RefCell<dyn CategoryFactoryInterface>>> = StoreThemeCategoryFactory::new(self.getTransformInfoInterface()).map(|reference| reference as Rc<RefCell<dyn CategoryFactoryInterface>>);
            let mut categoryLoaderInterface: Option<Rc<RefCell<dyn CategoryLoaderInterface>>> = CategoryLoaderFactory::getInstance(categoryFactoryInterface.clone());
            let mut rootStoreThemeCategoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>> = (jcall!(categoryFactoryInterface).getRootInstance()).clone() /* as StoreThemeCategoryInterface */;
            let mut storeThemeCategoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>> = (jcall!(categoryLoaderInterface).getAll((rootStoreThemeCategoryInterface).clone().map(|reference| reference as Rc<RefCell<dyn CategoryInterface>>))).clone() /* as StoreThemeCategoryInterface */;
            self.validationInterface = StoreThemeCategoryPathValidationView::new((storeThemeCategoryInterface).clone() /* as StoreThemeCategoryInterface */).map(|reference| reference as Rc<RefCell<dyn ValidationInterface>>);
            isValid = jcall!(self.validationInterface).isValid();
            let mut domNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = (self.validationInterface).clone() /* as DomNodeInterface */;
            let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create();
            jcall!(document).appendChild(jcall!(domNodeInterface).toXmlNode(document.clone()));
            let mut documentString: String = XmlDocumentHelper::toString(document.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Setting Data: "), documentString), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone());
            }
            jcall!(self.getTransformInfoInterface()).setData(documentString.clone());
            return isValid.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn validationInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(jcall!(self.validationInterface).validationInfo());
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Getting Validation Info");
            }
        }
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
}
impl std::ops::Deref for EditThemeValidationView {
    type Target = ThemeCustomizerView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for EditThemeValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::business::context::modules::storefront::customizer::includes::style::theme::ThemeCustomizerView::ThemeCustomizerViewDyn for EditThemeValidationView {
    fn as_ThemeCustomizerView(&self) -> &crate::views::business::context::modules::storefront::customizer::includes::style::theme::ThemeCustomizerView::ThemeCustomizerView { &self.base }
    fn as_ThemeCustomizerView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::customizer::includes::style::theme::ThemeCustomizerView::ThemeCustomizerView { &mut self.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for EditThemeValidationView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base.base }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for EditThemeValidationView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for EditThemeValidationView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base.base }
    fn view(&mut self) -> String { crate::views::business::context::modules::storefront::customizer::includes::style::theme::ThemeCustomizerView::ThemeCustomizerView::view(crate::views::business::context::modules::storefront::customizer::includes::style::theme::ThemeCustomizerView::ThemeCustomizerViewDyn::as_ThemeCustomizerView_mut(self)) }
}


