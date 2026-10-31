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
// package org.allbinary.game.graphics.hud
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtil;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::HudGraphicsPosition::HudGraphicsPosition;
use super::BasicHudFactory::BasicHudFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicHud_1 {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub canvasStrings: Option<Rc<RefCell<CanvasStrings>>>,
    pub basicSetColorUtil: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtilDyn>>>,
    pub displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub updateMyFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub location: i32,
    pub direction: i32,
    pub bufferZone: i32,
    pub hudGraphicsPosition: Option<Rc<RefCell<HudGraphicsPosition>>>,
    pub x: i32,
    pub y: i32,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
    pub color: i32,
    pub updateMaxWidth: i32,
    pub updateMaxHeight: i32,
    pub offsetY: i32,
}

impl UpdateMyFontInterface for BasicHud_1 {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl BasicHud_1 {
    

    pub fn new_struct(mut location: i32, mut direction: i32, mut bufferZone: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            canvasStrings: None,
            basicSetColorUtil: None,
            displayInfo: None,
            updateMyFontProcessor: None,
            myFontProcessor: None,
            location: 0,
            direction: 0,
            bufferZone: 0,
            hudGraphicsPosition: None,
            x: 0,
            y: 0,
            basicColor: None,
            color: 0,
            updateMaxWidth: 0,
            updateMaxHeight: 0,
            offsetY: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.canvasStrings = CanvasStrings::getInstance();
        this.basicSetColorUtil = BasicColorSetUtil::getInstance();
        this.displayInfo = DisplayInfoSingleton::getInstance();
        this.updateMyFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.myFontProcessor = this.updateMyFontProcessor.clone();
        this.hudGraphicsPosition = HudGraphicsPosition_NULL_HUD_GRAPHICS_POSITION.with(Clone::clone).clone();
        this.basicColor = jcall!(BasicColorFactory::getInstance()).BLACK.clone();
        this.setLocation(location.clone());
        this.setDirection(direction.clone());
        this.setBufferZone(bufferZone.clone());
        { let java_arguments = (jcall!(DisplayInfoSingleton::getInstance()).displayChangeEvent.clone(),); this.onDisplayChangeEvent(java_arguments.0) };
        this.setBasicColorP(basicColor.clone());
        this.color = jcall!(basicColor).intValue();
        this
    }

