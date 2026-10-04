use kotowari::{
    ChangesReport, CheckReport, Finding, ListItem, MutantsReport, PlanReport, QueryReport,
    ReadList, StatusReport,
};
use serde_json::{Value, json};

pub fn finding(value: &Finding) -> Value {
    json!({"kind":value.kind().as_str(),"severity":value.severity(),"path":value.path(),"line":value.line(),"detail":value.detail()})
}
fn findings(values: &[Finding]) -> Vec<Value> {
    values.iter().map(finding).collect()
}
fn tests(values: &[kotowari::TestRef]) -> Vec<Value> {
    values
        .iter()
        .map(|value| json!({"path":value.path(),"line":value.line(),"name":value.name()}))
        .collect()
}
fn item(value: &ListItem) -> Value {
    let variant = match value {
        ListItem::Requirement(i) => {
            json!({"name":i.name(),"type":i.type_(),"verification":i.verification(),"definition":i.definition(),"examples":i.examples(),"how_to_verify":i.how_to_verify(),"sources":i.sources(),"tests":tests(i.tests()),"fingerprint":i.fingerprint(),"deferred":i.deferred()})
        }
        ListItem::WithExamples(i) => {
            json!({"name":i.name(),"examples":i.examples(),"sources":i.sources(),"tests":tests(i.tests()),"fingerprint":i.fingerprint(),"deferred":i.deferred()})
        }
        ListItem::Scenario(i) => {
            json!({"name":i.name(),"sources":i.sources(),"tests":tests(i.tests()),"fingerprint":i.fingerprint(),"deferred":i.deferred()})
        }
        ListItem::Flag(i) => {
            json!({"name":i.name(),"type":i.type_(),"relations":i.relations(),"sources":i.sources(),"tests":tests(i.tests()),"fingerprint":i.fingerprint(),"deferred":i.deferred()})
        }
    };
    let (path, line) = value.location();
    let mut result =
        json!({"id":value.id(),"kind":value.kind(),"name":variant["name"],"path":path,"line":line});
    let object = result.as_object_mut().unwrap();
    object.extend(
        variant
            .as_object()
            .unwrap()
            .iter()
            .filter(|(key, _)| key.as_str() != "name")
            .map(|(key, value)| (key.clone(), value.clone())),
    );
    result
}
pub fn list(value: &ReadList) -> Value {
    json!({"items":value.items().iter().map(item).collect::<Vec<_>>()})
}
pub fn query(value: &QueryReport) -> Value {
    let items = value
        .items()
        .iter()
        .map(|i| {
            let mut value = item(i.item());
            value["body"] = json!(i.body());
            value["referenced_by"] = json!(
                i.referenced_by()
                    .iter()
                    .map(
                        |r| json!({"id":r.id(),"kind":r.kind(),"path":r.path(),"line":r.line(),"via":r.via()})
                    )
                    .collect::<Vec<_>>()
            );
            value
        })
        .collect::<Vec<_>>();
    json!({"items":items})
}
fn file_tallies(values: &std::collections::BTreeMap<String, kotowari::TestFileTally>) -> Value {
    Value::Object(
        values
            .iter()
            .map(|(name, value)| {
                (
                    name.clone(),
                    json!({"files":value.files(),"query":value.query()}),
                )
            })
            .collect(),
    )
}
/// REQ-core-288、REQ-core-289: "overview" の群。群が無ければ両方 0
fn overview(group: Option<&kotowari::GroupTally>) -> Value {
    json!({"files":group.map_or(0, |group| group.files()),"marks":group.map_or(0, |group| group.marks())})
}
pub fn check(value: &CheckReport) -> Value {
    let mut result = json!({"files":value.files(),"lines":value.lines(),"findings":findings(value.findings()),"counts":value.counts(),"tests":file_tallies(value.tests()),"guides":{"files":value.guides().files(),"marks":value.guides().marks()},"overview":overview(value.group(kotowari::OVERVIEW_GROUP))});
    if let Some(surface) = value.surface() {
        result["surface"] = json!({"unspecified":surface.unspecified()});
    }
    result
}
pub fn status(value: &StatusReport) -> Value {
    let d = value.documents();
    let i = value.items();
    let r = value.requirements();
    let s = value.scenarios();
    let t = value.tests();
    let g = value.guides();
    let u = value.surface();
    let f = value.findings();
    json!({"documents":{"files":d.files(),"lines":d.lines()},"items":{"requirement":i.requirement(),"table":i.table(),"property":i.property(),"scenario":i.scenario(),"flag":i.flag()},"requirements":{"unit":r.unit(),"property":r.property(),"proof":r.proof(),"review":r.review(),"with_tests":r.with_tests(),"without_tests":r.without_tests(),"review_with_how_to_verify":r.review_with_how_to_verify(),"review_without_how_to_verify":r.review_without_how_to_verify(),"without_examples":r.without_examples(),"deferred":r.deferred()},"scenarios":{"with_tests":s.with_tests(),"without_tests":s.without_tests(),"deferred":s.deferred()},"tests":{"marks":t.marks(),"files":file_tallies(t.files())},"guides":{"files":g.files(),"marks":g.marks()},"overview":overview(value.group(kotowari::OVERVIEW_GROUP)),"surface":{"total":u.total(),"specified":u.specified(),"unspecified":u.unspecified()},"findings":{"error":f.error(),"notice":f.notice()},"complete":value.complete()})
}
/// REQ-core-295: "written"、"removed"、"unchanged" の3つの鍵だけ
pub fn overview_build(value: &kotowari::OverviewBuild) -> Value {
    json!({"written":value.written(),"removed":value.removed(),"unchanged":value.unchanged()})
}
pub fn plan(value: &PlanReport) -> Value {
    json!({"findings":findings(value.findings()),"counts":value.counts()})
}
pub fn mutants(value: &MutantsReport) -> Value {
    let m = value.mutants();
    json!({"findings":findings(value.findings()),"counts":value.counts(),"mutants":{"caught":m.caught(),"survived":m.survived(),"timeout":m.timeout(),"unviable":m.unviable(),"equivalent":m.equivalent()}})
}
pub fn changes(value: &ChangesReport) -> Value {
    json!({"base":value.base(),"target":value.target(),"phase":value.phase(),"files":value.files(),"covered":value.covered(),"findings":findings(value.findings())})
}

#[cfg(test)]
mod tests {
    // @kotowari[REQ-core-249, REQ-core-254, EX-core-439]
    #[test]
    fn status_json_has_no_change_record_tally() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".kotowari")).unwrap();
        std::fs::create_dir_all(dir.path().join("docs/ir")).unwrap();
        std::fs::create_dir_all(dir.path().join("docs/changes")).unwrap();
        std::fs::create_dir_all(dir.path().join("docs/decision/records")).unwrap();
        std::fs::create_dir_all(dir.path().join("docs/decision/adr")).unwrap();
        std::fs::write(
            dir.path().join(".kotowari/config.yaml"),
            "changes:\n  files: ['src/**']\n  records: ['docs/changes/**']\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("docs/changes/test.yaml"),
            "version: 2\nentries: []\n",
        )
        .unwrap();
        let result = kotowari::Project::new(kotowari::ProjectOptions::new(dir.path()))
            .unwrap()
            .status()
            .unwrap();
        assert!(result.findings().error() > 0);
        let value = super::status(&result);
        assert!(value.get("changes").is_none());
    }
}
