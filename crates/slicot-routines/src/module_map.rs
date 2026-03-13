//! Maps SLICOT routine stems to the target Rust module families.

/// Target Rust module families that mirror the upstream SLICOT chapter layout.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TargetRustModule {
    Analysis,
    Benchmark,
    DataAnalysis,
    Filtering,
    Identification,
    Mathematics,
    Nonlinear,
    Synthesis,
    Transformation,
    Utility,
}

impl TargetRustModule {
    /// Returns a stable `snake_case` identifier for generated inventory output.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Analysis => "analysis",
            Self::Benchmark => "benchmark",
            Self::DataAnalysis => "data_analysis",
            Self::Filtering => "filtering",
            Self::Identification => "identification",
            Self::Mathematics => "mathematics",
            Self::Nonlinear => "nonlinear",
            Self::Synthesis => "synthesis",
            Self::Transformation => "transformation",
            Self::Utility => "utility",
        }
    }
}

/// Maps an uppercase SLICOT routine stem such as `TB04AD` to the target Rust
/// module family derived from the upstream chapter code.
#[must_use]
pub fn target_rust_module_for_stem(stem: &str) -> Option<TargetRustModule> {
    match stem.chars().next() {
        Some('A') => Some(TargetRustModule::Analysis),
        Some('B') => Some(TargetRustModule::Benchmark),
        Some('D') => Some(TargetRustModule::DataAnalysis),
        Some('F') => Some(TargetRustModule::Filtering),
        Some('I') => Some(TargetRustModule::Identification),
        Some('M') => Some(TargetRustModule::Mathematics),
        Some('N') => Some(TargetRustModule::Nonlinear),
        Some('S') => Some(TargetRustModule::Synthesis),
        Some('T') => Some(TargetRustModule::Transformation),
        Some('U') => Some(TargetRustModule::Utility),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{target_rust_module_for_stem, TargetRustModule};

    #[test]
    fn maps_known_chapter_codes() {
        assert_eq!(
            target_rust_module_for_stem("AB09AD"),
            Some(TargetRustModule::Analysis)
        );
        assert_eq!(
            target_rust_module_for_stem("SB10AD"),
            Some(TargetRustModule::Synthesis)
        );
        assert_eq!(
            target_rust_module_for_stem("TB04AD"),
            Some(TargetRustModule::Transformation)
        );
        assert_eq!(
            target_rust_module_for_stem("MB03RD"),
            Some(TargetRustModule::Mathematics)
        );
    }

    #[test]
    fn rejects_unknown_prefixes() {
        assert_eq!(target_rust_module_for_stem("ZB99ZZ"), None);
    }
}
