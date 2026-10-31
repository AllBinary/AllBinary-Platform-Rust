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
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::InputFeatureFactory::InputFeatureFactory;
use crate::org::allbinary::game::displayable::RepaintBehavior::RepaintBehavior;
use crate::org::allbinary::game::input::GameInputStrings::GameInputStrings;
use crate::org::allbinary::game::input::GameKey::GameKey;
use crate::org::allbinary::game::input::GameKeyFactory::GameKeyFactory;
use crate::org::allbinary::game::input::PlatformInputMappingFactory::PlatformInputMappingFactory;
use crate::org::allbinary::game::input::event::DownGameKeyEventHandler::DownGameKeyEventHandler;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::game::input::event::GameKeyEventHandler::GameKeyEventHandler;
use crate::org::allbinary::game::input::event::UpGameKeyEventHandler::UpGameKeyEventHandler;
use crate::org::allbinary::game::input::mapping::InputToGameKeyMapping::InputToGameKeyMapping;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEventHandler::DisplayChangeEventHandler;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEventListener::DisplayChangeEventListener;
use crate::org::allbinary::graphics::displayable::screen::ScreenRepaintProcessorFactory::ScreenRepaintProcessorFactory;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::form::CommandCurrentSelectionFormFactory::CommandCurrentSelectionFormFactory;
use crate::org::allbinary::graphics::form::FormPaintable::FormPaintable;
use crate::org::allbinary::graphics::form::FormTypeFactory::FormTypeFactory;
use crate::org::allbinary::graphics::form::PaintableForm::PaintableForm;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::graphics::form::ScrollSelectionFormNoneFactory::ScrollSelectionFormNoneFactory;
use crate::org::allbinary::graphics::form::item::CommandTextItemArrayFactory::CommandTextItemArrayFactory;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::graphics::form::item::validation::AllCommandsVisitor::AllCommandsVisitor;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::input::motion::gesture::observer::BasicMotionGesturesHandler::BasicMotionGesturesHandler;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::MenuListener::MenuListener;
use super::BasicMenuInputProcessor::BasicMenuInputProcessor;
use super::ImmediateCommandFormInputProcessor::ImmediateCommandFormInputProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameCommandCanvas {
    pub base: MyCanvas,
    pub repaintProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub gameInputStrings: Option<Rc<RefCell<GameInputStrings>>>,
    pub foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>,
    pub gameKeyFactory: Option<Rc<RefCell<GameKeyFactory>>>,
    pub gameKeyEventFactory: Option<Rc<RefCell<GameKeyEventFactory>>>,
    pub downGameKeyEventHandler: Option<Rc<RefCell<DownGameKeyEventHandler>>>,
    pub upGameKeyEventHandler: Option<Rc<RefCell<UpGameKeyEventHandler>>>,
    pub repaintBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::RepaintBehavior::RepaintBehaviorDyn>>>,
    pub updateMyFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub foregroundColor: i32,
    pub backgroundColor: i32,
    pub menuInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>>,
    pub menuPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub menuForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>>,
    pub isSingleKeyRepeatableProcessing: bool,
    pub fontHeight: i32,
}

