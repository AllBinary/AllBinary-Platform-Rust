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
// package org.allbinary.input.automation.module.generic.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfile::GenericProfile;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfiles::GenericProfiles;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::GenericProfileActions::GenericProfileActions;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::GenericProfileActionsJPanel::GenericProfileActionsJPanel;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::JPanel::JPanel;
use super::swing::swing;
use super::javax::javax;
use super::GenericModuleConfigurationInterface::GenericModuleConfigurationInterface;
use super::JDialog::JDialog;
use super::JButton::JButton;
use super::JLabel::JLabel;
use super::JTextField::JTextField;
use super::ButtonGroup::ButtonGroup;
use super::JTabbedPane::JTabbedPane;
use super::JScrollPane::JScrollPane;
use super::JList::JList;
use super::JRadioButton::JRadioButton;
use super::JCheckBox::JCheckBox;
use super::Dimension::Dimension;
use super::awt::awt;
use super::java::java;
use super::ActionListener::ActionListener;
use super::event::event;
use super::ActionEvent::ActionEvent;
use super::GroupLayout::GroupLayout;
use super::GridLayout::GridLayout;
use super::ListSelectionListener::ListSelectionListener;
use super::ListSelectionEvent::ListSelectionEvent;
use super::Insets::Insets;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericModuleConfigurationJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub genericProfiles: Option<Rc<RefCell<GenericProfiles>>>,
    pub selectedGenericProfile: Option<Rc<RefCell<GenericProfile>>>,
    pub blankGenericProfileActionsJPanel: Option<Rc<RefCell<JPanel>>>,
    pub buttonGroup2: Option<Rc<RefCell<ButtonGroup>>>,
    pub captureComparisonJCheckBox: Option<Rc<RefCell<JCheckBox>>>,
    pub captureMotionAnalysisJCheckBox: Option<Rc<RefCell<JCheckBox>>>,
    pub deleteProfileJButton: Option<Rc<RefCell<JButton>>>,
    pub genericJTabbedPane: Option<Rc<RefCell<JTabbedPane>>>,
    pub genericProfilesJList: Option<Rc<RefCell<JList>>>,
    pub genericProfilesJPanel: Option<Rc<RefCell<JPanel>>>,
    pub genericProfilesJScrollPane: Option<Rc<RefCell<JScrollPane>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
    pub jLabel2: Option<Rc<RefCell<JLabel>>>,
    pub jLabel3: Option<Rc<RefCell<JLabel>>>,
    pub jPanel1: Option<Rc<RefCell<JPanel>>>,
    pub newProfileJButton: Option<Rc<RefCell<JButton>>>,
    pub newProfileJDialog: Option<Rc<RefCell<JDialog>>>,
    pub newProfileNameJLabel: Option<Rc<RefCell<JLabel>>>,
    pub newProfileNameJTextField: Option<Rc<RefCell<JTextField>>>,
    pub okNewProfileJButton: Option<Rc<RefCell<JButton>>>,
    pub savedCaptureJRadioButton: Option<Rc<RefCell<JRadioButton>>>,
    pub screenCaptureJRadioButton: Option<Rc<RefCell<JRadioButton>>>,
}

impl GenericModuleConfigurationInterface for GenericModuleConfigurationJPanel {
    fn getSelectedGenericProfile(&mut self) -> Option<Rc<RefCell<GenericProfile>>> { Self::getSelectedGenericProfile(self) }
    fn setSelectedGenericProfile(&mut self, mut selectedGenericProfile: Option<Rc<RefCell<GenericProfile>>>) { Self::setSelectedGenericProfile(self, selectedGenericProfile) }
}

