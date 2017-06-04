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

#[derive(Copy, Clone)]
pub struct UCDRecord {
    pub category: u8,
    pub combining: u8,
    pub bidi_class: u8,
    pub mirrored: u8,
    pub east_asian_width: u8,
    pub script: u8,
    pub linebreak_class: u8
}

#[derive(Copy, Clone)]
pub struct MirrorPair {
    pub from: u16,
    pub to: u16
}
#[derive(Copy, Clone)]
pub struct BracketPair {
    pub from: u16,
    pub to: u16,
    pub bracket_type: u8
}

#[derive(Copy, Clone)]
pub struct ReIndex {
    pub start: u32,
    pub count: i16,
    pub index: i16
}