thread_local! { pub static GameCommandCanvas_id: i32 = 0; }
impl MenuListener for GameCommandCanvas {
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for GameCommandCanvas {
    fn getSourceId(&mut self) -> i32 { Self::getSourceId(self) }
}
impl DisplayChangeEventListener for GameCommandCanvas {
    fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) { Self::onDisplayChangeEvent(self, displayChangeEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for GameCommandCanvas {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}
impl UpdateMyFontInterface for GameCommandCanvas {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl GameCommandCanvas {
    

    pub fn new_struct(mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut name: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: MyCanvas::new_struct(name.clone(), jcall!(CanvasStrings::getInstance()).EMPTY_CHILD_NAME_LIST.clone()),
            repaintProcessor: None,
            gameInputStrings: None,
            foregroundBasicColor: None,
            backgroundBasicColor: None,
            inputToGameKeyMapping: None,
            gameKeyFactory: None,
            gameKeyEventFactory: None,
            downGameKeyEventHandler: None,
            upGameKeyEventHandler: None,
            repaintBehavior: None,
            updateMyFontProcessor: None,
            myFontProcessor: None,
            foregroundColor: 0,
            backgroundColor: 0,
            menuInputProcessor: None,
            menuPaintable: None,
            menuForm: None,
            isSingleKeyRepeatableProcessing: false,
            fontHeight: 0,
        };
        this.repaintProcessor = jcall!(ScreenRepaintProcessorFactory::getInstance()).create(this);
        this.gameInputStrings = GameInputStrings::getInstance();
        this.inputToGameKeyMapping = jcall!(jcall!(PlatformInputMappingFactory::getInstance()).getPersistentInputMappingInstance()).getInputMapping();
        this.gameKeyFactory = GameKeyFactory::getInstance();
        this.gameKeyEventFactory = GameKeyEventFactory::getInstance();
        this.downGameKeyEventHandler = DownGameKeyEventHandler::getInstance();
        this.upGameKeyEventHandler = UpGameKeyEventHandler::getInstance();
        this.updateMyFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.myFontProcessor = this.updateMyFontProcessor.clone();
        this.menuInputProcessor = NoMenuInputProcessor::NoMenuInputProcessor::getInstance();
        this.menuPaintable = NullPaintable::getInstance();
        this.menuForm = PaintableForm::getNullPaintableForm();
        this.isSingleKeyRepeatableProcessing = jcall!(Features::getInstance()).isFeature(jcall!(InputFeatureFactory::getInstance()).SINGLE_KEY_REPEAT_PRESS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
        this.fontHeight = 0;
        // super(...) is in the struct literal above
        this.repaintBehavior = RepaintBehavior::getInstance();
        this.foregroundBasicColor = foregroundBasicColor.clone();
        this.backgroundBasicColor = backgroundBasicColor.clone();
        this.foregroundColor = jcall!(this.foregroundBasicColor).intValue();
        this.backgroundColor = jcall!(this.backgroundBasicColor).intValue();
        this.initCommands(cmdListener.clone());
        if !match (&cmdListener, &crate::javax::microedition::lcdui::NullCommandListener::NullCommandListener_NULL_COMMAND_LISTENER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                this.initMenu();
                jcall!(this.repaintProcessor).process();
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
                }
            }
        }
        this
    }

    pub fn new(mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut name: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(cmdListener, name, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.commonStrings).START.clone()).append(jcall!(self.canvasStrings).FD_WIDTH.clone()).appendint(((MyFontProcessor::defaultCharWidth(font.clone())) as i32)).append(jcall!(self.canvasStrings).FD_HEIGHT.clone()).appendint(((jcall!(font).getHeight()) as i32)).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).UPDATE_MEASUREMENT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            self.fontHeight = jcall!(font).getHeight();
            let mut rectangle: Option<Rc<RefCell<Rectangle>>> = { let java_arguments = (jcall!(self.menuForm).as_PaintableForm_mut().size(),); self.createRectangle(java_arguments.0) };
            { let java_receiver = self.menuForm.clone(); let java_arguments = (rectangle.clone(), jcall!(FormTypeFactory::getInstance()).VERTICAL_CENTER_FORM.clone(),); jcall!(java_receiver).init(java_arguments.0, java_arguments.1); };
            self.update();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).ON_DISPLAY_CHANGE_EVENT.clone(), crate::java_into_object(e.clone()));
            }
        }
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.commonStrings).START.clone()).append(jcall!(DisplayInfoSingleton::getInstance()).toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).ON_DISPLAY_CHANGE_EVENT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.myFontProcessor = self.updateMyFontProcessor.clone();
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
        self.removeAllCommands();
        { let java_arguments = (jcall!(GameCommandsFactory::getInstance()).CLOSE_AND_SHOW_GAME_CANVAS.clone(),); self.addCommand(java_arguments.0) };
        self.setCommandListener(cmdListener.clone());
    }
    

    // throws Exception
    pub fn initMenu(&mut self) {
        let mut form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = self.createForm();
        self.menuForm = form.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>);
        if !match (&form, &ScrollSelectionFormNoneFactory::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.setMenuInputProcessor(ImmediateCommandFormInputProcessor::new(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>), -1, None /* Java this: a &mut self method has no Rc of itself */, form.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>));
            self.menuPaintable = FormPaintable::new(form.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        }
        jcall!(self.repaintBehavior).onChangeRepaint(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn createForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> {
        let mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>> = jcall!(CommandTextItemArrayFactory::new(AllCommandsVisitor::new())).getInstance(self.getCommandStack(), self.backgroundBasicColor.clone(), self.foregroundBasicColor.clone());
        let mut rectangle: Option<Rc<RefCell<Rectangle>>> = self.createRectangle((items.len() as i32).clone());
        return CommandCurrentSelectionFormFactory::getInstance(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), items.clone(), rectangle.clone(), jcall!(FormTypeFactory::getInstance()).VERTICAL_CENTER_FORM.clone(), 15, false, self.backgroundBasicColor.clone(), self.foregroundBasicColor.clone());
    }
    

    pub fn createRectangle(&mut self, mut size: i32) -> Option<Rc<RefCell<Rectangle>>> {
        let mut height: i32 = size * self.fontHeight;
        let mut startY: i32 = (jcall!(self.displayInfo).getLastHeight() * 2 / 3) - height;
        let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
        let mut rectangle: Option<Rc<RefCell<Rectangle>>> = Rectangle::new(jcall!(pointFactory).createXY(30, startY.clone()), jcall!(self.displayInfo).getLastWidth() - 30, startY.clone());
        return rectangle.clone();
    }
    

    pub fn open(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("open"));
        jcall!(BasicMotionGesturesHandler::getInstance()).addListenerInterface(self.getMenuInputProcessor());
        jcall!(GameKeyEventHandler::getInstance()).addListener(self.getMenuInputProcessor().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        jcall!(DisplayChangeEventHandler::getInstance()).addListenerInterface(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn close(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CLOSE.clone());
        jcall!(BasicMotionGesturesHandler::getInstance()).removeListener(self.getMenuInputProcessor());
        jcall!(GameKeyEventHandler::getInstance()).removeListener(self.getMenuInputProcessor().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        jcall!(DisplayChangeEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn update(&mut self) {
        jcall!(self.repaintProcessor).process();
    }
    

    pub fn getSourceId(&mut self) -> i32 {
        return GameCommandCanvas_id.with(Clone::clone).clone();
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
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).appendint(keyCode.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).KEY_PRESSED.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.addGameKeyEvent(keyCode.clone(), 0, false);
    }
    

    pub fn keyReleasedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
        self.removeGameKeyEvent(keyCode.clone(), deviceId.clone(), false);
    }
    

    pub fn keyRepeatedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
        if self.isSingleKeyRepeatableProcessing {
            self.addGameKeyEvent(keyCode.clone(), deviceId.clone(), true);
        }
    }
    

    pub fn addGameKeyEvent(&mut self, mut keyCode: i32, mut deviceId: i32, mut repeated: bool) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut gameKey: Option<Rc<RefCell<GameKey>>> = jcall!(self.inputToGameKeyMapping).getInstanceForCanvas(None /* Java this: a &mut self method has no Rc of itself */, keyCode.clone());
            if !match (&gameKey, &jcall!(self.gameKeyFactory).NONE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = jcall!(self.gameKeyEventFactory).getInstanceForInput(None /* Java this: a &mut self method has no Rc of itself */, gameKey.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
                jcall!(self.downGameKeyEventHandler).fireEvent(gameKeyEvent.clone());
                jcall!(jcall!(self.downGameKeyEventHandler).getInstanceForDevice(deviceId.clone())).fireEvent(gameKeyEvent.clone());
            } else {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.gameInputStrings).NO_KEY.clone()).appendint(keyCode.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).ADD_KEY_EVENT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(String::from("Key Event Error"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).ADD_KEY_EVENT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn removeGameKeyEvent(&mut self, mut keyCode: i32, mut deviceId: i32, mut repeated: bool) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut gameKey: Option<Rc<RefCell<GameKey>>> = jcall!(self.inputToGameKeyMapping).getInstanceForCanvas(None /* Java this: a &mut self method has no Rc of itself */, keyCode.clone());
            if !match (&gameKey, &jcall!(self.gameKeyFactory).NONE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = jcall!(self.gameKeyEventFactory).getInstanceForInput(None /* Java this: a &mut self method has no Rc of itself */, gameKey.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
                jcall!(self.upGameKeyEventHandler).fireEvent(gameKeyEvent.clone());
                jcall!(jcall!(self.upGameKeyEventHandler).getInstanceForDevice(deviceId.clone())).fireEvent(gameKeyEvent.clone());
            } else {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.gameInputStrings).NO_KEY.clone()).appendint(keyCode.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).REMOVE_KEY_EVENT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(String::from("Key Event Error"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).REMOVE_KEY_EVENT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        jcall!(self.menuPaintable).paint(graphics.clone());
        jcall!(self.repaintBehavior).repaint(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn setMenuInputProcessor(&mut self, mut menuInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>>) {
        self.menuInputProcessor = menuInputProcessor.clone();
    }
    

    pub fn getMenuInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>> {
        return self.menuInputProcessor.clone();
    }
}
impl std::ops::Deref for GameCommandCanvas {
    type Target = MyCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameCommandCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait GameCommandCanvasDyn: crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn {
    fn as_GameCommandCanvas(&self) -> &GameCommandCanvas;
    fn as_GameCommandCanvas_mut(&mut self) -> &mut GameCommandCanvas;
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { GameCommandCanvas::updateMeasurement(self.as_GameCommandCanvas_mut(), graphics) }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { GameCommandCanvas::onEvent(self.as_GameCommandCanvas_mut(), eventObject) }
    fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) { GameCommandCanvas::onDisplayChangeEvent(self.as_GameCommandCanvas_mut(), displayChangeEvent) }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { GameCommandCanvas::initCommands(self.as_GameCommandCanvas_mut(), cmdListener) }
    fn initMenu(&mut self) { GameCommandCanvas::initMenu(self.as_GameCommandCanvas_mut()) }
    fn createForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> { GameCommandCanvas::createForm(self.as_GameCommandCanvas_mut()) }
    fn createRectangle(&mut self, mut size: i32) -> Option<Rc<RefCell<Rectangle>>> { GameCommandCanvas::createRectangle(self.as_GameCommandCanvas_mut(), size) }
    fn open(&mut self) { GameCommandCanvas::open(self.as_GameCommandCanvas_mut()) }
    fn close(&mut self) { GameCommandCanvas::close(self.as_GameCommandCanvas_mut()) }
    fn update(&mut self) { GameCommandCanvas::update(self.as_GameCommandCanvas_mut()) }
    fn getSourceId(&mut self) -> i32 { GameCommandCanvas::getSourceId(self.as_GameCommandCanvas_mut()) }
    fn keyPressed(&mut self, mut keyCode: i32) { GameCommandCanvas::keyPressed(self.as_GameCommandCanvas_mut(), keyCode) }
    fn keyReleased(&mut self, mut keyCode: i32) { GameCommandCanvas::keyReleased(self.as_GameCommandCanvas_mut(), keyCode) }
    fn keyRepeated(&mut self, mut keyCode: i32) { GameCommandCanvas::keyRepeated(self.as_GameCommandCanvas_mut(), keyCode) }
    fn addGameKeyEvent(&mut self, mut keyCode: i32, mut deviceId: i32, mut repeated: bool) { GameCommandCanvas::addGameKeyEvent(self.as_GameCommandCanvas_mut(), keyCode, deviceId, repeated) }
    fn removeGameKeyEvent(&mut self, mut keyCode: i32, mut deviceId: i32, mut repeated: bool) { GameCommandCanvas::removeGameKeyEvent(self.as_GameCommandCanvas_mut(), keyCode, deviceId, repeated) }
    fn setMenuInputProcessor(&mut self, mut menuInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>>) { GameCommandCanvas::setMenuInputProcessor(self.as_GameCommandCanvas_mut(), menuInputProcessor) }
    fn getMenuInputProcessor(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::BasicMenuInputProcessor::BasicMenuInputProcessorDyn>>> { GameCommandCanvas::getMenuInputProcessor(self.as_GameCommandCanvas_mut()) }
}
impl GameCommandCanvasDyn for GameCommandCanvas {
    fn as_GameCommandCanvas(&self) -> &GameCommandCanvas { self }
    fn as_GameCommandCanvas_mut(&mut self) -> &mut GameCommandCanvas { self }
}

impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for GameCommandCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyPressedByDevice(self, keyCode, deviceId) }
    fn keyRepeatedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyRepeatedByDevice(self, keyCode, deviceId) }
    fn keyReleasedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyReleasedByDevice(self, keyCode, deviceId) }
}


