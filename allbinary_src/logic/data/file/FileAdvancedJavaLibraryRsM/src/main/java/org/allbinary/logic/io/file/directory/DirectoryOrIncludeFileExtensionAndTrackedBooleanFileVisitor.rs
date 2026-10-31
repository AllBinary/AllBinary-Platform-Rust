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
use crate::org::allbinary::java::runtime::process::git::GitProcessHelper::GitProcessHelper;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::CommonDataFileStrings::CommonDataFileStrings;
use crate::org::allbinary::logic::io::file::visitor::IncludeFileExtensionsBooleanFileVisitor::IncludeFileExtensionsBooleanFileVisitor;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::TrackedStrings::TrackedStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DirectoryOrIncludeFileExtensionAndTrackedBooleanFileVisitor {
    pub base: IncludeFileExtensionsBooleanFileVisitor,
    pub trackedStrings: Option<Rc<RefCell<TrackedStrings>>>,
    pub gitProcessHelper: Option<Rc<RefCell<GitProcessHelper>>>,
    pub includesString: String,
}


impl DirectoryOrIncludeFileExtensionAndTrackedBooleanFileVisitor {
    

    pub fn new_struct1(mut filterStringBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            base: IncludeFileExtensionsBooleanFileVisitor::new_struct(filterStringBasicArrayList.clone()),
            trackedStrings: None,
            gitProcessHelper: None,
            includesString: String::new(),
        };
        this.trackedStrings = TrackedStrings::getInstance();
        this.gitProcessHelper = GitProcessHelper::getInstance();
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
            trackedStrings: None,
            gitProcessHelper: None,
            includesString: String::new(),
        };
        this.trackedStrings = TrackedStrings::getInstance();
        this.gitProcessHelper = GitProcessHelper::getInstance();
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
    

    // Java overloaded method: Rust needs a different name
    pub fn visit2(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut fileNameString: String) -> Option<bool> {
        let mut filePath: String = jcall!(file).getPath();
        if self.base.visit2(file.clone(), fileNameString.clone()) {
            if !(filePath.contains(&jcall!(self.trackedStrings).APPLICATION.clone()) || filePath.contains(&jcall!(self.trackedStrings).APP.clone()) || filePath.contains(&jcall!(self.trackedStrings).HTML_TEMP.clone())) {
                if !(filePath.contains(&jcall!(self.trackedStrings).TARGET_PATH_WINDOWS.clone()) || filePath.contains(&jcall!(self.trackedStrings).TARGET_PATH_UNIX.clone())) {
                    if jcall!(self.gitProcessHelper).isTracked(file.clone()) {
                        return true;
                    } else {
                    }
                } else {
                }
            } else {
            }
        }
        return false;
    }
    

    pub fn main(mut args: Vec<String>) {
        let mut commonFileStrings: Option<Rc<RefCell<CommonDataFileStrings>>> = CommonDataFileStrings::getInstance();
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(list).add(crate::java_into_object(jcall!(commonFileStrings).JAVA.clone()));
        let mut filePath: String = String::from("G:\\mnt\\bc\\mydev\\games\\ZeptoRacer\\platform\\j2se\\ZeptoRacerStaticPathsJ2SEM\\target\\dependency\\org\\allbinary\\logic\\system\\security\\licensing\\ZeptoRacerPCClientInformation.java");
        let mut result: bool = jcall!(DirectoryOrIncludeFileExtensionAndTrackedBooleanFileVisitor::new1(list.clone())).visit(AbFile::createAbFileFromRawPath(filePath.clone()));
        println!("{}", format!("{}{}", format!("{}{}", filePath, String::from(" is Tracked: ")), result));
    }
}
impl std::ops::Deref for DirectoryOrIncludeFileExtensionAndTrackedBooleanFileVisitor {
    type Target = IncludeFileExtensionsBooleanFileVisitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DirectoryOrIncludeFileExtensionAndTrackedBooleanFileVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::io::file::visitor::BooleanFileVisitor::BooleanFileVisitorDyn for DirectoryOrIncludeFileExtensionAndTrackedBooleanFileVisitor {
    fn as_BooleanFileVisitor(&self) -> &crate::org::allbinary::logic::io::file::visitor::BooleanFileVisitor::BooleanFileVisitor { &self.base.base }
    fn as_BooleanFileVisitor_mut(&mut self) -> &mut crate::org::allbinary::logic::io::file::visitor::BooleanFileVisitor::BooleanFileVisitor { &mut self.base.base }
    fn visit2(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut fileNameString: String) -> Option<bool> { Self::visit2(self, file, fileNameString) }
}
impl crate::org::allbinary::logic::io::file::visitor::IncludeFileExtensionsBooleanFileVisitor::IncludeFileExtensionsBooleanFileVisitorDyn for DirectoryOrIncludeFileExtensionAndTrackedBooleanFileVisitor {
    fn as_IncludeFileExtensionsBooleanFileVisitor(&self) -> &crate::org::allbinary::logic::io::file::visitor::IncludeFileExtensionsBooleanFileVisitor::IncludeFileExtensionsBooleanFileVisitor { &self.base }
    fn as_IncludeFileExtensionsBooleanFileVisitor_mut(&mut self) -> &mut crate::org::allbinary::logic::io::file::visitor::IncludeFileExtensionsBooleanFileVisitor::IncludeFileExtensionsBooleanFileVisitor { &mut self.base }
    fn visit(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<bool> { Self::visit(self, file) }
}


