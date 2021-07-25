use std::ptr;
use openvr_sys::{EVREye_Eye_Left, EVREye_Eye_Right, EVREye};
use winapi::um::d3d11::*;
use winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM;
use winapi::shared::dxgitype::DXGI_SAMPLE_DESC;
use image::{ImageBuffer, RgbaImage};

use crate::directx::{self, D3DContext};
use crate::compositor::Compositor;

pub enum VREye {
	Left,
	Right,
}

impl Into<EVREye> for VREye {
	fn into(self) -> u32 {
		match self {
			VREye::Left => EVREye_Eye_Left,
			VREye::Right => EVREye_Eye_Right,
		}
	}
}

pub struct MirrorTexture<'c, 'd> {
	compositor: Compositor<'c>,
	d3d: &'d D3DContext,
	resource_view: &'static mut ID3D11ShaderResourceView,
	resource: &'static mut ID3D11Resource,
	texture: &'static mut ID3D11Texture2D,
}

impl<'c, 'd> MirrorTexture<'c, 'd> {
	pub unsafe fn new(compositor: Compositor<'c>, d3d: &'d D3DContext, resource_view: &'static mut ID3D11ShaderResourceView) -> MirrorTexture<'c, 'd> {
		let mut resource = ptr::null_mut();
		resource_view.GetResource(&mut resource);
		let resource = resource.as_mut().expect("GetResource failed");
		
		let mut texture = ptr::null_mut::<ID3D11Texture2D>();
		resource.QueryInterface(&IID_ID3D11Texture2D, &mut texture as *mut _ as *mut _);
		let texture = texture.as_mut().expect("QueryInterface failed");
		
		let mut desc = D3D11_TEXTURE2D_NULL;
		texture.GetDesc(&mut desc);
		desc.BindFlags = 0;
		desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ | D3D11_CPU_ACCESS_WRITE;
		desc.Usage = D3D11_USAGE_STAGING;
		desc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
		texture.Release();
		
		let mut texture = ptr::null_mut();
		directx::check_err(
			d3d.device().CreateTexture2D(&desc, ptr::null(), &mut texture)
		).expect("CreateTexture2D Fail");
		let texture = texture.as_mut().expect("CreateTexture2D Fail");
		
		MirrorTexture {
			compositor,
			d3d,
			resource_view,
			resource,
			texture,
		}
	}
	
	pub unsafe fn capture(&mut self) -> RgbaImage {
		let mut texture = ptr::null_mut::<ID3D11Texture2D>();
		self.resource.QueryInterface(&IID_ID3D11Texture2D, &mut texture as *mut _ as *mut _);
		let texture = texture.as_mut().expect("QueryInterface failed");
		
		let mut desc = D3D11_TEXTURE2D_NULL;
		texture.GetDesc(&mut desc);
		
		self.d3d.context().CopyResource(self.texture as *mut _ as *mut _, texture as *mut _ as *mut _);
		
		let mut mapped_resource = D3D11_MAPPED_SUBRESOURCE {
			pData: ptr::null_mut(),
			RowPitch: 0,
			DepthPitch: 0,
		};
		let subresource = D3D11CalcSubresource(0, 0, 0);
		directx::check_err(
			self.d3d.context().Map(
				self.texture as *mut _ as *mut _,
				subresource,
				D3D11_MAP_READ_WRITE,
				0,
				&mut mapped_resource
			)
		).expect("Map Fail");
		
		let width = desc.Width as usize;
		let height = desc.Height as usize;
		let row_pitch = mapped_resource.RowPitch as usize;
		
		let data = std::slice::from_raw_parts(mapped_resource.pData.cast(), row_pitch * height);
		let mut result = Vec::with_capacity(width * height * 4);
		
		for row in data.chunks(row_pitch) {
			result.extend_from_slice(&row[0..width * 4]);
		}
		
		ImageBuffer::from_vec(desc.Width, desc.Height, result).expect("Failed to create mirror image")
	}
}

impl<'c, 'd> Drop for MirrorTexture<'c, 'd> {
	fn drop(&mut self) {
		unsafe {
			self.texture.Release();
			self.resource.Release();
			self.compositor.release_mirror_texture_d3d11(self.resource_view);
		}
	}
}

const D3D11_TEXTURE2D_NULL: D3D11_TEXTURE2D_DESC = D3D11_TEXTURE2D_DESC {
	Width: 0,
	Height: 0,
	MipLevels: 0,
	ArraySize: 0,
	Format: 0,
	SampleDesc: DXGI_SAMPLE_DESC {
		Count: 0,
		Quality: 0,
	},
	Usage: 0,
	BindFlags: 0,
	CPUAccessFlags: 0,
	MiscFlags: 0,
};
