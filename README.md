# vrcamdump
Really bad Rust program to dump Valve Index calib data.

It's full of unsafe, it's bad, don't bother.

---

## How to use:
- Start SteamVR.
- Go to settings > Camera.
- Enable 2D Room View Opaque.
- **Turn On Room View** (click the eye icon bottom right)
- **Put headset somewhere well illuminated, looking above horizon on something with details.
  Your bookshelf, mandala, whatever, just don't put it in front of empty wall and don't make it look down.**
- Open VR View/Mirror window to check if everything is ok, you should be seeing camera image in it.
- Run the vrcamdump.exe.
- Move camera somewhere else and run program again (repeat 2-5 times).
- Compress `dumps` folder and send it to me.

### It will do the following:
- Load DirectX and OpenVR
- Lookup `InstallPath` in `HKEY_LOCAL_MACHINE\SOFTWARE\Valve\Steam`
  or `HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\Valve\Steam`
  for steam path or fallback to `C:\Program Files (x86)\Steam`
- Copy all calibration data from `%STEAM%\config\lighthouse\<serial number>\config.json`, there is nothing sensitive here
- Save all intrinsics data from OpenVR
- Save one frame from HMD output(that's why you need to enable Room View)
- Save one frame from camera
