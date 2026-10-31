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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::analog::AnalogLocationInputFactory::AnalogLocationInputFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LocalPlayerInputIdFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub MAX_NUMBER_OF_PLAYERS: i32,
    pub MAX_FAST_DEVICEID: i32,
    pub deviceIdToplayerId: Vec<i32>,
    pub playerIdToDeviceId: Vec<i32>,
    pub playersInPlay: Vec<bool>,
    pub totalDevicesMapped: i32,
}

thread_local! { pub static LocalPlayerInputIdFactory_instance: Option<Rc<RefCell<LocalPlayerInputIdFactory>>> = LocalPlayerInputIdFactory::new(); }

impl LocalPlayerInputIdFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            MAX_NUMBER_OF_PLAYERS: 0,
            MAX_FAST_DEVICEID: 0,
            deviceIdToplayerId: Vec::new(),
            playerIdToDeviceId: Vec::new(),
            playersInPlay: Vec::new(),
            totalDevicesMapped: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.MAX_NUMBER_OF_PLAYERS = 6;
        this.MAX_FAST_DEVICEID = 65;
        this.deviceIdToplayerId = vec![0; (this.MAX_FAST_DEVICEID) as usize];
        this.playerIdToDeviceId = vec![0; (this.MAX_NUMBER_OF_PLAYERS) as usize];
        this.playersInPlay = vec![false; (this.MAX_NUMBER_OF_PLAYERS) as usize];
        this.totalDevicesMapped = 0;
        for index in (0..=(this.deviceIdToplayerId.len() as i32) - 1).rev() {
            this.deviceIdToplayerId[(index) as usize] = -1;
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LocalPlayerInputIdFactory>>> {
        return LocalPlayerInputIdFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getDeviceId(&mut self, mut playerInputId: i32) -> i32 {
        return self.playerIdToDeviceId[(playerInputId) as usize].clone();
    }
    

    pub fn getPlayerForDevice(&mut self, mut deviceId: i32) -> i32 {
        let mut index: i32 = 0;
        if deviceId < self.MAX_FAST_DEVICEID {
            let mut playerInputId: i32 = self.deviceIdToplayerId[(deviceId) as usize].clone();
            if playerInputId != -1 {
                return playerInputId.clone();
            }
        } else {
            for index in (0..=self.totalDevicesMapped - 1).rev() {
                if self.playerIdToDeviceId[(index) as usize] == deviceId {
                    return index.clone();
                }
            }
        }
        index = self.totalDevicesMapped.clone();
        if deviceId < self.MAX_FAST_DEVICEID {
            self.deviceIdToplayerId[(deviceId) as usize] = index.clone();
        } else {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Added DeviceId: ")).appendint(deviceId.clone()).append(String::from(" beyond fast Id list")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getPlayerForDevice"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        self.playerIdToDeviceId[(index) as usize] = deviceId.clone();
        jcall!(AnalogLocationInputFactory::getInstance()).addPlayer(self.totalDevicesMapped.clone());
        self.totalDevicesMapped += 1;
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Added DeviceId: ")).appendint(deviceId.clone()).append(String::from(" at: ")).appendint(index.clone()).append(String::from(" Total: ")).appendint(self.totalDevicesMapped.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getPlayerForDevice"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return index.clone();
    }
    

    pub fn getTotalDevicesInPlay(&mut self) -> i32 {
        let mut total: i32 = 0;
        for index in (0..=self.totalDevicesMapped - 1).rev() {
            if self.playersInPlay[(index) as usize] == true {
                total += 1;
            }
        }
        return total.clone();
    }
    

    pub fn isPlayerInPlay(&mut self, mut playerInputId: i32) -> bool {
        return self.playersInPlay[(playerInputId) as usize].clone();
    }
    

    pub fn setPlayerInPlay(&mut self, mut playerInputId: i32) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Setting PlayerInPlay with playerInputId: ")).appendint(playerInputId.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setPlayerInPlay"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.playersInPlay[(playerInputId) as usize] = true;
    }
    

    pub fn setPlayerOutOfPlay(&mut self, mut playerInputId: i32) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Setting PlayerOutOfPlay with playerInputId: ")).appendint(playerInputId.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setPlayerOutOfPlay"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.playersInPlay[(playerInputId) as usize] = false;
    }
    

    pub fn getPlayersInPlay(&mut self) -> Vec<bool> {
        return self.playersInPlay.clone();
    }
}

