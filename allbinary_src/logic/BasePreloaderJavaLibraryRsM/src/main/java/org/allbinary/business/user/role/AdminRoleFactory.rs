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
// package org.allbinary.business.user.role
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::BasicUserRoleFactory::BasicUserRoleFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdminRoleFactory {
    pub adminRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub storeAdminRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub storeManagementRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub productManagementRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub reviewManagementRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub customerManagementRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub workflowManagementRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub orderManagementRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub adjusterManagementRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub shippingManagementRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub webManagementRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static AdminRoleFactory_instance: Option<Rc<RefCell<AdminRoleFactory>>> = AdminRoleFactory::new(); }

impl AdminRoleFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<AdminRoleFactory>>> {
        return AdminRoleFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            adminRoles: None,
            storeAdminRoles: None,
            storeManagementRoles: None,
            productManagementRoles: None,
            reviewManagementRoles: None,
            customerManagementRoles: None,
            workflowManagementRoles: None,
            orderManagementRoles: None,
            adjusterManagementRoles: None,
            shippingManagementRoles: None,
            webManagementRoles: None,
        };
        this.adminRoles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.storeAdminRoles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.storeManagementRoles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.productManagementRoles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.reviewManagementRoles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.customerManagementRoles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.workflowManagementRoles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.orderManagementRoles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.adjusterManagementRoles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.shippingManagementRoles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.webManagementRoles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut basicUserRoleFactory: Option<Rc<RefCell<BasicUserRoleFactory>>> = BasicUserRoleFactory::getInstance();
        jcall!(this.adminRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).ADMINISTRATOR.clone()));
        jcall!(this.storeAdminRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).ADMINISTRATOR.clone()));
        jcall!(this.storeAdminRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).STOREMANAGER.clone()));
        jcall!(this.storeManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).ADMINISTRATOR.clone()));
        jcall!(this.storeManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).STOREMANAGER.clone()));
        jcall!(this.storeManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).CUSTOMERMANAGER.clone()));
        jcall!(this.storeManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).ADJUSTER.clone()));
        jcall!(this.storeManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).PRODUCTMANAGER.clone()));
        jcall!(this.storeManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).REVIEWER.clone()));
        jcall!(this.storeManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).SHIPPING.clone()));
        jcall!(this.productManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).ADMINISTRATOR.clone()));
        jcall!(this.productManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).STOREMANAGER.clone()));
        jcall!(this.productManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).PRODUCTMANAGER.clone()));
        jcall!(this.reviewManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).ADMINISTRATOR.clone()));
        jcall!(this.reviewManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).STOREMANAGER.clone()));
        jcall!(this.reviewManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).REVIEWER.clone()));
        jcall!(this.customerManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).ADMINISTRATOR.clone()));
        jcall!(this.customerManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).STOREMANAGER.clone()));
        jcall!(this.customerManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).CUSTOMERMANAGER.clone()));
        jcall!(this.workflowManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).ADMINISTRATOR.clone()));
        jcall!(this.workflowManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).STOREMANAGER.clone()));
        jcall!(this.workflowManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).WORKFLOWEDITOR.clone()));
        jcall!(this.orderManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).ADMINISTRATOR.clone()));
        jcall!(this.orderManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).STOREMANAGER.clone()));
        jcall!(this.adjusterManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).ADMINISTRATOR.clone()));
        jcall!(this.adjusterManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).STOREMANAGER.clone()));
        jcall!(this.adjusterManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).ADJUSTER.clone()));
        jcall!(this.shippingManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).ADMINISTRATOR.clone()));
        jcall!(this.shippingManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).STOREMANAGER.clone()));
        jcall!(this.shippingManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).SHIPPING.clone()));
        jcall!(this.webManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).ADMINISTRATOR.clone()));
        jcall!(this.webManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).STOREMANAGER.clone()));
        jcall!(this.webManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).WEBMANAGER.clone()));
        jcall!(this.webManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).WORKFLOWEDITOR.clone()));
        jcall!(this.webManagementRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).VIEWEDITOR.clone()));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getProductManagementRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.productManagementRoles.clone();
    }
    

    pub fn getAdminRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.adminRoles.clone();
    }
    

    pub fn getStoreAdminRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.storeAdminRoles.clone();
    }
    

    pub fn getStoreManagementRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.storeManagementRoles.clone();
    }
    

    pub fn getReviewManagementRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.reviewManagementRoles.clone();
    }
    

    pub fn getCustomerManagementRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.customerManagementRoles.clone();
    }
    

    pub fn getWorkflowManagementRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.workflowManagementRoles.clone();
    }
    

    pub fn getWebManagementRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.webManagementRoles.clone();
    }
    

    pub fn getShippingManagementRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.shippingManagementRoles.clone();
    }
    

    pub fn getAdjusterManagementRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.adjusterManagementRoles.clone();
    }
    

    pub fn getOrderManagementRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.orderManagementRoles.clone();
    }
}

