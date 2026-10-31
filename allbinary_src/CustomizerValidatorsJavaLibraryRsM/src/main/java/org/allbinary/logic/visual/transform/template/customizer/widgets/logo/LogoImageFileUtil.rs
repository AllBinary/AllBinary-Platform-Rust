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
// package org.allbinary.logic.visual.transform.template.customizer.widgets.logo
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FileUtil::FileUtil;
use crate::org::allbinary::logic::io::file::directory::Directory::Directory;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::apache::commons::fileupload::FileItem::FileItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LogoImageFileUtil {
    pub directory: Option<Rc<RefCell<Directory>>>,
    pub fileName: String,
    pub imageFileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
}


impl LogoImageFileUtil {
    

    pub fn new_struct(mut imageAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut imageFile: String) -> Self {
        let mut this = Self {
            directory: None,
            fileName: String::new(),
            imageFileAbPath: None,
        };
        this.directory = Directory::getInstance();
        this.fileName = imageFile.clone();
        this.imageFileAbPath = imageAbPath.clone();
        this
    }

    pub fn new(mut imageAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut imageFile: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageAbPath, imageFile))))
    }
    

    // throws Exception
    pub fn saveFiles(&mut self, mut fileItem: Option<Rc<RefCell<FileItem>>>) {
        if false {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Image File Name Was Null"))));
        }
        if !jcall!(self.directory).create(self.imageFileAbPath.clone()) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Unable to save file to non creatable directory"))));
        }
        let mut originalImageFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(format!("{}{}", jcall!(self.imageFileAbPath).toString(), self.fileName));
        jcall!(originalImageFile).createNewFile();
        jcall!(FileUtil::getInstance()).write(ByteArrayInputStream::new(jcall!(fileItem).get()), originalImageFile.clone());
    }
}

