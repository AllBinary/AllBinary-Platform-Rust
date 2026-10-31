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
// use crate::javax::microedition::lcdui::Choice::Choice;
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::configuration::feature::GameFeatureFormUtil::GameFeatureFormUtil;
use crate::org::allbinary::game::configuration::feature::InGameFeatureChoiceGroups::InGameFeatureChoiceGroups;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InGameOptionsForm {
    pub base: CommandForm,
}

thread_local! { pub static InGameOptionsForm_DISPLAY: Option<Rc<RefCell<Command>>> = Command::new(String::from("Options In Game"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1); }
thread_local! { pub static InGameOptionsForm_SAVE: Option<Rc<RefCell<Command>>> = Command::new(String::from("Save"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1); }
thread_local! { pub static InGameOptionsForm_DEFAULT: Option<Rc<RefCell<Command>>> = Command::new(String::from("Default"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1); }
impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for InGameOptionsForm {
    fn open(&mut self) { self.base.open() }
    fn close(&mut self) { self.base.close() }
}

impl InGameOptionsForm {
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CommandForm::new_struct(commandListener.clone(), title.clone(), backgrounBasicColor.clone(), foregroundBasicColor.clone()),
        };
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        let mut gameFeatureFormUtil: Option<Rc<RefCell<GameFeatureFormUtil>>> = GameFeatureFormUtil::getInstance();
        this.initCommands(commandListener.clone());
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(commandListener, title, backgrounBasicColor, foregroundBasicColor))));
            jcall!(gameFeatureFormUtil).addChoiceGroup((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(InGameFeatureChoiceGroups::getExclusiveInstance()).get(), unsafe { crate::javax::microedition::lcdui::Choice::Choice_EXCLUSIVE.clone() }.clone());
            jcall!(gameFeatureFormUtil).addChoiceGroup((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(InGameFeatureChoiceGroups::getMultipleInstance()).get(), unsafe { crate::javax::microedition::lcdui::Choice::Choice_MULTIPLE.clone() }.clone());
        this
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
        self.removeAllCommands();
        { let java_arguments = (jcall!(GameCommandsFactory::getInstance()).CLOSE_AND_SHOW_GAME_CANVAS.clone(),); self.addCommand(java_arguments.0) };
        self.addCommand(InGameOptionsForm_DEFAULT.with(Clone::clone).clone());
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

impl crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn for InGameOptionsForm {
    fn as_CommandForm(&self) -> &crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &self.base }
    fn as_CommandForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { Self::initCommands(self, cmdListener) }
}


