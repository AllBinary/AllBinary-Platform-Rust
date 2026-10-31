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
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::visitor::IncludeFileExtensionsBooleanFileVisitor::IncludeFileExtensionsBooleanFileVisitor;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DirectoryOrIncludeFileExtensionBooleanFileVisitor {
    pub base: IncludeFileExtensionsBooleanFileVisitor,
    pub includesString: String,
}


impl DirectoryOrIncludeFileExtensionBooleanFileVisitor {
    

    pub fn new_struct1(mut filterStringBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            base: IncludeFileExtensionsBooleanFileVisitor::new_struct(filterStringBasicArrayList.clone()),
            includesString: String::new(),
        };
        // super(...) is in the struct literal above
        this.includesString = None;
        this
    }

    pub fn new1(mut filterStringBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(filterStringBasicArrayList))))
    }
    

    pub fn new_struct2(mut filterStringBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut includesString: String) -> Self {
        let mut this = Self {
            base: IncludeFileExtensionsBooleanFileVisitor::new_struct(filterStringBasicArrayList.clone()),
            includesString: String::new(),
        };
        // super(...) is in the struct literal above
        this.includesString = includesString.clone();
        this
    }

    pub fn new2(mut filterStringBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut includesString: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(filterStringBasicArrayList, includesString))))
    }
    

    pub fn visit(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<bool> {
        if jcall!(file).isDirectory() {
            return true;
        }
        if false || jcall!(jcall!(file).getAbsolutePath()).indexOf(self.includesString.clone()) >= 0 {
            return self.base.visit(file.clone());
        } else {
            return false;
        }
    }
}
impl std::ops::Deref for DirectoryOrIncludeFileExtensionBooleanFileVisitor {
    type Target = IncludeFileExtensionsBooleanFileVisitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DirectoryOrIncludeFileExtensionBooleanFileVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::io::file::visitor::BooleanFileVisitor::BooleanFileVisitorDyn for DirectoryOrIncludeFileExtensionBooleanFileVisitor {
    fn as_BooleanFileVisitor(&self) -> &crate::org::allbinary::logic::io::file::visitor::BooleanFileVisitor::BooleanFileVisitor { &self.base.base }
    fn as_BooleanFileVisitor_mut(&mut self) -> &mut crate::org::allbinary::logic::io::file::visitor::BooleanFileVisitor::BooleanFileVisitor { &mut self.base.base }
}
impl crate::org::allbinary::logic::io::file::visitor::IncludeFileExtensionsBooleanFileVisitor::IncludeFileExtensionsBooleanFileVisitorDyn for DirectoryOrIncludeFileExtensionBooleanFileVisitor {
    fn as_IncludeFileExtensionsBooleanFileVisitor(&self) -> &crate::org::allbinary::logic::io::file::visitor::IncludeFileExtensionsBooleanFileVisitor::IncludeFileExtensionsBooleanFileVisitor { &self.base }
    fn as_IncludeFileExtensionsBooleanFileVisitor_mut(&mut self) -> &mut crate::org::allbinary::logic::io::file::visitor::IncludeFileExtensionsBooleanFileVisitor::IncludeFileExtensionsBooleanFileVisitor { &mut self.base }
    fn visit(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<bool> { Self::visit(self, file) }
}


