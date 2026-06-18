<?php

declare(strict_types=1);

namespace Meco;

/**
 * Mongolian Encoding Converter.
 *
 * Thin PHP wrapper over the Rust `meco` core via FFI. Drop-in replacement for hand-maintained
 * PHP ports: one shared, Java-verified engine instead of a parallel reimplementation.
 *
 * Usage:
 *   use Meco\Meco;
 *   echo Meco::translate(Meco::Z52, Meco::MENK_SHAPE, $input);
 *
 * The native library (libmeco.{so,dylib,dll}) is resolved from, in order:
 *   1. the MECO_LIB environment variable (absolute path), then
 *   2. prebuilt/<os>-<arch>/libmeco.<ext> shipped in this package, then
 *   3. the bare name "libmeco.<ext>" (system loader path).
 */
final class Meco
{
    public const ZVVNMOD = 'zvvnmod';
    public const DELEHI = 'delehi';
    public const MENK_SHAPE = 'menk_shape';
    public const MENK_LETTER = 'menk_letter';
    public const Z52 = 'z52';

    private static ?\FFI $ffi = null;

    /**
     * Convert $input from one Mongolian encoding to another (UTF-8 in/out).
     *
     * @throws \RuntimeException on an unknown encoding name or an unsupported conversion.
     */
    public static function translate(string $from, string $to, string $input): string
    {
        $ffi = self::ffi();
        $ptr = $ffi->meco_translate($from, $to, $input);
        if (\FFI::isNull($ptr)) {
            throw new \RuntimeException("meco: translate failed (from=$from, to=$to)");
        }
        try {
            return \FFI::string($ptr);
        } finally {
            $ffi->meco_free($ptr);
        }
    }

    /** Native core version. */
    public static function version(): string
    {
        // PHP FFI auto-converts a `const char *` return into a PHP string; `char *` stays CData.
        $v = self::ffi()->meco_version();
        return \is_string($v) ? $v : \FFI::string($v);
    }

    private static function ffi(): \FFI
    {
        if (self::$ffi === null) {
            self::$ffi = \FFI::cdef(
                'char *meco_translate(const char *from, const char *to, const char *input);'
                . 'void meco_free(char *ptr);'
                . 'const char *meco_version(void);',
                self::libraryPath()
            );
        }
        return self::$ffi;
    }

    private static function libraryPath(): string
    {
        $env = getenv('MECO_LIB');
        if (is_string($env) && $env !== '') {
            return $env;
        }
        $ext = ['Darwin' => 'dylib', 'Windows' => 'dll'][\PHP_OS_FAMILY] ?? 'so';
        $arch = \in_array(php_uname('m'), ['arm64', 'aarch64'], true) ? 'aarch64' : 'x86_64';
        $os = strtolower(\PHP_OS_FAMILY);
        $bundled = __DIR__ . "/../prebuilt/$os-$arch/libmeco.$ext";
        return is_file($bundled) ? $bundled : "libmeco.$ext";
    }
}
