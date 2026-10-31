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
// package views.admin.inventory
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::item::ItemInterface::ItemInterface;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::http::file::upload::media::UploadMediaSingleton::UploadMediaSingleton;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FileUtil::FileUtil;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::media::MediaData::MediaData;
use crate::org::allbinary::logic::visual::media::MediaTypeData::MediaTypeData;
use crate::org::allbinary::logic::visual::media::MediaUtil::MediaUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InventoryUploadMediaUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub mediaUtil: Option<Rc<RefCell<MediaUtil>>>,
    pub LARGEWIDTH: i32,
    pub LARGEHEIGHT: i32,
    pub MEDIUMWIDTH: i32,
    pub MEDIUMHEIGHT: i32,
    pub SMALLWIDTH: i32,
    pub SMALLHEIGHT: i32,
    pub SMALL: String,
    pub MEDIUM: String,
    pub LARGE: String,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
    pub itemInterface: Option<Rc<RefCell<dyn ItemInterface>>>,
}


impl InventoryUploadMediaUtil {
    

    pub fn new_struct(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>, mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            mediaUtil: None,
            LARGEWIDTH: 0,
            LARGEHEIGHT: 0,
            MEDIUMWIDTH: 0,
            MEDIUMHEIGHT: 0,
            SMALLWIDTH: 0,
            SMALLHEIGHT: 0,
            SMALL: String::new(),
            MEDIUM: String::new(),
            LARGE: String::new(),
            storeFrontInterface: None,
            itemInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.mediaUtil = MediaUtil::getInstance();
        this.LARGEWIDTH = 1024;
        this.LARGEHEIGHT = 1024;
        this.MEDIUMWIDTH = 256;
        this.MEDIUMHEIGHT = 256;
        this.SMALLWIDTH = 128;
        this.SMALLHEIGHT = 128;
        this.SMALL = String::from("Small");
        this.MEDIUM = String::from("Medium");
        this.LARGE = String::from("Large");
        this.storeFrontInterface = storeFrontInterface.clone();
        this.itemInterface = itemInterface.clone();
        this
    }

