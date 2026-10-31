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
// package org.allbinary.graphics.displayable
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener;
use crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface;
use crate::org::allbinary::graphics::displayable::command::MyCommandsFactory::MyCommandsFactory;
use crate::org::allbinary::input::TouchJ2ME::TouchJ2ME;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::media::audio::Sound::Sound;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABStack::ABStack;
use super::DisplayableInterface::DisplayableInterface;
use super::CanvasStrings::CanvasStrings;
use super::DisplayInfoSingleton::DisplayInfoSingleton;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MyCanvas {
    pub base: Canvas,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub canvasStrings: Option<Rc<RefCell<CanvasStrings>>>,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub name: String,
    pub childNameList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub commandStack: Option<Rc<RefCell<ABStack<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub commandListener: Option<Rc<RefCell<CommandListener>>>,
    pub paused: bool,
    pub touchME: Option<Rc<RefCell<TouchJ2ME>>>,
}

// impl DisplayableInterface for MyCanvas with the methods of the trait
impl MyCommandInterface for MyCanvas {
    fn addCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) { Self::addCommand(self, command) }
    fn removeAllCommands(&mut self) { Self::removeAllCommands(self) }
}

impl MyCanvas {
    

    pub fn new_struct(mut name: String, mut childNameList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            base: Canvas::new_struct(),
            logUtil: None,
            commonStrings: None,
            canvasStrings: None,
            stringUtil: None,
            displayInfo: None,
            name: String::new(),
            childNameList: None,
            commandStack: None,
            commandListener: None,
            paused: false,
            touchME: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.canvasStrings = CanvasStrings::getInstance();
        this.stringUtil = StringUtil::getInstance();
        this.displayInfo = DisplayInfoSingleton::getInstance();
        this.commandListener = crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener_NULL_COMMAND_LISTENER.with(Clone::clone).clone();
        this.touchME = TouchJ2ME::new();
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.name = name.clone();
        this.childNameList = childNameList.clone();
        this.commandStack = jcall!(StdUtil::getInstance()).createStack();
        this
    }

