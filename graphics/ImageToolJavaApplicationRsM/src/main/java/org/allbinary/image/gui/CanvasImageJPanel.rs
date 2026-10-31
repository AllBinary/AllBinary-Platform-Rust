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
// use crate::java::io::File::File;
// use crate::java::io::IOException::IOException;
// use crate::javax::imageio::ImageIO::ImageIO;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::media::image::CanvasExpandUtil::CanvasExpandUtil;
use crate::org::allbinary::media::image::CanvasTrimUtil::CanvasTrimUtil;
use crate::org::allbinary::media::image::ImageProcessedVisitor::ImageProcessedVisitor;
use crate::org::allbinary::media::image::ImageProcessorInput::ImageProcessorInput;
use crate::org::allbinary::media::image::ImageStrings::ImageStrings;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Thread::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CanvasImageJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub imageStrings: Option<Rc<RefCell<ImageStrings>>>,
    pub imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>,
    pub result: Option<Rc<RefCell<BufferedImage>>>,
    pub generateJButton: Option<Rc<RefCell<JButton>>>,
    pub heightReductionTextField: Option<Rc<RefCell<JTextField>>>,
    pub heightReductionTextField1: Option<Rc<RefCell<JTextField>>>,
    pub jLabel4: Option<Rc<RefCell<JLabel>>>,
    pub jLabel5: Option<Rc<RefCell<JLabel>>>,
    pub jLabel6: Option<Rc<RefCell<JLabel>>>,
    pub jLabel7: Option<Rc<RefCell<JLabel>>>,
    pub jLabel8: Option<Rc<RefCell<JLabel>>>,
    pub jPanel1: Option<Rc<RefCell<JPanel>>>,
    pub jPanel2: Option<Rc<RefCell<JPanel>>>,
    pub widthReductionTextField: Option<Rc<RefCell<JTextField>>>,
    pub widthReductionTextField1: Option<Rc<RefCell<JTextField>>>,
    pub writeOverOriginalJCheckBox: Option<Rc<RefCell<JCheckBox>>>,
}

impl ImageProcessedVisitor for CanvasImageJPanel {
    fn visit(&mut self, mut generatedBufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut name: String, mut index: i32) { Self::visit(self, generatedBufferedImage, name, index) }
}
impl crate::org::allbinary::media::image::ImageProcessorInputCompositeInterface::ImageProcessorInputCompositeInterface for CanvasImageJPanel {
    fn getImageProcessorInput(&mut self) -> Option<Rc<RefCell<ImageProcessorInput>>> { Self::getImageProcessorInput(self) }
    fn setImageProcessorInput(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) { Self::setImageProcessorInput(self, imageProcessorInput) }
}

impl CanvasImageJPanel {
    

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
            heightReductionTextField1: None,
            jLabel4: None,
            jLabel5: None,
            jLabel6: None,
            jLabel7: None,
            jLabel8: None,
            jPanel1: None,
            jPanel2: None,
            widthReductionTextField: None,
            widthReductionTextField1: None,
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
                    let mut canvasTrimUtil: Option<Rc<RefCell<CanvasTrimUtil>>> = CanvasTrimUtil::getInstance();
                    let mut canvasExpandUtil: Option<Rc<RefCell<CanvasExpandUtil>>> = CanvasExpandUtil::getInstance();
                    let mut leftReduction: i32 = (jcall!(self.widthReductionTextField1).getText()).to_string();
                    let mut topReduction: i32 = (jcall!(self.heightReductionTextField1).getText()).to_string();
                    let mut widthReduction: i32 = (jcall!(self.widthReductionTextField).getText()).to_string();
                    let mut heightReduction: i32 = (jcall!(self.heightReductionTextField).getText()).to_string();
                    if leftReduction <= 0 || topReduction <= 0 || widthReduction <= 0 || heightReduction <= 0 {
                        jcall!(canvasTrimUtil).process(self.getImageProcessorInput(), -leftReduction, -topReduction, -widthReduction, -heightReduction, self);
                    } else {
                        jcall!(canvasExpandUtil).process(self.getImageProcessorInput(), leftReduction.clone(), topReduction.clone(), self);
                    }
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
    

