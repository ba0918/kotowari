//! 決定の`指紋`。"kotowari list" の "fingerprint" と、`ガイドの印`の照合が同じ計算を使う

use sha2::{Digest, Sha256};

/// 行を "\n" でつないだ UTF-8 のバイト列の SHA-256 を、16進の小文字で書いた先頭の8文字
pub(crate) fn digest(lines: &[String]) -> String {
    let joined = lines.join("\n");
    let digest = Sha256::digest(joined.as_bytes());
    digest.iter().take(4).map(|b| format!("{b:02x}")).collect()
}
