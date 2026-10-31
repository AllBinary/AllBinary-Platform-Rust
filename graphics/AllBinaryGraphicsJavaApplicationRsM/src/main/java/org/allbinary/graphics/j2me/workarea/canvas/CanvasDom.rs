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
// package org.allbinary.graphics.j2me.workarea.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::animation::vector::VectorCenterGenerator::VectorCenterGenerator;
use crate::org::allbinary::dom::DomHelper::DomHelper;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::GraphicsStrings::GraphicsStrings;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::j2me::workarea::WorkAreaJPanel::WorkAreaJPanel;
use crate::org::allbinary::graphics::j2me::workarea::tools::GraphicItemFactory::GraphicItemFactory;
use crate::org::allbinary::graphics::j2me::workarea::tools::GraphicItemInterface::GraphicItemInterface;
use crate::org::allbinary::graphics::j2me::workarea::tools::LinesGraphicItem::LinesGraphicItem;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::w3c::dom::Node::Node;
use super::IntegerDimension::IntegerDimension;
use super::Grid::Grid;
use super::CanvasJPanel::CanvasJPanel;
use super::Document::Document;
use super::dom::dom;
use super::w3c::w3c;
use super::org::org;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CanvasDom {
    pub graphicsStrings: Option<Rc<RefCell<GraphicsStrings>>>,
    pub graphicItemHashMap: Option<Rc<RefCell<HashMap>>>,
    pub angle: f64,
    pub dimension: Option<Rc<RefCell<IntegerDimension>>>,
    pub grid: Option<Rc<RefCell<Grid>>>,
}

thread_local! { pub static CanvasDom_FRAME: String = String::from("frame"); }
thread_local! { pub static CanvasDom_ROTATE: String = String::from("rotate"); }
thread_local! { pub static CanvasDom_SIZE: String = String::from("size"); }
thread_local! { pub static CanvasDom_X: String = String::from("X"); }
thread_local! { pub static CanvasDom_Y: String = String::from("Y"); }
thread_local! { pub static CanvasDom_ZOOM: String = String::from("zoom"); }
thread_local! { pub static CanvasDom_GRID: String = String::from("grid"); }
thread_local! { pub static CanvasDom_ENABLED: String = String::from("enabled"); }
thread_local! { pub static CanvasDom_POSSIBLE: String = String::from("possible"); }
thread_local! { pub static CanvasDom_GRAPHICITEMS: String = String::from("graphicItems"); }
thread_local! { pub static CanvasDom_REAL_SIZE: String = String::from("realSize"); }

impl CanvasDom {
    

