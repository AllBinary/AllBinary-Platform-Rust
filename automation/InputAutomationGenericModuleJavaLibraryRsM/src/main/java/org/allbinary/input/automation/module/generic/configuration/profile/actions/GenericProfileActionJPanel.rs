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
// package org.allbinary.input.automation.module.generic.configuration.profile.actions
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::automation::actions::script::JTreeInterfaceFactory::JTreeInterfaceFactory;
use crate::org::allbinary::input::automation::actions::script::condition::AlwaysActionScriptCondition::AlwaysActionScriptCondition;
use crate::org::allbinary::input::automation::actions::script::condition::ColorAtActionScriptCondition::ColorAtActionScriptCondition;
use crate::org::allbinary::input::automation::actions::script::condition::ProfileActionScriptConditionInterface::ProfileActionScriptConditionInterface;
use crate::org::allbinary::input::automation::actions::script::condition::TimeIntervalActionScriptCondition::TimeIntervalActionScriptCondition;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::script::GenericProfileActionScriptJPanel::GenericProfileActionScriptJPanel;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::JPanel::JPanel;
use super::swing::swing;
use super::javax::javax;
use super::GenericProfileAction::GenericProfileAction;
use super::GroupLayout::GroupLayout;
use super::JScrollPane::JScrollPane;
use super::JLabel::JLabel;
use super::JButton::JButton;
use super::Dimension::Dimension;
use super::awt::awt;
use super::java::java;
use super::ActionListener::ActionListener;
use super::event::event;
use super::ActionEvent::ActionEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericProfileActionJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub genericProfileAction: Option<Rc<RefCell<GenericProfileAction>>>,
    pub genericProfileActionScriptJPanel: Option<Rc<RefCell<GenericProfileActionScriptJPanel>>>,
    pub blankProfileActionScriptJPanel: Option<Rc<RefCell<JPanel>>>,
    pub genericProfileActionScriptJScrollPane: Option<Rc<RefCell<JScrollPane>>>,
    pub ifJLabel: Option<Rc<RefCell<JLabel>>>,
    pub newAlwaysJButton: Option<Rc<RefCell<JButton>>>,
    pub newColorAtJButton: Option<Rc<RefCell<JButton>>>,
    pub newTimeIntervalJButton: Option<Rc<RefCell<JButton>>>,
}


