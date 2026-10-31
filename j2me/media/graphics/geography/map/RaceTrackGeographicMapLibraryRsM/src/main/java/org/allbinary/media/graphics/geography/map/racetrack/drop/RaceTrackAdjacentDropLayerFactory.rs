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
// package org.allbinary.media.graphics.geography.map.racetrack.drop
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use crate::org::allbinary::layer::LayerInterfaceFactoryInterface::LayerInterfaceFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTrackAdjacentDropLayerFactory {
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
}

pub static mut RaceTrackAdjacentDropLayerFactory_DROP_LAYER_FACTORY: std::cell::LazyCell<Option<Rc<RefCell<RaceTrackAdjacentDropLayerFactory>>>> = std::cell::LazyCell::new(|| RaceTrackAdjacentDropLayerFactory::new());

impl RaceTrackAdjacentDropLayerFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            list: None,
            basicArrayListUtil: None,
        };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn clear(&mut self) {
        jcall!(self.list).clear();
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<RaceTrackAdjacentDropLayerFactory>>> {
        return unsafe { RaceTrackAdjacentDropLayerFactory_DROP_LAYER_FACTORY.clone() }.clone();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return jcall!(self.list).size();
    }
    

    pub fn getRandomInstance(&mut self) -> Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>> {
        return { let java_object = jcall!(self.basicArrayListUtil).getRandom(self.list.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn add(&mut self, mut layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>) {
        jcall!(self.list).add(crate::java_into_object(layerInterfaceFactoryInterface.clone()));
    }
}

