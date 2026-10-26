use crate::{PayloadAccelerometer, PayloadTachometer};

///
/// Служебный трэйт, чтобы функция знала, что у типа можно взять id
pub trait Identifiable {
    fn get_id(&self) -> &str;
}
//
impl Identifiable for PayloadAccelerometer {
    fn get_id(&self) -> &str {
        &self.id
    }
}
//
impl Identifiable for PayloadTachometer {
    fn get_id(&self) -> &str {
        &self.id
    }
}