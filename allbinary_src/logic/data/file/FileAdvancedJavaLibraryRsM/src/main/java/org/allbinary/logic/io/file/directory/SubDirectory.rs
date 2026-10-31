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
// package org.allbinary.logic.io.file.directory
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::FileFilter::FileFilter;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::Directory::Directory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SubDirectory {
    pub directory: Option<Rc<RefCell<Directory>>>,
}

thread_local! { pub static SubDirectory_instance: Option<Rc<RefCell<SubDirectory>>> = SubDirectory::new(); }

impl SubDirectory {
    

    pub fn getInstance() -> Option<Rc<RefCell<SubDirectory>>> {
        return SubDirectory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            directory: None,
        };
        this.directory = Directory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn search2(&mut self, mut fileFilter: Option<Rc<RefCell<FileFilter>>>, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return jcall!(self.directory).search3(fileFilter.clone(), file.clone(), true);
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn search(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return jcall!(self.directory).search2(file.clone(), true);
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn search2(&mut self, mut searchValue: String, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return jcall!(self.directory).search3(searchValue.clone(), file.clone(), true);
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn search2(&mut self, mut level: i32, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return jcall!(self.directory).search3(level.clone(), file.clone(), true);
    }
    

    pub fn toString(mut files: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut NEW_LINE: String = jcall!(CommonSeps::getInstance()).NEW_LINE.clone();
        let mut size: i32 = jcall!(files).size();
        for index in (0..size) {
            let mut file: Option<Rc<RefCell<AbFile>>> = { let java_object = (jcall!(files).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<AbFile>>>>().expect("ClassCastException").clone()) };
            jcall!(stringBuffer).append(jcall!(file).getPath());
            jcall!(stringBuffer).append(NEW_LINE.clone());
        }
        return jcall!(stringBuffer).toString();
    }
}

