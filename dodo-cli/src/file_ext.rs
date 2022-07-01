use crate::Result;

pub trait FileExt {
    fn is_empty(&self) -> Result<bool>;
}

impl FileExt for fs_err::File {
    fn is_empty(&self) -> Result<bool> {
        self.metadata()
            .map(|metadata| metadata.len() == 0)
            .map_err(Into::into)
    }
}
