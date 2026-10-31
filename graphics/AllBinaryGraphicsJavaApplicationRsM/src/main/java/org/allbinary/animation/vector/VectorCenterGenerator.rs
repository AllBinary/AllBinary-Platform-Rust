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
// package org.allbinary.animation.vector
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::j2me::workarea::tools::GraphicItemInterface::GraphicItemInterface;
use crate::org::allbinary::graphics::j2me::workarea::tools::LinesGraphicItem::LinesGraphicItem;
use crate::org::allbinary::graphics::j2me::workarea::tools::Points::Points;
use crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipeline;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VectorCenterGenerator {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub minX: i32,
    pub minY: i32,
    pub maxX: i32,
    pub maxY: i32,
    pub dx: i32,
    pub dy: i32,
    pub width: i32,
    pub height: i32,
}


impl VectorCenterGenerator {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            minX: 0,
            minY: 0,
            maxX: 0,
            maxY: 0,
            dx: 0,
            dy: 0,
            width: 0,
            height: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.minX = i32::MAX.clone();
        this.minY = i32::MAX.clone();
        this.maxX = i32::MIN.clone();
        this.maxY = i32::MIN.clone();
        this.dx = 0;
        this.dy = 0;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn calculate(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        let mut graphicItemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(hashMap).keySet()).toArray();
        let mut size: i32 = (graphicItemArray.len() as i32).clone();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(hashMap).get(graphicItemArray[(index) as usize].clone())).clone() /* as GraphicItemInterface */;
            if jcall!(item).getName() == LinesGraphicItem::getStaticName() {
                { let java_arguments = (jcall!(jcall!(item).getPointsInterface()).getPoints(),); self.getInstance(java_arguments.0) };
            }
        }
        jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("minX: "), self.minX), String::from(" minY: ")), self.minY), String::from(" maxX: ")), self.maxX), String::from(" maxY: ")), self.maxY), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone());
        self.setWidth(self.maxX - self.minX);
        self.setHeight(self.maxY - self.minY);
        let mut max: i32 = self.getWidth();
        if self.getHeight() > max {
            max = self.getHeight();
        }
        let mut middle: i32 = max / 2;
        let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
        let mut s: String = jcall!(StringMaker::new()).append(jcall!(commonLabels).WIDTH_LABEL.clone()).appendint(self.getWidth()).append(jcall!(commonLabels).HEIGHT_LABEL.clone()).appendint(self.getHeight()).append(String::from(" max: ")).appendint(max.clone()).append(String::from(" middle: ")).appendint(middle.clone()).toString();
        jcall!(self.logUtil).putF(s.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone());
        let mut currentMiddleX: i32 = self.minX + self.getWidth() / 2;
        let mut currentMiddleY: i32 = self.minY + self.getHeight() / 2;
        self.dx = middle - currentMiddleX;
        self.dy = middle - currentMiddleY;
    }
    

    // throws Exception
    pub fn transform(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        self.calculate(hashMap.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from(" dx: ")).appendint(self.dx.clone()).append(String::from(" dy: ")).appendint(self.dy.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut graphicItemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(hashMap).keySet()).toArray();
        let mut size: i32 = (graphicItemArray.len() as i32).clone();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(hashMap).get(graphicItemArray[(index) as usize].clone())).clone() /* as GraphicItemInterface */;
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = VectorCenterGenerator::translate(jcall!(jcall!(item).getPointsInterface()).getPoints(), self.dx.clone(), self.dy.clone());
            let mut newPoints: Option<Rc<RefCell<Points>>> = Points::new();
            jcall!(newPoints).addPoints(list.clone());
            jcall!(item).setPointsInterface(newPoints.clone());
        }
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut pointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut size: i32 = jcall!(pointVector).size();
            for index in (0..size) {
                let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = { let java_object = (jcall!(pointVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
                if jcall!(point).getX() < self.minX {
                    self.minX = jcall!(point).getX();
                }
                if jcall!(point).getX() > self.maxX {
                    self.maxX = jcall!(point).getX();
                }
                if jcall!(point).getY() < self.minY {
                    self.minY = jcall!(point).getY();
                }
                if jcall!(point).getY() > self.maxY {
                    self.maxY = jcall!(point).getY();
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()));
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn translate(mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut graphicsPipe: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipelineDyn>>> = BasicGraphicsPipeline::new(list.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::pipeline::BasicGraphicsPipeline::BasicGraphicsPipelineDyn>>);
        jcall!(graphicsPipe).createMatrix();
        jcall!(graphicsPipe).translate(x.clone(), y.clone());
        return jcall!(graphicsPipe).getMatrix();
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return self.width.clone();
    }
    

    pub fn setWidth(&mut self, mut width: i32) {
        self.width = width.clone();
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return self.height.clone();
    }
    

    pub fn setHeight(&mut self, mut height: i32) {
        self.height = height.clone();
    }
}

