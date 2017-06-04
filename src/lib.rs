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

#![feature(try_from)]
#[cfg(test)] mod tests;
use std::convert::TryFrom;

mod db;
pub mod types;
pub use types::*;
pub mod c_interface;

/* Generic Unicode */
const BMP_MAX_CODEPOINT: u32 = 0x10000;
const UNICODE_MAX_CODEPOINT: u32 = 0x110000;
const SURR_LOW: u32 = 0xd800;
const SURR_HIGH: u32 = 0xdc00;

/* Hangul Jamo constants */
const SBASE: u32 = 0xAC00;
const LBASE: u32 = 0x1100;
const VBASE: u32 = 0x1161;
const TBASE: u32 = 0x11A7;
const LCOUNT: u32 = 19;
const VCOUNT: u32 = 21;
const TCOUNT: u32 = 28;
const NCOUNT: u32 = VCOUNT * TCOUNT;
const SCOUNT: u32 = LCOUNT * NCOUNT;

fn get_ucd_record(code: u32) -> Result<UCDRecord, &'static str> {
    if code >= UNICODE_MAX_CODEPOINT {
        Err("invalid char")
    } else {
        let index  = (db::INDEX0[(code >> (db::SHIFT1+db::SHIFT2)) as usize] as usize) << db::SHIFT1;
        let offset = ((code >> db::SHIFT2) & ((1<<db::SHIFT1) - 1)) as usize;
        let index2  = (db::INDEX1[(index + offset)] as usize) << db::SHIFT2;
        let offset2 = (code & ((1<<db::SHIFT2) - 1)) as usize;
        let index3  = db::INDEX2[(index2 + offset2)] as usize;
        Ok(db::UCD_RECORDS[index3])
    }
}

fn get_decomp_record(code: u32) -> [u16; 19] {
    let mut index: usize;

    if code >= UNICODE_MAX_CODEPOINT {
        index = 0;
    } else {
        index = (db::DECOMP_INDEX0[(code >> (db::DECOMP_SHIFT1+db::DECOMP_SHIFT2)) as usize] as usize) << db::DECOMP_SHIFT1;
        let offset = ((code >> db::DECOMP_SHIFT2) & ((1u32 << db::DECOMP_SHIFT1) - 1)) as usize;
        index = (db::DECOMP_INDEX1[index + offset] as usize) << db::DECOMP_SHIFT2;
        let offset2 = (code & ((1 << db::DECOMP_SHIFT2) - 1)) as usize;
        index = db::DECOMP_INDEX2[index + offset2] as usize;
    }

    let record_len = (db::DECOMP_DATA[index] >> 8) as usize;
    let mut record: [u16; 19] = [0; 19];
    record[0..record_len + 1].clone_from_slice(&db::DECOMP_DATA[index .. index + record_len + 1]);
    record
}

fn get_comp_index(code: u32, idx: &[ReIndex]) -> Option<usize> {
    let mut i = 0;
    while idx[i].start != 0 {
        let cur = idx[i];
        i += 1;
        if code < cur.start {
            return None;
        }
        if code <= cur.start + cur.count as u32 {
            return Some(cur.index as usize + (code as usize - cur.start as usize));
        }
    }

    return None;
}

fn hangul_pair_decompose(code: u32) -> Option<(u32, u32)> {
    if code < SBASE || code >= (SBASE + SCOUNT) {
        return None
    }

    let si = code - SBASE;
    if (si % TCOUNT) != 0 {
        Some((
            SBASE + (si / TCOUNT) * TCOUNT,
            TBASE + (si % TCOUNT)
        ))
    } else {
        Some((
            LBASE + (si / NCOUNT),
            VBASE + (si % NCOUNT) / TCOUNT
        ))
    }
}

fn hangul_pair_compose(a: u32, b: u32) -> Option<u32> {
    if a >= SBASE && a < (SBASE + SCOUNT) && b >= TBASE && b < (TBASE + TCOUNT) {
        /* LV,T */
        Some(a + (b - TBASE))
    } else if a >= LBASE && a < (LBASE + LCOUNT) && b >= VBASE && b < (VBASE + VCOUNT) {
        /* L,V */
        Some(SBASE + (a - LBASE) * NCOUNT + (b - VBASE) * TCOUNT)
    } else {
        None
    }
}

fn decode_utf16(seq: &[u16]) -> (u32, usize) {
    if seq[0] < (SURR_LOW as u16) || seq[0] > (SURR_HIGH as u16) {
        (seq[0] as u32, 1)
    } else {
        (BMP_MAX_CODEPOINT + (seq[1] as u32) - SURR_HIGH + (((seq[0] as u32) - SURR_LOW) << 10), 2)
    }
}

pub fn get_unicode_version() -> &'static str {
    // strip off zero termination
    &db::UNIDATA_VERSION[0..db::UNIDATA_VERSION.len()-1]
}

pub fn get_combining_class(code: u32) -> Result<u8, &'static str> {
    match get_ucd_record(code) {
        Ok(v) => Ok(v.combining),
        Err(e) => Err(e) 
    }  
}

pub fn get_east_asian_width(code: u32) -> Result<EastAsianWidth, &'static str> {
    match get_ucd_record(code) {
        Ok(v) => EastAsianWidth::try_from(v.east_asian_width),
        Err(e) => Err(e) 
    }  
}

pub fn get_general_category(code: u32) -> Result<GeneralCategory, &'static str> {
    match get_ucd_record(code) {
        Ok(v) => GeneralCategory::try_from(v.category),
        Err(e) => Err(e) 
    }  
}

