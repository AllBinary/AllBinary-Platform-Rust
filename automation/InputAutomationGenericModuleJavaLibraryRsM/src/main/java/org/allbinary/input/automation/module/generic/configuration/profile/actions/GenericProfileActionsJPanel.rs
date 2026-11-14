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
use crate::org::allbinary::input::automation::module::DefaultListModelHelper::DefaultListModelHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::JPanel::JPanel;
use super::GenericProfileActions::GenericProfileActions;
use super::GenericProfileActionJPanel::GenericProfileActionJPanel;
use super::JDialog::JDialog;
use super::JLabel::JLabel;
use super::JButton::JButton;
use super::JTextField::JTextField;
use super::JScrollPane::JScrollPane;
use super::JList::JList;
use super::Dimension::Dimension;
use super::ActionListener::ActionListener;
use super::ActionEvent::ActionEvent;
use super::GroupLayout::GroupLayout;
use super::ListSelectionListener::ListSelectionListener;
use super::ListSelectionEvent::ListSelectionEvent;
use super::GenericProfileAction::GenericProfileAction;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericProfileActionsJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub genericProfileActions: Option<Rc<RefCell<GenericProfileActions>>>,
    pub genericProfileActionJPanel: Option<Rc<RefCell<GenericProfileActionJPanel>>>,
    pub blankGenericProfileActionJPanel: Option<Rc<RefCell<JPanel>>>,
    pub deleteActionJButton: Option<Rc<RefCell<JButton>>>,
    pub genericProfileActionsJList: Option<Rc<RefCell<JList>>>,
    pub genericProfileActionsJPanel: Option<Rc<RefCell<JPanel>>>,
    pub genericProfileActionsJScrollPane: Option<Rc<RefCell<JScrollPane>>>,
    pub newActionJButton: Option<Rc<RefCell<JButton>>>,
    pub newProfileActionJDialog: Option<Rc<RefCell<JDialog>>>,
    pub newProfileActionJLabel: Option<Rc<RefCell<JLabel>>>,
    pub newProfileActionJTextField: Option<Rc<RefCell<JTextField>>>,
    pub okNewProfileActionJButton: Option<Rc<RefCell<JButton>>>,
    pub saveJButton: Option<Rc<RefCell<JButton>>>,
}


