//! auth-gateway-vectorscan - FFI bindings for VectorScan
use std::ffi::c_uint;
use std::os::raw::{c_char, c_int};

/// Opaque compiled pattern database. Created by the compile API, used for scanning.
#[repr(C)]
pub struct VsDatabase {
   _opaque: [u8; 0],
}

#[repr(C)]
pub struct VsScratch {
   _opaque: [u8; 0],
}

/// Opaque compile error. Returned when compilation fails; use the API to read the message.
/// Compile error returned when compilation fails. Free with `hs_free_compile_error`.
#[repr(C)]
pub struct VsCompileError {
   /// NUL-terminated error message; null if not set.
   pub message: *const c_char,
   /// Zero-based pattern index that caused the error, or -1.
   pub expression: c_int,
}

#[repr(C)]
pub struct VsPlatformInfoT {
   _opaque: [u8; 0],
}

pub const HS_SUCCESS: c_int = 0;
pub const HS_NOMEM: c_int = -2;
pub const HS_COMPILER_ERROR: c_int = -4;
pub const HS_ARCH_ERROR: c_int = -11;

unsafe extern "C" {
   /// Compiles multiple patterns into a single database.
   ///
   /// `expressions` points to an array of NUL-terminated pattern strings.
   /// `flags` points to an array of per-pattern flags.
   /// `ids` points to an array of per-pattern IDs.
   /// `elements` is the number of patterns.
   /// `mode` selects the database mode, such as `HS_MODE_BLOCK`.
   /// `platform` may be null to use the default platform.
   /// `db` is an output parameter that receives the compiled database.
   /// `error` is an output parameter that receives compile error details on failure.
   ///
   /// `returns` HS_SUCCESS if successfully compiled, HS_COMPILER_ERROR if the pattern(s) are invalid and HS_ARCH_ERROR if the CPU architecture isn't supported
   pub fn hs_compile_multi(
      expressions: *const *const c_char,
      flags: *const c_uint,
      ids: *const c_uint,
      elements: c_uint,
      mode: c_uint,
      platform: *const VsPlatformInfoT,
      db: *mut *mut VsDatabase,
      error: *mut *mut VsCompileError,
   ) -> c_int;

   /// Allocates a VectorScan scratch space that will be used by calls to the hs_scan function
   ///
   /// `db` points to a compiled VectorScan database
   /// `scratch` points to the to be allocated by ths function scratch space
   ///
   /// `returns` HS_SUCCESS if successfully allocates scratch space, HS_NOMEM if there is not enough memory to allocate and possibly other errors if invalid pointer types are used
   pub fn hs_alloc_scratch(db: *const VsDatabase, scratch: *mut *mut VsScratch) -> c_int; // TODO: call

   //pub fn hs_scan(); // TODO: complete implementation
   pub fn hs_free_scratch(scratch: *mut VsScratch); // TODO: call

   /// Frees a VectorScan compile error returned by the library.
   ///
   /// `error` points to the VectorScan compiler error
   pub fn hs_free_compile_error(error: *mut VsCompileError);

   /// Frees a compiled database. Safe to call with a null pointer.
   ///
   /// `db` VectorScan database to free
   pub fn hs_free_database(db: *mut VsDatabase);
}
