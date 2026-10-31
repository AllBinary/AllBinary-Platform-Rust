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
use crate::org::allbinary::media::image::BufferedImageUtil2::BufferedImageUtil2;
use crate::org::allbinary::media::image::ImageProcessorInput::ImageProcessorInput;
use crate::org::allbinary::media::image::ImageProcessorInputCompositeInterface::ImageProcessorInputCompositeInterface;
use crate::org::allbinary::media::image::ImageStrings::ImageStrings;
use crate::org::allbinary::media::image::MirrorImageUtil::MirrorImageUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Thread::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MirrorSpriteImageJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub imageStrings: Option<Rc<RefCell<ImageStrings>>>,
    pub imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>,
    pub result: Option<Rc<RefCell<BufferedImage>>>,
    pub generateJButton: Option<Rc<RefCell<JButton>>>,
    pub horizontalJCheckBox: Option<Rc<RefCell<JCheckBox>>>,
    pub imageHeightIsFrameSizeJCheckBox: Option<Rc<RefCell<JCheckBox>>>,
    pub jLabel2: Option<Rc<RefCell<JLabel>>>,
    pub jLayeredPane1: Option<Rc<RefCell<JLayeredPane>>>,
    pub jPanel1: Option<Rc<RefCell<JPanel>>>,
    pub jPanel2: Option<Rc<RefCell<JPanel>>>,
    pub jPanel3: Option<Rc<RefCell<JPanel>>>,
    pub verticleJCheckBox: Option<Rc<RefCell<JCheckBox>>>,
    pub writeOverOriginalJCheckBox: Option<Rc<RefCell<JCheckBox>>>,
}

impl ImageProcessorInputCompositeInterface for MirrorSpriteImageJPanel {
    fn getImageProcessorInput(&mut self) -> Option<Rc<RefCell<ImageProcessorInput>>> { Self::getImageProcessorInput(self) }
    fn setImageProcessorInput(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) { Self::setImageProcessorInput(self, imageProcessorInput) }
}

impl MirrorSpriteImageJPanel {
    

