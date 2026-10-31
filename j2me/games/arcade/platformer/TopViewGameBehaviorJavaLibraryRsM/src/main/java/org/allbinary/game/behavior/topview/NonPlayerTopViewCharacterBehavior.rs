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
// package org.allbinary.game.behavior.topview
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::media::graphics::geography::map::MultiGeographicMapBehavior::MultiGeographicMapBehavior;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::direction::DirectionFactory::DirectionFactory;
use crate::org::allbinary::game::terrain::BasicTerrainInfo::BasicTerrainInfo;
use crate::org::allbinary::game::terrain::TerrainEvent::TerrainEvent;
use crate::org::allbinary::game::terrain::TerrainEventCircularStaticPool::TerrainEventCircularStaticPool;
use crate::org::allbinary::game::terrain::TerrainEventHandler::TerrainEventHandler;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;
use super::TopViewCharacterBehavior::TopViewCharacterBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NonPlayerTopViewCharacterBehavior {
    pub base: TopViewCharacterBehavior,
    pub geographicMapBehavior: Option<Rc<RefCell<MultiGeographicMapBehavior>>>,
    pub CLIFF: Option<Rc<RefCell<BasicTerrainInfo>>>,
}


impl NonPlayerTopViewCharacterBehavior {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TopViewCharacterBehavior::new_struct(),
            geographicMapBehavior: None,
            CLIFF: None,
        };
        this.geographicMapBehavior = MultiGeographicMapBehavior::new();
        this.CLIFF = BasicTerrainInfo::new(jcall!(AngleFactory::getInstance()).DOWN.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn terrainEvent(&mut self, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut x: i32, mut y: i32, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>) {
        let mut maxColumns: i32 = jcall!(jcall!(geographicMapInterfaceArray[(0) as usize]).getAllBinaryTiledLayer()).getColumns();
        if jcall!(geographicMapCellPosition).getColumn() > 0 && jcall!(geographicMapCellPosition).getColumn() < maxColumns {
            let mut nextTerrainGeographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>> = unsafe { crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.clone() }.clone();
            let mut geographicMapCellPositionFactory: Option<Rc<RefCell<BasicGeographicMapCellPositionFactory>>> = jcall!(geographicMapInterfaceArray[(0) as usize]).getGeographicMapCellPositionFactory();
            if match (&direction, &jcall!(DirectionFactory::getInstance()).LEFT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                nextTerrainGeographicMapCellPosition = jcall!(geographicMapCellPositionFactory).getAt(jcall!(geographicMapCellPosition).getColumn() - 1, jcall!(geographicMapCellPosition).getRow());
            } else if match (&direction, &jcall!(DirectionFactory::getInstance()).RIGHT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                nextTerrainGeographicMapCellPosition = jcall!(geographicMapCellPositionFactory).getAt(jcall!(geographicMapCellPosition).getColumn() + 1, jcall!(geographicMapCellPosition).getRow());
            }
            jcall!(self.geographicMapBehavior).getCellTypeAt(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), nextTerrainGeographicMapCellPosition.clone());
            let mut hasSolidBlock: bool = self.hasSolidBlock(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone());
            if !hasSolidBlock {
                let mut terrainEvent: Option<Rc<RefCell<TerrainEvent>>> = jcall!(TerrainEventCircularStaticPool::getInstance()).getNext(self.CLIFF.clone());
                jcall!(TerrainEventHandler::getInstance(layer.clone())).fireEvent(terrainEvent.clone());
            }
        }
    }
    

    pub fn terrainMove(&mut self, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut x: i32, mut y: i32) {
        jcall!(layer).moveDXY(x.clone(), y.clone());
    }
}
impl std::ops::Deref for NonPlayerTopViewCharacterBehavior {
    type Target = TopViewCharacterBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NonPlayerTopViewCharacterBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::behavior::topview::TopViewCharacterBehavior::TopViewCharacterBehaviorDyn for NonPlayerTopViewCharacterBehavior {
    fn as_TopViewCharacterBehavior(&self) -> &crate::org::allbinary::game::behavior::topview::TopViewCharacterBehavior::TopViewCharacterBehavior { &self.base }
    fn as_TopViewCharacterBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::behavior::topview::TopViewCharacterBehavior::TopViewCharacterBehavior { &mut self.base }
    fn terrainMove(&mut self, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut x: i32, mut y: i32) { Self::terrainMove(self, layer, geographicMapInterfaceArray, x, y) }
}


