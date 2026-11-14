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
// package org.allbinary.game.part
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::PartInterface::PartInterface;
use super::CountedLayerInterfaceFactoryPart::CountedLayerInterfaceFactoryPart;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CountedLayersHudPaintable {
    pub displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub partInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>>,
    pub countedTotalStringColor: i32,
    pub countedPartsBorder: i32,
    pub startIndex: i32,
    pub dropSize: i32,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub height: i32,
}

pub static mut CountedLayersHudPaintable_XXStringWidth: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);
impl PaintableInterface for CountedLayersHudPaintable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paintThreed(self, graphics) }
}
impl UpdateMyFontInterface for CountedLayersHudPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}

impl CountedLayersHudPaintable {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut partInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>>, mut dropSize: i32, mut startIndex: i32, mut countedTotalStringColor: i32, mut countedPartsBorder: i32) -> Self {
        let mut this = Self {
            displayInfoSingleton: None,
            partInterfaceArray: Vec::new(),
            countedTotalStringColor: 0,
            countedPartsBorder: 0,
            startIndex: 0,
            dropSize: 0,
            myFontProcessor: None,
            height: 0,
        };
        this.displayInfoSingleton = DisplayInfoSingleton::getInstance();
        this.partInterfaceArray = partInterfaceArray.clone();
        this.startIndex = startIndex.clone();
        this.countedTotalStringColor = countedTotalStringColor.clone();
        this.countedPartsBorder = countedPartsBorder.clone();
        this.dropSize = dropSize.clone();
        this
    }

    pub fn new(mut partInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>>, mut dropSize: i32, mut startIndex: i32, mut countedTotalStringColor: i32, mut countedPartsBorder: i32) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(partInterfaceArray.clone(), dropSize.clone(), startIndex.clone(), countedTotalStringColor.clone(), countedPartsBorder.clone()))));
        jcall!(this).myFontProcessor = UpdateMyFontProcessor::new(this.clone().map(|reference| reference as Rc<RefCell<dyn UpdateMyFontInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        self.height = { let java_value = (jcall!(font).getHeight()).clone(); java_value };
        if self.dropSize > jcall!(font).getHeight() {
            self.height = self.dropSize.clone();
        }
        if unsafe { CountedLayersHudPaintable_XXStringWidth.clone() } == 0 {
            let mut XXString: String = String::from("XX");
            unsafe { *CountedLayersHudPaintable_XXStringWidth = jcall!(font).stringWidth(XXString.clone()) };
        }
        self.myFontProcessor = { let java_value = (MyFontProcessor::getInstance()).clone(); java_value };
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        let mut lastWidth: i32 = jcall!(self.displayInfoSingleton).getLastWidth();
        let mut widthEdge: i32 = lastWidth - self.dropSize;
        let mut size: i32 = (self.partInterfaceArray.len() as i32).clone();
        let mut count: i32 = 0;
        let mut y: i32 = 0;
        let mut countedLayerInterfaceFactory: Option<Rc<RefCell<CountedLayerInterfaceFactoryPart>>> = None;
        let mut pickedUpLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>> = None;
        let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = None;
        let mut charArray: Vec<char> = Vec::new();
        for index in (self.startIndex..size) {
            countedLayerInterfaceFactory = (self.partInterfaceArray[(index) as usize]).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<CountedLayerInterfaceFactoryPart>() }, "ClassCastException: not a CountedLayerInterfaceFactoryPart"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<CountedLayerInterfaceFactoryPart>) } });
            if jcall!(countedLayerInterfaceFactory).getTotal() > 0 {
                pickedUpLayerInterfaceFactoryInterface = { let java_value = (jcall!(countedLayerInterfaceFactory).getCountedPickedUpLayerInterfaceFactory()).clone(); java_value }.map(|reference| reference as Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>);
                layerInterface = { let java_value = (jcall!(pickedUpLayerInterfaceFactoryInterface).getIconLayer()).clone(); java_value };
                y = 40 + (count * self.height);
                jcall!(layerInterface).setPosition(widthEdge.clone(), y.clone(), jcall!(layerInterface).getZP());
                jcall!(layerInterface).paint(graphics.clone());
                jcall!(graphics).setColor(self.countedTotalStringColor.clone());
                jcall!(countedLayerInterfaceFactory).paint(graphics.clone());
                charArray = { let java_value = (jcall!(countedLayerInterfaceFactory).getTotalString()).clone(); java_value };
                jcall!(graphics).drawChars(charArray.clone(), 0, (charArray.len() as i32).clone(), widthEdge - jcall!(countedLayerInterfaceFactory).getXOffset(), y.clone(), 0);
                count += 1;
            }
        }
        if count > 0 {
            jcall!(graphics).setColor(self.countedPartsBorder.clone());
            jcall!(graphics).drawRect(lastWidth - (unsafe { CountedLayersHudPaintable_XXStringWidth.clone() } + self.dropSize), 40, unsafe { CountedLayersHudPaintable_XXStringWidth.clone() } + self.dropSize, (count * self.height) + 3);
        }
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
}

