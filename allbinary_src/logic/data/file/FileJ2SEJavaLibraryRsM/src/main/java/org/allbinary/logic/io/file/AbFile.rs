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
// use crate::java::io::File::File;
// use crate::java::io::FileFilter::FileFilter;
// use crate::java::io::FilenameFilter::FilenameFilter;
// use crate::java::io::IOException::IOException;
// use crate::java::net::URI::URI;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::AbFileInterface::AbFileInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

fn java_string_hash(text: &str) -> i32 {
    text.chars().fold(0i32, |hash, character| hash.wrapping_mul(31).wrapping_add(character as i32))
}

pub struct AbFile {
    pub file: Option<Rc<RefCell<File>>>,
}

thread_local! { pub static AbFile_NULL_FILE: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromRawPath(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()); }
impl AbFileInterface for AbFile {
    fn canRead(&mut self) -> bool { Self::canRead(self) }
    fn canWrite(&mut self) -> bool { Self::canWrite(self) }
    fn createNewFile(&mut self) -> bool { Self::createNewFile(self) }
    fn delete(&mut self) -> bool { Self::delete(self) }
    fn deleteOnExit(&mut self) { Self::deleteOnExit(self) }
    fn exists(&mut self) -> bool { Self::exists(self) }
    fn getAbsolutePath(&mut self) -> String { Self::getAbsolutePath(self) }
    fn getCanonicalPath(&mut self) -> String { Self::getCanonicalPath(self) }
    fn getParent(&mut self) -> String { Self::getParent(self) }
    fn getPath(&mut self) -> String { Self::getPath(self) }
    fn isAbsolute(&mut self) -> bool { Self::isAbsolute(self) }
    fn isDirectory(&mut self) -> bool { Self::isDirectory(self) }
    fn isFile(&mut self) -> bool { Self::isFile(self) }
    fn isHidden(&mut self) -> bool { Self::isHidden(self) }
    fn lastModified(&mut self) -> i64 { Self::lastModified(self) }
    fn length(&mut self) -> i64 { Self::length(self) }
    fn list(&mut self) -> Vec<String> { Self::list(self) }
    fn listWithFilter(&mut self, mut filter: Option<Rc<RefCell<FilenameFilter>>>) -> Vec<String> { Self::listWithFilter(self, filter) }
    fn mkdir(&mut self) -> bool { Self::mkdir(self) }
    fn mkdirs(&mut self) -> bool { Self::mkdirs(self) }
    fn setLastModified(&mut self, mut time: i64) -> bool { Self::setLastModified(self, time) }
    fn setReadOnly(&mut self) -> bool { Self::setReadOnly(self) }
    fn toString(&mut self) -> String { Self::toString(self) }
    fn toURI(&mut self) -> Option<Rc<RefCell<URI>>> { Self::toURI(self) }
}

impl AbFile {
    

    // throws Exception
    pub fn createAbFileWithChild(mut file: Option<Rc<RefCell<AbFile>>>, mut childPathName: String) -> Option<Rc<RefCell<AbFile>>> {
        return AbFile::new(File::new(jcall!(file).getFile(), childPathName.clone()));
    }
    

    // throws Exception
    pub fn createAbFile(mut filePath: String) -> Option<Rc<RefCell<AbFile>>> {
        return AbFile::new(File::new(jcall!(AbPath::new(filePath.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).toFileSystemString()));
    }
    

    // throws Exception
    pub fn createAbFilePathAndName(mut filePath: String, mut fileName: String) -> Option<Rc<RefCell<AbFile>>> {
        return AbFile::new(File::new(jcall!(AbPath::new(filePath.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).toFileSystemString(), fileName.clone()));
    }
    

    pub fn createAbFileFromAbPath(mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<AbFile>>> {
        return AbFile::new(File::new(jcall!(abPath).toFileSystemString()));
    }
    

    pub fn createAbFileFromRawPath(mut filePath: String) -> Option<Rc<RefCell<AbFile>>> {
        return AbFile::new(File::new(filePath.clone()));
    }
    

    pub fn new_struct(mut file: Option<Rc<RefCell<File>>>) -> Self {
        let mut this = Self {
            file: None,
        };
        this.file = file.clone();
        this
    }

    pub fn new(mut file: Option<Rc<RefCell<File>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(file))))
    }
    

    pub fn getFile(&mut self) -> Option<Rc<RefCell<File>>> {
        return self.file.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return jcall!(self.file).getName();
    }
    

    pub fn getParent(&mut self) -> String {
        return jcall!(self.file).getParent();
    }
    

    pub fn getParentFile(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(self.file).getParentFile();
    }
    

    pub fn getPath(&mut self) -> String {
        return jcall!(self.file).getPath();
    }
    

    pub fn isAbsolute(&mut self) -> bool {
        return jcall!(self.file).isAbsolute();
    }
    

    pub fn getAbsolutePath(&mut self) -> String {
        return jcall!(self.file).getAbsolutePath();
    }
    

    pub fn getAbsoluteFile(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(self.file).getAbsoluteFile();
    }
    

    // throws IOException
    pub fn getCanonicalPath(&mut self) -> String {
        return jcall!(self.file).getCanonicalPath();
    }
    

