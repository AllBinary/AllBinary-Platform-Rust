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
// package org.allbinary.game.displayable.screen
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::TextField::TextField;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::graphics::paint::SimpleTextPaintable::SimpleTextPaintable;
use crate::org::allbinary::input::event::VirtualKeyboardEventHandler::VirtualKeyboardEventHandler;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LicenseRegistrationTextBox {
    pub base: CustomTextBox,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub pleaseWaitPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub paintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
}

thread_local! { pub static LicenseRegistrationTextBox_DISPLAY_COMMAND: Option<Rc<RefCell<Command>>> = Command::new(String::from("Register"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 3); }
thread_local! { pub static LicenseRegistrationTextBox_SUBMIT_COMMAND: Option<Rc<RefCell<Command>>> = Command::new(String::from("Submit"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 3); }
impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for LicenseRegistrationTextBox {
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for LicenseRegistrationTextBox {
    fn getSourceId(&mut self) -> i32 { self.base.getSourceId() }
}

impl LicenseRegistrationTextBox {
    

    pub fn new_struct(mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CustomTextBox::new_struct(cmdListener.clone(), String::from("Enter Registration Code:"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), 60, unsafe { crate::javax::microedition::lcdui::TextField::TextField_ANY.clone() }.clone(), Font::getDefaultFont(), backgrounBasicColor.clone(), foregroundBasicColor.clone()),
            logUtil: None,
            pleaseWaitPaintable: None,
            paintable: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.pleaseWaitPaintable = SimpleTextPaintable::new(jcall!(this.base.commonStrings).PLEASE_WAIT.clone(), jcall!(BasicColorFactory::getInstance()).WHITE.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        this.paintable = NullPaintable::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(cmdListener, backgrounBasicColor, foregroundBasicColor))))
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
        self.removeAllCommands();
        { let java_arguments = (jcall!(GameCommandsFactory::getInstance()).CLOSE_AND_SHOW_GAME_CANVAS.clone(),); self.addCommand(java_arguments.0) };
        { let java_arguments = (jcall!(GameCommandsFactory::getInstance()).TOGGLE_KEYBOARD.clone(),); self.addCommand(java_arguments.0) };
        self.addCommand(LicenseRegistrationTextBox_SUBMIT_COMMAND.with(Clone::clone).clone());
        self.setCommandListener(cmdListener.clone());
    }
    

    pub fn open(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut virtualKeyboardEventHandler: Option<Rc<RefCell<VirtualKeyboardEventHandler>>> = VirtualKeyboardEventHandler::getInstance();
            jcall!(virtualKeyboardEventHandler).fireEvent(jcall!(virtualKeyboardEventHandler).SHOW_EVENT.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.base.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("open"), crate::java_into_object(e.clone()));
            }
        }
        self.base.open();
        self.paintable = NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        self.repaint();
    }
    

    // throws Exception
    pub fn close(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut virtualKeyboardEventHandler: Option<Rc<RefCell<VirtualKeyboardEventHandler>>> = VirtualKeyboardEventHandler::getInstance();
            jcall!(virtualKeyboardEventHandler).fireEvent(jcall!(virtualKeyboardEventHandler).HIDE_EVENT.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.base.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("open"), crate::java_into_object(e.clone()));
            }
        }
        self.paintable = self.pleaseWaitPaintable.clone();
        self.repaint();
        self.base.close();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.paint(graphics.clone());
        jcall!(self.paintable).paint(graphics.clone());
    }
    

    pub fn submit(&mut self) {
        let mut commandListener: Option<Rc<RefCell<CommandListener>>> = self.getCustomCommandListener();
        jcall!(commandListener).commandAction(LicenseRegistrationTextBox_SUBMIT_COMMAND.with(Clone::clone).clone(), None /* Java this: a &mut self method has no Rc of itself */);
    }
}
impl std::ops::Deref for LicenseRegistrationTextBox {
    type Target = CustomTextBox;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LicenseRegistrationTextBox {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn for LicenseRegistrationTextBox {
    fn as_CustomTextBox(&self) -> &crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox { &self.base }
    fn as_CustomTextBox_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox { &mut self.base }
    fn submit(&mut self) { Self::submit(self) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for LicenseRegistrationTextBox {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn keyPressedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyPressedByDevice(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1, p2) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyReleasedByDevice(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1, p2) }
}
impl crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn for LicenseRegistrationTextBox {
    fn as_GameCommandCanvas(&self) -> &crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &self.base.base }
    fn as_GameCommandCanvas_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &mut self.base.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { Self::initCommands(self, cmdListener) }
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
    fn keyPressed(&mut self, p1: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyPressed(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1) }
    fn keyReleased(&mut self, p1: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyReleased(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyRepeated(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1) }
}
impl crate::org::allbinary::game::input::event::RawKeyEventListener::RawKeyEventListener for LicenseRegistrationTextBox {
    fn onEventRaw(&mut self, p1: i32, p2: i32, p3: bool) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::onEventRaw(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1, p2, p3) }
}


