use crate::rule_engine::RuleId;
use crate::types::Husstandstype;

/// Compact per-borger result: 3 rules × (f64 amount + bool eligible).
/// Layout verified by size_of test — no heap allocation, no strings.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CompactBorgerResult {
    pub amounts: [f64; 3],
    pub eligible: [bool; 3],
}

impl CompactBorgerResult {
    pub fn amount(&self, rule_id: RuleId) -> f64 {
        self.amounts[rule_index(rule_id)]
    }

    pub fn is_eligible(&self, rule_id: RuleId) -> bool {
        self.eligible[rule_index(rule_id)]
    }

    pub fn total_amount(&self) -> f64 {
        self.amounts.iter().sum()
    }
}

fn rule_index(rule_id: RuleId) -> usize {
    match rule_id {
        RuleId::Kontanthjaelp => 0,
        RuleId::Boligstoette => 1,
        RuleId::BoerneYdelse => 2,
    }
}

#[derive(Debug, Clone)]
pub struct RuleAggregation {
    pub rule_id: RuleId,
    pub total_amount: f64,
    pub eligible_count: usize,
    pub mean_amount: f64,
}

#[derive(Debug)]
pub struct BatchResult {
    pub per_rule: Vec<RuleAggregation>,
    pub borger_results: Vec<CompactBorgerResult>,
    pub count: usize,
}

impl BatchResult {
    pub fn heap_size_bytes(&self) -> usize {
        self.borger_results.capacity() * std::mem::size_of::<CompactBorgerResult>()
            + self.per_rule.capacity() * std::mem::size_of::<RuleAggregation>()
    }
}

/// Segment key for group-by aggregation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentKey {
    Kommune(u8),
    Husstandstype(Husstandstype),
}

#[derive(Debug, Clone)]
pub struct SegmentAggregation {
    pub key: SegmentKey,
    pub rule_id: RuleId,
    pub total_amount: f64,
    pub eligible_count: usize,
    pub borger_count: usize,
}

/// A top-N entry: borger index + total amount for ranking.
#[derive(Debug, Clone)]
pub struct TopNEntry {
    pub borger_index: usize,
    pub borger_id: u32,
    pub rule_id: RuleId,
    pub amount: f64,
}
