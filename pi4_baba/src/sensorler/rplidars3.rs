use crate::veri_tipleri::{LidarNokta, LidarVeri};
use tokio::sync::mpsc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialPortBuilderExt;

const LIDAR_PORT: &str = "amanin";
const LIDAR_BAUD: u32 = 115200;

pub async fn lidar_task(tx: mpsc::Sender<LidarVeri>)
{
    let mut port = tokio_serial::new(LIDAR_PORT, LIDAR_BAUD).open_native_async().expect("Lidar baslamadi!");


}