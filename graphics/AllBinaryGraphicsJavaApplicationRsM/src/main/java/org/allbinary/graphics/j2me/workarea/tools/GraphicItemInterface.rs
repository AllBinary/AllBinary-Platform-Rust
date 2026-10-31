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
// use crate::java::awt::*;
// use crate::javax::swing::tree::MutableTreeNode::MutableTreeNode;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::j2me::workarea::canvas::CanvasDom::CanvasDom;
use crate::org::allbinary::graphics::j2me::workarea::canvas::IntegerDimension::IntegerDimension;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait GraphicItemInterface {
    

    fn getName(&mut self) -> String;
    

    fn setColor(&mut self, color: Option<Rc<RefCell<Color>>>);
    

    fn getTreeNode(&mut self) -> Option<Rc<RefCell<MutableTreeNode>>>;
    

    fn getPointsInterface(&mut self) -> Option<Rc<RefCell<Points>>>;
    

    fn setPointsInterface(&mut self, points: Option<Rc<RefCell<Points>>>);
    

    fn isValid(&mut self) -> bool;
    

    fn isActive(&mut self) -> bool;
    

    fn deactivate(&mut self);
    

    fn activate(&mut self);
    

    fn setFulcrumPoint(&mut self, point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>);
    

    // throws Exception
    fn translate(&mut self, x: i32, y: i32);
    

    fn setRotate(&mut self, theta: f64);
    

    fn addRotate(&mut self, theta: f64);
    

    fn setAngle(&mut self, angle: f64);
    

    fn addAngle(&mut self, angle: f64);
    

    fn getAngle(&mut self) -> f64;
    

    fn paint(&mut self, g: Option<Rc<RefCell<Graphics>>>, canvasAngle: Option<f64>, dimension: Option<Rc<RefCell<IntegerDimension>>>, x: i32, y: i32);
    

    // throws Exception
    fn toDom(&mut self, canvasDom: Option<Rc<RefCell<CanvasDom>>>) -> Option<Rc<RefCell<Node>>>;
    

    fn removePoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>;
    

    fn addPoint(&mut self, point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>);
    

    fn mouseClicked(&mut self, mouseEvent: Option<Rc<RefCell<MouseEvent>>>, xPixelsPerCell: i32, yPixelsPerCell: i32);
    

    fn mouseEntered(&mut self, mouseEvent: Option<Rc<RefCell<MouseEvent>>>);
    

    fn mouseExited(&mut self, mouseEvent: Option<Rc<RefCell<MouseEvent>>>);
    

    fn mousePressed(&mut self, mouseEvent: Option<Rc<RefCell<MouseEvent>>>, xPixelsPerCell: i32, yPixelsPerCell: i32);
    

    fn mouseReleased(&mut self, mouseEvent: Option<Rc<RefCell<MouseEvent>>>, xPixelsPerCell: i32, yPixelsPerCell: i32);
    

    fn mouseDragged(&mut self, mouseEvent: Option<Rc<RefCell<MouseEvent>>>, xPixelsPerCell: i32, yPixelsPerCell: i32);
    

    fn mouseMoved(&mut self, mouseEvent: Option<Rc<RefCell<MouseEvent>>>, xPixelsPerCell: i32, yPixelsPerCell: i32);
    

    fn keyPressed(&mut self, keyEvent: Option<Rc<RefCell<KeyEvent>>>);
    

    fn keyReleased(&mut self, keyEvent: Option<Rc<RefCell<KeyEvent>>>);
    

    fn keyTyped(&mut self, keyEvent: Option<Rc<RefCell<KeyEvent>>>);
    

    // throws Exception
    fn duplicate(&mut self) -> Option<Rc<RefCell<dyn GraphicItemInterface>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


