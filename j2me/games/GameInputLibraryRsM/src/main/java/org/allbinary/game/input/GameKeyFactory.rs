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
// use crate::javax::microedition::lcdui::Canvas::Canvas;
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

pub static mut GameKeyFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl GameKeyFactory {

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
        this.NONE = GameKey::new(jcall!(InputFactory::InputFactory::getInstance()).MAX - 1, String::from("None"));
        this.UP = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).UP.clone());
        this.DOWN = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).DOWN.clone());
        this.LEFT = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).LEFT.clone());
        this.RIGHT = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).RIGHT.clone());
        this.KEY_POUND = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_POUND.clone() }.clone(), String::from("POUND"));
        this.KEY_STAR = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_STAR.clone() }.clone(), String::from("STAR"));
        this.KEY_NUM0 = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).ZERO.clone());
        this.KEY_NUM1 = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).ONE.clone());
        this.KEY_NUM2 = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM2.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).TWO.clone());
        this.KEY_NUM3 = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM3.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).THREE.clone());
        this.KEY_NUM4 = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM4.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).FOUR.clone());
        this.KEY_NUM5 = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM5.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).FIVE.clone());
        this.KEY_NUM6 = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM6.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).SIX.clone());
        this.KEY_NUM7 = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM7.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).SEVEN.clone());
        this.KEY_NUM8 = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM8.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).EIGHT.clone());
        this.KEY_NUM9 = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).NINE.clone());
        this.GAME_A = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_GAME_A.clone() }.clone(), String::from("Game A"));
        this.GAME_B = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_GAME_B.clone() }.clone(), String::from("Game B"));
        this.GAME_C = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_GAME_C.clone() }.clone(), String::from("Game C"));
        this.GAME_D = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_GAME_D.clone() }.clone(), String::from("Game D"));
        this.FIRE = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_FIRE.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).FIRE.clone());
        this.LEVEL_DOWN = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.clone() } + 1, String::from("Cheat Level Up"));
        this.LEVEL_UP = GameKey::new(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.clone() } + 2, String::from("Cheat Level Down"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameKeyFactory>>> {
        if match (&unsafe { GameKeyFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *GameKeyFactory_instance = GameKeyFactory::new() };
        }
        return (unsafe { GameKeyFactory_instance.clone() }).clone() /* as GameKeyFactory */;
    }
}

