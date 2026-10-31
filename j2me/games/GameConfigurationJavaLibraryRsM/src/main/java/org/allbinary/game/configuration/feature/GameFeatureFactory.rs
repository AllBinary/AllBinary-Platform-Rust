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
// package org.allbinary.game.configuration.feature
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::GameFeature::GameFeature;
use super::Feature::Feature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameFeatureFactory {
    pub SOUND: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
    pub MUSIC: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
    pub SCREEN_SHAKE: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>,
    pub POST_IMAGE_LOADING_MODIFICATION: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>,
    pub HEALTH_BARS: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
    pub DAMAGE_FLOATERS: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
    pub GAME_INPUT_LAYER_PROCESSOR: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
    pub ARTIFICIAL_INTELLEGENCE_PROCESSOR: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
    pub COLLIDABLE_INTERFACE_LAYER_PROCESSOR: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
    pub TICKABLE_LAYER_PROCESSOR: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
    pub CHEATING: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
    pub DROPPED_ITEMS: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
    pub DROPPED_ITEMS_FROM_DEATH: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
    pub TEST_DESTROYED_LAYER_PROCESSOR: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
    pub COLLISIONS_WITH_SOURCE: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
    pub COLLISIONS_FORCED_TWO_DIMENSIONAL: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>>,
}

thread_local! { pub static GameFeatureFactory_instance: Option<Rc<RefCell<GameFeatureFactory>>> = GameFeatureFactory::new(); }

impl GameFeatureFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<GameFeatureFactory>>> {
        return GameFeatureFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            SOUND: None,
            MUSIC: None,
            SCREEN_SHAKE: None,
            POST_IMAGE_LOADING_MODIFICATION: None,
            HEALTH_BARS: None,
            DAMAGE_FLOATERS: None,
            GAME_INPUT_LAYER_PROCESSOR: None,
            ARTIFICIAL_INTELLEGENCE_PROCESSOR: None,
            COLLIDABLE_INTERFACE_LAYER_PROCESSOR: None,
            TICKABLE_LAYER_PROCESSOR: None,
            CHEATING: None,
            DROPPED_ITEMS: None,
            DROPPED_ITEMS_FROM_DEATH: None,
            TEST_DESTROYED_LAYER_PROCESSOR: None,
            COLLISIONS_WITH_SOURCE: None,
            COLLISIONS_FORCED_TWO_DIMENSIONAL: None,
        };
        this.SOUND = GameFeature::new(String::from("Sound")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this.MUSIC = GameFeature::new(String::from("Music")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this.SCREEN_SHAKE = Feature::new(String::from("Screen Shake")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>);
        this.POST_IMAGE_LOADING_MODIFICATION = Feature::new(String::from("Post Image Loading Modification (Scaling, Rotation, or Other)")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>);
        this.HEALTH_BARS = GameFeature::new(String::from("Health Bars")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this.DAMAGE_FLOATERS = GameFeature::new(String::from("Damage Floaters")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this.GAME_INPUT_LAYER_PROCESSOR = GameFeature::new(String::from("Game Input Layer Processor")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this.ARTIFICIAL_INTELLEGENCE_PROCESSOR = GameFeature::new(String::from("Artificial Intelligence Layer Processor")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this.COLLIDABLE_INTERFACE_LAYER_PROCESSOR = GameFeature::new(String::from("Collidable Interface Layer Processor")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this.TICKABLE_LAYER_PROCESSOR = GameFeature::new(String::from("Tickable Layer Processor")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this.CHEATING = GameFeature::new(String::from("Cheating")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this.DROPPED_ITEMS = GameFeature::new(String::from("Drops")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this.DROPPED_ITEMS_FROM_DEATH = GameFeature::new(String::from("Drops from Dead")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this.TEST_DESTROYED_LAYER_PROCESSOR = GameFeature::new(String::from("Destroyed Layer Processor")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this.COLLISIONS_WITH_SOURCE = GameFeature::new(String::from("Allow Collisions With Source")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this.COLLISIONS_FORCED_TWO_DIMENSIONAL = GameFeature::new(String::from("Collisions Forced Two Dimensional")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::GameFeature::GameFeatureDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

