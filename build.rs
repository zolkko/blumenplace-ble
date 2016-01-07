extern crate gcc;

use std::env::set_var;


fn main() {
    set_var("CC", "arm-none-eabi-gcc");

    gcc::Config::new()
        .define("ARM_MATH_CM0", Some("1"))
        .define("NRF51", Some("1"))
        .define("TARGET_M0", Some("1"))
        .define("TARGET_MCU_NORDIC_16K", Some("1"))
        .define("TARGET_MCU_NRF51822", Some("1"))
        .define("TARGET_NORDIC", Some("1"))
        .define("TARGET_NRF51822", Some("1"))
        .define("TARGET_NRF51822_MKIT", Some("1"))
        .define("TOOLCHAIN_GCC", Some("1"))
        .define("TOOLCHAIN_GCC_ARM", Some("1"))
        .define("__CORTEX_M0", Some("1"))
        .flag("-mcpu=cortex-m0")
        .flag("-march=armv6-m")
        .flag("-mthumb")
        .flag("-mabi=aapcs")
        .flag("-mfloat-abi=soft")
        .flag("-std=gnu99")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-fmessage-length=0")
        .flag("-fno-delete-null-pointer-checks")
        .flag("-fomit-frame-pointer")
        .flag("-funsigned-bitfields")
        .flag("-fno-common")
        .flag("-fno-builtin")
        .flag("-Os")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-missing-field-initializers")
        .flag("-Wno-error=switch")
        .flag("-Wno-switch")
        .include("src")
        .include("third_party/nrf51_sdk/toolchain")
        .include("third_party/nrf51_sdk/toolchain/gcc")
        .include("third_party/nrf51_sdk/device")
        .include("third_party/nrf51_sdk/softdevice/s110/headers")
        .target("arm-none-eabi")
        .file("third_party/nrf51_sdk/toolchain/system_nrf51.c")
        .file("third_party/nrf51_sdk/toolchain/gcc/gcc_startup_nrf51.s")
        .compile("libnrf51.a");
}
