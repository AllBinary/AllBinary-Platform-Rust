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
// package org.allbinary.game.ag.layer.hud.basic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::business::advertisement::GameAdStateFactory::GameAdStateFactory;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::game::GameAdState::GameAdState;
use crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotification::GameNotification;
use crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHud;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEventHandler::GameNotificationEventHandler;
use crate::org::allbinary::graphics::CustomGPoint::CustomGPoint;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::GameTickTimeDelayHelper::GameTickTimeDelayHelper;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::SetAndRemoveProcessor::SetAndRemoveProcessor;
use super::NextUnremoveableProcessor::NextUnremoveableProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlayerGameNotificationHud {
    pub base: GameNotificationHud,
    pub gameTickTimeDelayHelper: Option<Rc<RefCell<GameTickTimeDelayHelper>>>,
    pub EMPTY_STRING: String,
    pub displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub gameNotification: Option<Rc<RefCell<GameNotification>>>,
    pub permanentGameNotification: Option<Rc<RefCell<GameNotification>>>,
    pub setAndRemoveProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub nextUnremoveableProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub processor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub PERMANENT_GAME_NOTIFICATION: String,
    pub lastString: String,
    pub string: String,
    pub point: Option<Rc<RefCell<CustomGPoint>>>,
    pub width: i32,
}

impl UpdateMyFontInterface for PlayerGameNotificationHud {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl PlayerGameNotificationHud {
    

    pub fn new_struct(mut location: i32, mut direction: i32, mut bufferZone: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: GameNotificationHud::new_struct(location.clone(), direction.clone(), bufferZone.clone(), basicColor.clone()),
            gameTickTimeDelayHelper: None,
            EMPTY_STRING: String::new(),
            displayInfo: None,
            timeDelayHelper: None,
            circularIndexUtil: None,
            gameNotification: None,
            permanentGameNotification: None,
            setAndRemoveProcessor: None,
            nextUnremoveableProcessor: None,
            processor: None,
            PERMANENT_GAME_NOTIFICATION: String::new(),
            lastString: String::new(),
            string: String::new(),
            point: None,
            width: 0,
        };
        this.gameTickTimeDelayHelper = GameTickTimeDelayHelperFactory::getInstance();
        this.EMPTY_STRING = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.displayInfo = DisplayInfoSingleton::getInstance();
        this.timeDelayHelper = TimeDelayHelper::new(0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.gameNotification = GameNotification::new();
        this.permanentGameNotification = GameNotification::new();
        this.setAndRemoveProcessor = SetAndRemoveProcessor::new(this);
        this.nextUnremoveableProcessor = NextUnremoveableProcessor::new(this);
        this.processor = Processor::getInstance();
        this.PERMANENT_GAME_NOTIFICATION = String::from("Permanent Game Notification: ");
        this.lastString = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.string = this.EMPTY_STRING.clone();
        this.point = crate::org::allbinary::graphics::CustomGPoint::CustomGPoint_NULL_CUSTOM_POINT.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this.circularIndexUtil = CircularIndexUtil::createInstance(0);
        let mut gameNotificationEventHandler: Option<Rc<RefCell<GameNotificationEventHandler>>> = GameNotificationEventHandler::getInstance();
        jcall!(gameNotificationEventHandler).removeAllListeners();
        this
    }

