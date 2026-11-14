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
// package org.allbinary.graphics.canvas.transition.progress
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::javax::microedition::lcdui::NullCanvas::NullCanvas;
use crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::form::item::ABCustomGaugeItem::ABCustomGaugeItem;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet;
use crate::org::allbinary::thread::PathFindingThreadPool::PathFindingThreadPool;
use crate::org::allbinary::thread::ThreadPool::ThreadPool;
use crate::java::lang::Exception::*;
use super::ProgressPaintable::ProgressPaintable;
use super::ProgressEndListener::ProgressEndListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ProgressCanvas {
    pub base: RunnableCanvas,
    pub hasPainted: bool,
    pub backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub GAUGE_PAINTABLE: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub maxValue: f32,
    pub gauge: Option<Rc<RefCell<ABCustomGaugeItem>>>,
    pub TEXT: String,
    pub updateMyFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub allbinaryMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn>>>,
    pub value: f32,
    pub text: String,
    pub background: bool,
    pub paintable: Option<Rc<RefCell<dyn PaintableInterface>>>,
    pub inProgress: bool,
    pub IN_GAME_PROCESSOR: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub inGameProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub progressEndListener: Option<Rc<RefCell<ProgressEndListener>>>,
    pub backgroundLabel: String,
    pub START_BACKGROUND: String,
    pub ADD_PORTION: String,
    pub ADD_EARLY_PORTION: String,
}

impl PaintableInterface for ProgressCanvas {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paintThreed(self, graphics) }
}
impl UpdateMyFontInterface for ProgressCanvas {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::thread::RunnableInterface::RunnableInterface for ProgressCanvas {
    fn isRunning(&mut self) -> bool { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::isRunning(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self)) }
    fn setRunning(&mut self, p1: bool) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::setRunning(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self), p1) }
    fn setThread(&mut self, p1: Option<Rc<RefCell<crate::java::lang::Thread::Thread>>>) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::setThread(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self), p1) }
}
impl crate::Runnable for ProgressCanvas {
    fn run(&mut self) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::run(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self)) }
}

impl ProgressCanvas {
    

