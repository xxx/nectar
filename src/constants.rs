pub const ECHO: u8 = 1;

pub const GA: u8 = 249; // Go Ahead
pub const SGA: u8 = 3; // Suppress Go Ahead
pub const IAC: u8 = 255; // Interpret As Command
pub const SB: u8 = 250; // Subnegotiation Begin
pub const NAWS: u8 = 31; // Negotiate About Window Size
pub const SE: u8 = 240; // Subnegotiation End

pub const EL: u8 = 248; // Erase Line

pub const NOP: u8 = 241; // No operation
pub const NULL: u8 = 0; // No operation

pub const CR: u8 = 13; // Carriage Return
pub const LF: u8 = 10; // Line Feed
pub const CRLF: &[u8] = b"\r\n";

// `<https://tools.ietf.org/search/rfc1116>` 2.1 The LINEMODE function
pub const LINEMODE: u8 = 34;

// `<https://tools.ietf.org/search/rfc1116>` 2.2 LINEMODE suboption MODE
pub const MODE: u8 = 1; //

// When set, the client side of the connection should process all input lines,
// performing any editing function, and only send completed lines to the remote
// side. When unset, client side should not process any input from the user, and
// the server side should take care of all character processing that needs to be
// done.
pub const LINEMODE_EDIT: u8 = 1;

// When set, the client side should translate appropriate interrupts/signals to
// their Telnet equivalent. (These would be IP, BRK, ABORT, EOF, and SUSP). When
// unset, the client should pass interrupts/signals as their normal ASCII
// values.
pub const LINEMODE_TRAPSIG: u8 = 2;

// Indicates the desire to begin performing, or confirmation that you are now
// performing, the indicated option.
pub const WILL: u8 = 251;

// Indicates the refusal to perform, or continue performing, the indicated
// option.
pub const WONT: u8 = 252;

// Indicates the request that the other party perform, or confirmation that you
// are expecting the other party to perform, the indicated option.
pub const DO: u8 = 253;

// Indicates the demand that the other party stop performing, or confirmation
// that you are no longer expecting the other party to perform, the indicated
// option.
pub const DONT: u8 = 254;

//
// Various mud-specific options
//

/// EOR - End of Record - https://tintin.mudhalla.net/protocols/eor/
pub const TELOPT_EOR: u8 = 25; // the telnet option for negotiation
pub const EOR: u8 = 239; // the byte to send to indicate end of record

/// CHARSET - https://tools.ietf.org/html/rfc2066
pub const CHARSET: u8 = 42;

/// Mud Server Status Protocol - https://mudhalla.net/tintin/protocols/mssp/
pub const MSSP: u8 = 70;

/// Mud Client Compression Protocol (v2) - https://www.gammon.com.au/mccp/protocol.html
pub const MCCP2: u8 = 86;

/// Mud Sound Protocol - https://www.zuggsoft.com/zmud/msp.htm
pub const MSP: u8 = 90;

/// Mud eXtension Protocol - https://www.zuggsoft.com/zmud/mxp.htm
pub const MXP: u8 = 91;

/// Generic Mud Communication Protocol - https://www.gammon.com.au/gmcp
pub const GMCP: u8 = 201;

/// CHARSET subnegotiation commands
pub const CHARSET_REQUEST: u8 = 1;
pub const CHARSET_ACCEPTED: u8 = 2;
pub const CHARSET_REJECTED: u8 = 3;
pub const CHARSET_TTABLE_IS: u8 = 4;
pub const CHARSET_TTABLE_REJECTED: u8 = 5;
pub const CHARSET_TTABLE_ACK: u8 = 6;
pub const CHARSET_TTABLE_NAK: u8 = 7;
