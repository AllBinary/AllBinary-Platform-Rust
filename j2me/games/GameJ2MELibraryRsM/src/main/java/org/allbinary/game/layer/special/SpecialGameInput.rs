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
// package org.allbinary.game.layer.special
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use super::SpecialGameInputInterface::SpecialGameInputInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SpecialGameInput {
}

impl SpecialGameInputInterface for SpecialGameInput {
    fn up(&mut self) { Self::up(self) }
    fn down(&mut self) { Self::down(self) }
    fn right(&mut self) { Self::right(self) }
    fn left(&mut self) { Self::left(self) }
    fn strafeLeft(&mut self) { Self::strafeLeft(self) }
    fn strafeRight(&mut self) { Self::strafeRight(self) }
    fn fire(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::fire(self, layerManager, gameKeyEvent) }
    fn special1(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::special1(self, layerManager, gameKeyEvent) }
    fn special2(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::special2(self, layerManager, gameKeyEvent) }
    fn special3(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::special3(self, layerManager, gameKeyEvent) }
    fn special4(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::special4(self, layerManager, gameKeyEvent) }
    fn special5(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::special5(self, layerManager, gameKeyEvent) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for SpecialGameInput {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}

impl SpecialGameInput {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn up(&mut self) {
    }
    

    // throws Exception
    pub fn down(&mut self) {
    }
    

    // throws Exception
    pub fn right(&mut self) {
    }
    

    // throws Exception
    pub fn left(&mut self) {
    }
    

    // throws Exception
    pub fn strafeLeft(&mut self) {
    }
    

    // throws Exception
    pub fn strafeRight(&mut self) {
    }
    

    // throws Exception
    pub fn fire(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // throws Exception
    pub fn special1(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // throws Exception
    pub fn special2(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // throws Exception
    pub fn special3(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // throws Exception
    pub fn special4(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // throws Exception
    pub fn special5(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
}
pub trait SpecialGameInputDyn {
    fn as_SpecialGameInput(&self) -> &SpecialGameInput;
    fn as_SpecialGameInput_mut(&mut self) -> &mut SpecialGameInput;
    fn up(&mut self) { SpecialGameInput::up(self.as_SpecialGameInput_mut()) }
    fn down(&mut self) { SpecialGameInput::down(self.as_SpecialGameInput_mut()) }
    fn right(&mut self) { SpecialGameInput::right(self.as_SpecialGameInput_mut()) }
    fn left(&mut self) { SpecialGameInput::left(self.as_SpecialGameInput_mut()) }
    fn strafeLeft(&mut self) { SpecialGameInput::strafeLeft(self.as_SpecialGameInput_mut()) }
    fn strafeRight(&mut self) { SpecialGameInput::strafeRight(self.as_SpecialGameInput_mut()) }
    fn fire(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { SpecialGameInput::fire(self.as_SpecialGameInput_mut(), layerManager, gameKeyEvent) }
    fn special1(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { SpecialGameInput::special1(self.as_SpecialGameInput_mut(), layerManager, gameKeyEvent) }
    fn special2(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { SpecialGameInput::special2(self.as_SpecialGameInput_mut(), layerManager, gameKeyEvent) }
    fn special3(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { SpecialGameInput::special3(self.as_SpecialGameInput_mut(), layerManager, gameKeyEvent) }
    fn special4(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { SpecialGameInput::special4(self.as_SpecialGameInput_mut(), layerManager, gameKeyEvent) }
    fn special5(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { SpecialGameInput::special5(self.as_SpecialGameInput_mut(), layerManager, gameKeyEvent) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { SpecialGameInput::paint(self.as_SpecialGameInput_mut(), graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { SpecialGameInput::paintThreed(self.as_SpecialGameInput_mut(), graphics) }
}
impl SpecialGameInputDyn for SpecialGameInput {
    fn as_SpecialGameInput(&self) -> &SpecialGameInput { self }
    fn as_SpecialGameInput_mut(&mut self) -> &mut SpecialGameInput { self }
}


