#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unaligned_references)]
#![allow(clippy::all)]
include!(concat!(env!("OUT_DIR"), "/aeron_cpp_wrapper_client.rs"));

#[cfg(test)]
mod tests {

    #[test]
    fn version_check() {
        let major = unsafe { crate::aeron_version_major() };
        let minor = unsafe { crate::aeron_version_minor() };
        let patch = unsafe { crate::aeron_version_patch() };
        assert_eq!(major, 1);
        assert_eq!(minor, 41);
        assert_eq!(patch, 4);
    }
}
