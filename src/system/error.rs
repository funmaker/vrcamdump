use std::{error, fmt};
use openvr_sys as sys;

pub struct TrackedPropertyError {
	pub code: sys::EVRTrackedCameraError,
	pub name: String,
}

impl error::Error for TrackedPropertyError {}

impl fmt::Debug for TrackedPropertyError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.pad(&format!("{}({})", self.name, self.code))
	}
}

impl fmt::Display for TrackedPropertyError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.pad(&self.name)
	}
}

pub fn check_err(code: sys::EVRTrackedCameraError) -> Result<(), TrackedPropertyError> {
	if code == sys::ETrackedPropertyError_TrackedProp_Success {
		Ok(())
	} else {
		let name = match code {
			sys::ETrackedPropertyError_TrackedProp_Success => "TrackedProp_Success",
			sys::ETrackedPropertyError_TrackedProp_WrongDataType => "TrackedProp_WrongDataType",
			sys::ETrackedPropertyError_TrackedProp_WrongDeviceClass => "TrackedProp_WrongDeviceClass",
			sys::ETrackedPropertyError_TrackedProp_BufferTooSmall => "TrackedProp_BufferTooSmall",
			sys::ETrackedPropertyError_TrackedProp_UnknownProperty => "TrackedProp_UnknownProperty",
			sys::ETrackedPropertyError_TrackedProp_InvalidDevice => "TrackedProp_InvalidDevice",
			sys::ETrackedPropertyError_TrackedProp_CouldNotContactServer => "TrackedProp_CouldNotContactServer",
			sys::ETrackedPropertyError_TrackedProp_ValueNotProvidedByDevice => "TrackedProp_ValueNotProvidedByDevice",
			sys::ETrackedPropertyError_TrackedProp_StringExceedsMaximumLength => "TrackedProp_StringExceedsMaximumLength",
			sys::ETrackedPropertyError_TrackedProp_NotYetAvailable => "TrackedProp_NotYetAvailable",
			sys::ETrackedPropertyError_TrackedProp_PermissionDenied => "TrackedProp_PermissionDenied",
			_ => "TrackedProp_UnknownError",
		}.into();
		
		Err(TrackedPropertyError { code, name })
	}
}
