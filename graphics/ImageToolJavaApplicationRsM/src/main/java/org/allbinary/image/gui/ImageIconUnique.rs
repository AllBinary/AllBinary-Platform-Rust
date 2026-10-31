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
// package org.allbinary.image.gui
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::javax::swing::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageIconUnique {
    pub base: ImageIcon,
    pub id: i32,
}


impl ImageIconUnique {
    

    pub fn new_struct(mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut index: i32) -> Self {
        let mut this = Self {
            base: ImageIcon::new_struct1(bufferedImage.clone()),
            id: 0,
        };
        // super(...) is in the struct literal above
        this.setId(index.clone());
        this
    }

    pub fn new(mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut index: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(bufferedImage, index))))
    }
    

    pub fn getId(&mut self) -> i32 {
        return self.id.clone();
    }
    

    pub fn setId(&mut self, mut id: i32) {
        self.id = id.clone();
    }
}
impl std::ops::Deref for ImageIconUnique {
    type Target = ImageIcon;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageIconUnique {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



