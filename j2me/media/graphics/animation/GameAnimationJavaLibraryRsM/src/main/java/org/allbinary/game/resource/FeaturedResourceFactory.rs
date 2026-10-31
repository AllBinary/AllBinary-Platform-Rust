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
// package org.allbinary.game.resource
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureControlledInterface::GameFeatureControlledInterface;
use crate::org::allbinary::game::configuration::feature::GraphicsFeatureFactory::GraphicsFeatureFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FeaturedResourceFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub ANIMATION_FEATURES: String,
    pub IMAGE_LABEL: String,
    pub IMAGE_GRAPHICS_ARRAY: String,
    pub IMAGE_GRAPHICS_ROTATION: String,
    pub SPRITE_QUARTER: String,
    pub SPRITE_FULL: String,
    pub IS_LOADING_LEVEL_LABEL: String,
    pub IS_FEATURE: String,
    pub GAME_FEATURE_CONTROLLED: String,
}


impl FeaturedResourceFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            list: None,
            commonStrings: None,
            ANIMATION_FEATURES: String::new(),
            IMAGE_LABEL: String::new(),
            IMAGE_GRAPHICS_ARRAY: String::new(),
            IMAGE_GRAPHICS_ROTATION: String::new(),
            SPRITE_QUARTER: String::new(),
            SPRITE_FULL: String::new(),
            IS_LOADING_LEVEL_LABEL: String::new(),
            IS_FEATURE: String::new(),
            GAME_FEATURE_CONTROLLED: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.commonStrings = CommonStrings::getInstance();
        this.ANIMATION_FEATURES = String::from("Animation Features: Vector: ");
        this.IMAGE_LABEL = String::from(" Image: ");
        this.IMAGE_GRAPHICS_ARRAY = String::from("Image Array: ");
        this.IMAGE_GRAPHICS_ROTATION = String::from("Image Rotate: ");
        this.SPRITE_QUARTER = String::from(" Sprite Quarter: ");
        this.SPRITE_FULL = String::from(" Sprite Full: ");
        this.IS_LOADING_LEVEL_LABEL = String::from(" isLoadingLevel ");
        this.IS_FEATURE = String::from(" isFeature: ");
        this.GAME_FEATURE_CONTROLLED = String::from("GameFeatureControlledInterface: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut level: i32) {
        let mut size: i32 = jcall!(self.list).size();
        for index in (0..size) {
            let mut featureInterface: Option<Rc<RefCell<dyn GameFeatureControlledInterface>>> = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn GameFeatureControlledInterface>>>>().expect("ClassCastException").clone()) };
            let mut isLoadingLevel: bool = jcall!(featureInterface).isLoadingLevel(level.clone());
            let mut isFeature: bool = jcall!(featureInterface).isFeature();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(self.GAME_FEATURE_CONTROLLED.clone()).append(jcall!(featureInterface).toString()).append(self.IS_LOADING_LEVEL_LABEL.clone()).append(jcall!(ResourceLoadingLevelFactory::ResourceLoadingLevelFactory::getInstance()).getLevelString(level.clone())).append(jcall!(CommonSeps::getInstance()).COLON_SEP.clone()).appendboolean(isLoadingLevel.clone()).append(self.IS_FEATURE.clone()).appendboolean(isFeature.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if isLoadingLevel && isFeature {
                jcall!(featureInterface).init(level.clone());
            }
        }
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut graphicsFeatureFactory: Option<Rc<RefCell<GraphicsFeatureFactory>>> = GraphicsFeatureFactory::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.ANIMATION_FEATURES.clone());
        jcall!(stringBuffer).appendboolean(jcall!(features).isFeature(jcall!(graphicsFeatureFactory).VECTOR_GRAPHICS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)));
        jcall!(stringBuffer).append(self.IMAGE_LABEL.clone());
        jcall!(stringBuffer).appendboolean(jcall!(features).isFeature(jcall!(graphicsFeatureFactory).IMAGE_GRAPHICS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        if jcall!(features).isFeature(jcall!(graphicsFeatureFactory).IMAGE_GRAPHICS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(self.IMAGE_GRAPHICS_ARRAY.clone());
            jcall!(stringBuffer).appendboolean(jcall!(features).isFeature(jcall!(graphicsFeatureFactory).IMAGE_TO_ARRAY_GRAPHICS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)));
            jcall!(stringBuffer).append(self.IMAGE_GRAPHICS_ROTATION.clone());
            jcall!(stringBuffer).appendboolean(jcall!(features).isFeature(jcall!(graphicsFeatureFactory).IMAGE_TO_ARRAY_GRAPHICS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)));
            jcall!(stringBuffer).append(self.SPRITE_QUARTER.clone());
            jcall!(stringBuffer).appendboolean(jcall!(features).isFeature(jcall!(graphicsFeatureFactory).SPRITE_QUARTER_ROTATION_GRAPHICS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)));
            jcall!(stringBuffer).append(self.SPRITE_FULL.clone());
            jcall!(stringBuffer).appendboolean(jcall!(features).isFeature(jcall!(graphicsFeatureFactory).SPRITE_FULL_GRAPHICS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)));
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    pub fn clear(&mut self) {
        jcall!(self.list).clear();
    }
    

    pub fn add(&mut self, mut featureInterface: Option<Rc<RefCell<dyn GameFeatureControlledInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).START_LABEL.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(featureInterface.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).ADD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(self.list).add(crate::java_into_object(featureInterface.clone()));
    }
    

    pub fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.list.clone();
    }
}
pub trait FeaturedResourceFactoryDyn {
    fn as_FeaturedResourceFactory(&self) -> &FeaturedResourceFactory;
    fn as_FeaturedResourceFactory_mut(&mut self) -> &mut FeaturedResourceFactory;
    fn init(&mut self, mut level: i32) { FeaturedResourceFactory::init(self.as_FeaturedResourceFactory_mut(), level) }
    fn clear(&mut self) { FeaturedResourceFactory::clear(self.as_FeaturedResourceFactory_mut()) }
    fn add(&mut self, mut featureInterface: Option<Rc<RefCell<dyn GameFeatureControlledInterface>>>) { FeaturedResourceFactory::add(self.as_FeaturedResourceFactory_mut(), featureInterface) }
    fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { FeaturedResourceFactory::getList(self.as_FeaturedResourceFactory_mut()) }
}
impl FeaturedResourceFactoryDyn for FeaturedResourceFactory {
    fn as_FeaturedResourceFactory(&self) -> &FeaturedResourceFactory { self }
    fn as_FeaturedResourceFactory_mut(&mut self) -> &mut FeaturedResourceFactory { self }
}


