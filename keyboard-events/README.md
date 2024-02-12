# keyboard-events

## Environment

```powershell
PS C:\Users\owner> [System.Environment]::OSVersion.Version
Major  Minor  Build  Revision
-----  -----  -----  --------
10     0      19045  0


PS C:\Users\owner> rustc -V
rustc 1.77.0-nightly (ef71f1047 2024-01-21)
```

## Execution Result

This section displays the result of the operation where the `Alt` key is pressed, `0` key is entered, and then the `Alt` key is released.

```text
[WH_KEYBOARD_LL] WM_SYSKEYDOWN: vkCode: 165(0xa5), scanCode: 56(0x38), flags: 33(0b100001), time: 1514303468, dwExtraInfo: 0
[WH_KEYBOARD] code: 0, wparam: 18(0x12), lparam: 557318145(0b100001001110000000000000000001)
[WH_KEYBOARD_LL] WM_SYSKEYDOWN: vkCode: 48(0x30), scanCode: 11(0xb), flags: 32(0b100000), time: 1514303640, dwExtraInfo: 0
[WH_KEYBOARD] code: 0, wparam: 48(0x30), lparam: 537591809(0b100000000010110000000000000001)
[WNDPROC] WM_SYSCHAR: wparam: 48(0x30), lparam: 537591809(0b100000000010110000000000000001)
[WH_KEYBOARD_LL] WM_SYSKEYUP: vkCode: 48(0x30), scanCode: 11(0xb), flags: 160(0b10100000), time: 1514303703, dwExtraInfo: 0
[WH_KEYBOARD] code: 0, wparam: 48(0x30), lparam: 3758817281(0b11100000000010110000000000000001)
[WH_KEYBOARD_LL] WM_KEYUP: vkCode: 165(0xa5), scanCode: 56(0x38), flags: 129(0b10000001), time: 1514303906, dwExtraInfo: 0
[WH_KEYBOARD] code: 0, wparam: 18(0x12), lparam: 3241672705(0b11000001001110000000000000000001)
[WNDPROC] WM_KEYUP: wparam: 18(0x12), lparam: 3241672705(0b11000001001110000000000000000001)
```
