fn main() {
    // Tell cargo to tell rustc to link the Gdiplus shared library.
    println!("cargo:rustc-link-lib=gdiplus");

    if !std::env::var("GDIP_BINDGEN").unwrap_or_default().is_empty() {
        bindgen();
    }
}

fn bindgen() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        .blocklist_type("LPMONITORINFOEXA?W?")
        .blocklist_type("LPTOP_LEVEL_EXCEPTION_FILTER")
        .blocklist_type("MONITORINFOEXA?W?")
        .blocklist_type("PEXCEPTION_FILTER")
        .blocklist_type("PEXCEPTION_ROUTINE")
        .blocklist_type("PSLIST_HEADER")
        .blocklist_type("PTOP_LEVEL_EXCEPTION_FILTER")
        .blocklist_type("PVECTORED_EXCEPTION_HANDLER")
        .blocklist_type("_?L?P?CONTEXT")
        .blocklist_type("_?L?P?EXCEPTION_POINTERS")
        .blocklist_type("_?P?DISPATCHER_CONTEXT")
        .blocklist_type("_?P?EXCEPTION_REGISTRATION_RECORD")
        .blocklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        .blocklist_type("_?P?NT_TIB")
        .blocklist_type("tagMONITORINFOEXA")
        .blocklist_type("tagMONITORINFOEXW")
        .blocklist_function("AddVectoredContinueHandler")
        .blocklist_function("AddVectoredExceptionHandler")
        .blocklist_function("CopyContext")
        .blocklist_function("GetThreadContext")
        .blocklist_function("GetXStateFeaturesMask")
        .blocklist_function("InitializeContext")
        .blocklist_function("InitializeContext2")
        .blocklist_function("InitializeSListHead")
        .blocklist_function("InterlockedFlushSList")
        .blocklist_function("InterlockedPopEntrySList")
        .blocklist_function("InterlockedPushEntrySList")
        .blocklist_function("InterlockedPushListSListEx")
        .blocklist_function("LocateXStateFeature")
        .blocklist_function("QueryDepthSList")
        .blocklist_function("RaiseFailFastException")
        .blocklist_function("RtlCaptureContext")
        .blocklist_function("RtlCaptureContext2")
        .blocklist_function("RtlFirstEntrySList")
        .blocklist_function("RtlInitializeSListHead")
        .blocklist_function("RtlInterlockedFlushSList")
        .blocklist_function("RtlInterlockedPopEntrySList")
        .blocklist_function("RtlInterlockedPushEntrySList")
        .blocklist_function("RtlInterlockedPushListSListEx")
        .blocklist_function("RtlQueryDepthSList")
        .blocklist_function("RtlRestoreContext")
        .blocklist_function("RtlUnwindEx")
        .blocklist_function("RtlVirtualUnwind")
        .blocklist_function("SetThreadContext")
        .blocklist_function("SetUnhandledExceptionFilter")
        .blocklist_function("SetXStateFeaturesMask")
        .blocklist_function("UnhandledExceptionFilter")
        .blocklist_function("__C_specific_handler")
        .allowlist_function(".*Gdiplus.*")
        .blocklist_type("_*CLSID_*")
        .blocklist_type("_*GUID_*")
        .blocklist_type("_*HBITMAP_*")
        .blocklist_type("_*HDC_*")
        .blocklist_type("_*HENHMETAFILE_*")
        .blocklist_type("_*HFONT_*")
        .blocklist_type("_*HICON_*")
        .blocklist_type("_*HPALETTE_*")
        .blocklist_type("_*HMETAFILE_*")
        .blocklist_type("_*HRGN_*")
        .blocklist_type("_*HWND_*")
        .blocklist_type("_*HINSTANCE_*")
        .blocklist_type("_*RECTL_*")
        .blocklist_type("_*IStream_*")
        .blocklist_type("_*BITMAPINFO_*")
        .blocklist_type("_*LOGFONTA_*")
        .blocklist_type("_*LOGFONTW_*")
        .blocklist_type("_*METAHEADER_*")
        .blocklist_type("_*FILETIME_*")
        .blocklist_type("HANDLE")
        .blocklist_type("LPWSTR")
        .blocklist_type("BITMAPINFO")
        .blocklist_type("BITMAPINFOHEADER")
        .blocklist_type("LOGFONTA")
        .blocklist_type("LOGFONTW")
        .blocklist_type("METAHEADER")
        .blocklist_type("RGBQUAD")
        .blocklist_type("SIZE")
        .blocklist_type("STATSTG")
        .blocklist_type("tag.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .ctypes_prefix("::core::ffi")
        .disable_name_namespacing()
        .generate_comments(true)
        .generate_block(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let mut binding_source = bindings
        .to_string()
        .replace(
            "pub type UINT_PTR = ::core::ffi::c_ulonglong;",
            "pub type UINT_PTR = *mut ::core::ffi::c_ulonglong;",
        )
        .replace(
            "pub type ULONG_PTR = ::core::ffi::c_ulonglong;",
            "pub type ULONG_PTR = *mut ::core::ffi::c_ulonglong;",
        )
        .replace(
            "pub type UINT_PTR = ::core::ffi::c_uint;",
            "pub type UINT_PTR = *mut ::core::ffi::c_uint;",
        )
        .replace(
            "pub type ULONG_PTR = ::core::ffi::c_ulong;",
            "pub type ULONG_PTR = *mut ::core::ffi::c_ulong;",
        );

    binding_source.insert_str(
        0,
        r##"
use windows_sys::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::Com::IStream,
        UI::WindowsAndMessaging::*,
    },
};
type CLSID = GUID;
type LPWSTR = PWSTR;

"##,
    );

    // pub type HDC = *mut HDC__;
    let target = std::env::var("TARGET").expect("Can't get build target");
    let arch = if let Some(p) = target.find('-') {
        &target[..p]
    } else {
        &target
    };

    std::fs::write(format!("src/bindings_{}.rs", arch), binding_source).unwrap();
}
