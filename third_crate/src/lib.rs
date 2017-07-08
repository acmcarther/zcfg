#![feature(used)]
#[macro_use]
extern crate gflags;

#[macro_use]
extern crate lazy_static;

pub use im_very_configurable::get_FLAG as FLAG_im_very_configurable;
define_flag! { im_very_configurable }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
