extern crate cc;
extern crate dirs;

use std::env;
use std::path::Path;

// ERPC_INFINIBAND=true cargo build
// ERPC_RAW=true cargo build
// ERPC_DPDK=true cargo build

fn main() {
    let home_dir = dirs::home_dir().unwrap();

    let mut erpc_flag: String = String::from("-DERPC_INFINIBAND=true");

    match env::var("ERPC_INFINIBAND") {
        Ok(_val) => erpc_flag = String::from("-DERPC_INFINIBAND=true"),
        Err(_e) => (),
    }
    match env::var("ERPC_RAW") {
        Ok(_val) => erpc_flag = String::from("-DERPC_RAW=true"),
        Err(_e) => (),
    }
    match env::var("ERPC_DPDK") {
        Ok(_val) => erpc_flag = String::from("-DERPC_DPDK=true"),
        Err(_e) => (),
    }

    cc::Build::new()
        .cpp(true)
        .flag("-g")
        .flag("-std=c++11") // -std=c++1y
        .flag(erpc_flag.as_str()) // "-DERPC_INFINIBAND=true"
        .include(Path::new(&home_dir).join("eRPC/src"))
        .include(Path::new(&home_dir).join("junction-install/include"))
        .include("src")
        .file("src/ffi.cpp")
        .file("src/server.cpp")
        .file("src/client.cpp")
        .warnings(false)
        .compile("erpc_ffi");

    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&home_dir).join("eRPC/build").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&home_dir).join("junction-install/lib").display()
    );

    println!("cargo:rustc-link-lib=erpc");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=numa");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=ibverbs");

    println!("cargo:rustc-link-lib=junction");
    println!("cargo:rustc-link-lib=turf");

    println!("cargo:rerun-if-changed=src/*");
    println!("cargo:rerun-if-changed=build.rs");
}

/*
# ~/eRPC/hello_world/Makefile

#Notes:
#1. The make target depends on how eRPC was compiled:
#   * If DTRANSPORT=dpdk, use `make dpdk`
#   * If DTRANSPORT=raw, use `make raw`
#   * If DTRANSPORT=infiniband, use `make infiniband`
LIBS = -lerpc -lpthread -lnuma -ldl

error:
    @echo "Please choose one of the following targets: infiniband, raw, dpdk, clean"
    @exit 2
infiniband:
    g++ -g -std=c++11 -o server server.cc -I ../src -L ../build $(LIBS) -libverbs -DERPC_INFINIBAND=true
    g++ -g -std=c++11 -o client client.cc -I ../src -L ../build $(LIBS) -libverbs -DERPC_INFINIBAND=true
raw:
    g++ -g -std=c++11 -o server server.cc -I ../src -L ../build $(LIBS) -libverbs -DERPC_RAW=true
    g++ -g -std=c++11 -o client client.cc -I ../src -L ../build $(LIBS) -libverbs -DERPC_RAW=true
dpdk:
    g++ -g -std=c++11 -o server server.cc -I ../src -I /usr/local/include/dpdk -L ../build $(LIBS) -ldpdk -DERPC_DPDK=true -march=native
    g++ -g -std=c++11 -o client client.cc -I ../src -I /usr/local/include/dpdk -L ../build $(LIBS) -ldpdk -DERPC_DPDK=true -march=native
clean:
    rm server client
*/
