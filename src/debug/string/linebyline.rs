use log::debug;

pub trait DebugOutLineByLine {
    fn debug(&self, prefix: Option<String>);
}

impl DebugOutLineByLine for String {
    fn debug(&self, prefix: Option<String>) {
        if let Some(prefix) = prefix {
            debug!("{}", prefix);
        }
        for line in self.lines() {
            debug!("{}", line);
        }
    }
}

impl DebugOutLineByLine for &str {
    fn debug(&self, prefix: Option<String>) {
        if let Some(prefix) = prefix {
            debug!("{}", prefix);
        }
        for line in self.lines() {
            debug!("{}", line);
        }
    }
}

#[cfg(test)]
#[cfg(feature = "debug-out-lbl")]
mod debug_out_lbl_tests {
    use super::*;
    use env_logger::try_init;

    #[test]
    fn debug_multiline_string_noprefix() {
        let _ = try_init();
        let s = "This string\nis split\non multiple\nlines.\n";
        s.debug(None);
    }
}
