/* Copyright (c) 2017 Grigori Goronzy <greg@chown.ath.cx>
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use super::*;

/* What to test:
    * - Normal cases, possibly all the different paths
    * - Use characters from different places inside BMP to test index table correctness
    * - Standard error cases (e.g. no mapping or decomposition exists)
    * - Unassigned codepoints
    * - Out of Unicode cases
    * - Surrogates and/or cases outside BMP
    * - Compositions and decompositions involving surrogates
    * - Additional special cases (e.g. very long compat decompositions)
    */

/* Basic properties etc. */
#[test]
fn test_basic() {
    assert_eq!(get_unicode_version(), "9.0.0");

    /* one sample check inside BMP for each property */
    assert_eq!(get_general_category(0x0040), Ok(GeneralCategory::PO));
    assert_eq!(get_script(0x0122), Ok(Script::LATIN));
    assert_eq!(get_bidi_class(0x0032), Ok(BiDiClass::EN));
    assert_eq!(get_east_asian_width(0x4000), Ok(EastAsianWidth::W));
    assert_eq!(get_linebreak_class(0xfeff), Ok(LinebreakClass::WJ));
    assert_eq!(get_mirrored(0x0028), Ok(true));
    assert_eq!(get_combining_class(0), Ok(0));

    /* check validity in various blocks and planes outside BMP */
    assert_eq!(get_script(0x103a0), Ok(Script::OLD_PERSIAN)); // SMP, Old Persian
    assert_eq!(get_script(0x14400), Ok(Script::ANATOLIAN_HIEROGLYPHS)); // SMP, Anatolian Hieroglyphs
    assert_eq!(get_script(0x1e910), Ok(Script::ADLAM)); // SMP, Adlam
    assert_eq!(get_script(0x20100), Ok(Script::HAN)); // SIP, CJK Unified Ideographs
    assert_eq!(get_script(0x28100), Ok(Script::HAN)); // SIP, CJK Unified Ideographs
    assert_eq!(get_script(0x2f810), Ok(Script::HAN)); // SIP, CJK Compatibility Ideographs
    assert_eq!(get_general_category(0xe0020), Ok(GeneralCategory::CF)); // SSP, Tags

    /* check error behavior */
    assert_eq!(get_general_category(0xfefe), Ok(GeneralCategory::CN)); // unassigned
    assert_eq!(get_general_category(0x200000), Err("invalid char"));  // outside Unicode

    /* boundaries */
    assert_eq!(get_general_category(0x10FFFF), Ok(GeneralCategory::CN)); // last valid codepoint
    assert_eq!(get_general_category(0x110000), Err("invalid char")); // first invalid codepoint
}

/* Canonical and compatibility decomposition */
#[test]
fn test_decomposition() {
    assert_eq!(decompose(0x00c4), Ok((0x0041, 0x0308))); // normal case
    assert_eq!(decompose(0xfb01), Err("no decomposition")); // no decomposition (only compatibility)
    assert_eq!(decompose(0x0065), Err("no decomposition")); // no decomposition

    /* Hangul Jamo */
    assert_eq!(decompose(0xac01), Ok((0xac00, 0x11a8))); // normal case (LV,T)
    assert_eq!(decompose(0xd7a3), Ok((0xd788, 0x11c2))); // normal case (LV,T)
    assert_eq!(decompose(0xac00), Ok((0x1100, 0x1161))); // normal case (L,V)
    assert_eq!(decompose(0xd7a4), Err("no decomposition")); // invalid Jamo (unassigned)

    let mut cmp: [u32; 18] = [0; 18];
    cmp[0..2].clone_from_slice(&[65, 776]);
    assert_eq!(compat_decompose(0x00c4), Ok((2, cmp))); // normal case
    assert_eq!(compat_decompose(0x0065), Err("no decomposition found"));  // no decomposition

    /* multi-part sequence */
    let a = decompose(0xfb2c);
    assert_eq!(a, Ok((0xfb49, 0x05c1)));
    match a {
        Ok((a, _)) => {
            let c = decompose(a);
            assert_eq!(c, Ok((0x5e9, 0x5bc)))
        },
        Err(_) => assert!(false)
    }

    /* outside BMP */
    assert_eq!(decompose(0x1109a), Ok((0x11099, 0x110ba)));
    cmp[0..2].clone_from_slice(&[0x2a600, 0]);
    assert_eq!(compat_decompose(0x2fa1d), Ok((1, cmp)));

    /* very long decomposition */
    cmp[0..18].clone_from_slice(&[0x0635, 0x0644, 0x0649, 0x0020, 0x0627, 0x0644, 0x0644, 0x0647, 0x0020, 0x0639, 0x0644, 0x064A, 0x0647, 0x0020, 0x0648, 0x0633, 0x0644, 0x0645]);
    assert_eq!(compat_decompose(0xfdfa), Ok((18, cmp)));
}

