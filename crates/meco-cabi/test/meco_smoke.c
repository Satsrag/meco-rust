/* Smoke test for the meco C ABI: translate(argv[1], argv[2], argv[3]) and print the result.
 * Drives the .dylib/.so exactly as a server (PHP FFI / cgo / JNI) would; the harness compares
 * its stdout against the Java golden corpus. */
#include "meco.h"
#include <stdio.h>

int main(int argc, char **argv) {
    if (argc < 4) {
        fprintf(stderr, "usage: %s <from> <to> <input>\n", argv[0]);
        return 2;
    }
    char *out = meco_translate(argv[1], argv[2], argv[3]);
    if (out == NULL) {
        fprintf(stderr, "meco_translate returned NULL\n");
        return 1;
    }
    fputs(out, stdout);
    meco_free(out);
    return 0;
}
