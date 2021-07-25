use std::{error, fmt};
use openvr_sys as sys;

pub struct CompositorError {
	pub code: sys::EVRScreenshotError,
	pub name: String,
}

impl error::Error for CompositorError {}

impl fmt::Debug for CompositorError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.pad(&format!("{}({})", self.name, self.code))
	}
}

impl fmt::Display for CompositorError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.pad(&self.name)
	}
}

pub fn check_err(code: sys::EVRCompositorError) -> Result<(), CompositorError> {
	if code == sys::EVRCompositorError_VRCompositorError_None {
		Ok(())
	} else {
		let name = match code {
			sys::EVRCompositorError_VRCompositorError_RequestFailed => "VRCompositorError_RequestFailed",
			sys::EVRCompositorError_VRCompositorError_IncompatibleVersion => "VRCompositorError_IncompatibleVersion",
			sys::EVRCompositorError_VRCompositorError_DoNotHaveFocus => "VRCompositorError_DoNotHaveFocus",
			sys::EVRCompositorError_VRCompositorError_InvalidTexture => "VRCompositorError_InvalidTexture",
			sys::EVRCompositorError_VRCompositorError_IsNotSceneApplication => "VRCompositorError_IsNotSceneApplication",
			sys::EVRCompositorError_VRCompositorError_TextureIsOnWrongDevice => "VRCompositorError_TextureIsOnWrongDevice",
			sys::EVRCompositorError_VRCompositorError_TextureUsesUnsupportedFormat => "VRCompositorError_TextureUsesUnsupportedFormat",
			sys::EVRCompositorError_VRCompositorError_SharedTexturesNotSupported => "VRCompositorError_SharedTexturesNotSupported",
			sys::EVRCompositorError_VRCompositorError_IndexOutOfRange => "VRCompositorError_IndexOutOfRange",
			sys::EVRCompositorError_VRCompositorError_AlreadySubmitted => "VRCompositorError_AlreadySubmitted",
			sys::EVRCompositorError_VRCompositorError_InvalidBounds => "VRCompositorError_InvalidBounds",
			_ => "VRCompositorError_UnknownError",
		}.into();
		
		Err(CompositorError{ code, name })
	}
}
