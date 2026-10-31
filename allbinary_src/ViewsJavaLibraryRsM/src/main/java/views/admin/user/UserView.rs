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
// package views.admin.user
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::UserDomNode::UserDomNode;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::views::business::context::HttpContextView::HttpContextView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserView {
    pub base: HttpContextView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub user: Option<Rc<RefCell<dyn UserInterface>>>,
}


impl UserView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpContextView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            user: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn addDomNodeInterfaces(&mut self) {
        self.addDomNodeInterface((UserDomNode::new(self.user.clone())).clone() /* as DomNodeInterface */);
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
impl std::ops::Deref for UserView {
    type Target = HttpContextView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UserView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait UserViewDyn: crate::views::business::context::HttpContextView::HttpContextViewDyn {
    fn as_UserView(&self) -> &UserView;
    fn as_UserView_mut(&mut self) -> &mut UserView;
    fn addDomNodeInterfaces(&mut self) { UserView::addDomNodeInterfaces(self.as_UserView_mut()) }
}
impl UserViewDyn for UserView {
    fn as_UserView(&self) -> &UserView { self }
    fn as_UserView_mut(&mut self) -> &mut UserView { self }
}

impl crate::views::business::context::HttpContextView::HttpContextViewDyn for UserView {
    fn as_HttpContextView(&self) -> &crate::views::business::context::HttpContextView::HttpContextView { &self.base }
    fn as_HttpContextView_mut(&mut self) -> &mut crate::views::business::context::HttpContextView::HttpContextView { &mut self.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for UserView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for UserView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base }
}