    // throws IOException
    pub fn getCanonicalFile(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(self.file).getCanonicalFile();
    }
    

    pub fn toURI(&mut self) -> Option<Rc<RefCell<URI>>> {
        return jcall!(self.file).toURI();
    }
    

    pub fn canRead(&mut self) -> bool {
        return jcall!(self.file).canRead();
    }
    

    pub fn canWrite(&mut self) -> bool {
        return jcall!(self.file).canWrite();
    }
    

    pub fn exists(&mut self) -> bool {
        return jcall!(self.file).exists();
    }
    

    pub fn isDirectory(&mut self) -> bool {
        return jcall!(self.file).isDirectory();
    }
    

    pub fn isFile(&mut self) -> bool {
        return jcall!(self.file).isFile();
    }
    

    pub fn isHidden(&mut self) -> bool {
        return jcall!(self.file).isHidden();
    }
    

    pub fn lastModified(&mut self) -> i64 {
        return jcall!(self.file).lastModified();
    }
    

    pub fn length(&mut self) -> i64 {
        return ((jcall!(self.file).length()) as i64);
    }
    

    // throws IOException
    pub fn createNewFile(&mut self) -> bool {
        return jcall!(self.file).createNewFile();
    }
    

    // throws IOException
    pub fn delete(&mut self) -> bool {
        return jcall!(self.file).delete();
    }
    

    pub fn deleteOnExit(&mut self) {
        jcall!(self.file).deleteOnExit();
    }
    

    pub fn list(&mut self) -> Vec<String> {
        return jcall!(self.file).list();
    }
    

    pub fn listWithFilter(&mut self, mut filter: Option<Rc<RefCell<FilenameFilter>>>) -> Vec<String> {
        return jcall!(self.file).list(filter.clone());
    }
    

    pub fn listFiles(&mut self) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        return (jcall!(self.file).listFiles()).clone() /* as  */;
    }
    

    pub fn listFilesFileNameFilter(&mut self, mut filter: Option<Rc<RefCell<FilenameFilter>>>) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        return (jcall!(self.file).listFiles(filter.clone())).clone() /* as  */;
    }
    

    pub fn listFilesFileFilter(&mut self, mut filter: Option<Rc<RefCell<FileFilter>>>) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        return (jcall!(self.file).listFiles(filter.clone())).clone() /* as  */;
    }
    

    pub fn mkdir(&mut self) -> bool {
        return jcall!(self.file).mkdir();
    }
    

    pub fn mkdirs(&mut self) -> bool {
        return jcall!(self.file).mkdirs();
    }
    

    pub fn renameTo(&mut self, mut dest: Option<Rc<RefCell<AbFile>>>) -> bool {
        return { let java_receiver = self.file.clone(); let java_arguments = (jcall!(dest).getFile(),); let java_result = jcall!(java_receiver).renameTo(java_arguments.0); java_result };
    }
    

    pub fn setLastModified(&mut self, mut time: i64) -> bool {
        return jcall!(self.file).setLastModified(time.clone());
    }
    

    pub fn setReadOnly(&mut self) -> bool {
        return jcall!(self.file).setReadOnly();
    }
    

    pub fn setWritable2(&mut self, mut writable: bool, mut ownerOnly: bool) -> bool {
        return jcall!(self.file).setExecutable2(writable.clone(), ownerOnly.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn setWritable(&mut self, mut writable: bool) -> bool {
        return jcall!(self.file).setWritable(writable.clone());
    }
    

    pub fn setReadable2(&mut self, mut readable: bool, mut ownerOnly: bool) -> bool {
        return jcall!(self.file).setReadable2(readable.clone(), ownerOnly.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn setReadable(&mut self, mut readable: bool) -> bool {
        return jcall!(self.file).setReadable(readable.clone());
    }
    

    pub fn setExecutable2(&mut self, mut executable: bool, mut ownerOnly: bool) -> bool {
        return jcall!(self.file).setExecutable2(executable.clone(), ownerOnly.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn setExecutable(&mut self, mut executable: bool) -> bool {
        return jcall!(self.file).setExecutable(executable.clone());
    }
    

    pub fn canExecute(&mut self) -> bool {
        return jcall!(self.file).canExecute();
    }
    

    pub fn getTotalSpace(&mut self) -> i64 {
        return jcall!(self.file).getTotalSpace();
    }
    

    pub fn getFreeSpace(&mut self) -> i64 {
        return jcall!(self.file).getFreeSpace();
    }
    

    pub fn getUsableSpace(&mut self) -> i64 {
        return jcall!(self.file).getUsableSpace();
    }
    

    pub fn compareTo(&mut self, mut pathname: Option<Rc<RefCell<File>>>) -> i32 {
        return jcall!(self.file).compareTo(pathname.clone());
    }
    

    pub fn equals(&mut self, mut obj: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return jcall!(TsUtil::getInstance()).equalsNotstring(crate::java_into_object(self.file.clone()), crate::java_into_object(obj.clone()));
    }
    

    pub fn hashCode(&mut self) -> i32 {
        return jcall!(self.file).hashCode();
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(self.file).toString();
    }
}

