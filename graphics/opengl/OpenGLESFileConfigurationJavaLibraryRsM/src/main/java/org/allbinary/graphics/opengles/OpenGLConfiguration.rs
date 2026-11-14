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
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::io::Closeable::Closeable;
use crate::java::io::DataInputStream::DataInputStream;
use crate::java::io::InputStream::InputStream;
use crate::java::io::OutputStream::OutputStream;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::io::AbDataOutputStream::AbDataOutputStream;
use crate::org::allbinary::logic::io::FileStreamFactory::FileStreamFactory;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::FileFactory::FileFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::game::configuration::event::ChangedGameFeatureListener::ChangedGameFeatureListener;
use crate::org::allbinary::game::configuration::feature::Feature::Feature;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::MainFeatureFactory::MainFeatureFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::io::NullCloseable::NullCloseable;
use crate::java::lang::Exception::*;
use super::OpenGLFeature::OpenGLFeature;
use super::OpenGLFeatureFactory::OpenGLFeatureFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLConfiguration {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub FILE: String,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
    pub opengl: bool,
    pub r#type: Option<Rc<RefCell<OpenGLFeature>>>,
    pub imageColor: Option<Rc<RefCell<OpenGLFeature>>>,
    pub color: Option<Rc<RefCell<OpenGLFeature>>>,
    pub versionSelector: Option<Rc<RefCell<OpenGLFeature>>>,
}

pub static mut OpenGLConfiguration_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());

