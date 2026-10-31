// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlayerOwnedRTSLayers {
    pub rtsLayerList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl PlayerOwnedRTSLayers {

    pub fn new_struct() -> Self {
        let mut this = Self {
            rtsLayerList: None,
        };
        this.rtsLayerList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("All: "));
        jcall!(stringBuffer).appendint(jcall!(self.rtsLayerList).size());
        return jcall!(stringBuffer).toString();
    }
}
pub trait PlayerOwnedRTSLayersDyn {
    fn as_PlayerOwnedRTSLayers(&self) -> &PlayerOwnedRTSLayers;
    fn as_PlayerOwnedRTSLayers_mut(&mut self) -> &mut PlayerOwnedRTSLayers;
    fn toString(&mut self) -> String { PlayerOwnedRTSLayers::toString(self.as_PlayerOwnedRTSLayers_mut()) }
}
impl PlayerOwnedRTSLayersDyn for PlayerOwnedRTSLayers {
    fn as_PlayerOwnedRTSLayers(&self) -> &PlayerOwnedRTSLayers { self }
    fn as_PlayerOwnedRTSLayers_mut(&mut self) -> &mut PlayerOwnedRTSLayers { self }
}


