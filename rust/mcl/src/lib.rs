use std::mem::MaybeUninit;
use std::os::raw::c_int;

#[link(name = "mcl", kind = "static")]
#[link(name = "mclbn384_256", kind = "static")]
#[link(name = "gmp")]
#[link(name = "stdc++")]
#[link(name = "crypto")]
#[allow(non_snake_case)]
extern "C" {
    fn mclBn_getVersion() -> u32;
    fn mclBn_getFrByteSize() -> u32;
    fn mclBn_getFpByteSize() -> u32;
    fn mclBn_init(curve: c_int, compiledTimeVar: c_int) -> c_int;
    fn mclBnFr_setInt32(x: *mut Fr, v: i32);
    fn mclBnFr_setStr(x: *mut Fr, buf: *const u8, bufSize: usize, ioMode: i32) -> c_int;
    fn mclBnFr_getStr(buf: *mut u8, maxBufSize: usize, x: *const Fr, ioMode: i32) -> usize;
    fn mclBnFr_serialize(buf: *mut u8, maxBufSize: usize, x: *const Fr) -> usize;
    fn mclBnFr_deserialize(x: *mut Fr, buf: *const u8, bufSize: usize) -> usize;
    fn mclBnFr_setLittleEndian(x: *mut Fr, buf: *const u8, bufSize: usize) -> i32;
    fn mclBnFr_isEqual(x: *const Fr, y: *const Fr) -> i32;
    fn mclBnFr_isValid(x: *const Fr) -> i32;
    fn mclBnFr_isZero(x: *const Fr) -> i32;
    fn mclBnFr_isOne(x: *const Fr) -> i32;
    fn mclBnFr_isOdd(x: *const Fr) -> i32;

	fn mclBnFr_add(z:*mut Fr, x:*const Fr, y:*const Fr);
	fn mclBnFr_sub(z:*mut Fr, x:*const Fr, y:*const Fr);
	fn mclBnFr_mul(z:*mut Fr, x:*const Fr, y:*const Fr);
	fn mclBnFr_div(z:*mut Fr, x:*const Fr, y:*const Fr);
}

pub enum CurveType {
    BN254 = 0,
    BN381 = 1,
    SNARK = 4,
    BLS12_381 = 5,
}

const MCLBN_FP_UNIT_SIZE: usize = 6;
const MCLBN_FR_UNIT_SIZE: usize = 4;
const MCLBN_COMPILED_TIME_VAR: c_int =
    (MCLBN_FR_UNIT_SIZE as c_int * 10 + MCLBN_FP_UNIT_SIZE as c_int);

macro_rules! serialize_impl {
    ($t:ty, $size:expr, $serialize_fn:ident, $deserialize_fn:ident) => {
        impl $t {
            pub fn deserialize(&mut self, buf: &[u8]) -> bool {
                unsafe { $deserialize_fn(self, buf.as_ptr(), buf.len()) > 0 }
            }
            pub fn serialize(&self) -> Vec<u8> {
                let size = unsafe { $size } as usize;
                let mut buf: Vec<u8> = Vec::with_capacity(size);
                let n: usize;
                unsafe {
                    n = $serialize_fn(buf.as_mut_ptr(), size, self);
                }
                if n == 0 {
                    panic!("serialize");
                }
                unsafe {
                    buf.set_len(n);
                }
                buf
            }
        }
    };
}

macro_rules! str_impl {
    ($t:ty, $maxBufSize:expr, $get_str_fn:ident, $set_str_fn:ident) => {
        impl $t {
            pub fn set_str(&mut self, s: &str, base: i32) -> bool {
                unsafe { $set_str_fn(self, s.as_ptr(), s.len(), base) == 0 }
            }
            pub fn get_str(&self, io_mode: i32) -> String {
                let mut buf: [u8; $maxBufSize] = unsafe { MaybeUninit::uninit().assume_init() };
                let n: usize;
                unsafe {
                    n = $get_str_fn(buf.as_mut_ptr(), buf.len(), self, io_mode);
                }
                if n == 0 {
                    panic!("mclBnFr_getStr");
                }
                buf[0..n].iter().map(|&s| s as char).collect::<String>()
            }
            /*
                        pub fn get_str2(&self, io_mode: i32) -> String {
                            let mut buf: Vec<u8> = Vec::with_capacity($maxBufSize);
                            let n: usize;
                            unsafe {
                                n = $get_str_fn(buf.as_mut_ptr(), buf.capacity(), self, io_mode as c_int);
                            }
                            if n == 0 {
                                panic!("mclBnFr_getStr");
                            }
                            unsafe {
                                buf.set_len(n);
                                String::from_utf8_unchecked(buf)
                            }
                        }
            */
        }
    };
}

