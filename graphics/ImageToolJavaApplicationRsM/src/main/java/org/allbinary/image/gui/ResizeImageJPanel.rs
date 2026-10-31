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
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::java::awt::image::Raster::Raster;
// use crate::java::io::File::File;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::FileWrapperUtil::FileWrapperUtil;
use crate::org::allbinary::media::image::ImagePersistanceUtil::ImagePersistanceUtil;
use crate::org::allbinary::media::image::ImageProcessorInput::ImageProcessorInput;
use crate::org::allbinary::media::image::ImageProcessorInputCompositeInterface::ImageProcessorInputCompositeInterface;
use crate::org::allbinary::media::image::ImageUtil::ImageUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Thread::*;
use super::JPanel::JPanel;
use super::swing::swing;
use super::javax::javax;
use super::DefaultComboBoxModel::DefaultComboBoxModel;
use super::JLabel::JLabel;
use super::JComboBox::JComboBox;
use super::JButton::JButton;
use super::JTextField::JTextField;
use super::ActionListener::ActionListener;
use super::event::event;
use super::awt::awt;
use super::java::java;
use super::ActionEvent::ActionEvent;
use super::Dimension::Dimension;
use super::GroupLayout::GroupLayout;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ResizeImageJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>,
    pub aboveJButton: Option<Rc<RefCell<JButton>>>,
    pub adjustFor16AboveJButton: Option<Rc<RefCell<JButton>>>,
    pub adjustFor16BelowJButton: Option<Rc<RefCell<JButton>>>,
    pub floatPercentJTextField: Option<Rc<RefCell<JTextField>>>,
    pub jComboBox1: Option<Rc<RefCell<JComboBox<String>>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
    pub jLabel2: Option<Rc<RefCell<JLabel>>>,
    pub jLabel3: Option<Rc<RefCell<JLabel>>>,
    pub jTextField1: Option<Rc<RefCell<JTextField>>>,
    pub jTextField2: Option<Rc<RefCell<JTextField>>>,
    pub updateJButton: Option<Rc<RefCell<JButton>>>,
}

impl ImageProcessorInputCompositeInterface for ResizeImageJPanel {
    fn getImageProcessorInput(&mut self) -> Option<Rc<RefCell<ImageProcessorInput>>> { Self::getImageProcessorInput(self) }
    fn setImageProcessorInput(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) { Self::setImageProcessorInput(self, imageProcessorInput) }
}

impl ResizeImageJPanel {
    

