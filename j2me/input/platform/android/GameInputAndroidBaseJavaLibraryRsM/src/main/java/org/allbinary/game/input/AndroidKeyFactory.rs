// Generated Code Do Not Modify
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::view::KeyEvent::KeyEvent;
use crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtil;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::AndroidGameKey::AndroidGameKey;
use super::Input::Input;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidKeyFactory {
    pub base: ActivityFractureUtil,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub UNKNOWN: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_L1: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_R1: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_THUMBR: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_THUMBL: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_START: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_MODE: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_B: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_A: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_X: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_Y: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_L2: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_R2: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_SELECT: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_C: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_Z: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_1: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_10: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_11: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_12: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_13: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_14: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_15: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_16: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_2: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_3: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_4: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_5: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_6: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_7: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_8: Option<Rc<RefCell<AndroidGameKey>>>,
    pub KEYCODE_BUTTON_9: Option<Rc<RefCell<AndroidGameKey>>>,
    pub SOFT_LEFT: Option<Rc<RefCell<AndroidGameKey>>>,
    pub SOFT_RIGHT: Option<Rc<RefCell<AndroidGameKey>>>,
    pub HOME: Option<Rc<RefCell<AndroidGameKey>>>,
    pub BACK: Option<Rc<RefCell<AndroidGameKey>>>,
    pub CALL: Option<Rc<RefCell<AndroidGameKey>>>,
    pub ENDCALL: Option<Rc<RefCell<AndroidGameKey>>>,
    pub ZERO: Option<Rc<RefCell<AndroidGameKey>>>,
    pub ONE: Option<Rc<RefCell<AndroidGameKey>>>,
    pub TWO: Option<Rc<RefCell<AndroidGameKey>>>,
    pub THREE: Option<Rc<RefCell<AndroidGameKey>>>,
    pub FOUR: Option<Rc<RefCell<AndroidGameKey>>>,
    pub FIVE: Option<Rc<RefCell<AndroidGameKey>>>,
    pub SIX: Option<Rc<RefCell<AndroidGameKey>>>,
    pub SEVEN: Option<Rc<RefCell<AndroidGameKey>>>,
    pub EIGHT: Option<Rc<RefCell<AndroidGameKey>>>,
    pub NINE: Option<Rc<RefCell<AndroidGameKey>>>,
    pub STAR: Option<Rc<RefCell<AndroidGameKey>>>,
    pub POUND: Option<Rc<RefCell<AndroidGameKey>>>,
    pub DPAD_UP: Option<Rc<RefCell<AndroidGameKey>>>,
    pub DPAD_DOWN: Option<Rc<RefCell<AndroidGameKey>>>,
    pub DPAD_LEFT: Option<Rc<RefCell<AndroidGameKey>>>,
    pub DPAD_RIGHT: Option<Rc<RefCell<AndroidGameKey>>>,
    pub DPAD_CENTER: Option<Rc<RefCell<AndroidGameKey>>>,
    pub VOLUME_UP: Option<Rc<RefCell<AndroidGameKey>>>,
    pub VOLUME_DOWN: Option<Rc<RefCell<AndroidGameKey>>>,
    pub POWER: Option<Rc<RefCell<AndroidGameKey>>>,
    pub CAMERA: Option<Rc<RefCell<AndroidGameKey>>>,
    pub CLEAR: Option<Rc<RefCell<AndroidGameKey>>>,
    pub A: Option<Rc<RefCell<AndroidGameKey>>>,
    pub B: Option<Rc<RefCell<AndroidGameKey>>>,
    pub C: Option<Rc<RefCell<AndroidGameKey>>>,
    pub D: Option<Rc<RefCell<AndroidGameKey>>>,
    pub E: Option<Rc<RefCell<AndroidGameKey>>>,
    pub F: Option<Rc<RefCell<AndroidGameKey>>>,
    pub G: Option<Rc<RefCell<AndroidGameKey>>>,
    pub H: Option<Rc<RefCell<AndroidGameKey>>>,
    pub I: Option<Rc<RefCell<AndroidGameKey>>>,
    pub J: Option<Rc<RefCell<AndroidGameKey>>>,
    pub K: Option<Rc<RefCell<AndroidGameKey>>>,
    pub L: Option<Rc<RefCell<AndroidGameKey>>>,
    pub M: Option<Rc<RefCell<AndroidGameKey>>>,
    pub N: Option<Rc<RefCell<AndroidGameKey>>>,
    pub O: Option<Rc<RefCell<AndroidGameKey>>>,
    pub P: Option<Rc<RefCell<AndroidGameKey>>>,
    pub Q: Option<Rc<RefCell<AndroidGameKey>>>,
    pub R: Option<Rc<RefCell<AndroidGameKey>>>,
    pub S: Option<Rc<RefCell<AndroidGameKey>>>,
    pub T: Option<Rc<RefCell<AndroidGameKey>>>,
    pub U: Option<Rc<RefCell<AndroidGameKey>>>,
    pub V: Option<Rc<RefCell<AndroidGameKey>>>,
    pub W: Option<Rc<RefCell<AndroidGameKey>>>,
    pub X: Option<Rc<RefCell<AndroidGameKey>>>,
    pub Y: Option<Rc<RefCell<AndroidGameKey>>>,
    pub Z: Option<Rc<RefCell<AndroidGameKey>>>,
    pub COMMA: Option<Rc<RefCell<AndroidGameKey>>>,
    pub PERIOD: Option<Rc<RefCell<AndroidGameKey>>>,
    pub ALT_LEFT: Option<Rc<RefCell<AndroidGameKey>>>,
    pub ALT_RIGHT: Option<Rc<RefCell<AndroidGameKey>>>,
    pub SHIFT_LEFT: Option<Rc<RefCell<AndroidGameKey>>>,
    pub SHIFT_RIGHT: Option<Rc<RefCell<AndroidGameKey>>>,
    pub TAB: Option<Rc<RefCell<AndroidGameKey>>>,
    pub SPACE: Option<Rc<RefCell<AndroidGameKey>>>,
    pub SYM: Option<Rc<RefCell<AndroidGameKey>>>,
    pub EXPLORER: Option<Rc<RefCell<AndroidGameKey>>>,
    pub ENVELOPE: Option<Rc<RefCell<AndroidGameKey>>>,
    pub ENTER: Option<Rc<RefCell<AndroidGameKey>>>,
    pub DEL: Option<Rc<RefCell<AndroidGameKey>>>,
    pub GRAVE: Option<Rc<RefCell<AndroidGameKey>>>,
    pub MINUS: Option<Rc<RefCell<AndroidGameKey>>>,
    pub EQUALS: Option<Rc<RefCell<AndroidGameKey>>>,
    pub LEFT_BRACKET: Option<Rc<RefCell<AndroidGameKey>>>,
    pub RIGHT_BRACKET: Option<Rc<RefCell<AndroidGameKey>>>,
    pub BACKSLASH: Option<Rc<RefCell<AndroidGameKey>>>,
    pub SEMICOLON: Option<Rc<RefCell<AndroidGameKey>>>,
    pub APOSTROPHE: Option<Rc<RefCell<AndroidGameKey>>>,
    pub SLASH: Option<Rc<RefCell<AndroidGameKey>>>,
    pub AT: Option<Rc<RefCell<AndroidGameKey>>>,
    pub NUM: Option<Rc<RefCell<AndroidGameKey>>>,
    pub HEADSETHOOK: Option<Rc<RefCell<AndroidGameKey>>>,
    pub FOCUS: Option<Rc<RefCell<AndroidGameKey>>>,
    pub PLUS: Option<Rc<RefCell<AndroidGameKey>>>,
    pub MENU: Option<Rc<RefCell<AndroidGameKey>>>,
    pub NOTIFICATION: Option<Rc<RefCell<AndroidGameKey>>>,
    pub SEARCH: Option<Rc<RefCell<AndroidGameKey>>>,
}


