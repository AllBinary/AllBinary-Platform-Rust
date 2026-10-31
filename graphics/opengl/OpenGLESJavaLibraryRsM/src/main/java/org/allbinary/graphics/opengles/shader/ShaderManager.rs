// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.graphics.opengles.shader
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ShaderManager {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub LOAD_SHADER: String,
    pub __VERSION__: String,
    pub GL_VERTEX_SHADER: i32,
    pub GL_FRAGMENT_SHADER: i32,
}

thread_local! { pub static ShaderManager_instance: Option<Rc<RefCell<dyn ShaderManagerDyn>>> = ShaderManager::new(-1, -1).map(|reference| reference as Rc<RefCell<dyn ShaderManagerDyn>>); }

impl ShaderManager {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ShaderManagerDyn>>> {
        return ShaderManager_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct(mut GL_VERTEX_SHADER: i32, mut GL_FRAGMENT_SHADER: i32) -> Self {
        let mut this = Self {
            commonStrings: None,
            LOAD_SHADER: String::new(),
            __VERSION__: String::new(),
            GL_VERTEX_SHADER: 0,
            GL_FRAGMENT_SHADER: 0,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.LOAD_SHADER = String::from("loadShader");
        this.__VERSION__ = String::from("__VERSION__");
        this.GL_VERTEX_SHADER = GL_VERTEX_SHADER.clone();
        this.GL_FRAGMENT_SHADER = GL_FRAGMENT_SHADER.clone();
        this
    }

    pub fn new(mut GL_VERTEX_SHADER: i32, mut GL_FRAGMENT_SHADER: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(GL_VERTEX_SHADER, GL_FRAGMENT_SHADER))))
    }
    

    pub fn load(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut resource: String, mut shaderAsStringArray: Vec<String>, mut shaderType: i32) -> i32 {
        return -1;
    }
    

    pub fn loadShader(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut resource: String, mut stringList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut shaderType: i32) -> i32 {
        return -1;
    }
}
pub trait ShaderManagerDyn {
    fn as_ShaderManager(&self) -> &ShaderManager;
    fn as_ShaderManager_mut(&mut self) -> &mut ShaderManager;
    fn load(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut resource: String, mut shaderAsStringArray: Vec<String>, mut shaderType: i32) -> i32 { ShaderManager::load(self.as_ShaderManager_mut(), gl, resource, shaderAsStringArray, shaderType) }
    fn loadShader(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut resource: String, mut stringList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut shaderType: i32) -> i32 { ShaderManager::loadShader(self.as_ShaderManager_mut(), gl, resource, stringList, shaderType) }
    fn load(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut resource: String, mut shaderAsStringArray: Vec<String>, mut shaderType: i32) -> i32 { ShaderManager::load(self.as_ShaderManager_mut(), gl, resource, shaderAsStringArray, shaderType) }
    fn loadShader(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut resource: String, mut stringList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut shaderType: i32) -> i32 { ShaderManager::loadShader(self.as_ShaderManager_mut(), gl, resource, stringList, shaderType) }
}
impl ShaderManagerDyn for ShaderManager {
    fn as_ShaderManager(&self) -> &ShaderManager { self }
    fn as_ShaderManager_mut(&mut self) -> &mut ShaderManager { self }
}


