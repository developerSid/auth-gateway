mod ffi;

use crate::ffi::hs_alloc_scratch;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uint};
use std::ptr;

#[derive(Debug)]
pub struct Database(*mut ffi::VsDatabase);
impl Drop for Database {
   fn drop(&mut self) {
      unsafe { ffi::hs_free_database(self.0) };
   }
}
#[derive(Debug)]
pub enum CompileError {
   EmptyPatterns,
   TooManyPatterns, // Why the name TooManyPatterns for this error? Because that failure would only happen if the number of patterns is so large that their indexes no longer fit into c_uint.
   PatternContainsNul { index: usize },
   UnsupportedArchitecture,
   CompileFailedOnlyExpression { expression: usize },
   CompileFailedOnlyMessage { message: String },
   CompileFailed { expression: usize, message: String },
   CompileFailedNoInformation,
}
pub fn compile_patterns<I, S>(patterns: I) -> Result<Database, CompileError>
where
   // IntoIterator<Item = S>, S: AsRef<str> lets SDK call it with Vec<String>, Vec<&str>, arrays, or slices.
   I: IntoIterator<Item = S>,
   S: AsRef<str>,
{
   let patterns: Vec<S> = patterns.into_iter().collect();
   if patterns.is_empty() {
      return Err(CompileError::EmptyPatterns);
   }
   let expressions: Vec<CString> = patterns // owns Vec of CStrings
      .iter()
      .enumerate()
      .map(|(index, pattern)| CString::new(pattern.as_ref()).map_err(|_| CompileError::PatternContainsNul { index }))
      .collect::<Result<_, _>>()?; /* When `?` is used it is an implicit return of a CompileError in this a PatternContainsNul
   - unwrap the Ok(...) value and continue, or
   - return the Err(...) immediately from compile_patterns */

   let expr_ptrs: Vec<*const c_char> = expressions.iter().map(|s| s.as_ptr()).collect(); // borrow pointers to the owned strings from `expressions` which will be passed to VectorScan
   let flags: Vec<c_uint> = vec![0; expr_ptrs.len()];
   let ids: Vec<c_uint> = (0..expr_ptrs.len())
      // TODO: the bellow with TooManyPatterns being something where the c_uint isn't large enough to hold all the patterns passed in really could be handled with a better patterns domain model that limits during creating and is checking there the platform limits
      .map(|i| c_uint::try_from(i).map_err(|_| CompileError::TooManyPatterns)) /* try_from
      expr_ptrs.len() uses usize, which can be larger than c_uint
      on some platforms. So this checks that the value fits instead of silently truncating. */
      .collect::<Result<_, _>>()?; /* When `?` is used it is an implicit return of a CompileError in this a TooManyPatterns
   - unwrap the Ok(...) value and continue, or
   - return the Err(...) immediately from compile_patterns */
   let elements = c_uint::try_from(expr_ptrs.len()).map_err(|_| CompileError::TooManyPatterns)?; // continue here with questions AI
   let mut db = ptr::null_mut();
   let mut error = ptr::null_mut();
   let rc = unsafe {
      ffi::hs_compile_multi(
         expr_ptrs.as_ptr(),
         flags.as_ptr(),
         ids.as_ptr(),
         elements,
         1, // HS_MODE_BLOCK, since we will only be dealing with URL paths
         ptr::null(),
         &mut db,
         &mut error,
      )
   };

   match rc {
      ffi::HS_SUCCESS => Ok(Database(db)),
      ffi::HS_ARCH_ERROR => Err(CompileError::UnsupportedArchitecture),
      ffi::HS_COMPILER_ERROR => {
         if error.is_null() {
            Err(CompileError::CompileFailedNoInformation)
         } else {
            unsafe {
               let message = if !(*error).message.is_null() {
                  Some(CStr::from_ptr((*error).message).to_string_lossy().into_owned())
               } else {
                  None
               };
               let expression = if (*error).expression > -1 {
                  Some((*error).expression as usize)
               } else {
                  None
               };

               let result = match (message, expression) {
                  (Some(message), Some(expression)) => Err(CompileError::CompileFailed { message, expression }),
                  (Some(message), None) => Err(CompileError::CompileFailedOnlyMessage { message }),
                  (None, Some(expression)) => Err(CompileError::CompileFailedOnlyExpression { expression }),
                  (None, None) => Err(CompileError::CompileFailedNoInformation),
               };

               ffi::hs_free_compile_error(error);
               result
            }
         }
      }
      _ => Err(CompileError::CompileFailedNoInformation),
   }
}

pub fn matches<S>(database: &Database /*, pattern: S*/)
where
   // S: AsRef<str> lets SDK call it with Vec<String>, Vec<&str>, arrays, or slices.
   S: AsRef<str>,
{
   let mut scratch = ptr::null_mut();
   let db = database.0;
   let rc = unsafe { hs_alloc_scratch(db, &mut scratch) };

   match rc {
      ffi::HS_SUCCESS => {}
      ffi::HS_NOMEM => (),
      _ => {
         unsafe {
            if !scratch.is_null() {
               ffi::hs_free_scratch(scratch)
            }
         };
      }
   }
   //println!("{:?}", rc)
}
