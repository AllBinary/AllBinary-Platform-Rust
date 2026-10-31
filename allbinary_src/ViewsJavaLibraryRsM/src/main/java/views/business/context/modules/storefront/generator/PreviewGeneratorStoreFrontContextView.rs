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
// package views.business.context.modules.storefront.generator
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoData::TransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpInterface::TransformInfoHttpInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfosData::TransformInfosData;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::GeneratorStoreFrontContextView::GeneratorStoreFrontContextView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PreviewGeneratorStoreFrontContextView {
    pub base: GeneratorStoreFrontContextView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl PreviewGeneratorStoreFrontContextView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: GeneratorStoreFrontContextView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = (self.getTransformInfoInterface()).clone() /* as TransformInfoHttpInterface */;
            jcall!(jcall!(httpTransformInfoInterface).getPropertiesHashMap()).put(jcall!(TransformInfoData::getInstance()).PARTIAL.clone(), format!("{}{}", jcall!(CommonSeps::getInstance()).SPACE, jcall!(TransformInfosData::getInstance()).PREVIEW));
            return { let java_arguments = (jcall!(TransformInfosData::getInstance()).PREVIEW.clone(),); self.view1(java_arguments.0) };
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
impl std::ops::Deref for PreviewGeneratorStoreFrontContextView {
    type Target = GeneratorStoreFrontContextView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PreviewGeneratorStoreFrontContextView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::business::context::modules::storefront::generator::GeneratorStoreFrontContextView::GeneratorStoreFrontContextViewDyn for PreviewGeneratorStoreFrontContextView {
    fn as_GeneratorStoreFrontContextView(&self) -> &crate::views::business::context::modules::storefront::generator::GeneratorStoreFrontContextView::GeneratorStoreFrontContextView { &self.base }
    fn as_GeneratorStoreFrontContextView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::generator::GeneratorStoreFrontContextView::GeneratorStoreFrontContextView { &mut self.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for PreviewGeneratorStoreFrontContextView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base.base }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for PreviewGeneratorStoreFrontContextView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for PreviewGeneratorStoreFrontContextView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}


