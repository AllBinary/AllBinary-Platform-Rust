// 
// AllBinary Open License Version 1 
// Copyright (c) 2026 AllBinary 
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
use crate::Vector;
use super::*;
// use crate::java::util::Vector::Vector;
use crate::jsinterop::annotations::JsType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

fn java_string_hash(text: &str) -> i32 {
    text.chars().fold(0i32, |hash, character| hash.wrapping_mul(31).wrapping_add(character as i32))
}

pub struct ABVector<E> {
    pub base: Vector<E>,
}


impl<E> ABVector<E> {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Vector::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn ensureCapacity(&mut self, mut minCapacity: i32) {
        self.base.ensureCapacity(minCapacity.clone());
    }
    

    pub fn capacity(&mut self) -> i32 {
        return self.base.capacity();
    }
    

    pub fn size(&mut self) -> i32 {
        return self.base.size();
    }
    

    pub fn isEmpty(&mut self) -> bool {
        return self.base.isEmpty();
    }
    

    pub fn contains(&mut self, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return self.base.contains(obj.clone());
    }
    

    pub fn indexOf(&mut self, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 {
        return self.base.indexOf(obj.clone());
    }
    

    pub fn lastIndexOf(&mut self, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 {
        return self.base.lastIndexOf(obj.clone());
    }
    

    pub fn elementAt(&mut self, mut index: i32) -> E {
        return self.base.elementAt(index.clone());
    }
    

    pub fn firstElement(&mut self) -> E {
        return self.base.firstElement();
    }
    

    pub fn lastElement(&mut self) -> E {
        return self.base.lastElement();
    }
    

    pub fn setElementAt(&mut self, mut obj: E, mut index: i32) {
        self.base.setElementAt(obj.clone(), index.clone());
    }
    

    pub fn removeElementAt(&mut self, mut index: i32) {
        self.base.removeElementAt(index.clone());
    }
    

    pub fn addElement(&mut self, mut obj: E) {
        self.base.addElement(obj.clone());
    }
    

    pub fn insertElementAt(&mut self, mut obj: E, mut index: i32) {
        self.base.insertElementAt(obj.clone(), index.clone());
    }
    

    pub fn removeElement(&mut self, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return self.base.removeElement(obj.clone());
    }
    

    pub fn removeAllElements(&mut self) {
        self.base.removeAllElements();
    }
    

    pub fn clone(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.base.clone();
    }
    

    pub fn toArray(&mut self) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        return self.base.toArray();
    }
    

    pub fn set(&mut self, mut index: i32, mut element: E) -> E {
        return self.base.set(index.clone(), element.clone());
    }
    

    pub fn add(&mut self, mut element: E) -> bool {
        return self.base.add(element.clone());
    }
    

    pub fn remove(&mut self, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return self.base.remove(obj.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn add2(&mut self, mut index: i32, mut element: E) {
        self.base.add2(index.clone(), element.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn remove(&mut self, mut index: i32) -> E {
        return self.base.remove(index.clone());
    }
    

    pub fn clear(&mut self) {
        self.base.clear();
    }
    

    pub fn equals(&mut self, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return self.base.equals(obj.clone());
    }
    

    pub fn hashCode(&mut self) -> i32 {
        return self.base.hashCode();
    }
    

    pub fn toString(&mut self) -> String {
        return self.base.toString();
    }
    

    pub fn removeRange(&mut self, mut fromIndex: i32, mut toIndex: i32) {
        self.base.removeRange(fromIndex.clone(), toIndex.clone());
    }
}
impl<E> std::ops::Deref for ABVector<E> {
    type Target = Vector<E>;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl<E> std::ops::DerefMut for ABVector<E> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}


