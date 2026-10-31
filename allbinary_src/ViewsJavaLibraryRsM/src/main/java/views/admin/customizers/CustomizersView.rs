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
// package views.admin.customizers
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::util::Arrays::Arrays;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tables::transform::info::TransformInfoEntityBuilder::TransformInfoEntityBuilder;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::globals::GLOBALS2::GLOBALS2;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::sort::StringComparator::StringComparator;
use crate::org::allbinary::logic::visual::transform::info::CustomizerTransformInfoData::CustomizerTransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::template::customizer::TransformTemplateCustomizerData::TransformTemplateCustomizerData;
use crate::org::allbinary::logic::visual::transform::template::customizer::TransformTemplateCustomizersData::TransformTemplateCustomizersData;
use crate::org::allbinary::logic::visual::transform::template::customizer::bodies::BodyData::BodyData;
use crate::org::allbinary::logic::visual::transform::template::customizer::widgets::title::TitleData::TitleData;
use crate::org::allbinary::logic::visual::transform::template::util::TransformTemplateCustomizerUtil::TransformTemplateCustomizerUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomizersView {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub customizersVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl DomNodeInterface for CustomizersView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl CustomizersView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            commonStrings: None,
            customizersVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        // super(...) is in the struct literal above
        this.customizersVector = jcall!(TransformInfoEntityBuilder::getInstance()).getNames(jcall!(this.getWeblisketSession()).getStoreName());
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(unsafe { crate::org::allbinary::logic::visual::transform::template::customizer::TransformTemplateCustomizersData::TransformTemplateCustomizersData_NAME.clone() }.clone());
            let mut unsortedCustomizerViewVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut size: i32 = jcall!(self.customizersVector).size();
            for index in (0..size) {
                let mut viewName: String = (jcall!(self.customizersVector).get(index.clone())).clone() /* as String */;
                if viewName.find(&crate::org::allbinary::logic::visual::transform::info::CustomizerTransformInfoData::CustomizerTransformInfoData_NAME.with(Clone::clone).clone()).map(|index| index as i32).unwrap_or(-1) > 0 && viewName.find(&crate::org::allbinary::globals::GLOBALS2::GLOBALS2_EDIT.with(Clone::clone).clone()).map(|index| index as i32).unwrap_or(-1) > 0 && viewName.find(&jcall!(BodyData::getInstance()).VIEWNAMEKEY.clone()).map(|index| index as i32).unwrap_or(-1) > 0 && viewName.find(&jcall!(TitleData::getInstance()).VIEWNAMEKEY.clone()).map(|index| index as i32).unwrap_or(-1) > 0 {
                    jcall!(unsortedCustomizerViewVector).add(crate::java_into_object(viewName.clone()));
                }
            }
            let mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = (jcall!(unsortedCustomizerViewVector).toArray()).clone() /* as  */;
            Arrays::sort(objectArray.clone(), StringComparator::new());
            for index in (0..(objectArray.len() as i32)) {
                let mut viewName: String = (objectArray[(index) as usize]).clone() /* as String */;
                let mut viewNameNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes4(document.clone(), unsafe { crate::org::allbinary::logic::visual::transform::template::customizer::TransformTemplateCustomizerData::TransformTemplateCustomizerData_NAME.clone() }.clone(), viewName.clone(), jcall!(TransformTemplateCustomizerUtil::getInstance()).getPageNameHack(viewName.clone(), jcall!(self.getWeblisketSession()).getStoreName()));
                jcall!(node).appendChild(viewNameNode.clone());
            }
            return node.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).XSLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn addDomNodeInterfaces(&mut self) {
        self.addDomNodeInterface((self).clone() /* as DomNodeInterface */);
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.addDomNodeInterfaces();
            return self.base.view();
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
impl std::ops::Deref for CustomizersView {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CustomizersView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CustomizersViewDyn: crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn {
    fn as_CustomizersView(&self) -> &CustomizersView;
    fn as_CustomizersView_mut(&mut self) -> &mut CustomizersView;
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { CustomizersView::toXmlNode(self.as_CustomizersView_mut(), document) }
    fn addDomNodeInterfaces(&mut self) { CustomizersView::addDomNodeInterfaces(self.as_CustomizersView_mut()) }
}
impl CustomizersViewDyn for CustomizersView {
    fn as_CustomizersView(&self) -> &CustomizersView { self }
    fn as_CustomizersView_mut(&mut self) -> &mut CustomizersView { self }
}

impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for CustomizersView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for CustomizersView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for CustomizersView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}


