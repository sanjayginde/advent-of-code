use std::ops::Range;

pub fn ranges_overlap<T: PartialOrd>(range1: &Range<T>, range2: &Range<T>) -> bool {
    !(range1.end <= range2.start || range2.end <= range1.start)
}
