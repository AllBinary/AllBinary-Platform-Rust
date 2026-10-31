// Generated Code Do Not Modify
// package org.allbinary.logic.visual.media
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::java::awt::image::RenderedImage::RenderedImage;
// use crate::javax::imageio::ImageIO::ImageIO;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::AbFileNativeUtil::AbFileNativeUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageIOUtil {
}


impl ImageIOUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn read(mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<BufferedImage>>> {
        return ImageIO::read(AbFileNativeUtil::get(file.clone()));
    }
    

    // throws Exception
    pub fn write(mut renderedImage: Option<Rc<RefCell<RenderedImage>>>, mut name: String, mut file: Option<Rc<RefCell<AbFile>>>) -> bool {
        return ImageIO::write(renderedImage.clone(), name.clone(), AbFileNativeUtil::get(file.clone()));
    }
}

