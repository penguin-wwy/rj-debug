//
// Created by penguin on 19-6-4.
//

#include <jni.h>
#include <jvmti.h>

extern jint on_load(JavaVM *jvm, jvmtiEnv *jvmti, char *opts);

JNIEXPORT jint JNICALL Agent_OnLoad(JavaVM *jvm, char *opts, void *reserved) {
    jvmtiEnv *jvmti = NULL;
    if (JNI_OK != (*jvm)->GetEnv(jvm, (void **)(&jvmti), JVMTI_VERSION_1_2)) {
        printf("ERROR: Unable to access JVMTI.\n");
    }
    return on_load(jvm, jvmti, opts);
}

JNIEXPORT void JNICALL Agent_OnUnload(JavaVM *vm) {
    // empty
}

int main() {
    printf("%d\n", sizeof(jvmtiCapabilities));
}