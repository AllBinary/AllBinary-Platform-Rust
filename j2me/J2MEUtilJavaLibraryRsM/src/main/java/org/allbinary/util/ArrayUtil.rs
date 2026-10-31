// Generated Code Do Not Modify
// package org.allbinary.util
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::org::allbinary::logic::math::MathUtil::MathUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::jsinterop::annotations::JsMethod;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ArrayUtil {
    pub mathUtil: Option<Rc<RefCell<MathUtil>>>,
}

thread_local! { pub static ArrayUtil_instance: Option<Rc<RefCell<ArrayUtil>>> = ArrayUtil::new(); }

impl ArrayUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            mathUtil: None,
        };
        this.mathUtil = MathUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ArrayUtil>>> {
        return ArrayUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn copyOf(&mut self, mut original: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>, mut newLength: i32) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        return { let java_arguments = (original.clone(), newLength.clone(), std::any::type_name_of_val(&original),); self.copyOfType(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
    

    pub fn copyOfType(&mut self, mut original: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>, mut newLength: i32, mut newType: &'static str) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        let mut copy: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (newLength) as usize];
        
                { let java_copy_source = original[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].to_vec(); copy[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].clone_from_slice(&java_copy_source); };
        return copy.clone();
    }
    

    pub fn copyOfbyte(&mut self, mut original: Vec<i8>, mut newLength: i32) -> Vec<i8> {
        let mut copy: Vec<i8> = vec![0; (newLength) as usize];
        
                { let java_copy_source = original[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].to_vec(); copy[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].clone_from_slice(&java_copy_source); };
        return copy.clone();
    }
    

    pub fn copyOfshort(&mut self, mut original: Vec<i16>, mut newLength: i32) -> Vec<i16> {
        let mut copy: Vec<i16> = vec![0; (newLength) as usize];
        
                { let java_copy_source = original[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].to_vec(); copy[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].clone_from_slice(&java_copy_source); };
        return copy.clone();
    }
    

    pub fn copyOfint(&mut self, mut original: Vec<i32>, mut newLength: i32) -> Vec<i32> {
        let mut copy: Vec<i32> = vec![0; (newLength) as usize];
        
                { let java_copy_source = original[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].to_vec(); copy[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].clone_from_slice(&java_copy_source); };
        return copy.clone();
    }
    

    pub fn copyOflong(&mut self, mut original: Vec<i64>, mut newLength: i32) -> Vec<i64> {
        let mut copy: Vec<i64> = vec![0_i64; (newLength) as usize];
        
                { let java_copy_source = original[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].to_vec(); copy[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].clone_from_slice(&java_copy_source); };
        return copy.clone();
    }
    

    pub fn copyOfchar(&mut self, mut original: Vec<char>, mut newLength: i32) -> Vec<char> {
        let mut copy: Vec<char> = vec!['\0'; (newLength) as usize];
        
                { let java_copy_source = original[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].to_vec(); copy[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].clone_from_slice(&java_copy_source); };
        return copy.clone();
    }
    

    pub fn copyOffloat(&mut self, mut original: Vec<f32>, mut newLength: i32) -> Vec<f32> {
        let mut copy: Vec<f32> = vec![0.0_f32; (newLength) as usize];
        
                { let java_copy_source = original[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].to_vec(); copy[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].clone_from_slice(&java_copy_source); };
        return copy.clone();
    }
    

    pub fn copyOfdouble(&mut self, mut original: Vec<f64>, mut newLength: i32) -> Vec<f64> {
        let mut copy: Vec<f64> = vec![0.0; (newLength) as usize];
        
                { let java_copy_source = original[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].to_vec(); copy[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].clone_from_slice(&java_copy_source); };
        return copy.clone();
    }
    

    pub fn copyOfboolean(&mut self, mut original: Vec<bool>, mut newLength: i32) -> Vec<bool> {
        let mut copy: Vec<bool> = vec![false; (newLength) as usize];
        
                { let java_copy_source = original[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].to_vec(); copy[(0) as usize..(0 + jcall!(self.mathUtil).min((original.len() as i32).clone(), newLength.clone())) as usize].clone_from_slice(&java_copy_source); };
        return copy.clone();
    }
    

    pub fn reverse(&mut self, mut intArray: Vec<i32>) {
        let mut length: i32 = (intArray.len() as i32) - 1;
        let mut temp: i32 = 0;
        let mut size: i32 = length / 2;
        for index in (0..=size) {
            temp = intArray[(index) as usize].clone();
            intArray[(index) as usize] = intArray[(length - index) as usize].clone();
            intArray[(length - index) as usize] = temp.clone();
        }
    }
    

    pub fn flip(&mut self, mut intArray: Vec<i32>, mut width: i32, mut height: i32) {
        let mut length: i32 = height - 1;
        let mut temp: i32 = 0;
        let mut index: i32 = 0;
        let mut rowIndex: i32 = 0;
        for y in (0..=length / 2) {
            for x in (0..width) {
                rowIndex = ((length - y) * width);
                index = x + (y * width);
                temp = intArray[(index) as usize].clone();
                intArray[(index) as usize] = intArray[(rowIndex + x) as usize].clone();
                intArray[(rowIndex + x) as usize] = temp.clone();
            }
        }
    }
    

    pub fn toStringFromTwoDimensionaArray(&mut self, mut twoDimensionalIntArray: Vec<Vec<i32>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut endIndex: i32 = (twoDimensionalIntArray.len() as i32).clone();
        let mut endIndex2: i32 = (twoDimensionalIntArray[(0) as usize].len() as i32).clone();
        for index in (0..endIndex) {
            jcall!(stringBuffer).append(jcall!(commonSeps).BRACE_OPEN.clone());
            for index2 in (0..endIndex2) {
                jcall!(stringBuffer).appendint(((twoDimensionalIntArray[(index) as usize][(index2) as usize]) as i32).clone());
                jcall!(stringBuffer).append(jcall!(commonSeps).COMMA_SEP.clone());
            }
            jcall!(stringBuffer).append(jcall!(commonSeps).BRACE_CLOSE.clone());
            jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn toStringFromObjectArray(&mut self, mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        for index in (0..(objectArray.len() as i32)) {
            jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(objectArray[(index) as usize].clone())));
            jcall!(stringBuffer).append(jcall!(commonSeps).COMMA_SEP.clone());
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn toStringFromByteArray(&mut self, mut byteArray: Vec<i8>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        for index in (0..(byteArray.len() as i32)) {
            jcall!(stringBuffer).appendbyte(byteArray[(index) as usize].clone());
            jcall!(stringBuffer).append(jcall!(commonSeps).COMMA_SEP.clone());
        }
        return jcall!(stringBuffer).toString();
    }
}

