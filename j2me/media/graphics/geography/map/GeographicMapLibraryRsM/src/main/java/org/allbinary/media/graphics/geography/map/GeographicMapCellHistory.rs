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
// package org.allbinary.media.graphics.geography.map
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::text::TextAnimation::TextAnimation;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use super::GeographicMapCellPosition::GeographicMapCellPosition;
use super::BasicGeographicMap::BasicGeographicMap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapCellHistory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub booleanFactory: Option<Rc<RefCell<BooleanFactory>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub visitedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub MISSED_INFO: String,
    pub animation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub totalVisited: i32,
    pub halfWidth: i32,
    pub fontHeight: i32,
    pub RED: i32,
}

thread_local! { pub static GeographicMapCellHistory_NULL_GEOGRPAHIC_MAP_HISTORY_ARRAY: Vec<Option<Rc<RefCell<GeographicMapCellHistory>>>> = vec![None; (0) as usize]; }
impl UpdateMyFontInterface for GeographicMapCellHistory {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl GeographicMapCellHistory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            booleanFactory: None,
            list: None,
            visitedList: None,
            MISSED_INFO: String::new(),
            animation: None,
            myFontProcessor: None,
            totalVisited: 0,
            halfWidth: 0,
            fontHeight: 0,
            RED: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.booleanFactory = BooleanFactory::getInstance();
        this.MISSED_INFO = String::from("Missed");
        this.animation = TextAnimation::new(this.MISSED_INFO.clone(), AnimationBehavior::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        this.myFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.halfWidth = 0;
        this.fontHeight = 0;
        this.RED = jcall!(jcall!(BasicColorFactory::getInstance()).RED).intValue();
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.visitedList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.init();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.fontHeight = jcall!(font).getHeight();
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn getTotalVisited(&mut self) -> i32 {
        return self.totalVisited.clone();
    }
    

    pub fn getTotalNotVisited(&mut self) -> i32 {
        return self.getSize() - self.totalVisited;
    }
    

    pub fn getSize(&mut self) -> i32 {
        return jcall!(self.list).size();
    }
    

    pub fn trackAll(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut size: i32 = jcall!(list).size();
        jcall!(self.list).ensureCapacity(size.clone());
        jcall!(self.visitedList).ensureCapacity(size.clone());
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = None;
        for index in (0..size) {
            geographicMapCellPosition = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
            self.track(geographicMapCellPosition.clone());
        }
    }
    

    pub fn track(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        if !jcall!(self.list).contains(crate::java_into_object(geographicMapCellPosition.clone())) {
            jcall!(self.list).add(crate::java_into_object(geographicMapCellPosition.clone()));
            jcall!(self.visitedList).add(crate::java_into_object(jcall!(self.booleanFactory).FALSE.clone()));
        }
    }
    

    pub fn getTracked(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.list.clone();
    }
    

    pub fn getVisited(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.visitedList.clone();
    }
    

    pub fn getAfterIfNotLast(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut localList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.list.clone();
        let mut index: i32 = jcall!(localList).indexOf(crate::java_into_object(geographicMapCellPosition.clone()));
        if jcall!(localList).size() > index + 1 {
            return { let java_object = (jcall!(localList).get(index + 1)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
        }
        return geographicMapCellPosition.clone();
    }
    

    pub fn getFirstUnvisited(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut localList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.list.clone();
        let mut localVisitedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.visitedList.clone();
        let mut size: i32 = jcall!(localVisitedList).size();
        let mut value: Option<bool> = None;
        for index in (0..size) {
            value = { let java_object = jcall!(self.visitedList).get(index.clone()); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<bool>().copied().or_else(|| object.downcast_ref::<Option<bool>>().copied().flatten()).expect("ClassCastException") }) };
            if match (&value, &jcall!(self.booleanFactory).FALSE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return { let java_object = (jcall!(localList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
            }
        }
        return { let java_object = (jcall!(localList).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getFirstUnvisitedIndex(&mut self) -> i32 {
        let mut localVisitedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.visitedList.clone();
        let mut size: i32 = jcall!(localVisitedList).size();
        let mut value: Option<bool> = None;
        for index in (0..size) {
            value = { let java_object = jcall!(self.visitedList).get(index.clone()); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<bool>().copied().or_else(|| object.downcast_ref::<Option<bool>>().copied().flatten()).expect("ClassCastException") }) };
            if match (&value, &jcall!(self.booleanFactory).FALSE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return index.clone();
            }
        }
        return 0;
    }
    

    pub fn getInPathButNotTracked(&mut self, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut inPathButNotTrackedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut localList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.list.clone();
        let mut size: i32 = jcall!(pathList).size();
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = None;
        for index in (0..size) {
            geographicMapCellPosition = { let java_object = (jcall!(pathList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
            if !jcall!(localList).contains(crate::java_into_object(geographicMapCellPosition.clone())) {
                jcall!(inPathButNotTrackedList).add(crate::java_into_object(geographicMapCellPosition.clone()));
            }
        }
        return inPathButNotTrackedList.clone();
    }
    

    pub fn isVisited(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool {
        let mut index: i32 = jcall!(self.list).indexOf(crate::java_into_object(geographicMapCellPosition.clone()));
        if index != -1 {
            let mut value: Option<bool> = { let java_object = jcall!(self.visitedList).get(index.clone()); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<bool>().copied().or_else(|| object.downcast_ref::<Option<bool>>().copied().flatten()).expect("ClassCastException") }) };
            if match (&value, &jcall!(self.booleanFactory).TRUE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return true;
            }
        } else {
        }
        return false;
    }
    

    pub fn visit(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool {
        let mut index: i32 = jcall!(self.list).indexOf(crate::java_into_object(geographicMapCellPosition.clone()));
        let mut value: Option<bool> = None;
        if index != -1 {
            value = { let java_object = jcall!(self.visitedList).get(index.clone()); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<bool>().copied().or_else(|| object.downcast_ref::<Option<bool>>().copied().flatten()).expect("ClassCastException") }) };
            let mut TRUE: Option<bool> = jcall!(self.booleanFactory).TRUE.clone();
            if !match (&value, &TRUE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.visitedList).set(index.clone(), crate::java_into_object((TRUE.clone()).expect("NullPointerException")));
                self.totalVisited += 1;
                return true;
            }
        } else {
        }
        return false;
    }
    

    // throws Exception
    pub fn isAllVisited(&mut self) -> bool {
        if self.totalVisited == self.getSize() - 1 {
            return true;
        } else {
            return false;
        }
    }
    

    // throws Exception
    pub fn isAllVisited2(&mut self) -> bool {
        if self.totalVisited == self.getSize() {
            return true;
        } else {
            return false;
        }
    }
    

    // throws Exception
    pub fn isPortionVisited(&mut self, mut basicDecimal: Option<Rc<RefCell<BasicDecimal>>>) -> bool {
        let mut size: i32 = self.getSize();
        let mut numberRequired: i32 = (size << jcall!(basicDecimal).getScaledFactor()) / ((jcall!(basicDecimal).getUnscaled()) as i32);
        let mut numberNotVisited: i32 = self.getSize() - 1 - self.totalVisited;
        if size - numberNotVisited > numberRequired {
            return true;
        } else {
            return false;
        }
    }
    

    // throws Exception
    pub fn reset(&mut self) {
        let mut localVisitedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.visitedList.clone();
        let mut localFalseBoolean: Option<bool> = jcall!(self.booleanFactory).FALSE.clone();
        let mut size: i32 = jcall!(localVisitedList).size();
        for index in (0..=size - 1).rev() {
            jcall!(localVisitedList).set(index.clone(), crate::java_into_object((localFalseBoolean.clone()).expect("NullPointerException")));
        }
        self.totalVisited = 0;
    }
    

    pub fn paintNotVisitedRelativeToPoint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        let mut x: i32 = jcall!(point).getX() - jcall!(tiledLayer).getXP();
        let mut y: i32 = jcall!(point).getY() - jcall!(tiledLayer).getYP();
        if self.halfWidth == 0 {
            self.halfWidth = (jcall!(jcall!(graphics).getFont()).stringWidth(self.MISSED_INFO.clone()) >> 1);
        }
        let mut height: i32 = 2 * self.fontHeight;
        jcall!(self.animation).paintXY(graphics.clone(), x + self.halfWidth, y + (height));
    }
    

    pub fn paintNotVisited(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.myFontProcessor).process(graphics.clone());
            jcall!(graphics).setColor(self.RED.clone());
            let mut localVisitedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.visitedList.clone();
            let mut size: i32 = jcall!(localVisitedList).size();
            let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = None;
            let mut isCellVisitedBoolean: Option<bool> = None;
            {
                let mut index: i32 = size.clone();
                while { index -= 1; index } >= 0 {
                geographicMapCellPosition = { let java_object = (jcall!(self.list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
                isCellVisitedBoolean = { let java_object = jcall!(localVisitedList).get(index.clone()); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<bool>().copied().or_else(|| object.downcast_ref::<Option<bool>>().copied().flatten()).expect("ClassCastException") }) };
                if !isCellVisitedBoolean.unwrap() {
                    { let java_arguments = (graphics.clone(), jcall!(geographicMapInterface).getAllBinaryTiledLayer(), jcall!(geographicMapCellPosition).getPoint(),); self.paintNotVisitedRelativeToPoint(java_arguments.0, java_arguments.1, java_arguments.2) };
                }
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("paintNotVisited"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn init(&mut self) {
        jcall!(self.list).clear();
        jcall!(self.visitedList).clear();
        self.totalVisited = 0;
    }
}

