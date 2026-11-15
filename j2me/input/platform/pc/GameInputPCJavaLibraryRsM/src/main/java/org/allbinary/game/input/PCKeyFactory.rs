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
use crate::org::allbinary::math::PositionStrings::PositionStrings;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::ABKeyEventWrapper::ABKeyEventWrapper;
use super::PCGameKey::PCGameKey;
use super::Input::Input;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PCKeyFactory {
    pub keyEventWrapper: Option<Rc<RefCell<ABKeyEventWrapper>>>,
    pub UNKNOWN: Option<Rc<RefCell<PCGameKey>>>,
    pub ENTER: Option<Rc<RefCell<PCGameKey>>>,
    pub _A: Option<Rc<RefCell<PCGameKey>>>,
    pub _B: Option<Rc<RefCell<PCGameKey>>>,
    pub _C: Option<Rc<RefCell<PCGameKey>>>,
    pub _D: Option<Rc<RefCell<PCGameKey>>>,
    pub _E: Option<Rc<RefCell<PCGameKey>>>,
    pub _F: Option<Rc<RefCell<PCGameKey>>>,
    pub _G: Option<Rc<RefCell<PCGameKey>>>,
    pub _H: Option<Rc<RefCell<PCGameKey>>>,
    pub _I: Option<Rc<RefCell<PCGameKey>>>,
    pub _J: Option<Rc<RefCell<PCGameKey>>>,
    pub _K: Option<Rc<RefCell<PCGameKey>>>,
    pub _L: Option<Rc<RefCell<PCGameKey>>>,
    pub _M: Option<Rc<RefCell<PCGameKey>>>,
    pub _N: Option<Rc<RefCell<PCGameKey>>>,
    pub _O: Option<Rc<RefCell<PCGameKey>>>,
    pub _P: Option<Rc<RefCell<PCGameKey>>>,
    pub _Q: Option<Rc<RefCell<PCGameKey>>>,
    pub _R: Option<Rc<RefCell<PCGameKey>>>,
    pub _S: Option<Rc<RefCell<PCGameKey>>>,
    pub _T: Option<Rc<RefCell<PCGameKey>>>,
    pub _U: Option<Rc<RefCell<PCGameKey>>>,
    pub _V: Option<Rc<RefCell<PCGameKey>>>,
    pub _W: Option<Rc<RefCell<PCGameKey>>>,
    pub _X: Option<Rc<RefCell<PCGameKey>>>,
    pub _Y: Option<Rc<RefCell<PCGameKey>>>,
    pub _Z: Option<Rc<RefCell<PCGameKey>>>,
    pub a: Option<Rc<RefCell<PCGameKey>>>,
    pub b: Option<Rc<RefCell<PCGameKey>>>,
    pub c: Option<Rc<RefCell<PCGameKey>>>,
    pub d: Option<Rc<RefCell<PCGameKey>>>,
    pub e: Option<Rc<RefCell<PCGameKey>>>,
    pub f: Option<Rc<RefCell<PCGameKey>>>,
    pub g: Option<Rc<RefCell<PCGameKey>>>,
    pub h: Option<Rc<RefCell<PCGameKey>>>,
    pub i: Option<Rc<RefCell<PCGameKey>>>,
    pub j: Option<Rc<RefCell<PCGameKey>>>,
    pub k: Option<Rc<RefCell<PCGameKey>>>,
    pub l: Option<Rc<RefCell<PCGameKey>>>,
    pub m: Option<Rc<RefCell<PCGameKey>>>,
    pub n: Option<Rc<RefCell<PCGameKey>>>,
    pub o: Option<Rc<RefCell<PCGameKey>>>,
    pub p: Option<Rc<RefCell<PCGameKey>>>,
    pub q: Option<Rc<RefCell<PCGameKey>>>,
    pub r: Option<Rc<RefCell<PCGameKey>>>,
    pub s: Option<Rc<RefCell<PCGameKey>>>,
    pub t: Option<Rc<RefCell<PCGameKey>>>,
    pub u: Option<Rc<RefCell<PCGameKey>>>,
    pub v: Option<Rc<RefCell<PCGameKey>>>,
    pub w: Option<Rc<RefCell<PCGameKey>>>,
    pub x: Option<Rc<RefCell<PCGameKey>>>,
    pub y: Option<Rc<RefCell<PCGameKey>>>,
    pub z: Option<Rc<RefCell<PCGameKey>>>,
    pub ZERO: Option<Rc<RefCell<PCGameKey>>>,
    pub ONE: Option<Rc<RefCell<PCGameKey>>>,
    pub TWO: Option<Rc<RefCell<PCGameKey>>>,
    pub THREE: Option<Rc<RefCell<PCGameKey>>>,
    pub FOUR: Option<Rc<RefCell<PCGameKey>>>,
    pub FIVE: Option<Rc<RefCell<PCGameKey>>>,
    pub SIX: Option<Rc<RefCell<PCGameKey>>>,
    pub SEVEN: Option<Rc<RefCell<PCGameKey>>>,
    pub EIGHT: Option<Rc<RefCell<PCGameKey>>>,
    pub NINE: Option<Rc<RefCell<PCGameKey>>>,
    pub DPAD_UP: Option<Rc<RefCell<PCGameKey>>>,
    pub DPAD_DOWN: Option<Rc<RefCell<PCGameKey>>>,
    pub DPAD_LEFT: Option<Rc<RefCell<PCGameKey>>>,
    pub DPAD_RIGHT: Option<Rc<RefCell<PCGameKey>>>,
    pub DPAD_UP2: Option<Rc<RefCell<PCGameKey>>>,
    pub DPAD_DOWN2: Option<Rc<RefCell<PCGameKey>>>,
    pub DPAD_LEFT2: Option<Rc<RefCell<PCGameKey>>>,
    pub DPAD_RIGHT2: Option<Rc<RefCell<PCGameKey>>>,
    pub COMMA: Option<Rc<RefCell<PCGameKey>>>,
    pub SPACE: Option<Rc<RefCell<PCGameKey>>>,
    pub DEL: Option<Rc<RefCell<PCGameKey>>>,
    pub STAR: Option<Rc<RefCell<PCGameKey>>>,
    pub QUESTION: Option<Rc<RefCell<PCGameKey>>>,
    pub PERIOD: Option<Rc<RefCell<PCGameKey>>>,
    pub ESCAPE: Option<Rc<RefCell<PCGameKey>>>,
    pub BACK_SPACE: Option<Rc<RefCell<PCGameKey>>>,
}

