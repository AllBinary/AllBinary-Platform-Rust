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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::TouchButtonInput::TouchButtonInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CurrentlyPressedTouchButtonSingleton {
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static CurrentlyPressedTouchButtonSingleton_instance: Option<Rc<RefCell<dyn CurrentlyPressedTouchButtonSingletonDyn>>> = CurrentlyPressedTouchButtonSingleton::new().map(|reference| reference as Rc<RefCell<dyn CurrentlyPressedTouchButtonSingletonDyn>>); }

impl CurrentlyPressedTouchButtonSingleton {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn CurrentlyPressedTouchButtonSingletonDyn>>> {
        return CurrentlyPressedTouchButtonSingleton_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            list: None,
        };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn size(&mut self) -> i32 {
        return jcall!(self.list).size();
    }
    

    pub fn removeAt(&mut self, mut index: i32) -> Option<Rc<RefCell<TouchButtonInput>>> {
        return { let java_object = (jcall!(self.list).removeAt(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<TouchButtonInput>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn remove(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) -> bool {
        return jcall!(self.list).remove(crate::java_into_object(touchButtonInput.clone()));
    }
    

    pub fn get(&mut self, mut index: i32) -> Option<Rc<RefCell<TouchButtonInput>>> {
        return { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<TouchButtonInput>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn contains(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) -> bool {
        return jcall!(self.list).contains(crate::java_into_object(touchButtonInput.clone()));
    }
    

    pub fn add(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) {
        jcall!(self.list).add(crate::java_into_object(touchButtonInput.clone()));
    }
}
pub trait CurrentlyPressedTouchButtonSingletonDyn {
    fn as_CurrentlyPressedTouchButtonSingleton(&self) -> &CurrentlyPressedTouchButtonSingleton;
    fn as_CurrentlyPressedTouchButtonSingleton_mut(&mut self) -> &mut CurrentlyPressedTouchButtonSingleton;
    fn size(&mut self) -> i32 { CurrentlyPressedTouchButtonSingleton::size(self.as_CurrentlyPressedTouchButtonSingleton_mut()) }
    fn removeAt(&mut self, mut index: i32) -> Option<Rc<RefCell<TouchButtonInput>>> { CurrentlyPressedTouchButtonSingleton::removeAt(self.as_CurrentlyPressedTouchButtonSingleton_mut(), index) }
    fn remove(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) -> bool { CurrentlyPressedTouchButtonSingleton::remove(self.as_CurrentlyPressedTouchButtonSingleton_mut(), touchButtonInput) }
    fn get(&mut self, mut index: i32) -> Option<Rc<RefCell<TouchButtonInput>>> { CurrentlyPressedTouchButtonSingleton::get(self.as_CurrentlyPressedTouchButtonSingleton_mut(), index) }
    fn contains(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) -> bool { CurrentlyPressedTouchButtonSingleton::contains(self.as_CurrentlyPressedTouchButtonSingleton_mut(), touchButtonInput) }
    fn add(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) { CurrentlyPressedTouchButtonSingleton::add(self.as_CurrentlyPressedTouchButtonSingleton_mut(), touchButtonInput) }
}
impl CurrentlyPressedTouchButtonSingletonDyn for CurrentlyPressedTouchButtonSingleton {
    fn as_CurrentlyPressedTouchButtonSingleton(&self) -> &CurrentlyPressedTouchButtonSingleton { self }
    fn as_CurrentlyPressedTouchButtonSingleton_mut(&mut self) -> &mut CurrentlyPressedTouchButtonSingleton { self }
}


