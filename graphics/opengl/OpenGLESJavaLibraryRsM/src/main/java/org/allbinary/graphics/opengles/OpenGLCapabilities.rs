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
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::khronos::opengles::GL11::GL11;
use crate::org::allbinary::AndroidUtil::AndroidUtil;
use crate::org::allbinary::image::opengles::OpenGLESGL10ImageFactory::OpenGLESGL10ImageFactory;
use crate::org::allbinary::image::opengles::OpenGLESGL11VBOImageFactory::OpenGLESGL11VBOImageFactory;
use crate::org::allbinary::image::opengles::OpenGLImageSpecificFactory::OpenGLImageSpecificFactory;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::tokens::Tokenizer::Tokenizer;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::OpenGLFeatureFactory::OpenGLFeatureFactory;
use super::OpenGLFeature::OpenGLFeature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLCapabilities {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub CUSTOM_GL_SURFACE_VIEW: bool,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub initialized: bool,
    pub glVersionString: String,
    pub glShaderVersionString: String,
    pub shaderVersion: i32,
    pub glRenderer: String,
    pub glVendor: String,
    pub glExtensions: String,
    pub possiblyAccelerated: bool,
    pub acceleratedString: String,
    pub maxTextureSize: i32,
    pub VERSION_1_0: String,
    pub VERSION_1_1: String,
    pub VERSION_2_0: String,
    pub VERSION_3_0: String,
    pub VERSION_3_1: String,
    pub VERSION_3_2: String,
    pub VERSION_HIGHER_THAN_EXISTS: String,
    pub VERSION_UNK: String,
    pub GL_EXT_GPU_SHADER_100: String,
    pub GL_EXT_GPU_SHADER4: String,
    pub glVersion: String,
    pub glInstanceVersion: String,
    pub glExtensionDrawTexture: bool,
    pub glThreedDrawTexture: bool,
    pub glExtensionGPUShader4: bool,
    pub glExtensionGPUShader100: bool,
    pub vertexBufferObjectSupport: bool,
}

thread_local! { pub static OpenGLCapabilities_instance: Option<Rc<RefCell<OpenGLCapabilities>>> = OpenGLCapabilities::new(); }

