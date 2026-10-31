// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::SelectionHudPaintable::SelectionHudPaintable;
use super::RTSLayer::RTSLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultiSelectPaintable {
    pub base: SelectionHudPaintable,
    pub rootNameList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub totalCharArray: Vec<char>,
    pub rootNamesString: String,
    pub TOTAL: String,
    pub backgroundColor: i32,
    pub totalWidth: i32,
    pub textLine2Y: i32,
}

impl crate::org::allbinary::graphics::paint::InitUpdatePaintableInterface::InitUpdatePaintableInterface for MultiSelectPaintable {
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for MultiSelectPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::init::InitInterface::InitInterface for MultiSelectPaintable {
    fn init(&mut self) { self.base.init() }
}
impl crate::org::allbinary::init::UpdateInterface::UpdateInterface for MultiSelectPaintable {
    fn update(&mut self) { self.base.update() }
}

impl MultiSelectPaintable {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: SelectionHudPaintable::new_struct(),
            rootNameList: None,
            totalCharArray: Vec::new(),
            rootNamesString: String::new(),
            TOTAL: String::new(),
            backgroundColor: 0,
            totalWidth: 0,
            textLine2Y: 0,
        };
        this.rootNameList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.totalCharArray = jcall!(NullUtil::getInstance()).NULL_CHAR_ARRAY.clone();
        this.rootNamesString = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.TOTAL = String::from("Total Selected: ");
        this.backgroundColor = jcall!(jcall!(BasicColorFactory::getInstance()).GREY).intValue();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.updateMeasurement(graphics.clone());
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.totalWidth = jcall!(font).stringWidth(self.TOTAL.clone());
        self.textLine2Y = (self.y + jcall!(font).getHeight());
    }
    

    pub fn update(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.clear();
        let mut size: i32 = jcall!(list).size();
        self.totalCharArray = jcall!(self.getPrimitiveLongUtil()).getCharArray(size.clone());
        for index in (0..=jcall!(list).size() - 1).rev() {
            let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>>().expect("ClassCastException").clone()) };
            if !{ let java_receiver = self.rootNameList.clone(); let java_arguments = (crate::java_into_object(jcall!(rtsLayer).getRootName()),); let java_result = jcall!(java_receiver).contains(java_arguments.0); java_result } {
                { let java_receiver = self.rootNameList.clone(); let java_arguments = (crate::java_into_object(jcall!(rtsLayer).getRootName()),); jcall!(java_receiver).add(java_arguments.0); };
            }
        }
        let mut COMMA_SEP: String = jcall!(CommonSeps::getInstance()).COMMA_SEP.clone();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        for index in (0..=jcall!(self.rootNameList).size() - 1).rev() {
            let mut rootName: String = (jcall!(self.rootNameList).get(index.clone())).clone() /* as String */;
            jcall!(stringBuffer).append(rootName.clone());
            if index != 0 {
                jcall!(stringBuffer).append(COMMA_SEP.clone());
            }
        }
        self.rootNamesString = jcall!(stringBuffer).toString();
    }
    

    pub fn clear(&mut self) {
        jcall!(self.rootNameList).clear();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(graphics).setColor(self.backgroundColor.clone());
        jcall!(graphics).drawRect(self.getX(), self.y.clone(), self.getWidth(), self.getHeight());
        jcall!(graphics).setColor(self.getColor());
        jcall!(graphics).drawString(self.TOTAL.clone(), self.textX.clone(), self.y.clone(), 0);
        jcall!(graphics).drawChars(self.totalCharArray.clone(), 0, jcall!(self.getPrimitiveLongUtil()).getCurrentTotalDigits(), self.textX + self.totalWidth, self.y.clone(), 0);
        jcall!(graphics).drawString(self.rootNamesString.clone(), self.textX.clone(), self.textLine2Y.clone(), 0);
    }
}
impl std::ops::Deref for MultiSelectPaintable {
    type Target = SelectionHudPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultiSelectPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn for MultiSelectPaintable {
    fn as_SelectionHudPaintable(&self) -> &crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable { &self.base }
    fn as_SelectionHudPaintable_mut(&mut self) -> &mut crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable { &mut self.base }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn for MultiSelectPaintable {
    fn as_InitUpdatePaintable(&self) -> &crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &self.base.base }
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &mut self.base.base }
    fn update(&mut self) { crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable::update(crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn::as_SelectionHudPaintable_mut(self)) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for MultiSelectPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base.base }
    fn setBasicColorP(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::color::BasicColor::BasicColor>>>) { crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable::setBasicColorP(crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn::as_SelectionHudPaintable_mut(self), p1) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


