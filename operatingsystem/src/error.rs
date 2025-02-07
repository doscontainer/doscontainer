#[derive(Debug)]
pub enum OsError {
    BpbNotApplicable,
    NotAFloppy,
    UnsupportedDiskType,
    UnsupportedOs,
}