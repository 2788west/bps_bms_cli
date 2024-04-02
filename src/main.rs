use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;
use csv::Writer;
use std::fs::File;
use std::task::Context;
use chrono::*;
use tokio::time::{sleep, Duration};
use serde::Serialize;

#[derive(Serialize)]
#[derive(Debug)]
struct BMSData {
    time: String,
    voltage: f32,
    no_cells: u16,
    soc: u16,
    capacity: f32,
    output_current: f32,
    charging_current: f32,
    temp_1: u16,
    temp_2: u16,
    temp_3: u16,
    cell_v_1: f32,
    cell_v_2: f32,
    cell_v_3: f32,
    cell_v_4: f32,
    cell_v_5: f32,
    cell_v_6: f32,
    cell_v_7: f32,
    cell_v_8: f32,
    cell_v_9: f32,
    cell_v_10: f32,
    cell_v_11: f32,
    cell_v_12: f32,
    cell_v_13: f32,
    cell_v_14: f32,
    cell_v_15: f32,
    cell_v_16: f32,
    cell_v_17: f32,
    cell_v_18: f32,
    cell_v_19: f32,
    cell_v_20: f32,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut ctx = create_connection();

    let file = File::create("bms_readings.csv")?;
    let mut writer = Writer::from_writer(file);

    loop {
        // Read all 29 holding registers starting at address 0
        // let rsp = ctx.read_holding_registers(0, 29).await?;
        // println!("Received: {rsp:?}");
        // let data = parse_response(rsp);
        let data = vec![
            // Test data for evaluation
            // The documented response includes 3 additional leading bytes
            // that are omitted here
            0x1770, 0x0011, 0x005A, 0x06F6, 0x0064,
            0x0000, 0x0016, 0x0017, 0x0018, 0x101B,
            0x1002, 0x1010, 0x107E, 0x0FAC, 0x0FC1,
            0x0FCC, 0x0FD7, 0x0FE2, 0x0FED, 0x0FF8,
            0x1003, 0x1004, 0x100F, 0x101A, 0x1025,
            0x1030, 0x103B, 0x1046, 0x1051
        ];
        let formatted_data = format_values(data);

        for data in &formatted_data {
            println!("{:#?}", data);
        }

        for data in formatted_data {
            writer.serialize(data)?;
        }
        writer.flush()?;

        println!("Data written to CSV.");

        sleep(Duration::from_secs(5)).await;

    }
}

/// Create a new serial connection to the BMS
/// Requires a USB to RS485 converter
fn create_connection() -> tokio_modbus::client::Context {
    let tty_path = "COM3";
    let slave = Slave(1); // Peripheral address is 1

    let builder = tokio_serial::new(tty_path, 9600);
    let port = SerialStream::open(&builder).unwrap();

    return rtu::attach_slave(port, slave);
}

/// Parses 2 byte HEX values into decimal values
fn parse_response(data: Vec<u16>) -> Vec<u16> {
    data.into_iter().collect()
}

/// Formats decimals values into a BMSData struct, performing
/// conversions according to the manufacturers specification
fn format_values(data: Vec<u16>) -> Vec<BMSData> {
    let data = vec![
        BMSData {
            time: Utc::now().to_rfc3339(),
            voltage: data[0] as f32 / 100.0,
            no_cells: data[1],
            soc: data[2],
            capacity: data[3] as f32 / 10.0,
            output_current: data[4] as f32 / 10.0,
            charging_current: data[5] as f32 / 10.0,
            temp_1: data[6],
            temp_2: data[7],
            temp_3: data[8],
            cell_v_1: data[9] as f32 / 1000.0,
            cell_v_2: data[10] as f32 / 1000.0,
            cell_v_3: data[11] as f32 / 1000.0,
            cell_v_4: data[12] as f32 / 1000.0,
            cell_v_5: data[13] as f32 / 1000.0,
            cell_v_6: data[14] as f32 / 1000.0,
            cell_v_7: data[15] as f32 / 1000.0,
            cell_v_8: data[16] as f32 / 1000.0,
            cell_v_9: data[17] as f32 / 1000.0,
            cell_v_10: data[18] as f32 / 1000.0,
            cell_v_11: data[19] as f32 / 1000.0,
            cell_v_12: data[20] as f32 / 1000.0,
            cell_v_13: data[21] as f32 / 1000.0,
            cell_v_14: data[22] as f32 / 1000.0,
            cell_v_15: data[23] as f32 / 1000.0,
            cell_v_16: data[24] as f32 / 1000.0,
            cell_v_17: data[25] as f32 / 1000.0,
            cell_v_18: data[26] as f32 / 1000.0,
            cell_v_19: data[27] as f32 / 1000.0,
            cell_v_20: data[28] as f32 / 1000.0,
        },
    ];

    data
}
