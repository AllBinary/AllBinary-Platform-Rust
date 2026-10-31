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
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;
use crate::org::allbinary::string::CommonLabels::CommonLabels;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathFindingNode {
    pub parent: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
    pub geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>,
}


impl PathFindingNode {
    

    pub fn new_struct(mut parent: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Self {
        let mut this = Self {
            parent: None,
            geographicMapCellPosition: None,
        };
        this.parent = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
        this.geographicMapCellPosition = crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        this.parent = parent.clone();
        this.setGeographicMapCellPositionP(geographicMapCellPosition.clone());
        if match (&this.geographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No GeographicMapCellPosition"))));
        }
        this
    }

    pub fn new(mut parent: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(parent, geographicMapCellPosition))))
    }
    

    pub fn getParentP(&mut self) -> Option<Rc<RefCell<dyn PathFindingNodeDyn>>> {
        return (self.parent).clone() /* as PathFindingNode */;
    }
    

    pub fn setGeographicMapCellPositionP(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        self.geographicMapCellPosition = geographicMapCellPosition.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(std::any::type_name_of_val(&self).to_string());
        jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).COLON_SEP.clone());
        jcall!(stringBuffer).append(String::from(" Path: "));
        jcall!(stringBuffer).append(jcall!(self.geographicMapCellPosition).toString());
        let mut pathFindingNode: Option<Rc<RefCell<dyn PathFindingNodeDyn>>> = self.getParentP();
        while pathFindingNode.is_some() {
            jcall!(stringBuffer).append(jcall!(jcall!(pathFindingNode).as_PathFindingNode_mut().geographicMapCellPosition).toString());
            jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
            pathFindingNode = jcall!(pathFindingNode).getParentP();
        }
        return jcall!(stringBuffer).toString();
    }
}
pub trait PathFindingNodeDyn {
    fn as_PathFindingNode(&self) -> &PathFindingNode;
    fn as_PathFindingNode_mut(&mut self) -> &mut PathFindingNode;
    fn getParentP(&mut self) -> Option<Rc<RefCell<dyn PathFindingNodeDyn>>> { PathFindingNode::getParentP(self.as_PathFindingNode_mut()) }
    fn setGeographicMapCellPositionP(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { PathFindingNode::setGeographicMapCellPositionP(self.as_PathFindingNode_mut(), geographicMapCellPosition) }
    fn toString(&mut self) -> String { PathFindingNode::toString(self.as_PathFindingNode_mut()) }
}
impl PathFindingNodeDyn for PathFindingNode {
    fn as_PathFindingNode(&self) -> &PathFindingNode { self }
    fn as_PathFindingNode_mut(&mut self) -> &mut PathFindingNode { self }
}


