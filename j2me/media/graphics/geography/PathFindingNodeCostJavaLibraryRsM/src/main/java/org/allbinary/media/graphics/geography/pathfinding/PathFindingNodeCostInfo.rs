//
// AllBinary Open License Version 1
// Copyright (c) 2006 AllBinary
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
// package org.allbinary.media.graphics.geography.pathfinding
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use super::Comparable::Comparable;
use super::PathFindingNodeCostInfoData::PathFindingNodeCostInfoData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathFindingNodeCostInfo {
    pub pathFindingNodeCostInfoData: Option<Rc<RefCell<PathFindingNodeCostInfoData>>>,
    pub costFromStartP: i64,
    pub costToEndP: i64,
    pub totalCostP: i64,
}

// impl Comparable for PathFindingNodeCostInfo with the methods of the trait

impl PathFindingNodeCostInfo {
    

    pub fn new_struct(mut costFromStart: i64, mut costToGoal: i64) -> Self {
        let mut this = Self {
            pathFindingNodeCostInfoData: None,
            costFromStartP: 0_i64,
            costToEndP: 0_i64,
            totalCostP: 0_i64,
        };
        this.pathFindingNodeCostInfoData = PathFindingNodeCostInfoData::getInstance();
        this.costFromStartP = costFromStart.clone();
        this.costToEndP = costToGoal.clone();
        this.setCalculatedTotalCost();
        this
    }

    pub fn new(mut costFromStart: i64, mut costToGoal: i64) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(costFromStart, costToGoal))))
    }
    

    pub fn setCostToEnd(&mut self, mut costToEnd: i64) {
        self.costToEndP = costToEnd.clone();
    }
    

    pub fn setTotalCost(&mut self, mut totalCost: i64) {
        self.totalCostP = totalCost.clone();
    }
    

    // throws Exception
    pub fn setCalculatedTotalCost(&mut self) {
        self.totalCostP = self.costFromStartP + self.costToEndP;
        if self.totalCostP > ((jcall!(self.pathFindingNodeCostInfoData).MAX_NODE_COST) as i64) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Max Cost Exceeded"))));
        }
    }
    

    pub fn setCostFromStart(&mut self, mut cost: i64) {
        self.costFromStartP = cost.clone();
    }
    

    pub fn addCostFromStart(&mut self, mut cost: i64) {
        self.costFromStartP += cost.clone();
    }
    

    pub fn compareTo(&mut self, mut pathFindingNodeCostInfo: Option<Rc<RefCell<PathFindingNodeCostInfo>>>) -> i32 {
        return jcall!(jcall!(unsafe { java_lang.clone() }).Long).compare(self.totalCostP.clone(), jcall!(pathFindingNodeCostInfo).totalCostP.clone());
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(std::any::type_name_of_val(&self).to_string());
        jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).COLON_SEP.clone());
        jcall!(stringBuffer).append(String::from("CostFromStart: "));
        jcall!(stringBuffer).appendlong(self.costFromStartP.clone());
        jcall!(stringBuffer).append(String::from(" CostToEnd: "));
        jcall!(stringBuffer).appendlong(self.costToEndP.clone());
        jcall!(stringBuffer).append(String::from(" TotalCost: "));
        jcall!(stringBuffer).appendlong(self.totalCostP.clone());
        return jcall!(stringBuffer).toString();
    }
}

