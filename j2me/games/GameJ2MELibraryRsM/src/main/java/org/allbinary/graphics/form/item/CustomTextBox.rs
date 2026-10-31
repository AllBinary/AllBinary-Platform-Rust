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
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
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
    fn open(&mut self) { self.base.open() }
    fn close(&mut self) { self.base.close() }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for CustomTextBox {
    fn getSourceId(&mut self) -> i32 { self.base.getSourceId() }
}

impl CustomTextBox {
    

    pub fn new_struct(mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut label: String, mut text: String, mut maxSize: i32, mut constraints: i32, mut font: Option<Rc<RefCell<Font>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: GameCommandCanvas::new_struct(cmdListener.clone(), label.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            textFieldItem: None,
            inputFactory: None,
        };
        this.inputFactory = InputFactory::getInstance();
        // super(...) is in the struct literal above
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        jcall!(textFieldItem).setString(text.clone());
        this.textFieldItem = textFieldItem.clone();
        this.setTitle(label.clone());
        this
    }

    pub fn new(mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut label: String, mut text: String, mut maxSize: i32, mut constraints: i32, mut font: Option<Rc<RefCell<Font>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(cmdListener, label, text, maxSize, constraints, font, backgroundBasicColor, foregroundBasicColor))));
            let mut textFieldItem: Option<Rc<RefCell<ABTextFieldItem>>> = ABTextFieldItem::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), TextItemVisitor::new(), jcall!(stringUtil).EMPTY_STRING.clone(), jcall!(stringUtil).EMPTY_STRING.clone(), maxSize.clone(), 0, jcall!(stringUtil).EMPTY_STRING.clone(), font.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone());
        this
    }
    

    pub fn submit(&mut self) {
    }
    

    pub fn onEventRaw(&mut self, mut keyCode: i32, mut deviceId: i32, mut repeated: bool) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.commonStrings).START.clone()).appendint(keyCode.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onEvent"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
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
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).appendint(keyCode.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).KEY_PRESSED.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
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
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).KEY_PRESSED.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn keyReleasedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(graphics).setColor(self.backgroundColor.clone());
        jcall!(graphics).fillRect(0, 0, jcall!(self.displayInfo).getLastWidth(), jcall!(self.displayInfo).getLastHeight());
        jcall!(graphics).setColor(self.foregroundColor.clone());
        if !J2MEUtil::isJ2ME() {
            jcall!(graphics).drawString(self.getTitle(), 8, 1, 0);
        }
        { let java_arguments = (graphics.clone(), 8, jcall!(jcall!(graphics).getFont()).getHeight() + 2,); self.paintXY(java_arguments.0, java_arguments.1, java_arguments.2) };
        self.base.paint(graphics.clone());
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
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
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { CustomTextBox::paintXY(self.as_CustomTextBox_mut(), graphics, x, y) }
    fn getTextFieldItem(&mut self) -> Option<Rc<RefCell<ABTextFieldItem>>> { CustomTextBox::getTextFieldItem(self.as_CustomTextBox_mut()) }
}
impl CustomTextBoxDyn for CustomTextBox {
    fn as_CustomTextBox(&self) -> &CustomTextBox { self }
    fn as_CustomTextBox_mut(&mut self) -> &mut CustomTextBox { self }
}

impl crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn for CustomTextBox {
    fn as_GameCommandCanvas(&self) -> &crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &self.base }
    fn as_GameCommandCanvas_mut(&mut self) -> &mut crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas { &mut self.base }
    fn keyPressed(&mut self, mut keyCode: i32) { Self::keyPressed(self, keyCode) }
    fn keyReleased(&mut self, mut keyCode: i32) { Self::keyReleased(self, keyCode) }
    fn keyRepeated(&mut self, mut keyCode: i32) { Self::keyRepeated(self, keyCode) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for CustomTextBox {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn keyPressedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyPressedByDevice(self, keyCode, deviceId) }
    fn keyRepeatedByDevice(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvas::keyRepeatedByDevice(crate::org::allbinary::game::displayable::canvas::GameCommandCanvas::GameCommandCanvasDyn::as_GameCommandCanvas_mut(self), p1, p2) }
    fn keyReleasedByDevice(&mut self, mut keyCode: i32, mut deviceId: i32) { Self::keyReleasedByDevice(self, keyCode, deviceId) }
}


