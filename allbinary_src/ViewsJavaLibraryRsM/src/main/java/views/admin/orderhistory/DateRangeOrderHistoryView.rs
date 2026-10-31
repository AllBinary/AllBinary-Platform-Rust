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
// package views.admin.orderhistory
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Calendar;
use super::*;
// use crate::java::util::Calendar::Calendar;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistory::OrderHistory;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderHistoryEntityFactory::OrderHistoryEntityFactory;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::globals::GLOBALS2::GLOBALS2;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::ValidationComponentInterface::ValidationComponentInterface;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::time::TimeUtil::TimeUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DateRangeOrderHistoryView {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub VIEWALLORDERS: String,
    pub VIEWORDERSINLASTHOUR: String,
    pub VIEWORDERSINLASTDAY: String,
    pub VIEWORDERSINLASTWEEK: String,
    pub VIEWORDERSINLAST30DAYS: String,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub shipped: String,
    pub partiallyShipped: String,
    pub processing: String,
    pub preprocessing: String,
    pub cancelled: String,
    pub dateType: String,
    pub toDate: String,
    pub fromDate: String,
    pub fromYear: String,
    pub fromMonth: String,
    pub fromDay: String,
    pub fromHour: String,
    pub toYear: String,
    pub toMonth: String,
    pub toDay: String,
    pub toHour: String,
    pub command: String,
    pub ON: String,
    pub MAXLEN: i32,
}

