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
// package org.allbinary.data.tree.dom
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FileData::FileData;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::io::path::PathUtil::PathUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FileTableMapping {
    pub file: Option<Rc<RefCell<AbFile>>>,
}

impl TableMappingInterface for FileTableMapping {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}

impl FileTableMapping {
    

    pub fn new_struct(mut file: Option<Rc<RefCell<AbFile>>>) -> Self {
        let mut this = Self {
            file: None,
        };
        this.file = file.clone();
        this
    }

    pub fn new(mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(file))))
    }
    

    // throws Exception
    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return (jcall!(self.file).getPath()).clone() /* as Any */;
    }
    

    // throws Exception
    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut filePropertyVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(filePropertyVector).add(crate::java_into_object(jcall!(self.file).getAbsolutePath()));
        jcall!(filePropertyVector).add(crate::java_into_object(jcall!(self.file).getCanonicalPath()));
        jcall!(filePropertyVector).add(crate::java_into_object(jcall!(self.file).getName()));
        jcall!(filePropertyVector).add(crate::java_into_object(jcall!(self.file).getParent()));
        jcall!(filePropertyVector).add(crate::java_into_object(jcall!(self.file).getPath()));
        return filePropertyVector.clone();
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut filePropertyHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut pathData: Option<Rc<RefCell<AbPathData>>> = AbPathData::getInstance();
        let mut pathUtil: Option<Rc<RefCell<PathUtil>>> = PathUtil::getInstance();
        let mut rootFileName: String = jcall!(pathData).getNameFromPath(jcall!(pathUtil).getWithoutExtension(jcall!(self.file).getPath()));
        let mut fileData: Option<Rc<RefCell<FileData>>> = FileData::getInstance();
        jcall!(filePropertyHashMap).put(jcall!(fileData).ROOT_NAME.clone(), rootFileName.clone());
        jcall!(filePropertyHashMap).put(jcall!(fileData).ISFILE.clone(), jcall!((jcall!(self.file).isFile()).to_string()).toString());
        jcall!(filePropertyHashMap).put(jcall!(fileData).ISDIRECTORY.clone(), jcall!((jcall!(self.file).isDirectory()).to_string()).toString());
        jcall!(filePropertyHashMap).put(jcall!(fileData).ISHIDDEN.clone(), jcall!((jcall!(self.file).isHidden()).to_string()).toString());
        jcall!(filePropertyHashMap).put(jcall!(fileData).ISABSOLUTE.clone(), jcall!((jcall!(self.file).isAbsolute()).to_string()).toString());
        jcall!(filePropertyHashMap).put(jcall!(fileData).ABSOLUTE_PATH.clone(), jcall!(self.file).getAbsolutePath());
        jcall!(filePropertyHashMap).put(jcall!(fileData).CANONICAL_PATH.clone(), jcall!(self.file).getCanonicalPath());
        jcall!(filePropertyHashMap).put(jcall!(fileData).NAME.clone(), jcall!(self.file).getName());
        jcall!(filePropertyHashMap).put(jcall!(fileData).PARENT.clone(), jcall!(self.file).getParent());
        jcall!(filePropertyHashMap).put(jcall!(fileData).PATH.clone(), jcall!(self.file).getPath());
        return filePropertyHashMap.clone();
    }
}

