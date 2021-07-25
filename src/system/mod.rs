use std::marker::PhantomData;
use std::ffi::CString;
use openvr_sys as sys;

mod error;

pub use error::*;
use crate::openvr_load::{load, InitError, Context, TrackedDeviceIndex, TrackedDeviceProperty};

pub type FnTable = &'static sys::VR_IVRSystem_FnTable;

#[derive(Copy, Clone)]
pub struct System<'a>(FnTable, PhantomData<&'a Context>);

impl<'a> System<'a> {
	pub fn new(_context: &Context) -> Result<System, InitError> {
		let fn_tab: FnTable = unsafe { &*load(sys::IVRSystem_Version)? };
		
		Ok(System(fn_tab, PhantomData))
	}
	
	pub fn string_tracked_device_property(
		&self,
		device: TrackedDeviceIndex,
		property: TrackedDeviceProperty,
	) -> Result<String, TrackedPropertyError> {
		unsafe {
			let mut error = sys::ETrackedPropertyError_TrackedProp_Success;
			let size = self.0.GetStringTrackedDeviceProperty.unwrap()(device, property, std::ptr::null_mut(), 0, &mut error);
			if error != sys::ETrackedPropertyError_TrackedProp_BufferTooSmall {
				check_err(error)?;
			}
			
			let mut output = vec![0u8; size as usize];
			self.0.GetStringTrackedDeviceProperty.unwrap()(device, property, output.as_mut_ptr() as *mut _, output.len() as u32, &mut error);
			check_err(error)?;
			
			Ok(CString::new(&output[0..output.len() - 1])
			           .expect("Failed to read property")
			           .into_string()
			           .expect("Failed to parse property"))
		}
	}
}
