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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//1.4.2
pub struct PathFinderGraphVisitorBase {
}


impl PathFinderGraphVisitorBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn visit(&mut self, mut graph: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn fixPath(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isValid(&mut self, mut graphPath: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getInvalidReason(&mut self, mut graphPath: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait PathFinderGraphVisitorBaseDyn {
    fn as_PathFinderGraphVisitorBase(&self) -> &PathFinderGraphVisitorBase;
    fn as_PathFinderGraphVisitorBase_mut(&mut self) -> &mut PathFinderGraphVisitorBase;
    fn visit(&mut self, mut graph: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { PathFinderGraphVisitorBase::visit(self.as_PathFinderGraphVisitorBase_mut(), graph, startPathFindingNodeList, endPathFindingNodeList) }
    fn fixPath(&mut self, mut startPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut endPathFindingNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { PathFinderGraphVisitorBase::fixPath(self.as_PathFinderGraphVisitorBase_mut(), startPathFindingNodeList, endPathFindingNodeList, pathList) }
    fn isValid(&mut self, mut graphPath: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool { PathFinderGraphVisitorBase::isValid(self.as_PathFinderGraphVisitorBase_mut(), graphPath) }
    fn getInvalidReason(&mut self, mut graphPath: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> String { PathFinderGraphVisitorBase::getInvalidReason(self.as_PathFinderGraphVisitorBase_mut(), graphPath) }
}
impl PathFinderGraphVisitorBaseDyn for PathFinderGraphVisitorBase {
    fn as_PathFinderGraphVisitorBase(&self) -> &PathFinderGraphVisitorBase { self }
    fn as_PathFinderGraphVisitorBase_mut(&mut self) -> &mut PathFinderGraphVisitorBase { self }
}


