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
// package org.allbinary.android.input.api3
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::android::app::Activity::Activity;
use crate::org::allbinary::android::input::VirtualKeyboard::VirtualKeyboard;
use crate::org::allbinary::input::event::VirtualKeyboardEvent::VirtualKeyboardEvent;
use crate::org::allbinary::input::event::VirtualKeyboardEventHandler::VirtualKeyboardEventHandler;
use crate::org::allbinary::input::event::VirtualKeyboardEventListenerInterface::VirtualKeyboardEventListenerInterface;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use super::ShowVirtualKeyboardRunnable::ShowVirtualKeyboardRunnable;
use super::HideVirtualKeyboardRunnable::HideVirtualKeyboardRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VirtualKeyboardAPI3 {
    pub base: VirtualKeyboard,
    pub virtualKeyboard: bool,
    pub activity: Option<Rc<RefCell<Activity>>>,
    pub showVirtualKeyboardRunnable: Option<Rc<RefCell<dyn Runnable>>>,
    pub hideVirtualKeyboardRunnable: Option<Rc<RefCell<dyn Runnable>>>,
}

pub static mut VirtualKeyboardAPI3_MAX_API: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| i32::MAX.clone());
pub static mut VirtualKeyboardAPI3_MIN_API: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 3);
impl VirtualKeyboardEventListenerInterface for VirtualKeyboardAPI3 {
    fn onVirtualKeyboardEvent(&mut self, mut virtualKeyboardEvent: Option<Rc<RefCell<VirtualKeyboardEvent>>>) { Self::onVirtualKeyboardEvent(self, virtualKeyboardEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for VirtualKeyboardAPI3 {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl VirtualKeyboardAPI3 {
    

    pub fn new_struct(mut activity: Option<Rc<RefCell<Activity>>>) -> Self {
        let mut this = Self {
            base: VirtualKeyboard::new_struct(),
            virtualKeyboard: false,
            activity: None,
            showVirtualKeyboardRunnable: None,
            hideVirtualKeyboardRunnable: None,
        };
        // super(...) is in the struct literal above
        this.activity = activity.clone();
        this.showVirtualKeyboardRunnable = ShowVirtualKeyboardRunnable::new(this.activity.clone()).map(|reference| reference as Rc<RefCell<dyn Runnable>>);
        this.hideVirtualKeyboardRunnable = HideVirtualKeyboardRunnable::new(this.activity.clone()).map(|reference| reference as Rc<RefCell<dyn Runnable>>);
        let mut virtualKeyboardEventHandler: Option<Rc<RefCell<VirtualKeyboardEventHandler>>> = VirtualKeyboardEventHandler::getInstance();
        jcall!(virtualKeyboardEventHandler).removeAllListeners();
        this
    }

    pub fn new(mut activity: Option<Rc<RefCell<Activity>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(activity))));
            jcall!(virtualKeyboardEventHandler).addListenerInterface((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onVirtualKeyboardEvent(&mut self, mut virtualKeyboardEvent: Option<Rc<RefCell<VirtualKeyboardEvent>>>) {
        let mut isShow: Option<bool> = { let java_object = jcall!(virtualKeyboardEvent).getSource(); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<bool>().copied().or_else(|| object.downcast_ref::<Option<bool>>().copied().flatten()).expect("ClassCastException") }) };
        if isShow.unwrap() {
            self.showVirtualKeyboard();
        } else {
            self.hideVirtualKeyboard();
        }
    }
    

    pub fn forceHide(&mut self) {
        self.hideVirtualKeyboard();
    }
    

    pub fn hide(&mut self) {
        if self.virtualKeyboard {
            self.forceHide();
        }
    }
    

    pub fn showVirtualKeyboard(&mut self) {
        jcall!(self.activity).runOnUiThread(self.showVirtualKeyboardRunnable.clone());
        self.virtualKeyboard = true;
    }
    

    pub fn hideVirtualKeyboard(&mut self) {
        jcall!(self.activity).runOnUiThread(self.hideVirtualKeyboardRunnable.clone());
        self.virtualKeyboard = false;
    }
}
impl std::ops::Deref for VirtualKeyboardAPI3 {
    type Target = VirtualKeyboard;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for VirtualKeyboardAPI3 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::android::input::VirtualKeyboard::VirtualKeyboardDyn for VirtualKeyboardAPI3 {
    fn as_VirtualKeyboard(&self) -> &crate::org::allbinary::android::input::VirtualKeyboard::VirtualKeyboard { &self.base }
    fn as_VirtualKeyboard_mut(&mut self) -> &mut crate::org::allbinary::android::input::VirtualKeyboard::VirtualKeyboard { &mut self.base }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
    fn onVirtualKeyboardEvent(&mut self, mut virtualKeyboardEvent: Option<Rc<RefCell<VirtualKeyboardEvent>>>) { Self::onVirtualKeyboardEvent(self, virtualKeyboardEvent) }
    fn forceHide(&mut self) { Self::forceHide(self) }
    fn hide(&mut self) { Self::hide(self) }
}


