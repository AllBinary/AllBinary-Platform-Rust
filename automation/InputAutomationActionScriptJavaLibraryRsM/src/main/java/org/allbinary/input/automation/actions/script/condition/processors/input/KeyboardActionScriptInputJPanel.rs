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
// package org.allbinary.input.automation.actions.script.condition.processors.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::swing::*;
use crate::org::allbinary::input::KeySingletonFactory::KeySingletonFactory;
use crate::org::allbinary::input::automation::robot::InputRobot::InputRobot;
use crate::org::allbinary::input::automation::robot::InputRobotFactory::InputRobotFactory;
use crate::org::allbinary::input::automation::robot::TempInputRobotNames::TempInputRobotNames;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct KeyboardActionScriptInputJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub keyActionScriptInputInterface: Option<Rc<RefCell<dyn KeyboardActionScriptInputInterface>>>,
    pub addKeyJLabel: Option<Rc<RefCell<JLabel>>>,
    pub delayBetweenJLabel: Option<Rc<RefCell<JLabel>>>,
    pub delayJTextField: Option<Rc<RefCell<JTextField>>>,
    pub holdTimeJLabel: Option<Rc<RefCell<JLabel>>>,
    pub inputAutomationTypeJComboBox: Option<Rc<RefCell<JComboBox>>>,
    pub inputTypeJLabel: Option<Rc<RefCell<JLabel>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
    pub jPanel1: Option<Rc<RefCell<JPanel>>>,
    pub keyActionJButton: Option<Rc<RefCell<JButton>>>,
    pub keyActionJDialog: Option<Rc<RefCell<JDialog>>>,
    pub keyIntegerJComboBox: Option<Rc<RefCell<JComboBox>>>,
    pub keyNameJComboBox: Option<Rc<RefCell<JComboBox>>>,
    pub normalJRadioButton: Option<Rc<RefCell<JRadioButton>>>,
    pub okJButton: Option<Rc<RefCell<JButton>>>,
    pub pressJRadioButton: Option<Rc<RefCell<JRadioButton>>>,
    pub releaseJRadioButton: Option<Rc<RefCell<JRadioButton>>>,
    pub sequenceJRadioButton: Option<Rc<RefCell<JRadioButton>>>,
    pub simultaneousJRadioButton: Option<Rc<RefCell<JRadioButton>>>,
    pub testJLabel: Option<Rc<RefCell<JLabel>>>,
    pub textJTextField: Option<Rc<RefCell<JTextField>>>,
    pub timeJTextField: Option<Rc<RefCell<JTextField>>>,
    pub titleJLabel: Option<Rc<RefCell<JLabel>>>,
}


impl KeyboardActionScriptInputJPanel {
    

