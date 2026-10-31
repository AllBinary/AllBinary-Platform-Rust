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
// package org.allbinary.input.automation.module.generic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfile::GenericProfile;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::SavedCaptureGenericProfileDataWorkerType::SavedCaptureGenericProfileDataWorkerType;
use crate::org::allbinary::input::media::image::capture::CaptureWorkerInterface::CaptureWorkerInterface;
use crate::org::allbinary::input::media::image::capture::ScreenCaptureImagesWorker::ScreenCaptureImagesWorker;
use super::SavedCaptureImagesWorker::SavedCaptureImagesWorker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericProfileCaptureWorkerFactory {
}


impl GenericProfileCaptureWorkerFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(mut genericProfile: Option<Rc<RefCell<GenericProfile>>>) -> Option<Rc<RefCell<dyn CaptureWorkerInterface>>> {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(genericProfile).getGenericProfileDataWorkerTypeVector();
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            let mut genericProfileDataWorkerType: Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerTypeDyn>>> = { let java_object = (jcall!(vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerTypeDyn>>>>().expect("ClassCastException").clone()) };
            if match (&genericProfileDataWorkerType, &unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_SCREEN_CAPTURE.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return ScreenCaptureImagesWorker::new().map(|reference| reference as Rc<RefCell<dyn CaptureWorkerInterface>>);
            } else if match (&genericProfileDataWorkerType, &unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_SAVED_CAPTURE.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return SavedCaptureImagesWorker::new((genericProfileDataWorkerType).clone() /* as SavedCaptureGenericProfileDataWorkerType */).map(|reference| reference as Rc<RefCell<dyn CaptureWorkerInterface>>);
            }
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No CaptureWorkerInterface specified in GenericProfile"))));
    }
}

