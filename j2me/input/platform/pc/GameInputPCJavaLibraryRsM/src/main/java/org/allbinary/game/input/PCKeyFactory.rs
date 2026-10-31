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

thread_local! { pub static PCKeyFactory_SINGLETON: Option<Rc<RefCell<PCKeyFactory>>> = PCKeyFactory::new(); }

impl PCKeyFactory {

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
        this.UNKNOWN = PCGameKey::new(jcall!(this.keyEventWrapper).VK_UNDEFINED.clone(), String::from("UNKNOWN"));
        this.ENTER = PCGameKey::new(5, String::from("Enter"));
        this._A = PCGameKey::new(jcall!(this.keyEventWrapper).VK_A.clone(), String::from("A"));
        this._B = PCGameKey::new(jcall!(this.keyEventWrapper).VK_B.clone(), String::from("B"));
        this._C = PCGameKey::new(jcall!(this.keyEventWrapper).VK_C.clone(), String::from("C"));
        this._D = PCGameKey::new(jcall!(this.keyEventWrapper).VK_D.clone(), String::from("D"));
        this._E = PCGameKey::new(jcall!(this.keyEventWrapper).VK_E.clone(), String::from("E"));
        this._F = PCGameKey::new(jcall!(this.keyEventWrapper).VK_F.clone(), String::from("F"));
        this._G = PCGameKey::new(jcall!(this.keyEventWrapper).VK_G.clone(), String::from("G"));
        this._H = PCGameKey::new(jcall!(this.keyEventWrapper).VK_H.clone(), String::from("H"));
        this._I = PCGameKey::new(jcall!(this.keyEventWrapper).VK_I.clone(), String::from("I"));
        this._J = PCGameKey::new(jcall!(this.keyEventWrapper).VK_J.clone(), String::from("J"));
        this._K = PCGameKey::new(jcall!(this.keyEventWrapper).VK_K.clone(), String::from("K"));
        this._L = PCGameKey::new(jcall!(this.keyEventWrapper).VK_L.clone(), String::from("L"));
        this._M = PCGameKey::new(jcall!(this.keyEventWrapper).VK_M.clone(), String::from("M"));
        this._N = PCGameKey::new(jcall!(this.keyEventWrapper).VK_N.clone(), String::from("N"));
        this._O = PCGameKey::new(jcall!(this.keyEventWrapper).VK_O.clone(), String::from("O"));
        this._P = PCGameKey::new(jcall!(this.keyEventWrapper).VK_P.clone(), String::from("P"));
        this._Q = PCGameKey::new(jcall!(this.keyEventWrapper).VK_Q.clone(), String::from("Q"));
        this._R = PCGameKey::new(jcall!(this.keyEventWrapper).VK_R.clone(), String::from("R"));
        this._S = PCGameKey::new(jcall!(this.keyEventWrapper).VK_S.clone(), String::from("S"));
        this._T = PCGameKey::new(jcall!(this.keyEventWrapper).VK_T.clone(), String::from("T"));
        this._U = PCGameKey::new(jcall!(this.keyEventWrapper).VK_U.clone(), String::from("U"));
        this._V = PCGameKey::new(jcall!(this.keyEventWrapper).VK_V.clone(), String::from("V"));
        this._W = PCGameKey::new(jcall!(this.keyEventWrapper).VK_W.clone(), String::from("W"));
        this._X = PCGameKey::new(jcall!(this.keyEventWrapper).VK_X.clone(), String::from("X"));
        this._Y = PCGameKey::new(jcall!(this.keyEventWrapper).VK_Y.clone(), String::from("Y"));
        this._Z = PCGameKey::new(jcall!(this.keyEventWrapper).VK_Z.clone(), String::from("Z"));
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
        this.x = PCGameKey::new(120, jcall!(PositionStrings::getInstance()).X.clone());
        this.y = PCGameKey::new(121, jcall!(PositionStrings::getInstance()).Y.clone());
        this.z = PCGameKey::new(122, String::from("z"));
        this.ZERO = PCGameKey::new(jcall!(this.keyEventWrapper).VK_0.clone(), jcall!(CommonPhoneStrings::getInstance()).ZERO.clone());
        this.ONE = PCGameKey::new(jcall!(this.keyEventWrapper).VK_1.clone(), jcall!(CommonPhoneStrings::getInstance()).ONE.clone());
        this.TWO = PCGameKey::new(jcall!(this.keyEventWrapper).VK_2.clone(), jcall!(CommonPhoneStrings::getInstance()).TWO.clone());
        this.THREE = PCGameKey::new(jcall!(this.keyEventWrapper).VK_3.clone(), jcall!(CommonPhoneStrings::getInstance()).THREE.clone());
        this.FOUR = PCGameKey::new(jcall!(this.keyEventWrapper).VK_4.clone(), jcall!(CommonPhoneStrings::getInstance()).FOUR.clone());
        this.FIVE = PCGameKey::new(jcall!(this.keyEventWrapper).VK_5.clone(), jcall!(CommonPhoneStrings::getInstance()).FIVE.clone());
        this.SIX = PCGameKey::new(jcall!(this.keyEventWrapper).VK_6.clone(), jcall!(CommonPhoneStrings::getInstance()).SIX.clone());
        this.SEVEN = PCGameKey::new(jcall!(this.keyEventWrapper).VK_7.clone(), jcall!(CommonPhoneStrings::getInstance()).SEVEN.clone());
        this.EIGHT = PCGameKey::new(jcall!(this.keyEventWrapper).VK_8.clone(), jcall!(CommonPhoneStrings::getInstance()).EIGHT.clone());
        this.NINE = PCGameKey::new(jcall!(this.keyEventWrapper).VK_9.clone(), jcall!(CommonPhoneStrings::getInstance()).NINE.clone());
        this.DPAD_UP = PCGameKey::new(jcall!(this.keyEventWrapper).VK_UP.clone(), jcall!(CommonPhoneStrings::getInstance()).UP.clone());
        this.DPAD_DOWN = PCGameKey::new(jcall!(this.keyEventWrapper).VK_DOWN.clone(), jcall!(CommonPhoneStrings::getInstance()).DOWN.clone());
        this.DPAD_LEFT = PCGameKey::new(jcall!(this.keyEventWrapper).VK_LEFT.clone(), jcall!(CommonPhoneStrings::getInstance()).LEFT.clone());
        this.DPAD_RIGHT = PCGameKey::new(jcall!(this.keyEventWrapper).VK_RIGHT.clone(), jcall!(CommonPhoneStrings::getInstance()).RIGHT.clone());
        this.DPAD_UP2 = this.DPAD_UP.clone();
        this.DPAD_DOWN2 = this.DPAD_DOWN.clone();
        this.DPAD_LEFT2 = this.DPAD_LEFT.clone();
        this.DPAD_RIGHT2 = this.DPAD_RIGHT.clone();
        this.COMMA = PCGameKey::new(jcall!(this.keyEventWrapper).VK_COMMA.clone(), String::from("COMMA"));
        this.SPACE = PCGameKey::new(jcall!(this.keyEventWrapper).VK_SPACE.clone(), String::from("SPACE"));
        this.DEL = PCGameKey::new(jcall!(this.keyEventWrapper).VK_DELETE.clone(), String::from("DEL"));
        this.STAR = PCGameKey::new(42, jcall!(CommonPhoneStrings::getInstance()).STAR.clone());
        this.QUESTION = PCGameKey::new(47, String::from("?"));
        this.PERIOD = PCGameKey::new(jcall!(this.keyEventWrapper).VK_PERIOD.clone(), jcall!(CommonSeps::getInstance()).PERIOD.clone());
        this.ESCAPE = PCGameKey::new(jcall!(this.keyEventWrapper).VK_ESCAPE.clone(), String::from("Esc"));
        this.BACK_SPACE = PCGameKey::new(jcall!(this.keyEventWrapper).VK_BACK_SPACE.clone(), String::from("Backspace"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PCKeyFactory>>> {
        return PCKeyFactory_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn isSubmission(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.ENTER) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isDelete(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.DEL) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isBackSpace(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.BACK_SPACE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isUp(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.DPAD_UP) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&input, &self.DPAD_UP2) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isDown(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.DPAD_DOWN) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&input, &self.DPAD_DOWN2) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isLeft(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.DPAD_LEFT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&input, &self.DPAD_LEFT2) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isRight(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.DPAD_RIGHT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&input, &self.DPAD_RIGHT2) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isEnter(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.ENTER) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
}

