use crate::borger_view::BorgerView;
use crate::types::{Beskaeftigelsesstatus, Husstandstype};

/// Kolonnebaseret (struct-of-arrays) lagring af borgerprofiler.
/// Hver kolonne er en Vec<T> for cache-friendly batch-access.
pub struct BorgerStore {
    pub borger_id: Vec<u32>,
    pub alder: Vec<u8>,
    pub husstandstype: Vec<Husstandstype>,
    pub bruttoindkomst: Vec<f64>,
    pub husleje: Vec<f64>,
    pub boligareal: Vec<u16>,
    pub antal_boern: Vec<u8>,
    pub boern_aldre: Vec<Vec<u8>>,
    pub beskaeftigelsesstatus: Vec<Beskaeftigelsesstatus>,
    pub kommune_id: Vec<u8>,
}

impl BorgerStore {
    pub fn len(&self) -> usize {
        self.alder.len()
    }

    pub fn is_empty(&self) -> bool {
        self.alder.is_empty()
    }

    /// Look up a borger by borger_id. Returns None if not found.
    pub fn find_by_id(&self, borger_id: u32) -> Option<usize> {
        self.borger_id.iter().position(|&id| id == borger_id)
    }

    pub fn view(&self, index: usize) -> BorgerView<'_> {
        BorgerView {
            borger_id: self.borger_id[index],
            alder: self.alder[index],
            husstandstype: self.husstandstype[index],
            bruttoindkomst: self.bruttoindkomst[index],
            husleje: self.husleje[index],
            boligareal: self.boligareal[index],
            antal_boern: self.antal_boern[index],
            boern_aldre: &self.boern_aldre[index],
            beskaeftigelsesstatus: self.beskaeftigelsesstatus[index],
            kommune_id: self.kommune_id[index],
        }
    }

    /// Approximate heap memory usage in bytes.
    pub fn heap_size_bytes(&self) -> usize {
        let fixed_cols = self.borger_id.capacity() * std::mem::size_of::<u32>()
            + self.alder.capacity() * std::mem::size_of::<u8>()
            + self.husstandstype.capacity() * std::mem::size_of::<Husstandstype>()
            + self.bruttoindkomst.capacity() * std::mem::size_of::<f64>()
            + self.husleje.capacity() * std::mem::size_of::<f64>()
            + self.boligareal.capacity() * std::mem::size_of::<u16>()
            + self.antal_boern.capacity() * std::mem::size_of::<u8>()
            + self.beskaeftigelsesstatus.capacity() * std::mem::size_of::<Beskaeftigelsesstatus>()
            + self.kommune_id.capacity() * std::mem::size_of::<u8>();

        let boern_outer = self.boern_aldre.capacity() * std::mem::size_of::<Vec<u8>>();
        let boern_inner: usize = self.boern_aldre.iter().map(|v| v.capacity()).sum();

        fixed_cols + boern_outer + boern_inner
    }
}
