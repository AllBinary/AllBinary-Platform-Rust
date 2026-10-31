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
// package org.allbinary.image.gui
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::java::awt::image::RenderedImage::RenderedImage;
// use crate::java::io::File::File;
// use crate::javax::imageio::ImageIO::ImageIO;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::image::ImageJ2SERotationUtil::ImageJ2SERotationUtil;
use crate::org::allbinary::media::image::ImageProcessorInput::ImageProcessorInput;
use crate::org::allbinary::media::image::ImageProcessorInputCompositeInterface::ImageProcessorInputCompositeInterface;
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

pub struct RotationSpriteImageJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub imageStrings: Option<Rc<RefCell<ImageStrings>>>,
    pub imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>,
    pub result: Option<Rc<RefCell<BufferedImage>>>,
    pub generateJButton: Option<Rc<RefCell<JButton>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
    pub jLabel2: Option<Rc<RefCell<JLabel>>>,
    pub jLabel3: Option<Rc<RefCell<JLabel>>>,
    pub jPanel1: Option<Rc<RefCell<JPanel>>>,
    pub jPanel2: Option<Rc<RefCell<JPanel>>>,
    pub jPanel3: Option<Rc<RefCell<JPanel>>>,
    pub totalAngleJComboBox: Option<Rc<RefCell<JComboBox>>>,
    pub totalFramesJComboBox: Option<Rc<RefCell<JComboBox>>>,
    pub writeOverOriginalJCheckBox: Option<Rc<RefCell<JCheckBox>>>,
}

impl ImageProcessorInputCompositeInterface for RotationSpriteImageJPanel {
    fn getImageProcessorInput(&mut self) -> Option<Rc<RefCell<ImageProcessorInput>>> { Self::getImageProcessorInput(self) }
    fn setImageProcessorInput(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) { Self::setImageProcessorInput(self, imageProcessorInput) }
}

impl RotationSpriteImageJPanel {
    

