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
use crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface;
use crate::org::allbinary::bounds::TopULayerBounds::TopULayerBounds;
use crate::org::allbinary::game::ai::ArtificialIntelligenceInterfaceFactoryInterface::ArtificialIntelligenceInterfaceFactoryInterface;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::game::physics::velocity::VelocityInterface::VelocityInterface;
use crate::org::allbinary::game::physics::velocity::VelocityInterfaceCompositeInterface::VelocityInterfaceCompositeInterface;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::BoundBounceAI::BoundBounceAI;
use super::ReverseVelocityBoundsVisitor::ReverseVelocityBoundsVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TopUBoundBounceAIFactory {
}

impl ArtificialIntelligenceInterfaceFactoryInterface for TopUBoundBounceAIFactory {
    fn getInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> { Self::getInstance(self, hashtable, ownerLayerInterface, gameInput) }
}

impl TopUBoundBounceAIFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> {
        let mut velocityInterfaceCompositeInterface: Option<Rc<RefCell<dyn VelocityInterfaceCompositeInterface>>> = (ownerLayerInterface).clone() /* as VelocityInterfaceCompositeInterface */;
        let mut velocityInterface: Option<Rc<RefCell<dyn VelocityInterface>>> = (jcall!(velocityInterfaceCompositeInterface).getVelocityProperties()).clone() /* as VelocityInterface */;
        let mut maxDistancePerTick: i32 = (jcall!(velocityInterface).getMaxForwardVelocity() >> jcall!(jcall!(velocityInterface).getVelocityXBasicDecimalP()).getScaledFactor());
        let mut halfWidth: i32 = jcall!(ownerLayerInterface).getWidth() * 2 + maxDistancePerTick + 1;
        let mut halfHeight: i32 = jcall!(ownerLayerInterface).getHeight() * 2 + maxDistancePerTick + 1;
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(PointFactory::getInstance()).createXY(halfWidth.clone(), halfHeight + 60);
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        let mut rectangle: Option<Rc<RefCell<Rectangle>>> = Rectangle::new(point.clone(), (jcall!(displayInfo).getLastWidth() - halfWidth - jcall!(point).getX()).clone(), (jcall!(displayInfo).getLastHeight() - halfHeight - jcall!(point).getY()).clone());
        return BoundBounceAI::new(ownerLayerInterface.clone(), gameInput.clone(), TopULayerBounds::new(rectangle.clone()), ReverseVelocityBoundsVisitor::new(ownerLayerInterface.clone())).map(|reference| reference as Rc<RefCell<dyn ArtificialIntelligenceInterface>>);
    }
}

