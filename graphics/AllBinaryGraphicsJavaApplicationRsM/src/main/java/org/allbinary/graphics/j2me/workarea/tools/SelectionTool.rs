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
use std::collections::HashMap;
use super::*;
// use crate::java::awt::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::swing::tree::MutableTreeNode::MutableTreeNode;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::j2me::workarea::canvas::CanvasDom::CanvasDom;
use crate::org::allbinary::graphics::j2me::workarea::canvas::CanvasJPanel::CanvasJPanel;
use crate::org::allbinary::graphics::j2me::workarea::canvas::IntegerDimension::IntegerDimension;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::math::RectangleCollisionPointUtil::RectangleCollisionPointUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SelectionTool {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>>,
    pub active: bool,
    pub controlIdDown: bool,
    pub rectColor: Option<Rc<RefCell<Color>>>,
    pub startPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub endPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
}

thread_local! { pub static SelectionTool_NAME: String = String::from("Selection Tool"); }
impl GraphicItemInterface for SelectionTool {
    fn getName(&mut self) -> String { Self::getName(self) }
    fn setColor(&mut self, mut color: Option<Rc<RefCell<Color>>>) { Self::setColor(self, color) }
    fn getTreeNode(&mut self) -> Option<Rc<RefCell<MutableTreeNode>>> { Self::getTreeNode(self) }
    fn getPointsInterface(&mut self) -> Option<Rc<RefCell<Points>>> { Self::getPointsInterface(self) }
    fn setPointsInterface(&mut self, mut points: Option<Rc<RefCell<Points>>>) { Self::setPointsInterface(self, points) }
    fn isValid(&mut self) -> bool { Self::isValid(self) }
    fn isActive(&mut self) -> bool { Self::isActive(self) }
    fn deactivate(&mut self) { Self::deactivate(self) }
    fn activate(&mut self) { Self::activate(self) }
    fn setFulcrumPoint(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) { Self::setFulcrumPoint(self, point) }
    fn translate(&mut self, mut x: i32, mut y: i32) { Self::translate(self, x, y) }
    fn setRotate(&mut self, mut theta: f64) { Self::setRotate(self, theta) }
    fn addRotate(&mut self, mut theta: f64) { Self::addRotate(self, theta) }
    fn setAngle(&mut self, mut angle: f64) { Self::setAngle(self, angle) }
    fn addAngle(&mut self, mut angle: f64) { Self::addAngle(self, angle) }
    fn getAngle(&mut self) -> f64 { Self::getAngle(self) }
    fn paint(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut canvasAngle: Option<f64>, mut dimension: Option<Rc<RefCell<IntegerDimension>>>, mut x: i32, mut y: i32) { Self::paint(self, g, canvasAngle, dimension, x, y) }
    fn toDom(&mut self, mut canvasDom: Option<Rc<RefCell<CanvasDom>>>) -> Option<Rc<RefCell<Node>>> { Self::toDom(self, canvasDom) }
    fn removePoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> { Self::removePoint(self) }
    fn addPoint(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) { Self::addPoint(self, point) }
    fn mouseClicked(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut xPixelsPerCell: i32, mut yPixelsPerCell: i32) { Self::mouseClicked(self, mouseEvent, xPixelsPerCell, yPixelsPerCell) }
    fn mouseEntered(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) { Self::mouseEntered(self, mouseEvent) }
    fn mouseExited(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) { Self::mouseExited(self, mouseEvent) }
    fn mousePressed(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut xPixelsPerCell: i32, mut yPixelsPerCell: i32) { Self::mousePressed(self, mouseEvent, xPixelsPerCell, yPixelsPerCell) }
    fn mouseReleased(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut xPixelsPerCell: i32, mut yPixelsPerCell: i32) { Self::mouseReleased(self, mouseEvent, xPixelsPerCell, yPixelsPerCell) }
    fn mouseDragged(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut xPixelsPerCell: i32, mut yPixelsPerCell: i32) { Self::mouseDragged(self, mouseEvent, xPixelsPerCell, yPixelsPerCell) }
    fn mouseMoved(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut xPixelsPerCell: i32, mut yPixelsPerCell: i32) { Self::mouseMoved(self, mouseEvent, xPixelsPerCell, yPixelsPerCell) }
    fn keyPressed(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) { Self::keyPressed(self, keyEvent) }
    fn keyReleased(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) { Self::keyReleased(self, keyEvent) }
    fn keyTyped(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) { Self::keyTyped(self, keyEvent) }
    fn duplicate(&mut self) -> Option<Rc<RefCell<dyn GraphicItemInterface>>> { Self::duplicate(self) }
}

