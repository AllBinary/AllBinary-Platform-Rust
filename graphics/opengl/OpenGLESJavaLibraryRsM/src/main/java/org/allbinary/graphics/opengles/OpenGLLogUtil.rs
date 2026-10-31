// Generated Code Do Not Modify
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::image::opengles::OpenGLStrings::OpenGLStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLLogUtil {
    pub MAX_TEXTURE: String,
    pub LOG_ERROR: String,
    pub IMAGE: String,
}

thread_local! { pub static OpenGLLogUtil_instance: Option<Rc<RefCell<OpenGLLogUtil>>> = OpenGLLogUtil::new(); }

impl OpenGLLogUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            MAX_TEXTURE: String::new(),
            LOG_ERROR: String::new(),
            IMAGE: String::new(),
        };
        this.MAX_TEXTURE = String::from(" Max Texture Size: ");
        this.LOG_ERROR = String::from("logError");
        this.IMAGE = String::from(" Image: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLLogUtil>>> {
        return OpenGLLogUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn logError(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
        let mut error: i32 = jcall!(gl).glGetError();
        if error != unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_NO_ERROR.clone() } {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(jcall!(OpenGLStrings::getInstance()).GL_ERROR_LABEL.clone());
            jcall!(stringBuffer).appendint(error.clone());
            if error == 1281 {
                jcall!(stringBuffer).append(self.MAX_TEXTURE.clone());
                let mut maxTextureSize: Vec<i32> = vec![0; (1) as usize];
                jcall!(gl).glGetIntegerv(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_MAX_TEXTURE_SIZE.clone() }.clone(), maxTextureSize.clone(), 0);
                jcall!(stringBuffer).appendint(maxTextureSize[(0) as usize].clone());
            }
            if error == 1280 {
                PreLogUtil::put(jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(OpenGLStrings::getInstance()).SET.clone());
            } else {
                ForcedLogUtil::log(jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn logError2(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut image: Option<Rc<RefCell<Image>>>) {
        let mut error: i32 = jcall!(gl).glGetError();
        if error != unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_NO_ERROR.clone() } {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(jcall!(OpenGLStrings::getInstance()).GL_ERROR_LABEL.clone());
            jcall!(stringBuffer).appendint(error.clone());
            if error == 1281 {
                jcall!(stringBuffer).append(self.MAX_TEXTURE.clone());
                let mut maxTextureSize: Vec<i32> = vec![0; (1) as usize];
                jcall!(gl).glGetIntegerv(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_MAX_TEXTURE_SIZE.clone() }.clone(), maxTextureSize.clone(), 0);
                jcall!(stringBuffer).appendint(maxTextureSize[(0) as usize].clone());
            }
            jcall!(stringBuffer).append(self.IMAGE.clone());
            jcall!(stringBuffer).append(jcall!(image).toString());
            PreLogUtil::put(jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.LOG_ERROR.clone());
        }
    }
}

