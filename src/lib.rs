extern crate encoding_rs as encode;
extern crate libc;

use self::encode::SHIFT_JIS;
use std::ffi::CStr;
use std::ffi::CString;
use widestring::{U16String,U32String,U16CString,U32CString};

pub trait ToEncode {
    fn to_shiftjis(&self) -> CString;
    fn to_cstring(&self) -> CString;
    fn to_u16string(&self) -> U16CString;
    fn to_u32string(&self) -> U32CString;
}
impl ToEncode for &str {
    fn to_shiftjis(&self) -> CString {
        let (res, _enc, errors) = SHIFT_JIS.encode(self);
        CString::new(res).unwrap_or_else(|_| CString::new("").unwrap())
    }

    fn to_cstring(&self) -> CString {
        CString::new(*self).expect("Failed to convert &str to CString")
    }
    fn to_u16string(&self) -> U16CString{
        U16CString::from_str(*self).unwrap()
    }
    fn to_u32string(&self) -> U32CString{
        U32CString::from_str(*self).unwrap()
    }

}

impl ToEncode for String {
    fn to_shiftjis(&self) -> CString {
        let (res, _enc, errors) = SHIFT_JIS.encode(self);
        CString::new(res).unwrap_or_else(|_| CString::new("").unwrap())
    }
    
    fn to_cstring(&self) -> CString {
        CString::new(self.as_str()).expect("Failed to convert String to CString")
    }
   fn to_u16string(&self) -> U16CString{
        U16CString::from_str(&*self.clone()).unwrap()
    }
    fn to_u32string(&self) -> U32CString{
        U32CString::from_str(&*self.clone()).unwrap()
    }


}
impl ToEncode for *mut i8 {
    fn to_cstring(&self) -> CString {
        unsafe {
            let ptr = CStr::from_ptr(*self);
            CString::new(ptr.to_str().expect("Failed to convert *mut i8 to str"))
                .expect("Failed to convert str to CString")
        }
    }
    fn to_shiftjis(&self) -> CString {
        CString::new("").unwrap()
    }
   fn to_u16string(&self) -> U16CString{
        let c_str = unsafe{CStr::from_ptr(*self as *const i8)};
        let rs_str = c_str.to_str().unwrap();
        U16CString::from_str(rs_str).unwrap()
    }
    fn to_u32string(&self) -> U32CString{
        let c_str = unsafe{CStr::from_ptr(*self as *const i8)};
        let rs_str = c_str.to_str().unwrap();
        U32CString::from_str(rs_str).unwrap()
    }


}
impl ToEncode for *const i8 {
    fn to_cstring(&self) -> CString {
        unsafe {
            let ptr = CStr::from_ptr(*self);
            CString::new(ptr.to_str().expect("Failed to convert *mut i8 to str"))
                .expect("Failed to convert str to CString")
        }
    }
    fn to_shiftjis(&self) -> CString {
        CString::new("").unwrap()
    }
 fn to_u16string(&self) -> U16CString{
        let c_str = unsafe{CStr::from_ptr(*self)};
        let rs_str = c_str.to_str().unwrap();
        U16CString::from_str(rs_str).unwrap()
    }
    fn to_u32string(&self) -> U32CString{
         let c_str = unsafe{CStr::from_ptr(*self)};
        let rs_str = c_str.to_str().unwrap();
        U32CString::from_str(rs_str).unwrap()
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::c_char;
    use libc::c_int;
    use libc::fclose;
    use libc::fgetc;
    use libc::fopen;
    use libc::EOF;
    use libc::FILE;
    use winapi::um::winuser::*;
    use std::ptr::null_mut;
    use widestring::*;
    #[test]
    fn test() {
        unsafe {
            /*
            let mut s = String::from("");
        
            let mut fp: *mut FILE =
                fopen("test.txt".to_cstring().as_ptr(), "r".to_cstring().as_ptr());
            if fp.is_null() {
                panic!("Failed to open file");
            }
            loop {
                let ch: c_int = fgetc(fp);
                print!("{}", ch as u8 as char);
                if ch == EOF {
                    break;
                }
            }
            fclose(fp);
        */
        let title = "タイトル".to_u16string();
        let content = "こんにちは世界".to_u16string();
        MessageBoxW(null_mut(),title.as_ptr(),content.as_ptr(),0);      
      }

    }
}
