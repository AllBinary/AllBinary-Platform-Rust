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
// package org.allbinary.game.layer.waypoint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources;
use crate::org::allbinary::game::layer::resources::GameGraphicsResourceUtil::GameGraphicsResourceUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FlagGameResources {
    pub base: BasicGameResources,
    pub NAME: String,
    pub RESOURCE_ICON_ANIMATION: String,
    pub RESOURCE_ICON_BUILD: String,
    pub RESOURCE_ICON: String,
    pub ICON_SIZE: String,
}

thread_local! { pub static FlagGameResources_ICON: String = String::from("_icon"); }
thread_local! { pub static FlagGameResources_BUILD: String = String::from("_build"); }
thread_local! { pub static FlagGameResources_DESTROY: String = String::from("_destroy"); }
thread_local! { pub static FlagGameResources_ICON_BUILD: String = format!("{}{}", String::from("_icon"), FlagGameResources_BUILD.with(Clone::clone)); }

impl FlagGameResources {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicGameResources::new_struct(),
            NAME: String::new(),
            RESOURCE_ICON_ANIMATION: String::new(),
            RESOURCE_ICON_BUILD: String::new(),
            RESOURCE_ICON: String::new(),
            ICON_SIZE: String::new(),
        };
        this.NAME = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.RESOURCE_ICON_ANIMATION = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.RESOURCE_ICON_BUILD = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.RESOURCE_ICON = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.ICON_SIZE = String::from("_64_by_64.png");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn append(&mut self, mut ROOT: String, mut sizeString: String) {
        self.base.append(ROOT.clone(), sizeString.clone());
        let mut string: String = jcall!(GameGraphicsResourceUtil::getInstance()).getName();
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).delete(0, jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).length());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(ROOT.clone());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(FlagGameResources_DESTROY.with(Clone::clone).clone());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(string.clone());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(sizeString.clone());
        self.RESOURCE_DESTROY = jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).toString();
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).delete(0, jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).length());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(FlagGameResources_ICON.with(Clone::clone).clone());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(ROOT.clone());
        self.RESOURCE_ICON_ANIMATION = jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).toString();
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).delete(0, jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).length());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(ROOT.clone());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(FlagGameResources_ICON.with(Clone::clone).clone());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(string.clone());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(self.ICON_SIZE.clone());
        self.RESOURCE_ICON = jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).toString();
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).delete(0, jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).length());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(ROOT.clone());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(FlagGameResources_ICON_BUILD.with(Clone::clone).clone());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(string.clone());
        jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).append(self.ICON_SIZE.clone());
        self.RESOURCE_ICON_BUILD = jcall!(crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources_stringBuffer.with(Clone::clone)).toString();
    }
}
impl std::ops::Deref for FlagGameResources {
    type Target = BasicGameResources;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FlagGameResources {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResourcesDyn for FlagGameResources {
    fn as_BasicGameResources(&self) -> &crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources { &self.base }
    fn as_BasicGameResources_mut(&mut self) -> &mut crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources { &mut self.base }
    fn append(&mut self, mut ROOT: String, mut sizeString: String) { Self::append(self, ROOT, sizeString) }
}


