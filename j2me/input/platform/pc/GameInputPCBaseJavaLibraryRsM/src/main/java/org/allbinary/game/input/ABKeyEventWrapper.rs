// 
// AllBinary Open License Version 1 
// Copyright (c) 2026 AllBinary 
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
// use crate::java::awt::event::KeyEvent::KeyEvent;
use crate::jsinterop::annotations::JsType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ABKeyEventWrapper {
    pub VK_ENTER: i32,
    pub VK_BACK_SPACE: i32,
    pub VK_TAB: i32,
    pub VK_CANCEL: i32,
    pub VK_CLEAR: i32,
    pub VK_SHIFT: i32,
    pub VK_CONTROL: i32,
    pub VK_ALT: i32,
    pub VK_PAUSE: i32,
    pub VK_CAPS_LOCK: i32,
    pub VK_ESCAPE: i32,
    pub VK_SPACE: i32,
    pub VK_PAGE_UP: i32,
    pub VK_PAGE_DOWN: i32,
    pub VK_END: i32,
    pub VK_HOME: i32,
    pub VK_LEFT: i32,
    pub VK_UP: i32,
    pub VK_RIGHT: i32,
    pub VK_DOWN: i32,
    pub VK_COMMA: i32,
    pub VK_MINUS: i32,
    pub VK_PERIOD: i32,
    pub VK_SLASH: i32,
    pub VK_0: i32,
    pub VK_1: i32,
    pub VK_2: i32,
    pub VK_3: i32,
    pub VK_4: i32,
    pub VK_5: i32,
    pub VK_6: i32,
    pub VK_7: i32,
    pub VK_8: i32,
    pub VK_9: i32,
    pub VK_SEMICOLON: i32,
    pub VK_EQUALS: i32,
    pub VK_A: i32,
    pub VK_B: i32,
    pub VK_C: i32,
    pub VK_D: i32,
    pub VK_E: i32,
    pub VK_F: i32,
    pub VK_G: i32,
    pub VK_H: i32,
    pub VK_I: i32,
    pub VK_J: i32,
    pub VK_K: i32,
    pub VK_L: i32,
    pub VK_M: i32,
    pub VK_N: i32,
    pub VK_O: i32,
    pub VK_P: i32,
    pub VK_Q: i32,
    pub VK_R: i32,
    pub VK_S: i32,
    pub VK_T: i32,
    pub VK_U: i32,
    pub VK_V: i32,
    pub VK_W: i32,
    pub VK_X: i32,
    pub VK_Y: i32,
    pub VK_Z: i32,
    pub VK_OPEN_BRACKET: i32,
    pub VK_BACK_SLASH: i32,
    pub VK_CLOSE_BRACKET: i32,
    pub VK_NUMPAD0: i32,
    pub VK_NUMPAD1: i32,
    pub VK_NUMPAD2: i32,
    pub VK_NUMPAD3: i32,
    pub VK_NUMPAD4: i32,
    pub VK_NUMPAD5: i32,
    pub VK_NUMPAD6: i32,
    pub VK_NUMPAD7: i32,
    pub VK_NUMPAD8: i32,
    pub VK_NUMPAD9: i32,
    pub VK_MULTIPLY: i32,
    pub VK_ADD: i32,
    pub VK_SEPARATER: i32,
    pub VK_SEPARATOR: i32,
    pub VK_SUBTRACT: i32,
    pub VK_DECIMAL: i32,
    pub VK_DIVIDE: i32,
    pub VK_DELETE: i32,
    pub VK_NUM_LOCK: i32,
    pub VK_SCROLL_LOCK: i32,
    pub VK_F1: i32,
    pub VK_F2: i32,
    pub VK_F3: i32,
    pub VK_F4: i32,
    pub VK_F5: i32,
    pub VK_F6: i32,
    pub VK_F7: i32,
    pub VK_F8: i32,
    pub VK_F9: i32,
    pub VK_F10: i32,
    pub VK_F11: i32,
    pub VK_F12: i32,
    pub VK_F13: i32,
    pub VK_F14: i32,
    pub VK_F15: i32,
    pub VK_F16: i32,
    pub VK_F17: i32,
    pub VK_F18: i32,
    pub VK_F19: i32,
    pub VK_F20: i32,
    pub VK_F21: i32,
    pub VK_F22: i32,
    pub VK_F23: i32,
    pub VK_F24: i32,
    pub VK_PRINTSCREEN: i32,
    pub VK_INSERT: i32,
    pub VK_HELP: i32,
    pub VK_META: i32,
    pub VK_BACK_QUOTE: i32,
    pub VK_QUOTE: i32,
    pub VK_KP_UP: i32,
    pub VK_KP_DOWN: i32,
    pub VK_KP_LEFT: i32,
    pub VK_KP_RIGHT: i32,
    pub VK_DEAD_GRAVE: i32,
    pub VK_DEAD_ACUTE: i32,
    pub VK_DEAD_CIRCUMFLEX: i32,
    pub VK_DEAD_TILDE: i32,
    pub VK_DEAD_MACRON: i32,
    pub VK_DEAD_BREVE: i32,
    pub VK_DEAD_ABOVEDOT: i32,
    pub VK_DEAD_DIAERESIS: i32,
    pub VK_DEAD_ABOVERING: i32,
    pub VK_DEAD_DOUBLEACUTE: i32,
    pub VK_DEAD_CARON: i32,
    pub VK_DEAD_CEDILLA: i32,
    pub VK_DEAD_OGONEK: i32,
    pub VK_DEAD_IOTA: i32,
    pub VK_DEAD_VOICED_SOUND: i32,
    pub VK_DEAD_SEMIVOICED_SOUND: i32,
    pub VK_AMPERSAND: i32,
    pub VK_ASTERISK: i32,
    pub VK_QUOTEDBL: i32,
    pub VK_LESS: i32,
    pub VK_GREATER: i32,
    pub VK_BRACELEFT: i32,
    pub VK_BRACERIGHT: i32,
    pub VK_AT: i32,
    pub VK_COLON: i32,
    pub VK_CIRCUMFLEX: i32,
    pub VK_DOLLAR: i32,
    pub VK_EURO_SIGN: i32,
    pub VK_EXCLAMATION_MARK: i32,
    pub VK_INVERTED_EXCLAMATION_MARK: i32,
    pub VK_LEFT_PARENTHESIS: i32,
    pub VK_NUMBER_SIGN: i32,
    pub VK_PLUS: i32,
    pub VK_RIGHT_PARENTHESIS: i32,
    pub VK_UNDERSCORE: i32,
    pub VK_WINDOWS: i32,
    pub VK_CONTEXT_MENU: i32,
    pub VK_FINAL: i32,
    pub VK_CONVERT: i32,
    pub VK_NONCONVERT: i32,
    pub VK_ACCEPT: i32,
    pub VK_MODECHANGE: i32,
    pub VK_KANA: i32,
    pub VK_KANJI: i32,
    pub VK_ALPHANUMERIC: i32,
    pub VK_KATAKANA: i32,
    pub VK_HIRAGANA: i32,
    pub VK_FULL_WIDTH: i32,
    pub VK_HALF_WIDTH: i32,
    pub VK_ROMAN_CHARACTERS: i32,
    pub VK_ALL_CANDIDATES: i32,
    pub VK_PREVIOUS_CANDIDATE: i32,
    pub VK_CODE_INPUT: i32,
    pub VK_JAPANESE_KATAKANA: i32,
    pub VK_JAPANESE_HIRAGANA: i32,
    pub VK_JAPANESE_ROMAN: i32,
    pub VK_KANA_LOCK: i32,
    pub VK_INPUT_METHOD_ON_OFF: i32,
    pub VK_CUT: i32,
    pub VK_COPY: i32,
    pub VK_PASTE: i32,
    pub VK_UNDO: i32,
    pub VK_AGAIN: i32,
    pub VK_FIND: i32,
    pub VK_PROPS: i32,
    pub VK_STOP: i32,
    pub VK_COMPOSE: i32,
    pub VK_ALT_GRAPH: i32,
    pub VK_BEGIN: i32,
    pub VK_UNDEFINED: i32,
}

