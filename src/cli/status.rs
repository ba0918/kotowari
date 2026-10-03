use kotowari::StatusReport;
/// REQ-core-166: TBL-core-028 の群ごとに "群名 鍵=値 鍵=値" の1行を、表の順に出す。
/// 鍵の語は JSON と同じで、値の間は半角空白1つ、桁揃えの空白は入れない
pub fn print_text(result: &StatusReport) {
    let documents = result.documents();
    println!(
        "documents files={} lines={}",
        documents.files(),
        documents.lines()
    );
    let items = result.items();
    println!(
        "items requirement={} table={} property={} scenario={} flag={}",
        items.requirement(),
        items.table(),
        items.property(),
        items.scenario(),
        items.flag()
    );
    let requirements = result.requirements();
    println!(
        "requirements unit={} property={} proof={} review={} with_tests={} without_tests={} \
review_with_how_to_verify={} review_without_how_to_verify={} without_examples={} deferred={}",
        requirements.unit(),
        requirements.property(),
        requirements.proof(),
        requirements.review(),
        requirements.with_tests(),
        requirements.without_tests(),
        requirements.review_with_how_to_verify(),
        requirements.review_without_how_to_verify(),
        requirements.without_examples(),
        requirements.deferred()
    );
    let scenarios = result.scenarios();
    println!(
        "scenarios with_tests={} without_tests={} deferred={}",
        scenarios.with_tests(),
        scenarios.without_tests(),
        scenarios.deferred()
    );
    // TBL-core-028: "text" では読んだテストのファイルを拡張子ごとに数える
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
    let surface = result.surface();
    println!(
        "surface total={} specified={} unspecified={}",
        surface.total(),
        surface.specified(),
        surface.unspecified()
    );
    let findings = result.findings();
    println!(
        "findings error={} notice={}",
        findings.error(),
        findings.notice()
    );
    println!("complete {}", result.complete());
}
