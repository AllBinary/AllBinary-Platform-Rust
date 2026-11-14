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
        { let java_arguments = (minCapacity.clone(),); self.base.ensureCapacity(java_arguments.0) };
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
        return { let java_arguments = (obj.clone(),); self.base.contains(java_arguments.0) };
    }
    

    pub fn indexOf(&mut self, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 {
        return { let java_arguments = (obj.clone(),); self.base.indexOf(java_arguments.0) };
    }
    

    pub fn lastIndexOf(&mut self, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 {
        return { let java_arguments = (obj.clone(),); self.base.lastIndexOf(java_arguments.0) };
    }
    

    pub fn elementAt(&mut self, mut index: i32) -> E {
        return { let java_arguments = (index.clone(),); self.base.elementAt(java_arguments.0) };
    }
    

    pub fn firstElement(&mut self) -> E {
        return self.base.firstElement();
    }
    

    pub fn lastElement(&mut self) -> E {
        return self.base.lastElement();
    }
    

    pub fn setElementAt(&mut self, mut obj: E, mut index: i32) {
        { let java_arguments = (crate::java_into_object(obj.clone()), index.clone(),); self.base.setElementAt(java_arguments.0, java_arguments.1) };
    }
    

    pub fn removeElementAt(&mut self, mut index: i32) {
        { let java_arguments = (index.clone(),); self.base.removeElementAt(java_arguments.0) };
    }
    

    pub fn addElement(&mut self, mut obj: E) {
        { let java_arguments = (crate::java_into_object(obj.clone()),); self.base.addElement(java_arguments.0) };
    }
    

    pub fn insertElementAt(&mut self, mut obj: E, mut index: i32) {
        { let java_arguments = (crate::java_into_object(obj.clone()), index.clone(),); self.base.insertElementAt(java_arguments.0, java_arguments.1) };
    }
    

    pub fn removeElement(&mut self, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return { let java_arguments = (obj.clone(),); self.base.removeElement(java_arguments.0) };
    }
    

    pub fn removeAllElements(&mut self) {
        self.base.removeAllElements();
    }
    

    pub fn clone(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return crate::java_into_object(self.base.clone());
    }
    

    pub fn toArray(&mut self) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        return self.base.toArray();
    }
    

    pub fn set(&mut self, mut index: i32, mut element: E) -> E {
        return { let java_arguments = (index.clone(), element.clone(),); self.base.set(java_arguments.0, java_arguments.1) };
    }
    

    pub fn add(&mut self, mut element: E) -> bool {
        return { let java_arguments = (crate::java_into_object(element.clone()),); self.base.add(java_arguments.0) };
    }
    

    pub fn remove(&mut self, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return { let java_arguments = (obj.clone(),); self.base.remove(java_arguments.0) };
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn add2(&mut self, mut index: i32, mut element: E) {
        { let java_arguments = (crate::java_into_object(index.clone()), element.clone(),); self.base.add2(java_arguments.0, java_arguments.1) };
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn remove_int(&mut self, mut index: i32) -> E {
        return { let java_arguments = (crate::java_into_object(index.clone()),); self.base.remove_int(java_arguments.0) };
    }
    

    pub fn clear(&mut self) {
        self.base.clear();
    }
    

    pub fn equals(&mut self, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return { let java_arguments = (obj.clone(),); self.base.equals(java_arguments.0) };
    }
    

    pub fn hashCode(&mut self) -> i32 {
        return self.base.hashCode();
    }
    

    pub fn toString(&mut self) -> String {
        return self.base.toString();
    }
    

    pub fn removeRange(&mut self, mut fromIndex: i32, mut toIndex: i32) {
        { let java_arguments = (fromIndex.clone(), toIndex.clone(),); self.base.removeRange(java_arguments.0, java_arguments.1) };
    }
}
impl<E> std::ops::Deref for ABVector<E> {
    type Target = Vector<E>;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl<E> std::ops::DerefMut for ABVector<E> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}


