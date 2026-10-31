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
// package org.allbinary.game.layer.resources
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::MainFeatureFactory::MainFeatureFactory;
use crate::org::allbinary::game::layer::GameLayerPlacementInterfaceBasicArrayListFactoryInterfaceFactory::GameLayerPlacementInterfaceBasicArrayListFactoryInterfaceFactory;
use crate::org::allbinary::game::layer::LayerPlacementInterfaceBasicArrayListFactoryInterface::LayerPlacementInterfaceBasicArrayListFactoryInterface;
use crate::org::allbinary::game::resource::ResourceLoadingLevelFactory::ResourceLoadingLevelFactory;
use crate::org::allbinary::image::ImageCache::ImageCache;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::OnDemandResourcesFactory::OnDemandResourcesFactory;
use super::OnDemandResources::OnDemandResources;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OnDemandLoader {
    pub onDemandResourcesFactory: Option<Rc<RefCell<OnDemandResourcesFactory>>>,
    pub gameLayerPlacementInterfaceBasicArrayListFactoryInterfaceFactory: Option<Rc<RefCell<dyn GameLayerPlacementInterfaceBasicArrayListFactoryInterfaceFactory>>>,
    pub resourceAnimationInterfaceFactoryInterfaceFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactoryDyn>>>,
}


impl OnDemandLoader {
    

    pub fn new_struct(mut onDemandResourcesFactory: Option<Rc<RefCell<OnDemandResourcesFactory>>>, mut gameLayerPlacementInterfaceBasicArrayListFactoryInterfaceFactory: Option<Rc<RefCell<dyn GameLayerPlacementInterfaceBasicArrayListFactoryInterfaceFactory>>>, mut resourceAnimationInterfaceFactoryInterfaceFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactoryDyn>>>) -> Self {
        let mut this = Self {
            onDemandResourcesFactory: None,
            gameLayerPlacementInterfaceBasicArrayListFactoryInterfaceFactory: None,
            resourceAnimationInterfaceFactoryInterfaceFactory: None,
        };
        this.onDemandResourcesFactory = onDemandResourcesFactory.clone();
        this.gameLayerPlacementInterfaceBasicArrayListFactoryInterfaceFactory = gameLayerPlacementInterfaceBasicArrayListFactoryInterfaceFactory.clone();
        this.resourceAnimationInterfaceFactoryInterfaceFactory = resourceAnimationInterfaceFactoryInterfaceFactory.clone();
        this
    }

    pub fn new(mut onDemandResourcesFactory: Option<Rc<RefCell<OnDemandResourcesFactory>>>, mut gameLayerPlacementInterfaceBasicArrayListFactoryInterfaceFactory: Option<Rc<RefCell<dyn GameLayerPlacementInterfaceBasicArrayListFactoryInterfaceFactory>>>, mut resourceAnimationInterfaceFactoryInterfaceFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(onDemandResourcesFactory, gameLayerPlacementInterfaceBasicArrayListFactoryInterfaceFactory, resourceAnimationInterfaceFactoryInterfaceFactory))))
    }
    

    // throws Exception
    pub fn init(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut level: i32, mut loadingString: String) {
        let mut resourceLoadingLevelFactory: Option<Rc<RefCell<ResourceLoadingLevelFactory>>> = ResourceLoadingLevelFactory::getInstance();
        if jcall!(Features::getInstance()).isFeature(jcall!(MainFeatureFactory::getInstance()).LOAD_ALL.clone()) {
            if jcall!(self.resourceAnimationInterfaceFactoryInterfaceFactory).isInitialized() {
                return;
            }
            self.initAll(imageCache.clone(), loadingString.clone());
        } else if level != jcall!(jcall!(resourceLoadingLevelFactory).LOAD_ALL).getLevel() {
            let mut layerPlacementInterfaceBasicArrayListFactoryInterface: Option<Rc<RefCell<dyn LayerPlacementInterfaceBasicArrayListFactoryInterface>>> = jcall!(self.gameLayerPlacementInterfaceBasicArrayListFactoryInterfaceFactory).getInstance(level.clone());
            self.initLevel(layerPlacementInterfaceBasicArrayListFactoryInterface.clone(), imageCache.clone(), level.clone(), loadingString.clone());
        }
    }
    

    // throws Exception
    pub fn initAll(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut loadingString: String) {
        let mut index: i32 = 0;
        let mut portion: i32 = 120;
        let mut onDemandResources: Option<Rc<RefCell<OnDemandResources>>> = None;
        {
            let mut onDemandIndex: i32 = jcall!(self.onDemandResourcesFactory).size();
            while { onDemandIndex -= 1; onDemandIndex } >= 0 {
            onDemandResources = jcall!(self.onDemandResourcesFactory).remove(onDemandIndex.clone());
            if !match (&onDemandResources, &OnDemandResources_NULL_ON_DEMAND_RESOURCES.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                index = jcall!(onDemandResources).initAt(imageCache.clone(), self.resourceAnimationInterfaceFactoryInterfaceFactory.clone(), portion.clone(), loadingString.clone(), index.clone());
            }
            }
        }
    }
    

    // throws Exception
    pub fn initLevel(&mut self, mut layerPlacementInterfaceBasicArrayListFactoryInterface: Option<Rc<RefCell<dyn LayerPlacementInterfaceBasicArrayListFactoryInterface>>>, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut level: i32, mut loadingString: String) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut portion: i32 = 120;
        let mut index: i32 = 0;
        let mut onDemandArray: Vec<i32> = jcall!(layerPlacementInterfaceBasicArrayListFactoryInterface).getOnDemandResources();
        PreLogUtil::put(jcall!(StringMaker::new()).append(jcall!(commonStrings).START.clone()).appendint((onDemandArray.len() as i32).clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("initLevel"));
        let mut onDemandResources: Option<Rc<RefCell<OnDemandResources>>> = None;
        {
            let mut onDemandIndex: i32 = (onDemandArray.len() as i32).clone();
            while { onDemandIndex -= 1; onDemandIndex } >= 0 {
            onDemandResources = jcall!(self.onDemandResourcesFactory).remove(onDemandArray[(onDemandIndex) as usize].clone());
            if !match (&onDemandResources, &OnDemandResources_NULL_ON_DEMAND_RESOURCES.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                index = jcall!(onDemandResources).initAt(imageCache.clone(), self.resourceAnimationInterfaceFactoryInterfaceFactory.clone(), portion.clone(), loadingString.clone(), index.clone());
            }
            }
        }
    }
}

