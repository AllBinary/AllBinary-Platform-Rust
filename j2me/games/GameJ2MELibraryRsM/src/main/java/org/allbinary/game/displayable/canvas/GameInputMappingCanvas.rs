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
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::input::GameKey::GameKey;
use crate::org::allbinary::game::input::GameKeyFactory::GameKeyFactory;
use crate::org::allbinary::game::input::Input::Input;
use crate::org::allbinary::game::input::InputFactory::InputFactory;
use crate::org::allbinary::game::input::PlatformInputMappingFactory::PlatformInputMappingFactory;
use crate::org::allbinary::game::input::mapping::InputMappingInterface::InputMappingInterface;
use crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMapping;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintable;
use crate::org::allbinary::game::paint::ColorFillPaintableFactory::ColorFillPaintableFactory;
use crate::org::allbinary::game::paint::help::HelpPaintable::HelpPaintable;
use crate::org::allbinary::game::paint::help::InputMappingHelpPaintable::InputMappingHelpPaintable;
use crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintable;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::java::lang::Exception::*;
use super::GameCommandCanvas::GameCommandCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameInputMappingCanvas {
    pub base: GameCommandCanvas,
    pub paintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintableDyn>>>,
    pub helpPaintable: Option<Rc<RefCell<InputMappingHelpPaintable>>>,
    pub colorFillPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::game::paint::ColorFillBasePaintable::ColorFillBasePaintableDyn>>>,
    pub inputMapping: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMappingDyn>>>,
    pub NONE: Option<Rc<RefCell<GameKey>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub selectedGameKey: Option<Rc<RefCell<GameKey>>>,
    pub selectedInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>,
    pub inputFactory: Option<Rc<RefCell<InputFactory>>>,
}

