//
// Created by penguin on 19-6-4.
//

#include <jni.h>
#include <jvmti.h>

JNIEXPORT jint JNICALL Agent_OnLoad(JavaVM *jvm, char *opts, void *reserved) {
    return JNI_OK;
}

JNIEXPORT void JNICALL Agent_OnUnload(JavaVM *vm) {
    // empty
}

int main() {
    printf("%d\n", sizeof(jvmtiCapabilities));
}