    pub fn new_struct(mut keyActionScriptInputInterface: Option<Rc<RefCell<dyn KeyboardActionScriptInputInterface>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            keyActionScriptInputInterface: None,
            addKeyJLabel: None,
            delayBetweenJLabel: None,
            delayJTextField: None,
            holdTimeJLabel: None,
            inputAutomationTypeJComboBox: None,
            inputTypeJLabel: None,
            jLabel1: None,
            jPanel1: None,
            keyActionJButton: None,
            keyActionJDialog: None,
            keyIntegerJComboBox: None,
            keyNameJComboBox: None,
            normalJRadioButton: None,
            okJButton: None,
            pressJRadioButton: None,
            releaseJRadioButton: None,
            sequenceJRadioButton: None,
            simultaneousJRadioButton: None,
            testJLabel: None,
            textJTextField: None,
            timeJTextField: None,
            titleJLabel: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.initComponents();
        this.keyActionScriptInputInterface = keyActionScriptInputInterface.clone();
        let mut defaultComboBoxModel: Option<Rc<RefCell<DefaultComboBoxModel>>> = DefaultComboBoxModel::new();
        jcall!(defaultComboBoxModel).addElement(crate::org::allbinary::input::automation::robot::TempInputRobotNames::TempInputRobotNames_DX_NAME.with(Clone::clone).clone());
        jcall!(defaultComboBoxModel).addElement(crate::org::allbinary::input::automation::robot::InputRobot::InputRobot_NAME.with(Clone::clone).clone());
        jcall!(this.inputAutomationTypeJComboBox).setModel(defaultComboBoxModel.clone());
        { let java_receiver = this.keyNameJComboBox.clone(); let java_arguments = (KeyNameDefaultComboBoxModelFactory::KeyNameDefaultComboBoxModelFactory::getInstance(),); jcall!(java_receiver).setModel(java_arguments.0); };
        { let java_receiver = this.keyIntegerJComboBox.clone(); let java_arguments = (KeyCharDefaultComboBoxModelFactory::KeyCharDefaultComboBoxModelFactory::getInstance(),); jcall!(java_receiver).setModel(java_arguments.0); };
        if jcall!(this.keyActionScriptInputInterface).isNormal() {
            jcall!(this.normalJRadioButton).setSelected(true);
            jcall!(this.pressJRadioButton).setSelected(false);
            jcall!(this.releaseJRadioButton).setSelected(false);
        } else if jcall!(this.keyActionScriptInputInterface).isPress() {
            jcall!(this.normalJRadioButton).setSelected(false);
            jcall!(this.pressJRadioButton).setSelected(true);
            jcall!(this.releaseJRadioButton).setSelected(false);
        } else if jcall!(this.keyActionScriptInputInterface).isRelease() {
            jcall!(this.normalJRadioButton).setSelected(false);
            jcall!(this.pressJRadioButton).setSelected(false);
            jcall!(this.releaseJRadioButton).setSelected(true);
        }
        this.set();
        this
    }

