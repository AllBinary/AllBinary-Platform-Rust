// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.game.layer.unit
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonLabels::CommonLabels;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UnitWaypointStrings {
    pub WANDERING: String,
    pub THINKING: String,
    pub THINKING_ABOUT_TARGET: String,
    pub TARGET: String,
    pub KILL: String,
    pub STOP: String,
    pub WAYPOINT_DESTROYED_SHORT: String,
    pub WAYPOINT_DESTROYED: String,
    pub ALL_VISITED_SHORT: String,
    pub ALL_VISITED: String,
    pub ALREADY_THERE_SHORT: String,
    pub ALREADY_THERE: String,
    pub NEXT_PATH_NODE: String,
}

thread_local! { pub static UnitWaypointStrings_instance: Option<Rc<RefCell<UnitWaypointStrings>>> = UnitWaypointStrings::new(); }

impl UnitWaypointStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            WANDERING: String::new(),
            THINKING: String::new(),
            THINKING_ABOUT_TARGET: String::new(),
            TARGET: String::new(),
            KILL: String::new(),
            STOP: String::new(),
            WAYPOINT_DESTROYED_SHORT: String::new(),
            WAYPOINT_DESTROYED: String::new(),
            ALL_VISITED_SHORT: String::new(),
            ALL_VISITED: String::new(),
            ALREADY_THERE_SHORT: String::new(),
            ALREADY_THERE: String::new(),
            NEXT_PATH_NODE: String::new(),
        };
        this.WANDERING = String::from("Order?");
        this.THINKING = String::from("Thinking");
        this.THINKING_ABOUT_TARGET = String::from("Hmmm");
        this.TARGET = String::from("Target");
        this.KILL = String::from("Kill!");
        this.STOP = String::from("Stop");
        this.WAYPOINT_DESTROYED_SHORT = String::from("Uh Oh");
        this.WAYPOINT_DESTROYED = String::from("Waypoint Destroyed");
        this.ALL_VISITED_SHORT = String::from("Arrived");
        this.ALL_VISITED = String::from("All Visited");
        this.ALREADY_THERE_SHORT = String::from("Again?");
        this.ALREADY_THERE = String::from("Already There");
        this.NEXT_PATH_NODE = format!("{}{}", jcall!(CommonLabels::getInstance()).NEXT, String::from(" Path Node"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<UnitWaypointStrings>>> {
        return UnitWaypointStrings_instance.with(Clone::clone).clone();
    }
}

