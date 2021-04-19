use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use lcov::Report;

use lcov::report::MergeError;

use lcov::report::section::branch::Value as BranchValue;
use lcov::report::section::function::Value as FunctionValue;
use lcov::report::section::line::Value as LineValue;
use lcov::report::section::Value as SectionValue;

pub fn diff_reports(first: &Report, second: &Report) -> Result<Report, MergeError> {
    let mut rep = Report::new();
    rep.merge(first.to_owned())?;
    rep.diff(second)?;
    Ok(rep)
}

pub trait Diff {
    fn diff(&mut self, other: &Self) -> Result<(), MergeError>;
}

impl Diff for Report {
    fn diff(&mut self, other: &Self) -> Result<(), MergeError> {
        self.sections.diff(&other.sections)
    }
}

impl Diff for BranchValue {
    fn diff(&mut self, other: &Self) -> Result<(), MergeError> {
        if let BranchValue { taken: Some(taken) } = *other {
            // We don't care about exact count. It's only important is the branch covered or not
            if taken > 0 {
                self.taken = None;
            }
        };
        Ok(())
    }
}

impl Diff for SectionValue {
    fn diff(&mut self, other: &Self) -> Result<(), MergeError> {
        self.functions.diff(&other.functions)?;
        self.branches.diff(&other.branches)?;
        self.lines.diff(&other.lines)?;
        Ok(())
    }
}

impl Diff for FunctionValue {
    fn diff(&mut self, other: &Self) -> Result<(), MergeError> {
        if let Some(start_line) = other.start_line.as_ref() {
            if let Some(my_start_line) = self.start_line.as_ref() {
                if start_line != my_start_line {
                    return Err(MergeError::UnmatchedFunctionLine);
                }
            }
        }
        // As for branch it's only important if it covered or not
        if other.count > 0 {
            self.count = 0;
        }
        Ok(())
    }
}

impl Diff for LineValue {
    fn diff(&mut self, other: &Self) -> Result<(), MergeError> {
        if let Some(checksum) = other.checksum.as_ref() {
            if let Some(my_checksum) = self.checksum.as_ref() {
                if checksum != my_checksum {
                    return Err(MergeError::UnmatchedChecksum);
                }
            }
        }
        // As for branch it's only important if it covered or not
        if other.count > 0 {
            self.count = 0;
        }
        Ok(())
    }
}

impl<K, V> Diff for BTreeMap<K, V>
where
    K: Ord + Clone,
    V: Diff,
{
    fn diff(&mut self, other: &Self) -> Result<(), MergeError> {
        for (key, value) in other {
            match self.entry(key.clone()) {
                Entry::Vacant(_) => {}
                Entry::Occupied(mut e) => e.get_mut().diff(value)?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::diff_reports;
    use lcov::report::MergeError;
    use lcov::{Reader, Report};

    #[test]
    fn diff_report() -> Result<(), MergeError> {
        let input = "\
TN:
SF:target.c
FN:1,main
FNDA:1,main
DA:1,1
DA:3,1
DA:4,1
DA:5,1
DA:6,1
DA:7,1
DA:8,0
DA:11,1
DA:12,0
DA:14,1
DA:15,1
DA:17,1
end_of_record
";
        let reader1 = Reader::new(input.as_bytes());
        let report1 = Report::from_reader(reader1).unwrap();

        let input2 = "\
TN:
SF:target.c
FN:1,main
FNDA:1,main
DA:1,1
DA:3,1
DA:4,1
DA:5,1
DA:6,1
DA:7,1
DA:8,1
DA:11,1
DA:12,0
DA:14,1
DA:15,1
DA:17,1
end_of_record
";

        let expected_lcov = "\
TN:
SF:target.c
FN:1,main
FNDA:0,main
FNF:1
FNH:0
DA:1,0
DA:3,0
DA:4,0
DA:5,0
DA:6,0
DA:7,0
DA:8,1
DA:11,0
DA:12,0
DA:14,0
DA:15,0
DA:17,0
LF:12
LH:1
end_of_record
";
        let reader2 = Reader::new(input2.as_bytes());
        let report2 = Report::from_reader(reader2).unwrap();

        let expected_report = Report::from_reader(Reader::new(expected_lcov.as_bytes())).unwrap();

        let diff_rep = diff_reports(&report2, &report1).unwrap();

        for pair in diff_rep.into_records().zip(expected_report.into_records()) {
            assert_eq!(pair.0, pair.1)
        }
        Ok(())
    }
}
