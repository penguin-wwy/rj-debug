extern crate rj_debug;

use rj_debug::config::*;

#[test]
fn parse_break_point() {
    let data = r#"
        {
            "class_name": "Object",
            "method_name": "getClassName",
            "method_signature": "()String",
            "line": 1
        }
    "#;
    let b: BreakPoint = BreakPoint::new_from_str(data);
    assert_eq!(*b.get_line_number().unwrap(), 1 as u32);
}

#[test]
fn parse_null_break_point() {
    let data = r#"
        {
            "class_name": null,
            "method_name": null,
            "method_signature": null,
            "line": 1
        }
    "#;
    let b: BreakPoint = BreakPoint::new_from_str(data);
    assert_eq!(b.get_method_full_name().unwrap(), "<null><null>");
}

#[test]
fn parse_break_points() {
    let data = r#"
        [
            {
              "class_name": "empty",
              "method_name": "main",
              "method_signature": "(String[])V",
              "line": 2
            },
            {
              "class_name": null,
              "method_name": null,
              "method_signature": null,
              "line": 0
            }
        ]
    "#;
    let bv: Vec<BreakPoint> = BreakPoint::vec_from_str(data);
    assert_eq!(bv.len(), 2);
    assert_eq!(*bv[0].get_line_number().unwrap(), 2 as u32);
    assert_eq!(bv[1].get_method_full_name().unwrap(), "<null><null>");
}