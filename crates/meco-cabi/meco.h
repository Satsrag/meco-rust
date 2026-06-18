#ifndef MECO_H
#define MECO_H

/* C ABI for the meco Mongolian Encoding Converter core.
 * Loadable from PHP FFI, Go (cgo), Java (JNI/Panama), Python (ctypes), etc. */

#ifdef __cplusplus
extern "C" {
#endif

/* Translate UTF-8 `input` from encoding `from` to `to`. `from`/`to` are canonical encoding names:
 * "zvvnmod", "delehi", "menk_shape", "menk_letter", "z52".
 * Returns a newly allocated UTF-8 C string the caller must release with meco_free(), or NULL on
 * error (NULL arg, invalid UTF-8, unknown encoding, unsupported conversion). */
char *meco_translate(const char *from, const char *to, const char *input);

/* Release a string returned by meco_translate(). NULL is ignored. */
void meco_free(char *ptr);

/* Library version (static; do not free). */
const char *meco_version(void);

#ifdef __cplusplus
}
#endif

#endif /* MECO_H */
