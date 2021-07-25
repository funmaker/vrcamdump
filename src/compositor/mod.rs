use std::marker::PhantomData;
use std::ptr;
use std::ops::DerefMut;
use openvr_sys as sys;
use winapi::um::d3d11::{ID3D11ShaderResourceView, ID3D11Device};

mod error;
mod utils;

pub use error::*;
pub use utils::*;
use crate::openvr_load::{load, InitError, Context};
use crate::directx::D3DContext;

pub type FnTable = &'static sys::VR_IVRCompositor_FnTable;

#[derive(Copy, Clone)]
pub struct Compositor<'a>(FnTable, PhantomData<&'a Context>);

impl<'a> Compositor<'a> {
	pub fn new(_context: &Context) -> Result<Compositor, InitError> {
		let fn_tab: FnTable = unsafe { &*load(sys::IVRCompositor_Version)? };
		
		Ok(Compositor(fn_tab, PhantomData))
	}
	
	pub fn get_mirror_texture_d3d11<'c, 'd>(&'c self, eye: VREye, d3d: &'d D3DContext) -> Result<MirrorTexture<'c, 'd>, CompositorError> {
		let mut resource_view = ptr::null_mut();
		
		check_err(unsafe {
			self.0.GetMirrorTextureD3D11.unwrap()(eye.into(),
			                                      d3d.device().deref_mut().deref_mut() as *mut ID3D11Device as *mut _,
			                                      &mut resource_view)
		})?;
		
		unsafe {
			Ok(MirrorTexture::new(self.clone(),
			                      d3d,
			                      resource_view.cast::<ID3D11ShaderResourceView>().as_mut().unwrap()))
		}
	}
	
	unsafe fn release_mirror_texture_d3d11(&self, resource_view: &mut ID3D11ShaderResourceView) {
		self.0.ReleaseMirrorTextureD3D11.unwrap()(resource_view as *mut _ as *mut _)
	}
}
