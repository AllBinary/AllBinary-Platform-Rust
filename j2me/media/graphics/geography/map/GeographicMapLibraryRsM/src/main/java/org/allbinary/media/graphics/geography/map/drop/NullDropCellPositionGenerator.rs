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
// package org.allbinary.media.graphics.geography.map.drop
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullDropCellPositionGenerator {
    pub base: BaseDropCellPositionGenerator,
}

thread_local! { pub static NullDropCellPositionGenerator_instance: Option<Rc<RefCell<NullDropCellPositionGenerator>>> = NullDropCellPositionGenerator::new(); }

impl NullDropCellPositionGenerator {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseDropCellPositionGenerator::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<NullDropCellPositionGenerator>>> {
        return NullDropCellPositionGenerator_instance.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for NullDropCellPositionGenerator {
    type Target = BaseDropCellPositionGenerator;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullDropCellPositionGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn for NullDropCellPositionGenerator {
    fn as_BaseDropCellPositionGenerator(&self) -> &crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator { &self.base }
    fn as_BaseDropCellPositionGenerator_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator { &mut self.base }
}
impl crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionGeneratorInterface::DropCellPositionGeneratorInterface for NullDropCellPositionGenerator {
    fn getName(&mut self) -> String { crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator::getName(crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn::as_BaseDropCellPositionGenerator_mut(self)) }
    fn update(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator::update(crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn::as_BaseDropCellPositionGenerator_mut(self), p1, p2) }
    fn processTick(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator::processTick(crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn::as_BaseDropCellPositionGenerator_mut(self), p1) }
    fn isDropAllowedAt(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator::isDropAllowedAt(crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn::as_BaseDropCellPositionGenerator_mut(self), p1) }
}