    pub fn new_struct(mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            imageProcessorInput: None,
            aboveJButton: None,
            adjustFor16AboveJButton: None,
            adjustFor16BelowJButton: None,
            floatPercentJTextField: None,
            jComboBox1: None,
            jLabel1: None,
            jLabel2: None,
            jLabel3: None,
            jTextField1: None,
            jTextField2: None,
            updateJButton: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        // super(...) is in the struct literal above
        this.initComponents();
        this.imageProcessorInput = imageProcessorInput.clone();
        let mut numberStringArray: Vec<String> = vec![String::new(); (101) as usize];
        numberStringArray[(0) as usize] = (-1).to_string();
        for index in (1..100) {
            numberStringArray[(index) as usize] = (index.clone()).to_string();
        }
        jcall!(this.jComboBox1).setModel(DefaultComboBoxModel::new(numberStringArray.clone()));
        let mut araster: Option<Rc<RefCell<Raster>>> = jcall!(jcall!(this.imageProcessorInput).getBufferedImageArray()[(0) as usize]).getAlphaRaster();
        if araster.is_none() {
            println!("{}", String::from("there is no Alpha channel!!!!!!!!!"));
        } else {
            println!("{}", String::from("Alpha channel found !"));
        }
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
                    let mut imageUtil: Option<Rc<RefCell<ImageUtil>>> = ImageUtil::getInstance();
                    let mut percent: Option<i32> = ((jcall!(self.jComboBox1).getSelectedItem()).clone() /* as String * /).to_string();
                    let mut percentAsFloat: Option<f32> = jcall!(self.floatPercentJTextField).getText().parse::<f32>().unwrap();
                    let mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>> = self.getImageProcessorInput();
                    let mut files: Vec<Option<Rc<RefCell<File>>>> = jcall!(imageProcessorInput).getFiles();
                    let mut generatedBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = None;
                    if ((percentAsFloat).unwrap() as i32) != -1 {
                        generatedBufferedImageArray = jcall!(imageUtil).createBufferedImage2(jcall!(imageProcessorInput).getBufferedImageArray(), percentAsFloat.clone(), true);
                    } else if ((percent).unwrap() as i32) != -1 {
                        generatedBufferedImageArray = jcall!(imageUtil).createBufferedImageForResize(jcall!(imageProcessorInput).getBufferedImageArray(), percent.clone(), true);
                    } else {
                        let mut width: Option<i32> = ((jcall!(self.jTextField1).getText()).clone() /* as String * /).to_string();
                        let mut height: Option<i32> = ((jcall!(self.jTextField2).getText()).clone() /* as String * /).to_string();
                        generatedBufferedImageArray = jcall!(imageUtil).createBufferedImage3(jcall!(imageProcessorInput).getBufferedImageArray(), width.clone(), height.clone(), true);
                    }
                    let mut araster: Option<Rc<RefCell<Raster>>> = jcall!(generatedBufferedImageArray[(0) as usize]).getAlphaRaster();
                    if araster.is_none() {
                        println!("{}", String::from("No Alpha Channel In Result"));
                    } else {
                        println!("{}", String::from("Found Alpha Channel In Result"));
                    }
                    let mut imagePersistanceUtil: Option<Rc<RefCell<ImagePersistanceUtil>>> = ImagePersistanceUtil::getInstance();
                    for index in (0..(generatedBufferedImageArray.len() as i32)) {
                        jcall!(imagePersistanceUtil).saveWithBatik(FileWrapperUtil::wrapFile(files[(index) as usize].clone()), generatedBufferedImageArray[(index) as usize].clone());
                    }
                    jcall!(self.getParent()).repaint();
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    jcall!(logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
*/).start();
    }
    

    pub fn update(&mut self) {
        jcall!(Thread::new() /* Java anonymous class body: implement it as a named type
            

            pub fn run(&mut self) {
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>> = self.getImageProcessorInput();
                    let mut size: i32 = (jcall!(imageProcessorInput).getBufferedImageArray().len() as i32).clone();
                    let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(imageProcessorInput).getBufferedImageArray();
                    let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = None;
                    for index in (0..size) {
                        bufferedImage = bufferedImageArray[(0) as usize].clone();
                        { let java_receiver = self.jTextField1.clone(); let java_arguments = ((jcall!(bufferedImage).getWidth()).to_string(),); jcall!(java_receiver).setText(java_arguments.0); };
                        { let java_receiver = self.jTextField2.clone(); let java_arguments = ((jcall!(bufferedImage).getHeight()).to_string(),); jcall!(java_receiver).setText(java_arguments.0); };
                    }
                    jcall!(self.getParent()).repaint();
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    jcall!(logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
*/).start();
    }
    

    pub fn updateFor16Above(&mut self) {
        jcall!(Thread::new() /* Java anonymous class body: implement it as a named type
            

            pub fn run(&mut self) {
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>> = self.getImageProcessorInput();
                    let mut size: i32 = (jcall!(imageProcessorInput).getBufferedImageArray().len() as i32).clone();
                    let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(imageProcessorInput).getBufferedImageArray();
                    let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = None;
                    for index in (0..size) {
                        bufferedImage = bufferedImageArray[(0) as usize].clone();
                        { let java_receiver = self.jTextField1.clone(); let java_arguments = ((jcall!(bufferedImage).getWidth() / 16 * 16).to_string(),); jcall!(java_receiver).setText(java_arguments.0); };
                        { let java_receiver = self.jTextField2.clone(); let java_arguments = ((jcall!(bufferedImage).getHeight() / 16 * 16).to_string(),); jcall!(java_receiver).setText(java_arguments.0); };
                    }
                    jcall!(self.getParent()).repaint();
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    jcall!(logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
*/).start();
    }
    

