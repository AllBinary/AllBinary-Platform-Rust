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
use crate::javax::microedition::lcdui::Command::Command;
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
    pub TOGGLE_FULLSCREEN: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub TOGGLE_KEYBOARD: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub SET_MENU_DISPLAYABLE: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub APPLICATION_UPDATE: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub DISPLAY_ABOUT: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub OPEN_WEB_URL: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub CLOSE_OPTIONS: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub DISPLAY_OPTIONS: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub START_TRACE: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub STOP_TRACE: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub DEFAULT_OPTIONS: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub SHOW_GAME_CANVAS: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub CLOSE_AND_SHOW_GAME_CANVAS: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub QUIT_COMMAND: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub RESTART_COMMAND: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub DISPLAY_SAVE_FORM: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub SAVE: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub DISPLAY_LOAD_FORM: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub LOAD_FILE: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub DELETE_FILE: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub EXIT_COMMAND: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub EXIT_WITHOUT_PROGRESS_COMMAND: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub START_COMMAND: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub CONTINUE_COMMAND: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub SELECT_COMMAND: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub FEATURE_UPDATE: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub BUY_COMMAND: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
}

pub static mut GameCommandsFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());

impl GameCommandsFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameCommandsFactory>>> {
        if match (&unsafe { GameCommandsFactory_instance.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *GameCommandsFactory_instance = crate::java_into_object(GameCommandsFactory::new()) };
        }
        return { let java_object = (unsafe { GameCommandsFactory_instance.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameCommandsFactory>>>>().expect("ClassCastException").clone()) };
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
        this.TOGGLE_FULLSCREEN = Command::new4(String::from("Toggle FullScreen"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.TOGGLE_KEYBOARD = Command::new4(String::from("Keyboard"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.SET_MENU_DISPLAYABLE = Command::new4(String::from("Set Menu Displayable"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.APPLICATION_UPDATE = Command::new4({ let java_field = jcall!(CommonStrings::getInstance()).UPDATE.clone(); java_field }.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.DISPLAY_ABOUT = Command::new4({ let java_field = jcall!(CanvasStrings::getInstance()).ABOUT.clone(); java_field }.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 2).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.OPEN_WEB_URL = Command::new4(String::from("Open Web URL"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 2).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.CLOSE_OPTIONS = Command::new4(String::from("Back"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.DISPLAY_OPTIONS = Command::new4({ let java_field = jcall!(CanvasStrings::getInstance()).OPTIONS.clone(); java_field }.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 2).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.START_TRACE = Command::new4(String::from("Start Trace"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.STOP_TRACE = Command::new4(String::from("Stop Trace"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.DEFAULT_OPTIONS = Command::new4(String::from("Default"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.SHOW_GAME_CANVAS = Command::new4(String::from("Back to Game"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.CLOSE_AND_SHOW_GAME_CANVAS = Command::new4(String::from("Back"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.QUIT_COMMAND = Command::new4(String::from("Quit"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_STOP.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.RESTART_COMMAND = Command::new4(String::from("Restart"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 0).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.DISPLAY_SAVE_FORM = Command::new4(String::from("Save To"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 3).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.SAVE = Command::new4(String::from("Save"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 3).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.DISPLAY_LOAD_FORM = Command::new4(String::from("Load"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 3).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.LOAD_FILE = Command::new4(String::from("Load File"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 3).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.DELETE_FILE = Command::new4(String::from("Delete File"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.EXIT_COMMAND = Command::new4(String::from("Exit"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_EXIT.with(Clone::clone).clone(), 2).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.EXIT_WITHOUT_PROGRESS_COMMAND = Command::new4(String::from("Exit Without Progress"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_EXIT.with(Clone::clone).clone(), 2).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.START_COMMAND = Command::new4({ let java_field = jcall!(CommonStrings::getInstance()).START.clone(); java_field }.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.CONTINUE_COMMAND = Command::new4(String::from("Continue"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.SELECT_COMMAND = Command::new4(String::from("Select"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.FEATURE_UPDATE = Command::new4(String::from("Feature Update"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.BUY_COMMAND = Command::new4(String::from("BUY"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

