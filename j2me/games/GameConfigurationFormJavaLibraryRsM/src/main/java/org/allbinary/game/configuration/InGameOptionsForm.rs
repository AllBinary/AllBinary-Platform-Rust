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
// package org.allbinary.game.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Choice::Choice;
use crate::javax::microedition::lcdui::Command::Command;
use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::configuration::feature::GameFeatureFormUtil::GameFeatureFormUtil;
use crate::org::allbinary::game::configuration::feature::InGameFeatureChoiceGroups::InGameFeatureChoiceGroups;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InGameOptionsForm {
    pub base: CommandForm,
}

thread_local! { pub static InGameOptionsForm_DISPLAY: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>> = Command::new4(String::from("Options In Game"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>); }
thread_local! { pub static InGameOptionsForm_SAVE: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>> = Command::new4(String::from("Save"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>); }
thread_local! { pub static InGameOptionsForm_DEFAULT: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>> = Command::new4(String::from("Default"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>); }
impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for InGameOptionsForm {
    fn open(&mut self) { crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm::open(crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn::as_CommandForm_mut(self)) }
    fn close(&mut self) { crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm::close(crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn::as_CommandForm_mut(self)) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for InGameOptionsForm {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm::getSourceId(crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn::as_CommandForm_mut(self)) }
}

impl InGameOptionsForm {
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CommandForm::new_struct(commandListener.clone(), title.clone(), backgrounBasicColor.clone(), foregroundBasicColor.clone()),
        };
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF({ let java_field = jcall!(this.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        let mut gameFeatureFormUtil: Option<Rc<RefCell<GameFeatureFormUtil>>> = GameFeatureFormUtil::getInstance();
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(commandListener.clone(), title.clone(), backgrounBasicColor.clone(), foregroundBasicColor.clone()))));
            let mut gameFeatureFormUtil: Option<Rc<RefCell<GameFeatureFormUtil>>> = GameFeatureFormUtil::getInstance();
            jcall!(gameFeatureFormUtil).addChoiceGroup(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>), jcall!(InGameFeatureChoiceGroups::getExclusiveInstance()).get(), crate::javax::microedition::lcdui::Choice::Choice_EXCLUSIVE.with(Clone::clone).clone());
            jcall!(gameFeatureFormUtil).addChoiceGroup(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>), jcall!(InGameFeatureChoiceGroups::getMultipleInstance()).get(), crate::javax::microedition::lcdui::Choice::Choice_MULTIPLE.with(Clone::clone).clone());
            jcall!(this).initCommands(commandListener.clone());
        this
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) {
        self.removeAllCommands();
        { let java_arguments = ({ let java_field = jcall!(GameCommandsFactory::getInstance()).CLOSE_AND_SHOW_GAME_CANVAS.clone(); java_field }.clone(),); self.addCommand(java_arguments.0) };
        { let java_arguments = (InGameOptionsForm_DEFAULT.with(Clone::clone).clone(),); self.addCommand(java_arguments.0) };
        self.setCommandListener(cmdListener.clone());
    }
}
impl std::ops::Deref for InGameOptionsForm {
    type Target = CommandForm;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InGameOptionsForm {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for InGameOptionsForm {
    fn as_Displayable(&self) -> &crate::javax::microedition::lcdui::Displayable::Displayable { &self.base.base.base.base }
    fn as_Displayable_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Displayable::Displayable { &mut self.base.base.base.base }
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm::addCommand(crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn::as_CommandForm_mut(self), p1) }
    fn getWidth(&mut self) -> i32 { crate::javax::microedition::lcdui::Form::Form::getWidth(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::javax::microedition::lcdui::Form::Form::getHeight(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self)) }
    fn hideNotify(&mut self) { crate::javax::microedition::lcdui::Form::Form::hideNotify(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self)) }
    fn keyPressed(&mut self, p1: i32) { crate::javax::microedition::lcdui::Form::Form::keyPressed(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::javax::microedition::lcdui::Screen::Screen::keyRepeated(crate::javax::microedition::lcdui::Screen::ScreenDyn::as_Screen_mut(self), p1) }
    fn paint(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::javax::microedition::lcdui::Screen::Screen::paint(crate::javax::microedition::lcdui::Screen::ScreenDyn::as_Screen_mut(self), p1) }
    fn repaint(&mut self) { crate::javax::microedition::lcdui::Screen::Screen::repaint(crate::javax::microedition::lcdui::Screen::ScreenDyn::as_Screen_mut(self)) }
    fn showNotify(&mut self) { crate::javax::microedition::lcdui::Form::Form::showNotify(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self)) }
}
impl crate::javax::microedition::lcdui::Form::FormDyn for InGameOptionsForm {
    fn as_Form(&self) -> &crate::javax::microedition::lcdui::Form::Form { &self.base.base }
    fn as_Form_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Form::Form { &mut self.base.base }
}
impl crate::javax::microedition::lcdui::Screen::ScreenDyn for InGameOptionsForm {
    fn as_Screen(&self) -> &crate::javax::microedition::lcdui::Screen::Screen { &self.base.base.base }
    fn as_Screen_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Screen::Screen { &mut self.base.base.base }
    fn traverse(&mut self, p1: i32, p2: i32, p3: i32) -> i32 { crate::javax::microedition::lcdui::Form::Form::traverse(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self), p1, p2, p3) }
    fn paintContent(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) -> i32 { crate::javax::microedition::lcdui::Form::Form::paintContent(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn for InGameOptionsForm {
    fn as_CommandForm(&self) -> &crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &self.base }
    fn as_CommandForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) { Self::initCommands(self, cmdListener) }
}
impl crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface for InGameOptionsForm {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm::addCommand(crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn::as_CommandForm_mut(self), p1) }
    fn removeAllCommands(&mut self) { crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm::removeAllCommands(crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn::as_CommandForm_mut(self)) }
}


