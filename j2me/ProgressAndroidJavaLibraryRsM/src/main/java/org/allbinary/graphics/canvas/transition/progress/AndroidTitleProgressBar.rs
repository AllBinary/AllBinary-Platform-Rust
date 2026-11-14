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
use crate::Runnable;
use super::*;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::android::activity::NullProgressActivity::NullProgressActivity;
use crate::org::allbinary::android::activity::ProgressActivityInterface::ProgressActivityInterface;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::ProgressCanvas::ProgressCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidTitleProgressBar {
    pub base: ProgressCanvas,
    pub showTitleProgressBarRunnable: Option<Rc<RefCell<ShowTitleProgressBarRunnable>>>,
    pub dismissTitleProgressBarRunnable: Option<Rc<RefCell<DismissTitleProgressBarRunnable>>>,
    pub progressDialogPortionSetProgressRunnable: Option<Rc<RefCell<TitleProgressBarPortionSetProgressRunnable>>>,
    pub progressDialogSetProgressRunnable: Option<Rc<RefCell<TitleProgressBarSetProgressRunnable>>>,
    pub progressActivity: Option<Rc<RefCell<dyn ProgressActivityInterface>>>,
    pub portion: i32,
}

impl crate::org::allbinary::thread::RunnableInterface::RunnableInterface for AndroidTitleProgressBar {
    fn isRunning(&mut self) -> bool { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::isRunning(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self)) }
    fn setRunning(&mut self, p1: bool) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::setRunning(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self), p1) }
    fn setThread(&mut self, p1: Option<Rc<RefCell<crate::java::lang::Thread::Thread>>>) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::setThread(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self), p1) }
}
impl Runnable for AndroidTitleProgressBar {
    fn run(&mut self) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::run(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self)) }
}

impl AndroidTitleProgressBar {
    

