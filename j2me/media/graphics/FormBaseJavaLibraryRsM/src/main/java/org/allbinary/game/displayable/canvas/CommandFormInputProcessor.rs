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
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::Command::Command;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::InputFeatureFactory::InputFeatureFactory;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::game::layer::SWTUtil::SWTUtil;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::graphics::form::CommandCurrentSelectionForm::CommandCurrentSelectionForm;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::input::motion::gesture::MotionGestureInput::MotionGestureInput;
use crate::org::allbinary::input::motion::gesture::TouchMotionGestureFactory::TouchMotionGestureFactory;
use crate::org::allbinary::input::motion::gesture::observer::MotionGestureEvent::MotionGestureEvent;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::audio::PrimaryPlayerQueueFactory::PrimaryPlayerQueueFactory;
use crate::org::allbinary::media::audio::SelectSound::SelectSound;
use crate::org::allbinary::thread::PrimaryThreadPool::PrimaryThreadPool;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::BasicMenuInputProcessor::BasicMenuInputProcessor;
use super::CommandRunnable::CommandRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//In general allow scrolling of the menu and selection the center item
pub struct CommandFormInputProcessor {
    pub base: BasicMenuInputProcessor,
    pub MOTION_GESTURE_SOURCE_ID: i32,
    pub CLICK_DELAY: i32,
    pub clickTimeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub DOUBLE_CLICK_DELAY: i32,
    pub doubleClickTimeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub isSingleKeyProcessing: bool,
    pub form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>,
    pub hasPressed: bool,
    pub PROCESS_COMMAND: String,
    pub NAME_LABEL: String,
}

impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for CommandFormInputProcessor {
}

impl CommandFormInputProcessor {
    

