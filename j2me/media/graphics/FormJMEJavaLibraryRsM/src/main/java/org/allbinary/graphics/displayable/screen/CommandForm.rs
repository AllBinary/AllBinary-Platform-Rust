// 
// AllBinary Open License Version 1 
// Copyright (c) 2002 AllBinary 
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
// package org.allbinary.graphics.displayable.screen
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Form::Form;
// use crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABStack::ABStack;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommandForm {
    pub base: Form,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub repaintProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub commandStack: Option<Rc<RefCell<ABStack<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
}

pub static mut CommandForm_NULL_COMMAND_FORM: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
impl MyCommandInterface for CommandForm {
    fn addCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) { Self::addCommand(self, command) }
    fn removeAllCommands(&mut self) { Self::removeAllCommands(self) }
}
impl MenuListener for CommandForm {
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for CommandForm {
    fn getSourceId(&mut self) -> i32 { Self::getSourceId(self) }
}

impl CommandForm {
    

    pub fn getNullCommandForm() -> Option<Rc<RefCell<dyn CommandFormDyn>>> {
        if match (&unsafe { CommandForm_NULL_COMMAND_FORM.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *CommandForm_NULL_COMMAND_FORM = CommandForm::new(crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener_NULL_COMMAND_LISTENER.with(Clone::clone).clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(BasicColorFactory::getInstance()).BLACK.clone(), jcall!(BasicColorFactory::getInstance()).WHITE.clone()).map(|reference| reference as Rc<RefCell<dyn CommandFormDyn>>) };
        }
        return (unsafe { CommandForm_NULL_COMMAND_FORM.clone() }).clone() /* as CommandForm */;
    }
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut formTitle: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: Form::new_struct1(formTitle.clone()),
            logUtil: None,
            commonStrings: None,
            repaintProcessor: None,
            commandStack: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.repaintProcessor = jcall!(ScreenRepaintProcessorFactory::ScreenRepaintProcessorFactory::getInstance()).create(this);
        // super(...) is in the struct literal above
        this.commandStack = jcall!(StdUtil::getInstance()).createStack();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(this.repaintProcessor).process();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(this.logUtil).put(jcall!(this.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut formTitle: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(commandListener, formTitle, backgrounBasicColor, foregroundBasicColor))))
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
    }
    

    pub fn open(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("open"));
    }
    

    // throws Exception
    pub fn close(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CLOSE.clone());
    }
    

    // throws Exception
    pub fn update(&mut self) {
        jcall!(self.repaintProcessor).process();
    }
    

    pub fn getSourceId(&mut self) -> i32 {
        return 0;
    }
    

    pub fn addCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) {
        jcall!(self.commandStack).push(command.clone());
        self.base.addCommand(command.clone());
    }
    

    pub fn removeAllCommands(&mut self) {
        let mut size: i32 = jcall!(self.commandStack).size();
        for index in (0..size) {
            self.base.removeCommand((jcall!(self.commandStack).pop()).clone() /* as Command */);
        }
    }
}
impl std::ops::Deref for CommandForm {
    type Target = Form;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CommandForm {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CommandFormDyn {
    fn as_CommandForm(&self) -> &CommandForm;
    fn as_CommandForm_mut(&mut self) -> &mut CommandForm;
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { CommandForm::initCommands(self.as_CommandForm_mut(), cmdListener) }
    fn open(&mut self) { CommandForm::open(self.as_CommandForm_mut()) }
    fn close(&mut self) { CommandForm::close(self.as_CommandForm_mut()) }
    fn update(&mut self) { CommandForm::update(self.as_CommandForm_mut()) }
    fn getSourceId(&mut self) -> i32 { CommandForm::getSourceId(self.as_CommandForm_mut()) }
    fn addCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) { CommandForm::addCommand(self.as_CommandForm_mut(), command) }
    fn removeAllCommands(&mut self) { CommandForm::removeAllCommands(self.as_CommandForm_mut()) }
}
impl CommandFormDyn for CommandForm {
    fn as_CommandForm(&self) -> &CommandForm { self }
    fn as_CommandForm_mut(&mut self) -> &mut CommandForm { self }
}



