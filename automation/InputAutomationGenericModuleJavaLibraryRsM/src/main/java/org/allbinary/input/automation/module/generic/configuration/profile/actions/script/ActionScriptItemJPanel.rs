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
// package org.allbinary.input.automation.module.generic.configuration.profile.actions.script
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::JPanel::JPanel;
use super::swing::swing;
use super::javax::javax;
use super::GridLayout::GridLayout;
use super::awt::awt;
use super::java::java;
use super::GroupLayout::GroupLayout;
use super::Dimension::Dimension;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ActionScriptItemJPanel {
    pub base: JPanel,
    pub childActionScriptConditionJPanel: Option<Rc<RefCell<JPanel>>>,
    pub newActionScriptConditionJPanel: Option<Rc<RefCell<JPanel>>>,
    pub newActionScriptInputJPanel: Option<Rc<RefCell<JPanel>>>,
    pub removeJPanel: Option<Rc<RefCell<JPanel>>>,
    pub scriptItemJPanel: Option<Rc<RefCell<JPanel>>>,
}


impl ActionScriptItemJPanel {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            childActionScriptConditionJPanel: None,
            newActionScriptConditionJPanel: None,
            newActionScriptInputJPanel: None,
            removeJPanel: None,
            scriptItemJPanel: None,
        };
        this.initComponents();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getNewActionScriptInputJPanel(&mut self) -> Option<Rc<RefCell<JPanel>>> {
        return self.newActionScriptInputJPanel.clone();
    }
    

    pub fn setNewActionScriptInputJPanel(&mut self, mut newActionScriptInputJPanel: Option<Rc<RefCell<JPanel>>>) {
        self.newActionScriptInputJPanel = newActionScriptInputJPanel.clone();
    }
    

    pub fn getRemoveJPanel(&mut self) -> Option<Rc<RefCell<JPanel>>> {
        return self.removeJPanel.clone();
    }
    

    pub fn setRemoveJPanel(&mut self, mut removeJPanel: Option<Rc<RefCell<JPanel>>>) {
        self.removeJPanel = removeJPanel.clone();
    }
    

    pub fn getScriptItemJPanel(&mut self) -> Option<Rc<RefCell<JPanel>>> {
        return self.scriptItemJPanel.clone();
    }
    

    pub fn setScriptItemJPanel(&mut self, mut scriptItemJPanel: Option<Rc<RefCell<JPanel>>>) {
        self.scriptItemJPanel = scriptItemJPanel.clone();
    }
    

    pub fn getNewActionScriptConditionJPanel(&mut self) -> Option<Rc<RefCell<JPanel>>> {
        return self.newActionScriptConditionJPanel.clone();
    }
    

    pub fn setNewActionScriptConditionJPanel(&mut self, mut newActionScriptConditionJPanel: Option<Rc<RefCell<JPanel>>>) {
        self.newActionScriptConditionJPanel = newActionScriptConditionJPanel.clone();
    }
    

    pub fn getChildActionScriptConditionJPanel(&mut self) -> Option<Rc<RefCell<JPanel>>> {
        return self.childActionScriptConditionJPanel.clone();
    }
    

    pub fn setChildActionScriptConditionJPanel(&mut self, mut childActionScriptConditionJPanel: Option<Rc<RefCell<JPanel>>>) {
        self.childActionScriptConditionJPanel = childActionScriptConditionJPanel.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.newActionScriptInputJPanel = JPanel::new();
        self.childActionScriptConditionJPanel = JPanel::new();
        self.newActionScriptConditionJPanel = JPanel::new();
        self.scriptItemJPanel = JPanel::new();
        self.removeJPanel = JPanel::new();
        jcall!(self.newActionScriptInputJPanel).setLayout(GridLayout::new(1, 0));
        let mut childActionScriptConditionJPanelLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.childActionScriptConditionJPanel.clone());
        jcall!(self.childActionScriptConditionJPanel).setLayout(childActionScriptConditionJPanelLayout.clone());
        jcall!(childActionScriptConditionJPanelLayout).setHorizontalGroup(jcall!(jcall!(childActionScriptConditionJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 229, i16::MAX.clone()));
        jcall!(childActionScriptConditionJPanelLayout).setVerticalGroup(jcall!(jcall!(childActionScriptConditionJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 8, i16::MAX.clone()));
        jcall!(self.newActionScriptConditionJPanel).setLayout(GridLayout::new(1, 0));
        jcall!(self.scriptItemJPanel).setLayout(GridLayout::new(1, 1));
        jcall!(self.removeJPanel).setLayout(GridLayout::new(1, 1));
        jcall!(self.removeJPanel).setMinimumSize(Dimension::new(30, 20));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.scriptItemJPanel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 167, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.removeJPanel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 56, i16::MAX.clone()))).addComponent(self.newActionScriptInputJPanel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 229, i16::MAX.clone())).addComponent(self.newActionScriptConditionJPanel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 229, i16::MAX.clone())).addComponent(self.childActionScriptConditionJPanel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone()));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.scriptItemJPanel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 31, i16::MAX.clone())).addComponent(self.removeJPanel.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 31, i16::MAX.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.childActionScriptConditionJPanel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.newActionScriptConditionJPanel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 28, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.newActionScriptInputJPanel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 30, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())));
    }
}
impl std::ops::Deref for ActionScriptItemJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ActionScriptItemJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



