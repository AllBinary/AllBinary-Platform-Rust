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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::persistance::GamePersistanceStrings::GamePersistanceStrings;
use crate::org::allbinary::game::input::mapping::InputToGameKeyMapping::InputToGameKeyMapping;
use crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMapping;
use super::PCKeyFactory::PCKeyFactory;
use super::GameKeyFactory::GameKeyFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DesktopDefaultGameInputMapping {
    pub base: PersistentInputMapping,
}


impl DesktopDefaultGameInputMapping {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PersistentInputMapping::new_struct1(jcall!(GamePersistanceStrings::getInstance()).DEFAULT_INPUT_MAPPING_RECORD_ID.clone()),
        };
        // super(...) is in the struct literal above
        let mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>> = this.getInputMapping();
        let mut pcKeyFactory: Option<Rc<RefCell<PCKeyFactory>>> = PCKeyFactory::getInstance();
        let mut gameKeyFactory: Option<Rc<RefCell<GameKeyFactory>>> = GameKeyFactory::getInstance();
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).FIRE.clone(), jcall!(pcKeyFactory).ENTER.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM1.clone(), jcall!(pcKeyFactory).FOUR.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM1.clone(), jcall!(pcKeyFactory)._P.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM1.clone(), jcall!(pcKeyFactory).p.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).UP.clone(), jcall!(pcKeyFactory).DPAD_UP.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).DOWN.clone(), jcall!(pcKeyFactory).DPAD_DOWN.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).LEFT.clone(), jcall!(pcKeyFactory).DPAD_LEFT.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).RIGHT.clone(), jcall!(pcKeyFactory).DPAD_RIGHT.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM1.clone(), jcall!(pcKeyFactory).FIVE.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM7.clone(), jcall!(pcKeyFactory)._Y.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM9.clone(), jcall!(pcKeyFactory)._I.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).UP.clone(), jcall!(pcKeyFactory)._U.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).LEFT.clone(), jcall!(pcKeyFactory)._H.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).RIGHT.clone(), jcall!(pcKeyFactory)._J.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).DOWN.clone(), jcall!(pcKeyFactory)._N.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM5.clone(), jcall!(pcKeyFactory)._M.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM7.clone(), jcall!(pcKeyFactory).y.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM9.clone(), jcall!(pcKeyFactory).i.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).UP.clone(), jcall!(pcKeyFactory).u.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).LEFT.clone(), jcall!(pcKeyFactory).h.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).RIGHT.clone(), jcall!(pcKeyFactory).j.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).DOWN.clone(), jcall!(pcKeyFactory).n.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM5.clone(), jcall!(pcKeyFactory).m.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM7.clone(), jcall!(pcKeyFactory)._Q.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM9.clone(), jcall!(pcKeyFactory)._E.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).UP.clone(), jcall!(pcKeyFactory)._W.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).LEFT.clone(), jcall!(pcKeyFactory)._A.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).RIGHT.clone(), jcall!(pcKeyFactory)._D.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).DOWN.clone(), jcall!(pcKeyFactory)._S.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM5.clone(), jcall!(pcKeyFactory)._X.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM7.clone(), jcall!(pcKeyFactory).q.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM9.clone(), jcall!(pcKeyFactory).e.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).UP.clone(), jcall!(pcKeyFactory).w.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).LEFT.clone(), jcall!(pcKeyFactory).a.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).RIGHT.clone(), jcall!(pcKeyFactory).d.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).DOWN.clone(), jcall!(pcKeyFactory).s.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM5.clone(), jcall!(pcKeyFactory).x.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM3.clone(), jcall!(pcKeyFactory).THREE.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM0.clone(), jcall!(pcKeyFactory).ZERO.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM0.clone(), jcall!(pcKeyFactory).ONE.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM0.clone(), jcall!(pcKeyFactory).SPACE.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_POUND.clone(), jcall!(pcKeyFactory).DEL.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_POUND.clone(), jcall!(pcKeyFactory).TWO.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_STAR.clone(), jcall!(pcKeyFactory).ESCAPE.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_STAR.clone(), jcall!(pcKeyFactory).STAR.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).LEVEL_DOWN.clone(), jcall!(pcKeyFactory).COMMA.clone());
        jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).LEVEL_UP.clone(), jcall!(pcKeyFactory).PERIOD.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for DesktopDefaultGameInputMapping {
    type Target = PersistentInputMapping;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DesktopDefaultGameInputMapping {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



