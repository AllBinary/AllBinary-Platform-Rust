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
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::javax::microedition::lcdui::Command::Command;
use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener;
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
use crate::java::lang::Exception::*;
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
    pub commandStack: Option<Rc<RefCell<ABStack<crate::JavaObject>>>>,
    pub commandListener: Option<Rc<RefCell<dyn CommandListener>>>,
    pub paused: bool,
    pub touchME: Option<Rc<RefCell<TouchJ2ME>>>,
}

impl DisplayableInterface for MyCanvas {
    fn addCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { Self::addCommand(self, command) }
    fn getHeight(&mut self) -> i32 { Self::getHeight(self) }
    fn getWidth(&mut self) -> i32 { Self::getWidth(self) }
    fn removeCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { Self::removeCommand(self, command) }
    fn setCommandListener(&mut self, mut l: Option<Rc<RefCell<dyn CommandListener>>>) { Self::setCommandListener(self, l) }
}
impl MyCommandInterface for MyCanvas {
    fn addCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { Self::addCommand(self, command) }
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
        this.commandListener = crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener_NULL_COMMAND_LISTENER.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn CommandListener>>);
        this.touchME = TouchJ2ME::new();
        jcall!(this.logUtil).putF({ let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        this
    }

    pub fn new(mut name: String, mut childNameList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(name.clone(), childNameList.clone()))));
            jcall!(jcall!(this).displayInfo).update(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Displayable::DisplayableDyn>>), { let java_field = jcall!(jcall!(this).canvasStrings).CONSTRUCTOR.clone(); java_field }.clone());
            jcall!(this).name = name.clone();
            jcall!(this).childNameList = childNameList.clone();
            jcall!(this).commandStack = { let java_value = (jcall!(StdUtil::getInstance()).createStack()).clone(); java_value };
        this
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return self.base.getWidth();
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return self.base.getHeight();
    }
    

    pub fn setFullScreenMode(&mut self, mut mode: bool) {
        { let java_arguments = (mode.clone(),); self.base.setFullScreenMode(java_arguments.0) };
        jcall!(self.displayInfo).update(None /* Java this: a &mut self method has no Rc of itself */, { let java_field = jcall!(self.canvasStrings).SET_FULL_SCREEN_MODE.clone(); java_field }.clone());
    }
    

    pub fn sizeChanged(&mut self, mut w: i32, mut h: i32) {
        jcall!(self.displayInfo).update(None /* Java this: a &mut self method has no Rc of itself */, { let java_field = jcall!(self.canvasStrings).SIZE_CHANGED.clone(); java_field }.clone());
    }
    

    pub fn getCommandStack(&mut self) -> Option<Rc<RefCell<ABStack<crate::JavaObject>>>> {
        return self.commandStack.clone();
    }
    

    // synchronized
    pub fn isCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) -> bool {
        return jcall!(self.commandStack).contains(crate::java_into_object(command.clone()));
    }
    

    // synchronized
    pub fn addCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) {
        if !jcall!(self.commandStack).contains(crate::java_into_object(command.clone())) {
            jcall!(self.commandStack).push(crate::java_into_object(command.clone()));
            { let java_arguments = (command.clone(),); self.base.addCommand(java_arguments.0) };
        }
    }
    

    // synchronized
    pub fn removeCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) {
        jcall!(self.commandStack).removeElement(crate::java_into_object(command.clone()));
        { let java_arguments = (command.clone(),); self.base.removeCommand(java_arguments.0) };
    }
    

    // synchronized
    pub fn removeAllCommands(&mut self) {
        let mut size: i32 = jcall!(self.commandStack).size();
        for index in (0..size) {
            { let java_arguments = ({ let java_object = (jcall!(self.commandStack).pop()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>>().expect("ClassCastException").clone()) },); self.base.removeCommand(java_arguments.0) };
        }
    }
    

    pub fn setCommandListener(&mut self, mut l: Option<Rc<RefCell<dyn CommandListener>>>) {
        { let java_arguments = (l.clone(),); self.base.setCommandListener(java_arguments.0) };
        self.commandListener = l.clone();
    }
    

    pub fn getCustomCommandListener(&mut self) -> Option<Rc<RefCell<dyn CommandListener>>> {
        return self.commandListener.clone();
    }
    

    // synchronized
    pub fn isPaused(&mut self) -> bool {
        return self.paused.clone();
    }
    

    pub fn removePauseCommand(&mut self) {
        { let java_arguments = ({ let java_field = jcall!(MyCommandsFactory::getInstance()).PAUSE_COMMAND.clone(); java_field }.clone(),); self.removeCommand(java_arguments.0) };
    }
    

    // synchronized
    pub fn pause(&mut self) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.canvasStrings).PAUSE.clone(); java_field }.clone());
        self.removePauseCommand();
        { let java_arguments = ({ let java_field = jcall!(MyCommandsFactory::getInstance()).RESUME_COMMAND.clone(); java_field }.clone(),); self.addCommand(java_arguments.0) };
        self.setPaused(true);
    }
    

    // synchronized
    pub fn unPause(&mut self) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.canvasStrings).UN_PAUSE.clone(); java_field }.clone());
        { let java_arguments = ({ let java_field = jcall!(MyCommandsFactory::getInstance()).RESUME_COMMAND.clone(); java_field }.clone(),); self.removeCommand(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(MyCommandsFactory::getInstance()).PAUSE_COMMAND.clone(); java_field }.clone(),); self.addCommand(java_arguments.0) };
        self.setPaused(false);
    }
    

    // throws Exception
    pub fn process(&mut self) {
        jcall!(self.displayInfo).process();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
    

    pub fn draw(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
    

    pub fn hasChild(&mut self, mut displayable: Option<Rc<RefCell<dyn MyCanvasDyn>>>) -> bool {
        return jcall!(self.childNameList).contains(crate::java_into_object({ let java_field = jcall!(displayable).as_MyCanvas_mut().name.clone(); java_field }.clone()));
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

pub trait MyCanvasDyn: crate::javax::microedition::lcdui::Canvas::CanvasDyn + crate::org::allbinary::graphics::displayable::DisplayableInterface::DisplayableInterface + crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface {
    fn as_MyCanvas(&self) -> &MyCanvas;
    fn as_MyCanvas_mut(&mut self) -> &mut MyCanvas;
    fn getCommandStack(&mut self) -> Option<Rc<RefCell<ABStack<crate::JavaObject>>>> { MyCanvas::getCommandStack(self.as_MyCanvas_mut()) }
    fn isCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) -> bool { MyCanvas::isCommand(self.as_MyCanvas_mut(), command) }
    fn getCustomCommandListener(&mut self) -> Option<Rc<RefCell<dyn CommandListener>>> { MyCanvas::getCustomCommandListener(self.as_MyCanvas_mut()) }
    fn isPaused(&mut self) -> bool { MyCanvas::isPaused(self.as_MyCanvas_mut()) }
    fn removePauseCommand(&mut self) { MyCanvas::removePauseCommand(self.as_MyCanvas_mut()) }
    fn pause(&mut self) { MyCanvas::pause(self.as_MyCanvas_mut()) }
    fn unPause(&mut self) { MyCanvas::unPause(self.as_MyCanvas_mut()) }
    fn process(&mut self) { MyCanvas::process(self.as_MyCanvas_mut()) }
    fn draw(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { MyCanvas::draw(self.as_MyCanvas_mut(), graphics) }
    fn hasChild(&mut self, mut displayable: Option<Rc<RefCell<dyn MyCanvasDyn>>>) -> bool { MyCanvas::hasChild(self.as_MyCanvas_mut(), displayable) }
    fn destroy(&mut self) { MyCanvas::destroy(self.as_MyCanvas_mut()) }
    fn setPaused(&mut self, mut isPaused: bool) { MyCanvas::setPaused(self.as_MyCanvas_mut(), isPaused) }
    fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { MyCanvas::keyPressedByDevice(self.as_MyCanvas_mut(), keyCode, deviceId) }
    fn keyRepeatedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { MyCanvas::keyRepeatedByDevice(self.as_MyCanvas_mut(), keyCode, deviceId) }
    fn keyReleasedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { MyCanvas::keyReleasedByDevice(self.as_MyCanvas_mut(), keyCode, deviceId) }
    fn nextSong(&mut self, mut nextSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>, mut leftVolume: i32, mut rightVolume: i32) { MyCanvas::nextSong(self.as_MyCanvas_mut(), nextSongSound, leftVolume, rightVolume) }
}
impl MyCanvasDyn for MyCanvas {
    fn as_MyCanvas(&self) -> &MyCanvas { self }
    fn as_MyCanvas_mut(&mut self) -> &mut MyCanvas { self }
}

impl crate::javax::microedition::lcdui::Canvas::CanvasDyn for MyCanvas {
    fn as_Canvas(&self) -> &crate::javax::microedition::lcdui::Canvas::Canvas { &self.base }
    fn as_Canvas_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Canvas::Canvas { &mut self.base }
}
impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for MyCanvas {
    fn as_Displayable(&self) -> &crate::javax::microedition::lcdui::Displayable::Displayable { &self.base.base }
    fn as_Displayable_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Displayable::Displayable { &mut self.base.base }
    fn addCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { Self::addCommand(self, command) }
    fn removeCommand(&mut self, mut command: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { Self::removeCommand(self, command) }
    fn getWidth(&mut self) -> i32 { Self::getWidth(self) }
    fn getHeight(&mut self) -> i32 { Self::getHeight(self) }
    fn setCommandListener(&mut self, mut l: Option<Rc<RefCell<dyn CommandListener>>>) { Self::setCommandListener(self, l) }
    fn setFullScreenMode(&mut self, mut mode: bool) { Self::setFullScreenMode(self, mode) }
    fn hideNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::hideNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn keyPressed(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyPressed(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyRepeated(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn keyReleased(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyReleased(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn pointerPressed(&mut self, mut x: i32, mut y: i32) { Self::pointerPressed(self, x, y) }
    fn pointerReleased(&mut self, mut x: i32, mut y: i32) { Self::pointerReleased(self, x, y) }
    fn pointerDragged(&mut self, mut x: i32, mut y: i32) { Self::pointerDragged(self, x, y) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn repaint(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaintXYWH(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaintXYWH(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn sizeChanged(&mut self, mut w: i32, mut h: i32) { Self::sizeChanged(self, w, h) }
    fn showNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::showNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
}


