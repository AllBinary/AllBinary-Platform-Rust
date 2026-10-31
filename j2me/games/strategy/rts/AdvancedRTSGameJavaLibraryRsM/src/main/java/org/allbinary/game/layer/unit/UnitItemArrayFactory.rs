// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.layer.unit
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::ImageItem::ImageItem;
use crate::org::allbinary::game::layer::RTSGameStrings::RTSGameStrings;
use crate::org::allbinary::game::layer::item::LayerInterfaceFactoryImageItem::LayerInterfaceFactoryImageItem;
use crate::org::allbinary::game::layer::item::RTSLayerTextAnimation::RTSLayerTextAnimation;
use crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources;
use crate::org::allbinary::game::layer::waypoint::FlagGameResources::FlagGameResources;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::image::GameFeatureImageCacheFactory::GameFeatureImageCacheFactory;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::form::item::CustomItemUtil::CustomItemUtil;
use crate::org::allbinary::graphics::form::item::ItemArraySingletonFactoryInterface::ItemArraySingletonFactoryInterface;
use crate::org::allbinary::image::ImageCache::ImageCache;
use crate::org::allbinary::layer::LayerInterfaceFactoryInterface::LayerInterfaceFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UnitItemArrayFactory {
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
    pub resources: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResourcesDyn>>>>,
}

thread_local! { pub static UnitItemArrayFactory_ITEMS: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>> = jcall!(CustomItemUtil::getInstance()).CUSTOM_ITEM_ARRAY.clone(); }
impl ItemArraySingletonFactoryInterface for UnitItemArrayFactory {
    fn getItems(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>> { Self::getItems(self) }
}

impl UnitItemArrayFactory {
    

    pub fn new_struct(mut resources: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResourcesDyn>>>>) -> Self {
        let mut this = Self {
            commonSeps: None,
            basicColor: None,
            resources: Vec::new(),
        };
        this.commonSeps = CommonSeps::getInstance();
        this.basicColor = jcall!(BasicColorFactory::getInstance()).WHITE.clone();
        this.resources = resources.clone();
        this
    }

    pub fn new(mut resources: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResourcesDyn>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(resources))))
    }
    

    // throws Exception
    pub fn createFlagItem(&mut self, mut flagResources: Option<Rc<RefCell<FlagGameResources>>>, mut layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> {
        let mut name: String = jcall!(flagResources).NAME.clone();
        let mut index: i32 = name.find(&jcall!(self.commonSeps).SPACE.clone()).map(|index| index as i32).unwrap_or(-1);
        if index >= 0 {
            name = name[(index.clone()) as usize..].to_string();
        }
        return self.createNamedFlagItem(flagResources.clone(), name.clone(), layerInterfaceFactoryInterface.clone());
    }
    

    // throws Exception
    pub fn createNamedFlagItem(&mut self, mut flagResources: Option<Rc<RefCell<FlagGameResources>>>, mut name: String, mut layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> {
        let mut IMAGE_CACHE: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>> = GameFeatureImageCacheFactory::getInstance();
        let mut image: Option<Rc<RefCell<Image>>> = jcall!(IMAGE_CACHE).getWithKey(crate::java_into_object(jcall!(flagResources).RESOURCE_ICON.clone()));
        let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> = LayerInterfaceFactoryImageItem::new(name.clone(), image.clone(), unsafe { crate::javax::microedition::lcdui::ImageItem::ImageItem_LAYOUT_DEFAULT.clone() }.clone(), jcall!(flagResources).NAME.clone(), self.basicColor.clone(), vec![RTSLayerTextAnimation::new(jcall!(RTSGameStrings::getInstance()).DRAGGABLE.clone(), image.clone())], layerInterfaceFactoryInterface.clone());
        return item.clone();
    }
    

    // throws Exception
    pub fn getItems(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>> {
        return UnitItemArrayFactory_ITEMS.with(Clone::clone).clone();
    }
    

    pub fn getResources(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResourcesDyn>>>> {
        return self.resources.clone();
    }
    

    pub fn indexOf(&mut self, mut basicGameResources: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResourcesDyn>>>) -> i32 {
        for index in (0..=(self.resources.len() as i32) - 1).rev() {
            if match (&self.resources[(index) as usize], &basicGameResources) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return index.clone();
            }
        }
        return -1;
    }
}

