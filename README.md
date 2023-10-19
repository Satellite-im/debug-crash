this is a minimal example to reproduce a crash Uplink experienced. 
note that the name of this executable is `suplink`, not `uplink`. 

reproducing the crash unfortunately requires making a .dmg file. One needs to run this command: `make dmg SIGNING_KEY=<your signing key>` and then `target/release/macos/Suplink.app/Contents/MacOs/suplink`. Running the app like this: `target/release/suplink` or this: `target/debug/suplink` will result in the app hanging, but not no crash report. 


## example terminal output

```
target/release/macos/Suplink.app/Contents/MacOS/suplink
2023-10-19T19:09:52.815402Z TRACE tao::platform_impl::platform::app_delegate: Triggered `applicationDidFinishLaunching`    
2023-10-19T19:09:52.815923Z TRACE dioxus_core::scope_context: providing context alloc::rc::Rc<dioxus_core::error_boundary::ErrorBoundary> (TypeId { t: 12767822429240257985 }) in app
2023-10-19T19:09:52.815947Z TRACE tao::platform_impl::platform::window: Creating new window    
2023-10-19T19:09:52.833074Z TRACE tao::platform_impl::platform::view: Triggered `viewDidMoveToWindow`    
2023-10-19T19:09:52.833093Z TRACE tao::platform_impl::platform::view: Completed `viewDidMoveToWindow`    
2023-10-19T19:09:52.833134Z TRACE tao::platform_impl::platform::view: Triggered `validAttributesForMarkedText`    
2023-10-19T19:09:52.833137Z TRACE tao::platform_impl::platform::view: Completed `validAttributesForMarkedText`    
2023-10-19T19:09:52.833141Z TRACE tao::platform_impl::platform::view: Triggered `validAttributesForMarkedText`    
2023-10-19T19:09:52.833143Z TRACE tao::platform_impl::platform::view: Completed `validAttributesForMarkedText`    
2023-10-19T19:09:52.833157Z TRACE tao::platform_impl::platform::view: Triggered `validAttributesForMarkedText`    
2023-10-19T19:09:52.833159Z TRACE tao::platform_impl::platform::view: Completed `validAttributesForMarkedText`    
2023-10-19T19:09:52.833161Z TRACE tao::platform_impl::platform::view: Triggered `validAttributesForMarkedText`    
2023-10-19T19:09:52.833162Z TRACE tao::platform_impl::platform::view: Completed `validAttributesForMarkedText`    
2023-10-19T19:09:52.833164Z TRACE tao::platform_impl::platform::view: Triggered `validAttributesForMarkedText`    
2023-10-19T19:09:52.833166Z TRACE tao::platform_impl::platform::view: Completed `validAttributesForMarkedText`    
2023-10-19T19:09:52.833754Z TRACE tao::platform_impl::platform::window: Locked shared state in `set_fullscreen`    
2023-10-19T19:09:52.833760Z TRACE tao::platform_impl::platform::window: Unlocked shared state in `set_fullscreen`    
2023-10-19T19:09:52.873669Z TRACE tao::platform_impl::platform::view: Triggered `validAttributesForMarkedText`    
2023-10-19T19:09:52.873682Z TRACE tao::platform_impl::platform::view: Completed `validAttributesForMarkedText`    
2023-10-19T19:09:52.873689Z TRACE tao::platform_impl::platform::view: Triggered `validAttributesForMarkedText`    
2023-10-19T19:09:52.873690Z TRACE tao::platform_impl::platform::view: Completed `validAttributesForMarkedText`    
2023-10-19T19:09:52.873704Z TRACE tao::platform_impl::platform::view: Triggered `validAttributesForMarkedText`    
2023-10-19T19:09:52.873706Z TRACE tao::platform_impl::platform::view: Completed `validAttributesForMarkedText`    
2023-10-19T19:09:52.873708Z TRACE tao::platform_impl::platform::view: Triggered `validAttributesForMarkedText`    
2023-10-19T19:09:52.873723Z TRACE tao::platform_impl::platform::view: Completed `validAttributesForMarkedText`    
2023-10-19T19:09:52.873725Z TRACE tao::platform_impl::platform::view: Triggered `validAttributesForMarkedText`    
2023-10-19T19:09:52.873727Z TRACE tao::platform_impl::platform::view: Completed `validAttributesForMarkedText`    
2023-10-19T19:09:52.873764Z TRACE tao::platform_impl::platform::view: Triggered `viewDidMoveToWindow`    
2023-10-19T19:09:52.873778Z TRACE tao::platform_impl::platform::view: Completed `viewDidMoveToWindow`    
2023-10-19T19:09:52.875647Z TRACE dioxus_core::scope_context: providing context alloc::rc::Rc<dioxus_desktop::desktop_context::DesktopService> (TypeId { t: 9208843188310989457 }) in app
2023-10-19T19:09:52.875662Z TRACE dioxus_core::scope_context: looking for context alloc::rc::Rc<dioxus_desktop::desktop_context::DesktopService> (TypeId { t: 9208843188310989457 }) in app
2023-10-19T19:09:52.875664Z TRACE dioxus_core::scope_context: providing context alloc::rc::Rc<dyn dioxus_html::eval::EvalProvider> (TypeId { t: 5610108328665325295 }) in app
2023-10-19T19:09:52.875675Z TRACE tao::platform_impl::platform::app_delegate: Completed `applicationDidFinishLaunching`    
zsh: segmentation fault  target/release/macos/Suplink.app/Contents/MacOS/suplink
```


