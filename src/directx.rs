use std::ptr;
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::cell::RefCell;
use std::ops::DerefMut;
use winapi::um::d3d11::*;
use winapi::um::d3dcommon::D3D_DRIVER_TYPE_HARDWARE;
use winapi::um::winnt::HRESULT;

pub struct D3DContext {
	device: RefCell<&'static mut ID3D11Device>,
	context: RefCell<&'static mut ID3D11DeviceContext>,
}

impl D3DContext {
	pub fn new() -> Result<D3DContext, D3DError> {
		unsafe {
			let mut device = ptr::null_mut();
			let mut context = ptr::null_mut();
			let mut feature_level = 0;
			
			check_err(D3D11CreateDevice(ptr::null_mut(),
			                            D3D_DRIVER_TYPE_HARDWARE,
			                            ptr::null_mut(),
			                            0,
			                            ptr::null_mut(),
			                            0,
			                            D3D11_SDK_VERSION,
			                            &mut device,
			                            &mut feature_level,
			                            &mut context))?;
			
			Ok(D3DContext{
				device: RefCell::new(device.as_mut().unwrap()),
				context: RefCell::new(context.as_mut().unwrap()),
			})
		}
	}
	
	pub fn device(&self) -> impl DerefMut<Target = &'static mut ID3D11Device> + '_ {
		self.device.borrow_mut()
	}
	
	pub fn context(&self) -> impl DerefMut<Target = &'static mut ID3D11DeviceContext> + '_ {
		self.context.borrow_mut()
	}
}

impl Drop for D3DContext {
	fn drop(&mut self) {
		unsafe {
			self.context().Release();
			self.device().Release();
		}
	}
}

pub fn check_err(hr: HRESULT) -> Result<(), D3DError> {
	if hr < 0 {
		Err(D3DError(hr))
	} else {
		Ok(())
	}
}

#[derive(Debug)]
pub struct D3DError(HRESULT);

impl Display for D3DError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		writeln!(f, "DirectX error: {}", self.0)
	}
}

impl Error for D3DError {}
