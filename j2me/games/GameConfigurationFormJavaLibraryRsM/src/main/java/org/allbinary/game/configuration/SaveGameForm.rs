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
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::TextField::TextField;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm;
use crate::org::allbinary::logic::ABSystemWrapper::ABSystemWrapper;
use crate::org::allbinary::logic::MEUtil::MEUtil;

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
    fn open(&mut self) { self.base.open() }
    fn close(&mut self) { self.base.close() }
}

impl SaveGameForm {
    

    // throws Exception
    pub fn createForm(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>> {
        unsafe { *SaveGameForm_FORM = SaveGameForm::new(commandListener.clone(), title.clone(), backgrounBasicColor.clone(), foregroundBasicColor.clone()) };
        return unsafe { SaveGameForm_FORM.clone() }.clone();
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>> {
        return unsafe { SaveGameForm_FORM.clone() }.clone();
    }
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CommandForm::new_struct(commandListener.clone(), title.clone(), backgrounBasicColor.clone(), foregroundBasicColor.clone()),
            meUtil: None,
        };
        this.meUtil = MEUtil::getInstance();
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        let mut timeString: String = (jcall!(ABSystemWrapper::getInstance()).currentTimeMillis()).to_string();
        this.initCommands(commandListener.clone());
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(commandListener, title, backgrounBasicColor, foregroundBasicColor))));
            jcall!(this.meUtil).appendItem((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), TextField::new(String::from("Name: "), timeString.clone(), 30, unsafe { crate::javax::microedition::lcdui::TextField::TextField_ANY.clone() }.clone()));
        this
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
        let mut gameCommandsFactory: Option<Rc<RefCell<GameCommandsFactory>>> = GameCommandsFactory::getInstance();
        self.removeAllCommands();
        self.addCommand(jcall!(gameCommandsFactory).SAVE.clone());
        self.addCommand(jcall!(gameCommandsFactory).CLOSE_AND_SHOW_GAME_CANVAS.clone());
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

impl crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn for SaveGameForm {
    fn as_CommandForm(&self) -> &crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &self.base }
    fn as_CommandForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { Self::initCommands(self, cmdListener) }
}


