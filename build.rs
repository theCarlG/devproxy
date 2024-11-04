use std::env;
use std::time::SystemTime;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

fn main() {
    env::var("GIT_SHA").map_or_else(
        |_| println!("cargo:rustc-env=GIT_SHA=dev"),
        |val| println!("cargo:rustc-env=GIT_SHA={val}"),
    );
    let val = OffsetDateTime::from(SystemTime::now())
        .format(&Rfc3339)
        .unwrap();
    println!("cargo:rustc-env=BUILD_DATE={val}");
}
