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
// package org.allbinary.vector
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PointsUtil {
}

thread_local! { pub static PointsUtil_instance: Option<Rc<RefCell<PointsUtil>>> = PointsUtil::new(); }

impl PointsUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PointsUtil>>> {
        return PointsUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn adjustXY(&mut self, mut points: Vec<Vec<Vec<i32>>>, mut x: i32, mut y: i32) -> Vec<Vec<Vec<i32>>> {
        let mut size: i32 = (points.len() as i32).clone();
        let mut newPoints: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; (0) as usize]; (0) as usize]; (size) as usize];
        for index in (0..size) {
            let mut size2: i32 = (points[(index) as usize].len() as i32).clone();
            newPoints[(index) as usize] = vec![vec![0; (2) as usize]; (size2) as usize];
            for index2 in (0..size2) {
                if points[(index) as usize][(index2) as usize][(0) as usize] != 1000 {
                    newPoints[(index) as usize][(index2) as usize][(0) as usize] = points[(index) as usize][(index2) as usize][(0) as usize] + x;
                    newPoints[(index) as usize][(index2) as usize][(1) as usize] = points[(index) as usize][(index2) as usize][(1) as usize] + y;
                } else {
                    newPoints[(index) as usize][(index2) as usize][(0) as usize] = 1000;
                    newPoints[(index) as usize][(index2) as usize][(1) as usize] = 1000;
                }
            }
        }
        return newPoints.clone();
    }
    

    pub fn adjust(&mut self, mut points: Vec<Vec<i32>>, mut both: i32) -> Vec<Vec<i32>> {
        return self.adjust3(points.clone(), both.clone(), both.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn adjust3(&mut self, mut points: Vec<Vec<i32>>, mut x: i32, mut y: i32) -> Vec<Vec<i32>> {
        let mut newPoints: Vec<Vec<i32>> = vec![vec![0; (2) as usize]; ((points.len() as i32)) as usize];
        for index2 in (0..(points.len() as i32)) {
            if points[(index2) as usize][(0) as usize] != 1000 {
                newPoints[(index2) as usize][(0) as usize] = points[(index2) as usize][(0) as usize] + x;
                newPoints[(index2) as usize][(1) as usize] = points[(index2) as usize][(1) as usize] + y;
            } else {
                newPoints[(index2) as usize][(0) as usize] = 1000;
                newPoints[(index2) as usize][(1) as usize] = 1000;
            }
        }
        return newPoints.clone();
    }
}

