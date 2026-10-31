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
use crate::org::allbinary::game::rand::MyRandomFactory::MyRandomFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::mapeditor::core::TileLayer::TileLayer;
use crate::org::mapeditor::core::TiledMap::TiledMap;
use super::TileMapPlacementVisitor::TileMapPlacementVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PropsTileMapPlacementVisitor {
    pub base: TileMapPlacementVisitor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub randomFactory: Option<Rc<RefCell<MyRandomFactory>>>,
}


impl PropsTileMapPlacementVisitor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TileMapPlacementVisitor::new_struct(),
            logUtil: None,
            randomFactory: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.randomFactory = MyRandomFactory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn visit(&mut self, mut lastMap: Option<Rc<RefCell<TiledMap>>>, mut mapData: Vec<Vec<i32>>) {
        let mut layerIndex: i32 = 0;
        let mut tileLayer: Option<Rc<RefCell<TileLayer>>> = ((jcall!(lastMap).getLayer(layerIndex.clone())).clone() /* as TileLayer */);
        let mut mapArray: Vec<Vec<i32>> = jcall!(tileLayer).getMapArray();
        let mut size: i32 = (mapData.len() as i32).clone();
        let mut size2: i32 = (mapData[(0) as usize].len() as i32).clone();
        for index in (0..size) {
            for index2 in (0..size2) {
                if mapArray[(index) as usize][(index2) as usize] == 14 {
                } else {
                    self.visit2(mapArray.clone(), mapData.clone(), index.clone(), index2.clone());
                }
                if index % 7 == 0 {
                    if mapArray[(index) as usize][(index2) as usize] == 19 {
                        let mut randomInt: i32 = jcall!(self.randomFactory).getAbsoluteNextIntAllowZero(3);
                        if randomInt == 0 {
                            mapData[(index) as usize][(index2) as usize] = 1;
                        } else if randomInt == 1 {
                            mapData[(index) as usize][(index2) as usize] = 81;
                        } else if randomInt == 2 {
                            mapData[(index) as usize][(index2) as usize] = 97;
                        }
                    } else if mapArray[(index) as usize][(index2) as usize] == 17 {
                        let mut randomInt: i32 = jcall!(self.randomFactory).getAbsoluteNextIntAllowZero(3);
                        if randomInt == 0 {
                            mapData[(index) as usize][(index2) as usize] = 17;
                        } else if randomInt == 1 {
                            mapData[(index) as usize][(index2) as usize] = 81;
                        } else if randomInt == 2 {
                            mapData[(index) as usize][(index2) as usize] = 97;
                        }
                    }
                }
                if index2 % 7 == 0 {
                    if mapArray[(index) as usize][(index2) as usize] == 34 {
                        let mut randomInt: i32 = jcall!(self.randomFactory).getAbsoluteNextIntAllowZero(4);
                        if randomInt == 0 {
                            mapData[(index) as usize][(index2) as usize] = 33;
                        } else if randomInt == 1 {
                            mapData[(index) as usize][(index2) as usize] = 65;
                        } else if randomInt == 2 {
                            mapData[(index) as usize][(index2) as usize] = 81;
                        } else if randomInt == 3 {
                            mapData[(index) as usize][(index2) as usize] = 97;
                        }
                    }
                }
            }
        }
    }
    

    pub fn visit2(&mut self, mut mapArray: Vec<Vec<i32>>, mut mapData: Vec<Vec<i32>>, mut index: i32, mut index2: i32) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut countX: i32 = 0;
        let mut countY: i32 = 0;
        let mut index3: i32 = index - 1;
        for index4 in ((0) + 1..=index2 - 1).rev() {
            index3 -= 1;
            if index3 < 0 || index4 < 0 {
                return;
            }
            if mapArray[(index3) as usize][(index4) as usize] != 14 || mapData[(index3) as usize][(index4) as usize] != 0 {
                if countX > 10 && countY > 10 {
                    for index5 in (index3 + 1..index - 1) {
                        for index6 in (index4 + 1..index2 - 1) {
                            if mapArray[(index5) as usize][(index6) as usize] != 14 || mapData[(index5) as usize][(index6) as usize] != 0 {
                                return;
                            }
                        }
                    }
                    let mut x: i32 = index - (countX / 2);
                    let mut y: i32 = index2 - (countY / 2);
                    if x >= 0 && y >= 0 && x < (mapData.len() as i32) && y < (mapData[(0) as usize].len() as i32) {
                    }
                }
                return;
            }
            countY += 1;
            countX += 1;
        }
    }
}
impl std::ops::Deref for PropsTileMapPlacementVisitor {
    type Target = TileMapPlacementVisitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PropsTileMapPlacementVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::behavior::topview::placement::TileMapPlacementVisitor::TileMapPlacementVisitorDyn for PropsTileMapPlacementVisitor {
    fn as_TileMapPlacementVisitor(&self) -> &crate::org::allbinary::game::behavior::topview::placement::TileMapPlacementVisitor::TileMapPlacementVisitor { &self.base }
    fn as_TileMapPlacementVisitor_mut(&mut self) -> &mut crate::org::allbinary::game::behavior::topview::placement::TileMapPlacementVisitor::TileMapPlacementVisitor { &mut self.base }
    fn visit(&mut self, mut lastMap: Option<Rc<RefCell<TiledMap>>>, mut mapData: Vec<Vec<i32>>) { Self::visit(self, lastMap, mapData) }
}


