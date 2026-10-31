// Generated Code Do Not Modify
// package org.allbinary.media.graphics.geography.pathfinding
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::logic::util::cache::BaseBasicArrayListCache::BaseBasicArrayListCache;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicGeographicMapExtractedPathsCacheFactory {
    pub base: BaseBasicArrayListCache,
    pub maxConcurrentPaths: i32,
    pub maxPaths: i32,
}

thread_local! { pub static BasicGeographicMapExtractedPathsCacheFactory_instance: Option<Rc<RefCell<BasicGeographicMapExtractedPathsCacheFactory>>> = BasicGeographicMapExtractedPathsCacheFactory::new(); }

impl BasicGeographicMapExtractedPathsCacheFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicGeographicMapExtractedPathsCacheFactory>>> {
        return BasicGeographicMapExtractedPathsCacheFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BaseBasicArrayListCache::new_struct(),
            maxConcurrentPaths: 0,
            maxPaths: 0,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self, mut maxConcurrentPaths: i32, mut maxPaths: i32) {
        if maxConcurrentPaths != self.maxConcurrentPaths {
            for index in (self.maxConcurrentPaths..=maxConcurrentPaths).rev() {
                jcall!(self.list).add(crate::java_into_object(BasicArrayListS::new(maxPaths.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn>>)));
            }
            self.maxConcurrentPaths = maxConcurrentPaths.clone();
        }
        self.maxPaths = maxPaths.clone();
    }
    

    pub fn getPaths(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        if jcall!(self.list).size() > 0 {
            return { let java_object = (jcall!(self.list).removeAt(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
        } else {
            return BasicArrayListS::new(self.maxPaths.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        }
    }
    

    pub fn release(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        for index in (0..=jcall!(list).size() - 1).rev() {
            jcall!(BasicGeographicMapExtractedPathCacheFactory::BasicGeographicMapExtractedPathCacheFactory::getInstance()).release({ let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) });
        }
        jcall!(list).clear();
        jcall!(self.list).add(crate::java_into_object(list.clone()));
    }
}
impl std::ops::Deref for BasicGeographicMapExtractedPathsCacheFactory {
    type Target = BaseBasicArrayListCache;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicGeographicMapExtractedPathsCacheFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::cache::BaseBasicArrayListCache::BaseBasicArrayListCacheDyn for BasicGeographicMapExtractedPathsCacheFactory {
    fn as_BaseBasicArrayListCache(&self) -> &crate::org::allbinary::logic::util::cache::BaseBasicArrayListCache::BaseBasicArrayListCache { &self.base }
    fn as_BaseBasicArrayListCache_mut(&mut self) -> &mut crate::org::allbinary::logic::util::cache::BaseBasicArrayListCache::BaseBasicArrayListCache { &mut self.base }
}