impl OpenGLConfiguration {
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLConfiguration>>> {
        if match (&unsafe { OpenGLConfiguration_instance.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *OpenGLConfiguration_instance = crate::java_into_object(OpenGLConfiguration::new()) };
        }
        return { let java_object = (unsafe { OpenGLConfiguration_instance.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<OpenGLConfiguration>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            FILE: String::new(),
            commonStrings: None,
            tsUtil: None,
            opengl: false,
            r#type: None,
            imageColor: None,
            color: None,
            versionSelector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.FILE = String::from("OpenGLConfiguration.dat");
        this.commonStrings = CommonStrings::getInstance();
        this.tsUtil = TsUtil::getInstance();
        this.opengl = false;
        this.r#type = { let java_field = jcall!(OpenGLFeatureFactory::getInstance()).OPENGL_AS_GAME_THREAD.clone(); java_field }.clone();
        this.imageColor = { let java_field = jcall!(OpenGLFeatureFactory::getInstance()).IMAGE_COLOR_DEPTH_4444.clone(); java_field }.clone();
        this.color = { let java_field = jcall!(OpenGLFeatureFactory::getInstance()).IMAGE_COLOR_DEPTH_4444.clone(); java_field }.clone();
        this.versionSelector = { let java_field = jcall!(OpenGLFeatureFactory::getInstance()).OPENGL_AUTO_SELECT.clone(); java_field }.clone();
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

            jcall!(this.logUtil).put({ let java_field = jcall!(this.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn read(&mut self) {
        let mut openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>> = OpenGLFeatureFactory::getInstance();
        let mut fileInputStreamFactory: Option<Rc<RefCell<FileStreamFactory>>> = FileStreamFactory::getInstance();
        let mut fileInputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> = jcall!(fileInputStreamFactory).getFileInputStreamInstance({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), self.FILE.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>);
        let mut dataInputStream: Option<Rc<RefCell<DataInputStream>>> = DataInputStream::new(fileInputStream.clone());
        let mut openGLValue: i32 = jcall!(dataInputStream).readInt();
        if openGLValue == 0 {
            self.setOpenGL(false);
        } else if openGLValue == 1 {
            self.setOpenGL(true);
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Invalid OpenGL Setting"))));
        }
        let mut version: String = jcall!(dataInputStream).readUTF();
        if { let left = (version).clone(); let right = (jcall!({ let java_field = jcall!(openGLFeatureFactory).OPENGL_AUTO_SELECT.clone(); java_field }).getName()).clone(); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            { let java_arguments = ({ let java_field = jcall!(openGLFeatureFactory).OPENGL_AUTO_SELECT.clone(); java_field }.clone(),); self.setVersionSelector(java_arguments.0) };
        } else if { let left = (version).clone(); let right = (jcall!({ let java_field = jcall!(openGLFeatureFactory).OPENGL_MINIMUM.clone(); java_field }).getName()).clone(); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            { let java_arguments = ({ let java_field = jcall!(openGLFeatureFactory).OPENGL_MINIMUM.clone(); java_field }.clone(),); self.setVersionSelector(java_arguments.0) };
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("OpenGLConfiguration: Error reading version selector: "), version))));
        }
        let mut r#type: String = jcall!(dataInputStream).readUTF();
        if { let left = (r#type).clone(); let right = (jcall!({ let java_field = jcall!(openGLFeatureFactory).OPENGL_AS_GAME_THREAD.clone(); java_field }).getName()).clone(); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            { let java_arguments = ({ let java_field = jcall!(openGLFeatureFactory).OPENGL_AS_GAME_THREAD.clone(); java_field }.clone(),); self.setType(java_arguments.0) };
        } else if { let left = (r#type).clone(); let right = (jcall!({ let java_field = jcall!(openGLFeatureFactory).OPENGL_AND_GAME_HAVE_DIFFERENT_THREADS.clone(); java_field }).getName()).clone(); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            { let java_arguments = ({ let java_field = jcall!(openGLFeatureFactory).OPENGL_AND_GAME_HAVE_DIFFERENT_THREADS.clone(); java_field }.clone(),); self.setType(java_arguments.0) };
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("OpenGLConfiguration: Error reading image color"))));
        }
        let mut imageColor: String = jcall!(dataInputStream).readUTF();
        if { let left = (imageColor).clone(); let right = (jcall!({ let java_field = jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_4444.clone(); java_field }).getName()).clone(); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            { let java_arguments = ({ let java_field = jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_4444.clone(); java_field }.clone(),); self.setImageColor(java_arguments.0) };
        } else if { let left = (imageColor).clone(); let right = (jcall!({ let java_field = jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_4444.clone(); java_field }).getName()).clone(); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            { let java_arguments = ({ let java_field = jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_4444.clone(); java_field }.clone(),); self.setImageColor(java_arguments.0) };
        } else if { let left = (imageColor).clone(); let right = (jcall!({ let java_field = jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_4444.clone(); java_field }).getName()).clone(); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            { let java_arguments = ({ let java_field = jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_4444.clone(); java_field }.clone(),); self.setImageColor(java_arguments.0) };
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("OpenGLConfiguration: Error reading image color: "), imageColor))));
        }
        let mut color: String = jcall!(dataInputStream).readUTF();
        if { let left = (color).clone(); let right = (jcall!({ let java_field = jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_4444.clone(); java_field }).getName()).clone(); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            { let java_arguments = ({ let java_field = jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_4444.clone(); java_field }.clone(),); self.setColor(java_arguments.0) };
        } else if { let left = (color).clone(); let right = (jcall!({ let java_field = jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_565.clone(); java_field }).getName()).clone(); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            { let java_arguments = ({ let java_field = jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_565.clone(); java_field }.clone(),); self.setColor(java_arguments.0) };
        } else if { let left = (color).clone(); let right = (jcall!({ let java_field = jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_8888.clone(); java_field }).getName()).clone(); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            { let java_arguments = ({ let java_field = jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_8888.clone(); java_field }.clone(),); self.setColor(java_arguments.0) };
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("OpenGLConfiguration: Error reading color: "), color))));
        }
        PreLogUtil::put(format!("{}{}", String::from("Read Configuration: "), self.toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("read"));
    }
    

    // throws Exception
    pub fn write(&mut self) {
        let mut closeable: Option<Rc<RefCell<dyn Closeable>>> = crate::org::allbinary::logic::io::NullCloseable::NullCloseable_NULL_CLOSEABLE.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn Closeable>>);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            PreLogUtil::put(format!("{}{}", String::from("Write Configuration: "), self.toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("write"));
            let mut fileInputStreamFactory: Option<Rc<RefCell<FileStreamFactory>>> = FileStreamFactory::getInstance();
            let mut fileOutputStream: Option<Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>> = jcall!(fileInputStreamFactory).getFileOutputStreamInstance({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), self.FILE.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>);
            let mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>> = AbDataOutputStream::new(fileOutputStream.clone());
            closeable = dataOutputStream.clone().map(|reference| reference as Rc<RefCell<dyn Closeable>>);
            if self.isOpenGL() {
                jcall!(dataOutputStream).writeInt(1);
            } else {
                jcall!(dataOutputStream).writeInt(0);
            }
            jcall!(dataOutputStream).writeUTF({ let java_argument = jcall!(self.getVersionSelector()).getName(); java_argument });
            jcall!(dataOutputStream).writeUTF({ let java_argument = jcall!(self.getType()).getName(); java_argument });
            jcall!(dataOutputStream).writeUTF({ let java_argument = jcall!(self.getImageColor()).getName(); java_argument });
            jcall!(dataOutputStream).writeUTF({ let java_argument = jcall!(self.getColor()).getName(); java_argument });
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
    pub fn init(&mut self) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if jcall!(ChangedGameFeatureListener::getInstance()).isChangedFeature({ let java_field = jcall!(MainFeatureFactory::getInstance()).STATIC.clone(); java_field }.clone()) {
            let mut openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>> = OpenGLFeatureFactory::getInstance();
            if self.isOpenGL() {
                if jcall!(features).isFeature({ let java_field = jcall!(openGLFeatureFactory).OPENGL_2D.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || jcall!(features).isFeature({ let java_field = jcall!(openGLFeatureFactory).OPENGL_3D.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || jcall!(features).isFeature({ let java_field = jcall!(openGLFeatureFactory).OPENGL_2D_AND_3D.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
                    if !jcall!(features).isDefault({ let java_field = jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
                        jcall!(self.logUtil).putF(String::from("Turning on OpenGL as Default"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone());
                        jcall!(features).addDefault({ let java_field = jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Using OpenGL Type Feature: "), self.getType()), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                        jcall!(features).addDefault(self.getType().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Using OpenGL ImageColor Feature: "), self.getImageColor()), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                        jcall!(features).addDefault(self.getImageColor().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Using OpenGL Color Feature: "), self.getColor()), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                        jcall!(features).addDefault(self.getColor().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Using OpenGL Version Selector Feature: "), self.getVersionSelector()), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                        jcall!(features).addDefault(self.getVersionSelector().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
                    }
                } else {
                    jcall!(self.logUtil).putF(String::from("OpenGL is enabled, but this game does not support OpenGL (Probably from installing with the same package name from a game that had OpenGL available)"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone());
                }
            } else {
                jcall!(self.logUtil).putF(String::from("OpenGL is Off"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone());
            }
        } else {
            if self.isOpenGL() && !jcall!(features).isDefault({ let java_field = jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
                PreLogUtil::put(String::from("OpenGL is set but not Enabled since statics where not cleared (Reboot or Complete exit is Required)"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone());
            }
        }
        PreLogUtil::put(self.toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone());
    }
    

    // throws Exception
    pub fn update(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>, mut colorLocked: bool) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>> = OpenGLFeatureFactory::getInstance();
        let mut modified: bool = false;
        if match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).OPENGL.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(features).isFeature(gameFeature.clone()) {
                if !self.isOpenGL() {
                    self.setOpenGL(true);
                    modified = true;
                }
            } else {
                if self.isOpenGL() {
                    self.setOpenGL(false);
                    modified = true;
                }
            }
        }
        if match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).OPENGL_AND_GAME_HAVE_DIFFERENT_THREADS.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).OPENGL_AS_GAME_THREAD.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(features).isFeature(gameFeature.clone()) {
                if !match (&gameFeature, &self.getType()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    self.setType((gameFeature).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<OpenGLFeature>() }, "ClassCastException: not a OpenGLFeature"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<OpenGLFeature>) } }));
                    modified = true;
                }
            }
        }
        if match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_4444.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_565.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_8888.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(features).isFeature(gameFeature.clone()) {
                if !match (&self.getImageColor(), &gameFeature) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    self.setImageColor((gameFeature).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<OpenGLFeature>() }, "ClassCastException: not a OpenGLFeature"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<OpenGLFeature>) } }));
                    if colorLocked {
                        if match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_4444.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                            { let java_arguments = ({ let java_field = jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_4444.clone(); java_field }.clone(),); self.setColor(java_arguments.0) };
                        } else if match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_565.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                            { let java_arguments = ({ let java_field = jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_565.clone(); java_field }.clone(),); self.setColor(java_arguments.0) };
                        } else if match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_8888.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                            { let java_arguments = ({ let java_field = jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_8888.clone(); java_field }.clone(),); self.setColor(java_arguments.0) };
                        }
                    }
                    modified = true;
                }
            }
        }
        if match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_4444.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_565.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_8888.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(features).isFeature(gameFeature.clone()) {
                if !match (&self.getColor(), &gameFeature) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    self.setColor((gameFeature).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<OpenGLFeature>() }, "ClassCastException: not a OpenGLFeature"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<OpenGLFeature>) } }));
                    modified = true;
                }
            }
        }
        if match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).OPENGL_AUTO_SELECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&gameFeature, &{ let java_field = jcall!(openGLFeatureFactory).OPENGL_MINIMUM.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(features).isFeature(gameFeature.clone()) {
                self.setVersionSelector((gameFeature).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<OpenGLFeature>() }, "ClassCastException: not a OpenGLFeature"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<OpenGLFeature>) } }));
            }
            modified = true;
        }
        if modified {
            self.write();
        }
    }
    

    pub fn setOpenGL(&mut self, mut opengl: bool) {
        self.opengl = opengl.clone();
    }
    

    pub fn isOpenGL(&mut self) -> bool {
        return self.opengl.clone();
    }
    

    pub fn setImageColor(&mut self, mut imageColor: Option<Rc<RefCell<OpenGLFeature>>>) {
        self.imageColor = imageColor.clone();
    }
    

    pub fn getImageColor(&mut self) -> Option<Rc<RefCell<OpenGLFeature>>> {
        return self.imageColor.clone();
    }
    

    pub fn setColor(&mut self, mut color: Option<Rc<RefCell<OpenGLFeature>>>) {
        self.color = color.clone();
    }
    

    pub fn getColor(&mut self) -> Option<Rc<RefCell<OpenGLFeature>>> {
        return self.color.clone();
    }
    

    pub fn setVersionSelector(&mut self, mut versionSelector: Option<Rc<RefCell<OpenGLFeature>>>) {
        self.versionSelector = versionSelector.clone();
    }
    

    pub fn getVersionSelector(&mut self) -> Option<Rc<RefCell<OpenGLFeature>>> {
        return self.versionSelector.clone();
    }
    

    pub fn setType(&mut self, mut r#type: Option<Rc<RefCell<OpenGLFeature>>>) {
        self.r#type = r#type.clone();
    }
    

    pub fn getType(&mut self) -> Option<Rc<RefCell<OpenGLFeature>>> {
        return self.r#type.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        jcall!(stringBuffer).append(String::from(" isOpenGL: "));
        jcall!(stringBuffer).appendboolean(self.isOpenGL());
        jcall!(stringBuffer).append(String::from(" VersionSelector: "));
        jcall!(stringBuffer).append({ let java_argument = jcall!(stringUtil).toString(crate::java_into_object(self.getVersionSelector())); java_argument });
        jcall!(stringBuffer).append(String::from(" Type: "));
        jcall!(stringBuffer).append({ let java_argument = jcall!(stringUtil).toString(crate::java_into_object(self.getType())); java_argument });
        jcall!(stringBuffer).append(String::from(" Image Color: "));
        jcall!(stringBuffer).append({ let java_argument = jcall!(stringUtil).toString(crate::java_into_object(self.getImageColor())); java_argument });
        jcall!(stringBuffer).append(String::from(" Color: "));
        jcall!(stringBuffer).append({ let java_argument = jcall!(stringUtil).toString(crate::java_into_object(self.getColor())); java_argument });
        return jcall!(stringBuffer).toString();
    }
}

