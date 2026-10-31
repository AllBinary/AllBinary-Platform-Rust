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
use super::TileMapPlacementVisitor::TileMapPlacementVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllAnimationsEverywhereTileMapPlacementVisitor {
    pub base: TileMapPlacementVisitor,
}


impl AllAnimationsEverywhereTileMapPlacementVisitor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TileMapPlacementVisitor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn visit(&mut self, mut lastMap: Option<Rc<RefCell<TiledMap>>>, mut mapData: Vec<Vec<i32>>) {
        let mut size: i32 = (mapData.len() as i32).clone();
        let mut size2: i32 = (mapData[(0) as usize].len() as i32) - 7;
        for index in (0..size) {
            for index2 in (0..size2).step_by((7) as usize) {
                mapData[(index) as usize][(index2) as usize] = 1;
                mapData[(index) as usize][(index2 + 1) as usize] = 17;
                mapData[(index) as usize][(index2 + 2) as usize] = 33;
                mapData[(index) as usize][(index2 + 3) as usize] = 49;
                mapData[(index) as usize][(index2 + 4) as usize] = 65;
                mapData[(index) as usize][(index2 + 5) as usize] = 81;
                mapData[(index) as usize][(index2 + 6) as usize] = 97;
            }
        }
    }
}
impl std::ops::Deref for AllAnimationsEverywhereTileMapPlacementVisitor {
    type Target = TileMapPlacementVisitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllAnimationsEverywhereTileMapPlacementVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::behavior::topview::placement::TileMapPlacementVisitor::TileMapPlacementVisitorDyn for AllAnimationsEverywhereTileMapPlacementVisitor {
    fn as_TileMapPlacementVisitor(&self) -> &crate::org::allbinary::game::behavior::topview::placement::TileMapPlacementVisitor::TileMapPlacementVisitor { &self.base }
    fn as_TileMapPlacementVisitor_mut(&mut self) -> &mut crate::org::allbinary::game::behavior::topview::placement::TileMapPlacementVisitor::TileMapPlacementVisitor { &mut self.base }
    fn visit(&mut self, mut lastMap: Option<Rc<RefCell<TiledMap>>>, mut mapData: Vec<Vec<i32>>) { Self::visit(self, lastMap, mapData) }
}


