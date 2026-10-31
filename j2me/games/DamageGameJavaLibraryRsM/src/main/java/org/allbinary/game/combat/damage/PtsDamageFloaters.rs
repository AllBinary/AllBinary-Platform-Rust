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
// package org.allbinary.game.combat.damage
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::rand::MyRandomFactory::MyRandomFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorArrayIndexer::BasicColorArrayIndexer;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtil;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::java::character::CharArrayFactory::CharArrayFactory;
use crate::org::allbinary::logic::math::PrimitiveLongUtil::PrimitiveLongUtil;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use crate::org::allbinary::view::ViewPosition::ViewPosition;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use super::DamageFloaters::DamageFloaters;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PtsDamageFloaters {
    pub base: DamageFloaters,
    pub basicSetColorUtil: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtilDyn>>>,
    pub layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
    pub lastDamage: Vec<i32>,
    pub lastDamageString: Vec<Vec<char>>,
    pub lastDamageStringSizeArray: Vec<i32>,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub primitiveLongUtil: Option<Rc<RefCell<PrimitiveLongUtil>>>,
    pub myRandomFactory: Option<Rc<RefCell<MyRandomFactory>>>,
}

thread_local! { pub static PtsDamageFloaters_basicColorArrayIndexer: Option<Rc<RefCell<BasicColorArrayIndexer>>> = BasicColorArrayIndexer::new(vec![jcall!(BasicColorFactory::getInstance()).WHITE.clone(), jcall!(BasicColorFactory::getInstance()).RED.clone(), jcall!(BasicColorFactory::getInstance()).YELLOW.clone(), jcall!(BasicColorFactory::getInstance()).BLUE.clone()]); }

impl PtsDamageFloaters {
    

    pub fn new_struct(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Self {
        let mut this = Self {
            base: DamageFloaters::new_struct(),
            basicSetColorUtil: None,
            layerInterface: None,
            lastDamage: Vec::new(),
            lastDamageString: Vec::new(),
            lastDamageStringSizeArray: Vec::new(),
            circularIndexUtil: None,
            primitiveLongUtil: None,
            myRandomFactory: None,
        };
        this.basicSetColorUtil = BasicColorSetUtil::getInstance();
        this.lastDamage = vec![0; (5) as usize];
        this.lastDamageString = vec![vec!['\0'; (0) as usize]; (5) as usize];
        this.lastDamageStringSizeArray = vec![0; (5) as usize];
        this.circularIndexUtil = CircularIndexUtil::createInstance(5);
        this.myRandomFactory = MyRandomFactory::getInstance();
        this.primitiveLongUtil = PrimitiveLongUtil::createPowerOfTen(100000);
        this.layerInterface = layerInterface.clone();
        let mut CHAR_ARRAY: Vec<char> = jcall!(CharArrayFactory::getInstance()).getZeroCharArray();
        for index in (0..=(this.lastDamageString.len() as i32) - 1).rev() {
            this.lastDamageString[(index) as usize] = CHAR_ARRAY.clone();
        }
        this
    }

    pub fn new(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layerInterface))))
    }
    

    pub fn add(&mut self, mut damage: i32) {
        let mut index: i32 = jcall!(self.circularIndexUtil).getIndex();
        self.lastDamage[(index) as usize] = damage.clone();
        self.lastDamageString[(index) as usize] = jcall!(self.primitiveLongUtil).getCharArray(self.lastDamage[(index) as usize].clone());
        self.lastDamageStringSizeArray[(index) as usize] = jcall!(self.primitiveLongUtil).getCurrentTotalDigits();
        jcall!(self.circularIndexUtil).next();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = jcall!(self.layerInterface).getViewPosition();
        let mut x: i32 = jcall!(viewPosition).getX();
        let mut y: i32 = jcall!(viewPosition).getY();
        for index in (0..=(self.lastDamage.len() as i32) - 1).rev() {
            if self.lastDamage[(index) as usize] != 0 {
                let mut delta: i32 = 10 * (index + 2);
                { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), jcall!(PtsDamageFloaters_basicColorArrayIndexer.with(Clone::clone)).get(),); jcall!(java_receiver).setBasicColorP(java_arguments.0, java_arguments.1); };
                jcall!(graphics).drawChars(self.lastDamageString[(index) as usize].clone(), 0, self.lastDamageStringSizeArray[(index) as usize].clone(), x - jcall!(self.myRandomFactory).getNextInt(delta.clone()), y - delta, 0);
                jcall!(PtsDamageFloaters_basicColorArrayIndexer.with(Clone::clone)).next();
            }
            self.lastDamage[(index) as usize] = 0;
        }
    }
}
impl std::ops::Deref for PtsDamageFloaters {
    type Target = DamageFloaters;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PtsDamageFloaters {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn for PtsDamageFloaters {
    fn as_DamageFloaters(&self) -> &crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloaters { &self.base }
    fn as_DamageFloaters_mut(&mut self) -> &mut crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloaters { &mut self.base }
    fn add(&mut self, mut damage: i32) { Self::add(self, damage) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for PtsDamageFloaters {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