pub static PCKeyFactory_SINGLETON: crate::JavaStatic<Option<Rc<RefCell<PCKeyFactory>>>> = crate::JavaStatic::new(|| PCKeyFactory::new());

impl PCKeyFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            keyEventWrapper: None,
            UNKNOWN: None,
            ENTER: None,
            _A: None,
            _B: None,
            _C: None,
            _D: None,
            _E: None,
            _F: None,
            _G: None,
            _H: None,
            _I: None,
            _J: None,
            _K: None,
            _L: None,
            _M: None,
            _N: None,
            _O: None,
            _P: None,
            _Q: None,
            _R: None,
            _S: None,
            _T: None,
            _U: None,
            _V: None,
            _W: None,
            _X: None,
            _Y: None,
            _Z: None,
            a: None,
            b: None,
            c: None,
            d: None,
            e: None,
            f: None,
            g: None,
            h: None,
            i: None,
            j: None,
            k: None,
            l: None,
            m: None,
            n: None,
            o: None,
            p: None,
            q: None,
            r: None,
            s: None,
            t: None,
            u: None,
            v: None,
            w: None,
            x: None,
            y: None,
            z: None,
            ZERO: None,
            ONE: None,
            TWO: None,
            THREE: None,
            FOUR: None,
            FIVE: None,
            SIX: None,
            SEVEN: None,
            EIGHT: None,
            NINE: None,
            DPAD_UP: None,
            DPAD_DOWN: None,
            DPAD_LEFT: None,
            DPAD_RIGHT: None,
            DPAD_UP2: None,
            DPAD_DOWN2: None,
            DPAD_LEFT2: None,
            DPAD_RIGHT2: None,
            COMMA: None,
            SPACE: None,
            DEL: None,
            STAR: None,
            QUESTION: None,
            PERIOD: None,
            ESCAPE: None,
            BACK_SPACE: None,
        };
        this.keyEventWrapper = ABKeyEventWrapper::getInstance();
        this.UNKNOWN = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_UNDEFINED.clone(); java_field }.clone(), String::from("UNKNOWN"));
        this.ENTER = PCGameKey::new(5, String::from("Enter"));
        this._A = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_A.clone(); java_field }.clone(), String::from("A"));
        this._B = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_B.clone(); java_field }.clone(), String::from("B"));
        this._C = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_C.clone(); java_field }.clone(), String::from("C"));
        this._D = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_D.clone(); java_field }.clone(), String::from("D"));
        this._E = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_E.clone(); java_field }.clone(), String::from("E"));
        this._F = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_F.clone(); java_field }.clone(), String::from("F"));
        this._G = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_G.clone(); java_field }.clone(), String::from("G"));
        this._H = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_H.clone(); java_field }.clone(), String::from("H"));
        this._I = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_I.clone(); java_field }.clone(), String::from("I"));
        this._J = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_J.clone(); java_field }.clone(), String::from("J"));
        this._K = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_K.clone(); java_field }.clone(), String::from("K"));
        this._L = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_L.clone(); java_field }.clone(), String::from("L"));
        this._M = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_M.clone(); java_field }.clone(), String::from("M"));
        this._N = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_N.clone(); java_field }.clone(), String::from("N"));
        this._O = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_O.clone(); java_field }.clone(), String::from("O"));
        this._P = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_P.clone(); java_field }.clone(), String::from("P"));
        this._Q = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_Q.clone(); java_field }.clone(), String::from("Q"));
        this._R = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_R.clone(); java_field }.clone(), String::from("R"));
        this._S = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_S.clone(); java_field }.clone(), String::from("S"));
        this._T = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_T.clone(); java_field }.clone(), String::from("T"));
        this._U = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_U.clone(); java_field }.clone(), String::from("U"));
        this._V = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_V.clone(); java_field }.clone(), String::from("V"));
        this._W = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_W.clone(); java_field }.clone(), String::from("W"));
        this._X = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_X.clone(); java_field }.clone(), String::from("X"));
        this._Y = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_Y.clone(); java_field }.clone(), String::from("Y"));
        this._Z = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_Z.clone(); java_field }.clone(), String::from("Z"));
        this.a = PCGameKey::new(97, String::from("a"));
        this.b = PCGameKey::new(98, String::from("b"));
        this.c = PCGameKey::new(99, String::from("c"));
        this.d = PCGameKey::new(100, String::from("d"));
        this.e = PCGameKey::new(101, String::from("e"));
        this.f = PCGameKey::new(102, String::from("f"));
        this.g = PCGameKey::new(103, String::from("g"));
        this.h = PCGameKey::new(104, String::from("h"));
        this.i = PCGameKey::new(105, String::from("i"));
        this.j = PCGameKey::new(106, String::from("j"));
        this.k = PCGameKey::new(107, String::from("k"));
        this.l = PCGameKey::new(108, String::from("l"));
        this.m = PCGameKey::new(109, String::from("m"));
        this.n = PCGameKey::new(110, String::from("n"));
        this.o = PCGameKey::new(111, String::from("o"));
        this.p = PCGameKey::new(112, String::from("p"));
        this.q = PCGameKey::new(113, String::from("q"));
        this.r = PCGameKey::new(114, String::from("r"));
        this.s = PCGameKey::new(115, String::from("s"));
        this.t = PCGameKey::new(116, String::from("t"));
        this.u = PCGameKey::new(117, String::from("u"));
        this.v = PCGameKey::new(118, String::from("v"));
        this.w = PCGameKey::new(119, String::from("w"));
        this.x = PCGameKey::new(120, { let java_field = jcall!(PositionStrings::getInstance()).X.clone(); java_field }.clone());
        this.y = PCGameKey::new(121, { let java_field = jcall!(PositionStrings::getInstance()).Y.clone(); java_field }.clone());
        this.z = PCGameKey::new(122, String::from("z"));
        this.ZERO = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_0.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(); java_field }.clone());
        this.ONE = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_1.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).ONE.clone(); java_field }.clone());
        this.TWO = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_2.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).TWO.clone(); java_field }.clone());
        this.THREE = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_3.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).THREE.clone(); java_field }.clone());
        this.FOUR = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_4.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).FOUR.clone(); java_field }.clone());
        this.FIVE = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_5.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).FIVE.clone(); java_field }.clone());
        this.SIX = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_6.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).SIX.clone(); java_field }.clone());
        this.SEVEN = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_7.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).SEVEN.clone(); java_field }.clone());
        this.EIGHT = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_8.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).EIGHT.clone(); java_field }.clone());
        this.NINE = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_9.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).NINE.clone(); java_field }.clone());
        this.DPAD_UP = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_UP.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).UP.clone(); java_field }.clone());
        this.DPAD_DOWN = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_DOWN.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).DOWN.clone(); java_field }.clone());
        this.DPAD_LEFT = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_LEFT.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).LEFT.clone(); java_field }.clone());
        this.DPAD_RIGHT = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_RIGHT.clone(); java_field }.clone(), { let java_field = jcall!(CommonPhoneStrings::getInstance()).RIGHT.clone(); java_field }.clone());
        this.DPAD_UP2 = this.DPAD_UP.clone();
        this.DPAD_DOWN2 = this.DPAD_DOWN.clone();
        this.DPAD_LEFT2 = this.DPAD_LEFT.clone();
        this.DPAD_RIGHT2 = this.DPAD_RIGHT.clone();
        this.COMMA = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_COMMA.clone(); java_field }.clone(), String::from("COMMA"));
        this.SPACE = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_SPACE.clone(); java_field }.clone(), String::from("SPACE"));
        this.DEL = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_DELETE.clone(); java_field }.clone(), String::from("DEL"));
        this.STAR = PCGameKey::new(42, { let java_field = jcall!(CommonPhoneStrings::getInstance()).STAR.clone(); java_field }.clone());
        this.QUESTION = PCGameKey::new(47, String::from("?"));
        this.PERIOD = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_PERIOD.clone(); java_field }.clone(), { let java_field = jcall!(CommonSeps::getInstance()).PERIOD.clone(); java_field }.clone());
        this.ESCAPE = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_ESCAPE.clone(); java_field }.clone(), String::from("Esc"));
        this.BACK_SPACE = PCGameKey::new({ let java_field = jcall!(this.keyEventWrapper).VK_BACK_SPACE.clone(); java_field }.clone(), String::from("Backspace"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PCKeyFactory>>> {
        return PCKeyFactory_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn isSubmission(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &self.ENTER) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isDelete(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &self.DEL) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isBackSpace(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &self.BACK_SPACE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isUp(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &self.DPAD_UP) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&input, &self.DPAD_UP2) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isDown(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &self.DPAD_DOWN) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&input, &self.DPAD_DOWN2) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isLeft(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &self.DPAD_LEFT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&input, &self.DPAD_LEFT2) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isRight(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &self.DPAD_RIGHT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&input, &self.DPAD_RIGHT2) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isEnter(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &self.ENTER) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
}

