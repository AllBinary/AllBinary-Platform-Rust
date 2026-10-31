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
// package org.allbinary.game.rand
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::util::Random::Random;
use crate::jsinterop::annotations::JsType;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use crate::org::allbinary::logic::math::MathUtil::MathUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// java.util.Random's seeded 48-bit linear congruential generator.
pub struct Random { seed: u64 }
impl Random {
    pub fn new(seed: i64) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self { seed: (seed as u64 ^ 0x5DEECE66D) & ((1u64 << 48) - 1) })))
    }
    pub fn nextInt(&mut self) -> i32 {
        self.seed = self.seed.wrapping_mul(0x5DEECE66D).wrapping_add(11) & ((1u64 << 48) - 1);
        (self.seed >> 16) as u32 as i32
    }
}

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Do not use in client code for money related things.
pub struct MyRandomFactory {
    pub mathUtil: Option<Rc<RefCell<MathUtil>>>,
    pub rand: Option<Rc<RefCell<Random>>>,
}

thread_local! { pub static MyRandomFactory_instance: Option<Rc<RefCell<MyRandomFactory>>> = MyRandomFactory::new(); }

impl MyRandomFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<MyRandomFactory>>> {
        return MyRandomFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            mathUtil: None,
            rand: None,
        };
        this.mathUtil = MathUtil::getInstance();
        this.rand = Random::new((std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setSeed(&mut self, mut seed: i64) {
        self.rand = Random::new(seed.clone());
    }
    

    pub fn getNextInt(&mut self, mut range: i32) -> i32 {
        let mut div: i32 = (i32::MAX / range) + 1;
        return (((jcall!(self.rand).nextInt() / div)) as i32);
    }
    

    pub fn getAbsoluteNextInt(&mut self, mut range: i32) -> i32 {
        return { let java_receiver = self.mathUtil.clone(); let java_arguments = (self.getNextInt(range.clone()),); let java_result = jcall!(java_receiver).abs(java_arguments.0); java_result };
    }
    

    pub fn getAbsoluteNextIntAllowZero(&mut self, mut range: i32) -> i32 {
        if range == 0 {
            return 0;
        } else {
            return { let java_receiver = self.mathUtil.clone(); let java_arguments = (self.getNextInt(range.clone()),); let java_result = jcall!(java_receiver).abs(java_arguments.0); java_result };
        }
    }
    

    pub fn shuffle(&mut self, mut intArray: Vec<i32>) {
        self.shuffleTotal(intArray.clone(), (intArray.len() as i32) * 7);
    }
    

    pub fn shuffleTotal(&mut self, mut intArray: Vec<i32>, mut shuffleTotal: i32) {
        let mut size: i32 = (intArray.len() as i32).clone();
        let mut randomIndex: i32 = 0;
        let mut randomIndex2: i32 = 0;
        let mut value: i32 = 0;
        for index in (0..shuffleTotal) {
            randomIndex = self.getAbsoluteNextIntAllowZero(size.clone());
            randomIndex2 = self.getAbsoluteNextIntAllowZero(size.clone());
            value = intArray[(randomIndex) as usize].clone();
            intArray[(randomIndex) as usize] = intArray[(randomIndex2) as usize].clone();
            intArray[(randomIndex2) as usize] = value.clone();
        }
    }
    

    pub fn shuffle2(&mut self, mut intArray: Vec<i32>, mut intArray2: Vec<i32>) {
        self.shuffle2Total(intArray.clone(), intArray2.clone(), (intArray.len() as i32) * 7);
    }
    

    pub fn shuffle2Total(&mut self, mut intArray: Vec<i32>, mut intArray2: Vec<i32>, mut shuffleTotal: i32) {
        let mut size: i32 = (intArray.len() as i32).clone();
        let mut randomIndex: i32 = 0;
        let mut randomIndex2: i32 = 0;
        let mut value: i32 = 0;
        let mut value2: i32 = 0;
        for index in (0..shuffleTotal) {
            randomIndex = self.getAbsoluteNextIntAllowZero(size.clone());
            randomIndex2 = self.getAbsoluteNextIntAllowZero(size.clone());
            value = intArray[(randomIndex) as usize].clone();
            value2 = intArray2[(randomIndex) as usize].clone();
            intArray[(randomIndex) as usize] = intArray[(randomIndex2) as usize].clone();
            intArray[(randomIndex2) as usize] = value.clone();
            intArray2[(randomIndex) as usize] = intArray2[(randomIndex2) as usize].clone();
            intArray2[(randomIndex2) as usize] = value2.clone();
        }
    }
    

    pub fn main(mut args: Vec<String>) {
        let mut randomFactory: Option<Rc<RefCell<MyRandomFactory>>> = MyRandomFactory::getInstance();
        let mut stringBuilder: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut size2: i32 = 100;
        for index in (0..size2) {
            jcall!(stringBuilder).appendint(jcall!(randomFactory).getAbsoluteNextInt(3)).append(jcall!(commonSeps).COMMA.clone());
        }
        println!("{}", jcall!(stringBuilder).toString());
        jcall!(stringBuilder).delete(0, jcall!(stringBuilder).length());
        let mut intArray: Vec<i32> = vec![0; (52) as usize];
        let mut intArray2: Vec<i32> = vec![0; (52) as usize];
        let mut size: i32 = (intArray.len() as i32).clone();
        for index in (0..size) {
            intArray[(index) as usize] = index.clone();
            intArray2[(index) as usize] = index.clone();
        }
        jcall!(randomFactory).shuffle2(intArray.clone(), intArray2.clone());
        for index in (0..size) {
            jcall!(stringBuilder).appendint(intArray[(index) as usize].clone()).append(jcall!(commonSeps).COMMA.clone());
        }
        println!("{}", jcall!(stringBuilder).toString());
        jcall!(stringBuilder).delete(0, jcall!(stringBuilder).length());
        for index in (0..size) {
            jcall!(stringBuilder).appendint(intArray2[(index) as usize].clone()).append(jcall!(commonSeps).COMMA.clone());
        }
        println!("{}", jcall!(stringBuilder).toString());
        let mut found: bool = false;
        for index in (0..size) {
            found = false;
            for index2 in (0..size) {
                if intArray[(index2) as usize] == index {
                    found = true;
                }
            }
            if !found {
                panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            }
        }
    }
}

