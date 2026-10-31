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
// package org.allbinary.game.layer.waypoint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::media::audio::NoSound::NoSound;
use crate::org::allbinary::media::audio::Sound::Sound;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::pathfinding::MultipassState::MultipassState;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingInfo::PathFindingInfo;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WaypointBase {
    pub connectedWaypointList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub sound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>,
    pub allBinaryGameLayerManagerP: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>,
}

pub static mut WaypointBase_NULL_WAYPOINT_BASE: std::cell::LazyCell<Option<Rc<RefCell<dyn WaypointBaseDyn>>>> = std::cell::LazyCell::new(|| WaypointBase::new(NoSound::getInstance()).map(|reference| reference as Rc<RefCell<dyn WaypointBaseDyn>>));
impl EventListenerInterface for WaypointBase {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl WaypointBase {
    

    pub fn new_struct(mut sound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>) -> Self {
        let mut this = Self {
            connectedWaypointList: None,
            sound: None,
            allBinaryGameLayerManagerP: None,
        };
        this.connectedWaypointList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.allBinaryGameLayerManagerP = AllBinaryGameLayerManager::getNullInstance();
        this.sound = sound.clone();
        this
    }

    pub fn new(mut sound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(sound))))
    }
    

    pub fn getSound(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>> {
        return self.sound.clone();
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        self.allBinaryGameLayerManagerP = allBinaryGameLayerManager.clone();
    }
    

    pub fn getConnectedWaypointList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.connectedWaypointList.clone();
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    pub fn getPathFindingInfo(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<PathFindingInfo>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getPathsList(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut multipassState: Option<Rc<RefCell<MultipassState>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getPathsListRunnable(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getPathsListFromCacheOnly(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn visit(&mut self, mut unitLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn reset(&mut self) {
    }
    

    pub fn getType(&mut self) -> i32 {
        return 0;
    }
}
pub trait WaypointBaseDyn: crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface {
    fn as_WaypointBase(&self) -> &WaypointBase;
    fn as_WaypointBase_mut(&mut self) -> &mut WaypointBase;
    fn getSound(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>> { WaypointBase::getSound(self.as_WaypointBase_mut()) }
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { WaypointBase::setAllBinaryGameLayerManager(self.as_WaypointBase_mut(), allBinaryGameLayerManager) }
    fn getConnectedWaypointList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { WaypointBase::getConnectedWaypointList(self.as_WaypointBase_mut()) }
    fn getPathFindingInfo(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<PathFindingInfo>>> { WaypointBase::getPathFindingInfo(self.as_WaypointBase_mut(), geographicMapCellPosition) }
    fn getPathsList(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut multipassState: Option<Rc<RefCell<MultipassState>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { WaypointBase::getPathsList(self.as_WaypointBase_mut(), geographicMapCellPosition, pathFindingInfo, multipassState) }
    fn getPathsListRunnable(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { WaypointBase::getPathsListRunnable(self.as_WaypointBase_mut(), geographicMapCellPosition) }
    fn getPathsListFromCacheOnly(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { WaypointBase::getPathsListFromCacheOnly(self.as_WaypointBase_mut(), geographicMapCellPosition) }
    fn visit(&mut self, mut unitLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { WaypointBase::visit(self.as_WaypointBase_mut(), unitLayer) }
    fn reset(&mut self) { WaypointBase::reset(self.as_WaypointBase_mut()) }
    fn getType(&mut self) -> i32 { WaypointBase::getType(self.as_WaypointBase_mut()) }
}
impl WaypointBaseDyn for WaypointBase {
    fn as_WaypointBase(&self) -> &WaypointBase { self }
    fn as_WaypointBase_mut(&mut self) -> &mut WaypointBase { self }
}


