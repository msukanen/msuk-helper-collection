#[cfg(feature = "debug-out-lbl")]
pub mod linebyline;
#[cfg(feature = "debug-out-lbl")]
pub use linebyline::DebugOutLineByLine;

#[cfg(test)]
mod string_tests {
    use super::*;
    use env_logger::try_init;

    #[cfg(feature = "debug-out-lbl")]
    #[test]
    fn linebyline_test() {
        let _ = try_init();
        let s = "Multi\n line\n  staircase\n   text of doom!";
        s.debug_noprefix();
    }
}

