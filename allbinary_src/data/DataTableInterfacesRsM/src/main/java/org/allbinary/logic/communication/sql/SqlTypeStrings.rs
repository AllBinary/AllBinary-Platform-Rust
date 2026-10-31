// Generated Code Do Not Modify
// package org.allbinary.logic.communication.sql
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SqlTypeStrings {
    pub CHAR_COLUMN: String,
    pub INT_COLUMN: String,
    pub MAX_SIZE: String,
    pub SIXTY_SIZE: String,
    pub NOT_NULL: String,
    pub ONE_SIZE: String,
    pub TWO_SIZE: String,
    pub THREE_SIZE: String,
    pub SIX_SIZE: String,
    pub ELEVEN_SIZE: String,
    pub TWELVE_SIZE: String,
    pub ONE_KB_CHAR_COLUMN: String,
    pub MAX_CHAR_COLUMN: String,
    pub MAX_CHAR_COLUMN_NOT_NULL: String,
    pub SIXTY_CHAR_COLUMN_NOT_NULL: String,
    pub ONE_CHAR_COLUMN_NOT_NULL: String,
    pub SIX_CHAR_COLUMN_NOT_NULL: String,
    pub TWELVE_CHAR_COLUMN_NOT_NULL: String,
    pub TWO_INT_NOT_NULL: String,
    pub THREE_INT_NOT_NULL: String,
    pub FOUR_INT_NOT_NULL: String,
    pub MAX_INT_NOT_NULL: String,
    pub MAX_INT_UNSIGNED: String,
    pub MAX_INT_UNSIGNED_NOT_NULL: String,
    pub MAX_BIG_INT_NOT_NULL: String,
    pub MAX_BIG_INT_UNSIGNED: String,
    pub MAX_BIG_INT_UNSIGNED_NOT_NULL: String,
    pub MAX_BIG_INT_UNSIGNED_AUTO_INCREMENT_NOT_NULL: String,
    pub LONG_BLOB: String,
    pub BLOB: String,
    pub BLOB_NOT_NULL: String,
}

thread_local! { pub static SqlTypeStrings_instance: Option<Rc<RefCell<SqlTypeStrings>>> = SqlTypeStrings::new(); }

