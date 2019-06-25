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
    let b: BreakPoint = *BreakPoint::new_from_str(data);
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
    let b: BreakPoint = *BreakPoint::new_from_str(data);
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
    let bv: Vec<BreakPoint> = *BreakPoint::vec_from_str(data);
    assert_eq!(bv.len(), 2);
    assert_eq!(*bv[0].get_line_number().unwrap(), 2 as u32);
    assert_eq!(bv[1].get_method_full_name().unwrap(), "<null><null>");
}

#[test]
fn init_methods_map() {
    let mut gc = GConfig {
        config: Some(Box::new(Configuration {
            verbose: false,
            log_file: None,
            bytecode_dump: vec![
                String::from("empty.main:([Ljava/lang/String;)V"),
                String::from("empty.getEmpty:(Ljava/lang/String;)I")
            ],
            heap_print: false,
            class_print: false,
            break_point_json: None,
            watch_var: None,
        })),
        breakpoints: None,
        watch_var: None,
        breakpoint_info: None,
        bytecode_methods: None,
    };
    gc.init_methods_map();
    let v1 = gc.bytecode_methods.as_ref().unwrap().get("empty").unwrap().get(0).unwrap();
    let v2 = gc.bytecode_methods.as_ref().unwrap().get("empty").unwrap().get(1).unwrap();
    assert_eq!(v1, "main:([Ljava/lang/String;)V");
    assert_eq!(v2, "getEmpty:(Ljava/lang/String;)I");
}