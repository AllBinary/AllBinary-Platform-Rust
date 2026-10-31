// Generated Code Do Not Modify
// package org.allbinary.logic.visual.media
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::java::awt::image::RenderedImage::RenderedImage;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::image::ImageUtil::ImageUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use super::MediaData::MediaData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//import com.sun.imageio.plugins.common.ImageUtil;
pub struct MediaUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static MediaUtil_instance: Option<Rc<RefCell<MediaUtil>>> = MediaUtil::new(); }

impl MediaUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<MediaUtil>>> {
        return MediaUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn saveImageFile(&mut self, mut originalImageFile: Option<Rc<RefCell<AbFile>>>, mut newImageFileName: String, mut category: String, mut mediaData: Option<Rc<RefCell<MediaData>>>, mut newWidth: i32, mut newHeight: i32) {
        if originalImageFile.is_none() || !jcall!(originalImageFile).isFile() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Original Image File Does Not Exist."))));
        }
        let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = ImageIOUtil::ImageIOUtil::read(originalImageFile.clone());
        if bufferedImage.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Unable to find ImageReader for this file."))));
        }
        if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).VIEW.clone())) {
            let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.getImageBufferPropertyHashMap(bufferedImage.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Image Properties: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveImageFile()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut imageFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(format!("{}{}", category, newImageFileName));
        jcall!(imageFile).createNewFile();
        let mut imageUtil: Option<Rc<RefCell<ImageUtil>>> = ImageUtil::getInstance();
        let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(imageUtil).createBufferedImageForSave(bufferedImage.clone(), newWidth.clone(), newHeight.clone());
        let mut isWritten: bool = ImageIOUtil::ImageIOUtil::write((newBufferedImage).clone() /* as RenderedImage */, jcall!(mediaData).getName(), imageFile.clone());
        if !isWritten {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Unable to write."))));
        }
        if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).VIEW.clone())) {
            let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Get Path: "));
            jcall!(stringBuffer).append(jcall!(originalImageFile).getPath());
            jcall!(stringBuffer).append(String::from("\nNewImageFileName: "));
            jcall!(stringBuffer).append(newImageFileName.clone());
            jcall!(stringBuffer).append(String::from("\nCategory: "));
            jcall!(stringBuffer).append(category.clone());
            jcall!(stringBuffer).append(String::from("\nSave File Type: "));
            jcall!(stringBuffer).append(jcall!(mediaData).getName());
            jcall!(stringBuffer).append(String::from("\nNew")).append(jcall!(commonLabels).WIDTH_LABEL.clone());
            jcall!(stringBuffer).appendint(newWidth.clone());
            jcall!(stringBuffer).append(String::from("\nNew")).append(jcall!(commonLabels).HEIGHT_LABEL.clone());
            jcall!(stringBuffer).appendint(newHeight.clone());
            jcall!(stringBuffer).append(String::from("\nFile Length: "));
            jcall!(stringBuffer).appendlong(((jcall!(originalImageFile).length()) as i64));
            jcall!(stringBuffer).append(String::from("\nNew File Length: "));
            jcall!(stringBuffer).appendlong(((jcall!(imageFile).length()) as i64));
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveImageFile()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    pub fn getImageBufferPropertyHashMap(mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut propertyStringArray: Vec<String> = jcall!(bufferedImage).getPropertyNames();
        if propertyStringArray.is_some() {
            for index in (0..(propertyStringArray.len() as i32)) {
                let mut propertyObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(bufferedImage).getProperty(propertyStringArray[(index) as usize].clone());
                jcall!(hashMap).put(propertyStringArray[(index) as usize].clone(), { let value = jcall!(propertyObject); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) });
            }
        }
        return hashMap.clone();
    }
}

