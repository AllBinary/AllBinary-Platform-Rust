// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.android.activity
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::app::Activity::Activity;
use crate::android::view::MotionEvent::MotionEvent;
use crate::android::view::View::View;
use crate::org::allbinary::android::GenericMotionEventVerifyInterface::GenericMotionEventVerifyInterface;
use crate::org::allbinary::game::configuration::event::GameFeatureEvent::GameFeatureEvent;
use crate::org::allbinary::game::configuration::event::GameFeatureListenerInterface::GameFeatureListenerInterface;
use crate::org::allbinary::game::configuration::event::GameInitializedEvent::GameInitializedEvent;
use crate::org::allbinary::game::configuration::event::GameInitializedListenerInterface::GameInitializedListenerInterface;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use super::SimpleProgressActivityInterface::SimpleProgressActivityInterface;
use super::KeyInputViewCompositeInterface::KeyInputViewCompositeInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MidletActivityBase {
    pub base: Activity,
}

impl GameFeatureListenerInterface for MidletActivityBase {
    fn onGameFeatureChange(&mut self, mut gameFeatureEvent: Option<Rc<RefCell<GameFeatureEvent>>>) { Self::onGameFeatureChange(self, gameFeatureEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for MidletActivityBase {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}
impl GameInitializedListenerInterface for MidletActivityBase {
    fn onGameInitialized(&mut self, mut gameInitializedEvent: Option<Rc<RefCell<GameInitializedEvent>>>) { Self::onGameInitialized(self, gameInitializedEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for MidletActivityBase {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}
// impl SimpleProgressActivityInterface for MidletActivityBase with the methods of the trait
// impl KeyInputViewCompositeInterface for MidletActivityBase with the methods of the trait
impl GenericMotionEventVerifyInterface for MidletActivityBase {
    fn onGenericMotionEvent(&mut self, mut event: Option<Rc<RefCell<MotionEvent>>>) -> bool { Self::onGenericMotionEvent(self, event) }
}

impl MidletActivityBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Activity::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn onGameFeatureChange(&mut self, mut gameFeatureEvent: Option<Rc<RefCell<GameFeatureEvent>>>) {
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
    }
    

    pub fn onGameInitialized(&mut self, mut gameInitializedEvent: Option<Rc<RefCell<GameInitializedEvent>>>) {
    }
    

    // throws Exception
    pub fn onSetProgress(&mut self, mut value: i32, mut text: String) {
    }
    

    // throws Exception
    pub fn onDismissProgress(&mut self, mut animationSequence: Vec<i32>) {
    }
    

    // throws Exception
    pub fn onShowProgress(&mut self, mut indeterminate: bool, mut animationSequence: Vec<i32>) {
    }
    

    pub fn getRootView(&mut self) -> Option<Rc<RefCell<View>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getView(&mut self) -> Option<Rc<RefCell<View>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setKeyInputView(&mut self, mut keyInputView: Option<Rc<RefCell<View>>>) {
    }
    

    pub fn onGenericMotionEvent(&mut self, mut event: Option<Rc<RefCell<MotionEvent>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
impl std::ops::Deref for MidletActivityBase {
    type Target = Activity;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MidletActivityBase {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait MidletActivityBaseDyn {
    fn as_MidletActivityBase(&self) -> &MidletActivityBase;
    fn as_MidletActivityBase_mut(&mut self) -> &mut MidletActivityBase;
    fn onGameFeatureChange(&mut self, mut gameFeatureEvent: Option<Rc<RefCell<GameFeatureEvent>>>) { MidletActivityBase::onGameFeatureChange(self.as_MidletActivityBase_mut(), gameFeatureEvent) }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { MidletActivityBase::onEvent(self.as_MidletActivityBase_mut(), eventObject) }
    fn onGameInitialized(&mut self, mut gameInitializedEvent: Option<Rc<RefCell<GameInitializedEvent>>>) { MidletActivityBase::onGameInitialized(self.as_MidletActivityBase_mut(), gameInitializedEvent) }
    fn onSetProgress(&mut self, mut value: i32, mut text: String) { MidletActivityBase::onSetProgress(self.as_MidletActivityBase_mut(), value, text) }
    fn onDismissProgress(&mut self, mut animationSequence: Vec<i32>) { MidletActivityBase::onDismissProgress(self.as_MidletActivityBase_mut(), animationSequence) }
    fn onShowProgress(&mut self, mut indeterminate: bool, mut animationSequence: Vec<i32>) { MidletActivityBase::onShowProgress(self.as_MidletActivityBase_mut(), indeterminate, animationSequence) }
    fn getRootView(&mut self) -> Option<Rc<RefCell<View>>> { MidletActivityBase::getRootView(self.as_MidletActivityBase_mut()) }
    fn getView(&mut self) -> Option<Rc<RefCell<View>>> { MidletActivityBase::getView(self.as_MidletActivityBase_mut()) }
    fn setKeyInputView(&mut self, mut keyInputView: Option<Rc<RefCell<View>>>) { MidletActivityBase::setKeyInputView(self.as_MidletActivityBase_mut(), keyInputView) }
    fn onGenericMotionEvent(&mut self, mut event: Option<Rc<RefCell<MotionEvent>>>) -> bool { MidletActivityBase::onGenericMotionEvent(self.as_MidletActivityBase_mut(), event) }
}
impl MidletActivityBaseDyn for MidletActivityBase {
    fn as_MidletActivityBase(&self) -> &MidletActivityBase { self }
    fn as_MidletActivityBase_mut(&mut self) -> &mut MidletActivityBase { self }
}



