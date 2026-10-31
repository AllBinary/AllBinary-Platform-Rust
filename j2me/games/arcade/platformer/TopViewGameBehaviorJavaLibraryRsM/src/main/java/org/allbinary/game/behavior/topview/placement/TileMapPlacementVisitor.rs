// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.game.behavior.topview.placement
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::mapeditor::core::TiledMap::TiledMap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TileMapPlacementVisitor {
}


impl TileMapPlacementVisitor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn visit(&mut self, mut lastMap: Option<Rc<RefCell<TiledMap>>>, mut mapData: Vec<Vec<i32>>) {
    }
}
pub trait TileMapPlacementVisitorDyn {
    fn as_TileMapPlacementVisitor(&self) -> &TileMapPlacementVisitor;
    fn as_TileMapPlacementVisitor_mut(&mut self) -> &mut TileMapPlacementVisitor;
    fn visit(&mut self, mut lastMap: Option<Rc<RefCell<TiledMap>>>, mut mapData: Vec<Vec<i32>>) { TileMapPlacementVisitor::visit(self.as_TileMapPlacementVisitor_mut(), lastMap, mapData) }
}
impl TileMapPlacementVisitorDyn for TileMapPlacementVisitor {
    fn as_TileMapPlacementVisitor(&self) -> &TileMapPlacementVisitor { self }
    fn as_TileMapPlacementVisitor_mut(&mut self) -> &mut TileMapPlacementVisitor { self }
}


