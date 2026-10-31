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
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::image::ImagePersistanceUtil::ImagePersistanceUtil;
use crate::org::allbinary::media::image::ImageProcessorInput::ImageProcessorInput;
use crate::org::allbinary::media::image::ImageProcessorInputCompositeInterface::ImageProcessorInputCompositeInterface;
use crate::org::allbinary::media::image::ImageUtil::ImageUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::java::lang::Thread::*;
use super::JPanel::JPanel;
use super::swing::swing;
use super::javax::javax;
use super::JButton::JButton;
use super::ActionListener::ActionListener;
use super::event::event;
use super::awt::awt;
use super::java::java;
use super::ActionEvent::ActionEvent;
use super::GroupLayout::GroupLayout;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CreateIconImagesJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>,
    pub aboveJButton: Option<Rc<RefCell<JButton>>>,
}

impl ImageProcessorInputCompositeInterface for CreateIconImagesJPanel {
    fn getImageProcessorInput(&mut self) -> Option<Rc<RefCell<ImageProcessorInput>>> { Self::getImageProcessorInput(self) }
    fn setImageProcessorInput(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) { Self::setImageProcessorInput(self, imageProcessorInput) }
}

impl CreateIconImagesJPanel {
    

    pub fn new_struct(mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            imageProcessorInput: None,
            aboveJButton: None,
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
                    let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    let mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>> = self.getImageProcessorInput();
                    let mut files: Vec<Option<Rc<RefCell<File>>>> = jcall!(imageProcessorInput).getFiles();
                    let mut iconWidthList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
                    let mut iconHeightList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
                    jcall!(iconWidthList).add(crate::java_into_object(864));
                    jcall!(iconHeightList).add(crate::java_into_object(864));
                    let mut ANDROID: String = String::from("android-icon-");
                    let mut IOS: String = String::from("ios-icon-");
                    let mut size: i32 = jcall!(iconWidthList).size();
                    let mut width: i32 = 0;
                    let mut height: i32 = 0;
                    for index2 in (0..size) {
                        width = ((({ let java_object = jcall!(iconWidthList).get(index2.clone()); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) })).unwrap() as i32);
                        height = ((({ let java_object = jcall!(iconHeightList).get(index2.clone()); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) })).unwrap() as i32);
                        let mut generatedBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(imageUtil).createBufferedImage3(jcall!(imageProcessorInput).getBufferedImageArray(), width.clone(), height.clone(), true);
                        let mut imagePersistanceUtil: Option<Rc<RefCell<ImagePersistanceUtil>>> = ImagePersistanceUtil::getInstance();
                        for index in (0..(generatedBufferedImageArray.len() as i32)) {
                            let mut filePath: String = jcall!(files[(index) as usize]).getAbsolutePath();
                            let mut endIndex: i32 = filePath.rfind('\\').map(|index| index as i32).unwrap_or(-1);
                            let mut path: String = filePath[(0) as usize..(endIndex + 1) as usize].to_string();
                            jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                            jcall!(stringMaker).append(path.clone()).append(ANDROID.clone()).appendint(width.clone()).append(jcall!(CanvasStrings::getInstance())._PNG.clone());
                            let mut newFilePath: String = jcall!(stringMaker).toString();
                            jcall!(imagePersistanceUtil).saveWithBatik(AbFile::createAbFile(newFilePath.clone()), generatedBufferedImageArray[(index) as usize].clone());
                            jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                            jcall!(stringMaker).append(path.clone()).append(IOS.clone()).appendint(width.clone()).append(jcall!(CanvasStrings::getInstance())._PNG.clone());
                            newFilePath = jcall!(stringMaker).toString();
                            jcall!(imagePersistanceUtil).saveWithBatik(AbFile::createAbFile(newFilePath.clone()), generatedBufferedImageArray[(index) as usize].clone());
                        }
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
        self.aboveJButton = JButton::new();
        jcall!(self.aboveJButton).setText(String::from("Process"));
        { let java_receiver = self.aboveJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.aboveJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.aboveJButton.clone())).addGap(0, 335, i16::MAX.clone())));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).addComponent(self.aboveJButton.clone())).addContainerGap(286, i16::MAX.clone())));
    }
    

    pub fn aboveJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.process();
    }
}
impl std::ops::Deref for CreateIconImagesJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CreateIconImagesJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



