fn main() {
    let nmxhep = "4000"; // 4000 is the default used by GEANT, POWHEG, etc.


    let output = format!("//This file is automatically generate by build.rs\n//Do not modify!\npub const NMXHEP: usize = {};", nmxhep);
    std::fs::write("src/hepevt_size.rs", output).expect("Failed to write generated code");

    let output = format!("!This file is automatically generate by build.rs\n!Do not modify!\n      PARAMETER (NMXHEP={})", nmxhep);
    std::fs::write("fortran/hepevt_size.h", output).expect("Failed to write generated code");

    cc::Build::new()
        .file("fortran/common_block.F")
        .compiler("gfortran")
        .define("hepevt_nmxhep", nmxhep) // Pass nmxhep as a define
        .compile("common_block"); // outputs libcommon_block.a

    println!("cargo:rerun-if-changed=fortran/common_block.F");
    println!("cargo:rerun-if-changed=fortran/hepevt.h");
    println!("cargo:rerun-if-changed=build.rs");
}