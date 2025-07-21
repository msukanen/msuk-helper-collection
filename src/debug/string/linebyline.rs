use log::debug;

pub trait DebugOutLineByLine {
    fn debug(&self, prefix: Option<String>);
}

#[cfg(feature = "debug-out-lbl")]
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

#[cfg(test)]
mod debug_out_lbl_tests {
    use super::*;

    #[test]
    fn debug_multiline_string_noprefix() {
        let s = "This string\nis split\non multiple\nlines.\n";
        s.debug(None);
    }
}
