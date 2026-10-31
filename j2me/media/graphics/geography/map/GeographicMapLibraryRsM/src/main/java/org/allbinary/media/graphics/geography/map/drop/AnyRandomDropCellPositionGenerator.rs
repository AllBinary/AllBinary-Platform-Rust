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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::game::rand::MyRandomFactory::MyRandomFactory;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use super::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AnyRandomDropCellPositionGenerator {
    pub base: BaseDropCellPositionGenerator,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub geographicMapInterface: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
}


impl AnyRandomDropCellPositionGenerator {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseDropCellPositionGenerator::new_struct(),
            list: None,
            geographicMapInterface: None,
        };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.geographicMapInterface = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn update(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) {
        self.geographicMapInterface = geographicMapInterface.clone();
        let mut basicGeographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(geographicMapInterface).getGeographicMapCellPositionFactory();
        let mut randomFactory: Option<Rc<RefCell<MyRandomFactory>>> = MyRandomFactory::getInstance();
        let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        let mut total: i32 = jcall!(tiledLayer).getColumns() * jcall!(tiledLayer).getRows();
        let mut randomColumn: i32 = 0;
        let mut randomRow: i32 = 0;
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = None;
        {
            let mut index: i32 = total.clone();
            while { index -= 1; index } >= 0 {
            randomColumn = jcall!(randomFactory).getAbsoluteNextInt(jcall!(tiledLayer).getColumns());
            randomRow = jcall!(randomFactory).getAbsoluteNextInt(jcall!(tiledLayer).getRows());
            geographicMapCellPosition = jcall!(basicGeographicMapCellPositionFactory).getAt(randomColumn.clone(), randomRow.clone());
            if self.isDropAllowedAt(geographicMapCellPosition.clone()) {
                jcall!(self.list).add(crate::java_into_object(geographicMapCellPosition.clone()));
            }
            }
        }
    }
    

    // throws Exception
    pub fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
    }
}
impl std::ops::Deref for AnyRandomDropCellPositionGenerator {
    type Target = BaseDropCellPositionGenerator;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AnyRandomDropCellPositionGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn for AnyRandomDropCellPositionGenerator {
    fn as_BaseDropCellPositionGenerator(&self) -> &crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator { &self.base }
    fn as_BaseDropCellPositionGenerator_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator { &mut self.base }
}
impl crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionGeneratorInterface::DropCellPositionGeneratorInterface for AnyRandomDropCellPositionGenerator {
    fn getName(&mut self) -> String { crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator::getName(crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn::as_BaseDropCellPositionGenerator_mut(self)) }
    fn update(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { Self::update(self, allBinaryGameLayerManager, geographicMapInterface) }
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processTick(self, allBinaryLayerManager) }
    fn isDropAllowedAt(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGenerator::isDropAllowedAt(crate::org::allbinary::media::graphics::geography::map::drop::BaseDropCellPositionGenerator::BaseDropCellPositionGeneratorDyn::as_BaseDropCellPositionGenerator_mut(self), p1) }
}


