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
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::java::util::Map::Map;
use crate::java::util::Set::Set;
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
            base: HashMap::new_struct(),
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
        return { let java_arguments = (key.clone(),); self.base.get(java_arguments.0) };
    }
    

    pub fn containsKey(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return { let java_arguments = (key.clone(),); self.base.containsKey(java_arguments.0) };
    }
    

    pub fn put(&mut self, mut key: K, mut value: V) -> V {
        return { let java_arguments = (crate::java_into_object(key.clone()), crate::java_into_object(value.clone()),); self.base.put(java_arguments.0, java_arguments.1) };
    }
    

    pub fn putAll(&mut self, mut map: Option<Rc<RefCell<dyn Map<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) {
        { let java_arguments = (map.clone(),); self.base.putAll(java_arguments.0) };
    }
    

    pub fn remove(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> V {
        return { let java_arguments = (key.clone(),); self.base.remove(java_arguments.0) };
    }
    

    pub fn clear(&mut self) {
        self.base.clear();
    }
    

    pub fn containsValue(&mut self, mut value: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return { let java_arguments = (value.clone(),); self.base.containsValue(java_arguments.0) };
    }
    

    pub fn keySet(&mut self) -> Option<Rc<RefCell<dyn Set<crate::JavaObject>>>> {
        return self.base.keySet();
    }
    

    pub fn entrySet(&mut self) -> Option<Rc<RefCell<dyn Set<crate::JavaObject>>>> {
        return self.base.entrySet();
    }
    

    pub fn getOrDefault(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut defaultValue: V) -> V {
        return { let java_arguments = (key.clone(), defaultValue.clone(),); self.base.getOrDefault(java_arguments.0, java_arguments.1) };
    }
    

    pub fn putIfAbsent(&mut self, mut key: K, mut value: V) -> V {
        return { let java_arguments = (key.clone(), value.clone(),); self.base.putIfAbsent(java_arguments.0, java_arguments.1) };
    }
    

    pub fn clone(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return crate::java_into_object(self.base.clone());
    }
}
impl<K, V> std::ops::Deref for ABHashMap<K, V> {
    type Target = HashMap<K, V>;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl<K, V> std::ops::DerefMut for ABHashMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}


