// Generated Code Do Not Modify
// package org.allbinary.util
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use crate::jsinterop::annotations::JsProperty;
use super::ArrayUtil::ArrayUtil;
use super::BasicArrayListS::BasicArrayListS;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//This is a very fast and simple resizable list.
pub struct BasicArrayList {
    pub arrayUtil: Option<Rc<RefCell<ArrayUtil>>>,
    pub objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>,
    pub currentIndex: i32,
}

thread_local! { pub static BasicArrayList_SIZE: String = String::from(", Size: "); }

impl BasicArrayList {
    

    pub fn new_struct(mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) -> Self {
        let mut this = Self {
            arrayUtil: None,
            objectArray: Vec::new(),
            currentIndex: 0,
        };
        this.arrayUtil = ArrayUtil::getInstance();
        this.currentIndex = 0;
        this.objectArray = objectArray.clone();
        this
    }

    pub fn new(mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(objectArray))))
    }
    

    pub fn addAt(&mut self, mut index: i32, mut element: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        if index > self.currentIndex || index < 0 {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).INDEX_LABEL.clone());
            jcall!(stringBuffer).appendint(index.clone());
            jcall!(stringBuffer).append(BasicArrayList_SIZE.with(Clone::clone).clone());
            jcall!(stringBuffer).appendint(self.currentIndex.clone());
            panic!("{:?}", format!("{}: {:?}", "IndexOutOfBoundsException", (jcall!(stringBuffer).toString())));
        }
        self.ensureCapacity(self.currentIndex + 1);
        
                { let java_copy_source = self.objectArray[(index) as usize..(index + self.currentIndex - index) as usize].to_vec(); self.objectArray[(index + 1) as usize..(index + 1 + self.currentIndex - index) as usize].clone_from_slice(&java_copy_source); };
        self.objectArray[(index) as usize] = element.clone();
        self.currentIndex += 1;
    }
    

    pub fn add(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        self.ensureCapacity(self.currentIndex + 1);
        self.objectArray[({ let value = self.currentIndex; self.currentIndex += 1; value }) as usize] = anyType.clone();
        return true;
    }
    

    pub fn removeAt(&mut self, mut index: i32) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        if index >= self.currentIndex {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).INDEX_LABEL.clone());
            jcall!(stringBuffer).appendint(index.clone());
            jcall!(stringBuffer).append(BasicArrayList_SIZE.with(Clone::clone).clone());
            jcall!(stringBuffer).appendint(self.currentIndex.clone());
            panic!("{:?}", format!("{}: {:?}", "IndexOutOfBoundsException", (jcall!(stringBuffer).toString())));
        }
        let mut oldValue: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = self.objectArray[(index) as usize].clone();
        let mut numMoved: i32 = self.currentIndex - index - 1;
        if numMoved > 0 {
            
                { let java_copy_source = self.objectArray[(index + 1) as usize..(index + 1 + numMoved) as usize].to_vec(); self.objectArray[(index) as usize..(index + numMoved) as usize].clone_from_slice(&java_copy_source); };
        }
        self.objectArray[({ self.currentIndex -= 1; self.currentIndex }) as usize] = None;
        return oldValue.clone();
    }
    

    pub fn remove(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        if anyType.is_none() {
            for index in (0..self.currentIndex) {
                if self.objectArray[(index) as usize].is_none() {
                    let mut numMoved: i32 = self.currentIndex - index - 1;
                    if numMoved > 0 {
                        
                { let java_copy_source = self.objectArray[(index + 1) as usize..(index + 1 + numMoved) as usize].to_vec(); self.objectArray[(index) as usize..(index + numMoved) as usize].clone_from_slice(&java_copy_source); };
                    }
                    self.objectArray[({ self.currentIndex -= 1; self.currentIndex }) as usize] = None;
                    return true;
                }
            }
        } else {
            for index in (0..self.currentIndex) {
                if match (&anyType, &self.objectArray[(index) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || { let java_receiver = (anyType).clone(); let java_argument = self.objectArray[(index) as usize].clone(); assert!(java_receiver.is_some(), "NullPointerException"); crate::java_object_equals(&java_receiver, &java_argument) } {
                    let mut numMoved: i32 = self.currentIndex - index - 1;
                    if numMoved > 0 {
                        
                { let java_copy_source = self.objectArray[(index + 1) as usize..(index + 1 + numMoved) as usize].to_vec(); self.objectArray[(index) as usize..(index + numMoved) as usize].clone_from_slice(&java_copy_source); };
                    }
                    self.objectArray[({ self.currentIndex -= 1; self.currentIndex }) as usize] = None;
                    return true;
                }
            }
        }
        return false;
    }
    

    pub fn removeAll2(&mut self, mut list: Option<Rc<RefCell<dyn BasicArrayListDyn>>>) -> bool {
        let mut result: bool = true;
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            if { let java_arguments = (jcall!(list).get(index.clone()),); self.remove(java_arguments.0) } {
            } else {
                result = false;
            }
        }
        return result.clone();
    }
    

    pub fn addAll2(&mut self, mut list: Option<Rc<RefCell<dyn BasicArrayListDyn>>>) -> bool {
        self.ensureCapacity(self.currentIndex + jcall!(list).as_BasicArrayList_mut().currentIndex);
        let mut listSize: i32 = jcall!(list).as_BasicArrayList_mut().currentIndex.clone();
        for index in (0..listSize) {
            self.objectArray[({ let value = self.currentIndex; self.currentIndex += 1; value }) as usize] = jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize].clone();
        }
        return true;
    }
    

    pub fn addAllList(&mut self, mut list: Option<Rc<RefCell<dyn BasicArrayListDyn>>>) -> bool {
        let mut newObjectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(list).toArray();
        return self.addAll(newObjectArray.clone());
    }
    

    pub fn addAll(&mut self, mut newObjectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) -> bool {
        let mut numSize: i32 = (newObjectArray.len() as i32).clone();
        self.ensureCapacity(self.currentIndex + numSize);
        
                { let java_copy_source = newObjectArray[(0) as usize..(0 + numSize) as usize].to_vec(); self.objectArray[(self.currentIndex) as usize..(self.currentIndex + numSize) as usize].clone_from_slice(&java_copy_source); };
        self.currentIndex += numSize.clone();
        return numSize != 0;
    }
    

    pub fn ensureCapacity(&mut self, mut minSize: i32) {
        let mut oldCapacity: i32 = (self.objectArray.len() as i32).clone();
        if minSize > oldCapacity {
            let mut newCapacity: i32 = ((oldCapacity * 3) >> 1) + 1;
            if newCapacity < minSize {
                newCapacity = minSize.clone();
            }
            self.objectArray = jcall!(self.arrayUtil).copyOf(self.objectArray.clone(), newCapacity.clone());
        }
    }
    

    pub fn trimToSize(&mut self) {
        let mut oldCapacity: i32 = (self.objectArray.len() as i32).clone();
        if self.currentIndex < oldCapacity {
            self.objectArray = jcall!(self.arrayUtil).copyOf(self.objectArray.clone(), self.currentIndex.clone());
        }
    }
    

    pub fn indexOf(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 {
        if anyType.is_none() {
            for i in (0..self.currentIndex) {
                if self.objectArray[(i) as usize].is_none() {
                    return i.clone();
                }
            }
        } else {
            for i in (0..self.currentIndex) {
                if { let java_receiver = (anyType).clone(); let java_argument = self.objectArray[(i) as usize].clone(); assert!(java_receiver.is_some(), "NullPointerException"); crate::java_object_equals(&java_receiver, &java_argument) } {
                    return i.clone();
                }
            }
        }
        return -1;
    }
    

    pub fn lastIndexOf(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 {
        if anyType.is_none() {
            for i in (0..=self.currentIndex - 1).rev() {
                if self.objectArray[(i) as usize].is_none() {
                    return i.clone();
                }
            }
        } else {
            for i in (0..=self.currentIndex - 1).rev() {
                if { let java_receiver = (anyType).clone(); let java_argument = self.objectArray[(i) as usize].clone(); assert!(java_receiver.is_some(), "NullPointerException"); crate::java_object_equals(&java_receiver, &java_argument) } {
                    return i.clone();
                }
            }
        }
        return -1;
    }
    

    pub fn get(&mut self, mut index: i32) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        if index >= self.currentIndex {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).INDEX_LABEL.clone());
            jcall!(stringBuffer).appendint(index.clone());
            jcall!(stringBuffer).append(BasicArrayList_SIZE.with(Clone::clone).clone());
            jcall!(stringBuffer).appendint(self.currentIndex.clone());
            panic!("{:?}", format!("{}: {:?}", "IndexOutOfBoundsException", (jcall!(stringBuffer).toString())));
        }
        return self.objectArray[(index) as usize].clone();
    }
    

    pub fn set(&mut self, mut index: i32, mut element: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        if index >= self.currentIndex {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).INDEX_LABEL.clone());
            jcall!(stringBuffer).appendint(index.clone());
            jcall!(stringBuffer).append(BasicArrayList_SIZE.with(Clone::clone).clone());
            jcall!(stringBuffer).appendint(self.currentIndex.clone());
            panic!("{:?}", format!("{}: {:?}", "IndexOutOfBoundsException", (jcall!(stringBuffer).toString())));
        }
        let mut oldValue: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = self.objectArray[(index) as usize].clone();
        self.objectArray[(index) as usize] = element.clone();
        return oldValue.clone();
    }
    

    pub fn clear(&mut self) {
        for i in (0..self.currentIndex) {
            self.objectArray[(i) as usize] = None;
        }
        self.currentIndex = 0;
    }
    

    pub fn size(&mut self) -> i32 {
        return self.currentIndex.clone();
    }
    

    pub fn isEmpty(&mut self) -> bool {
        return self.currentIndex == 0;
    }
    

    pub fn contains(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return self.indexOf(anyType.clone()) >= 0;
    }
    

    pub fn toArray(&mut self) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        return jcall!(self.arrayUtil).copyOf(self.objectArray.clone(), self.currentIndex.clone());
    }
    

    pub fn toArrayType(&mut self, mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        if (objectArray.len() as i32) < self.currentIndex {
            return { let java_receiver = self.arrayUtil.clone(); let java_arguments = (self.objectArray.clone(), self.currentIndex.clone(), std::any::type_name_of_val(&objectArray),); let java_result = jcall!(java_receiver).copyOfType(java_arguments.0, java_arguments.1, java_arguments.2); java_result };
        }
        
                { let java_copy_source = self.objectArray[(0) as usize..(0 + self.currentIndex) as usize].to_vec(); objectArray[(0) as usize..(0 + self.currentIndex) as usize].clone_from_slice(&java_copy_source); };
        if (objectArray.len() as i32) > self.currentIndex {
            objectArray[(self.currentIndex) as usize] = None;
        }
        return objectArray.clone();
    }
    

    pub fn clone(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut size: i32 = self.size();
        let mut list: Option<Rc<RefCell<dyn BasicArrayListDyn>>> = BasicArrayListS::new(size.clone()).map(|reference| reference as Rc<RefCell<dyn BasicArrayListDyn>>);
        let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = None;
        for index in (0..size) {
            anyType = self.objectArray[(index) as usize].clone();
            jcall!(list).add(crate::java_into_object(anyType.clone()));
        }
        return (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(list.clone());
    }
    

    pub fn toString(&mut self) -> String {
        let mut COMMA_SEP: String = jcall!(CommonSeps::getInstance()).COMMA_SEP.clone();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        for index in (0..self.currentIndex) {
            jcall!(stringBuffer).append({ let value = jcall!(self.objectArray[(index) as usize]); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) });
            jcall!(stringBuffer).append(COMMA_SEP.clone());
        }
        return jcall!(stringBuffer).toString();
    }
}
pub trait BasicArrayListDyn {
    fn as_BasicArrayList(&self) -> &BasicArrayList;
    fn as_BasicArrayList_mut(&mut self) -> &mut BasicArrayList;
    fn addAt(&mut self, mut index: i32, mut element: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { BasicArrayList::addAt(self.as_BasicArrayList_mut(), index, element) }
    fn add(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool { BasicArrayList::add(self.as_BasicArrayList_mut(), anyType) }
    fn removeAt(&mut self, mut index: i32) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { BasicArrayList::removeAt(self.as_BasicArrayList_mut(), index) }
    fn remove(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool { BasicArrayList::remove(self.as_BasicArrayList_mut(), anyType) }
    fn removeAll2(&mut self, mut list: Option<Rc<RefCell<dyn BasicArrayListDyn>>>) -> bool { BasicArrayList::removeAll2(self.as_BasicArrayList_mut(), list) }
    fn addAll2(&mut self, mut list: Option<Rc<RefCell<dyn BasicArrayListDyn>>>) -> bool { BasicArrayList::addAll2(self.as_BasicArrayList_mut(), list) }
    fn addAllList(&mut self, mut list: Option<Rc<RefCell<dyn BasicArrayListDyn>>>) -> bool { BasicArrayList::addAllList(self.as_BasicArrayList_mut(), list) }
    fn addAll(&mut self, mut newObjectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) -> bool { BasicArrayList::addAll(self.as_BasicArrayList_mut(), newObjectArray) }
    fn ensureCapacity(&mut self, mut minSize: i32) { BasicArrayList::ensureCapacity(self.as_BasicArrayList_mut(), minSize) }
    fn trimToSize(&mut self) { BasicArrayList::trimToSize(self.as_BasicArrayList_mut()) }
    fn indexOf(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 { BasicArrayList::indexOf(self.as_BasicArrayList_mut(), anyType) }
    fn lastIndexOf(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 { BasicArrayList::lastIndexOf(self.as_BasicArrayList_mut(), anyType) }
    fn get(&mut self, mut index: i32) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { BasicArrayList::get(self.as_BasicArrayList_mut(), index) }
    fn set(&mut self, mut index: i32, mut element: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { BasicArrayList::set(self.as_BasicArrayList_mut(), index, element) }
    fn clear(&mut self) { BasicArrayList::clear(self.as_BasicArrayList_mut()) }
    fn size(&mut self) -> i32 { BasicArrayList::size(self.as_BasicArrayList_mut()) }
    fn isEmpty(&mut self) -> bool { BasicArrayList::isEmpty(self.as_BasicArrayList_mut()) }
    fn contains(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool { BasicArrayList::contains(self.as_BasicArrayList_mut(), anyType) }
    fn toArray(&mut self) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> { BasicArrayList::toArray(self.as_BasicArrayList_mut()) }
    fn toArrayType(&mut self, mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> { BasicArrayList::toArrayType(self.as_BasicArrayList_mut(), objectArray) }
    fn clone(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { BasicArrayList::clone(self.as_BasicArrayList_mut()) }
    fn toString(&mut self) -> String { BasicArrayList::toString(self.as_BasicArrayList_mut()) }
}
impl BasicArrayListDyn for BasicArrayList {
    fn as_BasicArrayList(&self) -> &BasicArrayList { self }
    fn as_BasicArrayList_mut(&mut self) -> &mut BasicArrayList { self }
}