    pub fn new(mut location: i32, mut direction: i32, mut bufferZone: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(location, direction, bufferZone, basicColor))));
            jcall!(gameNotificationEventHandler).addListenerInterface((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.updateMeasurement(graphics.clone());
            let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
            self.width = jcall!(font).stringWidth(self.string.clone());
            jcall!(self.processor).process();
            self.processor = Processor::getInstance();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).UPDATE.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getPoint(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        self.point = CustomGPoint::getInstance3(0, 0);
        jcall!(self.point).setX(x.clone());
        jcall!(self.point).setY(y.clone());
        return self.point.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>);
    }
    

    pub fn add(&mut self, mut string: String, mut seconds: Option<i32>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut permanent: Option<bool>) {
        if permanent.unwrap() {
            if self.lastString != string {
                self.lastString = string.clone();
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(self.PERMANENT_GAME_NOTIFICATION.clone()).append(string.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).ADD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            jcall!(self.permanentGameNotification).add(string.clone(), seconds.clone(), basicColor.clone());
            { let java_receiver = self.circularIndexUtil.clone(); let java_arguments = (jcall!(self.permanentGameNotification).getSize(),); jcall!(java_receiver).setSize(java_arguments.0); };
        } else {
            jcall!(self.gameNotification).add(string.clone(), seconds.clone(), basicColor.clone());
        }
    }
    

    // throws Exception
    pub fn processTick(&mut self) {
        if jcall!(self.timeDelayHelper).isTime(jcall!(self.gameTickTimeDelayHelper).startTime.clone()) {
            let mut gameAdState: Option<Rc<RefCell<GameAdState>>> = jcall!(GameAdStateFactory::getInstance()).getCurrentInstance();
            if jcall!(gameAdState).isShowingAtLocation(self.getLocation()) {
                self.offsetY = -54;
            } else {
                self.offsetY = 0;
            }
            if jcall!(self.gameNotification).getSize() > 0 {
                self.setAndRemove();
            } else if jcall!(self.permanentGameNotification).getSize() > 0 {
                self.setNextUnremoveable();
            } else {
                self.string = self.EMPTY_STRING.clone();
            }
        }
    }
    

    // throws Exception
    pub fn setAndRemove(&mut self) {
        self.string = (jcall!(jcall!(self.gameNotification).stringList).removeAt(0)).clone() /* as String */;
        self.processor = self.setAndRemoveProcessor.clone();
    }
    

    // throws Exception
    pub fn setAndRemoveProcess(&mut self) {
        { let java_arguments = ((jcall!(self.displayInfo).getLastWidth() - self.width) >> 1,); self.setX(java_arguments.0) };
        { let java_receiver = self.point.clone(); let java_arguments = (self.getX(),); jcall!(java_receiver).setX(java_arguments.0); };
        { let java_receiver = self.point.clone(); let java_arguments = (self.getY(),); jcall!(java_receiver).setY(java_arguments.0); };
        let mut time: Option<i32> = { let java_object = jcall!(jcall!(self.gameNotification).timeList).removeAt(0); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
        let mut iTime: i32 = ((time).unwrap() as i32) * 1000;
        if iTime == 0 {
            iTime = 500;
        }
        jcall!(self.timeDelayHelper).as_TimeDelayHelper_mut().delay = iTime.clone();
        { let java_arguments = ({ let java_object = (jcall!(jcall!(self.gameNotification).colorList).removeAt(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<BasicColor>>>>().expect("ClassCastException").clone()) },); self.setBasicColorP(java_arguments.0) };
    }
    

    // throws Exception
    pub fn setNextUnremoveable(&mut self) {
        let mut index: i32 = jcall!(self.circularIndexUtil).getIndex();
        self.string = (jcall!(jcall!(self.permanentGameNotification).stringList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone() /* as String */;
        self.processor = self.nextUnremoveableProcessor.clone();
    }
    

    // throws Exception
    pub fn setNextUnremoveableProcess(&mut self) {
        let mut index: i32 = jcall!(self.circularIndexUtil).getIndex();
        { let java_arguments = ((jcall!(self.displayInfo).getLastWidth() - self.width) >> 1,); self.setX(java_arguments.0) };
        { let java_receiver = self.point.clone(); let java_arguments = (self.getX(),); jcall!(java_receiver).setX(java_arguments.0); };
        { let java_receiver = self.point.clone(); let java_arguments = (self.getY(),); jcall!(java_receiver).setY(java_arguments.0); };
        let mut time: Option<i32> = { let java_object = jcall!(jcall!(self.permanentGameNotification).timeList).as_BasicArrayList_mut().objectArray[(index) as usize]; java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
        jcall!(self.timeDelayHelper).as_TimeDelayHelper_mut().delay = ((time).unwrap() as i32) * 1000;
        self.setBasicColorP({ let java_object = (jcall!(jcall!(self.permanentGameNotification).colorList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<BasicColor>>>>().expect("ClassCastException").clone()) });
        jcall!(self.circularIndexUtil).next();
    }
    

    pub fn clear(&mut self) {
        jcall!(self.gameNotification).clear();
        jcall!(self.permanentGameNotification).clear();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        self.base.paintString(graphics.clone(), self.string.clone());
    }
}
impl std::ops::Deref for PlayerGameNotificationHud {
    type Target = GameNotificationHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PlayerGameNotificationHud {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait PlayerGameNotificationHudDyn: crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHudDyn {
    fn as_PlayerGameNotificationHud(&self) -> &PlayerGameNotificationHud;
    fn as_PlayerGameNotificationHud_mut(&mut self) -> &mut PlayerGameNotificationHud;
    fn setAndRemove(&mut self) { PlayerGameNotificationHud::setAndRemove(self.as_PlayerGameNotificationHud_mut()) }
    fn setAndRemoveProcess(&mut self) { PlayerGameNotificationHud::setAndRemoveProcess(self.as_PlayerGameNotificationHud_mut()) }
    fn setNextUnremoveable(&mut self) { PlayerGameNotificationHud::setNextUnremoveable(self.as_PlayerGameNotificationHud_mut()) }
    fn setNextUnremoveableProcess(&mut self) { PlayerGameNotificationHud::setNextUnremoveableProcess(self.as_PlayerGameNotificationHud_mut()) }
}
impl PlayerGameNotificationHudDyn for PlayerGameNotificationHud {
    fn as_PlayerGameNotificationHud(&self) -> &PlayerGameNotificationHud { self }
    fn as_PlayerGameNotificationHud_mut(&mut self) -> &mut PlayerGameNotificationHud { self }
}

impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for PlayerGameNotificationHud {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self.base.base }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self.base.base }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHud::onEvent(crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHudDyn::as_GameNotificationHud_mut(self), p1) }
    fn getPoint(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> { Self::getPoint(self, x, y) }
}
impl crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHudDyn for PlayerGameNotificationHud {
    fn as_GameNotificationHud(&self) -> &crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHud { &self.base }
    fn as_GameNotificationHud_mut(&mut self) -> &mut crate::org::allbinary::game::ag::layer::hud::basic::notification::GameNotificationHud::GameNotificationHud { &mut self.base }
    fn add(&mut self, mut string: String, mut seconds: Option<i32>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut permanent: Option<bool>) { Self::add(self, string, seconds, basicColor, permanent) }
    fn processTick(&mut self) { Self::processTick(self) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn clear(&mut self) { Self::clear(self) }
}


pub struct SetAndRemoveProcessor {
    pub base: Processor,
    pub updateMeasurementProcessor: Option<Rc<RefCell<dyn PlayerGameNotificationHudDyn>>>,
}


impl SetAndRemoveProcessor {
        

        pub fn new_struct(mut updateMeasurementProcessor: Option<Rc<RefCell<dyn PlayerGameNotificationHudDyn>>>) -> Self {
            let mut this = Self {
            base: Processor::new_struct(),
            updateMeasurementProcessor: None,
            };
            this.updateMeasurementProcessor = updateMeasurementProcessor.clone();
            this
        }

        pub fn new(mut updateMeasurementProcessor: Option<Rc<RefCell<dyn PlayerGameNotificationHudDyn>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(updateMeasurementProcessor))))
        }
        

        // throws Exception
        pub fn process(&mut self) {
            jcall!(self.updateMeasurementProcessor).setAndRemoveProcess();
        }
}
impl std::ops::Deref for SetAndRemoveProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SetAndRemoveProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for SetAndRemoveProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self }
    fn process(&mut self) { Self::process(self) }
}


pub struct NextUnremoveableProcessor {
    pub base: Processor,
    pub updateMeasurementProcessor: Option<Rc<RefCell<dyn PlayerGameNotificationHudDyn>>>,
}


impl NextUnremoveableProcessor {
        

        pub fn new_struct(mut updateMeasurementProcessor: Option<Rc<RefCell<dyn PlayerGameNotificationHudDyn>>>) -> Self {
            let mut this = Self {
            base: Processor::new_struct(),
            updateMeasurementProcessor: None,
            };
            this.updateMeasurementProcessor = updateMeasurementProcessor.clone();
            this
        }

        pub fn new(mut updateMeasurementProcessor: Option<Rc<RefCell<dyn PlayerGameNotificationHudDyn>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(updateMeasurementProcessor))))
        }
        

        // throws Exception
        pub fn process(&mut self) {
            jcall!(self.updateMeasurementProcessor).setNextUnremoveableProcess();
        }
}
impl std::ops::Deref for NextUnremoveableProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NextUnremoveableProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for NextUnremoveableProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self }
    fn process(&mut self) { Self::process(self) }
}


