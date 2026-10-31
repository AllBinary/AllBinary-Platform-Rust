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
// package org.allbinary.logic.io.file.visitor
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::BooleanFileVisitor::BooleanFileVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ExcludeFileExtensionsBooleanFileVisitor {
    pub base: BooleanFileVisitor,
}


impl ExcludeFileExtensionsBooleanFileVisitor {
    

    pub fn new_struct(mut filterStringBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            base: BooleanFileVisitor::new_struct(filterStringBasicArrayList.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut filterStringBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(filterStringBasicArrayList))))
    }
    

    pub fn visit(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<bool> {
        if jcall!(self.getFilterStringBasicArrayList()).size() == 0 {
            return true;
        }
        return self.base.visit(file.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn visit2(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut fileNameString: String) -> Option<bool> {
        let mut fileNameAndExtension: String = jcall!(file).getPath();
        if (fileNameAndExtension.chars().count() as i32) > (fileNameString.chars().count() as i32) {
            let mut startIndex: i32 = (fileNameAndExtension.chars().count() as i32) - (fileNameString.chars().count() as i32);
            let mut fileName: String = fileNameAndExtension[(startIndex.clone()) as usize..].to_string();
            if (fileName.cmp(&fileNameString.clone()) as i32) == 0 {
                return false;
            }
        }
        return true;
    }
}
impl std::ops::Deref for ExcludeFileExtensionsBooleanFileVisitor {
    type Target = BooleanFileVisitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ExcludeFileExtensionsBooleanFileVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::io::file::visitor::BooleanFileVisitor::BooleanFileVisitorDyn for ExcludeFileExtensionsBooleanFileVisitor {
    fn as_BooleanFileVisitor(&self) -> &crate::org::allbinary::logic::io::file::visitor::BooleanFileVisitor::BooleanFileVisitor { &self.base }
    fn as_BooleanFileVisitor_mut(&mut self) -> &mut crate::org::allbinary::logic::io::file::visitor::BooleanFileVisitor::BooleanFileVisitor { &mut self.base }
    fn visit2(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut fileNameString: String) -> Option<bool> { Self::visit2(self, file, fileNameString) }
}