    pub fn new_struct1(mut canvasNode: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            graphicsStrings: None,
            graphicItemHashMap: None,
            angle: 0.0,
            dimension: None,
            grid: None,
        };
        this.graphicsStrings = GraphicsStrings::getInstance();
        this.grid = Grid::new();
        jcall!(this.grid).grid = jcall!(PointFactory::getInstance()).createXY(0, 0);
        this.dimension = IntegerDimension::new(0, 0);
        let mut name: String = jcall!(canvasNode).getNodeName();
        if (name.cmp(&CanvasDom_FRAME.with(Clone::clone).clone()) as i32) == 0 {
            this.graphicItemHashMap = jcall!(StdUtil::getInstance()).createHashMap();
            let mut angleNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(CanvasDom_ROTATE.with(Clone::clone).clone(), jcall!(canvasNode).getChildNodes());
            let mut angleTextNode: Option<Rc<RefCell<Node>>> = jcall!(angleNode).getFirstChild();
            let mut angle: f64 = ((Some((jcall!(angleTextNode).getNodeValue()) as f64)).unwrap() as f64);
            this.setAngle(angle.clone());
            let mut sizeNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(CanvasDom_SIZE.with(Clone::clone).clone(), jcall!(canvasNode).getChildNodes());
            let mut xNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(CanvasDom_X.with(Clone::clone).clone(), jcall!(sizeNode).getChildNodes());
            let mut xTextNode: Option<Rc<RefCell<Node>>> = jcall!(xNode).getFirstChild();
            let mut x: i32 = ((Some((jcall!(xTextNode).getNodeValue()) as i32)).unwrap() as i32);
            let mut yNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(CanvasDom_Y.with(Clone::clone).clone(), jcall!(sizeNode).getChildNodes());
            let mut yTextNode: Option<Rc<RefCell<Node>>> = jcall!(yNode).getFirstChild();
            let mut y: i32 = ((Some((jcall!(yTextNode).getNodeValue()) as i32)).unwrap() as i32);
            this.setWorkArea(x.clone(), y.clone());
            let mut zoomNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(CanvasDom_ZOOM.with(Clone::clone).clone(), jcall!(canvasNode).getChildNodes());
            let mut zoomTextNode: Option<Rc<RefCell<Node>>> = jcall!(zoomNode).getFirstChild();
            { let java_receiver = this.grid.clone(); let java_arguments = (((Some((jcall!(zoomTextNode).getNodeValue()) as i32)).unwrap() as i32),); jcall!(java_receiver).setZoom(java_arguments.0); };
            let mut gridNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(CanvasDom_GRID.with(Clone::clone).clone(), jcall!(canvasNode).getChildNodes());
            let mut gridSizeNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(CanvasDom_SIZE.with(Clone::clone).clone(), jcall!(gridNode).getChildNodes());
            let mut gridXNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(CanvasDom_X.with(Clone::clone).clone(), jcall!(gridSizeNode).getChildNodes());
            let mut gridXTextNode: Option<Rc<RefCell<Node>>> = jcall!(gridXNode).getFirstChild();
            let mut gridX: i32 = ((Some((jcall!(gridXTextNode).getNodeValue()) as i32)).unwrap() as i32);
            let mut gridYNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(CanvasDom_Y.with(Clone::clone).clone(), jcall!(gridSizeNode).getChildNodes());
            let mut gridYTextNode: Option<Rc<RefCell<Node>>> = jcall!(gridYNode).getFirstChild();
            let mut gridY: i32 = ((Some((jcall!(gridYTextNode).getNodeValue()) as i32)).unwrap() as i32);
            this.setGrid(gridX.clone(), gridY.clone());
            let mut enableNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(CanvasDom_ENABLED.with(Clone::clone).clone(), jcall!(gridNode).getChildNodes());
            let mut enableTextNode: Option<Rc<RefCell<Node>>> = jcall!(enableNode).getFirstChild();
            { let java_receiver = this.grid.clone(); let java_arguments = (Some(jcall!(enableTextNode).getNodeValue()).unwrap(),); jcall!(java_receiver).showGrid(java_arguments.0); };
            let mut possibleNode: Option<Rc<RefCell<Node>>> = jcall!(DomHelper::getInstance()).searchNodeList(CanvasDom_POSSIBLE.with(Clone::clone).clone(), jcall!(gridNode).getChildNodes());
            let mut possibleTextNode: Option<Rc<RefCell<Node>>> = jcall!(possibleNode).getFirstChild();
            jcall!(this.grid).isGridPossible = Some(jcall!(possibleTextNode).getNodeValue()).unwrap();
            let mut graphicItemNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(DomHelper::getInstance()).getChildrenWithoutTextNodes(CanvasDom_GRAPHICITEMS.with(Clone::clone).clone(), jcall!(canvasNode).getChildNodes());
            if graphicItemNodeList.is_some() {
                this.graphicItemHashMap = jcall!(GraphicItemFactory::getInstance()).getInstance1(graphicItemNodeList.clone());
            }
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Frame Element Not Found but Found: "), name))));
        }
        this
    }

    pub fn new1(mut canvasNode: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(canvasNode))))
    }
    

    pub fn new_struct1(mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>>) -> Self {
        let mut this = Self {
            graphicsStrings: None,
            graphicItemHashMap: None,
            angle: 0.0,
            dimension: None,
            grid: None,
        };
        this.graphicsStrings = GraphicsStrings::getInstance();
        this.angle = jcall!(canvasJPanel).getAngle();
        this.graphicItemHashMap = jcall!(canvasJPanel).getGraphicItemHashMap();
        this.dimension = jcall!(canvasJPanel).getCanvasDimension();
        this.grid = Grid::new1(jcall!(canvasJPanel).getGrid());
        this.angle = jcall!(canvasJPanel).getAngle();
        this
    }

    pub fn new1(mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(canvasJPanel))))
    }
    

    pub fn getAngle(&mut self) -> f64 {
        return self.angle.clone();
    }
    

    pub fn getGraphicItemHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return self.graphicItemHashMap.clone();
    }
    

    pub fn setAngle(&mut self, mut angle: f64) {
        self.angle = angle.clone();
    }
    

    // throws Exception
    pub fn setWorkArea(&mut self, mut x: i32, mut y: i32) {
        self.setDimension(IntegerDimension::new(x.clone(), y.clone()));
    }
    

    // throws Exception
    pub fn setGrid(&mut self, mut gridX: i32, mut gridY: i32) {
        jcall!(self.grid).grid = jcall!(PointFactory::getInstance()).createXY(gridX.clone(), gridY.clone());
    }
    

    pub fn getPointNode(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut pointName: String) -> Option<Rc<RefCell<Node>>> {
        let mut document: Option<Rc<RefCell<Document>>> = WorkAreaJPanel::getDocument();
        let mut pointNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(pointName.clone())).clone() /* as Node */;
        let mut xNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(CanvasDom_X.with(Clone::clone).clone())).clone() /* as Node */;
        let mut xTextNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createTextNode((jcall!(point).getX()).to_string())).clone() /* as Node */;
        jcall!(xNode).appendChild(xTextNode.clone());
        let mut yNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(CanvasDom_Y.with(Clone::clone).clone())).clone() /* as Node */;
        let mut yTextNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createTextNode((jcall!(point).getY()).to_string())).clone() /* as Node */;
        jcall!(yNode).appendChild(yTextNode.clone());
        jcall!(pointNode).appendChild(xNode.clone());
        jcall!(pointNode).appendChild(yNode.clone());
        return pointNode.clone();
    }
    

    // throws Exception
    pub fn toDom(&mut self) -> Option<Rc<RefCell<Node>>> {
        let mut document: Option<Rc<RefCell<Document>>> = WorkAreaJPanel::getDocument();
        let mut frameNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(CanvasDom_FRAME.with(Clone::clone).clone())).clone() /* as Node */;
        let mut angleNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(CanvasDom_ROTATE.with(Clone::clone).clone())).clone() /* as Node */;
        let mut angleTextNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createTextNode(jcall!(Some((self.getAngle()) as f64)).toString())).clone() /* as Node */;
        jcall!(angleNode).appendChild(angleTextNode.clone());
        let mut sizeNode: Option<Rc<RefCell<Node>>> = ({ let java_arguments = (jcall!(PointFactory::getInstance()).createXY(jcall!(self.getDimension()).getWidth(), jcall!(self.getDimension()).getHeight()), CanvasDom_SIZE.with(Clone::clone).clone(),); self.getPointNode(java_arguments.0, java_arguments.1) }).clone() /* as Node */;
        let mut zoomNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(CanvasDom_ZOOM.with(Clone::clone).clone())).clone() /* as Node */;
        let mut zoomTextNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createTextNode(jcall!(Some((jcall!(self.getGrid()).getZoom()) as i32)).toString())).clone() /* as Node */;
        jcall!(zoomNode).appendChild(zoomTextNode.clone());
        let mut gridNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(CanvasDom_GRID.with(Clone::clone).clone())).clone() /* as Node */;
        let mut gridSizeNode: Option<Rc<RefCell<Node>>> = ({ let java_arguments = (jcall!(self.getGrid()).grid.clone(), CanvasDom_SIZE.with(Clone::clone).clone(),); self.getPointNode(java_arguments.0, java_arguments.1) }).clone() /* as Node */;
        jcall!(gridNode).appendChild(gridSizeNode.clone());
        let mut enableNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(CanvasDom_ENABLED.with(Clone::clone).clone())).clone() /* as Node */;
        let mut enableTextNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createTextNode(jcall!(Some(jcall!(self.getGrid()).isGridOn())).toString())).clone() /* as Node */;
        jcall!(enableNode).appendChild(enableTextNode.clone());
        jcall!(gridNode).appendChild(enableNode.clone());
        let mut possibleNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(CanvasDom_POSSIBLE.with(Clone::clone).clone())).clone() /* as Node */;
        let mut possibleTextNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createTextNode(jcall!(Some(jcall!(self.getGrid()).isGridPossible())).toString())).clone() /* as Node */;
        jcall!(possibleNode).appendChild(possibleTextNode.clone());
        jcall!(gridNode).appendChild(possibleNode.clone());
        let mut realSizeNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(CanvasDom_REAL_SIZE.with(Clone::clone).clone())).clone() /* as Node */;
        let mut vectorCenterGenerator: Option<Rc<RefCell<VectorCenterGenerator>>> = VectorCenterGenerator::new();
        jcall!(vectorCenterGenerator).calculate(self.getGraphicItemHashMap());
        let mut widthNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(jcall!(self.graphicsStrings).WIDTH.clone())).clone() /* as Node */;
        let mut widthTextNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createTextNode((jcall!(vectorCenterGenerator).getWidth()).to_string())).clone() /* as Node */;
        jcall!(widthNode).appendChild(widthTextNode.clone());
        jcall!(realSizeNode).appendChild(widthNode.clone());
        let mut heightNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(jcall!(self.graphicsStrings).HEIGHT.clone())).clone() /* as Node */;
        let mut heightTextNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createTextNode((jcall!(vectorCenterGenerator).getHeight()).to_string())).clone() /* as Node */;
        jcall!(heightNode).appendChild(heightTextNode.clone());
        jcall!(realSizeNode).appendChild(heightNode.clone());
        jcall!(frameNode).appendChild(angleNode.clone());
        jcall!(frameNode).appendChild(sizeNode.clone());
        jcall!(frameNode).appendChild(zoomNode.clone());
        jcall!(frameNode).appendChild(gridNode.clone());
        jcall!(frameNode).appendChild(realSizeNode.clone());
        let mut graphicItemNode: Option<Rc<RefCell<Node>>> = (jcall!(document).createElement(CanvasDom_GRAPHICITEMS.with(Clone::clone).clone())).clone() /* as Node */;
        let mut graphicItemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(self.getGraphicItemHashMap()).keySet()).toArray();
        let mut size: i32 = (graphicItemArray.len() as i32).clone();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(self.graphicItemHashMap).get(graphicItemArray[(index) as usize].clone())).clone() /* as GraphicItemInterface */;
            if jcall!(item).getName() == LinesGraphicItem::getStaticName() {
                if jcall!(item).isValid() {
                    jcall!(graphicItemNode).appendChild(jcall!(item).toDom(None /* Java this: a &mut self method has no Rc of itself */));
                }
            }
        }
        jcall!(frameNode).appendChild(graphicItemNode.clone());
        return frameNode.clone();
    }
    

    pub fn getDimension(&mut self) -> Option<Rc<RefCell<IntegerDimension>>> {
        return self.dimension.clone();
    }
    

    pub fn setDimension(&mut self, mut dimension: Option<Rc<RefCell<IntegerDimension>>>) {
        self.dimension = dimension.clone();
    }
    

    pub fn getGrid(&mut self) -> Option<Rc<RefCell<Grid>>> {
        return self.grid.clone();
    }
}

