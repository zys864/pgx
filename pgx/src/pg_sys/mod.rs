//
// we allow improper_ctypes just to eliminate these warnings:
//      = note: `#[warn(improper_ctypes)]` on by default
//      = note: 128-bit integers don't currently have a known stable ABI
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(clippy::unneeded_field_pattern)]

//
// our actual bindings modules -- these are generated by build.rs
//

mod common;

#[cfg(feature = "pg10")]
pub mod pg10_specific;

#[cfg(feature = "pg11")]
pub mod pg11_specific;

#[cfg(feature = "pg12")]
pub mod pg12_specific;

//
// publicly expose the contents of our version modules
// these are hidden behind feature gates because we can
// only expose one of these behind "pg_sys" at a time
//

pub use all_versions::*;
pub use internal::*;

// version modules
// These exist to allow us to add additional items to the various version namespaces
// that couldn't be auto-generated by bindgen

/// item declarations we want to add to all versions
mod all_versions {
    use memoffset::*;
    use std::str::FromStr;

    /// this comes from `postgres_ext.h`
    pub const InvalidOid: super::Oid = 0;
    pub const InvalidOffsetNumber: super::OffsetNumber = 0;
    pub const FirstOffsetNumber: super::OffsetNumber = 1;
    pub const MaxOffsetNumber: super::OffsetNumber =
        (super::BLCKSZ as usize / std::mem::size_of::<super::ItemIdData>()) as super::OffsetNumber;
    pub const VARHDRSZ: usize = std::mem::size_of::<super::int32>();

    #[inline]
    pub fn VARHDRSZ_EXTERNAL() -> usize {
        offset_of!(super::varattrib_1b_e, va_data)
    }

    #[inline]
    pub fn VARHDRSZ_SHORT() -> usize {
        offset_of!(super::varattrib_1b, va_data)
    }

    #[inline]
    pub fn get_pg_major_version_string() -> &'static str {
        let mver = std::ffi::CStr::from_bytes_with_nul(super::PG_MAJORVERSION).unwrap();
        mver.to_str().unwrap()
    }

    #[inline]
    pub fn get_pg_major_version_num() -> u16 {
        u16::from_str(super::get_pg_major_version_string()).unwrap()
    }

    #[inline]
    pub fn get_pg_version_string() -> &'static str {
        let ver = std::ffi::CStr::from_bytes_with_nul(super::PG_VERSION_STR).unwrap();
        ver.to_str().unwrap()
    }
}

mod internal {
    #[cfg(feature = "pg10")]
    pub use pg10::*;

    #[cfg(feature = "pg11")]
    pub use pg11::*;

    #[cfg(feature = "pg12")]
    pub use pg12::*;

    //
    // for specific versions
    //

    #[cfg(feature = "pg10")]
    mod pg10 {
        pub use crate::pg_sys::common::*;

        pub type FunctionCallInfo = *mut crate::pg_sys::pg10_specific::FunctionCallInfoData;
        pub type TupleDesc = *mut crate::pg_sys::pg10_specific::tupleDesc;

        pub use crate::pg_sys::pg10_specific::AllocSetContextCreate as AllocSetContextCreateExtended;
        pub use crate::pg_sys::pg10_specific::IndexAmRoutine;
        pub use crate::pg_sys::pg10_specific::IndexInfo;
        pub use crate::pg_sys::pg10_specific::IndexPath;
        pub use crate::pg_sys::pg10_specific::IndexScanDescData;
        pub use crate::pg_sys::pg10_specific::IndexVacuumInfo;
        pub use crate::pg_sys::pg10_specific::InterruptPending;
        pub use crate::pg_sys::pg10_specific::NodeTag_T_IndexAmRoutine;
        pub use crate::pg_sys::pg10_specific::PlannerInfo;
        pub use crate::pg_sys::pg10_specific::PG_MAJORVERSION;
        pub use crate::pg_sys::pg10_specific::PG_VERSION;
        pub use crate::pg_sys::pg10_specific::PG_VERSION_NUM;
        pub use crate::pg_sys::pg10_specific::PG_VERSION_STR;
    }

    #[cfg(feature = "pg11")]
    mod pg11 {
        pub use crate::pg_sys::common::*;

        pub type FunctionCallInfo = *mut crate::pg_sys::pg11_specific::FunctionCallInfoData;
        pub type TupleDesc = *mut crate::pg_sys::pg11_specific::tupleDesc;

        pub use crate::pg_sys::pg11_specific::AllocSetContextCreateExtended;
        pub use crate::pg_sys::pg11_specific::IndexAmRoutine;
        pub use crate::pg_sys::pg11_specific::IndexInfo;
        pub use crate::pg_sys::pg11_specific::IndexPath;
        pub use crate::pg_sys::pg11_specific::IndexScanDescData;
        pub use crate::pg_sys::pg11_specific::IndexVacuumInfo;
        pub use crate::pg_sys::pg11_specific::InterruptPending;
        pub use crate::pg_sys::pg11_specific::NodeTag_T_IndexAmRoutine;
        pub use crate::pg_sys::pg11_specific::PlannerInfo;
        pub use crate::pg_sys::pg11_specific::PG_MAJORVERSION;
        pub use crate::pg_sys::pg11_specific::PG_VERSION;
        pub use crate::pg_sys::pg11_specific::PG_VERSION_NUM;
        pub use crate::pg_sys::pg11_specific::PG_VERSION_STR;
    }

    #[cfg(feature = "pg12")]
    mod pg12 {
        pub use crate::pg_sys::common::*;

        pub type FunctionCallInfo = *mut crate::pg_sys::pg12_specific::FunctionCallInfoBaseData;
        pub type TupleDesc = *mut crate::pg_sys::pg12_specific::TupleDescData;

        pub use crate::pg_sys::pg12_specific::AllocSetContextCreateInternal as AllocSetContextCreateExtended;
        pub use crate::pg_sys::pg12_specific::IndexAmRoutine;
        pub use crate::pg_sys::pg12_specific::IndexInfo;
        pub use crate::pg_sys::pg12_specific::IndexPath;
        pub use crate::pg_sys::pg12_specific::IndexScanDescData;
        pub use crate::pg_sys::pg12_specific::IndexVacuumInfo;
        pub use crate::pg_sys::pg12_specific::InterruptPending;
        pub use crate::pg_sys::pg12_specific::NodeTag_T_IndexAmRoutine;
        pub use crate::pg_sys::pg12_specific::PlannerInfo;
        pub use crate::pg_sys::pg12_specific::PG_MAJORVERSION;
        pub use crate::pg_sys::pg12_specific::PG_VERSION;
        pub use crate::pg_sys::pg12_specific::PG_VERSION_NUM;
        pub use crate::pg_sys::pg12_specific::PG_VERSION_STR;
    }
}