// impl ValidationComponentInterface for DateRangeOrderHistoryView with the methods of the trait
impl DomNodeInterface for DateRangeOrderHistoryView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl DateRangeOrderHistoryView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            VIEWALLORDERS: String::new(),
            VIEWORDERSINLASTHOUR: String::new(),
            VIEWORDERSINLASTDAY: String::new(),
            VIEWORDERSINLASTWEEK: String::new(),
            VIEWORDERSINLAST30DAYS: String::new(),
            request: None,
            shipped: String::new(),
            partiallyShipped: String::new(),
            processing: String::new(),
            preprocessing: String::new(),
            cancelled: String::new(),
            dateType: String::new(),
            toDate: String::new(),
            fromDate: String::new(),
            fromYear: String::new(),
            fromMonth: String::new(),
            fromDay: String::new(),
            fromHour: String::new(),
            toYear: String::new(),
            toMonth: String::new(),
            toDay: String::new(),
            toHour: String::new(),
            command: String::new(),
            ON: String::new(),
            MAXLEN: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.VIEWALLORDERS = String::from("View All");
        this.VIEWORDERSINLASTHOUR = String::from("Last Hour");
        this.VIEWORDERSINLASTDAY = String::from("Last Day");
        this.VIEWORDERSINLASTWEEK = String::from("Last Week");
        this.VIEWORDERSINLAST30DAYS = String::from("30 Days");
        this.ON = String::from("on");
        this.MAXLEN = 15;
        // super(...) is in the struct literal above
        this.request = (jcall!(this.getPageContext()).getRequest()).clone() /* as HttpServletRequest */;
        this.getFormData();
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn getFormData(&mut self) {
        self.dateType = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_DATETYPE.with(Clone::clone).clone());
        self.preprocessing = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PREPROCESSINGNAME.with(Clone::clone).clone());
        self.shipped = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDNAME.with(Clone::clone).clone());
        self.partiallyShipped = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PARTIALLYSHIPPEDNAME.with(Clone::clone).clone());
        self.processing = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PROCESSINGNAME.with(Clone::clone).clone());
        self.cancelled = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELLEDNAME.with(Clone::clone).clone());
        self.fromYear = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_FROMYEAR.with(Clone::clone).clone());
        self.fromMonth = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_FROMMONTH.with(Clone::clone).clone());
        self.fromDay = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_FROMDAY.with(Clone::clone).clone());
        self.fromHour = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_FROMHOUR.with(Clone::clone).clone());
        self.toYear = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TOYEAR.with(Clone::clone).clone());
        self.toMonth = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TOMONTH.with(Clone::clone).clone());
        self.toDay = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TODAY.with(Clone::clone).clone());
        self.toHour = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TOHOUR.with(Clone::clone).clone());
        self.command = jcall!(self.request).getParameter(crate::org::allbinary::globals::GLOBALS2::GLOBALS2_ADMINCOMMAND.with(Clone::clone).clone());
    }
    

    pub fn addDomNodeInterfaces(&mut self) {
        self.addDomNodeInterface((self).clone() /* as DomNodeInterface */);
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.addDomNodeInterfaces();
            return self.base.view();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Attempt to View orders in (fromDate="));
                jcall!(stringBuffer).append(self.fromDate.clone());
                jcall!(stringBuffer).append(String::from(",toDate="));
                jcall!(stringBuffer).append(self.toDate.clone());
                jcall!(stringBuffer).append(String::from(") and status"));
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ORDERS.with(Clone::clone).clone());
            if true && (self.preprocessing.cmp(&self.ON.clone()) as i32) == 0 {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(String::from("Adding Preprocessing Orders"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"));
                }
                let mut orderHistoryVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(OrderHistoryEntityFactory::getInstance()).getOrders(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PREPROCESSING.with(Clone::clone).clone(), self.fromDate.clone(), self.toDate.clone());
                let mut size: i32 = jcall!(orderHistoryVector).size();
                for index in (0..size) {
                    let mut orderHistory: Option<Rc<RefCell<OrderHistory>>> = { let java_object = (jcall!(orderHistoryVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<OrderHistory>>>>().expect("ClassCastException").clone()) };
                    jcall!(node).appendChild(jcall!(orderHistory).toXmlNode(document.clone()));
                }
            }
            if true && (self.shipped.cmp(&self.ON.clone()) as i32) == 0 {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(String::from("Adding Shipped Orders"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"));
                }
                let mut orderHistoryVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(OrderHistoryEntityFactory::getInstance()).getOrders(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPED.with(Clone::clone).clone(), self.fromDate.clone(), self.toDate.clone());
                let mut size: i32 = jcall!(orderHistoryVector).size();
                for index in (0..size) {
                    let mut orderHistory: Option<Rc<RefCell<OrderHistory>>> = { let java_object = (jcall!(orderHistoryVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<OrderHistory>>>>().expect("ClassCastException").clone()) };
                    jcall!(node).appendChild(jcall!(orderHistory).toXmlNode(document.clone()));
                }
            }
            if true && (self.partiallyShipped.cmp(&self.ON.clone()) as i32) == 0 {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(String::from("Adding Partially Shipped Orders"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toDomNode"));
                }
                let mut orderHistoryVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(OrderHistoryEntityFactory::getInstance()).getOrders(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PARTIALLYSHIPPED.with(Clone::clone).clone(), self.fromDate.clone(), self.toDate.clone());
                let mut size: i32 = jcall!(orderHistoryVector).size();
                for index in (0..size) {
                    let mut orderHistory: Option<Rc<RefCell<OrderHistory>>> = { let java_object = (jcall!(orderHistoryVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<OrderHistory>>>>().expect("ClassCastException").clone()) };
                    jcall!(node).appendChild(jcall!(orderHistory).toXmlNode(document.clone()));
                }
            }
            if true && (self.processing.cmp(&self.ON.clone()) as i32) == 0 {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(String::from("Adding Processing Orders"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"));
                }
                let mut orderHistoryVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(OrderHistoryEntityFactory::getInstance()).getOrders(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PROCESSING.with(Clone::clone).clone(), self.fromDate.clone(), self.toDate.clone());
                let mut size: i32 = jcall!(orderHistoryVector).size();
                for index in (0..size) {
                    let mut orderHistory: Option<Rc<RefCell<OrderHistory>>> = { let java_object = (jcall!(orderHistoryVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<OrderHistory>>>>().expect("ClassCastException").clone()) };
                    jcall!(node).appendChild(jcall!(orderHistory).toXmlNode(document.clone()));
                }
            }
            if true && (self.cancelled.cmp(&self.ON.clone()) as i32) == 0 {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(String::from("Adding Cancelled Orders"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view"));
                }
                let mut orderHistoryVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(OrderHistoryEntityFactory::getInstance()).getOrders(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELLED.with(Clone::clone).clone(), self.fromDate.clone(), self.toDate.clone());
                let mut size: i32 = jcall!(orderHistoryVector).size();
                for index in (0..size) {
                    let mut orderHistory: Option<Rc<RefCell<OrderHistory>>> = { let java_object = (jcall!(orderHistoryVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<OrderHistory>>>>().expect("ClassCastException").clone()) };
                    jcall!(node).appendChild(jcall!(orderHistory).toXmlNode(document.clone()));
                }
            }
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PREPROCESSINGNAME.with(Clone::clone).clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PREPROCESSING.with(Clone::clone).clone()));
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PROCESSINGNAME.with(Clone::clone).clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PROCESSING.with(Clone::clone).clone()));
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELLEDNAME.with(Clone::clone).clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELLED.with(Clone::clone).clone()));
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PARTIALLYSHIPPEDNAME.with(Clone::clone).clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PARTIALLYSHIPPED.with(Clone::clone).clone()));
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDNAME.with(Clone::clone).clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPED.with(Clone::clone).clone()));
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::globals::GLOBALS2::GLOBALS2_VIEWNAME.with(Clone::clone).clone(), crate::org::allbinary::globals::GLOBALS2::GLOBALS2_VIEW.with(Clone::clone).clone()));
            return node.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).XSLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut isValid: Option<bool> = true;
            if false || ((self.dateType.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TYPELONG.with(Clone::clone).clone()) as i32) != 0 && (self.dateType.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TYPECAESAR.with(Clone::clone).clone()) as i32) != 0) {
                isValid = false;
            }
            if true {
                if (self.dateType.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TYPELONG.with(Clone::clone).clone()) as i32) == 0 {
                    let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
                    let mut currentTime: i64 = jcall!(calendar).getTimeInMillis();
                    let mut rangeDate: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
                    if (self.command.cmp(&self.VIEWORDERSINLASTHOUR.clone()) as i32) == 0 {
                        let mut oneHour: i64 = 60_i64 * 60_i64 * 1000_i64;
                        rangeDate = jcall!(Some((oneHour.clone()) as i64)).toString();
                    } else if (self.command.cmp(&self.VIEWORDERSINLASTDAY.clone()) as i32) == 0 {
                        let mut oneDay: i64 = 24_i64 * 60_i64 * 60_i64 * 1000_i64;
                        rangeDate = jcall!(Some((oneDay.clone()) as i64)).toString();
                    } else if (self.command.cmp(&self.VIEWORDERSINLASTWEEK.clone()) as i32) == 0 {
                        let mut oneWeek: i64 = 7_i64 * 24_i64 * 60_i64 * 60_i64 * 1000_i64;
                        rangeDate = jcall!(Some((oneWeek.clone()) as i64)).toString();
                    } else if (self.command.cmp(&self.VIEWORDERSINLAST30DAYS.clone()) as i32) == 0 {
                        let mut thirtyDays: i64 = 30_i64 * 24_i64 * 60_i64 * 60_i64 * 1000_i64;
                        rangeDate = jcall!(Some((thirtyDays.clone()) as i64)).toString();
                    } else if (self.command.cmp(&self.VIEWALLORDERS.clone()) as i32) == 0 {
                        rangeDate = jcall!(Some((currentTime.clone()) as i64)).toString();
                    }
                    let mut rangeDateLong: i64 = ((Some((rangeDate.clone()) as i64)).unwrap() as i64);
                    self.fromDate = jcall!(Some((currentTime - rangeDateLong) as i64)).toString();
                    let mut time: String = jcall!(Some((currentTime.clone()) as i64)).toString().clone();
                    self.toDate = time.clone();
                } else if (self.dateType.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TYPECAESAR.with(Clone::clone).clone()) as i32) == 0 {
                    let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
                    jcall!(TimeUtil::getInstance()).setCalendar(calendar.clone(), self.fromYear.clone(), self.fromMonth.clone(), self.fromDay.clone(), self.fromHour.clone());
                    self.fromDate = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString();
                    jcall!(TimeUtil::getInstance()).setCalendar(calendar.clone(), self.toYear.clone(), self.toMonth.clone(), self.toDay.clone(), self.toHour.clone());
                    self.toDate = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString();
                }
                if jcall!(StringValidationUtil::getInstance()).isEmpty(self.toDate.clone()) || (self.toDate.chars().count() as i32) > self.MAXLEN {
                    isValid = false;
                }
                if jcall!(StringValidationUtil::getInstance()).isEmpty(self.fromDate.clone()) || (self.fromDate.chars().count() as i32) > self.MAXLEN {
                    isValid = false;
                }
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Attempt to View orders in (fromDate="));
                jcall!(stringBuffer).append(self.fromDate.clone());
                jcall!(stringBuffer).append(String::from(",toDate="));
                jcall!(stringBuffer).append(self.toDate.clone());
                jcall!(stringBuffer).append(String::from(")"));
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return isValid.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).put(String::from("Exception in validation"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    // throws Exception
    pub fn validationInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut result: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            if false || ((self.dateType.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TYPELONG.with(Clone::clone).clone()) as i32) != 0 && (self.dateType.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TYPECAESAR.with(Clone::clone).clone()) as i32) != 0) {
                jcall!(result).append(String::from("DATETYPE not recognized"));
            }
            if jcall!(StringValidationUtil::getInstance()).isEmpty(self.toDate.clone()) || (self.toDate.chars().count() as i32) > self.MAXLEN {
                jcall!(result).append(String::from("Invalid To Date"));
            }
            if jcall!(StringValidationUtil::getInstance()).isEmpty(self.fromDate.clone()) || (self.fromDate.chars().count() as i32) > self.MAXLEN {
                jcall!(result).append(String::from("Invalid From Date"));
            }
            return jcall!(result).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Getting Validation Info");
            }
        }
    }
}
impl std::ops::Deref for DateRangeOrderHistoryView {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DateRangeOrderHistoryView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for DateRangeOrderHistoryView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for DateRangeOrderHistoryView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}


