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
// package org.allbinary.game.health
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::HealthInterface::HealthInterface;
use super::HealthListenerInterface::HealthListenerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Health {
    pub health: i32,
    pub maxHealth: i32,
    pub healthListenerInterface: Option<Rc<RefCell<dyn HealthListenerInterface>>>,
}

thread_local! { pub static Health_NULL_HEALTH: Option<Rc<RefCell<Health>>> = Health::new(0); }
thread_local! { pub static Health_NAME: String = String::from("Health Resource"); }
thread_local! { pub static Health_HEALTH_LABEL: String = String::from("Health: "); }
thread_local! { pub static Health_MAX_HEALTH_LABEL: String = String::from(" Max: "); }
impl HealthInterface for Health {
    fn isAlive(&mut self) -> bool { Self::isAlive(self) }
    fn damage(&mut self, mut ahealth: i32) { Self::damage(self, ahealth) }
    fn heal(&mut self, mut ahealth: i32) { Self::heal(self, ahealth) }
    fn healMax(&mut self) { Self::healMax(self) }
    fn getMaxHealth(&mut self) -> i32 { Self::getMaxHealth(self) }
    fn setMaxHealth(&mut self, mut maxHealth: i32) { Self::setMaxHealth(self, maxHealth) }
    fn getHealth(&mut self) -> i32 { Self::getHealth(self) }
    fn addListener(&mut self, mut healthGraphic: Option<Rc<RefCell<dyn HealthListenerInterface>>>) { Self::addListener(self, healthGraphic) }
}

impl Health {
    

    pub fn new_struct(mut maxHealth: i32) -> Self {
        let mut this = Self {
            health: 0,
            maxHealth: 0,
            healthListenerInterface: None,
        };
        this.healthListenerInterface = NullHealthListener_NULL_HEALTH_LISTENER.with(Clone::clone).clone();
        this.setMaxHealth(maxHealth.clone());
        this.setHealth(maxHealth.clone());
        this
    }

    pub fn new(mut maxHealth: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(maxHealth))))
    }
    

    pub fn heal(&mut self, mut ahealth: i32) {
        let mut newHealth: i32 = self.getHealth() + ahealth;
        if newHealth > self.getMaxHealth() {
            { let java_arguments = (self.getMaxHealth(),); self.setHealth(java_arguments.0) };
        } else {
            self.setHealth(newHealth.clone());
        }
    }
    

    pub fn healMax(&mut self) {
        { let java_arguments = (self.getMaxHealth(),); self.setHealth(java_arguments.0) };
    }
    

    pub fn isDamaged(&mut self) -> bool {
        if self.getHealth() != self.getMaxHealth() {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn damage(&mut self, mut ahealth: i32) {
        if ahealth < 0 {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("******* Trying to heal when damaging. Damage: "));
            jcall!(stringBuffer).appendint(ahealth.clone());
            jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
            jcall!(stringBuffer).append(self.toString());
            ForcedLogUtil::log(jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
            return;
        }
        let mut health: i32 = self.getHealth() - ahealth;
        self.setHealth(health.clone());
    }
    

    pub fn isAlive(&mut self) -> bool {
        if self.getHealth() <= 0 {
            return false;
        } else {
            return true;
        }
    }
    

    pub fn getMaxHealth(&mut self) -> i32 {
        return self.maxHealth.clone();
    }
    

    pub fn setMaxHealth(&mut self, mut maxHealth: i32) {
        self.maxHealth = maxHealth.clone();
    }
    

    pub fn setHealth(&mut self, mut health: i32) {
        if health > self.getMaxHealth() {
            self.health = self.getMaxHealth();
        } else if health < 0 {
            self.health = 0;
        } else {
            self.health = health.clone();
        }
        if self.healthListenerInterface.is_some() {
            jcall!(self.healthListenerInterface).onHealthChange();
        }
    }
    

    pub fn getHealth(&mut self) -> i32 {
        return self.health.clone();
    }
    

    pub fn addListener(&mut self, mut healthGraphic: Option<Rc<RefCell<dyn HealthListenerInterface>>>) {
        self.healthListenerInterface = healthGraphic.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(Health_HEALTH_LABEL.with(Clone::clone).clone());
        jcall!(stringBuffer).appendint(self.health.clone());
        jcall!(stringBuffer).append(Health_MAX_HEALTH_LABEL.with(Clone::clone).clone());
        jcall!(stringBuffer).appendint(self.maxHealth.clone());
        return jcall!(stringBuffer).toString();
    }
}

