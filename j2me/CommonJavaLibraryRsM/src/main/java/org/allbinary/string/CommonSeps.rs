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
// package org.allbinary.string
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsProperty;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommonSeps {
    pub NEW_LINE: String,
    pub SEMICOLON: String,
    pub EQUALS: String,
    pub SPACE: String,
    pub COMMA: String,
    pub COLON: String,
    pub PERIOD: String,
    pub UNDERSCORE: String,
    pub COLON_SEP: String,
    pub COMMA_SEP: String,
    pub PARENTHESIS_OPEN: String,
    pub PARENTHESIS_CLOSE: String,
    pub BRACE_OPEN: String,
    pub BRACE_CLOSE: String,
    pub BRACKET_OPEN: String,
    pub BRACKET_CLOSE: String,
    pub DASH: String,
    pub PERCENT: String,
    pub AMPERSAND: String,
    pub FORWARD_SLASH: String,
    pub BACK_SLASH: String,
    pub QUESTION: String,
    pub QUOTE: String,
    pub QUOTE_END: String,
    pub COMMENT: String,
    pub INDENT: String,
}

thread_local! { pub static CommonSeps_instance: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::new(); }

impl CommonSeps {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NEW_LINE: String::new(),
            SEMICOLON: String::new(),
            EQUALS: String::new(),
            SPACE: String::new(),
            COMMA: String::new(),
            COLON: String::new(),
            PERIOD: String::new(),
            UNDERSCORE: String::new(),
            COLON_SEP: String::new(),
            COMMA_SEP: String::new(),
            PARENTHESIS_OPEN: String::new(),
            PARENTHESIS_CLOSE: String::new(),
            BRACE_OPEN: String::new(),
            BRACE_CLOSE: String::new(),
            BRACKET_OPEN: String::new(),
            BRACKET_CLOSE: String::new(),
            DASH: String::new(),
            PERCENT: String::new(),
            AMPERSAND: String::new(),
            FORWARD_SLASH: String::new(),
            BACK_SLASH: String::new(),
            QUESTION: String::new(),
            QUOTE: String::new(),
            QUOTE_END: String::new(),
            COMMENT: String::new(),
            INDENT: String::new(),
        };
        this.NEW_LINE = String::from("\n");
        this.SEMICOLON = String::from(";");
        this.EQUALS = String::from("=");
        this.SPACE = String::from(" ");
        this.COMMA = String::from(",");
        this.COLON = String::from(":");
        this.PERIOD = String::from(".");
        this.UNDERSCORE = String::from("_");
        this.COLON_SEP = jcall!(StringMaker::new()).append(this.COLON.clone()).append(this.SPACE.clone()).toString();
        this.COMMA_SEP = jcall!(StringMaker::new()).append(this.COMMA.clone()).append(this.SPACE.clone()).toString();
        this.PARENTHESIS_OPEN = String::from("(");
        this.PARENTHESIS_CLOSE = String::from(")");
        this.BRACE_OPEN = String::from("{");
        this.BRACE_CLOSE = String::from("}");
        this.BRACKET_OPEN = String::from("[");
        this.BRACKET_CLOSE = String::from("]");
        this.DASH = String::from("-");
        this.PERCENT = String::from("%");
        this.AMPERSAND = String::from("&");
        this.FORWARD_SLASH = String::from("/");
        this.BACK_SLASH = String::from("\\");
        this.QUESTION = String::from("?");
        this.QUOTE = String::from("\"");
        this.QUOTE_END = String::from("\",");
        this.COMMENT = String::from("//");
        this.INDENT = String::from("    ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CommonSeps>>> {
        return CommonSeps_instance.with(Clone::clone).clone();
    }
}

