// TODO: Implement stubs
// https://github.com/heim-rs/heim/issues/12

use std::path::Path;

use heim_common::prelude::*;

use crate::FileSystem;

pub struct Partition;

impl Partition {
    pub fn device(&self) -> Option<&str> {
        unimplemented!()
    }

    pub fn mount_point(&self) -> &Path {
        unimplemented!()
    }

    pub fn file_system(&self) -> &FileSystem {
        unimplemented!()
    }

    pub fn options(&self) -> &str {
        unimplemented!()
    }
}

pub fn partitions() -> impl Stream<Item = Partition, Error = Error> {
    stream::iter_ok(vec![])
}

pub fn partitions_physical() -> impl Stream<Item = Partition, Error = Error> {
    partitions()
}
