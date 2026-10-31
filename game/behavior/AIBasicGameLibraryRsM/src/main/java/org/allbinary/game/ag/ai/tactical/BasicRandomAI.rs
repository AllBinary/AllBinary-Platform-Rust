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
// package org.allbinary.game.ag.ai.tactical
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::ai::BasicAI::BasicAI;
use crate::org::allbinary::game::ag::ai::InputProbability::InputProbability;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::game::rand::MyRandomFactory::MyRandomFactory;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicRandomAI {
    pub base: BasicAI,
    pub i_random: i32,
    pub inputProbability: Option<Rc<RefCell<InputProbability>>>,
    pub visitor: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>,
    pub keyArray: Vec<i32>,
    pub myRandomFactory: Option<Rc<RefCell<MyRandomFactory>>>,
}

impl crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface for BasicRandomAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for BasicRandomAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl BasicRandomAI {
    

    pub fn new_struct(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>, mut inputProbability: Option<Rc<RefCell<InputProbability>>>, mut visitor: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>) -> Self {
        let mut this = Self {
            base: BasicAI::new_struct(ownerLayerInterface.clone(), gameInput.clone()),
            i_random: 0,
            inputProbability: None,
            visitor: None,
            keyArray: Vec::new(),
            myRandomFactory: None,
        };
        this.i_random = 0;
        this.keyArray = vec![unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }.clone(), unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() }.clone(), unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone(), unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone(), unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }.clone(), unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM7.clone() }.clone(), unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.clone() }.clone()];
        this.myRandomFactory = MyRandomFactory::getInstance();
        // super(...) is in the struct literal above
        this.inputProbability = inputProbability.clone();
        this.visitor = visitor.clone();
        this
    }

    pub fn new(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>, mut inputProbability: Option<Rc<RefCell<InputProbability>>>, mut visitor: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerLayerInterface, gameInput, inputProbability, visitor))))
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        if jcall!(jcall!(self.inputProbability).getTimeDelayHelper()).isTime(jcall!(GameTickTimeDelayHelperFactory::getInstance()).startTime.clone()) {
            self.i_random = { let java_receiver = self.myRandomFactory.clone(); let java_arguments = (jcall!(self.inputProbability).getMax(),); let java_result = jcall!(java_receiver).getAbsoluteNextInt(java_arguments.0); java_result };
            let mut repeat: bool = false;
            let mut repeatLikelyhoodIntegerArray: Vec<Option<i32>> = jcall!(self.inputProbability).getRepeatLikelyhoodIntegerArray();
            if self.getLastKey() != -1 && ((repeatLikelyhoodIntegerArray[(self.getLastKey()) as usize]).unwrap() as i32) >= self.i_random {
                repeat = true;
            }
            if !repeat {
                let mut likelyhoodIntegerArray: Vec<Vec<Option<i32>>> = jcall!(self.inputProbability).getLikelyhoodIntegerArray();
                self.i_random = { let java_receiver = self.myRandomFactory.clone(); let java_arguments = (jcall!(self.inputProbability).getMax(),); let java_result = jcall!(java_receiver).getAbsoluteNextInt(java_arguments.0); java_result };
                let mut size: i32 = (self.keyArray.len() as i32).clone();
                let mut likelyhoodIntegerKeyArray: Vec<Option<i32>> = Vec::new();
                for index in (0..size) {
                    let mut key: i32 = self.keyArray[(index) as usize].clone();
                    likelyhoodIntegerKeyArray = likelyhoodIntegerArray[(key) as usize].clone();
                    if (likelyhoodIntegerArray.len() as i32) >= key && likelyhoodIntegerKeyArray[(0) as usize].is_some() {
                        if self.i_random >= ((likelyhoodIntegerKeyArray[(0) as usize]).unwrap() as i32) && self.i_random < ((likelyhoodIntegerKeyArray[(1) as usize]).unwrap() as i32) {
                            self.setLastKey(key.clone());
                            break;
                        }
                    }
                }
            }
            jcall!(self.visitor).visit(None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
}
impl std::ops::Deref for BasicRandomAI {
    type Target = BasicAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicRandomAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for BasicRandomAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base }
}
impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for BasicRandomAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for BasicRandomAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


