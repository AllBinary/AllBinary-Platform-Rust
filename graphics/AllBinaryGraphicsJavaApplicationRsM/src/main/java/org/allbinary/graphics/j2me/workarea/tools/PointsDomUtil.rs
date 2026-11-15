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
// package org.allbinary.graphics.j2me.workarea.tools
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::j2me::workarea::WorkAreaJPanel::WorkAreaJPanel;
use crate::org::allbinary::graphics::j2me::workarea::canvas::CanvasDom::CanvasDom;
use crate::org::allbinary::math::PositionStrings::PositionStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Exception::*;
use super::Document::Document;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PointsDomUtil {
    pub LINES: String,
    pub POINTONE: String,
    pub POINTTWO: String,
    pub ROTATEDPOINTONE: String,
    pub ROTATEDPOINTTWO: String,
    pub LINE: String,
}

pub static PointsDomUtil_instance: crate::JavaStatic<Option<Rc<RefCell<PointsDomUtil>>>> = crate::JavaStatic::new(|| PointsDomUtil::new());

impl PointsDomUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            LINES: String::new(),
            POINTONE: String::new(),
            POINTTWO: String::new(),
            ROTATEDPOINTONE: String::new(),
            ROTATEDPOINTTWO: String::new(),
            LINE: String::new(),
        };
        this.LINES = String::from("lines");
        this.POINTONE = String::from("pointOne");
        this.POINTTWO = String::from("pointTwo");
        this.ROTATEDPOINTONE = String::from("rotatedPointOne");
        this.ROTATEDPOINTTWO = String::from("rotatedPointTwo");
        this.LINE = String::from("line");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PointsDomUtil>>> {
        return PointsDomUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn toDom(&mut self, mut canvasDom: Option<Rc<RefCell<CanvasDom>>>, mut pointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Node>>> {
        let mut tempPointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(PointsUtil::PointsUtil::getInstance()).doTransforms(pointVector.clone(), Some((jcall!(canvasDom).getAngle()) as f64), jcall!(PointFactory::getInstance()).createXY(jcall!(jcall!(canvasDom).getDimension()).getWidth(), jcall!(jcall!(canvasDom).getDimension()).getHeight()));
        let mut document: Option<Rc<RefCell<Document>>> = WorkAreaJPanel::getDocument();
        let mut linesNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(self.LINES.clone())).clone() /* as Node */;
        let mut size2: i32 = jcall!(tempPointVector).size();
        let mut size: i32 = jcall!(pointVector).size();
        let mut firstPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        let mut rotatedFirstPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        let mut index: i32 = 0;
        if index < size && index < size2 {
            rotatedFirstPoint = { let java_value = ({ let java_object = (jcall!(tempPointVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            firstPoint = { let java_value = ({ let java_object = (jcall!(pointVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            index += 1;
        }
        while index < size && index < size2 {
            let mut lineNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(self.LINE.clone())).clone() /* as Node */;
            let mut rotatedSecondPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = { let java_object = (jcall!(tempPointVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            let mut secondPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = { let java_object = (jcall!(pointVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            let mut pointOneNode: Option<Rc<RefCell<Node>>> = { let java_arguments = (firstPoint.clone(), self.POINTONE.clone(),); self.getPointNode(java_arguments.0, java_arguments.1) };
            let mut pointTwoNode: Option<Rc<RefCell<Node>>> = { let java_arguments = (secondPoint.clone(), self.POINTTWO.clone(),); self.getPointNode(java_arguments.0, java_arguments.1) };
            let mut rotatedPointOneNode: Option<Rc<RefCell<Node>>> = { let java_arguments = (rotatedFirstPoint.clone(), self.ROTATEDPOINTONE.clone(),); self.getPointNode(java_arguments.0, java_arguments.1) };
            let mut rotatedPointTwoNode: Option<Rc<RefCell<Node>>> = { let java_arguments = (rotatedSecondPoint.clone(), self.ROTATEDPOINTTWO.clone(),); self.getPointNode(java_arguments.0, java_arguments.1) };
            jcall!(lineNode).appendChild(pointOneNode.clone());
            jcall!(lineNode).appendChild(pointTwoNode.clone());
            jcall!(lineNode).appendChild(rotatedPointOneNode.clone());
            jcall!(lineNode).appendChild(rotatedPointTwoNode.clone());
            firstPoint = secondPoint.clone();
            rotatedFirstPoint = rotatedSecondPoint.clone();
            jcall!(linesNode).appendChild(lineNode.clone());
            index += 1;
        }
        return linesNode.clone();
    }
    

    pub fn getPointNode(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut pointName: String) -> Option<Rc<RefCell<Node>>> {
        let mut document: Option<Rc<RefCell<Document>>> = WorkAreaJPanel::getDocument();
        let mut pointNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(pointName.clone())).clone() /* as Node */;
        let mut xNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement({ let java_field = jcall!(PositionStrings::getInstance()).X.clone(); java_field }.clone())).clone() /* as Node */;
        let mut xTextNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createTextNode((jcall!(point).getX()).to_string())).clone() /* as Node */;
        jcall!(xNode).appendChild(xTextNode.clone());
        let mut yNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement({ let java_field = jcall!(PositionStrings::getInstance()).Y.clone(); java_field }.clone())).clone() /* as Node */;
        let mut yTextNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createTextNode((jcall!(point).getY()).to_string())).clone() /* as Node */;
        jcall!(yNode).appendChild(yTextNode.clone());
        jcall!(pointNode).appendChild(xNode.clone());
        jcall!(pointNode).appendChild(yNode.clone());
        return pointNode.clone();
    }
}

