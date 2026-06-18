import com.zvvnmod.meco.translate.enumeration.CodeType;
import com.zvvnmod.meco.translate.service.TranslateService;
import com.zvvnmod.meco.translate.shape.ShapeRuleHolder;
import com.zvvnmod.meco.translate.shape.ShapeTranslateRule;
import com.zvvnmod.meco.translate.shape.from.z52.Z52TranslateRuleFrom;
import com.zvvnmod.meco.translate.shape.to.z52.Z52TranslateRuleTo;
import com.zvvnmod.meco.translate.shape.from.menk.MenkShapeTranslateRuleFrom;
import com.zvvnmod.meco.translate.shape.to.menk.MenkShapeTranslateRuleTo;
import com.zvvnmod.meco.translate.letter.from.LetterFromRuleHolder;
import com.zvvnmod.meco.translate.letter.from.LetterTranslateRuleFrom;
import com.zvvnmod.meco.translate.letter.from.delehi.DelehiTranslateRuleFrom;
import com.zvvnmod.meco.translate.letter.from.menk.MenkLetterTranslateRuleFrom;
import com.zvvnmod.meco.translate.letter.to.LetterToRuleHolder;
import com.zvvnmod.meco.translate.letter.to.LetterTranslateRuleTo;
import com.zvvnmod.meco.translate.letter.to.delehi.DelehiTranslateRuleTo;
import com.zvvnmod.meco.translate.letter.to.menk.MenkTranslateRuleTo;

import java.lang.reflect.Field;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.HashSet;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;

/**
 * Golden-corpus generator: runs the REAL meco Java TranslateService (the authoritative oracle)
 * over a Delehi word list and emits the full (from,to) conversion matrix.
 *
 * No Spring: rule holders are wired by hand (every rule has a no-arg constructor) and injected
 * into TranslateService via reflection, so this runs on any JDK (no CGLIB/ASM proxying).
 *
 * Output: TSV, one row per (from,to,input). Fields: from <TAB> to <TAB> input <TAB> output <TAB> threw.
 * input/output are escaped so every char outside printable-ASCII (and backslash) becomes \\uXXXX,
 * making the file ASCII, diff-friendly, and parseable in Rust without a JSON dependency.
 *
 * Args: <corpus-file> <output-tsv>. Corpus = one Delehi (standard Unicode Mongolian) string per line.
 */
public class OracleHarness {

    // Encodings the Rust port will support (Oyun + Utn57 are deferred/unsupported).
    static final CodeType[] ENCODINGS = {
        CodeType.Zvvnmod, CodeType.Delehi, CodeType.Menk_Letter, CodeType.Menk_Shape, CodeType.Z52
    };

    public static void main(String[] args) throws Exception {
        if (args.length < 2) {
            System.err.println("usage: OracleHarness <corpus-file> <output-tsv>");
            System.exit(2);
        }
        quietLogging();

        TranslateService ts = buildService();

        List<String> words = Files.readAllLines(Paths.get(args[0]), StandardCharsets.UTF_8);
        StringBuilder out = new StringBuilder(1 << 20);
        Set<String> seen = new HashSet<>();
        int rows = 0;

        for (String w : words) {
            if (w.isEmpty()) {
                continue;
            }
            // Derive each encoding form from the Delehi nominal via the oracle itself.
            Map<CodeType, String> forms = new LinkedHashMap<>();
            forms.put(CodeType.Delehi, w);
            for (CodeType e : ENCODINGS) {
                if (e == CodeType.Delehi) {
                    continue;
                }
                try {
                    forms.put(e, ts.translate(CodeType.Delehi, e, w));
                } catch (Throwable ignored) {
                    // form unavailable from this word; skip it as a source
                }
            }
            // Emit the full matrix using each available form as the input.
            for (Map.Entry<CodeType, String> fromEntry : forms.entrySet()) {
                CodeType from = fromEntry.getKey();
                String input = fromEntry.getValue();
                for (CodeType to : ENCODINGS) {
                    if (to == from) {
                        continue;
                    }
                    String key = from.name() + '|' + to.name() + '|' + input;
                    if (!seen.add(key)) {
                        continue;
                    }
                    String output;
                    boolean threw;
                    try {
                        output = ts.translate(from, to, input);
                        threw = false;
                    } catch (Throwable t) {
                        output = "";
                        threw = true;
                    }
                    out.append(from.name()).append('\t').append(to.name()).append('\t')
                       .append(esc(input)).append('\t').append(esc(output)).append('\t')
                       .append(threw ? '1' : '0').append('\n');
                    rows++;
                }
            }
        }
        Files.write(Paths.get(args[1]), out.toString().getBytes(StandardCharsets.UTF_8));
        System.err.println("oracle: wrote " + rows + " rows from " + words.size() + " words");
    }

    static TranslateService buildService() throws Exception {
        ShapeRuleHolder shape = new ShapeRuleHolder(Arrays.<ShapeTranslateRule>asList(
            new Z52TranslateRuleFrom(), new Z52TranslateRuleTo(),
            new MenkShapeTranslateRuleFrom(), new MenkShapeTranslateRuleTo()));
        LetterFromRuleHolder letterFrom = new LetterFromRuleHolder(Arrays.<LetterTranslateRuleFrom>asList(
            new DelehiTranslateRuleFrom(), new MenkLetterTranslateRuleFrom()));
        LetterToRuleHolder letterTo = new LetterToRuleHolder(Arrays.<LetterTranslateRuleTo>asList(
            new DelehiTranslateRuleTo(), new MenkTranslateRuleTo()));

        TranslateService ts = new TranslateService();
        inject(ts, "shapeRuleHolder", shape);
        inject(ts, "letterFromRuleHolder", letterFrom);
        inject(ts, "letterToRuleHolder", letterTo);
        return ts;
    }

    static void inject(Object target, String field, Object value) throws Exception {
        Field f = target.getClass().getDeclaredField(field);
        f.setAccessible(true);
        f.set(target, value);
    }

    /** Escape every char to \\uXXXX except printable ASCII (and backslash, which is always escaped). */
    static String esc(String s) {
        StringBuilder b = new StringBuilder(s.length() * 2);
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c >= 0x20 && c <= 0x7E && c != '\\') {
                b.append(c);
            } else {
                b.append("\\u").append(String.format("%04x", (int) c));
            }
        }
        return b.toString();
    }

    static void quietLogging() {
        try {
            ch.qos.logback.classic.Logger root = (ch.qos.logback.classic.Logger)
                org.slf4j.LoggerFactory.getLogger(org.slf4j.Logger.ROOT_LOGGER_NAME);
            root.setLevel(ch.qos.logback.classic.Level.OFF);
        } catch (Throwable ignored) {
            // no logback binding -> nothing to quiet
        }
    }
}