    pub fn initComponents(&mut self) {
        self.writeOverOriginalJCheckBox = JCheckBox::new();
        self.jPanel1 = JPanel::new() /* Java anonymous class body: implement it as a named type
            

            pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
                if self.result.is_some() {
                    jcall!(graphics).drawImage(self.result.clone(), 0, 0, jcall!(self.result).getWidth(None), jcall!(self.result).getHeight(None), None);
                }
            }
*/;
        self.generateJButton = JButton::new();
        self.jLabel4 = JLabel::new();
        self.jPanel2 = JPanel::new();
        self.jLabel5 = JLabel::new();
        self.widthReductionTextField = JTextField::new();
        self.jLabel6 = JLabel::new();
        self.heightReductionTextField = JTextField::new();
        self.widthReductionTextField1 = JTextField::new();
        self.jLabel7 = JLabel::new();
        self.jLabel8 = JLabel::new();
        self.heightReductionTextField1 = JTextField::new();
        jcall!(self.writeOverOriginalJCheckBox).setSelected(true);
        jcall!(self.writeOverOriginalJCheckBox).setText(String::from("Write Over Original"));
        { let java_receiver = self.writeOverOriginalJCheckBox.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.writeOverOriginalJCheckBoxActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        let mut jPanel1Layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.jPanel1.clone());
        jcall!(self.jPanel1).setLayout(jPanel1Layout.clone());
        jcall!(jPanel1Layout).setHorizontalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 0, i16::MAX.clone()));
        jcall!(jPanel1Layout).setVerticalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 196, i16::MAX.clone()));
        jcall!(self.generateJButton).setText(String::from("Generate"));
        { let java_receiver = self.generateJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.generateJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel4).setText(String::from("Results:"));
        jcall!(self.jLabel5).setText(String::from("Right:"));
        { let java_receiver = self.widthReductionTextField.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.widthReductionTextField.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.widthReductionTextFieldActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel6).setText(String::from("Bottom"));
        { let java_receiver = self.heightReductionTextField.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.widthReductionTextField1.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.widthReductionTextField1.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.widthReductionTextField1ActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel7).setText(String::from("Top"));
        jcall!(self.jLabel8).setText(String::from("Left:"));
        { let java_receiver = self.heightReductionTextField1.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        let mut jPanel2Layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.jPanel2.clone());
        jcall!(self.jPanel2).setLayout(jPanel2Layout.clone());
        jcall!(jPanel2Layout).setHorizontalGroup(jcall!(jcall!(jPanel2Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jPanel2Layout).createSequentialGroup()).addContainerGap()).addComponent(self.jLabel8.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.widthReductionTextField1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 41, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jLabel5.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.widthReductionTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 41, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jLabel7.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.heightReductionTextField1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 46, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jLabel6.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.heightReductionTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 46, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addContainerGap(140, i16::MAX.clone())));
        jcall!(jPanel2Layout).setVerticalGroup(jcall!(jcall!(jPanel2Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(jcall!(jPanel2Layout).createSequentialGroup()).addGap(0, 16, i16::MAX.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jPanel2Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jPanel2Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel6.clone())).addComponent(self.heightReductionTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jPanel2Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel8.clone())).addComponent(self.widthReductionTextField1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 22, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.jLabel7.clone())).addComponent(self.heightReductionTextField1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addGroup(jcall!(jcall!(jcall!(jPanel2Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel5.clone())).addComponent(self.widthReductionTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 22, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())))));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.jPanel2.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).addGap(78, 78, 78))).addGroup(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).addComponent(self.jPanel1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone()))).addGroup(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.jLabel4.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.writeOverOriginalJCheckBox.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.generateJButton.clone())))).addGap(0, 0, i16::MAX.clone())))).addContainerGap()));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.jPanel2.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.generateJButton.clone())).addComponent(self.writeOverOriginalJCheckBox.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jLabel4.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jPanel1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())));
    }
    

    pub fn writeOverOriginalJCheckBoxActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn generateJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.process();
    }
    

    pub fn widthReductionTextFieldActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn widthReductionTextField1ActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn getImageProcessorInput(&mut self) -> Option<Rc<RefCell<ImageProcessorInput>>> {
        return self.imageProcessorInput.clone();
    }
    

    pub fn setImageProcessorInput(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) {
        self.imageProcessorInput = imageProcessorInput.clone();
    }
    

    // throws IOException
    pub fn visit(&mut self, mut generatedBufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut name: String, mut index: i32) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).VISIT.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).VISIT.clone());
        self.result = generatedBufferedImage.clone();
        jcall!(self.getParent()).repaint();
        let mut file: Option<Rc<RefCell<File>>> = jcall!(self.imageProcessorInput).getFiles()[(index) as usize].clone();
        if !jcall!(self.writeOverOriginalJCheckBox).isSelected() {
            let mut filePath: String = jcall!(file).getAbsolutePath();
            let mut extensionIndex: i32 = filePath.find(&jcall!(self.imageStrings).PNG_EXTENSION.clone()).map(|index| index as i32).unwrap_or(-1);
            filePath = jcall!(StringMaker::new()).append(filePath[(0) as usize..(extensionIndex) as usize].to_string()).append(jcall!(self.commonSeps).UNDERSCORE.clone()).append(name.clone()).append(jcall!(self.imageStrings).PNG_EXTENSION.clone()).toString();
            file = File::new(filePath.clone());
        }
        let mut isWritten: bool = ImageIO::write((self.result).clone() /* as RenderedImage */, jcall!(self.imageStrings).PNG.clone(), file.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("File: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(file.clone()))).append(String::from(" Wrote: ")).appendboolean(isWritten.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
}
impl std::ops::Deref for CanvasImageJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CanvasImageJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



