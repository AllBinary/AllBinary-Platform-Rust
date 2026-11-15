// 
// AllBinary Open License Version 1 
// Copyright (c) 2026 AllBinary 
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
// package org.allbinary.java.runtime.process.git
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::io::File::File;
use crate::java::io::IOException::IOException;
// use crate::java::nio::charset::StandardCharsets::StandardCharsets;
use crate::java::util::ArrayList::ArrayList;
// use crate::java::util::List::List;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::AbFileNativeUtil::AbFileNativeUtil;
use crate::org::allbinary::logic::io::file::directory::TrackedStrings::TrackedStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::ProcessBuilder::ProcessBuilder;
use super::Process::Process;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Do not use this instead just skip target, build, Application paths.
pub struct GitProcessHelper {
    pub trackedStrings: Option<Rc<RefCell<TrackedStrings>>>,
}

pub static GitProcessHelper_instance: crate::JavaStatic<Option<Rc<RefCell<GitProcessHelper>>>> = crate::JavaStatic::new(|| GitProcessHelper::new());

impl GitProcessHelper {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GitProcessHelper>>> {
        return GitProcessHelper_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            trackedStrings: None,
        };
        this.trackedStrings = TrackedStrings::getInstance();
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut THIS_IS_SLOW: String = String::from("This is slow consider path exclusion instead");
        jcall!(logUtil).putF(THIS_IS_SLOW.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws IOException, InterruptedException
    pub fn trackedFiles(&mut self, mut rootAsString: String, mut pathspec: String) -> Option<Rc<RefCell<dyn List<String>>>> {
        return self.trackedFiles3(rootAsString.clone(), pathspec.clone(), String::from(".java"));
    }
    

    // throws IOException, InterruptedException
    // Java overloaded method: Rust needs a different name
    pub fn trackedFiles3(&mut self, mut rootAsString: String, mut pathspec: String, mut suffix: String) -> Option<Rc<RefCell<dyn List<String>>>> {
        let mut process: Option<Rc<RefCell<Process>>> = jcall!(jcall!(ProcessBuilder::new({ let java_field = jcall!(self.trackedStrings).GIT_COMMAND.clone(); java_field }.clone(), { let java_field = jcall!(self.trackedStrings).CHANGE_DIRECTORY_OPTION.clone(); java_field }.clone(), rootAsString.clone(), { let java_field = jcall!(self.trackedStrings).LIST_FILES_COMMAND.clone(); java_field }.clone(), { let java_field = jcall!(self.trackedStrings).SEP_BY_NULL_CHAR_INSTEAD_OF_NEW_LINE.clone(); java_field }.clone(), { let java_field = jcall!(self.trackedStrings).PATHSPEC_SEPARATOR.clone(); java_field }.clone(), pathspec.clone())).redirectError({ let java_field = jcall!(unsafe { ProcessBuilder_Redirect.clone() }).INHERIT.clone(); java_field }.clone())).start();
        let mut output: Vec<i8> = jcall!(jcall!(process).getInputStream()).readAllBytes();
        let mut exitCode: i32 = jcall!(process).waitFor();
        if exitCode != 0 {
            panic!("{:?}", format!("{}: {:?}", "IllegalStateException", (jcall!(StringMaker::new()).append(String::from("git ls-files failed for ")).append(rootAsString.clone()).append(String::from(" with exit code ")).appendint(exitCode.clone()).toString())));
        }
        let mut files: Option<Rc<RefCell<dyn List<String>>>> = ArrayList::new();
        let mut start: i32 = 0;
        let mut relativePath: String = String::new();
        for index in (0..(output.len() as i32)) {
            if ((output[(index) as usize]) as i32) == 0 {
                relativePath = { let java_value = (String::new(output.clone(), start.clone(), index - start, unsafe { crate::java::nio::charset::StandardCharsets::StandardCharsets_UTF_8.clone() }.clone())).clone(); java_value };
                if relativePath.ends_with(&suffix.clone()) {
                    jcall!(files).add(relativePath.clone());
                }
                start = index + 1;
            }
        }
        return files.clone();
    }
    

    pub fn isTracked(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut nativeFile: Option<Rc<RefCell<File>>> = AbFileNativeUtil::get(file.clone());
            let mut parentFile: Option<Rc<RefCell<File>>> = jcall!(nativeFile).getParentFile();
            let mut process: Option<Rc<RefCell<Process>>> = jcall!(jcall!(ProcessBuilder::new({ let java_field = jcall!(self.trackedStrings).GIT_COMMAND.clone(); java_field }.clone(), { let java_field = jcall!(self.trackedStrings).CHANGE_DIRECTORY_OPTION.clone(); java_field }.clone(), jcall!(parentFile).getPath(), { let java_field = jcall!(self.trackedStrings).LIST_FILES_COMMAND.clone(); java_field }.clone(), { let java_field = jcall!(self.trackedStrings).ERROR_UNMATCH_OPTION.clone(); java_field }.clone(), { let java_field = jcall!(self.trackedStrings).PATHSPEC_SEPARATOR.clone(); java_field }.clone(), jcall!(nativeFile).getName())).redirectErrorStream(true)).start();
            jcall!(jcall!(process).getInputStream()).readAllBytes();
            return Some(jcall!(process).waitFor() == 0);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

