use std::fmt::Display;

macro_rules! maybe_styled {
    ($fn:ident, [$($style:ident),*]) => {
        #[cfg(feature = "colors")]
        #[inline]
        #[allow(dead_code)]
        pub fn $fn(s: &impl Display) -> impl Display + '_ {
            use owo_colors::{OwoColorize, Stream, Style};
            let style = Style::new() $(.$style())*;
            s.if_supports_color(Stream::Stderr, move |s| s.style(style))
        }

        #[cfg(not(feature = "colors"))]
        #[inline]
        #[allow(dead_code)]
        pub fn $fn(s: &impl Display) -> impl Display + '_ {
            s
        }
    };
}

// Error messages
maybe_styled!(dimmed, [dimmed]);
maybe_styled!(error, [bright_red]);
maybe_styled!(reference, [yellow]);

// Diffs
maybe_styled!(added, [green]);
maybe_styled!(removed, [red]);
maybe_styled!(emphasize_added, [green, bold, underline]);
maybe_styled!(emphasize_removed, [red, bold, underline]);
