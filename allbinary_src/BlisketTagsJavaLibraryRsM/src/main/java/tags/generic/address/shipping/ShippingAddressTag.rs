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
// package tags.generic.address.shipping
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::servlet::jsp::JspTagException::JspTagException;
use crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData;
use crate::org::allbinary::logic::communication::http::request::AbResponseHandler::AbResponseHandler;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::tags::StoreValidationTransformTag::StoreValidationTransformTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ShippingAddressTag {
    pub base: StoreValidationTransformTag,
    pub value: String,
}


impl ShippingAddressTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: StoreValidationTransformTag::new_struct(),
            value: String::new(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws JspTagException
    pub fn doStartTag(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.getCommand().is_some() {
                if jcall!(self.getCommand()).compareTo(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).VIEW.clone()) == 0 {
                    self.setName(String::from("Basic Shipping Address View"));
                    self.setObjectFile(String::from("views.generic.address.shipping.ViewShippingAddressesView"));
                } else if jcall!(self.getCommand()).compareTo(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).EDIT.clone()) == 0 {
                    self.setName(String::from("Basic Shipping Address View"));
                    self.setObjectFile(String::from("views.generic.address.shipping.EditShippingAddressesView"));
                } else if jcall!(self.getCommand()).compareTo(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).GLOBALS2).NEW.clone()) == 0 {
                    self.setName(String::from("Basic Shipping Address View"));
                    self.setObjectFile(String::from("views.generic.address.shipping.NewShippingAddressesView"));
                } else if jcall!(self.getCommand()).compareTo(jcall!(self.commonStrings).INSERT.clone()) == 0 {
                    self.setName(String::from("Basic Shipping Address View"));
                    self.setObjectFile(String::from("views.generic.address.shipping.AddShippingAddressesView"));
                } else if jcall!(self.getCommand()).compareTo(jcall!(self.commonStrings).DELETE.clone()) == 0 {
                    self.setName(String::from("Basic Shipping Address View"));
                    self.setObjectFile(String::from("views.generic.address.shipping.DeleteShippingAddressesView"));
                } else if jcall!(self.getCommand()).compareTo(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_SELECT.with(Clone::clone).clone()) == 0 {
                    self.setName(String::from("Basic Shipping Address View"));
                    self.setObjectFile(String::from("views.generic.address.shipping.SelectShippingAddressesView"));
                } else if jcall!(self.getCommand()).compareTo(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_SETTOBILLINGADDRESS.with(Clone::clone).clone()) == 0 {
                    self.setName(String::from("Basic Shipping Address View"));
                    self.setObjectFile(String::from("views.generic.address.shipping.SetShippingAddressesView"));
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("No Such View Command: "), self.getCommand()))));
                }
                return self.base.doStartTag();
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Command Null"))));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            AbResponseHandler::sendJspTagLicensingRedirect(self.pageContext.clone(), e.clone());
            return SKIP_BODY.clone();
            }
        }
    }
}
impl std::ops::Deref for ShippingAddressTag {
    type Target = StoreValidationTransformTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ShippingAddressTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::tags::ModifyTag::ModifyTagDyn for ShippingAddressTag {
    fn as_ModifyTag(&self) -> &crate::tags::ModifyTag::ModifyTag { &self.base.base.base.base.base }
    fn as_ModifyTag_mut(&mut self) -> &mut crate::tags::ModifyTag::ModifyTag { &mut self.base.base.base.base.base }
    fn doStartTag(&mut self) -> i32 { Self::doStartTag(self) }
}
impl crate::tags::StoreValidationTransformTag::StoreValidationTransformTagDyn for ShippingAddressTag {
    fn as_StoreValidationTransformTag(&self) -> &crate::tags::StoreValidationTransformTag::StoreValidationTransformTag { &self.base }
    fn as_StoreValidationTransformTag_mut(&mut self) -> &mut crate::tags::StoreValidationTransformTag::StoreValidationTransformTag { &mut self.base }
}
impl crate::tags::TransformTag::TransformTagDyn for ShippingAddressTag {
    fn as_TransformTag(&self) -> &crate::tags::TransformTag::TransformTag { &self.base.base.base }
    fn as_TransformTag_mut(&mut self) -> &mut crate::tags::TransformTag::TransformTag { &mut self.base.base.base }
}
impl crate::tags::ValidationTransformTag::ValidationTransformTagDyn for ShippingAddressTag {
    fn as_ValidationTransformTag(&self) -> &crate::tags::ValidationTransformTag::ValidationTransformTag { &self.base.base }
    fn as_ValidationTransformTag_mut(&mut self) -> &mut crate::tags::ValidationTransformTag::ValidationTransformTag { &mut self.base.base }
}
impl crate::tags::transform::info::TransformInfoTag::TransformInfoTagDyn for ShippingAddressTag {
    fn as_TransformInfoTag(&self) -> &crate::tags::transform::info::TransformInfoTag::TransformInfoTag { &self.base.base.base.base }
    fn as_TransformInfoTag_mut(&mut self) -> &mut crate::tags::transform::info::TransformInfoTag::TransformInfoTag { &mut self.base.base.base.base }
    fn setStoreName(&mut self, p1: String) { crate::tags::StoreValidationTransformTag::StoreValidationTransformTag::setStoreName(crate::tags::StoreValidationTransformTag::StoreValidationTransformTagDyn::as_StoreValidationTransformTag_mut(self), p1) }
}


