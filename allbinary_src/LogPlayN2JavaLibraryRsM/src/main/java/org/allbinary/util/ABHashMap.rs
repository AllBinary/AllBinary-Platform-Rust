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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Map::Map;
// use crate::java::util::Set::Set;
use crate::jsinterop::annotations::JsType;
use super::Entry::Entry;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABHashMap<K, V> {
    pub base: HashMap<K, V>,
}


impl<K, V> ABHashMap<K, V> {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: HashMap::new(),
        };
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
    

    pub fn get(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> V {
        return self.base.get(key.clone());
    }
    

    pub fn containsKey(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return self.base.containsKey(key.clone());
    }
    

    pub fn put(&mut self, mut key: K, mut value: V) -> V {
        return self.base.put(key.clone(), value.clone());
    }
    

    pub fn putAll(&mut self, mut map: Option<Rc<RefCell<dyn Map<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) {
        self.base.putAll(map.clone());
    }
    

    pub fn remove(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> V {
        return self.base.remove(key.clone());
    }
    

    pub fn clear(&mut self) {
        self.base.clear();
    }
    

    pub fn containsValue(&mut self, mut value: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return self.base.containsValue(value.clone());
    }
    

    pub fn keySet(&mut self) -> Option<Rc<RefCell<dyn Set<K>>>> {
        return self.base.keySet();
    }
    

    pub fn entrySet(&mut self) -> Option<Rc<RefCell<dyn Set<Option<Rc<RefCell<Entry<K, V>>>>>>>> {
        return self.base.entrySet();
    }
    

    pub fn getOrDefault(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut defaultValue: V) -> V {
        return self.base.getOrDefault(key.clone(), defaultValue.clone());
    }
    

    pub fn putIfAbsent(&mut self, mut key: K, mut value: V) -> V {
        return self.base.putIfAbsent(key.clone(), value.clone());
    }
    

    pub fn clone(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.base.clone();
    }
}
impl<K, V> std::ops::Deref for ABHashMap<K, V> {
    type Target = HashMap<K, V>;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl<K, V> std::ops::DerefMut for ABHashMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}


