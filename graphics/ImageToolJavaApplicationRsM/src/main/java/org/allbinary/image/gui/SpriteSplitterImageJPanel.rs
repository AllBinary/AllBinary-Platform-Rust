// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.image.gui
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::java::awt::image::RenderedImage::RenderedImage;
use crate::java::io::File::File;
use crate::java::io::IOException::IOException;
// use crate::javax::imageio::ImageIO::ImageIO;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::media::image::ImageProcessedVisitor::ImageProcessedVisitor;
use crate::org::allbinary::media::image::ImageProcessorInput::ImageProcessorInput;
use crate::org::allbinary::media::image::ImageStrings::ImageStrings;
use crate::org::allbinary::media::image::SpriteSplitterUtil::SpriteSplitterUtil;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Thread::*;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SpriteSplitterImageJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub imageStrings: Option<Rc<RefCell<ImageStrings>>>,
    pub imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>,
    pub result: Option<Rc<RefCell<BufferedImage>>>,
    pub generateJButton: Option<Rc<RefCell<JButton>>>,
    pub heightReductionTextField: Option<Rc<RefCell<JTextField>>>,
    pub increaseHeightTextField: Option<Rc<RefCell<JTextField>>>,
    pub increaseWidthTextField: Option<Rc<RefCell<JTextField>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
    pub jLabel2: Option<Rc<RefCell<JLabel>>>,
    pub jLabel3: Option<Rc<RefCell<JLabel>>>,
    pub jLabel4: Option<Rc<RefCell<JLabel>>>,
    pub jLabel5: Option<Rc<RefCell<JLabel>>>,
    pub jLabel6: Option<Rc<RefCell<JLabel>>>,
    pub jLabel7: Option<Rc<RefCell<JLabel>>>,
    pub jLabel8: Option<Rc<RefCell<JLabel>>>,
    pub jPanel1: Option<Rc<RefCell<JPanel>>>,
    pub jPanel2: Option<Rc<RefCell<JPanel>>>,
    pub spriteTypeJComboBox: Option<Rc<RefCell<JComboBox<String>>>>,
    pub totalAnimationsJComboBox: Option<Rc<RefCell<JComboBox<String>>>>,
    pub totalFramesJComboBox: Option<Rc<RefCell<JComboBox<String>>>>,
    pub widthReductionTextField: Option<Rc<RefCell<JTextField>>>,
    pub writeOverOriginalJCheckBox: Option<Rc<RefCell<JCheckBox>>>,
}

impl ImageProcessedVisitor for SpriteSplitterImageJPanel {
    fn visit(&mut self, mut generatedBufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut name: String, mut index: i32) { Self::visit(self, generatedBufferedImage, name, index) }
}
impl crate::org::allbinary::media::image::ImageProcessorInputCompositeInterface::ImageProcessorInputCompositeInterface for SpriteSplitterImageJPanel {
    fn getImageProcessorInput(&mut self) -> Option<Rc<RefCell<ImageProcessorInput>>> { Self::getImageProcessorInput(self) }
    fn setImageProcessorInput(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) { Self::setImageProcessorInput(self, imageProcessorInput) }
}

impl SpriteSplitterImageJPanel {
    

    pub fn new_struct(mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            commonSeps: None,
            imageStrings: None,
            imageProcessorInput: None,
            result: None,
            generateJButton: None,
            heightReductionTextField: None,
            increaseHeightTextField: None,
            increaseWidthTextField: None,
            jLabel1: None,
            jLabel2: None,
            jLabel3: None,
            jLabel4: None,
            jLabel5: None,
            jLabel6: None,
            jLabel7: None,
            jLabel8: None,
            jPanel1: None,
            jPanel2: None,
            spriteTypeJComboBox: None,
            totalAnimationsJComboBox: None,
            totalFramesJComboBox: None,
            widthReductionTextField: None,
            writeOverOriginalJCheckBox: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.imageStrings = ImageStrings::getInstance();
        // super(...) is in the struct literal above
        this.initComponents();
        this.imageProcessorInput = imageProcessorInput.clone();
        this
    }

