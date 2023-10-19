this is a minimal example to reproduce a crash Uplink experienced. 
note that the name of this executable is `suplink`, not `uplink`. 

reproducing the crash unfortunately requires making a .dmg file. One needs to run this command: `make dmg SIGNING_KEY=<your signing key>` and then `target/release/macos/Suplink.app/Contents/MacOs/suplink`. Running the app like this: `target/release/suplink` or this: `target/debug/suplink` will result in the app hanging, but not no crash report. 