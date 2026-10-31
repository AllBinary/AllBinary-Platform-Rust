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
// package org.allbinary.media.graphics.geography.map
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::GeographicMapCellHistoryProgressBase::GeographicMapCellHistoryProgressBase;
use super::GeographicMapCellHistory::GeographicMapCellHistory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapCellHistoryProgress {
    pub base: GeographicMapCellHistoryProgressBase,
    pub totalVisitedArray: Vec<i32>,
    pub geographicMapCellHistoryArray: Vec<Option<Rc<RefCell<GeographicMapCellHistory>>>>,
}


impl GeographicMapCellHistoryProgress {
    

    pub fn new_struct(mut geographicMapCellHistoryArray: Vec<Option<Rc<RefCell<GeographicMapCellHistory>>>>) -> Self {
        let mut this = Self {
            base: GeographicMapCellHistoryProgressBase::new_struct(),
            totalVisitedArray: Vec::new(),
            geographicMapCellHistoryArray: Vec::new(),
        };
        this.totalVisitedArray = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY.clone();
        this.geographicMapCellHistoryArray = GeographicMapCellHistory_NULL_GEOGRPAHIC_MAP_HISTORY_ARRAY.with(Clone::clone).clone();
        this.geographicMapCellHistoryArray = geographicMapCellHistoryArray.clone();
        this.init();
        this
    }

    pub fn new(mut geographicMapCellHistoryArray: Vec<Option<Rc<RefCell<GeographicMapCellHistory>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(geographicMapCellHistoryArray))))
    }
    

    pub fn init(&mut self) {
        self.totalVisitedArray = vec![0; ((self.geographicMapCellHistoryArray.len() as i32)) as usize];
    }
    

    pub fn isAnyProgress(&mut self) -> bool {
        let mut isProgressing: bool = false;
        let mut size: i32 = (self.geographicMapCellHistoryArray.len() as i32).clone();
        let mut currentTotalVisited: i32 = 0;
        let mut totalVisited: i32 = 0;
        for index in (0..size) {
            totalVisited = jcall!(self.geographicMapCellHistoryArray[(index) as usize]).getTotalVisited();
            currentTotalVisited = self.totalVisitedArray[(index) as usize].clone();
            if currentTotalVisited < totalVisited {
                isProgressing = true;
                self.totalVisitedArray[(index) as usize] = totalVisited.clone();
            }
        }
        return isProgressing.clone();
    }
}
impl std::ops::Deref for GeographicMapCellHistoryProgress {
    type Target = GeographicMapCellHistoryProgressBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GeographicMapCellHistoryProgress {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistoryProgressBase::GeographicMapCellHistoryProgressBaseDyn for GeographicMapCellHistoryProgress {
    fn as_GeographicMapCellHistoryProgressBase(&self) -> &crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistoryProgressBase::GeographicMapCellHistoryProgressBase { &self.base }
    fn as_GeographicMapCellHistoryProgressBase_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistoryProgressBase::GeographicMapCellHistoryProgressBase { &mut self.base }
    fn init(&mut self) { Self::init(self) }
    fn isAnyProgress(&mut self) -> bool { Self::isAnyProgress(self) }
}


