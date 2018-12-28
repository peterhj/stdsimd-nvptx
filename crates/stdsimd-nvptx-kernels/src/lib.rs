#![feature(abi_ptx)]
//#![feature(core_intrinsics)]
#![feature(no_core)]
#![feature(stdsimd)]
#![no_core]
//#![no_std]

extern crate coresimd;

use coresimd::arch::nvptx::*;

#[no_mangle]
pub unsafe extern "ptx-kernel" fn _stdsimd_nvptx_test_syncthreads_kernel() {
  _syncthreads();
}