thread_local! { pub static ABKeyEventWrapper_instance: Option<Rc<RefCell<ABKeyEventWrapper>>> = ABKeyEventWrapper::new(); }

impl ABKeyEventWrapper {

    pub fn new_struct() -> Self {
        let mut this = Self {
            VK_ENTER: 0,
            VK_BACK_SPACE: 0,
            VK_TAB: 0,
            VK_CANCEL: 0,
            VK_CLEAR: 0,
            VK_SHIFT: 0,
            VK_CONTROL: 0,
            VK_ALT: 0,
            VK_PAUSE: 0,
            VK_CAPS_LOCK: 0,
            VK_ESCAPE: 0,
            VK_SPACE: 0,
            VK_PAGE_UP: 0,
            VK_PAGE_DOWN: 0,
            VK_END: 0,
            VK_HOME: 0,
            VK_LEFT: 0,
            VK_UP: 0,
            VK_RIGHT: 0,
            VK_DOWN: 0,
            VK_COMMA: 0,
            VK_MINUS: 0,
            VK_PERIOD: 0,
            VK_SLASH: 0,
            VK_0: 0,
            VK_1: 0,
            VK_2: 0,
            VK_3: 0,
            VK_4: 0,
            VK_5: 0,
            VK_6: 0,
            VK_7: 0,
            VK_8: 0,
            VK_9: 0,
            VK_SEMICOLON: 0,
            VK_EQUALS: 0,
            VK_A: 0,
            VK_B: 0,
            VK_C: 0,
            VK_D: 0,
            VK_E: 0,
            VK_F: 0,
            VK_G: 0,
            VK_H: 0,
            VK_I: 0,
            VK_J: 0,
            VK_K: 0,
            VK_L: 0,
            VK_M: 0,
            VK_N: 0,
            VK_O: 0,
            VK_P: 0,
            VK_Q: 0,
            VK_R: 0,
            VK_S: 0,
            VK_T: 0,
            VK_U: 0,
            VK_V: 0,
            VK_W: 0,
            VK_X: 0,
            VK_Y: 0,
            VK_Z: 0,
            VK_OPEN_BRACKET: 0,
            VK_BACK_SLASH: 0,
            VK_CLOSE_BRACKET: 0,
            VK_NUMPAD0: 0,
            VK_NUMPAD1: 0,
            VK_NUMPAD2: 0,
            VK_NUMPAD3: 0,
            VK_NUMPAD4: 0,
            VK_NUMPAD5: 0,
            VK_NUMPAD6: 0,
            VK_NUMPAD7: 0,
            VK_NUMPAD8: 0,
            VK_NUMPAD9: 0,
            VK_MULTIPLY: 0,
            VK_ADD: 0,
            VK_SEPARATER: 0,
            VK_SEPARATOR: 0,
            VK_SUBTRACT: 0,
            VK_DECIMAL: 0,
            VK_DIVIDE: 0,
            VK_DELETE: 0,
            VK_NUM_LOCK: 0,
            VK_SCROLL_LOCK: 0,
            VK_F1: 0,
            VK_F2: 0,
            VK_F3: 0,
            VK_F4: 0,
            VK_F5: 0,
            VK_F6: 0,
            VK_F7: 0,
            VK_F8: 0,
            VK_F9: 0,
            VK_F10: 0,
            VK_F11: 0,
            VK_F12: 0,
            VK_F13: 0,
            VK_F14: 0,
            VK_F15: 0,
            VK_F16: 0,
            VK_F17: 0,
            VK_F18: 0,
            VK_F19: 0,
            VK_F20: 0,
            VK_F21: 0,
            VK_F22: 0,
            VK_F23: 0,
            VK_F24: 0,
            VK_PRINTSCREEN: 0,
            VK_INSERT: 0,
            VK_HELP: 0,
            VK_META: 0,
            VK_BACK_QUOTE: 0,
            VK_QUOTE: 0,
            VK_KP_UP: 0,
            VK_KP_DOWN: 0,
            VK_KP_LEFT: 0,
            VK_KP_RIGHT: 0,
            VK_DEAD_GRAVE: 0,
            VK_DEAD_ACUTE: 0,
            VK_DEAD_CIRCUMFLEX: 0,
            VK_DEAD_TILDE: 0,
            VK_DEAD_MACRON: 0,
            VK_DEAD_BREVE: 0,
            VK_DEAD_ABOVEDOT: 0,
            VK_DEAD_DIAERESIS: 0,
            VK_DEAD_ABOVERING: 0,
            VK_DEAD_DOUBLEACUTE: 0,
            VK_DEAD_CARON: 0,
            VK_DEAD_CEDILLA: 0,
            VK_DEAD_OGONEK: 0,
            VK_DEAD_IOTA: 0,
            VK_DEAD_VOICED_SOUND: 0,
            VK_DEAD_SEMIVOICED_SOUND: 0,
            VK_AMPERSAND: 0,
            VK_ASTERISK: 0,
            VK_QUOTEDBL: 0,
            VK_LESS: 0,
            VK_GREATER: 0,
            VK_BRACELEFT: 0,
            VK_BRACERIGHT: 0,
            VK_AT: 0,
            VK_COLON: 0,
            VK_CIRCUMFLEX: 0,
            VK_DOLLAR: 0,
            VK_EURO_SIGN: 0,
            VK_EXCLAMATION_MARK: 0,
            VK_INVERTED_EXCLAMATION_MARK: 0,
            VK_LEFT_PARENTHESIS: 0,
            VK_NUMBER_SIGN: 0,
            VK_PLUS: 0,
            VK_RIGHT_PARENTHESIS: 0,
            VK_UNDERSCORE: 0,
            VK_WINDOWS: 0,
            VK_CONTEXT_MENU: 0,
            VK_FINAL: 0,
            VK_CONVERT: 0,
            VK_NONCONVERT: 0,
            VK_ACCEPT: 0,
            VK_MODECHANGE: 0,
            VK_KANA: 0,
            VK_KANJI: 0,
            VK_ALPHANUMERIC: 0,
            VK_KATAKANA: 0,
            VK_HIRAGANA: 0,
            VK_FULL_WIDTH: 0,
            VK_HALF_WIDTH: 0,
            VK_ROMAN_CHARACTERS: 0,
            VK_ALL_CANDIDATES: 0,
            VK_PREVIOUS_CANDIDATE: 0,
            VK_CODE_INPUT: 0,
            VK_JAPANESE_KATAKANA: 0,
            VK_JAPANESE_HIRAGANA: 0,
            VK_JAPANESE_ROMAN: 0,
            VK_KANA_LOCK: 0,
            VK_INPUT_METHOD_ON_OFF: 0,
            VK_CUT: 0,
            VK_COPY: 0,
            VK_PASTE: 0,
            VK_UNDO: 0,
            VK_AGAIN: 0,
            VK_FIND: 0,
            VK_PROPS: 0,
            VK_STOP: 0,
            VK_COMPOSE: 0,
            VK_ALT_GRAPH: 0,
            VK_BEGIN: 0,
            VK_UNDEFINED: 0,
        };
        this.VK_ENTER = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_ENTER.clone() }.clone();
        this.VK_BACK_SPACE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_BACK_SPACE.clone() }.clone();
        this.VK_TAB = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_TAB.clone() }.clone();
        this.VK_CANCEL = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_CANCEL.clone() }.clone();
        this.VK_CLEAR = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_CLEAR.clone() }.clone();
        this.VK_SHIFT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_SHIFT.clone() }.clone();
        this.VK_CONTROL = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_CONTROL.clone() }.clone();
        this.VK_ALT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_ALT.clone() }.clone();
        this.VK_PAUSE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_PAUSE.clone() }.clone();
        this.VK_CAPS_LOCK = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_CAPS_LOCK.clone() }.clone();
        this.VK_ESCAPE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_ESCAPE.clone() }.clone();
        this.VK_SPACE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_SPACE.clone() }.clone();
        this.VK_PAGE_UP = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_PAGE_UP.clone() }.clone();
        this.VK_PAGE_DOWN = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_PAGE_DOWN.clone() }.clone();
        this.VK_END = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_END.clone() }.clone();
        this.VK_HOME = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_HOME.clone() }.clone();
        this.VK_LEFT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_LEFT.clone() }.clone();
        this.VK_UP = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_UP.clone() }.clone();
        this.VK_RIGHT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_RIGHT.clone() }.clone();
        this.VK_DOWN = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DOWN.clone() }.clone();
        this.VK_COMMA = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_COMMA.clone() }.clone();
        this.VK_MINUS = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_MINUS.clone() }.clone();
        this.VK_PERIOD = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_PERIOD.clone() }.clone();
        this.VK_SLASH = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_SLASH.clone() }.clone();
        this.VK_0 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_0.clone() }.clone();
        this.VK_1 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_1.clone() }.clone();
        this.VK_2 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_2.clone() }.clone();
        this.VK_3 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_3.clone() }.clone();
        this.VK_4 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_4.clone() }.clone();
        this.VK_5 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_5.clone() }.clone();
        this.VK_6 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_6.clone() }.clone();
        this.VK_7 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_7.clone() }.clone();
        this.VK_8 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_8.clone() }.clone();
        this.VK_9 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_9.clone() }.clone();
        this.VK_SEMICOLON = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_SEMICOLON.clone() }.clone();
        this.VK_EQUALS = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_EQUALS.clone() }.clone();
        this.VK_A = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_A.clone() }.clone();
        this.VK_B = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_B.clone() }.clone();
        this.VK_C = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_C.clone() }.clone();
        this.VK_D = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_D.clone() }.clone();
        this.VK_E = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_E.clone() }.clone();
        this.VK_F = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F.clone() }.clone();
        this.VK_G = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_G.clone() }.clone();
        this.VK_H = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_H.clone() }.clone();
        this.VK_I = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_I.clone() }.clone();
        this.VK_J = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_J.clone() }.clone();
        this.VK_K = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_K.clone() }.clone();
        this.VK_L = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_L.clone() }.clone();
        this.VK_M = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_M.clone() }.clone();
        this.VK_N = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_N.clone() }.clone();
        this.VK_O = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_O.clone() }.clone();
        this.VK_P = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_P.clone() }.clone();
        this.VK_Q = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_Q.clone() }.clone();
        this.VK_R = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_R.clone() }.clone();
        this.VK_S = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_S.clone() }.clone();
        this.VK_T = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_T.clone() }.clone();
        this.VK_U = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_U.clone() }.clone();
        this.VK_V = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_V.clone() }.clone();
        this.VK_W = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_W.clone() }.clone();
        this.VK_X = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_X.clone() }.clone();
        this.VK_Y = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_Y.clone() }.clone();
        this.VK_Z = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_Z.clone() }.clone();
        this.VK_OPEN_BRACKET = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_OPEN_BRACKET.clone() }.clone();
        this.VK_BACK_SLASH = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_BACK_SLASH.clone() }.clone();
        this.VK_CLOSE_BRACKET = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_CLOSE_BRACKET.clone() }.clone();
        this.VK_NUMPAD0 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD0.clone() }.clone();
        this.VK_NUMPAD1 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD1.clone() }.clone();
        this.VK_NUMPAD2 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD2.clone() }.clone();
        this.VK_NUMPAD3 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD3.clone() }.clone();
        this.VK_NUMPAD4 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD4.clone() }.clone();
        this.VK_NUMPAD5 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD5.clone() }.clone();
        this.VK_NUMPAD6 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD6.clone() }.clone();
        this.VK_NUMPAD7 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD7.clone() }.clone();
        this.VK_NUMPAD8 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD8.clone() }.clone();
        this.VK_NUMPAD9 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMPAD9.clone() }.clone();
        this.VK_MULTIPLY = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_MULTIPLY.clone() }.clone();
        this.VK_ADD = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_ADD.clone() }.clone();
        this.VK_SEPARATER = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_SEPARATER.clone() }.clone();
        this.VK_SEPARATOR = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_SEPARATOR.clone() }.clone();
        this.VK_SUBTRACT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_SUBTRACT.clone() }.clone();
        this.VK_DECIMAL = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DECIMAL.clone() }.clone();
        this.VK_DIVIDE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DIVIDE.clone() }.clone();
        this.VK_DELETE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DELETE.clone() }.clone();
        this.VK_NUM_LOCK = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUM_LOCK.clone() }.clone();
        this.VK_SCROLL_LOCK = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_SCROLL_LOCK.clone() }.clone();
        this.VK_F1 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F1.clone() }.clone();
        this.VK_F2 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F2.clone() }.clone();
        this.VK_F3 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F3.clone() }.clone();
        this.VK_F4 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F4.clone() }.clone();
        this.VK_F5 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F5.clone() }.clone();
        this.VK_F6 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F6.clone() }.clone();
        this.VK_F7 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F7.clone() }.clone();
        this.VK_F8 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F8.clone() }.clone();
        this.VK_F9 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F9.clone() }.clone();
        this.VK_F10 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F10.clone() }.clone();
        this.VK_F11 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F11.clone() }.clone();
        this.VK_F12 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F12.clone() }.clone();
        this.VK_F13 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F13.clone() }.clone();
        this.VK_F14 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F14.clone() }.clone();
        this.VK_F15 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F15.clone() }.clone();
        this.VK_F16 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F16.clone() }.clone();
        this.VK_F17 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F17.clone() }.clone();
        this.VK_F18 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F18.clone() }.clone();
        this.VK_F19 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F19.clone() }.clone();
        this.VK_F20 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F20.clone() }.clone();
        this.VK_F21 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F21.clone() }.clone();
        this.VK_F22 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F22.clone() }.clone();
        this.VK_F23 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F23.clone() }.clone();
        this.VK_F24 = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_F24.clone() }.clone();
        this.VK_PRINTSCREEN = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_PRINTSCREEN.clone() }.clone();
        this.VK_INSERT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_INSERT.clone() }.clone();
        this.VK_HELP = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_HELP.clone() }.clone();
        this.VK_META = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_META.clone() }.clone();
        this.VK_BACK_QUOTE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_BACK_QUOTE.clone() }.clone();
        this.VK_QUOTE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_QUOTE.clone() }.clone();
        this.VK_KP_UP = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_KP_UP.clone() }.clone();
        this.VK_KP_DOWN = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_KP_DOWN.clone() }.clone();
        this.VK_KP_LEFT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_KP_LEFT.clone() }.clone();
        this.VK_KP_RIGHT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_KP_RIGHT.clone() }.clone();
        this.VK_DEAD_GRAVE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_GRAVE.clone() }.clone();
        this.VK_DEAD_ACUTE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_ACUTE.clone() }.clone();
        this.VK_DEAD_CIRCUMFLEX = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_CIRCUMFLEX.clone() }.clone();
        this.VK_DEAD_TILDE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_TILDE.clone() }.clone();
        this.VK_DEAD_MACRON = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_MACRON.clone() }.clone();
        this.VK_DEAD_BREVE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_BREVE.clone() }.clone();
        this.VK_DEAD_ABOVEDOT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_ABOVEDOT.clone() }.clone();
        this.VK_DEAD_DIAERESIS = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_DIAERESIS.clone() }.clone();
        this.VK_DEAD_ABOVERING = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_ABOVERING.clone() }.clone();
        this.VK_DEAD_DOUBLEACUTE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_DOUBLEACUTE.clone() }.clone();
        this.VK_DEAD_CARON = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_CARON.clone() }.clone();
        this.VK_DEAD_CEDILLA = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_CEDILLA.clone() }.clone();
        this.VK_DEAD_OGONEK = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_OGONEK.clone() }.clone();
        this.VK_DEAD_IOTA = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_IOTA.clone() }.clone();
        this.VK_DEAD_VOICED_SOUND = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_VOICED_SOUND.clone() }.clone();
        this.VK_DEAD_SEMIVOICED_SOUND = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DEAD_SEMIVOICED_SOUND.clone() }.clone();
        this.VK_AMPERSAND = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_AMPERSAND.clone() }.clone();
        this.VK_ASTERISK = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_ASTERISK.clone() }.clone();
        this.VK_QUOTEDBL = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_QUOTEDBL.clone() }.clone();
        this.VK_LESS = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_LESS.clone() }.clone();
        this.VK_GREATER = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_GREATER.clone() }.clone();
        this.VK_BRACELEFT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_BRACELEFT.clone() }.clone();
        this.VK_BRACERIGHT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_BRACERIGHT.clone() }.clone();
        this.VK_AT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_AT.clone() }.clone();
        this.VK_COLON = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_COLON.clone() }.clone();
        this.VK_CIRCUMFLEX = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_CIRCUMFLEX.clone() }.clone();
        this.VK_DOLLAR = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_DOLLAR.clone() }.clone();
        this.VK_EURO_SIGN = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_EURO_SIGN.clone() }.clone();
        this.VK_EXCLAMATION_MARK = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_EXCLAMATION_MARK.clone() }.clone();
        this.VK_INVERTED_EXCLAMATION_MARK = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_INVERTED_EXCLAMATION_MARK.clone() }.clone();
        this.VK_LEFT_PARENTHESIS = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_LEFT_PARENTHESIS.clone() }.clone();
        this.VK_NUMBER_SIGN = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NUMBER_SIGN.clone() }.clone();
        this.VK_PLUS = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_PLUS.clone() }.clone();
        this.VK_RIGHT_PARENTHESIS = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_RIGHT_PARENTHESIS.clone() }.clone();
        this.VK_UNDERSCORE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_UNDERSCORE.clone() }.clone();
        this.VK_WINDOWS = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_WINDOWS.clone() }.clone();
        this.VK_CONTEXT_MENU = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_CONTEXT_MENU.clone() }.clone();
        this.VK_FINAL = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_FINAL.clone() }.clone();
        this.VK_CONVERT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_CONVERT.clone() }.clone();
        this.VK_NONCONVERT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_NONCONVERT.clone() }.clone();
        this.VK_ACCEPT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_ACCEPT.clone() }.clone();
        this.VK_MODECHANGE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_MODECHANGE.clone() }.clone();
        this.VK_KANA = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_KANA.clone() }.clone();
        this.VK_KANJI = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_KANJI.clone() }.clone();
        this.VK_ALPHANUMERIC = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_ALPHANUMERIC.clone() }.clone();
        this.VK_KATAKANA = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_KATAKANA.clone() }.clone();
        this.VK_HIRAGANA = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_HIRAGANA.clone() }.clone();
        this.VK_FULL_WIDTH = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_FULL_WIDTH.clone() }.clone();
        this.VK_HALF_WIDTH = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_HALF_WIDTH.clone() }.clone();
        this.VK_ROMAN_CHARACTERS = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_ROMAN_CHARACTERS.clone() }.clone();
        this.VK_ALL_CANDIDATES = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_ALL_CANDIDATES.clone() }.clone();
        this.VK_PREVIOUS_CANDIDATE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_PREVIOUS_CANDIDATE.clone() }.clone();
        this.VK_CODE_INPUT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_CODE_INPUT.clone() }.clone();
        this.VK_JAPANESE_KATAKANA = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_JAPANESE_KATAKANA.clone() }.clone();
        this.VK_JAPANESE_HIRAGANA = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_JAPANESE_HIRAGANA.clone() }.clone();
        this.VK_JAPANESE_ROMAN = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_JAPANESE_ROMAN.clone() }.clone();
        this.VK_KANA_LOCK = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_KANA_LOCK.clone() }.clone();
        this.VK_INPUT_METHOD_ON_OFF = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_INPUT_METHOD_ON_OFF.clone() }.clone();
        this.VK_CUT = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_CUT.clone() }.clone();
        this.VK_COPY = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_COPY.clone() }.clone();
        this.VK_PASTE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_PASTE.clone() }.clone();
        this.VK_UNDO = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_UNDO.clone() }.clone();
        this.VK_AGAIN = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_AGAIN.clone() }.clone();
        this.VK_FIND = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_FIND.clone() }.clone();
        this.VK_PROPS = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_PROPS.clone() }.clone();
        this.VK_STOP = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_STOP.clone() }.clone();
        this.VK_COMPOSE = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_COMPOSE.clone() }.clone();
        this.VK_ALT_GRAPH = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_ALT_GRAPH.clone() }.clone();
        this.VK_BEGIN = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_BEGIN.clone() }.clone();
        this.VK_UNDEFINED = unsafe { crate::java::awt::event::KeyEvent::KeyEvent_VK_UNDEFINED.clone() }.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ABKeyEventWrapper>>> {
        return ABKeyEventWrapper_instance.with(Clone::clone).clone();
    }
}

