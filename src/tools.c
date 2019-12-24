//
// Created by penguin on 19-12-23.
//

#include <stdlib.h>
#include <memory.h>

#include <jni.h>
#include <jvmti.h>

#include "tools.h"

JNIEXPORT jint JNICALL Agent_OnLoad(JavaVM *vm, char *options, void *reserved) {
	jvmtiEnv *jvmti = NULL;
	if (JNI_OK != (*vm)->GetEnv(vm, (void **) (&jvmti), JVMTI_VERSION_1_2)) {
		ERROR_EXIT(1, "ERROR: Unable to access JVMTI.\n");
	}
	INFO("Options: %s\n", options);
}