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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use super::GameKey::GameKey;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameKeyFactory {
    pub NONE: Option<Rc<RefCell<GameKey>>>,
    pub UP: Option<Rc<RefCell<GameKey>>>,
    pub DOWN: Option<Rc<RefCell<GameKey>>>,
    pub LEFT: Option<Rc<RefCell<GameKey>>>,
    pub RIGHT: Option<Rc<RefCell<GameKey>>>,
    pub KEY_POUND: Option<Rc<RefCell<GameKey>>>,
    pub KEY_STAR: Option<Rc<RefCell<GameKey>>>,
    pub KEY_NUM0: Option<Rc<RefCell<GameKey>>>,
    pub KEY_NUM1: Option<Rc<RefCell<GameKey>>>,
    pub KEY_NUM2: Option<Rc<RefCell<GameKey>>>,
    pub KEY_NUM3: Option<Rc<RefCell<GameKey>>>,
    pub KEY_NUM4: Option<Rc<RefCell<GameKey>>>,
    pub KEY_NUM5: Option<Rc<RefCell<GameKey>>>,
    pub KEY_NUM6: Option<Rc<RefCell<GameKey>>>,
    pub KEY_NUM7: Option<Rc<RefCell<GameKey>>>,
    pub KEY_NUM8: Option<Rc<RefCell<GameKey>>>,
    pub KEY_NUM9: Option<Rc<RefCell<GameKey>>>,
    pub GAME_A: Option<Rc<RefCell<GameKey>>>,
    pub GAME_B: Option<Rc<RefCell<GameKey>>>,
    pub GAME_C: Option<Rc<RefCell<GameKey>>>,
    pub GAME_D: Option<Rc<RefCell<GameKey>>>,
    pub FIRE: Option<Rc<RefCell<GameKey>>>,
    pub LEVEL_DOWN: Option<Rc<RefCell<GameKey>>>,
    pub LEVEL_UP: Option<Rc<RefCell<GameKey>>>,
}

pub static mut GameKeyFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());

impl GameKeyFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            NONE: None,
            UP: None,
            DOWN: None,
            LEFT: None,
            RIGHT: None,
            KEY_POUND: None,
            KEY_STAR: None,
            KEY_NUM0: None,
            KEY_NUM1: None,
            KEY_NUM2: None,
            KEY_NUM3: None,
            KEY_NUM4: None,
            KEY_NUM5: None,
            KEY_NUM6: None,
            KEY_NUM7: None,
            KEY_NUM8: None,
            KEY_NUM9: None,
            GAME_A: None,
            GAME_B: None,
            GAME_C: None,
            GAME_D: None,
            FIRE: None,
            LEVEL_DOWN: None,
            LEVEL_UP: None,
        };
        this.NONE = GameKey::new({ let java_field = jcall!(InputFactory::InputFactory::getInstance()).MAX.clone(); java_field } - 1, String::from("None"));
        this.UP = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_UP.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).UP.clone(); java_field }.clone());
        this.DOWN = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).DOWN.clone(); java_field }.clone());
        this.LEFT = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).LEFT.clone(); java_field }.clone());
        this.RIGHT = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).RIGHT.clone(); java_field }.clone());
        this.KEY_POUND = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_POUND.with(Clone::clone).clone(), String::from("POUND"));
        this.KEY_STAR = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_STAR.with(Clone::clone).clone(), String::from("STAR"));
        this.KEY_NUM0 = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(); java_field }.clone());
        this.KEY_NUM1 = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).ONE.clone(); java_field }.clone());
        this.KEY_NUM2 = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM2.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).TWO.clone(); java_field }.clone());
        this.KEY_NUM3 = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM3.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).THREE.clone(); java_field }.clone());
        this.KEY_NUM4 = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM4.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).FOUR.clone(); java_field }.clone());
        this.KEY_NUM5 = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM5.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).FIVE.clone(); java_field }.clone());
        this.KEY_NUM6 = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM6.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).SIX.clone(); java_field }.clone());
        this.KEY_NUM7 = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM7.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).SEVEN.clone(); java_field }.clone());
        this.KEY_NUM8 = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM8.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).EIGHT.clone(); java_field }.clone());
        this.KEY_NUM9 = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).NINE.clone(); java_field }.clone());
        this.GAME_A = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_GAME_A.with(Clone::clone).clone(), String::from("Game A"));
        this.GAME_B = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_GAME_B.with(Clone::clone).clone(), String::from("Game B"));
        this.GAME_C = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_GAME_C.with(Clone::clone).clone(), String::from("Game C"));
        this.GAME_D = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_GAME_D.with(Clone::clone).clone(), String::from("Game D"));
        this.FIRE = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_FIRE.with(Clone::clone).clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).FIRE.clone(); java_field }.clone());
        this.LEVEL_DOWN = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.with(Clone::clone) + 1, String::from("Cheat Level Up"));
        this.LEVEL_UP = GameKey::new(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.with(Clone::clone) + 2, String::from("Cheat Level Down"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameKeyFactory>>> {
        if match (&unsafe { GameKeyFactory_instance.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *GameKeyFactory_instance = crate::java_into_object(GameKeyFactory::new()) };
        }
        return { let java_object = (unsafe { GameKeyFactory_instance.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameKeyFactory>>>>().expect("ClassCastException").clone()) };
    }
}

