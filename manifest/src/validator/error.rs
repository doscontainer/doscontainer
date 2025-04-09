#[derive(Debug, PartialEq)]
pub enum ValidationError {
    RequiredLayerMissing,
    PhysicalFieldsMissing,
}
