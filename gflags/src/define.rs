
enum FlagInitializer {
  Uninitialized,
  Initializing,
  Complete
}

static flag_initializer: FlagInitializer = FlagInitializer::Uninitialized;

/*
macro_rules! define_flag {
  (name:$ident : t: $ty, v: $expr) {

    mod flag_decl {
      mod $name {
      #[used]
      #[cfg_attr(target_os = "linux", link_section = ".ctors")]
      #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
      #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
      fn __define_flag_internal() {
      }

      static __define_flag: extern "C" fn() = __define_flag_internal()
    }
  }
}
*/
