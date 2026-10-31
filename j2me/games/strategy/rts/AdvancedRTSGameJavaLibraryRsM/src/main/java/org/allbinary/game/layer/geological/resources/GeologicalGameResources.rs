// 
// AllBinary Open License Version 1 
// Copyright (c) 2002 AllBinary 
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
// package org.allbinary.game.layer.geological.resources
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources;
use crate::org::allbinary::game::layer::resources::GameGraphicsResourceUtil::GameGraphicsResourceUtil;
use crate::org::allbinary::AndroidUtil::AndroidUtil;
use crate::org::allbinary::game::configuration::feature::GraphicsFeatureFactory::GraphicsFeatureFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeologicalGameResources {
    pub base: BasicGameResources,
}

thread_local! { pub static GeologicalGameResources_instance: Option<Rc<RefCell<GeologicalGameResources>>> = GeologicalGameResources::new(); }

impl GeologicalGameResources {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicGameResources::new_struct(),
        };
        let mut ROOT: String = String::from("/resource");
        let mut SMALL: String = String::from("_20_by_20.png");
        let mut MEDIUM: String = SMALL.clone();
        let mut SIZE_FOUR: String = SMALL.clone();
        let mut SIZE_FIVE: String = SMALL.clone();
        let mut SIZE_SIX: String = SMALL.clone();
        let mut SIZE: Vec<String> = vec![SMALL.clone(), MEDIUM.clone(), SIZE_FOUR.clone(), SIZE_FIVE.clone(), SIZE_SIX.clone()];
        this.base.init(ROOT.clone(), SIZE.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn append(&mut self, mut ROOT: String, mut sizeString: String) {
        self.base.append(ROOT.clone(), sizeString.clone());
    }
    

    // throws Exception
    pub fn getString(&mut self) -> String {
        if AndroidUtil::isAndroid() {
            return jcall!(GameGraphicsResourceUtil::getInstance()).getStringForFeature(jcall!(GraphicsFeatureFactory::getInstance()).SPRITE_QUARTER_ROTATION_GRAPHICS.clone());
        } else {
            return self.base.getString();
        }
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResourcesDyn>>> {
        return GeologicalGameResources_instance.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for GeologicalGameResources {
    type Target = BasicGameResources;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GeologicalGameResources {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResourcesDyn for GeologicalGameResources {
    fn as_BasicGameResources(&self) -> &crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources { &self.base }
    fn as_BasicGameResources_mut(&mut self) -> &mut crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources { &mut self.base }
    fn append(&mut self, mut ROOT: String, mut sizeString: String) { Self::append(self, ROOT, sizeString) }
    fn getString(&mut self) -> String { Self::getString(self) }
}


