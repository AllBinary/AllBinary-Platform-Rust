// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
use crate::org::allbinary::game::identification::GroupCommonFactory::GroupCommonFactory;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::direction::DirectionFactory::DirectionFactory;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::game::combat::weapon::WeaponProperties::WeaponProperties;
use crate::org::allbinary::game::identification::BasicGroupFactory::BasicGroupFactory;
use crate::org::allbinary::game::part::PartInterface::PartInterface;
use crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPart;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::racetrack::BaseRaceTrackGeographicMap::BaseRaceTrackGeographicMap;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::java::lang::Exception::*;
use super::RTSLayer::RTSLayer;
use super::FakeLayerManager::FakeLayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSLayerUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub groupCommonFactory: Option<Rc<RefCell<GroupCommonFactory>>>,
    pub MAX_RELOAD_TIME: i64,
}

thread_local! { pub static RTSLayerUtil_instance: Option<Rc<RefCell<RTSLayerUtil>>> = RTSLayerUtil::new(); }

impl RTSLayerUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            groupCommonFactory: None,
            MAX_RELOAD_TIME: 0_i64,
        };
        this.logUtil = LogUtil::getInstance();
        this.groupCommonFactory = GroupCommonFactory::getInstance();
        this.MAX_RELOAD_TIME = ((i32::MAX) as i64) / 100000_i64;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<RTSLayerUtil>>> {
        return RTSLayerUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn createWeaponProperties(&mut self, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut currentLevel: i32, mut lastLevel: i32) -> Option<Rc<RefCell<WeaponProperties>>> {
        let mut reloadTime: i64 = jcall!(weaponProperties).getReloadTime();
        if lastLevel > currentLevel {
            reloadTime = reloadTime + 50_i64;
        } else if lastLevel < currentLevel {
            reloadTime = reloadTime - 50_i64;
        }
        let mut newWeaponProperties: Option<Rc<RefCell<WeaponProperties>>> = WeaponProperties::new(reloadTime.clone(), jcall!(weaponProperties).getTargetingTime(), jcall!(jcall!(weaponProperties).getSpeed()).getUnscaled(), jcall!(weaponProperties).getDamage() / lastLevel * currentLevel, jcall!(weaponProperties).getDissipation());
        return newWeaponProperties.clone();
    }
    

    pub fn downgrade(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        jcall!(rtsLayer).setLevel(jcall!(rtsLayer).as_RTSLayer_mut().getLevel() - 1);
        let mut partInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>> = jcall!(rtsLayer).getPartInterfaceArray();
        let mut size: i32 = (partInterfaceArray.len() as i32).clone();
        let mut partInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> = None;
        {
            let mut index: i32 = size.clone();
            while { index -= 1; index } >= 0 {
            partInterface = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> { panic!("ClassCastException: a Rust dyn PartInterface cannot be cast to BasicWeaponPart") })();
            jcall!(partInterface).setWeaponProperties({ let java_arguments = (jcall!(partInterface).getWeaponProperties(), jcall!(rtsLayer).as_RTSLayer_mut().getLevel(), jcall!(rtsLayer).as_RTSLayer_mut().getLevel() + 1,); self.createWeaponProperties(java_arguments.0, java_arguments.1, java_arguments.2) });
            }
        }
        jcall!(rtsLayer).select();
    }
    

    pub fn upgrade(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        jcall!(rtsLayer).setLevel(jcall!(rtsLayer).as_RTSLayer_mut().getLevel() + 1);
        let mut partInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>> = jcall!(rtsLayer).getPartInterfaceArray();
        let mut size: i32 = (partInterfaceArray.len() as i32).clone();
        let mut partInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> = None;
        {
            let mut index: i32 = size.clone();
            while { index -= 1; index } >= 0 {
            partInterface = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> { panic!("ClassCastException: a Rust dyn PartInterface cannot be cast to BasicWeaponPart") })();
            jcall!(partInterface).setWeaponProperties({ let java_arguments = (jcall!(partInterface).getWeaponProperties(), jcall!(rtsLayer).as_RTSLayer_mut().getLevel(), jcall!(rtsLayer).as_RTSLayer_mut().getLevel() - 1,); self.createWeaponProperties(java_arguments.0, java_arguments.1, java_arguments.2) });
            }
        }
        jcall!(rtsLayer).select();
    }
    

    pub fn getCostExponential(&mut self, mut cost: i64) -> i64 {
        return ((cost * cost * cost) / (cost * 1000_i64));
    }
    

    pub fn getWeaponPropertiesCost(&mut self, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>) -> i32 {
        let mut cost: i64 = (((jcall!(weaponProperties).getDamage() + jcall!(weaponProperties).getRange()) as i64) + ((self.MAX_RELOAD_TIME / jcall!(weaponProperties).getReloadTime()) >> 1));
        cost = self.getCostExponential(cost.clone());
        return ((cost) as i32);
    }
    

    pub fn getCost(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> i32 {
        let mut total: i32 = 0;
        let mut partInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>> = jcall!(rtsLayer).getPartInterfaceArray();
        let mut size: i32 = (partInterfaceArray.len() as i32).clone();
        let mut partInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> = None;
        let mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>> = None;
        for index in (0..=size - 1).rev() {
            partInterface = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> { panic!("ClassCastException: a Rust dyn PartInterface cannot be cast to BasicWeaponPart") })();
            weaponProperties = { let java_value = (jcall!(partInterface).getWeaponProperties()).clone(); java_value };
            total += self.getWeaponPropertiesCost(weaponProperties.clone());
        }
        return total.clone();
    }
    

    pub fn getDowngradeCost(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> i32 {
        let mut partInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>> = jcall!(rtsLayer).getPartInterfaceArray();
        let mut size: i32 = (partInterfaceArray.len() as i32).clone();
        let mut downgradeCost: i32 = 0;
        let mut partInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> = None;
        let mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>> = None;
        let mut downgradeWeaponCost: i32 = 0;
        let mut currentWeaponCost: i32 = 0;
        for index in (0..=size - 1).rev() {
            partInterface = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> { panic!("ClassCastException: a Rust dyn PartInterface cannot be cast to BasicWeaponPart") })();
            weaponProperties = { let java_value = (jcall!(partInterface).getWeaponProperties()).clone(); java_value };
            downgradeWeaponCost = { let java_value = ({ let java_arguments = ({ let java_arguments = (weaponProperties.clone(), jcall!(rtsLayer).as_RTSLayer_mut().getLevel() - 1, jcall!(rtsLayer).as_RTSLayer_mut().getLevel(),); self.createWeaponProperties(java_arguments.0, java_arguments.1, java_arguments.2) },); self.getWeaponPropertiesCost(java_arguments.0) }).clone(); java_value };
            currentWeaponCost = self.getWeaponPropertiesCost(weaponProperties.clone());
            downgradeCost += (currentWeaponCost - downgradeWeaponCost);
        }
        downgradeCost = downgradeCost * 9 / 10;
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Total Cost: ")).appendint(downgradeCost.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getDowngradeCost"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return downgradeCost.clone();
    }
    

    pub fn getUpgradeCost(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> i32 {
        let mut upgradeCost: i32 = 0;
        let mut partInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>> = jcall!(rtsLayer).getPartInterfaceArray();
        let mut size: i32 = (partInterfaceArray.len() as i32).clone();
        let mut partInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> = None;
        let mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>> = None;
        let mut upgradedWeaponCost: i32 = 0;
        let mut currentWeaponCost: i32 = 0;
        for index in (0..=size - 1).rev() {
            partInterface = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> { panic!("ClassCastException: a Rust dyn PartInterface cannot be cast to BasicWeaponPart") })();
            weaponProperties = { let java_value = (jcall!(partInterface).getWeaponProperties()).clone(); java_value };
            upgradedWeaponCost = { let java_value = ({ let java_arguments = ({ let java_arguments = (weaponProperties.clone(), jcall!(rtsLayer).as_RTSLayer_mut().getLevel() + 1, jcall!(rtsLayer).as_RTSLayer_mut().getLevel(),); self.createWeaponProperties(java_arguments.0, java_arguments.1, java_arguments.2) },); self.getWeaponPropertiesCost(java_arguments.0) }).clone(); java_value };
            currentWeaponCost = self.getWeaponPropertiesCost(weaponProperties.clone());
            upgradeCost += (upgradedWeaponCost - currentWeaponCost);
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Total Cost: ")).appendint(upgradeCost.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getUpgradeCost"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return upgradeCost.clone();
    }
    

    // throws Exception
    pub fn destroyAndClear(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = None;
        for index in (0..={ let java_bound = jcall!(list).size() - 1; java_bound }).rev() {
            rtsLayer = { let java_value = ({ let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            jcall!(rtsLayer).setDestroyed(true);
        }
        jcall!(list).clear();
    }
    

    pub fn createFakeRTSLayerHashtable(&mut self, mut baseRaceTrackGeographicMap: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::BaseRaceTrackGeographicMap::BaseRaceTrackGeographicMapDyn>>>) -> Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> {
        let mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashtable();
        let mut layerManager: Option<Rc<RefCell<FakeLayerManager>>> = FakeLayerManager::new(crate::org::allbinary::game::GameInfo::GameInfo_NONE.with(Clone::clone).clone());
        jcall!(layerManager).setGeographicMapInterface(vec![baseRaceTrackGeographicMap.clone()]);
        jcall!(hashtable).put(crate::java_into_object(AllBinaryGameLayerManager_ID.with(Clone::clone).clone()), crate::java_into_object(layerManager.clone()));
        jcall!(hashtable).put(crate::java_into_object({ let java_field = jcall!(DirectionFactory::getInstance()).NAME.clone(); java_field }.clone()), crate::java_into_object({ let java_field = jcall!(DirectionFactory::getInstance()).LEFT.clone(); java_field }.clone()));
        jcall!(hashtable).put(crate::java_into_object({ let java_field = jcall!(self.groupCommonFactory).ID.clone(); java_field }.clone()), crate::java_into_object({ let java_field = jcall!(BasicGroupFactory::getInstance()).NONE_ARRAY.clone(); java_field }.clone()));
        return hashtable.clone();
    }
}

