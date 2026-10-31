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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Enumeration;
use super::*;
// use crate::java::util::Enumeration::Enumeration;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::LayerInterfaceFactory::LayerInterfaceFactory;
use crate::org::allbinary::layer::LayerInterfaceVisitor::LayerInterfaceVisitor;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::EnumerationUtil::EnumerationUtil;
use super::LayerPlacementInterface::LayerPlacementInterface;
use super::LayerPlacementType::LayerPlacementType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerPlacer {
    pub enumerationUtil: Option<Rc<RefCell<EnumerationUtil>>>,
    pub layerInterfaceVisitor: Option<Rc<RefCell<dyn LayerInterfaceVisitor>>>,
    pub dimension: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
}


impl LayerPlacer {
    

    pub fn new_struct(mut layerInterfaceVisitor: Option<Rc<RefCell<dyn LayerInterfaceVisitor>>>, mut dimension: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> Self {
        let mut this = Self {
            enumerationUtil: None,
            layerInterfaceVisitor: None,
            dimension: None,
        };
        this.enumerationUtil = EnumerationUtil::getInstance();
        this.layerInterfaceVisitor = layerInterfaceVisitor.clone();
        this.dimension = dimension.clone();
        this
    }

    pub fn new(mut layerInterfaceVisitor: Option<Rc<RefCell<dyn LayerInterfaceVisitor>>>, mut dimension: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layerInterfaceVisitor, dimension))))
    }
    

    // throws Exception
    pub fn processList(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut size: i32 = jcall!(list).size();
        let mut layerPlacementInterface: Option<Rc<RefCell<dyn LayerPlacementInterface>>> = None;
        for index in (0..size) {
            layerPlacementInterface = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn LayerPlacementInterface>>>>().expect("ClassCastException").clone()) };
            self.process(layerPlacementInterface.clone());
        }
    }
    

    // throws Exception
    pub fn process(&mut self, mut layerPlacementInterface: Option<Rc<RefCell<dyn LayerPlacementInterface>>>) {
        let mut relativePoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = self.getPoint(layerPlacementInterface.clone());
        let mut layerInterfaceFactory: Option<Rc<RefCell<LayerInterfaceFactory>>> = LayerInterfaceFactory::getInstance();
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(layerPlacementInterface).getInstance();
        let mut enumeration: Option<Rc<RefCell<dyn Enumeration>>> = jcall!(hashtable).keys();
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        let mut layerHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = None;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut z: i32 = 0;
        let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = None;
        while jcall!(self.enumerationUtil).hasMoreElements(enumeration.clone()) {
            point = { let java_object = (jcall!(self.enumerationUtil).nextElement(enumeration.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            layerHashtable = { let java_object = (jcall!(hashtable).get(crate::java_into_object(point.clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<ABHashtable>>>>().expect("ClassCastException").clone()) };
            x = jcall!(point).getX() + jcall!(relativePoint).getX();
            y = jcall!(point).getY() + jcall!(relativePoint).getY();
            z = jcall!(point).getZ() + jcall!(relativePoint).getZ();
            layerInterface = jcall!(layerInterfaceFactory).getNexInstance(layerHashtable.clone(), x.clone(), y.clone(), z.clone());
            jcall!(self.layerInterfaceVisitor).visit(layerInterface.clone());
        }
    }
    

    // throws Exception
    pub fn getPoint(&mut self, mut layerPlacementInterface: Option<Rc<RefCell<dyn LayerPlacementInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
        let mut layerPlacementType: Option<Rc<RefCell<LayerPlacementType>>> = jcall!(layerPlacementInterface).getLayerType();
        if match (&layerPlacementType, &jcall!(LayerPlacementTypeFactory::LayerPlacementTypeFactory::getInstance()).MAP) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut width: i32 = jcall!(layerPlacementInterface).getWidth();
            let mut height: i32 = jcall!(layerPlacementInterface).getHeight();
            let mut x: i32 = ((jcall!(self.dimension).getX() - width) / 2);
            let mut y: i32 = ((jcall!(self.dimension).getY() - height) / 2);
            return jcall!(pointFactory).createXY(x.clone(), y.clone());
        } else if match (&layerPlacementType, &jcall!(LayerPlacementTypeFactory::LayerPlacementTypeFactory::getInstance()).UP) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut width: i32 = jcall!(layerPlacementInterface).getWidth();
            let mut height: i32 = jcall!(layerPlacementInterface).getHeight();
            let mut x: i32 = ((jcall!(self.dimension).getX() - width) / 2);
            let mut y: i32 = -height;
            return jcall!(pointFactory).createXY(x.clone(), y.clone());
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("PlacementType Not Recognized"))));
        }
    }
}

