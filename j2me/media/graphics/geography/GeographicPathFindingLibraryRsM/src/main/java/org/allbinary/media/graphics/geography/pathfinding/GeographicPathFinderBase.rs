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
// package org.allbinary.media.graphics.geography.pathfinding
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::MultipassState::MultipassState;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicPathFinderBase {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

thread_local! { pub static GeographicPathFinderBase_NULL_GEOGRAPHIC_PATH_FINDER_BASE: Option<Rc<RefCell<dyn GeographicPathFinderBaseDyn>>> = GeographicPathFinderBase::new().map(|reference| reference as Rc<RefCell<dyn GeographicPathFinderBaseDyn>>); }

impl GeographicPathFinderBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn searchTotalPath(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut totalPaths: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn searchTotalPathN(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut totalPaths: i32, mut multipassState: Option<Rc<RefCell<MultipassState>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait GeographicPathFinderBaseDyn {
    fn as_GeographicPathFinderBase(&self) -> &GeographicPathFinderBase;
    fn as_GeographicPathFinderBase_mut(&mut self) -> &mut GeographicPathFinderBase;
    fn searchTotalPath(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut totalPaths: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { GeographicPathFinderBase::searchTotalPath(self.as_GeographicPathFinderBase_mut(), startPathFindingNodeList, endPathFindingNodeList, totalPaths) }
    fn searchTotalPathN(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut totalPaths: i32, mut multipassState: Option<Rc<RefCell<MultipassState>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { GeographicPathFinderBase::searchTotalPathN(self.as_GeographicPathFinderBase_mut(), startPathFindingNodeList, endPathFindingNodeList, totalPaths, multipassState) }
}
impl GeographicPathFinderBaseDyn for GeographicPathFinderBase {
    fn as_GeographicPathFinderBase(&self) -> &GeographicPathFinderBase { self }
    fn as_GeographicPathFinderBase_mut(&mut self) -> &mut GeographicPathFinderBase { self }
}


