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
// use crate::java::awt::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub canvasStrings: Option<Rc<RefCell<CanvasStrings>>>,
    pub isDrawn: bool,
    pub bufferedImage: Option<Rc<RefCell<BufferedImage>>>,
}


impl ImageJPanel {
    

    pub fn new_struct(mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            canvasStrings: None,
            isDrawn: false,
            bufferedImage: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.canvasStrings = CanvasStrings::getInstance();
        this.isDrawn = false;
        this.initComponents();
        this.bufferedImage = bufferedImage.clone();
        { let java_arguments = (jcall!(this.bufferedImage).getWidth(None), jcall!(this.bufferedImage).getHeight(None),); this.setSize(java_arguments.0, java_arguments.1) };
        this
    }

    pub fn new(mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(bufferedImage))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        if !self.isDrawn {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).PAINT.clone());
            jcall!(graphics).drawImage(self.bufferedImage.clone(), 0, 0, jcall!(self.bufferedImage).getWidth(None), jcall!(self.bufferedImage).getHeight(None), None);
            self.isDrawn = true;
        }
    }
    

    pub fn initComponents(&mut self) {
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 400, i16::MAX.clone()));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 300, i16::MAX.clone()));
    }
}
impl std::ops::Deref for ImageJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



