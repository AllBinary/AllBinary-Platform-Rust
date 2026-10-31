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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::transition::shake::NoShakeAnimationListener::NoShakeAnimationListener;
use crate::org::allbinary::animation::transition::shake::ShakeAnimationListener::ShakeAnimationListener;
use crate::org::allbinary::animation::transition::shake::ShakeAnimationListenerFactory::ShakeAnimationListenerFactory;
use crate::org::allbinary::game::configuration::GameConfigurationCentral::GameConfigurationCentral;
use crate::org::allbinary::game::tick::TickableInterface::TickableInterface;
use crate::org::allbinary::media::AllBinaryNoVibration::AllBinaryNoVibration;
use crate::org::allbinary::media::AllBinaryVibration::AllBinaryVibration;
use crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationME;
use super::WaypointBehaviorBase::WaypointBehaviorBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdvancedRTSProperties {
    pub shakeListener: Option<Rc<RefCell<dyn crate::org::allbinary::animation::transition::shake::ShakeAnimationListener::ShakeAnimationListenerDyn>>>,
    pub vibration: Option<Rc<RefCell<dyn crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationMEDyn>>>,
    pub duration: i32,
    pub waypointBehaviorBase: Option<Rc<RefCell<dyn TickableInterface>>>,
}


impl AdvancedRTSProperties {
    

    pub fn createPropertiesSimulated() -> Option<Rc<RefCell<AdvancedRTSProperties>>> {
        return AdvancedRTSProperties::new(crate::org::allbinary::animation::transition::shake::NoShakeAnimationListener::NoShakeAnimationListener_NO_SHAKE_ANIMATION_LISTENER.with(Clone::clone).clone(), unsafe { crate::org::allbinary::media::AllBinaryNoVibration::AllBinaryNoVibration_NO_VIBRATION.clone() }.clone(), 0, WaypointBehaviorBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>));
    }
    

    pub fn createProperties() -> Option<Rc<RefCell<AdvancedRTSProperties>>> {
        return AdvancedRTSProperties::new(ShakeAnimationListenerFactory::getInstance(), AllBinaryVibration::getInstance(), ((jcall!(jcall!(GameConfigurationCentral::getInstance()).VIBRATION).getValue()).unwrap() as i32) * 100, WaypointBehaviorBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>));
    }
    

    pub fn new_struct(mut shakeListener: Option<Rc<RefCell<dyn crate::org::allbinary::animation::transition::shake::ShakeAnimationListener::ShakeAnimationListenerDyn>>>, mut vibration: Option<Rc<RefCell<dyn crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationMEDyn>>>, mut duration: i32, mut waypointBehaviorBase: Option<Rc<RefCell<dyn TickableInterface>>>) -> Self {
        let mut this = Self {
            shakeListener: None,
            vibration: None,
            duration: 0,
            waypointBehaviorBase: None,
        };
        this.shakeListener = shakeListener.clone();
        this.vibration = vibration.clone();
        this.duration = duration.clone();
        this.waypointBehaviorBase = waypointBehaviorBase.clone();
        this
    }

    pub fn new(mut shakeListener: Option<Rc<RefCell<dyn crate::org::allbinary::animation::transition::shake::ShakeAnimationListener::ShakeAnimationListenerDyn>>>, mut vibration: Option<Rc<RefCell<dyn crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationMEDyn>>>, mut duration: i32, mut waypointBehaviorBase: Option<Rc<RefCell<dyn TickableInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(shakeListener, vibration, duration, waypointBehaviorBase))))
    }
}