## associated crash report
```
-------------------------------------
Translated Report (Full Report Below)
-------------------------------------

Process:               suplink [68839]
Path:                  /Users/USER/*/Suplink.app/Contents/MacOS/suplink
Identifier:            im.satellite.suplink
Version:               0.0.0-pre (1)
Code Type:             ARM-64 (Native)
Parent Process:        zsh [54490]
Responsible:           Terminal [1432]
User ID:               501

Date/Time:             2023-10-19 15:11:44.3629 -0400
OS Version:            macOS 14.0 (23A344)
Report Version:        12
Anonymous UUID:        DAD91F1B-94ED-81DA-F94B-82E9A959EF93

Sleep/Wake UUID:       821CE9F6-C6B8-40FD-B190-F04B9D21AC12

Time Awake Since Boot: 28000 seconds
Time Since Wake:       20119 seconds

System Integrity Protection: enabled

Crashed Thread:        0  main  Dispatch queue: com.apple.main-thread

Exception Type:        EXC_BAD_ACCESS (SIGSEGV)
Exception Codes:       KERN_INVALID_ADDRESS at 0x0000529f4a75d3a0
Exception Codes:       0x0000000000000001, 0x0000529f4a75d3a0

Termination Reason:    Namespace SIGNAL, Code 11 Segmentation fault: 11
Terminating Process:   exc handler [68839]

VM Region Info: 0x529f4a75d3a0 is not in any region.  Bytes after previous region: 90363066176417  Bytes before following region: 14709013752928
      REGION TYPE                    START - END         [ VSIZE] PRT/MAX SHRMOD  REGION DETAIL
      commpage (reserved)        1000000000-7000000000   [384.0G] ---/--- SM=NUL  ...(unallocated)
--->  GAP OF 0x5f9000000000 BYTES
      MALLOC_NANO              600000000000-600020000000 [512.0M] rw-/rwx SM=PRV  

Thread 0 Crashed:: main Dispatch queue: com.apple.main-thread
0   libobjc.A.dylib               	       0x18e29f7e0 objc_retain + 16
1   AppKit                        	       0x192ce20ac -[NSMenuBarDisplayWindow _ensureReplicantWindows] + 104
2   AppKit                        	       0x192bf4578 __62-[NSMenuBarPresentationInstance _setBoundsAndUpdateResolution]_block_invoke + 736
3   AppKit                        	       0x192bf79a0 -[NSMenuBarPresentationInstance _forEachWindowDo:withBlock:] + 476
4   AppKit                        	       0x192bf426c -[NSMenuBarPresentationInstance _setBoundsAndUpdateResolution] + 124
5   AppKit                        	       0x192bf6908 -[NSMenuBarPresentationInstance _show:shouldNotify:inOrder:forAutoShow:requestVisibility:] + 160
6   AppKit                        	       0x192bf8d14 +[NSMenuBarPresentationInstance _updateMenuBarObscured:] + 352
7   AppKit                        	       0x192bf8b60 +[NSMenuBarPresentationInstance _setMenuBarObscured:] + 60
8   AppKit                        	       0x19206db08 -[NSApplication _handleActivatedEvent:] + 268
9   AppKit                        	       0x1926efcb8 -[NSApplication(NSEventRouting) sendEvent:] + 1732
10  suplink                       	       0x100ced184 objc_exception::try_no_ret::try_objc_execute_closure::h7845ee00c1cc98bf + 48
11  suplink                       	       0x100d2f888 RustObjCExceptionTryCatch + 28
12  suplink                       	       0x100d0e1ec objc::message::platform::send_super_unverified::h2048aaab928b86aa + 104
13  suplink                       	       0x100d0ecc0 tao::platform_impl::platform::app::send_event::h75c3fc1453409912 + 972
14  AppKit                        	       0x1923431bc -[NSApplication _handleEvent:] + 60
15  AppKit                        	       0x191f10460 -[NSApplication run] + 512
16  suplink                       	       0x100c80164 objc_exception::try_no_ret::try_objc_execute_closure::hc952903a968aeed1 + 44
17  suplink                       	       0x100d2f888 RustObjCExceptionTryCatch + 28
18  suplink                       	       0x100c807d4 objc::message::platform::send_unverified::hd609a2b9339a3d61 + 92
19  suplink                       	       0x100c9af28 tao::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::run::h68279befe1b7b5fa + 604
20  suplink                       	       0x100caf378 dioxus_desktop::launch_with_props::h7452008fb08c365d + 436
21  suplink                       	       0x100caf1c4 dioxus_desktop::launch_cfg::hee1b635f90fa0dfd + 12
22  suplink                       	       0x100c6a260 suplink::main::hddbd39ada35b5f47 + 3124
23  suplink                       	       0x100c68334 std::sys_common::backtrace::__rust_begin_short_backtrace::h3a46b8da45c93f72 + 12
24  suplink                       	       0x100c6ad94 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc33dfcbfc4163385 + 16
25  suplink                       	       0x100dab73c std::rt::lang_start_internal::h8ee16b8f6c950a26 + 648
26  suplink                       	       0x100c6a64c main + 52
27  dyld                          	       0x18e2ed058 start + 2224
```