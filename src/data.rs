use crate::fuzz_input::FuzzInput;

pub struct FuzzData {
    /// Last index of FuzzInput vector, that has been fed (before frame update)
    pub(crate) last_index_id: Option<usize>,

    /// Fuzzer-produced array of input events to apply
    pub(crate) fuzz_inputs: Vec<FuzzInput>,
}

impl FuzzData {
    pub fn new(fuzz_inputs: Vec<FuzzInput>) -> Self {
        Self {
            last_index_id: None,
            fuzz_inputs,
        }
    }

    pub(crate) fn iter_next(&self) -> impl Iterator<Item = &FuzzInput> {
        let skip_count = self.last_index_id.map(|v| v).unwrap_or(0);
        self.fuzz_inputs.iter().skip(skip_count)
    }

    pub(crate) fn set_last_idx(&mut self, last_idx: Option<usize>) {
        if let Some(last_idx) = last_idx {
            self.last_index_id = Some(last_idx + self.last_index_id.unwrap_or(0) + 1);
        } else {
            let len = self.fuzz_inputs.len();

            if len > 0 {
                self.last_index_id = Some(self.fuzz_inputs.len() + 1);
            } else {
                self.last_index_id = Some(0);
            }
        }
    }

    pub(crate) fn is_finished(&self) -> bool {
        if let Some(last_index_id) = self.last_index_id {
            last_index_id + 1 > self.fuzz_inputs.len()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut data = FuzzData::new(vec![FuzzInput::RunFrame]);
        assert_eq!(data.is_finished(), false);
        assert_eq!(data.iter_next().count(), 1);
        data.set_last_idx(None);
        assert_eq!(data.is_finished(), true);
        assert_eq!(data.iter_next().count(), 0);

        let mut data = FuzzData::new(vec![FuzzInput::RunFrame, FuzzInput::RunFrame]);
        assert_eq!(data.is_finished(), false);
        assert_eq!(data.iter_next().count(), 2);
        assert_eq!(data.last_index_id, None);
        data.set_last_idx(Some(0));
        assert_eq!(data.last_index_id, Some(1));
        assert_eq!(data.is_finished(), false);
        assert_eq!(data.iter_next().count(), 1);
        data.set_last_idx(Some(0));
        assert_eq!(data.last_index_id, Some(2));
        assert_eq!(data.is_finished(), true);
        assert_eq!(data.iter_next().count(), 0);
    }
}
