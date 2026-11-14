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
// package org.allbinary.logic.visual.transform
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
use crate::java::io::InputStream::InputStream;
// use crate::javax::xml::transform::URIResolver::URIResolver;
use crate::org::allbinary::data::tree::dom::BasicUriResolver::BasicUriResolver;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::crypt::jcehelper::AbCrypt::AbCrypt;
use crate::org::allbinary::logic::control::crypt::jcehelper::KeySpecFactory::KeySpecFactory;
use crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStream;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::io::path::PathUtil::PathUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::AbKeys::AbKeys;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::template::TransformInfoTemplateData::TransformInfoTemplateData;
use crate::java::lang::Exception::*;
use super::AbTransformer::AbTransformer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicTransformer {
    pub base: AbTransformer,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub streamUtil: Option<Rc<RefCell<StreamUtil>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
}

impl crate::org::allbinary::logic::visual::transform::info::CompositeTransformInfoInterface::CompositeTransformInfoInterface for BasicTransformer {
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface>>> { crate::org::allbinary::logic::visual::transform::AbTransformer::AbTransformer::getTransformInfoInterface(crate::org::allbinary::logic::visual::transform::AbTransformer::AbTransformerDyn::as_AbTransformer_mut(self)) }
}

impl BasicTransformer {
    

    pub fn new_struct(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: AbTransformer::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            streamUtil: None,
            abeClientInformation: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.streamUtil = StreamUtil::getInstance();
        // super(...) is in the struct literal above
        this.abeClientInformation = abeClientInformation.clone();
        this.setTemplateAsInputStream();
        this
    }