    pub fn new_struct(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ProgressCanvas::new_struct(title.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            showTitleProgressBarRunnable: None,
            dismissTitleProgressBarRunnable: None,
            progressDialogPortionSetProgressRunnable: None,
            progressDialogSetProgressRunnable: None,
            progressActivity: None,
            portion: 0,
        };
        this.progressActivity = crate::org::allbinary::android::activity::NullProgressActivity::NullProgressActivity_NULL_PROGRESS_ACTIVITY.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn ProgressActivityInterface>>);
        this.portion = 0;
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(title.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()))));
        jcall!(this).showTitleProgressBarRunnable = ShowTitleProgressBarRunnable::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        jcall!(this).dismissTitleProgressBarRunnable = DismissTitleProgressBarRunnable::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        jcall!(this).progressDialogPortionSetProgressRunnable = TitleProgressBarPortionSetProgressRunnable::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        jcall!(this).progressDialogSetProgressRunnable = TitleProgressBarSetProgressRunnable::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn init(&mut self, mut activity: Option<Rc<RefCell<dyn ProgressActivityInterface>>>) {
        self.progressActivity = activity.clone();
    }
    

    pub fn isInitialized(&mut self) -> bool {
        if !match (&self.progressActivity, &crate::org::allbinary::android::activity::NullProgressActivity::NullProgressActivity_NULL_PROGRESS_ACTIVITY.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn start(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).START_METHOD_NAME.clone(); java_field }.clone());
            self.base.start();
            jcall!(self.progressActivity).runOnUiThread(self.showTitleProgressBarRunnable.clone().map(|reference| reference as Rc<RefCell<dyn Runnable>>));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).START_METHOD_NAME.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn end(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).END_METHOD_NAME.clone(); java_field }.clone());
            jcall!(self.progressActivity).runOnUiThread(self.dismissTitleProgressBarRunnable.clone().map(|reference| reference as Rc<RefCell<dyn Runnable>>));
            self.base.end();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).END_METHOD_NAME.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn addPortion(&mut self, mut value: i32, mut text: String, mut index: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_arguments = (value.clone(), text.clone(), index.clone(),); self.base.addPortion(java_arguments.0, java_arguments.1, java_arguments.2) };
            self.portion = value.clone();
            jcall!(self.progressActivity).runOnUiThread(self.progressDialogPortionSetProgressRunnable.clone().map(|reference| reference as Rc<RefCell<dyn Runnable>>));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ADD_PORTION.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn addNormalPortion(&mut self, mut value: i32, mut text: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_arguments = (value.clone(), text.clone(),); self.base.addNormalPortion(java_arguments.0, java_arguments.1) };
            self.portion = value.clone();
            jcall!(self.progressActivity).runOnUiThread(self.progressDialogPortionSetProgressRunnable.clone().map(|reference| reference as Rc<RefCell<dyn Runnable>>));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ADD_PORTION.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn setValue(&mut self, mut value: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_arguments = (value.clone(),); self.base.setValue(java_arguments.0) };
            jcall!(self.progressActivity).runOnUiThread(self.progressDialogSetProgressRunnable.clone().map(|reference| reference as Rc<RefCell<dyn Runnable>>));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setValue"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn waitUntilDisplayed(&mut self) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
}
impl std::ops::Deref for AndroidTitleProgressBar {
    type Target = ProgressCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidTitleProgressBar {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn for AndroidTitleProgressBar {
    fn as_ProgressCanvas(&self) -> &crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas { &self.base }
    fn as_ProgressCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas { &mut self.base }
    fn start(&mut self) { Self::start(self) }
    fn end(&mut self) { Self::end(self) }
    fn addPortion(&mut self, mut value: i32, mut text: String, mut index: i32) { Self::addPortion(self, value, text, index) }
    fn addNormalPortion(&mut self, mut value: i32, mut text: String) { Self::addNormalPortion(self, value, text) }
    fn setValue(&mut self, mut value: i32) { Self::setValue(self, value) }
}
impl crate::javax::microedition::lcdui::Canvas::CanvasDyn for AndroidTitleProgressBar {
    fn as_Canvas(&self) -> &crate::javax::microedition::lcdui::Canvas::Canvas { &self.base.base.base.base }
    fn as_Canvas_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Canvas::Canvas { &mut self.base.base.base.base }
}
impl crate::javax::microedition::lcdui::Displayable::DisplayableDyn for AndroidTitleProgressBar {
    fn as_Displayable(&self) -> &crate::javax::microedition::lcdui::Displayable::Displayable { &self.base.base.base.base.base }
    fn as_Displayable_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Displayable::Displayable { &mut self.base.base.base.base.base }
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn hideNotify(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::hideNotify(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn keyPressed(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyPressed(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn keyRepeated(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyRepeated(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn keyReleased(&mut self, p1: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::keyReleased(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1) }
    fn pointerPressed(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerPressed(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerReleased(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerReleased(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn pointerDragged(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::pointerDragged(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas::paintThreed(crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn::as_ProgressCanvas_mut(self), p1) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn repaint(&mut self) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaint4(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaint4(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn sizeChanged2(&mut self, p1: i32, p2: i32) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::sizeChanged2(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1, p2) }
    fn showNotify(&mut self) { crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas::showNotify(crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn::as_RunnableCanvas_mut(self)) }
    fn setFullScreenMode(&mut self, p1: bool) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setFullScreenMode(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getTypeAsInt(&mut self) -> i32 { crate::javax::microedition::lcdui::Canvas::Canvas::getTypeAsInt(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
    fn repaintXYWH(&mut self, p1: i32, p2: i32, p3: i32, p4: i32) { crate::javax::microedition::lcdui::Canvas::Canvas::repaintXYWH(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self), p1, p2, p3, p4) }
    fn getDisplayableType(&mut self) -> i32 { crate::javax::microedition::lcdui::Canvas::Canvas::getDisplayableType(crate::javax::microedition::lcdui::Canvas::CanvasDyn::as_Canvas_mut(self)) }
}
impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for AndroidTitleProgressBar {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base.base.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base.base.base }
}
impl crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvasDyn for AndroidTitleProgressBar {
    fn as_RunnableCanvas(&self) -> &crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &self.base.base }
    fn as_RunnableCanvas_mut(&mut self) -> &mut crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas { &mut self.base.base }
    fn initCommands(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas::initCommands(crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn::as_ProgressCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::DisplayableInterface::DisplayableInterface for AndroidTitleProgressBar {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getHeight(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::getWidth(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
    fn removeCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn setCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::CommandListener::CommandListener>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::setCommandListener(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for AndroidTitleProgressBar {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas::paintThreed(crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn::as_ProgressCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for AndroidTitleProgressBar {
    fn updateMeasurement(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas::updateMeasurement(crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn::as_ProgressCanvas_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::command::MyCommandInterface::MyCommandInterface for AndroidTitleProgressBar {
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::addCommand(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self), p1) }
    fn removeAllCommands(&mut self) { crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas::removeAllCommands(crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn::as_MyCanvas_mut(self)) }
}


pub struct TitleProgressBarSetProgressRunnable {
    pub androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>,
}

impl Runnable for TitleProgressBarSetProgressRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl TitleProgressBarSetProgressRunnable {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
        

        pub fn new_struct(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Self {
            let mut this = Self {
            androidTitleProgressBar: None,
            };
            this.androidTitleProgressBar = androidTitleProgressBar.clone();
            this
        }

        pub fn new(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(androidTitleProgressBar))))
        }
        

        pub fn run(&mut self) {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut value: i32 = ((jcall!(self.androidTitleProgressBar).getValue()) as i32);
                jcall!({ let java_field = jcall!(self.androidTitleProgressBar).progressActivity.clone(); java_field }).onTitleProgressBarSetProgress(value.clone());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
}

pub struct TitleProgressBarPortionSetProgressRunnable {
    pub androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>,
}

impl Runnable for TitleProgressBarPortionSetProgressRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl TitleProgressBarPortionSetProgressRunnable {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
        

        pub fn new_struct(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Self {
            let mut this = Self {
            androidTitleProgressBar: None,
            };
            this.androidTitleProgressBar = androidTitleProgressBar.clone();
            this
        }

        pub fn new(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(androidTitleProgressBar))))
        }
        

        pub fn run(&mut self) {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut value: i32 = (((jcall!(self.androidTitleProgressBar).getValue() + jcall!(self.androidTitleProgressBar).getMaxValue() / (({ let java_field = jcall!(self.androidTitleProgressBar).portion.clone(); java_field }) as f32))) as i32);
                jcall!({ let java_field = jcall!(self.androidTitleProgressBar).progressActivity.clone(); java_field }).onTitleProgressBarSetProgress(value.clone());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
}

pub struct ShowTitleProgressBarRunnable {
    pub androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>,
}

impl Runnable for ShowTitleProgressBarRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl ShowTitleProgressBarRunnable {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
        

        pub fn new_struct(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Self {
            let mut this = Self {
            androidTitleProgressBar: None,
            };
            this.androidTitleProgressBar = androidTitleProgressBar.clone();
            this
        }

        pub fn new(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(androidTitleProgressBar))))
        }
        

        pub fn run(&mut self) {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut maxValue: i32 = ((jcall!(self.androidTitleProgressBar).getMaxValue()) as i32);
                jcall!({ let java_field = jcall!(self.androidTitleProgressBar).progressActivity.clone(); java_field }).onShowTitleProgressBar(maxValue.clone(), false);
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
}

pub struct DismissTitleProgressBarRunnable {
    pub androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>,
}

impl Runnable for DismissTitleProgressBarRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl DismissTitleProgressBarRunnable {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
        

        pub fn new_struct(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Self {
            let mut this = Self {
            androidTitleProgressBar: None,
            };
            this.androidTitleProgressBar = androidTitleProgressBar.clone();
            this
        }

        pub fn new(mut androidTitleProgressBar: Option<Rc<RefCell<AndroidTitleProgressBar>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(androidTitleProgressBar))))
        }
        

        pub fn run(&mut self) {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                jcall!({ let java_field = jcall!(self.androidTitleProgressBar).progressActivity.clone(); java_field }).onDismissTitleProgressBar();
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
}

