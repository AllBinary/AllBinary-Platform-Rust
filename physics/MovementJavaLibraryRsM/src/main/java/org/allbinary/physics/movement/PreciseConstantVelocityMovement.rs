// Generated Code Do Not Modify
// package org.allbinary.physics.movement
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer;
use crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityProperties;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use crate::org::allbinary::logic::math::MathUtil::MathUtil;
use crate::org::allbinary::logic::math::ScaleFactorFactory::ScaleFactorFactory;
use super::BasicConstantVelocityMovement::BasicConstantVelocityMovement;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PreciseConstantVelocityMovement {
    pub base: BasicConstantVelocityMovement,
    pub accumulatedX: i32,
    pub accumulatedY: i32,
    pub accumulatedZ: i32,
    pub factorValue: i32,
}


impl PreciseConstantVelocityMovement {
    

    pub fn new_struct(mut basicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>) -> Self {
        let mut this = Self {
            base: BasicConstantVelocityMovement::new_struct(basicDecimal.clone(), velocityProperties.clone()),
            accumulatedX: 0,
            accumulatedY: 0,
            accumulatedZ: 0,
            factorValue: 0,
        };
        this.factorValue = jcall!(ScaleFactorFactory::getInstance()).DEFAULT_SCALE_VALUE.clone();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut basicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicDecimal, velocityProperties))))
    }
    

    pub fn init(&mut self, mut speedBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut angle: i32, mut otherAngle: i32) {
    }
    

    // throws Exception
    pub fn process(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) {
        let mut mathUtil: Option<Rc<RefCell<MathUtil>>> = MathUtil::getInstance();
        let mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>> = self.getVelocityProperties();
        self.accumulatedX = (((self.accumulatedX + jcall!(jcall!(velocityProperties).getVelocityXBasicDecimalP()).getUnscaled())) as i32);
        self.accumulatedY = (((self.accumulatedY + jcall!(jcall!(velocityProperties).getVelocityYBasicDecimalP()).getUnscaled())) as i32);
        self.accumulatedZ = (((self.accumulatedZ + jcall!(jcall!(velocityProperties).getVelocityZBasicDecimalP()).getUnscaled())) as i32);
        jcall!(layer).moveDXYZ(self.accumulatedX / self.factorValue, self.accumulatedY / self.factorValue, self.accumulatedZ / self.factorValue);
        if jcall!(mathUtil).abs(self.accumulatedX.clone()) > self.factorValue {
            self.accumulatedX = 0;
        }
        if jcall!(mathUtil).abs(self.accumulatedY.clone()) > self.factorValue {
            self.accumulatedY = 0;
        }
        if jcall!(mathUtil).abs(self.accumulatedZ.clone()) > self.factorValue {
            self.accumulatedZ = 0;
        }
    }
}
impl std::ops::Deref for PreciseConstantVelocityMovement {
    type Target = BasicConstantVelocityMovement;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PreciseConstantVelocityMovement {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn for PreciseConstantVelocityMovement {
    fn as_BasicConstantVelocityMovement(&self) -> &crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement { &self.base }
    fn as_BasicConstantVelocityMovement_mut(&mut self) -> &mut crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement { &mut self.base }
}
impl crate::org::allbinary::physics::movement::Movement::MovementDyn for PreciseConstantVelocityMovement {
    fn as_Movement(&self) -> &crate::org::allbinary::physics::movement::Movement::Movement { &self.base.base }
    fn as_Movement_mut(&mut self) -> &mut crate::org::allbinary::physics::movement::Movement::Movement { &mut self.base.base }
}
impl crate::org::allbinary::physics::movement::MovementInterface::MovementInterface for PreciseConstantVelocityMovement {
    fn init(&mut self, mut speedBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut angle: i32, mut otherAngle: i32) { Self::init(self, speedBasicDecimal, angle, otherAngle) }
    fn process(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>>) { Self::process(self, layer) }
    fn stop(&mut self) { crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement::stop(crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn::as_BasicConstantVelocityMovement_mut(self)) }
}
impl crate::org::allbinary::game::physics::velocity::VelocityInterfaceCompositeInterface::VelocityInterfaceCompositeInterface for PreciseConstantVelocityMovement {
    fn getVelocityProperties(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>> { crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovement::getVelocityProperties(crate::org::allbinary::physics::movement::BasicConstantVelocityMovement::BasicConstantVelocityMovementDyn::as_BasicConstantVelocityMovement_mut(self)) }
}


