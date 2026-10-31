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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::swing::tree::DefaultMutableTreeNode::DefaultMutableTreeNode;
// use crate::javax::swing::tree::MutableTreeNode::MutableTreeNode;
use crate::org::allbinary::dom::DomHelper::DomHelper;
use crate::org::allbinary::game::input::GameInputStrings::GameInputStrings;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::j2me::StatusFactory::StatusFactory;
use crate::org::allbinary::graphics::j2me::workarea::canvas::CanvasDom::CanvasDom;
use crate::org::allbinary::graphics::j2me::workarea::canvas::IntegerDimension::IntegerDimension;
use crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipeline;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::math::PositionStrings::PositionStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LinesGraphicItem {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub gameInputStrings: Option<Rc<RefCell<GameInputStrings>>>,
    pub treeNode: Option<Rc<RefCell<DefaultMutableTreeNode>>>,
    pub pointTreeNodeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub currentMousePoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub fulcrumPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub active: bool,
    pub theta: f64,
    pub color: Option<Rc<RefCell<Color>>>,
    pub points: Option<Rc<RefCell<Points>>>,
}

thread_local! { pub static LinesGraphicItem_RECT: String = String::from("rect"); }
thread_local! { pub static LinesGraphicItem_ARC: String = String::from("arc"); }
thread_local! { pub static LinesGraphicItem_STRING: String = String::from("string"); }
thread_local! { pub static LinesGraphicItem_CHAR: String = String::from("char"); }
thread_local! { pub static LinesGraphicItem_IMAGE: String = String::from("image"); }
pub static mut LinesGraphicItem_item: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);
thread_local! { pub static LinesGraphicItem_NAME: String = String::from("Line Tool"); }
// impl GraphicItemInterface for LinesGraphicItem with the methods of the trait

