//! 2つのページの描き比べ

/// 2つのページが食い違う範囲の、left の中のバイトの位置。同じなら None
pub fn changed(left: &str, right: &str) -> Option<(usize, usize)> {
    let (a, b) = (left.as_bytes(), right.as_bytes());
    let prefix = a.iter().zip(b).take_while(|(x, y)| x == y).count();
    if prefix == a.len() && prefix == b.len() {
        return None;
    }
    let suffix = a[prefix..]
        .iter()
        .rev()
        .zip(b[prefix..].iter().rev())
        .take_while(|(x, y)| x == y)
        .count();
    Some((prefix, a.len() - suffix))
}
