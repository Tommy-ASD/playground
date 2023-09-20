use super::{Core, Handle};

impl Handle {
    pub(crate) fn trace_core(&self, core: Box<Core>) -> Box<Core> {
        core
    }
}
