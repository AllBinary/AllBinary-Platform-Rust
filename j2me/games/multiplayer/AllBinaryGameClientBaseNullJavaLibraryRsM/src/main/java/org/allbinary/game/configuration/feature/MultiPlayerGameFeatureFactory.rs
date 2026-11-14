// Generated Code Do Not Modify
// package org.allbinary.game.configuration.feature
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultiPlayerGameFeatureFactory {
}

thread_local! { pub static MultiPlayerGameFeatureFactory_instance: Option<Rc<RefCell<MultiPlayerGameFeatureFactory>>> = MultiPlayerGameFeatureFactory::new(); }

impl MultiPlayerGameFeatureFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<MultiPlayerGameFeatureFactory>>> {
        return MultiPlayerGameFeatureFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addCommands(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        let mut gameCommandsFactory: Option<Rc<RefCell<GameCommandsFactory>>> = GameCommandsFactory::getInstance();
        let mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>> = { let java_object = (anyType).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>>().expect("ClassCastException").clone()) };
        jcall!(canvas).addCommand({ let java_field = jcall!(gameCommandsFactory).QUIT_COMMAND.clone(); java_field }.clone());
    }
}

