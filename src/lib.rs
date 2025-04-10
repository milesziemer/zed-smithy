use zed_extension_api as zed;

struct SmithyExtension {}

impl zed::Extension for SmithyExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }
}

zed::register_extension!(SmithyExtension);
