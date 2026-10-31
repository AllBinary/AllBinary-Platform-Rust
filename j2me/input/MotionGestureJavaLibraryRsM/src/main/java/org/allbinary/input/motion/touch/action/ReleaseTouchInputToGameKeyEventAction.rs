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
// package org.allbinary.input.motion.touch.action
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::GameKey::GameKey;
use crate::org::allbinary::game::input::GameKeyFactory::GameKeyFactory;
use crate::org::allbinary::game::input::PlatformInputMappingFactory::PlatformInputMappingFactory;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEvent;
use crate::org::allbinary::input::motion::gesture::TouchMotionGestureFactory::TouchMotionGestureFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ReleaseTouchInputToGameKeyEventAction {
    pub base: GameKeyCompleteMotionGestureInputEvent,
    pub NONE: Option<Rc<RefCell<GameKey>>>,
}

pub static mut ReleaseTouchInputToGameKeyEventAction_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
impl crate::org::allbinary::game::input::CompleteMotionGestureInputInterface::CompleteMotionGestureInputInterface for ReleaseTouchInputToGameKeyEventAction {
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for ReleaseTouchInputToGameKeyEventAction {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl ReleaseTouchInputToGameKeyEventAction {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEventDyn>>> {
        if match (&unsafe { ReleaseTouchInputToGameKeyEventAction_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *ReleaseTouchInputToGameKeyEventAction_instance = ReleaseTouchInputToGameKeyEventAction::new() };
        }
        return (unsafe { ReleaseTouchInputToGameKeyEventAction_instance.clone() }).clone() /* as GameKeyCompleteMotionGestureInputEvent */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: GameKeyCompleteMotionGestureInputEvent::new_struct(String::from("Release Action"), jcall!(TouchMotionGestureFactory::getInstance()).RELEASED.clone(), jcall!(jcall!(PlatformInputMappingFactory::getInstance()).getPersistentInputMappingInstance()).getInputMapping()),
            NONE: None,
        };
        this.NONE = jcall!(GameKeyFactory::getInstance()).NONE.clone();
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct())));
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.setGameKey(this.NONE.clone());
            { let java_arguments = (jcall!(GameKeyEventFactory::getInstance()).getInstanceForInput((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), this.NONE.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>)),); this.setGameKeyEvent(java_arguments.0) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }
    

    pub fn update(&mut self) {
    }
}
impl std::ops::Deref for ReleaseTouchInputToGameKeyEventAction {
    type Target = GameKeyCompleteMotionGestureInputEvent;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ReleaseTouchInputToGameKeyEventAction {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEventDyn for ReleaseTouchInputToGameKeyEventAction {
    fn as_GameKeyCompleteMotionGestureInputEvent(&self) -> &crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEvent { &self.base }
    fn as_GameKeyCompleteMotionGestureInputEvent_mut(&mut self) -> &mut crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEvent { &mut self.base }
    fn update(&mut self) { Self::update(self) }
}
impl crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn for ReleaseTouchInputToGameKeyEventAction {
    fn as_CompleteMotionGestureInputEvent(&self) -> &crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEvent { &self.base.base }
    fn as_CompleteMotionGestureInputEvent_mut(&mut self) -> &mut crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEvent { &mut self.base.base }
}
impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for ReleaseTouchInputToGameKeyEventAction {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base.base.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base.base.base }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for ReleaseTouchInputToGameKeyEventAction {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEvent::getSourceId(crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEventDyn::as_GameKeyCompleteMotionGestureInputEvent_mut(self)) }
}


