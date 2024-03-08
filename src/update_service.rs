/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

use std::{fmt};



#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Platform {
    LinuxAlpineX64,
    LinuxAlpineARM64,
    LinuxX64,
    LinuxARM64,
    LinuxARM32,
    DarwinX64,
    DarwinARM64,
    WindowsX64,
    WindowsX86,
    WindowsARM64,
}

impl Platform {
    pub fn archive(&self) -> Option<String> {
        match self {
            Platform::LinuxX64 => Some("linux-x64".to_owned()),
            Platform::LinuxARM64 => Some("linux-arm64".to_owned()),
            Platform::LinuxARM32 => Some("linux-armhf".to_owned()),
            Platform::DarwinX64 => Some("darwin".to_owned()),
            Platform::DarwinARM64 => Some("darwin-arm64".to_owned()),
            Platform::WindowsX64 => Some("win32-x64-archive".to_owned()),
            Platform::WindowsX86 => Some("win32-archive".to_owned()),
            Platform::WindowsARM64 => Some("win32-arm64-archive".to_owned()),
            _ => None,
        }
    }
    pub fn headless(&self) -> String {
        match self {
            Platform::LinuxAlpineARM64 => "server-alpine-arm64",
            Platform::LinuxAlpineX64 => "server-linux-alpine",
            Platform::LinuxX64 => "server-linux-x64",
            Platform::LinuxARM64 => "server-linux-arm64",
            Platform::LinuxARM32 => "server-linux-armhf",
            Platform::DarwinX64 => "server-darwin",
            Platform::DarwinARM64 => "server-darwin-arm64",
            Platform::WindowsX64 => "server-win32-x64",
            Platform::WindowsX86 => "server-win32",
            Platform::WindowsARM64 => "server-win32-x64", // we don't publish an arm64 server build yet
        }
        .to_owned()
    }

    pub fn cli(&self) -> String {
        match self {
            Platform::LinuxAlpineARM64 => "cli-alpine-arm64",
            Platform::LinuxAlpineX64 => "cli-alpine-x64",
            Platform::LinuxX64 => "cli-linux-x64",
            Platform::LinuxARM64 => "cli-linux-arm64",
            Platform::LinuxARM32 => "cli-linux-armhf",
            Platform::DarwinX64 => "cli-darwin-x64",
            Platform::DarwinARM64 => "cli-darwin-arm64",
            Platform::WindowsARM64 => "cli-win32-arm64",
            Platform::WindowsX64 => "cli-win32-x64",
            Platform::WindowsX86 => "cli-win32",
        }
        .to_owned()
    }

    pub fn web(&self) -> String {
        format!("{}-web", self.headless())
    }

    pub fn env_default() -> Option<Platform> {
        if cfg!(all(
            target_os = "linux",
            target_arch = "x86_64",
            target_env = "musl"
        )) {
            Some(Platform::LinuxAlpineX64)
        } else if cfg!(all(
            target_os = "linux",
            target_arch = "aarch64",
            target_env = "musl"
        )) {
            Some(Platform::LinuxAlpineARM64)
        } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
            Some(Platform::LinuxX64)
        } else if cfg!(all(target_os = "linux", target_arch = "arm")) {
            Some(Platform::LinuxARM32)
        } else if cfg!(all(target_os = "linux", target_arch = "aarch64")) {
            Some(Platform::LinuxARM64)
        } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
            Some(Platform::DarwinX64)
        } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            Some(Platform::DarwinARM64)
        } else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
            Some(Platform::WindowsX64)
        } else if cfg!(all(target_os = "windows", target_arch = "x86")) {
            Some(Platform::WindowsX86)
        } else if cfg!(all(target_os = "windows", target_arch = "aarch64")) {
            Some(Platform::WindowsARM64)
        } else {
            None
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Platform::LinuxAlpineARM64 => "LinuxAlpineARM64",
            Platform::LinuxAlpineX64 => "LinuxAlpineX64",
            Platform::LinuxX64 => "LinuxX64",
            Platform::LinuxARM64 => "LinuxARM64",
            Platform::LinuxARM32 => "LinuxARM32",
            Platform::DarwinX64 => "DarwinX64",
            Platform::DarwinARM64 => "DarwinARM64",
            Platform::WindowsX64 => "WindowsX64",
            Platform::WindowsX86 => "WindowsX86",
            Platform::WindowsARM64 => "WindowsARM64",
        })
    }
}
