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
// package org.allbinary.game.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::event::GameFeatureEvent::GameFeatureEvent;
use crate::org::allbinary::game::configuration::event::GameFeatureEventHandler::GameFeatureEventHandler;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameConfiguration {
    pub name: String,
    pub defaultValue: Option<i32>,
    pub value: Option<i32>,
    pub minValue: Option<i32>,
    pub maxValue: Option<i32>,
    pub modifiable: Option<bool>,
}

thread_local! { pub static GameConfiguration_MIN_VALUE: Option<i32> = Some((i32::MIN.clone()) as i32); }

impl GameConfiguration {
    

    pub fn new_struct(mut name: String, mut defaultValue: Option<i32>, mut minValue: Option<i32>, mut maxValue: Option<i32>) -> Self {
        let mut this = Self {
            name: String::new(),
            defaultValue: None,
            value: None,
            minValue: None,
            maxValue: None,
            modifiable: None,
        };
        this.name = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.defaultValue = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(0);
        this.value = GameConfiguration_MIN_VALUE.with(Clone::clone).clone();
        this.minValue = this.defaultValue.clone();
        this.maxValue = this.defaultValue.clone();
        this.modifiable = jcall!(BooleanFactory::getInstance()).TRUE.clone();
        this.setName(name.clone());
        this.setDefaultValue(defaultValue.clone());
        this.value = this.getDefaultValue();
        this.setMinValue(minValue.clone());
        this.setMaxValue(maxValue.clone());
        this
    }

    pub fn new(mut name: String, mut defaultValue: Option<i32>, mut minValue: Option<i32>, mut maxValue: Option<i32>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(name, defaultValue, minValue, maxValue))));
            jcall!(jcall!(GameConfigurationSingleton::GameConfigurationSingleton::getInstance()).getHashtable()).put(this.getName(), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    // throws Exception
    pub fn setValue(&mut self, mut value: Option<i32>) {
        if !match (&self.value, &value) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut oldValue: Option<i32> = self.value.clone();
            self.value = value.clone();
            jcall!(GameFeatureEventHandler::getInstance()).fireEvent(GameFeatureEvent::new(None /* Java this: a &mut self method has no Rc of itself */, jcall!(StringMaker::new()).append(self.name.clone()).append(String::from(" value: from: ")).appendint(((oldValue).unwrap() as i32)).append(String::from(" to ")).appendint(((self.value).unwrap() as i32)).toString()));
        }
    }
    

    pub fn getValue(&mut self) -> Option<i32> {
        return self.value.clone();
    }
    

    pub fn setMaxValue(&mut self, mut maxValue: Option<i32>) {
        self.maxValue = maxValue.clone();
    }
    

    pub fn getMaxValue(&mut self) -> Option<i32> {
        return self.maxValue.clone();
    }
    

    pub fn setMinValue(&mut self, mut minValue: Option<i32>) {
        self.minValue = minValue.clone();
    }
    

    pub fn getMinValue(&mut self) -> Option<i32> {
        return self.minValue.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setDefaultValue(&mut self, mut defaultValue: Option<i32>) {
        self.defaultValue = defaultValue.clone();
    }
    

    pub fn getDefaultValue(&mut self) -> Option<i32> {
        return self.defaultValue.clone();
    }
    

    // throws Exception
    pub fn setDefault(&mut self) {
        { let java_arguments = (self.getDefaultValue(),); self.setValue(java_arguments.0) };
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Name: "));
        jcall!(stringBuffer).append(self.name.clone());
        jcall!(stringBuffer).append(String::from(" Min: "));
        jcall!(stringBuffer).appendint(((self.getMinValue()).unwrap() as i32));
        jcall!(stringBuffer).append(String::from(" Max: "));
        jcall!(stringBuffer).appendint(((self.getMaxValue()).unwrap() as i32));
        jcall!(stringBuffer).append(String::from(" Value: "));
        jcall!(stringBuffer).appendint(((self.getValue()).unwrap() as i32));
        jcall!(stringBuffer).append(String::from(" Default: "));
        jcall!(stringBuffer).appendint(((self.getDefaultValue()).unwrap() as i32));
        jcall!(stringBuffer).append(String::from(" Modifiable: "));
        jcall!(stringBuffer).appendboolean(self.isModifiable().unwrap());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn setModifiable(&mut self, mut modifiable: Option<bool>) {
        self.modifiable = modifiable.clone();
    }
    

    pub fn isModifiable(&mut self) -> Option<bool> {
        return self.modifiable.clone();
    }
}
pub trait GameConfigurationDyn {
    fn as_GameConfiguration(&self) -> &GameConfiguration;
    fn as_GameConfiguration_mut(&mut self) -> &mut GameConfiguration;
    fn setValue(&mut self, mut value: Option<i32>) { GameConfiguration::setValue(self.as_GameConfiguration_mut(), value) }
    fn getValue(&mut self) -> Option<i32> { GameConfiguration::getValue(self.as_GameConfiguration_mut()) }
    fn setMaxValue(&mut self, mut maxValue: Option<i32>) { GameConfiguration::setMaxValue(self.as_GameConfiguration_mut(), maxValue) }
    fn getMaxValue(&mut self) -> Option<i32> { GameConfiguration::getMaxValue(self.as_GameConfiguration_mut()) }
    fn setMinValue(&mut self, mut minValue: Option<i32>) { GameConfiguration::setMinValue(self.as_GameConfiguration_mut(), minValue) }
    fn getMinValue(&mut self) -> Option<i32> { GameConfiguration::getMinValue(self.as_GameConfiguration_mut()) }
    fn setName(&mut self, mut name: String) { GameConfiguration::setName(self.as_GameConfiguration_mut(), name) }
    fn getName(&mut self) -> String { GameConfiguration::getName(self.as_GameConfiguration_mut()) }
    fn setDefaultValue(&mut self, mut defaultValue: Option<i32>) { GameConfiguration::setDefaultValue(self.as_GameConfiguration_mut(), defaultValue) }
    fn getDefaultValue(&mut self) -> Option<i32> { GameConfiguration::getDefaultValue(self.as_GameConfiguration_mut()) }
    fn setDefault(&mut self) { GameConfiguration::setDefault(self.as_GameConfiguration_mut()) }
    fn toString(&mut self) -> String { GameConfiguration::toString(self.as_GameConfiguration_mut()) }
    fn setModifiable(&mut self, mut modifiable: Option<bool>) { GameConfiguration::setModifiable(self.as_GameConfiguration_mut(), modifiable) }
    fn isModifiable(&mut self) -> Option<bool> { GameConfiguration::isModifiable(self.as_GameConfiguration_mut()) }
}
impl GameConfigurationDyn for GameConfiguration {
    fn as_GameConfiguration(&self) -> &GameConfiguration { self }
    fn as_GameConfiguration_mut(&mut self) -> &mut GameConfiguration { self }
}