    pub fn new_struct(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: RunnableCanvas::new_struct(crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener_NULL_COMMAND_LISTENER.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn CommandListener>>), { let java_field = jcall!(CanvasStrings::getInstance()).EMPTY_CHILD_NAME_LIST.clone(); java_field }.clone(), false),
            hasPainted: false,
            backgroundBasicColor: None,
            GAUGE_PAINTABLE: None,
            maxValue: 0.0_f32,
            gauge: None,
            TEXT: String::new(),
            updateMyFontProcessor: None,
            myFontProcessor: None,
            allbinaryMidlet: None,
            value: 0.0_f32,
            text: String::new(),
            background: false,
            paintable: None,
            inProgress: false,
            IN_GAME_PROCESSOR: None,
            inGameProcessor: None,
            progressEndListener: None,
            backgroundLabel: String::new(),
            START_BACKGROUND: String::new(),
            ADD_PORTION: String::new(),
            ADD_EARLY_PORTION: String::new(),
        };
        this.maxValue = 100.0_f32;
        this.TEXT = { let java_field = jcall!(this.commonStrings).LOADING.clone(); java_field }.clone();
        this.myFontProcessor = this.updateMyFontProcessor.clone();
        this.allbinaryMidlet = AllBinaryMidlet::getNullInstance();
        this.text = this.TEXT.clone();
        this.background = true;
        this.paintable = this.GAUGE_PAINTABLE.clone().map(|reference| reference as Rc<RefCell<dyn PaintableInterface>>);
        this.inProgress = false;
        this.IN_GAME_PROCESSOR = Processor::new() /* Java anonymous class body: implement it as a named type
        pathFindingThreadPool: PathFindingThreadPool::getInstance(),
        

        // throws Exception
        pub fn process(&mut self) {
            jcall!(this.pathFindingThreadPool).runAPriorityTask();
        }
*/.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>);
        this.inGameProcessor = Processor::getInstance();
        this.progressEndListener = ProgressEndListener_instance.with(Clone::clone).clone();
        this.backgroundLabel = String::from("Background AI Game Loading...");
        this.START_BACKGROUND = String::from("startBackground");
        this.ADD_PORTION = String::from("addPortion");
        this.ADD_EARLY_PORTION = String::from("addEarlyPortion");
        // super(...) is in the struct literal above
        this.backgroundBasicColor = backgroundBasicColor.clone();
        this.gauge = { let java_value = (ABCustomGaugeItem::new({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), ((this.maxValue) as i32), 0, backgroundBasicColor.clone(), foregroundBasicColor.clone())).clone(); java_value };
        this
    }

    pub fn new(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(title.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()))));
        jcall!(this).GAUGE_PAINTABLE = ProgressPaintable::new(this.clone().map(|reference| reference as Rc<RefCell<dyn ProgressCanvasDyn>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        jcall!(this).updateMyFontProcessor = UpdateMyFontProcessor::new(this.clone().map(|reference| reference as Rc<RefCell<dyn UpdateMyFontInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        { let java_receiver = self.gauge.clone(); let java_arguments = (jcall!(font).getHeight() + 2,); jcall!(java_receiver).setHeight(java_arguments.0); };
        self.myFontProcessor = { let java_value = (MyFontProcessor::getInstance()).clone(); java_value };
    }
    

    pub fn init(&mut self, mut gameMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn>>>) {
        self.allbinaryMidlet = gameMidlet.clone();
    }
    

    // throws Exception
    pub fn update(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) {
    }
    

    pub fn getMaxValue(&mut self) -> f32 {
        return self.maxValue.clone();
    }
    

    pub fn start(&mut self) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).START_METHOD_NAME.clone(); java_field }.clone());
        self.setBackground(true);
        jcall!(self.gauge).setHeight(30);
        jcall!(self.gauge).setLabel({ let java_field = jcall!(self.commonStrings).PLEASE_WAIT.clone(); java_field }.clone());
        { let java_arguments = (self.TEXT.clone(),); self.setText(java_arguments.0) };
        self.setValue(0);
        self.inGameProcessor = { let java_value = (Processor::getInstance()).clone(); java_value };
        self.paintable = self.GAUGE_PAINTABLE.clone().map(|reference| reference as Rc<RefCell<dyn PaintableInterface>>);
        self.inProgress = true;
    }
    

    pub fn startBackground(&mut self, mut background: bool) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.START_BACKGROUND.clone());
        self.setBackground(background.clone());
        self.myFontProcessor = self.updateMyFontProcessor.clone();
        jcall!(self.gauge).setLabel(self.backgroundLabel.clone());
        { let java_arguments = (self.TEXT.clone(),); self.setText(java_arguments.0) };
        self.setValue(0);
        self.inGameProcessor = { let java_value = (Processor::getInstance()).clone(); java_value };
        self.paintable = self.GAUGE_PAINTABLE.clone().map(|reference| reference as Rc<RefCell<dyn PaintableInterface>>);
    }
    

    pub fn endActual(&mut self) {
        { let java_receiver = self.allbinaryMidlet.clone(); let java_arguments = ({ let java_field = jcall!(GameCommandsFactory::getInstance()).SHOW_GAME_CANVAS.clone(); java_field }.clone(), crate::javax::microedition::lcdui::NullCanvas::NullCanvas_NULL_CANVAS.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Displayable::DisplayableDyn>>),); jcall!(java_receiver).as_AllBinaryMidlet_mut().commandAction(java_arguments.0, java_arguments.1); };
        self.inProgress = false;
        self.inGame();
    }
    

    pub fn inGame(&mut self) {
        jcall!(self.progressEndListener).onEndEvent();
        self.inGameProcessor = self.IN_GAME_PROCESSOR.clone();
    }
    

    pub fn end(&mut self) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).END_METHOD_NAME.clone(); java_field }.clone());
        { let java_receiver = self.gauge.clone(); let java_arguments = (self.getMaxValue(),); jcall!(java_receiver).setValue(java_arguments.0); };
        self.endActual();
        self.paintable = { let java_value = (NullPaintable::getInstance()).clone(); java_value }.map(|reference| reference as Rc<RefCell<dyn PaintableInterface>>);
    }
    

    pub fn endFromInitialLazyLoadingComplete(&mut self) {
        jcall!(self.progressEndListener).onEndEvent();
        { let java_receiver = self.gauge.clone(); let java_arguments = (self.getMaxValue(),); jcall!(java_receiver).setValue(java_arguments.0); };
        self.inGameProcessor = self.IN_GAME_PROCESSOR.clone();
    }
    

    pub fn isInGame(&mut self) -> bool {
        if match (&self.inGameProcessor, &self.IN_GAME_PROCESSOR) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn endIfPaintedSinceStart(&mut self) {
    }
    

    pub fn addEarlyPortion(&mut self, mut value: i32, mut text: String, mut index: i32) {
        { let java_arguments = (jcall!(StringMaker::new()).append(text.clone()).append((jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(index.clone())).expect("NullPointerException").to_string()).toString(),); self.setText(java_arguments.0) };
        { let java_receiver = self.gauge.clone(); let java_arguments = ({ let java_value = jcall!(self.gauge).getValue(); java_value } + self.getMaxValue() / ((value) as f32),); jcall!(java_receiver).setValue(java_arguments.0); };
    }
    

    pub fn addPortion(&mut self, mut value: i32, mut text: String, mut index: i32) {
        { let java_arguments = (jcall!(StringMaker::new()).append(text.clone()).append((jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(index.clone())).expect("NullPointerException").to_string()).toString(),); self.setText(java_arguments.0) };
        PreLogUtil::put(self.text.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ADD_PORTION.clone());
        { let java_receiver = self.gauge.clone(); let java_arguments = ({ let java_value = jcall!(self.gauge).getValue(); java_value } + self.getMaxValue() / ((value) as f32),); jcall!(java_receiver).setValue(java_arguments.0); };
    }
    

    pub fn addNormalPortion(&mut self, mut value: i32, mut text: String) {
        if self.text != text {
            PreLogUtil::put(text.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ADD_PORTION.clone());
        }
        self.setText(text.clone());
        { let java_receiver = self.gauge.clone(); let java_arguments = ({ let java_value = jcall!(self.gauge).getValue(); java_value } + self.getMaxValue() / ((value) as f32),); jcall!(java_receiver).setValue(java_arguments.0); };
    }
    

    pub fn setValue(&mut self, mut value: i32) {
        self.value = ((value) as f32);
        jcall!(self.gauge).setValue(((value) as f32));
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.paintable).paint(graphics.clone());
    }
    

    pub fn paint2(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        jcall!(graphics).setColor(jcall!(self.backgroundBasicColor).intValue());
        jcall!(graphics).fillRect(0, 0, jcall!(self.displayInfo).getLastWidth(), jcall!(self.displayInfo).getLastHeight());
        jcall!(self.gauge).paintXY(graphics.clone(), 0, 0);
        self.hasPainted = true;
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
    

    pub fn getValue(&mut self) -> f32 {
        return self.value.clone();
    }
    

    pub fn setText(&mut self, mut text: String) {
        self.text = text.clone();
    }
    

    pub fn getText(&mut self) -> String {
        return self.text.clone();
    }
    

    pub fn setBackground(&mut self, mut background: bool) {
        self.background = background.clone();
    }
    

    pub fn isBackground(&mut self) -> bool {
        return self.background.clone();
    }
}
impl std::ops::Deref for ProgressCanvas {
    type Target = RunnableCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ProgressCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ProgressCanvasDyn: crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn + crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface + crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface {
    fn as_ProgressCanvas(&self) -> &ProgressCanvas;
    fn as_ProgressCanvas_mut(&mut self) -> &mut ProgressCanvas;
    fn init(&mut self, mut gameMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn>>>) { ProgressCanvas::init(self.as_ProgressCanvas_mut(), gameMidlet) }
    fn update(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { ProgressCanvas::update(self.as_ProgressCanvas_mut(), graphics) }
    fn getMaxValue(&mut self) -> f32 { ProgressCanvas::getMaxValue(self.as_ProgressCanvas_mut()) }
    fn start(&mut self) { ProgressCanvas::start(self.as_ProgressCanvas_mut()) }
    fn startBackground(&mut self, mut background: bool) { ProgressCanvas::startBackground(self.as_ProgressCanvas_mut(), background) }
    fn endActual(&mut self) { ProgressCanvas::endActual(self.as_ProgressCanvas_mut()) }
    fn inGame(&mut self) { ProgressCanvas::inGame(self.as_ProgressCanvas_mut()) }
    fn end(&mut self) { ProgressCanvas::end(self.as_ProgressCanvas_mut()) }
    fn endFromInitialLazyLoadingComplete(&mut self) { ProgressCanvas::endFromInitialLazyLoadingComplete(self.as_ProgressCanvas_mut()) }
    fn isInGame(&mut self) -> bool { ProgressCanvas::isInGame(self.as_ProgressCanvas_mut()) }
    fn endIfPaintedSinceStart(&mut self) { ProgressCanvas::endIfPaintedSinceStart(self.as_ProgressCanvas_mut()) }
    fn addEarlyPortion(&mut self, mut value: i32, mut text: String, mut index: i32) { ProgressCanvas::addEarlyPortion(self.as_ProgressCanvas_mut(), value, text, index) }
    fn addPortion(&mut self, mut value: i32, mut text: String, mut index: i32) { ProgressCanvas::addPortion(self.as_ProgressCanvas_mut(), value, text, index) }
    fn addNormalPortion(&mut self, mut value: i32, mut text: String) { ProgressCanvas::addNormalPortion(self.as_ProgressCanvas_mut(), value, text) }
    fn setValue(&mut self, mut value: i32) { ProgressCanvas::setValue(self.as_ProgressCanvas_mut(), value) }
    fn paint2(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { ProgressCanvas::paint2(self.as_ProgressCanvas_mut(), graphics) }
    fn getValue(&mut self) -> f32 { ProgressCanvas::getValue(self.as_ProgressCanvas_mut()) }
    fn setText(&mut self, mut text: String) { ProgressCanvas::setText(self.as_ProgressCanvas_mut(), text) }
    fn getText(&mut self) -> String { ProgressCanvas::getText(self.as_ProgressCanvas_mut()) }
    fn setBackground(&mut self, mut background: bool) { ProgressCanvas::setBackground(self.as_ProgressCanvas_mut(), background) }
    fn isBackground(&mut self) -> bool { ProgressCanvas::isBackground(self.as_ProgressCanvas_mut()) }
}
impl ProgressCanvasDyn for ProgressCanvas {
    fn as_ProgressCanvas(&self) -> &ProgressCanvas { self }
    fn as_ProgressCanvas_mut(&mut self) -> &mut ProgressCanvas { self }
}

impl crate::javax::microedition::lcdui::Canvas::CanvasDyn for ProgressCanvas {
    fn as_Canvas(&self) -> &crate::javax::microedition::lcdui::Canvas::Canvas { &self.base.base.base }
    fn as_Canvas_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Canvas::Canvas { &mut self.base.base.base }
}
impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for ProgressCanvas {
    fn as_Displayable(&self) -> &crate::javax::microedition::lcdui::Displayable::Displayable { &self.base.base.base.base }
    fn as_Displayable_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Displayable::Displayable { &mut self.base.base.base.base }
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setFullScreenMode(&mut self, p1: bool) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setFullScreenMode(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn hideNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::hideNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn keyPressed(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyPressed(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyRepeated(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn keyReleased(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyReleased(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn pointerPressed(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerPressed(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerReleased(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerReleased(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerDragged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerDragged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn repaint(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaintXYWH(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaintXYWH(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn sizeChanged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::sizeChanged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn showNotify(&mut self) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::showNotify(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self)) }
}
impl crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn for ProgressCanvas {
    fn as_RunnableCanvas(&self) -> &crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &self.base }
    fn as_RunnableCanvas_mut(&mut self) -> &mut crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) { Self::initCommands(self, cmdListener) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for ProgressCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base }
}
impl crate::org::allbinary::graphics::displayable::DisplayableInterface::DisplayableInterface for ProgressCanvas {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface for ProgressCanvas {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeAllCommands(&mut self) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeAllCommands(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
}


