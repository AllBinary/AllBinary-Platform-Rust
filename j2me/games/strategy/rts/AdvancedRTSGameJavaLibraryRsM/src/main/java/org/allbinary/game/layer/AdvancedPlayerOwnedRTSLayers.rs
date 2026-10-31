// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::PlayerOwnedRTSLayers::PlayerOwnedRTSLayers;
use super::RTSLayer::RTSLayer;
use super::AllBinaryGameLayer::AllBinaryGameLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdvancedPlayerOwnedRTSLayers {
    pub base: PlayerOwnedRTSLayers,
    pub unitsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub waypointsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub buildingList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub baseList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub garrisonList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub houseList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub labList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub factoryList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl AdvancedPlayerOwnedRTSLayers {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PlayerOwnedRTSLayers::new_struct(),
            unitsList: None,
            waypointsList: None,
            buildingList: None,
            baseList: None,
            garrisonList: None,
            houseList: None,
            labList: None,
            factoryList: None,
        };
        this.unitsList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.waypointsList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.buildingList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.baseList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.garrisonList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.houseList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.labList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.factoryList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addUnit(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        jcall!(self.unitsList).add(crate::java_into_object(rtsLayer.clone()));
        jcall!(self.rtsLayerList).add(crate::java_into_object(rtsLayer.clone()));
    }
    

    pub fn addWaypoint(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        jcall!(self.waypointsList).add(crate::java_into_object(rtsLayer.clone()));
        jcall!(self.rtsLayerList).add(crate::java_into_object(rtsLayer.clone()));
    }
    

    pub fn addBase(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        jcall!(self.baseList).add(crate::java_into_object(rtsLayer.clone()));
        jcall!(self.buildingList).add(crate::java_into_object(rtsLayer.clone()));
        jcall!(self.rtsLayerList).add(crate::java_into_object(rtsLayer.clone()));
    }
    

    pub fn addGarrison(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        jcall!(self.garrisonList).add(crate::java_into_object(rtsLayer.clone()));
        jcall!(self.buildingList).add(crate::java_into_object(rtsLayer.clone()));
        jcall!(self.rtsLayerList).add(crate::java_into_object(rtsLayer.clone()));
    }
    

    pub fn addHouse(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        jcall!(self.houseList).add(crate::java_into_object(rtsLayer.clone()));
        jcall!(self.buildingList).add(crate::java_into_object(rtsLayer.clone()));
        jcall!(self.rtsLayerList).add(crate::java_into_object(rtsLayer.clone()));
    }
    

    pub fn addLab(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        jcall!(self.labList).add(crate::java_into_object(rtsLayer.clone()));
        jcall!(self.buildingList).add(crate::java_into_object(rtsLayer.clone()));
        jcall!(self.rtsLayerList).add(crate::java_into_object(rtsLayer.clone()));
    }
    

    pub fn addFactory(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        jcall!(self.factoryList).add(crate::java_into_object(rtsLayer.clone()));
        jcall!(self.buildingList).add(crate::java_into_object(rtsLayer.clone()));
        jcall!(self.rtsLayerList).add(crate::java_into_object(rtsLayer.clone()));
    }
    

    pub fn removeUnit(&mut self, mut gameLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) {
        jcall!(self.unitsList).remove(crate::java_into_object(gameLayer.clone()));
        jcall!(self.rtsLayerList).remove(crate::java_into_object(gameLayer.clone()));
    }
    

    pub fn removeWaypoint(&mut self, mut gameLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) {
        jcall!(self.waypointsList).remove(crate::java_into_object(gameLayer.clone()));
        jcall!(self.rtsLayerList).remove(crate::java_into_object(gameLayer.clone()));
    }
    

    pub fn getUnitsList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.unitsList.clone();
    }
    

    pub fn getWaypointsList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.waypointsList.clone();
    }
    

    pub fn getBaseList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.baseList.clone();
    }
    

    pub fn getGarrisonList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.garrisonList.clone();
    }
    

    pub fn getHouseList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.houseList.clone();
    }
    

    pub fn getLabList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.labList.clone();
    }
    

    pub fn getFactoryList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.factoryList.clone();
    }
    

    pub fn getTotalBuildings(&mut self) -> i32 {
        return jcall!(self.baseList).size() + jcall!(self.garrisonList).size() + jcall!(self.houseList).size() + jcall!(self.labList).size() + jcall!(self.factoryList).size();
    }
    

    pub fn removeBuilding(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) {
        jcall!(self.buildingList).add(crate::java_into_object(layerInterface.clone()));
        if jcall!(self.getBaseList()).remove(layerInterface.clone()) {
            jcall!(self.rtsLayerList).remove(crate::java_into_object(layerInterface.clone()));
        } else if jcall!(self.getFactoryList()).remove(layerInterface.clone()) {
            jcall!(self.rtsLayerList).remove(crate::java_into_object(layerInterface.clone()));
        } else if jcall!(self.getGarrisonList()).remove(layerInterface.clone()) {
            jcall!(self.rtsLayerList).remove(crate::java_into_object(layerInterface.clone()));
        } else if jcall!(self.getLabList()).remove(layerInterface.clone()) {
            jcall!(self.rtsLayerList).remove(crate::java_into_object(layerInterface.clone()));
        } else if jcall!(self.getHouseList()).remove(layerInterface.clone()) {
            jcall!(self.rtsLayerList).remove(crate::java_into_object(layerInterface.clone()));
        }
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.base.toString());
        jcall!(stringBuffer).append(String::from(" Bases: "));
        jcall!(stringBuffer).appendint(jcall!(self.getBaseList()).size());
        jcall!(stringBuffer).append(String::from(" Factory: "));
        jcall!(stringBuffer).appendint(jcall!(self.getFactoryList()).size());
        jcall!(stringBuffer).append(String::from(" Garrison: "));
        jcall!(stringBuffer).appendint(jcall!(self.getGarrisonList()).size());
        jcall!(stringBuffer).append(String::from(" Labs: "));
        jcall!(stringBuffer).appendint(jcall!(self.getLabList()).size());
        jcall!(stringBuffer).append(String::from(" Houses: "));
        jcall!(stringBuffer).appendint(jcall!(self.getHouseList()).size());
        jcall!(stringBuffer).append(String::from(" Waypoints: "));
        jcall!(stringBuffer).appendint(jcall!(self.getWaypointsList()).size());
        jcall!(stringBuffer).append(String::from(" Units: "));
        jcall!(stringBuffer).appendint(jcall!(self.getUnitsList()).size());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getBuildingList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.buildingList.clone();
    }
}
impl std::ops::Deref for AdvancedPlayerOwnedRTSLayers {
    type Target = PlayerOwnedRTSLayers;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AdvancedPlayerOwnedRTSLayers {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::PlayerOwnedRTSLayers::PlayerOwnedRTSLayersDyn for AdvancedPlayerOwnedRTSLayers {
    fn as_PlayerOwnedRTSLayers(&self) -> &crate::org::allbinary::game::layer::PlayerOwnedRTSLayers::PlayerOwnedRTSLayers { &self.base }
    fn as_PlayerOwnedRTSLayers_mut(&mut self) -> &mut crate::org::allbinary::game::layer::PlayerOwnedRTSLayers::PlayerOwnedRTSLayers { &mut self.base }
    fn toString(&mut self) -> String { Self::toString(self) }
}


