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
// package org.allbinary.game.terrain
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::util::event::AllBinaryEventCircularPool::AllBinaryEventCircularPool;
use super::TerrainEventFactory::TerrainEventFactory;
use super::TerrainEvent::TerrainEvent;
use super::BasicTerrainInfo::BasicTerrainInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TerrainEventCircularStaticPool {
    pub EVENT_POOL: Option<Rc<RefCell<AllBinaryEventCircularPool>>>,
}

thread_local! { pub static TerrainEventCircularStaticPool_instance: Option<Rc<RefCell<TerrainEventCircularStaticPool>>> = TerrainEventCircularStaticPool::new(); }

impl TerrainEventCircularStaticPool {

    pub fn new_struct() -> Self {
        let mut this = Self {
            EVENT_POOL: None,
        };
        this.EVENT_POOL = AllBinaryEventCircularPool::new(20);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<TerrainEventCircularStaticPool>>> {
        return TerrainEventCircularStaticPool_instance.with(Clone::clone).clone();
    }
    

    pub fn init(&mut self) {
        jcall!(self.EVENT_POOL).initAllBinaryEventCircularPool(TerrainEventFactory::new());
    }
    

    // throws Exception
    // synchronized
    pub fn getNext(&mut self, mut basicTerrainInfo: Option<Rc<RefCell<BasicTerrainInfo>>>) -> Option<Rc<RefCell<TerrainEvent>>> {
        let mut trackingEvent: Option<Rc<RefCell<TerrainEvent>>> = { let java_object = (jcall!(self.EVENT_POOL).getNextInstance()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<TerrainEvent>>>>().expect("ClassCastException").clone()) };
        jcall!(trackingEvent).setBasicTerrainInfoForCircularStaticPool(basicTerrainInfo.clone());
        return trackingEvent.clone();
    }
}

