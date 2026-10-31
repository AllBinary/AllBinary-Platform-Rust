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
// package org.allbinary.logic.communication.sql
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Calendar;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
// use crate::java::sql::ResultSet::ResultSet;
// use crate::java::sql::ResultSetMetaData::ResultSetMetaData;
// use crate::java::util::Calendar::Calendar;
use crate::org::allbinary::business::installer::Portion::Portion;
use crate::org::allbinary::logic::io::AbFileOutputStream::AbFileOutputStream;
use crate::org::allbinary::logic::io::BufferedLineReader::BufferedLineReader;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FileUtil::FileUtil;
use crate::org::allbinary::logic::io::file::directory::Directory::Directory;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::globals::PATH_GLOBALS::PATH_GLOBALS;
// use crate::java::io::OutputStream::OutputStream;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::SqlStrings::SqlStrings;
use super::AbSqlTable::AbSqlTable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbSqlTableUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub EXTENSION: String,
    pub END: String,
    pub SAVING_BACKUP_PATH: String,
    pub FILE_LABEL: String,
    pub METHOD_RESTORE_TABLE: String,
    pub METHOD_BACKUP_TABLE: String,
    pub METHOD_BACKUP_FILE: String,
    pub METHOD_GET_OUTPUT_STREAM: String,
    pub TABLE_CREATION_SUCCESS: String,
    pub DROPPED_SUCCESS: String,
    pub SAVING: String,
    pub APPENDING: String,
    pub TABLE_LABEL: String,
    pub BACKUP_SUCCESS: String,
    pub ERROR_CREATING: String,
    pub TOTAL_LABEL: String,
    pub SECTION_LABEL: String,
    pub DASH: String,
    pub PORTION_RESTORED: String,
    pub specialCharArray: Vec<char>,
    pub NEW_LINE: String,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub sqlStrings: Option<Rc<RefCell<SqlStrings>>>,
}

thread_local! { pub static AbSqlTableUtil_instance: Option<Rc<RefCell<AbSqlTableUtil>>> = AbSqlTableUtil::new(); }

