use crate::harness::Harness;

pub fn run(h: &mut Harness) {
    h.begin(16).run_part(1, |_text| 0).run_part(2, |_text| 0);
}
