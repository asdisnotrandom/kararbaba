
use crate::veri_tipleri::{LidarNokta, LidarVeri};
use tokio::sync::mpsc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialPortBuilderExt;
use std::time::{UNIX_EPOCH, SystemTime};

const LIDAR_PORT: &str = "amanin";
const LIDAR_BAUD: u32 = 115200;

pub async fn lidar_task(tx: mpsc::Sender<LidarVeri>)
{
    let mut port = tokio_serial::new(LIDAR_PORT, LIDAR_BAUD).open_native_async().expect("Lidar baslamadi!");
    let durdur: [u8; 2] = [0xA5, 0x25];
    let _ = port.write_all(&durdur).await;
    let baslat: [u8; 2] = [0xA5, 0x20];
    if let Err(_) = port.write_all(&baslat).await
    {
        eprintln!("lidar baslamadi!");
    }
    let mut desc = [0u8; 7];
    if let Err(e) = port.read_exact(&mut desc).await {
        eprintln!("Response yanlis: {:?}", e);
        return;
    }
    let mut buf: [u8; 5] = [0u8; 5];{
    let mut bufid = 0;
    let mut anlik_tur: Vec<LidarNokta> = Vec::with_capacity(1500);
    loop {
        if let Ok(_) = port.write_all(&mut buf[bufid..bufid+1]).await
        {
            bufid += 1;
            if bufid == 5
            {
                let check1 = buf[0] & 0x01;
                let check2 = (buf[0] >> 1) & 0x01;
                let check3 = buf[1] & 0x01;
                if check1 != check2 && check3 == 1
                {
                    let start_flag = 1;
                    if start_flag && !anlik_tur.is_empty()
                    {
                        let simdiki_zaman = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as u64;
                    }
                }
                else {
                    buf.copy_within(1..5, 0 );
                    bufid = 4;
                }
            }
        }
        }
    } 
}