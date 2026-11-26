use kotowari::{
    CheckReport, Finding, ListItem, MutantsReport, PlanReport, QueryReport, ReadList, StatusReport,
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
        ListItem::Confirmation(i) => {
            json!({"name":i.name(),"decision":i.decision(),"state":i.state(),"by_reading":i.by_reading(),"tests":tests(i.tests())})
        }
        ListItem::Decision(i) => {
            json!({"name":i.name(),"state":i.state(),"fingerprint":i.fingerprint()})
        }
    };
    let (path, line) = value.location();
    let mut result =
        json!({"id":value.id(),"kind":value.kind(),"name":variant["name"],"path":path,"line":line});
    let object = result
        .as_object_mut()
        .expect("the result is built as a JSON object literal");
    object.extend(
        variant
            .as_object()
            .expect("every variant is built as a JSON object literal")
            .iter()
            .filter(|(key, _)| key.as_str() != "name")
            .map(|(key, value)| (key.clone(), value.clone())),
    );
    result
}
pub fn list(value: &ReadList) -> Value {
    let mut out = json!({"items":value.items().iter().map(item).collect::<Vec<_>>()});
    // "translations" は言語が2つ以上のときだけ出す
    if let Some(translations) = value.translations() {
        out["translations"] = json!(
            translations
                .iter()
                .map(|translation| json!({
                    "path": translation.path(),
                    "sides": translation
                        .sides()
                        .iter()
                        .map(|side| json!({"language":side.language(),"path":side.path(),"blob":side.blob()}))
                        .collect::<Vec<_>>()
                }))
                .collect::<Vec<_>>()
        );
    }
    out
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
/// "overview" の群。群が無ければ両方 0
fn overview(group: Option<&kotowari::GroupTally>) -> Value {
    json!({"files":group.map_or(0, |group| group.files()),"marks":group.map_or(0, |group| group.marks())})
}
pub fn check(value: &CheckReport) -> Value {
    let mut result = json!({"findings":findings(value.findings()),"counts":value.counts(),"tests":file_tallies(value.tests()),"guides":{"files":value.guides().files(),"marks":value.guides().marks()},"overview":overview(value.group(kotowari::OVERVIEW_GROUP))});
    if let Some(surface) = value.surface() {
        result["surface"] = json!({"unspecified":surface.unspecified()});
    }
    let decisions = value.decisions();
    result["decisions"] = json!({"documents":decisions.documents(),"decisions":decisions.decisions(),"confirmations":decisions.confirmations()});
    result
}
pub fn status(value: &StatusReport) -> Value {
    let d = value.documents();
    let t = value.tests();
    let g = value.guides();
    let u = value.surface();
    let f = value.findings();
    let decisions = value.decisions();
    let rows = value.confirmations();
    json!({"documents":{"files":d.files(),"lines":d.lines()},"tests":{"marks":t.marks(),"files":file_tallies(t.files())},"guides":{"files":g.files(),"marks":g.marks()},"overview":overview(value.group(kotowari::OVERVIEW_GROUP)),"surface":{"total":u.total(),"specified":u.specified(),"unspecified":u.unspecified()},"decisions":{"proposed":decisions.proposed(),"implemented":decisions.implemented()},"confirmations":{"with_tests":rows.with_tests(),"without_tests":rows.without_tests(),"by_reading":rows.by_reading()},"findings":{"error":f.error(),"notice":f.notice()},"complete":value.complete()})
}
/// "written"、"removed"、"unchanged" の3つの鍵だけ
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
