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
use crate::javax::microedition::lcdui::Command::Command;
use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::javax::microedition::lcdui::TextField::TextField;
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
use crate::java::lang::Exception::*;

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

thread_local! { pub static LicenseRegistrationTextBox_DISPLAY_COMMAND: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>> = Command::new4(String::from("Register"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 3).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>); }
thread_local! { pub static LicenseRegistrationTextBox_SUBMIT_COMMAND: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>> = Command::new4(String::from("Submit"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 3).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>); }
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for LicenseRegistrationTextBox {
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::onEvent(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}
impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for LicenseRegistrationTextBox {
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for LicenseRegistrationTextBox {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::getSourceId(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self)) }
}
impl crate::org::allbinary::graphics::displayable::event::DisplayChangeEventListener::DisplayChangeEventListener for LicenseRegistrationTextBox {
    fn onDisplayChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::onDisplayChangeEvent(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}

impl LicenseRegistrationTextBox {
    

    pub fn new_struct(mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CustomTextBox::new_struct(cmdListener.clone(), String::from("Enter Registration Code:"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), 60, crate::javax::microedition::lcdui::TextField::TextField_ANY.with(Clone::clone).clone(), Font::getDefaultFont(), backgrounBasicColor.clone(), foregroundBasicColor.clone()),
            logUtil: None,
            pleaseWaitPaintable: None,
            paintable: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.pleaseWaitPaintable = SimpleTextPaintable::new({ let java_field = jcall!(this.base.commonStrings).PLEASE_WAIT.clone(); java_field }.clone(), { let java_field = jcall!(BasicColorFactory::getInstance()).WHITE.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        this.paintable = NullPaintable::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(cmdListener, backgrounBasicColor, foregroundBasicColor))))
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) {
        self.removeAllCommands();
        { let java_arguments = ({ let java_field = jcall!(GameCommandsFactory::getInstance()).CLOSE_AND_SHOW_GAME_CANVAS.clone(); java_field }.clone(),); self.addCommand(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(GameCommandsFactory::getInstance()).TOGGLE_KEYBOARD.clone(); java_field }.clone(),); self.addCommand(java_arguments.0) };
        self.addCommand(LicenseRegistrationTextBox_SUBMIT_COMMAND.with(Clone::clone).clone());
        self.setCommandListener(cmdListener.clone());
    }
    

    pub fn open(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut virtualKeyboardEventHandler: Option<Rc<RefCell<VirtualKeyboardEventHandler>>> = VirtualKeyboardEventHandler::getInstance();
            jcall!(virtualKeyboardEventHandler).fireEvent({ let java_field = jcall!(virtualKeyboardEventHandler).SHOW_EVENT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.base.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("open"), crate::java_into_object(e.clone()));
            }
        }
        self.base.open();
        self.paintable = { let java_value = (NullPaintable::getInstance()).clone(); java_value };
        self.repaint();
    }
    

    // throws Exception
    pub fn close(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut virtualKeyboardEventHandler: Option<Rc<RefCell<VirtualKeyboardEventHandler>>> = VirtualKeyboardEventHandler::getInstance();
            jcall!(virtualKeyboardEventHandler).fireEvent({ let java_field = jcall!(virtualKeyboardEventHandler).HIDE_EVENT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.base.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("open"), crate::java_into_object(e.clone()));
            }
        }
        self.paintable = self.pleaseWaitPaintable.clone();
        self.repaint();
        self.base.close();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        { let java_arguments = (graphics.clone(),); self.base.paint(java_arguments.0) };
        jcall!(self.paintable).as_Paintable_mut().paint(graphics.clone());
    }
    

    pub fn submit(&mut self) {
        let mut commandListener: Option<Rc<RefCell<dyn CommandListener>>> = self.getCustomCommandListener();
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

impl crate::javax::microedition::lcdui::Canvas::CanvasDyn for LicenseRegistrationTextBox {
    fn as_Canvas(&self) -> &crate::javax::microedition::lcdui::Canvas::Canvas { &self.base.base.base.base }
    fn as_Canvas_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Canvas::Canvas { &mut self.base.base.base.base }
}
impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for LicenseRegistrationTextBox {
    fn as_Displayable(&self) -> &crate::javax::microedition::lcdui::Displayable::Displayable { &self.base.base.base.base.base }
    fn as_Displayable_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Displayable::Displayable { &mut self.base.base.base.base.base }
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn hideNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::hideNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn keyPressed(&mut self, p1: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyPressed(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyRepeated(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1) }
    fn keyReleased(&mut self, p1: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyReleased(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1) }
    fn pointerPressed(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerPressed(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerReleased(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerReleased(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerDragged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerDragged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn repaint(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaint4(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint4(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn sizeChanged2(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::sizeChanged2(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn showNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::showNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn setFullScreenMode(&mut self, p1: bool) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setFullScreenMode(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getTypeAsInt(&mut self) -> i32 { crate::javax::microedition::lcdui::Canvas::Canvas::getTypeAsInt(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaintXYWH(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaintXYWH(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn getDisplayableType(&mut self) -> i32 { crate::javax::microedition::lcdui::Canvas::Canvas::getDisplayableType(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
}
impl crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn for LicenseRegistrationTextBox {
    fn as_CustomTextBox(&self) -> &crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox { &self.base }
    fn as_CustomTextBox_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox { &mut self.base }
    fn submit(&mut self) { Self::submit(self) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for LicenseRegistrationTextBox {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base.base }
    fn keyPressedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyPressedByDevice(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1, p2) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyReleasedByDevice(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1, p2) }
}
impl crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn for LicenseRegistrationTextBox {
    fn as_GameCommandCanvas(&self) -> &crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &self.base.base }
    fn as_GameCommandCanvas_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &mut self.base.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) { Self::initCommands(self, cmdListener) }
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for LicenseRegistrationTextBox {
    fn updateMeasurement(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::updateMeasurement(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::DisplayableInterface::DisplayableInterface for LicenseRegistrationTextBox {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
}
impl crate::org::allbinary::game::input::event::RawKeyEventListener::RawKeyEventListener for LicenseRegistrationTextBox {
    fn onEventRaw(&mut self, p1: i32, p2: i32, p3: bool) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::onEventRaw(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1, p2, p3) }
}
impl crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface for LicenseRegistrationTextBox {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeAllCommands(&mut self) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeAllCommands(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
}


