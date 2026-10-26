use serde::{Serialize, Deserialize};
///
/// Структура данных акселерометра
#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadAccelerometer {
    pub id: String, // ID датчика
    pub work_time: f64, // время работы в секундах
    pub buf: Vec<f64>, // буфер с амплитудами вибраций
}