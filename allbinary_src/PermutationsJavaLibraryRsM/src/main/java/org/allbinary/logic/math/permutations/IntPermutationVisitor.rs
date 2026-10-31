// Generated Code Do Not Modify
// package org.allbinary.logic.math.permutations
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct IntPermutationVisitor {
    pub base: Visitor,
    pub data: Vec<i32>,
    pub stringBuilder: Option<Rc<RefCell<StringMaker>>>,
}


impl IntPermutationVisitor {
    

    pub fn new_struct(mut data: Vec<i32>) -> Self {
        let mut this = Self {
            base: Visitor::new_struct(),
            data: Vec::new(),
            stringBuilder: None,
        };
        this.stringBuilder = StringMaker::new();
        this.data = data.clone();
        this
    }

    pub fn new(mut data: Vec<i32>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(data))))
    }
    

    pub fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        { let java_receiver = self.stringBuilder.clone(); let java_arguments = (0, jcall!(self.stringBuilder).length(),); jcall!(java_receiver).delete(java_arguments.0, java_arguments.1); };
        self.print2(self.data.clone(), self.stringBuilder.clone());
        return jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
    }
    

    pub fn print(&mut self, mut data: Vec<i32>) {
        let mut stringBuilder: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        self.print2(data.clone(), stringBuilder.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn print2(&mut self, mut data: Vec<i32>, mut stringBuilder: Option<Rc<RefCell<StringMaker>>>) {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut size: i32 = (data.len() as i32).clone();
        for i in (0..size) {
            jcall!(stringBuilder).appendint(data[(i) as usize].clone());
            jcall!(stringBuilder).append(jcall!(commonSeps).SPACE.clone());
        }
        println!("{}", jcall!(stringBuilder).toString());
    }
}
impl std::ops::Deref for IntPermutationVisitor {
    type Target = Visitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for IntPermutationVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait IntPermutationVisitorDyn: crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn {
    fn as_IntPermutationVisitor(&self) -> &IntPermutationVisitor;
    fn as_IntPermutationVisitor_mut(&mut self) -> &mut IntPermutationVisitor;
    fn print(&mut self, mut data: Vec<i32>) { IntPermutationVisitor::print(self.as_IntPermutationVisitor_mut(), data) }
    fn print2(&mut self, mut data: Vec<i32>, mut stringBuilder: Option<Rc<RefCell<StringMaker>>>) { IntPermutationVisitor::print2(self.as_IntPermutationVisitor_mut(), data, stringBuilder) }
}
impl IntPermutationVisitorDyn for IntPermutationVisitor {
    fn as_IntPermutationVisitor(&self) -> &IntPermutationVisitor { self }
    fn as_IntPermutationVisitor_mut(&mut self) -> &mut IntPermutationVisitor { self }
}

impl crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn for IntPermutationVisitor {
    fn as_Visitor(&self) -> &crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &self.base }
    fn as_Visitor_mut(&mut self) -> &mut crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &mut self.base }
}
impl crate::org::allbinary::logic::util::visitor::VisitorInterface::VisitorInterface for IntPermutationVisitor {
    fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::visit(self, anyType) }
}


