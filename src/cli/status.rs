use kotowari::StatusReport;
/// 群ごとに "群名 鍵=値 鍵=値" の1行を出す（docs/decisions/core/status.md）。
/// 鍵の語は JSON と同じで、値の間は半角空白1つ、桁揃えの空白は入れない
pub fn print_text(result: &StatusReport) {
    let documents = result.documents();
    println!(
        "documents files={} lines={}",
        documents.files(),
        documents.lines()
    );
    // "text" では読んだテストのファイルを拡張子ごとに数える
    let mut tests = format!("tests marks={}", result.tests().marks());
    for (extension, tally) in result.tests().files() {
        tests.push_str(&format!(" {extension}={}", tally.files()));
    }
    println!("{tests}");
    println!(
        "guides files={} marks={}",
        result.guides().files(),
        result.guides().marks()
    );
    let overview = result.group(kotowari::OVERVIEW_GROUP);
    println!(
        "overview files={} marks={}",
        overview.map_or(0, |group| group.files()),
        overview.map_or(0, |group| group.marks())
    );
    let surface = result.surface();
    println!(
        "surface total={} specified={} unspecified={}",
        surface.total(),
        surface.specified(),
        surface.unspecified()
    );
    let decisions = result.decisions();
    println!(
        "decisions proposed={} implemented={}",
        decisions.proposed(),
        decisions.implemented()
    );
    let rows = result.confirmations();
    println!(
        "confirmations with_tests={} without_tests={} by_reading={}",
        rows.with_tests(),
        rows.without_tests(),
        rows.by_reading()
    );
    let findings = result.findings();
    println!(
        "findings error={} notice={}",
        findings.error(),
        findings.notice()
    );
    println!("complete {}", result.complete());
}
