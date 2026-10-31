// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::game::Sprite::Sprite;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::SpriteFactoryInterface::SpriteFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SpriteFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static SpriteFactory_instance: Option<Rc<RefCell<SpriteFactory>>> = SpriteFactory::new(); }
impl SpriteFactoryInterface for SpriteFactory {
    fn createSprite(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut frameWidth: i32, mut frameHeight: i32) -> Option<Rc<RefCell<Sprite>>> { Self::createSprite(self, image, frameWidth, frameHeight) }
}

impl SpriteFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<SpriteFactory>>> {
        return SpriteFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn createSprite(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut frameWidth: i32, mut frameHeight: i32) -> Option<Rc<RefCell<Sprite>>> {
        return Sprite::new(image.clone(), frameWidth.clone(), frameHeight.clone());
    }
}