    pub fn new_struct(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>, mut form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>) -> Self {
        let mut this = Self {
            base: BasicMenuInputProcessor::new_struct(gameKeyEventList.clone(), playerInputId.clone(), gameCanvas.clone()),
            MOTION_GESTURE_SOURCE_ID: 0,
            CLICK_DELAY: 0,
            clickTimeHelper: None,
            DOUBLE_CLICK_DELAY: 0,
            doubleClickTimeHelper: None,
            isSingleKeyProcessing: false,
            form: None,
            hasPressed: false,
            PROCESS_COMMAND: String::new(),
            NAME_LABEL: String::new(),
        };
        this.MOTION_GESTURE_SOURCE_ID = jcall!(GameKeyEventFactory::getInstance()).MOTION_GESTURE_SOURCE_ID.clone();
        this.CLICK_DELAY = 150;
        this.clickTimeHelper = TimeDelayHelper::new(this.CLICK_DELAY.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.DOUBLE_CLICK_DELAY = 1200;
        this.doubleClickTimeHelper = TimeDelayHelper::new(this.DOUBLE_CLICK_DELAY.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.isSingleKeyProcessing = jcall!(InputFeatureFactory::getInstance()).isSingleKeyProcessing();
        this.hasPressed = false;
        this.PROCESS_COMMAND = String::from("processCommand");
        this.NAME_LABEL = String::from(" ScrollSelectionForm: ");
        // super(...) is in the struct literal above
        this.form = form.clone();
        this
    }

    pub fn new(mut gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>, mut form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameKeyEventList, playerInputId, gameCanvas, form))))
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut key: i32) -> i32 {
        if key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() } || key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() } || key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() } || key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() } {
            jcall!(PrimaryPlayerQueueFactory::getInstance()).add(SelectSound::getInstance());
            jcall!(self.form).processInputKey(key.clone());
            return 1;
        } else if key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_FIRE.clone() } {
            jcall!(PrimaryPlayerQueueFactory::getInstance()).add(SelectSound::getInstance());
            return self.processCommand();
        }
        return 0;
    }
    

    pub fn processCommand(&mut self) -> i32 {
        let mut commandCurrentSelectionForm: Option<Rc<RefCell<CommandCurrentSelectionForm>>> = (self.form).clone() /* as CommandCurrentSelectionForm */;
        let mut command: Option<Rc<RefCell<Command>>> = jcall!(commandCurrentSelectionForm).getSelectedCommand();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(command).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.PROCESS_COMMAND.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>> = OpenGLFeatureFactory::getInstance();
        if unsafe { crate::org::allbinary::game::layer::SWTUtil::SWTUtil_isSWT.clone() } && jcall!(features).isFeature(jcall!(openGLFeatureFactory).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) && !match (&command, &jcall!(GameCommandsFactory::getInstance()).EXIT_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(CommandRunnable::new(None /* Java this: a &mut self method has no Rc of itself */, command.clone())).run();
        } else {
            jcall!(PrimaryThreadPool::getInstance()).runTask(CommandRunnable::new(None /* Java this: a &mut self method has no Rc of itself */, command.clone()));
        }
        if match (&command, &jcall!(GameCommandsFactory::getInstance()).QUIT_COMMAND) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return -1;
        } else {
            return 1;
        }
    }
    

    // throws Exception
    pub fn processInputList(&mut self) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut motionInputsIndex: i32 = self.processMotionInputs();
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getGameKeyEventList();
            let mut size: i32 = jcall!(list).size();
            let mut key: i32 = 0;
            let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = None;
            for index in (0..size) {
                gameKeyEvent = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameKeyEvent>>>>().expect("ClassCastException").clone()) };
                key = jcall!(gameKeyEvent).getKey();
                if jcall!(gameKeyEvent).getSourceId() != self.MOTION_GESTURE_SOURCE_ID {
                    if self.processInput(key.clone()) == 1 {
                        break;
                    }
                }
            }
            self.clear();
            if size > 0 || motionInputsIndex >= 0 {
                return 1;
            } else {
                return -1;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).PROCESS_INPUT.clone(), crate::java_into_object(e.clone()));
            return -1;
            }
        }
    }
    

    // throws Exception
    pub fn processMotionInputs(&mut self) -> i32 {
        let mut lastIndex: i32 = jcall!(self.motionGestureEventList).size() - 1;
        if lastIndex >= 0 {
            let mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>> = (jcall!(self.motionGestureEventList).as_BasicArrayList_mut().objectArray[(lastIndex) as usize]).clone() /* as MotionGestureEvent */;
            self.processMotionInput(motionGestureEvent.clone());
        }
        jcall!(self.motionGestureEventList).clear();
        return lastIndex.clone();
    }
    

    // throws Exception
    pub fn processMotionInput(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) {
        let mut touchMotionGestureFactory: Option<Rc<RefCell<TouchMotionGestureFactory>>> = TouchMotionGestureFactory::getInstance();
        let mut motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>> = jcall!(motionGestureEvent).getMotionGesture();
        if match (&motionGestureInput, &jcall!(touchMotionGestureFactory).RELEASED) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(motionGestureEvent).getCurrentPoint();
            if jcall!(self.form).isInForm(point.clone()) {
                let mut index: i32 = jcall!(self.form).getSelectedIndexForPoint(point.clone());
                if index != -1 {
                    jcall!(PrimaryPlayerQueueFactory::getInstance()).add(SelectSound::getInstance());
                    if index == jcall!(self.form).as_ScrollSelectionForm_mut().getSelectedIndex() {
                        if jcall!(self.clickTimeHelper).isTimeTNT() {
                            self.processCommand();
                        }
                    } else {
                        jcall!(self.form).as_ScrollSelectionForm_mut().setSelectedIndex(index.clone());
                    }
                } else {
                }
            }
            if self.hasPressed {
                if !jcall!(self.doubleClickTimeHelper).isTimeTNT() {
                    jcall!(self.logUtil).putF(String::from("Double Press"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).PROCESS_MOTION_INPUT.clone());
                    self.processCommand();
                }
                jcall!(self.doubleClickTimeHelper).as_TimeDelayHelper_mut().delay = self.DOUBLE_CLICK_DELAY.clone();
                jcall!(self.doubleClickTimeHelper).setStartTimeTNT();
            }
            self.hasPressed = false;
        } else if match (&motionGestureInput, &jcall!(touchMotionGestureFactory).PRESSED) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.doubleClickTimeHelper).as_TimeDelayHelper_mut().delay = 0;
            self.hasPressed = true;
        }
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(self.base.toString()).append(self.NAME_LABEL.clone()).append(jcall!(self.form).as_ScrollSelectionForm_mut().toString()).toString();
    }
}
impl std::ops::Deref for CommandFormInputProcessor {
    type Target = BasicMenuInputProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CommandFormInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CommandFormInputProcessorDyn: crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn {
    fn as_CommandFormInputProcessor(&self) -> &CommandFormInputProcessor;
    fn as_CommandFormInputProcessor_mut(&mut self) -> &mut CommandFormInputProcessor;
    fn processInput(&mut self, mut key: i32) -> i32 { CommandFormInputProcessor::processInput(self.as_CommandFormInputProcessor_mut(), key) }
    fn processCommand(&mut self) -> i32 { CommandFormInputProcessor::processCommand(self.as_CommandFormInputProcessor_mut()) }
    fn processMotionInputs(&mut self) -> i32 { CommandFormInputProcessor::processMotionInputs(self.as_CommandFormInputProcessor_mut()) }
    fn processMotionInput(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) { CommandFormInputProcessor::processMotionInput(self.as_CommandFormInputProcessor_mut(), motionGestureEvent) }
    fn toString(&mut self) -> String { CommandFormInputProcessor::toString(self.as_CommandFormInputProcessor_mut()) }
}
impl CommandFormInputProcessorDyn for CommandFormInputProcessor {
    fn as_CommandFormInputProcessor(&self) -> &CommandFormInputProcessor { self }
    fn as_CommandFormInputProcessor_mut(&mut self) -> &mut CommandFormInputProcessor { self }
}

impl crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn for CommandFormInputProcessor {
    fn as_BasicMenuInputProcessor(&self) -> &crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessor { &self.base }
    fn as_BasicMenuInputProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessor { &mut self.base }
    fn processInputList(&mut self) -> i32 { Self::processInputList(self) }
}
impl crate::org::allbinary::game::input::GameInput::GameInputDyn for CommandFormInputProcessor {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base.base }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for CommandFormInputProcessor {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base.base }
}


