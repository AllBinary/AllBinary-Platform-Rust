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
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintable;
use crate::org::allbinary::game::paint::ColorFillPaintableFactory::ColorFillPaintableFactory;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::draw::DrawStringUtil::DrawStringUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
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

thread_local! { pub static GameInputMappingInstructionsCanvas_DISPLAY: Option<Rc<RefCell<Command>>> = Command::new(String::from("Help"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1); }
thread_local! { pub static GameInputMappingInstructionsCanvas_CLOSE: Option<Rc<RefCell<Command>>> = Command::new(String::from("Close"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1); }
thread_local! { pub static GameInputMappingInstructionsCanvas_NAME: String = String::from("GameInputMappingInstructionsCanvas"); }
impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for GameInputMappingInstructionsCanvas {
    fn open(&mut self) { self.base.open() }
    fn close(&mut self) { self.base.close() }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for GameInputMappingInstructionsCanvas {
    fn getSourceId(&mut self) -> i32 { self.base.getSourceId() }
}

impl GameInputMappingInstructionsCanvas {
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) -> Self {
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
        this.instructions = vec![String::from("Add Input Mapping:"), String::from("1. Select the desired action by generating a currently defined input for a given action."), String::from("(Example: Press the '1' key to select the Fire action)"), String::from("2. Make the newly desired input for the selected action."), String::from("(Example: Press 'f' to add it to the selected action)"), String::from("Note: If the input is already mapped to another action then it will not be added. You"), String::from("will need to deleted from the other action before adding it to another."), String::from("Warning: Not all keys map for a given platform and show as Unknown."), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), String::from("Remove Input Mapping:"), String::from("1. Select the desired action by generating a currently defined input for a given action."), String::from("(Example: Press the key '1' to select the Fire action)"), String::from("2. Select a mapped input for the selected action."), String::from("(Example: Press the key '1' to select the '1' key input for the selected Fire action)"), String::from("3. Press the Delete Key.")];
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.colorFillPaintable = jcall!(ColorFillPaintableFactory::getInstance()).getInstance2(jcall!(allBinaryGameLayerManager).getBackgroundBasicColor(), false);
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(commandListener, allBinaryGameLayerManager))))
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
        self.removeAllCommands();
        self.addCommand(GameInputMappingInstructionsCanvas_CLOSE.with(Clone::clone).clone());
        self.setCommandListener(cmdListener.clone());
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        jcall!(self.colorFillPaintable).paint(graphics.clone());
        let mut halfWidth: i32 = jcall!(self.displayInfo).getLastHalfWidth();
        let mut beginWidth: i32 = (jcall!(jcall!(graphics).getFont()).stringWidth(self.TITLE.clone()) >> 1);
        jcall!(graphics).setColor(self.foregroundColor.clone());
        jcall!(graphics).drawString(self.TITLE.clone(), halfWidth - beginWidth, self.fontHeight.clone(), self.anchor.clone());
        { let java_receiver = self.drawStringUtil.clone(); let java_arguments = (graphics.clone(), self.instructions.clone(), jcall!(self.displayInfo).getLastWidth(), self.fontHeight.clone(), halfWidth.clone(), 3 * self.fontHeight,); jcall!(java_receiver).drawCenterStrings(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); };
        self.base.paint(graphics.clone());
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
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { Self::initCommands(self, cmdListener) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for GameInputMappingInstructionsCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn keyPressedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyPressedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyReleasedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
}