    pub fn new_struct(mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            imageStrings: None,
            imageProcessorInput: None,
            result: None,
            generateJButton: None,
            horizontalJCheckBox: None,
            imageHeightIsFrameSizeJCheckBox: None,
            jLabel2: None,
            jLayeredPane1: None,
            jPanel1: None,
            jPanel2: None,
            jPanel3: None,
            verticleJCheckBox: None,
            writeOverOriginalJCheckBox: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.imageStrings = ImageStrings::getInstance();
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(String::from("Starting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
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
                        generatedBufferedImageArray = jcall!(MirrorImageUtil::getInstance()).getImages(bufferedImageArray[(index) as usize].clone(), jcall!(self.verticleJCheckBox).isSelected(), jcall!(self.horizontalJCheckBox).isSelected());
                        self.result = jcall!(BufferedImageUtil2::getInstance()).createSpriteImage(generatedBufferedImageArray.clone());
                        jcall!(self.getParent()).repaint();
                        let mut file: Option<Rc<RefCell<File>>> = jcall!(imageProcessorInput).getFiles()[(index) as usize].clone();
                        if !jcall!(self.writeOverOriginalJCheckBox).isSelected() {
                            let mut filePath: String = jcall!(file).getAbsolutePath();
                            let mut extensionIndex: i32 = filePath.find(&jcall!(self.imageStrings).PNG_EXTENSION.clone()).map(|index| index as i32).unwrap_or(-1);
                            filePath = format!("{}{}", format!("{}{}", filePath[(0) as usize..(extensionIndex) as usize].to_string(), String::from("_mirror")), jcall!(self.imageStrings).PNG_EXTENSION);
                            jcall!(logUtil).putF(format!("{}{}", String::from("Renamed File: "), filePath), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
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
        self.jLayeredPane1 = JLayeredPane::new();
        self.jPanel1 = JPanel::new() /* Java anonymous class body: implement it as a named type
            

            pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
                if self.result.is_some() {
                    jcall!(graphics).drawImage(self.result.clone(), 0, 0, jcall!(self.result).getWidth(None), jcall!(self.result).getHeight(None), None);
                }
            }
*/;
        self.jPanel2 = JPanel::new();
        self.imageHeightIsFrameSizeJCheckBox = JCheckBox::new();
        self.horizontalJCheckBox = JCheckBox::new();
        self.verticleJCheckBox = JCheckBox::new();
        self.writeOverOriginalJCheckBox = JCheckBox::new();
        self.generateJButton = JButton::new();
        self.jPanel3 = JPanel::new();
        self.jLabel2 = JLabel::new();
        let mut jPanel1Layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.jPanel1.clone());
        jcall!(self.jPanel1).setLayout(jPanel1Layout.clone());
        jcall!(jPanel1Layout).setHorizontalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(0, 507, i16::MAX.clone()));
        jcall!(jPanel1Layout).setVerticalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(0, 230, i16::MAX.clone()));
        jcall!(self.imageHeightIsFrameSizeJCheckBox).setSelected(true);
        jcall!(self.imageHeightIsFrameSizeJCheckBox).setText(String::from("Image Height Is Frame Size"));
        jcall!(self.imageHeightIsFrameSizeJCheckBox).setEnabled(false);
        { let java_receiver = self.imageHeightIsFrameSizeJCheckBox.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.imageHeightIsFrameSizeJCheckBoxActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.horizontalJCheckBox).setSelected(true);
        jcall!(self.horizontalJCheckBox).setText(String::from("Horizontal"));
        jcall!(self.horizontalJCheckBox).setEnabled(false);
        jcall!(self.verticleJCheckBox).setText(String::from("Verticle"));
        jcall!(self.verticleJCheckBox).setEnabled(false);
        jcall!(self.writeOverOriginalJCheckBox).setText(String::from("Write Over Original"));
        jcall!(self.generateJButton).setText(String::from("Generate"));
        { let java_receiver = self.generateJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.generateJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        let mut jPanel2Layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.jPanel2.clone());
        jcall!(self.jPanel2).setLayout(jPanel2Layout.clone());
        jcall!(jPanel2Layout).setHorizontalGroup(jcall!(jcall!(jPanel2Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jPanel2Layout).createSequentialGroup()).addContainerGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).add(self.verticleJCheckBox.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).UNRELATED.clone())).add(self.horizontalJCheckBox.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(self.imageHeightIsFrameSizeJCheckBox.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(self.writeOverOriginalJCheckBox.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(self.generateJButton.clone())));
        jcall!(jPanel2Layout).setVerticalGroup(jcall!(jcall!(jPanel2Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(jPanel2Layout).createSequentialGroup()).add(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jPanel2Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).BASELINE.clone())).add(self.imageHeightIsFrameSizeJCheckBox.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).add(self.horizontalJCheckBox.clone())).add(self.verticleJCheckBox.clone())).add(self.writeOverOriginalJCheckBox.clone())).add(self.generateJButton.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), 25, jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone()))).addContainerGap()));
        jcall!(self.jLabel2).setText(String::from("Results:"));
        let mut jPanel3Layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.jPanel3.clone());
        jcall!(self.jPanel3).setLayout(jPanel3Layout.clone());
        jcall!(jPanel3Layout).setHorizontalGroup(jcall!(jcall!(jPanel3Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(jPanel3Layout).createSequentialGroup()).add(self.jLabel2.clone())).addContainerGap(448, i16::MAX.clone())));
        jcall!(jPanel3Layout).setVerticalGroup(jcall!(jcall!(jPanel3Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(self.jLabel2.clone()));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).TRAILING.clone(), jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).add(self.jPanel3.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).addContainerGap())).add(self.jPanel1.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).add(jcall!(jcall!(jcall!(layout).createSequentialGroup()).add(self.jPanel2.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).addContainerGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).add(self.jPanel2.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), 25, jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(self.jPanel3.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(self.jPanel1.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())));
    }
    

    pub fn generateJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.process();
    }
    

    pub fn imageHeightIsFrameSizeJCheckBoxActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn getImageProcessorInput(&mut self) -> Option<Rc<RefCell<ImageProcessorInput>>> {
        return self.imageProcessorInput.clone();
    }
    

    pub fn setImageProcessorInput(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) {
        self.imageProcessorInput = imageProcessorInput.clone();
    }
}
impl std::ops::Deref for MirrorSpriteImageJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MirrorSpriteImageJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



