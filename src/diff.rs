#[cfg(not(feature = "diff"))]
pub fn fmt_diff(_expected: &str, _actual: &str) -> Option<String> {
    None
}

#[cfg(feature = "diff")]
pub fn fmt_diff(expected: &str, actual: &str) -> Option<String> {
    use diff::Result;

    let lines = diff::lines(expected, actual);
    let mut output = String::with_capacity(expected.len().max(actual.len()));
    let mut state = diff_utils::LineDiffState::NoDiff;
    let mut different = false; // make sure there is actually a change

    for line in lines {
        different = different || matches!(line, Result::Left(_) | Result::Right(_));
        state = state.step(&mut output, line);
    }

    if different {
        Some(output)
    } else {
        None
    }
}

#[cfg(feature = "diff")]
mod diff_utils {
    use std::fmt::Write;

    use diff::Result;

    use crate::styles;

    #[derive(Debug, Default)]
    pub enum LineDiffState<'a> {
        #[default]
        NoDiff,
        Removing(Vec<&'a str>),
        Adding(Vec<&'a str>),
        Removed {
            removed: Vec<&'a str>,
            adding: Vec<&'a str>,
        },
        Added {
            added: Vec<&'a str>,
            removing: Vec<&'a str>,
        },
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum ChangeType {
        NoChange,
        Removed,
        Added,
    }

    fn flush_buffer(
        removed: &mut String,
        added: &mut String,
        buffer: &mut String,
        buffer_type: ChangeType,
    ) {
        if buffer.is_empty() {
            return;
        }

        match buffer_type {
            ChangeType::NoChange => {
                write!(removed, "{}", styles::removed(&buffer)).unwrap();
                write!(added, "{}", styles::added(&buffer)).unwrap();
            }
            ChangeType::Removed => {
                write!(removed, "{}", styles::emphasize_removed(&buffer)).unwrap();
            }
            ChangeType::Added => {
                write!(added, "{}", styles::emphasize_added(&buffer)).unwrap();
            }
        }

        buffer.clear();
    }

    fn diff_line(output: &mut String, removed: &str, added: &str) {
        // Get removed/added representations
        let diff = diff::chars(removed, added);
        let mut removed_repr = String::with_capacity(removed.len());
        let mut added_repr = String::with_capacity(added.len());
        let mut buffer = String::new();
        let mut buffer_type = ChangeType::NoChange;
        for c in diff {
            match c {
                Result::Both(c, _) => {
                    if buffer_type != ChangeType::NoChange {
                        flush_buffer(&mut removed_repr, &mut added_repr, &mut buffer, buffer_type);
                        buffer_type = ChangeType::NoChange;
                    }

                    buffer.push(c);
                }
                Result::Left(c) => {
                    if buffer_type != ChangeType::Removed {
                        flush_buffer(&mut removed_repr, &mut added_repr, &mut buffer, buffer_type);
                        buffer_type = ChangeType::Removed;
                    }

                    buffer.push(c);
                }
                Result::Right(c) => {
                    if buffer_type != ChangeType::Added {
                        flush_buffer(&mut removed_repr, &mut added_repr, &mut buffer, buffer_type);
                        buffer_type = ChangeType::Added;
                    }

                    buffer.push(c);
                }
            }
        }

        flush_buffer(&mut removed_repr, &mut added_repr, &mut buffer, buffer_type);
        writeln!(output, "{} {removed_repr}", styles::removed(&"-")).unwrap();
        writeln!(output, "{} {added_repr}", styles::added(&"+")).unwrap();
    }

    impl<'a> LineDiffState<'a> {
        fn flush(self, output: &mut String) {
            match self {
                LineDiffState::NoDiff => {}
                LineDiffState::Removing(removed) => {
                    for line in removed {
                        writeln!(output, "{}", styles::removed(&format_args!("- {line}"))).unwrap();
                    }
                }
                LineDiffState::Adding(added) => {
                    for line in added {
                        writeln!(output, "{}", styles::added(&format_args!("+ {line}"))).unwrap();
                    }
                }
                LineDiffState::Removed {
                    removed,
                    adding: added,
                }
                | LineDiffState::Added {
                    added,
                    removing: removed,
                } => {
                    // Interleave as much as possible
                    let mut removed = removed.into_iter();
                    let mut added = added.into_iter();
                    loop {
                        match (removed.next(), added.next()) {
                            (None, None) => break,
                            (None, Some(line)) => {
                                writeln!(output, "{}", styles::added(&format_args!("+ {line}")))
                                    .unwrap();
                            }
                            (Some(line), None) => {
                                writeln!(output, "{}", styles::removed(&format_args!("- {line}")))
                                    .unwrap();
                            }
                            (Some(removed), Some(added)) => {
                                diff_line(output, removed, added);
                            }
                        }
                    }
                }
            }
        }

        pub fn step(self, output: &mut String, result: Result<&'a str>) -> Self {
            match (self, result) {
                // NoDiff
                (LineDiffState::NoDiff, Result::Left(line)) => LineDiffState::Removing(vec![line]),
                (LineDiffState::NoDiff, Result::Right(line)) => LineDiffState::Adding(vec![line]),
                (LineDiffState::NoDiff, Result::Both(line, _)) => {
                    writeln!(output, "  {line}").unwrap();
                    LineDiffState::NoDiff
                }

                // Removing
                (LineDiffState::Removing(mut removed), Result::Left(line)) => {
                    removed.push(line);
                    LineDiffState::Removing(removed)
                }
                (LineDiffState::Removing(removed), Result::Right(line)) => LineDiffState::Removed {
                    removed,
                    adding: vec![line],
                },

                // Adding
                (LineDiffState::Adding(added), Result::Left(line)) => LineDiffState::Added {
                    added,
                    removing: vec![line],
                },
                (LineDiffState::Adding(mut added), Result::Right(line)) => {
                    added.push(line);
                    LineDiffState::Adding(added)
                }

                // Removed
                (
                    LineDiffState::Removed {
                        removed,
                        mut adding,
                    },
                    Result::Right(line),
                ) => {
                    adding.push(line);
                    LineDiffState::Removed { removed, adding }
                }

                // Added
                (
                    LineDiffState::Added {
                        added,
                        mut removing,
                    },
                    Result::Left(line),
                ) => {
                    removing.push(line);
                    LineDiffState::Added { added, removing }
                }

                // Flush
                (state, result) => {
                    state.flush(output);
                    LineDiffState::NoDiff.step(output, result)
                }
            }
        }
    }
}