    pub fn new_struct(mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            imageStrings: None,
            imageProcessorInput: None,
            result: None,
            generateJButton: None,
            jLabel1: None,
            jLabel2: None,
            jLabel3: None,
            jPanel1: None,
            jPanel2: None,
            jPanel3: None,
            totalAngleJComboBox: None,
            totalFramesJComboBox: None,
            writeOverOriginalJCheckBox: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
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
                    let mut generatedBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = Vec::new();
                    let mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>> = self.getImageProcessorInput();
                    let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(imageProcessorInput).getBufferedImageArray();
                    for index in (0..(bufferedImageArray.len() as i32)) {
                        let mut totalFrames: Option<i32> = ((jcall!(self.totalFramesJComboBox).getSelectedItem()).clone() /* as String * /).to_string();
                        let mut totalAngle: Option<i32> = ((jcall!(self.totalAngleJComboBox).getSelectedItem()).clone() /* as String * /).to_string();
                        generatedBufferedImageArray = jcall!(ImageJ2SERotationUtil::getInstance()).getRotatedImages(bufferedImageArray[(index) as usize].clone(), totalFrames.clone(), totalAngle.clone());
                        self.result = jcall!(ImageJ2SERotationUtil::getInstance()).createSpriteImage(generatedBufferedImageArray.clone());
                        jcall!(self.getParent()).repaint();
                        let mut file: Option<Rc<RefCell<File>>> = jcall!(imageProcessorInput).getFiles()[(index) as usize].clone();
                        if !jcall!(self.writeOverOriginalJCheckBox).isSelected() {
                            let mut filePath: String = jcall!(file).getAbsolutePath();
                            let mut extensionIndex: i32 = filePath.find(&jcall!(self.imageStrings).PNG_EXTENSION.clone()).map(|index| index as i32).unwrap_or(-1);
                            filePath = jcall!(StringMaker::new()).append(filePath[(0) as usize..(extensionIndex) as usize].to_string()).append(jcall!(CommonSeps::getInstance()).UNDERSCORE.clone()).append(String::from("sprite")).append(jcall!(self.imageStrings).PNG_EXTENSION.clone()).toString();
                            jcall!(logUtil).putF(format!("{}{}", String::from("New File Path: "), filePath), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
                            file = File::new(filePath.clone());
                        }
                        let mut isWritten: bool = ImageIO::write((self.result).clone() /* as RenderedImage * /, jcall!(self.imageStrings).PNG.clone(), file.clone());
                        jcall!(logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("File: "), file), String::from(" Wrote: ")), isWritten), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
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
        self.jPanel1 = JPanel::new() /* Java anonymous class body: implement it as a named type
            

            pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
                if self.result.is_some() {
                    jcall!(graphics).drawImage(self.result.clone(), 0, 0, jcall!(self.result).getWidth(None), jcall!(self.result).getHeight(None), None);
                }
            }
*/;
        self.jPanel2 = JPanel::new();
        self.totalFramesJComboBox = JComboBox::new();
        self.jLabel1 = JLabel::new();
        self.generateJButton = JButton::new();
        self.totalAngleJComboBox = JComboBox::new();
        self.jLabel3 = JLabel::new();
        self.writeOverOriginalJCheckBox = JCheckBox::new();
        self.jPanel3 = JPanel::new();
        self.jLabel2 = JLabel::new();
        let mut jPanel1Layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.jPanel1.clone());
        jcall!(self.jPanel1).setLayout(jPanel1Layout.clone());
        jcall!(jPanel1Layout).setHorizontalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(0, 436, i16::MAX.clone()));
        jcall!(jPanel1Layout).setVerticalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(0, 227, i16::MAX.clone()));
        { let java_receiver = self.totalFramesJComboBox.clone(); let java_arguments = (DefaultComboBoxModel::new(vec![jcall!(CommonPhoneStrings::getInstance()).FOUR.clone(), jcall!(CommonPhoneStrings::getInstance()).NINE.clone(), String::from("18"), String::from("20"), String::from("36")]),); jcall!(java_receiver).setModel(java_arguments.0); };
        jcall!(self.jLabel1).setText(String::from("Total Frames:"));
        jcall!(self.generateJButton).setText(String::from("Generate"));
        { let java_receiver = self.generateJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.generateJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.totalAngleJComboBox).setModel(DefaultComboBoxModel::new(vec![String::from("360"), String::from("90")]));
        { let java_receiver = self.totalAngleJComboBox.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.totalAngleJComboBoxActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel3).setText(String::from("Total Angle:"));
        jcall!(self.writeOverOriginalJCheckBox).setText(String::from("Write Over Original"));
        let mut jPanel2Layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.jPanel2.clone());
        jcall!(self.jPanel2).setLayout(jPanel2Layout.clone());
        jcall!(jPanel2Layout).setHorizontalGroup(jcall!(jcall!(jPanel2Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jPanel2Layout).createSequentialGroup()).add(8, 8, 8)).add(self.jLabel1.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(self.totalFramesJComboBox.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(self.jLabel3.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(self.totalAngleJComboBox.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(self.writeOverOriginalJCheckBox.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(self.generateJButton.clone())).addContainerGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())));
        jcall!(jPanel2Layout).setVerticalGroup(jcall!(jcall!(jPanel2Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jPanel2Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).BASELINE.clone())).add(self.jLabel1.clone())).add(self.totalFramesJComboBox.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).add(self.generateJButton.clone())).add(self.totalAngleJComboBox.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).add(self.jLabel3.clone())).add(self.writeOverOriginalJCheckBox.clone())));
        jcall!(self.jLabel2).setText(String::from("Results:"));
        let mut jPanel3Layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.jPanel3.clone());
        jcall!(self.jPanel3).setLayout(jPanel3Layout.clone());
        jcall!(jPanel3Layout).setHorizontalGroup(jcall!(jcall!(jPanel3Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(jPanel3Layout).createSequentialGroup()).add(self.jLabel2.clone())).addContainerGap(377, i16::MAX.clone())));
        jcall!(jPanel3Layout).setVerticalGroup(jcall!(jcall!(jPanel3Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(self.jLabel2.clone()));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).TRAILING.clone(), self.jPanel2.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).add(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).add(self.jPanel3.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).addContainerGap())).add(self.jPanel1.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone()));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).add(self.jPanel2.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).UNRELATED.clone())).add(self.jPanel3.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(self.jPanel1.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())));
    }
    

    pub fn generateJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.process();
    }
    

    pub fn totalAngleJComboBoxActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn getImageProcessorInput(&mut self) -> Option<Rc<RefCell<ImageProcessorInput>>> {
        return self.imageProcessorInput.clone();
    }
    

    pub fn setImageProcessorInput(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) {
        self.imageProcessorInput = imageProcessorInput.clone();
    }
}
impl std::ops::Deref for RotationSpriteImageJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RotationSpriteImageJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



