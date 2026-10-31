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
// package org.allbinary.game.terrain
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use super::BasicTerrainInfo::BasicTerrainInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TerrainEvent {
    pub base: AllBinaryEventObject,
    pub basicTerrainInfo: Option<Rc<RefCell<BasicTerrainInfo>>>,
}


impl TerrainEvent {
    

    pub fn new_struct(mut basicTerrainInfo: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(basicTerrainInfo.clone()),
            basicTerrainInfo: None,
        };
        this.basicTerrainInfo = BasicTerrainInfo_NULL_BASIC_TERRAIN_INFO.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this.setBasicTerrainInfo(this.basicTerrainInfo.clone());
        this
    }

    pub fn new(mut basicTerrainInfo: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicTerrainInfo))))
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("TerrainEvent: \n"));
        jcall!(stringBuffer).append(String::from("LayerInterface: "));
        jcall!(stringBuffer).append(jcall!(self.basicTerrainInfo).toString());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getBasicTerrainInfo(&mut self) -> Option<Rc<RefCell<BasicTerrainInfo>>> {
        return self.basicTerrainInfo.clone();
    }
    

    pub fn setBasicTerrainInfo(&mut self, mut basicTerrainInfo: Option<Rc<RefCell<BasicTerrainInfo>>>) {
        self.basicTerrainInfo = basicTerrainInfo.clone();
    }
    

    pub fn setBasicTerrainInfoForCircularStaticPool(&mut self, mut basicTerrainInfo: Option<Rc<RefCell<BasicTerrainInfo>>>) {
        self.basicTerrainInfo = basicTerrainInfo.clone();
    }
}
impl std::ops::Deref for TerrainEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TerrainEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for TerrainEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}


