// 
// AllBinary Open License Version 1 
// Copyright (c) 2002 AllBinary 
//  
// By agreeing to this license you and any business entity you represent are 
// legally bound to the AllBinary Open License Version 1 legal agreement. 
//  
// You may obtain the AllBinary Open License Version 1 legal agreement from 
// AllBinary or the root directory of AllBinary's AllBinary Platform repository. 
//  
// Created By: Travis Berthelot  
// Generated Code Do Not Modify
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use super::AllBinaryGameLayerManager::AllBinaryGameLayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FakeLayerManager {
    pub base: AllBinaryGameLayerManager,
    pub geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>,
    pub geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>,
}

impl GeographicMapCompositeInterface for FakeLayerManager {
    fn geographicMapCellTypeArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>> { Self::geographicMapCellTypeArray(self) }
    fn getGeographicMapInterface(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>> { Self::getGeographicMapInterface(self) }
    fn setGeographicMapInterface(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>) { Self::setGeographicMapInterface(self, geographicMapInterfaceArray) }
}

impl FakeLayerManager {
    

    pub fn new_struct(mut gameInfo: Option<Rc<RefCell<GameInfo>>>) -> Self {
        let mut this = Self {
            base: AllBinaryGameLayerManager::new_struct(jcall!(BasicColorFactory::getInstance()).BLACK.clone(), jcall!(BasicColorFactory::getInstance()).WHITE.clone(), gameInfo.clone()),
            geographicMapInterfaceArray: Vec::new(),
            geographicMapCellTypeArray: Vec::new(),
        };
        this.geographicMapInterfaceArray = crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap_NULL_BASIC_GEOGRAPHIC_MAP_ARRAY.with(Clone::clone).clone();
        this.geographicMapCellTypeArray = crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType_NULL_GEOGRAPHIC_MAP_CELL_TYPE_ARRAY.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut gameInfo: Option<Rc<RefCell<GameInfo>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameInfo))))
    }
    

    pub fn getGeographicMapInterface(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>> {
        return self.geographicMapInterfaceArray.clone();
    }
    

    pub fn setGeographicMapInterface(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>) {
        self.geographicMapInterfaceArray = geographicMapInterfaceArray.clone();
        self.geographicMapCellTypeArray = vec![None; ((self.geographicMapInterfaceArray.len() as i32)) as usize];
    }
    

    pub fn geographicMapCellTypeArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>> {
        return self.geographicMapCellTypeArray.clone();
    }
}
impl std::ops::Deref for FakeLayerManager {
    type Target = AllBinaryGameLayerManager;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FakeLayerManager {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn for FakeLayerManager {
    fn as_AllBinaryGameLayerManager(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager { &self.base }
    fn as_AllBinaryGameLayerManager_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager { &mut self.base }
}
impl crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn for FakeLayerManager {
    fn as_AllBinaryLayerManager(&self) -> &crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager { &self.base.base }
    fn as_AllBinaryLayerManager_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager { &mut self.base.base }
}
impl crate::org::allbinary::layer::LayerManager::LayerManagerDyn for FakeLayerManager {
    fn as_LayerManager(&self) -> &crate::org::allbinary::layer::LayerManager::LayerManager { &self.base.base.base }
    fn as_LayerManager_mut(&mut self) -> &mut crate::org::allbinary::layer::LayerManager::LayerManager { &mut self.base.base.base }
    fn append(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager::append(crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn::as_AllBinaryLayerManager_mut(self), p1) }
    fn appendAt(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, p2: i32) { crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager::appendAt(crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn::as_AllBinaryLayerManager_mut(self), p1, p2) }
    fn remove(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager::remove(crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn::as_AllBinaryLayerManager_mut(self), p1) }
    fn cleanup(&mut self) { crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager::cleanup(crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn::as_AllBinaryLayerManager_mut(self)) }
}


