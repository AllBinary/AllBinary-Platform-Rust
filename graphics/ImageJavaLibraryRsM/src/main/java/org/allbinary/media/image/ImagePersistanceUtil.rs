// 
// To change this license header, choose License Headers in Project Properties. 
// To change this template file, choose Tools | Templates 
// and open the template in the editor.  
// Generated Code Do Not Modify
// package org.allbinary.media.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::java::io::File::File;
// use crate::java::io::IOException::IOException;
// use crate::java::util::Iterator::Iterator;
// use crate::javax::imageio::IIOImage::IIOImage;
// use crate::javax::imageio::ImageIO::ImageIO;
// use crate::javax::imageio::ImageWriteParam::ImageWriteParam;
// use crate::javax::imageio::ImageWriter::ImageWriter;
// use crate::javax::imageio::stream::ImageOutputStream::ImageOutputStream;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::AbFileOutputStream::AbFileOutputStream;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::apache::batik::ext::awt::image::codec::png::PNGImageWriter::PNGImageWriter;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImagePersistanceUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

thread_local! { pub static ImagePersistanceUtil_instance: Option<Rc<RefCell<ImagePersistanceUtil>>> = ImagePersistanceUtil::new(); }

impl ImagePersistanceUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ImagePersistanceUtil>>> {
        return ImagePersistanceUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn saveWithBatik(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) {
        let mut batikPNGImageWriter: Option<Rc<RefCell<PNGImageWriter>>> = PNGImageWriter::new();
        let mut fileOutputStream: Option<Rc<RefCell<AbFileOutputStream>>> = AbFileOutputStream::createFromAbFile(file.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(batikPNGImageWriter).writeImage(bufferedImage.clone(), fileOutputStream.clone());
        })) {
            Ok(_) => {}
            Err(error) => std::panic::resume_unwind(error),
        }
        { // finally
            jcall!(fileOutputStream).flush();
            jcall!(StreamUtil::getInstance()).close(fileOutputStream.clone());
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Wrote Image: "), jcall!(file).getAbsolutePath()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).SAVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn saveWithImageIO(&mut self, mut filePath: String, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) {
        self.saveWithImageIO(File::new(filePath.clone()), bufferedImage.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn saveWithImageIO(&mut self, mut file: Option<Rc<RefCell<File>>>, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) {
        let mut writer: Option<Rc<RefCell<ImageWriter>>> = None;
        let mut ios: Option<Rc<RefCell<ImageOutputStream>>> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut iter: Option<Rc<RefCell<dyn Iterator>>> = ImageIO::getImageWritersByFormatName(String::from("jpeg"));
            if !jcall!(iter).hasNext() {
                jcall!(self.logUtil).putF(String::from("Unable to save image to jpeg file type."), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).SAVE.clone());
                return;
            }
            writer = (jcall!(iter).next()).clone() /* as ImageWriter */;
            ios = ImageIO::createImageOutputStream(file.clone());
            jcall!(writer).setOutput(ios.clone());
            let mut iwp: Option<Rc<RefCell<ImageWriteParam>>> = jcall!(writer).getDefaultWriteParam();
            jcall!(iwp).setCompressionMode(unsafe { crate::javax::imageio::ImageWriteParam::ImageWriteParam_MODE_EXPLICIT.clone() }.clone());
            jcall!(iwp).setCompressionQuality(0.95_f32);
            jcall!(writer).write(None, IIOImage::new(bufferedImage.clone(), None, None), iwp.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Wrote Image: "), jcall!(file).getAbsolutePath()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).SAVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).SAVE.clone(), crate::java_into_object(e.clone()));
            }
        }
        { // finally
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if ios.is_some() {
                    jcall!(ios).flush();
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        if ios.is_some() {
                            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(ios).toString(), crate::java_into_object(ios.clone()), jcall!(self.commonStrings).CLOSE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                            jcall!(ios).close();
                        }
                    })) {
                        Ok(_) => {}
                        Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                        jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(ios.clone()), jcall!(self.commonStrings).CLOSE.clone(), crate::java_into_object(e.clone()));
                        }
                    }
                }
                if writer.is_some() {
                    jcall!(writer).dispose();
                }
            })) {
                Ok(_) => {}
                Err(e2) => {
            let e2: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e2 as Box<dyn std::any::Any>)));

                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).SAVE.clone(), crate::java_into_object(e2.clone()));
                }
            }
        }
    }
}

