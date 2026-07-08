
use crate::veri_tipleri::{LidarNokta, LidarVeri};
use tokio::sync::mpsc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialPortBuilderExt;

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
    let mut buf: [u8; 5] = [0u8; 5];
    let mut bufid = 0;
    let mut anlik_tur: Vec<LidarNokta> = Vec::with_capacity(1500);
    loop {
        
    } 
}