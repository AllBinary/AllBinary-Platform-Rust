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
// package org.allbinary.media.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::eclipse::swt::SWT::SWT;
use crate::org::eclipse::swt::widgets::Canvas::Canvas;
use crate::org::eclipse::swt::widgets::Composite::Composite;
use crate::org::eclipse::swt::graphics::GC::GC;
use crate::org::eclipse::swt::graphics::ImageData::ImageData;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::eclipse::swt::events::PaintEvent::PaintEvent;
use crate::org::eclipse::swt::events::PaintListener::PaintListener;
use crate::org::eclipse::swt::graphics::Transform::Transform;
use crate::org::eclipse::swt::widgets::Display::Display;
use crate::org::microemu::app::BareMain::BareMain;
use crate::java::lang::Exception::*;
use super::Image::Image;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub IIOIMAGE_POOL_NAME: String,
    pub CREATE_BUFFERED_IMAGE: String,
}

thread_local! { pub static ImageUtil_instance: Option<Rc<RefCell<ImageUtil>>> = ImageUtil::new(); }

impl ImageUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageUtil>>> {
        return ImageUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            commonSeps: None,
            IIOIMAGE_POOL_NAME: String::new(),
            CREATE_BUFFERED_IMAGE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.IIOIMAGE_POOL_NAME = String::from("IIOIMAGE_POOL_NAME");
        this.CREATE_BUFFERED_IMAGE = String::from("createBufferedImage");
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(this.logUtil).putF({ let java_field = jcall!(this.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(this.logUtil).put({ let java_field = jcall!(this.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn create(&mut self, mut width: i32, mut height: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut display: Option<Rc<RefCell<Display>>> = jcall!(unsafe { crate::org::microemu::app::BareMain::BareMain_shell.clone() }).getDisplay();
        return Image::new3(display.clone(), width.clone(), height.clone()).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>);
    }
    

    // throws Exception
    pub fn createBufferedImage(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>, mut percent: i32, mut scale: bool) -> Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>> {
        let mut size: i32 = (bufferedImageArray.len() as i32).clone();
        let mut scaledBufferedImageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>> = vec![None; (size) as usize];
        let mut bufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
        let mut imageData: Option<Rc<RefCell<ImageData>>> = None;
        for index in (0..size) {
            bufferedImage = { let java_value = (bufferedImageArray[(index) as usize]).clone(); java_value }.clone();
            imageData = { let java_value = (jcall!(bufferedImage).as_Image_mut().getImageData()).clone(); java_value };
            let mut newWidth: i32 = ((({ let java_field = jcall!(imageData).width.clone(); java_field } * percent / 100)) as i32);
            let mut newHeight: i32 = ((({ let java_field = jcall!(imageData).height.clone(); java_field } * percent / 100)) as i32);
            scaledBufferedImageArray[(index) as usize] = self.createBufferedImage5(bufferedImage.clone(), newWidth.clone(), newHeight.clone(), scale.clone());
        }
        return scaledBufferedImageArray.clone();
    }
    

    // throws Exception
    pub fn createBufferedImage2(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>, mut percent: f32, mut scale: bool) -> Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>> {
        let mut size: i32 = (bufferedImageArray.len() as i32).clone();
        let mut scaledBufferedImageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>> = vec![None; (size) as usize];
        let mut bufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
        let mut imageData: Option<Rc<RefCell<ImageData>>> = None;
        for index in (0..size) {
            bufferedImage = { let java_value = (bufferedImageArray[(index) as usize]).clone(); java_value }.clone();
            imageData = { let java_value = (jcall!(bufferedImage).as_Image_mut().getImageData()).clone(); java_value };
            let mut newWidth: i32 = ((({ let java_field = jcall!(imageData).width.clone(); java_field } * percent)) as i32);
            let mut newHeight: i32 = ((({ let java_field = jcall!(imageData).height.clone(); java_field } * percent)) as i32);
            scaledBufferedImageArray[(index) as usize] = self.createBufferedImage5(bufferedImage.clone(), newWidth.clone(), newHeight.clone(), scale.clone());
        }
        return scaledBufferedImageArray.clone();
    }
    

    // throws Exception
    pub fn createBufferedImage3(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>, mut width: i32, mut height: i32, mut scale: bool) -> Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>> {
        let mut size: i32 = (bufferedImageArray.len() as i32).clone();
        let mut scaledBufferedImageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>> = vec![None; (size) as usize];
        for index in (0..size) {
            scaledBufferedImageArray[(index) as usize] = self.createBufferedImage5(bufferedImageArray[(index) as usize].clone(), width.clone(), height.clone(), scale.clone());
        }
        return scaledBufferedImageArray.clone();
    }
    

    // throws Exception
    pub fn createBufferedImage4(&mut self, mut bufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut newWidth: i32, mut newHeight: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        return self.createBufferedImage5(bufferedImage.clone(), newWidth.clone(), newHeight.clone(), true);
    }
    

    // throws Exception
    pub fn createBufferedImage5(&mut self, mut bufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut newWidth: i32, mut newHeight: i32, mut scale: bool) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        return self.createBufferedImage6(bufferedImage.clone(), newWidth.clone(), newHeight.clone(), scale.clone(), false);
    }
    

    // throws Exception
    pub fn createBufferedImage6(&mut self, mut bufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut newWidth: i32, mut newHeight: i32, mut scale: bool, mut allowTranslate: bool) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut imageData: Option<Rc<RefCell<ImageData>>> = jcall!(bufferedImage).as_Image_mut().getImageData();
        let mut width: f32 = { let java_field = jcall!(imageData).width.clone(); java_field }.clone();
        let mut height: f32 = { let java_field = jcall!(imageData).height.clone(); java_field }.clone();
        let mut d_newWidth: f32 = ((newWidth) as f32).clone();
        let mut d_newHeight: f32 = ((newHeight) as f32).clone();
        let mut widthRatio: f32 = d_newWidth / width;
        let mut heightRatio: f32 = d_newHeight / height;
        let mut ratioX: f32 = 1.0_f32;
        let mut ratioY: f32 = 1.0_f32;
        if scale {
            ratioX = widthRatio.clone();
            ratioY = heightRatio.clone();
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).appendfloat(width.clone()).append({ let java_field = jcall!(self.commonSeps).FORWARD_SLASH.clone(); java_field }.clone()).appendfloat(height.clone()).append({ let java_field = jcall!(self.commonSeps).COLON.clone(); java_field }.clone()).appendint(newWidth.clone()).append({ let java_field = jcall!(self.commonSeps).FORWARD_SLASH.clone(); java_field }.clone()).appendint(newHeight.clone()).append({ let java_field = jcall!(self.commonSeps).COLON.clone(); java_field }.clone()).appendfloat(widthRatio.clone()).append({ let java_field = jcall!(self.commonSeps).FORWARD_SLASH.clone(); java_field }.clone()).appendfloat(heightRatio.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.CREATE_BUFFERED_IMAGE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut dx: f32 = 0.0_f32;
        let mut dy: f32 = 0.0_f32;
        if !scale && allowTranslate {
            dx = (((newWidth) as f32) - width) / 2.0_f32;
            dy = (((newHeight) as f32) - height) / 2.0_f32;
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Translate dx: ")).appendfloat(dx.clone()).append(String::from(" dy: ")).appendfloat(dy.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.CREATE_BUFFERED_IMAGE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut newBufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = Image::new3(jcall!(unsafe { crate::org::microemu::app::BareMain::BareMain_shell.clone() }).getDisplay(), newWidth.clone(), newHeight.clone()).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>);
        let mut gc: Option<Rc<RefCell<GC>>> = GC::new(newBufferedImage.clone());
        let mut affineTransform: Option<Rc<RefCell<Transform>>> = Transform::new(jcall!(gc).getDevice());
        jcall!(affineTransform).scale(ratioX.clone(), ratioY.clone());
        jcall!(affineTransform).translate(dx.clone(), dy.clone());
        jcall!(gc).setTransform(affineTransform.clone());
        jcall!(gc).setAntialias(unsafe { crate::org::eclipse::swt::SWT::SWT_ON.clone() }.clone());
        jcall!(gc).setInterpolation(unsafe { crate::org::eclipse::swt::SWT::SWT_HIGH.clone() }.clone());
        jcall!(gc).drawImage(bufferedImage.clone(), 0, 0, { let java_field = jcall!(jcall!(bufferedImage).as_Image_mut().getBounds()).width.clone(); java_field }.clone(), { let java_field = jcall!(jcall!(bufferedImage).as_Image_mut().getBounds()).height.clone(); java_field }.clone(), 0, 0, newWidth.clone(), newHeight.clone());
        jcall!(gc).dispose();
        return newBufferedImage.clone();
    }
    

    pub fn drawTransformedImage(&mut self, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut parent: Option<Rc<RefCell<Composite>>>, mut dx: i32, mut dy: i32, mut width: i32, mut height: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>> {
        let mut imageCanvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>> = Canvas::new(parent.clone(), unsafe { crate::org::eclipse::swt::SWT::SWT_NONE.clone() }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>);
        jcall!(imageCanvas).as_Canvas_mut().addPaintListener(PaintListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn paintControl(&mut self, mut event: Option<Rc<RefCell<PaintEvent>>>) {
                let mut gc: Option<Rc<RefCell<GC>>> = { let java_field = jcall!(event).gc.clone(); java_field }.clone();
                let mut affineTransform: Option<Rc<RefCell<Transform>>> = Transform::new(jcall!(gc).getDevice());
                jcall!(affineTransform).translate(dx.clone(), dy.clone());
                jcall!(gc).setAntialias(unsafe { crate::org::eclipse::swt::SWT::SWT_ON.clone() }.clone());
                jcall!(gc).setInterpolation(unsafe { crate::org::eclipse::swt::SWT::SWT_HIGH.clone() }.clone());
                jcall!(gc).drawImage(image.clone(), 0, 0, { let java_field = jcall!(jcall!(image).as_Image_mut().getBounds()).width.clone(); java_field }.clone(), { let java_field = jcall!(jcall!(image).as_Image_mut().getBounds()).height.clone(); java_field }.clone(), 0, 0, width.clone(), height.clone());
            }
*/);
        return imageCanvas.clone();
    }
    

    pub fn toString(&mut self, mut bufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>) -> String {
        let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut imageData: Option<Rc<RefCell<ImageData>>> = jcall!(bufferedImage).as_Image_mut().getImageData();
        jcall!(stringBuffer).append(String::from(" org.eclipse.swt.graphics.Image: "));
        jcall!(stringBuffer).append({ let java_field = jcall!(commonLabels).WIDTH_LABEL.clone(); java_field }.clone());
        jcall!(stringBuffer).appendint((({ let java_field = jcall!(imageData).width.clone(); java_field }) as i32).clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(commonLabels).HEIGHT_LABEL.clone(); java_field }.clone());
        jcall!(stringBuffer).appendint((({ let java_field = jcall!(imageData).height.clone(); java_field }) as i32).clone());
        jcall!(stringBuffer).append(String::from(" Type: "));
        jcall!(stringBuffer).appendint((({ let java_field = jcall!(imageData).r#type.clone(); java_field }) as i32).clone());
        return jcall!(stringBuffer).toString();
    }
}

