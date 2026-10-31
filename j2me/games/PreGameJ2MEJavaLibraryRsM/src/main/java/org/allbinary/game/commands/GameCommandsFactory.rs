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
// package org.allbinary.game.commands
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Command::Command;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameCommandsFactory {
    pub TOGGLE_FULLSCREEN: Option<Rc<RefCell<Command>>>,
    pub TOGGLE_KEYBOARD: Option<Rc<RefCell<Command>>>,
    pub SET_MENU_DISPLAYABLE: Option<Rc<RefCell<Command>>>,
    pub APPLICATION_UPDATE: Option<Rc<RefCell<Command>>>,
    pub DISPLAY_ABOUT: Option<Rc<RefCell<Command>>>,
    pub OPEN_WEB_URL: Option<Rc<RefCell<Command>>>,
    pub CLOSE_OPTIONS: Option<Rc<RefCell<Command>>>,
    pub DISPLAY_OPTIONS: Option<Rc<RefCell<Command>>>,
    pub START_TRACE: Option<Rc<RefCell<Command>>>,
    pub STOP_TRACE: Option<Rc<RefCell<Command>>>,
    pub DEFAULT_OPTIONS: Option<Rc<RefCell<Command>>>,
    pub SHOW_GAME_CANVAS: Option<Rc<RefCell<Command>>>,
    pub CLOSE_AND_SHOW_GAME_CANVAS: Option<Rc<RefCell<Command>>>,
    pub QUIT_COMMAND: Option<Rc<RefCell<Command>>>,
    pub RESTART_COMMAND: Option<Rc<RefCell<Command>>>,
    pub DISPLAY_SAVE_FORM: Option<Rc<RefCell<Command>>>,
    pub SAVE: Option<Rc<RefCell<Command>>>,
    pub DISPLAY_LOAD_FORM: Option<Rc<RefCell<Command>>>,
    pub LOAD_FILE: Option<Rc<RefCell<Command>>>,
    pub DELETE_FILE: Option<Rc<RefCell<Command>>>,
    pub EXIT_COMMAND: Option<Rc<RefCell<Command>>>,
    pub EXIT_WITHOUT_PROGRESS_COMMAND: Option<Rc<RefCell<Command>>>,
    pub START_COMMAND: Option<Rc<RefCell<Command>>>,
    pub CONTINUE_COMMAND: Option<Rc<RefCell<Command>>>,
    pub SELECT_COMMAND: Option<Rc<RefCell<Command>>>,
    pub FEATURE_UPDATE: Option<Rc<RefCell<Command>>>,
    pub BUY_COMMAND: Option<Rc<RefCell<Command>>>,
}

pub static mut GameCommandsFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl GameCommandsFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<GameCommandsFactory>>> {
        if match (&unsafe { GameCommandsFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *GameCommandsFactory_instance = GameCommandsFactory::new() };
        }
        return (unsafe { GameCommandsFactory_instance.clone() }).clone() /* as GameCommandsFactory */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            TOGGLE_FULLSCREEN: None,
            TOGGLE_KEYBOARD: None,
            SET_MENU_DISPLAYABLE: None,
            APPLICATION_UPDATE: None,
            DISPLAY_ABOUT: None,
            OPEN_WEB_URL: None,
            CLOSE_OPTIONS: None,
            DISPLAY_OPTIONS: None,
            START_TRACE: None,
            STOP_TRACE: None,
            DEFAULT_OPTIONS: None,
            SHOW_GAME_CANVAS: None,
            CLOSE_AND_SHOW_GAME_CANVAS: None,
            QUIT_COMMAND: None,
            RESTART_COMMAND: None,
            DISPLAY_SAVE_FORM: None,
            SAVE: None,
            DISPLAY_LOAD_FORM: None,
            LOAD_FILE: None,
            DELETE_FILE: None,
            EXIT_COMMAND: None,
            EXIT_WITHOUT_PROGRESS_COMMAND: None,
            START_COMMAND: None,
            CONTINUE_COMMAND: None,
            SELECT_COMMAND: None,
            FEATURE_UPDATE: None,
            BUY_COMMAND: None,
        };
        this.TOGGLE_FULLSCREEN = Command::new(String::from("Toggle FullScreen"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.TOGGLE_KEYBOARD = Command::new(String::from("Keyboard"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.SET_MENU_DISPLAYABLE = Command::new(String::from("Set Menu Displayable"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.APPLICATION_UPDATE = Command::new(jcall!(CommonStrings::getInstance()).UPDATE.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.DISPLAY_ABOUT = Command::new(jcall!(CanvasStrings::getInstance()).ABOUT.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 2);
        this.OPEN_WEB_URL = Command::new(String::from("Open Web URL"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 2);
        this.CLOSE_OPTIONS = Command::new(String::from("Back"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.DISPLAY_OPTIONS = Command::new(jcall!(CanvasStrings::getInstance()).OPTIONS.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 2);
        this.START_TRACE = Command::new(String::from("Start Trace"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.STOP_TRACE = Command::new(String::from("Stop Trace"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.DEFAULT_OPTIONS = Command::new(String::from("Default"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.SHOW_GAME_CANVAS = Command::new(String::from("Back to Game"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.CLOSE_AND_SHOW_GAME_CANVAS = Command::new(String::from("Back"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.QUIT_COMMAND = Command::new(String::from("Quit"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_STOP.clone() }.clone(), 1);
        this.RESTART_COMMAND = Command::new(String::from("Restart"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 0);
        this.DISPLAY_SAVE_FORM = Command::new(String::from("Save To"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 3);
        this.SAVE = Command::new(String::from("Save"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 3);
        this.DISPLAY_LOAD_FORM = Command::new(String::from("Load"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 3);
        this.LOAD_FILE = Command::new(String::from("Load File"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 3);
        this.DELETE_FILE = Command::new(String::from("Delete File"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.EXIT_COMMAND = Command::new(String::from("Exit"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_EXIT.clone() }.clone(), 2);
        this.EXIT_WITHOUT_PROGRESS_COMMAND = Command::new(String::from("Exit Without Progress"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_EXIT.clone() }.clone(), 2);
        this.START_COMMAND = Command::new(jcall!(CommonStrings::getInstance()).START.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.CONTINUE_COMMAND = Command::new(String::from("Continue"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.SELECT_COMMAND = Command::new(String::from("Select"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.FEATURE_UPDATE = Command::new(String::from("Feature Update"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.BUY_COMMAND = Command::new(String::from("BUY"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