    pub fn new(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>, mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(storeFrontInterface, itemInterface))))
    }
    

    // throws Exception
    pub fn saveFiles(&mut self, mut byteArray: Vec<i8>, mut fileName: String, mut mediaData: Option<Rc<RefCell<MediaData>>>) -> Option<Rc<RefCell<dyn ItemInterface>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Start FileName: "), fileName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveFiles()"));
        }
        if jcall!(StringValidationUtil::getInstance()).isEmpty(fileName.clone()) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Image File Name Was Null"))));
        }
        let mut fileUtil: Option<Rc<RefCell<FileUtil>>> = FileUtil::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
        jcall!(stringBuffer).append(jcall!(self.storeFrontInterface).getCurrentHostNamePath());
        jcall!(stringBuffer).append(jcall!(self.itemInterface).getCategory());
        let mut fullPath: String = jcall!(stringBuffer).toString();
        let mut imageDirectoryFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(fullPath.clone());
        if !jcall!(imageDirectoryFile).isDirectory() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Unable to save file to non existant directory"))));
        }
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(fullPath.clone());
        jcall!(stringBuffer).append(fileName.clone());
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone());
        jcall!(stringBuffer).append(jcall!(mediaData).getName());
        let mut originalImageFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(jcall!(stringBuffer).toString());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Saving Original Image: "), originalImageFile), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveFiles()"));
        }
        jcall!(originalImageFile).createNewFile();
        jcall!(fileUtil).write(ByteArrayInputStream::new(byteArray.clone()), originalImageFile.clone());
        let mut uploadMedia: Option<Rc<RefCell<UploadMediaSingleton>>> = UploadMediaSingleton::getInstance();
        let mut isMediaSupported: bool = jcall!(uploadMedia).isWriterSupported(jcall!(mediaData).getName()) || jcall!(uploadMedia).isReaderSupported(jcall!(mediaData).getName());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(String::from("Is Image Type: "));
            jcall!(stringBuffer).append(jcall!(mediaData).getName());
            jcall!(stringBuffer).append(String::from(" supported: "));
            jcall!(stringBuffer).appendboolean(isMediaSupported.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveFiles()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut defaultMediaData: Option<Rc<RefCell<MediaData>>> = MediaData::getDefault();
        self.setFileNames(fileName.clone());
        if isMediaSupported {
            let mut isMediaResizable: bool = jcall!(uploadMedia).isWriterMedia(jcall!(mediaData).getName(), jcall!(MediaTypeData::getInstance()).RESIZABLE_MEDIA.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(String::from("Is "));
                jcall!(stringBuffer).append(jcall!(mediaData).toString());
                jcall!(stringBuffer).append(String::from(" Image Resize Supported:"));
                jcall!(stringBuffer).appendboolean(jcall!(uploadMedia).isSupported(jcall!(mediaData).getName()));
                jcall!(stringBuffer).append(String::from(" WriterMedia: "));
                jcall!(stringBuffer).appendboolean(isMediaResizable.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveFiles()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            if isMediaResizable {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Saving Small Image: "), jcall!(self.itemInterface).getSmallImage()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveFiles()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                { let java_receiver = self.mediaUtil.clone(); let java_arguments = (originalImageFile.clone(), jcall!(self.itemInterface).getSmallImage(), fullPath.clone(), mediaData.clone(), self.SMALLWIDTH.clone(), self.SMALLHEIGHT.clone(),); jcall!(java_receiver).saveImageFile(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); };
                { let java_receiver = self.mediaUtil.clone(); let java_arguments = (originalImageFile.clone(), jcall!(self.itemInterface).getMediumImage(), fullPath.clone(), mediaData.clone(), self.MEDIUMWIDTH.clone(), self.MEDIUMHEIGHT.clone(),); jcall!(java_receiver).saveImageFile(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); };
                { let java_receiver = self.mediaUtil.clone(); let java_arguments = (originalImageFile.clone(), jcall!(self.itemInterface).getLargeImage(), fullPath.clone(), mediaData.clone(), self.LARGEWIDTH.clone(), self.LARGEHEIGHT.clone(),); jcall!(java_receiver).saveImageFile(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); };
            } else {
                let mut isConvertable: bool = jcall!(mediaData).isConvertableTo(defaultMediaData.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                    jcall!(stringBuffer).append(String::from("Converting: "));
                    jcall!(stringBuffer).append(jcall!(defaultMediaData).getName());
                    jcall!(stringBuffer).append(String::from(" into "));
                    jcall!(stringBuffer).append(jcall!(mediaData).getName());
                    jcall!(stringBuffer).append(String::from(" if Convertable is it: "));
                    jcall!(stringBuffer).appendboolean(isConvertable.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveFiles()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                if isConvertable {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Saving Small Image: "), jcall!(self.itemInterface).getSmallImage()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveFiles()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    { let java_receiver = self.mediaUtil.clone(); let java_arguments = (originalImageFile.clone(), jcall!(self.itemInterface).getSmallImage(), fullPath.clone(), defaultMediaData.clone(), self.SMALLWIDTH.clone(), self.SMALLHEIGHT.clone(),); jcall!(java_receiver).saveImageFile(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); };
                    { let java_receiver = self.mediaUtil.clone(); let java_arguments = (originalImageFile.clone(), jcall!(self.itemInterface).getMediumImage(), fullPath.clone(), defaultMediaData.clone(), self.MEDIUMWIDTH.clone(), self.MEDIUMHEIGHT.clone(),); jcall!(java_receiver).saveImageFile(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); };
                    { let java_receiver = self.mediaUtil.clone(); let java_arguments = (originalImageFile.clone(), jcall!(self.itemInterface).getLargeImage(), fullPath.clone(), defaultMediaData.clone(), self.LARGEWIDTH.clone(), self.LARGEHEIGHT.clone(),); jcall!(java_receiver).saveImageFile(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); };
                } else {
                    let mut smallImageFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(jcall!(self.itemInterface).getSmallImage());
                    jcall!(smallImageFile).createNewFile();
                    jcall!(fileUtil).write(ByteArrayInputStream::new(byteArray.clone()), smallImageFile.clone());
                    let mut mediumImageFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(jcall!(self.itemInterface).getMediumImage());
                    jcall!(smallImageFile).createNewFile();
                    jcall!(fileUtil).write(ByteArrayInputStream::new(byteArray.clone()), mediumImageFile.clone());
                    let mut largeImageFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(jcall!(self.itemInterface).getSmallImage());
                    jcall!(largeImageFile).createNewFile();
                    jcall!(fileUtil).write(ByteArrayInputStream::new(byteArray.clone()), largeImageFile.clone());
                }
            }
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).putF(jcall!(commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveFiles()"));
        }
        return self.itemInterface.clone();
    }
    

    pub fn setFileNames(&mut self, mut fileName: String) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.itemInterface).getId());
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone());
        jcall!(stringBuffer).append(jcall!(MediaData::getDefault()).getName());
        let mut END: String = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(fileName.clone());
        jcall!(stringBuffer).append(self.SMALL.clone());
        jcall!(stringBuffer).append(END.clone());
        let mut newImageFileName: String = jcall!(stringBuffer).toString();
        jcall!(self.itemInterface).setSmallImage(newImageFileName.clone());
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(fileName.clone());
        jcall!(stringBuffer).append(self.MEDIUM.clone());
        jcall!(stringBuffer).append(END.clone());
        newImageFileName = jcall!(stringBuffer).toString();
        jcall!(self.itemInterface).setMediumImage(newImageFileName.clone());
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(fileName.clone());
        jcall!(stringBuffer).append(self.LARGE.clone());
        jcall!(stringBuffer).append(END.clone());
        newImageFileName = jcall!(stringBuffer).toString();
        jcall!(self.itemInterface).setLargeImage(newImageFileName.clone());
    }
}

