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
// package org.allbinary.game.combat.weapon
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::SimpleWeaponProperties::SimpleWeaponProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WeaponProperties {
    pub base: SimpleWeaponProperties,
    pub reloadTime: i64,
    pub targetingTime: i64,
    pub speed: Option<Rc<RefCell<BasicDecimal>>>,
    pub MAX: i64,
    pub ZERO: i16,
}

thread_local! { pub static WeaponProperties_NULL_WEAPON_PROPERTIES: Option<Rc<RefCell<WeaponProperties>>> = WeaponProperties::new(0_i64, 0_i64, 0_i64, 0, ((0) as i16)); }
pub static mut WeaponProperties_messageSent: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| false);
thread_local! { pub static WeaponProperties_DAMAGE: String = String::from("Damage: "); }
thread_local! { pub static WeaponProperties_RANGE: String = String::from("Range: "); }
thread_local! { pub static WeaponProperties_RELOAD: String = String::from("Reload: "); }

impl WeaponProperties {
    

    pub fn new_struct(mut reloadTime: i64, mut targetingTime: i64, mut speed: i64, mut damage: i32, mut dissipation: i16) -> Self {
        let mut this = Self {
            base: SimpleWeaponProperties::new_struct(),
            reloadTime: 0_i64,
            targetingTime: 0_i64,
            speed: None,
            MAX: 0_i64,
            ZERO: 0,
        };
        this.speed = crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal_ZERO_BIGDECIMAL.with(Clone::clone).clone();
        this.MAX = 10240_i64;
        this.ZERO = 0;
        if speed < this.MAX && speed != 0_i64 && !unsafe { WeaponProperties_messageSent.clone() } {
            let mut MESSAGE: String = String::from("Danger Danger Danger: Speed probably to slow if using 1 degree calculations as velocity for a single axis could be below 1024: ");
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::put(jcall!(StringMaker::new()).append(MESSAGE.clone()).appendlong(speed.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
            unsafe { *WeaponProperties_messageSent = true };
        }
        this.setReloadTime(reloadTime.clone());
        this.setTargetingTime(targetingTime.clone());
        this.setDamage(damage.clone());
        this.setDissipation(dissipation.clone());
        this.setSpeed(BasicDecimal::new(speed.clone()));
        if ((dissipation) as i32) != ((this.ZERO) as i32) {
            let mut unscaledDamage: i64 = jcall!(this.speed).getUnscaled() * ((damage) as i64);
            let mut scaledDissipation: i32 = ((dissipation) as i32) * jcall!(this.speed).getScaledFactorValue();
            let mut value: i64 = (unscaledDamage / ((scaledDissipation) as i64));
            this.setRange((((value * 9_i64)) as i32) / 10);
        }
        this
    }

    pub fn new(mut reloadTime: i64, mut targetingTime: i64, mut speed: i64, mut damage: i32, mut dissipation: i16) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(reloadTime, targetingTime, speed, damage, dissipation))))
    }
    

    pub fn setReloadTime(&mut self, mut reloadTime: i64) {
        self.reloadTime = reloadTime.clone();
    }
    

    pub fn getReloadTime(&mut self) -> i64 {
        return self.reloadTime.clone();
    }
    

    pub fn setTargetingTime(&mut self, mut targetingTime: i64) {
        self.targetingTime = targetingTime.clone();
    }
    

    pub fn getTargetingTime(&mut self) -> i64 {
        return self.targetingTime.clone();
    }
    

    pub fn getSpeed(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> {
        return self.speed.clone();
    }
    

    pub fn setSpeed(&mut self, mut speed: Option<Rc<RefCell<BasicDecimal>>>) {
        self.speed = speed.clone();
    }
    

    pub fn getDamageI(&mut self, mut range: i32) -> i32 {
        return self.getDamage() - ((self.getDissipation() * range) / jcall!(self.speed).getScaled());
    }
    

    pub fn toStringArray(&mut self) -> Vec<String> {
        let mut index: i32 = 0;
        let mut stringArray: Vec<String> = vec![String::new(); (3) as usize];
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        stringArray[({ let value = index; index += 1; value }) as usize] = jcall!(stringBuffer).append(WeaponProperties_DAMAGE.with(Clone::clone).clone()).appendint(((self.getDamage()) as i32)).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        stringArray[({ let value = index; index += 1; value }) as usize] = jcall!(stringBuffer).append(WeaponProperties_RANGE.with(Clone::clone).clone()).appendint(((self.getRange()) as i32)).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        stringArray[({ let value = index; index += 1; value }) as usize] = jcall!(stringBuffer).append(WeaponProperties_RELOAD.with(Clone::clone).clone()).appendlong(self.getReloadTime()).toString();
        return stringArray.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(WeaponProperties_DAMAGE.with(Clone::clone).clone()).appendint(((self.getDamage()) as i32));
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(WeaponProperties_RANGE.with(Clone::clone).clone()).appendint(((self.getRange()) as i32));
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(WeaponProperties_RELOAD.with(Clone::clone).clone()).appendlong(self.getReloadTime());
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for WeaponProperties {
    type Target = SimpleWeaponProperties;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WeaponProperties {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::combat::weapon::SimpleWeaponProperties::SimpleWeaponPropertiesDyn for WeaponProperties {
    fn as_SimpleWeaponProperties(&self) -> &crate::org::allbinary::game::combat::weapon::SimpleWeaponProperties::SimpleWeaponProperties { &self.base }
    fn as_SimpleWeaponProperties_mut(&mut self) -> &mut crate::org::allbinary::game::combat::weapon::SimpleWeaponProperties::SimpleWeaponProperties { &mut self.base }
}


