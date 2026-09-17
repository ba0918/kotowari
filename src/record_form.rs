//! 判断の記録の形の検査（REQ-129〜REQ-135、TBL-022）。
//! 読み取りは `sources::parse_records_file` が済ませてあり、ここはその構造だけを読む（REQ-136）。

use crate::sources::{required_field_of, SourceContext};
use crate::{Finding, FindingKind};

/// 読んだ判断の記録の形を検査する
pub fn check_record_forms(ctx: &SourceContext, findings: &mut Vec<Finding>) {
    for record in &ctx.records_files {
        // REQ-129: "## Context" の見出しを持つ記録だけが補足の行の検査を受ける
        if !record.has_context {
            continue;
        }
        let path = crate::join_display_path(&ctx.records_path, &record.rel_path);
        for section in &record.sections {
            let Some(required) = required_field_of(&section.name) else {
                continue;
            };
            for numbered in &section.numbered_lines {
                // REQ-133: 値が空の補足の行は無いものとして数える
                let has_required = numbered
                    .fields
                    .iter()
                    .any(|field| field.name == required && !field.value.is_empty());
                if !has_required {
                    findings.push(Finding::new(
                        FindingKind::RecordFieldMissing,
                        path.clone(),
                        Some(numbered.line),
                        required.to_string(),
                    ));
                }
            }
        }
    }
}