impl OpenGLCapabilities {
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLCapabilities>>> {
        return OpenGLCapabilities_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            CUSTOM_GL_SURFACE_VIEW: false,
            stringUtil: None,
            initialized: false,
            glVersionString: String::new(),
            glShaderVersionString: String::new(),
            shaderVersion: 0,
            glRenderer: String::new(),
            glVendor: String::new(),
            glExtensions: String::new(),
            possiblyAccelerated: false,
            acceleratedString: String::new(),
            maxTextureSize: 0,
            VERSION_1_0: String::new(),
            VERSION_1_1: String::new(),
            VERSION_2_0: String::new(),
            VERSION_3_0: String::new(),
            VERSION_3_1: String::new(),
            VERSION_3_2: String::new(),
            VERSION_HIGHER_THAN_EXISTS: String::new(),
            VERSION_UNK: String::new(),
            GL_EXT_GPU_SHADER_100: String::new(),
            GL_EXT_GPU_SHADER4: String::new(),
            glVersion: String::new(),
            glInstanceVersion: String::new(),
            glExtensionDrawTexture: false,
            glThreedDrawTexture: false,
            glExtensionGPUShader4: false,
            glExtensionGPUShader100: false,
            vertexBufferObjectSupport: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.CUSTOM_GL_SURFACE_VIEW = true;
        this.stringUtil = StringUtil::getInstance();
        this.initialized = false;
        this.glVersionString = jcall!(this.stringUtil).EMPTY_STRING.clone();
        this.glShaderVersionString = jcall!(this.stringUtil).EMPTY_STRING.clone();
        this.shaderVersion = 0;
        this.glRenderer = jcall!(this.stringUtil).EMPTY_STRING.clone();
        this.glVendor = jcall!(this.stringUtil).EMPTY_STRING.clone();
        this.glExtensions = jcall!(this.stringUtil).EMPTY_STRING.clone();
        this.acceleratedString = jcall!(this.stringUtil).EMPTY_STRING.clone();
        this.maxTextureSize = 64;
        this.VERSION_1_0 = String::from("1.0");
        this.VERSION_1_1 = String::from("1.1");
        this.VERSION_2_0 = String::from("2.0");
        this.VERSION_3_0 = String::from("3.0");
        this.VERSION_3_1 = String::from("3.1");
        this.VERSION_3_2 = String::from("3.2");
        this.VERSION_HIGHER_THAN_EXISTS = String::from("999.999");
        this.VERSION_UNK = String::from("Unk");
        this.GL_EXT_GPU_SHADER_100 = String::from("GL_ARB_shading_language_100");
        this.GL_EXT_GPU_SHADER4 = String::from("GL_EXT_gpu_shader4");
        this.glVersion = this.glVersionString.clone();
        this.glInstanceVersion = this.VERSION_UNK.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isInitialized(&mut self) -> bool {
        return self.initialized.clone();
    }
    

    pub fn requireInitialization(&mut self) {
    }
    

    pub fn initCapabilities(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
        let mut METHOD_NAME: String = String::from("initGLCapabilities");
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
            let mut openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>> = OpenGLFeatureFactory::getInstance();
            let mut openGLImageSpecificFactory: Option<Rc<RefCell<OpenGLImageSpecificFactory>>> = OpenGLImageSpecificFactory::getInstance();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            self.glVersionString = jcall!(gl).glGetString(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VERSION.clone() }.clone());
            let mut GL_SHADING_LANGUAGE_VERSION: i32 = 0x8b8c_u32 as i32;
            self.glShaderVersionString = jcall!(gl).glGetString(GL_SHADING_LANGUAGE_VERSION.clone());
            if false {
                self.glShaderVersionString = jcall!(self.stringUtil).EMPTY_STRING.clone();
            }
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if true && self.glShaderVersionString.find('.').map(|index| index as i32).unwrap_or(-1) >= 0 {
                    let mut startIndex: i32 = self.glShaderVersionString.rfind(' ').map(|index| index as i32).unwrap_or(-1);
                    if startIndex >= 0 {
                        self.glShaderVersionString = self.glShaderVersionString[(startIndex + 1) as usize..].to_string();
                    }
                    self.shaderVersion = self.glShaderVersionString.replace(&jcall!(CommonSeps::getInstance()).PERIOD, &jcall!(StringUtil::getInstance()).EMPTY_STRING).parse::<i32>().unwrap();
                }
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(), crate::java_into_object(e.clone()));
                }
            }
            self.glRenderer = jcall!(gl).glGetString(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_RENDERER.clone() }.clone());
            self.glVendor = jcall!(gl).glGetString(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VENDOR.clone() }.clone());
            self.glExtensions = jcall!(gl).glGetString(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_EXTENSIONS.clone() }.clone());
            if self.glExtensions.find(&self.GL_EXT_GPU_SHADER_100.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                self.glExtensionGPUShader100 = true;
            } else {
                self.glExtensionGPUShader100 = false;
            }
            if self.glExtensions.find(&self.GL_EXT_GPU_SHADER4.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                self.glExtensionGPUShader4 = true;
            } else {
                self.glExtensionGPUShader4 = false;
            }
            if false {
                self.glRenderer = jcall!(self.stringUtil).EMPTY_STRING.clone();
            }
            if self.glRenderer.to_lowercase().find(&String::from("pixelflinger")).map(|index| index as i32).unwrap_or(-1) >= 0 {
                self.acceleratedString = format!("{}{}", String::from("Probably Not for "), self.glRenderer);
                self.possiblyAccelerated = false;
            } else {
                self.acceleratedString = format!("{}{}", String::from("Probably for "), self.glRenderer);
                self.possiblyAccelerated = true;
            }
            self.glExtensionDrawTexture = false;
            if false {
                self.glVersionString = jcall!(self.stringUtil).EMPTY_STRING.clone();
                self.glVersion = self.VERSION_UNK.clone();
            } else if self.glVersionString.find(&String::from(" 1.0")).map(|index| index as i32).unwrap_or(-1) >= 0 {
                self.glVersion = self.VERSION_1_0.clone();
            } else if self.glVersionString.find(&String::from(" 1.1")).map(|index| index as i32).unwrap_or(-1) >= 0 {
                self.glVersion = self.VERSION_1_1.clone();
            } else {
                self.glVersion = self.VERSION_UNK.clone();
            }
            if (gl).is_some() /* instanceof GL11 */ {
                self.glInstanceVersion = self.VERSION_1_1.clone();
            } else if (gl).is_some() /* instanceof GL10 */ {
                self.glInstanceVersion = self.VERSION_1_0.clone();
            }
            if false {
                self.glVendor = jcall!(self.stringUtil).EMPTY_STRING.clone();
            }
            if false {
                self.glExtensions = jcall!(self.stringUtil).EMPTY_STRING.clone();
            }
            if self.possiblyAccelerated {
                PreLogUtil::put(jcall!(StringMaker::new()).append(String::from("VBO:?")).appendboolean((self.glInstanceVersion == self.VERSION_1_1).clone()).append(String::from("||")).appendboolean(self.isExtension(jcall!(openGLFeatureFactory).OPENGL_VERTEX_BUFFER_OBJECT.clone())).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
                if (self.glInstanceVersion == self.VERSION_1_1 || self.isExtension(jcall!(openGLFeatureFactory).OPENGL_VERTEX_BUFFER_OBJECT.clone())) {
                    if AndroidUtil::isAndroid() {
                        PreLogUtil::put(String::from("VBO implementation was detected, but disabled by default for Android"), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
                    } else {
                        PreLogUtil::put(String::from("VBO implementation was detected, but disabled for now"), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
                    }
                }
            }
            let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
            if jcall!(features).isDefault(jcall!(openGLFeatureFactory).OPENGL_AUTO_SELECT.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
                if self.vertexBufferObjectSupport {
                    jcall!(openGLImageSpecificFactory).setImageFactory(OpenGLESGL11VBOImageFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLImageFactory::OpenGLImageFactoryDyn>>));
                } else {
                    jcall!(stringBuffer).append(String::from("OpenGL is on but "));
                    jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(jcall!(openGLFeatureFactory).OPENGL_DRAW_TEXTURE.clone())));
                    jcall!(stringBuffer).append(String::from(" was not available"));
                    PreLogUtil::put(jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
                    jcall!(openGLImageSpecificFactory).setImageFactory(OpenGLESGL10ImageFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLImageFactory::OpenGLImageFactoryDyn>>));
                }
            } else {
                jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(jcall!(openGLFeatureFactory).OPENGL_AUTO_SELECT.clone())));
                jcall!(stringBuffer).append(String::from(" is not on"));
                PreLogUtil::put(jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
                jcall!(openGLImageSpecificFactory).setImageFactory(OpenGLESGL10ImageFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLImageFactory::OpenGLImageFactoryDyn>>));
            }
            let mut maxTextureSizeArray: Vec<i32> = vec![0; (1) as usize];
            jcall!(gl).glGetIntegerv(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_MAX_TEXTURE_SIZE.clone() }.clone(), maxTextureSizeArray.clone(), 0);
            self.maxTextureSize = maxTextureSizeArray[(0) as usize].clone();
            self.initialized = true;
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn isExtension(&mut self, mut gameFeature: Option<Rc<RefCell<OpenGLFeature>>>) -> bool {
        let mut index: i32 = self.glExtensions.find(&jcall!(gameFeature).getName()).map(|index| index as i32).unwrap_or(-1);
        if index >= 0 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isGlExtensionDrawTexture(&mut self) -> bool {
        self.requireInitialization();
        return self.glExtensionDrawTexture.clone();
    }
    

    pub fn isGlExtensionGPUShader100(&mut self) -> bool {
        return self.glExtensionGPUShader100.clone();
    }
    

    pub fn isGlExtensionGPUShader4(&mut self) -> bool {
        self.requireInitialization();
        return self.glExtensionGPUShader4.clone();
    }
    

    pub fn getGlVersion(&mut self) -> String {
        self.requireInitialization();
        return self.glVersion.clone();
    }
    

    pub fn getGlVersionString(&mut self) -> String {
        self.requireInitialization();
        return self.glVersionString.clone();
    }
    

    pub fn getGlShaderVersion(&mut self) -> String {
        self.requireInitialization();
        return self.glShaderVersionString.clone();
    }
    

    pub fn isGlThreedDrawTexture(&mut self) -> bool {
        self.requireInitialization();
        return self.glThreedDrawTexture.clone();
    }
    

    pub fn getGlRenderer(&mut self) -> String {
        self.requireInitialization();
        return self.glRenderer.clone();
    }
    

    pub fn isVertexBufferObjectSupport(&mut self) -> bool {
        self.requireInitialization();
        return self.vertexBufferObjectSupport.clone();
    }
    

    pub fn isTextureSizeValid(&mut self, mut widthAndHeight: i32) -> bool {
        self.requireInitialization();
        if self.maxTextureSize >= widthAndHeight {
            return true;
        }
        return false;
    }
    

    pub fn toString(&mut self) -> String {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("GL_VERSION: "));
        jcall!(stringBuffer).append(self.glVersionString.clone());
        jcall!(stringBuffer).append(String::from(" GL_SHADING_LANGUAGE_VERSION: "));
        jcall!(stringBuffer).append(self.glShaderVersionString.clone());
        jcall!(stringBuffer).append(String::from(" GL_RENDERER: "));
        jcall!(stringBuffer).append(self.glRenderer.clone());
        jcall!(stringBuffer).append(String::from(" GL_VENDOR: "));
        jcall!(stringBuffer).append(self.glVendor.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        jcall!(stringBuffer).append(String::from(" Is Accelerated: "));
        jcall!(stringBuffer).append(self.acceleratedString.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        jcall!(stringBuffer).append(String::from(" VBO Support: "));
        jcall!(stringBuffer).appendboolean(self.isVertexBufferObjectSupport());
        jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        jcall!(stringBuffer).append(String::from(" Max Texture Size: "));
        jcall!(stringBuffer).appendint(self.maxTextureSize.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        jcall!(stringBuffer).append(String::from(" GL_EXTENSIONS: "));
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut tokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new(jcall!(commonSeps).SPACE.clone());
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(tokenizer).getTokensFromString(self.glExtensions.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
            let mut size: i32 = jcall!(list).size();
            for index in (0..size) {
                jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
                jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize].clone())));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).TOSTRING.clone(), e.clone());
            }
        }
        return jcall!(stringBuffer).toString();
    }
}