impl AbSqlTableUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            EXTENSION: String::new(),
            END: String::new(),
            SAVING_BACKUP_PATH: String::new(),
            FILE_LABEL: String::new(),
            METHOD_RESTORE_TABLE: String::new(),
            METHOD_BACKUP_TABLE: String::new(),
            METHOD_BACKUP_FILE: String::new(),
            METHOD_GET_OUTPUT_STREAM: String::new(),
            TABLE_CREATION_SUCCESS: String::new(),
            DROPPED_SUCCESS: String::new(),
            SAVING: String::new(),
            APPENDING: String::new(),
            TABLE_LABEL: String::new(),
            BACKUP_SUCCESS: String::new(),
            ERROR_CREATING: String::new(),
            TOTAL_LABEL: String::new(),
            SECTION_LABEL: String::new(),
            DASH: String::new(),
            PORTION_RESTORED: String::new(),
            specialCharArray: Vec::new(),
            NEW_LINE: String::new(),
            stringUtil: None,
            commonStrings: None,
            commonSeps: None,
            sqlStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.EXTENSION = String::from(".adb");
        this.END = String::from("')\n");
        this.SAVING_BACKUP_PATH = String::from("Saving Backup: Path: ");
        this.FILE_LABEL = String::from(" File: ");
        this.METHOD_RESTORE_TABLE = String::from("restoreTable()");
        this.METHOD_BACKUP_TABLE = String::from("backupTable()");
        this.METHOD_BACKUP_FILE = String::from("backupFile()");
        this.METHOD_GET_OUTPUT_STREAM = String::from("getOutputStream()");
        this.TABLE_CREATION_SUCCESS = String::from("Table Creation Successful: ");
        this.DROPPED_SUCCESS = String::from(" Dropped Successfully");
        this.SAVING = String::from("Saving: ");
        this.APPENDING = String::from("Appending: ");
        this.TABLE_LABEL = String::from("Table: ");
        this.BACKUP_SUCCESS = String::from(" Backup Success");
        this.ERROR_CREATING = String::from("Error Creating: ");
        this.TOTAL_LABEL = String::from(" Total: ");
        this.SECTION_LABEL = String::from(" Section: ");
        this.DASH = String::from(" - ");
        this.PORTION_RESTORED = String::from(" Portion Restored");
        this.specialCharArray = vec!['\n', '\f', '\r'];
        this.NEW_LINE = String::from("\\n");
        this.stringUtil = StringUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.sqlStrings = SqlStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AbSqlTableUtil>>> {
        return AbSqlTableUtil_instance.with(Clone::clone).clone();
    }
    

    // synchronized
    pub fn getOutputStream(&mut self, mut backupPath: String, mut tableName: String) -> Option<Rc<RefCell<OutputStream>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut fileName: String = format!("{}{}", tableName, self.EXTENSION);
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", self.SAVING, tableName), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_OUTPUT_STREAM.clone());
            }
            let mut backupFilePath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(backupPath.clone(), fileName.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            let mut backupFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(backupFilePath.clone());
            if jcall!(backupFile).exists() {
                self.backupFile(backupFilePath.clone(), backupPath.clone(), tableName.clone());
                jcall!(backupFile).delete();
            }
            jcall!(backupFile).createNewFile();
            let mut outputStream: Option<Rc<RefCell<OutputStream>>> = AbFileOutputStream::new(backupFile.clone());
            return outputStream.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Create File"), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_GET_OUTPUT_STREAM.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // synchronized
    pub fn backupFile(&mut self, mut path: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut backupPath: String, mut tableName: String) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut timeLong: Option<i64> = Some((jcall!(calendar).getTimeInMillis()) as i64);
            let mut time: String = (timeLong).expect("NullPointerException").to_string();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut fileName: String = format!("{}{}", tableName, self.EXTENSION);
            jcall!(stringBuffer).append(backupPath.clone());
            jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
            jcall!(stringBuffer).append(time.clone());
            let mut backupAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(self.SAVING_BACKUP_PATH.clone());
                jcall!(stringBuffer).append(jcall!(backupAbPath).toFileSystemString());
                jcall!(stringBuffer).append(self.FILE_LABEL.clone());
                jcall!(stringBuffer).append(fileName.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_BACKUP_FILE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            Directory::create(backupAbPath.clone());
            let mut backupFileBak: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(jcall!(backupAbPath).toFileSystemString());
            jcall!(backupFileBak).createNewFile();
            jcall!(FileUtil::getInstance()).copy(path.clone(), backupAbPath.clone());
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_BACKUP_FILE.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn convertNewLines(&mut self, mut value: String) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut index: i32 = 0;
        let mut lastIndex: i32 = 0;
        while index < (value.chars().count() as i32) {
            index = jcall!(value).indexOf(self.specialCharArray[(0) as usize].clone(), lastIndex.clone());
            if index != -1 {
                let mut nextLine: String = value[(lastIndex) as usize..(index - 1) as usize].to_string();
                jcall!(stringBuffer).append(nextLine.clone());
                jcall!(stringBuffer).append(self.NEW_LINE.clone());
                lastIndex = index + 1;
            } else {
                break;
            }
        }
        if lastIndex < (value.chars().count() as i32) {
            jcall!(stringBuffer).append(value[(lastIndex) as usize..((value.chars().count() as i32)) as usize].to_string());
        }
        return jcall!(stringBuffer).toString();
    }
    

    // synchronized
    pub fn backupTable(&mut self, mut abSqlTable: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn>>>) -> String {
        let mut tableName: String = jcall!(abSqlTable).getTableName();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut count: i32 = 0;
            let mut sqlStatement: String = format!("{}{}", jcall!(self.sqlStrings).SELECT_ALL_FROM, tableName);
            let mut path: String = format!("{}{}", jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).URLGLOBALS).getMainPath(), jcall!(PATH_GLOBALS::getInstance()).BACKUP_PATH);
            if !Directory::create(AbPath::new(path.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)) {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", self.ERROR_CREATING, path), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_BACKUP_TABLE.clone());
                }
            }
            let mut rset: Option<Rc<RefCell<ResultSet>>> = jcall!(abSqlTable).executeSQLStatement(sqlStatement.clone());
            let mut rsmd: Option<Rc<RefCell<ResultSetMetaData>>> = jcall!(rset).getMetaData();
            let mut colNum: i32 = jcall!(rsmd).getColumnCount();
            let mut QUERY_START: String = jcall!(String::new()).append(jcall!(self.sqlStrings).INSERT_INTO.clone()).append(tableName.clone()).append(jcall!(self.sqlStrings).VALUES.clone()).toString();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut outputStream: Option<Rc<RefCell<OutputStream>>> = self.getOutputStream(path.clone(), tableName.clone());
            while jcall!(rset).next() {
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(QUERY_START.clone());
                for i in (1..colNum) {
                    let mut value: String = jcall!(rset).getString(i.clone());
                    jcall!(stringBuffer).append(self.convertNewLines(value.clone()));
                    jcall!(stringBuffer).append(jcall!(self.sqlStrings).SINGLE_QUOTE_COMMA_SEP.clone());
                }
                jcall!(stringBuffer).append(jcall!(rset).getString(colNum.clone()));
                jcall!(stringBuffer).append(self.END.clone());
                let mut sqlStatementLine: String = jcall!(stringBuffer).toString();
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", self.APPENDING, sqlStatementLine), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_BACKUP_TABLE.clone());
                }
                jcall!(outputStream).write(sqlStatementLine.bytes().map(|value| value as i8).collect::<Vec<i8>>());
            }
            jcall!(StreamUtil::getInstance()).close(outputStream.clone());
            return format!("{}{}", format!("{}{}", self.TABLE_LABEL, tableName), self.BACKUP_SUCCESS);
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Backup Table Failed\nSQL Statement"), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_BACKUP_TABLE.clone(), crate::java_into_object(e.clone()));
            }
            return format!("{}{}", format!("{}{}", self.TABLE_LABEL, tableName), String::from(" Backup Failed"));
            }
        }
    }
    

    // synchronized
    pub fn restoreTable(&mut self, mut abSqlTable: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn>>>, mut portion: Option<Rc<RefCell<Portion>>>) -> String {
        let mut tableName: String = jcall!(abSqlTable).getTableName();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut path: String = format!("{}{}", jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).URLGLOBALS).getMainPath(), jcall!(PATH_GLOBALS::getInstance()).BACKUP_PATH);
            let mut current: i32 = ((jcall!(portion).getCurrent()).unwrap() as i32);
            if current == 0 {
                if Directory::create(AbPath::new(path.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)) {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                        jcall!(self.logUtil).putF(format!("{}{}", self.ERROR_CREATING, path), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_RESTORE_TABLE.clone());
                    }
                }
            }
            let mut backupFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(path.clone(), format!("{}{}", tableName, self.EXTENSION));
            let mut bufferedLineReader: Option<Rc<RefCell<BufferedLineReader>>> = BufferedLineReader::new(backupFile.clone());
            let mut size: i64 = jcall!(bufferedLineReader).getSize();
            let mut section: i64 = size / ((((jcall!(portion).getTotal()).unwrap() as i32)) as i64) + 1_i64;
            let mut start: i64 = section * ((current) as i64);
            let mut end: i64 = start + section;
            if end > size {
                end = size.clone();
            }
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(self.TOTAL_LABEL.clone());
            jcall!(stringBuffer).append(size.clone());
            jcall!(stringBuffer).append(self.SECTION_LABEL.clone());
            jcall!(stringBuffer).append(start.clone());
            jcall!(stringBuffer).append(self.DASH.clone());
            jcall!(stringBuffer).append(end.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_RESTORE_TABLE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            jcall!(bufferedLineReader).readUpToLines(start.clone());
            let mut line: String = jcall!(self.stringUtil).EMPTY_STRING.clone();
            while ((jcall!(bufferedLineReader).getCurrent()) as i64) < end && ({ line = jcall!(bufferedLineReader).readLine(); line.clone() }).is_some() {
                if (line.chars().count() as i32) > 1 {
                    jcall!(abSqlTable).executeSQLStatement(line.clone());
                }
            }
            jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
            jcall!(stringBuffer).append(self.TABLE_LABEL.clone());
            jcall!(stringBuffer).append(tableName.clone());
            jcall!(stringBuffer).append(self.PORTION_RESTORED.clone());
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Restore Table Failed\nSQL Statement"), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD_RESTORE_TABLE.clone(), crate::java_into_object(e.clone()));
            }
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(self.TABLE_LABEL.clone());
            jcall!(stringBuffer).append(tableName.clone());
            jcall!(stringBuffer).append(String::from(" Restoration Failed"));
            return jcall!(stringBuffer).toString();
            }
        }
    }
}

