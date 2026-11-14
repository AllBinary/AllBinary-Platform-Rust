// Generated Code Do Not Modify
// package org.allbinary.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::io::Closeable::Closeable;
use crate::java::io::InputStream::InputStream;
use crate::java::io::OutputStream::OutputStream;
use crate::org::allbinary::game::configuration::feature::Feature::Feature;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::MainFeatureFactory::MainFeatureFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::AbDataInputStream::AbDataInputStream;
use crate::org::allbinary::logic::io::AbDataOutputStream::AbDataOutputStream;
use crate::org::allbinary::logic::io::FileStreamFactory::FileStreamFactory;
use crate::org::allbinary::logic::io::NullCloseable::NullCloseable;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::FileFactory::FileFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ApplicationConfiguration {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub fullscreen: bool,
    pub showTitleBar: bool,
    pub progressBarView: bool,
    pub FILE: String,
}

pub static mut ApplicationConfiguration_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());

impl ApplicationConfiguration {
    

    pub fn getInstance() -> Option<Rc<RefCell<ApplicationConfiguration>>> {
        if match (&unsafe { ApplicationConfiguration_instance.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *ApplicationConfiguration_instance = crate::java_into_object(ApplicationConfiguration::new()) };
        }
        return { let java_object = (unsafe { ApplicationConfiguration_instance.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<ApplicationConfiguration>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            fullscreen: false,
            showTitleBar: false,
            progressBarView: false,
            FILE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.fullscreen = true;
        this.showTitleBar = false;
        this.progressBarView = false;
        this.FILE = String::from("ReloadConfiguration.dat");
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(FileFactory::getInstance()).isFile(this.FILE.clone()) {
                this.read();
            } else {
                this.write();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).CONSTRUCTOR.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn read(&mut self) {
        let mut fileInputStreamFactory: Option<Rc<RefCell<FileStreamFactory>>> = FileStreamFactory::getInstance();
        let mut fileInputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> = jcall!(fileInputStreamFactory).getFileInputStreamInstance({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), self.FILE.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>);
        let mut dataInputStream: Option<Rc<RefCell<AbDataInputStream>>> = AbDataInputStream::new(fileInputStream.clone());
        let mut fullScreen: i32 = jcall!(dataInputStream).readInt();
        if fullScreen == 0 {
            self.setFullscreen(false);
        } else if fullScreen == 1 {
            self.setFullscreen(true);
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Invalid FullScreen ActivityConfiguration"))));
        }
        let mut progressBarView: i32 = jcall!(dataInputStream).readInt();
        if progressBarView == 0 {
            self.setProgressBarView(false);
        } else if progressBarView == 1 {
            self.setProgressBarView(true);
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Invalid ProgressBarView ActivityConfiguration"))));
        }
        let mut showTitleBar: i32 = jcall!(dataInputStream).readInt();
        if showTitleBar == 0 {
            self.setShowTitleBar(false);
        } else if showTitleBar == 1 {
            self.setShowTitleBar(true);
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Invalid ShowTitleBar ActivityConfiguration"))));
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Read Configuration: "), self.toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("read"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    // throws Exception
    pub fn write(&mut self) {
        let mut closeable: Option<Rc<RefCell<dyn Closeable>>> = crate::org::allbinary::logic::io::NullCloseable::NullCloseable_NULL_CLOSEABLE.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn Closeable>>);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Write Configuration: "), self.toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("write"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            let mut fileInputStreamFactory: Option<Rc<RefCell<FileStreamFactory>>> = FileStreamFactory::getInstance();
            let mut fileOutputStream: Option<Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>> = jcall!(fileInputStreamFactory).getFileOutputStreamInstance({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), self.FILE.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>);
            let mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>> = AbDataOutputStream::new(fileOutputStream.clone());
            closeable = dataOutputStream.clone().map(|reference| reference as Rc<RefCell<dyn Closeable>>);
            if self.isFullscreen() {
                jcall!(dataOutputStream).writeInt(1);
            } else {
                jcall!(dataOutputStream).writeInt(0);
            }
            if self.isProgressBarView() {
                jcall!(dataOutputStream).writeInt(1);
            } else {
                jcall!(dataOutputStream).writeInt(0);
            }
            if self.isShowTitleBar() {
                jcall!(dataOutputStream).writeInt(1);
            } else {
                jcall!(dataOutputStream).writeInt(0);
            }
            jcall!(dataOutputStream).flush();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            panic!("{:?}", e);
            }
        }
        { // finally
            jcall!(StreamUtil::getInstance()).close(closeable.clone());
        }
    }
    

    // throws Exception
    pub fn update(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>) {
        if match (&gameFeature, &{ let java_field = jcall!(MainFeatureFactory::getInstance()).FULL_SCREEN.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
            if jcall!(features).isFeature(gameFeature.clone()) {
                if !self.isFullscreen() {
                    self.setFullscreen(true);
                    self.write();
                }
            } else {
                if self.isFullscreen() {
                    self.setFullscreen(false);
                    self.write();
                }
            }
        }
    }
    

    pub fn setFullscreen(&mut self, mut fullscreen: bool) {
        self.fullscreen = fullscreen.clone();
    }
    

    pub fn isFullscreen(&mut self) -> bool {
        return self.fullscreen.clone();
    }
    

    pub fn setShowTitleBar(&mut self, mut showTitleBar: bool) {
        self.showTitleBar = showTitleBar.clone();
    }
    

    pub fn isShowTitleBar(&mut self) -> bool {
        return self.showTitleBar.clone();
    }
    

    pub fn setProgressBarView(&mut self, mut progressBarView: bool) {
        self.progressBarView = progressBarView.clone();
    }
    

    pub fn isProgressBarView(&mut self) -> bool {
        return self.progressBarView.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("isFullscreen: "));
        jcall!(stringBuffer).appendboolean(self.isFullscreen());
        jcall!(stringBuffer).append(String::from(" isProgressBarView: "));
        jcall!(stringBuffer).appendboolean(self.isProgressBarView());
        jcall!(stringBuffer).append(String::from(" isShowTitleBar: "));
        jcall!(stringBuffer).appendboolean(self.isShowTitleBar());
        return jcall!(stringBuffer).toString();
    }
}

