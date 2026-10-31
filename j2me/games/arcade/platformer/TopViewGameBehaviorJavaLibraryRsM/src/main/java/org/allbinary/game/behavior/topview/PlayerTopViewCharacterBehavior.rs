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
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::game::tracking::TrackingEventCircularStaticPool::TrackingEventCircularStaticPool;
use crate::org::allbinary::game::tracking::TrackingEventHandler::TrackingEventHandler;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapUtil::BasicGeographicMapUtil;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use super::TopViewCharacterBehavior::TopViewCharacterBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlayerTopViewCharacterBehavior {
    pub base: TopViewCharacterBehavior,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl PlayerTopViewCharacterBehavior {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TopViewCharacterBehavior::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn terrainEvent(&mut self, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut x: i32, mut y: i32, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>) {
        jcall!(TrackingEventHandler::getInstance()).fireEvent(jcall!(TrackingEventCircularStaticPool::getInstance()).getNextInstance(layer.clone()));
    }
    

    pub fn terrainMove(&mut self, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut x: i32, mut y: i32) {
        let mut basicGeographicMapUtil: Option<Rc<RefCell<BasicGeographicMapUtil>>> = BasicGeographicMapUtil::getInstance();
        jcall!(basicGeographicMapUtil).setPosition(geographicMapInterfaceArray.clone(), x.clone(), y.clone());
    }
    

    pub fn moveIfOnScreen(&mut self, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut ax: i32, mut ay: i32) {
    }
    

    pub fn isTiledLayerMoveable(&mut self, mut terrainTiledLayer: Option<Rc<RefCell<AllBinaryTiledLayer>>>, mut x: i32, mut y: i32) -> bool {
        return (jcall!(terrainTiledLayer).getXP() + jcall!(DisplayInfoSingleton::getInstance()).getLastWidth() < jcall!(terrainTiledLayer).getWidth() || x < 0) && (jcall!(terrainTiledLayer).getXP() > 0 || x > 0);
    }
}
impl std::ops::Deref for PlayerTopViewCharacterBehavior {
    type Target = TopViewCharacterBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PlayerTopViewCharacterBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::behavior::topview::TopViewCharacterBehavior::TopViewCharacterBehaviorDyn for PlayerTopViewCharacterBehavior {
    fn as_TopViewCharacterBehavior(&self) -> &crate::org::allbinary::game::behavior::topview::TopViewCharacterBehavior::TopViewCharacterBehavior { &self.base }
    fn as_TopViewCharacterBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::behavior::topview::TopViewCharacterBehavior::TopViewCharacterBehavior { &mut self.base }
    fn terrainEvent(&mut self, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut x: i32, mut y: i32, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>) { Self::terrainEvent(self, layer, direction, x, y, geographicMapInterfaceArray, geographicMapCellPosition) }
    fn terrainMove(&mut self, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut x: i32, mut y: i32) { Self::terrainMove(self, layer, geographicMapInterfaceArray, x, y) }
}


