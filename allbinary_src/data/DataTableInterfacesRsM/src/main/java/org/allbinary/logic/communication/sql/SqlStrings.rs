// Generated Code Do Not Modify
// package org.allbinary.logic.communication.sql
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SqlStrings {
    pub CREATE_DATABASE: String,
    pub DROP_TABLE: String,
    pub CREATE_TABLE: String,
    pub START: String,
    pub SELECT: String,
    pub FROM: String,
    pub SELECT_ALL: String,
    pub WHERE: String,
    pub AND: String,
    pub UPDATE: String,
    pub SET: String,
    pub SELECT_ALL_FROM: String,
    pub DELETE: String,
    pub LIKE_QUOTE: String,
    pub INSERT_INTO: String,
    pub VALUES: String,
    pub ORDER_BY: String,
    pub COUNT: String,
    pub ASC: String,
    pub DESC: String,
    pub PRIMARY_KEY: String,
    pub END: String,
    pub EQUAL_QUOTE: String,
    pub CLOSE_QUOTE: String,
    pub EQUAL_QUOTE_NO_SPACE: String,
    pub ESCAPE: String,
    pub DOUBLE_ESCAPE: String,
    pub MORE_THAN_QUOTE: String,
    pub LESS_THAN_QUOTE: String,
    pub SINGLE_QUOTE_COMMA_SEP: String,
    pub ID: String,
    pub SQL_STATEMENT_LABEL: String,
    pub COLUMN_VALUE: String,
    pub CREATE_RETURN: String,
    pub FIELD_VALUE: String,
}

thread_local! { pub static SqlStrings_instance: Option<Rc<RefCell<SqlStrings>>> = SqlStrings::new(); }

impl SqlStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            CREATE_DATABASE: String::new(),
            DROP_TABLE: String::new(),
            CREATE_TABLE: String::new(),
            START: String::new(),
            SELECT: String::new(),
            FROM: String::new(),
            SELECT_ALL: String::new(),
            WHERE: String::new(),
            AND: String::new(),
            UPDATE: String::new(),
            SET: String::new(),
            SELECT_ALL_FROM: String::new(),
            DELETE: String::new(),
            LIKE_QUOTE: String::new(),
            INSERT_INTO: String::new(),
            VALUES: String::new(),
            ORDER_BY: String::new(),
            COUNT: String::new(),
            ASC: String::new(),
            DESC: String::new(),
            PRIMARY_KEY: String::new(),
            END: String::new(),
            EQUAL_QUOTE: String::new(),
            CLOSE_QUOTE: String::new(),
            EQUAL_QUOTE_NO_SPACE: String::new(),
            ESCAPE: String::new(),
            DOUBLE_ESCAPE: String::new(),
            MORE_THAN_QUOTE: String::new(),
            LESS_THAN_QUOTE: String::new(),
            SINGLE_QUOTE_COMMA_SEP: String::new(),
            ID: String::new(),
            SQL_STATEMENT_LABEL: String::new(),
            COLUMN_VALUE: String::new(),
            CREATE_RETURN: String::new(),
            FIELD_VALUE: String::new(),
        };
        this.CREATE_DATABASE = String::from("CREATE DATABASE ");
        this.DROP_TABLE = String::from("DROP TABLE ");
        this.CREATE_TABLE = String::from("CREATE TABLE ");
        this.START = String::from(" (");
        this.SELECT = String::from("SELECT ");
        this.FROM = String::from(" FROM ");
        this.SELECT_ALL = String::from("SELECT *");
        this.WHERE = String::from(" WHERE ");
        this.AND = String::from(" AND ");
        this.UPDATE = String::from("UPDATE ");
        this.SET = String::from(" SET ");
        this.SELECT_ALL_FROM = format!("{}{}", this.SELECT_ALL, this.FROM);
        this.DELETE = String::from("DELETE");
        this.LIKE_QUOTE = String::from(" LIKE \"");
        this.INSERT_INTO = String::from("INSERT INTO ");
        this.VALUES = String::from(" VALUES ('");
        this.ORDER_BY = String::from("ORDER BY");
        this.COUNT = String::from("COUNT(");
        this.ASC = String::from("ASC");
        this.DESC = String::from("DESC");
        this.PRIMARY_KEY = String::from("PRIMARY KEY(");
        this.END = String::from(") )");
        this.EQUAL_QUOTE = String::from(" = \"");
        this.CLOSE_QUOTE = jcall!(CommonSeps::getInstance()).QUOTE.clone();
        this.EQUAL_QUOTE_NO_SPACE = String::from("=\"");
        this.ESCAPE = String::from("\\");
        this.DOUBLE_ESCAPE = String::from("\\\\");
        this.MORE_THAN_QUOTE = String::from(" > \"");
        this.LESS_THAN_QUOTE = String::from(" < \"");
        this.SINGLE_QUOTE_COMMA_SEP = String::from("','");
        this.ID = String::from("ID");
        this.SQL_STATEMENT_LABEL = String::from("SQL Statement: ");
        this.COLUMN_VALUE = String::from("\nColumn Value: ");
        this.CREATE_RETURN = String::from(" Created Successfully");
        this.FIELD_VALUE = String::from("\nField Value: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<SqlStrings>>> {
        return SqlStrings_instance.with(Clone::clone).clone();
    }
}

