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
// package org.allbinary.logic.math.permutations
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::util::Arrays::Arrays;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;
use super::ComparableObject::ComparableObject;
use super::ObjectPermutationVisitor::ObjectPermutationVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

fn java_string_hash(text: &str) -> i32 {
    text.chars().fold(0i32, |hash, character| hash.wrapping_mul(31).wrapping_add(character as i32))
}

pub struct OrderedObjectArrayPermutationUtil {
    pub TOTAL_ITERATIONS_TABLE: String,
    pub FINISHED: String,
}

thread_local! { pub static OrderedObjectArrayPermutationUtil_instance: Option<Rc<RefCell<OrderedObjectArrayPermutationUtil>>> = OrderedObjectArrayPermutationUtil::new(); }

impl OrderedObjectArrayPermutationUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<OrderedObjectArrayPermutationUtil>>> {
        return OrderedObjectArrayPermutationUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            TOTAL_ITERATIONS_TABLE: String::new(),
            FINISHED: String::new(),
        };
        this.TOTAL_ITERATIONS_TABLE = String::from("Total Iterations: ");
        this.FINISHED = String::from("\nFINISHED");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getNext(&mut self, mut data: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) {
        let mut length: i32 = (data.len() as i32).clone();
        let mut i: i32 = length - 1;
        let mut tsUtil: Option<Rc<RefCell<TsUtil>>> = TsUtil::getInstance();
        while jcall!(tsUtil).hashCode(crate::java_into_object(data[(i - 1) as usize].clone())) >= jcall!(tsUtil).hashCode(crate::java_into_object(data[(i) as usize].clone())) {
            i = i - 1;
        }
        let mut j: i32 = length.clone();
        while jcall!(tsUtil).hashCode(crate::java_into_object(data[(j - 1) as usize].clone())) <= jcall!(tsUtil).hashCode(crate::java_into_object(data[(i - 1) as usize].clone())) {
            j = j - 1;
        }
        self.swap(data.clone(), i - 1, j - 1);
        i += 1;
        j = length.clone();
        while i < j {
            self.swap(data.clone(), i - 1, j - 1);
            i += 1;
            j -= 1;
        }
    }
    

    pub fn swap(&mut self, mut data: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>, mut a: i32, mut b: i32) {
        let mut temp: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = data[(a) as usize].clone();
        data[(a) as usize] = data[(b) as usize].clone();
        data[(b) as usize] = temp.clone();
    }
    

    pub fn factorial(&mut self, mut dataLength: i32) -> i64 {
        let mut temp: i64 = 1_i64;
        if dataLength > 1 {
            for i in (1..=dataLength) {
                temp *= ((i) as i64).clone();
            }
        }
        return temp.clone();
    }
    

    pub fn generate(&mut self, mut data: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>, mut visitor: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>) {
        let mut iterations: i64 = self.factorial((data.len() as i32).clone());
        print!("{}", self.TOTAL_ITERATIONS_TABLE.clone());
        println!("{}", iterations.clone());
        for count in (0..iterations - 1_i64) {
            self.getNext(data.clone());
            jcall!(visitor).visit(None /* Java this: a &mut self method has no Rc of itself */);
        }
        println!("{}", self.FINISHED.clone());
    }
    

    pub fn main(mut args: Vec<String>) {
        let mut TOTAL: i32 = 7;
        let mut data: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (TOTAL) as usize];
        for i in (0..TOTAL) {
            data[(i) as usize] = crate::java_into_object(ComparableObject::new());
        }
        Arrays::sort(data.clone());
        let mut permutationVisitor: Option<Rc<RefCell<ObjectPermutationVisitor>>> = ObjectPermutationVisitor::new(data.clone());
        let mut ORIGINAL_DATA_LABEL: String = String::from("Original Data: ");
        print!("{}", ORIGINAL_DATA_LABEL.clone());
        jcall!(permutationVisitor).visit(crate::java_into_object(data.clone()));
        jcall!(OrderedObjectArrayPermutationUtil::getInstance()).generate(data.clone(), permutationVisitor.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>));
    }
}

