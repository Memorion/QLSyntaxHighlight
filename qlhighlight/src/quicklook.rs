use core_foundation::base::Boolean;
use core_foundation::data::CFDataRef;
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::{CFString, CFStringRef};

#[allow(dead_code)]
extern "C" {
    #[link_name = "kUTTypeHTML"]
    pub static kUTTypeHTML: CFStringRef;
    #[link_name = "kUTTypePlainText"]
    pub static kUTTypePlainText: CFStringRef;
    pub fn QLPreviewRequestSetDataRepresentation(
        preview: QLPreviewRequestRef,
        data: CFDataRef,
        contentTypeUTI: CFStringRef,
        properties: CFDictionaryRef,
    );
    pub fn QLPreviewRequestIsCancelled(preview: QLPreviewRequestRef) -> Boolean;
    #[allow(improper_ctypes)]
    pub fn CFLog(level: usize, log: CFString);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __QLPreviewRequest {
    _unused: [u8; 0],
}
pub type QLPreviewRequestRef = *mut __QLPreviewRequest;
