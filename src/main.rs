use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;
use csv::Writer;
use std::fs::File;
use chrono::*;
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tty_path = "COM3";
    let slave = Slave(0x17);

    let builder = tokio_serial::new(tty_path, 19200);
    let port = SerialStream::open(&builder).unwrap();

    let mut ctx = rtu::attach_slave(port, slave);

    let file = File::create("bms_readings.csv")?;
    let mut writer = Writer::from_writer(file);

    writer.write_record(&["Timestamp", "BMS Value"])?;

    loop {
        let rsp = ctx.read_holding_registers(0x082B, 1).await?;
        let bms_value = rsp[0];
        println!("BMS value is: {}", bms_value);

        let now = Utc::now().to_rfc3339();
        writer.write_record(&[now, bms_value.to_string()])?;
        writer.flush()?;

        sleep(Duration::from_secs(3)).await;

    }
}