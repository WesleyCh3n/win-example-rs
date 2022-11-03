use std::os::raw::c_void;
use windows::core::Result;
use windows::w;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Storage::FileSystem::{
    GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW,
    VS_FIXEDFILEINFO,
};

fn main() {
    unsafe {
        if let Err(e) = get_file_version() {
            println!("Got error: {}", e);
        }
    }
}

unsafe fn get_file_version() -> Result<()> {
    let filepath = w!("C:/Windows/System32/kernel32.dll");
    let dw_size = GetFileVersionInfoSizeW(filepath, None);
    if dw_size == 0 {
        println!("Error in GetFileVersionInfoSizeW: {:?}", GetLastError());
    }
    let mut data: Vec<u8> = vec![0; dw_size as usize];

    if !GetFileVersionInfoW(filepath, 0, dw_size, data.as_mut_ptr() as _)
        .as_bool()
    {
        println!("Error in GetFileVersionInfo: {:?}", GetLastError());
        return Ok(());
    };

    let mut lp_file_info: *mut c_void = std::ptr::null_mut();
    let mut cb_file_info = 0;
    if !VerQueryValueW(
        data.as_ptr() as _,
        w!("\\"),
        &mut lp_file_info as _,
        &mut cb_file_info,
    )
    .as_bool()
    {
        println!("Error in VerQueryValue: {:?}", GetLastError());
        return Ok(());
    };
    let file_info = &mut *(lp_file_info as *mut VS_FIXEDFILEINFO);

    println!(
        "Version: {}.{}.{}.{}",
        ((file_info.dwFileVersionMS >> 16) & 0xffff),
        file_info.dwFileVersionMS & 0xffff,
        ((file_info.dwFileVersionLS >> 16) & 0xffff),
        file_info.dwFileVersionLS & 0xffff,
    );
    println!("Signature: {:x}", file_info.dwSignature);
    println!("FileOS: {:?}", file_info.dwFileOS);
    println!("FileType: {:?}", file_info.dwFileType);
    println!("FileSubtype: {:?}", file_info.dwFileSubtype);
    println!("FileFlags: {:?}", file_info.dwFileFlags);

    Ok(())
}
