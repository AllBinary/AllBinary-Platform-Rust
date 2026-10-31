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
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FileFactory::FileFactory;
use crate::org::allbinary::logic::io::file::FileWrapperUtil::FileWrapperUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Directory {
}

thread_local! { pub static Directory_instance: Option<Rc<RefCell<Directory>>> = Directory::new(); }

impl Directory {
    

    pub fn getInstance() -> Option<Rc<RefCell<Directory>>> {
        return Directory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn create(&mut self, mut directoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> bool {
        return { let java_arguments = (jcall!(directoryAbPath).toFileSystemString(),); self.create(java_arguments.0) };
    }
    

    // throws Exception
    pub fn remove(&mut self, mut existingDirectoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) {
        { let java_arguments = (jcall!(existingDirectoryAbPath).toFileSystemString(),); self.remove(java_arguments.0) };
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn create(&mut self, mut directory: String) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut directoryFile: Option<Rc<RefCell<AbFile>>> = jcall!(FileFactory::getInstance()).getInstance1(directory.clone());
            if !jcall!(directoryFile).isDirectory() {
                PreLogUtil::put(format!("{}{}", String::from("Creating Directories: "), directory), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("create"));
                if !jcall!(directoryFile).mkdirs() {
                    return false;
                }
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(format!("{}{}", String::from("Error Creating Directories: "), directory), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("create"), e.clone());
            return false;
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn remove(&mut self, mut existingDirectory: String) {
        let mut existingDirectoryFile: Option<Rc<RefCell<AbFile>>> = jcall!(FileFactory::getInstance()).getInstance1(existingDirectory.clone());
        if jcall!(existingDirectoryFile).isDirectory() {
            if (jcall!(existingDirectoryFile).list().len() as i32) > 0 {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", format!("{}{}", String::from("Did not remove category: "), existingDirectory), String::from(" because files exist")))));
            } else if jcall!(existingDirectoryFile).delete() {
                PreLogUtil::put(format!("{}{}", String::from("Successfully Removed: "), existingDirectory), String::from("Directory"), String::from("remove"));
            }
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Category Directory Is Missing:"), existingDirectory))));
        }
    }
    

    pub fn search2(&mut self, mut fileFilter: Option<Rc<RefCell<FileFilter>>>, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.search3(fileFilter.clone(), file.clone(), false);
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn search3(&mut self, mut fileFilter: Option<Rc<RefCell<FileFilter>>>, mut file: Option<Rc<RefCell<AbFile>>>, mut isRecursiveSearch: bool) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut fileList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut recursiveFileList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        if jcall!(file).isDirectory() {
            let mut fileArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(file).listFilesFileFilter(fileFilter.clone());
            if fileArray.is_none() {
                return fileList.clone();
            }
            let mut files: Vec<Option<Rc<RefCell<AbFile>>>> = FileWrapperUtil::wrapFiles(fileArray.clone());
            if files.is_none() {
                return fileList.clone();
            }
            let mut size: i32 = (files.len() as i32).clone();
            for index in (0..size) {
                jcall!(fileList).add(crate::java_into_object(files[(index) as usize].clone()));
                if isRecursiveSearch {
                    recursiveFileList = self.search3(fileFilter.clone(), files[(index) as usize].clone(), isRecursiveSearch.clone());
                    jcall!(fileList).addAllList(recursiveFileList.clone());
                }
            }
        }
        return fileList.clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn search(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.search2(file.clone(), false);
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn search2(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut isRecursiveSearch: bool) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut fileList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut recursiveFileList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        if jcall!(file).isDirectory() {
            let mut fileArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(file).listFiles();
            if fileArray.is_none() {
                return fileList.clone();
            }
            let mut files: Vec<Option<Rc<RefCell<AbFile>>>> = FileWrapperUtil::wrapFiles(fileArray.clone());
            if files.is_none() {
                return fileList.clone();
            }
            for index in (0..(files.len() as i32)) {
                jcall!(fileList).add(crate::java_into_object(files[(index) as usize].clone()));
                if isRecursiveSearch {
                    recursiveFileList = self.search2(files[(index) as usize].clone(), isRecursiveSearch.clone());
                    jcall!(fileList).addAllList(recursiveFileList.clone());
                }
            }
        }
        return fileList.clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn search2(&mut self, mut searchValue: String, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.search3(searchValue.clone(), file.clone(), false);
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn search3(&mut self, mut searchValue: String, mut file: Option<Rc<RefCell<AbFile>>>, mut isRecursiveSearch: bool) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut fileList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut recursiveFileList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        if jcall!(file).isDirectory() {
            let mut fileArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(file).listFiles();
            if fileArray.is_none() {
                return fileList.clone();
            }
            let mut files: Vec<Option<Rc<RefCell<AbFile>>>> = FileWrapperUtil::wrapFiles(fileArray.clone());
            if files.is_none() {
                return fileList.clone();
            }
            for index in (0..(files.len() as i32)) {
                if jcall!(jcall!(files[(index) as usize]).getPath()).indexOf(searchValue.clone()) >= 0 {
                    jcall!(fileList).add(crate::java_into_object(files[(index) as usize].clone()));
                }
                if isRecursiveSearch {
                    recursiveFileList = self.search3(searchValue.clone(), files[(index) as usize].clone(), isRecursiveSearch.clone());
                    jcall!(fileList).addAllList(recursiveFileList.clone());
                }
            }
        }
        return fileList.clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn search2(&mut self, mut level: i32, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.search3(level.clone(), file.clone(), false);
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn search3(&mut self, mut level: i32, mut file: Option<Rc<RefCell<AbFile>>>, mut isRecursiveSearch: bool) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut fileList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut recursiveFileList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        if jcall!(file).isDirectory() {
            let mut fileArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(file).listFiles();
            if fileArray.is_none() {
                return fileList.clone();
            }
            let mut files: Vec<Option<Rc<RefCell<AbFile>>>> = FileWrapperUtil::wrapFiles(fileArray.clone());
            if files.is_none() {
                return fileList.clone();
            }
            for index in (0..(files.len() as i32)) {
                jcall!(fileList).add(crate::java_into_object(files[(index) as usize].clone()));
                if level <= 0 {
                    return fileList.clone();
                }
                recursiveFileList = self.search3(level - 1, files[(index) as usize].clone(), isRecursiveSearch.clone());
                jcall!(fileList).addAllList(recursiveFileList.clone());
            }
        }
        return fileList.clone();
    }
}