    pub fn new(mut keyActionScriptInputInterface: Option<Rc<RefCell<dyn KeyboardActionScriptInputInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(keyActionScriptInputInterface))))
    }
    

    pub fn set(&mut self) {
        let mut inputTypeComboBoxModel: Option<Rc<RefCell<ComboBoxModel>>> = jcall!(self.inputAutomationTypeJComboBox).getModel();
        jcall!(inputTypeComboBoxModel).setSelectedItem(jcall!(jcall!(self.keyActionScriptInputInterface).getInputRobotInterface()).getName());
        { let java_receiver = self.textJTextField.clone(); let java_arguments = (jcall!(self.keyActionScriptInputInterface).getText(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.delayJTextField.clone(); let java_arguments = ((jcall!(self.keyActionScriptInputInterface).getDelayBetweenKeys()).to_string(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.timeJTextField.clone(); let java_arguments = ((jcall!(self.keyActionScriptInputInterface).getTime()).to_string(),); jcall!(java_receiver).setText(java_arguments.0); };
        jcall!(self.keyActionScriptInputInterface).log();
    }
    

    // throws Exception
    pub fn updateInputType(&mut self) {
        let mut comboBoxModel: Option<Rc<RefCell<ComboBoxModel>>> = jcall!(self.inputAutomationTypeJComboBox).getModel();
        let mut selectedItem: String = (jcall!(comboBoxModel).getSelectedItem()).clone() /* as String */;
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(selectedItem.clone()) {
            { let java_receiver = self.keyActionScriptInputInterface.clone(); let java_arguments = (jcall!(InputRobotFactory::getInstance()).get1(selectedItem.clone()),); jcall!(java_receiver).setInputRobotInterface(java_arguments.0); };
        }
    }
    

    pub fn getSelectedKey(&mut self) -> Option<i32> {
        let mut comboBoxModel: Option<Rc<RefCell<ComboBoxModel>>> = jcall!(self.keyNameJComboBox).getModel();
        let mut selectedItem: String = (jcall!(comboBoxModel).getSelectedItem()).clone() /* as String */;
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(selectedItem.clone()) {
            return (jcall!(KeySingletonFactory::getHashtable()).get(selectedItem.clone())).clone() /* as Integer */;
        }
        return None;
    }
    

    pub fn updateKeys(&mut self) {
        let mut text: String = jcall!(self.textJTextField).getText();
        jcall!(self.keyActionScriptInputInterface).setText(text.clone());
    }
    

    // throws Exception
    pub fn update(&mut self) {
        self.updateInputType();
        self.updateKeys();
        { let java_receiver = self.keyActionScriptInputInterface.clone(); let java_arguments = ((((jcall!(self.timeJTextField).getText()).to_string()).unwrap() as i32),); jcall!(java_receiver).setTime(java_arguments.0); };
        jcall!(self.keyActionScriptInputInterface).log();
    }
    

    pub fn getKeyActionJDialog(&mut self) -> Option<Rc<RefCell<JDialog>>> {
        return self.keyActionJDialog.clone();
    }
    

    pub fn setKeyActionJDialog(&mut self, mut keyActionJDialog: Option<Rc<RefCell<JDialog>>>) {
        self.keyActionJDialog = keyActionJDialog.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.keyActionJDialog = JDialog::new();
        self.titleJLabel = JLabel::new();
        self.addKeyJLabel = JLabel::new();
        self.keyNameJComboBox = JComboBox::new();
        self.timeJTextField = JTextField::new();
        self.holdTimeJLabel = JLabel::new();
        self.inputAutomationTypeJComboBox = JComboBox::new();
        self.inputTypeJLabel = JLabel::new();
        self.delayJTextField = JTextField::new();
        self.delayBetweenJLabel = JLabel::new();
        self.testJLabel = JLabel::new();
        self.textJTextField = JTextField::new();
        self.pressJRadioButton = JRadioButton::new();
        self.releaseJRadioButton = JRadioButton::new();
        self.normalJRadioButton = JRadioButton::new();
        self.simultaneousJRadioButton = JRadioButton::new();
        self.sequenceJRadioButton = JRadioButton::new();
        self.jPanel1 = JPanel::new();
        self.okJButton = JButton::new();
        self.keyIntegerJComboBox = JComboBox::new();
        self.keyActionJButton = JButton::new();
        self.jLabel1 = JLabel::new();
        jcall!(self.keyActionJDialog).setMinimumSize(Dimension::new(255, 280));
        jcall!(self.titleJLabel).setHorizontalAlignment(jcall!(jcall!(unsafe { javax_swing.clone() }).SwingConstants).CENTER.clone());
        jcall!(self.titleJLabel).setText(String::from("Keyboard Input Options"));
        jcall!(self.addKeyJLabel).setText(String::from("Add Key:"));
        { let java_receiver = self.keyNameJComboBox.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.keyNameJComboBoxActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        { let java_receiver = self.timeJTextField.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        jcall!(self.holdTimeJLabel).setText(String::from("Hold Time (ms):"));
        jcall!(self.inputTypeJLabel).setText(String::from("Input Type:"));
        { let java_receiver = self.delayJTextField.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        jcall!(self.delayBetweenJLabel).setText(String::from("Delay Between Keys (ms):"));
        jcall!(self.testJLabel).setText(String::from("Text:"));
        { let java_receiver = self.textJTextField.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        jcall!(self.pressJRadioButton).setText(String::from("Press"));
        { let java_receiver = self.pressJRadioButton.clone(); let java_arguments = (jcall!(jcall!(unsafe { javax_swing.clone() }).BorderFactory).createEmptyBorder(0, 0, 0, 0),); jcall!(java_receiver).setBorder(java_arguments.0); };
        jcall!(self.pressJRadioButton).setMargin(Insets::new(0, 0, 0, 0));
        { let java_receiver = self.pressJRadioButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.pressJRadioButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.releaseJRadioButton).setText(String::from("Release"));
        { let java_receiver = self.releaseJRadioButton.clone(); let java_arguments = (jcall!(jcall!(unsafe { javax_swing.clone() }).BorderFactory).createEmptyBorder(0, 0, 0, 0),); jcall!(java_receiver).setBorder(java_arguments.0); };
        jcall!(self.releaseJRadioButton).setMargin(Insets::new(0, 0, 0, 0));
        { let java_receiver = self.releaseJRadioButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.releaseJRadioButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.normalJRadioButton).setSelected(true);
        jcall!(self.normalJRadioButton).setText(String::from("Normal"));
        { let java_receiver = self.normalJRadioButton.clone(); let java_arguments = (jcall!(jcall!(unsafe { javax_swing.clone() }).BorderFactory).createEmptyBorder(0, 0, 0, 0),); jcall!(java_receiver).setBorder(java_arguments.0); };
        jcall!(self.normalJRadioButton).setMargin(Insets::new(0, 0, 0, 0));
        { let java_receiver = self.normalJRadioButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.normalJRadioButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.simultaneousJRadioButton).setSelected(true);
        jcall!(self.simultaneousJRadioButton).setText(String::from("Simultaneous"));
        { let java_receiver = self.simultaneousJRadioButton.clone(); let java_arguments = (jcall!(jcall!(unsafe { javax_swing.clone() }).BorderFactory).createEmptyBorder(0, 0, 0, 0),); jcall!(java_receiver).setBorder(java_arguments.0); };
        jcall!(self.simultaneousJRadioButton).setMargin(Insets::new(0, 0, 0, 0));
        { let java_receiver = self.simultaneousJRadioButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.simultaneousJRadioButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.sequenceJRadioButton).setText(String::from("Sequence"));
        { let java_receiver = self.sequenceJRadioButton.clone(); let java_arguments = (jcall!(jcall!(unsafe { javax_swing.clone() }).BorderFactory).createEmptyBorder(0, 0, 0, 0),); jcall!(java_receiver).setBorder(java_arguments.0); };
        jcall!(self.sequenceJRadioButton).setMargin(Insets::new(0, 0, 0, 0));
        { let java_receiver = self.sequenceJRadioButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.sequenceJRadioButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.okJButton).setText(String::from("OK"));
        jcall!(self.okJButton).setHorizontalTextPosition(jcall!(jcall!(unsafe { javax_swing.clone() }).SwingConstants).CENTER.clone());
        { let java_receiver = self.okJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.okJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jPanel1).add(self.okJButton.clone());
        { let java_receiver = self.keyIntegerJComboBox.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.keyIntegerJComboBoxActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        let mut keyActionJDialogLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(jcall!(self.keyActionJDialog).getContentPane());
        jcall!(jcall!(self.keyActionJDialog).getContentPane()).setLayout(keyActionJDialogLayout.clone());
        jcall!(keyActionJDialogLayout).setHorizontalGroup(jcall!(jcall!(jcall!(jcall!(keyActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(keyActionJDialogLayout).createSequentialGroup()).addContainerGap()).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(keyActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(keyActionJDialogLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(keyActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), false)).addComponent(self.addKeyJLabel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).addComponent(self.inputTypeJLabel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(keyActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone())).addComponent(self.inputAutomationTypeJComboBox.clone(), 0, 148, i16::MAX.clone())).addComponent(self.textJTextField.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 148, i16::MAX.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(keyActionJDialogLayout).createSequentialGroup()).addComponent(self.keyNameJComboBox.clone(), 0, 101, i16::MAX.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.keyIntegerJComboBox.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 41, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))))).addComponent(self.testJLabel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 60, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(keyActionJDialogLayout).createSequentialGroup()).addComponent(self.normalJRadioButton.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.pressJRadioButton.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.releaseJRadioButton.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), 48, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(jcall!(jcall!(keyActionJDialogLayout).createSequentialGroup()).addComponent(self.holdTimeJLabel.clone())).addGap(22, 22, 22)).addComponent(self.timeJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 113, i16::MAX.clone()))).addGroup(jcall!(jcall!(jcall!(jcall!(keyActionJDialogLayout).createSequentialGroup()).addComponent(self.delayBetweenJLabel.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), 15, i16::MAX.clone())).addComponent(self.delayJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 68, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addGroup(jcall!(jcall!(jcall!(jcall!(keyActionJDialogLayout).createSequentialGroup()).addComponent(self.simultaneousJRadioButton.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.sequenceJRadioButton.clone())))).addContainerGap())).addComponent(self.titleJLabel.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 229, i16::MAX.clone())).addComponent(self.jPanel1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 229, i16::MAX.clone()));
        jcall!(keyActionJDialogLayout).setVerticalGroup(jcall!(jcall!(keyActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(keyActionJDialogLayout).createSequentialGroup()).addComponent(self.titleJLabel.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(keyActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.inputTypeJLabel.clone())).addComponent(self.inputAutomationTypeJComboBox.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(keyActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.addKeyJLabel.clone())).addComponent(self.keyIntegerJComboBox.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.keyNameJComboBox.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(keyActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.testJLabel.clone())).addComponent(self.textJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(keyActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.pressJRadioButton.clone())).addComponent(self.normalJRadioButton.clone())).addComponent(self.releaseJRadioButton.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(keyActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.holdTimeJLabel.clone())).addComponent(self.timeJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(keyActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.delayBetweenJLabel.clone())).addComponent(self.delayJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(keyActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.simultaneousJRadioButton.clone())).addComponent(self.sequenceJRadioButton.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jPanel1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addContainerGap()));
        self.setMinimumSize(Dimension::new(100, 0));
        jcall!(self.keyActionJButton).setText(String::from("Edit"));
        { let java_receiver = self.keyActionJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.keyActionJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel1).setText(String::from("Key:"));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.jLabel1.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), 49, i16::MAX.clone())).addComponent(self.keyActionJButton.clone())));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel1.clone())).addComponent(self.keyActionJButton.clone())));
    }
    

    pub fn prependText(&mut self, mut integer: Option<i32>) {
        if integer.is_some() {
            let mut newKey: String = format!("{}{}", format!("{}{}", String::from("&#"), (integer).expect("NullPointerException").to_string()), String::from(";"));
            { let java_receiver = self.textJTextField.clone(); let java_arguments = (format!("{}{}", newKey, jcall!(self.textJTextField).getText()),); jcall!(java_receiver).setText(java_arguments.0); };
        }
    }
    

    pub fn keyIntegerJComboBoxActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        jcall!(self.keyIntegerJComboBox).getSelectedIndex();
        { let java_receiver = self.keyNameJComboBox.clone(); let java_arguments = (jcall!(self.keyIntegerJComboBox).getSelectedIndex(),); jcall!(java_receiver).setSelectedIndex(java_arguments.0); };
    }
    

    pub fn sequenceJRadioButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        if jcall!(self.sequenceJRadioButton).isSelected() {
            { let java_receiver = self.delayJTextField.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).ONE.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
            jcall!(self.simultaneousJRadioButton).setSelected(false);
        }
    }
    

    pub fn simultaneousJRadioButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        if jcall!(self.simultaneousJRadioButton).isSelected() {
            { let java_receiver = self.delayJTextField.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
            jcall!(self.sequenceJRadioButton).setSelected(false);
        }
    }
    

    pub fn keyNameJComboBoxActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        let mut integer: Option<i32> = self.getSelectedKey();
        self.prependText(integer.clone());
        { let java_receiver = self.keyIntegerJComboBox.clone(); let java_arguments = (jcall!(self.keyNameJComboBox).getSelectedIndex(),); jcall!(java_receiver).setSelectedIndex(java_arguments.0); };
    }
    

    pub fn okJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.update();
            jcall!(self.getKeyActionJDialog()).setVisible(false);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("okButtonActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn keyActionJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        jcall!(self.getKeyActionJDialog()).setVisible(true);
    }
    

    pub fn normalJRadioButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        if jcall!(self.normalJRadioButton).isSelected() {
            jcall!(self.keyActionScriptInputInterface).setNormal();
            jcall!(self.pressJRadioButton).setSelected(false);
            jcall!(self.releaseJRadioButton).setSelected(false);
        }
    }
    

    pub fn releaseJRadioButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        if jcall!(self.releaseJRadioButton).isSelected() {
            jcall!(self.keyActionScriptInputInterface).setRelease(true);
            jcall!(self.keyActionScriptInputInterface).setPress(false);
            jcall!(self.pressJRadioButton).setSelected(false);
            jcall!(self.normalJRadioButton).setSelected(false);
        }
    }
    

    pub fn pressJRadioButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        if jcall!(self.pressJRadioButton).isSelected() {
            jcall!(self.keyActionScriptInputInterface).setPress(true);
            jcall!(self.keyActionScriptInputInterface).setRelease(false);
            jcall!(self.releaseJRadioButton).setSelected(false);
            jcall!(self.normalJRadioButton).setSelected(false);
        }
    }
}
impl std::ops::Deref for KeyboardActionScriptInputJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for KeyboardActionScriptInputJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



