mod sensorler;
mod motorlar;
mod veri_tipleri;
mod beyin;

use tokio::sync::{mpsc, watch};
use veri_tipleri::{ImuVeri};

#[tokio::main]
async fn main()
{
    let (imu_tx, mut imu_rx) = mpsc::channel::<ImuVeri>(100);
    tokio::spawn(async move {
        sensorler::bno085::imu_task(imu_tx).await;
    });
    let imu_paket = imu_rx.recv().await;
}