// Generated Code Do Not Modify
// package org.allbinary.media.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BufferedImageUtil2 {
    pub imageUtil: Option<Rc<RefCell<ImageUtil>>>,
}

thread_local! { pub static BufferedImageUtil2_instance: Option<Rc<RefCell<BufferedImageUtil2>>> = BufferedImageUtil2::new(); }

impl BufferedImageUtil2 {

    pub fn new_struct() -> Self {
        let mut this = Self {
            imageUtil: None,
        };
        this.imageUtil = ImageUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BufferedImageUtil2>>> {
        return BufferedImageUtil2_instance.with(Clone::clone).clone();
    }
    

    pub fn createSpriteImage(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>>) -> Option<Rc<RefCell<BufferedImage>>> {
        let mut columns: i32 = (bufferedImageArray.len() as i32).clone();
        let mut max: i32 = columns.clone();
        let mut rows: i32 = 0;
        if (bufferedImageArray.len() as i32) < columns {
            columns = (bufferedImageArray.len() as i32).clone();
        }
        rows = ((bufferedImageArray.len() as i32) / columns);
        if (bufferedImageArray.len() as i32) % columns != 0 {
            rows += 1;
        }
        let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = { let java_receiver = self.imageUtil.clone(); let java_arguments = (jcall!(bufferedImageArray[(0) as usize]).getWidth(None) * columns, jcall!(bufferedImageArray[(0) as usize]).getHeight(None) * rows,); let java_result = jcall!(java_receiver).create(java_arguments.0, java_arguments.1); java_result };
        let mut g: Option<Rc<RefCell<Graphics2D>>> = jcall!(bufferedImage).createGraphics();
        let mut columnIndex: i32 = 0;
        let mut rowIndex: i32 = 0;
        for index in (0..(bufferedImageArray.len() as i32)) {
            if index / max != 0 && index % max == 0 {
                rowIndex += 1;
                columnIndex = 0;
            }
            jcall!(g).drawImage(bufferedImageArray[(index) as usize].clone(), jcall!(bufferedImageArray[(index) as usize]).getWidth(None) * columnIndex, jcall!(bufferedImageArray[(index) as usize]).getHeight(None) * rowIndex, jcall!(bufferedImageArray[(index) as usize]).getWidth(None), jcall!(bufferedImageArray[(index) as usize]).getHeight(None), None);
            columnIndex += 1;
        }
        jcall!(g).dispose();
        return bufferedImage.clone();
    }
}

