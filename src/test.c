
#include <stdbool.h>
#include <stdint.h>

#include <ble.h>
#include <nrf_sdm.h>
#include <softdevice_handler.h>

#include "ble_advdata.h"
#include "nordic_common.h"
#include "app_timer.h"



uint32_t wrap_ble_enable(ble_enable_params_t*);

uint32_t wrap_ble_version_get(ble_version_t*);

void wrap_softdevice_handler_init(void);


uint32_t wrap_ble_enable(ble_enable_params_t * params)
{
    return sd_ble_enable(params);
}

uint32_t wrap_ble_version_get(ble_version_t * params)
{
    return sd_ble_version_get(params);
}

void wrap_softdevice_handler_init(void)
{
    SOFTDEVICE_HANDLER_INIT(NRF_CLOCK_LFCLKSRC_XTAL_20_PPM, false);
}

//--------------------

#define IS_SRVC_CHANGED_CHARACT_PRESENT 0

#define APP_CFG_NON_CONN_ADV_TIMEOUT    0                                 /**< Time for which the device must be advertising in non-connectable mode (in seconds). 0 disables timeout. */
#define NON_CONNECTABLE_ADV_INTERVAL    MSEC_TO_UNITS(100, UNIT_0_625_MS) /**< The advertising interval for non-connectable advertisement (100 ms). This value can vary between 100ms to 10.24s). */

#define APP_BEACON_INFO_LENGTH          0x17
/**< Total length of information advertised by the Beacon. */

#define APP_ADV_DATA_LENGTH             0x15
/**< Length of manufacturer specific data in the advertisement. */

#define APP_DEVICE_TYPE                 0x02
/**< 0x02 refers to Beacon. */

#define APP_MEASURED_RSSI               0xC3
/**< The Beacon's measured RSSI at 1 meter distance in dBm. */

#define APP_COMPANY_IDENTIFIER          0x0059
/**< Company identifier for Nordic Semiconductor ASA. as per www.bluetooth.org. */

#define APP_MAJOR_VALUE                 0x01, 0x02
/**< Major value used to identify Beacons. */

#define APP_MINOR_VALUE                 0x03, 0x04
/**< Minor value used to identify Beacons. */

#define APP_BEACON_UUID                 0x01, 0x12, 0x23, 0x34, \
                                        0x45, 0x56, 0x67, 0x78, \
                                        0x89, 0x9a, 0xab, 0xbc, \
                                        0xcd, 0xde, 0xef, 0xf0            /**< Proprietary UUID for Beacon. */

#define DEAD_BEEF                       0xDEADBEEF
/**< Value used as error code on stack dump, can be used to identify stack
 * location on stack unwind. */

#define APP_TIMER_PRESCALER             0
/**< Value of the RTC1 PRESCALER register. */

#define APP_TIMER_OP_QUEUE_SIZE         4
/**< Size of timer operation queues. */


static ble_gap_adv_params_t m_adv_params;
/**< Parameters to be passed to the stack when starting advertising. */


static uint8_t m_beacon_info[APP_BEACON_INFO_LENGTH] =
/**< Information advertised by the Beacon. */
{
    APP_DEVICE_TYPE,     // Manufacturer specific information. Specifies the device
                         // type in this
                         // implementation.
    APP_ADV_DATA_LENGTH, // Manufacturer specific information. Specifies the length of the
                         // manufacturer specific data in this implementation.
    APP_BEACON_UUID,     // 128 bit UUID value.
    APP_MAJOR_VALUE,     // Major arbitrary value that can be used to distinguish
                         // between Beacons.
    APP_MINOR_VALUE,     // Minor arbitrary value that can be used to distinguish between Beacons.
    APP_MEASURED_RSSI    // Manufacturer specific information. The Beacon's measured TX power in
                         // this implementation.
};

void advertising_init(void)
{
    uint32_t      err_code;
    ble_advdata_t advdata;
    uint8_t       flags = BLE_GAP_ADV_FLAG_BR_EDR_NOT_SUPPORTED;

    ble_advdata_manuf_data_t manuf_specific_data;

    manuf_specific_data.company_identifier = APP_COMPANY_IDENTIFIER;

    /// ---------

    manuf_specific_data.data.p_data = (uint8_t *) m_beacon_info;
    manuf_specific_data.data.size   = APP_BEACON_INFO_LENGTH;

    // Build and set advertising data.
    memset(&advdata, 0, sizeof(advdata));

    advdata.name_type             = BLE_ADVDATA_NO_NAME;
    advdata.flags                 = flags;
    advdata.p_manuf_specific_data = &manuf_specific_data;

    err_code = ble_advdata_set(&advdata, NULL);
    APP_ERROR_CHECK(err_code);

    // Initialize advertising parameters (used when starting advertising).
    memset(&m_adv_params, 0, sizeof(m_adv_params));

    m_adv_params.type        = BLE_GAP_ADV_TYPE_ADV_NONCONN_IND;
    m_adv_params.p_peer_addr = NULL;                             // Undirected advertisement.
    m_adv_params.fp          = BLE_GAP_ADV_FP_ANY;
    m_adv_params.interval    = NON_CONNECTABLE_ADV_INTERVAL;
    m_adv_params.timeout     = APP_CFG_NON_CONN_ADV_TIMEOUT;
}

void advertising_start(void)
{
    uint32_t err_code;

    err_code = sd_ble_gap_adv_start(&m_adv_params);
}

void app_init(void)
{
    APP_TIMER_INIT(APP_TIMER_PRESCALER, APP_TIMER_OP_QUEUE_SIZE, false);
}

void assert_nrf_callback(uint16_t line_num, const uint8_t * p_file_name)
{
    do {} while (1) ;
    app_error_handler(DEAD_BEEF, line_num, p_file_name);
}

uint32_t wrap_sd_app_evt_wait(void)
{
    uint32_t err_code = sd_app_evt_wait();

    APP_ERROR_CHECK(err_code);

    return err_code;
}
