#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Husstandstype {
    Enlig,
    ParUdenBoern,
    ParMedBoern,
    EnligForsoerger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Beskaeftigelsesstatus {
    Fuldtid,
    Deltid,
    Ledig,
    Aktivitetsparat,
    Sygemeldt,
}