    pub fn new(mut name: String, mut childNameList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(name, childNameList))));
            jcall!(this.displayInfo).update((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(this.canvasStrings).CONSTRUCTOR.clone());
        this
    }
    

    pub fn setFullScreenMode(&mut self, mut mode: bool) {
        self.base.setFullScreenMode(mode.clone());
        jcall!(self.displayInfo).update(None /* Java this: a &mut self method has no Rc of itself */, jcall!(self.canvasStrings).SET_FULL_SCREEN_MODE.clone());
    }
    

    pub fn sizeChanged(&mut self, mut w: i32, mut h: i32) {
        jcall!(self.displayInfo).update(None /* Java this: a &mut self method has no Rc of itself */, jcall!(self.canvasStrings).SIZE_CHANGED.clone());
    }
    

    pub fn getCommandStack(&mut self) -> Option<Rc<RefCell<ABStack<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return self.commandStack.clone();
    }
    

    // synchronized
    pub fn isCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) -> bool {
        return jcall!(self.commandStack).contains(command.clone());
    }
    

    // synchronized
    pub fn addCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) {
        if !jcall!(self.commandStack).contains(command.clone()) {
            jcall!(self.commandStack).push(command.clone());
            self.base.addCommand(command.clone());
        }
    }
    

    // synchronized
    pub fn removeCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) {
        jcall!(self.commandStack).removeElement(command.clone());
        self.base.removeCommand(command.clone());
    }
    

    // synchronized
    pub fn removeAllCommands(&mut self) {
        let mut size: i32 = jcall!(self.commandStack).size();
        for index in (0..size) {
            self.base.removeCommand((jcall!(self.commandStack).pop()).clone() /* as Command */);
        }
    }
    

    pub fn setCommandListener(&mut self, mut l: Option<Rc<RefCell<CommandListener>>>) {
        self.base.setCommandListener(l.clone());
        self.commandListener = l.clone();
    }
    

    pub fn getCustomCommandListener(&mut self) -> Option<Rc<RefCell<CommandListener>>> {
        return self.commandListener.clone();
    }
    

    // synchronized
    pub fn isPaused(&mut self) -> bool {
        return self.paused.clone();
    }
    

    pub fn removePauseCommand(&mut self) {
        { let java_arguments = (jcall!(MyCommandsFactory::getInstance()).PAUSE_COMMAND.clone(),); self.removeCommand(java_arguments.0) };
    }
    

    // synchronized
    pub fn pause(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).PAUSE.clone());
        self.removePauseCommand();
        { let java_arguments = (jcall!(MyCommandsFactory::getInstance()).RESUME_COMMAND.clone(),); self.addCommand(java_arguments.0) };
        self.setPaused(true);
    }
    

    // synchronized
    pub fn unPause(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).UN_PAUSE.clone());
        { let java_arguments = (jcall!(MyCommandsFactory::getInstance()).RESUME_COMMAND.clone(),); self.removeCommand(java_arguments.0) };
        { let java_arguments = (jcall!(MyCommandsFactory::getInstance()).PAUSE_COMMAND.clone(),); self.addCommand(java_arguments.0) };
        self.setPaused(false);
    }
    

    // throws Exception
    pub fn process(&mut self) {
        jcall!(self.displayInfo).process();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn draw(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn hasChild(&mut self, mut displayable: Option<Rc<RefCell<dyn MyCanvasDyn>>>) -> bool {
        return jcall!(self.childNameList).contains(crate::java_into_object(jcall!(displayable).as_MyCanvas_mut().name.clone()));
    }
    

    pub fn destroy(&mut self) {
        jcall!(self.logUtil).putF(String::from("Destroyed MyCanvas"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("destroy"));
    }
    

    pub fn setPaused(&mut self, mut isPaused: bool) {
        self.paused = isPaused.clone();
    }
    

    pub fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
    }
    

    pub fn keyRepeatedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
    }
    

    pub fn keyReleasedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
    }
    

    pub fn pointerDragged(&mut self, mut x: i32, mut y: i32) {
        jcall!(self.touchME).pointerDragged(x.clone(), y.clone());
    }
    

    pub fn pointerPressed(&mut self, mut x: i32, mut y: i32) {
        jcall!(self.touchME).pointerPressed(x.clone(), y.clone());
    }
    

    pub fn pointerReleased(&mut self, mut x: i32, mut y: i32) {
        jcall!(self.touchME).pointerReleased(x.clone(), y.clone());
    }
    

    pub fn nextSong(&mut self, mut nextSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>, mut leftVolume: i32, mut rightVolume: i32) {
    }
}
impl std::ops::Deref for MyCanvas {
    type Target = Canvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MyCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait MyCanvasDyn {
    fn as_MyCanvas(&self) -> &MyCanvas;
    fn as_MyCanvas_mut(&mut self) -> &mut MyCanvas;
    fn setFullScreenMode(&mut self, mut mode: bool) { MyCanvas::setFullScreenMode(self.as_MyCanvas_mut(), mode) }
    fn sizeChanged(&mut self, mut w: i32, mut h: i32) { MyCanvas::sizeChanged(self.as_MyCanvas_mut(), w, h) }
    fn getCommandStack(&mut self) -> Option<Rc<RefCell<ABStack<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { MyCanvas::getCommandStack(self.as_MyCanvas_mut()) }
    fn isCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) -> bool { MyCanvas::isCommand(self.as_MyCanvas_mut(), command) }
    fn addCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) { MyCanvas::addCommand(self.as_MyCanvas_mut(), command) }
    fn removeCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) { MyCanvas::removeCommand(self.as_MyCanvas_mut(), command) }
    fn removeAllCommands(&mut self) { MyCanvas::removeAllCommands(self.as_MyCanvas_mut()) }
    fn setCommandListener(&mut self, mut l: Option<Rc<RefCell<CommandListener>>>) { MyCanvas::setCommandListener(self.as_MyCanvas_mut(), l) }
    fn getCustomCommandListener(&mut self) -> Option<Rc<RefCell<CommandListener>>> { MyCanvas::getCustomCommandListener(self.as_MyCanvas_mut()) }
    fn isPaused(&mut self) -> bool { MyCanvas::isPaused(self.as_MyCanvas_mut()) }
    fn removePauseCommand(&mut self) { MyCanvas::removePauseCommand(self.as_MyCanvas_mut()) }
    fn pause(&mut self) { MyCanvas::pause(self.as_MyCanvas_mut()) }
    fn unPause(&mut self) { MyCanvas::unPause(self.as_MyCanvas_mut()) }
    fn process(&mut self) { MyCanvas::process(self.as_MyCanvas_mut()) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { MyCanvas::paint(self.as_MyCanvas_mut(), graphics) }
    fn draw(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { MyCanvas::draw(self.as_MyCanvas_mut(), graphics) }
    fn hasChild(&mut self, mut displayable: Option<Rc<RefCell<dyn MyCanvasDyn>>>) -> bool { MyCanvas::hasChild(self.as_MyCanvas_mut(), displayable) }
    fn destroy(&mut self) { MyCanvas::destroy(self.as_MyCanvas_mut()) }
    fn setPaused(&mut self, mut isPaused: bool) { MyCanvas::setPaused(self.as_MyCanvas_mut(), isPaused) }
    fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { MyCanvas::keyPressedByDevice(self.as_MyCanvas_mut(), keyCode, deviceId) }
    fn keyRepeatedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { MyCanvas::keyRepeatedByDevice(self.as_MyCanvas_mut(), keyCode, deviceId) }
    fn keyReleasedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { MyCanvas::keyReleasedByDevice(self.as_MyCanvas_mut(), keyCode, deviceId) }
    fn pointerDragged(&mut self, mut x: i32, mut y: i32) { MyCanvas::pointerDragged(self.as_MyCanvas_mut(), x, y) }
    fn pointerPressed(&mut self, mut x: i32, mut y: i32) { MyCanvas::pointerPressed(self.as_MyCanvas_mut(), x, y) }
    fn pointerReleased(&mut self, mut x: i32, mut y: i32) { MyCanvas::pointerReleased(self.as_MyCanvas_mut(), x, y) }
    fn nextSong(&mut self, mut nextSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>, mut leftVolume: i32, mut rightVolume: i32) { MyCanvas::nextSong(self.as_MyCanvas_mut(), nextSongSound, leftVolume, rightVolume) }
}
impl MyCanvasDyn for MyCanvas {
    fn as_MyCanvas(&self) -> &MyCanvas { self }
    fn as_MyCanvas_mut(&mut self) -> &mut MyCanvas { self }
}