pub static GameInputMappingCanvas_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("GameInputMappingCanvas"));
pub static GameInputMappingCanvas_DISPLAY: crate::JavaStatic<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>> = crate::JavaStatic::new(|| Command::new4(String::from("Controls"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 2).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>));
pub static GameInputMappingCanvas_DEFAULT: crate::JavaStatic<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>> = crate::JavaStatic::new(|| Command::new4(String::from("Default"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 2).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>));
impl InputMappingInterface for GameInputMappingCanvas {
    fn processInputMapping(&mut self, mut gameKey: Option<Rc<RefCell<GameKey>>>, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) { Self::processInputMapping(self, gameKey, input) }
}
impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for GameInputMappingCanvas {
    fn open(&mut self) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::open(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self)) }
    fn close(&mut self) { Self::close(self) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for GameInputMappingCanvas {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::getSourceId(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self)) }
}
impl crate::org::allbinary::graphics::displayable::event::DisplayChangeEventListener::DisplayChangeEventListener for GameInputMappingCanvas {
    fn onDisplayChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::onDisplayChangeEvent(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for GameInputMappingCanvas {
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::onEvent(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}

impl GameInputMappingCanvas {
    

    pub fn new_struct(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut helpPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::game::paint::help::HelpPaintable::HelpPaintableDyn>>>) -> Self {
        let mut this = Self {
            base: GameCommandCanvas::new_struct(commandListener.clone(), GameInputMappingCanvas_NAME.with(Clone::clone).clone(), jcall!(allBinaryGameLayerManager).getBackgroundBasicColor(), jcall!(allBinaryGameLayerManager).getForegroundBasicColor()),
            paintable: None,
            helpPaintable: None,
            colorFillPaintable: None,
            inputMapping: None,
            NONE: None,
            abeClientInformation: None,
            selectedGameKey: None,
            selectedInput: None,
            inputFactory: None,
        };
        this.NONE = { let java_field = jcall!(GameKeyFactory::getInstance()).NONE.clone(); java_field }.clone();
        this.selectedGameKey = this.NONE.clone();
        this.selectedInput = this.NONE.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>);
        this.inputFactory = InputFactory::getInstance();
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF({ let java_field = jcall!(this.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        if helpPaintable.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Help Paintable Exception"))));
        }
        this.abeClientInformation = abeClientInformation.clone();
        this.helpPaintable = (helpPaintable).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<InputMappingHelpPaintable>() }, "ClassCastException: not a InputMappingHelpPaintable"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<InputMappingHelpPaintable>) } });
        this.inputMapping = { let java_value = (jcall!(PlatformInputMappingFactory::getInstance()).getPersistentInputMappingInstance()).clone(); java_value };
        this.paintable = ProcessPaintable::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::ProcessPaintable::ProcessPaintableDyn>>);
        this.colorFillPaintable = { let java_value = (jcall!(ColorFillPaintableFactory::getInstance()).getInstance2(jcall!(allBinaryGameLayerManager).getBackgroundBasicColor(), false)).clone(); java_value };
        this
    }

    pub fn new(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut commandListener: Option<Rc<RefCell<dyn CommandListener>>>, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>, mut helpPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::game::paint::help::HelpPaintable::HelpPaintableDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(abeClientInformation, commandListener, allBinaryGameLayerManager, helpPaintable))))
    }
    

    // throws Exception
    pub fn close(&mut self) {
        self.base.close();
        jcall!(self.paintable).process();
        self.selectedGameKey = self.NONE.clone();
        self.selectedInput = self.NONE.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>);
        self.update();
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) {
        self.removeAllCommands();
        { let java_arguments = ({ let java_field = jcall!(GameCommandsFactory::getInstance()).CLOSE_AND_SHOW_GAME_CANVAS.clone(); java_field }.clone(),); self.addCommand(java_arguments.0) };
        { let java_arguments = (GameInputMappingCanvas_DEFAULT.with(Clone::clone).clone(),); self.addCommand(java_arguments.0) };
        { let java_arguments = (GameInputMappingInstructionsCanvas_DISPLAY.with(Clone::clone).clone(),); self.addCommand(java_arguments.0) };
        self.setCommandListener(cmdListener.clone());
    }
    

    pub fn keyPressed(&mut self, mut keyCode: i32) {
        self.keyPressedByDevice(keyCode.clone(), 0);
    }
    

    pub fn keyReleased(&mut self, mut keyCode: i32) {
        self.keyReleasedByDevice(keyCode.clone(), 0);
    }
    

    pub fn keyRepeated(&mut self, mut keyCode: i32) {
        self.keyRepeatedByDevice(keyCode.clone(), 0);
    }
    

    pub fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
        self.addGameKey(keyCode.clone(), false);
        { let java_arguments = (keyCode.clone(), 0,); self.base.keyPressedByDevice(java_arguments.0, java_arguments.1) };
    }
    

    pub fn addGameKey(&mut self, mut keyCode: i32, mut repeated: bool) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Raw Device Key Code: ")).append(format!("{:x}", keyCode.clone())).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.gameInputStrings).ADD_KEY_EVENT.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            let mut gameKey: Option<Rc<RefCell<GameKey>>> = jcall!(self.inputToGameKeyMapping).getInstanceForCanvas(None /* Java this: a &mut self method has no Rc of itself */, keyCode.clone());
            let mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = jcall!(self.inputFactory).getInstanceById(keyCode.clone());
            self.processInputMapping(gameKey.clone(), input.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put(String::from("Key Event Error"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.gameInputStrings).ADD_KEY_EVENT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn processInputMapping(&mut self, mut gameKey: Option<Rc<RefCell<GameKey>>>, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Start Passed GameKey: "));
        jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(gameKey.clone())));
        jcall!(stringBuffer).append(String::from(" Input: "));
        jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(input.clone())));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        if !match (&self.selectedGameKey, &self.NONE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.gameActionCrud(gameKey.clone(), input.clone());
        } else {
            self.setSelectedAction(gameKey.clone());
        }
    }
    

    pub fn setSelectedAction(&mut self, mut gameKey: Option<Rc<RefCell<GameKey>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Selected GameKey: ")).append(jcall!(self.stringUtil).toString(crate::java_into_object(gameKey.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setSelectedAction"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.selectedGameKey = gameKey.clone();
        self.selectedInput = self.NONE.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>);
        jcall!(self.helpPaintable).update(self.selectedGameKey.clone(), self.selectedInput.clone());
        jcall!(self.repaintBehavior).onChangeRepaint(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn gameActionCrud(&mut self, mut gameKey: Option<Rc<RefCell<GameKey>>>, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Start GameKey: "));
        jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(self.selectedGameKey.clone())));
        jcall!(stringBuffer).append(String::from(" Input: "));
        jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(self.selectedInput.clone())));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("gameActionCrud"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        if match (&self.selectedInput, &self.NONE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(jcall!(self.inputMapping).getInputMapping()).getMappedInput(self.selectedGameKey.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
            let mut isInputAlreadyMappedToSelectedAction: bool = jcall!(list).contains(crate::java_into_object(input.clone()));
            if isInputAlreadyMappedToSelectedAction {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Already Mapped Input: ")).append(jcall!(self.stringUtil).toString(crate::java_into_object(input.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("gameActionCrud"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                self.selectedInput = input.clone();
                jcall!(self.helpPaintable).update(self.selectedGameKey.clone(), self.selectedInput.clone());
                jcall!(self.repaintBehavior).onChangeRepaint(None /* Java this: a &mut self method has no Rc of itself */);
            } else {
                self.addNewMapping(gameKey.clone(), input.clone());
            }
        } else if jcall!(self.inputMapping).isDelete(input.clone()) {
            self.deleteCurrentMapping();
        } else {
            self.setSelectedAction(gameKey.clone());
        }
    }
    

    // throws Exception
    pub fn addNewMapping(&mut self, mut gameKey: Option<Rc<RefCell<GameKey>>>, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) {
        let mut METHOD_NAME: String = String::from("addNewMapping");
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
        let mut isInputAlreadyMapped: bool = jcall!(jcall!(self.inputMapping).getInputMapping()).isMapped(input.clone());
        if !isInputAlreadyMapped && !jcall!(self.inputMapping).isSystemInput(input.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Add Key Mapping : GameKey: "));
            jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(self.selectedGameKey.clone())));
            jcall!(stringBuffer).append(String::from(" Input: "));
            jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(self.selectedInput.clone())));
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(jcall!(self.inputMapping).getInputMapping()).add(self.selectedGameKey.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), input.clone());
            self.selectedInput = input.clone();
            self.update();
        } else {
            jcall!(self.logUtil).putF(String::from("Unable to add Mapping since one already exists or is MENU, HOME, or BACK key and setting selected action to what it is already mapped to"), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
            self.setSelectedAction(gameKey.clone());
        }
    }
    

    // throws Exception
    pub fn deleteCurrentMapping(&mut self) {
        let mut METHOD_NAME: String = String::from("deleteCurrentMapping");
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(jcall!(self.inputMapping).getInputMapping()).getMappedInput(self.selectedGameKey.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        if jcall!(list).size() > 1 {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Start GameKey: "));
            jcall!(stringBuffer).append(String::from("Remove Key Mapping: GameKey: "));
            jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(self.selectedGameKey.clone())));
            jcall!(stringBuffer).append(String::from(" Input: "));
            jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(self.selectedInput.clone())));
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(jcall!(self.inputMapping).getInputMapping()).remove(self.selectedGameKey.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), self.selectedInput.clone());
            self.selectedInput = self.NONE.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>);
            self.update();
        } else {
            jcall!(self.logUtil).putF(String::from("Can't Remove Last Key Mapping"), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
        }
    }
    

    // throws Exception
    pub fn setDefault(&mut self) {
        jcall!(self.inputMapping).setDefault(self.abeClientInformation.clone());
        jcall!(self.helpPaintable).update(self.NONE.clone(), self.NONE.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(self.repaintBehavior).onChangeRepaint(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn update(&mut self) {
        jcall!(self.inputMapping).update(self.abeClientInformation.clone());
        jcall!(self.helpPaintable).update(self.selectedGameKey.clone(), self.selectedInput.clone());
        jcall!(self.repaintBehavior).onChangeRepaint(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.colorFillPaintable).as_ColorFillBasePaintable_mut().paint(graphics.clone());
        jcall!(self.helpPaintable).paint(graphics.clone());
        jcall!(self.paintable).as_ProcessPaintable_mut().paint(graphics.clone());
        { let java_arguments = (graphics.clone(),); self.base.paint(java_arguments.0) };
    }
}
impl std::ops::Deref for GameInputMappingCanvas {
    type Target = GameCommandCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameInputMappingCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn for GameInputMappingCanvas {
    fn as_GameCommandCanvas(&self) -> &crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &self.base }
    fn as_GameCommandCanvas_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>) { Self::initCommands(self, cmdListener) }
    fn close(&mut self) { Self::close(self) }
    fn update(&mut self) { Self::update(self) }
}
impl crate::javax::microedition::lcdui::Canvas::CanvasDyn for GameInputMappingCanvas {
    fn as_Canvas(&self) -> &crate::javax::microedition::lcdui::Canvas::Canvas { &self.base.base.base }
    fn as_Canvas_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Canvas::Canvas { &mut self.base.base.base }
}
impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for GameInputMappingCanvas {
    fn as_Displayable(&self) -> &crate::javax::microedition::lcdui::Displayable::Displayable { &self.base.base.base.base }
    fn as_Displayable_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Displayable::Displayable { &mut self.base.base.base.base }
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setFullScreenMode(&mut self, p1: bool) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setFullScreenMode(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn hideNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::hideNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn keyPressed(&mut self, mut keyCode: i32) { Self::keyPressed(self, keyCode) }
    fn keyRepeated(&mut self, mut keyCode: i32) { Self::keyRepeated(self, keyCode) }
    fn keyReleased(&mut self, mut keyCode: i32) { Self::keyReleased(self, keyCode) }
    fn pointerPressed(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerPressed(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerReleased(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerReleased(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerDragged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerDragged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn repaint(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaintXYWH(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaintXYWH(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn sizeChanged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::sizeChanged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn showNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::showNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for GameInputMappingCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base }
    fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyPressedByDevice(self, keyCode, deviceId) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyReleasedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
}
impl crate::org::allbinary::graphics::displayable::DisplayableInterface::DisplayableInterface for GameInputMappingCanvas {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface for GameInputMappingCanvas {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeAllCommands(&mut self) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeAllCommands(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for GameInputMappingCanvas {
    fn updateMeasurement(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::updateMeasurement(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}


