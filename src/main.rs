use std::{env, fmt, fs, thread};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::{SystemTime, Duration, Instant};
use image::{RgbaImage, DynamicImage, GenericImage};
use openvr_sys::k_unTrackedDeviceIndex_Hmd as HMD;

pub mod openvr_load;
mod compositor;
mod tracked_camera;
mod settings;
mod directx;
mod system;

use system::System;
use compositor::{Compositor, VREye};
use tracked_camera::{TrackedCamera, FrameType};
use settings::Settings;
use openvr_load::{ApplicationType, Context};
use directx::D3DContext;
use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let curtime = SystemTime::now()
                             .duration_since(SystemTime::UNIX_EPOCH)?
                             .as_secs();
    let destination = env::current_dir()?
                          .join("dumps")
                          .join(format!("{}", curtime));
    
    println!("Initializing DirectX...");
    
    let d3d = D3DContext::new()?;
    
    println!("Initializing OpenVR...");
    
    let context = Context::new(ApplicationType::Other)?;
    let system = System::new(&context)?;
    let compositor = Compositor::new(&context)?;
    let tracked_camera = TrackedCamera::new(&context)?;
    let settings = Settings::new(&context)?;
    
    if !tracked_camera.has_camera(HMD) {
        return Err(StrError::new("No camera in HMD"));
    }
    
    if !settings.get_bool("camera", "enableCamera")? {
        println!("Camera is not enabled, enabling...");
        settings.set_bool("camera", "enableCamera", true)?;
    }
    
    if settings.get_i32("camera", "roomView")? != 1 {
        println!("Room view is not set to 2D, setting...");
        settings.set_i32("camera", "roomView", 1)?;
    }
    
    if settings.get_i32("camera", "roomViewStyle")? != 4 {
        println!("Room view style is not set to opaque, setting...");
        settings.set_i32("camera", "roomViewStyle", 4)?;
    }
    
    println!("Searching for calibration data...");
    
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let steam_location = hklm.open_subkey("SOFTWARE\\Valve\\Steam")
                             .or(hklm.open_subkey("SOFTWARE\\Wow6432Node\\Valve\\Steam"))
                             .and_then(|key| key.get_value("InstallPath"))
                             .unwrap_or_else(|err| {
                                 let fallback = "C:\\Program Files (x86)\\Steam";
                                 eprintln!("Unable to find steam path! {}\nFallback to: {}", err, fallback);
                                 fallback.to_string()
                             });
    println!("Using steam location: {}", steam_location);
    
    let serial_number = system.string_tracked_device_property(HMD, openvr_sys::ETrackedDeviceProperty_Prop_SerialNumber_String)?;
    let mut config_path = PathBuf::new();
    config_path.push(steam_location);
    config_path.push("config");
    config_path.push("lighthouse");
    config_path.push(serial_number.to_lowercase());
    config_path.push("config.json");
    println!("Using config location: {}", config_path.to_string_lossy());
    
    let config = fs::read_to_string(config_path).unwrap_or_else(|err| {
        eprintln!("Unable to read config! {}", err);
        "N/A".into()
    });
    
    println!("Fetching intrinsics...");
    
    let intrinsics = (
        tracked_camera.get_camera_intrinsics(HMD, 0, FrameType::Distorted),
        tracked_camera.get_camera_intrinsics(HMD, 1, FrameType::Distorted),
        tracked_camera.get_camera_projection(HMD, 0, FrameType::Distorted, 0.01, 100.01),
        tracked_camera.get_camera_projection(HMD, 1, FrameType::Distorted, 0.01, 100.01),
        tracked_camera.get_camera_intrinsics(HMD, 0, FrameType::Undistorted),
        tracked_camera.get_camera_intrinsics(HMD, 1, FrameType::Undistorted),
        tracked_camera.get_camera_projection(HMD, 0, FrameType::Undistorted, 0.01, 100.01),
        tracked_camera.get_camera_projection(HMD, 1, FrameType::Undistorted, 0.01, 100.01),
        tracked_camera.get_camera_intrinsics(HMD, 0, FrameType::MaximumUndistorted),
        tracked_camera.get_camera_intrinsics(HMD, 1, FrameType::MaximumUndistorted),
        tracked_camera.get_camera_projection(HMD, 0, FrameType::MaximumUndistorted, 0.01, 100.01),
        tracked_camera.get_camera_projection(HMD, 1, FrameType::MaximumUndistorted, 0.01, 100.01),
    );
    
    println!("Initializing Mirror Textures...");
    
    let mut left_eye = compositor.get_mirror_texture_d3d11(VREye::Left, &d3d)?;
    let mut right_eye = compositor.get_mirror_texture_d3d11(VREye::Right, &d3d)?;
    
    println!("Spin up sleep...");
    
    thread::sleep(Duration::from_secs(1));
    
    print!("Fetching camera frame...");
    
    let frame_size;
    let mut buffer;
    let header;
    unsafe {
        let start = Instant::now();
        let service = tracked_camera.acquire_video_streaming_service(HMD)?;
        frame_size = tracked_camera.get_camera_frame_size(HMD, FrameType::Distorted)?;
        buffer = vec![0u8; frame_size.frame_buffer_size as usize];
        
        loop {
            match tracked_camera.get_video_stream_frame_buffer(service, FrameType::Distorted, &mut buffer) {
                Ok(result) => {
                    header = result;
                    break;
                },
                Err(err) => {
                    print!(".");
    
                    if start.elapsed().as_secs() > 5 {
                        println!();
                        return Err(err.into());
                    }
                }
            };
            
            thread::sleep(Duration::from_secs(1));
        }
        
        println!();
        
        // tracked_camera.release_video_streaming_service(service)?; // Just doesn't work ¯\_(ツ)_/¯
    };
    
    for i in (3..buffer.len()).step_by(4) {
        buffer[i] = 255;
    }
    
    let camera_image = RgbaImage::from_raw(frame_size.width, frame_size.height, buffer)
        .ok_or(StrError::new("Failed to parse camera frame"))?;
    
    println!("Fetching mirror image...");
    
    let mirror_image= unsafe {
        let left_image = left_eye.capture();
        let right_image = right_eye.capture();
        let mut mirror_image = DynamicImage::new_rgb8(
            left_image.width() + right_image.width(),
            left_image.height().max(right_image.height())
        );
    
        mirror_image.copy_from(&left_image, 0, 0)?;
        mirror_image.copy_from(&right_image, left_image.width(), 0)?;
    
        mirror_image
    };
    
    println!("Saving results to {}...", destination.as_os_str().to_string_lossy());
    
    fs::create_dir_all(&destination)?;
    
    fs::write(destination.join("config.json"), config)?;
    camera_image.save(destination.join("camera.png"))?;
    mirror_image.save(destination.join("mirror.png"))?;
    fs::write(destination.join("frame.txt"), format!("{:#?}", header))?;
    fs::write(destination.join("intrinsics.txt"), format!("{:#?}", intrinsics))?;
    
    println!("\nDone!");
    
    Ok(())
}



#[derive(Debug)]
struct StrError(String);

impl StrError {
    fn new(text: impl Into<String>) -> Box<StrError> { Box::new(StrError(text.into())) }
}

impl Display for StrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> { self.0.fmt(f) }
}

impl Error for StrError {}
