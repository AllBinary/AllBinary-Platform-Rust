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
use crate::javax::microedition::lcdui::Command::Command;
use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::javax::microedition::lcdui::Form::Form;
use crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener;
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
use crate::java::lang::Exception::*;

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
    pub commandStack: Option<Rc<RefCell<ABStack<crate::JavaObject>>>>,
}

pub static mut CommandForm_NULL_COMMAND_FORM: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());
impl MyCommandInterface for CommandForm {
    fn addCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { Self::addCommand(self, command) }
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
        if match (&unsafe { CommandForm_NULL_COMMAND_FORM.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *CommandForm_NULL_COMMAND_FORM = crate::java_into_object(CommandForm::new(crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener_NULL_COMMAND_LISTENER.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn CommandListener>>), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), { let java_field = jcall!(BasicColorFactory::getInstance()).BLACK.clone(); java_field }.clone(), { let java_field = jcall!(BasicColorFactory::getInstance()).WHITE.clone(); java_field }.clone())) };
        }
        return { let java_object = (unsafe { CommandForm_NULL_COMMAND_FORM.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn CommandFormDyn>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut formTitle: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: Form::new_struct1(formTitle.clone()),
            logUtil: None,
            commonStrings: None,
            repaintProcessor: None,
            commandStack: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        // super(...) is in the struct literal above
        this.commandStack = { let java_value = (jcall!(StdUtil::getInstance()).createStack()).clone(); java_value };
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(this.repaintProcessor).process();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(this.logUtil).put({ let java_field = jcall!(this.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut formTitle: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(commandListener.clone(), formTitle.clone(), backgrounBasicColor.clone(), foregroundBasicColor.clone()))));
        jcall!(this).repaintProcessor = jcall!(ScreenRepaintProcessorFactory::ScreenRepaintProcessorFactory::getInstance()).create(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Displayable::DisplayableDyn>>));
        this
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) {
    }
    

    pub fn open(&mut self) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("open"));
    }
    

    // throws Exception
    pub fn close(&mut self) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CLOSE.clone(); java_field }.clone());
    }
    

    // throws Exception
    pub fn update(&mut self) {
        jcall!(self.repaintProcessor).process();
    }
    

    pub fn getSourceId(&mut self) -> i32 {
        return 0;
    }
    

    pub fn addCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) {
        jcall!(self.commandStack).push(crate::java_into_object(command.clone()));
        { let java_arguments = (command.clone(),); self.base.addCommand(java_arguments.0) };
    }
    

    pub fn removeAllCommands(&mut self) {
        let mut size: i32 = jcall!(self.commandStack).size();
        for index in (0..size) {
            { let java_arguments = ({ let java_object = (jcall!(self.commandStack).pop()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>>().expect("ClassCastException").clone()) },); self.base.removeCommand(java_arguments.0) };
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

pub trait CommandFormDyn: crate::javax::microedition::lcdui::Form::FormDyn + crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface {
    fn as_CommandForm(&self) -> &CommandForm;
    fn as_CommandForm_mut(&mut self) -> &mut CommandForm;
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) { CommandForm::initCommands(self.as_CommandForm_mut(), cmdListener) }
    fn open(&mut self) { CommandForm::open(self.as_CommandForm_mut()) }
    fn close(&mut self) { CommandForm::close(self.as_CommandForm_mut()) }
    fn update(&mut self) { CommandForm::update(self.as_CommandForm_mut()) }
    fn getSourceId(&mut self) -> i32 { CommandForm::getSourceId(self.as_CommandForm_mut()) }
}
impl CommandFormDyn for CommandForm {
    fn as_CommandForm(&self) -> &CommandForm { self }
    fn as_CommandForm_mut(&mut self) -> &mut CommandForm { self }
}

impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for CommandForm {
    fn as_Displayable(&self) -> &crate::javax::microedition::lcdui::Displayable::Displayable { &self.base.base.base }
    fn as_Displayable_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Displayable::Displayable { &mut self.base.base.base }
    fn addCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { Self::addCommand(self, command) }
    fn getWidth(&mut self) -> i32 { crate::javax::microedition::lcdui::Form::Form::getWidth(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::javax::microedition::lcdui::Form::Form::getHeight(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self)) }
    fn hideNotify(&mut self) { crate::javax::microedition::lcdui::Form::Form::hideNotify(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self)) }
    fn keyPressed(&mut self, p1: i32) { crate::javax::microedition::lcdui::Form::Form::keyPressed(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::javax::microedition::lcdui::Screen::Screen::keyRepeated(crate::javax::microedition::lcdui::Screen::ScreenDyn::as_Screen_mut(self), p1) }
    fn paint(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::javax::microedition::lcdui::Screen::Screen::paint(crate::javax::microedition::lcdui::Screen::ScreenDyn::as_Screen_mut(self), p1) }
    fn repaint(&mut self) { crate::javax::microedition::lcdui::Screen::Screen::repaint(crate::javax::microedition::lcdui::Screen::ScreenDyn::as_Screen_mut(self)) }
    fn showNotify(&mut self) { crate::javax::microedition::lcdui::Form::Form::showNotify(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self)) }
}
impl crate::javax::microedition::lcdui::Form::FormDyn for CommandForm {
    fn as_Form(&self) -> &crate::javax::microedition::lcdui::Form::Form { &self.base }
    fn as_Form_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Form::Form { &mut self.base }
}
impl crate::javax::microedition::lcdui::Screen::ScreenDyn for CommandForm {
    fn as_Screen(&self) -> &crate::javax::microedition::lcdui::Screen::Screen { &self.base.base }
    fn as_Screen_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Screen::Screen { &mut self.base.base }
    fn traverse(&mut self, p1: i32, p2: i32, p3: i32) -> i32 { crate::javax::microedition::lcdui::Form::Form::traverse(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self), p1, p2, p3) }
    fn paintContent(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) -> i32 { crate::javax::microedition::lcdui::Form::Form::paintContent(crate::javax::microedition::lcdui::Form::FormDyn::as_Form_mut(self), p1) }
}