impl SelectionTool {
    

    pub fn new_struct(mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            canvasJPanel: None,
            active: false,
            controlIdDown: false,
            rectColor: None,
            startPoint: None,
            endPoint: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.active = true;
        this.rectColor = Color::new(jcall!(jcall!(BasicColorFactory::getInstance()).RED).intValue());
        this.startPoint = jcall!(PointFactory::getInstance()).ZERO_ZERO.clone();
        this.endPoint = this.startPoint.clone();
        this.canvasJPanel = canvasJPanel.clone();
        this
    }

    pub fn new(mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(canvasJPanel))))
    }
    

    pub fn setColor(&mut self, mut color: Option<Rc<RefCell<Color>>>) {
    }
    

    pub fn getTreeNode(&mut self) -> Option<Rc<RefCell<MutableTreeNode>>> {
        return None;
    }
    

    pub fn getPointsInterface(&mut self) -> Option<Rc<RefCell<Points>>> {
        return None;
    }
    

    pub fn setPointsInterface(&mut self, mut points: Option<Rc<RefCell<Points>>>) {
    }
    

    pub fn isValid(&mut self) -> bool {
        return true;
    }
    

    pub fn isActive(&mut self) -> bool {
        return self.active.clone();
    }
    

    pub fn deactivate(&mut self) {
    }
    

    pub fn activate(&mut self) {
    }
    

    pub fn setFulcrumPoint(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
    }
    

    // throws Exception
    pub fn translate(&mut self, mut x: i32, mut y: i32) {
    }
    

    pub fn setRotate(&mut self, mut theta: f64) {
    }
    

    pub fn addRotate(&mut self, mut theta: f64) {
    }
    

    pub fn setAngle(&mut self, mut angle: f64) {
    }
    

    pub fn addAngle(&mut self, mut angle: f64) {
    }
    

    pub fn getAngle(&mut self) -> f64 {
        return 0.0;
    }
    

    pub fn makeSelection(&mut self) {
        let mut xRect: i32 = self.getXRect();
        let mut xRect2: i32 = (jcall!(self.endPoint).getX() - jcall!(self.startPoint).getX()).abs() + xRect;
        let mut yRect: i32 = self.getYRect();
        let mut yRect2: i32 = (jcall!(self.endPoint).getY() - jcall!(self.startPoint).getY()).abs() + yRect;
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(self.canvasJPanel).getGraphicItemHashMap();
        let mut graphicItemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(hashMap).keySet()).toArray();
        let mut size: i32 = (graphicItemArray.len() as i32).clone();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(hashMap).get(graphicItemArray[(index) as usize].clone())).clone() /* as GraphicItemInterface */;
            if jcall!(item).getName() == LinesGraphicItem::LinesGraphicItem::getStaticName() {
                if RectangleCollisionPointUtil::allPointsInsideCell(xRect.clone(), yRect.clone(), xRect2.clone(), yRect2.clone(), jcall!(jcall!(item).getPointsInterface()).getPoints(), jcall!(self.canvasJPanel).getXPixelsPerWorkAreaPixel(), jcall!(self.canvasJPanel).getYPixelsPerWorkAreaPixel()) {
                    jcall!(item).setColor(unsafe { Color_YELLOW.clone() }.clone());
                    jcall!(item).activate();
                } else if !self.controlIdDown {
                    jcall!(item).setColor(unsafe { Color_WHITE.clone() }.clone());
                    jcall!(item).deactivate();
                }
            }
        }
    }
    

    pub fn getXRect(&mut self) -> i32 {
        let mut xDiff: i32 = jcall!(self.endPoint).getX() - jcall!(self.startPoint).getX();
        let mut xRect: i32 = jcall!(self.startPoint).getX();
        if xDiff < 0 {
            xRect = jcall!(self.endPoint).getX();
        }
        return xRect.clone();
    }
    

    pub fn getYRect(&mut self) -> i32 {
        let mut yDiff: i32 = jcall!(self.endPoint).getY() - jcall!(self.startPoint).getY();
        let mut yRect: i32 = jcall!(self.startPoint).getY();
        if yDiff < 0 {
            yRect = jcall!(self.endPoint).getY();
        }
        return yRect.clone();
    }
    

    pub fn paint(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut canvasAngle: Option<f64>, mut dimension: Option<Rc<RefCell<IntegerDimension>>>, mut x: i32, mut y: i32) {
        jcall!(g).setColor(self.rectColor.clone());
        let mut xRect: i32 = self.getXRect();
        let mut width: i32 = (jcall!(self.endPoint).getX() - jcall!(self.startPoint).getX()).abs();
        let mut yRect: i32 = self.getYRect();
        let mut height: i32 = (jcall!(self.endPoint).getY() - jcall!(self.startPoint).getY()).abs();
        jcall!(g).drawRect(xRect.clone(), yRect.clone(), width.clone(), height.clone());
    }
    

    // throws Exception
    pub fn toDom(&mut self, mut canvasDom: Option<Rc<RefCell<CanvasDom>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    pub fn removePoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return None;
    }
    

    pub fn addPoint(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
    }
    

    pub fn getName(&mut self) -> String {
        return SelectionTool_NAME.with(Clone::clone).clone();
    }
    

    pub fn getStaticName() -> String {
        return SelectionTool_NAME.with(Clone::clone).clone();
    }
    

    pub fn mouseClicked(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut xPixelsPerCell: i32, mut yPixelsPerCell: i32) {
    }
    

    pub fn mouseMoved(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut xPixelsPerCell: i32, mut yPixelsPerCell: i32) {
    }
    

    pub fn mouseEntered(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) {
    }
    

    pub fn mouseExited(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) {
    }
    

    pub fn mousePressed(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut xPixelsPerCell: i32, mut yPixelsPerCell: i32) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(MouseStrings::MouseStrings::getInstance()).MOUSE_PRESSED.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.startPoint = jcall!(PointFactory::getInstance()).createXY(jcall!(mouseEvent).getX(), jcall!(mouseEvent).getY());
        self.endPoint = self.startPoint.clone();
    }
    

    pub fn mouseReleased(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut xPixelsPerCell: i32, mut yPixelsPerCell: i32) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(MouseStrings::MouseStrings::getInstance()).MOUSE_RELEASED.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.endPoint = jcall!(PointFactory::getInstance()).createXY(jcall!(mouseEvent).getX(), jcall!(mouseEvent).getY());
        self.active = false;
        self.makeSelection();
    }
    

    pub fn mouseDragged(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut xPixelsPerCell: i32, mut yPixelsPerCell: i32) {
        self.endPoint = jcall!(PointFactory::getInstance()).createXY(jcall!(mouseEvent).getX(), jcall!(mouseEvent).getY());
    }
    

    pub fn keyPressed(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
        if match (&jcall!(keyEvent).getKeyCode(), &jcall!(keyEvent).VK_CONTROL) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.controlIdDown = true;
        }
    }
    

    pub fn keyReleased(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
        if match (&jcall!(keyEvent).getKeyCode(), &jcall!(keyEvent).VK_CONTROL) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.controlIdDown = false;
        }
    }
    

    pub fn keyTyped(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
    }
    

    // throws Exception
    pub fn duplicate(&mut self) -> Option<Rc<RefCell<dyn GraphicItemInterface>>> {
        return None;
    }
}

