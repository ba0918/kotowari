//! 判断の記録の形の検査（REQ-core-129〜REQ-core-135、TBL-core-022）と superseded_by のリンクの検査（REQ-core-132、TBL-core-023）。
//! 読み取りは `sources::parse_records_file` が済ませてあり、ここはその構造だけを読む（REQ-core-136）。

use crate::sources::{
    is_decision_number, is_under_place, required_field_of, RecordsFile, SourceContext,
    KNOWN_FIELD_NAMES,
};
use crate::{Finding, FindingKind};

/// 読んだ判断の記録の形を検査する
pub fn check_record_forms(ctx: &SourceContext, findings: &mut Vec<Finding>) {
    for record in &ctx.records_files {
        let path = crate::join_display_path(&ctx.records_path, &record.rel_path);
        // REQ-core-129: 補足の行の有無と名前の検査は "## Context" を持つ記録だけが受ける
        if record.has_context {
            check_fields(record, &path, findings);
        }
        // REQ-core-129: リンクの検査は "## Context" の有無を見ず、すべての判断の記録が受ける
        check_revision_links(ctx, record, &path, findings);
    }
}

/// TBL-core-022: 必須の補足の行の欠けと、知らない名前
fn check_fields(record: &RecordsFile, path: &str, findings: &mut Vec<Finding>) {
    for section in &record.sections {
        let Some(required) = required_field_of(&section.name) else {
            continue;
        };
        for numbered in &section.numbered_lines {
            // REQ-core-133: 値が空の補足の行は無いものとして数える
            let has_required = numbered
                .fields
                .iter()
                .any(|field| field.name == required && !field.value.is_empty());
            if !has_required {
                findings.push(Finding::new(
                    FindingKind::RecordFieldMissing,
                    path.to_string(),
                    Some(numbered.line),
                    required.to_string(),
                ));
            }

            // REQ-core-131: 6つ以外の名前はその行の誤り（値が空でも受ける）
            for field in &numbered.fields {
                if !KNOWN_FIELD_NAMES.contains(&field.name.as_str()) {
                    findings.push(Finding::new(
                        FindingKind::RecordFieldUnknown,
                        path.to_string(),
                        Some(field.line),
                        field.name.clone(),
                    ));
                }
            }
        }
    }
}

/// TBL-core-023: superseded_by の行のリンクを1つずつ判定する
fn check_revision_links(
    ctx: &SourceContext,
    record: &RecordsFile,
    path: &str,
    findings: &mut Vec<Finding>,
) {
    for section in &record.sections {
        for numbered in &section.numbered_lines {
            for field in &numbered.fields {
                // REQ-core-133: 値が空の superseded_by は判定の対象にしない
                if field.name != "superseded_by" || field.value.is_empty() {
                    continue;
                }
                let mut invalid = |detail: String| {
                    findings.push(Finding::new(
                        FindingKind::RevisionLinkInvalid,
                        path.to_string(),
                        Some(field.line),
                        detail,
                    ));
                };
                // 順1: 値にリンクが1つも無い。detail は行の値
                if field.links.is_empty() {
                    invalid(field.value.clone());
                    continue;
                }
                // 順2〜6: リンクごとに判定し、無効なら出現ごとに1件
                for link in &field.links {
                    if !is_valid_link(ctx, record, &link.href) {
                        invalid(link.href.clone());
                    }
                }
            }
        }
    }
}

/// TBL-core-023 の順2から順6
fn is_valid_link(ctx: &SourceContext, record: &RecordsFile, href: &str) -> bool {
    // 順2: "#" が無い、または "#" の後が決定の番号の形でない
    let Some(hash) = href.find('#') else {
        return false;
    };
    let before = &href[..hash];
    let number = &href[hash + 1..];
    if !is_decision_number(number) {
        return false;
    }

    // 順3・順4: 先のファイルを決める
    let target = if before.is_empty() {
        // "#" より前が空なら同じ記録
        record
    } else {
        // 順3: "/" か "\" で始まる href は、つなぐ前に置き場の外と決まる
        if before.starts_with('/') || before.starts_with('\\') {
            return false;
        }
        let record_path = crate::join_display_path(&ctx.records_path, &record.rel_path);
        let dir = match record_path.rfind('/') {
            Some(pos) => &record_path[..pos],
            None => "",
        };
        let joined = crate::normalize_path(&crate::join_display_path(dir, before));
        let Some(resolved) = resolve_parent_dirs(&joined) else {
            return false;
        };
        if !is_under_place(&resolved, &ctx.records_path) {
            return false;
        }
        // 順4: 先が読んだ判断の記録でない
        let found = ctx.records_files.iter().find(|candidate| {
            crate::join_display_path(&ctx.records_path, &candidate.rel_path) == resolved
        });
        let Some(found) = found else {
            return false;
        };
        found
    };

    // 順5: 先の決定の節と Superseded の節のどこにもその番号の行が無い
    target.has_revision_target(number)
}

/// 正規化した後のパスの ".." を左から順に直前の要素を消して解く（A42）。
/// 消す要素が無ければ置き場の外なので None を返す。
fn resolve_parent_dirs(path: &str) -> Option<String> {
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        if part == ".." {
            parts.pop()?;
        } else {
            parts.push(part);
        }
    }
    Some(parts.join("/"))
}
