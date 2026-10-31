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
// package org.allbinary.game.combat.damage
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::DamageableInterface::DamageableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DamageUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub COLLIDE: String,
}

thread_local! { pub static DamageUtil_instance: Option<Rc<RefCell<DamageUtil>>> = DamageUtil::new(); }

impl DamageUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<DamageUtil>>> {
        return DamageUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            COLLIDE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.COLLIDE = String::from("collide");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn process(&mut self, mut damageableInterface: Option<Rc<RefCell<dyn DamageableInterface>>>, mut damageableInterface2: Option<Rc<RefCell<dyn DamageableInterface>>>) {
        jcall!(damageableInterface).damage(jcall!(damageableInterface2).getDamage(0), 0);
    }
    

    // throws Exception
    pub fn debugDamage(&mut self, mut collidableInterfaceCompositeInterface1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) {
        let mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = (collidableInterfaceCompositeInterface).clone() /* as CollidableDestroyableDamageableLayer */;
        let mut damage: i32 = jcall!(collidableDestroyableDamageableLayer).getDamage(0);
        if damage > 20000 {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Collision "));
            jcall!(stringBuffer).append(jcall!(collidableInterfaceCompositeInterface1).toString());
            jcall!(stringBuffer).append(String::from(" with: "));
            jcall!(stringBuffer).append(jcall!(collidableInterfaceCompositeInterface).toString());
            jcall!(stringBuffer).append(String::from(" damage: "));
            jcall!(stringBuffer).appendint(damage.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(collidableInterfaceCompositeInterface1.clone()), self.COLLIDE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
}

