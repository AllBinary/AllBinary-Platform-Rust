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
use crate::javax::microedition::lcdui::Graphics::Graphics;
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
use crate::java::lang::Exception::*;
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
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}

impl BasicHud_1 {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
        this.myFontProcessor = this.updateMyFontProcessor.clone();
        this.hudGraphicsPosition = HudGraphicsPosition_NULL_HUD_GRAPHICS_POSITION.with(Clone::clone).clone();
        this.basicColor = { let java_field = jcall!(BasicColorFactory::getInstance()).BLACK.clone(); java_field }.clone();
        this.setLocation(location.clone());
        this.setDirection(direction.clone());
        this.setBufferZone(bufferZone.clone());
        { let java_arguments = ({ let java_field = jcall!(DisplayInfoSingleton::getInstance()).displayChangeEvent.clone(); java_field }.clone(),); this.onDisplayChangeEvent(java_arguments.0) };
        this.setBasicColorP(basicColor.clone());
        this.color = { let java_value = (jcall!(basicColor).intValue()).clone(); java_value };
        this
    }

    pub fn new(mut location: i32, mut direction: i32, mut bufferZone: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(location.clone(), direction.clone(), bufferZone.clone(), basicColor.clone()))));
        jcall!(this).updateMyFontProcessor = UpdateMyFontProcessor::new(this.clone().map(|reference| reference as Rc<RefCell<dyn UpdateMyFontInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.hudGraphicsPosition = { let java_value = ({ let java_arguments = (jcall!(self.displayInfo).getLastWidth(), jcall!(self.displayInfo).getLastHeight(), self.updateMaxWidth.clone(), self.updateMaxHeight.clone(),); self.getHudGraphicsPositionWH(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3) }).clone(); java_value };
            self.x = { let java_value = (jcall!(jcall!(self.hudGraphicsPosition).getPoint()).getX()).clone(); java_value };
            { let java_arguments = (jcall!(jcall!(self.hudGraphicsPosition).getPoint()).getY(),); self.setY(java_arguments.0) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.canvasStrings).ON_DISPLAY_CHANGE_EVENT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
        self.myFontProcessor = { let java_value = (MyFontProcessor::getInstance()).clone(); java_value };
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
        if { let java_field = jcall!(basicHudFactory).BOTTOMLEFT.clone(); java_field } == self.getLocation() {
            x = self.bufferZone + 2;
            y = height - maxHeight - self.bufferZone;
            anchor = { let java_value = (crate::javax::microedition::lcdui::Graphics::Graphics_BOTTOM.with(Clone::clone) & crate::javax::microedition::lcdui::Graphics::Graphics_LEFT.with(Clone::clone)).clone(); java_value };
        } else if { let java_field = jcall!(basicHudFactory).BOTTOMRIGHT.clone(); java_field } == self.getLocation() {
            x = width - maxWidth;
            y = height - maxHeight - self.bufferZone;
            anchor = { let java_value = (crate::javax::microedition::lcdui::Graphics::Graphics_BOTTOM.with(Clone::clone) & crate::javax::microedition::lcdui::Graphics::Graphics_RIGHT.with(Clone::clone)).clone(); java_value };
        } else if { let java_field = jcall!(basicHudFactory).TOPLEFT.clone(); java_field } == self.getLocation() {
            x = self.bufferZone + 2;
            y = self.bufferZone + 5;
            anchor = { let java_value = (unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }).clone(); java_value }.clone();
        } else if { let java_field = jcall!(basicHudFactory).TOPRIGHT.clone(); java_field } == self.getLocation() {
            x = width - maxWidth;
            y = self.bufferZone + 5;
            anchor = { let java_value = (crate::javax::microedition::lcdui::Graphics::Graphics_TOP.with(Clone::clone) & crate::javax::microedition::lcdui::Graphics::Graphics_RIGHT.with(Clone::clone)).clone(); java_value };
        } else if { let java_field = jcall!(basicHudFactory).TOPCENTER.clone(); java_field } == self.getLocation() {
            x = ((width - maxWidth) / 2);
            y = self.bufferZone + 5;
            anchor = { let java_value = (crate::javax::microedition::lcdui::Graphics::Graphics_TOP.with(Clone::clone) & crate::javax::microedition::lcdui::Graphics::Graphics_HCENTER.with(Clone::clone)).clone(); java_value };
        } else if { let java_field = jcall!(basicHudFactory).BOTTOMCENTER.clone(); java_field } == self.getLocation() {
            x = ((width - maxWidth) / 2);
            y = height - maxHeight - self.bufferZone;
            anchor = { let java_value = (crate::javax::microedition::lcdui::Graphics::Graphics_BOTTOM.with(Clone::clone) & crate::javax::microedition::lcdui::Graphics::Graphics_HCENTER.with(Clone::clone)).clone(); java_value };
        } else if { let java_field = jcall!(basicHudFactory).ABSOLUTE.clone(); java_field } == self.getLocation() {
            x = maxHeight.clone();
            y = maxWidth.clone();
            anchor = 0;
        }
        return HudGraphicsPosition::new(self.getPoint(x.clone(), y.clone()), anchor.clone());
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log({ let java_field = jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
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
    

    pub fn paintSSO(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut string: String, mut string2: String, mut offset: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(),); jcall!(java_receiver).setBasicColorP(java_arguments.0, java_arguments.1); };
        jcall!(graphics).drawString(string.clone(), self.x.clone(), self.getY(), { let java_argument = jcall!(self.hudGraphicsPosition).getAnchor(); java_argument });
        jcall!(graphics).drawString(string2.clone(), self.x + offset, self.getY(), { let java_argument = jcall!(self.hudGraphicsPosition).getAnchor(); java_argument });
    }
    

    pub fn paintSSOO(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut string: String, mut string2: String, mut offset: i32, mut offset2: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        jcall!(graphics).setColor(self.getColor());
        jcall!(graphics).drawString(string.clone(), self.x + offset, self.getY(), { let java_argument = jcall!(self.hudGraphicsPosition).getAnchor(); java_argument });
        jcall!(graphics).drawString(string2.clone(), self.x + offset2, self.getY(), { let java_argument = jcall!(self.hudGraphicsPosition).getAnchor(); java_argument });
    }
    

    pub fn paintDXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut charArray: Vec<char>, mut offset: i32, mut len: i32, mut charArray2: Vec<char>, mut offset2: i32, mut len2: i32, mut xOffset: i32, mut xOffset2: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(),); jcall!(java_receiver).setBasicColorP(java_arguments.0, java_arguments.1); };
        let mut y: i32 = self.getY();
        jcall!(graphics).drawChars(charArray.clone(), offset.clone(), len.clone(), self.x + xOffset, y.clone(), jcall!(self.hudGraphicsPosition).getAnchor());
        jcall!(graphics).drawChars(charArray2.clone(), offset2.clone(), len2.clone(), self.x + xOffset2, y.clone(), jcall!(self.hudGraphicsPosition).getAnchor());
    }
    

    pub fn paintDX(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut charArray: Vec<char>, mut offset: i32, mut len: i32, mut charArray2: Vec<char>, mut offset2: i32, mut len2: i32, mut xOffset: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(),); jcall!(java_receiver).setBasicColorP(java_arguments.0, java_arguments.1); };
        let mut y: i32 = self.getY();
        jcall!(graphics).drawChars(charArray.clone(), offset.clone(), len.clone(), self.x.clone(), y.clone(), jcall!(self.hudGraphicsPosition).getAnchor());
        jcall!(graphics).drawChars(charArray2.clone(), offset2.clone(), len2.clone(), self.x + xOffset, y.clone(), jcall!(self.hudGraphicsPosition).getAnchor());
    }
    

    pub fn paintOffsetAndLength(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut charArray: Vec<char>, mut offset: i32, mut len: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(),); jcall!(java_receiver).setBasicColorP(java_arguments.0, java_arguments.1); };
        let mut y: i32 = self.getY();
        jcall!(graphics).drawChars(charArray.clone(), offset.clone(), len.clone(), self.x.clone(), y.clone(), jcall!(self.hudGraphicsPosition).getAnchor());
    }
    

    pub fn paintString(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut string: String) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(),); jcall!(java_receiver).setBasicColorP(java_arguments.0, java_arguments.1); };
        let mut y: i32 = self.getY() + self.offsetY;
        jcall!(graphics).drawString(string.clone(), self.x.clone(), y.clone(), jcall!(self.hudGraphicsPosition).getAnchor());
    }
}

