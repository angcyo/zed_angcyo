use std::io::{Write, stdout};
use zed_extension_api as zed;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2026/05/06
///
/// # Zed 插件开发
/// https://zed.dev/docs/extensions/developing-extensions
///
#[derive(Default)]
struct ZenAngcyoExtension {
    // ... state
}

impl zed::Extension for ZenAngcyoExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        stdout().write(b"ZenAngcyoExtension::new()\n").ok();
        ZenAngcyoExtension {
            ..Default::default()
        }
    }
}

zed::register_extension!(ZenAngcyoExtension);