impl AndroidKeyFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ActivityFractureUtil::new_struct(),
            logUtil: None,
            UNKNOWN: None,
            KEYCODE_BUTTON_L1: None,
            KEYCODE_BUTTON_R1: None,
            KEYCODE_BUTTON_THUMBR: None,
            KEYCODE_BUTTON_THUMBL: None,
            KEYCODE_BUTTON_START: None,
            KEYCODE_BUTTON_MODE: None,
            KEYCODE_BUTTON_B: None,
            KEYCODE_BUTTON_A: None,
            KEYCODE_BUTTON_X: None,
            KEYCODE_BUTTON_Y: None,
            KEYCODE_BUTTON_L2: None,
            KEYCODE_BUTTON_R2: None,
            KEYCODE_BUTTON_SELECT: None,
            KEYCODE_BUTTON_C: None,
            KEYCODE_BUTTON_Z: None,
            KEYCODE_BUTTON_1: None,
            KEYCODE_BUTTON_10: None,
            KEYCODE_BUTTON_11: None,
            KEYCODE_BUTTON_12: None,
            KEYCODE_BUTTON_13: None,
            KEYCODE_BUTTON_14: None,
            KEYCODE_BUTTON_15: None,
            KEYCODE_BUTTON_16: None,
            KEYCODE_BUTTON_2: None,
            KEYCODE_BUTTON_3: None,
            KEYCODE_BUTTON_4: None,
            KEYCODE_BUTTON_5: None,
            KEYCODE_BUTTON_6: None,
            KEYCODE_BUTTON_7: None,
            KEYCODE_BUTTON_8: None,
            KEYCODE_BUTTON_9: None,
            SOFT_LEFT: None,
            SOFT_RIGHT: None,
            HOME: None,
            BACK: None,
            CALL: None,
            ENDCALL: None,
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
            STAR: None,
            POUND: None,
            DPAD_UP: None,
            DPAD_DOWN: None,
            DPAD_LEFT: None,
            DPAD_RIGHT: None,
            DPAD_CENTER: None,
            VOLUME_UP: None,
            VOLUME_DOWN: None,
            POWER: None,
            CAMERA: None,
            CLEAR: None,
            A: None,
            B: None,
            C: None,
            D: None,
            E: None,
            F: None,
            G: None,
            H: None,
            I: None,
            J: None,
            K: None,
            L: None,
            M: None,
            N: None,
            O: None,
            P: None,
            Q: None,
            R: None,
            S: None,
            T: None,
            U: None,
            V: None,
            W: None,
            X: None,
            Y: None,
            Z: None,
            COMMA: None,
            PERIOD: None,
            ALT_LEFT: None,
            ALT_RIGHT: None,
            SHIFT_LEFT: None,
            SHIFT_RIGHT: None,
            TAB: None,
            SPACE: None,
            SYM: None,
            EXPLORER: None,
            ENVELOPE: None,
            ENTER: None,
            DEL: None,
            GRAVE: None,
            MINUS: None,
            EQUALS: None,
            LEFT_BRACKET: None,
            RIGHT_BRACKET: None,
            BACKSLASH: None,
            SEMICOLON: None,
            APOSTROPHE: None,
            SLASH: None,
            AT: None,
            NUM: None,
            HEADSETHOOK: None,
            FOCUS: None,
            PLUS: None,
            MENU: None,
            NOTIFICATION: None,
            SEARCH: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.UNKNOWN = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_UNKNOWN.clone() }.clone(), String::from("UNKNOWN"));
        this.KEYCODE_BUTTON_L1 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_R1 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_THUMBR = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_THUMBL = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_START = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_MODE = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_B = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_A = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_X = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_Y = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_L2 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_R2 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_SELECT = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_C = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_Z = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_1 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_10 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_11 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_12 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_13 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_14 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_15 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_16 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_2 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_3 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_4 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_5 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_6 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_7 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_8 = this.UNKNOWN.clone();
        this.KEYCODE_BUTTON_9 = this.UNKNOWN.clone();
        this.SOFT_LEFT = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_SOFT_LEFT.clone() }.clone(), String::from("SOFT_LEFT"));
        this.SOFT_RIGHT = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_SOFT_RIGHT.clone() }.clone(), String::from("SOFT_RIGHT"));
        this.HOME = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_HOME.clone() }.clone(), String::from("HOME"));
        this.BACK = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BACK.clone() }.clone(), String::from("BACK"));
        this.CALL = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_CALL.clone() }.clone(), String::from("CALL"));
        this.ENDCALL = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_ENDCALL.clone() }.clone(), String::from("ENDCALL"));
        this.ZERO = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_0.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).ZERO.clone());
        this.ONE = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_1.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).ONE.clone());
        this.TWO = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_2.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).TWO.clone());
        this.THREE = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_3.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).THREE.clone());
        this.FOUR = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_4.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).FOUR.clone());
        this.FIVE = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_5.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).FIVE.clone());
        this.SIX = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_6.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).SIX.clone());
        this.SEVEN = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_7.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).SEVEN.clone());
        this.EIGHT = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_8.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).EIGHT.clone());
        this.NINE = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_9.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).NINE.clone());
        this.STAR = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_STAR.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).STAR.clone());
        this.POUND = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_POUND.clone() }.clone(), jcall!(CommonPhoneStrings::getInstance()).POUND.clone());
        this.DPAD_UP = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_DPAD_UP.clone() }.clone(), String::from("UP"));
        this.DPAD_DOWN = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_DPAD_DOWN.clone() }.clone(), String::from("DOWN"));
        this.DPAD_LEFT = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_DPAD_LEFT.clone() }.clone(), String::from("LEFT"));
        this.DPAD_RIGHT = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_DPAD_RIGHT.clone() }.clone(), String::from("RIGHT"));
        this.DPAD_CENTER = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_DPAD_CENTER.clone() }.clone(), String::from("CENTER"));
        this.VOLUME_UP = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_VOLUME_UP.clone() }.clone(), String::from("VOLUME_UP"));
        this.VOLUME_DOWN = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_VOLUME_DOWN.clone() }.clone(), String::from("VOLUME_DOWN"));
        this.POWER = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_POWER.clone() }.clone(), String::from("POWER"));
        this.CAMERA = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_CAMERA.clone() }.clone(), String::from("CAMERA"));
        this.CLEAR = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_CLEAR.clone() }.clone(), String::from("CLEAR"));
        this.A = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_A.clone() }.clone(), String::from("A"));
        this.B = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_B.clone() }.clone(), String::from("B"));
        this.C = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_C.clone() }.clone(), String::from("C"));
        this.D = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_D.clone() }.clone(), String::from("D"));
        this.E = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_E.clone() }.clone(), String::from("E"));
        this.F = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_F.clone() }.clone(), String::from("F"));
        this.G = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_G.clone() }.clone(), String::from("G"));
        this.H = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_H.clone() }.clone(), String::from("H"));
        this.I = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_I.clone() }.clone(), String::from("I"));
        this.J = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_J.clone() }.clone(), String::from("J"));
        this.K = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_K.clone() }.clone(), String::from("K"));
        this.L = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_L.clone() }.clone(), String::from("L"));
        this.M = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_M.clone() }.clone(), String::from("M"));
        this.N = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_N.clone() }.clone(), String::from("N"));
        this.O = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_O.clone() }.clone(), String::from("O"));
        this.P = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_P.clone() }.clone(), String::from("P"));
        this.Q = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_Q.clone() }.clone(), String::from("Q"));
        this.R = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_R.clone() }.clone(), String::from("R"));
        this.S = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_S.clone() }.clone(), String::from("S"));
        this.T = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_T.clone() }.clone(), String::from("T"));
        this.U = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_U.clone() }.clone(), String::from("U"));
        this.V = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_V.clone() }.clone(), String::from("V"));
        this.W = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_W.clone() }.clone(), String::from("W"));
        this.X = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_X.clone() }.clone(), String::from("X"));
        this.Y = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_Y.clone() }.clone(), String::from("Y"));
        this.Z = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_Z.clone() }.clone(), String::from("Z"));
        this.COMMA = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_COMMA.clone() }.clone(), String::from("COMMA"));
        this.PERIOD = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_PERIOD.clone() }.clone(), jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone());
        this.ALT_LEFT = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_ALT_LEFT.clone() }.clone(), String::from("ALT_LEFT"));
        this.ALT_RIGHT = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_ALT_RIGHT.clone() }.clone(), String::from("ALT_RIGHT"));
        this.SHIFT_LEFT = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_SHIFT_LEFT.clone() }.clone(), String::from("SHIFT_LEFT"));
        this.SHIFT_RIGHT = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_SHIFT_RIGHT.clone() }.clone(), String::from("SHIFT_RIGHT"));
        this.TAB = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_TAB.clone() }.clone(), String::from("TAB"));
        this.SPACE = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_SPACE.clone() }.clone(), String::from("SPACE"));
        this.SYM = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_SYM.clone() }.clone(), String::from("SYM"));
        this.EXPLORER = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_EXPLORER.clone() }.clone(), String::from("EXPLORER"));
        this.ENVELOPE = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_ENVELOPE.clone() }.clone(), String::from("ENVELOPE"));
        this.ENTER = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_ENTER.clone() }.clone(), String::from("ENTER"));
        this.DEL = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_DEL.clone() }.clone(), String::from("DEL"));
        this.GRAVE = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_GRAVE.clone() }.clone(), String::from("GRAVE"));
        this.MINUS = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_MINUS.clone() }.clone(), String::from("MINUS"));
        this.EQUALS = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_EQUALS.clone() }.clone(), String::from("EQUALS"));
        this.LEFT_BRACKET = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_LEFT_BRACKET.clone() }.clone(), String::from("LEFT_BRACKET"));
        this.RIGHT_BRACKET = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_RIGHT_BRACKET.clone() }.clone(), String::from("RIGHT_BRACKET"));
        this.BACKSLASH = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_BACKSLASH.clone() }.clone(), String::from("BACKSLASH"));
        this.SEMICOLON = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_SEMICOLON.clone() }.clone(), String::from("SEMICOLON"));
        this.APOSTROPHE = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_APOSTROPHE.clone() }.clone(), String::from("APOSTROPHE"));
        this.SLASH = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_SLASH.clone() }.clone(), String::from("SLASH"));
        this.AT = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_AT.clone() }.clone(), String::from("AT"));
        this.NUM = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_NUM.clone() }.clone(), String::from("NUM"));
        this.HEADSETHOOK = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_HEADSETHOOK.clone() }.clone(), String::from("HEADSETHOOK"));
        this.FOCUS = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_FOCUS.clone() }.clone(), String::from("FOCUS"));
        this.PLUS = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_PLUS.clone() }.clone(), String::from("PLUS"));
        this.MENU = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_MENU.clone() }.clone(), String::from("MENU"));
        this.NOTIFICATION = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_NOTIFICATION.clone() }.clone(), String::from("NOTIFICATION"));
        this.SEARCH = AndroidGameKey::new(unsafe { crate::android::view::KeyEvent::KeyEvent_KEYCODE_SEARCH.clone() }.clone(), String::from("SEARCH"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(self.logUtil).putF(String::from("Start/End"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone());
    }
    

    pub fn isSubmission(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.ENTER) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        } else if match (&input, &self.DPAD_CENTER) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isDelete(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        return false;
    }
    

    pub fn isBackSpace(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.DEL) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isLeft(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.DPAD_LEFT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isRight(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.DPAD_RIGHT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isUp(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.DPAD_UP) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isDown(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.DPAD_DOWN) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isEnter(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &self.DPAD_CENTER) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        } else if match (&input, &self.ENTER) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        } else if match (&input, &self.CALL) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
}
impl std::ops::Deref for AndroidKeyFactory {
    type Target = ActivityFractureUtil;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidKeyFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AndroidKeyFactoryDyn {
    fn as_AndroidKeyFactory(&self) -> &AndroidKeyFactory;
    fn as_AndroidKeyFactory_mut(&mut self) -> &mut AndroidKeyFactory;
    fn init(&mut self) { AndroidKeyFactory::init(self.as_AndroidKeyFactory_mut()) }
    fn isSubmission(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool { AndroidKeyFactory::isSubmission(self.as_AndroidKeyFactory_mut(), input) }
    fn isDelete(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool { AndroidKeyFactory::isDelete(self.as_AndroidKeyFactory_mut(), input) }
    fn isBackSpace(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool { AndroidKeyFactory::isBackSpace(self.as_AndroidKeyFactory_mut(), input) }
    fn isLeft(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool { AndroidKeyFactory::isLeft(self.as_AndroidKeyFactory_mut(), input) }
    fn isRight(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool { AndroidKeyFactory::isRight(self.as_AndroidKeyFactory_mut(), input) }
    fn isUp(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool { AndroidKeyFactory::isUp(self.as_AndroidKeyFactory_mut(), input) }
    fn isDown(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool { AndroidKeyFactory::isDown(self.as_AndroidKeyFactory_mut(), input) }
    fn isEnter(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool { AndroidKeyFactory::isEnter(self.as_AndroidKeyFactory_mut(), input) }
}
impl AndroidKeyFactoryDyn for AndroidKeyFactory {
    fn as_AndroidKeyFactory(&self) -> &AndroidKeyFactory { self }
    fn as_AndroidKeyFactory_mut(&mut self) -> &mut AndroidKeyFactory { self }
}



