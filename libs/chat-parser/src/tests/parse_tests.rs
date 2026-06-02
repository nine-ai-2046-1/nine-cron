use crate::parse_and_normalize;

#[test]
fn test_top_level_title_with_params_no_title() {
    let raw = r#"
    {
      "action": "schedule_add",
      "needs_clarification": false,
      "clarification_question": null,
      "title": "lunch-reminder",
      "params": {
        "time": "12:07",
        "date": "2026-06-02",
        "recurrence": null,
        "cmd": "opencb send 'hi'"
      }
    }
    "#;

    let res = parse_and_normalize(raw).expect("should parse");
    assert_eq!(res.title.unwrap(), "lunch-reminder");
    let p = res.params.unwrap();
    assert_eq!(p.cmd, "opencb send 'hi'");
}