pub fn get_bidi_class(code: u32) -> Result<BiDiClass, &'static str> {
    match get_ucd_record(code) {
        Ok(v) => BiDiClass::try_from(v.bidi_class),
        Err(e) => Err(e) 
    }  
}

pub fn get_mirrored(code: u32) -> Result<bool, &'static str> {
    match get_ucd_record(code) {
        Ok(v) => Ok(v.mirrored > 0),
        Err(e) => Err(e) 
    }  
}

pub fn get_script(code: u32) -> Result<Script, &'static str> {
    match get_ucd_record(code) {
        Ok(v) => Script::try_from(v.script),
        Err(e) => Err(e) 
    }  
}

pub fn get_linebreak_class(code: u32) -> Result<LinebreakClass, &'static str> {
    match get_ucd_record(code) {
        Ok(v) => LinebreakClass::try_from(v.linebreak_class),
        Err(e) => Err(e) 
    }    
}

pub fn get_resolved_linebreak_class(code: u32) -> LinebreakClass {
    match get_ucd_record(code) {
        Ok(w) => match LinebreakClass::try_from(w.linebreak_class) {
            Ok(v) => match v {
                LinebreakClass::AI | LinebreakClass::SG | LinebreakClass::XX => LinebreakClass::AL,
                LinebreakClass::SA => 
                    if w.category == db::GeneralCategory::MC as u8 || w.category == db::GeneralCategory::MN as u8 {
                        LinebreakClass::CM
                    } else {
                        LinebreakClass::AL
                    },
                LinebreakClass::CJ => LinebreakClass::NS,
                LinebreakClass::CB => LinebreakClass::B2,
                LinebreakClass::NL => LinebreakClass::BK,
                v => v
            },
            Err(_) => LinebreakClass::XX
        },
        Err(_) => LinebreakClass::XX
    }
}

pub fn mirror(code: u32) -> Result<u32, &'static str> {
    if code >= BMP_MAX_CODEPOINT {
        return Err("invalid char")
    }

    let res = db::MIRROR_PAIRS.binary_search_by(|probe| probe.from.cmp(&(code as u16)));
    match res {
        Ok(v) => Ok(db::MIRROR_PAIRS[v].to as u32),
        Err(_) => Err("no mirrored character found")
    }
}

fn get_paired_bracket(code: u32) -> Option<BracketPair> {
    if code >= BMP_MAX_CODEPOINT {
        return None
    }
    
    let res = db::BRACKET_PAIRS.binary_search_by(|probe| probe.from.cmp(&(code as u16)));
    match res {
        Ok(v) => Some(db::BRACKET_PAIRS[v]),
        Err(_) => None
    }
}

pub fn paired_bracket(code: u32) -> Result<u32, &'static str> {
    match get_paired_bracket(code) {
        Some(v) => Ok(v.to as u32),
        None => Err("no paired bracket found")
    }
}

pub fn decompose(code: u32) -> Result<(u32, u32), &'static str> {
    // Try Hangul decomposition
    match hangul_pair_decompose(code) {
        Some(v) => return Ok(v),
        _ => {}
    }

    let record = get_decomp_record(code);
    let record_len = record[0] >> 8;

    if (record[0] & 0xff) != 0 || record_len == 0 {
        Err("no decomposition")
    } else {
        // XXX: this is a bit ugly, also not sure about scoping
        let mut b: u32 = 0;
        let (a, step) = decode_utf16(&record[1 .. 3]);
        if record_len > 1 {
            let (x, _) = decode_utf16(&record[step + 1 .. step + 3]);
            b = x;
        }
        Ok((a, b))
    }
}

pub fn paired_bracket_type(code: u32) -> Result<BracketType, &'static str> {
    match get_paired_bracket(code) {
        Some(v) => BracketType::try_from(v.bracket_type),
        None => Ok(BracketType::None)
    }
}

fn get_comp_data(l: usize, r: usize) -> u32 {
    let indexi = l * db::TOTAL_LAST as usize + r;
    let index  = (db::COMP_INDEX0[indexi >> (db::COMP_SHIFT1+db::COMP_SHIFT2)] << db::COMP_SHIFT1) as usize;
    let offset = ((indexi >> db::COMP_SHIFT2) & ((1<<db::COMP_SHIFT1) - 1)) as usize;
    let index2  = (db::COMP_INDEX1[index + offset] << db::COMP_SHIFT2) as usize;
    let offset2 = (indexi & ((1<<db::COMP_SHIFT2) - 1)) as usize;
    db::COMP_DATA[index2 + offset2]
}

pub fn compose(a: u32, b: u32) -> Result<u32, &'static str> {
    // Try Hangul composition
    match hangul_pair_compose(a, b) {
        Some(v) => return Ok(v),
        _ => {}
    }

    // Otherwise, try composition with table data
    let l = get_comp_index(a, &db::NFC_FIRST);
    let r = get_comp_index(b, &db::NFC_LAST);
    match (l, r) {
        (Some(l), Some(r)) => Ok(get_comp_data(l, r)),
        _ => Err("no composition found")
    }    
}

pub fn compat_decompose(code: u32) -> Result<(usize, [u32; 18]), &'static str>  {
    let record = get_decomp_record(code);
    let record_len = (record[0] >> 8) as usize;
    let mut decomposed: [u32; 18] = [0; 18];

    if record_len == 0 {
        Err("no decomposition found")
    } else {
        let mut step: usize = 0;
        let mut i = 0;
        while step < record_len {
            let (c, s) = if (record_len - step) < 2 {
                (record[step+1] as u32, 1)
            } else {
                decode_utf16(&record[step + 1 .. step + 3])
            };
            decomposed[i] = c;
            step += s;
            i += 1;
        }
        Ok((i, decomposed))
    }
}
