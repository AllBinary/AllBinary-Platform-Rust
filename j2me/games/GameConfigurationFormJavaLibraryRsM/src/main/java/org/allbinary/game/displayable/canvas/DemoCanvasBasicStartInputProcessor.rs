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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::NullCanvas::NullCanvas;
use crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::input::GameInputStrings::GameInputStrings;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::BasicMenuInputProcessor::BasicMenuInputProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DemoCanvasBasicStartInputProcessor {
    pub base: BasicMenuInputProcessor,
}

impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for DemoCanvasBasicStartInputProcessor {
}

impl DemoCanvasBasicStartInputProcessor {
    

    pub fn new_struct(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn>>>) -> Self {
        let mut this = Self {
            base: BasicMenuInputProcessor::new_struct(gameKeyEventList.clone(), -1, gameCanvas.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameKeyEventList, gameCanvas))))
    }
    

    // throws Exception
    pub fn processInputList(&mut self) -> i32 {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getGameKeyEventList();
        let mut size: i32 = jcall!(list).size();
        let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = None;
        for index in (0..size) {
            gameKeyEvent = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameKeyEvent>>>>().expect("ClassCastException").clone()) };
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Start GameKey: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(gameKeyEvent.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(GameInputStrings::getInstance()).PROCESS_INPUT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        jcall!(jcall!(self.getCanvas()).getCustomCommandListener()).commandAction(jcall!(GameCommandsFactory::getInstance()).START_COMMAND.clone(), crate::javax::microedition::lcdui::NullCanvas::NullCanvas_NULL_CANVAS.with(Clone::clone).clone());
        return size.clone();
    }
}
impl std::ops::Deref for DemoCanvasBasicStartInputProcessor {
    type Target = BasicMenuInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DemoCanvasBasicStartInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn for DemoCanvasBasicStartInputProcessor {
    fn as_BasicMenuInputProcessor(&self) -> &crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessor { &self.base }
    fn as_BasicMenuInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessor { &mut self.base }
    fn processInputList(&mut self) -> i32 { Self::processInputList(self) }
}
impl crate::org::allbinary::game::input::GameInput::GameInputDyn for DemoCanvasBasicStartInputProcessor {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base.base }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for DemoCanvasBasicStartInputProcessor {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base.base }
}


