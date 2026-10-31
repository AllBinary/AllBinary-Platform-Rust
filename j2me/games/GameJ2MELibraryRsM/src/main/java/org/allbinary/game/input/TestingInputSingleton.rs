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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer;
use crate::org::allbinary::graphics::RectangleFactory::RectangleFactory;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::view::ViewPosition::ViewPosition;
use super::GameInputInterface::GameInputInterface;
use super::GameInputProcessor::GameInputProcessor;
use super::TestInputInterface::TestInputInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TestingInputSingleton {
    pub base: AllBinaryGameLayer,
    pub inputProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>>>,
    pub paintable: Option<Rc<RefCell<dyn PaintableInterface>>>,
}

thread_local! { pub static TestingInputSingleton_instance: Option<Rc<RefCell<TestingInputSingleton>>> = TestingInputSingleton::new(); }
// impl GameInputInterface for TestingInputSingleton with the methods of the trait

impl TestingInputSingleton {
    

    pub fn getInstance() -> Option<Rc<RefCell<TestingInputSingleton>>> {
        return TestingInputSingleton_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AllBinaryGameLayer::new_struct(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone).clone(), ViewPosition::getInstanceD()),
            inputProcessorArray: Vec::new(),
            paintable: None,
        };
        this.inputProcessorArray = vec![None; (jcall!(InputFactory::InputFactory::getInstance()).MAX) as usize];
        this.paintable = NullPaintable::getInstance();
        // super(...) is in the struct literal above
        GameInputProcessorUtil::GameInputProcessorUtil::init(this.inputProcessorArray.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn initInputProcessors(&mut self) {
    }
    

    pub fn initInputProcessorsForTestInput(&mut self, mut aTestInputInterface: Option<Rc<RefCell<dyn TestInputInterface>>>) {
        let mut testInputInterface: Option<Rc<RefCell<dyn TestInputInterface>>> = aTestInputInterface.clone();
        self.paintable = testInputInterface.clone().map(|reference| reference as Rc<RefCell<dyn PaintableInterface>>);
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }) as usize] = GameInputProcessor::new() /* Java anonymous class body: implement it as a named type
            

            // throws Exception
            pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
                jcall!(testInputInterface).up();
            }
*/;
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() }) as usize] = GameInputProcessor::new() /* Java anonymous class body: implement it as a named type
            

            // throws Exception
            pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
                jcall!(testInputInterface).down();
            }
*/;
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }) as usize] = GameInputProcessor::new() /* Java anonymous class body: implement it as a named type
            

            // throws Exception
            pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
                jcall!(testInputInterface).left();
            }
*/;
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }) as usize] = GameInputProcessor::new() /* Java anonymous class body: implement it as a named type
            

            // throws Exception
            pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
                jcall!(testInputInterface).right();
            }
*/;
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM7.clone() }) as usize] = GameInputProcessor::new() /* Java anonymous class body: implement it as a named type
            

            // throws Exception
            pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
                jcall!(testInputInterface).strafeLeft();
            }
*/;
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.clone() }) as usize] = GameInputProcessor::new() /* Java anonymous class body: implement it as a named type
            

            // throws Exception
            pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
                jcall!(testInputInterface).strafeRight();
            }
*/;
        GameInputProcessorUtil::GameInputProcessorUtil::init(self.inputProcessorArray.clone());
    }
    

    // throws Exception
    // synchronized
    pub fn processInputList(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut key: i32 = 0;
        let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = None;
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            gameKeyEvent = ({ let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameKeyEvent>>>>().expect("ClassCastException").clone()) });
            key = jcall!(gameKeyEvent).getKey();
            jcall!(self.inputProcessorArray[(key) as usize]).processEvent(allbinaryLayerManager.clone(), crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone());
        }
    }
    

    // throws Exception
    // synchronized
    pub fn processInput(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        { let java_arguments = (allbinaryLayerManager.clone(), self.getGameKeyEventList(),); self.processInputList(java_arguments.0, java_arguments.1) };
        jcall!(self.getGameKeyEventList()).clear();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.paintable).paint(graphics.clone());
    }
    

    pub fn implmentsGameInputInterface(&mut self) -> bool {
        return true;
    }
}
impl std::ops::Deref for TestingInputSingleton {
    type Target = AllBinaryGameLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TestingInputSingleton {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for TestingInputSingleton {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base }
    fn processInput(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processInput(self, allbinaryLayerManager) }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for TestingInputSingleton {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn implmentsGameInputInterface(&mut self) -> bool { Self::implmentsGameInputInterface(self) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for TestingInputSingleton {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::toStringAppend(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for TestingInputSingleton {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::set(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}


