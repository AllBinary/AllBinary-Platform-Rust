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
// package org.allbinary.game.layer.pickup
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::PickedUpLayerType::PickedUpLayerType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PickedUpLayerTypeFactory {
    pub NONE: Option<Rc<RefCell<PickedUpLayerType>>>,
    pub BEAM: Option<Rc<RefCell<PickedUpLayerType>>>,
    pub MINE: Option<Rc<RefCell<PickedUpLayerType>>>,
    pub PROJECTILE: Option<Rc<RefCell<PickedUpLayerType>>>,
    pub ENHANCEMENT: Option<Rc<RefCell<PickedUpLayerType>>>,
    pub MORPH: Option<Rc<RefCell<PickedUpLayerType>>>,
    pub PART: Option<Rc<RefCell<PickedUpLayerType>>>,
    pub HEAL: Option<Rc<RefCell<PickedUpLayerType>>>,
    pub LIFE: Option<Rc<RefCell<PickedUpLayerType>>>,
    pub POINTS: Option<Rc<RefCell<PickedUpLayerType>>>,
}

thread_local! { pub static PickedUpLayerTypeFactory_instance: Option<Rc<RefCell<PickedUpLayerTypeFactory>>> = PickedUpLayerTypeFactory::new(); }

impl PickedUpLayerTypeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NONE: None,
            BEAM: None,
            MINE: None,
            PROJECTILE: None,
            ENHANCEMENT: None,
            MORPH: None,
            PART: None,
            HEAL: None,
            LIFE: None,
            POINTS: None,
        };
        this.NONE = PickedUpLayerType::new(jcall!(StringUtil::getInstance()).NULL_STRING.clone());
        this.BEAM = PickedUpLayerType::new(String::from("BEAM"));
        this.MINE = PickedUpLayerType::new(String::from("MINE"));
        this.PROJECTILE = PickedUpLayerType::new(String::from("PROJECTILE"));
        this.ENHANCEMENT = PickedUpLayerType::new(String::from("Enhancement"));
        this.MORPH = PickedUpLayerType::new(String::from("Morph"));
        this.PART = PickedUpLayerType::new(String::from("Part"));
        this.HEAL = PickedUpLayerType::new(String::from("Heal"));
        this.LIFE = PickedUpLayerType::new(String::from("Life"));
        this.POINTS = PickedUpLayerType::new(String::from("Points"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PickedUpLayerTypeFactory>>> {
        return PickedUpLayerTypeFactory_instance.with(Clone::clone).clone();
    }
}

