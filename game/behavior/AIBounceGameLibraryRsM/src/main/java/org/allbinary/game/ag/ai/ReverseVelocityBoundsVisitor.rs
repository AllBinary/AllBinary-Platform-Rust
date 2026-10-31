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
// package org.allbinary.game.ag.ai
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::bounds::BoundsVisitorInterface::BoundsVisitorInterface;
use crate::org::allbinary::game::physics::velocity::VelocityInterface::VelocityInterface;
use crate::org::allbinary::game::physics::velocity::VelocityInterfaceCompositeInterface::VelocityInterfaceCompositeInterface;
use crate::org::allbinary::layer::LayerInterface::LayerInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ReverseVelocityBoundsVisitor {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub velocityInterface: Option<Rc<RefCell<dyn VelocityInterface>>>,
    pub layerInterface: Option<Rc<RefCell<dyn LayerInterface>>>,
}

impl BoundsVisitorInterface for ReverseVelocityBoundsVisitor {
    fn getX(&mut self) -> i32 { Self::getX(self) }
    fn getY(&mut self) -> i32 { Self::getY(self) }
    fn maxX(&mut self) { Self::maxX(self) }
    fn maxY(&mut self) { Self::maxY(self) }
    fn minX(&mut self) { Self::minX(self) }
    fn minY(&mut self) { Self::minY(self) }
}

impl ReverseVelocityBoundsVisitor {
    

    pub fn new_struct(mut layerInterface: Option<Rc<RefCell<dyn LayerInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            velocityInterface: None,
            layerInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.layerInterface = layerInterface.clone();
        let mut velocityInterfaceCompositeInterface: Option<Rc<RefCell<dyn VelocityInterfaceCompositeInterface>>> = panic!("ClassCastException: a Rust dyn LayerInterface cannot be cast to dyn VelocityInterfaceCompositeInterface");
        this.velocityInterface = (jcall!(velocityInterfaceCompositeInterface).getVelocityProperties()).clone() /* as VelocityInterface */;
        this
    }

    pub fn new(mut layerInterface: Option<Rc<RefCell<dyn LayerInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layerInterface))))
    }
    

    pub fn getX(&mut self) -> i32 {
        return jcall!(self.layerInterface).getXP();
    }
    

    pub fn getY(&mut self) -> i32 {
        return jcall!(self.layerInterface).getYP();
    }
    

    pub fn minX(&mut self) {
        if jcall!(jcall!(self.velocityInterface).getVelocityXBasicDecimalP()).getUnscaled() < 0 {
            jcall!(jcall!(self.velocityInterface).getVelocityXBasicDecimalP()).multiplyint(-1);
        }
    }
    

    pub fn maxX(&mut self) {
        if jcall!(jcall!(self.velocityInterface).getVelocityXBasicDecimalP()).getUnscaled() > 0 {
            jcall!(jcall!(self.velocityInterface).getVelocityXBasicDecimalP()).multiplyint(-1);
        }
    }
    

    pub fn minY(&mut self) {
        if jcall!(jcall!(self.velocityInterface).getVelocityYBasicDecimalP()).getUnscaled() < 0 {
            jcall!(jcall!(self.velocityInterface).getVelocityYBasicDecimalP()).multiplyint(-1);
        }
    }
    

    pub fn maxY(&mut self) {
        if jcall!(jcall!(self.velocityInterface).getVelocityYBasicDecimalP()).getUnscaled() > 0 {
            jcall!(jcall!(self.velocityInterface).getVelocityYBasicDecimalP()).multiplyint(-1);
        }
    }
}

