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
// package org.allbinary.media.graphics.geography.map.racetrack
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::feature::GameFeature::GameFeature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTrackGameFeature {
    pub base: GameFeature,
}

thread_local! { pub static RaceTrackGameFeature_MINI_MAP: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>> = RaceTrackGameFeature::new(String::from("Mini Map")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>); }
thread_local! { pub static RaceTrackGameFeature_AUTO_FINISH_AI: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>> = RaceTrackGameFeature::new(String::from("Auto Finish AI")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>); }

impl RaceTrackGameFeature {
    

    pub fn new_struct(mut name: String) -> Self {
        let mut this = Self {
            base: GameFeature::new_struct(name.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name))))
    }
}
impl std::ops::Deref for RaceTrackGameFeature {
    type Target = GameFeature;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RaceTrackGameFeature {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn for RaceTrackGameFeature {
    fn as_Feature(&self) -> &crate::org::allbinary::game::configuration::feature::Feature::Feature { &self.base.base }
    fn as_Feature_mut(&mut self) -> &mut crate::org::allbinary::game::configuration::feature::Feature::Feature { &mut self.base.base }
}
impl crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn for RaceTrackGameFeature {
    fn as_GameFeature(&self) -> &crate::org::allbinary::game::configuration::feature::GameFeature::GameFeature { &self.base }
    fn as_GameFeature_mut(&mut self) -> &mut crate::org::allbinary::game::configuration::feature::GameFeature::GameFeature { &mut self.base }
}


