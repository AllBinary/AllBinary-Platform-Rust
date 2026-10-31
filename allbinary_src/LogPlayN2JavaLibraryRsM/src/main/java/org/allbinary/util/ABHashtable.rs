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
use crate::Enumeration;
use crate::Hashtable;
use super::*;
// use crate::java::util::Enumeration::Enumeration;
// use crate::java::util::Hashtable::Hashtable;
// use crate::java::util::Map::Map;
// use crate::java::util::Set::Set;
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

pub struct ABHashtable<K, V> {
    pub base: Hashtable<K, V>,
}


impl<K, V> ABHashtable<K, V> {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Hashtable::new_struct(),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn size(&mut self) -> i32 {
        return self.base.size();
    }
    

    pub fn isEmpty(&mut self) -> bool {
        return self.base.isEmpty();
    }
    

    pub fn elements(&mut self) -> Option<Rc<RefCell<dyn Enumeration<V>>>> {
        return self.base.elements();
    }
    

    pub fn contains(&mut self, mut value: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return self.base.contains(value.clone());
    }
    

    pub fn containsValue(&mut self, mut value: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return self.base.containsValue(value.clone());
    }
    

    pub fn containsKey(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return self.base.containsKey(key.clone());
    }
    

    pub fn get(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> V {
        return self.base.get(key.clone());
    }
    

    pub fn clear(&mut self) {
        self.base.clear();
    }
    

    pub fn clone(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.base.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return self.base.toString();
    }
    

    pub fn equals(&mut self, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return self.base.equals(obj.clone());
    }
    

    pub fn hashCode(&mut self) -> i32 {
        return self.base.hashCode();
    }
}
impl<K, V> std::ops::Deref for ABHashtable<K, V> {
    type Target = Hashtable<K, V>;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl<K, V> std::ops::DerefMut for ABHashtable<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}


