use std::{error, fmt};
use std::ffi::CStr;
use openvr_sys as sys;

use super::FnTable;

pub struct SettingsError {
	pub code: sys::EVRSettingsError,
	pub name: String,
}

impl error::Error for SettingsError {}

impl fmt::Debug for SettingsError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.pad(&format!("{}({})", self.name, self.code))
	}
}

impl fmt::Display for SettingsError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.pad(&self.name)
	}
}

pub fn check_err(fn_tab: FnTable, code: sys::EVRSettingsError) -> Result<(), SettingsError> {
	if code == sys::EVRSettingsError_VRSettingsError_None {
		Ok(())
	} else {
		let name = fn_tab.GetSettingsErrorNameFromEnum
		                 .map(|f| unsafe { f(code) })
		                 .map(|msg| unsafe { CStr::from_ptr(msg) })
		                 .map(CStr::to_str)
		                 .map(Result::ok)
		                 .flatten()
		                 .unwrap_or("VRSettingsError_UnknownError")
		                 .into();
		
		Err(SettingsError{ code, name })
	}
}
