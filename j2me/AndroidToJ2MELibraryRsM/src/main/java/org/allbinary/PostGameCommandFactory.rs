// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::configuration::feature::MultiPlayerGameFeatureFactory::MultiPlayerGameFeatureFactory;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;
use crate::org::allbinary::logic::system::os::OperatingSystemFactory::OperatingSystemFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PostGameCommandFactory {
}

thread_local! { pub static PostGameCommandFactory_instance: Option<Rc<RefCell<PostGameCommandFactory>>> = PostGameCommandFactory::new(); }

impl PostGameCommandFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PostGameCommandFactory>>> {
        return PostGameCommandFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn addCommands(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>) {
        let mut operatingSystemInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> = jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance();
        let mut gameCommandsFactory: Option<Rc<RefCell<GameCommandsFactory>>> = GameCommandsFactory::getInstance();
        if !jcall!(operatingSystemInterface).isOverScan() {
            jcall!(canvas).addCommand(jcall!(gameCommandsFactory).TOGGLE_KEYBOARD.clone());
        }
        jcall!(MultiPlayerGameFeatureFactory::getInstance()).addCommands(crate::java_into_object(canvas.clone()));
    }
}

