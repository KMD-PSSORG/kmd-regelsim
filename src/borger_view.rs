use crate::types::{Beskaeftigelsesstatus, Husstandstype};

/// Zero-copy view over a single borger's data.
/// References point directly into BorgerStore columns.
pub struct BorgerView<'a> {
    pub borger_id: u32,
    pub alder: u8,
    pub husstandstype: Husstandstype,
    pub bruttoindkomst: f64,
    pub husleje: f64,
    pub boligareal: u16,
    pub antal_boern: u8,
    pub boern_aldre: &'a [u8],
    pub beskaeftigelsesstatus: Beskaeftigelsesstatus,
    pub kommune_id: u8,
}
