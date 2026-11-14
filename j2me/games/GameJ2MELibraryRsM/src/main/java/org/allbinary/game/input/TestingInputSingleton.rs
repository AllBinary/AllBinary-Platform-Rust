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
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer;
use crate::org::allbinary::graphics::RectangleFactory::RectangleFactory;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::view::ViewPosition::ViewPosition;
use crate::java::lang::Exception::*;
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
impl PaintableInterface for TestingInputSingleton {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::paintThreed(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}
impl crate::org::allbinary::layer::LayerInterface::LayerInterface for TestingInputSingleton {
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getHeight(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getWidth(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getHalfHeight(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getHalfHeight(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getHalfWidth(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getHalfWidth(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getXP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getXP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getYP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getYP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getZP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getZP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getX2(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getX2(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getY2(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getY2(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getZ2(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getZ2(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn isVisible(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::isVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn moveDXYZ(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::moveDXYZ(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
    fn setVisible(&mut self, p1: bool) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
    fn implmentsTickableInterface(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::implmentsTickableInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::implmentsCollidableInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn implmentsGameInputInterface(&mut self) -> bool { Self::implmentsGameInputInterface(self) }
    fn implmentsArtificialIntelligenceCompositeInterface(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::implmentsArtificialIntelligenceCompositeInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getType(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getType(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for TestingInputSingleton {
    fn getName(&mut self) -> String { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getName(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}
impl crate::org::allbinary::layer::PositionInterface::PositionInterface for TestingInputSingleton {
    fn setPosition(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setPosition(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for TestingInputSingleton {
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::view::event::ViewPositionEventListenerInterface::ViewPositionEventListenerInterface for TestingInputSingleton {
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::identification::GroupInterfaceCompositeInterface::GroupInterfaceCompositeInterface for TestingInputSingleton {
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<crate::org::allbinary::game::identification::Group::Group>>>> { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getGroupInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}

impl TestingInputSingleton {
    

    pub fn getInstance() -> Option<Rc<RefCell<TestingInputSingleton>>> {
        return TestingInputSingleton_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AllBinaryGameLayer::new_struct({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone).clone(), ViewPosition::getInstanceD().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>)),
            inputProcessorArray: Vec::new(),
            paintable: None,
        };
        this.inputProcessorArray = vec![None; ({ let java_field = jcall!(InputFactory::InputFactory::getInstance()).MAX.clone(); java_field }) as usize];
        this.paintable = NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn PaintableInterface>>);
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
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_UP.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = GameInputProcessor::new() /* Java anonymous class body: implement it as a named type
            

            // throws Exception
            pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
                jcall!(testInputInterface).up();
            }
*/.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>); };
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = GameInputProcessor::new() /* Java anonymous class body: implement it as a named type
            

            // throws Exception
            pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
                jcall!(testInputInterface).down();
            }
*/.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>); };
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = GameInputProcessor::new() /* Java anonymous class body: implement it as a named type
            

            // throws Exception
            pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
                jcall!(testInputInterface).left();
            }
*/.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>); };
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = GameInputProcessor::new() /* Java anonymous class body: implement it as a named type
            

            // throws Exception
            pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
                jcall!(testInputInterface).right();
            }
*/.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>); };
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM7.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = GameInputProcessor::new() /* Java anonymous class body: implement it as a named type
            

            // throws Exception
            pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
                jcall!(testInputInterface).strafeLeft();
            }
*/.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>); };
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = GameInputProcessor::new() /* Java anonymous class body: implement it as a named type
            

            // throws Exception
            pub fn processEvent(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
                jcall!(testInputInterface).strafeRight();
            }
*/.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>); };
        GameInputProcessorUtil::GameInputProcessorUtil::init(self.inputProcessorArray.clone());
    }
    

    // throws Exception
    // synchronized
    pub fn processInputList(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut key: i32 = 0;
        let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = None;
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            gameKeyEvent = { let java_value = (({ let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameKeyEvent>>>>().expect("ClassCastException").clone()) })).clone(); java_value };
            key = { let java_value = (jcall!(gameKeyEvent).getKey()).clone(); java_value };
            jcall!(self.inputProcessorArray[(key) as usize]).processEvent(allbinaryLayerManager.clone(), crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone());
        }
    }
    

    // throws Exception
    // synchronized
    pub fn processInput(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        { let java_arguments = (allbinaryLayerManager.clone(), self.getGameKeyEventList(),); self.processInputList(java_arguments.0, java_arguments.1) };
        jcall!(self.getGameKeyEventList()).clear();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
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
    fn setPosition(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setPosition(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
    fn moveDXYZ(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::moveDXYZ(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
    fn getXP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getXP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getYP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getYP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getZP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getZP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getWidth(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getHeight(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn setVisible(&mut self, p1: bool) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
    fn isVisible(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::isVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::toStringAppend(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for TestingInputSingleton {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::set(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}