#[test]
fn test_composition() {
    assert_eq!(compose(0x0041, 0x0308), Ok(0x00c4)); //  normal case
    assert_eq!(compose(0x0066, 0x0069), Err("no composition found")); // compatibility decomposition forms don't have a recomposition
    assert_eq!(compose(0x0028, 0x0028), Err("no composition found")); // no composition exists
    assert_eq!(compose(0x200000, 0x0028), Err("no composition found")); // outside Unicode

    /* Hangul Jamo */
    assert_eq!(compose(0xac00, 0x11a8), Ok(0xac01)); // normal case (LV,T)
    assert_eq!(compose(0x1100, 0x1161), Ok(0xac00)); // normal case (L,V)
    assert_eq!(compose(0xd788, 0x11a3), Err("no composition found")); // invalid Jamo combinaton (LV,T with invalid T)

    /* multi-part sequence */
    let ab  = compose(0x0041, 0x0308);
    assert_eq!(ab, Ok(0x00c4));
    match ab {
        Ok(v) => { 
            let abc = compose(v, 0x0304);
            assert_eq!(abc, Ok(0x01de));
        },
        Err(_) => assert!(false)
    }

    /* excluded compositions */
    assert_eq!(compose(0xfb49, 0x05c1), Err("no composition found"));
    assert_eq!(compose(0x2add, 0x0338), Err("no composition found"));

    /* outside BMP */
    assert_eq!(compose(0x11099, 0x110ba), Ok(0x1109A));
}

#[test]
fn test_mirror() {
    assert_eq!(mirror(0x0028), Ok(0x0029)); //  normal case
    assert_eq!(mirror(0x223d), Ok(0x223c)); // normal case
    assert_eq!(mirror(0x0032), Err("no mirrored character found")); // no mirroring exists
    assert_eq!(mirror(0x200000), Err("invalid char")); // outside Unicode
}

#[test]
fn test_bidi_bracket() {
    assert_eq!(paired_bracket(0x0028), Ok(0x0029)); // normal case
    assert_eq!(paired_bracket(0xff08), Ok(0xff09)); // normal case
    assert_eq!(paired_bracket(0x00ab), Err("no paired bracket found")); // mirrored, but not a bracket
    assert_eq!(paired_bracket(0x200000), Err("no paired bracket found")); // outside Unicode

    assert_eq!(paired_bracket_type(0x0028), Ok(BracketType::Open)); // normal case
    assert_eq!(paired_bracket_type(0x0029), Ok(BracketType::Close)); // normal case
    assert_eq!(paired_bracket_type(0x0020), Ok(BracketType::None)); // normal case
    assert_eq!(paired_bracket_type(0x200000), Ok(BracketType::None)); // outside Unicode
}

#[test]
fn test_linebreak_class() {
    assert_eq!(get_linebreak_class(0x0020), Ok(LinebreakClass::SP)); // normal case
    assert_eq!(get_linebreak_class(0xffef), Ok(LinebreakClass::XX)); // unassigned
    assert_eq!(get_linebreak_class(0xd800), Ok(LinebreakClass::SG)); // surrogate
    assert_eq!(get_linebreak_class(0x3400), Ok(LinebreakClass::ID)); // not part of database
    assert_eq!(get_linebreak_class(0x1f46e), Ok(LinebreakClass::EB)); // outside BMP, unusual class

    // error cases
    assert_eq!(get_linebreak_class(0x200000), Err("invalid char")); // outside of Unicode
}

#[test]
fn test_resolved_linebreak_class() {
    /* cases that resolve to AL */
    assert_eq!(get_resolved_linebreak_class(0xa7), LinebreakClass::AL); // AI
    assert_eq!(get_resolved_linebreak_class(0xd801), LinebreakClass::AL); // SG 
    assert_eq!(get_resolved_linebreak_class(0xffef), LinebreakClass::AL); // XX

    /* cases that resolve to CM/AL */
    assert_eq!(get_resolved_linebreak_class(0x0e31), LinebreakClass::CM); // category MN
    assert_eq!(get_resolved_linebreak_class(0x1a55), LinebreakClass::CM); // category MC
    assert_eq!(get_resolved_linebreak_class(0x19da), LinebreakClass::AL); // other category

    /* cases that resolve to NS */
    assert_eq!(get_resolved_linebreak_class(0x3041), LinebreakClass::NS); // CJ

    /* cases that resolve to B2 */
    assert_eq!(get_resolved_linebreak_class(0xfffc), LinebreakClass::B2); // CB

    /* cases that resolve to BK */
    assert_eq!(get_resolved_linebreak_class(0x0085), LinebreakClass::BK); // NL

    /* error case */
    assert_eq!(get_resolved_linebreak_class(0x200000), LinebreakClass::XX); // outside of Unicode
}