// Generated Code Do Not Modify
// package org.platform.graphics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::image::opengles::OpenGLImageUtilBase::OpenGLImageUtilBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLImageUtil {
    pub base: OpenGLImageUtilBase,
}

thread_local! { pub static OpenGLImageUtil_instance: Option<Rc<RefCell<OpenGLImageUtil>>> = OpenGLImageUtil::new(); }

impl OpenGLImageUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: OpenGLImageUtilBase::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLImageUtil>>> {
        return OpenGLImageUtil_instance.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for OpenGLImageUtil {
    type Target = OpenGLImageUtilBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OpenGLImageUtil {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::image::opengles::OpenGLImageUtilBase::OpenGLImageUtilBaseDyn for OpenGLImageUtil {
    fn as_OpenGLImageUtilBase(&self) -> &crate::org::allbinary::image::opengles::OpenGLImageUtilBase::OpenGLImageUtilBase { &self.base }
    fn as_OpenGLImageUtilBase_mut(&mut self) -> &mut crate::org::allbinary::image::opengles::OpenGLImageUtilBase::OpenGLImageUtilBase { &mut self.base }
}


