//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    // let your_command = format!(
    //     "cargo:TEST_FOO={}",
    //     timestamp
    // );
    // let s = std::env::var("TEST_FOO").unwrap();
    // let e: u64 = s.parse().unwrap();
    // if timestamp<e{
    //     timestamp=e;
    // }
    // if timestamp>e{
    //     timestamp=e;
    // }
    // println!("cargo:TEST_FOO={}", timestamp);
    let range_start = timestamp;
    let range_end = timestamp + 10;

    // Set the TEST_FOO environment variable with the range information.
    println!("cargo:rustc-env=TEST_FOO={}", range_start);
    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    //let your_command = "cargo:rustc-cfg=pass";
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