    pub fn new(mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageProcessorInput))))
    }
    

    pub fn process(&mut self) {
        jcall!(Thread::new() /* Java anonymous class body: implement it as a named type
            

            pub fn run(&mut self) {
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut spriteSplitterUtil: Option<Rc<RefCell<SpriteSplitterUtil>>> = SpriteSplitterUtil::getInstance();
                    let mut spriteType: String = (jcall!(self.spriteTypeJComboBox).getSelectedItem()).clone() / * as String * /;
                    let mut totalFrames: Option<i32> = Some(((jcall!(self.totalFramesJComboBox).getSelectedItem()).clone() / * as String * /) as i32);
                    let mut totalAnimations: Option<i32> = Some(((jcall!(self.totalAnimationsJComboBox).getSelectedItem()).clone() / * as String * /) as i32);
                    let mut widthReduction: i32 = Some((jcall!(self.widthReductionTextField).getText()) as i32);
                    let mut heightReduction: i32 = Some((jcall!(self.heightReductionTextField).getText()) as i32);
                    let mut increaseWidth: i32 = Some((jcall!(self.increaseWidthTextField).getText()) as i32);
                    let mut increaseHeight: i32 = Some((jcall!(self.increaseHeightTextField).getText()) as i32);
                    jcall!(spriteSplitterUtil).process(self.getImageProcessorInput(), totalFrames.clone(), totalAnimations.clone(), widthReduction.clone(), heightReduction.clone(), increaseWidth.clone(), increaseHeight.clone(), spriteType.clone(), self);
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                    jcall!(logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
*/).start();
    }
    

    pub fn initComponents(&mut self) {
        self.writeOverOriginalJCheckBox = JCheckBox::new();
        self.widthReductionTextField = JTextField::new();
        self.jLabel5 = JLabel::new();
        self.jPanel1 = { let java_value = (JPanel::new() /* Java anonymous class body: implement it as a named type
            

            pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
                if self.result.is_some() {
                    jcall!(graphics).as_Graphics_mut().drawImage(self.result.clone(), 0, 0, jcall!(self.result).getWidth(None), jcall!(self.result).getHeight(None), None);
                }
            }
*/).clone(); java_value };
        self.jLabel4 = JLabel::new();
        self.jLabel6 = JLabel::new();
        self.heightReductionTextField = JTextField::new();
        self.generateJButton = JButton::new();
        self.jPanel2 = JPanel::new();
        self.jLabel1 = JLabel::new();
        self.totalFramesJComboBox = JComboBox::new();
        let mut size: i32 = 70;
        let mut animationStringArray2: Vec<String> = vec![String::new(); (size - 1) as usize];
        for index in (1..size) {
            animationStringArray2[(index - 1) as usize] = (index.clone()).to_string();
        }
        jcall!(self.totalFramesJComboBox).setModel(DefaultComboBoxModel::new(animationStringArray2.clone()));
        self.jLabel2 = JLabel::new();
        self.totalAnimationsJComboBox = JComboBox::new();
        let mut animationStringArray: Vec<String> = vec![String::new(); (size - 1) as usize];
        for index in (1..size) {
            animationStringArray[(index - 1) as usize] = (index.clone()).to_string();
        }
        jcall!(self.totalAnimationsJComboBox).setModel(DefaultComboBoxModel::new(animationStringArray.clone()));
        self.jLabel3 = JLabel::new();
        self.spriteTypeJComboBox = JComboBox::new();
        { let java_receiver = self.spriteTypeJComboBox.clone(); let java_arguments = (DefaultComboBoxModel::new(vec![{ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).media.clone(); java_field }).image.clone(); java_field }).SpriteSplitterUtil.clone(); java_field }).getInstance()).DIRECTIONAL_ANIMATIONS.clone(); java_field }.clone(), { let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).media.clone(); java_field }).image.clone(); java_field }).SpriteSplitterUtil.clone(); java_field }).getInstance()).HORIZONTAL_ANIMATIONS.clone(); java_field }.clone(), { let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).media.clone(); java_field }).image.clone(); java_field }).SpriteSplitterUtil.clone(); java_field }).getInstance()).HORIZONTAL_SPRITE.clone(); java_field }.clone()]),); jcall!(java_receiver).setModel(java_arguments.0); };
        self.jLabel7 = JLabel::new();
        self.increaseWidthTextField = JTextField::new();
        self.jLabel8 = JLabel::new();
        self.increaseHeightTextField = JTextField::new();
        jcall!(self.writeOverOriginalJCheckBox).setText(String::from("Write Over Original"));
        { let java_receiver = self.writeOverOriginalJCheckBox.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.writeOverOriginalJCheckBoxActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        { let java_receiver = self.widthReductionTextField.clone(); let java_arguments = ({ let java_field = jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(); java_field }.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.widthReductionTextField.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.widthReductionTextFieldActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel5).setText(String::from("Less Width:"));
        jcall!(self.jLabel4).setText(String::from("Results:"));
        let mut jPanel1Layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.jPanel1.clone());
        jcall!(self.jPanel1).setLayout(jPanel1Layout.clone());
        jcall!(jPanel1Layout).setHorizontalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(jPanel1Layout).createSequentialGroup()).addComponent(self.jLabel4.clone())).addGap(0, 0, i16::MAX.clone())));
        jcall!(jPanel1Layout).setVerticalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(jPanel1Layout).createSequentialGroup()).addComponent(self.jLabel4.clone())).addGap(0, 184, i16::MAX.clone())));
        jcall!(self.jLabel6).setText(String::from("Less Height:"));
        { let java_receiver = self.heightReductionTextField.clone(); let java_arguments = ({ let java_field = jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(); java_field }.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        jcall!(self.generateJButton).setText(String::from("Generate"));
        { let java_receiver = self.generateJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.generateJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel1).setText(String::from("Row Total:"));
        jcall!(self.totalFramesJComboBox).setSelectedIndex(1);
        { let java_receiver = self.totalFramesJComboBox.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.totalFramesJComboBoxActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel2).setText(String::from("Column Total :"));
        jcall!(self.totalAnimationsJComboBox).setSelectedIndex(3);
        { let java_receiver = self.totalAnimationsJComboBox.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.totalAnimationsJComboBoxActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel3).setText(String::from("Type:"));
        { let java_receiver = self.spriteTypeJComboBox.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.spriteTypeJComboBoxActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        let mut jPanel2Layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.jPanel2.clone());
        jcall!(self.jPanel2).setLayout(jPanel2Layout.clone());
        jcall!(jPanel2Layout).setHorizontalGroup(jcall!(jcall!(jPanel2Layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).TRAILING.clone(); java_field }.clone(), jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jPanel2Layout).createSequentialGroup()).addContainerGap()).addComponent(self.jLabel1.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.totalFramesJComboBox.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 48, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.jLabel2.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.totalAnimationsJComboBox.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 39, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.jLabel3.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.spriteTypeJComboBox.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 172, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addContainerGap({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), i16::MAX.clone())));
        jcall!(jPanel2Layout).setVerticalGroup(jcall!(jcall!(jPanel2Layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(jPanel2Layout).createSequentialGroup()).addContainerGap({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), i16::MAX.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jPanel2Layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).BASELINE.clone(); java_field }.clone())).addComponent(self.jLabel1.clone())).addComponent(self.totalFramesJComboBox.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addComponent(self.jLabel2.clone())).addComponent(self.totalAnimationsJComboBox.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addComponent(self.jLabel3.clone())).addComponent(self.spriteTypeJComboBox.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone()))));
        jcall!(self.jLabel7).setText(String::from("More Width:"));
        { let java_receiver = self.increaseWidthTextField.clone(); let java_arguments = ({ let java_field = jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(); java_field }.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.increaseWidthTextField.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.increaseWidthTextFieldActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel8).setText(String::from("More Height:"));
        { let java_receiver = self.increaseHeightTextField.clone(); let java_arguments = ({ let java_field = jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(); java_field }.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.jPanel2.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 415, i16::MAX.clone())).addGap(78, 78, 78))).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addComponent(self.jPanel1.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), i16::MAX.clone())).addGroup(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.writeOverOriginalJCheckBox.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.generateJButton.clone()))).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.jLabel5.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.widthReductionTextField.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 41, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.jLabel6.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.heightReductionTextField.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 46, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.jLabel7.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.increaseWidthTextField.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 41, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.jLabel8.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.increaseHeightTextField.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 46, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())))).addGap(0, 0, i16::MAX.clone())))).addContainerGap()));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.jPanel2.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).BASELINE.clone(); java_field }.clone())).addComponent(self.jLabel7.clone())).addComponent(self.increaseWidthTextField.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 22, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addComponent(self.jLabel8.clone())).addComponent(self.increaseHeightTextField.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone()))).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).BASELINE.clone(); java_field }.clone())).addComponent(self.jLabel5.clone())).addComponent(self.widthReductionTextField.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 22, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addComponent(self.jLabel6.clone())).addComponent(self.heightReductionTextField.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())))).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).BASELINE.clone(); java_field }.clone())).addComponent(self.writeOverOriginalJCheckBox.clone())).addComponent(self.generateJButton.clone()))).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addComponent(self.jPanel1.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), i16::MAX.clone())));
    }
    

    pub fn writeOverOriginalJCheckBoxActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn generateJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.process();
    }
    

    pub fn spriteTypeJComboBoxActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        let mut spriteSplitterUtil: Option<Rc<RefCell<SpriteSplitterUtil>>> = SpriteSplitterUtil::getInstance();
        { let java_receiver = self.jLabel1.clone(); let java_arguments = (jcall!(spriteSplitterUtil).ANIMATIONS_LABELS[(jcall!(self.spriteTypeJComboBox).getSelectedIndex()) as usize].clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.jLabel2.clone(); let java_arguments = (jcall!(spriteSplitterUtil).ANIMATIONS_LABELS[(jcall!(self.spriteTypeJComboBox).getSelectedIndex() + 1) as usize].clone(),); jcall!(java_receiver).setText(java_arguments.0); };
    }
    

    pub fn totalAnimationsJComboBoxActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn totalFramesJComboBoxActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn widthReductionTextFieldActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn increaseWidthTextFieldActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn getImageProcessorInput(&mut self) -> Option<Rc<RefCell<ImageProcessorInput>>> {
        return self.imageProcessorInput.clone();
    }
    

    pub fn setImageProcessorInput(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) {
        self.imageProcessorInput = imageProcessorInput.clone();
    }
    

    // throws IOException
    pub fn visit(&mut self, mut generatedBufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut name: String, mut index: i32) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).VISIT.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).VISIT.clone(); java_field }.clone());
        self.result = generatedBufferedImage.clone();
        jcall!(self.getParent()).repaint();
        let mut file: Option<Rc<RefCell<File>>> = jcall!(self.imageProcessorInput).getFiles()[(index) as usize].clone();
        if !jcall!(self.writeOverOriginalJCheckBox).isSelected() {
            let mut filePath: String = jcall!(file).getAbsolutePath();
            let mut extensionIndex: i32 = filePath.find(&{ let java_field = jcall!(self.imageStrings).PNG_EXTENSION.clone(); java_field }.clone()).map(|index| index as i32).unwrap_or(-1);
            filePath = { let java_value = (jcall!(StringMaker::new()).append(filePath[(0) as usize..(extensionIndex) as usize].to_string()).append({ let java_field = jcall!(self.commonSeps).UNDERSCORE.clone(); java_field }.clone()).append(name.clone()).append({ let java_field = jcall!(self.imageStrings).PNG_EXTENSION.clone(); java_field }.clone()).toString()).clone(); java_value };
            file = File::new1(filePath.clone());
        }
        let mut isWritten: bool = ImageIO::write((self.result).clone() /* as RenderedImage */, { let java_field = jcall!(self.imageStrings).PNG.clone(); java_field }.clone(), file.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("File: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(file.clone()))).append(String::from(" Wrote: ")).appendboolean(isWritten.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
}
impl std::ops::Deref for SpriteSplitterImageJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SpriteSplitterImageJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



