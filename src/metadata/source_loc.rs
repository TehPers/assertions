use std::fmt::{Display, Formatter};

#[macro_export]
#[doc(hidden)]
macro_rules! source_loc {
    () => {{
        const SOURCE_LOC: $crate::metadata::SourceLoc = $crate::metadata::SourceLoc::new(
            ::core::module_path!(),
            ::core::file!(),
            ::core::line!(),
            ::core::column!(),
        );
        &SOURCE_LOC
    }};
}

/// A location in a source code file.
#[derive(Clone, Copy, Debug)]
pub struct SourceLoc {
    module_path: &'static str,
    file: &'static str,
    line: u32,
    column: u32,
}

impl SourceLoc {
    #[doc(hidden)]
    #[must_use]
    pub const fn new(
        module_path: &'static str,
        file: &'static str,
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            module_path,
            file,
            line,
            column,
        }
    }

    /// The [`module_path`] of the source code.
    #[inline]
    #[must_use]
    pub const fn module_path(&self) -> &'static str {
        self.module_path
    }

    /// The name of the source code file.
    #[inline]
    #[must_use]
    pub const fn file(&self) -> &'static str {
        self.file
    }

    /// The line within the source code file.
    #[inline]
    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// The column within the line of source code.
    #[inline]
    #[must_use]
    pub const fn column(&self) -> u32 {
        self.column
    }
}

impl Display for SourceLoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{file}:{line}:{column} [{module}]",
            file = self.file,
            line = self.line,
            column = self.column,
            module = self.module_path,
        )
    }
}
