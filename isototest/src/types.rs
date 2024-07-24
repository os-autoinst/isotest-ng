// SPDX-FileCopyrightText: Christopher Hock <christopher-hock@suse.com>
// SPDX-LicenseIdentifier: GPL-2.0-or-later
// TODO: Add extended ASCII and control sequences.

#[allow(non_camel_case_types, dead_code)]
/// Map Characters and Keys to their ASCII representation.
///
/// Oriented on [this table](https://theasciicode.com.ar/ascii-printable-characters/exclamation-mark-ascii-code-33.html)
pub enum KeyCode {
    NULL = 00,
    SOH = 01,
    STX = 02,
    ETX = 03,
    EOT = 04,
    ENQ = 05,
    ACK = 06,
    BEL = 07,
    BckSpc = 08,
    HorTab = 09,
    LineFeed = 10,
    VerTab = 11,
    FormFeed = 12,
    CarrRet = 13,
    ShiftOut = 14,
    ShiftIn = 15,
    DatLinkEsc = 16,
    DevCont1 = 17,
    DevCont2 = 18,
    DevCont3 = 19,
    DevCont4 = 20,
    NoAck = 21,
    SYN = 22,
    ETB = 23,
    CANCEL = 24,
    EndMed = 25,
    SUB = 26,
    ESC = 27,
    FileSep = 28,
    GroupSep = 29,
    RecSep = 30,
    UnitSep = 31,
    DEL = 127,
    SPACE = 32,
    ExcMrk = 33,
    DblQuote = 34,
    Pound = 35,
    Dollar = 36,
    Percent = 37,
    And = 38,
    Apo = 39,
    LRBrace = 40,
    RRBrace = 41,
    /// Asterisk ('*')
    Ast = 42,
    Plus = 43,
    Comma = 44,
    Minus = 45,
    Period = 46,
    FwdSlash = 47,
    Key0 = 48,
    Key1 = 49,
    Key2 = 50,
    Key3 = 51,
    Key4 = 52,
    Key5 = 53,
    Key6 = 54,
    Key7 = 55,
    Key8 = 56,
    Key9 = 57,
    Colon = 58,
    SColon = 59,
    LThan = 60,
    Equals = 61,
    GThan = 62,
    Question = 63,
    At = 64,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    /// Left straight bracket ('[')
    LBracket = 91,
    BckSlash = 92,
    /// Right straight bracket (']')
    RBracket = 93,
    Caret = 94,
    UScore = 95,
    /// Grave Accent or Backtick
    GraveAcc = 96,
    a = 97,
    b = 98,
    c = 99,
    d = 100,
    e = 101,
    f = 102,
    g = 103,
    h = 104,
    i = 105,
    j = 106,
    k = 107,
    l = 108,
    m = 109,
    n = 110,
    o = 111,
    p = 112,
    q = 113,
    r = 114,
    s = 115,
    t = 116,
    u = 117,
    v = 118,
    w = 119,
    x = 120,
    y = 121,
    z = 122,
    LCrlBrace = 123,
    Pipe = 124,
    RCrlBrace = 125,
    Circumflex = 126,
}
