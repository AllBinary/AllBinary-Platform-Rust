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
// package org.allbinary.util
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::org::allbinary::game::rand::MyRandomFactory::MyRandomFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsProperty;
use super::BasicArrayList::BasicArrayList;
use super::ImmutableBasicArrayList::ImmutableBasicArrayList;
use super::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicArrayListUtil {
    pub NULL_ARRAY_OF_ARRAY: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>>,
    pub myRandomFactory: Option<Rc<RefCell<MyRandomFactory>>>,
    pub immutableInstance: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static BasicArrayListUtil_instance: Option<Rc<RefCell<BasicArrayListUtil>>> = BasicArrayListUtil::new(); }

impl BasicArrayListUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NULL_ARRAY_OF_ARRAY: Vec::new(),
            myRandomFactory: None,
            immutableInstance: None,
        };
        this.NULL_ARRAY_OF_ARRAY = vec![vec![None; (0) as usize]; (0) as usize];
        this.myRandomFactory = MyRandomFactory::getInstance();
        this.immutableInstance = ImmutableBasicArrayList::new(String::from("Immutable"), 0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicArrayListUtil>>> {
        return BasicArrayListUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getRandom(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut i_random: i32 = self.getRandomIndex(list.clone());
        return jcall!(list).as_BasicArrayList_mut().objectArray[(i_random) as usize].clone();
    }
    

    pub fn getRandomIndex(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> i32 {
        return { let java_receiver = self.myRandomFactory.clone(); let java_arguments = (jcall!(list).size(),); let java_result = jcall!(java_receiver).getAbsoluteNextInt(java_arguments.0); java_result };
    }
    

    pub fn reverse(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut lastMinusOneIndex: i32 = jcall!(list).size() - 2;
        let mut temp: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = None;
        let mut size: i32 = lastMinusOneIndex / 2;
        let mut secondSwapIndex: i32 = 0;
        for index in (0..=size) {
            secondSwapIndex = lastMinusOneIndex - index;
            temp = jcall!(list).removeAt(index.clone());
            jcall!(list).addAt(index.clone(), crate::java_into_object(jcall!(list).removeAt(secondSwapIndex.clone())));
            jcall!(list).addAt(secondSwapIndex + 1, crate::java_into_object(temp.clone()));
        }
    }
    

    pub fn log(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> String {
        let mut S_LABEL: String = String::from(" s: ");
        let mut size: i32 = jcall!(list).size();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).TOTAL_LABEL.clone());
        jcall!(stringBuffer).appendint(size.clone());
        for index in (0..=size - 1).rev() {
            let mut usedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(stringBuffer).append(S_LABEL.clone());
            jcall!(stringBuffer).appendint(jcall!(usedList).size());
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getImmutableInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.immutableInstance.clone();
    }
    

    pub fn main(mut args: Vec<String>) {
        let mut commonPhoneStrings: Option<Rc<RefCell<CommonPhoneStrings>>> = CommonPhoneStrings::getInstance();
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(list).add(crate::java_into_object(jcall!(commonPhoneStrings).ONE.clone()));
        jcall!(list).add(crate::java_into_object(jcall!(commonPhoneStrings).TWO.clone()));
        jcall!(list).add(crate::java_into_object(jcall!(commonPhoneStrings).THREE.clone()));
        jcall!(list).add(crate::java_into_object(jcall!(commonPhoneStrings).FOUR.clone()));
        jcall!(list).add(crate::java_into_object(jcall!(commonPhoneStrings).FIVE.clone()));
        jcall!(list).add(crate::java_into_object(jcall!(commonPhoneStrings).SIX.clone()));
        jcall!(list).add(crate::java_into_object(jcall!(commonPhoneStrings).SEVEN.clone()));
        jcall!(list).add(crate::java_into_object(jcall!(commonPhoneStrings).EIGHT.clone()));
        jcall!(list).add(crate::java_into_object(jcall!(commonPhoneStrings).NINE.clone()));
        jcall!(BasicArrayListUtil::getInstance()).reverse(list.clone());
        println!("{}", jcall!(list).toString());
    }
}