    pub fn new(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(abeClientInformation, transformInfoInterface))))
    }
    

    // throws Exception
    pub fn setEncryptedTemplateFileAsInputStream(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) {
        let mut outputStream: Option<Rc<RefCell<crate::java::io::OutputStream::OutputStream>>> = None;
        let mut inputStream: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            outputStream = crate::java::io::OutputStream::OutputStream::new();
            inputStream = AbFileInputStream::new(file.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>);
            outputStream = { let java_value = ((jcall!(self.streamUtil).get(inputStream.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>), outputStream.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>), vec![0; (16384) as usize])).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<ByteArrayOutputStream>() }, "ClassCastException: not a ByteArrayOutputStream"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<ByteArrayOutputStream>) } })).clone(); java_value };
            let mut abCrypt: Option<Rc<RefCell<AbCrypt>>> = AbCrypt::new(crate::org::allbinary::logic::control::crypt::jcehelper::KeySpecFactory::KeySpecFactory_DESEDE.with(Clone::clone).clone(), jcall!(AbKeys::getInstance()).getKey(self.abeClientInformation.clone(), jcall!(file).getAbsolutePath()));
            let mut decrypted: Vec<i8> = jcall!(abCrypt).decrypt(jcall!(outputStream).toByteArray());
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORY.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Decrypted Template: \n"), String::from_utf8(decrypted.clone().iter().map(|value| *value as u8).collect()).unwrap()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setEncryptedTemplateFileAsInputStream(file)"));
            }
            self.setInputStream((crate::java::io::InputStream::InputStream::new1(decrypted.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>)).clone() /* as InputStream */);
            { let java_arguments = ((BasicUriResolver::new({ let java_field = jcall!(TransformInfoTemplateData::getInstance()).ENCRYPTED_EXTENSION.clone(); java_field }.clone())).clone() /* as URIResolver */,); self.setURIResolver(java_arguments.0) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORYERROR.clone(); java_field }.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Failed to get Encrypted File: "));
                jcall!(stringBuffer).append(jcall!(file).getPath());
                jcall!(stringBuffer).append(String::from(" \nName:"));
                jcall!(stringBuffer).append(jcall!(file).getName());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setEncryptedTemplateFileAsInputStream()"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
        { // finally
            jcall!(self.streamUtil).close(outputStream.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::io::Closeable::Closeable>>));
            jcall!(self.streamUtil).close(inputStream.clone().map(|_java_reference| -> Rc<RefCell<dyn crate::java::io::Closeable::Closeable>> { panic!("ClassCastException: the Rust trait of AbFileInputStream does not have Closeable as a base trait") }));
        }
    }
    

    // throws Exception
    pub fn setEncryptedTemplateAsInputStream(&mut self) {
        let mut outputStream: Option<Rc<RefCell<crate::java::io::OutputStream::OutputStream>>> = None;
        let mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            outputStream = crate::java::io::OutputStream::OutputStream::new();
            inputStream = { let java_value = (crate::java::io::InputStream::InputStream::new1({ let java_argument = jcall!(self.getTransformInfoInterface()).getTemplate().bytes().map(|value| value as i8).collect::<Vec<i8>>(); java_argument }).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>)).clone(); java_value };
            outputStream = { let java_value = ((jcall!(self.streamUtil).get(inputStream.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>), outputStream.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>), vec![0; (16384) as usize])).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<ByteArrayOutputStream>() }, "ClassCastException: not a ByteArrayOutputStream"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<ByteArrayOutputStream>) } })).clone(); java_value };
            let mut abCrypt: Option<Rc<RefCell<AbCrypt>>> = AbCrypt::new(crate::org::allbinary::logic::control::crypt::jcehelper::KeySpecFactory::KeySpecFactory_DESEDE.with(Clone::clone).clone(), { let java_argument = jcall!(AbKeys::getInstance()).getKey(self.abeClientInformation.clone(), { let java_argument = jcall!(self.getTransformInfoInterface()).getName(); java_argument }); java_argument });
            let mut decrypted: Vec<i8> = jcall!(abCrypt).decrypt(jcall!(outputStream).toByteArray());
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORY.clone(); java_field }.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Decrypted Template: \n"), jcall!(decrypted).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setEncryptedTemplateAsInputStream()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            self.setInputStream((crate::java::io::InputStream::InputStream::new1(decrypted.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>)).clone() /* as InputStream */);
            { let java_arguments = ((BasicUriResolver::new({ let java_field = jcall!(TransformInfoTemplateData::getInstance()).ENCRYPTED_EXTENSION.clone(); java_field }.clone())).clone() /* as URIResolver */,); self.setURIResolver(java_arguments.0) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORYERROR.clone(); java_field }.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Failed to set with template: "), { let java_value = jcall!(self.getTransformInfoInterface()).getTemplateFilePath(); java_value }), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setEncryptedTemplateFileAsInputStream()"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
        { // finally
            jcall!(self.streamUtil).close(outputStream.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::io::Closeable::Closeable>>));
            jcall!(self.streamUtil).close(inputStream.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::io::Closeable::Closeable>>));
        }
    }
    

    // throws Exception
    pub fn setTemplateAsInputStream(&mut self) {
        let mut inputStream: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>> = None;
        let mut outputStream: Option<Rc<RefCell<crate::java::io::OutputStream::OutputStream>>> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORY.clone(); java_field }.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_argument = jcall!(self.getTransformInfoInterface()).log(); java_argument }, crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setTemplateAsInputStream()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut transformInfoTemplateData: Option<Rc<RefCell<TransformInfoTemplateData>>> = TransformInfoTemplateData::getInstance();
            let mut pathUtil: Option<Rc<RefCell<PathUtil>>> = PathUtil::getInstance();
            if { let java_value = jcall!(self.getTransformInfoInterface()).getTemplateFilePath(); java_value }.is_some() {
                let mut extension: String = jcall!(pathUtil).getExtensionWithAbPath({ let java_argument = jcall!(self.getTransformInfoInterface()).getTemplateFilePath(); java_argument });
                let mut filePath: String = jcall!(pathUtil).getWithoutExtensionWithAbPath({ let java_argument = jcall!(self.getTransformInfoInterface()).getTemplateFilePath(); java_argument });
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORY.clone(); java_field }.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_argument = jcall!(self.getTransformInfoInterface()).log(); java_argument }, crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setTemplateAsInputStream()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                if (extension.cmp(&{ let java_field = jcall!(transformInfoTemplateData).UNCRYPTED_EXTENSION.clone(); java_field }.clone()) as i32) == 0 {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(filePath.clone());
                    jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone(); java_field }.clone());
                    jcall!(stringBuffer).append({ let java_field = jcall!(transformInfoTemplateData).ENCRYPTED_EXTENSION.clone(); java_field }.clone());
                    let mut encFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(jcall!(stringBuffer).toString());
                    if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORY.clone(); java_field }.clone()) {
                        { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_argument = jcall!(self.getTransformInfoInterface()).log(); java_argument }, crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setTemplateAsInputStream()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORY.clone(); java_field }.clone()) {
                        stringBuffer = StringMaker::new();
                        jcall!(stringBuffer).append(String::from("Encrypted Template File isFile="));
                        jcall!(stringBuffer).appendboolean(jcall!(encFile).isFile());
                        jcall!(stringBuffer).append(String::from("\nEncTemplateFilePath: "));
                        jcall!(stringBuffer).append(jcall!(encFile).getPath());
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setTemplateAsInputStream()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    if jcall!(encFile).isFile() {
                        self.setEncryptedTemplateFileAsInputStream(encFile.clone());
                        return;
                    } else {
                        let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath({ let java_argument = jcall!(self.getTransformInfoInterface()).getTemplateFilePath(); java_argument });
                        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORY.clone(); java_field }.clone()) {
                            { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_argument = jcall!(self.getTransformInfoInterface()).log(); java_argument }, crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setTemplateAsInputStream()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                        }
                        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORY.clone(); java_field }.clone()) {
                            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Template File isFile="), jcall!(file).isFile()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setTemplateAsInputStream()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                        }
                        if jcall!(file).isFile() {
                            inputStream = AbFileInputStream::new(file.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>);
                            outputStream = crate::java::io::OutputStream::OutputStream::new();
                            outputStream = { let java_value = ((jcall!(self.streamUtil).get(inputStream.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>), outputStream.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>), vec![0; (16384) as usize])).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<ByteArrayOutputStream>() }, "ClassCastException: not a ByteArrayOutputStream"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<ByteArrayOutputStream>) } })).clone(); java_value };
                            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORY.clone(); java_field }.clone()) {
                                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Template: "), jcall!(outputStream).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getTemplateAsInputStream()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                            }
                            { let java_arguments = ((crate::java::io::InputStream::InputStream::new1(jcall!(outputStream).toByteArray()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>)).clone() /* as InputStream */,); self.setInputStream(java_arguments.0) };
                            { let java_arguments = ((BasicUriResolver::new({ let java_field = jcall!(transformInfoTemplateData).UNCRYPTED_EXTENSION.clone(); java_field }.clone())).clone() /* as URIResolver */,); self.setURIResolver(java_arguments.0) };
                            return;
                        }
                    }
                } else if (extension.cmp(&{ let java_field = jcall!(transformInfoTemplateData).ENCRYPTED_EXTENSION.clone(); java_field }.clone()) as i32) == 0 {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(filePath.clone());
                    jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone(); java_field }.clone());
                    jcall!(stringBuffer).append({ let java_field = jcall!(transformInfoTemplateData).ENCRYPTED_EXTENSION.clone(); java_field }.clone());
                    let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(jcall!(stringBuffer).toString());
                    if jcall!(file).isFile() {
                        self.setEncryptedTemplateFileAsInputStream(file.clone());
                    }
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("View Template File Type Is Not Recognized: "), { let java_value = jcall!(self.getTransformInfoInterface()).getTemplateFilePath(); java_value }))));
                }
            } else {
                self.setEncryptedTemplateAsInputStream();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERFACTORYERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to get Template Data"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setTemplateAsInputStream()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
        { // finally
            jcall!(self.streamUtil).close(outputStream.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::io::Closeable::Closeable>>));
            jcall!(self.streamUtil).close(inputStream.clone().map(|_java_reference| -> Rc<RefCell<dyn crate::java::io::Closeable::Closeable>> { panic!("ClassCastException: the Rust trait of AbFileInputStream does not have Closeable as a base trait") }));
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Error setTemplateAsInputStream()"))));
    }
}
impl std::ops::Deref for BasicTransformer {
    type Target = AbTransformer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicTransformer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasicTransformerDyn: crate::org::allbinary::logic::visual::transform::AbTransformer::AbTransformerDyn {
    fn as_BasicTransformer(&self) -> &BasicTransformer;
    fn as_BasicTransformer_mut(&mut self) -> &mut BasicTransformer;
    fn setEncryptedTemplateFileAsInputStream(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) { BasicTransformer::setEncryptedTemplateFileAsInputStream(self.as_BasicTransformer_mut(), file) }
    fn setEncryptedTemplateAsInputStream(&mut self) { BasicTransformer::setEncryptedTemplateAsInputStream(self.as_BasicTransformer_mut()) }
    fn setTemplateAsInputStream(&mut self) { BasicTransformer::setTemplateAsInputStream(self.as_BasicTransformer_mut()) }
}
impl BasicTransformerDyn for BasicTransformer {
    fn as_BasicTransformer(&self) -> &BasicTransformer { self }
    fn as_BasicTransformer_mut(&mut self) -> &mut BasicTransformer { self }
}

impl crate::org::allbinary::logic::visual::transform::AbTransformer::AbTransformerDyn for BasicTransformer {
    fn as_AbTransformer(&self) -> &crate::org::allbinary::logic::visual::transform::AbTransformer::AbTransformer { &self.base }
    fn as_AbTransformer_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::AbTransformer::AbTransformer { &mut self.base }
}