impl GenericProfileActionJPanel {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            genericProfileAction: None,
            genericProfileActionScriptJPanel: None,
            blankProfileActionScriptJPanel: None,
            genericProfileActionScriptJScrollPane: None,
            ifJLabel: None,
            newAlwaysJButton: None,
            newColorAtJButton: None,
            newTimeIntervalJButton: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.initComponents();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateProfileActionUI(&mut self) {
        jcall!(self.blankProfileActionScriptJPanel).removeAll();
        let mut genericProfileActionScriptJPanel: Option<Rc<RefCell<GenericProfileActionScriptJPanel>>> = GenericProfileActionScriptJPanel::new();
        jcall!(JTreeInterfaceFactory::getInstance()).set(genericProfileActionScriptJPanel.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::automation::actions::script::JTreeInterface::JTreeInterface>>));
        self.setGenericProfileActionScriptJPanel(genericProfileActionScriptJPanel.clone());
        jcall!(self.getGenericProfileActionScriptJPanel()).setGenericProfileActionScript(jcall!(self.getGenericProfileAction()).getGenericProfileActionScript());
        jcall!(self.getGenericProfileActionScriptJPanel()).updateJTree();
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = (jcall!(self.blankProfileActionScriptJPanel).getLayout()).clone() /* as GroupLayout */;
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 395, i16::MAX.clone())).addComponent(self.getGenericProfileActionScriptJPanel(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 256, i16::MAX.clone())).addComponent(self.getGenericProfileActionScriptJPanel(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()));
        jcall!(self.logUtil).putF(String::from("Updated UI"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateUI"));
    }
    

    pub fn getGenericProfileAction(&mut self) -> Option<Rc<RefCell<GenericProfileAction>>> {
        return self.genericProfileAction.clone();
    }
    

    pub fn setGenericProfileAction(&mut self, mut genericProfileAction: Option<Rc<RefCell<GenericProfileAction>>>) {
        self.genericProfileAction = genericProfileAction.clone();
    }
    

    pub fn getGenericProfileActionScriptJPanel(&mut self) -> Option<Rc<RefCell<GenericProfileActionScriptJPanel>>> {
        return self.genericProfileActionScriptJPanel.clone();
    }
    

    pub fn setGenericProfileActionScriptJPanel(&mut self, mut genericProfileActionScriptJPanel: Option<Rc<RefCell<GenericProfileActionScriptJPanel>>>) {
        self.genericProfileActionScriptJPanel = genericProfileActionScriptJPanel.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.genericProfileActionScriptJScrollPane = JScrollPane::new();
        self.blankProfileActionScriptJPanel = JPanel::new();
        self.ifJLabel = JLabel::new();
        self.newColorAtJButton = JButton::new();
        self.newTimeIntervalJButton = JButton::new();
        self.newAlwaysJButton = JButton::new();
        self.setMinimumSize(Dimension::new(440, 275));
        self.setPreferredSize(Dimension::new(440, 275));
        jcall!(self.genericProfileActionScriptJScrollPane).setPreferredSize(Dimension::new(474, 270));
        jcall!(self.blankProfileActionScriptJPanel).setMinimumSize(Dimension::new(400, 200));
        let mut blankProfileActionScriptJPanelLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.blankProfileActionScriptJPanel.clone());
        jcall!(self.blankProfileActionScriptJPanel).setLayout(blankProfileActionScriptJPanelLayout.clone());
        jcall!(blankProfileActionScriptJPanelLayout).setHorizontalGroup(jcall!(jcall!(blankProfileActionScriptJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 9996, i16::MAX.clone()));
        jcall!(blankProfileActionScriptJPanelLayout).setVerticalGroup(jcall!(jcall!(blankProfileActionScriptJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 9967, i16::MAX.clone()));
        jcall!(self.genericProfileActionScriptJScrollPane).setViewportView(self.blankProfileActionScriptJPanel.clone());
        jcall!(self.ifJLabel).setText(String::from("Conditions:"));
        jcall!(self.newColorAtJButton).setText(String::from("Color Range At"));
        { let java_receiver = self.newColorAtJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.newColorAtJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.newTimeIntervalJButton).setText(String::from("Time Interval"));
        { let java_receiver = self.newTimeIntervalJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.newTimeIntervalJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.newAlwaysJButton).setText(String::from("Always On/Off"));
        { let java_receiver = self.newAlwaysJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.newAlwaysJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.ifJLabel.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.newColorAtJButton.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.newTimeIntervalJButton.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.newAlwaysJButton.clone())).addContainerGap(61, i16::MAX.clone()))).addComponent(self.genericProfileActionScriptJScrollPane.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 440, i16::MAX.clone()));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.ifJLabel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 14, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.newColorAtJButton.clone())).addComponent(self.newTimeIntervalJButton.clone())).addComponent(self.newAlwaysJButton.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.genericProfileActionScriptJScrollPane.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 246, i16::MAX.clone())));
    }
    

    pub fn newTimeIntervalJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        jcall!(self.logUtil).putF(String::from("New Time Interval Action Condition"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("newTimeIntervalJButtonActionPerformed"));
        jcall!(jcall!(self.getGenericProfileAction()).getGenericProfileActionScript()).addCondition((TimeIntervalActionScriptCondition::new()).clone() /* as ProfileActionScriptConditionInterface */);
        self.updateProfileActionUI();
    }
    

    pub fn newAlwaysJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        jcall!(self.logUtil).putF(String::from("New EveryTime Action Condition"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("newEveryTimeJButtonActionPerformed"));
        jcall!(jcall!(self.getGenericProfileAction()).getGenericProfileActionScript()).addCondition((AlwaysActionScriptCondition::new()).clone() /* as ProfileActionScriptConditionInterface */);
        self.updateProfileActionUI();
    }
    

    pub fn newColorAtJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        jcall!(self.logUtil).putF(String::from("New Color At Action Condition"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("newColorAtJButtonActionPerformed"));
        jcall!(jcall!(self.getGenericProfileAction()).getGenericProfileActionScript()).addCondition((ColorAtActionScriptCondition::new()).clone() /* as ProfileActionScriptConditionInterface */);
        self.updateProfileActionUI();
    }
}
impl std::ops::Deref for GenericProfileActionJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GenericProfileActionJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