impl SqlTypeStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            CHAR_COLUMN: String::new(),
            INT_COLUMN: String::new(),
            MAX_SIZE: String::new(),
            SIXTY_SIZE: String::new(),
            NOT_NULL: String::new(),
            ONE_SIZE: String::new(),
            TWO_SIZE: String::new(),
            THREE_SIZE: String::new(),
            SIX_SIZE: String::new(),
            ELEVEN_SIZE: String::new(),
            TWELVE_SIZE: String::new(),
            ONE_KB_CHAR_COLUMN: String::new(),
            MAX_CHAR_COLUMN: String::new(),
            MAX_CHAR_COLUMN_NOT_NULL: String::new(),
            SIXTY_CHAR_COLUMN_NOT_NULL: String::new(),
            ONE_CHAR_COLUMN_NOT_NULL: String::new(),
            SIX_CHAR_COLUMN_NOT_NULL: String::new(),
            TWELVE_CHAR_COLUMN_NOT_NULL: String::new(),
            TWO_INT_NOT_NULL: String::new(),
            THREE_INT_NOT_NULL: String::new(),
            FOUR_INT_NOT_NULL: String::new(),
            MAX_INT_NOT_NULL: String::new(),
            MAX_INT_UNSIGNED: String::new(),
            MAX_INT_UNSIGNED_NOT_NULL: String::new(),
            MAX_BIG_INT_NOT_NULL: String::new(),
            MAX_BIG_INT_UNSIGNED: String::new(),
            MAX_BIG_INT_UNSIGNED_NOT_NULL: String::new(),
            MAX_BIG_INT_UNSIGNED_AUTO_INCREMENT_NOT_NULL: String::new(),
            LONG_BLOB: String::new(),
            BLOB: String::new(),
            BLOB_NOT_NULL: String::new(),
        };
        this.CHAR_COLUMN = String::from("VARCHAR");
        this.INT_COLUMN = String::from("VARCHAR");
        this.MAX_SIZE = String::from("(255)");
        this.SIXTY_SIZE = String::from("(60)");
        this.NOT_NULL = String::from("NOT NULL");
        this.ONE_SIZE = String::from("(1)");
        this.TWO_SIZE = String::from("(2)");
        this.THREE_SIZE = String::from("(3)");
        this.SIX_SIZE = String::from("(6)");
        this.ELEVEN_SIZE = String::from("(11)");
        this.TWELVE_SIZE = String::from("(12)");
        this.ONE_KB_CHAR_COLUMN = jcall!(StringMaker::new()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(this.CHAR_COLUMN.clone()).append(String::from("(1024)")).append(jcall!(CommonSeps::getInstance()).COMMA_SEP.clone()).toString();
        this.MAX_CHAR_COLUMN = jcall!(StringMaker::new()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(this.CHAR_COLUMN.clone()).append(this.MAX_SIZE.clone()).append(jcall!(CommonSeps::getInstance()).COMMA_SEP.clone()).toString();
        this.MAX_CHAR_COLUMN_NOT_NULL = jcall!(StringMaker::new()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(this.CHAR_COLUMN.clone()).append(this.MAX_SIZE.clone()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(this.NOT_NULL.clone()).append(jcall!(CommonSeps::getInstance()).COMMA_SEP.clone()).toString();
        this.SIXTY_CHAR_COLUMN_NOT_NULL = jcall!(StringMaker::new()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(this.CHAR_COLUMN.clone()).append(this.SIXTY_SIZE.clone()).append(this.NOT_NULL.clone()).append(jcall!(CommonSeps::getInstance()).COMMA_SEP.clone()).toString();
        this.ONE_CHAR_COLUMN_NOT_NULL = jcall!(StringMaker::new()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(this.CHAR_COLUMN.clone()).append(this.ONE_SIZE.clone()).append(this.NOT_NULL.clone()).append(jcall!(CommonSeps::getInstance()).COMMA_SEP.clone()).toString();
        this.SIX_CHAR_COLUMN_NOT_NULL = jcall!(StringMaker::new()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(this.CHAR_COLUMN.clone()).append(this.SIX_SIZE.clone()).append(this.NOT_NULL.clone()).append(jcall!(CommonSeps::getInstance()).COMMA_SEP.clone()).toString();
        this.TWELVE_CHAR_COLUMN_NOT_NULL = jcall!(StringMaker::new()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(this.CHAR_COLUMN.clone()).append(this.TWELVE_SIZE.clone()).append(this.NOT_NULL.clone()).append(jcall!(CommonSeps::getInstance()).COMMA_SEP.clone()).toString();
        this.TWO_INT_NOT_NULL = String::from(" INT(2) NOT NULL, ");
        this.THREE_INT_NOT_NULL = String::from(" INT(3) NOT NULL, ");
        this.FOUR_INT_NOT_NULL = String::from(" INT(4) NOT NULL, ");
        this.MAX_INT_NOT_NULL = String::from(" INT(11) NOT NULL, ");
        this.MAX_INT_UNSIGNED = String::from(" INT(11) UNSIGNED, ");
        this.MAX_INT_UNSIGNED_NOT_NULL = String::from(" INT(11) UNSIGNED NOT NULL, ");
        this.MAX_BIG_INT_NOT_NULL = String::from(" BIGINT(19) NOT NULL, ");
        this.MAX_BIG_INT_UNSIGNED = String::from(" BIGINT(19) UNSIGNED, ");
        this.MAX_BIG_INT_UNSIGNED_NOT_NULL = String::from(" BIGINT(19) UNSIGNED NOT NULL, ");
        this.MAX_BIG_INT_UNSIGNED_AUTO_INCREMENT_NOT_NULL = String::from(" BIGINT(19) UNSIGNED AUTO_INCREMENT NOT NULL,");
        this.LONG_BLOB = String::from(" LONGBLOB, ");
        this.BLOB = String::from(" BLOB, ");
        this.BLOB_NOT_NULL = String::from(" BLOB NOT NULL, ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<SqlTypeStrings>>> {
        return SqlTypeStrings_instance.with(Clone::clone).clone();
    }
}

