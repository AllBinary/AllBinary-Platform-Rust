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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::CellPositionFactory::CellPositionFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TouchButtonLocationHelper::TouchButtonLocationHelper;
use super::CommonButtons::CommonButtons;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonsMappingPaintable {
    pub base: Paintable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub foregroundColor: i32,
    pub paintableTable: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>>>,
    pub touchButtonLocationHelper: Option<Rc<RefCell<TouchButtonLocationHelper>>>,
}


impl TouchButtonsMappingPaintable {
    

    pub fn new_struct(mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            logUtil: None,
            foregroundColor: 0,
            paintableTable: Vec::new(),
            touchButtonLocationHelper: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.paintableTable = vec![vec![None; (0) as usize]; (0) as usize];
        this.touchButtonLocationHelper = TouchButtonLocationHelper::new();
        this.foregroundColor = jcall!(basicColor).intValue();
        this.init();
        this
    }

    pub fn new(mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicColor))))
    }
    

    pub fn init(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.paintableTable = self.createPaintableTable();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateRectangle"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn createPaintableTable(&mut self) -> Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>>> {
        let mut totalColumns: i32 = jcall!(self.touchButtonLocationHelper).getTotalColumns();
        let mut totalRows: i32 = jcall!(self.touchButtonLocationHelper).getTotalRows();
        let mut paintableTable: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>>> = vec![vec![None; (totalRows) as usize]; (totalColumns) as usize];
        let mut cellPositionFactory: Option<Rc<RefCell<CellPositionFactory>>> = CellPositionFactory::getInstance();
        let mut commonButtons: Option<Rc<RefCell<CommonButtons>>> = CommonButtons::getInstance();
        for index in (0..=totalColumns - 1).rev() {
            for rowIndex in (0..=totalRows - 1).rev() {
                self.paintableTable[(index) as usize][(rowIndex) as usize] = TouchButton::TouchButton::createButton(jcall!(BasicTouchInputFactory::BasicTouchInputFactory::getInstance()).NONE.clone(), TouchButtonBlankResource::TouchButtonBlankResource::getInstance(), jcall!(commonButtons).NORMAL_BUTTON.clone(), jcall!(cellPositionFactory).getInstanceColRow(index.clone(), rowIndex.clone()), jcall!(self.touchButtonLocationHelper).getColumnsRemainderHalf(), jcall!(self.touchButtonLocationHelper).getRowsRemainderHalf());
            }
        }
        return paintableTable.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut totalColumns: i32 = jcall!(self.touchButtonLocationHelper).getTotalColumns();
        let mut totalRows: i32 = jcall!(self.touchButtonLocationHelper).getTotalRows();
        for index in (0..=totalColumns - 1).rev() {
            for rowIndex in (0..=totalRows - 1).rev() {
                jcall!(self.paintableTable[(index) as usize][(rowIndex) as usize]).paint(graphics.clone());
            }
        }
    }
}
impl std::ops::Deref for TouchButtonsMappingPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TouchButtonsMappingPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for TouchButtonsMappingPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