    pub fn new(mut location: i32, mut direction: i32, mut bufferZone: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(location, direction, bufferZone, basicColor))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.hudGraphicsPosition = { let java_arguments = (jcall!(self.displayInfo).getLastWidth(), jcall!(self.displayInfo).getLastHeight(), self.updateMaxWidth.clone(), self.updateMaxHeight.clone(),); self.getHudGraphicsPositionWH(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3) };
            self.x = jcall!(jcall!(self.hudGraphicsPosition).getPoint()).getX();
            { let java_arguments = (jcall!(jcall!(self.hudGraphicsPosition).getPoint()).getY(),); self.setY(java_arguments.0) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).ON_DISPLAY_CHANGE_EVENT.clone(), crate::java_into_object(e.clone()));
            }
        }
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn getLocation(&mut self) -> i32 {
        return self.location.clone();
    }
    

    pub fn getDirection(&mut self) -> i32 {
        return self.direction.clone();
    }
    

    // throws Exception
    pub fn getHudGraphicsPositionWH(&mut self, mut width: i32, mut height: i32, mut maxWidth: i32, mut maxHeight: i32) -> Option<Rc<RefCell<HudGraphicsPosition>>> {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut anchor: i32 = 0;
        let mut basicHudFactory: Option<Rc<RefCell<BasicHudFactory>>> = BasicHudFactory::getInstance();
        if jcall!(basicHudFactory).BOTTOMLEFT == self.getLocation() {
            x = self.bufferZone + 2;
            y = height - maxHeight - self.bufferZone;
            anchor = unsafe { crate::javax::microedition::lcdui::Graphics::Graphics_BOTTOM.clone() } & unsafe { crate::javax::microedition::lcdui::Graphics::Graphics_LEFT.clone() };
        } else if jcall!(basicHudFactory).BOTTOMRIGHT == self.getLocation() {
            x = width - maxWidth;
            y = height - maxHeight - self.bufferZone;
            anchor = unsafe { crate::javax::microedition::lcdui::Graphics::Graphics_BOTTOM.clone() } & unsafe { crate::javax::microedition::lcdui::Graphics::Graphics_RIGHT.clone() };
        } else if jcall!(basicHudFactory).TOPLEFT == self.getLocation() {
            x = self.bufferZone + 2;
            y = self.bufferZone + 5;
            anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        } else if jcall!(basicHudFactory).TOPRIGHT == self.getLocation() {
            x = width - maxWidth;
            y = self.bufferZone + 5;
            anchor = unsafe { crate::javax::microedition::lcdui::Graphics::Graphics_TOP.clone() } & unsafe { crate::javax::microedition::lcdui::Graphics::Graphics_RIGHT.clone() };
        } else if jcall!(basicHudFactory).TOPCENTER == self.getLocation() {
            x = ((width - maxWidth) / 2);
            y = self.bufferZone + 5;
            anchor = unsafe { crate::javax::microedition::lcdui::Graphics::Graphics_TOP.clone() } & unsafe { crate::javax::microedition::lcdui::Graphics::Graphics_HCENTER.clone() };
        } else if jcall!(basicHudFactory).BOTTOMCENTER == self.getLocation() {
            x = ((width - maxWidth) / 2);
            y = height - maxHeight - self.bufferZone;
            anchor = unsafe { crate::javax::microedition::lcdui::Graphics::Graphics_BOTTOM.clone() } & unsafe { crate::javax::microedition::lcdui::Graphics::Graphics_HCENTER.clone() };
        } else if jcall!(basicHudFactory).ABSOLUTE == self.getLocation() {
            x = maxHeight.clone();
            y = maxWidth.clone();
            anchor = 0;
        }
        return HudGraphicsPosition::new(self.getPoint(x.clone(), y.clone()), anchor.clone());
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) {
        self.myFontProcessor = self.updateMyFontProcessor.clone();
    }
    

    pub fn getPoint(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return jcall!(PointFactory::getInstance()).createXY(x.clone(), y.clone());
    }
    

    pub fn getBufferZone(&mut self) -> i32 {
        return self.bufferZone.clone();
    }
    

    pub fn setBufferZone(&mut self, mut bufferZone: i32) {
        self.bufferZone = bufferZone.clone();
    }
    

    pub fn setLocation(&mut self, mut location: i32) {
        self.location = location.clone();
    }
    

    pub fn setDirection(&mut self, mut direction: i32) {
        self.direction = direction.clone();
    }
    

    pub fn getHudGraphicsPosition(&mut self) -> Option<Rc<RefCell<HudGraphicsPosition>>> {
        return self.hudGraphicsPosition.clone();
    }
    

    pub fn getColor(&mut self) -> i32 {
        return self.color.clone();
    }
    

    pub fn getX(&mut self) -> i32 {
        return self.x.clone();
    }
    

    pub fn setX(&mut self, mut x: i32) {
        self.x = x.clone();
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.basicColor = basicColor.clone();
    }
    

    pub fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.basicColor.clone();
    }
    

    pub fn setY(&mut self, mut y: i32) {
        self.y = y.clone();
    }
    

    pub fn getY(&mut self) -> i32 {
        return self.y.clone();
    }
    

    pub fn paintSSO(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut string: String, mut string2: String, mut offset: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(),); jcall!(java_receiver).setBasicColorP(java_arguments.0, java_arguments.1); };
        jcall!(graphics).drawString(string.clone(), self.x.clone(), self.getY(), jcall!(self.hudGraphicsPosition).getAnchor());
        jcall!(graphics).drawString(string2.clone(), self.x + offset, self.getY(), jcall!(self.hudGraphicsPosition).getAnchor());
    }
    

    pub fn paintSSOO(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut string: String, mut string2: String, mut offset: i32, mut offset2: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        jcall!(graphics).setColor(self.getColor());
        jcall!(graphics).drawString(string.clone(), self.x + offset, self.getY(), jcall!(self.hudGraphicsPosition).getAnchor());
        jcall!(graphics).drawString(string2.clone(), self.x + offset2, self.getY(), jcall!(self.hudGraphicsPosition).getAnchor());
    }
    

    pub fn paintDXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut charArray: Vec<char>, mut offset: i32, mut len: i32, mut charArray2: Vec<char>, mut offset2: i32, mut len2: i32, mut xOffset: i32, mut xOffset2: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(),); jcall!(java_receiver).setBasicColorP(java_arguments.0, java_arguments.1); };
        let mut y: i32 = self.getY();
        jcall!(graphics).drawChars(charArray.clone(), offset.clone(), len.clone(), self.x + xOffset, y.clone(), jcall!(self.hudGraphicsPosition).getAnchor());
        jcall!(graphics).drawChars(charArray2.clone(), offset2.clone(), len2.clone(), self.x + xOffset2, y.clone(), jcall!(self.hudGraphicsPosition).getAnchor());
    }
    

    pub fn paintDX(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut charArray: Vec<char>, mut offset: i32, mut len: i32, mut charArray2: Vec<char>, mut offset2: i32, mut len2: i32, mut xOffset: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(),); jcall!(java_receiver).setBasicColorP(java_arguments.0, java_arguments.1); };
        let mut y: i32 = self.getY();
        jcall!(graphics).drawChars(charArray.clone(), offset.clone(), len.clone(), self.x.clone(), y.clone(), jcall!(self.hudGraphicsPosition).getAnchor());
        jcall!(graphics).drawChars(charArray2.clone(), offset2.clone(), len2.clone(), self.x + xOffset, y.clone(), jcall!(self.hudGraphicsPosition).getAnchor());
    }
    

    pub fn paintOffsetAndLength(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut charArray: Vec<char>, mut offset: i32, mut len: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(),); jcall!(java_receiver).setBasicColorP(java_arguments.0, java_arguments.1); };
        let mut y: i32 = self.getY();
        jcall!(graphics).drawChars(charArray.clone(), offset.clone(), len.clone(), self.x.clone(), y.clone(), jcall!(self.hudGraphicsPosition).getAnchor());
    }
    

    pub fn paintString(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut string: String) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(),); jcall!(java_receiver).setBasicColorP(java_arguments.0, java_arguments.1); };
        let mut y: i32 = self.getY() + self.offsetY;
        jcall!(graphics).drawString(string.clone(), self.x.clone(), y.clone(), jcall!(self.hudGraphicsPosition).getAnchor());
    }
}