    pub fn updateFor16Below(&mut self) {
        jcall!(Thread::new() /* Java anonymous class body: implement it as a named type
            

            pub fn run(&mut self) {
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>> = self.getImageProcessorInput();
                    let mut size: i32 = (jcall!(imageProcessorInput).getBufferedImageArray().len() as i32).clone();
                    let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(imageProcessorInput).getBufferedImageArray();
                    let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = None;
                    for index in (0..size) {
                        bufferedImage = bufferedImageArray[(0) as usize].clone();
                        { let java_receiver = self.jTextField1.clone(); let java_arguments = ((((jcall!(bufferedImage).getWidth() / 16) + 1) * 16).to_string(),); jcall!(java_receiver).setText(java_arguments.0); };
                        { let java_receiver = self.jTextField2.clone(); let java_arguments = ((((jcall!(bufferedImage).getHeight() / 16) + 1) * 16).to_string(),); jcall!(java_receiver).setText(java_arguments.0); };
                    }
                    jcall!(self.getParent()).repaint();
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    jcall!(logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
*/).start();
    }
    

    pub fn getImageProcessorInput(&mut self) -> Option<Rc<RefCell<ImageProcessorInput>>> {
        return self.imageProcessorInput.clone();
    }
    

    pub fn setImageProcessorInput(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) {
        self.imageProcessorInput = imageProcessorInput.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.jLabel1 = JLabel::new();
        self.jComboBox1 = JComboBox::new();
        self.aboveJButton = JButton::new();
        self.jTextField1 = JTextField::new();
        self.jLabel2 = JLabel::new();
        self.jLabel3 = JLabel::new();
        self.jTextField2 = JTextField::new();
        self.updateJButton = JButton::new();
        self.floatPercentJTextField = JTextField::new();
        self.adjustFor16AboveJButton = JButton::new();
        self.adjustFor16BelowJButton = JButton::new();
        jcall!(self.jLabel1).setText(String::from("Percent:"));
        jcall!(self.jComboBox1).setModel(DefaultComboBoxModel::new(vec![String::from("Item 1"), String::from("Item 2"), String::from("Item 3"), String::from("Item 4")]));
        jcall!(self.aboveJButton).setText(String::from("Process"));
        { let java_receiver = self.aboveJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.aboveJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel2).setText(String::from("Width:"));
        jcall!(self.jLabel3).setText(String::from("Height:"));
        jcall!(self.updateJButton).setText(String::from("Update"));
        { let java_receiver = self.updateJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.updateJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.floatPercentJTextField).setText(String::from("-1.000000"));
        jcall!(self.floatPercentJTextField).setMinimumSize(Dimension::new(120, 22));
        jcall!(self.adjustFor16AboveJButton).setText(String::from("Adjust for 16 Above"));
        { let java_receiver = self.adjustFor16AboveJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.adjustFor16AboveJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.adjustFor16BelowJButton).setText(String::from("Adjust for 16 Below"));
        { let java_receiver = self.adjustFor16BelowJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.adjustFor16BelowJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.jLabel1.clone())).addComponent(self.jLabel2.clone())).addComponent(self.jLabel3.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), false)).addComponent(self.jComboBox1.clone(), 0, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).addComponent(self.jTextField1.clone())).addComponent(self.jTextField2.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).UNRELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.floatPercentJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.adjustFor16AboveJButton.clone())).addComponent(self.adjustFor16BelowJButton.clone()))).addContainerGap(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone()))).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.updateJButton.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.aboveJButton.clone())).addGap(0, 250, i16::MAX.clone())));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel1.clone())).addComponent(self.jComboBox1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.floatPercentJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jTextField1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.jLabel2.clone())).addComponent(self.adjustFor16AboveJButton.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel3.clone())).addComponent(self.jTextField2.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.adjustFor16BelowJButton.clone()))).addGap(18, 18, 18)).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.aboveJButton.clone())).addComponent(self.updateJButton.clone()))).addContainerGap(175, i16::MAX.clone())));
    }
    

    pub fn aboveJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.process();
    }
    

    pub fn updateJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.update();
    }
    

    pub fn adjustFor16AboveJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.updateFor16Above();
    }
    

    pub fn adjustFor16BelowJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.updateFor16Below();
    }
}
impl std::ops::Deref for ResizeImageJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ResizeImageJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



