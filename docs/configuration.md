# Configuration

The library requires no secrets or environment variables. Capture behavior is configured through the public `capturer::Options` value shown in the root README, including frame rate, target, cursor and highlight visibility, crop area, frame type, resolution, and macOS-only excluded targets.

Platform prerequisites differ:

- macOS uses ScreenCaptureKit and requires Screen Recording permission;
- Windows uses Windows.Graphics.Capture; and
- Linux uses PipeWire and D-Bus development libraries when compiling.

Call `is_supported()` and `has_permission()` before building a capturer. Applications may call `request_permission()` when appropriate. No `.env` file is used.