macro_rules! is_compare_base_impl {
    ($t:ty, $is_equal_fn:ident, $is_valid_fn:ident, $is_zero_fn:ident) => {
        impl PartialEq for $t {
            fn eq(&self, rhs: &Self) -> bool {
                unsafe { $is_equal_fn(self, rhs) == 1 }
            }
        }
        impl $t {
            pub fn is_valid(&self) -> bool {
                unsafe { $is_valid_fn(self) == 1 }
            }
            pub fn is_zero(&self) -> bool {
                unsafe { $is_zero_fn(self) == 1 }
            }
        }
    };
}

macro_rules! is_one_impl {
    ($t:ty, $is_one_fn:ident) => {
        impl $t {
            pub fn is_one(&self) -> bool {
                unsafe { $is_one_fn(self) == 1 }
            }
        }
    };
}

macro_rules! is_odd_impl {
    ($t:ty, $is_odd_fn:ident) => {
        impl $t {
            pub fn is_odd(&self) -> bool {
                unsafe { $is_odd_fn(self) == 1 }
            }
        }
    };
}

macro_rules! field_op_impl {
	($t:ty, $add_fn:ident, $sub_fn:ident, $mul_fn:ident, $div_fn:ident) => {
		impl $t {
			pub fn add(z:&mut $t, x: &$t, y: &$t) {
				unsafe { $add_fn(z, x, y) }
			}
			pub fn sub(z:&mut $t, x: &$t, y: &$t) {
				unsafe { $sub_fn(z, x, y) }
			}
			pub fn mul(z:&mut $t, x: &$t, y: &$t) {
				unsafe { $mul_fn(z, x, y) }
			}
			pub fn div(z:&mut $t, x: &$t, y: &$t) {
				unsafe { $div_fn(z, x, y) }
			}
		}
	};
}

#[allow(dead_code)]
#[repr(C)]
pub struct Fp {
    d: [u64; MCLBN_FP_UNIT_SIZE],
}

#[allow(dead_code)]
#[repr(C)]
pub struct Fp2 {
    d: [Fr; 2],
}

#[repr(C)]
#[derive(Default)]
pub struct Fr {
    d: [u64; MCLBN_FR_UNIT_SIZE],
}
serialize_impl![
    Fr,
    mclBn_getFrByteSize(),
    mclBnFr_serialize,
    mclBnFr_deserialize
];
str_impl![Fr, 1024, mclBnFr_getStr, mclBnFr_setStr];
is_compare_base_impl![Fr, mclBnFr_isEqual, mclBnFr_isValid, mclBnFr_isZero];
is_one_impl![Fr, mclBnFr_isOne];
is_odd_impl![Fr, mclBnFr_isOdd];
field_op_impl![Fr, mclBnFr_add, mclBnFr_sub, mclBnFr_mul, mclBnFr_div];

#[allow(dead_code)]
#[repr(C)]
pub struct G1 {
    x: Fp,
    y: Fp,
    z: Fp,
}

#[allow(dead_code)]
#[repr(C)]
pub struct G2 {
    x: Fp2,
    y: Fp2,
    z: Fp2,
}

#[allow(dead_code)]
#[repr(C)]
pub struct GT {
    d: [Fp; 12],
}

pub fn get_version() -> u32 {
    unsafe { mclBn_getVersion() }
}

pub fn init(curve: CurveType) -> bool {
    unsafe { mclBn_init(curve as c_int, MCLBN_COMPILED_TIME_VAR) == 0 }
}

impl Fr {
    pub fn zero() -> Fr {
        Default::default()
    }
    pub fn clear(&mut self) {
        *self = Fr::zero()
    }
    pub fn set_int(&mut self, x: i32) {
        unsafe {
            mclBnFr_setInt32(self, x);
        }
    }
}
