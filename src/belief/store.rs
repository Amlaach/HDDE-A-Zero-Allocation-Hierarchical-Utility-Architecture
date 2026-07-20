use super::record::BeliefRecord;

pub const MAX_BELIEFS_PER_ENTITY: usize = 64;

#[derive(Clone, Debug)]
pub struct BeliefStore {
    records: [Option<BeliefRecord>; MAX_BELIEFS_PER_ENTITY],
}

impl Default for BeliefStore {
    fn default() -> Self {
        const INIT: Option<BeliefRecord> = None;
        Self {
            records: [INIT; MAX_BELIEFS_PER_ENTITY],
        }
    }
}

impl BeliefStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_or_update(&mut self, new_record: BeliefRecord) {
        let mut lowest_confidence_idx = 0;
        let mut lowest_confidence = f32::MAX;

        for i in 0..MAX_BELIEFS_PER_ENTITY {
            if let Some(existing) = &mut self.records[i] {
                if existing.subject_id == new_record.subject_id
                    && core::mem::discriminant(&existing.kind)
                        == core::mem::discriminant(&new_record.kind)
                {
                    if new_record.observed_at >= existing.observed_at {
                        *existing = new_record;
                    }
                    return;
                }
                if existing.confidence < lowest_confidence {
                    lowest_confidence = existing.confidence;
                    lowest_confidence_idx = i;
                }
            } else {
                self.records[i] = Some(new_record);
                return;
            }
        }

        if new_record.confidence > lowest_confidence {
            self.records[lowest_confidence_idx] = Some(new_record);
        }
    }

    pub fn remove_stale(&mut self, threshold: f32) {
        for i in 0..MAX_BELIEFS_PER_ENTITY {
            if let Some(record) = &self.records[i] {
                if record.confidence < threshold {
                    self.records[i] = None;
                }
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &BeliefRecord> {
        self.records.iter().filter_map(|r| r.as_ref())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut BeliefRecord> {
        self.records.iter_mut().filter_map(|r| r.as_mut())
    }
}
