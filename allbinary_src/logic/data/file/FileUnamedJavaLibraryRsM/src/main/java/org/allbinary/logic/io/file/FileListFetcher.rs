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
// package org.allbinary.logic.io.file
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::file::directory::DirectoryBooleanFileVisitor::DirectoryBooleanFileVisitor;
use crate::org::allbinary::logic::io::file::directory::DirectoryOrIncludeFileExtensionAndTrackedBooleanFileVisitor::DirectoryOrIncludeFileExtensionAndTrackedBooleanFileVisitor;
use crate::org::allbinary::logic::io::file::directory::DirectoryOrIncludeFileExtensionBooleanFileVisitor::DirectoryOrIncludeFileExtensionBooleanFileVisitor;
use crate::org::allbinary::logic::io::file::directory::SubDirectory::SubDirectory;
use crate::org::allbinary::logic::io::file::filter::VisitorFileFilter::VisitorFileFilter;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FileListFetcher {
    pub subDirectory: Option<Rc<RefCell<SubDirectory>>>,
}

thread_local! { pub static FileListFetcher_instance: Option<Rc<RefCell<FileListFetcher>>> = FileListFetcher::new(); }

impl FileListFetcher {

    pub fn new_struct() -> Self {
        let mut this = Self {
            subDirectory: None,
        };
        this.subDirectory = SubDirectory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<FileListFetcher>>> {
        return FileListFetcher_instance.with(Clone::clone).clone();
    }
    

    pub fn getFiles(&mut self, mut path: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut files: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_receiver = self.subDirectory.clone(); let java_arguments = (AbFile::AbFile::createAbFile(path.clone()),); let java_result = jcall!(java_receiver).search(java_arguments.0); java_result };
            return files.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            eprintln!("{:?}", e);
            println!("{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Error: "), e), String::from("\nMsg: ")), e.to_string()));
            }
        }
        return None;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getFiles2(&mut self, mut path: String, mut includeExtensions: Vec<String>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut includeExtensionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut size: i32 = (includeExtensions.len() as i32).clone();
            for index in (0..size) {
                jcall!(includeExtensionBasicArrayList).add(crate::java_into_object(includeExtensions[(index) as usize].clone()));
            }
            let mut visitorFileFilter: Option<Rc<RefCell<VisitorFileFilter>>> = VisitorFileFilter::new(DirectoryOrIncludeFileExtensionBooleanFileVisitor::new1(includeExtensionBasicArrayList.clone()));
            let mut files: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_receiver = self.subDirectory.clone(); let java_arguments = (visitorFileFilter.clone(), AbFile::AbFile::createAbFile(path.clone()),); let java_result = jcall!(java_receiver).search2(java_arguments.0, java_arguments.1); java_result };
            return files.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            eprintln!("{:?}", e);
            println!("{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Error: "), e), String::from("\nMsg: ")), e.to_string()));
            }
        }
        return None;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getFiles2(&mut self, mut path: String, mut includeExtension: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut includeExtensionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(includeExtensionBasicArrayList).add(crate::java_into_object(includeExtension.clone()));
            let mut visitorFileFilter: Option<Rc<RefCell<VisitorFileFilter>>> = VisitorFileFilter::new(DirectoryOrIncludeFileExtensionBooleanFileVisitor::new1(includeExtensionBasicArrayList.clone()));
            let mut files: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_receiver = self.subDirectory.clone(); let java_arguments = (visitorFileFilter.clone(), AbFile::AbFile::createAbFile(path.clone()),); let java_result = jcall!(java_receiver).search2(java_arguments.0, java_arguments.1); java_result };
            return files.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            eprintln!("{:?}", e);
            println!("{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Error: "), e), String::from("\nMsg: ")), e.to_string()));
            }
        }
        return None;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getFiles3(&mut self, mut path: String, mut pathIncludes: String, mut includeExtension: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut includeExtensionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(includeExtensionBasicArrayList).add(crate::java_into_object(includeExtension.clone()));
            let mut visitorFileFilter: Option<Rc<RefCell<VisitorFileFilter>>> = VisitorFileFilter::new(DirectoryOrIncludeFileExtensionBooleanFileVisitor::new2(includeExtensionBasicArrayList.clone(), pathIncludes.clone()));
            let mut files: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_receiver = self.subDirectory.clone(); let java_arguments = (visitorFileFilter.clone(), AbFile::AbFile::createAbFile(path.clone()),); let java_result = jcall!(java_receiver).search2(java_arguments.0, java_arguments.1); java_result };
            return files.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            eprintln!("{:?}", e);
            println!("{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Error: "), e), String::from("\nMsg: ")), e.to_string()));
            }
        }
        return None;
    }
    

    pub fn getTrackedFiles(&mut self, mut path: String, mut includeExtensions: Vec<String>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut includeExtensionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut size: i32 = (includeExtensions.len() as i32).clone();
            for index in (0..size) {
                jcall!(includeExtensionBasicArrayList).add(crate::java_into_object(includeExtensions[(index) as usize].clone()));
            }
            let mut visitorFileFilter: Option<Rc<RefCell<VisitorFileFilter>>> = VisitorFileFilter::new(DirectoryOrIncludeFileExtensionAndTrackedBooleanFileVisitor::new1(includeExtensionBasicArrayList.clone()));
            let mut files: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_receiver = self.subDirectory.clone(); let java_arguments = (visitorFileFilter.clone(), AbFile::AbFile::createAbFile(path.clone()),); let java_result = jcall!(java_receiver).search2(java_arguments.0, java_arguments.1); java_result };
            return files.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            eprintln!("{:?}", e);
            println!("{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Error: "), e), String::from("\nMsg: ")), e.to_string()));
            }
        }
        return None;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getTrackedFiles(&mut self, mut path: String, mut includeExtension: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut includeExtensionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(includeExtensionBasicArrayList).add(crate::java_into_object(includeExtension.clone()));
            let mut visitorFileFilter: Option<Rc<RefCell<VisitorFileFilter>>> = VisitorFileFilter::new(DirectoryOrIncludeFileExtensionAndTrackedBooleanFileVisitor::new1(includeExtensionBasicArrayList.clone()));
            let mut files: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_receiver = self.subDirectory.clone(); let java_arguments = (visitorFileFilter.clone(), AbFile::AbFile::createAbFile(path.clone()),); let java_result = jcall!(java_receiver).search2(java_arguments.0, java_arguments.1); java_result };
            return files.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            eprintln!("{:?}", e);
            println!("{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Error: "), e), String::from("\nMsg: ")), e.to_string()));
            }
        }
        return None;
    }
    

    pub fn getDirectories(&mut self, mut path: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut visitorFileFilter: Option<Rc<RefCell<VisitorFileFilter>>> = VisitorFileFilter::new(DirectoryBooleanFileVisitor::new());
            let mut files: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_receiver = self.subDirectory.clone(); let java_arguments = (visitorFileFilter.clone(), AbFile::AbFile::createAbFile(path.clone()),); let java_result = jcall!(java_receiver).search2(java_arguments.0, java_arguments.1); java_result };
            return files.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            eprintln!("{:?}", e);
            println!("{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Error: "), e), String::from("\nMsg: ")), e.to_string()));
            }
        }
        return None;
    }
}

