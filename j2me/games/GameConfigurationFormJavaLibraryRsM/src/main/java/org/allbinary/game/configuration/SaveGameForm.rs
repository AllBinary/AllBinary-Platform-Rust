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
use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::javax::microedition::lcdui::TextField::TextField;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm;
use crate::org::allbinary::logic::ABSystemWrapper::ABSystemWrapper;
use crate::org::allbinary::logic::MEUtil::MEUtil;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SaveGameForm {
    pub base: CommandForm,
    pub meUtil: Option<Rc<RefCell<MEUtil>>>,
}

pub static mut SaveGameForm_FORM: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>>> = std::cell::LazyCell::new(|| CommandForm::getNullCommandForm());
impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for SaveGameForm {
    fn open(&mut self) { crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm::open(crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn::as_CommandForm_mut(self)) }
    fn close(&mut self) { crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm::close(crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn::as_CommandForm_mut(self)) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for SaveGameForm {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm::getSourceId(crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn::as_CommandForm_mut(self)) }
}

impl SaveGameForm {
    

    // throws Exception
    pub fn createForm(mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>> {
        unsafe { *SaveGameForm_FORM = SaveGameForm::new(commandListener.clone(), title.clone(), backgrounBasicColor.clone(), foregroundBasicColor.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>) };
        return unsafe { SaveGameForm_FORM.clone() }.clone();
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>> {
        return unsafe { SaveGameForm_FORM.clone() }.clone();
    }
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CommandForm::new_struct(commandListener.clone(), title.clone(), backgrounBasicColor.clone(), foregroundBasicColor.clone()),
            meUtil: None,
        };
        this.meUtil = MEUtil::getInstance();
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF({ let java_field = jcall!(this.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        let mut timeString: String = (jcall!(ABSystemWrapper::getInstance()).currentTimeMillis()).to_string();
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(commandListener.clone(), title.clone(), backgrounBasicColor.clone(), foregroundBasicColor.clone()))));
            let mut timeString: String = (jcall!(ABSystemWrapper::getInstance()).currentTimeMillis()).to_string();
            jcall!(jcall!(this).meUtil).appendItem(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>), TextField::new(String::from("Name: "), timeString.clone(), 30, crate::javax::microedition::lcdui::TextField::TextField_ANY.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::TextField::TextFieldDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Item::ItemDyn>>));
            jcall!(this).initCommands(commandListener.clone());
        this
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) {
        let mut gameCommandsFactory: Option<Rc<RefCell<GameCommandsFactory>>> = GameCommandsFactory::getInstance();
        self.removeAllCommands();
        { let java_arguments = ({ let java_field = jcall!(gameCommandsFactory).SAVE.clone(); java_field }.clone(),); self.addCommand(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameCommandsFactory).CLOSE_AND_SHOW_GAME_CANVAS.clone(); java_field }.clone(),); self.addCommand(java_arguments.0) };
        self.setCommandListener(cmdListener.clone());
    }
}
impl std::ops::Deref for SaveGameForm {
    type Target = CommandForm;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SaveGameForm {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for SaveGameForm {
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
impl crate::javax::microedition::lcdui::Form::FormDyn for SaveGameForm {
    fn as_Form(&self) -> &crate::javax::microedition::lcdui::Form::Form { &self.base.base }
    fn as_Form_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Form::Form { &mut self.base.base }
}
impl crate::javax::microedition::lcdui::Screen::ScreenDyn for SaveGameForm {
    fn as_Screen(&self) -> &crate::javax::microedition::lcdui::Screen::Screen { &self.base.base.base }
    fn as_Screen_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Screen::Screen { &mut self.base.base.base }
    fn traverse(&mut self, p1: i32, p2: i32, p3: i32) -> i32 { crate::javax::microedition::lcdui::Form::Form::traverse(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self), p1, p2, p3) }
    fn paintContent(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) -> i32 { crate::javax::microedition::lcdui::Form::Form::paintContent(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn for SaveGameForm {
    fn as_CommandForm(&self) -> &crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &self.base }
    fn as_CommandForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) { Self::initCommands(self, cmdListener) }
}
impl crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface for SaveGameForm {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm::addCommand(crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn::as_CommandForm_mut(self), p1) }
    fn removeAllCommands(&mut self) { crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm::removeAllCommands(crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn::as_CommandForm_mut(self)) }
}


