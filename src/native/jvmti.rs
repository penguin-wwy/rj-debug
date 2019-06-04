use super::jni::*;
use super::jni_md::*;

pub const JVMTI_VERSION_1: jint = 0x30010000;
pub const JVMTI_VERSION_1_0: jint = 0x30010000;
pub const JVMTI_VERSION_1_1: jint = 0x30010100;
pub const JVMTI_VERSION_1_2: jint = 0x30010200;

pub const JVMTI_VERSION: jint = 0x30000000 + (1 * 0x10000) + (2 * 0x100) + 1;  /* version: 1.2.1 */