/****************************************************************************
 * apps/examples/hello_rust/hello_rust_main.rs
 *
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.  The
 * ASF licenses this file to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance with the
 * License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
 * WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.  See the
 * License for the specific language governing permissions and limitations
 * under the License.
 *
 ****************************************************************************/

/****************************************************************************
 * Attributes
 ****************************************************************************/
 
 #![no_main]
 #![no_std]
 
 /****************************************************************************
  * Uses
  ****************************************************************************/
 
 use core::panic::PanicInfo;
 use core::ffi::{ c_void };
 
 /****************************************************************************
  * Externs
  ****************************************************************************/
 
 extern "C" {
     pub fn printf(format: *const u8, ...) -> i32;
     pub fn puts(s: *const u8) -> i32;
     pub fn fgets(buf: *mut u8, n: i32, stream: *mut c_void) -> *mut u8;
     pub fn lib_get_stream(fd: i32) -> *mut c_void;
     
     pub fn open(path: *const u8, oflag: i32, ...) -> i32;
     pub fn read(fd: i32, buf: *mut u8, count: u32) -> i32;
     pub fn write(fd: i32, buf: *const u8, count: u32) -> i32;
     pub fn close(fd: i32) -> i32;
     pub fn ioctl(fd: i32, request: i32, ...) -> i32;
     pub fn sleep(secs: u32) -> u32;
     pub fn usleep(usec: u32) -> u32;
     pub fn exit(status: u32) -> !;
 }
 
 /****************************************************************************
  * Rust Constants
  ****************************************************************************/
 pub const O_WRONLY: i32 = 1 << 1;
 pub const ULEDIOC_SETALL: i32 = 0x1d03;
 
 /****************************************************************************
  * Private functions
  ****************************************************************************/
 
 /****************************************************************************
  * Panic handler (needed for [no_std] compilation)
  ****************************************************************************/
 
 #[panic_handler]
 fn panic(_panic: &PanicInfo<'_>) -> ! {
     loop {}
 }
 
 /****************************************************************************
  * Public functions
  ****************************************************************************/
 
 /****************************************************************************
  * hello_rust_main
  ****************************************************************************/
 
 #[no_mangle]
 pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
     
     unsafe {
         printf(b"Hello, Rust!!\n\0" as *const u8);
         
         printf(b"Opening /dev/userleds\n\0" as *const u8);
         
         let fd = open(b"/dev/userleds\0" as *const u8, O_WRONLY);
         
         if fd < 0 {
             printf(b"Unable to open /dev/userleds, skipping the blinking\n\0" as *const u8);
             return 1;
         }
         
         printf(b"Set LED 1 to 1\n\0" as *const u8);
         let ret = ioctl(fd, ULEDIOC_SETALL, 1); // Convert ULEDIOC_SETALL (u16) to i32
         
         if ret < 0 {
             printf(b"ERROR: ioctl(ULEDIOC_SUPPORTED) failed\n\0" as *const u8);
             return 1;
         }
         
         // Sleep a while...
         printf(b"Waiting...\n\0" as *const u8);
         usleep(500 * 1000);
         
         printf(b"Set LED 1 to 0\n\0" as *const u8);
         let ret = ioctl(fd, ULEDIOC_SETALL, 0);
         
         if ret < 0 {
             printf(b"ERROR: ioctl(ULEDIOC_SUPPORTED) failed\n\0" as *const u8);
             return 1;
         }
         close(fd);
     }
     
     /* exit with status 0 */
     0
 }
 