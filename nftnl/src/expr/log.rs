use super::{Expression, Rule};
use nftnl_sys as sys;
use std::os::raw::c_char;

pub enum Log {
    Group(u32),
}

impl Expression for Log {
    fn to_expr(&self, _rule: &Rule) -> *mut sys::nftnl_expr {
        match *self {
            Log::Group(id) => unsafe {
                let expr = try_alloc!(sys::nftnl_expr_alloc(b"log\0" as *const _ as *const c_char));
                sys::nftnl_expr_set_u32(expr, sys::NFTNL_EXPR_LOG_GROUP as u16, id);
                expr
            },
        }
    }
}

#[macro_export]
macro_rules! nft_expr_log {
    (group $id:expr) => {
        $crate::expr::Log::Group($id)
    };
}
