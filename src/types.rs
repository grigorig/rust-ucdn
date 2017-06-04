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

//! Types associated with Unicode character properties.

pub use db::BiDiClass;
pub use db::EastAsianWidth;
pub use db::GeneralCategory;
pub use db::Script;

use std::mem::transmute;
use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq, Debug)] #[repr(u8)]
pub enum BracketType {
    Open = 0,
    Close = 1,
    None = 2
}

#[derive(Copy, Clone, PartialEq, Debug)] #[repr(u8)]
pub enum LinebreakClass {
    OP = 0,
    CL = 1,
    CP = 2,
    QU = 3,
    GL = 4,
    NS = 5,
    EX = 6,
    SY = 7,
    IS = 8,
    PR = 9,
    PO = 10,
    NU = 11,
    AL = 12,
    HL = 13,
    ID = 14,
    IN = 15,
    HY = 16,
    BA = 17,
    BB = 18,
    B2 = 19,
    ZW = 20,
    CM = 21,
    WJ = 22,
    H2 = 23,
    H3 = 24,
    JL = 25,
    JV = 26,
    JT = 27,
    RI = 28,
    AI = 29,
    BK = 30,
    CB = 31,
    CJ = 32,
    CR = 33,
    LF = 34,
    NL = 35,
    SA = 36,
    SG = 37,
    SP = 38,
    XX = 39,
    ZWJ = 40,
    EB = 41,
    EM = 42
}

impl TryFrom<u8> for LinebreakClass {
    type Error = &'static str;
    fn try_from(t: u8) -> Result<LinebreakClass, &'static str> {
        if 0 as u8 <= t && t <= LinebreakClass::EM as u8 {
            unsafe { Ok(transmute(t)) }
        } else {
            Err("invalid variant")
        }
    }
}

impl TryFrom<u8> for BiDiClass {
    type Error = &'static str;
    fn try_from(t: u8) -> Result<BiDiClass, &'static str> {
        if 0 as u8 <= t && t <= BiDiClass::PDI as u8 {
            unsafe { Ok(transmute(t)) }
        } else {
            Err("invalid variant")
        }
    }
}

impl TryFrom<u8> for EastAsianWidth {
    type Error = &'static str;
    fn try_from(t: u8) -> Result<EastAsianWidth, &'static str> {
        if 0 as u8 <= t && t <= EastAsianWidth::N as u8 {
            unsafe { Ok(transmute(t)) }
        } else {
            Err("invalid variant")
        }
    }
}

impl TryFrom<u8> for GeneralCategory {
    type Error = &'static str;
    fn try_from(t: u8) -> Result<GeneralCategory, &'static str> {
        if 0 as u8 <= t && t <= GeneralCategory::ZS as u8 {
            unsafe { Ok(transmute(t)) }
        } else {
            Err("invalid variant")
        }
    }
}

impl TryFrom<u8> for Script {
    type Error = &'static str;
    fn try_from(t: u8) -> Result<Script, &'static str> {
        if 0 as u8 <= t && t <= Script::TANGUT as u8 {
            unsafe { Ok(transmute(t)) }
        } else {
            Err("invalid variant")
        }
    }
}

impl TryFrom<u8> for BracketType {
    type Error = &'static str;
    fn try_from(t: u8) -> Result<BracketType, &'static str> {
        if 0 as u8 <= t && t <= BracketType::None as u8 {
            unsafe { Ok(transmute(t)) }
        } else {
            Err("invalid variant")
        }
    }
}