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
use crate::java::lang::Exception::*;
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
        this.CLIFF = BasicTerrainInfo::new({ let java_field = jcall!(AngleFactory::getInstance()).DOWN.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn terrainEvent(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut x: i32, mut y: i32, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        let mut maxColumns: i32 = jcall!(jcall!(geographicMapInterfaceArray[(0) as usize]).getAllBinaryTiledLayer()).getColumns();
        if jcall!(geographicMapCellPosition).getColumn() > 0 && jcall!(geographicMapCellPosition).getColumn() < maxColumns {
            let mut nextTerrainGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
            let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(geographicMapInterfaceArray[(0) as usize]).getGeographicMapCellPositionFactory();
            if match (&direction, &{ let java_field = jcall!(DirectionFactory::getInstance()).LEFT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                nextTerrainGeographicMapCellPosition = { let java_value = (jcall!(geographicMapCellPositionFactory).getAt(jcall!(geographicMapCellPosition).getColumn() - 1, jcall!(geographicMapCellPosition).getRow())).clone(); java_value };
            } else if match (&direction, &{ let java_field = jcall!(DirectionFactory::getInstance()).RIGHT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                nextTerrainGeographicMapCellPosition = { let java_value = (jcall!(geographicMapCellPositionFactory).getAt(jcall!(geographicMapCellPosition).getColumn() + 1, jcall!(geographicMapCellPosition).getRow())).clone(); java_value };
            }
            jcall!(self.geographicMapBehavior).getCellTypeAt(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone(), nextTerrainGeographicMapCellPosition.clone());
            let mut hasSolidBlock: bool = self.hasSolidBlock(geographicMapInterfaceArray.clone(), geographicMapCellTypeArray.clone());
            if !hasSolidBlock {
                let mut terrainEvent: Option<Rc<RefCell<TerrainEvent>>> = jcall!(TerrainEventCircularStaticPool::getInstance()).getNext(self.CLIFF.clone());
                jcall!(TerrainEventHandler::getInstance(crate::java_into_object(layer.clone()))).fireEvent(terrainEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
            }
        }
    }
    

    pub fn terrainMove(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut x: i32, mut y: i32) {
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
    fn terrainMove(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut x: i32, mut y: i32) { Self::terrainMove(self, layer, geographicMapInterfaceArray, x, y) }
}


