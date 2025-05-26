use disk::Disk;
use operatingsystem::OperatingSystem;

pub struct Builder{}

impl Builder {
    pub fn build_disk<T: Disk>(disk: &mut T, os: &OperatingSystem) {
    }
}