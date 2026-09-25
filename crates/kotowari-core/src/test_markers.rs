/// 印の出現ごとの (ID, 印のある行)。A152: 同じ ID の印が複数あっても出現ごとに数える
pub type MarkerIds = Vec<(String, usize)>;

/// 中身が空または閉じ括弧のない印の (印のある行, 行の文字)
pub type InvalidMarkers = Vec<(usize, String)>;

/// 印の解析結果
#[derive(Debug, Clone)]
pub struct Marker {
    pub ids: Vec<String>,
    pub line: usize,
}

/// @kotowari[...] 印を1行から抽出する
pub fn parse_markers_in_line(line: &str, line_num: usize) -> Vec<Marker> {
    let mut markers = Vec::new();
    let mut search_start = 0;

    while let Some(start) = line[search_start..].find("@kotowari[") {
        let abs_start = search_start + start;
        let content_start = abs_start + "@kotowari[".len();

        if let Some(close) = line[content_start..].find(']') {
            // "]" の位置。"]" から "@kotowari[" は始まらないので、次の探索はここから始める
            let close_pos = content_start + close;
            let content = &line[content_start..close_pos];
            let ids: Vec<String> = content
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            markers.push(Marker {
                ids,
                line: line_num,
            });
            search_start = close_pos;
        } else {
            // 閉じ括弧がない
            markers.push(Marker {
                ids: vec![],
                line: line_num,
            });
            break;
        }
    }

    markers
}
