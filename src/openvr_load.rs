use std::{error, fmt};
use std::ffi::CStr;
use std::sync::atomic::{AtomicBool, Ordering};
use openvr_sys as sys;

pub type TrackedDeviceIndex = sys::TrackedDeviceIndex_t;
pub type TrackedCameraHandle = sys::TrackedCameraHandle_t;
pub type TrackedDeviceProperty = sys::TrackedDeviceProperty;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ApplicationType {
	/// Some other kind of application that isn't covered by the other entries
	Other = sys::EVRApplicationType_VRApplication_Other as isize,
	/// Application will submit 3D frames
	Scene = sys::EVRApplicationType_VRApplication_Scene as isize,
	/// Application only interacts with overlays
	Overlay = sys::EVRApplicationType_VRApplication_Overlay as isize,
	/// Application should not start SteamVR if it's not already running, and should not keep it running if everything
	/// else quits.
	Background = sys::EVRApplicationType_VRApplication_Background as isize,
	/// Init should not try to load any drivers. The application needs access to utility interfaces (like IVRSettings
	/// and IVRApplications) but not hardware.
	Utility = sys::EVRApplicationType_VRApplication_Utility as isize,
	/// Reserved for vrmonitor
	VRMonitor = sys::EVRApplicationType_VRApplication_VRMonitor as isize,
	/// Reserved for Steam
	SteamWatchdog = sys::EVRApplicationType_VRApplication_SteamWatchdog as isize,
	/// Start up SteamVR
	Bootstrapper = sys::EVRApplicationType_VRApplication_Bootstrapper as isize,
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Context;

impl Context {
	pub fn new(ty: ApplicationType) -> Result<Self, InitError> {
		if INITIALIZED.swap(true, Ordering::Acquire) {
			panic!("OpenVR has already been initialized!");
		}
		
		unsafe {
			let mut error = sys::EVRInitError_VRInitError_None;
			sys::VR_InitInternal(&mut error, ty as sys::EVRApplicationType);
			
			if error != sys::EVRInitError_VRInitError_None {
				return Err(InitError(error));
			}
			
			if !sys::VR_IsInterfaceVersionValid(sys::IVRSystem_Version.as_ptr() as *const i8) {
				sys::VR_ShutdownInternal();
				return Err(InitError(
					sys::EVRInitError_VRInitError_Init_InterfaceNotFound,
				));
			}
		}
		
		Ok(Context)
	}
}

impl Drop for Context {
	fn drop(&mut self) {
		unsafe {
			sys::VR_ShutdownInternal();
			INITIALIZED.store(false, Ordering::Release);
		}
	}
}

pub fn load<T>(suffix: &[u8]) -> Result<*const T, InitError> {
	let mut magic = Vec::from(b"FnTable:".as_ref());
	magic.extend(suffix);
	let mut error = sys::EVRInitError_VRInitError_None;
	let result = unsafe { sys::VR_GetGenericInterface(magic.as_ptr() as *const i8, &mut error) };
	if error != sys::EVRInitError_VRInitError_None {
		return Err(InitError(
			sys::EVRInitError_VRInitError_Init_InterfaceNotFound,
		));
	}
	Ok(result as *const T)
}

pub struct InitError(pub sys::EVRInitError);

impl error::Error for InitError {}

impl fmt::Debug for InitError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let msg = unsafe { CStr::from_ptr(sys::VR_GetVRInitErrorAsSymbol(self.0)) };
		f.pad(
			msg.to_str()
			   .expect("OpenVR init error symbol was not valid UTF-8"),
		)
	}
}

impl fmt::Display for InitError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let msg = unsafe { CStr::from_ptr(sys::VR_GetVRInitErrorAsEnglishDescription(self.0)) };
		f.pad(
			msg.to_str()
			   .expect("OpenVR init error description was not valid UTF-8")
		)
	}
}
