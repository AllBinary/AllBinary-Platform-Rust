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
// package org.allbinary.game.score
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
// use crate::java::io::DataInputStream::DataInputStream;
// use crate::java::io::EOFException::EOFException;
// use crate::java::io::IOException::IOException;
// use crate::javax::microedition::rms::RecordComparator::RecordComparator;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScoreComparator {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub isHighestBest: bool,
}

// impl RecordComparator for ScoreComparator with the methods of the trait

impl ScoreComparator {
    

    pub fn new_struct(mut isHighestBest: bool) -> Self {
        let mut this = Self {
            logUtil: None,
            isHighestBest: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.isHighestBest = isHighestBest.clone();
        this
    }

    pub fn new(mut isHighestBest: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(isHighestBest))))
    }
    

    pub fn getBestScore(&mut self) -> i64 {
        return (if self.isHighestBest { i64::MAX } else { 0 });
    }
    

    pub fn compare(&mut self, mut recordOne: Vec<i8>, mut recordTwo: Vec<i8>) -> i32 {
        let mut byteArrayInputStreamOne: Option<Rc<RefCell<ByteArrayInputStream>>> = ByteArrayInputStream::new(recordOne.clone());
        let mut inputStreamOne: Option<Rc<RefCell<DataInputStream>>> = DataInputStream::new(byteArrayInputStreamOne.clone());
        let mut byteArrayInputStreamTwo: Option<Rc<RefCell<ByteArrayInputStream>>> = ByteArrayInputStream::new(recordTwo.clone());
        let mut inputStreamTwo: Option<Rc<RefCell<DataInputStream>>> = DataInputStream::new(byteArrayInputStreamTwo.clone());
        let mut scoreOne: i64 = 0_i64;
        let mut scoreTwo: i64 = 0_i64;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(inputStreamOne).readUTF();
            jcall!(inputStreamTwo).readUTF();
            scoreOne = jcall!(inputStreamOne).readLong();
            scoreTwo = jcall!(inputStreamTwo).readLong();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("compare"), crate::java_into_object(e.clone()));
            }
        }
        if self.isHighestBest {
            return self.getHighTooLow(scoreOne.clone(), scoreTwo.clone());
        } else {
            return self.getLowTooHigh(scoreOne.clone(), scoreTwo.clone());
        }
    }
    

    pub fn getHighTooLow(&mut self, mut scoreOne: i64, mut scoreTwo: i64) -> i32 {
        if scoreOne > scoreTwo {
            return unsafe { crate::javax::microedition::rms::RecordComparator::RecordComparator_PRECEDES.clone() }.clone();
        } else if scoreOne < scoreTwo {
            return unsafe { crate::javax::microedition::rms::RecordComparator::RecordComparator_FOLLOWS.clone() }.clone();
        } else {
            return unsafe { crate::javax::microedition::rms::RecordComparator::RecordComparator_EQUIVALENT.clone() }.clone();
        }
    }
    

    pub fn getLowTooHigh(&mut self, mut scoreOne: i64, mut scoreTwo: i64) -> i32 {
        if scoreOne < scoreTwo {
            return unsafe { crate::javax::microedition::rms::RecordComparator::RecordComparator_PRECEDES.clone() }.clone();
        } else if scoreOne > scoreTwo {
            return unsafe { crate::javax::microedition::rms::RecordComparator::RecordComparator_FOLLOWS.clone() }.clone();
        } else {
            return unsafe { crate::javax::microedition::rms::RecordComparator::RecordComparator_EQUIVALENT.clone() }.clone();
        }
    }
}

