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
// package org.allbinary.graphics.form.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas;
use crate::org::allbinary::game::input::Input::Input;
use crate::org::allbinary::game::input::InputFactory::InputFactory;
use crate::org::allbinary::game::input::PlatformKeyFactory::PlatformKeyFactory;
use crate::org::allbinary::game::input::event::RawKeyEventListener::RawKeyEventListener;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::form::item::validation::TextItemVisitor::TextItemVisitor;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::java::lang::Exception::*;
use super::ABTextFieldItem::ABTextFieldItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomTextBox {
    pub base: GameCommandCanvas,
    pub textFieldItem: Option<Rc<RefCell<ABTextFieldItem>>>,
    pub inputFactory: Option<Rc<RefCell<InputFactory>>>,
}

impl RawKeyEventListener for CustomTextBox {
    fn onEventRaw(&mut self, mut keyCode: i32, mut deviceId: i32, mut repeated: bool) { Self::onEventRaw(self, keyCode, deviceId, repeated) }
}
impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for CustomTextBox {
    fn open(&mut self) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::open(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self)) }
    fn close(&mut self) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::close(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self)) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for CustomTextBox {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::getSourceId(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self)) }
}
impl crate::org::allbinary::graphics::displayable::event::DisplayChangeEventListener::DisplayChangeEventListener for CustomTextBox {
    fn onDisplayChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::onDisplayChangeEvent(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for CustomTextBox {
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::onEvent(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}

impl CustomTextBox {
    

    pub fn new_struct(mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>, mut label: String, mut text: String, mut maxSize: i32, mut constraints: i32, mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: GameCommandCanvas::new_struct(cmdListener.clone(), label.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            textFieldItem: None,
            inputFactory: None,
        };
        this.inputFactory = InputFactory::getInstance();
        // super(...) is in the struct literal above
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        this
    }

    pub fn new(mut cmdListener: Option<Rc<RefCell<dyn CommandListener>>>, mut label: String, mut text: String, mut maxSize: i32, mut constraints: i32, mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(cmdListener.clone(), label.clone(), text.clone(), maxSize.clone(), constraints.clone(), font.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()))));
            let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
            let mut textFieldItem: Option<Rc<RefCell<ABTextFieldItem>>> = ABTextFieldItem::new(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>), TextItemVisitor::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>), { let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone(), { let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone(), maxSize.clone(), 0, { let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone(), font.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone());
            jcall!(textFieldItem).setString(text.clone());
            jcall!(this).textFieldItem = textFieldItem.clone();
            jcall!(this).setTitle(label.clone());
        this
    }
    

    pub fn submit(&mut self) {
    }
    

    pub fn onEventRaw(&mut self, mut keyCode: i32, mut deviceId: i32, mut repeated: bool) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone()).appendint(keyCode.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onEvent"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.keyPressedByDevice(keyCode.clone(), deviceId.clone());
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
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append({ let java_field = jcall!(CommonSeps::getInstance()).SPACE.clone(); java_field }.clone()).appendint(keyCode.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.gameInputStrings).KEY_PRESSED.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            let mut platformKeyFactory: Option<Rc<RefCell<PlatformKeyFactory>>> = PlatformKeyFactory::getInstance();
            let mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = jcall!(self.inputFactory).getInstanceById(keyCode.clone());
            if jcall!(platformKeyFactory).isSubmission(input.clone()) {
                self.submit();
            } else {
                jcall!(self.textFieldItem).keyPressed(keyCode.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.gameInputStrings).KEY_PRESSED.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn keyReleasedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(graphics).setColor(self.backgroundColor.clone());
        jcall!(graphics).fillRect(0, 0, jcall!(self.displayInfo).getLastWidth(), jcall!(self.displayInfo).getLastHeight());
        jcall!(graphics).setColor(self.foregroundColor.clone());
        if !J2MEUtil::isJ2ME() {
            jcall!(graphics).drawString(self.getTitle(), 8, 1, 0);
        }
        { let java_arguments = (graphics.clone(), 8, jcall!(jcall!(graphics).getFont()).getHeight() + 2,); self.paintXY(java_arguments.0, java_arguments.1, java_arguments.2) };
        { let java_arguments = (graphics.clone(),); self.base.paint(java_arguments.0) };
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) {
        jcall!(self.textFieldItem).paintXY(graphics.clone(), x.clone(), y.clone());
    }
    

    pub fn getTextFieldItem(&mut self) -> Option<Rc<RefCell<ABTextFieldItem>>> {
        return self.textFieldItem.clone();
    }
}
impl std::ops::Deref for CustomTextBox {
    type Target = GameCommandCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CustomTextBox {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CustomTextBoxDyn: crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn + crate::org::allbinary::game::input::event::RawKeyEventListener::RawKeyEventListener {
    fn as_CustomTextBox(&self) -> &CustomTextBox;
    fn as_CustomTextBox_mut(&mut self) -> &mut CustomTextBox;
    fn submit(&mut self) { CustomTextBox::submit(self.as_CustomTextBox_mut()) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { CustomTextBox::paintXY(self.as_CustomTextBox_mut(), graphics, x, y) }
    fn getTextFieldItem(&mut self) -> Option<Rc<RefCell<ABTextFieldItem>>> { CustomTextBox::getTextFieldItem(self.as_CustomTextBox_mut()) }
}
impl CustomTextBoxDyn for CustomTextBox {
    fn as_CustomTextBox(&self) -> &CustomTextBox { self }
    fn as_CustomTextBox_mut(&mut self) -> &mut CustomTextBox { self }
}

impl crate::javax::microedition::lcdui::Canvas::CanvasDyn for CustomTextBox {
    fn as_Canvas(&self) -> &crate::javax::microedition::lcdui::Canvas::Canvas { &self.base.base.base }
    fn as_Canvas_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Canvas::Canvas { &mut self.base.base.base }
}
impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for CustomTextBox {
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
impl crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn for CustomTextBox {
    fn as_GameCommandCanvas(&self) -> &crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &self.base }
    fn as_GameCommandCanvas_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &mut self.base }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for CustomTextBox {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base }
    fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyPressedByDevice(self, keyCode, deviceId) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyReleasedByDevice(self, keyCode, deviceId) }
}
impl crate::org::allbinary::graphics::displayable::DisplayableInterface::DisplayableInterface for CustomTextBox {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for CustomTextBox {
    fn updateMeasurement(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::updateMeasurement(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface for CustomTextBox {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeAllCommands(&mut self) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeAllCommands(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
}