impl GenericProfileActionsJPanel {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            genericProfileActions: None,
            genericProfileActionJPanel: None,
            blankGenericProfileActionJPanel: None,
            deleteActionJButton: None,
            genericProfileActionsJList: None,
            genericProfileActionsJPanel: None,
            genericProfileActionsJScrollPane: None,
            newActionJButton: None,
            newProfileActionJDialog: None,
            newProfileActionJLabel: None,
            newProfileActionJTextField: None,
            okNewProfileActionJButton: None,
            saveJButton: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.initComponents();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn initComponents(&mut self) {
        self.newProfileActionJDialog = JDialog::new();
        self.newProfileActionJLabel = JLabel::new();
        self.okNewProfileActionJButton = JButton::new();
        self.newProfileActionJTextField = JTextField::new();
        self.genericProfileActionsJPanel = JPanel::new();
        self.genericProfileActionsJScrollPane = JScrollPane::new();
        self.genericProfileActionsJList = JList::new();
        self.newActionJButton = JButton::new();
        self.deleteActionJButton = JButton::new();
        self.saveJButton = JButton::new();
        self.blankGenericProfileActionJPanel = JPanel::new();
        jcall!(self.newProfileActionJDialog).setMinimumSize(Dimension::new(250, 150));
        jcall!(self.newProfileActionJDialog).setResizable(false);
        jcall!(self.newProfileActionJLabel).setText(String::from("New Action Name"));
        jcall!(self.okNewProfileActionJButton).setText(String::from("Ok"));
        { let java_receiver = self.okNewProfileActionJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.okNewProfileActionJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        let mut newProfileActionJDialogLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(jcall!(self.newProfileActionJDialog).getContentPane());
        jcall!(jcall!(self.newProfileActionJDialog).getContentPane()).setLayout(newProfileActionJDialogLayout.clone());
        jcall!(newProfileActionJDialogLayout).setHorizontalGroup(jcall!(jcall!(newProfileActionJDialogLayout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(newProfileActionJDialogLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(jcall!(newProfileActionJDialogLayout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(newProfileActionJDialogLayout).createSequentialGroup()).addGap(93, 93, 93)).addComponent(self.okNewProfileActionJButton.clone()))).addGroup(jcall!(jcall!(jcall!(newProfileActionJDialogLayout).createSequentialGroup()).addGap(75, 75, 75)).addComponent(self.newProfileActionJLabel.clone()))).addGroup(jcall!(jcall!(jcall!(newProfileActionJDialogLayout).createSequentialGroup()).addGap(29, 29, 29)).addComponent(self.newProfileActionJTextField.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 178, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())))).addContainerGap(34, i16::MAX.clone())));
        jcall!(newProfileActionJDialogLayout).setVerticalGroup(jcall!(jcall!(newProfileActionJDialogLayout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).TRAILING.clone(); java_field }.clone(), jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(newProfileActionJDialogLayout).createSequentialGroup()).addContainerGap()).addComponent(self.newProfileActionJLabel.clone())).addGap(21, 21, 21)).addComponent(self.newProfileActionJTextField.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone(), 20, i16::MAX.clone())).addComponent(self.okNewProfileActionJButton.clone())).addContainerGap()));
        jcall!(self.genericProfileActionsJPanel).setPreferredSize(Dimension::new(10000, 10000));
        { let java_receiver = self.genericProfileActionsJList.clone(); let java_arguments = (ListSelectionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn valueChanged(&mut self, mut evt: Option<Rc<RefCell<ListSelectionEvent>>>) {
                self.genericProfileActionsJListValueChanged(evt.clone());
            }
*/,); jcall!(java_receiver).addListSelectionListener(java_arguments.0); };
        jcall!(self.genericProfileActionsJScrollPane).setViewportView(self.genericProfileActionsJList.clone());
        jcall!(self.newActionJButton).setText(String::from("New"));
        { let java_receiver = self.newActionJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.newActionJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.deleteActionJButton).setText(String::from("Delete"));
        { let java_receiver = self.deleteActionJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.deleteActionJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.saveJButton).setText(String::from("Save"));
        { let java_receiver = self.saveJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.saveJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.blankGenericProfileActionJPanel).setMinimumSize(Dimension::new(500, 300));
        let mut blankGenericProfileActionJPanelLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.blankGenericProfileActionJPanel.clone());
        jcall!(self.blankGenericProfileActionJPanel).setLayout(blankGenericProfileActionJPanelLayout.clone());
        jcall!(blankGenericProfileActionJPanelLayout).setHorizontalGroup(jcall!(jcall!(blankGenericProfileActionJPanelLayout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGap(0, 500, i16::MAX.clone()));
        jcall!(blankGenericProfileActionJPanelLayout).setVerticalGroup(jcall!(jcall!(blankGenericProfileActionJPanelLayout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGap(0, 296, i16::MAX.clone()));
        let mut genericProfileActionsJPanelLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.genericProfileActionsJPanel.clone());
        jcall!(self.genericProfileActionsJPanel).setLayout(genericProfileActionsJPanelLayout.clone());
        jcall!(genericProfileActionsJPanelLayout).setHorizontalGroup(jcall!(jcall!(jcall!(genericProfileActionsJPanelLayout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(genericProfileActionsJPanelLayout).createSequentialGroup()).addComponent(self.genericProfileActionsJScrollPane.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 95, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.blankGenericProfileActionJPanel.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), i16::MAX.clone()))).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(genericProfileActionsJPanelLayout).createSequentialGroup()).addComponent(self.newActionJButton.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.deleteActionJButton.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.saveJButton.clone())).addContainerGap()));
        jcall!(genericProfileActionsJPanelLayout).setVerticalGroup(jcall!(jcall!(genericProfileActionsJPanelLayout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(genericProfileActionsJPanelLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(jcall!(genericProfileActionsJPanelLayout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).BASELINE.clone(); java_field }.clone())).addComponent(self.newActionJButton.clone())).addComponent(self.deleteActionJButton.clone())).addComponent(self.saveJButton.clone()))).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(genericProfileActionsJPanelLayout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone(), false)).addComponent(self.blankGenericProfileActionJPanel.clone(), 0, 296, i16::MAX.clone())).addComponent(self.genericProfileActionsJScrollPane.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), 296, i16::MAX.clone()))).addContainerGap({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), i16::MAX.clone())));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.genericProfileActionsJPanel.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 596, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addContainerGap({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), i16::MAX.clone())));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addComponent(self.genericProfileActionsJPanel.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 336, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone()));
    }
    

    pub fn setModel(&mut self) {
        let mut defaultListModelHelper: Option<Rc<RefCell<DefaultListModelHelper>>> = jcall!(self.getGenericProfileActions()).getDefaultListModelHelper();
        if jcall!(jcall!(defaultListModelHelper).getListModel()).getSize() > 0 {
            { let java_receiver = self.genericProfileActionsJList.clone(); let java_arguments = (jcall!(defaultListModelHelper).getListModel(),); jcall!(java_receiver).setModel(java_arguments.0); };
        }
    }
    

    pub fn saveJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.getGenericProfileActions()).save();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveJButtonActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn updateProfileActionUI(&mut self, mut profileActionNameString: String) {
        jcall!(self.blankGenericProfileActionJPanel).removeAll();
        if true {
            let mut genericProfileAction: Option<Rc<RefCell<GenericProfileAction>>> = jcall!(self.getGenericProfileActions()).getAction(profileActionNameString.clone());
            self.setGenericProfileActionJPanel(GenericProfileActionJPanel::new());
            jcall!(self.getGenericProfileActionJPanel()).setGenericProfileAction(genericProfileAction.clone());
            jcall!(self.getGenericProfileActionJPanel()).updateProfileActionUI();
            let mut layout: Option<Rc<RefCell<GroupLayout>>> = (jcall!(self.blankGenericProfileActionJPanel).getLayout()).clone() /* as GroupLayout */;
            jcall!(layout).setHorizontalGroup({ let java_argument = jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGap(0, 395, i16::MAX.clone())).addComponent(self.getGenericProfileActionJPanel(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone()); java_argument });
            jcall!(layout).setVerticalGroup({ let java_argument = jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGap(0, 256, i16::MAX.clone())).addComponent(self.getGenericProfileActionJPanel(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone()); java_argument });
            jcall!(self.logUtil).putF(String::from("Setting Configuration Profile Actions JPanel"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("genericProfileActionsJListValueChanged"));
        }
    }
    

    pub fn genericProfileActionsJListValueChanged(&mut self, mut evt: Option<Rc<RefCell<ListSelectionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut profileActionNameString: String = (jcall!(self.genericProfileActionsJList).getSelectedValue()).clone() /* as String */;
            self.updateProfileActionUI(profileActionNameString.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("genericProfileActionsJListValueChanged"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn okNewProfileActionJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut newActionName: String = jcall!(self.newProfileActionJTextField).getText();
            jcall!(self.getGenericProfileActions()).add(newActionName.clone());
            self.setModel();
            jcall!(self.genericProfileActionsJList).setSelectedValue(newActionName.clone(), true);
            jcall!(self.newProfileActionJDialog).setVisible(false);
            self.updateProfileActionUI(newActionName.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("okNewProfileActionJButtonActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn deleteActionJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.genericProfileActionsJList).getSelectedValues();
            for index in (0..(anyType.len() as i32)) {
                let mut nextSelectedValueString: String = { let java_object = (anyType[(index) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                jcall!(self.getGenericProfileActions()).remove(nextSelectedValueString.clone());
            }
            self.selectFirstElement();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("deleteActionJButtonActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn selectFirstElement(&mut self) {
        self.setModel();
        if jcall!(jcall!(self.genericProfileActionsJList).getModel()).getSize() > 0 {
            { let java_receiver = self.genericProfileActionsJList.clone(); let java_arguments = (jcall!(jcall!(self.genericProfileActionsJList).getModel()).getElementAt(0), true,); jcall!(java_receiver).setSelectedValue(java_arguments.0, java_arguments.1); };
            { let java_arguments = ((jcall!(jcall!(self.genericProfileActionsJList).getModel()).getElementAt(0)).clone() /* as String */,); self.updateProfileActionUI(java_arguments.0) };
        } else {
            jcall!(self.blankGenericProfileActionJPanel).removeAll();
            jcall!(self.blankGenericProfileActionJPanel).repaint();
        }
    }
    

    pub fn getGenericProfileActionJPanel(&mut self) -> Option<Rc<RefCell<GenericProfileActionJPanel>>> {
        return self.genericProfileActionJPanel.clone();
    }
    

    pub fn setGenericProfileActionJPanel(&mut self, mut genericProfileActionJPanel: Option<Rc<RefCell<GenericProfileActionJPanel>>>) {
        self.genericProfileActionJPanel = genericProfileActionJPanel.clone();
    }
    

    pub fn getGenericProfileActions(&mut self) -> Option<Rc<RefCell<GenericProfileActions>>> {
        return self.genericProfileActions.clone();
    }
    

    pub fn setGenericProfileActions(&mut self, mut genericProfileActions: Option<Rc<RefCell<GenericProfileActions>>>) {
        self.genericProfileActions = genericProfileActions.clone();
    }
    

    pub fn newActionJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        jcall!(self.newProfileActionJDialog).setVisible(true);
    }
}
impl std::ops::Deref for GenericProfileActionsJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GenericProfileActionsJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



