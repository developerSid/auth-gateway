#[cfg(test)]
mod pattern_compiler_tests {
   use auth_gateway_vectorscan::{CompileError, compile_patterns};
   #[test]
   fn test_compile_valid_pattern() {
      let result = compile_patterns(["^/api/company/(1|2)$"]);

      assert!(result.is_ok());
   }

   #[test]
   fn test_compile_invalid_pattern() {
      let result = compile_patterns([r"^/api/company/(\d+)/\1$"]);

      assert!(result.is_err());

      match result.unwrap_err() {
         CompileError::CompileFailed { expression, message } => {
            assert_eq!(expression, 0);
            assert!(!message.is_empty());
            assert_eq!(message, String::from("Back-references are unsupported."));
         }
         other => panic!("unexpected error: {other:?}"),
      }
   }

   #[test]
   fn test_compile_invalid_unbalanced_pattern() {
      let result = compile_patterns([r"^/api/company/([0-9]+$"]);

      assert!(result.is_err());

      match result.unwrap_err() {
         CompileError::CompileFailed { expression, message } => {
            assert_eq!(expression, 0);
            assert!(!message.is_empty());
            assert_eq!(
               message,
               String::from("Missing close parenthesis for group started at index 14.")
            );
         }
         other => panic!("unexpected error: {other:?}"),
      }
   }
}
