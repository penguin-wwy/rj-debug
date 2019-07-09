rj-debug
===================================
Rust library to auto debug jvm by use agent.

Constructing...

~~JNI Wrapper~~

~~JVMTI Wrapper~~

~~class print~~

~~argument parse~~

~~auto break point~~

--> auto assert equal

memory monitoring

bytecode debug consle

...

===================================

### Usage

#### Build

```
cargo build
```

#### Run

compile java to bytecode

if want to use *auto breakpoint*, add compile option -g
```
javac -g Main.java
java -agentpath:/path/librj_debug.so=config.json Main
```

#### Option

Config Json Example

```
{
  "verbose": false,
  "log_file": null,
  "bytecode_dump": [],
  "heap_print": false,
  "class_print": false,
  "break_point_json": null,
  "watch_var": null
}
```

*log_file*: specify the log file path;

*bytecode_dump*: use method signature to dump bytecode;

*class_print*: print which class be load by jvm;

*break_point_json*: specify auto breakpoint config file.

BreakPoint Config Json Example

```
[
  {
    "class_name": "Class Name",
    "method_name": "Method Name",
    "method_signature": "Method Signature",
    "line": line number,
    "var": "Variable Name"
  },
  {
    "class_name": "Class Name",
    "method_name": "Method Name",
    "method_signature": "Method Signature",
    "line": line number,
    "var": "Variable Name"
  }
  ...
]
```
