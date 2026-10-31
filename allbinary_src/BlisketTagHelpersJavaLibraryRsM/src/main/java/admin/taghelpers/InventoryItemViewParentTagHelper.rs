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
// package admin.taghelpers
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::http::request::RequestMapInterface::RequestMapInterface;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::transform::TransformInterface::TransformInterface;
use crate::taghelpers::ValidationViewHelper::ValidationViewHelper;
use crate::tags::HelperTag::HelperTag;
use crate::views::admin::inventory::InventoryItemView::InventoryItemView;
use crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InventoryItemViewParentTagHelper {
}

thread_local! { pub static InventoryItemViewParentTagHelper_instance: Option<Rc<RefCell<InventoryItemViewParentTagHelper>>> = InventoryItemViewParentTagHelper::new(); }

impl InventoryItemViewParentTagHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<InventoryItemViewParentTagHelper>>> {
        return InventoryItemViewParentTagHelper_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getInventoryItemView(&mut self, mut inventoryTag: Option<Rc<RefCell<dyn crate::tags::HelperTag::HelperTagDyn>>>) -> Option<Rc<RefCell<dyn crate::views::admin::inventory::InventoryItemView::InventoryItemViewDyn>>> {
        if inventoryTag.is_some() {
            { let java_arguments = (jcall!(inventoryTag).getHelper(),); self.checkForValidationViewHelper(java_arguments.0) };
            let mut validationViewHelper: Option<Rc<RefCell<ValidationViewHelper>>> = { let java_object = (jcall!(inventoryTag).getHelper()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<ValidationViewHelper>>>>().expect("ClassCastException").clone()) };
            let mut viewObject: Option<Rc<RefCell<dyn TransformInterface>>> = jcall!(validationViewHelper).getViewObject();
            if jcall!(viewObject).getTypeId() != unsafe { crate::views::admin::inventory::InventoryItemView::InventoryItemView_TYPE_ID.clone() } {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Must have "));
                jcall!(stringBuffer).append(String::from("InventoryItemView and not: "));
                jcall!(stringBuffer).append(std::any::type_name_of_val(&jcall!(inventoryTag).getHelper()).to_string());
                panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(stringBuffer).toString())));
            }
            return (viewObject).clone() /* as InventoryItemView */;
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Must have parent tag."))));
        }
    }
    

    // throws Exception
    pub fn getDownloadableInventoryItemView(&mut self, mut inventoryTag: Option<Rc<RefCell<dyn crate::tags::HelperTag::HelperTagDyn>>>) -> Option<Rc<RefCell<dyn crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemViewDyn>>> {
        if inventoryTag.is_some() {
            { let java_arguments = (jcall!(inventoryTag).getHelper(),); self.checkForValidationViewHelper(java_arguments.0) };
            let mut validationViewHelper: Option<Rc<RefCell<ValidationViewHelper>>> = { let java_object = (jcall!(inventoryTag).getHelper()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<ValidationViewHelper>>>>().expect("ClassCastException").clone()) };
            let mut viewObject: Option<Rc<RefCell<dyn TransformInterface>>> = jcall!(validationViewHelper).getViewObject();
            if jcall!(viewObject).getTypeId() != unsafe { crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemView_TYPE_ID.clone() } {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Must have "));
                jcall!(stringBuffer).append(String::from("DownloadableInventoryItemView and not: "));
                jcall!(stringBuffer).append(std::any::type_name_of_val(&jcall!(inventoryTag).getHelper()).to_string());
                panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(stringBuffer).toString())));
            }
            return (viewObject).clone() /* as DownloadableInventoryItemView */;
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Must have parent tag."))));
        }
    }
    

    // throws Exception
    pub fn getRequestMapInterface(&mut self, mut inventoryTag: Option<Rc<RefCell<dyn crate::tags::HelperTag::HelperTagDyn>>>) -> Option<Rc<RefCell<dyn RequestMapInterface>>> {
        if inventoryTag.is_some() {
            { let java_arguments = (jcall!(inventoryTag).getHelper(),); self.checkForValidationViewHelper(java_arguments.0) };
            let mut validationViewHelper: Option<Rc<RefCell<ValidationViewHelper>>> = { let java_object = (jcall!(inventoryTag).getHelper()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<ValidationViewHelper>>>>().expect("ClassCastException").clone()) };
            let mut viewObject: Option<Rc<RefCell<dyn TransformInterface>>> = jcall!(validationViewHelper).getViewObject();
            if jcall!(viewObject).getTypeId() != unsafe { crate::views::admin::inventory::InventoryItemView::InventoryItemView_TYPE_ID.clone() } && jcall!(viewObject).getTypeId() != unsafe { crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemView_TYPE_ID.clone() } {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Must have "));
                jcall!(stringBuffer).append(String::from("specific TYPE_ID and not: "));
                jcall!(stringBuffer).append(std::any::type_name_of_val(&jcall!(inventoryTag).getHelper()).to_string());
                panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(stringBuffer).toString())));
            }
            return panic!("ClassCastException: a Rust dyn TransformInterface cannot be cast to dyn RequestMapInterface");
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Must have parent tag."))));
        }
    }
    

    // throws Exception
    pub fn checkForValidationViewHelper(&mut self, mut helperObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        if !((helperObject).is_some() /* instanceof ValidationViewHelper */) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Must have "));
            jcall!(stringBuffer).append(String::from("ValidationViewHelper"));
            jcall!(stringBuffer).append(String::from("and not: "));
            jcall!(stringBuffer).append(std::any::type_name_of_val(&helperObject).to_string());
            panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(stringBuffer).toString())));
        }
    }
}

