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
// package org.allbinary.graphics.j2me.workarea.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::GPoint::GPoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Grid {
    pub grid: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub isGridOn: bool,
    pub isGridPossible: bool,
    pub zoom: i32,
    pub isChanged: bool,
}


impl Grid {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            grid: None,
            isGridOn: false,
            isGridPossible: false,
            zoom: 0,
            isChanged: false,
        };
        this.isGridOn = true;
        this.isGridPossible = false;
        this.zoom = 3;
        this.isChanged = true;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut grid: Option<Rc<RefCell<Grid>>>) -> Self {
        let mut this = Self {
            grid: None,
            isGridOn: false,
            isGridPossible: false,
            zoom: 0,
            isChanged: false,
        };
        this.isGridOn = true;
        this.isGridPossible = false;
        this.zoom = 3;
        this.isChanged = true;
        this.grid = jcall!(grid).as_GPoint_mut().grid.clone();
        this.showGrid(jcall!(grid).as_GPoint_mut().isGridOn.clone());
        this.isGridPossible = jcall!(grid).as_GPoint_mut().isGridPossible.clone();
        { let java_arguments = (jcall!(grid).as_GPoint_mut().getZoom(),); this.setZoom(java_arguments.0) };
        this
    }

    pub fn new1(mut grid: Option<Rc<RefCell<Grid>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(grid))))
    }
    

    pub fn isGridOn(&mut self) -> bool {
        return self.isGridOn.clone();
    }
    

    pub fn isGridPossible(&mut self) -> bool {
        return self.isGridPossible.clone();
    }
    

    pub fn getGrid(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return self.grid.clone();
    }
    

    pub fn getZoom(&mut self) -> i32 {
        return self.zoom.clone();
    }
    

    pub fn showGrid(&mut self, mut value: bool) {
        self.isGridOn = value.clone();
    }
    

    pub fn setZoom(&mut self, mut zoom: i32) {
        self.zoom = zoom.clone();
        if self.zoom < 1 {
            zoom = 1;
        }
        self.isChanged = true;
    }
}

