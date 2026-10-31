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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::physics::velocity::VelocityProperties::VelocityProperties;
use super::VehicleFrictionProperties::VehicleFrictionProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VehicleProperties {
    pub velocityProperties: Option<Rc<RefCell<VelocityProperties>>>,
    pub vehicleFrictionProperties: Option<Rc<RefCell<VehicleFrictionProperties>>>,
}


impl VehicleProperties {
    

    pub fn new_struct(mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut vehicleFrictionProperties: Option<Rc<RefCell<VehicleFrictionProperties>>>) -> Self {
        let mut this = Self {
            velocityProperties: None,
            vehicleFrictionProperties: None,
        };
        this.velocityProperties = velocityProperties.clone();
        this.vehicleFrictionProperties = vehicleFrictionProperties.clone();
        this
    }

    pub fn new(mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut vehicleFrictionProperties: Option<Rc<RefCell<VehicleFrictionProperties>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(velocityProperties, vehicleFrictionProperties))))
    }
    

    pub fn getVehicleFrictionProperties(&mut self) -> Option<Rc<RefCell<VehicleFrictionProperties>>> {
        return self.vehicleFrictionProperties.clone();
    }
    

    pub fn setVehicleFrictionProperties(&mut self, mut vehicleFrictionProperties: Option<Rc<RefCell<VehicleFrictionProperties>>>) {
        self.vehicleFrictionProperties = vehicleFrictionProperties.clone();
    }
    

    pub fn getVelocityProperties(&mut self) -> Option<Rc<RefCell<VelocityProperties>>> {
        return self.velocityProperties.clone();
    }
}

