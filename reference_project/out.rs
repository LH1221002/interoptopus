#![feature(prelude_import)]
//! A reference project for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! This project tries to use every Interoptopus feature at least once.
//! When submitting new features or making changes to existing ones the types and functions in
//! here will ensure existing backends still work as expected.
//!
//! Note, many items here are deliberately not documented as testing how and if documentation
//! is generated is part of the test.
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use interoptopus::{constant, extra_type, function, pattern, Inventory, InventoryBuilder};
pub mod constants {
    //! Various ways to define constants.
    use interoptopus::ffi_constant;
    const fn f(x: i32) -> i32 {
        -x
    }
    pub const U8: u8 = u8::MAX;
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct U8 {}
    unsafe impl ::interoptopus::lang::rust::ConstantInfo for U8 {
        fn constant_info() -> interoptopus::lang::c::Constant {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            let value = ::interoptopus::lang::c::ConstantValue::from(U8);
            ::interoptopus::lang::c::Constant::new("U8".to_string(), value, meta)
        }
    }
    pub const F32_MIN_POSITIVE: f32 = f32::MIN_POSITIVE;
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct F32_MIN_POSITIVE {}
    unsafe impl ::interoptopus::lang::rust::ConstantInfo for F32_MIN_POSITIVE {
        fn constant_info() -> interoptopus::lang::c::Constant {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            let value = ::interoptopus::lang::c::ConstantValue::from(F32_MIN_POSITIVE);
            ::interoptopus::lang::c::Constant::new(
                "F32_MIN_POSITIVE".to_string(),
                value,
                meta,
            )
        }
    }
    pub const COMPUTED_I32: i32 = f(i32::MAX);
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct COMPUTED_I32 {}
    unsafe impl ::interoptopus::lang::rust::ConstantInfo for COMPUTED_I32 {
        fn constant_info() -> interoptopus::lang::c::Constant {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            let value = ::interoptopus::lang::c::ConstantValue::from(COMPUTED_I32);
            ::interoptopus::lang::c::Constant::new(
                "COMPUTED_I32".to_string(),
                value,
                meta,
            )
        }
    }
}
pub mod functions {
    //! Functions using all supported type patterns.
    use crate::patterns::result::{Error, FFIError};
    use crate::types::{
        ambiguous1, ambiguous2, common, some_foreign_type, Array, Callbacku8u8,
        EnumDocumented, EnumRenamedXYZ, Generic, Generic2, Generic3, Generic4, Opaque,
        Phantom, SomeForeignType, StructDocumented, StructRenamedXYZ, Transparent,
        Tupled, Vec3f32, Visibility1, Visibility2, Weird1, Weird2,
    };
    use interoptopus::patterns::option::FFIOption;
    use interoptopus::patterns::result::panics_and_errors_to_ffi_enum;
    use interoptopus::patterns::slice::FFISlice;
    use interoptopus::patterns::slice::FFISliceMut;
    use interoptopus::{ffi_function, ffi_surrogates, here};
    use std::ptr::null;
    use std::time::Duration;
    #[no_mangle]
    pub extern "C" fn primitive_void() {}
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct primitive_void {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for primitive_void {
        type Signature = extern "C" fn();
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                ::interoptopus::lang::c::CType::Primitive(
                    interoptopus::lang::c::PrimitiveType::Void,
                ),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "primitive_void".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    #[allow(clippy::unused_unit)]
    pub extern "C" fn primitive_void2() -> () {}
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct primitive_void2 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for primitive_void2 {
        type Signature = extern "C" fn() -> ();
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                ::interoptopus::lang::c::CType::Primitive(
                    ::interoptopus::lang::c::PrimitiveType::Void,
                ),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "primitive_void2".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn primitive_bool(x: bool) -> bool {
        !x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct primitive_bool {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for primitive_bool {
        type Signature = extern "C" fn(bool) -> bool;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <bool as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <bool as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "primitive_bool".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn primitive_u8(x: u8) -> u8 {
        u8::MAX - x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct primitive_u8 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for primitive_u8 {
        type Signature = extern "C" fn(u8) -> u8;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "primitive_u8".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn primitive_u16(x: u16) -> u16 {
        u16::MAX - x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct primitive_u16 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for primitive_u16 {
        type Signature = extern "C" fn(u16) -> u16;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <u16 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <u16 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "primitive_u16".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn primitive_u32(x: u32) -> u32 {
        u32::MAX - x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct primitive_u32 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for primitive_u32 {
        type Signature = extern "C" fn(u32) -> u32;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "primitive_u32".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn primitive_u64(x: u64) -> u64 {
        u64::MAX - x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct primitive_u64 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for primitive_u64 {
        type Signature = extern "C" fn(u64) -> u64;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <u64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <u64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "primitive_u64".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn primitive_i8(x: i8) -> i8 {
        -x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct primitive_i8 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for primitive_i8 {
        type Signature = extern "C" fn(i8) -> i8;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <i8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <i8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "primitive_i8".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn primitive_i16(x: i16) -> i16 {
        -x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct primitive_i16 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for primitive_i16 {
        type Signature = extern "C" fn(i16) -> i16;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <i16 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <i16 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "primitive_i16".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn primitive_i32(x: i32) -> i32 {
        -x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct primitive_i32 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for primitive_i32 {
        type Signature = extern "C" fn(i32) -> i32;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "primitive_i32".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn primitive_i64(x: i64) -> i64 {
        -x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct primitive_i64 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for primitive_i64 {
        type Signature = extern "C" fn(i64) -> i64;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "primitive_i64".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn many_args_5(x0: i64, x1: i64, x2: i64, x3: i64, x4: i64) -> i64 {
        x0 + x1 + x2 + x3 + x4
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct many_args_5 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for many_args_5 {
        type Signature = extern "C" fn(i64, i64, i64, i64, i64) -> i64;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x0".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x1".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x2".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x3".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x4".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "many_args_5".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn many_args_10(
        x0: i64,
        x1: i64,
        x2: i64,
        x3: i64,
        x4: i64,
        x5: i64,
        x6: i64,
        x7: i64,
        x8: i64,
        x9: i64,
    ) -> i64 {
        x0 + x1 + x2 + x3 + x4 + x5 + x6 + x7 + x8 + x9
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct many_args_10 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for many_args_10 {
        type Signature = extern "C" fn(
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
        ) -> i64;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x0".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x1".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x2".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x3".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x4".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x5".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x6".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x7".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x8".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x9".to_string(),
                        <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "many_args_10".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn ptr(x: *const i64) -> *const i64 {
        x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct ptr {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for ptr {
        type Signature = extern "C" fn(*const i64) -> *const i64;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <*const i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <*const i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new("ptr".to_string(), signature, meta)
        }
    }
    #[no_mangle]
    pub extern "C" fn ptr_ptr(x: *const *const i64) -> *const *const i64 {
        x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct ptr_ptr {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for ptr_ptr {
        type Signature = extern "C" fn(*const *const i64) -> *const *const i64;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <*const *const i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <*const *const i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "ptr_ptr".to_string(),
                signature,
                meta,
            )
        }
    }
    /// # Safety
    ///
    /// Parameter x must point to valid data.
    #[no_mangle]
    #[allow(unused_unsafe)]
    pub unsafe extern "C" fn ptr_mut(x: *mut i64) -> *mut i64 {
        unsafe { *x = -*x };
        x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct ptr_mut {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for ptr_mut {
        type Signature = extern "C" fn(*mut i64) -> *mut i64;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <*mut i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            doc_lines.push(" # Safety".to_string());
            doc_lines.push("".to_string());
            doc_lines.push(" Parameter x must point to valid data.".to_string());
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <*mut i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "ptr_mut".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn ref_simple(x: &i64) -> &i64 {
        x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct ref_simple {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for ref_simple {
        type Signature = extern "C" fn(&i64) -> &i64;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <&i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <&i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "ref_simple".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn ref_mut_simple(x: &mut i64) -> &mut i64 {
        *x = -*x;
        x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct ref_mut_simple {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for ref_mut_simple {
        type Signature = extern "C" fn(&mut i64) -> &mut i64;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <&mut i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <&mut i64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "ref_mut_simple".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn ref_option(x: Option<&i64>) -> bool {
        x.is_some()
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct ref_option {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for ref_option {
        type Signature = extern "C" fn(Option<&i64>) -> bool;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <Option<
                            &i64,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <bool as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "ref_option".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn ref_mut_option(x: Option<&mut i64>) -> bool {
        x.is_some()
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct ref_mut_option {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for ref_mut_option {
        type Signature = extern "C" fn(Option<&mut i64>) -> bool;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <Option<
                            &mut i64,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <bool as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "ref_mut_option".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn tupled(x: Tupled) -> Tupled {
        Tupled(x.0 * 2)
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct tupled {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for tupled {
        type Signature = extern "C" fn(Tupled) -> Tupled;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <Tupled as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <Tupled as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new("tupled".to_string(), signature, meta)
        }
    }
    #[no_mangle]
    pub extern "C" fn repr_transparent(x: Transparent, _r: &Transparent) -> Transparent {
        x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct repr_transparent {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for repr_transparent {
        type Signature = extern "C" fn(Transparent, &Transparent) -> Transparent;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <Transparent as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "r".to_string(),
                        <&Transparent as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <Transparent as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "repr_transparent".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn complex_args_1(_a: Vec3f32, _b: Option<&Tupled>) -> FFIError {
        FFIError::Ok
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct complex_args_1 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for complex_args_1 {
        type Signature = extern "C" fn(Vec3f32, Option<&Tupled>) -> FFIError;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "a".to_string(),
                        <Vec3f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "b".to_string(),
                        <Option<
                            &Tupled,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "complex_args_1".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn complex_args_2(_cmplx: SomeForeignType) -> *const Opaque {
        null()
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct complex_args_2 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for complex_args_2 {
        type Signature = extern "C" fn(SomeForeignType) -> *const Opaque;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "cmplx".to_string(),
                        some_foreign_type(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <*const Opaque as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "complex_args_2".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn callback(callback: Callbacku8u8, value: u8) -> u8 {
        callback(value)
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct callback {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for callback {
        type Signature = extern "C" fn(Callbacku8u8, u8) -> u8;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "callback".to_string(),
                        <Callbacku8u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "value".to_string(),
                        <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "callback".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn generic_1a(x: Generic<u32>, _y: Phantom<u8>) -> u32 {
        *x.x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct generic_1a {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for generic_1a {
        type Signature = extern "C" fn(Generic<u32>, Phantom<u8>) -> u32;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <Generic<
                            u32,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "y".to_string(),
                        <Phantom<
                            u8,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "generic_1a".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn generic_1b(x: Generic<u8>, _y: Phantom<u8>) -> u8 {
        *x.x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct generic_1b {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for generic_1b {
        type Signature = extern "C" fn(Generic<u8>, Phantom<u8>) -> u8;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <Generic<
                            u8,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "y".to_string(),
                        <Phantom<
                            u8,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "generic_1b".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn generic_1c<'a>(
        _x: Option<&'a Generic<'a, u8>>,
        y: &Generic<'a, u8>,
    ) -> u8 {
        *y.x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct generic_1c<'a> {
        a: ::std::marker::PhantomData<&'a ()>,
    }
    unsafe impl<'a> ::interoptopus::lang::rust::FunctionInfo for generic_1c<'a> {
        type Signature = extern "C" fn(
            Option<&'a Generic<'a, u8>>,
            &Generic<'a, u8>,
        ) -> u8;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <Option<
                            &Generic<u8>,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "y".to_string(),
                        <&Generic<
                            u8,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "generic_1c".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn generic_2(x: &Generic2<u8>) -> u8 {
        x.x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct generic_2 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for generic_2 {
        type Signature = extern "C" fn(&Generic2<u8>) -> u8;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <&Generic2<
                            u8,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "generic_2".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn generic_3(x: &Generic3<u8>) -> u8 {
        x.x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct generic_3 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for generic_3 {
        type Signature = extern "C" fn(&Generic3<u8>) -> u8;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <&Generic3<
                            u8,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "generic_3".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn generic_4(x: &Generic4<u8>) -> u8 {
        x.x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct generic_4 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for generic_4 {
        type Signature = extern "C" fn(&Generic4<u8>) -> u8;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <&Generic4<
                            u8,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "generic_4".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn array_1(x: Array) -> u8 {
        x.data[0]
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct array_1 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for array_1 {
        type Signature = extern "C" fn(Array) -> u8;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <Array as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "array_1".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn renamed(x: StructRenamedXYZ) -> EnumRenamedXYZ {
        x.e
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct renamed {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for renamed {
        type Signature = extern "C" fn(StructRenamedXYZ) -> EnumRenamedXYZ;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <StructRenamedXYZ as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <EnumRenamedXYZ as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "renamed".to_string(),
                signature,
                meta,
            )
        }
    }
    /// This function has documentation.
    #[no_mangle]
    pub extern "C" fn documented(_x: StructDocumented) -> EnumDocumented {
        EnumDocumented::A
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct documented {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for documented {
        type Signature = extern "C" fn(StructDocumented) -> EnumDocumented;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <StructDocumented as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            doc_lines.push(" This function has documentation.".to_string());
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <EnumDocumented as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "documented".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn ambiguous_1(x: ambiguous1::Vec) -> ambiguous1::Vec {
        x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct ambiguous_1 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for ambiguous_1 {
        type Signature = extern "C" fn(ambiguous1::Vec) -> ambiguous1::Vec;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <ambiguous1::Vec as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <ambiguous1::Vec as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "ambiguous_1".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn ambiguous_2(x: ambiguous2::Vec) -> ambiguous2::Vec {
        x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct ambiguous_2 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for ambiguous_2 {
        type Signature = extern "C" fn(ambiguous2::Vec) -> ambiguous2::Vec;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <ambiguous2::Vec as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <ambiguous2::Vec as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "ambiguous_2".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn ambiguous_3(x: ambiguous1::Vec, y: ambiguous2::Vec) -> bool {
        (x.x as f64 - y.x).abs() < 0.5
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct ambiguous_3 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for ambiguous_3 {
        type Signature = extern "C" fn(ambiguous1::Vec, ambiguous2::Vec) -> bool;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <ambiguous1::Vec as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "y".to_string(),
                        <ambiguous2::Vec as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <bool as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "ambiguous_3".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn namespaced_type(x: common::Vec) -> common::Vec {
        x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct namespaced_type {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for namespaced_type {
        type Signature = extern "C" fn(common::Vec) -> common::Vec;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <common::Vec as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <common::Vec as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "namespaced_type".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn namespaced_inner_option(
        x: FFIOption<common::Vec>,
    ) -> FFIOption<common::Vec> {
        x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct namespaced_inner_option {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for namespaced_inner_option {
        type Signature = extern "C" fn(FFIOption<common::Vec>) -> FFIOption<common::Vec>;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <FFIOption<
                            common::Vec,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <FFIOption<
                    common::Vec,
                > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "namespaced_inner_option".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn namespaced_inner_slice(
        x: FFISlice<common::Vec>,
    ) -> FFISlice<common::Vec> {
        x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct namespaced_inner_slice {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for namespaced_inner_slice {
        type Signature = extern "C" fn(FFISlice<common::Vec>) -> FFISlice<common::Vec>;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <FFISlice<
                            common::Vec,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <FFISlice<
                    common::Vec,
                > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "namespaced_inner_slice".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn namespaced_inner_slice_mut(
        x: FFISliceMut<common::Vec>,
    ) -> FFISliceMut<common::Vec> {
        x
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct namespaced_inner_slice_mut {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for namespaced_inner_slice_mut {
        type Signature = extern "C" fn(
            FFISliceMut<common::Vec>,
        ) -> FFISliceMut<common::Vec>;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <FFISliceMut<
                            common::Vec,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <FFISliceMut<
                    common::Vec,
                > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "namespaced_inner_slice_mut".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    #[allow(unreachable_code)]
    pub extern "C" fn panics() -> FFIError {
        panics_and_errors_to_ffi_enum(
            || {
                {
                    ::std::rt::begin_panic("Oh no");
                };
                Ok::<(), Error>(())
            },
            "reference_project\\src\\functions.rs:284",
        )
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct panics {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for panics {
        type Signature = extern "C" fn() -> FFIError;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new("panics".to_string(), signature, meta)
        }
    }
    #[no_mangle]
    pub extern "C" fn sleep(millis: u64) {
        std::thread::sleep(Duration::from_millis(millis));
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct sleep {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for sleep {
        type Signature = extern "C" fn(u64);
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "millis".to_string(),
                        <u64 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                ::interoptopus::lang::c::CType::Primitive(
                    interoptopus::lang::c::PrimitiveType::Void,
                ),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new("sleep".to_string(), signature, meta)
        }
    }
    #[no_mangle]
    pub extern "C" fn weird_1(_x: Weird1<u32>, _y: Weird2<u8, 5>) -> bool {
        true
    }
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct weird_1 {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for weird_1 {
        type Signature = extern "C" fn(Weird1<u32>, Weird2<u8, 5>) -> bool;
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <Weird1<
                            u32,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "y".to_string(),
                        <Weird2<
                            u8,
                            5,
                        > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                <bool as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "weird_1".to_string(),
                signature,
                meta,
            )
        }
    }
    #[no_mangle]
    pub extern "C" fn visibility(_x: Visibility1, _y: Visibility2) {}
    #[allow(non_camel_case_types)]
    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct visibility {}
    unsafe impl ::interoptopus::lang::rust::FunctionInfo for visibility {
        type Signature = extern "C" fn(Visibility1, Visibility2);
        fn function_info() -> ::interoptopus::lang::c::Function {
            let mut doc_lines = ::std::vec::Vec::new();
            let mut params = ::std::vec::Vec::new();
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "x".to_string(),
                        <Visibility1 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            params
                .push(
                    ::interoptopus::lang::c::Parameter::new(
                        "y".to_string(),
                        <Visibility2 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                    ),
                );
            let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                params,
                ::interoptopus::lang::c::CType::Primitive(
                    interoptopus::lang::c::PrimitiveType::Void,
                ),
            );
            let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                doc_lines,
            );
            let meta = ::interoptopus::lang::c::Meta::with_documentation(
                documentation,
                None,
            );
            ::interoptopus::lang::c::Function::new(
                "visibility".to_string(),
                signature,
                meta,
            )
        }
    }
}
/// Reference implementations of patterns.
pub mod patterns {
    pub mod api_guard {
        use interoptopus::ffi_function;
        use interoptopus::patterns::api_guard::APIVersion;
        #[no_mangle]
        pub extern "C" fn pattern_api_guard() -> APIVersion {
            crate::ffi_inventory().into()
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_api_guard {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_api_guard {
            type Signature = extern "C" fn() -> APIVersion;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <APIVersion as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_api_guard".to_string(),
                    signature,
                    meta,
                )
            }
        }
    }
    pub mod ascii_pointer {
        use crate::types::UseAsciiStringPattern;
        use interoptopus::ffi_function;
        use interoptopus::patterns::slice::FFISlice;
        use interoptopus::patterns::string::AsciiPointer;
        #[no_mangle]
        pub extern "C" fn pattern_ascii_pointer_1(x: AsciiPointer) -> u32 {
            x.as_c_str().map(|x| x.to_bytes().len()).unwrap_or(0) as u32
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ascii_pointer_1 {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for pattern_ascii_pointer_1 {
            type Signature = extern "C" fn(AsciiPointer) -> u32;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "x".to_string(),
                            <AsciiPointer as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ascii_pointer_1".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ascii_pointer_2() -> AsciiPointer<'static> {
            AsciiPointer::empty()
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ascii_pointer_2 {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for pattern_ascii_pointer_2 {
            type Signature = extern "C" fn() -> AsciiPointer<'static>;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <AsciiPointer<
                        'static,
                    > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ascii_pointer_2".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ascii_pointer_len(
            x: AsciiPointer,
            y: UseAsciiStringPattern,
        ) -> u32 {
            let x1 = x.as_str().map(|x| x.len()).unwrap_or(0);
            let x2 = y.ascii_string.as_str().map(|x| x.len()).unwrap_or(0);
            (x1 + x2) as u32
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ascii_pointer_len {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for pattern_ascii_pointer_len {
            type Signature = extern "C" fn(AsciiPointer, UseAsciiStringPattern) -> u32;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "x".to_string(),
                            <AsciiPointer as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "y".to_string(),
                            <UseAsciiStringPattern as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ascii_pointer_len".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ascii_pointer_return_slice() -> FFISlice<
            'static,
            UseAsciiStringPattern<'static>,
        > {
            FFISlice::empty()
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ascii_pointer_return_slice {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for pattern_ascii_pointer_return_slice {
            type Signature = extern "C" fn() -> FFISlice<
                'static,
                UseAsciiStringPattern<'static>,
            >;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFISlice<
                        'static,
                        UseAsciiStringPattern<'static>,
                    > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ascii_pointer_return_slice".to_string(),
                    signature,
                    meta,
                )
            }
        }
    }
    pub mod callbacks {
        use interoptopus::{callback, ffi_function, ffi_type};
        use std::ffi::c_void;
        #[repr(transparent)]
        pub struct MyCallback(Option<extern "C" fn(u32) -> u32>);
        #[automatically_derived]
        impl ::core::default::Default for MyCallback {
            #[inline]
            fn default() -> MyCallback {
                MyCallback(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MyCallback {
            #[inline]
            fn clone(&self) -> MyCallback {
                MyCallback(::core::clone::Clone::clone(&self.0))
            }
        }
        impl MyCallback {
            /// Will call function if it exists, panic otherwise.
            pub fn call(&self, value: u32) -> u32 {
                self.0.expect("Assumed function would exist but it didn't.")(value)
            }
            /// Will call function only if it exists
            pub fn call_if_some(&self, value: u32) -> Option<u32> {
                match self.0 {
                    Some(c) => Some(c(value)),
                    None => None,
                }
            }
        }
        impl From<extern "C" fn(u32) -> u32> for MyCallback {
            fn from(x: extern "C" fn(u32) -> u32) -> Self {
                Self(Some(x))
            }
        }
        unsafe impl interoptopus::lang::rust::CTypeInfo for MyCallback {
            fn type_info() -> interoptopus::lang::c::CType {
                use interoptopus::lang::rust::CTypeInfo;
                let rval = <u32 as CTypeInfo>::type_info();
                let params = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        interoptopus::lang::c::Parameter::new(
                            "value".to_string(),
                            <u32 as CTypeInfo>::type_info(),
                        ),
                    ]),
                );
                let sig = interoptopus::lang::c::FunctionSignature::new(params, rval);
                let fn_pointer = interoptopus::lang::c::FnPointerType::new(sig);
                let named_callback = interoptopus::patterns::callbacks::NamedCallback::new(
                    "MyCallback".to_string(),
                    fn_pointer,
                );
                interoptopus::lang::c::CType::Pattern(
                    interoptopus::patterns::TypePattern::NamedCallback(named_callback),
                )
            }
        }
        #[repr(transparent)]
        pub struct MyCallbackVoid(Option<extern "C" fn(*const c_void) -> ()>);
        #[automatically_derived]
        impl ::core::default::Default for MyCallbackVoid {
            #[inline]
            fn default() -> MyCallbackVoid {
                MyCallbackVoid(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MyCallbackVoid {
            #[inline]
            fn clone(&self) -> MyCallbackVoid {
                MyCallbackVoid(::core::clone::Clone::clone(&self.0))
            }
        }
        impl MyCallbackVoid {
            /// Will call function if it exists, panic otherwise.
            pub fn call(&self, ptr: *const c_void) -> () {
                self.0.expect("Assumed function would exist but it didn't.")(ptr)
            }
            /// Will call function only if it exists
            pub fn call_if_some(&self, ptr: *const c_void) -> Option<()> {
                match self.0 {
                    Some(c) => Some(c(ptr)),
                    None => None,
                }
            }
        }
        impl From<extern "C" fn(*const c_void) -> ()> for MyCallbackVoid {
            fn from(x: extern "C" fn(*const c_void) -> ()) -> Self {
                Self(Some(x))
            }
        }
        unsafe impl interoptopus::lang::rust::CTypeInfo for MyCallbackVoid {
            fn type_info() -> interoptopus::lang::c::CType {
                use interoptopus::lang::rust::CTypeInfo;
                let rval = <() as CTypeInfo>::type_info();
                let params = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        interoptopus::lang::c::Parameter::new(
                            "ptr".to_string(),
                            <*const c_void as CTypeInfo>::type_info(),
                        ),
                    ]),
                );
                let sig = interoptopus::lang::c::FunctionSignature::new(params, rval);
                let fn_pointer = interoptopus::lang::c::FnPointerType::new(sig);
                let named_callback = interoptopus::patterns::callbacks::NamedCallback::new(
                    "MyCallbackVoid".to_string(),
                    fn_pointer,
                );
                interoptopus::lang::c::CType::Pattern(
                    interoptopus::patterns::TypePattern::NamedCallback(named_callback),
                )
            }
        }
        #[repr(transparent)]
        pub struct MyCallbackContextual(Option<extern "C" fn(*const c_void, u32) -> ()>);
        #[automatically_derived]
        impl ::core::default::Default for MyCallbackContextual {
            #[inline]
            fn default() -> MyCallbackContextual {
                MyCallbackContextual(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MyCallbackContextual {
            #[inline]
            fn clone(&self) -> MyCallbackContextual {
                MyCallbackContextual(::core::clone::Clone::clone(&self.0))
            }
        }
        impl MyCallbackContextual {
            /// Will call function if it exists, panic otherwise.
            pub fn call(&self, context: *const c_void, value: u32) -> () {
                self
                    .0
                    .expect(
                        "Assumed function would exist but it didn't.",
                    )(context, value)
            }
            /// Will call function only if it exists
            pub fn call_if_some(
                &self,
                context: *const c_void,
                value: u32,
            ) -> Option<()> {
                match self.0 {
                    Some(c) => Some(c(context, value)),
                    None => None,
                }
            }
        }
        impl From<extern "C" fn(*const c_void, u32) -> ()> for MyCallbackContextual {
            fn from(x: extern "C" fn(*const c_void, u32) -> ()) -> Self {
                Self(Some(x))
            }
        }
        unsafe impl interoptopus::lang::rust::CTypeInfo for MyCallbackContextual {
            fn type_info() -> interoptopus::lang::c::CType {
                use interoptopus::lang::rust::CTypeInfo;
                let rval = <() as CTypeInfo>::type_info();
                let params = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <*const c_void as CTypeInfo>::type_info(),
                        ),
                        interoptopus::lang::c::Parameter::new(
                            "value".to_string(),
                            <u32 as CTypeInfo>::type_info(),
                        ),
                    ]),
                );
                let sig = interoptopus::lang::c::FunctionSignature::new(params, rval);
                let fn_pointer = interoptopus::lang::c::FnPointerType::new(sig);
                let named_callback = interoptopus::patterns::callbacks::NamedCallback::new(
                    "MyCallbackContextual".to_string(),
                    fn_pointer,
                );
                interoptopus::lang::c::CType::Pattern(
                    interoptopus::patterns::TypePattern::NamedCallback(named_callback),
                )
            }
        }
        impl DelegateResult for MyCallbackContextual {
            type Input = u32;
            fn complete(&self, ctx: *const c_void, value: Self::Input) {
                self.call_if_some(ctx, value);
            }
        }
        #[repr(C)]
        pub struct DelegateCallback<DeleResult> {
            pub callback: DeleResult,
            pub context: *const c_void,
        }
        unsafe impl<DeleResult> ::interoptopus::lang::rust::CTypeInfo
        for DelegateCallback<DeleResult>
        where
            DeleResult: interoptopus::lang::rust::CTypeInfo,
        {
            fn type_info() -> ::interoptopus::lang::c::CType {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                    "".to_string(),
                    documentation,
                    None,
                );
                let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
                let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
                {
                    generics
                        .push(
                            <DeleResult as ::interoptopus::lang::rust::CTypeInfo>::type_info()
                                .rust_name(),
                        );
                }
                let name = {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "{0}{1}",
                            "DelegateCallback".to_string(),
                            generics.join(""),
                        ),
                    );
                    res
                };
                {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    let the_type = <DeleResult as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                    let field = ::interoptopus::lang::c::Field::with_documentation(
                        "callback".to_string(),
                        the_type,
                        interoptopus::lang::c::Visibility::Public,
                        documentation,
                    );
                    fields.push(field);
                }
                {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    let the_type = <*const c_void as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                    let field = ::interoptopus::lang::c::Field::with_documentation(
                        "context".to_string(),
                        the_type,
                        interoptopus::lang::c::Visibility::Public,
                        documentation,
                    );
                    fields.push(field);
                }
                let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                    name,
                    fields,
                    meta,
                );
                ::interoptopus::lang::c::CType::Composite(rval)
            }
        }
        impl<DeleResult> DelegateCallback<DeleResult>
        where
            DeleResult: DelegateResult,
        {
            pub fn call(&self, value: DeleResult::Input) {
                self.callback.complete(self.context, value)
            }
        }
        pub trait DelegateResult {
            type Input;
            fn complete(&self, ctx: *const c_void, value: Self::Input);
        }
        #[no_mangle]
        pub extern "C" fn pattern_callback_1(callback: MyCallback, x: u32) -> u32 {
            callback.call(x)
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_callback_1 {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_callback_1 {
            type Signature = extern "C" fn(MyCallback, u32) -> u32;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "callback".to_string(),
                            <MyCallback as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "x".to_string(),
                            <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_callback_1".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_callback_2(
            callback: MyCallbackVoid,
        ) -> MyCallbackVoid {
            callback
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_callback_2 {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_callback_2 {
            type Signature = extern "C" fn(MyCallbackVoid) -> MyCallbackVoid;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "callback".to_string(),
                            <MyCallbackVoid as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <MyCallbackVoid as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_callback_2".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_callback_3(
            callback: DelegateCallback<MyCallbackContextual>,
            x: u32,
        ) {
            callback.call(x)
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_callback_3 {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_callback_3 {
            type Signature = extern "C" fn(DelegateCallback<MyCallbackContextual>, u32);
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "callback".to_string(),
                            <DelegateCallback<
                                MyCallbackContextual,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "x".to_string(),
                            <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    ::interoptopus::lang::c::CType::Primitive(
                        interoptopus::lang::c::PrimitiveType::Void,
                    ),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_callback_3".to_string(),
                    signature,
                    meta,
                )
            }
        }
    }
    pub mod option {
        use interoptopus::patterns::option::FFIOption;
        use interoptopus::{ffi_function, ffi_type};
        #[repr(C)]
        pub struct Inner {
            x: f32,
        }
        unsafe impl ::interoptopus::lang::rust::CTypeInfo for Inner {
            fn type_info() -> ::interoptopus::lang::c::CType {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                    "".to_string(),
                    documentation,
                    None,
                );
                let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
                let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
                let name = {
                    let res = ::alloc::fmt::format(
                        format_args!("{0}{1}", "Inner".to_string(), generics.join("")),
                    );
                    res
                };
                {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    let the_type = <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                    let field = ::interoptopus::lang::c::Field::with_documentation(
                        "x".to_string(),
                        the_type,
                        interoptopus::lang::c::Visibility::Private,
                        documentation,
                    );
                    fields.push(field);
                }
                let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                    name,
                    fields,
                    meta,
                );
                ::interoptopus::lang::c::CType::Composite(rval)
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_option_1(
            ffi_slice: FFIOption<Inner>,
        ) -> FFIOption<Inner> {
            ffi_slice
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_option_1 {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_ffi_option_1 {
            type Signature = extern "C" fn(FFIOption<Inner>) -> FFIOption<Inner>;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "ffi_slice".to_string(),
                            <FFIOption<
                                Inner,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIOption<
                        Inner,
                    > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_option_1".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_option_2(ffi_slice: FFIOption<Inner>) -> Inner {
            ffi_slice.into_option().unwrap_or(Inner { x: f32::NAN })
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_option_2 {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_ffi_option_2 {
            type Signature = extern "C" fn(FFIOption<Inner>) -> Inner;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "ffi_slice".to_string(),
                            <FFIOption<
                                Inner,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <Inner as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_option_2".to_string(),
                    signature,
                    meta,
                )
            }
        }
    }
    pub mod primitives {
        use interoptopus::ffi_function;
        use interoptopus::patterns::primitives::{FFIBool, FFICChar};
        #[no_mangle]
        pub extern "C" fn pattern_ffi_bool(ffi_bool: FFIBool) -> FFIBool {
            !ffi_bool
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_bool {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_ffi_bool {
            type Signature = extern "C" fn(FFIBool) -> FFIBool;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "ffi_bool".to_string(),
                            <FFIBool as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIBool as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_bool".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_cchar(ffi_cchar: FFICChar) -> FFICChar {
            ffi_cchar
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_cchar {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_ffi_cchar {
            type Signature = extern "C" fn(FFICChar) -> FFICChar;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "ffi_cchar".to_string(),
                            <FFICChar as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFICChar as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_cchar".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_cchar_const_pointer(
            ffi_cchar: *const FFICChar,
        ) -> *const FFICChar {
            ffi_cchar
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_cchar_const_pointer {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for pattern_ffi_cchar_const_pointer {
            type Signature = extern "C" fn(*const FFICChar) -> *const FFICChar;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "ffi_cchar".to_string(),
                            <*const FFICChar as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <*const FFICChar as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_cchar_const_pointer".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_cchar_mut_pointer(
            ffi_cchar: *mut FFICChar,
        ) -> *mut FFICChar {
            ffi_cchar
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_cchar_mut_pointer {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for pattern_ffi_cchar_mut_pointer {
            type Signature = extern "C" fn(*mut FFICChar) -> *mut FFICChar;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "ffi_cchar".to_string(),
                            <*mut FFICChar as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <*mut FFICChar as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_cchar_mut_pointer".to_string(),
                    signature,
                    meta,
                )
            }
        }
    }
    pub mod result {
        use interoptopus::ffi_type;
        use std::fmt::{Display, Formatter};
        #[repr(C)]
        pub enum FFIError {
            Ok = 0,
            Null = 100,
            Panic = 200,
            Fail = 300,
        }
        unsafe impl ::interoptopus::lang::rust::VariantInfo for FFIError {
            fn variant_info(&self) -> ::interoptopus::lang::c::Variant {
                match self {
                    Self::Ok => {
                        let documentation = ::interoptopus::lang::c::Documentation::from_line(
                            "",
                        );
                        ::interoptopus::lang::c::Variant::new(
                            "Ok".to_string(),
                            0i32 as usize,
                            documentation,
                        )
                    }
                    Self::Null => {
                        let documentation = ::interoptopus::lang::c::Documentation::from_line(
                            "",
                        );
                        ::interoptopus::lang::c::Variant::new(
                            "Null".to_string(),
                            100i32 as usize,
                            documentation,
                        )
                    }
                    Self::Panic => {
                        let documentation = ::interoptopus::lang::c::Documentation::from_line(
                            "",
                        );
                        ::interoptopus::lang::c::Variant::new(
                            "Panic".to_string(),
                            200i32 as usize,
                            documentation,
                        )
                    }
                    Self::Fail => {
                        let documentation = ::interoptopus::lang::c::Documentation::from_line(
                            "",
                        );
                        ::interoptopus::lang::c::Variant::new(
                            "Fail".to_string(),
                            300i32 as usize,
                            documentation,
                        )
                    }
                }
            }
        }
        unsafe impl ::interoptopus::lang::rust::CTypeInfo for FFIError {
            fn type_info() -> ::interoptopus::lang::c::CType {
                use ::interoptopus::lang::rust::VariantInfo;
                let mut variants = ::std::vec::Vec::new();
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                    "".to_string(),
                    documentation,
                    None,
                );
                {
                    variants.push(Self::Ok.variant_info());
                }
                {
                    variants.push(Self::Null.variant_info());
                }
                {
                    variants.push(Self::Panic.variant_info());
                }
                {
                    variants.push(Self::Fail.variant_info());
                }
                let rval = ::interoptopus::lang::c::EnumType::new(
                    "FFIError".to_string(),
                    variants,
                    meta,
                );
                use ::interoptopus::patterns::result::FFIError as _;
                let success_variant = Self::SUCCESS.variant_info();
                let the_success_enum = ::interoptopus::patterns::result::FFIErrorEnum::new(
                    rval,
                    success_variant,
                );
                let the_pattern = ::interoptopus::patterns::TypePattern::FFIErrorEnum(
                    the_success_enum,
                );
                ::interoptopus::lang::c::CType::Pattern(the_pattern)
            }
        }
        pub enum Error {
            Bad,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "Bad")
            }
        }
        impl From<Error> for FFIError {
            fn from(x: Error) -> Self {
                match x {
                    Error::Bad => Self::Fail,
                }
            }
        }
        impl Default for FFIError {
            fn default() -> Self {
                Self::Ok
            }
        }
        impl interoptopus::patterns::result::FFIError for FFIError {
            const SUCCESS: Self = Self::Ok;
            const NULL: Self = Self::Null;
            const PANIC: Self = Self::Panic;
        }
        impl Display for Error {
            fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
                Ok(())
            }
        }
        impl std::error::Error for Error {}
    }
    pub mod service {
        use crate::patterns::callbacks::MyCallback;
        use crate::patterns::result::{Error, FFIError};
        use interoptopus::patterns::primitives::FFIBool;
        use interoptopus::patterns::slice::{FFISlice, FFISliceMut};
        use interoptopus::patterns::string::AsciiPointer;
        use interoptopus::{
            ffi_service, ffi_service_ctor, ffi_service_ignore, ffi_service_method,
            ffi_type,
        };
        use std::ffi::CString;
        /// Some struct we want to expose as a class.
        pub struct SimpleService {
            pub some_value: u32,
            pub c_string: CString,
            pub data: Vec<u32>,
        }
        #[automatically_derived]
        impl ::core::default::Default for SimpleService {
            #[inline]
            fn default() -> SimpleService {
                SimpleService {
                    some_value: ::core::default::Default::default(),
                    c_string: ::core::default::Default::default(),
                    data: ::core::default::Default::default(),
                }
            }
        }
        unsafe impl ::interoptopus::lang::rust::CTypeInfo for SimpleService {
            fn type_info() -> ::interoptopus::lang::c::CType {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    " Some struct we want to expose as a class.",
                );
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                    "".to_string(),
                    documentation,
                    None,
                );
                let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
                let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
                let name = {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "{0}{1}",
                            "SimpleService".to_string(),
                            generics.join(""),
                        ),
                    );
                    res
                };
                let mut rval = ::interoptopus::lang::c::OpaqueType::new(name, meta);
                ::interoptopus::lang::c::CType::Opaque(rval)
            }
        }
        impl SimpleService {
            /// The constructor must return a `Result<Self, Error>`.
            pub fn new_with(some_value: u32) -> Result<Self, Error> {
                Ok(Self {
                    some_value,
                    c_string: CString::new("Hello new_with").unwrap(),
                    data: ::alloc::vec::from_elem(some_value, some_value as usize),
                })
            }
            pub fn new_without() -> Result<Self, Error> {
                Ok(Self {
                    some_value: 0,
                    c_string: CString::new("Hello new_without").unwrap(),
                    data: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([1, 2, 3]),
                    ),
                })
            }
            pub fn new_with_string(ascii: AsciiPointer) -> Result<Self, Error> {
                Ok(Self {
                    some_value: 0,
                    c_string: ascii.as_c_str().unwrap().into(),
                    data: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([1, 2, 3]),
                    ),
                })
            }
            pub fn new_failing(_some_value: u8) -> Result<Self, Error> {
                Err(Error::Bad)
            }
            /// Methods returning a Result<(), _> are the default and do not
            /// need annotations.
            pub fn method_result(&self, _: u32) -> Result<(), Error> {
                Ok(())
            }
            pub fn method_value(&self, x: u32) -> u32 {
                x
            }
            /// This method should be documented.
            ///
            /// Multiple lines.
            pub fn method_void(&self) {}
            pub fn method_mut_self(&mut self, slice: FFISlice<u8>) -> u8 {
                *slice.as_slice().first().unwrap_or(&0)
            }
            /// Single line.
            pub fn method_mut_self_void(&mut self, _slice: FFISlice<FFIBool>) {}
            pub fn method_mut_self_ref(&mut self, x: &u8, _y: &mut u8) -> u8 {
                *x
            }
            pub fn method_mut_self_ref_slice(
                &mut self,
                x: &u8,
                _y: &mut u8,
                _slice: FFISlice<u8>,
            ) -> u8 {
                *x
            }
            #[allow(clippy::needless_lifetimes)]
            pub fn method_mut_self_ref_slice_limited<'a, 'b>(
                &mut self,
                x: &u8,
                _y: &mut u8,
                _slice: FFISlice<'a, u8>,
                _slice2: FFISlice<'b, u8>,
            ) -> u8 {
                *x
            }
            pub fn method_mut_self_ffi_error(
                &mut self,
                _slice: FFISliceMut<u8>,
            ) -> Result<(), Error> {
                Ok(())
            }
            pub fn method_mut_self_no_error(
                &mut self,
                mut slice: FFISliceMut<u8>,
            ) -> Result<(), Error> {
                slice.as_slice_mut();
                Ok(())
            }
            /// Warning, you _must_ discard the returned slice object before calling into this service
            /// again, as otherwise undefined behavior might happen.
            pub fn return_slice(&mut self) -> FFISlice<u32> {
                self.data.as_slice().into()
            }
            /// Warning, you _must_ discard the returned slice object before calling into this service
            /// again, as otherwise undefined behavior might happen.
            pub fn return_slice_mut(&mut self) -> FFISliceMut<u32> {
                FFISliceMut::from_slice(&mut self.data)
            }
            /// This function has no panic safeguards. If it panics your host app will be in an undefined state.
            pub fn return_string(&mut self) -> AsciiPointer {
                AsciiPointer::from_cstr(&self.c_string)
            }
            pub fn method_void_ffi_error(&mut self) -> Result<(), Error> {
                Ok(())
            }
            pub fn this_is_ignored(&mut self) -> Result<(), Error> {
                Ok(())
            }
            pub fn method_callback(
                &mut self,
                callback: MyCallback,
            ) -> Result<(), Error> {
                callback.call(0);
                Ok(())
            }
            /// No FFI bindings are generated for non-pub methods.
            #[allow(unused)]
            fn not_exposed<T>(&mut self, _: T) -> Result<(), Error> {
                Ok(())
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        /// The constructor must return a `Result<Self, Error>`.
        pub extern "C" fn simple_service_new_with(
            context: &mut *mut SimpleService,
            some_value: u32,
        ) -> FFIError {
            *context = ::std::ptr::null_mut();
            let result_result = std::panic::catch_unwind(
                ::std::panic::AssertUnwindSafe(|| {
                    <SimpleService>::new_with(some_value)
                }),
            );
            match result_result {
                Ok(Ok(obj)) => {
                    let boxed = ::std::boxed::Box::new(obj);
                    let raw = ::std::boxed::Box::into_raw(boxed);
                    *context = raw;
                    <FFIError as ::interoptopus::patterns::result::FFIError>::SUCCESS
                }
                Ok(Err(e)) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Error in ({0}): {1:?}",
                                "simple_service_new_with",
                                e,
                            ),
                        );
                        res
                    });
                    e.into()
                }
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_new_with",
                                e,
                            ),
                        );
                        res
                    });
                    <FFIError as ::interoptopus::patterns::result::FFIError>::PANIC
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_new_with {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_new_with {
            type Signature = extern "C" fn(&mut *mut SimpleService, u32) -> FFIError;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut *mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "some_value".to_string(),
                            <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                doc_lines
                    .push(
                        " The constructor must return a `Result<Self, Error>`."
                            .to_string(),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_new_with".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_new_without(
            context: &mut *mut SimpleService,
        ) -> FFIError {
            *context = ::std::ptr::null_mut();
            let result_result = std::panic::catch_unwind(
                ::std::panic::AssertUnwindSafe(|| { <SimpleService>::new_without() }),
            );
            match result_result {
                Ok(Ok(obj)) => {
                    let boxed = ::std::boxed::Box::new(obj);
                    let raw = ::std::boxed::Box::into_raw(boxed);
                    *context = raw;
                    <FFIError as ::interoptopus::patterns::result::FFIError>::SUCCESS
                }
                Ok(Err(e)) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Error in ({0}): {1:?}",
                                "simple_service_new_without",
                                e,
                            ),
                        );
                        res
                    });
                    e.into()
                }
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_new_without",
                                e,
                            ),
                        );
                        res
                    });
                    <FFIError as ::interoptopus::patterns::result::FFIError>::PANIC
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_new_without {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_new_without {
            type Signature = extern "C" fn(&mut *mut SimpleService) -> FFIError;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut *mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_new_without".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_new_with_string(
            context: &mut *mut SimpleService,
            ascii: AsciiPointer,
        ) -> FFIError {
            *context = ::std::ptr::null_mut();
            let result_result = std::panic::catch_unwind(
                ::std::panic::AssertUnwindSafe(|| {
                    <SimpleService>::new_with_string(ascii)
                }),
            );
            match result_result {
                Ok(Ok(obj)) => {
                    let boxed = ::std::boxed::Box::new(obj);
                    let raw = ::std::boxed::Box::into_raw(boxed);
                    *context = raw;
                    <FFIError as ::interoptopus::patterns::result::FFIError>::SUCCESS
                }
                Ok(Err(e)) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Error in ({0}): {1:?}",
                                "simple_service_new_with_string",
                                e,
                            ),
                        );
                        res
                    });
                    e.into()
                }
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_new_with_string",
                                e,
                            ),
                        );
                        res
                    });
                    <FFIError as ::interoptopus::patterns::result::FFIError>::PANIC
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_new_with_string {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_new_with_string {
            type Signature = extern "C" fn(
                &mut *mut SimpleService,
                AsciiPointer,
            ) -> FFIError;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut *mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "ascii".to_string(),
                            <AsciiPointer as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_new_with_string".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_new_failing(
            context: &mut *mut SimpleService,
            _some_value: u8,
        ) -> FFIError {
            *context = ::std::ptr::null_mut();
            let result_result = std::panic::catch_unwind(
                ::std::panic::AssertUnwindSafe(|| {
                    <SimpleService>::new_failing(_some_value)
                }),
            );
            match result_result {
                Ok(Ok(obj)) => {
                    let boxed = ::std::boxed::Box::new(obj);
                    let raw = ::std::boxed::Box::into_raw(boxed);
                    *context = raw;
                    <FFIError as ::interoptopus::patterns::result::FFIError>::SUCCESS
                }
                Ok(Err(e)) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Error in ({0}): {1:?}",
                                "simple_service_new_failing",
                                e,
                            ),
                        );
                        res
                    });
                    e.into()
                }
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_new_failing",
                                e,
                            ),
                        );
                        res
                    });
                    <FFIError as ::interoptopus::patterns::result::FFIError>::PANIC
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_new_failing {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_new_failing {
            type Signature = extern "C" fn(&mut *mut SimpleService, u8) -> FFIError;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut *mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "some_value".to_string(),
                            <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_new_failing".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        /// Methods returning a Result<(), _> are the default and do not
        /// need annotations.
        pub extern "C" fn simple_service_method_result(
            context: &SimpleService,
            _anon1: u32,
        ) -> FFIError {
            ::interoptopus::patterns::result::panics_and_errors_to_ffi_enum(
                move || { <SimpleService>::method_result(context, _anon1) },
                "simple_service_method_result",
            )
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_method_result {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_method_result {
            type Signature = extern "C" fn(&SimpleService, u32) -> FFIError;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "anon1".to_string(),
                            <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                doc_lines
                    .push(
                        " Methods returning a Result<(), _> are the default and do not"
                            .to_string(),
                    );
                doc_lines.push(" need annotations.".to_string());
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_method_result".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_method_value(
            context: &SimpleService,
            x: u32,
        ) -> u32 {
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let context = context;
                    let x = x;
                    <SimpleService>::method_value(context, x)
                }),
            );
            match result_result {
                Ok(x) => x,
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_method_value",
                                e,
                            ),
                        );
                        res
                    });
                    <u32>::default()
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_method_value {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_method_value {
            type Signature = extern "C" fn(&SimpleService, u32) -> u32;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "x".to_string(),
                            <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_method_value".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        /// This method should be documented.
        ///
        /// Multiple lines.
        pub extern "C" fn simple_service_method_void(context: &SimpleService) -> () {
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let context = context;
                    <SimpleService>::method_void(context)
                }),
            );
            match result_result {
                Ok(x) => x,
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_method_void",
                                e,
                            ),
                        );
                        res
                    });
                    <()>::default()
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_method_void {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_method_void {
            type Signature = extern "C" fn(&SimpleService) -> ();
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                doc_lines.push(" This method should be documented.".to_string());
                doc_lines.push("".to_string());
                doc_lines.push(" Multiple lines.".to_string());
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    ::interoptopus::lang::c::CType::Primitive(
                        ::interoptopus::lang::c::PrimitiveType::Void,
                    ),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_method_void".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_method_mut_self(
            context: &mut SimpleService,
            slice: FFISlice<u8>,
        ) -> u8 {
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let context = context;
                    let slice = slice;
                    <SimpleService>::method_mut_self(context, slice)
                }),
            );
            match result_result {
                Ok(x) => x,
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_method_mut_self",
                                e,
                            ),
                        );
                        res
                    });
                    <u8>::default()
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_method_mut_self {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_method_mut_self {
            type Signature = extern "C" fn(&mut SimpleService, FFISlice<u8>) -> u8;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <FFISlice<
                                u8,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_method_mut_self".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        /// Single line.
        pub extern "C" fn simple_service_method_mut_self_void(
            context: &mut SimpleService,
            _slice: FFISlice<FFIBool>,
        ) -> () {
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let context = context;
                    let _slice = _slice;
                    <SimpleService>::method_mut_self_void(context, _slice)
                }),
            );
            match result_result {
                Ok(x) => x,
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_method_mut_self_void",
                                e,
                            ),
                        );
                        res
                    });
                    <()>::default()
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_method_mut_self_void {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_method_mut_self_void {
            type Signature = extern "C" fn(&mut SimpleService, FFISlice<FFIBool>) -> ();
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <FFISlice<
                                FFIBool,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                doc_lines.push(" Single line.".to_string());
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    ::interoptopus::lang::c::CType::Primitive(
                        ::interoptopus::lang::c::PrimitiveType::Void,
                    ),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_method_mut_self_void".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_method_mut_self_ref(
            context: &mut SimpleService,
            x: &u8,
            _y: &mut u8,
        ) -> u8 {
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let context = context;
                    let x = x;
                    let _y = _y;
                    <SimpleService>::method_mut_self_ref(context, x, _y)
                }),
            );
            match result_result {
                Ok(x) => x,
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_method_mut_self_ref",
                                e,
                            ),
                        );
                        res
                    });
                    <u8>::default()
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_method_mut_self_ref {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_method_mut_self_ref {
            type Signature = extern "C" fn(&mut SimpleService, &u8, &mut u8) -> u8;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "x".to_string(),
                            <&u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "y".to_string(),
                            <&mut u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_method_mut_self_ref".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_method_mut_self_ref_slice(
            context: &mut SimpleService,
            x: &u8,
            _y: &mut u8,
            _slice: FFISlice<u8>,
        ) -> u8 {
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let context = context;
                    let x = x;
                    let _y = _y;
                    let _slice = _slice;
                    <SimpleService>::method_mut_self_ref_slice(context, x, _y, _slice)
                }),
            );
            match result_result {
                Ok(x) => x,
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_method_mut_self_ref_slice",
                                e,
                            ),
                        );
                        res
                    });
                    <u8>::default()
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_method_mut_self_ref_slice {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_method_mut_self_ref_slice {
            type Signature = extern "C" fn(
                &mut SimpleService,
                &u8,
                &mut u8,
                FFISlice<u8>,
            ) -> u8;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "x".to_string(),
                            <&u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "y".to_string(),
                            <&mut u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <FFISlice<
                                u8,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_method_mut_self_ref_slice".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_method_mut_self_ref_slice_limited<'a, 'b>(
            context: &mut SimpleService,
            x: &u8,
            _y: &mut u8,
            _slice: FFISlice<'a, u8>,
            _slice2: FFISlice<'b, u8>,
        ) -> u8 {
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let context = context;
                    let x = x;
                    let _y = _y;
                    let _slice = _slice;
                    let _slice2 = _slice2;
                    <SimpleService>::method_mut_self_ref_slice_limited(
                        context,
                        x,
                        _y,
                        _slice,
                        _slice2,
                    )
                }),
            );
            match result_result {
                Ok(x) => x,
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_method_mut_self_ref_slice_limited",
                                e,
                            ),
                        );
                        res
                    });
                    <u8>::default()
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_method_mut_self_ref_slice_limited<'a, 'b> {
            a: ::std::marker::PhantomData<&'a ()>,
            b: ::std::marker::PhantomData<&'b ()>,
        }
        unsafe impl<'a, 'b> ::interoptopus::lang::rust::FunctionInfo
        for simple_service_method_mut_self_ref_slice_limited<'a, 'b> {
            type Signature = extern "C" fn(
                &mut SimpleService,
                &u8,
                &mut u8,
                FFISlice<'a, u8>,
                FFISlice<'b, u8>,
            ) -> u8;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "x".to_string(),
                            <&u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "y".to_string(),
                            <&mut u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <FFISlice<
                                u8,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice2".to_string(),
                            <FFISlice<
                                u8,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_method_mut_self_ref_slice_limited".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_method_mut_self_ffi_error(
            context: &mut SimpleService,
            _slice: FFISliceMut<u8>,
        ) -> FFIError {
            ::interoptopus::patterns::result::panics_and_errors_to_ffi_enum(
                move || { <SimpleService>::method_mut_self_ffi_error(context, _slice) },
                "simple_service_method_mut_self_ffi_error",
            )
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_method_mut_self_ffi_error {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_method_mut_self_ffi_error {
            type Signature = extern "C" fn(
                &mut SimpleService,
                FFISliceMut<u8>,
            ) -> FFIError;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <FFISliceMut<
                                u8,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_method_mut_self_ffi_error".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_method_mut_self_no_error(
            context: &mut SimpleService,
            mut slice: FFISliceMut<u8>,
        ) -> FFIError {
            ::interoptopus::patterns::result::panics_and_errors_to_ffi_enum(
                move || { <SimpleService>::method_mut_self_no_error(context, slice) },
                "simple_service_method_mut_self_no_error",
            )
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_method_mut_self_no_error {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_method_mut_self_no_error {
            type Signature = extern "C" fn(
                &mut SimpleService,
                FFISliceMut<u8>,
            ) -> FFIError;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <FFISliceMut<
                                u8,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_method_mut_self_no_error".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        /// Warning, you _must_ discard the returned slice object before calling into this service
        /// again, as otherwise undefined behavior might happen.
        pub extern "C" fn simple_service_return_slice(
            context: &mut SimpleService,
        ) -> FFISlice<u32> {
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let context = context;
                    <SimpleService>::return_slice(context)
                }),
            );
            match result_result {
                Ok(x) => x,
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_return_slice",
                                e,
                            ),
                        );
                        res
                    });
                    <FFISlice<u32>>::default()
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_return_slice {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_return_slice {
            type Signature = extern "C" fn(&mut SimpleService) -> FFISlice<u32>;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                doc_lines
                    .push(
                        " Warning, you _must_ discard the returned slice object before calling into this service"
                            .to_string(),
                    );
                doc_lines
                    .push(
                        " again, as otherwise undefined behavior might happen."
                            .to_string(),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFISlice<u32> as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_return_slice".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        /// Warning, you _must_ discard the returned slice object before calling into this service
        /// again, as otherwise undefined behavior might happen.
        pub extern "C" fn simple_service_return_slice_mut(
            context: &mut SimpleService,
        ) -> FFISliceMut<u32> {
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let context = context;
                    <SimpleService>::return_slice_mut(context)
                }),
            );
            match result_result {
                Ok(x) => x,
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_return_slice_mut",
                                e,
                            ),
                        );
                        res
                    });
                    <FFISliceMut<u32>>::default()
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_return_slice_mut {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_return_slice_mut {
            type Signature = extern "C" fn(&mut SimpleService) -> FFISliceMut<u32>;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                doc_lines
                    .push(
                        " Warning, you _must_ discard the returned slice object before calling into this service"
                            .to_string(),
                    );
                doc_lines
                    .push(
                        " again, as otherwise undefined behavior might happen."
                            .to_string(),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFISliceMut<
                        u32,
                    > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_return_slice_mut".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        /// This function has no panic safeguards. If it panics your host app will be in an undefined state.
        pub extern "C" fn simple_service_return_string(
            context: &mut SimpleService,
        ) -> AsciiPointer {
            <SimpleService>::return_string(context)
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_return_string {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_return_string {
            type Signature = extern "C" fn(&mut SimpleService) -> AsciiPointer;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                doc_lines
                    .push(
                        " This function has no panic safeguards. If it panics your host app will be in an undefined state."
                            .to_string(),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <AsciiPointer as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_return_string".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_method_void_ffi_error(
            context: &mut SimpleService,
        ) -> FFIError {
            ::interoptopus::patterns::result::panics_and_errors_to_ffi_enum(
                move || { <SimpleService>::method_void_ffi_error(context) },
                "simple_service_method_void_ffi_error",
            )
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_method_void_ffi_error {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_method_void_ffi_error {
            type Signature = extern "C" fn(&mut SimpleService) -> FFIError;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_method_void_ffi_error".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_method_callback(
            context: &mut SimpleService,
            callback: MyCallback,
        ) -> FFIError {
            ::interoptopus::patterns::result::panics_and_errors_to_ffi_enum(
                move || { <SimpleService>::method_callback(context, callback) },
                "simple_service_method_callback",
            )
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_method_callback {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_method_callback {
            type Signature = extern "C" fn(&mut SimpleService, MyCallback) -> FFIError;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "callback".to_string(),
                            <MyCallback as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_method_callback".to_string(),
                    signature,
                    meta,
                )
            }
        }
        /// Destroys the given instance.
        ///
        /// # Safety
        ///
        /// The passed parameter MUST have been created with the corresponding init function;
        /// passing any other value results in undefined behavior.
        #[allow(unused_mut, unsafe_op_in_unsafe_fn, unused_unsafe)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        #[no_mangle]
        pub unsafe extern "C" fn simple_service_destroy(
            context: &mut *mut SimpleService,
        ) -> FFIError {
            if context.is_null() {
                return <FFIError as ::interoptopus::patterns::result::FFIError>::NULL;
            }
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    unsafe { drop(::std::boxed::Box::from_raw(*context)) };
                }),
            );
            *context = ::std::ptr::null_mut();
            match result_result {
                Ok(_) => {
                    <FFIError as ::interoptopus::patterns::result::FFIError>::SUCCESS
                }
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_destroy",
                                e,
                            ),
                        );
                        res
                    });
                    <FFIError as ::interoptopus::patterns::result::FFIError>::PANIC
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_destroy {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for simple_service_destroy {
            type Signature = extern "C" fn(&mut *mut SimpleService) -> FFIError;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut *mut SimpleService as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                doc_lines.push(" Destroys the given instance.".to_string());
                doc_lines.push("".to_string());
                doc_lines.push(" # Safety".to_string());
                doc_lines.push("".to_string());
                doc_lines
                    .push(
                        " The passed parameter MUST have been created with the corresponding init function;"
                            .to_string(),
                    );
                doc_lines
                    .push(
                        " passing any other value results in undefined behavior."
                            .to_string(),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_destroy".to_string(),
                    signature,
                    meta,
                )
            }
        }
        impl ::interoptopus::patterns::LibraryPatternInfo for SimpleService {
            fn pattern_info() -> ::interoptopus::patterns::LibraryPattern {
                use ::interoptopus::lang::rust::CTypeInfo;
                use ::interoptopus::lang::rust::FunctionInfo;
                let mut methods = Vec::new();
                let mut ctors = Vec::new();
                {
                    use simple_service_method_result as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_method_value as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_method_void as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_method_mut_self as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_method_mut_self_void as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_method_mut_self_ref as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_method_mut_self_ref_slice as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_method_mut_self_ref_slice_limited as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_method_mut_self_ffi_error as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_method_mut_self_no_error as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_return_slice as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_return_slice_mut as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_return_string as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_method_void_ffi_error as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_method_callback as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_new_with as x;
                    ctors.push(x::function_info());
                }
                {
                    use simple_service_new_without as x;
                    ctors.push(x::function_info());
                }
                {
                    use simple_service_new_with_string as x;
                    ctors.push(x::function_info());
                }
                {
                    use simple_service_new_failing as x;
                    ctors.push(x::function_info());
                }
                let dtor = {
                    use simple_service_destroy as x;
                    x::function_info()
                };
                let service = ::interoptopus::patterns::service::Service::new(
                    ctors,
                    dtor,
                    methods,
                );
                service.assert_valid();
                ::interoptopus::patterns::LibraryPattern::Service(service)
            }
        }
        pub struct SimpleServiceLifetime<'a> {
            pub some_value: &'a u32,
        }
        unsafe impl<'a> ::interoptopus::lang::rust::CTypeInfo
        for SimpleServiceLifetime<'a> {
            fn type_info() -> ::interoptopus::lang::c::CType {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                    "".to_string(),
                    documentation,
                    None,
                );
                let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
                let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
                let name = {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "{0}{1}",
                            "SimpleServiceLifetime".to_string(),
                            generics.join(""),
                        ),
                    );
                    res
                };
                let mut rval = ::interoptopus::lang::c::OpaqueType::new(name, meta);
                ::interoptopus::lang::c::CType::Opaque(rval)
            }
        }
        impl<'a> SimpleServiceLifetime<'a> {
            pub fn new_with(some_value: &'a u32) -> Result<Self, Error> {
                Ok(Self { some_value })
            }
            pub fn method_lt(&mut self, _slice: FFISlice<'a, FFIBool>) {}
            pub fn method_lt2(&mut self, _slice: FFISlice<FFIBool>) {}
            pub fn return_string_accept_slice<'b>(
                _: &mut SimpleServiceLifetime<'b>,
                _: FFISlice<'b, u8>,
            ) -> AsciiPointer<'b> {
                AsciiPointer::empty()
            }
            pub fn method_void_ffi_error(&mut self) -> Result<(), Error> {
                Ok(())
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_lt_new_with<'a>(
            context: &mut *mut SimpleServiceLifetime<'a>,
            some_value: &'a u32,
        ) -> FFIError {
            *context = ::std::ptr::null_mut();
            let result_result = std::panic::catch_unwind(
                ::std::panic::AssertUnwindSafe(|| {
                    <SimpleServiceLifetime<'a>>::new_with(some_value)
                }),
            );
            match result_result {
                Ok(Ok(obj)) => {
                    let boxed = ::std::boxed::Box::new(obj);
                    let raw = ::std::boxed::Box::into_raw(boxed);
                    *context = raw;
                    <FFIError as ::interoptopus::patterns::result::FFIError>::SUCCESS
                }
                Ok(Err(e)) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Error in ({0}): {1:?}",
                                "simple_service_lt_new_with",
                                e,
                            ),
                        );
                        res
                    });
                    e.into()
                }
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_lt_new_with",
                                e,
                            ),
                        );
                        res
                    });
                    <FFIError as ::interoptopus::patterns::result::FFIError>::PANIC
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_lt_new_with<'a> {
            a: ::std::marker::PhantomData<&'a ()>,
        }
        unsafe impl<'a> ::interoptopus::lang::rust::FunctionInfo
        for simple_service_lt_new_with<'a> {
            type Signature = extern "C" fn(
                &mut *mut SimpleServiceLifetime<'a>,
                &'a u32,
            ) -> FFIError;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut *mut SimpleServiceLifetime as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "some_value".to_string(),
                            <&u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_lt_new_with".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_lt_method_lt<'a>(
            context: &mut SimpleServiceLifetime<'a>,
            _slice: FFISlice<'a, FFIBool>,
        ) -> () {
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let context = context;
                    let _slice = _slice;
                    <SimpleServiceLifetime>::method_lt(context, _slice)
                }),
            );
            match result_result {
                Ok(x) => x,
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_lt_method_lt",
                                e,
                            ),
                        );
                        res
                    });
                    <()>::default()
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_lt_method_lt<'a> {
            a: ::std::marker::PhantomData<&'a ()>,
        }
        unsafe impl<'a> ::interoptopus::lang::rust::FunctionInfo
        for simple_service_lt_method_lt<'a> {
            type Signature = extern "C" fn(
                &mut SimpleServiceLifetime<'a>,
                FFISlice<'a, FFIBool>,
            ) -> ();
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleServiceLifetime as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <FFISlice<
                                FFIBool,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    ::interoptopus::lang::c::CType::Primitive(
                        ::interoptopus::lang::c::PrimitiveType::Void,
                    ),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_lt_method_lt".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_lt_method_lt2<'a>(
            context: &mut SimpleServiceLifetime<'a>,
            _slice: FFISlice<FFIBool>,
        ) -> () {
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let context = context;
                    let _slice = _slice;
                    <SimpleServiceLifetime>::method_lt2(context, _slice)
                }),
            );
            match result_result {
                Ok(x) => x,
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_lt_method_lt2",
                                e,
                            ),
                        );
                        res
                    });
                    <()>::default()
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_lt_method_lt2<'a> {
            a: ::std::marker::PhantomData<&'a ()>,
        }
        unsafe impl<'a> ::interoptopus::lang::rust::FunctionInfo
        for simple_service_lt_method_lt2<'a> {
            type Signature = extern "C" fn(
                &mut SimpleServiceLifetime<'a>,
                FFISlice<FFIBool>,
            ) -> ();
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleServiceLifetime as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <FFISlice<
                                FFIBool,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    ::interoptopus::lang::c::CType::Primitive(
                        ::interoptopus::lang::c::PrimitiveType::Void,
                    ),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_lt_method_lt2".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_lt_return_string_accept_slice<'b, 'a>(
            _anon0: &mut SimpleServiceLifetime<'b>,
            _anon1: FFISlice<'b, u8>,
        ) -> AsciiPointer<'b> {
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _anon0 = _anon0;
                    let _anon1 = _anon1;
                    <SimpleServiceLifetime>::return_string_accept_slice(_anon0, _anon1)
                }),
            );
            match result_result {
                Ok(x) => x,
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_lt_return_string_accept_slice",
                                e,
                            ),
                        );
                        res
                    });
                    <AsciiPointer<'b>>::default()
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_lt_return_string_accept_slice<'b, 'a> {
            b: ::std::marker::PhantomData<&'b ()>,
            a: ::std::marker::PhantomData<&'a ()>,
        }
        unsafe impl<'b, 'a> ::interoptopus::lang::rust::FunctionInfo
        for simple_service_lt_return_string_accept_slice<'b, 'a> {
            type Signature = extern "C" fn(
                &mut SimpleServiceLifetime<'b>,
                FFISlice<'b, u8>,
            ) -> AsciiPointer<'b>;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "anon0".to_string(),
                            <&mut SimpleServiceLifetime as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "anon1".to_string(),
                            <FFISlice<
                                u8,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <AsciiPointer<
                        'b,
                    > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_lt_return_string_accept_slice".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        pub extern "C" fn simple_service_lt_method_void_ffi_error<'a>(
            context: &mut SimpleServiceLifetime<'a>,
        ) -> FFIError {
            ::interoptopus::patterns::result::panics_and_errors_to_ffi_enum(
                move || { <SimpleServiceLifetime>::method_void_ffi_error(context) },
                "simple_service_lt_method_void_ffi_error",
            )
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_lt_method_void_ffi_error<'a> {
            a: ::std::marker::PhantomData<&'a ()>,
        }
        unsafe impl<'a> ::interoptopus::lang::rust::FunctionInfo
        for simple_service_lt_method_void_ffi_error<'a> {
            type Signature = extern "C" fn(&mut SimpleServiceLifetime<'a>) -> FFIError;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut SimpleServiceLifetime as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_lt_method_void_ffi_error".to_string(),
                    signature,
                    meta,
                )
            }
        }
        /// Destroys the given instance.
        ///
        /// # Safety
        ///
        /// The passed parameter MUST have been created with the corresponding init function;
        /// passing any other value results in undefined behavior.
        #[allow(unused_mut, unsafe_op_in_unsafe_fn, unused_unsafe)]
        #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
        #[no_mangle]
        pub unsafe extern "C" fn simple_service_lt_destroy(
            context: &mut *mut SimpleServiceLifetime,
        ) -> FFIError {
            if context.is_null() {
                return <FFIError as ::interoptopus::patterns::result::FFIError>::NULL;
            }
            let result_result = ::std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    unsafe { drop(::std::boxed::Box::from_raw(*context)) };
                }),
            );
            *context = ::std::ptr::null_mut();
            match result_result {
                Ok(_) => {
                    <FFIError as ::interoptopus::patterns::result::FFIError>::SUCCESS
                }
                Err(e) => {
                    ::interoptopus::util::log_error(|| {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Panic in ({0}): {1:?}",
                                "simple_service_lt_destroy",
                                e,
                            ),
                        );
                        res
                    });
                    <FFIError as ::interoptopus::patterns::result::FFIError>::PANIC
                }
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct simple_service_lt_destroy {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for simple_service_lt_destroy {
            type Signature = extern "C" fn(&mut *mut SimpleServiceLifetime) -> FFIError;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "context".to_string(),
                            <&mut *mut SimpleServiceLifetime as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                doc_lines.push(" Destroys the given instance.".to_string());
                doc_lines.push("".to_string());
                doc_lines.push(" # Safety".to_string());
                doc_lines.push("".to_string());
                doc_lines
                    .push(
                        " The passed parameter MUST have been created with the corresponding init function;"
                            .to_string(),
                    );
                doc_lines
                    .push(
                        " passing any other value results in undefined behavior."
                            .to_string(),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <FFIError as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "simple_service_lt_destroy".to_string(),
                    signature,
                    meta,
                )
            }
        }
        impl<'a> ::interoptopus::patterns::LibraryPatternInfo
        for SimpleServiceLifetime<'a> {
            fn pattern_info() -> ::interoptopus::patterns::LibraryPattern {
                use ::interoptopus::lang::rust::CTypeInfo;
                use ::interoptopus::lang::rust::FunctionInfo;
                let mut methods = Vec::new();
                let mut ctors = Vec::new();
                {
                    use simple_service_lt_method_lt as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_lt_method_lt2 as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_lt_return_string_accept_slice as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_lt_method_void_ffi_error as x;
                    methods.push(x::function_info());
                }
                {
                    use simple_service_lt_new_with as x;
                    ctors.push(x::function_info());
                }
                let dtor = {
                    use simple_service_lt_destroy as x;
                    x::function_info()
                };
                let service = ::interoptopus::patterns::service::Service::new(
                    ctors,
                    dtor,
                    methods,
                );
                service.assert_valid();
                ::interoptopus::patterns::LibraryPattern::Service(service)
            }
        }
    }
    pub mod slice {
        use crate::types::{CallbackFFISlice, Vec3f32};
        use interoptopus::patterns::slice::{FFISlice, FFISliceMut};
        use interoptopus::{callback, ffi_function};
        static HUGE_VEC_SLICE: [Vec3f32; 100_000] = [Vec3f32 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }; 100_000];
        #[repr(transparent)]
        pub struct CallbackHugeVecSlice(
            Option<extern "C" fn(FFISlice<Vec3f32>) -> Vec3f32>,
        );
        #[automatically_derived]
        impl ::core::default::Default for CallbackHugeVecSlice {
            #[inline]
            fn default() -> CallbackHugeVecSlice {
                CallbackHugeVecSlice(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CallbackHugeVecSlice {
            #[inline]
            fn clone(&self) -> CallbackHugeVecSlice {
                CallbackHugeVecSlice(::core::clone::Clone::clone(&self.0))
            }
        }
        impl CallbackHugeVecSlice {
            /// Will call function if it exists, panic otherwise.
            pub fn call(&self, slice: FFISlice<Vec3f32>) -> Vec3f32 {
                self.0.expect("Assumed function would exist but it didn't.")(slice)
            }
            /// Will call function only if it exists
            pub fn call_if_some(&self, slice: FFISlice<Vec3f32>) -> Option<Vec3f32> {
                match self.0 {
                    Some(c) => Some(c(slice)),
                    None => None,
                }
            }
        }
        impl From<extern "C" fn(FFISlice<Vec3f32>) -> Vec3f32> for CallbackHugeVecSlice {
            fn from(x: extern "C" fn(FFISlice<Vec3f32>) -> Vec3f32) -> Self {
                Self(Some(x))
            }
        }
        unsafe impl interoptopus::lang::rust::CTypeInfo for CallbackHugeVecSlice {
            fn type_info() -> interoptopus::lang::c::CType {
                use interoptopus::lang::rust::CTypeInfo;
                let rval = <Vec3f32 as CTypeInfo>::type_info();
                let params = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <FFISlice<Vec3f32> as CTypeInfo>::type_info(),
                        ),
                    ]),
                );
                let sig = interoptopus::lang::c::FunctionSignature::new(params, rval);
                let fn_pointer = interoptopus::lang::c::FnPointerType::new(sig);
                let named_callback = interoptopus::patterns::callbacks::NamedCallback::new(
                    "CallbackHugeVecSlice".to_string(),
                    fn_pointer,
                );
                interoptopus::lang::c::CType::Pattern(
                    interoptopus::patterns::TypePattern::NamedCallback(named_callback),
                )
            }
        }
        #[repr(transparent)]
        pub struct CallbackSliceMut(Option<extern "C" fn(FFISliceMut<'_, u8>) -> ()>);
        #[automatically_derived]
        impl ::core::default::Default for CallbackSliceMut {
            #[inline]
            fn default() -> CallbackSliceMut {
                CallbackSliceMut(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CallbackSliceMut {
            #[inline]
            fn clone(&self) -> CallbackSliceMut {
                CallbackSliceMut(::core::clone::Clone::clone(&self.0))
            }
        }
        impl CallbackSliceMut {
            /// Will call function if it exists, panic otherwise.
            pub fn call(&self, slice: FFISliceMut<'_, u8>) -> () {
                self.0.expect("Assumed function would exist but it didn't.")(slice)
            }
            /// Will call function only if it exists
            pub fn call_if_some(&self, slice: FFISliceMut<'_, u8>) -> Option<()> {
                match self.0 {
                    Some(c) => Some(c(slice)),
                    None => None,
                }
            }
        }
        impl From<extern "C" fn(FFISliceMut<'_, u8>) -> ()> for CallbackSliceMut {
            fn from(x: extern "C" fn(FFISliceMut<'_, u8>) -> ()) -> Self {
                Self(Some(x))
            }
        }
        unsafe impl interoptopus::lang::rust::CTypeInfo for CallbackSliceMut {
            fn type_info() -> interoptopus::lang::c::CType {
                use interoptopus::lang::rust::CTypeInfo;
                let rval = <() as CTypeInfo>::type_info();
                let params = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <FFISliceMut<'_, u8> as CTypeInfo>::type_info(),
                        ),
                    ]),
                );
                let sig = interoptopus::lang::c::FunctionSignature::new(params, rval);
                let fn_pointer = interoptopus::lang::c::FnPointerType::new(sig);
                let named_callback = interoptopus::patterns::callbacks::NamedCallback::new(
                    "CallbackSliceMut".to_string(),
                    fn_pointer,
                );
                interoptopus::lang::c::CType::Pattern(
                    interoptopus::patterns::TypePattern::NamedCallback(named_callback),
                )
            }
        }
        #[repr(transparent)]
        pub struct CallbackU8(Option<extern "C" fn(u8) -> u8>);
        #[automatically_derived]
        impl ::core::default::Default for CallbackU8 {
            #[inline]
            fn default() -> CallbackU8 {
                CallbackU8(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CallbackU8 {
            #[inline]
            fn clone(&self) -> CallbackU8 {
                CallbackU8(::core::clone::Clone::clone(&self.0))
            }
        }
        impl CallbackU8 {
            /// Will call function if it exists, panic otherwise.
            pub fn call(&self, value: u8) -> u8 {
                self.0.expect("Assumed function would exist but it didn't.")(value)
            }
            /// Will call function only if it exists
            pub fn call_if_some(&self, value: u8) -> Option<u8> {
                match self.0 {
                    Some(c) => Some(c(value)),
                    None => None,
                }
            }
        }
        impl From<extern "C" fn(u8) -> u8> for CallbackU8 {
            fn from(x: extern "C" fn(u8) -> u8) -> Self {
                Self(Some(x))
            }
        }
        unsafe impl interoptopus::lang::rust::CTypeInfo for CallbackU8 {
            fn type_info() -> interoptopus::lang::c::CType {
                use interoptopus::lang::rust::CTypeInfo;
                let rval = <u8 as CTypeInfo>::type_info();
                let params = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        interoptopus::lang::c::Parameter::new(
                            "value".to_string(),
                            <u8 as CTypeInfo>::type_info(),
                        ),
                    ]),
                );
                let sig = interoptopus::lang::c::FunctionSignature::new(params, rval);
                let fn_pointer = interoptopus::lang::c::FnPointerType::new(sig);
                let named_callback = interoptopus::patterns::callbacks::NamedCallback::new(
                    "CallbackU8".to_string(),
                    fn_pointer,
                );
                interoptopus::lang::c::CType::Pattern(
                    interoptopus::patterns::TypePattern::NamedCallback(named_callback),
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_slice_1(ffi_slice: FFISlice<u32>) -> u32 {
            ffi_slice.as_slice().len() as u32
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_slice_1 {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_ffi_slice_1 {
            type Signature = extern "C" fn(FFISlice<u32>) -> u32;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "ffi_slice".to_string(),
                            <FFISlice<
                                u32,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_slice_1".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_slice_1b(ffi_slice: FFISliceMut<u32>) -> u32 {
            ffi_slice.as_slice().len() as u32
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_slice_1b {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_ffi_slice_1b {
            type Signature = extern "C" fn(FFISliceMut<u32>) -> u32;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "ffi_slice".to_string(),
                            <FFISliceMut<
                                u32,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_slice_1b".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_slice_2(
            ffi_slice: FFISlice<Vec3f32>,
            i: i32,
        ) -> Vec3f32 {
            ffi_slice
                .as_slice()
                .get(i as usize)
                .copied()
                .unwrap_or(Vec3f32 {
                    x: f32::NAN,
                    y: f32::NAN,
                    z: f32::NAN,
                })
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_slice_2 {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_ffi_slice_2 {
            type Signature = extern "C" fn(FFISlice<Vec3f32>, i32) -> Vec3f32;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "ffi_slice".to_string(),
                            <FFISlice<
                                Vec3f32,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "i".to_string(),
                            <i32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <Vec3f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_slice_2".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_slice_delegate(callback: CallbackFFISlice) -> u8 {
            callback.call(FFISlice::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_slice_delegate {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for pattern_ffi_slice_delegate {
            type Signature = extern "C" fn(CallbackFFISlice) -> u8;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "callback".to_string(),
                            <CallbackFFISlice as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_slice_delegate".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_slice_delegate_huge(
            callback: CallbackHugeVecSlice,
        ) -> Vec3f32 {
            callback.call(FFISlice::from_slice(&HUGE_VEC_SLICE))
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_slice_delegate_huge {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo
        for pattern_ffi_slice_delegate_huge {
            type Signature = extern "C" fn(CallbackHugeVecSlice) -> Vec3f32;
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "callback".to_string(),
                            <CallbackHugeVecSlice as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    <Vec3f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_slice_delegate_huge".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_slice_3(
            mut slice: FFISliceMut<u8>,
            callback: CallbackSliceMut,
        ) {
            if let [x, ..] = slice.as_slice_mut() {
                *x += 1;
            }
            callback.call(slice);
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_slice_3 {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_ffi_slice_3 {
            type Signature = extern "C" fn(FFISliceMut<u8>, CallbackSliceMut);
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <FFISliceMut<
                                u8,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "callback".to_string(),
                            <CallbackSliceMut as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    ::interoptopus::lang::c::CType::Primitive(
                        interoptopus::lang::c::PrimitiveType::Void,
                    ),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_slice_3".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_slice_4(
            _slice: FFISlice<u8>,
            _slice2: FFISliceMut<u8>,
        ) {}
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_slice_4 {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_ffi_slice_4 {
            type Signature = extern "C" fn(FFISlice<u8>, FFISliceMut<u8>);
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <FFISlice<
                                u8,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice2".to_string(),
                            <FFISliceMut<
                                u8,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    ::interoptopus::lang::c::CType::Primitive(
                        interoptopus::lang::c::PrimitiveType::Void,
                    ),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_slice_4".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_slice_5(
            slice: &FFISlice<u8>,
            slice2: &mut FFISliceMut<u8>,
        ) {
            let _ = slice.as_slice().len();
            let _ = slice2.as_slice().len();
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_slice_5 {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_ffi_slice_5 {
            type Signature = extern "C" fn(&FFISlice<u8>, &mut FFISliceMut<u8>);
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <&FFISlice<
                                u8,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice2".to_string(),
                            <&mut FFISliceMut<
                                u8,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    ::interoptopus::lang::c::CType::Primitive(
                        interoptopus::lang::c::PrimitiveType::Void,
                    ),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_slice_5".to_string(),
                    signature,
                    meta,
                )
            }
        }
        #[no_mangle]
        pub extern "C" fn pattern_ffi_slice_6(
            slice: &FFISliceMut<u8>,
            callback: CallbackU8,
        ) {
            callback.call(slice.as_slice().first().copied().unwrap_or(0));
        }
        #[allow(non_camel_case_types)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct pattern_ffi_slice_6 {}
        unsafe impl ::interoptopus::lang::rust::FunctionInfo for pattern_ffi_slice_6 {
            type Signature = extern "C" fn(&FFISliceMut<u8>, CallbackU8);
            fn function_info() -> ::interoptopus::lang::c::Function {
                let mut doc_lines = ::std::vec::Vec::new();
                let mut params = ::std::vec::Vec::new();
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "slice".to_string(),
                            <&FFISliceMut<
                                u8,
                            > as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                params
                    .push(
                        ::interoptopus::lang::c::Parameter::new(
                            "callback".to_string(),
                            <CallbackU8 as ::interoptopus::lang::rust::CTypeInfo>::type_info(),
                        ),
                    );
                let mut signature = ::interoptopus::lang::c::FunctionSignature::new(
                    params,
                    ::interoptopus::lang::c::CType::Primitive(
                        interoptopus::lang::c::PrimitiveType::Void,
                    ),
                );
                let documentation = ::interoptopus::lang::c::Documentation::from_lines(
                    doc_lines,
                );
                let meta = ::interoptopus::lang::c::Meta::with_documentation(
                    documentation,
                    None,
                );
                ::interoptopus::lang::c::Function::new(
                    "pattern_ffi_slice_6".to_string(),
                    signature,
                    meta,
                )
            }
        }
    }
}
pub mod types {
    //! All supported type patterns.
    use interoptopus::lang::c::{CType, CompositeType, Field, PrimitiveType};
    use interoptopus::lang::rust::CTypeInfo;
    use interoptopus::patterns::slice::FFISlice;
    use interoptopus::patterns::string::AsciiPointer;
    use interoptopus::{callback, ffi_surrogates, ffi_type};
    use std::fmt::Debug;
    use std::marker::PhantomData;
    pub trait Helper {}
    impl Helper for u8 {}
    #[repr(C)]
    pub struct SomeForeignType {
        x: u32,
    }
    pub fn some_foreign_type() -> CType {
        let composite = CompositeType::new(
            "SomeForeignType".to_string(),
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    Field::new("x".to_string(), CType::Primitive(PrimitiveType::U32)),
                ]),
            ),
        );
        CType::Composite(composite)
    }
    /// Empty structs are only allowed as opaques.
    #[repr(C)]
    pub struct Empty {}
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Empty {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line(
                " Empty structs are only allowed as opaques.",
            );
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Empty".to_string(), generics.join("")),
                );
                res
            };
            let mut rval = ::interoptopus::lang::c::OpaqueType::new(name, meta);
            ::interoptopus::lang::c::CType::Opaque(rval)
        }
    }
    pub struct Opaque {
        _internal: *const Vec3f32,
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Opaque {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Opaque".to_string(), generics.join("")),
                );
                res
            };
            let mut rval = ::interoptopus::lang::c::OpaqueType::new(name, meta);
            ::interoptopus::lang::c::CType::Opaque(rval)
        }
    }
    #[repr(C)]
    pub struct Tupled(pub u8);
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Tupled {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Tupled".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "x0".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(transparent)]
    pub struct Transparent(Tupled);
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Transparent {
        fn type_info() -> ::interoptopus::lang::c::CType {
            <Tupled>::type_info()
        }
    }
    #[repr(C)]
    pub struct Generic<'a, T>
    where
        T: 'static,
        T: CTypeInfo,
    {
        pub x: &'a T,
    }
    unsafe impl<'a, T> ::interoptopus::lang::rust::CTypeInfo for Generic<'a, T>
    where
        T: interoptopus::lang::rust::CTypeInfo,
        T: 'static,
        T: CTypeInfo,
    {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            {
                generics
                    .push(
                        <T as ::interoptopus::lang::rust::CTypeInfo>::type_info()
                            .rust_name(),
                    );
            }
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Generic".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <&'a T as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "x".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C)]
    pub struct Generic2<T>
    where
        T: CTypeInfo,
    {
        pub x: T,
    }
    unsafe impl<T> ::interoptopus::lang::rust::CTypeInfo for Generic2<T>
    where
        T: interoptopus::lang::rust::CTypeInfo,
        T: CTypeInfo,
    {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            {
                generics
                    .push(
                        <T as ::interoptopus::lang::rust::CTypeInfo>::type_info()
                            .rust_name(),
                    );
            }
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Generic2".to_string(), generics.join("")),
                );
                res
            };
            let mut rval = ::interoptopus::lang::c::OpaqueType::new(name, meta);
            ::interoptopus::lang::c::CType::Opaque(rval)
        }
    }
    #[repr(C)]
    pub struct Generic3<T> {
        pub x: T,
    }
    unsafe impl<T> ::interoptopus::lang::rust::CTypeInfo for Generic3<T>
    where
        T: interoptopus::lang::rust::CTypeInfo,
    {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = "Generic3".to_string();
            let mut rval = ::interoptopus::lang::c::OpaqueType::new(name, meta);
            ::interoptopus::lang::c::CType::Opaque(rval)
        }
    }
    #[repr(C)]
    pub struct Generic4<T>
    where
        T: Helper,
    {
        pub x: T,
    }
    unsafe impl<T> ::interoptopus::lang::rust::CTypeInfo for Generic4<T>
    where
        T: interoptopus::lang::rust::CTypeInfo,
        T: Helper,
    {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = "Generic4".to_string();
            let mut rval = ::interoptopus::lang::c::OpaqueType::new(name, meta);
            ::interoptopus::lang::c::CType::Opaque(rval)
        }
    }
    #[repr(C)]
    pub struct StructRenamedXYZ {
        pub e: EnumRenamedXYZ,
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for StructRenamedXYZ {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = "StructRenamed".to_string();
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <EnumRenamedXYZ as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "e".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C)]
    pub struct Phantom<'a, T>
    where
        T: 'static,
        T: CTypeInfo,
    {
        pub x: u32,
        pub p: PhantomData<&'a T>,
    }
    unsafe impl<'a, T> ::interoptopus::lang::rust::CTypeInfo for Phantom<'a, T>
    where
        T: interoptopus::lang::rust::CTypeInfo,
        T: 'static,
        T: CTypeInfo,
    {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            {
                generics
                    .push(
                        <T as ::interoptopus::lang::rust::CTypeInfo>::type_info()
                            .rust_name(),
                    );
            }
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Phantom".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u32 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "x".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C)]
    pub struct Vec3f32 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Vec3f32 {}
    #[automatically_derived]
    impl ::core::clone::Clone for Vec3f32 {
        #[inline]
        fn clone(&self) -> Vec3f32 {
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Vec3f32 {
        #[inline]
        fn default() -> Vec3f32 {
            Vec3f32 {
                x: ::core::default::Default::default(),
                y: ::core::default::Default::default(),
                z: ::core::default::Default::default(),
            }
        }
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Vec3f32 {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Vec3f32".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "x".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "y".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "z".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C)]
    pub struct Container {
        pub foreign1: SomeForeignType,
        pub foreign2: SomeForeignType,
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Container {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Container".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = some_foreign_type();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "foreign1".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = some_foreign_type();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "foreign2".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C)]
    pub struct Array {
        pub data: [u8; 16],
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Array {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Array".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <[u8; 16] as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "data".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C)]
    pub struct GenericArray<T>
    where
        T: CTypeInfo,
    {
        pub data: [T; 16],
    }
    unsafe impl<T> ::interoptopus::lang::rust::CTypeInfo for GenericArray<T>
    where
        T: interoptopus::lang::rust::CTypeInfo,
        T: CTypeInfo,
    {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            {
                generics
                    .push(
                        <T as ::interoptopus::lang::rust::CTypeInfo>::type_info()
                            .rust_name(),
                    );
            }
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "GenericArray".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <[T; 16] as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "data".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    /// Documented enum.
    #[repr(C)]
    pub enum EnumDocumented {
        /// Variant A.
        A,
        /// Variant B.
        B,
        /// Variant B.
        C,
    }
    unsafe impl ::interoptopus::lang::rust::VariantInfo for EnumDocumented {
        fn variant_info(&self) -> ::interoptopus::lang::c::Variant {
            match self {
                Self::A => {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        " Variant A.",
                    );
                    ::interoptopus::lang::c::Variant::new(
                        "A".to_string(),
                        0i32 as usize,
                        documentation,
                    )
                }
                Self::B => {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        " Variant B.",
                    );
                    ::interoptopus::lang::c::Variant::new(
                        "B".to_string(),
                        1i32 as usize,
                        documentation,
                    )
                }
                Self::C => {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        " Variant B.",
                    );
                    ::interoptopus::lang::c::Variant::new(
                        "C".to_string(),
                        2i32 as usize,
                        documentation,
                    )
                }
            }
        }
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for EnumDocumented {
        fn type_info() -> ::interoptopus::lang::c::CType {
            use ::interoptopus::lang::rust::VariantInfo;
            let mut variants = ::std::vec::Vec::new();
            let documentation = ::interoptopus::lang::c::Documentation::from_line(
                " Documented enum.",
            );
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            {
                variants.push(Self::A.variant_info());
            }
            {
                variants.push(Self::B.variant_info());
            }
            {
                variants.push(Self::C.variant_info());
            }
            let rval = ::interoptopus::lang::c::EnumType::new(
                "EnumDocumented".to_string(),
                variants,
                meta,
            );
            ::interoptopus::lang::c::CType::Enum(rval)
        }
    }
    #[repr(C)]
    pub enum EnumRenamedXYZ {
        X,
    }
    unsafe impl ::interoptopus::lang::rust::VariantInfo for EnumRenamedXYZ {
        fn variant_info(&self) -> ::interoptopus::lang::c::Variant {
            match self {
                Self::X => {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    ::interoptopus::lang::c::Variant::new(
                        "X".to_string(),
                        0i32 as usize,
                        documentation,
                    )
                }
            }
        }
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for EnumRenamedXYZ {
        fn type_info() -> ::interoptopus::lang::c::CType {
            use ::interoptopus::lang::rust::VariantInfo;
            let mut variants = ::std::vec::Vec::new();
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            {
                variants.push(Self::X.variant_info());
            }
            let rval = ::interoptopus::lang::c::EnumType::new(
                "EnumRenamed".to_string(),
                variants,
                meta,
            );
            ::interoptopus::lang::c::CType::Enum(rval)
        }
    }
    /// Documented struct.
    #[repr(C)]
    pub struct StructDocumented {
        /// Documented field.
        pub x: f32,
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for StructDocumented {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line(
                " Documented struct.",
            );
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!(
                        "{0}{1}",
                        "StructDocumented".to_string(),
                        generics.join(""),
                    ),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    " Documented field.",
                );
                let the_type = <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "x".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C)]
    pub struct ExtraType<T> {
        pub x: T,
    }
    unsafe impl<T> ::interoptopus::lang::rust::CTypeInfo for ExtraType<T>
    where
        T: interoptopus::lang::rust::CTypeInfo,
    {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            {
                generics
                    .push(
                        <T as ::interoptopus::lang::rust::CTypeInfo>::type_info()
                            .rust_name(),
                    );
            }
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "ExtraType".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <T as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "x".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C)]
    pub struct UseAsciiStringPattern<'a> {
        pub ascii_string: AsciiPointer<'a>,
    }
    unsafe impl<'a> ::interoptopus::lang::rust::CTypeInfo for UseAsciiStringPattern<'a> {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!(
                        "{0}{1}",
                        "UseAsciiStringPattern".to_string(),
                        generics.join(""),
                    ),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <AsciiPointer<
                    'a,
                > as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "ascii_string".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    /// This can also be used for the `class` pattern.
    #[repr(C)]
    pub struct SomeContext {
        pub(crate) some_field: u32,
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for SomeContext {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line(
                " This can also be used for the `class` pattern.",
            );
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "SomeContext".to_string(), generics.join("")),
                );
                res
            };
            let mut rval = ::interoptopus::lang::c::OpaqueType::new(name, meta);
            ::interoptopus::lang::c::CType::Opaque(rval)
        }
    }
    #[repr(C)]
    pub struct Weird1<T: Clone>
    where
        T: Copy + Copy,
    {
        x: T,
    }
    unsafe impl<T> ::interoptopus::lang::rust::CTypeInfo for Weird1<T>
    where
        T: interoptopus::lang::rust::CTypeInfo,
        T: Clone,
        T: Copy + Copy,
    {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            {
                generics
                    .push(
                        <T as ::interoptopus::lang::rust::CTypeInfo>::type_info()
                            .rust_name(),
                    );
            }
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Weird1".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <T as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "x".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Private,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C)]
    pub struct Weird2<'a, T: Clone, const N: usize>
    where
        T: Copy + Copy + 'a,
        T: Debug,
    {
        t: T,
        a: [T; N],
        r: &'a u8,
    }
    unsafe impl<'a, T, const N: usize> ::interoptopus::lang::rust::CTypeInfo
    for Weird2<'a, T, N>
    where
        T: interoptopus::lang::rust::CTypeInfo,
        T: Clone,
        T: Copy + Copy + 'a,
        T: Debug,
    {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            {
                generics
                    .push(
                        <T as ::interoptopus::lang::rust::CTypeInfo>::type_info()
                            .rust_name(),
                    );
            }
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Weird2".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <T as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "t".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Private,
                    documentation,
                );
                fields.push(field);
            }
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <[T; N] as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "a".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Private,
                    documentation,
                );
                fields.push(field);
            }
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <&'a u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "r".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Private,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C)]
    pub struct Visibility1 {
        pblc: u8,
        pub prvt: u8,
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Visibility1 {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Visibility1".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "pblc".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "prvt".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Private,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C)]
    pub struct Visibility2 {
        pblc1: u8,
        pblc2: u8,
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Visibility2 {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                None,
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Visibility2".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "pblc1".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "pblc2".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C)]
    #[repr(packed)]
    pub struct Packed1 {
        pub x: u8,
        pub y: u16,
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Packed1 {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                Some(1usize),
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Packed1".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "x".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u16 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "y".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C, packed)]
    pub struct Packed2 {
        pub x: u8,
        pub y: u16,
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Packed2 {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                Some(1usize),
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Packed2".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "x".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u16 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "y".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C)]
    #[repr(align(2))]
    pub struct Align1 {
        pub x: u8,
        pub y: u16,
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Align1 {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                Some(2usize),
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Align1".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "x".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u16 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "y".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    #[repr(C, align(64))]
    pub struct Align2 {
        pub x: u8,
        pub y: u16,
    }
    unsafe impl ::interoptopus::lang::rust::CTypeInfo for Align2 {
        fn type_info() -> ::interoptopus::lang::c::CType {
            let documentation = ::interoptopus::lang::c::Documentation::from_line("");
            let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                "".to_string(),
                documentation,
                Some(64usize),
            );
            let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
            let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
            let name = {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", "Align2".to_string(), generics.join("")),
                );
                res
            };
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "x".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let the_type = <u16 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                let field = ::interoptopus::lang::c::Field::with_documentation(
                    "y".to_string(),
                    the_type,
                    interoptopus::lang::c::Visibility::Public,
                    documentation,
                );
                fields.push(field);
            }
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                name,
                fields,
                meta,
            );
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    }
    pub type Callbacku8u8 = extern "C" fn(u8) -> u8;
    #[repr(transparent)]
    pub struct CallbackFFISlice(Option<extern "C" fn(FFISlice<u8>) -> u8>);
    #[automatically_derived]
    impl ::core::default::Default for CallbackFFISlice {
        #[inline]
        fn default() -> CallbackFFISlice {
            CallbackFFISlice(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CallbackFFISlice {
        #[inline]
        fn clone(&self) -> CallbackFFISlice {
            CallbackFFISlice(::core::clone::Clone::clone(&self.0))
        }
    }
    impl CallbackFFISlice {
        /// Will call function if it exists, panic otherwise.
        pub fn call(&self, slice: FFISlice<u8>) -> u8 {
            self.0.expect("Assumed function would exist but it didn't.")(slice)
        }
        /// Will call function only if it exists
        pub fn call_if_some(&self, slice: FFISlice<u8>) -> Option<u8> {
            match self.0 {
                Some(c) => Some(c(slice)),
                None => None,
            }
        }
    }
    impl From<extern "C" fn(FFISlice<u8>) -> u8> for CallbackFFISlice {
        fn from(x: extern "C" fn(FFISlice<u8>) -> u8) -> Self {
            Self(Some(x))
        }
    }
    unsafe impl interoptopus::lang::rust::CTypeInfo for CallbackFFISlice {
        fn type_info() -> interoptopus::lang::c::CType {
            use interoptopus::lang::rust::CTypeInfo;
            let rval = <u8 as CTypeInfo>::type_info();
            let params = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    interoptopus::lang::c::Parameter::new(
                        "slice".to_string(),
                        <FFISlice<u8> as CTypeInfo>::type_info(),
                    ),
                ]),
            );
            let sig = interoptopus::lang::c::FunctionSignature::new(params, rval);
            let fn_pointer = interoptopus::lang::c::FnPointerType::new(sig);
            let named_callback = interoptopus::patterns::callbacks::NamedCallback::new(
                "CallbackFFISlice".to_string(),
                fn_pointer,
            );
            interoptopus::lang::c::CType::Pattern(
                interoptopus::patterns::TypePattern::NamedCallback(named_callback),
            )
        }
    }
    pub mod ambiguous1 {
        use interoptopus::ffi_type;
        #[repr(C)]
        pub struct Vec {
            pub x: f32,
            pub y: f32,
        }
        unsafe impl ::interoptopus::lang::rust::CTypeInfo for Vec {
            fn type_info() -> ::interoptopus::lang::c::CType {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                    "".to_string(),
                    documentation,
                    None,
                );
                let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
                let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
                let name = "Vec1".to_string();
                {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    let the_type = <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                    let field = ::interoptopus::lang::c::Field::with_documentation(
                        "x".to_string(),
                        the_type,
                        interoptopus::lang::c::Visibility::Public,
                        documentation,
                    );
                    fields.push(field);
                }
                {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    let the_type = <f32 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                    let field = ::interoptopus::lang::c::Field::with_documentation(
                        "y".to_string(),
                        the_type,
                        interoptopus::lang::c::Visibility::Public,
                        documentation,
                    );
                    fields.push(field);
                }
                let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                    name,
                    fields,
                    meta,
                );
                ::interoptopus::lang::c::CType::Composite(rval)
            }
        }
        #[repr(C)]
        pub enum Status {
            X = 1,
            Y = 2,
        }
        unsafe impl ::interoptopus::lang::rust::VariantInfo for Status {
            fn variant_info(&self) -> ::interoptopus::lang::c::Variant {
                match self {
                    Self::X => {
                        let documentation = ::interoptopus::lang::c::Documentation::from_line(
                            "",
                        );
                        ::interoptopus::lang::c::Variant::new(
                            "X".to_string(),
                            1i32 as usize,
                            documentation,
                        )
                    }
                    Self::Y => {
                        let documentation = ::interoptopus::lang::c::Documentation::from_line(
                            "",
                        );
                        ::interoptopus::lang::c::Variant::new(
                            "Y".to_string(),
                            2i32 as usize,
                            documentation,
                        )
                    }
                }
            }
        }
        unsafe impl ::interoptopus::lang::rust::CTypeInfo for Status {
            fn type_info() -> ::interoptopus::lang::c::CType {
                use ::interoptopus::lang::rust::VariantInfo;
                let mut variants = ::std::vec::Vec::new();
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                    "".to_string(),
                    documentation,
                    None,
                );
                {
                    variants.push(Self::X.variant_info());
                }
                {
                    variants.push(Self::Y.variant_info());
                }
                let rval = ::interoptopus::lang::c::EnumType::new(
                    "Status1".to_string(),
                    variants,
                    meta,
                );
                ::interoptopus::lang::c::CType::Enum(rval)
            }
        }
    }
    pub mod ambiguous2 {
        use interoptopus::ffi_type;
        #[repr(C)]
        pub struct Vec {
            pub x: f64,
            pub z: f64,
        }
        unsafe impl ::interoptopus::lang::rust::CTypeInfo for Vec {
            fn type_info() -> ::interoptopus::lang::c::CType {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                    "".to_string(),
                    documentation,
                    None,
                );
                let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
                let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
                let name = "Vec2".to_string();
                {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    let the_type = <f64 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                    let field = ::interoptopus::lang::c::Field::with_documentation(
                        "x".to_string(),
                        the_type,
                        interoptopus::lang::c::Visibility::Public,
                        documentation,
                    );
                    fields.push(field);
                }
                {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    let the_type = <f64 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                    let field = ::interoptopus::lang::c::Field::with_documentation(
                        "z".to_string(),
                        the_type,
                        interoptopus::lang::c::Visibility::Public,
                        documentation,
                    );
                    fields.push(field);
                }
                let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                    name,
                    fields,
                    meta,
                );
                ::interoptopus::lang::c::CType::Composite(rval)
            }
        }
        #[repr(C)]
        pub enum Status {
            X = 100,
            Z = 200,
        }
        unsafe impl ::interoptopus::lang::rust::VariantInfo for Status {
            fn variant_info(&self) -> ::interoptopus::lang::c::Variant {
                match self {
                    Self::X => {
                        let documentation = ::interoptopus::lang::c::Documentation::from_line(
                            "",
                        );
                        ::interoptopus::lang::c::Variant::new(
                            "X".to_string(),
                            100i32 as usize,
                            documentation,
                        )
                    }
                    Self::Z => {
                        let documentation = ::interoptopus::lang::c::Documentation::from_line(
                            "",
                        );
                        ::interoptopus::lang::c::Variant::new(
                            "Z".to_string(),
                            200i32 as usize,
                            documentation,
                        )
                    }
                }
            }
        }
        unsafe impl ::interoptopus::lang::rust::CTypeInfo for Status {
            fn type_info() -> ::interoptopus::lang::c::CType {
                use ::interoptopus::lang::rust::VariantInfo;
                let mut variants = ::std::vec::Vec::new();
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                    "".to_string(),
                    documentation,
                    None,
                );
                {
                    variants.push(Self::X.variant_info());
                }
                {
                    variants.push(Self::Z.variant_info());
                }
                let rval = ::interoptopus::lang::c::EnumType::new(
                    "Status2".to_string(),
                    variants,
                    meta,
                );
                ::interoptopus::lang::c::CType::Enum(rval)
            }
        }
    }
    pub mod common {
        use interoptopus::ffi_type;
        #[repr(C)]
        pub struct Vec {
            pub x: f64,
            pub z: f64,
        }
        unsafe impl ::interoptopus::lang::rust::CTypeInfo for Vec {
            fn type_info() -> ::interoptopus::lang::c::CType {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                    "common".to_string(),
                    documentation,
                    None,
                );
                let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
                let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
                let name = {
                    let res = ::alloc::fmt::format(
                        format_args!("{0}{1}", "Vec".to_string(), generics.join("")),
                    );
                    res
                };
                {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    let the_type = <f64 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                    let field = ::interoptopus::lang::c::Field::with_documentation(
                        "x".to_string(),
                        the_type,
                        interoptopus::lang::c::Visibility::Public,
                        documentation,
                    );
                    fields.push(field);
                }
                {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    let the_type = <f64 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                    let field = ::interoptopus::lang::c::Field::with_documentation(
                        "z".to_string(),
                        the_type,
                        interoptopus::lang::c::Visibility::Public,
                        documentation,
                    );
                    fields.push(field);
                }
                let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                    name,
                    fields,
                    meta,
                );
                ::interoptopus::lang::c::CType::Composite(rval)
            }
        }
    }
    pub mod associated_types {
        use interoptopus::ffi_type;
        pub trait Helper {
            type X;
        }
        #[repr(C)]
        pub struct Chicken(u8);
        unsafe impl ::interoptopus::lang::rust::CTypeInfo for Chicken {
            fn type_info() -> ::interoptopus::lang::c::CType {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                    "".to_string(),
                    documentation,
                    None,
                );
                let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
                let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
                let name = {
                    let res = ::alloc::fmt::format(
                        format_args!("{0}{1}", "Chicken".to_string(), generics.join("")),
                    );
                    res
                };
                {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    let the_type = <u8 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                    let field = ::interoptopus::lang::c::Field::with_documentation(
                        "x0".to_string(),
                        the_type,
                        interoptopus::lang::c::Visibility::Private,
                        documentation,
                    );
                    fields.push(field);
                }
                let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                    name,
                    fields,
                    meta,
                );
                ::interoptopus::lang::c::CType::Composite(rval)
            }
        }
        #[repr(C)]
        pub struct Cow(u16);
        unsafe impl ::interoptopus::lang::rust::CTypeInfo for Cow {
            fn type_info() -> ::interoptopus::lang::c::CType {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                    "".to_string(),
                    documentation,
                    None,
                );
                let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
                let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
                let name = {
                    let res = ::alloc::fmt::format(
                        format_args!("{0}{1}", "Cow".to_string(), generics.join("")),
                    );
                    res
                };
                {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    let the_type = <u16 as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                    let field = ::interoptopus::lang::c::Field::with_documentation(
                        "x0".to_string(),
                        the_type,
                        interoptopus::lang::c::Visibility::Private,
                        documentation,
                    );
                    fields.push(field);
                }
                let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                    name,
                    fields,
                    meta,
                );
                ::interoptopus::lang::c::CType::Composite(rval)
            }
        }
        impl Helper for Chicken {
            type X = Cow;
        }
        #[repr(C)]
        pub struct FieldsViaAssociatedType {
            pub x: <Chicken as Helper>::X,
        }
        unsafe impl ::interoptopus::lang::rust::CTypeInfo for FieldsViaAssociatedType {
            fn type_info() -> ::interoptopus::lang::c::CType {
                let documentation = ::interoptopus::lang::c::Documentation::from_line(
                    "",
                );
                let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(
                    "".to_string(),
                    documentation,
                    None,
                );
                let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
                let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();
                let name = {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "{0}{1}",
                            "FieldsViaAssociatedType".to_string(),
                            generics.join(""),
                        ),
                    );
                    res
                };
                {
                    let documentation = ::interoptopus::lang::c::Documentation::from_line(
                        "",
                    );
                    let the_type = <<Chicken as Helper>::X as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                    let field = ::interoptopus::lang::c::Field::with_documentation(
                        "x".to_string(),
                        the_type,
                        interoptopus::lang::c::Visibility::Public,
                        documentation,
                    );
                    fields.push(field);
                }
                let rval = ::interoptopus::lang::c::CompositeType::with_meta(
                    name,
                    fields,
                    meta,
                );
                ::interoptopus::lang::c::CType::Composite(rval)
            }
        }
    }
}
pub fn ffi_inventory() -> Inventory {
    {
        InventoryBuilder::new()
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::primitive_void>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::primitive_void2>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::primitive_bool>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::primitive_u8>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::primitive_u16>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::primitive_u32>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::primitive_u64>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::primitive_i8>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::primitive_i16>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::primitive_i32>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::primitive_i64>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::many_args_5>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::many_args_10>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::ptr>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::ptr_mut>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::ptr_ptr>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::ref_simple>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::ref_mut_simple>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::ref_option>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::ref_mut_option>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::tupled>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::complex_args_1>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::complex_args_2>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::callback>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::generic_1a>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::generic_1b>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::generic_1c>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::generic_2>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::generic_3>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::generic_4>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::array_1>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::documented>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::ambiguous_1>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::ambiguous_2>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::ambiguous_3>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::namespaced_type>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::namespaced_inner_option>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::namespaced_inner_slice>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::namespaced_inner_slice_mut>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::panics>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::renamed>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::sleep>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::weird_1>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::visibility>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <functions::repr_transparent>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::ascii_pointer::pattern_ascii_pointer_1>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::ascii_pointer::pattern_ascii_pointer_2>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::ascii_pointer::pattern_ascii_pointer_len>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::ascii_pointer::pattern_ascii_pointer_return_slice>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::slice::pattern_ffi_slice_1>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::slice::pattern_ffi_slice_1b>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::slice::pattern_ffi_slice_2>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::slice::pattern_ffi_slice_3>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::slice::pattern_ffi_slice_4>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::slice::pattern_ffi_slice_5>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::slice::pattern_ffi_slice_6>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::slice::pattern_ffi_slice_delegate>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::slice::pattern_ffi_slice_delegate_huge>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::option::pattern_ffi_option_1>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::option::pattern_ffi_option_2>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::primitives::pattern_ffi_bool>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::primitives::pattern_ffi_cchar>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::primitives::pattern_ffi_cchar_const_pointer>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::primitives::pattern_ffi_cchar_mut_pointer>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::api_guard::pattern_api_guard>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::callbacks::pattern_callback_1>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::callbacks::pattern_callback_2>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::FunctionInfo;
                let info = <patterns::callbacks::pattern_callback_3>::function_info();
                ::interoptopus::Symbol::Function(info)
            })
            .register({
                use ::interoptopus::lang::rust::ConstantInfo;
                use constants::U8 as constant;
                let info = constant::constant_info();
                ::interoptopus::Symbol::Constant(info)
            })
            .register({
                use ::interoptopus::lang::rust::ConstantInfo;
                use constants::F32_MIN_POSITIVE as constant;
                let info = constant::constant_info();
                ::interoptopus::Symbol::Constant(info)
            })
            .register({
                use ::interoptopus::lang::rust::ConstantInfo;
                use constants::COMPUTED_I32 as constant;
                let info = constant::constant_info();
                ::interoptopus::Symbol::Constant(info)
            })
            .register({
                let info = <types::ExtraType<
                    f32,
                > as ::interoptopus::lang::rust::CTypeInfo>::type_info();
                ::interoptopus::Symbol::Type(info)
            })
            .register({
                let info: ::interoptopus::patterns::LibraryPattern = <patterns::service::SimpleService as ::interoptopus::patterns::LibraryPatternInfo>::pattern_info();
                ::interoptopus::Symbol::Pattern(info)
            })
            .register({
                let info: ::interoptopus::patterns::LibraryPattern = <patterns::service::SimpleServiceLifetime as ::interoptopus::patterns::LibraryPatternInfo>::pattern_info();
                ::interoptopus::Symbol::Pattern(info)
            })
            .inventory()
    }
}
