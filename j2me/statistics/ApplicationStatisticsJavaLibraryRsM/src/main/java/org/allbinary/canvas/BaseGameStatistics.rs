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
// package org.allbinary.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::math::PrimitiveLongSingleton::PrimitiveLongSingleton;
use crate::org::allbinary::logic::math::ScaleFactorFactory::ScaleFactorFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::time::GameTickTimeDelayHelper::GameTickTimeDelayHelper;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseGameStatistics {
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub updateDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub totalRefreshes: i64,
    pub totalFrames: i64,
    pub gameTickTimeDelayHelper: Option<Rc<RefCell<GameTickTimeDelayHelper>>>,
    pub DEFAULT_SCALE_FACTOR: i32,
    pub NOT: String,
    pub EMPTY_STRING: String,
    pub STRING_ARRAY: Vec<String>,
    pub SPACE_CHAR: char,
    pub PLUS_CHAR: char,
    pub X_CHAR: char,
    pub CHAR_ARRAY: Vec<Vec<char>>,
    pub primitiveLongSingleton: Option<Rc<RefCell<PrimitiveLongSingleton>>>,
}


impl BaseGameStatistics {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            timeDelayHelper: None,
            updateDelayHelper: None,
            totalRefreshes: 0_i64,
            totalFrames: 0_i64,
            gameTickTimeDelayHelper: None,
            DEFAULT_SCALE_FACTOR: 0,
            NOT: String::new(),
            EMPTY_STRING: String::new(),
            STRING_ARRAY: Vec::new(),
            SPACE_CHAR: '\0',
            PLUS_CHAR: '\0',
            X_CHAR: '\0',
            CHAR_ARRAY: Vec::new(),
            primitiveLongSingleton: None,
        };
        this.timeDelayHelper = TimeDelayHelper::new(i32::MAX.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.updateDelayHelper = TimeDelayHelper::new(2000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.gameTickTimeDelayHelper = GameTickTimeDelayHelperFactory::getInstance();
        this.DEFAULT_SCALE_FACTOR = jcall!(ScaleFactorFactory::getInstance()).DEFAULT_SCALE_FACTOR.clone();
        this.NOT = String::from("Not enough Time to Calculate");
        this.EMPTY_STRING = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.STRING_ARRAY = vec![String::from(" Total Time: "), this.EMPTY_STRING.clone(), String::from(" Total Frames: "), this.EMPTY_STRING.clone(), String::from(" Frames/10 Sec: "), this.EMPTY_STRING.clone(), String::from(" Total Paints: "), this.EMPTY_STRING.clone(), String::from(" Paints/10 Sec: "), this.EMPTY_STRING.clone()];
        this.SPACE_CHAR = ' ';
        this.PLUS_CHAR = '+';
        this.X_CHAR = 'X';
        this.CHAR_ARRAY = vec![vec!['F', 'r', 'a', 'm', 'e', 's', '/', '1', '0', ' ', 'S', 'e', 'c', ':'], vec!['X', 'X'], vec!['P', 'a', 'i', 'n', 't', 's', '/', '1', '0', ' ', 'S', 'e', 'c', ':'], vec!['X', 'X']];
        this.primitiveLongSingleton = PrimitiveLongSingleton::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        jcall!(self.timeDelayHelper).setStartTimeTNT();
        self.totalRefreshes = 0_i64;
        self.totalFrames = 0_i64;
    }
    

    pub fn add(&mut self, mut string: String) {
    }
    

    pub fn process(&mut self) {
    }
    

    pub fn nextFrame(&mut self) {
        self.totalFrames += 1;
    }
    

    pub fn nextRefresh(&mut self) {
        self.totalRefreshes += 1;
    }
    

    pub fn getRefreshRate(&mut self) -> i16 {
        let mut elapsed: i64 = jcall!(self.timeDelayHelper).getElapsed(jcall!(self.gameTickTimeDelayHelper).startTime.clone());
        if elapsed > 1_i64 {
            let mut time: i16 = (((self.totalRefreshes / (elapsed >> self.DEFAULT_SCALE_FACTOR))) as i16);
            return time.clone();
        } else {
            return 0;
        }
    }
    

    pub fn getTimeDelayHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> {
        return self.timeDelayHelper.clone();
    }
    

    pub fn to2DCharArray(&mut self) -> Vec<Vec<char>> {
        let mut totalTime: i64 = jcall!(self.timeDelayHelper).getElapsed(jcall!(self.gameTickTimeDelayHelper).startTime.clone());
        totalTime = (totalTime >> self.DEFAULT_SCALE_FACTOR);
        if totalTime > 0_i64 && jcall!(self.updateDelayHelper).isTime(jcall!(self.gameTickTimeDelayHelper).startTime.clone()) {
            let mut framesPerSec: i32 = (((self.totalFrames / totalTime)) as i32);
            if framesPerSec < 10 {
                self.CHAR_ARRAY[(1) as usize][(0) as usize] = self.SPACE_CHAR.clone();
                self.CHAR_ARRAY[(1) as usize][(1) as usize] = jcall!(self.primitiveLongSingleton).NUMBER_CHAR_ARRAY[(framesPerSec) as usize].clone();
            } else if framesPerSec < 100 {
                let mut tens: i32 = framesPerSec / 10;
                let mut removeTens: i32 = tens * 10;
                self.CHAR_ARRAY[(1) as usize][(0) as usize] = jcall!(self.primitiveLongSingleton).NUMBER_CHAR_ARRAY[(tens) as usize].clone();
                self.CHAR_ARRAY[(1) as usize][(1) as usize] = jcall!(self.primitiveLongSingleton).NUMBER_CHAR_ARRAY[(framesPerSec - removeTens) as usize].clone();
            } else {
                self.CHAR_ARRAY[(1) as usize][(0) as usize] = self.PLUS_CHAR.clone();
                self.CHAR_ARRAY[(1) as usize][(1) as usize] = self.PLUS_CHAR.clone();
            }
            let mut refreshesPerSec: i32 = (((self.totalRefreshes / totalTime)) as i32);
            if refreshesPerSec < 10 {
                self.CHAR_ARRAY[(3) as usize][(0) as usize] = self.X_CHAR.clone();
                self.CHAR_ARRAY[(3) as usize][(1) as usize] = jcall!(self.primitiveLongSingleton).NUMBER_CHAR_ARRAY[(refreshesPerSec) as usize].clone();
            } else if refreshesPerSec < 100 {
                let mut tens: i32 = refreshesPerSec / 10;
                let mut removeTens: i32 = tens * 10;
                self.CHAR_ARRAY[(3) as usize][(0) as usize] = jcall!(self.primitiveLongSingleton).NUMBER_CHAR_ARRAY[(refreshesPerSec / 10) as usize].clone();
                self.CHAR_ARRAY[(3) as usize][(1) as usize] = jcall!(self.primitiveLongSingleton).NUMBER_CHAR_ARRAY[(refreshesPerSec - removeTens) as usize].clone();
            } else {
                self.CHAR_ARRAY[(3) as usize][(0) as usize] = self.PLUS_CHAR.clone();
                self.CHAR_ARRAY[(3) as usize][(1) as usize] = self.PLUS_CHAR.clone();
            }
        }
        return self.CHAR_ARRAY.clone();
    }
    

    pub fn toStringArray(&mut self) -> Vec<String> {
        let mut totalTime: i64 = jcall!(self.timeDelayHelper).getElapsed(jcall!(self.gameTickTimeDelayHelper).startTime.clone());
        totalTime = (totalTime / 10000_i64);
        if totalTime > 0_i64 {
            self.STRING_ARRAY[(1) as usize] = (totalTime.clone()).to_string();
            self.STRING_ARRAY[(3) as usize] = (self.totalFrames.clone()).to_string();
            self.STRING_ARRAY[(5) as usize] = (self.totalFrames / totalTime).to_string();
            self.STRING_ARRAY[(7) as usize] = (self.totalRefreshes.clone()).to_string();
            self.STRING_ARRAY[(9) as usize] = (self.totalRefreshes / totalTime).to_string();
        } else {
            let mut string: String = self.EMPTY_STRING.clone();
            self.STRING_ARRAY[(1) as usize] = string.clone();
            self.STRING_ARRAY[(3) as usize] = string.clone();
            self.STRING_ARRAY[(5) as usize] = string.clone();
            self.STRING_ARRAY[(7) as usize] = string.clone();
            self.STRING_ARRAY[(9) as usize] = string.clone();
        }
        return self.STRING_ARRAY.clone();
    }
    

    pub fn toStringAt(&mut self, mut totalTime: i64) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.STRING_ARRAY[(0) as usize].clone());
        jcall!(stringBuffer).appendlong(totalTime.clone());
        jcall!(stringBuffer).append(self.STRING_ARRAY[(2) as usize].clone());
        jcall!(stringBuffer).appendlong(self.totalFrames.clone());
        jcall!(stringBuffer).append(self.STRING_ARRAY[(4) as usize].clone());
        jcall!(stringBuffer).appendlong(self.totalFrames / totalTime);
        if self.totalRefreshes > 0_i64 {
            jcall!(stringBuffer).append(self.STRING_ARRAY[(6) as usize].clone());
            jcall!(stringBuffer).appendlong(self.totalRefreshes.clone());
            jcall!(stringBuffer).append(self.STRING_ARRAY[(8) as usize].clone());
            jcall!(stringBuffer).appendlong(self.totalRefreshes / totalTime);
            jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn toString(&mut self) -> String {
        let mut totalTime: i64 = jcall!(self.timeDelayHelper).getElapsed(jcall!(self.gameTickTimeDelayHelper).startTime.clone());
        totalTime = (totalTime / 1000_i64);
        if totalTime > 0_i64 {
            return self.toStringAt(totalTime.clone());
        } else {
            return self.NOT.clone();
        }
    }
}
pub trait BaseGameStatisticsDyn {
    fn as_BaseGameStatistics(&self) -> &BaseGameStatistics;
    fn as_BaseGameStatistics_mut(&mut self) -> &mut BaseGameStatistics;
    fn init(&mut self) { BaseGameStatistics::init(self.as_BaseGameStatistics_mut()) }
    fn add(&mut self, mut string: String) { BaseGameStatistics::add(self.as_BaseGameStatistics_mut(), string) }
    fn process(&mut self) { BaseGameStatistics::process(self.as_BaseGameStatistics_mut()) }
    fn nextFrame(&mut self) { BaseGameStatistics::nextFrame(self.as_BaseGameStatistics_mut()) }
    fn nextRefresh(&mut self) { BaseGameStatistics::nextRefresh(self.as_BaseGameStatistics_mut()) }
    fn getRefreshRate(&mut self) -> i16 { BaseGameStatistics::getRefreshRate(self.as_BaseGameStatistics_mut()) }
    fn getTimeDelayHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> { BaseGameStatistics::getTimeDelayHelper(self.as_BaseGameStatistics_mut()) }
    fn to2DCharArray(&mut self) -> Vec<Vec<char>> { BaseGameStatistics::to2DCharArray(self.as_BaseGameStatistics_mut()) }
    fn toStringArray(&mut self) -> Vec<String> { BaseGameStatistics::toStringArray(self.as_BaseGameStatistics_mut()) }
    fn toStringAt(&mut self, mut totalTime: i64) -> String { BaseGameStatistics::toStringAt(self.as_BaseGameStatistics_mut(), totalTime) }
    fn toString(&mut self) -> String { BaseGameStatistics::toString(self.as_BaseGameStatistics_mut()) }
}
impl BaseGameStatisticsDyn for BaseGameStatistics {
    fn as_BaseGameStatistics(&self) -> &BaseGameStatistics { self }
    fn as_BaseGameStatistics_mut(&mut self) -> &mut BaseGameStatistics { self }
}


