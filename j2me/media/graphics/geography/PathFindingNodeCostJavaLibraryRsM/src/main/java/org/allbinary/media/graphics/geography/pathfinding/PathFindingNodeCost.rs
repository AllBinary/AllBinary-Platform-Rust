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
// package org.allbinary.media.graphics.geography.pathfinding
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use super::PathFindingNode::PathFindingNode;
use super::Comparable::Comparable;
use super::PathFindingNodeCostInfo::PathFindingNodeCostInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathFindingNodeCost {
    pub base: PathFindingNode,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub pathFindingNodeCostInfoP: Option<Rc<RefCell<PathFindingNodeCostInfo>>>,
}

thread_local! { pub static PathFindingNodeCost_NULL_PATH_FINDING_NODE_COST_ARRAY_ARRAY: Vec<Vec<Option<Rc<RefCell<PathFindingNodeCost>>>>> = vec![vec![None; (0) as usize]; (0) as usize]; }
// impl Comparable for PathFindingNodeCost with the methods of the trait

impl PathFindingNodeCost {
    

    pub fn new_struct(mut parent: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut pathFindingNodeCostInfo: Option<Rc<RefCell<PathFindingNodeCostInfo>>>) -> Self {
        let mut this = Self {
            base: PathFindingNode::new_struct(parent.clone(), geographicMapCellPosition.clone()),
            commonSeps: None,
            pathFindingNodeCostInfoP: None,
        };
        this.commonSeps = CommonSeps::getInstance();
        // super(...) is in the struct literal above
        this.pathFindingNodeCostInfoP = pathFindingNodeCostInfo.clone();
        if this.geographicMapCellPosition.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No GeographicMapCellPosition"))));
        }
        if this.getPathFindingNodeCostInfo().is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No PathFindingNodeCostInfo"))));
        }
        this
    }

    pub fn new(mut parent: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut pathFindingNodeCostInfo: Option<Rc<RefCell<PathFindingNodeCostInfo>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(parent, geographicMapCellPosition, pathFindingNodeCostInfo))))
    }
    

    pub fn getPathFindingNodeCostInfo(&mut self) -> Option<Rc<RefCell<PathFindingNodeCostInfo>>> {
        return self.pathFindingNodeCostInfoP.clone();
    }
    

    pub fn setPathFindingNodeCostInfo(&mut self, mut pathFindingNodeCostInfo: Option<Rc<RefCell<PathFindingNodeCostInfo>>>) {
        self.pathFindingNodeCostInfoP = pathFindingNodeCostInfo.clone();
    }
    

    pub fn compareTo(&mut self, mut pathFindingNodeCost: Option<Rc<RefCell<PathFindingNodeCost>>>) -> i32 {
        return jcall!(self.pathFindingNodeCostInfoP).compareTo(jcall!(pathFindingNodeCost).pathFindingNodeCostInfoP.clone());
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(std::any::type_name_of_val(&self).to_string());
        jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).COLON_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.getPathFindingNodeCostInfo()).toString());
        jcall!(stringBuffer).append(String::from(" Path: "));
        jcall!(stringBuffer).append(jcall!(self.geographicMapCellPosition).toString());
        let mut pathFindingNode: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>> = (self.parent).clone() /* as PathFindingNode */;
        while pathFindingNode.is_some() {
            jcall!(stringBuffer).append(jcall!(jcall!(pathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition).toString());
            jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
            pathFindingNode = (jcall!(pathFindingNode).as_PathFindingNode_mut().parent).clone() /* as PathFindingNode */;
        }
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for PathFindingNodeCost {
    type Target = PathFindingNode;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PathFindingNodeCost {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn for PathFindingNodeCost {
    fn as_PathFindingNode(&self) -> &crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNode { &self.base }
    fn as_PathFindingNode_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNode { &mut self.base }
    fn toString(&mut self) -> String { Self::toString(self) }
}