impl LinesGraphicItem {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            gameInputStrings: None,
            treeNode: None,
            pointTreeNodeVector: None,
            currentMousePoint: None,
            fulcrumPoint: None,
            active: false,
            theta: 0.0,
            color: None,
            points: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.gameInputStrings = GameInputStrings::getInstance();
        this.active = false;
        this.theta = 0.0;
        this.color = Color::new(jcall!(jcall!(BasicColorFactory::getInstance()).WHITE).intValue());
        this.points = Points::new();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.init();
            this.active = true;
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(this.logUtil).put(jcall!(this.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut linesNode: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            gameInputStrings: None,
            treeNode: None,
            pointTreeNodeVector: None,
            currentMousePoint: None,
            fulcrumPoint: None,
            active: false,
            theta: 0.0,
            color: None,
            points: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.gameInputStrings = GameInputStrings::getInstance();
        this.active = false;
        this.theta = 0.0;
        this.color = Color::new(jcall!(jcall!(BasicColorFactory::getInstance()).WHITE).intValue());
        this.points = Points::new();
        this.init();
        this.active = false;
        let mut lineNodes: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(DomHelper::getInstance()).getWithoutTextNodes(jcall!(linesNode).getChildNodes());
        let mut numberOfLines: i32 = jcall!(lineNodes).size();
        for index in (0..numberOfLines) {
            let mut lineNode: Option<Rc<RefCell<Node>>> = (jcall!(lineNodes).get(index.clone())).clone() /* as Node */;
            let mut pointNodes: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(DomHelper::getInstance()).getWithoutTextNodes(jcall!(lineNode).getChildNodes());
            let mut pointOneNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(jcall!(PointsDomUtil::PointsDomUtil::getInstance()).POINTONE.clone(), pointNodes.clone());
            { let java_arguments = (jcall!(pointOneNode).getChildNodes(),); this.addPoint(java_arguments.0) };
            if index == numberOfLines - 1 {
                let mut pointTwoNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(jcall!(PointsDomUtil::PointsDomUtil::getInstance()).POINTTWO.clone(), pointNodes.clone());
                { let java_arguments = (jcall!(pointTwoNode).getChildNodes(),); this.addPoint(java_arguments.0) };
            }
        }
        if numberOfLines == 0 {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Lines node does not contain a line"))));
        }
        this
    }

    pub fn new1(mut linesNode: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(linesNode))))
    }
    

    // throws Exception
    pub fn init(&mut self) {
        self.treeNode = DefaultMutableTreeNode::new(format!("{}{}", jcall!(PointsDomUtil::PointsDomUtil::getInstance()).LINES, unsafe { LinesGraphicItem_item.clone() }));
        unsafe { *LinesGraphicItem_item += 1 };
        jcall!(self.points).init();
        self.pointTreeNodeVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        self.fulcrumPoint = jcall!(PointFactory::getInstance()).createXY(0, 0);
    }
    

    // throws Exception
    pub fn translate(&mut self, mut x: i32, mut y: i32) {
        jcall!(StatusFactory::getInstance()).setStatus(format!("{}{}", String::from("Translating: "), jcall!(self.points).getPoints()));
        let mut basicGraphicsPipeline: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipelineDyn>>> = BasicGraphicsPipeline::new(jcall!(self.points).getPoints()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipelineDyn>>);
        jcall!(basicGraphicsPipeline).translate(x.clone(), y.clone());
        self.points = Points::new();
        { let java_receiver = self.points.clone(); let java_arguments = (jcall!(basicGraphicsPipeline).getMatrix(),); jcall!(java_receiver).addPoints(java_arguments.0); };
    }
    

    pub fn setRotate(&mut self, mut theta: f64) {
        while theta > 2 * unsafe { Math_PI.clone() } {
            theta -= 2 * unsafe { Math_PI.clone() };
        }
        while theta < 0.0 {
            theta += 2 * unsafe { Math_PI.clone() };
        }
        self.theta = theta.clone();
    }
    

    pub fn addRotate(&mut self, mut theta: f64) {
        self.setRotate(self.theta + theta);
    }
    

    pub fn setAngle(&mut self, mut angle: f64) {
        { let java_arguments = (((angle.clone()) as f64).to_radians(),); self.setRotate(java_arguments.0) };
    }
    

    pub fn addAngle(&mut self, mut angle: f64) {
        { let java_arguments = (self.theta + ((angle.clone()) as f64).to_radians(),); self.setRotate(java_arguments.0) };
    }
    

    pub fn getTreeNode(&mut self) -> Option<Rc<RefCell<MutableTreeNode>>> {
        return (self.treeNode).clone() /* as MutableTreeNode */;
    }
    

    pub fn getName(&mut self) -> String {
        return LinesGraphicItem_NAME.with(Clone::clone).clone();
    }
    

    pub fn getStaticName() -> String {
        return LinesGraphicItem_NAME.with(Clone::clone).clone();
    }
    

    // throws Exception
    // synchronized
    pub fn addPoint(&mut self, mut pointNodes: Option<Rc<RefCell<NodeList>>>) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(DomHelper::getInstance()).getWithoutTextNodes(pointNodes.clone());
        let mut xNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(jcall!(PositionStrings::getInstance()).X.clone(), list.clone());
        let mut xTextNode: Option<Rc<RefCell<Node>>> = jcall!(xNode).getFirstChild();
        let mut xInteger: Option<i32> = Some((jcall!(xTextNode).getNodeValue()) as i32);
        let mut yNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(jcall!(PositionStrings::getInstance()).Y.clone(), list.clone());
        let mut yTextNode: Option<Rc<RefCell<Node>>> = jcall!(yNode).getFirstChild();
        let mut yInteger: Option<i32> = Some((jcall!(yTextNode).getNodeValue()) as i32);
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(PointFactory::getInstance()).createXY(((xInteger).unwrap() as i32), ((yInteger).unwrap() as i32));
        self.addPoint(point.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn addPoint(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        jcall!(jcall!(self.points).getPoints()).add(point.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(point).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addPoint"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        { let java_receiver = self.pointTreeNodeVector.clone(); let java_arguments = (crate::java_into_object(DefaultMutableTreeNode::new(jcall!(point).toString())),); jcall!(java_receiver).add(java_arguments.0); };
        { let java_receiver = self.treeNode.clone(); let java_arguments = (({ let java_receiver = self.pointTreeNodeVector.clone(); let java_arguments = (jcall!(self.pointTreeNodeVector).size() - 1,); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as DefaultMutableTreeNode */,); jcall!(java_receiver).add(java_arguments.0); };
    }
    

    // synchronized
    pub fn removePoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        if jcall!(self.points).getSize() > 0 {
            let mut lastPoint: i32 = jcall!(self.points).getSize() - 1;
            point = (jcall!(jcall!(self.points).getPoints()).removeAt(lastPoint.clone())).clone() /* as GPoint */;
            let mut index: i32 = jcall!(self.pointTreeNodeVector).size() - 1;
            if index > 0 {
                { let java_receiver = self.treeNode.clone(); let java_arguments = ((jcall!(self.pointTreeNodeVector).get(index.clone())).clone() /* as DefaultMutableTreeNode */,); jcall!(java_receiver).remove(java_arguments.0); };
                jcall!(self.pointTreeNodeVector).remove(crate::java_into_object(index.clone()));
            }
        }
        return point.clone();
    }
    

    pub fn isActive(&mut self) -> bool {
        return self.active.clone();
    }
    

    pub fn deactivate(&mut self) {
        self.active = false;
    }
    

    pub fn activate(&mut self) {
        self.active = true;
    }
    

    // throws Exception
    pub fn duplicatePoints(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            if point.is_some() {
                jcall!(jcall!(self.points).getPoints()).add(jcall!(PointFactory::getInstance()).createXY(jcall!(point).getX(), jcall!(point).getY()));
                { let java_receiver = self.pointTreeNodeVector.clone(); let java_arguments = (crate::java_into_object(DefaultMutableTreeNode::new(jcall!(point).toString())),); jcall!(java_receiver).add(java_arguments.0); };
                { let java_receiver = self.treeNode.clone(); let java_arguments = (({ let java_receiver = self.pointTreeNodeVector.clone(); let java_arguments = (jcall!(self.pointTreeNodeVector).size() - 1,); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as DefaultMutableTreeNode */,); jcall!(java_receiver).add(java_arguments.0); };
            }
        }
    }
    

    // throws Exception
    pub fn duplicate(&mut self) -> Option<Rc<RefCell<dyn GraphicItemInterface>>> {
        let mut linesGraphicItem: Option<Rc<RefCell<LinesGraphicItem>>> = LinesGraphicItem::new();
        jcall!(linesGraphicItem).duplicatePoints(jcall!(self.points).getPoints());
        jcall!(linesGraphicItem).deactivate();
        return linesGraphicItem.clone().map(|reference| reference as Rc<RefCell<dyn GraphicItemInterface>>);
    }
    

    pub fn getAngle(&mut self) -> f64 {
        return ((self.theta.clone()) as f64).to_degrees();
    }
    

    pub fn setFulcrumPoint(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        self.fulcrumPoint = point.clone();
    }
    

    pub fn paint(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut canvasAngle: Option<f64>, mut dimension: Option<Rc<RefCell<IntegerDimension>>>, mut x: i32, mut y: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut graphics: Option<Rc<RefCell<Graphics2D>>> = (g).clone() /* as Graphics2D */;
            jcall!(graphics).setColor(self.getColor());
            jcall!(graphics).setStroke(BasicStroke::new(x.clone()));
            if self.isActive() && self.currentMousePoint.is_some() {
                jcall!(jcall!(self.points).getPoints()).add(self.currentMousePoint.clone());
            }
            let mut tempPointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(PointsUtil::PointsUtil::getInstance()).doTransforms(jcall!(self.points).getPoints(), canvasAngle.clone(), jcall!(PointFactory::getInstance()).createXY(jcall!(dimension).getWidth(), jcall!(dimension).getHeight()));
            if self.isActive() && self.currentMousePoint.is_some() {
                self.currentMousePoint = (jcall!(jcall!(self.points).getPoints()).removeAt(jcall!(self.points).getSize() - 1)).clone() /* as GPoint */;
            }
            let mut size: i32 = jcall!(tempPointVector).size();
            let mut firstPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
            if size > 0 {
                firstPoint = { let java_object = (jcall!(tempPointVector).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            }
            for index in (1..size) {
                let mut secondPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = { let java_object = (jcall!(tempPointVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
                jcall!(graphics).drawLine((jcall!(firstPoint).getX() * x) - (x / 4), (jcall!(firstPoint).getY() * y) - (y / 4), (jcall!(secondPoint).getX() * x) - (x / 4), (jcall!(secondPoint).getY() * y) - (y / 4));
                firstPoint = secondPoint.clone();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("mouseMoved"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn toDom(&mut self, mut canvasDom: Option<Rc<RefCell<CanvasDom>>>) -> Option<Rc<RefCell<Node>>> {
        return jcall!(PointsDomUtil::PointsDomUtil::getInstance()).toDom(canvasDom.clone(), jcall!(self.points).getPoints());
    }
    

    pub fn isValid(&mut self) -> bool {
        return jcall!(self.points).isValid();
    }
    

    pub fn getPointsInterface(&mut self) -> Option<Rc<RefCell<Points>>> {
        return self.points.clone();
    }
    

    pub fn setPointsInterface(&mut self, mut points: Option<Rc<RefCell<Points>>>) {
        self.points = points.clone();
    }
    

    pub fn getColor(&mut self) -> Option<Rc<RefCell<Color>>> {
        return self.color.clone();
    }
    

    pub fn setColor(&mut self, mut color: Option<Rc<RefCell<Color>>>) {
        self.color = color.clone();
    }
    

    pub fn mouseClicked(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut x: i32, mut y: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut mousePoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(PointFactory::getInstance()).createXY(jcall!(jcall!(mouseEvent).getPoint()).x.clone(), jcall!(jcall!(mouseEvent).getPoint()).y.clone());
            if match (&(jcall!(mouseEvent).getModifiers() & jcall!(mouseEvent).BUTTON1_MASK), &jcall!(mouseEvent).BUTTON1_MASK) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(PointFactory::getInstance()).createXY(jcall!(mousePoint).getX() / x, jcall!(mousePoint).getY() / y);
                jcall!(StatusFactory::getInstance()).setStatus(format!("{}{}", String::from("Line Point Added: "), jcall!(point).toString()));
                self.addPoint(point.clone());
            } else if match (&(jcall!(mouseEvent).getModifiers() & jcall!(mouseEvent).BUTTON3_MASK), &jcall!(mouseEvent).BUTTON3_MASK) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(StatusFactory::getInstance()).setStatus(String::from("Line Point Removed"));
                self.removePoint();
            }
            self.currentMousePoint = mousePoint.clone();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("mouseMoved"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn mouseEntered(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) {
    }
    

    pub fn mouseExited(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) {
    }
    

    pub fn mousePressed(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut xPixelsPerCell: i32, mut yPixelsPerCell: i32) {
    }
    

    pub fn mouseReleased(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut xPixelsPerCell: i32, mut yPixelsPerCell: i32) {
    }
    

    pub fn mouseDragged(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut xPixelsPerCell: i32, mut yPixelsPerCell: i32) {
    }
    

    pub fn mouseMoved(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>, mut x: i32, mut y: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut mousePoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(PointFactory::getInstance()).createXY(jcall!(jcall!(mouseEvent).getPoint()).x.clone(), jcall!(jcall!(mouseEvent).getPoint()).y.clone());
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(PointFactory::getInstance()).createXY(jcall!(mousePoint).getX() / x, jcall!(mousePoint).getY() / y);
            self.currentMousePoint = point.clone();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("mouseMoved"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn keyPressed(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut keyCode: i32 = jcall!(keyEvent).getKeyCode();
            if keyCode == jcall!(keyEvent).VK_ESCAPE {
                jcall!(StatusFactory::getInstance()).setStatus(String::from("Deactivated"));
                self.deactivate();
            } else if keyCode == jcall!(keyEvent).VK_UP {
                self.translate(0, -1);
            } else if keyCode == jcall!(keyEvent).VK_DOWN {
                self.translate(0, 1);
            } else if keyCode == jcall!(keyEvent).VK_LEFT {
                self.translate(-1, 0);
            } else if keyCode == jcall!(keyEvent).VK_RIGHT {
                self.translate(1, 0);
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).KEY_PRESSED.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn keyReleased(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
    }
    

    pub fn keyTyped(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
    }
}

