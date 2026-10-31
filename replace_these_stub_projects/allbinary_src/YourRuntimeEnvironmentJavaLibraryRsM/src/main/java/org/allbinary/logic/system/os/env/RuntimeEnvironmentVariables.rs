// Generated Code Do Not Modify
// package org.allbinary.logic.system.os.env
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::BufferedReader::BufferedReader;
// use crate::java::io::InputStreamReader::InputStreamReader;
// use crate::java::util::Properties::Properties;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;
use crate::org::allbinary::logic::system::os::OperatingSystemFactory::OperatingSystemFactory;
use crate::org::allbinary::logic::system::os::OperatingSystems::OperatingSystems;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Runtime::*;
use super::Process::Process;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RuntimeEnvironmentVariables {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}


impl RuntimeEnvironmentVariables {
    

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
    

    pub fn getTest1(&mut self) -> String {
        return match String::from("java.java.home").as_str() { "os.name" => std::env::consts::OS.to_string(), "os.arch" => std::env::consts::ARCH.to_string(), "user.home" => std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_default(), _ => String::new() };
    }
    

    pub fn getVariable(&mut self, mut var: String) -> String {
        let mut environmentVariables: Option<Rc<RefCell<Properties>>> = self.get();
        return jcall!(environmentVariables).getProperty(var.clone());
    }
    

    pub fn get(&mut self) -> Option<Rc<RefCell<Properties>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut process: Option<Rc<RefCell<Process>>> = None;
            let mut environmentVariables: Option<Rc<RefCell<Properties>>> = Properties::new();
            let mut operatingSystemInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> = jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance();
            let mut osName: String = jcall!(operatingSystemInterface).getName();
            let mut linuxCommand: String = String::from("env");
            let mut windowsCommand: String = String::from("cmd.exe /c set");
            let mut solarisCommand: String = linuxCommand.clone();
            let mut operatingSystems: Option<Rc<RefCell<OperatingSystems>>> = OperatingSystems::getInstance();
            if (osName.cmp(&jcall!(operatingSystems).LINUX.clone()) as i32) == 0 {
                process = jcall!(Runtime::getRuntime()).exec(linuxCommand.clone());
            } else if osName.find(&jcall!(operatingSystems).WINDOWS.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                process = jcall!(Runtime::getRuntime()).exec(windowsCommand.clone());
            } else if (osName.cmp(&jcall!(operatingSystems).SOLARIS.clone()) as i32) == 0 {
                process = jcall!(Runtime::getRuntime()).exec(solarisCommand.clone());
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", ()));
            }
            let mut processInputStreamReader: Option<Rc<RefCell<InputStreamReader>>> = InputStreamReader::new(jcall!(process).getInputStream());
            let mut bufferedReader: Option<Rc<RefCell<BufferedReader>>> = BufferedReader::new(processInputStreamReader.clone());
            let mut keyValuePair: String = String::new();
            while ({ keyValuePair = jcall!(bufferedReader).readLine(); keyValuePair.clone() }).is_some() {
                let mut index: i32 = keyValuePair.find('=').map(|index| index as i32).unwrap_or(-1);
                let mut key: String = keyValuePair[(0) as usize..(index) as usize].to_string();
                let mut value: String = keyValuePair[(index + 1) as usize..].to_string();
                jcall!(environmentVariables).put(key.clone(), value.clone());
            }
            return environmentVariables.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).OSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("get()"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn getTempDir(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return match String::from("java.io.tmpdir").as_str() { "os.name" => std::env::consts::OS.to_string(), "os.arch" => std::env::consts::ARCH.to_string(), "user.home" => std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_default(), _ => String::new() };
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).OSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getTempDir()"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn getLibraryPath(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return match String::from("java.library.path").as_str() { "os.name" => std::env::consts::OS.to_string(), "os.arch" => std::env::consts::ARCH.to_string(), "user.home" => std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_default(), _ => String::new() };
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).OSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getLibraryPath()"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn getClassPath(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return match String::from("java.class.path").as_str() { "os.name" => std::env::consts::OS.to_string(), "os.arch" => std::env::consts::ARCH.to_string(), "user.home" => std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_default(), _ => String::new() };
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).OSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getClassPath()"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn updateLibraryPath(&mut self, mut libPath: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut existingPath: String = self.getLibraryPath();
            System::System::setProperty(String::from("java.library.path"), format!("{}{}", format!("{}{}", libPath, String::from(":")), existingPath));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).OSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateLibraryPath()"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn getUserHome(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return match String::from("user.home").as_str() { "os.name" => std::env::consts::OS.to_string(), "os.arch" => std::env::consts::ARCH.to_string(), "user.home" => std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_default(), _ => String::new() };
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).OSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getUserHome()"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn getUserDir(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return match String::from("user.dir").as_str() { "os.name" => std::env::consts::OS.to_string(), "os.arch" => std::env::consts::ARCH.to_string(), "user.home" => std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_default(), _ => String::new() };
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).OSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getUserDir()"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn set(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).OSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("get()"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn setVariable(&mut self, mut var: String, mut val: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut process: Option<Rc<RefCell<Process>>> = None;
            let mut operatingSystemInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> = jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance();
            let mut operatingSystems: Option<Rc<RefCell<OperatingSystems>>> = OperatingSystems::getInstance();
            let mut osName: String = jcall!(operatingSystemInterface).getName();
            let mut windowsCommand: String = format!("{}{}", format!("{}{}", format!("{}{}", String::from("cmd.exe /c set"), var), String::from("=")), val);
            if (osName.cmp(&jcall!(operatingSystems).LINUX.clone()) as i32) == 0 {
                panic!("{:?}", format!("{}: {:?}", "Exception", ()));
            } else if osName.find(&jcall!(operatingSystems).WINDOWS.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                process = jcall!(Runtime::getRuntime()).exec(windowsCommand.clone());
            } else if (osName.cmp(&jcall!(operatingSystems).SOLARIS.clone()) as i32) == 0 {
                panic!("{:?}", format!("{}: {:?}", "Exception", ()));
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", ()));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).OSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setVariable()"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
}

