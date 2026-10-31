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
// package org.allbinary.game.media.graphics.geography.map.racetrack
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorFactoryBase::PathFinderGraphVisitorFactoryBase;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorBase::PathFinderGraphVisitorBase;
use super::PathFinderGraphHackVisitor::PathFinderGraphHackVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathFinderGraphHackVisitorFactory {
    pub base: PathFinderGraphVisitorFactoryBase,
    pub edgeMinimum: i32,
    pub minPathWeight: i32,
    pub maxPathWeight: i32,
}


impl PathFinderGraphHackVisitorFactory {
    

    pub fn new_struct(mut edgeMinimum: i32, mut minPathWeight: i32, mut maxPathWeight: i32) -> Self {
        let mut this = Self {
            base: PathFinderGraphVisitorFactoryBase::new_struct(),
            edgeMinimum: 0,
            minPathWeight: 0,
            maxPathWeight: 0,
        };
        this.edgeMinimum = edgeMinimum.clone();
        this.minPathWeight = minPathWeight.clone();
        this.maxPathWeight = maxPathWeight.clone();
        this
    }

    pub fn new(mut edgeMinimum: i32, mut minPathWeight: i32, mut maxPathWeight: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(edgeMinimum, minPathWeight, maxPathWeight))))
    }
    

    pub fn getInstance(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) -> Option<Rc<RefCell<PathFinderGraphVisitorBase>>> {
        return PathFinderGraphHackVisitor::new(geographicMapInterface.clone(), self.edgeMinimum.clone(), self.minPathWeight.clone(), self.maxPathWeight.clone()).map(|reference| Rc::new(RefCell::new(Rc::try_unwrap(reference).ok().expect("new object is not shared").into_inner().base.base)));
    }
}
impl std::ops::Deref for PathFinderGraphHackVisitorFactory {
    type Target = PathFinderGraphVisitorFactoryBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PathFinderGraphHackVisitorFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorFactoryBase::PathFinderGraphVisitorFactoryBaseDyn for PathFinderGraphHackVisitorFactory {
    fn as_PathFinderGraphVisitorFactoryBase(&self) -> &crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorFactoryBase::PathFinderGraphVisitorFactoryBase { &self.base }
    fn as_PathFinderGraphVisitorFactoryBase_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::pathfinding::PathFinderGraphVisitorFactoryBase::PathFinderGraphVisitorFactoryBase { &mut self.base }
    fn getInstance(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) -> Option<Rc<RefCell<PathFinderGraphVisitorBase>>> { Self::getInstance(self, geographicMapInterface) }
}


