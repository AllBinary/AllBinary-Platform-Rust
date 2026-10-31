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
// package org.allbinary.game.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Gauge::Gauge;
use super::GameConfiguration::GameConfiguration;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameConfigurationGauge {
    pub base: Gauge,
}


impl GameConfigurationGauge {
    

    pub fn new_struct(mut gameConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>) -> Self {
        let mut this = Self {
            base: Gauge::new_struct4(jcall!(gameConfiguration).getName(), jcall!(gameConfiguration).isModifiable().unwrap(), ((jcall!(gameConfiguration).getMaxValue()).unwrap() as i32) - ((jcall!(gameConfiguration).getMinValue()).unwrap() as i32), ((jcall!(gameConfiguration).getValue()).unwrap() as i32) - ((jcall!(gameConfiguration).getMinValue()).unwrap() as i32)),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut gameConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameConfiguration))))
    }
}
impl std::ops::Deref for GameConfigurationGauge {
    type Target = Gauge;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameConfigurationGauge {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



