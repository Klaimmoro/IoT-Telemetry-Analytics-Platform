use serde::{Serialize, Deserialize};
///
/// Структура данных тахометра
#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadTachometer {
    pub id: String, // ID датчика
    pub work_time: f64, // время работы в секундах
    pub rpm: f64, // скорость вращения
}