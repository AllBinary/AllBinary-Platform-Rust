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
// package org.allbinary.android.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::AndroidResources::AndroidResources;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::init::Init::Init;
use crate::org::allbinary::input::motion::button::CommonButtons::CommonButtons;
use crate::org::allbinary::input::motion::button::TouchButtonBlankResource::TouchButtonBlankResource;
use crate::org::allbinary::input::motion::button::TouchButtonDownResource::TouchButtonDownResource;
use crate::org::allbinary::input::motion::button::TouchButtonGenericActionResource::TouchButtonGenericActionResource;
use crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResource;
use crate::org::allbinary::input::motion::button::TouchButtonStartResource::TouchButtonStartResource;
use crate::org::allbinary::input::motion::button::TouchButtonStrafeLeftResource::TouchButtonStrafeLeftResource;
use crate::org::allbinary::input::motion::button::TouchButtonStrafeRightResource::TouchButtonStrafeRightResource;
use crate::org::allbinary::input::motion::button::TouchButtonTurnLeftResource::TouchButtonTurnLeftResource;
use crate::org::allbinary::input::motion::button::TouchButtonTurnRightResource::TouchButtonTurnRightResource;
use crate::org::allbinary::input::motion::button::TouchButtonUpResource::TouchButtonUpResource;
use crate::org::allbinary::logic::system::os::OperatingSystemFactory::OperatingSystemFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonAndroidResources {
    pub base: Init,
}


impl TouchButtonAndroidResources {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Init::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        let mut resourceUtil: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::getInstance();
        let mut androidResources: Option<Rc<RefCell<AndroidResources>>> = AndroidResources::getInstance();
        let mut RESOURCE_STRING_ARRAY: Vec<Option<Rc<RefCell<TouchButtonResource>>>> = vec![TouchButtonBlankResource::getInstance(), TouchButtonGenericActionResource::getInstance(), TouchButtonStartResource::getInstance(), TouchButtonUpResource::getInstance(), TouchButtonDownResource::getInstance(), TouchButtonTurnLeftResource::getInstance(), TouchButtonTurnRightResource::getInstance(), TouchButtonStrafeLeftResource::getInstance(), TouchButtonStrafeRightResource::getInstance()];
        let mut RESOURCE_ARRAY: Vec<i32> = vec![0; ((RESOURCE_STRING_ARRAY.len() as i32)) as usize];
        if jcall!(CommonButtons::getInstance()).STANDARD_BUTTON_SIZE == 128 {
            RESOURCE_ARRAY[(0) as usize] = jcall!(jcall!(androidResources).raw).touch_button_128_by_128.clone();
            RESOURCE_ARRAY[(1) as usize] = jcall!(jcall!(androidResources).raw).touch_button_generic_action_128_by_128.clone();
            RESOURCE_ARRAY[(2) as usize] = jcall!(jcall!(androidResources).raw).touch_button_start_128_by_128.clone();
            RESOURCE_ARRAY[(3) as usize] = jcall!(jcall!(androidResources).raw).touch_button_up_arrow_128_by_128.clone();
            RESOURCE_ARRAY[(4) as usize] = jcall!(jcall!(androidResources).raw).touch_button_down_arrow_128_by_128.clone();
            RESOURCE_ARRAY[(5) as usize] = jcall!(jcall!(androidResources).raw).touch_button_turn_left_arrow_128_by_128.clone();
            RESOURCE_ARRAY[(6) as usize] = jcall!(jcall!(androidResources).raw).touch_button_turn_right_arrow_128_by_128.clone();
            RESOURCE_ARRAY[(7) as usize] = jcall!(jcall!(androidResources).raw).touch_button_strafe_left_arrow_128_by_128.clone();
            RESOURCE_ARRAY[(8) as usize] = jcall!(jcall!(androidResources).raw).touch_button_strafe_right_arrow_128_by_128.clone();
            if jcall!(jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance()).isOverScan() {
                jcall!(resourceUtil).addResource(jcall!(RESOURCE_STRING_ARRAY[(2) as usize]).HINT.clone(), (jcall!(jcall!(androidResources).raw).ouya_o.clone()).to_string());
            }
        } else {
            RESOURCE_ARRAY[(0) as usize] = jcall!(jcall!(androidResources).raw).touch_button_64_by_64.clone();
            RESOURCE_ARRAY[(1) as usize] = jcall!(jcall!(androidResources).raw).touch_button_generic_action_64_by_64.clone();
            RESOURCE_ARRAY[(2) as usize] = jcall!(jcall!(androidResources).raw).touch_button_start_64_by_64.clone();
            RESOURCE_ARRAY[(3) as usize] = jcall!(jcall!(androidResources).raw).touch_button_up_arrow_64_by_64.clone();
            RESOURCE_ARRAY[(4) as usize] = jcall!(jcall!(androidResources).raw).touch_button_down_arrow_64_by_64.clone();
            RESOURCE_ARRAY[(5) as usize] = jcall!(jcall!(androidResources).raw).touch_button_turn_left_arrow_64_by_64.clone();
            RESOURCE_ARRAY[(6) as usize] = jcall!(jcall!(androidResources).raw).touch_button_turn_right_arrow_64_by_64.clone();
            RESOURCE_ARRAY[(7) as usize] = jcall!(jcall!(androidResources).raw).touch_button_strafe_left_arrow_64_by_64.clone();
            RESOURCE_ARRAY[(8) as usize] = jcall!(jcall!(androidResources).raw).touch_button_strafe_right_arrow_64_by_64.clone();
        }
        for index in (0..(RESOURCE_ARRAY.len() as i32)) {
            jcall!(resourceUtil).addResource(jcall!(RESOURCE_STRING_ARRAY[(index) as usize]).RESOURCE.clone(), (RESOURCE_ARRAY[(index) as usize].clone()).to_string());
        }
    }
}
impl std::ops::Deref for TouchButtonAndroidResources {
    type Target = Init;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TouchButtonAndroidResources {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



