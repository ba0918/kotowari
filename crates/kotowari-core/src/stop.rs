//! 停止の理由（TBL-core-018）

/// 停止の理由と、標準エラーの1行目で詳細の前に出る文言（TBL-core-018）。
/// この呼び出しの一覧が変種の唯一の在り処で、列挙体・`WORDINGS`・`Display` はここから作る。
macro_rules! stop_reasons {
    ($($variant:ident => $wording:literal,)+) => {
        /// 停止の理由
        #[derive(Debug)]
        pub enum StopReason {
            $($variant(String)),+
        }

        impl StopReason {
            /// 標準エラーの1行目に出しうる文言のすべて
            pub const WORDINGS: &'static [&'static str] = &[$($wording),+];

            /// この理由の文言と詳細
            fn wording_and_detail(&self) -> (&'static str, &str) {
                match self {
                    $(StopReason::$variant(detail) => ($wording, detail)),+
                }
            }
        }

        impl std::fmt::Display for StopReason {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let (wording, detail) = self.wording_and_detail();
                write!(f, "{wording}: {detail}")
            }
        }
    };
}

stop_reasons! {
    ArgumentError => "argument error",
    ConfigError => "config error",
    UnreadableFile => "unreadable file",
    NonUtf8File => "non-UTF-8 file",
    ResultsError => "results error",
    MappingError => "mapping error",
}

impl std::error::Error for StopReason {}
