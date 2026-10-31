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
use crate::org::allbinary::input::automation::actions::script::condition::ProfileActionScriptConditionInterface::ProfileActionScriptConditionInterface;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::GenericProfileAction::GenericProfileAction;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::GenericProfileActionJPanel::GenericProfileActionJPanel;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::Panel::Panel;
use super::awt::awt;
use super::java::java;
use super::JButton::JButton;
use super::swing::swing;
use super::javax::javax;
use super::GridLayout::GridLayout;
use super::ActionListener::ActionListener;
use super::event::event;
use super::ActionEvent::ActionEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RemoveActionScriptItemJPanel {
    pub base: Panel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub genericProfileActionJPanel: Option<Rc<RefCell<GenericProfileActionJPanel>>>,
    pub profileActionScriptConditionInterface: Option<Rc<RefCell<dyn ProfileActionScriptConditionInterface>>>,
    pub removeJButton: Option<Rc<RefCell<JButton>>>,
}


impl RemoveActionScriptItemJPanel {
    

    pub fn new_struct(mut genericProfileActionJPanel: Option<Rc<RefCell<GenericProfileActionJPanel>>>, mut profileActionScriptConditionInterface: Option<Rc<RefCell<dyn ProfileActionScriptConditionInterface>>>) -> Self {
        let mut this = Self {
            base: Panel::new_struct(),
            logUtil: None,
            genericProfileActionJPanel: None,
            profileActionScriptConditionInterface: None,
            removeJButton: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.initComponents();
        this.genericProfileActionJPanel = genericProfileActionJPanel.clone();
        this.profileActionScriptConditionInterface = profileActionScriptConditionInterface.clone();
        this
    }

    pub fn new(mut genericProfileActionJPanel: Option<Rc<RefCell<GenericProfileActionJPanel>>>, mut profileActionScriptConditionInterface: Option<Rc<RefCell<dyn ProfileActionScriptConditionInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(genericProfileActionJPanel, profileActionScriptConditionInterface))))
    }
    

    pub fn initComponents(&mut self) {
        self.removeJButton = JButton::new();
        self.setLayout(GridLayout::new());
        jcall!(self.removeJButton).setText(String::from("Remove"));
        { let java_receiver = self.removeJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.removeJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        self.add(self.removeJButton.clone());
    }
    

    pub fn removeJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        let mut genericProfileAction: Option<Rc<RefCell<GenericProfileAction>>> = jcall!(self.genericProfileActionJPanel).getGenericProfileAction();
        if genericProfileAction.is_none() {
            jcall!(self.logUtil).putF(String::from("GenericProfileAction was null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeJButtonActionPerformed"));
            return;
        }
        jcall!(jcall!(genericProfileAction).getGenericProfileActionScript()).removeCondition(self.profileActionScriptConditionInterface.clone());
        jcall!(self.genericProfileActionJPanel).updateProfileActionUI();
    }
}
impl std::ops::Deref for RemoveActionScriptItemJPanel {
    type Target = Panel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RemoveActionScriptItemJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