impl GenericModuleConfigurationJPanel {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            stringUtil: None,
            commonStrings: None,
            genericProfiles: None,
            selectedGenericProfile: None,
            blankGenericProfileActionsJPanel: None,
            buttonGroup2: None,
            captureComparisonJCheckBox: None,
            captureMotionAnalysisJCheckBox: None,
            deleteProfileJButton: None,
            genericJTabbedPane: None,
            genericProfilesJList: None,
            genericProfilesJPanel: None,
            genericProfilesJScrollPane: None,
            jLabel1: None,
            jLabel2: None,
            jLabel3: None,
            jPanel1: None,
            newProfileJButton: None,
            newProfileJDialog: None,
            newProfileNameJLabel: None,
            newProfileNameJTextField: None,
            okNewProfileJButton: None,
            savedCaptureJRadioButton: None,
            screenCaptureJRadioButton: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.stringUtil = StringUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.initComponents();
        this.setGenericProfiles(GenericProfiles::new(crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfiles::GenericProfiles_DEFAULT_FILE.with(Clone::clone).clone()));
        this.selectFirstElement();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn initComponents(&mut self) {
        self.newProfileJDialog = JDialog::new();
        self.okNewProfileJButton = JButton::new();
        self.newProfileNameJLabel = JLabel::new();
        self.newProfileNameJTextField = JTextField::new();
        self.buttonGroup2 = ButtonGroup::new();
        self.genericJTabbedPane = JTabbedPane::new();
        self.genericProfilesJPanel = JPanel::new();
        self.genericProfilesJScrollPane = JScrollPane::new();
        self.genericProfilesJList = JList::new();
        self.newProfileJButton = JButton::new();
        self.deleteProfileJButton = JButton::new();
        self.jPanel1 = JPanel::new();
        self.screenCaptureJRadioButton = JRadioButton::new();
        self.jLabel1 = JLabel::new();
        self.jLabel2 = JLabel::new();
        self.savedCaptureJRadioButton = JRadioButton::new();
        self.captureMotionAnalysisJCheckBox = JCheckBox::new();
        self.captureComparisonJCheckBox = JCheckBox::new();
        self.jLabel3 = JLabel::new();
        self.blankGenericProfileActionsJPanel = JPanel::new();
        jcall!(self.newProfileJDialog).setMinimumSize(Dimension::new(250, 150));
        jcall!(self.newProfileJDialog).setResizable(false);
        jcall!(self.okNewProfileJButton).setText(String::from("Ok"));
        { let java_receiver = self.okNewProfileJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.okNewProfileJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.newProfileNameJLabel).setText(String::from("New Profile Name"));
        let mut newProfileJDialogLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(jcall!(self.newProfileJDialog).getContentPane());
        jcall!(jcall!(self.newProfileJDialog).getContentPane()).setLayout(newProfileJDialogLayout.clone());
        jcall!(newProfileJDialogLayout).setHorizontalGroup(jcall!(jcall!(newProfileJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(newProfileJDialogLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(jcall!(newProfileJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(newProfileJDialogLayout).createSequentialGroup()).addGap(80, 80, 80)).addComponent(self.newProfileNameJLabel.clone()))).addGroup(jcall!(jcall!(jcall!(newProfileJDialogLayout).createSequentialGroup()).addGap(30, 30, 30)).addComponent(self.newProfileNameJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 180, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addGroup(jcall!(jcall!(jcall!(newProfileJDialogLayout).createSequentialGroup()).addGap(100, 100, 100)).addComponent(self.okNewProfileJButton.clone())))).addContainerGap(35, i16::MAX.clone())));
        jcall!(newProfileJDialogLayout).setVerticalGroup(jcall!(jcall!(newProfileJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(newProfileJDialogLayout).createSequentialGroup()).addGap(10, 10, 10)).addComponent(self.newProfileNameJLabel.clone())).addGap(16, 16, 16)).addComponent(self.newProfileNameJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addGap(20, 20, 20)).addComponent(self.okNewProfileJButton.clone())));
        self.setLayout(GridLayout::new(1, 0));
        self.setMinimumSize(Dimension::new(400, 300));
        self.setPreferredSize(Dimension::new(700, 700));
        jcall!(self.genericJTabbedPane).setToolTipText(jcall!(self.stringUtil).EMPTY_STRING.clone());
        jcall!(self.genericJTabbedPane).setMinimumSize(Dimension::new(800, 600));
        jcall!(self.genericProfilesJPanel).setMinimumSize(Dimension::new(400, 300));
        jcall!(self.genericProfilesJPanel).setPreferredSize(Dimension::new(0, 0));
        { let java_receiver = self.genericProfilesJList.clone(); let java_arguments = (ListSelectionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn valueChanged(&mut self, mut evt: Option<Rc<RefCell<ListSelectionEvent>>>) {
                self.genericProfilesJListValueChanged(evt.clone());
            }
*/,); jcall!(java_receiver).addListSelectionListener(java_arguments.0); };
        jcall!(self.genericProfilesJScrollPane).setViewportView(self.genericProfilesJList.clone());
        jcall!(self.newProfileJButton).setText(String::from("New"));
        jcall!(self.newProfileJButton).setToolTipText(String::from("Creates a new profile"));
        { let java_receiver = self.newProfileJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.newProfileJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.deleteProfileJButton).setText(String::from("Delete"));
        jcall!(self.deleteProfileJButton).setToolTipText(String::from("Deletes the selected profile"));
        { let java_receiver = self.deleteProfileJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.deleteProfileJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jPanel1).setPreferredSize(Dimension::new(1000, 100));
        jcall!(self.screenCaptureJRadioButton).setText(String::from("Screen Captures"));
        jcall!(self.screenCaptureJRadioButton).setToolTipText(String::from("Records the screen into memory for logical processing"));
        { let java_receiver = self.screenCaptureJRadioButton.clone(); let java_arguments = (jcall!(jcall!(unsafe { javax_swing.clone() }).BorderFactory).createEmptyBorder(0, 0, 0, 0),); jcall!(java_receiver).setBorder(java_arguments.0); };
        jcall!(self.screenCaptureJRadioButton).setMargin(Insets::new(0, 0, 0, 0));
        { let java_receiver = self.screenCaptureJRadioButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.screenCaptureJRadioButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel1).setText(String::from("Profile Options"));
        jcall!(self.jLabel2).setText(String::from("Process From:"));
        jcall!(self.savedCaptureJRadioButton).setText(String::from("Saved Images"));
        jcall!(self.savedCaptureJRadioButton).setToolTipText(String::from("Loads images from files for logical processing.  Usually for debugging your profile actions"));
        { let java_receiver = self.savedCaptureJRadioButton.clone(); let java_arguments = (jcall!(jcall!(unsafe { javax_swing.clone() }).BorderFactory).createEmptyBorder(0, 0, 0, 0),); jcall!(java_receiver).setBorder(java_arguments.0); };
        jcall!(self.savedCaptureJRadioButton).setMargin(Insets::new(0, 0, 0, 0));
        { let java_receiver = self.savedCaptureJRadioButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.savedCaptureJRadioButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.captureMotionAnalysisJCheckBox).setText(String::from("Motion Analysis"));
        jcall!(self.captureMotionAnalysisJCheckBox).setToolTipText(String::from("Uses image comparison and contraints to locate moving objects"));
        { let java_receiver = self.captureMotionAnalysisJCheckBox.clone(); let java_arguments = (jcall!(jcall!(unsafe { javax_swing.clone() }).BorderFactory).createEmptyBorder(0, 0, 0, 0),); jcall!(java_receiver).setBorder(java_arguments.0); };
        jcall!(self.captureMotionAnalysisJCheckBox).setMargin(Insets::new(0, 0, 0, 0));
        { let java_receiver = self.captureMotionAnalysisJCheckBox.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.captureMotionAnalysisJCheckBoxActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.captureComparisonJCheckBox).setText(String::from("Comparison"));
        jcall!(self.captureComparisonJCheckBox).setToolTipText(String::from("Compares the last two input images for multi image logic"));
        { let java_receiver = self.captureComparisonJCheckBox.clone(); let java_arguments = (jcall!(jcall!(unsafe { javax_swing.clone() }).BorderFactory).createEmptyBorder(0, 0, 0, 0),); jcall!(java_receiver).setBorder(java_arguments.0); };
        jcall!(self.captureComparisonJCheckBox).setMargin(Insets::new(0, 0, 0, 0));
        { let java_receiver = self.captureComparisonJCheckBox.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.captureComparisonJCheckBoxActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel3).setText(String::from("Image Processing:"));
        jcall!(self.jLabel3).setToolTipText(String::from("Turns on/off various extended image processing"));
        let mut jPanel1Layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.jPanel1.clone());
        jcall!(self.jPanel1).setLayout(jPanel1Layout.clone());
        jcall!(jPanel1Layout).setHorizontalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jPanel1Layout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.screenCaptureJRadioButton.clone())).addComponent(self.jLabel1.clone())).addComponent(self.jLabel2.clone())).addComponent(self.savedCaptureJRadioButton.clone())).addComponent(self.captureMotionAnalysisJCheckBox.clone())).addComponent(self.captureComparisonJCheckBox.clone())).addComponent(self.jLabel3.clone()))).addContainerGap(22, i16::MAX.clone())));
        jcall!(jPanel1Layout).setVerticalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jPanel1Layout).createSequentialGroup()).addComponent(self.jLabel1.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jLabel2.clone())).addGap(7, 7, 7)).addComponent(self.screenCaptureJRadioButton.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.savedCaptureJRadioButton.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jLabel3.clone())).addGap(7, 7, 7)).addComponent(self.captureComparisonJCheckBox.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.captureMotionAnalysisJCheckBox.clone())).addContainerGap(106, i16::MAX.clone())));
        let mut genericProfilesJPanelLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.genericProfilesJPanel.clone());
        jcall!(self.genericProfilesJPanel).setLayout(genericProfilesJPanelLayout.clone());
        jcall!(genericProfilesJPanelLayout).setHorizontalGroup(jcall!(jcall!(genericProfilesJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(genericProfilesJPanelLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(genericProfilesJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(genericProfilesJPanelLayout).createSequentialGroup()).addComponent(self.newProfileJButton.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.deleteProfileJButton.clone()))).addGroup(jcall!(jcall!(jcall!(jcall!(genericProfilesJPanelLayout).createSequentialGroup()).addComponent(self.genericProfilesJScrollPane.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jPanel1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 119, i16::MAX.clone())))).addContainerGap()));
        jcall!(genericProfilesJPanelLayout).setVerticalGroup(jcall!(jcall!(genericProfilesJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(genericProfilesJPanelLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(genericProfilesJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.newProfileJButton.clone())).addComponent(self.deleteProfileJButton.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(genericProfilesJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.jPanel1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 246, i16::MAX.clone())).addComponent(self.genericProfilesJScrollPane.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 246, i16::MAX.clone()))));
        jcall!(self.genericJTabbedPane).addTab(String::from("Profiles"), self.genericProfilesJPanel.clone());
        let mut blankGenericProfileActionsJPanelLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.blankGenericProfileActionsJPanel.clone());
        jcall!(self.blankGenericProfileActionsJPanel).setLayout(blankGenericProfileActionsJPanelLayout.clone());
        jcall!(blankGenericProfileActionsJPanelLayout).setHorizontalGroup(jcall!(jcall!(blankGenericProfileActionsJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 395, i16::MAX.clone()));
        jcall!(blankGenericProfileActionsJPanelLayout).setVerticalGroup(jcall!(jcall!(blankGenericProfileActionsJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 275, i16::MAX.clone()));
        jcall!(self.genericJTabbedPane).addTab(String::from("Actions"), self.blankGenericProfileActionsJPanel.clone());
        self.add(self.genericJTabbedPane.clone());
    }
    

    pub fn savedCaptureJRadioButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(self.savedCaptureJRadioButton).isSelected() {
                jcall!(self.logUtil).putF(String::from("Selected"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("savedCaptureJRadioButtonActionPerformed"));
                jcall!(self.getSelectedGenericProfile()).add(unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_SAVED_CAPTURE.clone() }.clone());
                jcall!(self.getSelectedGenericProfile()).remove(unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_SCREEN_CAPTURE.clone() }.clone());
                jcall!(self.screenCaptureJRadioButton).setSelected(false);
                jcall!(self.getGenericProfiles()).save();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("savedCaptureJRadioButtonActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn screenCaptureJRadioButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(self.screenCaptureJRadioButton).isSelected() {
                jcall!(self.logUtil).putF(String::from("Selected"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("screenCaptureJRadioButtonActionPerformed"));
                jcall!(self.getSelectedGenericProfile()).add(unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_SCREEN_CAPTURE.clone() }.clone());
                jcall!(self.getSelectedGenericProfile()).remove(unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_SAVED_CAPTURE.clone() }.clone());
                jcall!(self.savedCaptureJRadioButton).setSelected(false);
                jcall!(self.getGenericProfiles()).save();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("screenCaptureJRadioButtonActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn captureMotionAnalysisJCheckBoxActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(self.captureMotionAnalysisJCheckBox).isSelected() {
                jcall!(self.getSelectedGenericProfile()).add(unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_MOTION.clone() }.clone());
            } else {
                jcall!(self.getSelectedGenericProfile()).remove(unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_MOTION.clone() }.clone());
            }
            jcall!(self.getGenericProfiles()).save();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("captureComparisonJCheckBoxActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn captureComparisonJCheckBoxActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(self.captureComparisonJCheckBox).isSelected() {
                jcall!(self.getSelectedGenericProfile()).add(unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_COMPARISON.clone() }.clone());
            } else {
                jcall!(self.getSelectedGenericProfile()).remove(unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_COMPARISON.clone() }.clone());
            }
            jcall!(self.getGenericProfiles()).save();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("captureComparisonJCheckBoxActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn updateProfileUI(&mut self, mut profileNameString: String) {
        jcall!(self.getBlankGenericProfileActionsJPanel()).removeAll();
        if true {
            let mut genericProfileActionsJPanel: Option<Rc<RefCell<GenericProfileActionsJPanel>>> = GenericProfileActionsJPanel::new();
            { let java_arguments = (jcall!(self.getGenericProfiles()).get(profileNameString.clone()),); self.setSelectedGenericProfile(java_arguments.0) };
            jcall!(self.getSelectedGenericProfile()).setGenericProfileActions(GenericProfileActions::new2(genericProfileActionsJPanel.clone(), profileNameString.clone()));
            jcall!(genericProfileActionsJPanel).setGenericProfileActions(jcall!(self.getSelectedGenericProfile()).getGenericProfileActions());
            jcall!(genericProfileActionsJPanel).selectFirstElement();
            let mut layout: Option<Rc<RefCell<GroupLayout>>> = (jcall!(self.getBlankGenericProfileActionsJPanel()).getLayout()).clone() /* as GroupLayout */;
            jcall!(layout).setHorizontalGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 395, i16::MAX.clone())).addComponent(genericProfileActionsJPanel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()));
            jcall!(layout).setVerticalGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 256, i16::MAX.clone())).addComponent(genericProfileActionsJPanel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()));
            self.updateProfileOptions();
            jcall!(self.logUtil).putF(String::from("Setting Configuration Profile JPanel"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateProfileUI"));
        } else {
            jcall!(self.logUtil).putF(String::from("Null Configuration Profile"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateProfileUI"));
        }
    }
    

    // throws Exception
    pub fn selectFirstElement(&mut self) {
        { let java_receiver = self.genericProfilesJList.clone(); let java_arguments = (jcall!(jcall!(self.getGenericProfiles()).getDefaultListModelHelper()).getListModel(),); jcall!(java_receiver).setModel(java_arguments.0); };
        if jcall!(jcall!(self.genericProfilesJList).getModel()).getSize() > 0 {
            { let java_receiver = self.genericProfilesJList.clone(); let java_arguments = (jcall!(jcall!(self.genericProfilesJList).getModel()).getElementAt(0), true,); jcall!(java_receiver).setSelectedValue(java_arguments.0, java_arguments.1); };
            { let java_arguments = ((jcall!(jcall!(self.genericProfilesJList).getModel()).getElementAt(0)).clone() /* as String */,); self.updateProfileUI(java_arguments.0) };
        }
    }
    

    // throws Exception
    pub fn updateProfileOptions(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateProfileOptions"));
        let mut genericProfile: Option<Rc<RefCell<GenericProfile>>> = self.getSelectedGenericProfile();
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(genericProfile).getGenericProfileDataWorkerTypeVector();
        jcall!(self.screenCaptureJRadioButton).setSelected(false);
        jcall!(self.savedCaptureJRadioButton).setSelected(false);
        jcall!(self.captureComparisonJCheckBox).setSelected(false);
        jcall!(self.captureMotionAnalysisJCheckBox).setSelected(false);
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            let mut genericProfileDataWorkerType: Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerTypeDyn>>> = { let java_object = (jcall!(vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerTypeDyn>>>>().expect("ClassCastException").clone()) };
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", jcall!(genericProfile).getName(), String::from(" has GenericProfileDataWorkerType: ")), jcall!(genericProfileDataWorkerType).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateProfileOptions"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if match (&genericProfileDataWorkerType, &unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_SCREEN_CAPTURE.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.logUtil).putF(String::from("Screen Capture Selected"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateProfileOptions"));
                jcall!(self.screenCaptureJRadioButton).setSelected(true);
            } else if match (&genericProfileDataWorkerType, &unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_SAVED_CAPTURE.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.logUtil).putF(String::from("Saved Capture Selected"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateProfileOptions"));
                jcall!(self.savedCaptureJRadioButton).setSelected(true);
            } else if match (&genericProfileDataWorkerType, &unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_COMPARISON.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.logUtil).putF(String::from("Checking Comparison Box"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateProfileOptions"));
                jcall!(self.captureComparisonJCheckBox).setSelected(true);
            } else if match (&genericProfileDataWorkerType, &unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_MOTION.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.logUtil).putF(String::from("Checking Motion Box"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateProfileOptions"));
                jcall!(self.captureMotionAnalysisJCheckBox).setSelected(true);
            }
        }
        if !jcall!(self.screenCaptureJRadioButton).isSelected() && !jcall!(self.savedCaptureJRadioButton).isSelected() {
            jcall!(self.screenCaptureJRadioButton).setSelected(true);
            jcall!(self.getSelectedGenericProfile()).add(unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::GenericProfileDataWorkerType::GenericProfileDataWorkerType_SCREEN_CAPTURE.clone() }.clone());
        }
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateProfileOptions"));
    }
    

    pub fn getBlankGenericProfileActionsJPanel(&mut self) -> Option<Rc<RefCell<JPanel>>> {
        return self.blankGenericProfileActionsJPanel.clone();
    }
    

    pub fn setBlankGenericProfileActionsJPanel(&mut self, mut blankGenericProfileActionsJPanel: Option<Rc<RefCell<JPanel>>>) {
        self.blankGenericProfileActionsJPanel = blankGenericProfileActionsJPanel.clone();
    }
    

    pub fn getGenericProfiles(&mut self) -> Option<Rc<RefCell<GenericProfiles>>> {
        return self.genericProfiles.clone();
    }
    

    pub fn setGenericProfiles(&mut self, mut genericProfiles: Option<Rc<RefCell<GenericProfiles>>>) {
        self.genericProfiles = genericProfiles.clone();
    }
    

    pub fn getSelectedGenericProfile(&mut self) -> Option<Rc<RefCell<GenericProfile>>> {
        return self.selectedGenericProfile.clone();
    }
    

    pub fn setSelectedGenericProfile(&mut self, mut selectedGenericProfile: Option<Rc<RefCell<GenericProfile>>>) {
        self.selectedGenericProfile = selectedGenericProfile.clone();
    }
    

    pub fn genericProfilesJListValueChanged(&mut self, mut evt: Option<Rc<RefCell<ListSelectionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut profileNameString: String = (jcall!(self.genericProfilesJList).getSelectedValue()).clone() /* as String */;
            self.updateProfileUI(profileNameString.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("genericProfilesJListValueChanged"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn deleteProfileJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut anyType: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.genericProfilesJList).getSelectedValues();
            for index in (0..(anyType.len() as i32)) {
                let mut nextSelectedValueString: String = (anyType[(index) as usize]).clone() /* as String */;
                jcall!(self.getGenericProfiles()).remove(nextSelectedValueString.clone());
            }
            self.selectFirstElement();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("deleteProfileJButtonActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn okNewProfileJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut newProfileName: String = jcall!(self.newProfileNameJTextField).getText();
            jcall!(self.getGenericProfiles()).add(newProfileName.clone());
            { let java_receiver = self.genericProfilesJList.clone(); let java_arguments = (jcall!(jcall!(self.getGenericProfiles()).getDefaultListModelHelper()).getListModel(),); jcall!(java_receiver).setModel(java_arguments.0); };
            jcall!(self.genericProfilesJList).setSelectedValue(newProfileName.clone(), true);
            jcall!(self.newProfileJDialog).setVisible(false);
            self.updateProfileUI(newProfileName.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("okNewProfileJButtonActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn newProfileJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        jcall!(self.newProfileJDialog).setVisible(true);
    }
}
impl std::ops::Deref for GenericModuleConfigurationJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GenericModuleConfigurationJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



