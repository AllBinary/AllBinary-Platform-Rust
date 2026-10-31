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
// package org.allbinary.game.score.displayable
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::TextField::TextField;
use crate::org::allbinary::PostGameCommandFactory::PostGameCommandFactory;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::game::score::HighScore::HighScore;
use crate::org::allbinary::game::score::HighScoresHelperBaseInterface::HighScoresHelperBaseInterface;
use crate::org::allbinary::game::score::HighScoresFactoryInterface::HighScoresFactoryInterface;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::graphics::paint::SimpleTextPaintable::SimpleTextPaintable;
use crate::org::allbinary::input::event::VirtualKeyboardEventHandler::VirtualKeyboardEventHandler;
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;
use crate::org::allbinary::logic::system::os::OperatingSystemFactory::OperatingSystemFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::InApplicationPurchaseFactory::InApplicationPurchaseFactory;
use super::HighScoreUtil::HighScoreUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScoreTextBox {
    pub base: CustomTextBox,
    pub virtualKeyboardEventHandler: Option<Rc<RefCell<VirtualKeyboardEventHandler>>>,
    pub highScoreUtil: Option<Rc<RefCell<HighScoreUtil>>>,
    pub pleaseWait: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub paintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub submitted: bool,
}

impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for HighScoreTextBox {
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for HighScoreTextBox {
    fn getSourceId(&mut self) -> i32 { self.base.getSourceId() }
}

impl HighScoreTextBox {
    

    pub fn new_struct(mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut highScoresHelper: Option<Rc<RefCell<dyn HighScoresHelperBaseInterface>>>, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut name: String, mut highScore: Option<Rc<RefCell<HighScore>>>, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CustomTextBox::new_struct(cmdListener.clone(), String::from("New High Score Enter Name:"), name.clone(), 12, unsafe { crate::javax::microedition::lcdui::TextField::TextField_ANY.clone() }.clone(), Font::getDefaultFont(), backgrounBasicColor.clone(), foregroundBasicColor.clone()),
            virtualKeyboardEventHandler: None,
            highScoreUtil: None,
            pleaseWait: None,
            paintable: None,
            submitted: false,
        };
        this.virtualKeyboardEventHandler = VirtualKeyboardEventHandler::getInstance();
        this.pleaseWait = SimpleTextPaintable::new(jcall!(this.commonStrings).PLEASE_WAIT.clone(), jcall!(BasicColorFactory::getInstance()).WHITE.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        this.paintable = NullPaintable::getInstance();
        this.submitted = false;
        // super(...) is in the struct literal above
        this.highScoreUtil = HighScoreUtil::new(highScoresFactoryInterface.clone(), highScoresHelper.clone(), abeClientInformation.clone(), gameInfo.clone(), cmdListener.clone(), name.clone(), highScore.clone());
        let mut operatingSystemInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> = jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance();
        if jcall!(operatingSystemInterface).isOverScan() {
            let mut username: String = jcall!(InApplicationPurchaseFactory::getInstance()).getUserName();
            if true {
                jcall!(this.getTextFieldItem()).setString(username.clone());
            } else {
                jcall!(this.getTextFieldItem()).setString(String::from("NoGamerInfo"));
            }
        }
        this
    }

    pub fn new(mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut highScoresHelper: Option<Rc<RefCell<dyn HighScoresHelperBaseInterface>>>, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut name: String, mut highScore: Option<Rc<RefCell<HighScore>>>, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(highScoresFactoryInterface, highScoresHelper, abeClientInformation, gameInfo, cmdListener, name, highScore, backgrounBasicColor, foregroundBasicColor))))
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
        self.removeAllCommands();
        jcall!(PostGameCommandFactory::getInstance()).addCommands(None /* Java this: a &mut self method has no Rc of itself */);
        self.addCommand(HighScoreUtil_SUBMIT_TEXTBOX_COMMAND.with(Clone::clone).clone());
        self.setCommandListener(cmdListener.clone());
    }
    

    pub fn open(&mut self) {
        jcall!(self.virtualKeyboardEventHandler).open();
        self.paintable = NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        self.base.open();
    }
    

    // throws Exception
    pub fn close(&mut self) {
        jcall!(self.virtualKeyboardEventHandler).close();
        self.paintable = self.pleaseWait.clone();
        jcall!(self.repaintBehavior).onChangeRepaint(None /* Java this: a &mut self method has no Rc of itself */);
        self.base.close();
        self.removeCommand(HighScoreUtil_SUBMIT_TEXTBOX_COMMAND.with(Clone::clone).clone());
        self.update();
        if self.submitted {
            jcall!(self.highScoreUtil).saveHighScore();
        } else {
        }
        self.paintable = NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        jcall!(self.repaintBehavior).onChangeRepaint(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn update(&mut self) {
        let mut name: String = jcall!(self.getTextFieldItem()).getString();
        jcall!(self.highScoreUtil).update(name.clone());
        self.base.update();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.paint(graphics.clone());
        jcall!(self.paintable).paint(graphics.clone());
    }
    

    pub fn saveHighScore(&mut self) {
        jcall!(self.highScoreUtil).saveHighScore();
    }
    

    pub fn submit(&mut self) {
        jcall!(self.highScoreUtil).submit(None /* Java this: a &mut self method has no Rc of itself */);
    }
}
impl std::ops::Deref for HighScoreTextBox {
    type Target = CustomTextBox;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HighScoreTextBox {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn for HighScoreTextBox {
    fn as_GameCommandCanvas(&self) -> &crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &self.base.base }
    fn as_GameCommandCanvas_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &mut self.base.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { Self::initCommands(self, cmdListener) }
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
    fn update(&mut self) { Self::update(self) }
    fn keyPressed(&mut self, p1: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyPressed(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1) }
    fn keyReleased(&mut self, p1: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyReleased(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyRepeated(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1) }
}
impl crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn for HighScoreTextBox {
    fn as_CustomTextBox(&self) -> &crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox { &self.base }
    fn as_CustomTextBox_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox { &mut self.base }
    fn submit(&mut self) { Self::submit(self) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for HighScoreTextBox {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn keyPressedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyPressedByDevice(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1, p2) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::keyReleasedByDevice(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1, p2) }
}
impl crate::org::allbinary::game::input::event::RawKeyEventListener::RawKeyEventListener for HighScoreTextBox {
    fn onEventRaw(&mut self, p1: i32, p2: i32, p3: bool) { crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBox::onEventRaw(crate::org::allbinary::graphics::form::item::CustomTextBox::CustomTextBoxDyn::as_CustomTextBox_mut(self), p1, p2, p3) }
}


