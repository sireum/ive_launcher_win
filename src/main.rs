/*
 Copyright (c) 2017-2024, Robby, Kansas State University
 All rights reserved.

 Redistribution and use in source and binary forms, with or without
 modification, are permitted provided that the following conditions are met:

 1. Redistributions of source code must retain the above copyright notice, this
    list of conditions and the following disclaimer.
 2. Redistributions in binary form must reproduce the above copyright notice,
    this list of conditions and the following disclaimer in the documentation
    and/or other materials provided with the distribution.

 THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
#![windows_subsystem = "windows"]
use std::env;
use std::process::{Command, exit};
use std::os::windows::process::CommandExt;
extern crate winapi;

fn main() {
    let mut pb = env::current_exe().unwrap();
    pb.pop();
    let mut bin_pb = pb.clone();
    bin_pb.push("idea.bat");
    pb.pop();
    pb.pop();
    pb.pop();
    pb.pop();
    let sireum_home = pb.as_path().as_os_str().to_str().unwrap();
    let idea_bat_path = bin_pb.as_path();
    let idea_bat = idea_bat_path.as_os_str().to_str().unwrap();
    if idea_bat_path.exists() {
        let output = Command::new("PowerShell")
            .args(&["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command",
                format!("Start-Process -FilePath \"{}\" -WindowStyle Hidden", idea_bat).as_str()])
            .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
            .env("SIREUM_HOME", sireum_home)
            .spawn();
        if output.is_err() {
            eprintln!("Could not launch {}", idea_bat);
            exit(-1)
        }
    } else {
        eprintln!("{} is not found", idea_bat);
        exit(-2)
    }
}
