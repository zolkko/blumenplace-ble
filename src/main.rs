#![feature(lang_items, main)]
#![no_std]
#![no_main]
#![feature(asm)]


#[no_mangle]
#[allow(dead_code)]
#[allow(unreachable_code)]
pub extern "C" fn abort() {
    loop {}
}

const SVC_BLE_BASE: u8 = 0x60;
const SD_BLE_ENABLE: u8 = SVC_BLE_BASE;
const SD_BLE_EVT_GET: u8 = SD_BLE_ENABLE + 1;
const SD_BLE_TX_BUFFER_COUNT_GET: u8 = SD_BLE_EVT_GET + 1;
const SD_BLE_UUID_VS_ADD: u8 = SD_BLE_TX_BUFFER_COUNT_GET + 1;
const SD_BLE_UUID_DECODE: u8 = SD_BLE_UUID_VS_ADD + 1;
const SD_BLE_UUID_ENCODE: u8 = SD_BLE_UUID_DECODE + 1;
const SD_BLE_VERSION_GET: u8 = SD_BLE_UUID_ENCODE + 1;
const SD_BLE_USER_MEM_REPLY: u8 = SD_BLE_VERSION_GET + 1;
const SD_BLE_OPT_SET: u8 = SD_BLE_USER_MEM_REPLY + 1;
const SD_BLE_OPT_GET: u8 = SD_BLE_OPT_SET + 1;

const NRF_ERROR_BASE_NUM: u32 = 0;
const NRF_SUCCESS: u32 = NRF_ERROR_BASE_NUM + 0;
const NRF_ERROR_SVC_HANDLER_MISSING: u32 = NRF_ERROR_BASE_NUM + 1;
const NRF_ERROR_SOFTDEVICE_NOT_ENABLED: u32 = NRF_ERROR_BASE_NUM + 2;
const NRF_ERROR_INTERNAL: u32 = NRF_ERROR_BASE_NUM + 3;
const NRF_ERROR_NO_MEM: u32 = NRF_ERROR_BASE_NUM + 4;
const NRF_ERROR_NOT_FOUND: u32 = NRF_ERROR_BASE_NUM + 5;
const NRF_ERROR_NOT_SUPPORTED: u32 = NRF_ERROR_BASE_NUM + 6;
const NRF_ERROR_INVALID_PARAM: u32 = NRF_ERROR_BASE_NUM + 7;
const NRF_ERROR_INVALID_STATE: u32 = NRF_ERROR_BASE_NUM + 8;
const NRF_ERROR_INVALID_LENGTH: u32 = NRF_ERROR_BASE_NUM + 9;
const NRF_ERROR_INVALID_FLAGS: u32 = NRF_ERROR_BASE_NUM + 10;
const NRF_ERROR_INVALID_DATA: u32 = NRF_ERROR_BASE_NUM + 11;
const NRF_ERROR_DATA_SIZE: u32 = NRF_ERROR_BASE_NUM + 12;
const NRF_ERROR_TIMEOUT: u32 = NRF_ERROR_BASE_NUM + 13;
const NRF_ERROR_NULL: u32 = NRF_ERROR_BASE_NUM + 14;
const NRF_ERROR_FORBIDDEN: u32 = NRF_ERROR_BASE_NUM + 15;
const NRF_ERROR_INVALID_ADDR: u32 = NRF_ERROR_BASE_NUM + 16;
const NRF_ERROR_BUSY: u32 = NRF_ERROR_BASE_NUM + 17;

enum NrfError {
    SvcHandlerMissing,
    SoftDeviceNotEnabled,
    Internal,
    NoMem,
    NotFound,
    NotSupported,
    InvalidParam,
    InvalidState,
    InvalidLength,
    InvalidFlags,
    InvalidData,
    Timeout,
    Null,
    Forbidden,
    InvalidAddr,
    Busy,
    Unknown(u32)
}

const BLE_GATTS_ATTR_TAB_SIZE_MIN: u32 = 216;
const BLE_GATTS_ATTR_TAB_SIZE_DEFAULT: u32 = 0;

#[repr(C)]
struct Version {
    version_number: u8,
    company_id: u16,
    subversion_number: u16
}

impl Version {
    fn new() -> Version {
        Version { version_number: 0, company_id: 0, subversion_number: 0 }
    }
}

#[repr(C)]
struct GattsEnableParams {
    service_changed: u8,
    attr_tab_size: u32
}

#[repr(C)]
struct EnableParams {
    gatts_enable_params: GattsEnableParams
}

#[link(name = "nrf51")]
extern {
    fn wrap_ble_enable(params: *const EnableParams) -> u32;
    fn wrap_ble_version_get(params: *mut Version) -> u32;
    fn wrap_softdevice_handler_init();
    fn advertising_init();
    fn advertising_start();
    fn wrap_sd_app_evt_wait() -> u32;
    fn app_init();
    fn assert_nrf_callback(line_num: u16, p_file_name: usize);
}

fn ble_enable(params: &EnableParams) -> Result<(), NrfError> {
    match unsafe { wrap_ble_enable(params as *const EnableParams) } {
        NRF_SUCCESS => { Ok(()) },
        NRF_ERROR_INVALID_STATE => { Err(NrfError::InvalidState) },
        NRF_ERROR_INVALID_ADDR => { Err(NrfError::InvalidAddr) },
        NRF_ERROR_INVALID_LENGTH => { Err(NrfError::InvalidLength) },
        NRF_ERROR_NO_MEM => { Err(NrfError::NoMem) },
        err => { Err(NrfError::Unknown(err)) }
    }
}

fn ble_version_get() -> Result<Version, NrfError> {
    let mut ver: Version = Version::new();

    match unsafe { wrap_ble_version_get(&mut ver as *mut Version) } {
        0 => { Ok(ver) },
        NRF_ERROR_INVALID_ADDR => { Err(NrfError::InvalidAddr) },
        NRF_ERROR_BUSY => { Err(NrfError::Busy) },
        err => { Err(NrfError::Unknown(err)) }
    }
}

#[no_mangle]
#[allow(dead_code)]
#[allow(unreachable_code)]
pub extern fn app_error_handler(error_code: u32, line_num: u32, p_file_name: *const u8) {
    loop { }
}

#[no_mangle]
#[allow(dead_code)]
#[allow(unreachable_code)]
#[main]
pub extern fn main() -> i32 {

    let mut version: u8 = 0;

    let par = EnableParams {
        gatts_enable_params: GattsEnableParams {
            service_changed: 0,
            attr_tab_size: BLE_GATTS_ATTR_TAB_SIZE_DEFAULT
        }
    };

    unsafe { app_init(); }

    unsafe { wrap_softdevice_handler_init(); }

    loop {
        match ble_enable(&par) {
            Ok(_) => {
                loop {
                    match ble_version_get() {
                        Ok(x) => {
                            version = x.version_number;

                            unsafe { advertising_init(); }
                            unsafe { advertising_start(); }

                            loop {
                                unsafe { wrap_sd_app_evt_wait(); }
                            }
                        }
                        _ => { }
                    }
                }
            },
            Err(_) => { }
        }
    }

    unsafe { assert_nrf_callback(0, 0); }

    return 0;
}


#[lang="eh_personality"]
#[allow(dead_code)]
#[allow(unreachable_code)]
extern fn eh_personality() {
    loop {}
}


#[lang="panic_fmt"]
#[allow(dead_code)]
#[allow(unreachable_code)]
extern fn panic_fmt() -> ! {
    loop {}
}

