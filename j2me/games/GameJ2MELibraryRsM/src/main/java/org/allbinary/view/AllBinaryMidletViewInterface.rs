// Generated Code Do Not Modify
// package org.allbinary.view
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::event::GameInitializedListenerInterface::GameInitializedListenerInterface;
use crate::org::allbinary::game::midlet::DemoGameMidletEventListener::DemoGameMidletEventListener;
use crate::org::allbinary::midlet::MidletCompositeInterface::MidletCompositeInterface;
use super::EmulatorViewInterface::EmulatorViewInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait AllBinaryMidletViewInterface: MidletCompositeInterface + EmulatorViewInterface + DemoGameMidletEventListener + GameInitializedListenerInterface {
    

    fn onResume(&mut self);
    

    fn onPause(&mut self);
    

    fn onDestroy(&mut self);
}


