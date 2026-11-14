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
use crate::javax::microedition::lcdui::Command::Command;
use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintable;
use crate::org::allbinary::game::paint::ColorFillPaintableFactory::ColorFillPaintableFactory;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::draw::DrawStringUtil::DrawStringUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::java::lang::Exception::*;
use super::GameCommandCanvas::GameCommandCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameInputMappingInstructionsCanvas {
    pub base: GameCommandCanvas,
    pub drawStringUtil: Option<Rc<RefCell<DrawStringUtil>>>,
    pub TITLE: String,
    pub instructions: Vec<String>,
    pub colorFillPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintableDyn>>>,
    pub anchor: i32,
}

thread_local! { pub static GameInputMappingInstructionsCanvas_DISPLAY: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>> = Command::new4(String::from("Help"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>); }
thread_local! { pub static GameInputMappingInstructionsCanvas_CLOSE: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>> = Command::new4(String::from("Close"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>); }
thread_local! { pub static GameInputMappingInstructionsCanvas_NAME: String = String::from("GameInputMappingInstructionsCanvas"); }
impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for GameInputMappingInstructionsCanvas {
    fn open(&mut self) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::open(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self)) }
    fn close(&mut self) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::close(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self)) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for GameInputMappingInstructionsCanvas {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::getSourceId(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self)) }
}
impl crate::org::allbinary::graphics::displayable::event::DisplayChangeEventListener::DisplayChangeEventListener for GameInputMappingInstructionsCanvas {
    fn onDisplayChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::onDisplayChangeEvent(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for GameInputMappingInstructionsCanvas {
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::onEvent(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}

impl GameInputMappingInstructionsCanvas {
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) -> Self {
        let mut this = Self {
            base: GameCommandCanvas::new_struct(commandListener.clone(), GameInputMappingInstructionsCanvas_NAME.with(Clone::clone).clone(), jcall!(allBinaryGameLayerManager).getBackgroundBasicColor(), jcall!(allBinaryGameLayerManager).getForegroundBasicColor()),
            drawStringUtil: None,
            TITLE: String::new(),
            instructions: Vec::new(),
            colorFillPaintable: None,
            anchor: 0,
        };
        this.drawStringUtil = DrawStringUtil::getInstance();
        this.TITLE = String::from("Input Instructions");
        this.instructions = vec![String::from("Add Input Mapping:"), String::from("1. Select the desired action by generating a currently defined input for a given action."), String::from("(Example: Press the '1' key to select the Fire action)"), String::from("2. Make the newly desired input for the selected action."), String::from("(Example: Press 'f' to add it to the selected action)"), String::from("Note: If the input is already mapped to another action then it will not be added. You"), String::from("will need to deleted from the other action before adding it to another."), String::from("Warning: Not all keys map for a given platform and show as Unknown."), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), String::from("Remove Input Mapping:"), String::from("1. Select the desired action by generating a currently defined input for a given action."), String::from("(Example: Press the key '1' to select the Fire action)"), String::from("2. Select a mapped input for the selected action."), String::from("(Example: Press the key '1' to select the '1' key input for the selected Fire action)"), String::from("3. Press the Delete Key.")];
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF({ let java_field = jcall!(this.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        this.colorFillPaintable = { let java_value = (jcall!(ColorFillPaintableFactory::getInstance()).getInstance2(jcall!(allBinaryGameLayerManager).getBackgroundBasicColor(), false)).clone(); java_value };
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(commandListener, allBinaryGameLayerManager))))
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) {
        self.removeAllCommands();
        { let java_arguments = (GameInputMappingInstructionsCanvas_CLOSE.with(Clone::clone).clone(),); self.addCommand(java_arguments.0) };
        self.setCommandListener(cmdListener.clone());
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        jcall!(self.colorFillPaintable).as_ColorFillBasePaintable_mut().paint(graphics.clone());
        let mut halfWidth: i32 = jcall!(self.displayInfo).getLastHalfWidth();
        let mut beginWidth: i32 = (jcall!(jcall!(graphics).getFont()).stringWidth(self.TITLE.clone()) >> 1);
        jcall!(graphics).setColor(self.foregroundColor.clone());
        jcall!(graphics).drawString(self.TITLE.clone(), halfWidth - beginWidth, self.fontHeight.clone(), self.anchor.clone());
        { let java_receiver = self.drawStringUtil.clone(); let java_arguments = (graphics.clone(), self.instructions.clone(), jcall!(self.displayInfo).getLastWidth(), self.fontHeight.clone(), halfWidth.clone(), 3 * self.fontHeight,); jcall!(java_receiver).drawCenterStrings(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); };
        { let java_arguments = (graphics.clone(),); self.base.paint(java_arguments.0) };
    }
}
impl std::ops::Deref for GameInputMappingInstructionsCanvas {
    type Target = GameCommandCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameInputMappingInstructionsCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn for GameInputMappingInstructionsCanvas {
    fn as_GameCommandCanvas(&self) -> &crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &self.base }
    fn as_GameCommandCanvas_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) { Self::initCommands(self, cmdListener) }
}
impl crate::javax::microedition::lcdui::Canvas::CanvasDyn for GameInputMappingInstructionsCanvas {
    fn as_Canvas(&self) -> &crate::javax::microedition::lcdui::Canvas::Canvas { &self.base.base.base }
    fn as_Canvas_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Canvas::Canvas { &mut self.base.base.base }
}
impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for GameInputMappingInstructionsCanvas {
    fn as_Displayable(&self) -> &crate::javax::microedition::lcdui::Displayable::Displayable { &self.base.base.base.base }
    fn as_Displayable_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Displayable::Displayable { &mut self.base.base.base.base }
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setFullScreenMode(&mut self, p1: bool) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setFullScreenMode(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn hideNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::hideNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn keyPressed(&mut self, p1: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyPressed(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeated(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
    fn keyReleased(&mut self, p1: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyReleased(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
    fn pointerPressed(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerPressed(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerReleased(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerReleased(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerDragged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerDragged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn repaint(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaintXYWH(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaintXYWH(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn sizeChanged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::sizeChanged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn showNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::showNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for GameInputMappingInstructionsCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base }
    fn keyPressedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyPressedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyReleasedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
}
impl crate::org::allbinary::graphics::displayable::DisplayableInterface::DisplayableInterface for GameInputMappingInstructionsCanvas {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface for GameInputMappingInstructionsCanvas {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeAllCommands(&mut self) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeAllCommands(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for GameInputMappingInstructionsCanvas {
    fn updateMeasurement(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::updateMeasurement(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}


