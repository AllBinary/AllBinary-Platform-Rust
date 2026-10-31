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
// package org.allbinary.game.layer.special
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor;
use super::SpecialUpGameInputProcessor::SpecialUpGameInputProcessor;
use super::SpecialDownGameInputProcessor::SpecialDownGameInputProcessor;
use super::SpecialRightGameInputProcessor::SpecialRightGameInputProcessor;
use super::SpecialLeftGameInputProcessor::SpecialLeftGameInputProcessor;
use super::Special1GameInputProcessor::Special1GameInputProcessor;
use super::Special3GameInputProcessor::Special3GameInputProcessor;
use super::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use super::SpecialAnalogUpGameInputProcessor::SpecialAnalogUpGameInputProcessor;
use super::SpecialAnalogDownGameInputProcessor::SpecialAnalogDownGameInputProcessor;
use super::SpecialAnalogRightGameInputProcessor::SpecialAnalogRightGameInputProcessor;
use super::SpecialAnalogLeftGameInputProcessor::SpecialAnalogLeftGameInputProcessor;
use super::SpecialAnalogLeftTriggerGameInputProcessor::SpecialAnalogLeftTriggerGameInputProcessor;
use super::SpecialAnalogRightTriggerGameInputProcessor::SpecialAnalogRightTriggerGameInputProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AnalogInputBuilder {
}

thread_local! { pub static AnalogInputBuilder_instance: Option<Rc<RefCell<AnalogInputBuilder>>> = AnalogInputBuilder::new(); }

impl AnalogInputBuilder {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AnalogInputBuilder>>> {
        return AnalogInputBuilder_instance.with(Clone::clone).clone();
    }
    

    pub fn disable(&mut self, mut inputProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>>>, mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) {
        inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }) as usize] = SpecialUpGameInputProcessor::new(collidableDestroyableDamageableLayer.clone());
        inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() }) as usize] = SpecialDownGameInputProcessor::new(collidableDestroyableDamageableLayer.clone());
        inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }) as usize] = SpecialRightGameInputProcessor::new(collidableDestroyableDamageableLayer.clone());
        inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }) as usize] = SpecialLeftGameInputProcessor::new(collidableDestroyableDamageableLayer.clone());
        inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.clone() }) as usize] = Special1GameInputProcessor::new(collidableDestroyableDamageableLayer.clone());
        inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM5.clone() }) as usize] = Special3GameInputProcessor::new(collidableDestroyableDamageableLayer.clone());
    }
    

    pub fn enable(&mut self, mut inputProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>>>, mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) {
        inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }) as usize] = SpecialAnalogUpGameInputProcessor::new(collidableDestroyableDamageableLayer.clone());
        inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() }) as usize] = SpecialAnalogDownGameInputProcessor::new(collidableDestroyableDamageableLayer.clone());
        inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }) as usize] = SpecialAnalogRightGameInputProcessor::new(collidableDestroyableDamageableLayer.clone());
        inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }) as usize] = SpecialAnalogLeftGameInputProcessor::new(collidableDestroyableDamageableLayer.clone());
        inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.clone() }) as usize] = SpecialAnalogLeftTriggerGameInputProcessor::new(collidableDestroyableDamageableLayer.clone());
        inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM5.clone() }) as usize] = SpecialAnalogRightTriggerGameInputProcessor::new(collidableDestroyableDamageableLayer.clone());
    }
}

