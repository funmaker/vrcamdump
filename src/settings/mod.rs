use std::marker::PhantomData;
use std::ffi::CString;
use openvr_sys as sys;

mod error;

pub use error::*;
use crate::openvr_load::{load, InitError, Context};

pub type FnTable = &'static sys::VR_IVRSettings_FnTable;

#[derive(Copy, Clone)]
pub struct Settings<'a>(FnTable, PhantomData<&'a Context>);

impl<'a> Settings<'a> {
	pub fn new(_context: &Context) -> Result<Settings, InitError> {
		let fn_tab: FnTable = unsafe { &*load(sys::IVRSettings_Version)? };
		
		Ok(Settings(fn_tab, PhantomData))
	}
	
	pub fn get_i32(&self, section: &str, key: &str) -> Result<i32, SettingsError> {
		let mut err = sys::EVRSettingsError_VRSettingsError_None;
		
		let out = unsafe {
			self.0.GetInt32.unwrap()(CString::new(section).unwrap().into_raw(),
			                         CString::new(key).unwrap().into_raw(),
			                         &mut err)
		};
		
		check_err(self.0, err)?;
		
		Ok(out)
	}
	
	pub fn get_bool(&self, section: &str, key: &str) -> Result<bool, SettingsError> {
		let mut err = sys::EVRSettingsError_VRSettingsError_None;
		
		let out = unsafe {
			self.0.GetBool.unwrap()(CString::new(section).unwrap().into_raw(),
			                        CString::new(key).unwrap().into_raw(),
			                        &mut err)
		};
		
		check_err(self.0, err)?;
		
		Ok(out)
	}
	
	pub fn set_i32(&self, section: &str, key: &str, value: i32) -> Result<(), SettingsError> {
		let mut err = sys::EVRSettingsError_VRSettingsError_None;
		
		unsafe {
			self.0.SetInt32.unwrap()(CString::new(section).unwrap().into_raw(),
			                         CString::new(key).unwrap().into_raw(),
			                         value,
			                         &mut err)
		};
		
		check_err(self.0, err)?;
		
		Ok(())
	}
	
	pub fn set_bool(&self, section: &str, key: &str, value: bool) -> Result<(), SettingsError> {
		let mut err = sys::EVRSettingsError_VRSettingsError_None;
		
		unsafe {
			self.0.SetBool.unwrap()(CString::new(section).unwrap().into_raw(),
			                        CString::new(key).unwrap().into_raw(),
			                        value,
			                        &mut err)
		};
		
		check_err(self.0, err)?;
		
		Ok(())
	}
}
