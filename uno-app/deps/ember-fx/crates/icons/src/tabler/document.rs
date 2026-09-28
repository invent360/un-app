//! Document icons from Tabler Icons.
//!
//! This module contains 793 icons.

use crate::tabler::TablerIconData;

// SVG Constants
const A_B_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 21h3c.81 0 1.48 -.67 1.48 -1.48l.02 -.02c0 -.82 -.69 -1.5 -1.5 -1.5h-3v3" /> <path d="M16 15h2.5c.84 -.01 1.5 .66 1.5 1.5s-.66 1.5 -1.5 1.5h-2.5v-3" /> <path d="M4 9v-4c0 -1.036 .895 -2 2 -2s2 .964 2 2v4" /> <path d="M2.99 11.98a9 9 0 0 0 9 9m9 -9a9 9 0 0 0 -9 -9" /> <path d="M8 7h-4" /> </svg>"##;
const A_B_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 16v-5.5a2.5 2.5 0 0 1 5 0v5.5m0 -4h-5" /> <path d="M12 12v6" /> <path d="M12 6v2" /> <path d="M16 8h3a2 2 0 1 1 0 4h-3m3 0a2 2 0 0 1 .83 3.82m-3.83 -3.82v-4" /> <path d="M3 3l18 18" /> </svg>"##;
const ABC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 16v-6a2 2 0 1 1 4 0v6" /> <path d="M3 13h4" /> <path d="M10 8v6a2 2 0 1 0 4 0v-1a2 2 0 1 0 -4 0v1" /> <path d="M20.732 12a2 2 0 0 0 -3.732 1v1a2 2 0 0 0 3.726 1.01" /> </svg>"##;
const ALIGN_BOX_BOTTOM_CENTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 15v2" /> <path d="M12 11v6" /> <path d="M15 13v4" /> </svg>"##;
const ALIGN_BOX_BOTTOM_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M7 15v2" /> <path d="M10 11v6" /> <path d="M13 13v4" /> </svg>"##;
const ALIGN_BOX_BOTTOM_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M11 15v2" /> <path d="M14 11v6" /> <path d="M17 13v4" /> </svg>"##;
const ALIGN_BOX_CENTER_BOTTOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2" /> <path d="M11 17h2" /> <path d="M9 14h6" /> <path d="M10 11h4" /> </svg>"##;
const ALIGN_BOX_CENTER_MIDDLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2" /> <path d="M11 15h2" /> <path d="M9 12h6" /> <path d="M10 9h4" /> </svg>"##;
const ALIGN_BOX_CENTER_STRETCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2" /> <path d="M11 17h2" /> <path d="M9 12h6" /> <path d="M10 7h4" /> </svg>"##;
const ALIGN_BOX_CENTER_TOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2" /> <path d="M11 13h2" /> <path d="M9 10h6" /> <path d="M10 7h4" /> </svg>"##;
const ALIGN_BOX_LEFT_BOTTOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 17h-2" /> <path d="M13 14h-6" /> <path d="M11 11h-4" /> </svg>"##;
const ALIGN_BOX_LEFT_MIDDLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 15h-2" /> <path d="M13 12h-6" /> <path d="M11 9h-4" /> </svg>"##;
const ALIGN_BOX_LEFT_STRETCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 17h-2" /> <path d="M13 12h-6" /> <path d="M11 7h-4" /> </svg>"##;
const ALIGN_BOX_LEFT_TOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 13h-2" /> <path d="M13 10h-6" /> <path d="M11 7h-4" /> </svg>"##;
const ALIGN_BOX_RIGHT_BOTTOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M15 17h2" /> <path d="M11 14h6" /> <path d="M13 11h4" /> </svg>"##;
const ALIGN_BOX_RIGHT_MIDDLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 15h2" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M11 12h6" /> <path d="M13 9h4" /> </svg>"##;
const ALIGN_BOX_RIGHT_STRETCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 17h2" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M11 12h6" /> <path d="M13 7h4" /> </svg>"##;
const ALIGN_BOX_RIGHT_TOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M15 13h2" /> <path d="M11 10h6" /> <path d="M13 7h4" /> </svg>"##;
const ALIGN_BOX_TOP_CENTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 9v-2" /> <path d="M12 13v-6" /> <path d="M15 11v-4" /> </svg>"##;
const ALIGN_BOX_TOP_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M7 9v-2" /> <path d="M10 13v-6" /> <path d="M13 11v-4" /> </svg>"##;
const ALIGN_BOX_TOP_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M11 9v-2" /> <path d="M14 13v-6" /> <path d="M17 11v-4" /> </svg>"##;
const ALIGN_CENTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6l16 0" /> <path d="M8 12l8 0" /> <path d="M6 18l12 0" /> </svg>"##;
const ALIGN_JUSTIFIED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6l16 0" /> <path d="M4 12l16 0" /> <path d="M4 18l12 0" /> </svg>"##;
const ALIGN_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6l16 0" /> <path d="M4 12l10 0" /> <path d="M4 18l14 0" /> </svg>"##;
const ALIGN_LEFT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4v16" /> <path d="M8 6h12" /> <path d="M8 12h6" /> <path d="M8 18h10" /> </svg>"##;
const ALIGN_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6l16 0" /> <path d="M10 12l10 0" /> <path d="M6 18l14 0" /> </svg>"##;
const ALIGN_RIGHT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 4v16" /> <path d="M4 6h12" /> <path d="M10 12h6" /> <path d="M6 18h10" /> </svg>"##;
const ALPHA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.1 6c-1.1 2.913 -1.9 4.913 -2.4 6c-1.879 4.088 -3.713 6 -6 6c-2.4 0 -4.8 -2.4 -4.8 -6s2.4 -6 4.8 -6c2.267 0 4.135 1.986 6 6c.512 1.102 1.312 3.102 2.4 6" /> </svg>"##;
const ALPHABET_ARABIC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 6v4" /> <path d="M13 14h8q -2.518 -3 -4 -3" /> <path d="M13 6v9.958c0 .963 0 1.444 -.293 1.743s-.764 .299 -1.707 .299h-1" /> <path d="M7 6v9.958c0 .963 0 1.444 -.293 1.743s-.764 .299 -1.707 .299h-1" /> </svg>"##;
const ALPHABET_BANGLA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 12c.904 -.027 3 2 3 7" /> <path d="M10 11c0 -.955 0 -2 .786 -2.677c1.262 -1.089 3.025 .55 3.2 2.06c.086 .741 -.215 3.109 -1.489 4.527c-.475 .53 -.904 .992 -1.711 1.074c-.75 .076 -1.364 -.122 -2.076 -.588c-1.138 -.743 -2.327 -1.997 -3.336 -3.73c-1.078 -1.849 -1.66 -3.113 -2.374 -5.666" /> <path d="M7.37 7.072c.769 -.836 5.246 -4.094 8.4 -.202c.382 .472 .573 .708 .9 1.63c.326 .921 .326 1.562 .326 2.844v7.656" /> <path d="M17 10c0 -1.989 1.5 -4 4 -4" /> </svg>"##;
const ALPHABET_CYRILLIC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 10h2a2 2 0 0 1 2 2v5h-3a2 2 0 1 1 0 -4h3" /> <path d="M19 7h-3a2 2 0 0 0 -2 2v6a2 2 0 0 0 2 2h1a2 2 0 0 0 2 -2v-3a2 2 0 0 0 -2 -2h-3" /> </svg>"##;
const ALPHABET_GREEK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 10v7" /> <path d="M5 12a2 2 0 0 1 2 -2h1a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-1a2 2 0 0 1 -2 -2l0 -3" /> <path d="M14 20v-11a2 2 0 0 1 2 -2h1a2 2 0 0 1 2 2v1a2 2 0 0 1 -2 2a2 2 0 0 1 2 2v1a2 2 0 0 1 -2 2" /> </svg>"##;
const ALPHABET_HEBREW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 6c2.333 5.143 6.611 6.857 9.333 12" /> <path d="M13.667 14c2.505 -1.5 2.666 -4.141 2.666 -5.333c0 -1.778 -.443 -2.667 -.443 -2.667" /> <path d="M7.485 18s-.485 -.905 -.485 -2.714c0 -1.172 .164 -3.722 2.641 -5.27" /> </svg>"##;
const ALPHABET_KOREAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7h6c0 2.5 -1.593 8.474 -6 10" /> <path d="M16 5v14l0 -14" /> <path d="M16 12h2" /> </svg>"##;
const ALPHABET_LATIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 10h2a2 2 0 0 1 2 2v5h-3a2 2 0 1 1 0 -4h3" /> <path d="M14 7v10" /> <path d="M14 12a2 2 0 0 1 2 -2h1a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-1a2 2 0 0 1 -2 -2l0 -3" /> </svg>"##;
const ALPHABET_POLISH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 10h2a2 2 0 0 1 2 2v5h-3a2 2 0 1 1 0 -4h3" /> <path d="M16 7v10" /> <path d="M18 11l-4 2" /> <path d="M10.5 17a1.5 1.5 0 0 0 0 3" /> </svg>"##;
const ALPHABET_RUNES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 18v-12" /> <path d="M13 6l4 4l4 -4" /> <path d="M11 18l-7 -8l4 -4l4 4l-7 8" /> </svg>"##;
const ALPHABET_THAI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 18v-3.444c0 -.49 .165 -.924 .494 -1.363c.326 -.449 1.009 -.76 1.506 -.934c.032 -.011 .035 -.079 .004 -.095c-.434 -.22 -1.294 -.52 -1.626 -1.032l-.014 -.021l-.083 -.125c-.281 -.42 -.281 -1.246 -.281 -1.246c0 -1.456 .849 -2.62 1.837 -3.199q .9 -.54 2.137 -.541q 1.077 0 1.995 .47c1.328 .647 2.031 2.202 2.031 3.976v7.554" /> </svg>"##;
const ARCHIVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2" /> <path d="M5 8v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-10" /> <path d="M10 12l4 0" /> </svg>"##;
const ARCHIVE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h11a2 2 0 1 1 0 4h-7m-4 0h-3a2 2 0 0 1 -.826 -3.822" /> <path d="M5 8v10a2 2 0 0 0 2 2h10a2 2 0 0 0 1.824 -1.18m.176 -3.82v-7" /> <path d="M10 12h2" /> <path d="M3 3l18 18" /> </svg>"##;
const ARTICLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -12" /> <path d="M7 8h10" /> <path d="M7 12h10" /> <path d="M7 16h10" /> </svg>"##;
const ARTICLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h11a2 2 0 0 1 2 2v11m-1.172 2.821a1.993 1.993 0 0 1 -.828 .179h-14a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 1.156 -1.814" /> <path d="M7 8h1m4 0h5" /> <path d="M7 12h5m4 0h1" /> <path d="M7 16h9" /> <path d="M3 3l18 18" /> </svg>"##;
const ASTERISK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12l8 -4.5" /> <path d="M12 12v9" /> <path d="M12 12l-8 -4.5" /> <path d="M12 12l8 4.5" /> <path d="M12 3v9" /> <path d="M12 12l-8 4.5" /> </svg>"##;
const ASTERISK_SIMPLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12v-9" /> <path d="M12 12l-9 -2.5" /> <path d="M12 12l9 -2.5" /> <path d="M12 12l6 8.5" /> <path d="M12 12l-6 8.5" /> </svg>"##;
const AT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M16 12v1.5a2.5 2.5 0 0 0 5 0v-1.5a9 9 0 1 0 -5.5 8.28" /> </svg>"##;
const AT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.174 9.17a4 4 0 0 0 5.646 5.668m1.18 -2.838a4 4 0 0 0 -4 -4" /> <path d="M19.695 15.697a2.5 2.5 0 0 0 1.305 -2.197v-1.5a9 9 0 0 0 -13.055 -8.047m-2.322 1.683a9 9 0 0 0 9.877 14.644" /> <path d="M3 3l18 18" /> </svg>"##;
const BACKSPACE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 6a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-11l-5 -5a1.5 1.5 0 0 1 0 -2l5 -5l11 0" /> <path d="M12 10l4 4m0 -4l-4 4" /> </svg>"##;
const BALLPEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 6l7 7l-4 4" /> <path d="M5.828 18.172a2.828 2.828 0 0 0 4 0l10.586 -10.586a2 2 0 0 0 0 -2.829l-1.171 -1.171a2 2 0 0 0 -2.829 0l-10.586 10.586a2.828 2.828 0 0 0 0 4" /> <path d="M4 20l1.768 -1.768" /> </svg>"##;
const BALLPEN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 6l7 7l-2 2" /> <path d="M10 10l-4.172 4.172a2.828 2.828 0 1 0 4 4l4.172 -4.172" /> <path d="M16 12l4.414 -4.414a2 2 0 0 0 0 -2.829l-1.171 -1.171a2 2 0 0 0 -2.829 0l-4.414 4.414" /> <path d="M4 20l1.768 -1.768" /> <path d="M3 3l18 18" /> </svg>"##;
const BASELINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h16" /> <path d="M8 16v-8a4 4 0 1 1 8 0v8" /> <path d="M8 10h8" /> </svg>"##;
const BASELINE_DENSITY_LARGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4h16" /> <path d="M4 20h16" /> </svg>"##;
const BASELINE_DENSITY_MEDIUM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h16" /> <path d="M4 12h16" /> <path d="M4 4h16" /> </svg>"##;
const BASELINE_DENSITY_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 3h16" /> <path d="M4 9h16" /> <path d="M4 15h16" /> <path d="M4 21h16" /> </svg>"##;
const BETA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 22v-14a4 4 0 0 1 4 -4h.5a3.5 3.5 0 0 1 0 7h-.5h.5a4.5 4.5 0 1 1 -4.5 4.5v-.5" /> </svg>"##;
const BIBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 4v16h-12a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12" /> <path d="M19 16h-12a2 2 0 0 0 -2 2" /> <path d="M12 7v6" /> <path d="M10 9h4" /> </svg>"##;
const BLOCKQUOTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 15h15" /> <path d="M21 19h-15" /> <path d="M15 11h6" /> <path d="M21 7h-6" /> <path d="M9 9h1a1 1 0 1 1 -1 1v-2.5a2 2 0 0 1 2 -2" /> <path d="M3 9h1a1 1 0 1 1 -1 1v-2.5a2 2 0 0 1 2 -2" /> </svg>"##;
const BOLD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5h6a3.5 3.5 0 0 1 0 7h-6l0 -7" /> <path d="M13 12h1a3.5 3.5 0 0 1 0 7h-7v-7" /> </svg>"##;
const BOLD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h4a3.5 3.5 0 0 1 2.222 6.204m-3.222 .796h-5v-5" /> <path d="M17.107 17.112a3.5 3.5 0 0 1 -3.107 1.888h-7v-7" /> <path d="M3 3l18 18" /> </svg>"##;
const BOOK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19a9 9 0 0 1 9 0a9 9 0 0 1 9 0" /> <path d="M3 6a9 9 0 0 1 9 0a9 9 0 0 1 9 0" /> <path d="M3 6l0 13" /> <path d="M12 6l0 13" /> <path d="M21 6l0 13" /> </svg>"##;
const BOOK_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 4v16h-12a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12" /> <path d="M19 16h-12a2 2 0 0 0 -2 2" /> <path d="M9 8h6" /> </svg>"##;
const BOOK_DOWNLOAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-6a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12v5" /> <path d="M13 16h-7a2 2 0 0 0 -2 2" /> <path d="M15 19l3 3l3 -3" /> <path d="M18 22v-9" /> </svg>"##;
const BOOK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19a9 9 0 0 1 9 0a9 9 0 0 1 5.899 -1.096" /> <path d="M3 6a9 9 0 0 1 2.114 -.884m3.8 -.21c1.07 .17 2.116 .534 3.086 1.094a9 9 0 0 1 9 0" /> <path d="M3 6v13" /> <path d="M12 6v2m0 4v7" /> <path d="M21 6v11" /> <path d="M3 3l18 18" /> </svg>"##;
const BOOK_UPLOAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 20h-8a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12v5" /> <path d="M11 16h-5a2 2 0 0 0 -2 2" /> <path d="M15 16l3 -3l3 3" /> <path d="M18 13v9" /> </svg>"##;
const BOOKMARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 7v14l-6 -4l-6 4v-14a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4" /> </svg>"##;
const BOOKMARK_AI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.02 18.32l-4.02 2.68v-14a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v4.5" /> <path d="M14 21v-4a2 2 0 1 1 4 0v4" /> <path d="M14 19h4" /> <path d="M21 15v6" /> </svg>"##;
const BOOKMARK_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17l-6 4v-14a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v4" /> <path d="M18.42 15.61a2.1 2.1 0 1 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const BOOKMARK_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17l-6 4v-14a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v8" /> <path d="M16 19h6" /> </svg>"##;
const BOOKMARK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.708 3.721a3.982 3.982 0 0 1 2.292 -.721h4a4 4 0 0 1 4 4v7m0 4v3l-6 -4l-6 4v-14c0 -.308 .035 -.609 .1 -.897" /> <path d="M3 3l18 18" /> </svg>"##;
const BOOKMARK_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17l-6 4v-14a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v5" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const BOOKMARK_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 19l-3 -2l-6 4v-14a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v4" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const BOOKMARKS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 10v11l-5 -3l-5 3v-11a3 3 0 0 1 3 -3h4a3 3 0 0 1 3 3" /> <path d="M11 3h5a3 3 0 0 1 3 3v11" /> </svg>"##;
const BOOKMARKS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 7h2a2 2 0 0 1 2 2v2m0 4v6l-5 -3l-5 3v-12a2 2 0 0 1 2 -2" /> <path d="M9.265 4a2 2 0 0 1 1.735 -1h6a2 2 0 0 1 2 2v10" /> <path d="M3 3l18 18" /> </svg>"##;
const BOOKS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -14" /> <path d="M9 5a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -14" /> <path d="M5 8h4" /> <path d="M9 16h4" /> <path d="M13.803 4.56l2.184 -.53c.562 -.135 1.133 .19 1.282 .732l3.695 13.418a1.02 1.02 0 0 1 -.634 1.219l-.133 .041l-2.184 .53c-.562 .135 -1.133 -.19 -1.282 -.732l-3.695 -13.418a1.02 1.02 0 0 1 .634 -1.219l.133 -.041" /> <path d="M14 9l4 -1" /> <path d="M16 16l3.923 -.98" /> </svg>"##;
const BOOKS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 9v10a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-14" /> <path d="M8 4a1 1 0 0 1 1 1" /> <path d="M9 5a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v4" /> <path d="M13 13v6a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-10" /> <path d="M5 8h3" /> <path d="M9 16h4" /> <path d="M14.254 10.244l-1.218 -4.424a1.02 1.02 0 0 1 .634 -1.219l.133 -.041l2.184 -.53c.562 -.135 1.133 .19 1.282 .732l3.236 11.75" /> <path d="M19.585 19.589l-1.572 .38c-.562 .136 -1.133 -.19 -1.282 -.731l-.952 -3.458" /> <path d="M14 9l4 -1" /> <path d="M19.207 15.199l.716 -.18" /> <path d="M3 3l18 18" /> </svg>"##;
const BOX_MULTIPLE_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 6a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M7 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -10" /> <path d="M17 17v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h2" /> </svg>"##;
const BOX_MULTIPLE_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -10" /> <path d="M17 17v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h2" /> <path d="M14 14v-8l-2 2" /> </svg>"##;
const BOX_MULTIPLE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -10" /> <path d="M17 17v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h2" /> <path d="M12 8a2 2 0 1 1 4 0c0 .591 -.417 1.318 -.816 1.858l-3.184 4.143l4 0" /> </svg>"##;
const BOX_MULTIPLE_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -10" /> <path d="M17 17v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h2" /> <path d="M14 10a2 2 0 1 0 -2 -2" /> <path d="M12 12a2 2 0 1 0 2 -2" /> </svg>"##;
const BOX_MULTIPLE_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -10" /> <path d="M17 17v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h2" /> <path d="M15 14v-8l-4 6h5" /> </svg>"##;
const BOX_MULTIPLE_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -10" /> <path d="M12 14h2a2 2 0 1 0 0 -4h-2v-4h4" /> <path d="M17 17v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h2" /> </svg>"##;
const BOX_MULTIPLE_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -10" /> <path d="M12 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 8a2 2 0 1 0 -4 0v4" /> <path d="M17 17v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h2" /> </svg>"##;
const BOX_MULTIPLE_7_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -10" /> <path d="M12 6h4l-2 8" /> <path d="M17 17v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h2" /> </svg>"##;
const BOX_MULTIPLE_8_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -10" /> <path d="M12 8a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M12 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 17v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h2" /> </svg>"##;
const BOX_MULTIPLE_9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -10" /> <path d="M12 8a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M12 12a2 2 0 1 0 4 0v-4" /> <path d="M17 17v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h2" /> </svg>"##;
const CERTIFICATE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 15a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M13 17.5v4.5l2 -1.5l2 1.5v-4.5" /> <path d="M10 19h-5a2 2 0 0 1 -2 -2v-10c0 -1.1 .9 -2 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -1 1.73" /> <path d="M6 9l12 0" /> <path d="M6 12l3 0" /> <path d="M6 15l2 0" /> </svg>"##;
const CERTIFICATE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M10 7h4" /> <path d="M10 18v4l2 -1l2 1v-4" /> <path d="M10 19h-2a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-2" /> </svg>"##;
const CERTIFICATE_2_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12a3 3 0 1 0 3 3" /> <path d="M11 7h3" /> <path d="M10 18v4l2 -1l2 1v-4" /> <path d="M10 19h-2a2 2 0 0 1 -2 -2v-11m1.18 -2.825c.25 -.112 .529 -.175 .82 -.175h8a2 2 0 0 1 2 2v9m-.175 3.82a2 2 0 0 1 -1.825 1.18h-2" /> <path d="M3 3l18 18" /> </svg>"##;
const CERTIFICATE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.876 12.881a3 3 0 0 0 4.243 4.243m.588 -3.42a3.012 3.012 0 0 0 -1.437 -1.423" /> <path d="M13 17.5v4.5l2 -1.5l2 1.5v-4.5" /> <path d="M10 19h-5a2 2 0 0 1 -2 -2v-10c0 -1.1 .9 -2 2 -2m4 0h10a2 2 0 0 1 2 2v10" /> <path d="M6 9h3m4 0h5" /> <path d="M6 12h3" /> <path d="M6 15h2" /> <path d="M3 3l18 18" /> </svg>"##;
const CHALKBOARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 19h-3a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v11a1 1 0 0 1 -1 1" /> <path d="M11 17a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -1" /> </svg>"##;
const CHALKBOARD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 19h-3a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2m4 0h10a2 2 0 0 1 2 2v10" /> <path d="M17 17v1a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h4" /> <path d="M3 3l18 18" /> </svg>"##;
const CIRCLE_DASHED_LETTER_A_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-6a2 2 0 1 1 4 0v6" /> <path d="M10 13h4" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_B_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16h2a2 2 0 1 0 0 -4h-2h2a2 2 0 1 0 0 -4h-2l0 8" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_C_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10a2 2 0 1 0 -4 0v4a2 2 0 1 0 4 0" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_D_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2l-2 0" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_E_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h-4v8h4" /> <path d="M10 12h2.5" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_F_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h3" /> <path d="M14 8h-4v8" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_G_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_H_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-8m4 0v8" /> <path d="M10 12h4" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_I_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8v8" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_J_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4v6a2 2 0 1 1 -4 0" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_K_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v8" /> <path d="M14 8l-2.5 4l2.5 4" /> <path d="M10 12h1.5" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_L_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v8h4" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_M_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 16v-8l3 5l3 -5v8" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_N_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-8l4 8v-8" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_O_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_P_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_Q_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M13 15l1 1" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_R_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8m4 0l-3 -4" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_S_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_T_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4" /> <path d="M12 8v8" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_U_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v6a2 2 0 1 0 4 0v-6" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_V_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8l2 8l2 -8" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_W_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 8l1 8l2 -5l2 5l1 -8" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8l4 8" /> <path d="M10 16l4 -8" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8l2 5l2 -5" /> <path d="M12 16v-3" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_LETTER_Z_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4l-4 8h4" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_NUMBER_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M10 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> </svg>"##;
const CIRCLE_DASHED_NUMBER_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M10 10l2 -2v8" /> </svg>"##;
const CIRCLE_DASHED_NUMBER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M10 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const CIRCLE_DASHED_NUMBER_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M10 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const CIRCLE_DASHED_NUMBER_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M10 8v3a1 1 0 0 0 1 1h3" /> <path d="M14 8v8" /> </svg>"##;
const CIRCLE_DASHED_NUMBER_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const CIRCLE_DASHED_NUMBER_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M14 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const CIRCLE_DASHED_NUMBER_7_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M10 8h4l-2 8" /> </svg>"##;
const CIRCLE_DASHED_NUMBER_8_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M12 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const CIRCLE_DASHED_NUMBER_9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_A_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-6a2 2 0 1 1 4 0v6" /> <path d="M10 13h4" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_B_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16h2a2 2 0 1 0 0 -4h-2h2a2 2 0 1 0 0 -4h-2l0 8" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_C_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10a2 2 0 1 0 -4 0v4a2 2 0 1 0 4 0" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_D_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2l-2 0" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_E_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h-4v8h4" /> <path d="M10 12h2.5" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_F_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h3" /> <path d="M14 8h-4v8" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_G_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_H_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-8m4 0v8" /> <path d="M10 12h4" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_I_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8v8" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_J_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4v6a2 2 0 1 1 -4 0" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_K_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v8" /> <path d="M14 8l-2.5 4l2.5 4" /> <path d="M10 12h1.5" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_L_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v8h4" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_M_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 16v-8l3 5l3 -5v8" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_N_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-8l4 8v-8" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_O_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_P_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_Q_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M13 15l1 1" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_R_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8m4 0l-3 -4" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_S_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_T_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4" /> <path d="M12 8v8" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_U_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v6a2 2 0 1 0 4 0v-6" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_V_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8l2 8l2 -8" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_W_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 8l1 8l2 -5l2 5l1 -8" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8l4 8" /> <path d="M10 16l4 -8" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8l2 5l2 -5" /> <path d="M12 16v-3" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_DOTTED_LETTER_Z_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4l-4 8h4" /> <path d="M7.5 4.21v.01" /> <path d="M4.21 7.5v.01" /> <path d="M3 12v.01" /> <path d="M4.21 16.5v.01" /> <path d="M7.5 19.79v.01" /> <path d="M12 21v.01" /> <path d="M16.5 19.79v.01" /> <path d="M19.79 16.5v.01" /> <path d="M21 12v.01" /> <path d="M19.79 7.5v.01" /> <path d="M16.5 4.21v.01" /> <path d="M12 3v.01" /> </svg>"##;
const CIRCLE_LETTER_A_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 16v-6a2 2 0 1 1 4 0v6" /> <path d="M10 13h4" /> </svg>"##;
const CIRCLE_LETTER_B_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 16h2a2 2 0 1 0 0 -4h-2h2a2 2 0 1 0 0 -4h-2v8" /> </svg>"##;
const CIRCLE_LETTER_C_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14 10a2 2 0 1 0 -4 0v4a2 2 0 1 0 4 0" /> </svg>"##;
const CIRCLE_LETTER_D_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2h-2" /> </svg>"##;
const CIRCLE_LETTER_E_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14 8h-4v8h4" /> <path d="M10 12h2.5" /> </svg>"##;
const CIRCLE_LETTER_F_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 12h3" /> <path d="M14 8h-4v8" /> </svg>"##;
const CIRCLE_LETTER_G_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> </svg>"##;
const CIRCLE_LETTER_H_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 16v-8m4 0v8" /> <path d="M10 12h4" /> </svg>"##;
const CIRCLE_LETTER_I_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 8v8" /> </svg>"##;
const CIRCLE_LETTER_J_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 8h4v6a2 2 0 1 1 -4 0" /> </svg>"##;
const CIRCLE_LETTER_K_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 8v8" /> <path d="M14 8l-2.5 4l2.5 4" /> <path d="M10 12h1.5" /> </svg>"##;
const CIRCLE_LETTER_L_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 8v8h4" /> </svg>"##;
const CIRCLE_LETTER_M_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 16v-8l3 5l3 -5v8" /> </svg>"##;
const CIRCLE_LETTER_N_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 16v-8l4 8v-8" /> </svg>"##;
const CIRCLE_LETTER_O_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> </svg>"##;
const CIRCLE_LETTER_P_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8" /> </svg>"##;
const CIRCLE_LETTER_Q_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M13 15l1 1" /> </svg>"##;
const CIRCLE_LETTER_R_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8m4 0l-3 -4" /> </svg>"##;
const CIRCLE_LETTER_S_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1" /> </svg>"##;
const CIRCLE_LETTER_T_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 8h4" /> <path d="M12 8v8" /> </svg>"##;
const CIRCLE_LETTER_U_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 8v6a2 2 0 1 0 4 0v-6" /> </svg>"##;
const CIRCLE_LETTER_V_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 8l2 8l2 -8" /> </svg>"##;
const CIRCLE_LETTER_W_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 8l1 8l2 -5l2 5l1 -8" /> </svg>"##;
const CIRCLE_LETTER_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 8l4 8" /> <path d="M10 16l4 -8" /> </svg>"##;
const CIRCLE_LETTER_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 8l2 5l2 -5" /> <path d="M12 16v-3" /> </svg>"##;
const CIRCLE_LETTER_Z_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 8h4l-4 8h4" /> </svg>"##;
const CIRCLE_NUMBER_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> </svg>"##;
const CIRCLE_NUMBER_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 10l2 -2v8" /> </svg>"##;
const CIRCLE_NUMBER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const CIRCLE_NUMBER_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 9a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1" /> </svg>"##;
const CIRCLE_NUMBER_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 8v3a1 1 0 0 0 1 1h3" /> <path d="M14 8v8" /> </svg>"##;
const CIRCLE_NUMBER_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const CIRCLE_NUMBER_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const CIRCLE_NUMBER_7_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 8h4l-2 8" /> </svg>"##;
const CIRCLE_NUMBER_8_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const CIRCLE_NUMBER_9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const CLEAR_FORMATTING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 15l4 4m0 -4l-4 4" /> <path d="M7 6v-1h11v1" /> <path d="M7 19l4 0" /> <path d="M13 5l-4 14" /> </svg>"##;
const CLIPBOARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> </svg>"##;
const CLIPBOARD_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M9 14l2 2l4 -4" /> </svg>"##;
const CLIPBOARD_COPY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h3m9 -9v-5a2 2 0 0 0 -2 -2h-2" /> <path d="M13 17v-1a1 1 0 0 1 1 -1h1m3 0h1a1 1 0 0 1 1 1v1m0 3v1a1 1 0 0 1 -1 1h-1m-3 0h-1a1 1 0 0 1 -1 -1v-1" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> </svg>"##;
const CLIPBOARD_DATA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M9 17v-4" /> <path d="M12 17v-1" /> <path d="M15 17v-2" /> <path d="M12 17v-1" /> </svg>"##;
const CLIPBOARD_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M11.993 16.75l2.747 -2.815a1.9 1.9 0 0 0 0 -2.632a1.775 1.775 0 0 0 -2.56 0l-.183 .188l-.183 -.189a1.775 1.775 0 0 0 -2.56 0a1.899 1.899 0 0 0 0 2.632l2.738 2.825l.001 -.009" /> </svg>"##;
const CLIPBOARD_LIST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M9 12l.01 0" /> <path d="M13 12l2 0" /> <path d="M9 16l.01 0" /> <path d="M13 16l2 0" /> </svg>"##;
const CLIPBOARD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.575 5.597a2 2 0 0 0 -.575 1.403v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2m0 -4v-8a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 1 1 0 4h-2" /> <path d="M3 3l18 18" /> </svg>"##;
const CLIPBOARD_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M10 14h4" /> <path d="M12 12v4" /> </svg>"##;
const CLIPBOARD_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h4.5m7.5 -10v-4a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const CLIPBOARD_SMILE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 13h.01" /> <path d="M14 13h.01" /> <path d="M10 16a3.5 3.5 0 0 0 4 0" /> <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> </svg>"##;
const CLIPBOARD_TEXT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M9 12h6" /> <path d="M9 16h6" /> </svg>"##;
const CLIPBOARD_TYPOGRAPHY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M9 12v-1h6v1" /> <path d="M12 11v6" /> <path d="M11 17h2" /> </svg>"##;
const CLIPBOARD_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M10 12l4 4m0 -4l-4 4" /> </svg>"##;
const CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8l-4 4l4 4" /> <path d="M17 8l4 4l-4 4" /> <path d="M14 4l-4 16" /> </svg>"##;
const CODE_ASTERISK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 19a2 2 0 0 1 -2 -2v-4l-1 -1l1 -1v-4a2 2 0 0 1 2 -2" /> <path d="M12 11.875l3 -1.687" /> <path d="M12 11.875v3.375" /> <path d="M12 11.875l-3 -1.687" /> <path d="M12 11.875l3 1.688" /> <path d="M12 8.5v3.375" /> <path d="M12 11.875l-3 1.688" /> <path d="M18 19a2 2 0 0 0 2 -2v-4l1 -1l-1 -1v-4a2 2 0 0 0 -2 -2" /> </svg>"##;
const CODE_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 14l-2 -2l2 -2" /> <path d="M14 10l2 2l-2 2" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CODE_CIRCLE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.5 13.5l-1.5 -1.5l1.5 -1.5" /> <path d="M15.5 10.5l1.5 1.5l-1.5 1.5" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M13 9.5l-2 5.5" /> </svg>"##;
const CODE_DOTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 12h.01" /> <path d="M12 12h.01" /> <path d="M9 12h.01" /> <path d="M6 19a2 2 0 0 1 -2 -2v-4l-1 -1l1 -1v-4a2 2 0 0 1 2 -2" /> <path d="M18 19a2 2 0 0 0 2 -2v-4l1 -1l-1 -1v-4a2 2 0 0 0 -2 -2" /> </svg>"##;
const CODE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12h6" /> <path d="M6 19a2 2 0 0 1 -2 -2v-4l-1 -1l1 -1v-4a2 2 0 0 1 2 -2" /> <path d="M18 19a2 2 0 0 0 2 -2v-4l1 -1l-1 -1v-4a2 2 0 0 0 -2 -2" /> </svg>"##;
const CODE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8l-4 4l4 4" /> <path d="M17 8l4 4l-2.5 2.5" /> <path d="M14 4l-1.201 4.805m-.802 3.207l-2 7.988" /> <path d="M3 3l18 18" /> </svg>"##;
const CODE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12h6" /> <path d="M12 9v6" /> <path d="M6 19a2 2 0 0 1 -2 -2v-4l-1 -1l1 -1v-4a2 2 0 0 1 2 -2" /> <path d="M18 19a2 2 0 0 0 2 -2v-4l1 -1l-1 -1v-4a2 2 0 0 0 -2 -2" /> </svg>"##;
const COLUMNS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6l5.5 0" /> <path d="M4 10l5.5 0" /> <path d="M4 14l5.5 0" /> <path d="M4 18l5.5 0" /> <path d="M14.5 6l5.5 0" /> <path d="M14.5 10l5.5 0" /> <path d="M14.5 14l5.5 0" /> <path d="M14.5 18l5.5 0" /> </svg>"##;
const COLUMNS_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v16a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1l0 -16" /> </svg>"##;
const COLUMNS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v16a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1v-16" /> <path d="M12 3v18" /> </svg>"##;
const COLUMNS_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v16a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1v-16" /> <path d="M9 3v18" /> <path d="M15 3v18" /> </svg>"##;
const COLUMNS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h2" /> <path d="M4 10h5.5" /> <path d="M4 14h5.5" /> <path d="M4 18h5.5" /> <path d="M14.5 6h5.5" /> <path d="M14.5 10h5.5" /> <path d="M18 14h2" /> <path d="M14.5 18h3.5" /> <path d="M3 3l18 18" /> </svg>"##;
const CONTRACT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 21h-2a3 3 0 0 1 -3 -3v-1h5.5" /> <path d="M17 8.5v-3.5a2 2 0 1 1 2 2h-2" /> <path d="M19 3h-11a3 3 0 0 0 -3 3v11" /> <path d="M9 7h4" /> <path d="M9 11h4" /> <path d="M18.42 12.61a2.1 2.1 0 0 1 2.97 2.97l-6.39 6.42h-3v-3l6.42 -6.39" /> </svg>"##;
const COPY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9.667a2.667 2.667 0 0 1 2.667 -2.667h8.666a2.667 2.667 0 0 1 2.667 2.667v8.666a2.667 2.667 0 0 1 -2.667 2.667h-8.666a2.667 2.667 0 0 1 -2.667 -2.667l0 -8.666" /> <path d="M4.012 16.737a2.005 2.005 0 0 1 -1.012 -1.737v-10c0 -1.1 .9 -2 2 -2h10c.75 0 1.158 .385 1.5 1" /> </svg>"##;
const COPY_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9.667a2.667 2.667 0 0 1 2.667 -2.667h8.666a2.667 2.667 0 0 1 2.667 2.667v8.666a2.667 2.667 0 0 1 -2.667 2.667h-8.666a2.667 2.667 0 0 1 -2.667 -2.667l0 -8.666" /> <path d="M4.012 16.737a2 2 0 0 1 -1.012 -1.737v-10c0 -1.1 .9 -2 2 -2h10c.75 0 1.158 .385 1.5 1" /> <path d="M11 14l2 2l4 -4" /> </svg>"##;
const COPY_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9.667a2.667 2.667 0 0 1 2.667 -2.667h8.666a2.667 2.667 0 0 1 2.667 2.667v8.666a2.667 2.667 0 0 1 -2.667 2.667h-8.666a2.667 2.667 0 0 1 -2.667 -2.667l0 -8.666" /> <path d="M4.012 16.737a2 2 0 0 1 -1.012 -1.737v-10c0 -1.1 .9 -2 2 -2h10c.75 0 1.158 .385 1.5 1" /> <path d="M11 14h6" /> </svg>"##;
const COPY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.414 19.415a2 2 0 0 1 -1.414 .585h-8a2 2 0 0 1 -2 -2v-8c0 -.554 .225 -1.055 .589 -1.417m3.411 -.583h6a2 2 0 0 1 2 2v6" /> <path d="M16 8v-2a2 2 0 0 0 -2 -2h-6m-3.418 .59c-.36 .36 -.582 .86 -.582 1.41v8a2 2 0 0 0 2 2h2" /> <path d="M3 3l18 18" /> </svg>"##;
const COPY_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9.667a2.667 2.667 0 0 1 2.667 -2.667h8.666a2.667 2.667 0 0 1 2.667 2.667v8.666a2.667 2.667 0 0 1 -2.667 2.667h-8.666a2.667 2.667 0 0 1 -2.667 -2.667l0 -8.666" /> <path d="M4.012 16.737a2 2 0 0 1 -1.012 -1.737v-10c0 -1.1 .9 -2 2 -2h10c.75 0 1.158 .385 1.5 1" /> <path d="M11 14h6" /> <path d="M14 11v6" /> </svg>"##;
const COPY_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9.667a2.667 2.667 0 0 1 2.667 -2.667h8.666a2.667 2.667 0 0 1 2.667 2.667v8.666a2.667 2.667 0 0 1 -2.667 2.667h-8.666a2.667 2.667 0 0 1 -2.667 -2.667l0 -8.666" /> <path d="M4.012 16.737a2 2 0 0 1 -1.012 -1.737v-10c0 -1.1 .9 -2 2 -2h10c.75 0 1.158 .385 1.5 1" /> <path d="M11.5 11.5l4.9 5" /> <path d="M16.5 11.5l-5.1 5" /> </svg>"##;
const CURSOR_TEXT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h4" /> <path d="M9 4a3 3 0 0 1 3 3v10a3 3 0 0 1 -3 3" /> <path d="M15 4a3 3 0 0 0 -3 3v10a3 3 0 0 0 3 3" /> </svg>"##;
const DELTA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h16l-8 -16l-8 16" /> </svg>"##;
const EMPHASIS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 5h-8v10h8m-1 -5h-7" /> <path d="M6 20l0 .01" /> <path d="M10 20l0 .01" /> <path d="M14 20l0 .01" /> <path d="M18 20l0 .01" /> </svg>"##;
const ERASER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 20h-10.5l-4.21 -4.3a1 1 0 0 1 0 -1.41l10 -10a1 1 0 0 1 1.41 0l5 5a1 1 0 0 1 0 1.41l-9.2 9.3" /> <path d="M18 13.3l-6.3 -6.3" /> </svg>"##;
const ERASER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M19 20h-10.5l-4.21 -4.3a1 1 0 0 1 0 -1.41l5 -4.993m2.009 -2.01l3 -3a1 1 0 0 1 1.41 0l5 5a1 1 0 0 1 0 1.41c-1.417 1.431 -2.406 2.432 -2.97 3m-2.02 2.043l-4.211 4.256" /> <path d="M18 13.3l-6.3 -6.3" /> </svg>"##;
const FILE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> </svg>"##;
const FILE_3D_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M12 13.5l4 -1.5" /> <path d="M8 11.846l4 1.654v4.5l4 -1.846v-4.308l-4 -1.846l-4 1.846" /> <path d="M8 12v4.2l4 1.8" /> </svg>"##;
const FILE_AI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M10 21h-3a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M14 21v-4a2 2 0 1 1 4 0v4" /> <path d="M14 19h4" /> <path d="M21 15v6" /> </svg>"##;
const FILE_ALERT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M12 17l.01 0" /> <path d="M12 11l0 3" /> </svg>"##;
const FILE_ANALYTICS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M9 17l0 -5" /> <path d="M12 17l0 -1" /> <path d="M15 17l0 -3" /> </svg>"##;
const FILE_ARROW_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M15 15h-6" /> <path d="M11.5 17.5l-2.5 -2.5l2.5 -2.5" /> </svg>"##;
const FILE_ARROW_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M9 15h6" /> <path d="M12.5 17.5l2.5 -2.5l-2.5 -2.5" /> </svg>"##;
const FILE_BARCODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M8 13h1v3h-1l0 -3" /> <path d="M12 13v3" /> <path d="M15 13h1v3h-1l0 -3" /> </svg>"##;
const FILE_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M12 21h-5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v2" /> <path d="M17 21v-6m2 0v-1.5m0 9v-1.5m-2 -3h3m-1 0h.5a1.5 1.5 0 0 1 0 3h-3.5m3 -3h.5a1.5 1.5 0 0 0 0 -3h-3.5" /> </svg>"##;
const FILE_BROKEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 7v-2a2 2 0 0 1 2 -2h7l5 5v2" /> <path d="M19 19a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2" /> <path d="M5 16h.01" /> <path d="M5 13h.01" /> <path d="M5 10h.01" /> <path d="M19 13h.01" /> <path d="M19 16h.01" /> </svg>"##;
const FILE_CERTIFICATE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 8v-3a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2h-5" /> <path d="M3 14a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M4.5 17l-1.5 5l3 -1.5l3 1.5l-1.5 -5" /> </svg>"##;
const FILE_CHART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M12 10v4h4" /> <path d="M8 14a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> </svg>"##;
const FILE_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M9 15l2 2l4 -4" /> </svg>"##;
const FILE_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M10 13l-1 2l1 2" /> <path d="M14 13l1 2l-1 2" /> </svg>"##;
const FILE_CODE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h-1v5h1" /> <path d="M14 12h1v5h-1" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> </svg>"##;
const FILE_CV_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M11 12.5a1.5 1.5 0 0 0 -3 0v3a1.5 1.5 0 0 0 3 0" /> <path d="M13 11l1.5 6l1.5 -6" /> </svg>"##;
const FILE_DATABASE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12.75a4 1.75 0 1 0 8 0a4 1.75 0 1 0 -8 0" /> <path d="M8 12.5v3.75c0 .966 1.79 1.75 4 1.75s4 -.784 4 -1.75v-3.75" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> </svg>"##;
const FILE_DELTA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M9 17h6l-3 -6l-3 6" /> </svg>"##;
const FILE_DESCRIPTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M9 17h6" /> <path d="M9 13h6" /> </svg>"##;
const FILE_DIFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M12 10l0 4" /> <path d="M10 12l4 0" /> <path d="M10 17l4 0" /> </svg>"##;
const FILE_DIGIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M9 13a1 1 0 0 1 1 -1h1a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-1a1 1 0 0 1 -1 -1l0 -3" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M15 12v5" /> </svg>"##;
const FILE_DISLIKE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 15a1 1 0 0 1 1 -1h1a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-1a1 1 0 0 1 -1 -1l0 -3" /> <path d="M6 15a1 1 0 0 1 1 -1h3.756a1 1 0 0 1 .958 .713l1.2 3c.09 .303 .133 .63 -.056 .884c-.188 .254 -.542 .403 -.858 .403h-2v2.467a1.1 1.1 0 0 1 -2.015 .61l-1.985 -3.077v-4" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 11v-6a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2h-2.5" /> </svg>"##;
const FILE_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M14 11h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M12 17v1m0 -8v1" /> </svg>"##;
const FILE_DOTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M9 14v.01" /> <path d="M12 14v.01" /> <path d="M15 14v.01" /> </svg>"##;
const FILE_DOWNLOAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M12 17v-6" /> <path d="M9.5 14.5l2.5 2.5l2.5 -2.5" /> </svg>"##;
const FILE_EURO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M12 14h-3" /> <path d="M14 11.172a3 3 0 1 0 0 5.656" /> </svg>"##;
const FILE_EXCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M10 12l4 5" /> <path d="M10 17l4 -5" /> </svg>"##;
const FILE_EXPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M11.5 21h-4.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v5m-5 6h7m-3 -3l3 3l-3 3" /> </svg>"##;
const FILE_FUNCTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M10.5 17h.333c.474 0 .87 -.323 .916 -.746l.502 -4.508c.047 -.423 .443 -.746 .916 -.746h.333" /> <path d="M10.5 14h3" /> </svg>"##;
const FILE_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 5v4a1 1 0 0 0 1 1h4" /> <path d="M3 7v10a2 2 0 0 0 2 2h14a2 2 0 0 0 2 -2v-7l-5 -5h-11a2 2 0 0 0 -2 2" /> </svg>"##;
const FILE_IMPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 13v-8a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2h-5.5m-9.5 -2h7m-3 -3l3 3l-3 3" /> </svg>"##;
const FILE_INFINITY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.536 17.586a2.123 2.123 0 0 0 -2.929 0a1.951 1.951 0 0 0 0 2.828c.809 .781 2.12 .781 2.929 0c.809 -.781 -.805 .778 0 0l1.46 -1.41l1.46 -1.419" /> <path d="M15.54 17.582l1.46 1.42l1.46 1.41c.809 .78 -.805 -.779 0 0s2.12 .781 2.929 0a1.951 1.951 0 0 0 0 -2.828a2.123 2.123 0 0 0 -2.929 0" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M9.5 21h-2.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v6" /> </svg>"##;
const FILE_INFO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M11 14h1v4h1" /> <path d="M12 11h.01" /> </svg>"##;
const FILE_INVOICE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M9 7l1 0" /> <path d="M9 13l6 0" /> <path d="M13 17l2 0" /> </svg>"##;
const FILE_ISR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 3v4a1 1 0 0 0 1 1h4" /> <path d="M15 3v4a1 1 0 0 0 1 1h4" /> <path d="M6 8v-3a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-7" /> <path d="M3 15l3 -3l3 3" /> </svg>"##;
const FILE_LAMBDA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M10 17l2 -3" /> <path d="M15 17c-2.5 0 -2.5 -6 -5 -6" /> </svg>"##;
const FILE_LIKE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a1 1 0 0 1 1 -1h1a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-1a1 1 0 0 1 -1 -1l0 -3" /> <path d="M6 20a1 1 0 0 0 1 1h3.756a1 1 0 0 0 .958 -.713l1.2 -3c.09 -.303 .133 -.63 -.056 -.884c-.188 -.254 -.542 -.403 -.858 -.403h-2v-2.467a1.1 1.1 0 0 0 -2.015 -.61l-1.985 3.077v4" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12.1v-7.1a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2h-2.3" /> </svg>"##;
const FILE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M9 14l6 0" /> </svg>"##;
const FILE_MUSIC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M10 16a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M12 16l0 -5l2 1" /> </svg>"##;
const FILE_NEUTRAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2m-7 -7h.01m3.99 0h.01m-4.01 3h4" /> </svg>"##;
const FILE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M7 3h7l5 5v7m0 4a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const FILE_ORIENTATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M10 21h-3a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v2" /> <path d="M13 20h5a2 2 0 0 0 2 -2v-5" /> <path d="M15 22l-2 -2l2 -2" /> <path d="M18 15l2 -2l2 2" /> </svg>"##;
const FILE_PENCIL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M10 18l5 -5a1.414 1.414 0 0 0 -2 -2l-5 5v2h2" /> </svg>"##;
const FILE_PERCENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 17l4 -4" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M10 13h.01" /> <path d="M14 17h.01" /> </svg>"##;
const FILE_PHONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M9 12a.5 .5 0 0 0 1 0v-1a.5 .5 0 0 0 -1 0v1a5 5 0 0 0 5 5h1a.5 .5 0 0 0 0 -1h-1a.5 .5 0 0 0 0 1" /> </svg>"##;
const FILE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M12 11l0 6" /> <path d="M9 14l6 0" /> </svg>"##;
const FILE_POWER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M12 11l-2 3h4l-2 3" /> </svg>"##;
const FILE_REPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 17a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M17 13v4h4" /> <path d="M12 3v4a1 1 0 0 0 1 1h4" /> <path d="M11.5 21h-6.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v2m0 3v4" /> </svg>"##;
const FILE_RSS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M12 17a3 3 0 0 0 -3 -3" /> <path d="M15 17a6 6 0 0 0 -6 -6" /> <path d="M9 17h.01" /> </svg>"##;
const FILE_SAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2m-7 -7h.01m3.99 0h.01" /> <path d="M10 18a3.5 3.5 0 0 1 4 0" /> </svg>"##;
const FILE_SCISSORS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M14 17a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M8 17a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M9 17l6 -6" /> <path d="M15 17l-6 -6" /> </svg>"##;
const FILE_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M12 21h-5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v4.5" /> <path d="M14 17.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> <path d="M18.5 19.5l2.5 2.5" /> </svg>"##;
const FILE_SETTINGS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 14a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M12 10.5v1.5" /> <path d="M12 16v1.5" /> <path d="M15.031 12.25l-1.299 .75" /> <path d="M10.268 15l-1.3 .75" /> <path d="M15 15.803l-1.285 -.773" /> <path d="M10.285 12.97l-1.285 -.773" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> </svg>"##;
const FILE_SHREDDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M3 12l18 0" /> <path d="M6 16l0 2" /> <path d="M10 16l0 6" /> <path d="M14 16l0 2" /> <path d="M18 16l0 4" /> </svg>"##;
const FILE_SIGNAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M12 14v.01" /> <path d="M9.525 11.525a3.5 3.5 0 0 0 0 4.95m4.95 0a3.5 3.5 0 0 0 0 -4.95" /> </svg>"##;
const FILE_SMILE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2m-7 -7h.01m3.99 0h.01" /> <path d="M10 17a3.5 3.5 0 0 0 4 0" /> </svg>"##;
const FILE_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M12 21h-5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v3.5" /> </svg>"##;
const FILE_SPREADSHEET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M8 11h8v7h-8l0 -7" /> <path d="M8 15h8" /> <path d="M11 11v7" /> </svg>"##;
const FILE_STACK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M5 21h14" /> <path d="M5 18h14" /> <path d="M5 15h14" /> </svg>"##;
const FILE_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M11.8 16.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const FILE_SYMLINK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 21v-4a3 3 0 0 1 3 -3h5" /> <path d="M9 17l3 -3l-3 -3" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 11v-6a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2h-9.5" /> </svg>"##;
const FILE_TEXT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M9 9l1 0" /> <path d="M9 13l6 0" /> <path d="M9 17l6 0" /> </svg>"##;
const FILE_TEXT_AI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M10 21h-3a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v3.5" /> <path d="M9 9h1" /> <path d="M9 13h2.5" /> <path d="M9 17h1" /> <path d="M14 21v-4a2 2 0 1 1 4 0v4" /> <path d="M14 19h4" /> <path d="M21 15v6" /> </svg>"##;
const FILE_TEXT_SHIELD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 3v4a.997 .997 0 0 0 1 1h4" /> <path d="M11 21h-5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v3.5" /> <path d="M8 9h1" /> <path d="M8 12.994l3 0" /> <path d="M8 16.997l2 0" /> <path d="M21 15.994c0 4 -2.5 6 -3.5 6s-3.5 -2 -3.5 -6c1 0 2.5 -.5 3.5 -1.5c1 1 2.5 1.5 3.5 1.5" /> </svg>"##;
const FILE_TEXT_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M12 21h-5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v3.5" /> <path d="M9 9h1" /> <path d="M9 13h6" /> <path d="M9 17h3" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const FILE_TIME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M8 14a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M12 12.496v1.504l1 1" /> </svg>"##;
const FILE_TYPE_BMP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M18 18h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6" /> <path d="M4 21h1.5a1.5 1.5 0 0 0 0 -3h-1.5h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6" /> <path d="M10 21v-6l2.5 3l2.5 -3v6" /> </svg>"##;
const FILE_TYPE_CSS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M8 16.5a1.5 1.5 0 0 0 -3 0v3a1.5 1.5 0 0 0 3 0" /> <path d="M11 20.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1.25a.75 .75 0 0 1 .75 .75" /> <path d="M17 20.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1.25a.75 .75 0 0 1 .75 .75" /> </svg>"##;
const FILE_TYPE_CSV_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M7 16.5a1.5 1.5 0 0 0 -3 0v3a1.5 1.5 0 0 0 3 0" /> <path d="M10 20.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1.25a.75 .75 0 0 1 .75 .75" /> <path d="M16 15l2 6l2 -6" /> </svg>"##;
const FILE_TYPE_DOC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M5 15v6h1a2 2 0 0 0 2 -2v-2a2 2 0 0 0 -2 -2h-1" /> <path d="M20 16.5a1.5 1.5 0 0 0 -3 0v3a1.5 1.5 0 0 0 3 0" /> <path d="M12.5 15a1.5 1.5 0 0 1 1.5 1.5v3a1.5 1.5 0 0 1 -3 0v-3a1.5 1.5 0 0 1 1.5 -1.5" /> </svg>"##;
const FILE_TYPE_DOCX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M2 15v6h1a2 2 0 0 0 2 -2v-2a2 2 0 0 0 -2 -2h-1" /> <path d="M17 16.5a1.5 1.5 0 0 0 -3 0v3a1.5 1.5 0 0 0 3 0" /> <path d="M9.5 15a1.5 1.5 0 0 1 1.5 1.5v3a1.5 1.5 0 0 1 -3 0v-3a1.5 1.5 0 0 1 1.5 -1.5" /> <path d="M19.5 15l3 6" /> <path d="M19.5 21l3 -6" /> </svg>"##;
const FILE_TYPE_HTML_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M2 21v-6" /> <path d="M5 15v6" /> <path d="M2 18h3" /> <path d="M20 15v6h2" /> <path d="M13 21v-6l2 3l2 -3v6" /> <path d="M7.5 15h3" /> <path d="M9 15v6" /> </svg>"##;
const FILE_TYPE_JPG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M11 18h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6" /> <path d="M20 15h-1a2 2 0 0 0 -2 2v2a2 2 0 0 0 2 2h1v-3" /> <path d="M5 15h3v4.5a1.5 1.5 0 0 1 -3 0" /> </svg>"##;
const FILE_TYPE_JS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M3 15h3v4.5a1.5 1.5 0 0 1 -3 0" /> <path d="M9 20.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1.25a.75 .75 0 0 1 .75 .75" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2h-1" /> </svg>"##;
const FILE_TYPE_JSX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M4 15h3v4.5a1.5 1.5 0 0 1 -3 0" /> <path d="M10 20.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1.25a.75 .75 0 0 1 .75 .75" /> <path d="M16 15l4 6" /> <path d="M16 21l4 -6" /> </svg>"##;
const FILE_TYPE_PDF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M5 18h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6" /> <path d="M17 18h2" /> <path d="M20 15h-3v6" /> <path d="M11 15v6h1a2 2 0 0 0 2 -2v-2a2 2 0 0 0 -2 -2h-1" /> </svg>"##;
const FILE_TYPE_PHP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M5 18h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6" /> <path d="M17 18h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6" /> <path d="M11 21v-6" /> <path d="M14 15v6" /> <path d="M11 18h3" /> </svg>"##;
const FILE_TYPE_PNG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M20 15h-1a2 2 0 0 0 -2 2v2a2 2 0 0 0 2 2h1v-3" /> <path d="M5 18h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6" /> <path d="M11 21v-6l3 6v-6" /> </svg>"##;
const FILE_TYPE_PPT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 18h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6" /> <path d="M11 18h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6" /> <path d="M16.5 15h3" /> <path d="M18 15v6" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> </svg>"##;
const FILE_TYPE_RS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M9 20.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1.25a.75 .75 0 0 1 .75 .75" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2h-1" /> <path d="M3 18h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6m3 0l-2 -3" /> </svg>"##;
const FILE_TYPE_SQL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 20.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1.25a.75 .75 0 0 1 .75 .75" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M18 15v6h2" /> <path d="M13 15a2 2 0 0 1 2 2v2a2 2 0 1 1 -4 0v-2a2 2 0 0 1 2 -2" /> <path d="M14 20l1.5 1.5" /> </svg>"##;
const FILE_TYPE_SVG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M4 20.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1.25a.75 .75 0 0 1 .75 .75" /> <path d="M10 15l2 6l2 -6" /> <path d="M20 15h-1a2 2 0 0 0 -2 2v2a2 2 0 0 0 2 2h1v-3" /> </svg>"##;
const FILE_TYPE_TS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2h-1" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M9 20.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1.25a.75 .75 0 0 1 .75 .75" /> <path d="M3.5 15h3" /> <path d="M5 15v6" /> </svg>"##;
const FILE_TYPE_TSX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M16 15l4 6" /> <path d="M16 21l4 -6" /> <path d="M10 20.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1.25a.75 .75 0 0 1 .75 .75" /> <path d="M4.5 15h3" /> <path d="M6 15v6" /> </svg>"##;
const FILE_TYPE_TXT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M16.5 15h3" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M4.5 15h3" /> <path d="M6 15v6" /> <path d="M18 15v6" /> <path d="M10 15l4 6" /> <path d="M10 21l4 -6" /> </svg>"##;
const FILE_TYPE_VUE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M4 15l2 6l2 -6" /> <path d="M11 15v4.5a1.5 1.5 0 0 0 3 0v-4.5" /> <path d="M20 15h-3v6h3" /> <path d="M17 18h2" /> </svg>"##;
const FILE_TYPE_XLS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M4 15l4 6" /> <path d="M4 21l4 -6" /> <path d="M17 20.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1.25a.75 .75 0 0 1 .75 .75" /> <path d="M11 15v6h3" /> </svg>"##;
const FILE_TYPE_XML_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M4 15l4 6" /> <path d="M4 21l4 -6" /> <path d="M19 15v6h3" /> <path d="M11 21v-6l2.5 3l2.5 -3v6" /> </svg>"##;
const FILE_TYPE_ZIP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4" /> <path d="M16 18h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6" /> <path d="M12 15v6" /> <path d="M5 15h3l-3 6h3" /> </svg>"##;
const FILE_TYPOGRAPHY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M11 18h2" /> <path d="M12 18v-7" /> <path d="M9 12v-1h6v1" /> </svg>"##;
const FILE_UNKNOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M12 17v.01" /> <path d="M12 14a1.5 1.5 0 1 0 -1.14 -2.474" /> </svg>"##;
const FILE_UPLOAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M12 11v6" /> <path d="M9.5 13.5l2.5 -2.5l2.5 2.5" /> </svg>"##;
const FILE_VECTOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M8 16.5a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> <path d="M13 12.5a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M9.5 15a2.5 2.5 0 0 1 2.5 -2.5h1" /> </svg>"##;
const FILE_WORD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M9 12l1.333 5l1.667 -4l1.667 4l1.333 -5" /> </svg>"##;
const FILE_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" /> <path d="M10 12l4 4m0 -4l-4 4" /> </svg>"##;
const FILE_ZIP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 20.735a2 2 0 0 1 -1 -1.735v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2h-1" /> <path d="M11 17a2 2 0 0 1 2 2v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-2a2 2 0 0 1 2 -2" /> <path d="M11 5l-1 0" /> <path d="M13 7l-1 0" /> <path d="M11 9l-1 0" /> <path d="M13 11l-1 0" /> <path d="M11 13l-1 0" /> <path d="M13 15l-1 0" /> </svg>"##;
const FILES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 3v4a1 1 0 0 0 1 1h4" /> <path d="M18 17h-7a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h4l5 5v7a2 2 0 0 1 -2 2" /> <path d="M16 17v2a2 2 0 0 1 -2 2h-7a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h2" /> </svg>"##;
const FILES_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 3v4a1 1 0 0 0 1 1h4" /> <path d="M17 17h-6a2 2 0 0 1 -2 -2v-6m0 -4a2 2 0 0 1 2 -2h4l5 5v7c0 .294 -.063 .572 -.177 .823" /> <path d="M16 17v2a2 2 0 0 1 -2 2h-7a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2" /> <path d="M3 3l18 18" /> </svg>"##;
const FLOAT_CENTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 6a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M4 7l1 0" /> <path d="M4 11l1 0" /> <path d="M19 7l1 0" /> <path d="M19 11l1 0" /> <path d="M4 15l16 0" /> <path d="M4 19l16 0" /> </svg>"##;
const FLOAT_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M14 7l6 0" /> <path d="M14 11l6 0" /> <path d="M4 15l16 0" /> <path d="M4 19l16 0" /> </svg>"##;
const FLOAT_NONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M4 15l16 0" /> <path d="M4 19l16 0" /> </svg>"##;
const FLOAT_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 6a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M4 7l6 0" /> <path d="M4 11l6 0" /> <path d="M4 15l16 0" /> <path d="M4 19l16 0" /> </svg>"##;
const FOLDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h4l3 3h7a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2" /> </svg>"##;
const FOLDER_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19h-8a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v3.5" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const FOLDER_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-7a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v3" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const FOLDER_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 19h-6a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v4" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const FOLDER_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 19h-6a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v4" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const FOLDER_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 19h-7.5a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v3" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const FOLDER_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 19h-8.5a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v1.5" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const FOLDER_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-7a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v3.5" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const FOLDER_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 19h-10a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v3.5" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const FOLDER_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.5 19h-5.5a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v2" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const FOLDER_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-7a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v6" /> <path d="M16 19h6" /> </svg>"##;
const FOLDER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h1l3 3h7a2 2 0 0 1 2 2v8m-2 2h-14a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 1.189 -1.829" /> <path d="M3 3l18 18" /> </svg>"##;
const FOLDER_OPEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 19l2.757 -7.351a1 1 0 0 1 .936 -.649h12.307a1 1 0 0 1 .986 1.164l-.996 5.211a2 2 0 0 1 -1.964 1.625h-14.026a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v2" /> </svg>"##;
const FOLDER_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19h-8a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v4" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const FOLDER_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-7a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v2.5" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const FOLDER_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-7a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v3.5" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const FOLDER_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 19h-10a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v2.5" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const FOLDER_ROOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 13a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M12 15v4" /> <path d="M5 4h4l3 3h7a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2" /> </svg>"##;
const FOLDER_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 19h-6a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v2.5" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const FOLDER_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19h-8a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v4" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const FOLDER_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 19h-5a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v2.5" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const FOLDER_SYMLINK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21v-4a3 3 0 0 1 3 -3h5" /> <path d="M8 17l3 -3l-3 -3" /> <path d="M3 11v-5a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8" /> </svg>"##;
const FOLDER_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-7a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v3.5" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const FOLDER_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 19h-8.5a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v4" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const FOLDERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 3h3l2 2h5a2 2 0 0 1 2 2v7a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2" /> <path d="M17 16v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h2" /> </svg>"##;
const FOLDERS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 17h-8a2 2 0 0 1 -2 -2v-8m1.177 -2.823c.251 -.114 .53 -.177 .823 -.177h3l2 2h5a2 2 0 0 1 2 2v7c0 .55 -.223 1.05 -.583 1.411" /> <path d="M17 17v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h2" /> <path d="M3 3l18 18" /> </svg>"##;
const FORMS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a3 3 0 0 0 -3 3v12a3 3 0 0 0 3 3" /> <path d="M6 3a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3" /> <path d="M13 7h7a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-7" /> <path d="M5 7h-1a1 1 0 0 0 -1 1v8a1 1 0 0 0 1 1h1" /> <path d="M17 12h.01" /> <path d="M13 12h.01" /> </svg>"##;
const H_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 18v-8l-2 2" /> <path d="M4 6v12" /> <path d="M12 6v12" /> <path d="M11 18h2" /> <path d="M3 18h2" /> <path d="M4 12h8" /> <path d="M3 6h2" /> <path d="M11 6h2" /> </svg>"##;
const H_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 12a2 2 0 1 1 4 0c0 .591 -.417 1.318 -.816 1.858l-3.184 4.143l4 0" /> <path d="M4 6v12" /> <path d="M12 6v12" /> <path d="M11 18h2" /> <path d="M3 18h2" /> <path d="M4 12h8" /> <path d="M3 6h2" /> <path d="M11 6h2" /> </svg>"##;
const H_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 14a2 2 0 1 0 -2 -2" /> <path d="M17 16a2 2 0 1 0 2 -2" /> <path d="M4 6v12" /> <path d="M12 6v12" /> <path d="M11 18h2" /> <path d="M3 18h2" /> <path d="M4 12h8" /> <path d="M3 6h2" /> <path d="M11 6h2" /> </svg>"##;
const H_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 18v-8l-4 6h5" /> <path d="M4 6v12" /> <path d="M12 6v12" /> <path d="M11 18h2" /> <path d="M3 18h2" /> <path d="M4 12h8" /> <path d="M3 6h2" /> <path d="M11 6h2" /> </svg>"##;
const H_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 18h2a2 2 0 1 0 0 -4h-2v-4h4" /> <path d="M4 6v12" /> <path d="M12 6v12" /> <path d="M11 18h2" /> <path d="M3 18h2" /> <path d="M4 12h8" /> <path d="M3 6h2" /> <path d="M11 6h2" /> </svg>"##;
const H_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 14a2 2 0 1 0 0 4a2 2 0 0 0 0 -4" /> <path d="M21 12a2 2 0 1 0 -4 0v4" /> <path d="M4 6v12" /> <path d="M12 6v12" /> <path d="M11 18h2" /> <path d="M3 18h2" /> <path d="M4 12h8" /> <path d="M3 6h2" /> <path d="M11 6h2" /> </svg>"##;
const HEADING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 12h10" /> <path d="M7 5v14" /> <path d="M17 5v14" /> <path d="M15 19h4" /> <path d="M15 5h4" /> <path d="M5 19h4" /> <path d="M5 5h4" /> </svg>"##;
const HEADING_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 12h5m4 0h1" /> <path d="M7 7v12" /> <path d="M17 5v8m0 4v2" /> <path d="M15 19h4" /> <path d="M15 5h4" /> <path d="M5 19h4" /> <path d="M3 3l18 18" /> </svg>"##;
const HEXAGON_LETTER_A_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 16v-6a2 2 0 1 1 4 0v6" /> <path d="M10 13h4" /> </svg>"##;
const HEXAGON_LETTER_B_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 16h2a2 2 0 1 0 0 -4h-2h2a2 2 0 1 0 0 -4h-2v8" /> </svg>"##;
const HEXAGON_LETTER_C_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M14 10a2 2 0 1 0 -4 0v4a2 2 0 1 0 4 0" /> </svg>"##;
const HEXAGON_LETTER_D_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2h-2" /> </svg>"##;
const HEXAGON_LETTER_E_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M14 8h-4v8h4" /> <path d="M10 12h2.5" /> </svg>"##;
const HEXAGON_LETTER_F_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 12h3" /> <path d="M14 8h-4v8" /> </svg>"##;
const HEXAGON_LETTER_G_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M14 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> </svg>"##;
const HEXAGON_LETTER_H_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 16v-8m4 0v8" /> <path d="M10 12h4" /> </svg>"##;
const HEXAGON_LETTER_I_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M12 8v8" /> </svg>"##;
const HEXAGON_LETTER_J_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 8h4v6a2 2 0 1 1 -4 0" /> </svg>"##;
const HEXAGON_LETTER_K_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 8v8" /> <path d="M14 8l-2.5 4l2.5 4" /> <path d="M10 12h1.5" /> </svg>"##;
const HEXAGON_LETTER_L_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 8v8h4" /> </svg>"##;
const HEXAGON_LETTER_M_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M9 16v-8l3 5l3 -5v8" /> </svg>"##;
const HEXAGON_LETTER_N_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 16v-8l4 8v-8" /> </svg>"##;
const HEXAGON_LETTER_O_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> </svg>"##;
const HEXAGON_LETTER_P_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8" /> </svg>"##;
const HEXAGON_LETTER_Q_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M13 15l1 1" /> </svg>"##;
const HEXAGON_LETTER_R_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8m4 0l-3 -4" /> </svg>"##;
const HEXAGON_LETTER_S_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1" /> </svg>"##;
const HEXAGON_LETTER_T_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 8h4" /> <path d="M12 8v8" /> </svg>"##;
const HEXAGON_LETTER_U_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 8v6a2 2 0 1 0 4 0v-6" /> </svg>"##;
const HEXAGON_LETTER_V_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 8l2 8l2 -8" /> </svg>"##;
const HEXAGON_LETTER_W_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M9 8l1 8l2 -5l2 5l1 -8" /> </svg>"##;
const HEXAGON_LETTER_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 8l4 8" /> <path d="M10 16l4 -8" /> </svg>"##;
const HEXAGON_LETTER_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 8l2 5l2 -5" /> <path d="M12 16v-3" /> </svg>"##;
const HEXAGON_LETTER_Z_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 8h4l-4 8h4" /> </svg>"##;
const HEXAGON_NUMBER_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> </svg>"##;
const HEXAGON_NUMBER_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 10l2 -2v8" /> </svg>"##;
const HEXAGON_NUMBER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const HEXAGON_NUMBER_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 9a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1" /> </svg>"##;
const HEXAGON_NUMBER_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 8v3a1 1 0 0 0 1 1h3" /> <path d="M14 8v8" /> </svg>"##;
const HEXAGON_NUMBER_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const HEXAGON_NUMBER_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M14 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const HEXAGON_NUMBER_7_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.02 6.858a2 2 0 0 1 1 1.752v6.555c0 .728 -.395 1.4 -1.032 1.753l-6.017 3.844a2 2 0 0 1 -1.948 0l-6.016 -3.844a2 2 0 0 1 -1.032 -1.752v-6.556c0 -.728 .395 -1.4 1.032 -1.753l6.017 -3.582a2.062 2.062 0 0 1 2 0l6.017 3.583h-.029l.008 0" /> <path d="M10 8h4l-2 8" /> </svg>"##;
const HEXAGON_NUMBER_8_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M12 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const HEXAGON_NUMBER_9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const HIGHLIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M12.5 5.5l4 4" /> <path d="M4.5 13.5l4 4" /> <path d="M21 15v4h-8l4 -4l4 0" /> </svg>"##;
const HIGHLIGHT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 9l-6 6v4h4l6 -6m2 -2l2.503 -2.503a2.828 2.828 0 1 0 -4 -4l-2.497 2.497" /> <path d="M12.5 5.5l4 4" /> <path d="M4.5 13.5l4 4" /> <path d="M19 15h2v2m-2 2h-6l3 -3" /> <path d="M3 3l18 18" /> </svg>"##;
const INDENT_DECREASE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 6l-7 0" /> <path d="M20 12l-9 0" /> <path d="M20 18l-7 0" /> <path d="M8 8l-4 4l4 4" /> </svg>"##;
const INDENT_INCREASE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 6l-11 0" /> <path d="M20 12l-7 0" /> <path d="M20 18l-11 0" /> <path d="M4 8l4 4l-4 4" /> </svg>"##;
const INPUT_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 11v-2a2 2 0 0 0 -2 -2h-12a2 2 0 0 0 -2 2v5a2 2 0 0 0 2 2h5" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const INVOICE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M19 12v7a1.78 1.78 0 0 1 -3.1 1.4a1.65 1.65 0 0 0 -2.6 0a1.65 1.65 0 0 1 -2.6 0a1.65 1.65 0 0 0 -2.6 0a1.78 1.78 0 0 1 -3.1 -1.4v-14a2 2 0 0 1 2 -2h7l5 5v4.25" /> </svg>"##;
const ITALIC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 5l6 0" /> <path d="M7 19l6 0" /> <path d="M14 5l-4 14" /> </svg>"##;
const KERNING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 15v-3.5a2.5 2.5 0 1 1 5 0v3.5m0 -2h-5" /> <path d="M3 9l3 6l3 -6" /> <path d="M9 20l6 -16" /> </svg>"##;
const LAMBDA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 20l6.5 -9" /> <path d="M19 20c-6 0 -6 -16 -12 -16" /> </svg>"##;
const LANGUAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 6.371c0 4.418 -2.239 6.629 -5 6.629" /> <path d="M4 6.371h7" /> <path d="M5 9c0 2.144 2.252 3.908 6 4" /> <path d="M12 20l4 -9l4 9" /> <path d="M19.1 18h-6.2" /> <path d="M6.694 3l.793 .582" /> </svg>"##;
const LANGUAGE_HIRAGANA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5h7" /> <path d="M7 4c0 4.846 0 7 .5 8" /> <path d="M10 8.5c0 2.286 -2 4.5 -3.5 4.5s-2.5 -1.135 -2.5 -2c0 -2 1 -3 3 -3s5 .57 5 2.857c0 1.524 -.667 2.571 -2 3.143" /> <path d="M12 20l4 -9l4 9" /> <path d="M19.1 18h-6.2" /> </svg>"##;
const LANGUAGE_KATAKANA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5h6.586a1 1 0 0 1 .707 1.707l-1.293 1.293" /> <path d="M8 8c0 1.5 .5 3 -2 5" /> <path d="M12 20l4 -9l4 9" /> <path d="M19.1 18h-6.2" /> </svg>"##;
const LANGUAGE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l2.463 -5.541m1.228 -2.764l.309 -.695l.8 1.8" /> <path d="M18 18h-5.1" /> <path d="M8.747 8.748c-.66 2.834 -2.536 4.252 -4.747 4.252" /> <path d="M4 6.371l2.371 0" /> <path d="M5 9c0 2.144 2.252 3.908 6 4" /> <path d="M3 3l18 18" /> </svg>"##;
const LETTER_A_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 20v-12a4 4 0 0 1 4 -4h2a4 4 0 0 1 4 4v12" /> <path d="M7 13l10 0" /> </svg>"##;
const LETTER_A_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-6a2 2 0 1 1 4 0v6" /> <path d="M10 13h4" /> </svg>"##;
const LETTER_B_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 20v-16h6a4 4 0 0 1 0 8a4 4 0 0 1 0 8h-6" /> <path d="M7 12l6 0" /> </svg>"##;
const LETTER_B_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16h2a2 2 0 1 0 0 -4h-2h2a2 2 0 1 0 0 -4h-2v8" /> </svg>"##;
const LETTER_C_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9a5 5 0 0 0 -5 -5h-2a5 5 0 0 0 -5 5v6a5 5 0 0 0 5 5h2a5 5 0 0 0 5 -5" /> </svg>"##;
const LETTER_C_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10a2 2 0 1 0 -4 0v4a2 2 0 1 0 4 0" /> </svg>"##;
const LETTER_CASE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15.5a3.5 3.5 0 1 0 7 0a3.5 3.5 0 1 0 -7 0" /> <path d="M3 19v-10.5a3.5 3.5 0 0 1 7 0v10.5" /> <path d="M3 13h7" /> <path d="M21 12v7" /> </svg>"##;
const LETTER_CASE_LOWER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 15.5a3.5 3.5 0 1 0 7 0a3.5 3.5 0 1 0 -7 0" /> <path d="M10 12v7" /> <path d="M14 15.5a3.5 3.5 0 1 0 7 0a3.5 3.5 0 1 0 -7 0" /> <path d="M21 12v7" /> </svg>"##;
const LETTER_CASE_TOGGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 15.5a3.5 3.5 0 1 0 7 0a3.5 3.5 0 1 0 -7 0" /> <path d="M14 19v-10.5a3.5 3.5 0 0 1 7 0v10.5" /> <path d="M14 13h7" /> <path d="M10 12v7" /> </svg>"##;
const LETTER_CASE_UPPER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19v-10.5a3.5 3.5 0 0 1 7 0v10.5" /> <path d="M3 13h7" /> <path d="M14 19v-10.5a3.5 3.5 0 0 1 7 0v10.5" /> <path d="M14 13h7" /> </svg>"##;
const LETTER_D_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 4h6a5 5 0 0 1 5 5v6a5 5 0 0 1 -5 5h-6v-16" /> </svg>"##;
const LETTER_D_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2h-2" /> </svg>"##;
const LETTER_E_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 4h-10v16h10" /> <path d="M7 12l8 0" /> </svg>"##;
const LETTER_E_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h-4v8h4" /> <path d="M10 12h2.5" /> </svg>"##;
const LETTER_F_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 4h-10v16" /> <path d="M7 12l8 0" /> </svg>"##;
const LETTER_F_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h3" /> <path d="M14 8h-4v8" /> </svg>"##;
const LETTER_G_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9a5 5 0 0 0 -5 -5h-2a5 5 0 0 0 -5 5v6a5 5 0 0 0 5 5h2a5 5 0 0 0 5 -5v-2h-4" /> </svg>"##;
const LETTER_G_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> </svg>"##;
const LETTER_H_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 4l0 16" /> <path d="M7 12l10 0" /> <path d="M7 4l0 16" /> </svg>"##;
const LETTER_H_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-8" /> <path d="M14 8v8" /> <path d="M10 12h4" /> </svg>"##;
const LETTER_I_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4l0 16" /> </svg>"##;
const LETTER_I_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8v8" /> </svg>"##;
const LETTER_J_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 4v12a4 4 0 0 1 -4 4h-2a4 4 0 0 1 -4 -4" /> </svg>"##;
const LETTER_J_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4v6a2 2 0 1 1 -4 0" /> </svg>"##;
const LETTER_K_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 4l0 16" /> <path d="M7 12h2l8 -8" /> <path d="M9 12l8 8" /> </svg>"##;
const LETTER_K_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.5 8v8" /> <path d="M14.5 8l-3 4l3 4" /> <path d="M10.5 12h1" /> </svg>"##;
const LETTER_L_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 4v16h10" /> </svg>"##;
const LETTER_L_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v8h4" /> </svg>"##;
const LETTER_M_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 20v-16l6 14l6 -14v16" /> </svg>"##;
const LETTER_M_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 16v-8l3 5l3 -5v8" /> </svg>"##;
const LETTER_N_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 20v-16l10 16v-16" /> </svg>"##;
const LETTER_N_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-8l4 8v-8" /> </svg>"##;
const LETTER_O_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9a5 5 0 0 0 -5 -5h-2a5 5 0 0 0 -5 5v6a5 5 0 0 0 5 5h2a5 5 0 0 0 5 -5v-6" /> </svg>"##;
const LETTER_O_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> </svg>"##;
const LETTER_P_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 20v-16h5.5a4 4 0 0 1 0 9h-5.5" /> </svg>"##;
const LETTER_P_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8" /> </svg>"##;
const LETTER_Q_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9a5 5 0 0 0 -5 -5h-2a5 5 0 0 0 -5 5v6a5 5 0 0 0 5 5h2a5 5 0 0 0 5 -5v-6" /> <path d="M13 15l5 5" /> </svg>"##;
const LETTER_Q_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M13 15l1 1" /> </svg>"##;
const LETTER_R_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 20v-16h5.5a4 4 0 0 1 0 9h-5.5" /> <path d="M12 13l5 7" /> </svg>"##;
const LETTER_R_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8m4 0l-3 -4" /> </svg>"##;
const LETTER_S_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 8a4 4 0 0 0 -4 -4h-2a4 4 0 0 0 0 8h2a4 4 0 0 1 0 8h-2a4 4 0 0 1 -4 -4" /> </svg>"##;
const LETTER_S_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1" /> </svg>"##;
const LETTER_SPACING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12v-5.5a2.5 2.5 0 0 1 5 0v5.5m0 -4h-5" /> <path d="M13 4l3 8l3 -8" /> <path d="M5 18h14" /> <path d="M17 20l2 -2l-2 -2" /> <path d="M7 16l-2 2l2 2" /> </svg>"##;
const LETTER_T_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4l12 0" /> <path d="M12 4l0 16" /> </svg>"##;
const LETTER_T_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4" /> <path d="M12 8v8" /> </svg>"##;
const LETTER_U_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4v11a5 5 0 0 0 5 5h2a5 5 0 0 0 5 -5v-11" /> </svg>"##;
const LETTER_U_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v6a2 2 0 1 0 4 0v-6" /> </svg>"##;
const LETTER_V_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4l6 16l6 -16" /> </svg>"##;
const LETTER_V_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8l2 8l2 -8" /> </svg>"##;
const LETTER_W_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4l4 16l4 -14l4 14l4 -16" /> </svg>"##;
const LETTER_W_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 8l1 8l2 -5l2 5l1 -8" /> </svg>"##;
const LETTER_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 4l10 16" /> <path d="M17 4l-10 16" /> </svg>"##;
const LETTER_X_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8l4 8" /> <path d="M10 16l4 -8" /> </svg>"##;
const LETTER_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 4l5 9l5 -9" /> <path d="M12 13l0 7" /> </svg>"##;
const LETTER_Y_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8l2 5l2 -5" /> <path d="M12 16v-3" /> </svg>"##;
const LETTER_Z_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 4h10l-10 16h10" /> </svg>"##;
const LETTER_Z_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4l-4 8h4" /> </svg>"##;
const LICENSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21h-9a3 3 0 0 1 -3 -3v-1h10v2a2 2 0 0 0 4 0v-14a2 2 0 1 1 2 2h-2m2 -4h-11a3 3 0 0 0 -3 3v11" /> <path d="M9 7l4 0" /> <path d="M9 11l4 0" /> </svg>"##;
const LICENSE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21h-9a3 3 0 0 1 -3 -3v-1h10v2a2 2 0 1 0 4 0v-2m0 -4v-8a2 2 0 1 1 2 2h-2m2 -4h-11a3 3 0 0 0 -.864 .126m-2.014 2.025a3 3 0 0 0 -.122 .849v11" /> <path d="M11 7h2" /> <path d="M9 11h2" /> <path d="M3 3l18 18" /> </svg>"##;
const LINE_HEIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8l3 -3l3 3" /> <path d="M3 16l3 3l3 -3" /> <path d="M6 5l0 14" /> <path d="M13 6l7 0" /> <path d="M13 12l7 0" /> <path d="M13 18l7 0" /> </svg>"##;
const LINK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l6 -6" /> <path d="M11 6l.463 -.536a5 5 0 0 1 7.071 7.072l-.534 .464" /> <path d="M13 18l-.397 .534a5.068 5.068 0 0 1 -7.127 0a4.972 4.972 0 0 1 0 -7.071l.524 -.463" /> </svg>"##;
const LINK_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l6 -6" /> <path d="M11 6l.463 -.536a5 5 0 1 1 7.071 7.072l-.534 .464" /> <path d="M12.603 18.534a5.07 5.07 0 0 1 -7.127 0a4.972 4.972 0 0 1 0 -7.071l.524 -.463" /> <path d="M16 19h6" /> </svg>"##;
const LINK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l3 -3m2 -2l1 -1" /> <path d="M11 6l.463 -.536a5 5 0 0 1 7.071 7.072l-.534 .464" /> <path d="M3 3l18 18" /> <path d="M13 18l-.397 .534a5.068 5.068 0 0 1 -7.127 0a4.972 4.972 0 0 1 0 -7.071l.524 -.463" /> </svg>"##;
const LINK_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l6 -6" /> <path d="M11 6l.463 -.536a5 5 0 0 1 7.072 0a4.993 4.993 0 0 1 -.001 7.072" /> <path d="M12.603 18.534a5.07 5.07 0 0 1 -7.127 0a4.972 4.972 0 0 1 0 -7.071l.524 -.463" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const LIST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 6l11 0" /> <path d="M9 12l11 0" /> <path d="M9 18l11 0" /> <path d="M5 6l0 .01" /> <path d="M5 12l0 .01" /> <path d="M5 18l0 .01" /> </svg>"##;
const LIST_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.5 5.5l1.5 1.5l2.5 -2.5" /> <path d="M3.5 11.5l1.5 1.5l2.5 -2.5" /> <path d="M3.5 17.5l1.5 1.5l2.5 -2.5" /> <path d="M11 6l9 0" /> <path d="M11 12l9 0" /> <path d="M11 18l9 0" /> </svg>"##;
const LIST_DETAILS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 5h8" /> <path d="M13 9h5" /> <path d="M13 15h8" /> <path d="M13 19h5" /> <path d="M3 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M3 15a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> </svg>"##;
const LIST_NUMBERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 6h9" /> <path d="M11 12h9" /> <path d="M12 18h8" /> <path d="M4 16a2 2 0 1 1 4 0c0 .591 -.5 1 -1 1.5l-3 2.5h4" /> <path d="M6 10v-6l-2 2" /> </svg>"##;
const LIST_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 15a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M18.5 18.5l2.5 2.5" /> <path d="M4 6h16" /> <path d="M4 12h4" /> <path d="M4 18h4" /> </svg>"##;
const MANUAL_GEARBOX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 8l0 8" /> <path d="M12 8l0 8" /> <path d="M19 8v2a2 2 0 0 1 -2 2h-12" /> </svg>"##;
const MARKDOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M7 15v-6l2 2l2 -2v6" /> <path d="M14 13l2 2l2 -2m-2 2v-6" /> </svg>"##;
const MARKDOWN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h10a2 2 0 0 1 2 2v10" /> <path d="M19 19h-14a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 1.85 -2" /> <path d="M7 15v-6l2 2l1 -1m1 1v4" /> <path d="M17.5 13.5l.5 -.5m-2 -1v-3" /> <path d="M3 3l18 18" /> </svg>"##;
const NEWS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 6h3a1 1 0 0 1 1 1v11a2 2 0 0 1 -4 0v-13a1 1 0 0 0 -1 -1h-10a1 1 0 0 0 -1 1v12a3 3 0 0 0 3 3h11" /> <path d="M8 8l4 0" /> <path d="M8 12l4 0" /> <path d="M8 16l4 0" /> </svg>"##;
const NEWS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 6h3a1 1 0 0 1 1 1v9m-.606 3.435a2 2 0 0 1 -3.394 -1.435v-2m0 -4v-7a1 1 0 0 0 -1 -1h-7m-3.735 .321a1 1 0 0 0 -.265 .679v12a3 3 0 0 0 3 3h11" /> <path d="M8 12h4" /> <path d="M8 16h4" /> <path d="M3 3l18 18" /> </svg>"##;
const NOTDEF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.49 3.666l-12.976 16.673" /> <path d="M5.51 3.666l12.976 16.673" /> <path d="M5 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const NOTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 20l7 -7" /> <path d="M13 20v-6a1 1 0 0 1 1 -1h6v-7a2 2 0 0 0 -2 -2h-12a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h7" /> </svg>"##;
const NOTE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 20l3.505 -3.505m2 -2l1.501 -1.501" /> <path d="M17 13h3v-7a2 2 0 0 0 -2 -2h-10m-3.427 .6c-.355 .36 -.573 .853 -.573 1.4v12a2 2 0 0 0 2 2h7v-6c0 -.272 .109 -.519 .285 -.699" /> <path d="M3 3l18 18" /> </svg>"##;
const NOTEBOOK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4h11a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-11a1 1 0 0 1 -1 -1v-14a1 1 0 0 1 1 -1m3 0v18" /> <path d="M13 8l2 0" /> <path d="M13 12l2 0" /> </svg>"##;
const NOTEBOOK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h9a2 2 0 0 1 2 2v9m-.179 3.828a2 2 0 0 1 -1.821 1.172h-11a1 1 0 0 1 -1 -1v-14m4 -1v1m0 4v13" /> <path d="M13 8h2" /> <path d="M3 3l18 18" /> </svg>"##;
const NOTES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -14" /> <path d="M9 7l6 0" /> <path d="M9 11l6 0" /> <path d="M9 15l4 0" /> </svg>"##;
const NOTES_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h10a2 2 0 0 1 2 2v10m0 4a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14" /> <path d="M11 7h4" /> <path d="M9 11h2" /> <path d="M9 15h4" /> <path d="M3 3l18 18" /> </svg>"##;
const NUMBER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 17v-10l7 10v-10" /> <path d="M15 17h5" /> <path d="M15 10a2.5 3 0 1 0 5 0a2.5 3 0 1 0 -5 0" /> </svg>"##;
const NUMBER_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 16v-8" /> <path d="M12 20a4 4 0 0 0 4 -4v-8a4 4 0 1 0 -8 0v8a4 4 0 0 0 4 4" /> </svg>"##;
const NUMBER_0_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> </svg>"##;
const NUMBER_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 20v-16l-5 5" /> </svg>"##;
const NUMBER_1_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 8h1v8" /> </svg>"##;
const NUMBER_10_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 20v-16l-5 5" /> <path d="M16 20a4 4 0 0 0 4 -4v-8a4 4 0 1 0 -8 0v8a4 4 0 0 0 4 4" /> </svg>"##;
const NUMBER_10_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h1v8" /> <path d="M14 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> </svg>"##;
const NUMBER_100_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8h1v8" /> <path d="M9 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M16 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> </svg>"##;
const NUMBER_11_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 20v-16l-5 5" /> <path d="M18 20v-16l-5 5" /> </svg>"##;
const NUMBER_11_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h1v8" /> <path d="M14 8h1v8" /> </svg>"##;
const NUMBER_12_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h1v8" /> <path d="M13 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_123_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10l2 -2v8" /> <path d="M9 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M17 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const NUMBER_13_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h1v8" /> <path d="M13 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const NUMBER_14_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h1v8" /> <path d="M13 8v3a1 1 0 0 0 1 1h3" /> <path d="M17 8v8" /> </svg>"##;
const NUMBER_15_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h1v8" /> <path d="M13 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const NUMBER_16_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h1v8" /> <path d="M17 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const NUMBER_17_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h1v8" /> <path d="M13 8h4l-2 8" /> </svg>"##;
const NUMBER_18_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h1v8" /> <path d="M15 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const NUMBER_19_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h1v8" /> <path d="M13 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8a4 4 0 1 1 8 0c0 1.098 -.564 2.025 -1.159 2.815l-6.841 9.185h8" /> </svg>"##;
const NUMBER_2_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_20_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M6 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_21_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h1v8" /> <path d="M7 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_22_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M6 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_23_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> <path d="M6 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_24_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8v3a1 1 0 0 0 1 1h3" /> <path d="M18 8v8" /> <path d="M6 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_25_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> <path d="M6 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_26_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> <path d="M6 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_27_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h4l-2 8" /> <path d="M6 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_28_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> <path d="M6 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_29_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M6 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12a4 4 0 1 0 -4 -4" /> <path d="M8 16a4 4 0 1 0 4 -4" /> </svg>"##;
const NUMBER_3_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const NUMBER_30_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M6 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const NUMBER_31_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h1v8" /> <path d="M7 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const NUMBER_32_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M6 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const NUMBER_33_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> <path d="M6 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const NUMBER_34_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8v3a1 1 0 0 0 1 1h3" /> <path d="M18 8v8" /> <path d="M6 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const NUMBER_35_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> <path d="M6 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const NUMBER_36_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> <path d="M6 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const NUMBER_37_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h4l-2 8" /> <path d="M6 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const NUMBER_38_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> <path d="M6 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const NUMBER_39_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M6 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const NUMBER_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 20v-15l-8 11h10" /> </svg>"##;
const NUMBER_4_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v3a1 1 0 0 0 1 1h3" /> <path d="M14 8v8" /> </svg>"##;
const NUMBER_40_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M6 8v3a1 1 0 0 0 1 1h3" /> <path d="M10 8v8" /> </svg>"##;
const NUMBER_41_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h1v8" /> <path d="M6 8v3a1 1 0 0 0 1 1h3" /> <path d="M10 8v8" /> </svg>"##;
const NUMBER_42_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M6 8v3a1 1 0 0 0 1 1h3" /> <path d="M10 8v8" /> </svg>"##;
const NUMBER_43_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> <path d="M6 8v3a1 1 0 0 0 1 1h3" /> <path d="M10 8v8" /> </svg>"##;
const NUMBER_44_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8v3a1 1 0 0 0 1 1h3" /> <path d="M18 8v8" /> <path d="M6 8v3a1 1 0 0 0 1 1h3" /> <path d="M10 8v8" /> </svg>"##;
const NUMBER_45_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> <path d="M6 8v3a1 1 0 0 0 1 1h3" /> <path d="M10 8v8" /> </svg>"##;
const NUMBER_46_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> <path d="M6 8v3a1 1 0 0 0 1 1h3" /> <path d="M10 8v8" /> </svg>"##;
const NUMBER_47_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h4l-2 8" /> <path d="M6 8v3a1 1 0 0 0 1 1h3" /> <path d="M10 8v8" /> </svg>"##;
const NUMBER_48_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> <path d="M6 8v3a1 1 0 0 0 1 1h3" /> <path d="M10 8v8" /> </svg>"##;
const NUMBER_49_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M6 8v3a1 1 0 0 0 1 1h3" /> <path d="M10 8v8" /> </svg>"##;
const NUMBER_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 20h4a4 4 0 1 0 0 -8h-4v-8h8" /> </svg>"##;
const NUMBER_5_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const NUMBER_50_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const NUMBER_51_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h1v8" /> <path d="M7 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const NUMBER_52_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const NUMBER_53_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const NUMBER_54_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8v3a1 1 0 0 0 1 1h3" /> <path d="M18 8v8" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const NUMBER_55_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const NUMBER_56_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const NUMBER_57_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h4l-2 8" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const NUMBER_58_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const NUMBER_59_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const NUMBER_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16a4 4 0 1 0 8 0v-1a4 4 0 1 0 -8 0" /> <path d="M16 8a4 4 0 1 0 -8 0v8" /> </svg>"##;
const NUMBER_6_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const NUMBER_60_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M10 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const NUMBER_61_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h1v8" /> <path d="M11 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const NUMBER_62_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M10 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const NUMBER_63_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> <path d="M10 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const NUMBER_64_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8v3a1 1 0 0 0 1 1h3" /> <path d="M18 8v8" /> <path d="M10 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const NUMBER_65_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> <path d="M10 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const NUMBER_66_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> <path d="M10 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const NUMBER_67_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h4l-2 8" /> <path d="M10 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const NUMBER_68_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> <path d="M10 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const NUMBER_69_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M10 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const NUMBER_7_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h8l-4 16" /> </svg>"##;
const NUMBER_7_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4l-2 8" /> </svg>"##;
const NUMBER_70_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M6 8h4l-2 8" /> </svg>"##;
const NUMBER_71_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h1v8" /> <path d="M7 8h4l-2 8" /> </svg>"##;
const NUMBER_72_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M6 8h4l-2 8" /> </svg>"##;
const NUMBER_73_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> <path d="M6 8h4l-2 8" /> </svg>"##;
const NUMBER_74_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8v3a1 1 0 0 0 1 1h3" /> <path d="M18 8v8" /> <path d="M6 8h4l-2 8" /> </svg>"##;
const NUMBER_75_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> <path d="M6 8h4l-2 8" /> </svg>"##;
const NUMBER_76_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> <path d="M6 8h4l-2 8" /> </svg>"##;
const NUMBER_77_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h4l-2 8" /> <path d="M6 8h4l-2 8" /> </svg>"##;
const NUMBER_78_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> <path d="M6 8h4l-2 8" /> </svg>"##;
const NUMBER_79_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M6 8h4l-2 8" /> </svg>"##;
const NUMBER_8_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M8 16a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> </svg>"##;
const NUMBER_8_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const NUMBER_80_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M8 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const NUMBER_81_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h1v8" /> <path d="M9 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const NUMBER_82_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M8 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const NUMBER_83_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> <path d="M8 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const NUMBER_84_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8v3a1 1 0 0 0 1 1h3" /> <path d="M18 8v8" /> <path d="M8 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const NUMBER_85_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> <path d="M8 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const NUMBER_86_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> <path d="M8 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const NUMBER_87_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h4l-2 8" /> <path d="M8 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const NUMBER_88_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> <path d="M8 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const NUMBER_89_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M8 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const NUMBER_9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 8a4 4 0 1 0 -8 0v1a4 4 0 1 0 8 0" /> <path d="M8 16a4 4 0 1 0 8 0v-8" /> </svg>"##;
const NUMBER_9_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_90_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_91_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h1v8" /> <path d="M7 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_92_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_93_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_94_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8v3a1 1 0 0 0 1 1h3" /> <path d="M18 8v8" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_95_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_96_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_97_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h4l-2 8" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_98_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBER_99_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const NUMBERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 10v-7l-2 2" /> <path d="M6 16a2 2 0 1 1 4 0c0 .591 -.601 1.46 -1 2l-3 3h4" /> <path d="M15 14a2 2 0 1 0 2 -2a2 2 0 1 0 -2 -2" /> <path d="M6.5 10h3" /> </svg>"##;
const OVERLINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9v5a5 5 0 0 0 10 0v-5" /> <path d="M5 5h14" /> </svg>"##;
const PAGE_BREAK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3v4a1 1 0 0 0 1 1h4" /> <path d="M19 18v1a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-1" /> <path d="M3 14h3m4.5 0h3m4.5 0h3" /> <path d="M5 10v-5a2 2 0 0 1 2 -2h7l5 5v2" /> </svg>"##;
const PAPERCLIP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 7l-6.5 6.5a1.5 1.5 0 0 0 3 3l6.5 -6.5a3 3 0 0 0 -6 -6l-6.5 6.5a4.5 4.5 0 0 0 9 9l6.5 -6.5" /> </svg>"##;
const PENTAGON_NUMBER_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> <path d="M10 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> </svg>"##;
const PENTAGON_NUMBER_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> <path d="M10 10l2 -2v8" /> </svg>"##;
const PENTAGON_NUMBER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> <path d="M10 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const PENTAGON_NUMBER_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> <path d="M10 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const PENTAGON_NUMBER_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> <path d="M10 8v3a1 1 0 0 0 1 1h3" /> <path d="M14 8v8" /> </svg>"##;
const PENTAGON_NUMBER_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const PENTAGON_NUMBER_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> <path d="M14 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const PENTAGON_NUMBER_7_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> <path d="M10 8h4l-2 8" /> </svg>"##;
const PENTAGON_NUMBER_8_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> <path d="M12 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const PENTAGON_NUMBER_9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const PILCROW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 4v16" /> <path d="M17 4v16" /> <path d="M19 4h-9.5a4.5 4.5 0 0 0 0 9h3.5" /> </svg>"##;
const PILCROW_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 9h-2a3 3 0 1 1 0 -6h7" /> <path d="M11 3v11" /> <path d="M15 3v11" /> <path d="M3 18h18" /> <path d="M6 15l-3 3l3 3" /> </svg>"##;
const PILCROW_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 9h-2a3 3 0 1 1 0 -6h7" /> <path d="M11 3v11" /> <path d="M15 3v11" /> <path d="M21 18h-18" /> <path d="M18 15l3 3l-3 3" /> </svg>"##;
const PRESENTATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4l18 0" /> <path d="M4 4v10a2 2 0 0 0 2 2h12a2 2 0 0 0 2 -2v-10" /> <path d="M12 16l0 4" /> <path d="M9 20l6 0" /> <path d="M8 12l3 -3l2 2l3 -3" /> </svg>"##;
const PRESENTATION_ANALYTICS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12v-4" /> <path d="M15 12v-2" /> <path d="M12 12v-1" /> <path d="M3 4h18" /> <path d="M4 4v10a2 2 0 0 0 2 2h12a2 2 0 0 0 2 -2v-10" /> <path d="M12 16v4" /> <path d="M9 20h6" /> </svg>"##;
const PRESENTATION_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4h1m4 0h13" /> <path d="M4 4v10a2 2 0 0 0 2 2h10m3.42 -.592c.359 -.362 .58 -.859 .58 -1.408v-10" /> <path d="M12 16v4" /> <path d="M9 20h6" /> <path d="M8 12l2 -2m4 0l2 -2" /> <path d="M3 3l18 18" /> </svg>"##;
const QUOTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 11h-4a1 1 0 0 1 -1 -1v-3a1 1 0 0 1 1 -1h3a1 1 0 0 1 1 1v6c0 2.667 -1.333 4.333 -4 5" /> <path d="M19 11h-4a1 1 0 0 1 -1 -1v-3a1 1 0 0 1 1 -1h3a1 1 0 0 1 1 1v6c0 2.667 -1.333 4.333 -4 5" /> </svg>"##;
const QUOTE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 11h-4a1 1 0 0 1 -1 -1v-3a1 1 0 0 1 1 -1m4 4v3c0 2.667 -1.333 4.333 -4 5" /> <path d="M19 11h-4m-1 -1v-3a1 1 0 0 1 1 -1h3a1 1 0 0 1 1 1v6c0 .66 -.082 1.26 -.245 1.798m-1.653 2.29c-.571 .4 -1.272 .704 -2.102 .912" /> <path d="M3 3l18 18" /> </svg>"##;
const QUOTE_OPEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 13h4a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-3a1 1 0 0 1 -1 -1v-6q 0 -4 4 -5" /> <path d="M5 13h4a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-3a1 1 0 0 1 -1 -1v-6q 0 -4 4 -5" /> </svg>"##;
const QUOTES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12c-1.333 -1.854 -1.333 -4.146 0 -6" /> <path d="M8 12c-1.333 -1.854 -1.333 -4.146 0 -6" /> <path d="M16 18c1.333 -1.854 1.333 -4.146 0 -6" /> <path d="M20 18c1.333 -1.854 1.333 -4.146 0 -6" /> </svg>"##;
const RECEIPT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16l-3 -2l-2 2l-2 -2l-2 2l-2 -2l-3 2m4 -14h6m-6 4h6m-2 4h2" /> </svg>"##;
const RECEIPT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16l-3 -2l-2 2l-2 -2l-2 2l-2 -2l-3 2" /> <path d="M14 8h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5m2 0v1.5m0 -9v1.5" /> </svg>"##;
const RECEIPT_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16l-3 -2l-2 2l-2 -2l-2 2l-2 -2l-3 2" /> <path d="M14.8 8a2 2 0 0 0 -1.8 -1h-2a2 2 0 1 0 0 4h2a2 2 0 1 1 0 4h-2a2 2 0 0 1 -1.8 -1" /> <path d="M12 6v10" /> </svg>"##;
const RECEIPT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21v-16m2 -2h10a2 2 0 0 1 2 2v10m0 4.01v1.99l-3 -2l-2 2l-2 -2l-2 2l-2 -2l-3 2" /> <path d="M11 7l4 0" /> <path d="M9 11l2 0" /> <path d="M13 15l2 0" /> <path d="M15 11l0 .01" /> <path d="M3 3l18 18" /> </svg>"##;
const RECEIPT_REFUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16l-3 -2l-2 2l-2 -2l-2 2l-2 -2l-3 2" /> <path d="M15 14v-2a2 2 0 0 0 -2 -2h-4l2 -2m0 4l-2 -2" /> </svg>"##;
const RECEIPT_TAX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 14l6 -6" /> <path d="M9 8.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M14 13.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16l-3 -2l-2 2l-2 -2l-2 2l-2 -2l-3 2" /> </svg>"##;
const REGEX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.5 15a2.5 2.5 0 1 1 0 5a2.5 2.5 0 0 1 0 -5" /> <path d="M17 7.875l3 -1.687" /> <path d="M17 7.875v3.375" /> <path d="M17 7.875l-3 -1.687" /> <path d="M17 7.875l3 1.688" /> <path d="M17 4.5v3.375" /> <path d="M17 7.875l-3 1.688" /> </svg>"##;
const REGEX_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.5 15a2.5 2.5 0 1 1 0 5a2.5 2.5 0 0 1 0 -5" /> <path d="M17 7.875l3 -1.687" /> <path d="M17 7.875v3.375" /> <path d="M17 7.875l-3 -1.687" /> <path d="M17 7.875l3 1.688" /> <path d="M17 4.5v3.375" /> <path d="M17 7.875l-3 1.688" /> <path d="M3 3l18 18" /> </svg>"##;
const REPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h5.697" /> <path d="M18 14v4h4" /> <path d="M18 11v-4a2 2 0 0 0 -2 -2h-2" /> <path d="M8 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M14 18a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M8 11h4" /> <path d="M8 15h3" /> </svg>"##;
const REPORT_ANALYTICS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M9 17v-5" /> <path d="M12 17v-1" /> <path d="M15 17v-3" /> </svg>"##;
const REPORT_MEDICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M10 14l4 0" /> <path d="M12 12l0 4" /> </svg>"##;
const REPORT_MONEY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M14 11h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M12 17v1m0 -8v1" /> </svg>"##;
const REPORT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.576 5.595a2 2 0 0 0 -.576 1.405v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2m0 -4v-8a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 1 1 0 4h-2" /> <path d="M3 3l18 18" /> </svg>"##;
const REPORT_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h5.697" /> <path d="M18 12v-5a2 2 0 0 0 -2 -2h-2" /> <path d="M8 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M8 11h4" /> <path d="M8 15h3" /> <path d="M14 17.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> <path d="M18.5 19.5l2.5 2.5" /> </svg>"##;
const ROSETTE_NUMBER_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1" /> </svg>"##;
const ROSETTE_NUMBER_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 10l2 -2v8" /> <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1" /> </svg>"##;
const ROSETTE_NUMBER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1" /> </svg>"##;
const ROSETTE_NUMBER_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 9a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1" /> <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1" /> </svg>"##;
const ROSETTE_NUMBER_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v3a1 1 0 0 0 1 1h3" /> <path d="M14 8v8" /> <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1" /> </svg>"##;
const ROSETTE_NUMBER_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1" /> </svg>"##;
const ROSETTE_NUMBER_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1" /> </svg>"##;
const ROSETTE_NUMBER_7_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4l-2 8" /> <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1" /> </svg>"##;
const ROSETTE_NUMBER_8_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1" /> </svg>"##;
const ROSETTE_NUMBER_9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1" /> </svg>"##;
const RUBBER_STAMP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 17.85h-18c0 -4.05 1.421 -4.05 3.79 -4.05c5.21 0 1.21 -4.59 1.21 -6.8a4 4 0 1 1 8 0c0 2.21 -4 6.8 1.21 6.8c2.369 0 3.79 0 3.79 4.05" /> <path d="M5 21h14" /> </svg>"##;
const RUBBER_STAMP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.273 8.273c.805 2.341 2.857 5.527 -1.484 5.527c-2.368 0 -3.789 0 -3.789 4.05h14.85" /> <path d="M5 21h14" /> <path d="M3 3l18 18" /> <path d="M8.712 4.722a3.99 3.99 0 0 1 3.288 -1.722a4 4 0 0 1 4 4c0 .992 -.806 2.464 -1.223 3.785m6.198 6.196c-.182 -2.883 -1.332 -3.153 -3.172 -3.178" /> </svg>"##;
const SCAN_LETTER_A_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 16v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v6" /> <path d="M9 13h6" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const SCAN_LETTER_T_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 8h6" /> <path d="M12 8v8" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const SCRIBBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 15c2 3 4 4 7 4s7 -3 7 -7s-3 -7 -6 -7s-5 1.5 -5 4s2 5 6 5s8.408 -2.453 10 -5" /> </svg>"##;
const SCRIBBLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 15c2 3 4 4 7 4c1.95 0 4.324 -1.268 5.746 -3.256m1.181 -2.812a5.97 5.97 0 0 0 .073 -.932c0 -4 -3 -7 -6 -7c-.642 0 -1.239 .069 -1.78 .201m-2.492 1.515c-.47 .617 -.728 1.386 -.728 2.284c0 2.5 2 5 6 5c.597 0 1.203 -.055 1.808 -.156m3.102 -.921c2.235 -.953 4.152 -2.423 5.09 -3.923" /> <path d="M3 3l18 18" /> </svg>"##;
const SCRIPT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 20h-11a3 3 0 0 1 0 -6h11a3 3 0 0 0 0 6h1a3 3 0 0 0 3 -3v-11a2 2 0 0 0 -2 -2h-10a2 2 0 0 0 -2 2v8" /> </svg>"##;
const SCRIPT_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 19h4" /> <path d="M14 20h-8a3 3 0 0 1 0 -6h11a3 3 0 0 0 -3 3m7 -2v-9a2 2 0 0 0 -2 -2h-10a2 2 0 0 0 -2 2v8" /> </svg>"##;
const SCRIPT_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 19h4" /> <path d="M14 20h-8a3 3 0 0 1 0 -6h11a3 3 0 0 0 -3 3m7 -3v-8a2 2 0 0 0 -2 -2h-10a2 2 0 0 0 -2 2v8" /> <path d="M19 17v4" /> </svg>"##;
const SCRIPT_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 20h-8a3 3 0 0 1 0 -6h11a3 3 0 0 0 -3 3m7 -3v-8a2 2 0 0 0 -2 -2h-10a2 2 0 0 0 -2 2v8" /> <path d="M17 17l4 4m0 -4l-4 4" /> </svg>"##;
const SECTION_SIGN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.172 19a3 3 0 1 0 2.828 -4" /> <path d="M14.83 5a3 3 0 1 0 -2.83 4" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const SEPARATOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12l0 .01" /> <path d="M7 12l10 0" /> <path d="M21 12l0 .01" /> </svg>"##;
const SEPARATOR_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12l16 0" /> <path d="M8 8l4 -4l4 4" /> <path d="M16 16l-4 4l-4 -4" /> </svg>"##;
const SEPARATOR_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4l0 16" /> <path d="M8 8l-4 4l4 4" /> <path d="M16 16l4 -4l-4 -4" /> </svg>"##;
const SIGNATURE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17c3.333 -3.333 5 -6 5 -8c0 -3 -1 -3 -2 -3s-2.032 1.085 -2 3c.034 2.048 1.658 4.877 2.5 6c1.5 2 2.5 2.5 3.5 1l2 -3c.333 2.667 1.333 4 3 4c.53 0 2.639 -2 3 -2c.517 0 1.517 .667 3 2" /> </svg>"##;
const SIGNATURE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17c3.333 -3.333 5 -6 5 -8c0 -.394 -.017 -.735 -.05 -1.033m-1.95 -1.967c-1 0 -2.032 1.085 -2 3c.034 2.048 1.658 4.877 2.5 6c1.5 2 2.5 2.5 3.5 1l2 -3c.333 2.667 1.333 4 3 4c.219 0 .708 -.341 1.231 -.742m3.769 -.258c.303 .245 .64 .677 1 1" /> <path d="M3 3l18 18" /> </svg>"##;
const SLIDESHOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 6l.01 0" /> <path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3l0 -8" /> <path d="M3 13l4 -4a3 5 0 0 1 3 0l4 4" /> <path d="M13 12l2 -2a3 5 0 0 1 3 0l3 3" /> <path d="M8 21l.01 0" /> <path d="M12 21l.01 0" /> <path d="M16 21l.01 0" /> </svg>"##;
const SORT_0_9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12h2" /> <path d="M4 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M16 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const SORT_9_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M16 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M11 12h2" /> </svg>"##;
const SORT_A_Z_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 8h4l-4 8h4" /> <path d="M4 16v-6a2 2 0 1 1 4 0v6" /> <path d="M4 13h4" /> <path d="M11 12h2" /> </svg>"##;
const SORT_ASCENDING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6l7 0" /> <path d="M4 12l7 0" /> <path d="M4 18l9 0" /> <path d="M15 9l3 -3l3 3" /> <path d="M18 6l0 12" /> </svg>"##;
const SORT_ASCENDING_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 9l3 -3l3 3" /> <path d="M5 5.5a.5 .5 0 0 1 .5 -.5h4a.5 .5 0 0 1 .5 .5v4a.5 .5 0 0 1 -.5 .5h-4a.5 .5 0 0 1 -.5 -.5l0 -4" /> <path d="M5 14.5a.5 .5 0 0 1 .5 -.5h4a.5 .5 0 0 1 .5 .5v4a.5 .5 0 0 1 -.5 .5h-4a.5 .5 0 0 1 -.5 -.5l0 -4" /> <path d="M17 6v12" /> </svg>"##;
const SORT_ASCENDING_LETTERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 10v-5c0 -1.38 .62 -2 2 -2s2 .62 2 2v5m0 -3h-4" /> <path d="M19 21h-4l4 -7h-4" /> <path d="M4 15l3 3l3 -3" /> <path d="M7 6v12" /> </svg>"##;
const SORT_ASCENDING_NUMBERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 15l3 3l3 -3" /> <path d="M7 6v12" /> <path d="M17 3a2 2 0 0 1 2 2v3a2 2 0 1 1 -4 0v-3a2 2 0 0 1 2 -2" /> <path d="M15 16a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19 16v3a2 2 0 0 1 -2 2h-1.5" /> </svg>"##;
const SORT_DESCENDING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6l9 0" /> <path d="M4 12l7 0" /> <path d="M4 18l7 0" /> <path d="M15 15l3 3l3 -3" /> <path d="M18 6l0 12" /> </svg>"##;
const SORT_DESCENDING_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5.5a.5 .5 0 0 1 .5 -.5h4a.5 .5 0 0 1 .5 .5v4a.5 .5 0 0 1 -.5 .5h-4a.5 .5 0 0 1 -.5 -.5l0 -4" /> <path d="M5 14.5a.5 .5 0 0 1 .5 -.5h4a.5 .5 0 0 1 .5 .5v4a.5 .5 0 0 1 -.5 .5h-4a.5 .5 0 0 1 -.5 -.5l0 -4" /> <path d="M14 15l3 3l3 -3" /> <path d="M17 18v-12" /> </svg>"##;
const SORT_DESCENDING_LETTERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21v-5c0 -1.38 .62 -2 2 -2s2 .62 2 2v5m0 -3h-4" /> <path d="M19 10h-4l4 -7h-4" /> <path d="M4 15l3 3l3 -3" /> <path d="M7 6v12" /> </svg>"##;
const SORT_DESCENDING_NUMBERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 15l3 3l3 -3" /> <path d="M7 6v12" /> <path d="M17 14a2 2 0 0 1 2 2v3a2 2 0 1 1 -4 0v-3a2 2 0 0 1 2 -2" /> <path d="M15 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19 5v3a2 2 0 0 1 -2 2h-1.5" /> </svg>"##;
const SORT_Z_A_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8h4l-4 8h4" /> <path d="M16 16v-6a2 2 0 1 1 4 0v6" /> <path d="M16 13h4" /> <path d="M11 12h2" /> </svg>"##;
const SPACE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10v3a1 1 0 0 0 1 1h14a1 1 0 0 0 1 -1v-3" /> </svg>"##;
const SPACE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10v3a1 1 0 0 0 1 1h9m4 0h1a1 1 0 0 0 1 -1v-3" /> <path d="M3 3l18 18" /> </svg>"##;
const SPACING_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 20h-2a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h2" /> <path d="M4 20h2a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M12 8v8" /> </svg>"##;
const SPACING_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20v-2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v2" /> <path d="M4 4v2a2 2 0 0 0 2 2h12a2 2 0 0 0 2 -2v-2" /> <path d="M16 12h-8" /> </svg>"##;
const SQUARE_LETTER_A_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 16v-6a2 2 0 1 1 4 0v6" /> <path d="M10 13h4" /> </svg>"##;
const SQUARE_LETTER_B_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 16h2a2 2 0 1 0 0 -4h-2h2a2 2 0 1 0 0 -4h-2v8" /> </svg>"##;
const SQUARE_LETTER_C_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M14 10a2 2 0 1 0 -4 0v4a2 2 0 1 0 4 0" /> </svg>"##;
const SQUARE_LETTER_D_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2h-2" /> </svg>"##;
const SQUARE_LETTER_E_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M14 8h-4v8h4" /> <path d="M10 12h2.5" /> </svg>"##;
const SQUARE_LETTER_F_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 12h3" /> <path d="M14 8h-4v8" /> </svg>"##;
const SQUARE_LETTER_G_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M14 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> </svg>"##;
const SQUARE_LETTER_H_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 16v-8m4 0v8" /> <path d="M10 12h4" /> </svg>"##;
const SQUARE_LETTER_I_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M12 8v8" /> </svg>"##;
const SQUARE_LETTER_J_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 8h4v6a2 2 0 1 1 -4 0" /> </svg>"##;
const SQUARE_LETTER_K_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 8v8" /> <path d="M14 8l-2.5 4l2.5 4" /> <path d="M10 12h1.5" /> </svg>"##;
const SQUARE_LETTER_L_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 8v8h4" /> </svg>"##;
const SQUARE_LETTER_M_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 16v-8l3 5l3 -5v8" /> </svg>"##;
const SQUARE_LETTER_N_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 16v-8l4 8v-8" /> </svg>"##;
const SQUARE_LETTER_O_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> </svg>"##;
const SQUARE_LETTER_P_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8" /> </svg>"##;
const SQUARE_LETTER_Q_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M13 15l1 1" /> </svg>"##;
const SQUARE_LETTER_R_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8m4 0l-3 -4" /> </svg>"##;
const SQUARE_LETTER_S_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1" /> </svg>"##;
const SQUARE_LETTER_T_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 8h4" /> <path d="M12 8v8" /> </svg>"##;
const SQUARE_LETTER_U_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 8v6a2 2 0 1 0 4 0v-6" /> </svg>"##;
const SQUARE_LETTER_V_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 8l2 8l2 -8" /> </svg>"##;
const SQUARE_LETTER_W_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 8l1 8l2 -5l2 5l1 -8" /> </svg>"##;
const SQUARE_LETTER_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 8l4 8" /> <path d="M10 16l4 -8" /> </svg>"##;
const SQUARE_LETTER_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 8l2 5l2 -5" /> <path d="M12 16v-3" /> </svg>"##;
const SQUARE_LETTER_Z_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 8h4l-4 8h4" /> </svg>"##;
const SQUARE_NUMBER_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> </svg>"##;
const SQUARE_NUMBER_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 10l2 -2v8" /> </svg>"##;
const SQUARE_NUMBER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const SQUARE_NUMBER_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 9a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1" /> </svg>"##;
const SQUARE_NUMBER_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 8v3a1 1 0 0 0 1 1h3" /> <path d="M14 8v8" /> </svg>"##;
const SQUARE_NUMBER_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const SQUARE_NUMBER_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M14 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const SQUARE_NUMBER_7_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 8h4l-2 8" /> </svg>"##;
const SQUARE_NUMBER_8_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M12 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> </svg>"##;
const SQUARE_NUMBER_9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_A_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-6a2 2 0 1 1 4 0v6" /> <path d="M10 13h4" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_B_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16h2a2 2 0 1 0 0 -4h-2h2a2 2 0 1 0 0 -4h-2v8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_C_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10a2 2 0 1 0 -4 0v4a2 2 0 1 0 4 0" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_D_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2h-2" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_E_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h-4v8h4" /> <path d="M10 12h2.5" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_F_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h3" /> <path d="M14 8h-4v8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_G_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_H_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-8m4 0v8" /> <path d="M10 12h4" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_I_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8v8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_J_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4v6a2 2 0 1 1 -4 0" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_K_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v8" /> <path d="M14 8l-2.5 4l2.5 4" /> <path d="M10 12h1.5" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_L_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v8h4" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_M_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 16v-8l3 5l3 -5v8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_N_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-8l4 8v-8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_O_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_P_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_Q_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M13 15l1 1" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_R_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8m4 0l-3 -4" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_S_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_T_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4" /> <path d="M12 8v8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_U_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v6a2 2 0 1 0 4 0v-6" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_V_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8l2 8l2 -8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_W_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 8l1 8l2 -5l2 5l1 -8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8l4 8" /> <path d="M10 16l4 -8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8l2 5l2 -5" /> <path d="M12 16v-3" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_LETTER_Z_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4l-4 8h4" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_NUMBER_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_NUMBER_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 10l2 -2v8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_NUMBER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_NUMBER_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 9a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_NUMBER_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v3a1 1 0 0 0 1 1h3" /> <path d="M14 8v8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_NUMBER_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_NUMBER_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_NUMBER_7_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4l-2 8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_NUMBER_8_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_NUMBER_9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const STRIKETHROUGH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12l14 0" /> <path d="M16 6.5a4 2 0 0 0 -4 -1.5h-1a3.5 3.5 0 0 0 0 7h2a3.5 3.5 0 0 1 0 7h-1.5a4 2 0 0 1 -4 -1.5" /> </svg>"##;
const SUBSCRIPT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7l8 10m-8 0l8 -10" /> <path d="M21 20h-4l3.5 -4a1.73 1.73 0 0 0 -3.5 -2" /> </svg>"##;
const SUPERSCRIPT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7l8 10m-8 0l8 -10" /> <path d="M21 11h-4l3.5 -4a1.73 1.73 0 0 0 -3.5 -2" /> </svg>"##;
const TEX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 8v-1h-6v1" /> <path d="M6 15v-8" /> <path d="M21 15l-5 -8" /> <path d="M16 15l5 -8" /> <path d="M14 11h-4v8h4" /> <path d="M10 15h3" /> </svg>"##;
const TEXT_CAPTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 15h16" /> <path d="M4 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M4 20h12" /> </svg>"##;
const TEXT_COLOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15v-7a3 3 0 0 1 6 0v7" /> <path d="M9 11h6" /> <path d="M5 19h14" /> </svg>"##;
const TEXT_DECREASE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19v-10.5a3.5 3.5 0 1 1 7 0v10.5" /> <path d="M4 13h7" /> <path d="M21 12h-6" /> </svg>"##;
const TEXT_DIRECTION_LTR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 19h14" /> <path d="M17 21l2 -2l-2 -2" /> <path d="M16 4h-6.5a3.5 3.5 0 0 0 0 7h.5" /> <path d="M14 15v-11" /> <path d="M10 15v-11" /> </svg>"##;
const TEXT_DIRECTION_RTL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 4h-6.5a3.5 3.5 0 0 0 0 7h.5" /> <path d="M14 15v-11" /> <path d="M10 15v-11" /> <path d="M5 19h14" /> <path d="M7 21l-2 -2l2 -2" /> </svg>"##;
const TEXT_GRAMMAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 9a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M4 12v-5a3 3 0 1 1 6 0v5" /> <path d="M4 9h6" /> <path d="M20 6v6" /> <path d="M4 16h12" /> <path d="M4 20h6" /> <path d="M14 20l2 2l5 -5" /> </svg>"##;
const TEXT_INCREASE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19v-10.5a3.5 3.5 0 1 1 7 0v10.5" /> <path d="M4 13h7" /> <path d="M18 9v6" /> <path d="M21 12h-6" /> </svg>"##;
const TEXT_ORIENTATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l-5 -5c-1.367 -1.367 -1.367 -3.633 0 -5s3.633 -1.367 5 0l5 5" /> <path d="M5.5 11.5l5 -5" /> <path d="M21 12l-9 9" /> <path d="M21 12v4" /> <path d="M21 12h-4" /> </svg>"##;
const TEXT_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 10h-14" /> <path d="M5 6h14" /> <path d="M14 14h-9" /> <path d="M5 18h6" /> <path d="M18 15v6" /> <path d="M15 18h6" /> </svg>"##;
const TEXT_RECOGNITION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> <path d="M12 16v-7" /> <path d="M9 9h6" /> </svg>"##;
const TEXT_SIZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7v-2h13v2" /> <path d="M10 5v14" /> <path d="M12 19h-4" /> <path d="M15 13v-1h6v1" /> <path d="M18 12v7" /> <path d="M17 19h2" /> </svg>"##;
const TEXT_SPELLCHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 15v-7.5a3.5 3.5 0 0 1 7 0v7.5" /> <path d="M5 10h7" /> <path d="M10 18l3 3l7 -7" /> </svg>"##;
const TEXT_WRAP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6l16 0" /> <path d="M4 18l5 0" /> <path d="M4 12h13a3 3 0 0 1 0 6h-4l2 -2m0 4l-2 -2" /> </svg>"##;
const TEXT_WRAP_COLUMN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9h7a3 3 0 0 1 0 6h-4l2 -2" /> <path d="M12 17l-2 -2" /> <path d="M3 3v18" /> <path d="M21 3v18" /> </svg>"##;
const TEXT_WRAP_DISABLED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6l10 0" /> <path d="M4 18l10 0" /> <path d="M4 12h17l-3 -3m0 6l3 -3" /> </svg>"##;
const TICKET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 5l0 2" /> <path d="M15 11l0 2" /> <path d="M15 17l0 2" /> <path d="M5 5h14a2 2 0 0 1 2 2v3a2 2 0 0 0 0 4v3a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-3a2 2 0 0 0 0 -4v-3a2 2 0 0 1 2 -2" /> </svg>"##;
const TICKET_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 5v2" /> <path d="M15 17v2" /> <path d="M9 5h10a2 2 0 0 1 2 2v3a2 2 0 1 0 0 4v3m-2 2h-14a2 2 0 0 1 -2 -2v-3a2 2 0 1 0 0 -4v-3a2 2 0 0 1 2 -2" /> <path d="M3 3l18 18" /> </svg>"##;
const TYPOGRAPHY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20l3 0" /> <path d="M14 20l7 0" /> <path d="M6.9 15l6.9 0" /> <path d="M10.2 6.3l5.8 13.7" /> <path d="M5 20l6 -16l2 0l7 16" /> </svg>"##;
const TYPOGRAPHY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h3" /> <path d="M14 20h6" /> <path d="M6.9 15h6.9" /> <path d="M13 13l3 7" /> <path d="M5 20l4.09 -10.906" /> <path d="M10.181 6.183l.819 -2.183h2l3.904 8.924" /> <path d="M3 3l18 18" /> </svg>"##;
const UNDERLINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5v5a5 5 0 0 0 10 0v-5" /> <path d="M5 19h14" /> </svg>"##;
const UNLINK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 22v-2" /> <path d="M9 15l6 -6" /> <path d="M11 6l.463 -.536a5 5 0 0 1 7.071 7.072l-.534 .464" /> <path d="M13 18l-.397 .534a5.068 5.068 0 0 1 -7.127 0a4.972 4.972 0 0 1 0 -7.071l.524 -.463" /> <path d="M20 17h2" /> <path d="M2 7h2" /> <path d="M7 2v2" /> </svg>"##;
const VOCABULARY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 19h-6a1 1 0 0 1 -1 -1v-14a1 1 0 0 1 1 -1h6a2 2 0 0 1 2 2a2 2 0 0 1 2 -2h6a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-6a2 2 0 0 0 -2 2a2 2 0 0 0 -2 -2" /> <path d="M12 5v16" /> <path d="M7 7h1" /> <path d="M7 11h1" /> <path d="M16 7h1" /> <path d="M16 11h1" /> <path d="M16 15h1" /> </svg>"##;
const VOCABULARY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h3a2 2 0 0 1 2 2a2 2 0 0 1 2 -2h6a1 1 0 0 1 1 1v13m-2 2h-5a2 2 0 0 0 -2 2a2 2 0 0 0 -2 -2h-6a1 1 0 0 1 -1 -1v-14c0 -.279 .114 -.53 .298 -.712" /> <path d="M12 5v3m0 4v9" /> <path d="M7 11h1" /> <path d="M16 7h1" /> <path d="M16 11h1" /> <path d="M3 3l18 18" /> </svg>"##;
const WRITING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 17v-12c0 -1.121 -.879 -2 -2 -2s-2 .879 -2 2v12l2 2l2 -2" /> <path d="M16 7h4" /> <path d="M18 19h-13a2 2 0 1 1 0 -4h4a2 2 0 1 0 0 -4h-3" /> </svg>"##;
const WRITING_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 7h4" /> <path d="M16 16v1l2 2l.5 -.5m1.5 -2.5v-11c0 -1.121 -.879 -2 -2 -2s-2 .879 -2 2v7" /> <path d="M18 19h-13a2 2 0 1 1 0 -4h4a2 2 0 1 0 0 -4h-3" /> <path d="M3 3l18 18" /> </svg>"##;
const WRITING_SIGN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19c3.333 -2 5 -4 5 -6c0 -3 -1 -3 -2 -3s-2.032 1.085 -2 3c.034 2.048 1.658 2.877 2.5 4c1.5 2 2.5 2.5 3.5 1c.667 -1 1.167 -1.833 1.5 -2.5c1 2.333 2.333 3.5 4 3.5h2.5" /> <path d="M20 17v-12c0 -1.121 -.879 -2 -2 -2s-2 .879 -2 2v12l2 2l2 -2" /> <path d="M16 7h4" /> </svg>"##;
const WRITING_SIGN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19c3.333 -2 5 -4 5 -6c0 -3 -1 -3 -2 -3s-2.032 1.085 -2 3c.034 2.048 1.658 2.877 2.5 4c1.5 2 2.5 2.5 3.5 1c.667 -1 1.167 -1.833 1.5 -2.5c1 2.333 2.333 3.5 4 3.5h2.5" /> <path d="M16 16v1l2 2l.5 -.5m1.5 -2.5v-11c0 -1.121 -.879 -2 -2 -2s-2 .879 -2 2v7" /> <path d="M16 7h4" /> <path d="M3 3l18 18" /> </svg>"##;

/// Document icon variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DocumentIcon {
    AB2,
    ABOff,
    Abc,
    AlignBoxBottomCenter,
    AlignBoxBottomLeft,
    AlignBoxBottomRight,
    AlignBoxCenterBottom,
    AlignBoxCenterMiddle,
    AlignBoxCenterStretch,
    AlignBoxCenterTop,
    AlignBoxLeftBottom,
    AlignBoxLeftMiddle,
    AlignBoxLeftStretch,
    AlignBoxLeftTop,
    AlignBoxRightBottom,
    AlignBoxRightMiddle,
    AlignBoxRightStretch,
    AlignBoxRightTop,
    AlignBoxTopCenter,
    AlignBoxTopLeft,
    AlignBoxTopRight,
    AlignCenter,
    AlignJustified,
    AlignLeft,
    AlignLeft2,
    AlignRight,
    AlignRight2,
    Alpha,
    AlphabetArabic,
    AlphabetBangla,
    AlphabetCyrillic,
    AlphabetGreek,
    AlphabetHebrew,
    AlphabetKorean,
    AlphabetLatin,
    AlphabetPolish,
    AlphabetRunes,
    AlphabetThai,
    Archive,
    ArchiveOff,
    Article,
    ArticleOff,
    Asterisk,
    AsteriskSimple,
    At,
    AtOff,
    Backspace,
    Ballpen,
    BallpenOff,
    Baseline,
    BaselineDensityLarge,
    BaselineDensityMedium,
    BaselineDensitySmall,
    Beta,
    Bible,
    Blockquote,
    Bold,
    BoldOff,
    Book,
    Book2,
    BookDownload,
    BookOff,
    BookUpload,
    Bookmark,
    BookmarkAi,
    BookmarkEdit,
    BookmarkMinus,
    BookmarkOff,
    BookmarkPlus,
    BookmarkQuestion,
    Bookmarks,
    BookmarksOff,
    Books,
    BooksOff,
    BoxMultiple0,
    BoxMultiple1,
    BoxMultiple2,
    BoxMultiple3,
    BoxMultiple4,
    BoxMultiple5,
    BoxMultiple6,
    BoxMultiple7,
    BoxMultiple8,
    BoxMultiple9,
    Certificate,
    Certificate2,
    Certificate2Off,
    CertificateOff,
    Chalkboard,
    ChalkboardOff,
    CircleDashedLetterA,
    CircleDashedLetterB,
    CircleDashedLetterC,
    CircleDashedLetterD,
    CircleDashedLetterE,
    CircleDashedLetterF,
    CircleDashedLetterG,
    CircleDashedLetterH,
    CircleDashedLetterI,
    CircleDashedLetterJ,
    CircleDashedLetterK,
    CircleDashedLetterL,
    CircleDashedLetterM,
    CircleDashedLetterN,
    CircleDashedLetterO,
    CircleDashedLetterP,
    CircleDashedLetterQ,
    CircleDashedLetterR,
    CircleDashedLetterS,
    CircleDashedLetterT,
    CircleDashedLetterU,
    CircleDashedLetterV,
    CircleDashedLetterW,
    CircleDashedLetterX,
    CircleDashedLetterY,
    CircleDashedLetterZ,
    CircleDashedNumber0,
    CircleDashedNumber1,
    CircleDashedNumber2,
    CircleDashedNumber3,
    CircleDashedNumber4,
    CircleDashedNumber5,
    CircleDashedNumber6,
    CircleDashedNumber7,
    CircleDashedNumber8,
    CircleDashedNumber9,
    CircleDottedLetterA,
    CircleDottedLetterB,
    CircleDottedLetterC,
    CircleDottedLetterD,
    CircleDottedLetterE,
    CircleDottedLetterF,
    CircleDottedLetterG,
    CircleDottedLetterH,
    CircleDottedLetterI,
    CircleDottedLetterJ,
    CircleDottedLetterK,
    CircleDottedLetterL,
    CircleDottedLetterM,
    CircleDottedLetterN,
    CircleDottedLetterO,
    CircleDottedLetterP,
    CircleDottedLetterQ,
    CircleDottedLetterR,
    CircleDottedLetterS,
    CircleDottedLetterT,
    CircleDottedLetterU,
    CircleDottedLetterV,
    CircleDottedLetterW,
    CircleDottedLetterX,
    CircleDottedLetterY,
    CircleDottedLetterZ,
    CircleLetterA,
    CircleLetterB,
    CircleLetterC,
    CircleLetterD,
    CircleLetterE,
    CircleLetterF,
    CircleLetterG,
    CircleLetterH,
    CircleLetterI,
    CircleLetterJ,
    CircleLetterK,
    CircleLetterL,
    CircleLetterM,
    CircleLetterN,
    CircleLetterO,
    CircleLetterP,
    CircleLetterQ,
    CircleLetterR,
    CircleLetterS,
    CircleLetterT,
    CircleLetterU,
    CircleLetterV,
    CircleLetterW,
    CircleLetterX,
    CircleLetterY,
    CircleLetterZ,
    CircleNumber0,
    CircleNumber1,
    CircleNumber2,
    CircleNumber3,
    CircleNumber4,
    CircleNumber5,
    CircleNumber6,
    CircleNumber7,
    CircleNumber8,
    CircleNumber9,
    ClearFormatting,
    Clipboard,
    ClipboardCheck,
    ClipboardCopy,
    ClipboardData,
    ClipboardHeart,
    ClipboardList,
    ClipboardOff,
    ClipboardPlus,
    ClipboardSearch,
    ClipboardSmile,
    ClipboardText,
    ClipboardTypography,
    ClipboardX,
    Code,
    CodeAsterisk,
    CodeCircle,
    CodeCircle2,
    CodeDots,
    CodeMinus,
    CodeOff,
    CodePlus,
    Columns,
    Columns1,
    Columns2,
    Columns3,
    ColumnsOff,
    Contract,
    Copy,
    CopyCheck,
    CopyMinus,
    CopyOff,
    CopyPlus,
    CopyX,
    CursorText,
    Delta,
    Emphasis,
    Eraser,
    EraserOff,
    File,
    File3d,
    FileAi,
    FileAlert,
    FileAnalytics,
    FileArrowLeft,
    FileArrowRight,
    FileBarcode,
    FileBitcoin,
    FileBroken,
    FileCertificate,
    FileChart,
    FileCheck,
    FileCode,
    FileCode2,
    FileCv,
    FileDatabase,
    FileDelta,
    FileDescription,
    FileDiff,
    FileDigit,
    FileDislike,
    FileDollar,
    FileDots,
    FileDownload,
    FileEuro,
    FileExcel,
    FileExport,
    FileFunction,
    FileHorizontal,
    FileImport,
    FileInfinity,
    FileInfo,
    FileInvoice,
    FileIsr,
    FileLambda,
    FileLike,
    FileMinus,
    FileMusic,
    FileNeutral,
    FileOff,
    FileOrientation,
    FilePencil,
    FilePercent,
    FilePhone,
    FilePlus,
    FilePower,
    FileReport,
    FileRss,
    FileSad,
    FileScissors,
    FileSearch,
    FileSettings,
    FileShredder,
    FileSignal,
    FileSmile,
    FileSpark,
    FileSpreadsheet,
    FileStack,
    FileStar,
    FileSymlink,
    FileText,
    FileTextAi,
    FileTextShield,
    FileTextSpark,
    FileTime,
    FileTypeBmp,
    FileTypeCss,
    FileTypeCsv,
    FileTypeDoc,
    FileTypeDocx,
    FileTypeHtml,
    FileTypeJpg,
    FileTypeJs,
    FileTypeJsx,
    FileTypePdf,
    FileTypePhp,
    FileTypePng,
    FileTypePpt,
    FileTypeRs,
    FileTypeSql,
    FileTypeSvg,
    FileTypeTs,
    FileTypeTsx,
    FileTypeTxt,
    FileTypeVue,
    FileTypeXls,
    FileTypeXml,
    FileTypeZip,
    FileTypography,
    FileUnknown,
    FileUpload,
    FileVector,
    FileWord,
    FileX,
    FileZip,
    Files,
    FilesOff,
    FloatCenter,
    FloatLeft,
    FloatNone,
    FloatRight,
    Folder,
    FolderBolt,
    FolderCancel,
    FolderCheck,
    FolderCode,
    FolderCog,
    FolderDollar,
    FolderDown,
    FolderExclamation,
    FolderHeart,
    FolderMinus,
    FolderOff,
    FolderOpen,
    FolderPause,
    FolderPin,
    FolderPlus,
    FolderQuestion,
    FolderRoot,
    FolderSearch,
    FolderShare,
    FolderStar,
    FolderSymlink,
    FolderUp,
    FolderX,
    Folders,
    FoldersOff,
    Forms,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Heading,
    HeadingOff,
    HexagonLetterA,
    HexagonLetterB,
    HexagonLetterC,
    HexagonLetterD,
    HexagonLetterE,
    HexagonLetterF,
    HexagonLetterG,
    HexagonLetterH,
    HexagonLetterI,
    HexagonLetterJ,
    HexagonLetterK,
    HexagonLetterL,
    HexagonLetterM,
    HexagonLetterN,
    HexagonLetterO,
    HexagonLetterP,
    HexagonLetterQ,
    HexagonLetterR,
    HexagonLetterS,
    HexagonLetterT,
    HexagonLetterU,
    HexagonLetterV,
    HexagonLetterW,
    HexagonLetterX,
    HexagonLetterY,
    HexagonLetterZ,
    HexagonNumber0,
    HexagonNumber1,
    HexagonNumber2,
    HexagonNumber3,
    HexagonNumber4,
    HexagonNumber5,
    HexagonNumber6,
    HexagonNumber7,
    HexagonNumber8,
    HexagonNumber9,
    Highlight,
    HighlightOff,
    IndentDecrease,
    IndentIncrease,
    InputSearch,
    Invoice,
    Italic,
    Kerning,
    Lambda,
    Language,
    LanguageHiragana,
    LanguageKatakana,
    LanguageOff,
    LetterA,
    LetterASmall,
    LetterB,
    LetterBSmall,
    LetterC,
    LetterCSmall,
    LetterCase,
    LetterCaseLower,
    LetterCaseToggle,
    LetterCaseUpper,
    LetterD,
    LetterDSmall,
    LetterE,
    LetterESmall,
    LetterF,
    LetterFSmall,
    LetterG,
    LetterGSmall,
    LetterH,
    LetterHSmall,
    LetterI,
    LetterISmall,
    LetterJ,
    LetterJSmall,
    LetterK,
    LetterKSmall,
    LetterL,
    LetterLSmall,
    LetterM,
    LetterMSmall,
    LetterN,
    LetterNSmall,
    LetterO,
    LetterOSmall,
    LetterP,
    LetterPSmall,
    LetterQ,
    LetterQSmall,
    LetterR,
    LetterRSmall,
    LetterS,
    LetterSSmall,
    LetterSpacing,
    LetterT,
    LetterTSmall,
    LetterU,
    LetterUSmall,
    LetterV,
    LetterVSmall,
    LetterW,
    LetterWSmall,
    LetterX,
    LetterXSmall,
    LetterY,
    LetterYSmall,
    LetterZ,
    LetterZSmall,
    License,
    LicenseOff,
    LineHeight,
    Link,
    LinkMinus,
    LinkOff,
    LinkPlus,
    List,
    ListCheck,
    ListDetails,
    ListNumbers,
    ListSearch,
    ManualGearbox,
    Markdown,
    MarkdownOff,
    News,
    NewsOff,
    Notdef,
    Note,
    NoteOff,
    Notebook,
    NotebookOff,
    Notes,
    NotesOff,
    Number,
    Number0,
    Number0Small,
    Number1,
    Number1Small,
    Number10,
    Number10Small,
    Number100Small,
    Number11,
    Number11Small,
    Number12Small,
    Number123,
    Number13Small,
    Number14Small,
    Number15Small,
    Number16Small,
    Number17Small,
    Number18Small,
    Number19Small,
    Number2,
    Number2Small,
    Number20Small,
    Number21Small,
    Number22Small,
    Number23Small,
    Number24Small,
    Number25Small,
    Number26Small,
    Number27Small,
    Number28Small,
    Number29Small,
    Number3,
    Number3Small,
    Number30Small,
    Number31Small,
    Number32Small,
    Number33Small,
    Number34Small,
    Number35Small,
    Number36Small,
    Number37Small,
    Number38Small,
    Number39Small,
    Number4,
    Number4Small,
    Number40Small,
    Number41Small,
    Number42Small,
    Number43Small,
    Number44Small,
    Number45Small,
    Number46Small,
    Number47Small,
    Number48Small,
    Number49Small,
    Number5,
    Number5Small,
    Number50Small,
    Number51Small,
    Number52Small,
    Number53Small,
    Number54Small,
    Number55Small,
    Number56Small,
    Number57Small,
    Number58Small,
    Number59Small,
    Number6,
    Number6Small,
    Number60Small,
    Number61Small,
    Number62Small,
    Number63Small,
    Number64Small,
    Number65Small,
    Number66Small,
    Number67Small,
    Number68Small,
    Number69Small,
    Number7,
    Number7Small,
    Number70Small,
    Number71Small,
    Number72Small,
    Number73Small,
    Number74Small,
    Number75Small,
    Number76Small,
    Number77Small,
    Number78Small,
    Number79Small,
    Number8,
    Number8Small,
    Number80Small,
    Number81Small,
    Number82Small,
    Number83Small,
    Number84Small,
    Number85Small,
    Number86Small,
    Number87Small,
    Number88Small,
    Number89Small,
    Number9,
    Number9Small,
    Number90Small,
    Number91Small,
    Number92Small,
    Number93Small,
    Number94Small,
    Number95Small,
    Number96Small,
    Number97Small,
    Number98Small,
    Number99Small,
    Numbers,
    Overline,
    PageBreak,
    Paperclip,
    PentagonNumber0,
    PentagonNumber1,
    PentagonNumber2,
    PentagonNumber3,
    PentagonNumber4,
    PentagonNumber5,
    PentagonNumber6,
    PentagonNumber7,
    PentagonNumber8,
    PentagonNumber9,
    Pilcrow,
    PilcrowLeft,
    PilcrowRight,
    Presentation,
    PresentationAnalytics,
    PresentationOff,
    Quote,
    QuoteOff,
    QuoteOpen,
    Quotes,
    Receipt,
    Receipt2,
    ReceiptDollar,
    ReceiptOff,
    ReceiptRefund,
    ReceiptTax,
    Regex,
    RegexOff,
    Report,
    ReportAnalytics,
    ReportMedical,
    ReportMoney,
    ReportOff,
    ReportSearch,
    RosetteNumber0,
    RosetteNumber1,
    RosetteNumber2,
    RosetteNumber3,
    RosetteNumber4,
    RosetteNumber5,
    RosetteNumber6,
    RosetteNumber7,
    RosetteNumber8,
    RosetteNumber9,
    RubberStamp,
    RubberStampOff,
    ScanLetterA,
    ScanLetterT,
    Scribble,
    ScribbleOff,
    Script,
    ScriptMinus,
    ScriptPlus,
    ScriptX,
    SectionSign,
    Separator,
    SeparatorHorizontal,
    SeparatorVertical,
    Signature,
    SignatureOff,
    Slideshow,
    Sort09,
    Sort90,
    SortAZ,
    SortAscending,
    SortAscending2,
    SortAscendingLetters,
    SortAscendingNumbers,
    SortDescending,
    SortDescending2,
    SortDescendingLetters,
    SortDescendingNumbers,
    SortZA,
    Space,
    SpaceOff,
    SpacingHorizontal,
    SpacingVertical,
    SquareLetterA,
    SquareLetterB,
    SquareLetterC,
    SquareLetterD,
    SquareLetterE,
    SquareLetterF,
    SquareLetterG,
    SquareLetterH,
    SquareLetterI,
    SquareLetterJ,
    SquareLetterK,
    SquareLetterL,
    SquareLetterM,
    SquareLetterN,
    SquareLetterO,
    SquareLetterP,
    SquareLetterQ,
    SquareLetterR,
    SquareLetterS,
    SquareLetterT,
    SquareLetterU,
    SquareLetterV,
    SquareLetterW,
    SquareLetterX,
    SquareLetterY,
    SquareLetterZ,
    SquareNumber0,
    SquareNumber1,
    SquareNumber2,
    SquareNumber3,
    SquareNumber4,
    SquareNumber5,
    SquareNumber6,
    SquareNumber7,
    SquareNumber8,
    SquareNumber9,
    SquareRoundedLetterA,
    SquareRoundedLetterB,
    SquareRoundedLetterC,
    SquareRoundedLetterD,
    SquareRoundedLetterE,
    SquareRoundedLetterF,
    SquareRoundedLetterG,
    SquareRoundedLetterH,
    SquareRoundedLetterI,
    SquareRoundedLetterJ,
    SquareRoundedLetterK,
    SquareRoundedLetterL,
    SquareRoundedLetterM,
    SquareRoundedLetterN,
    SquareRoundedLetterO,
    SquareRoundedLetterP,
    SquareRoundedLetterQ,
    SquareRoundedLetterR,
    SquareRoundedLetterS,
    SquareRoundedLetterT,
    SquareRoundedLetterU,
    SquareRoundedLetterV,
    SquareRoundedLetterW,
    SquareRoundedLetterX,
    SquareRoundedLetterY,
    SquareRoundedLetterZ,
    SquareRoundedNumber0,
    SquareRoundedNumber1,
    SquareRoundedNumber2,
    SquareRoundedNumber3,
    SquareRoundedNumber4,
    SquareRoundedNumber5,
    SquareRoundedNumber6,
    SquareRoundedNumber7,
    SquareRoundedNumber8,
    SquareRoundedNumber9,
    Strikethrough,
    Subscript,
    Superscript,
    Tex,
    TextCaption,
    TextColor,
    TextDecrease,
    TextDirectionLtr,
    TextDirectionRtl,
    TextGrammar,
    TextIncrease,
    TextOrientation,
    TextPlus,
    TextRecognition,
    TextSize,
    TextSpellcheck,
    TextWrap,
    TextWrapColumn,
    TextWrapDisabled,
    Ticket,
    TicketOff,
    Typography,
    TypographyOff,
    Underline,
    Unlink,
    Vocabulary,
    VocabularyOff,
    Writing,
    WritingOff,
    WritingSign,
    WritingSignOff,
}

impl DocumentIcon {
    /// Returns all available icons in this category.
    pub fn all() -> &'static [Self] {
        &[Self::AB2, Self::ABOff, Self::Abc, Self::AlignBoxBottomCenter, Self::AlignBoxBottomLeft, Self::AlignBoxBottomRight, Self::AlignBoxCenterBottom, Self::AlignBoxCenterMiddle, Self::AlignBoxCenterStretch, Self::AlignBoxCenterTop, Self::AlignBoxLeftBottom, Self::AlignBoxLeftMiddle, Self::AlignBoxLeftStretch, Self::AlignBoxLeftTop, Self::AlignBoxRightBottom, Self::AlignBoxRightMiddle, Self::AlignBoxRightStretch, Self::AlignBoxRightTop, Self::AlignBoxTopCenter, Self::AlignBoxTopLeft, Self::AlignBoxTopRight, Self::AlignCenter, Self::AlignJustified, Self::AlignLeft, Self::AlignLeft2, Self::AlignRight, Self::AlignRight2, Self::Alpha, Self::AlphabetArabic, Self::AlphabetBangla, Self::AlphabetCyrillic, Self::AlphabetGreek, Self::AlphabetHebrew, Self::AlphabetKorean, Self::AlphabetLatin, Self::AlphabetPolish, Self::AlphabetRunes, Self::AlphabetThai, Self::Archive, Self::ArchiveOff, Self::Article, Self::ArticleOff, Self::Asterisk, Self::AsteriskSimple, Self::At, Self::AtOff, Self::Backspace, Self::Ballpen, Self::BallpenOff, Self::Baseline, Self::BaselineDensityLarge, Self::BaselineDensityMedium, Self::BaselineDensitySmall, Self::Beta, Self::Bible, Self::Blockquote, Self::Bold, Self::BoldOff, Self::Book, Self::Book2, Self::BookDownload, Self::BookOff, Self::BookUpload, Self::Bookmark, Self::BookmarkAi, Self::BookmarkEdit, Self::BookmarkMinus, Self::BookmarkOff, Self::BookmarkPlus, Self::BookmarkQuestion, Self::Bookmarks, Self::BookmarksOff, Self::Books, Self::BooksOff, Self::BoxMultiple0, Self::BoxMultiple1, Self::BoxMultiple2, Self::BoxMultiple3, Self::BoxMultiple4, Self::BoxMultiple5, Self::BoxMultiple6, Self::BoxMultiple7, Self::BoxMultiple8, Self::BoxMultiple9, Self::Certificate, Self::Certificate2, Self::Certificate2Off, Self::CertificateOff, Self::Chalkboard, Self::ChalkboardOff, Self::CircleDashedLetterA, Self::CircleDashedLetterB, Self::CircleDashedLetterC, Self::CircleDashedLetterD, Self::CircleDashedLetterE, Self::CircleDashedLetterF, Self::CircleDashedLetterG, Self::CircleDashedLetterH, Self::CircleDashedLetterI, Self::CircleDashedLetterJ, Self::CircleDashedLetterK, Self::CircleDashedLetterL, Self::CircleDashedLetterM, Self::CircleDashedLetterN, Self::CircleDashedLetterO, Self::CircleDashedLetterP, Self::CircleDashedLetterQ, Self::CircleDashedLetterR, Self::CircleDashedLetterS, Self::CircleDashedLetterT, Self::CircleDashedLetterU, Self::CircleDashedLetterV, Self::CircleDashedLetterW, Self::CircleDashedLetterX, Self::CircleDashedLetterY, Self::CircleDashedLetterZ, Self::CircleDashedNumber0, Self::CircleDashedNumber1, Self::CircleDashedNumber2, Self::CircleDashedNumber3, Self::CircleDashedNumber4, Self::CircleDashedNumber5, Self::CircleDashedNumber6, Self::CircleDashedNumber7, Self::CircleDashedNumber8, Self::CircleDashedNumber9, Self::CircleDottedLetterA, Self::CircleDottedLetterB, Self::CircleDottedLetterC, Self::CircleDottedLetterD, Self::CircleDottedLetterE, Self::CircleDottedLetterF, Self::CircleDottedLetterG, Self::CircleDottedLetterH, Self::CircleDottedLetterI, Self::CircleDottedLetterJ, Self::CircleDottedLetterK, Self::CircleDottedLetterL, Self::CircleDottedLetterM, Self::CircleDottedLetterN, Self::CircleDottedLetterO, Self::CircleDottedLetterP, Self::CircleDottedLetterQ, Self::CircleDottedLetterR, Self::CircleDottedLetterS, Self::CircleDottedLetterT, Self::CircleDottedLetterU, Self::CircleDottedLetterV, Self::CircleDottedLetterW, Self::CircleDottedLetterX, Self::CircleDottedLetterY, Self::CircleDottedLetterZ, Self::CircleLetterA, Self::CircleLetterB, Self::CircleLetterC, Self::CircleLetterD, Self::CircleLetterE, Self::CircleLetterF, Self::CircleLetterG, Self::CircleLetterH, Self::CircleLetterI, Self::CircleLetterJ, Self::CircleLetterK, Self::CircleLetterL, Self::CircleLetterM, Self::CircleLetterN, Self::CircleLetterO, Self::CircleLetterP, Self::CircleLetterQ, Self::CircleLetterR, Self::CircleLetterS, Self::CircleLetterT, Self::CircleLetterU, Self::CircleLetterV, Self::CircleLetterW, Self::CircleLetterX, Self::CircleLetterY, Self::CircleLetterZ, Self::CircleNumber0, Self::CircleNumber1, Self::CircleNumber2, Self::CircleNumber3, Self::CircleNumber4, Self::CircleNumber5, Self::CircleNumber6, Self::CircleNumber7, Self::CircleNumber8, Self::CircleNumber9, Self::ClearFormatting, Self::Clipboard, Self::ClipboardCheck, Self::ClipboardCopy, Self::ClipboardData, Self::ClipboardHeart, Self::ClipboardList, Self::ClipboardOff, Self::ClipboardPlus, Self::ClipboardSearch, Self::ClipboardSmile, Self::ClipboardText, Self::ClipboardTypography, Self::ClipboardX, Self::Code, Self::CodeAsterisk, Self::CodeCircle, Self::CodeCircle2, Self::CodeDots, Self::CodeMinus, Self::CodeOff, Self::CodePlus, Self::Columns, Self::Columns1, Self::Columns2, Self::Columns3, Self::ColumnsOff, Self::Contract, Self::Copy, Self::CopyCheck, Self::CopyMinus, Self::CopyOff, Self::CopyPlus, Self::CopyX, Self::CursorText, Self::Delta, Self::Emphasis, Self::Eraser, Self::EraserOff, Self::File, Self::File3d, Self::FileAi, Self::FileAlert, Self::FileAnalytics, Self::FileArrowLeft, Self::FileArrowRight, Self::FileBarcode, Self::FileBitcoin, Self::FileBroken, Self::FileCertificate, Self::FileChart, Self::FileCheck, Self::FileCode, Self::FileCode2, Self::FileCv, Self::FileDatabase, Self::FileDelta, Self::FileDescription, Self::FileDiff, Self::FileDigit, Self::FileDislike, Self::FileDollar, Self::FileDots, Self::FileDownload, Self::FileEuro, Self::FileExcel, Self::FileExport, Self::FileFunction, Self::FileHorizontal, Self::FileImport, Self::FileInfinity, Self::FileInfo, Self::FileInvoice, Self::FileIsr, Self::FileLambda, Self::FileLike, Self::FileMinus, Self::FileMusic, Self::FileNeutral, Self::FileOff, Self::FileOrientation, Self::FilePencil, Self::FilePercent, Self::FilePhone, Self::FilePlus, Self::FilePower, Self::FileReport, Self::FileRss, Self::FileSad, Self::FileScissors, Self::FileSearch, Self::FileSettings, Self::FileShredder, Self::FileSignal, Self::FileSmile, Self::FileSpark, Self::FileSpreadsheet, Self::FileStack, Self::FileStar, Self::FileSymlink, Self::FileText, Self::FileTextAi, Self::FileTextShield, Self::FileTextSpark, Self::FileTime, Self::FileTypeBmp, Self::FileTypeCss, Self::FileTypeCsv, Self::FileTypeDoc, Self::FileTypeDocx, Self::FileTypeHtml, Self::FileTypeJpg, Self::FileTypeJs, Self::FileTypeJsx, Self::FileTypePdf, Self::FileTypePhp, Self::FileTypePng, Self::FileTypePpt, Self::FileTypeRs, Self::FileTypeSql, Self::FileTypeSvg, Self::FileTypeTs, Self::FileTypeTsx, Self::FileTypeTxt, Self::FileTypeVue, Self::FileTypeXls, Self::FileTypeXml, Self::FileTypeZip, Self::FileTypography, Self::FileUnknown, Self::FileUpload, Self::FileVector, Self::FileWord, Self::FileX, Self::FileZip, Self::Files, Self::FilesOff, Self::FloatCenter, Self::FloatLeft, Self::FloatNone, Self::FloatRight, Self::Folder, Self::FolderBolt, Self::FolderCancel, Self::FolderCheck, Self::FolderCode, Self::FolderCog, Self::FolderDollar, Self::FolderDown, Self::FolderExclamation, Self::FolderHeart, Self::FolderMinus, Self::FolderOff, Self::FolderOpen, Self::FolderPause, Self::FolderPin, Self::FolderPlus, Self::FolderQuestion, Self::FolderRoot, Self::FolderSearch, Self::FolderShare, Self::FolderStar, Self::FolderSymlink, Self::FolderUp, Self::FolderX, Self::Folders, Self::FoldersOff, Self::Forms, Self::H1, Self::H2, Self::H3, Self::H4, Self::H5, Self::H6, Self::Heading, Self::HeadingOff, Self::HexagonLetterA, Self::HexagonLetterB, Self::HexagonLetterC, Self::HexagonLetterD, Self::HexagonLetterE, Self::HexagonLetterF, Self::HexagonLetterG, Self::HexagonLetterH, Self::HexagonLetterI, Self::HexagonLetterJ, Self::HexagonLetterK, Self::HexagonLetterL, Self::HexagonLetterM, Self::HexagonLetterN, Self::HexagonLetterO, Self::HexagonLetterP, Self::HexagonLetterQ, Self::HexagonLetterR, Self::HexagonLetterS, Self::HexagonLetterT, Self::HexagonLetterU, Self::HexagonLetterV, Self::HexagonLetterW, Self::HexagonLetterX, Self::HexagonLetterY, Self::HexagonLetterZ, Self::HexagonNumber0, Self::HexagonNumber1, Self::HexagonNumber2, Self::HexagonNumber3, Self::HexagonNumber4, Self::HexagonNumber5, Self::HexagonNumber6, Self::HexagonNumber7, Self::HexagonNumber8, Self::HexagonNumber9, Self::Highlight, Self::HighlightOff, Self::IndentDecrease, Self::IndentIncrease, Self::InputSearch, Self::Invoice, Self::Italic, Self::Kerning, Self::Lambda, Self::Language, Self::LanguageHiragana, Self::LanguageKatakana, Self::LanguageOff, Self::LetterA, Self::LetterASmall, Self::LetterB, Self::LetterBSmall, Self::LetterC, Self::LetterCSmall, Self::LetterCase, Self::LetterCaseLower, Self::LetterCaseToggle, Self::LetterCaseUpper, Self::LetterD, Self::LetterDSmall, Self::LetterE, Self::LetterESmall, Self::LetterF, Self::LetterFSmall, Self::LetterG, Self::LetterGSmall, Self::LetterH, Self::LetterHSmall, Self::LetterI, Self::LetterISmall, Self::LetterJ, Self::LetterJSmall, Self::LetterK, Self::LetterKSmall, Self::LetterL, Self::LetterLSmall, Self::LetterM, Self::LetterMSmall, Self::LetterN, Self::LetterNSmall, Self::LetterO, Self::LetterOSmall, Self::LetterP, Self::LetterPSmall, Self::LetterQ, Self::LetterQSmall, Self::LetterR, Self::LetterRSmall, Self::LetterS, Self::LetterSSmall, Self::LetterSpacing, Self::LetterT, Self::LetterTSmall, Self::LetterU, Self::LetterUSmall, Self::LetterV, Self::LetterVSmall, Self::LetterW, Self::LetterWSmall, Self::LetterX, Self::LetterXSmall, Self::LetterY, Self::LetterYSmall, Self::LetterZ, Self::LetterZSmall, Self::License, Self::LicenseOff, Self::LineHeight, Self::Link, Self::LinkMinus, Self::LinkOff, Self::LinkPlus, Self::List, Self::ListCheck, Self::ListDetails, Self::ListNumbers, Self::ListSearch, Self::ManualGearbox, Self::Markdown, Self::MarkdownOff, Self::News, Self::NewsOff, Self::Notdef, Self::Note, Self::NoteOff, Self::Notebook, Self::NotebookOff, Self::Notes, Self::NotesOff, Self::Number, Self::Number0, Self::Number0Small, Self::Number1, Self::Number1Small, Self::Number10, Self::Number10Small, Self::Number100Small, Self::Number11, Self::Number11Small, Self::Number12Small, Self::Number123, Self::Number13Small, Self::Number14Small, Self::Number15Small, Self::Number16Small, Self::Number17Small, Self::Number18Small, Self::Number19Small, Self::Number2, Self::Number2Small, Self::Number20Small, Self::Number21Small, Self::Number22Small, Self::Number23Small, Self::Number24Small, Self::Number25Small, Self::Number26Small, Self::Number27Small, Self::Number28Small, Self::Number29Small, Self::Number3, Self::Number3Small, Self::Number30Small, Self::Number31Small, Self::Number32Small, Self::Number33Small, Self::Number34Small, Self::Number35Small, Self::Number36Small, Self::Number37Small, Self::Number38Small, Self::Number39Small, Self::Number4, Self::Number4Small, Self::Number40Small, Self::Number41Small, Self::Number42Small, Self::Number43Small, Self::Number44Small, Self::Number45Small, Self::Number46Small, Self::Number47Small, Self::Number48Small, Self::Number49Small, Self::Number5, Self::Number5Small, Self::Number50Small, Self::Number51Small, Self::Number52Small, Self::Number53Small, Self::Number54Small, Self::Number55Small, Self::Number56Small, Self::Number57Small, Self::Number58Small, Self::Number59Small, Self::Number6, Self::Number6Small, Self::Number60Small, Self::Number61Small, Self::Number62Small, Self::Number63Small, Self::Number64Small, Self::Number65Small, Self::Number66Small, Self::Number67Small, Self::Number68Small, Self::Number69Small, Self::Number7, Self::Number7Small, Self::Number70Small, Self::Number71Small, Self::Number72Small, Self::Number73Small, Self::Number74Small, Self::Number75Small, Self::Number76Small, Self::Number77Small, Self::Number78Small, Self::Number79Small, Self::Number8, Self::Number8Small, Self::Number80Small, Self::Number81Small, Self::Number82Small, Self::Number83Small, Self::Number84Small, Self::Number85Small, Self::Number86Small, Self::Number87Small, Self::Number88Small, Self::Number89Small, Self::Number9, Self::Number9Small, Self::Number90Small, Self::Number91Small, Self::Number92Small, Self::Number93Small, Self::Number94Small, Self::Number95Small, Self::Number96Small, Self::Number97Small, Self::Number98Small, Self::Number99Small, Self::Numbers, Self::Overline, Self::PageBreak, Self::Paperclip, Self::PentagonNumber0, Self::PentagonNumber1, Self::PentagonNumber2, Self::PentagonNumber3, Self::PentagonNumber4, Self::PentagonNumber5, Self::PentagonNumber6, Self::PentagonNumber7, Self::PentagonNumber8, Self::PentagonNumber9, Self::Pilcrow, Self::PilcrowLeft, Self::PilcrowRight, Self::Presentation, Self::PresentationAnalytics, Self::PresentationOff, Self::Quote, Self::QuoteOff, Self::QuoteOpen, Self::Quotes, Self::Receipt, Self::Receipt2, Self::ReceiptDollar, Self::ReceiptOff, Self::ReceiptRefund, Self::ReceiptTax, Self::Regex, Self::RegexOff, Self::Report, Self::ReportAnalytics, Self::ReportMedical, Self::ReportMoney, Self::ReportOff, Self::ReportSearch, Self::RosetteNumber0, Self::RosetteNumber1, Self::RosetteNumber2, Self::RosetteNumber3, Self::RosetteNumber4, Self::RosetteNumber5, Self::RosetteNumber6, Self::RosetteNumber7, Self::RosetteNumber8, Self::RosetteNumber9, Self::RubberStamp, Self::RubberStampOff, Self::ScanLetterA, Self::ScanLetterT, Self::Scribble, Self::ScribbleOff, Self::Script, Self::ScriptMinus, Self::ScriptPlus, Self::ScriptX, Self::SectionSign, Self::Separator, Self::SeparatorHorizontal, Self::SeparatorVertical, Self::Signature, Self::SignatureOff, Self::Slideshow, Self::Sort09, Self::Sort90, Self::SortAZ, Self::SortAscending, Self::SortAscending2, Self::SortAscendingLetters, Self::SortAscendingNumbers, Self::SortDescending, Self::SortDescending2, Self::SortDescendingLetters, Self::SortDescendingNumbers, Self::SortZA, Self::Space, Self::SpaceOff, Self::SpacingHorizontal, Self::SpacingVertical, Self::SquareLetterA, Self::SquareLetterB, Self::SquareLetterC, Self::SquareLetterD, Self::SquareLetterE, Self::SquareLetterF, Self::SquareLetterG, Self::SquareLetterH, Self::SquareLetterI, Self::SquareLetterJ, Self::SquareLetterK, Self::SquareLetterL, Self::SquareLetterM, Self::SquareLetterN, Self::SquareLetterO, Self::SquareLetterP, Self::SquareLetterQ, Self::SquareLetterR, Self::SquareLetterS, Self::SquareLetterT, Self::SquareLetterU, Self::SquareLetterV, Self::SquareLetterW, Self::SquareLetterX, Self::SquareLetterY, Self::SquareLetterZ, Self::SquareNumber0, Self::SquareNumber1, Self::SquareNumber2, Self::SquareNumber3, Self::SquareNumber4, Self::SquareNumber5, Self::SquareNumber6, Self::SquareNumber7, Self::SquareNumber8, Self::SquareNumber9, Self::SquareRoundedLetterA, Self::SquareRoundedLetterB, Self::SquareRoundedLetterC, Self::SquareRoundedLetterD, Self::SquareRoundedLetterE, Self::SquareRoundedLetterF, Self::SquareRoundedLetterG, Self::SquareRoundedLetterH, Self::SquareRoundedLetterI, Self::SquareRoundedLetterJ, Self::SquareRoundedLetterK, Self::SquareRoundedLetterL, Self::SquareRoundedLetterM, Self::SquareRoundedLetterN, Self::SquareRoundedLetterO, Self::SquareRoundedLetterP, Self::SquareRoundedLetterQ, Self::SquareRoundedLetterR, Self::SquareRoundedLetterS, Self::SquareRoundedLetterT, Self::SquareRoundedLetterU, Self::SquareRoundedLetterV, Self::SquareRoundedLetterW, Self::SquareRoundedLetterX, Self::SquareRoundedLetterY, Self::SquareRoundedLetterZ, Self::SquareRoundedNumber0, Self::SquareRoundedNumber1, Self::SquareRoundedNumber2, Self::SquareRoundedNumber3, Self::SquareRoundedNumber4, Self::SquareRoundedNumber5, Self::SquareRoundedNumber6, Self::SquareRoundedNumber7, Self::SquareRoundedNumber8, Self::SquareRoundedNumber9, Self::Strikethrough, Self::Subscript, Self::Superscript, Self::Tex, Self::TextCaption, Self::TextColor, Self::TextDecrease, Self::TextDirectionLtr, Self::TextDirectionRtl, Self::TextGrammar, Self::TextIncrease, Self::TextOrientation, Self::TextPlus, Self::TextRecognition, Self::TextSize, Self::TextSpellcheck, Self::TextWrap, Self::TextWrapColumn, Self::TextWrapDisabled, Self::Ticket, Self::TicketOff, Self::Typography, Self::TypographyOff, Self::Underline, Self::Unlink, Self::Vocabulary, Self::VocabularyOff, Self::Writing, Self::WritingOff, Self::WritingSign, Self::WritingSignOff]
    }

    /// Returns the icon count.
    pub fn count() -> usize {
        793
    }

    /// Creates an icon from its kebab-case name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "a-b-2" => Some(Self::AB2),
            "a-b-off" => Some(Self::ABOff),
            "abc" => Some(Self::Abc),
            "align-box-bottom-center" => Some(Self::AlignBoxBottomCenter),
            "align-box-bottom-left" => Some(Self::AlignBoxBottomLeft),
            "align-box-bottom-right" => Some(Self::AlignBoxBottomRight),
            "align-box-center-bottom" => Some(Self::AlignBoxCenterBottom),
            "align-box-center-middle" => Some(Self::AlignBoxCenterMiddle),
            "align-box-center-stretch" => Some(Self::AlignBoxCenterStretch),
            "align-box-center-top" => Some(Self::AlignBoxCenterTop),
            "align-box-left-bottom" => Some(Self::AlignBoxLeftBottom),
            "align-box-left-middle" => Some(Self::AlignBoxLeftMiddle),
            "align-box-left-stretch" => Some(Self::AlignBoxLeftStretch),
            "align-box-left-top" => Some(Self::AlignBoxLeftTop),
            "align-box-right-bottom" => Some(Self::AlignBoxRightBottom),
            "align-box-right-middle" => Some(Self::AlignBoxRightMiddle),
            "align-box-right-stretch" => Some(Self::AlignBoxRightStretch),
            "align-box-right-top" => Some(Self::AlignBoxRightTop),
            "align-box-top-center" => Some(Self::AlignBoxTopCenter),
            "align-box-top-left" => Some(Self::AlignBoxTopLeft),
            "align-box-top-right" => Some(Self::AlignBoxTopRight),
            "align-center" => Some(Self::AlignCenter),
            "align-justified" => Some(Self::AlignJustified),
            "align-left" => Some(Self::AlignLeft),
            "align-left-2" => Some(Self::AlignLeft2),
            "align-right" => Some(Self::AlignRight),
            "align-right-2" => Some(Self::AlignRight2),
            "alpha" => Some(Self::Alpha),
            "alphabet-arabic" => Some(Self::AlphabetArabic),
            "alphabet-bangla" => Some(Self::AlphabetBangla),
            "alphabet-cyrillic" => Some(Self::AlphabetCyrillic),
            "alphabet-greek" => Some(Self::AlphabetGreek),
            "alphabet-hebrew" => Some(Self::AlphabetHebrew),
            "alphabet-korean" => Some(Self::AlphabetKorean),
            "alphabet-latin" => Some(Self::AlphabetLatin),
            "alphabet-polish" => Some(Self::AlphabetPolish),
            "alphabet-runes" => Some(Self::AlphabetRunes),
            "alphabet-thai" => Some(Self::AlphabetThai),
            "archive" => Some(Self::Archive),
            "archive-off" => Some(Self::ArchiveOff),
            "article" => Some(Self::Article),
            "article-off" => Some(Self::ArticleOff),
            "asterisk" => Some(Self::Asterisk),
            "asterisk-simple" => Some(Self::AsteriskSimple),
            "at" => Some(Self::At),
            "at-off" => Some(Self::AtOff),
            "backspace" => Some(Self::Backspace),
            "ballpen" => Some(Self::Ballpen),
            "ballpen-off" => Some(Self::BallpenOff),
            "baseline" => Some(Self::Baseline),
            "baseline-density-large" => Some(Self::BaselineDensityLarge),
            "baseline-density-medium" => Some(Self::BaselineDensityMedium),
            "baseline-density-small" => Some(Self::BaselineDensitySmall),
            "beta" => Some(Self::Beta),
            "bible" => Some(Self::Bible),
            "blockquote" => Some(Self::Blockquote),
            "bold" => Some(Self::Bold),
            "bold-off" => Some(Self::BoldOff),
            "book" => Some(Self::Book),
            "book-2" => Some(Self::Book2),
            "book-download" => Some(Self::BookDownload),
            "book-off" => Some(Self::BookOff),
            "book-upload" => Some(Self::BookUpload),
            "bookmark" => Some(Self::Bookmark),
            "bookmark-ai" => Some(Self::BookmarkAi),
            "bookmark-edit" => Some(Self::BookmarkEdit),
            "bookmark-minus" => Some(Self::BookmarkMinus),
            "bookmark-off" => Some(Self::BookmarkOff),
            "bookmark-plus" => Some(Self::BookmarkPlus),
            "bookmark-question" => Some(Self::BookmarkQuestion),
            "bookmarks" => Some(Self::Bookmarks),
            "bookmarks-off" => Some(Self::BookmarksOff),
            "books" => Some(Self::Books),
            "books-off" => Some(Self::BooksOff),
            "box-multiple-0" => Some(Self::BoxMultiple0),
            "box-multiple-1" => Some(Self::BoxMultiple1),
            "box-multiple-2" => Some(Self::BoxMultiple2),
            "box-multiple-3" => Some(Self::BoxMultiple3),
            "box-multiple-4" => Some(Self::BoxMultiple4),
            "box-multiple-5" => Some(Self::BoxMultiple5),
            "box-multiple-6" => Some(Self::BoxMultiple6),
            "box-multiple-7" => Some(Self::BoxMultiple7),
            "box-multiple-8" => Some(Self::BoxMultiple8),
            "box-multiple-9" => Some(Self::BoxMultiple9),
            "certificate" => Some(Self::Certificate),
            "certificate-2" => Some(Self::Certificate2),
            "certificate-2-off" => Some(Self::Certificate2Off),
            "certificate-off" => Some(Self::CertificateOff),
            "chalkboard" => Some(Self::Chalkboard),
            "chalkboard-off" => Some(Self::ChalkboardOff),
            "circle-dashed-letter-a" => Some(Self::CircleDashedLetterA),
            "circle-dashed-letter-b" => Some(Self::CircleDashedLetterB),
            "circle-dashed-letter-c" => Some(Self::CircleDashedLetterC),
            "circle-dashed-letter-d" => Some(Self::CircleDashedLetterD),
            "circle-dashed-letter-e" => Some(Self::CircleDashedLetterE),
            "circle-dashed-letter-f" => Some(Self::CircleDashedLetterF),
            "circle-dashed-letter-g" => Some(Self::CircleDashedLetterG),
            "circle-dashed-letter-h" => Some(Self::CircleDashedLetterH),
            "circle-dashed-letter-i" => Some(Self::CircleDashedLetterI),
            "circle-dashed-letter-j" => Some(Self::CircleDashedLetterJ),
            "circle-dashed-letter-k" => Some(Self::CircleDashedLetterK),
            "circle-dashed-letter-l" => Some(Self::CircleDashedLetterL),
            "circle-dashed-letter-m" => Some(Self::CircleDashedLetterM),
            "circle-dashed-letter-n" => Some(Self::CircleDashedLetterN),
            "circle-dashed-letter-o" => Some(Self::CircleDashedLetterO),
            "circle-dashed-letter-p" => Some(Self::CircleDashedLetterP),
            "circle-dashed-letter-q" => Some(Self::CircleDashedLetterQ),
            "circle-dashed-letter-r" => Some(Self::CircleDashedLetterR),
            "circle-dashed-letter-s" => Some(Self::CircleDashedLetterS),
            "circle-dashed-letter-t" => Some(Self::CircleDashedLetterT),
            "circle-dashed-letter-u" => Some(Self::CircleDashedLetterU),
            "circle-dashed-letter-v" => Some(Self::CircleDashedLetterV),
            "circle-dashed-letter-w" => Some(Self::CircleDashedLetterW),
            "circle-dashed-letter-x" => Some(Self::CircleDashedLetterX),
            "circle-dashed-letter-y" => Some(Self::CircleDashedLetterY),
            "circle-dashed-letter-z" => Some(Self::CircleDashedLetterZ),
            "circle-dashed-number-0" => Some(Self::CircleDashedNumber0),
            "circle-dashed-number-1" => Some(Self::CircleDashedNumber1),
            "circle-dashed-number-2" => Some(Self::CircleDashedNumber2),
            "circle-dashed-number-3" => Some(Self::CircleDashedNumber3),
            "circle-dashed-number-4" => Some(Self::CircleDashedNumber4),
            "circle-dashed-number-5" => Some(Self::CircleDashedNumber5),
            "circle-dashed-number-6" => Some(Self::CircleDashedNumber6),
            "circle-dashed-number-7" => Some(Self::CircleDashedNumber7),
            "circle-dashed-number-8" => Some(Self::CircleDashedNumber8),
            "circle-dashed-number-9" => Some(Self::CircleDashedNumber9),
            "circle-dotted-letter-a" => Some(Self::CircleDottedLetterA),
            "circle-dotted-letter-b" => Some(Self::CircleDottedLetterB),
            "circle-dotted-letter-c" => Some(Self::CircleDottedLetterC),
            "circle-dotted-letter-d" => Some(Self::CircleDottedLetterD),
            "circle-dotted-letter-e" => Some(Self::CircleDottedLetterE),
            "circle-dotted-letter-f" => Some(Self::CircleDottedLetterF),
            "circle-dotted-letter-g" => Some(Self::CircleDottedLetterG),
            "circle-dotted-letter-h" => Some(Self::CircleDottedLetterH),
            "circle-dotted-letter-i" => Some(Self::CircleDottedLetterI),
            "circle-dotted-letter-j" => Some(Self::CircleDottedLetterJ),
            "circle-dotted-letter-k" => Some(Self::CircleDottedLetterK),
            "circle-dotted-letter-l" => Some(Self::CircleDottedLetterL),
            "circle-dotted-letter-m" => Some(Self::CircleDottedLetterM),
            "circle-dotted-letter-n" => Some(Self::CircleDottedLetterN),
            "circle-dotted-letter-o" => Some(Self::CircleDottedLetterO),
            "circle-dotted-letter-p" => Some(Self::CircleDottedLetterP),
            "circle-dotted-letter-q" => Some(Self::CircleDottedLetterQ),
            "circle-dotted-letter-r" => Some(Self::CircleDottedLetterR),
            "circle-dotted-letter-s" => Some(Self::CircleDottedLetterS),
            "circle-dotted-letter-t" => Some(Self::CircleDottedLetterT),
            "circle-dotted-letter-u" => Some(Self::CircleDottedLetterU),
            "circle-dotted-letter-v" => Some(Self::CircleDottedLetterV),
            "circle-dotted-letter-w" => Some(Self::CircleDottedLetterW),
            "circle-dotted-letter-x" => Some(Self::CircleDottedLetterX),
            "circle-dotted-letter-y" => Some(Self::CircleDottedLetterY),
            "circle-dotted-letter-z" => Some(Self::CircleDottedLetterZ),
            "circle-letter-a" => Some(Self::CircleLetterA),
            "circle-letter-b" => Some(Self::CircleLetterB),
            "circle-letter-c" => Some(Self::CircleLetterC),
            "circle-letter-d" => Some(Self::CircleLetterD),
            "circle-letter-e" => Some(Self::CircleLetterE),
            "circle-letter-f" => Some(Self::CircleLetterF),
            "circle-letter-g" => Some(Self::CircleLetterG),
            "circle-letter-h" => Some(Self::CircleLetterH),
            "circle-letter-i" => Some(Self::CircleLetterI),
            "circle-letter-j" => Some(Self::CircleLetterJ),
            "circle-letter-k" => Some(Self::CircleLetterK),
            "circle-letter-l" => Some(Self::CircleLetterL),
            "circle-letter-m" => Some(Self::CircleLetterM),
            "circle-letter-n" => Some(Self::CircleLetterN),
            "circle-letter-o" => Some(Self::CircleLetterO),
            "circle-letter-p" => Some(Self::CircleLetterP),
            "circle-letter-q" => Some(Self::CircleLetterQ),
            "circle-letter-r" => Some(Self::CircleLetterR),
            "circle-letter-s" => Some(Self::CircleLetterS),
            "circle-letter-t" => Some(Self::CircleLetterT),
            "circle-letter-u" => Some(Self::CircleLetterU),
            "circle-letter-v" => Some(Self::CircleLetterV),
            "circle-letter-w" => Some(Self::CircleLetterW),
            "circle-letter-x" => Some(Self::CircleLetterX),
            "circle-letter-y" => Some(Self::CircleLetterY),
            "circle-letter-z" => Some(Self::CircleLetterZ),
            "circle-number-0" => Some(Self::CircleNumber0),
            "circle-number-1" => Some(Self::CircleNumber1),
            "circle-number-2" => Some(Self::CircleNumber2),
            "circle-number-3" => Some(Self::CircleNumber3),
            "circle-number-4" => Some(Self::CircleNumber4),
            "circle-number-5" => Some(Self::CircleNumber5),
            "circle-number-6" => Some(Self::CircleNumber6),
            "circle-number-7" => Some(Self::CircleNumber7),
            "circle-number-8" => Some(Self::CircleNumber8),
            "circle-number-9" => Some(Self::CircleNumber9),
            "clear-formatting" => Some(Self::ClearFormatting),
            "clipboard" => Some(Self::Clipboard),
            "clipboard-check" => Some(Self::ClipboardCheck),
            "clipboard-copy" => Some(Self::ClipboardCopy),
            "clipboard-data" => Some(Self::ClipboardData),
            "clipboard-heart" => Some(Self::ClipboardHeart),
            "clipboard-list" => Some(Self::ClipboardList),
            "clipboard-off" => Some(Self::ClipboardOff),
            "clipboard-plus" => Some(Self::ClipboardPlus),
            "clipboard-search" => Some(Self::ClipboardSearch),
            "clipboard-smile" => Some(Self::ClipboardSmile),
            "clipboard-text" => Some(Self::ClipboardText),
            "clipboard-typography" => Some(Self::ClipboardTypography),
            "clipboard-x" => Some(Self::ClipboardX),
            "code" => Some(Self::Code),
            "code-asterisk" => Some(Self::CodeAsterisk),
            "code-circle" => Some(Self::CodeCircle),
            "code-circle-2" => Some(Self::CodeCircle2),
            "code-dots" => Some(Self::CodeDots),
            "code-minus" => Some(Self::CodeMinus),
            "code-off" => Some(Self::CodeOff),
            "code-plus" => Some(Self::CodePlus),
            "columns" => Some(Self::Columns),
            "columns-1" => Some(Self::Columns1),
            "columns-2" => Some(Self::Columns2),
            "columns-3" => Some(Self::Columns3),
            "columns-off" => Some(Self::ColumnsOff),
            "contract" => Some(Self::Contract),
            "copy" => Some(Self::Copy),
            "copy-check" => Some(Self::CopyCheck),
            "copy-minus" => Some(Self::CopyMinus),
            "copy-off" => Some(Self::CopyOff),
            "copy-plus" => Some(Self::CopyPlus),
            "copy-x" => Some(Self::CopyX),
            "cursor-text" => Some(Self::CursorText),
            "delta" => Some(Self::Delta),
            "emphasis" => Some(Self::Emphasis),
            "eraser" => Some(Self::Eraser),
            "eraser-off" => Some(Self::EraserOff),
            "file" => Some(Self::File),
            "file-3d" => Some(Self::File3d),
            "file-ai" => Some(Self::FileAi),
            "file-alert" => Some(Self::FileAlert),
            "file-analytics" => Some(Self::FileAnalytics),
            "file-arrow-left" => Some(Self::FileArrowLeft),
            "file-arrow-right" => Some(Self::FileArrowRight),
            "file-barcode" => Some(Self::FileBarcode),
            "file-bitcoin" => Some(Self::FileBitcoin),
            "file-broken" => Some(Self::FileBroken),
            "file-certificate" => Some(Self::FileCertificate),
            "file-chart" => Some(Self::FileChart),
            "file-check" => Some(Self::FileCheck),
            "file-code" => Some(Self::FileCode),
            "file-code-2" => Some(Self::FileCode2),
            "file-cv" => Some(Self::FileCv),
            "file-database" => Some(Self::FileDatabase),
            "file-delta" => Some(Self::FileDelta),
            "file-description" => Some(Self::FileDescription),
            "file-diff" => Some(Self::FileDiff),
            "file-digit" => Some(Self::FileDigit),
            "file-dislike" => Some(Self::FileDislike),
            "file-dollar" => Some(Self::FileDollar),
            "file-dots" => Some(Self::FileDots),
            "file-download" => Some(Self::FileDownload),
            "file-euro" => Some(Self::FileEuro),
            "file-excel" => Some(Self::FileExcel),
            "file-export" => Some(Self::FileExport),
            "file-function" => Some(Self::FileFunction),
            "file-horizontal" => Some(Self::FileHorizontal),
            "file-import" => Some(Self::FileImport),
            "file-infinity" => Some(Self::FileInfinity),
            "file-info" => Some(Self::FileInfo),
            "file-invoice" => Some(Self::FileInvoice),
            "file-isr" => Some(Self::FileIsr),
            "file-lambda" => Some(Self::FileLambda),
            "file-like" => Some(Self::FileLike),
            "file-minus" => Some(Self::FileMinus),
            "file-music" => Some(Self::FileMusic),
            "file-neutral" => Some(Self::FileNeutral),
            "file-off" => Some(Self::FileOff),
            "file-orientation" => Some(Self::FileOrientation),
            "file-pencil" => Some(Self::FilePencil),
            "file-percent" => Some(Self::FilePercent),
            "file-phone" => Some(Self::FilePhone),
            "file-plus" => Some(Self::FilePlus),
            "file-power" => Some(Self::FilePower),
            "file-report" => Some(Self::FileReport),
            "file-rss" => Some(Self::FileRss),
            "file-sad" => Some(Self::FileSad),
            "file-scissors" => Some(Self::FileScissors),
            "file-search" => Some(Self::FileSearch),
            "file-settings" => Some(Self::FileSettings),
            "file-shredder" => Some(Self::FileShredder),
            "file-signal" => Some(Self::FileSignal),
            "file-smile" => Some(Self::FileSmile),
            "file-spark" => Some(Self::FileSpark),
            "file-spreadsheet" => Some(Self::FileSpreadsheet),
            "file-stack" => Some(Self::FileStack),
            "file-star" => Some(Self::FileStar),
            "file-symlink" => Some(Self::FileSymlink),
            "file-text" => Some(Self::FileText),
            "file-text-ai" => Some(Self::FileTextAi),
            "file-text-shield" => Some(Self::FileTextShield),
            "file-text-spark" => Some(Self::FileTextSpark),
            "file-time" => Some(Self::FileTime),
            "file-type-bmp" => Some(Self::FileTypeBmp),
            "file-type-css" => Some(Self::FileTypeCss),
            "file-type-csv" => Some(Self::FileTypeCsv),
            "file-type-doc" => Some(Self::FileTypeDoc),
            "file-type-docx" => Some(Self::FileTypeDocx),
            "file-type-html" => Some(Self::FileTypeHtml),
            "file-type-jpg" => Some(Self::FileTypeJpg),
            "file-type-js" => Some(Self::FileTypeJs),
            "file-type-jsx" => Some(Self::FileTypeJsx),
            "file-type-pdf" => Some(Self::FileTypePdf),
            "file-type-php" => Some(Self::FileTypePhp),
            "file-type-png" => Some(Self::FileTypePng),
            "file-type-ppt" => Some(Self::FileTypePpt),
            "file-type-rs" => Some(Self::FileTypeRs),
            "file-type-sql" => Some(Self::FileTypeSql),
            "file-type-svg" => Some(Self::FileTypeSvg),
            "file-type-ts" => Some(Self::FileTypeTs),
            "file-type-tsx" => Some(Self::FileTypeTsx),
            "file-type-txt" => Some(Self::FileTypeTxt),
            "file-type-vue" => Some(Self::FileTypeVue),
            "file-type-xls" => Some(Self::FileTypeXls),
            "file-type-xml" => Some(Self::FileTypeXml),
            "file-type-zip" => Some(Self::FileTypeZip),
            "file-typography" => Some(Self::FileTypography),
            "file-unknown" => Some(Self::FileUnknown),
            "file-upload" => Some(Self::FileUpload),
            "file-vector" => Some(Self::FileVector),
            "file-word" => Some(Self::FileWord),
            "file-x" => Some(Self::FileX),
            "file-zip" => Some(Self::FileZip),
            "files" => Some(Self::Files),
            "files-off" => Some(Self::FilesOff),
            "float-center" => Some(Self::FloatCenter),
            "float-left" => Some(Self::FloatLeft),
            "float-none" => Some(Self::FloatNone),
            "float-right" => Some(Self::FloatRight),
            "folder" => Some(Self::Folder),
            "folder-bolt" => Some(Self::FolderBolt),
            "folder-cancel" => Some(Self::FolderCancel),
            "folder-check" => Some(Self::FolderCheck),
            "folder-code" => Some(Self::FolderCode),
            "folder-cog" => Some(Self::FolderCog),
            "folder-dollar" => Some(Self::FolderDollar),
            "folder-down" => Some(Self::FolderDown),
            "folder-exclamation" => Some(Self::FolderExclamation),
            "folder-heart" => Some(Self::FolderHeart),
            "folder-minus" => Some(Self::FolderMinus),
            "folder-off" => Some(Self::FolderOff),
            "folder-open" => Some(Self::FolderOpen),
            "folder-pause" => Some(Self::FolderPause),
            "folder-pin" => Some(Self::FolderPin),
            "folder-plus" => Some(Self::FolderPlus),
            "folder-question" => Some(Self::FolderQuestion),
            "folder-root" => Some(Self::FolderRoot),
            "folder-search" => Some(Self::FolderSearch),
            "folder-share" => Some(Self::FolderShare),
            "folder-star" => Some(Self::FolderStar),
            "folder-symlink" => Some(Self::FolderSymlink),
            "folder-up" => Some(Self::FolderUp),
            "folder-x" => Some(Self::FolderX),
            "folders" => Some(Self::Folders),
            "folders-off" => Some(Self::FoldersOff),
            "forms" => Some(Self::Forms),
            "h-1" => Some(Self::H1),
            "h-2" => Some(Self::H2),
            "h-3" => Some(Self::H3),
            "h-4" => Some(Self::H4),
            "h-5" => Some(Self::H5),
            "h-6" => Some(Self::H6),
            "heading" => Some(Self::Heading),
            "heading-off" => Some(Self::HeadingOff),
            "hexagon-letter-a" => Some(Self::HexagonLetterA),
            "hexagon-letter-b" => Some(Self::HexagonLetterB),
            "hexagon-letter-c" => Some(Self::HexagonLetterC),
            "hexagon-letter-d" => Some(Self::HexagonLetterD),
            "hexagon-letter-e" => Some(Self::HexagonLetterE),
            "hexagon-letter-f" => Some(Self::HexagonLetterF),
            "hexagon-letter-g" => Some(Self::HexagonLetterG),
            "hexagon-letter-h" => Some(Self::HexagonLetterH),
            "hexagon-letter-i" => Some(Self::HexagonLetterI),
            "hexagon-letter-j" => Some(Self::HexagonLetterJ),
            "hexagon-letter-k" => Some(Self::HexagonLetterK),
            "hexagon-letter-l" => Some(Self::HexagonLetterL),
            "hexagon-letter-m" => Some(Self::HexagonLetterM),
            "hexagon-letter-n" => Some(Self::HexagonLetterN),
            "hexagon-letter-o" => Some(Self::HexagonLetterO),
            "hexagon-letter-p" => Some(Self::HexagonLetterP),
            "hexagon-letter-q" => Some(Self::HexagonLetterQ),
            "hexagon-letter-r" => Some(Self::HexagonLetterR),
            "hexagon-letter-s" => Some(Self::HexagonLetterS),
            "hexagon-letter-t" => Some(Self::HexagonLetterT),
            "hexagon-letter-u" => Some(Self::HexagonLetterU),
            "hexagon-letter-v" => Some(Self::HexagonLetterV),
            "hexagon-letter-w" => Some(Self::HexagonLetterW),
            "hexagon-letter-x" => Some(Self::HexagonLetterX),
            "hexagon-letter-y" => Some(Self::HexagonLetterY),
            "hexagon-letter-z" => Some(Self::HexagonLetterZ),
            "hexagon-number-0" => Some(Self::HexagonNumber0),
            "hexagon-number-1" => Some(Self::HexagonNumber1),
            "hexagon-number-2" => Some(Self::HexagonNumber2),
            "hexagon-number-3" => Some(Self::HexagonNumber3),
            "hexagon-number-4" => Some(Self::HexagonNumber4),
            "hexagon-number-5" => Some(Self::HexagonNumber5),
            "hexagon-number-6" => Some(Self::HexagonNumber6),
            "hexagon-number-7" => Some(Self::HexagonNumber7),
            "hexagon-number-8" => Some(Self::HexagonNumber8),
            "hexagon-number-9" => Some(Self::HexagonNumber9),
            "highlight" => Some(Self::Highlight),
            "highlight-off" => Some(Self::HighlightOff),
            "indent-decrease" => Some(Self::IndentDecrease),
            "indent-increase" => Some(Self::IndentIncrease),
            "input-search" => Some(Self::InputSearch),
            "invoice" => Some(Self::Invoice),
            "italic" => Some(Self::Italic),
            "kerning" => Some(Self::Kerning),
            "lambda" => Some(Self::Lambda),
            "language" => Some(Self::Language),
            "language-hiragana" => Some(Self::LanguageHiragana),
            "language-katakana" => Some(Self::LanguageKatakana),
            "language-off" => Some(Self::LanguageOff),
            "letter-a" => Some(Self::LetterA),
            "letter-a-small" => Some(Self::LetterASmall),
            "letter-b" => Some(Self::LetterB),
            "letter-b-small" => Some(Self::LetterBSmall),
            "letter-c" => Some(Self::LetterC),
            "letter-c-small" => Some(Self::LetterCSmall),
            "letter-case" => Some(Self::LetterCase),
            "letter-case-lower" => Some(Self::LetterCaseLower),
            "letter-case-toggle" => Some(Self::LetterCaseToggle),
            "letter-case-upper" => Some(Self::LetterCaseUpper),
            "letter-d" => Some(Self::LetterD),
            "letter-d-small" => Some(Self::LetterDSmall),
            "letter-e" => Some(Self::LetterE),
            "letter-e-small" => Some(Self::LetterESmall),
            "letter-f" => Some(Self::LetterF),
            "letter-f-small" => Some(Self::LetterFSmall),
            "letter-g" => Some(Self::LetterG),
            "letter-g-small" => Some(Self::LetterGSmall),
            "letter-h" => Some(Self::LetterH),
            "letter-h-small" => Some(Self::LetterHSmall),
            "letter-i" => Some(Self::LetterI),
            "letter-i-small" => Some(Self::LetterISmall),
            "letter-j" => Some(Self::LetterJ),
            "letter-j-small" => Some(Self::LetterJSmall),
            "letter-k" => Some(Self::LetterK),
            "letter-k-small" => Some(Self::LetterKSmall),
            "letter-l" => Some(Self::LetterL),
            "letter-l-small" => Some(Self::LetterLSmall),
            "letter-m" => Some(Self::LetterM),
            "letter-m-small" => Some(Self::LetterMSmall),
            "letter-n" => Some(Self::LetterN),
            "letter-n-small" => Some(Self::LetterNSmall),
            "letter-o" => Some(Self::LetterO),
            "letter-o-small" => Some(Self::LetterOSmall),
            "letter-p" => Some(Self::LetterP),
            "letter-p-small" => Some(Self::LetterPSmall),
            "letter-q" => Some(Self::LetterQ),
            "letter-q-small" => Some(Self::LetterQSmall),
            "letter-r" => Some(Self::LetterR),
            "letter-r-small" => Some(Self::LetterRSmall),
            "letter-s" => Some(Self::LetterS),
            "letter-s-small" => Some(Self::LetterSSmall),
            "letter-spacing" => Some(Self::LetterSpacing),
            "letter-t" => Some(Self::LetterT),
            "letter-t-small" => Some(Self::LetterTSmall),
            "letter-u" => Some(Self::LetterU),
            "letter-u-small" => Some(Self::LetterUSmall),
            "letter-v" => Some(Self::LetterV),
            "letter-v-small" => Some(Self::LetterVSmall),
            "letter-w" => Some(Self::LetterW),
            "letter-w-small" => Some(Self::LetterWSmall),
            "letter-x" => Some(Self::LetterX),
            "letter-x-small" => Some(Self::LetterXSmall),
            "letter-y" => Some(Self::LetterY),
            "letter-y-small" => Some(Self::LetterYSmall),
            "letter-z" => Some(Self::LetterZ),
            "letter-z-small" => Some(Self::LetterZSmall),
            "license" => Some(Self::License),
            "license-off" => Some(Self::LicenseOff),
            "line-height" => Some(Self::LineHeight),
            "link" => Some(Self::Link),
            "link-minus" => Some(Self::LinkMinus),
            "link-off" => Some(Self::LinkOff),
            "link-plus" => Some(Self::LinkPlus),
            "list" => Some(Self::List),
            "list-check" => Some(Self::ListCheck),
            "list-details" => Some(Self::ListDetails),
            "list-numbers" => Some(Self::ListNumbers),
            "list-search" => Some(Self::ListSearch),
            "manual-gearbox" => Some(Self::ManualGearbox),
            "markdown" => Some(Self::Markdown),
            "markdown-off" => Some(Self::MarkdownOff),
            "news" => Some(Self::News),
            "news-off" => Some(Self::NewsOff),
            "notdef" => Some(Self::Notdef),
            "note" => Some(Self::Note),
            "note-off" => Some(Self::NoteOff),
            "notebook" => Some(Self::Notebook),
            "notebook-off" => Some(Self::NotebookOff),
            "notes" => Some(Self::Notes),
            "notes-off" => Some(Self::NotesOff),
            "number" => Some(Self::Number),
            "number-0" => Some(Self::Number0),
            "number-0-small" => Some(Self::Number0Small),
            "number-1" => Some(Self::Number1),
            "number-1-small" => Some(Self::Number1Small),
            "number-10" => Some(Self::Number10),
            "number-10-small" => Some(Self::Number10Small),
            "number-100-small" => Some(Self::Number100Small),
            "number-11" => Some(Self::Number11),
            "number-11-small" => Some(Self::Number11Small),
            "number-12-small" => Some(Self::Number12Small),
            "number-123" => Some(Self::Number123),
            "number-13-small" => Some(Self::Number13Small),
            "number-14-small" => Some(Self::Number14Small),
            "number-15-small" => Some(Self::Number15Small),
            "number-16-small" => Some(Self::Number16Small),
            "number-17-small" => Some(Self::Number17Small),
            "number-18-small" => Some(Self::Number18Small),
            "number-19-small" => Some(Self::Number19Small),
            "number-2" => Some(Self::Number2),
            "number-2-small" => Some(Self::Number2Small),
            "number-20-small" => Some(Self::Number20Small),
            "number-21-small" => Some(Self::Number21Small),
            "number-22-small" => Some(Self::Number22Small),
            "number-23-small" => Some(Self::Number23Small),
            "number-24-small" => Some(Self::Number24Small),
            "number-25-small" => Some(Self::Number25Small),
            "number-26-small" => Some(Self::Number26Small),
            "number-27-small" => Some(Self::Number27Small),
            "number-28-small" => Some(Self::Number28Small),
            "number-29-small" => Some(Self::Number29Small),
            "number-3" => Some(Self::Number3),
            "number-3-small" => Some(Self::Number3Small),
            "number-30-small" => Some(Self::Number30Small),
            "number-31-small" => Some(Self::Number31Small),
            "number-32-small" => Some(Self::Number32Small),
            "number-33-small" => Some(Self::Number33Small),
            "number-34-small" => Some(Self::Number34Small),
            "number-35-small" => Some(Self::Number35Small),
            "number-36-small" => Some(Self::Number36Small),
            "number-37-small" => Some(Self::Number37Small),
            "number-38-small" => Some(Self::Number38Small),
            "number-39-small" => Some(Self::Number39Small),
            "number-4" => Some(Self::Number4),
            "number-4-small" => Some(Self::Number4Small),
            "number-40-small" => Some(Self::Number40Small),
            "number-41-small" => Some(Self::Number41Small),
            "number-42-small" => Some(Self::Number42Small),
            "number-43-small" => Some(Self::Number43Small),
            "number-44-small" => Some(Self::Number44Small),
            "number-45-small" => Some(Self::Number45Small),
            "number-46-small" => Some(Self::Number46Small),
            "number-47-small" => Some(Self::Number47Small),
            "number-48-small" => Some(Self::Number48Small),
            "number-49-small" => Some(Self::Number49Small),
            "number-5" => Some(Self::Number5),
            "number-5-small" => Some(Self::Number5Small),
            "number-50-small" => Some(Self::Number50Small),
            "number-51-small" => Some(Self::Number51Small),
            "number-52-small" => Some(Self::Number52Small),
            "number-53-small" => Some(Self::Number53Small),
            "number-54-small" => Some(Self::Number54Small),
            "number-55-small" => Some(Self::Number55Small),
            "number-56-small" => Some(Self::Number56Small),
            "number-57-small" => Some(Self::Number57Small),
            "number-58-small" => Some(Self::Number58Small),
            "number-59-small" => Some(Self::Number59Small),
            "number-6" => Some(Self::Number6),
            "number-6-small" => Some(Self::Number6Small),
            "number-60-small" => Some(Self::Number60Small),
            "number-61-small" => Some(Self::Number61Small),
            "number-62-small" => Some(Self::Number62Small),
            "number-63-small" => Some(Self::Number63Small),
            "number-64-small" => Some(Self::Number64Small),
            "number-65-small" => Some(Self::Number65Small),
            "number-66-small" => Some(Self::Number66Small),
            "number-67-small" => Some(Self::Number67Small),
            "number-68-small" => Some(Self::Number68Small),
            "number-69-small" => Some(Self::Number69Small),
            "number-7" => Some(Self::Number7),
            "number-7-small" => Some(Self::Number7Small),
            "number-70-small" => Some(Self::Number70Small),
            "number-71-small" => Some(Self::Number71Small),
            "number-72-small" => Some(Self::Number72Small),
            "number-73-small" => Some(Self::Number73Small),
            "number-74-small" => Some(Self::Number74Small),
            "number-75-small" => Some(Self::Number75Small),
            "number-76-small" => Some(Self::Number76Small),
            "number-77-small" => Some(Self::Number77Small),
            "number-78-small" => Some(Self::Number78Small),
            "number-79-small" => Some(Self::Number79Small),
            "number-8" => Some(Self::Number8),
            "number-8-small" => Some(Self::Number8Small),
            "number-80-small" => Some(Self::Number80Small),
            "number-81-small" => Some(Self::Number81Small),
            "number-82-small" => Some(Self::Number82Small),
            "number-83-small" => Some(Self::Number83Small),
            "number-84-small" => Some(Self::Number84Small),
            "number-85-small" => Some(Self::Number85Small),
            "number-86-small" => Some(Self::Number86Small),
            "number-87-small" => Some(Self::Number87Small),
            "number-88-small" => Some(Self::Number88Small),
            "number-89-small" => Some(Self::Number89Small),
            "number-9" => Some(Self::Number9),
            "number-9-small" => Some(Self::Number9Small),
            "number-90-small" => Some(Self::Number90Small),
            "number-91-small" => Some(Self::Number91Small),
            "number-92-small" => Some(Self::Number92Small),
            "number-93-small" => Some(Self::Number93Small),
            "number-94-small" => Some(Self::Number94Small),
            "number-95-small" => Some(Self::Number95Small),
            "number-96-small" => Some(Self::Number96Small),
            "number-97-small" => Some(Self::Number97Small),
            "number-98-small" => Some(Self::Number98Small),
            "number-99-small" => Some(Self::Number99Small),
            "numbers" => Some(Self::Numbers),
            "overline" => Some(Self::Overline),
            "page-break" => Some(Self::PageBreak),
            "paperclip" => Some(Self::Paperclip),
            "pentagon-number-0" => Some(Self::PentagonNumber0),
            "pentagon-number-1" => Some(Self::PentagonNumber1),
            "pentagon-number-2" => Some(Self::PentagonNumber2),
            "pentagon-number-3" => Some(Self::PentagonNumber3),
            "pentagon-number-4" => Some(Self::PentagonNumber4),
            "pentagon-number-5" => Some(Self::PentagonNumber5),
            "pentagon-number-6" => Some(Self::PentagonNumber6),
            "pentagon-number-7" => Some(Self::PentagonNumber7),
            "pentagon-number-8" => Some(Self::PentagonNumber8),
            "pentagon-number-9" => Some(Self::PentagonNumber9),
            "pilcrow" => Some(Self::Pilcrow),
            "pilcrow-left" => Some(Self::PilcrowLeft),
            "pilcrow-right" => Some(Self::PilcrowRight),
            "presentation" => Some(Self::Presentation),
            "presentation-analytics" => Some(Self::PresentationAnalytics),
            "presentation-off" => Some(Self::PresentationOff),
            "quote" => Some(Self::Quote),
            "quote-off" => Some(Self::QuoteOff),
            "quote-open" => Some(Self::QuoteOpen),
            "quotes" => Some(Self::Quotes),
            "receipt" => Some(Self::Receipt),
            "receipt-2" => Some(Self::Receipt2),
            "receipt-dollar" => Some(Self::ReceiptDollar),
            "receipt-off" => Some(Self::ReceiptOff),
            "receipt-refund" => Some(Self::ReceiptRefund),
            "receipt-tax" => Some(Self::ReceiptTax),
            "regex" => Some(Self::Regex),
            "regex-off" => Some(Self::RegexOff),
            "report" => Some(Self::Report),
            "report-analytics" => Some(Self::ReportAnalytics),
            "report-medical" => Some(Self::ReportMedical),
            "report-money" => Some(Self::ReportMoney),
            "report-off" => Some(Self::ReportOff),
            "report-search" => Some(Self::ReportSearch),
            "rosette-number-0" => Some(Self::RosetteNumber0),
            "rosette-number-1" => Some(Self::RosetteNumber1),
            "rosette-number-2" => Some(Self::RosetteNumber2),
            "rosette-number-3" => Some(Self::RosetteNumber3),
            "rosette-number-4" => Some(Self::RosetteNumber4),
            "rosette-number-5" => Some(Self::RosetteNumber5),
            "rosette-number-6" => Some(Self::RosetteNumber6),
            "rosette-number-7" => Some(Self::RosetteNumber7),
            "rosette-number-8" => Some(Self::RosetteNumber8),
            "rosette-number-9" => Some(Self::RosetteNumber9),
            "rubber-stamp" => Some(Self::RubberStamp),
            "rubber-stamp-off" => Some(Self::RubberStampOff),
            "scan-letter-a" => Some(Self::ScanLetterA),
            "scan-letter-t" => Some(Self::ScanLetterT),
            "scribble" => Some(Self::Scribble),
            "scribble-off" => Some(Self::ScribbleOff),
            "script" => Some(Self::Script),
            "script-minus" => Some(Self::ScriptMinus),
            "script-plus" => Some(Self::ScriptPlus),
            "script-x" => Some(Self::ScriptX),
            "section-sign" => Some(Self::SectionSign),
            "separator" => Some(Self::Separator),
            "separator-horizontal" => Some(Self::SeparatorHorizontal),
            "separator-vertical" => Some(Self::SeparatorVertical),
            "signature" => Some(Self::Signature),
            "signature-off" => Some(Self::SignatureOff),
            "slideshow" => Some(Self::Slideshow),
            "sort-0-9" => Some(Self::Sort09),
            "sort-9-0" => Some(Self::Sort90),
            "sort-a-z" => Some(Self::SortAZ),
            "sort-ascending" => Some(Self::SortAscending),
            "sort-ascending-2" => Some(Self::SortAscending2),
            "sort-ascending-letters" => Some(Self::SortAscendingLetters),
            "sort-ascending-numbers" => Some(Self::SortAscendingNumbers),
            "sort-descending" => Some(Self::SortDescending),
            "sort-descending-2" => Some(Self::SortDescending2),
            "sort-descending-letters" => Some(Self::SortDescendingLetters),
            "sort-descending-numbers" => Some(Self::SortDescendingNumbers),
            "sort-z-a" => Some(Self::SortZA),
            "space" => Some(Self::Space),
            "space-off" => Some(Self::SpaceOff),
            "spacing-horizontal" => Some(Self::SpacingHorizontal),
            "spacing-vertical" => Some(Self::SpacingVertical),
            "square-letter-a" => Some(Self::SquareLetterA),
            "square-letter-b" => Some(Self::SquareLetterB),
            "square-letter-c" => Some(Self::SquareLetterC),
            "square-letter-d" => Some(Self::SquareLetterD),
            "square-letter-e" => Some(Self::SquareLetterE),
            "square-letter-f" => Some(Self::SquareLetterF),
            "square-letter-g" => Some(Self::SquareLetterG),
            "square-letter-h" => Some(Self::SquareLetterH),
            "square-letter-i" => Some(Self::SquareLetterI),
            "square-letter-j" => Some(Self::SquareLetterJ),
            "square-letter-k" => Some(Self::SquareLetterK),
            "square-letter-l" => Some(Self::SquareLetterL),
            "square-letter-m" => Some(Self::SquareLetterM),
            "square-letter-n" => Some(Self::SquareLetterN),
            "square-letter-o" => Some(Self::SquareLetterO),
            "square-letter-p" => Some(Self::SquareLetterP),
            "square-letter-q" => Some(Self::SquareLetterQ),
            "square-letter-r" => Some(Self::SquareLetterR),
            "square-letter-s" => Some(Self::SquareLetterS),
            "square-letter-t" => Some(Self::SquareLetterT),
            "square-letter-u" => Some(Self::SquareLetterU),
            "square-letter-v" => Some(Self::SquareLetterV),
            "square-letter-w" => Some(Self::SquareLetterW),
            "square-letter-x" => Some(Self::SquareLetterX),
            "square-letter-y" => Some(Self::SquareLetterY),
            "square-letter-z" => Some(Self::SquareLetterZ),
            "square-number-0" => Some(Self::SquareNumber0),
            "square-number-1" => Some(Self::SquareNumber1),
            "square-number-2" => Some(Self::SquareNumber2),
            "square-number-3" => Some(Self::SquareNumber3),
            "square-number-4" => Some(Self::SquareNumber4),
            "square-number-5" => Some(Self::SquareNumber5),
            "square-number-6" => Some(Self::SquareNumber6),
            "square-number-7" => Some(Self::SquareNumber7),
            "square-number-8" => Some(Self::SquareNumber8),
            "square-number-9" => Some(Self::SquareNumber9),
            "square-rounded-letter-a" => Some(Self::SquareRoundedLetterA),
            "square-rounded-letter-b" => Some(Self::SquareRoundedLetterB),
            "square-rounded-letter-c" => Some(Self::SquareRoundedLetterC),
            "square-rounded-letter-d" => Some(Self::SquareRoundedLetterD),
            "square-rounded-letter-e" => Some(Self::SquareRoundedLetterE),
            "square-rounded-letter-f" => Some(Self::SquareRoundedLetterF),
            "square-rounded-letter-g" => Some(Self::SquareRoundedLetterG),
            "square-rounded-letter-h" => Some(Self::SquareRoundedLetterH),
            "square-rounded-letter-i" => Some(Self::SquareRoundedLetterI),
            "square-rounded-letter-j" => Some(Self::SquareRoundedLetterJ),
            "square-rounded-letter-k" => Some(Self::SquareRoundedLetterK),
            "square-rounded-letter-l" => Some(Self::SquareRoundedLetterL),
            "square-rounded-letter-m" => Some(Self::SquareRoundedLetterM),
            "square-rounded-letter-n" => Some(Self::SquareRoundedLetterN),
            "square-rounded-letter-o" => Some(Self::SquareRoundedLetterO),
            "square-rounded-letter-p" => Some(Self::SquareRoundedLetterP),
            "square-rounded-letter-q" => Some(Self::SquareRoundedLetterQ),
            "square-rounded-letter-r" => Some(Self::SquareRoundedLetterR),
            "square-rounded-letter-s" => Some(Self::SquareRoundedLetterS),
            "square-rounded-letter-t" => Some(Self::SquareRoundedLetterT),
            "square-rounded-letter-u" => Some(Self::SquareRoundedLetterU),
            "square-rounded-letter-v" => Some(Self::SquareRoundedLetterV),
            "square-rounded-letter-w" => Some(Self::SquareRoundedLetterW),
            "square-rounded-letter-x" => Some(Self::SquareRoundedLetterX),
            "square-rounded-letter-y" => Some(Self::SquareRoundedLetterY),
            "square-rounded-letter-z" => Some(Self::SquareRoundedLetterZ),
            "square-rounded-number-0" => Some(Self::SquareRoundedNumber0),
            "square-rounded-number-1" => Some(Self::SquareRoundedNumber1),
            "square-rounded-number-2" => Some(Self::SquareRoundedNumber2),
            "square-rounded-number-3" => Some(Self::SquareRoundedNumber3),
            "square-rounded-number-4" => Some(Self::SquareRoundedNumber4),
            "square-rounded-number-5" => Some(Self::SquareRoundedNumber5),
            "square-rounded-number-6" => Some(Self::SquareRoundedNumber6),
            "square-rounded-number-7" => Some(Self::SquareRoundedNumber7),
            "square-rounded-number-8" => Some(Self::SquareRoundedNumber8),
            "square-rounded-number-9" => Some(Self::SquareRoundedNumber9),
            "strikethrough" => Some(Self::Strikethrough),
            "subscript" => Some(Self::Subscript),
            "superscript" => Some(Self::Superscript),
            "tex" => Some(Self::Tex),
            "text-caption" => Some(Self::TextCaption),
            "text-color" => Some(Self::TextColor),
            "text-decrease" => Some(Self::TextDecrease),
            "text-direction-ltr" => Some(Self::TextDirectionLtr),
            "text-direction-rtl" => Some(Self::TextDirectionRtl),
            "text-grammar" => Some(Self::TextGrammar),
            "text-increase" => Some(Self::TextIncrease),
            "text-orientation" => Some(Self::TextOrientation),
            "text-plus" => Some(Self::TextPlus),
            "text-recognition" => Some(Self::TextRecognition),
            "text-size" => Some(Self::TextSize),
            "text-spellcheck" => Some(Self::TextSpellcheck),
            "text-wrap" => Some(Self::TextWrap),
            "text-wrap-column" => Some(Self::TextWrapColumn),
            "text-wrap-disabled" => Some(Self::TextWrapDisabled),
            "ticket" => Some(Self::Ticket),
            "ticket-off" => Some(Self::TicketOff),
            "typography" => Some(Self::Typography),
            "typography-off" => Some(Self::TypographyOff),
            "underline" => Some(Self::Underline),
            "unlink" => Some(Self::Unlink),
            "vocabulary" => Some(Self::Vocabulary),
            "vocabulary-off" => Some(Self::VocabularyOff),
            "writing" => Some(Self::Writing),
            "writing-off" => Some(Self::WritingOff),
            "writing-sign" => Some(Self::WritingSign),
            "writing-sign-off" => Some(Self::WritingSignOff),
            _ => None,
        }
    }
}

impl TablerIconData for DocumentIcon {
    fn name(&self) -> &'static str {
        match self {
            Self::AB2 => "a-b-2",
            Self::ABOff => "a-b-off",
            Self::Abc => "abc",
            Self::AlignBoxBottomCenter => "align-box-bottom-center",
            Self::AlignBoxBottomLeft => "align-box-bottom-left",
            Self::AlignBoxBottomRight => "align-box-bottom-right",
            Self::AlignBoxCenterBottom => "align-box-center-bottom",
            Self::AlignBoxCenterMiddle => "align-box-center-middle",
            Self::AlignBoxCenterStretch => "align-box-center-stretch",
            Self::AlignBoxCenterTop => "align-box-center-top",
            Self::AlignBoxLeftBottom => "align-box-left-bottom",
            Self::AlignBoxLeftMiddle => "align-box-left-middle",
            Self::AlignBoxLeftStretch => "align-box-left-stretch",
            Self::AlignBoxLeftTop => "align-box-left-top",
            Self::AlignBoxRightBottom => "align-box-right-bottom",
            Self::AlignBoxRightMiddle => "align-box-right-middle",
            Self::AlignBoxRightStretch => "align-box-right-stretch",
            Self::AlignBoxRightTop => "align-box-right-top",
            Self::AlignBoxTopCenter => "align-box-top-center",
            Self::AlignBoxTopLeft => "align-box-top-left",
            Self::AlignBoxTopRight => "align-box-top-right",
            Self::AlignCenter => "align-center",
            Self::AlignJustified => "align-justified",
            Self::AlignLeft => "align-left",
            Self::AlignLeft2 => "align-left-2",
            Self::AlignRight => "align-right",
            Self::AlignRight2 => "align-right-2",
            Self::Alpha => "alpha",
            Self::AlphabetArabic => "alphabet-arabic",
            Self::AlphabetBangla => "alphabet-bangla",
            Self::AlphabetCyrillic => "alphabet-cyrillic",
            Self::AlphabetGreek => "alphabet-greek",
            Self::AlphabetHebrew => "alphabet-hebrew",
            Self::AlphabetKorean => "alphabet-korean",
            Self::AlphabetLatin => "alphabet-latin",
            Self::AlphabetPolish => "alphabet-polish",
            Self::AlphabetRunes => "alphabet-runes",
            Self::AlphabetThai => "alphabet-thai",
            Self::Archive => "archive",
            Self::ArchiveOff => "archive-off",
            Self::Article => "article",
            Self::ArticleOff => "article-off",
            Self::Asterisk => "asterisk",
            Self::AsteriskSimple => "asterisk-simple",
            Self::At => "at",
            Self::AtOff => "at-off",
            Self::Backspace => "backspace",
            Self::Ballpen => "ballpen",
            Self::BallpenOff => "ballpen-off",
            Self::Baseline => "baseline",
            Self::BaselineDensityLarge => "baseline-density-large",
            Self::BaselineDensityMedium => "baseline-density-medium",
            Self::BaselineDensitySmall => "baseline-density-small",
            Self::Beta => "beta",
            Self::Bible => "bible",
            Self::Blockquote => "blockquote",
            Self::Bold => "bold",
            Self::BoldOff => "bold-off",
            Self::Book => "book",
            Self::Book2 => "book-2",
            Self::BookDownload => "book-download",
            Self::BookOff => "book-off",
            Self::BookUpload => "book-upload",
            Self::Bookmark => "bookmark",
            Self::BookmarkAi => "bookmark-ai",
            Self::BookmarkEdit => "bookmark-edit",
            Self::BookmarkMinus => "bookmark-minus",
            Self::BookmarkOff => "bookmark-off",
            Self::BookmarkPlus => "bookmark-plus",
            Self::BookmarkQuestion => "bookmark-question",
            Self::Bookmarks => "bookmarks",
            Self::BookmarksOff => "bookmarks-off",
            Self::Books => "books",
            Self::BooksOff => "books-off",
            Self::BoxMultiple0 => "box-multiple-0",
            Self::BoxMultiple1 => "box-multiple-1",
            Self::BoxMultiple2 => "box-multiple-2",
            Self::BoxMultiple3 => "box-multiple-3",
            Self::BoxMultiple4 => "box-multiple-4",
            Self::BoxMultiple5 => "box-multiple-5",
            Self::BoxMultiple6 => "box-multiple-6",
            Self::BoxMultiple7 => "box-multiple-7",
            Self::BoxMultiple8 => "box-multiple-8",
            Self::BoxMultiple9 => "box-multiple-9",
            Self::Certificate => "certificate",
            Self::Certificate2 => "certificate-2",
            Self::Certificate2Off => "certificate-2-off",
            Self::CertificateOff => "certificate-off",
            Self::Chalkboard => "chalkboard",
            Self::ChalkboardOff => "chalkboard-off",
            Self::CircleDashedLetterA => "circle-dashed-letter-a",
            Self::CircleDashedLetterB => "circle-dashed-letter-b",
            Self::CircleDashedLetterC => "circle-dashed-letter-c",
            Self::CircleDashedLetterD => "circle-dashed-letter-d",
            Self::CircleDashedLetterE => "circle-dashed-letter-e",
            Self::CircleDashedLetterF => "circle-dashed-letter-f",
            Self::CircleDashedLetterG => "circle-dashed-letter-g",
            Self::CircleDashedLetterH => "circle-dashed-letter-h",
            Self::CircleDashedLetterI => "circle-dashed-letter-i",
            Self::CircleDashedLetterJ => "circle-dashed-letter-j",
            Self::CircleDashedLetterK => "circle-dashed-letter-k",
            Self::CircleDashedLetterL => "circle-dashed-letter-l",
            Self::CircleDashedLetterM => "circle-dashed-letter-m",
            Self::CircleDashedLetterN => "circle-dashed-letter-n",
            Self::CircleDashedLetterO => "circle-dashed-letter-o",
            Self::CircleDashedLetterP => "circle-dashed-letter-p",
            Self::CircleDashedLetterQ => "circle-dashed-letter-q",
            Self::CircleDashedLetterR => "circle-dashed-letter-r",
            Self::CircleDashedLetterS => "circle-dashed-letter-s",
            Self::CircleDashedLetterT => "circle-dashed-letter-t",
            Self::CircleDashedLetterU => "circle-dashed-letter-u",
            Self::CircleDashedLetterV => "circle-dashed-letter-v",
            Self::CircleDashedLetterW => "circle-dashed-letter-w",
            Self::CircleDashedLetterX => "circle-dashed-letter-x",
            Self::CircleDashedLetterY => "circle-dashed-letter-y",
            Self::CircleDashedLetterZ => "circle-dashed-letter-z",
            Self::CircleDashedNumber0 => "circle-dashed-number-0",
            Self::CircleDashedNumber1 => "circle-dashed-number-1",
            Self::CircleDashedNumber2 => "circle-dashed-number-2",
            Self::CircleDashedNumber3 => "circle-dashed-number-3",
            Self::CircleDashedNumber4 => "circle-dashed-number-4",
            Self::CircleDashedNumber5 => "circle-dashed-number-5",
            Self::CircleDashedNumber6 => "circle-dashed-number-6",
            Self::CircleDashedNumber7 => "circle-dashed-number-7",
            Self::CircleDashedNumber8 => "circle-dashed-number-8",
            Self::CircleDashedNumber9 => "circle-dashed-number-9",
            Self::CircleDottedLetterA => "circle-dotted-letter-a",
            Self::CircleDottedLetterB => "circle-dotted-letter-b",
            Self::CircleDottedLetterC => "circle-dotted-letter-c",
            Self::CircleDottedLetterD => "circle-dotted-letter-d",
            Self::CircleDottedLetterE => "circle-dotted-letter-e",
            Self::CircleDottedLetterF => "circle-dotted-letter-f",
            Self::CircleDottedLetterG => "circle-dotted-letter-g",
            Self::CircleDottedLetterH => "circle-dotted-letter-h",
            Self::CircleDottedLetterI => "circle-dotted-letter-i",
            Self::CircleDottedLetterJ => "circle-dotted-letter-j",
            Self::CircleDottedLetterK => "circle-dotted-letter-k",
            Self::CircleDottedLetterL => "circle-dotted-letter-l",
            Self::CircleDottedLetterM => "circle-dotted-letter-m",
            Self::CircleDottedLetterN => "circle-dotted-letter-n",
            Self::CircleDottedLetterO => "circle-dotted-letter-o",
            Self::CircleDottedLetterP => "circle-dotted-letter-p",
            Self::CircleDottedLetterQ => "circle-dotted-letter-q",
            Self::CircleDottedLetterR => "circle-dotted-letter-r",
            Self::CircleDottedLetterS => "circle-dotted-letter-s",
            Self::CircleDottedLetterT => "circle-dotted-letter-t",
            Self::CircleDottedLetterU => "circle-dotted-letter-u",
            Self::CircleDottedLetterV => "circle-dotted-letter-v",
            Self::CircleDottedLetterW => "circle-dotted-letter-w",
            Self::CircleDottedLetterX => "circle-dotted-letter-x",
            Self::CircleDottedLetterY => "circle-dotted-letter-y",
            Self::CircleDottedLetterZ => "circle-dotted-letter-z",
            Self::CircleLetterA => "circle-letter-a",
            Self::CircleLetterB => "circle-letter-b",
            Self::CircleLetterC => "circle-letter-c",
            Self::CircleLetterD => "circle-letter-d",
            Self::CircleLetterE => "circle-letter-e",
            Self::CircleLetterF => "circle-letter-f",
            Self::CircleLetterG => "circle-letter-g",
            Self::CircleLetterH => "circle-letter-h",
            Self::CircleLetterI => "circle-letter-i",
            Self::CircleLetterJ => "circle-letter-j",
            Self::CircleLetterK => "circle-letter-k",
            Self::CircleLetterL => "circle-letter-l",
            Self::CircleLetterM => "circle-letter-m",
            Self::CircleLetterN => "circle-letter-n",
            Self::CircleLetterO => "circle-letter-o",
            Self::CircleLetterP => "circle-letter-p",
            Self::CircleLetterQ => "circle-letter-q",
            Self::CircleLetterR => "circle-letter-r",
            Self::CircleLetterS => "circle-letter-s",
            Self::CircleLetterT => "circle-letter-t",
            Self::CircleLetterU => "circle-letter-u",
            Self::CircleLetterV => "circle-letter-v",
            Self::CircleLetterW => "circle-letter-w",
            Self::CircleLetterX => "circle-letter-x",
            Self::CircleLetterY => "circle-letter-y",
            Self::CircleLetterZ => "circle-letter-z",
            Self::CircleNumber0 => "circle-number-0",
            Self::CircleNumber1 => "circle-number-1",
            Self::CircleNumber2 => "circle-number-2",
            Self::CircleNumber3 => "circle-number-3",
            Self::CircleNumber4 => "circle-number-4",
            Self::CircleNumber5 => "circle-number-5",
            Self::CircleNumber6 => "circle-number-6",
            Self::CircleNumber7 => "circle-number-7",
            Self::CircleNumber8 => "circle-number-8",
            Self::CircleNumber9 => "circle-number-9",
            Self::ClearFormatting => "clear-formatting",
            Self::Clipboard => "clipboard",
            Self::ClipboardCheck => "clipboard-check",
            Self::ClipboardCopy => "clipboard-copy",
            Self::ClipboardData => "clipboard-data",
            Self::ClipboardHeart => "clipboard-heart",
            Self::ClipboardList => "clipboard-list",
            Self::ClipboardOff => "clipboard-off",
            Self::ClipboardPlus => "clipboard-plus",
            Self::ClipboardSearch => "clipboard-search",
            Self::ClipboardSmile => "clipboard-smile",
            Self::ClipboardText => "clipboard-text",
            Self::ClipboardTypography => "clipboard-typography",
            Self::ClipboardX => "clipboard-x",
            Self::Code => "code",
            Self::CodeAsterisk => "code-asterisk",
            Self::CodeCircle => "code-circle",
            Self::CodeCircle2 => "code-circle-2",
            Self::CodeDots => "code-dots",
            Self::CodeMinus => "code-minus",
            Self::CodeOff => "code-off",
            Self::CodePlus => "code-plus",
            Self::Columns => "columns",
            Self::Columns1 => "columns-1",
            Self::Columns2 => "columns-2",
            Self::Columns3 => "columns-3",
            Self::ColumnsOff => "columns-off",
            Self::Contract => "contract",
            Self::Copy => "copy",
            Self::CopyCheck => "copy-check",
            Self::CopyMinus => "copy-minus",
            Self::CopyOff => "copy-off",
            Self::CopyPlus => "copy-plus",
            Self::CopyX => "copy-x",
            Self::CursorText => "cursor-text",
            Self::Delta => "delta",
            Self::Emphasis => "emphasis",
            Self::Eraser => "eraser",
            Self::EraserOff => "eraser-off",
            Self::File => "file",
            Self::File3d => "file-3d",
            Self::FileAi => "file-ai",
            Self::FileAlert => "file-alert",
            Self::FileAnalytics => "file-analytics",
            Self::FileArrowLeft => "file-arrow-left",
            Self::FileArrowRight => "file-arrow-right",
            Self::FileBarcode => "file-barcode",
            Self::FileBitcoin => "file-bitcoin",
            Self::FileBroken => "file-broken",
            Self::FileCertificate => "file-certificate",
            Self::FileChart => "file-chart",
            Self::FileCheck => "file-check",
            Self::FileCode => "file-code",
            Self::FileCode2 => "file-code-2",
            Self::FileCv => "file-cv",
            Self::FileDatabase => "file-database",
            Self::FileDelta => "file-delta",
            Self::FileDescription => "file-description",
            Self::FileDiff => "file-diff",
            Self::FileDigit => "file-digit",
            Self::FileDislike => "file-dislike",
            Self::FileDollar => "file-dollar",
            Self::FileDots => "file-dots",
            Self::FileDownload => "file-download",
            Self::FileEuro => "file-euro",
            Self::FileExcel => "file-excel",
            Self::FileExport => "file-export",
            Self::FileFunction => "file-function",
            Self::FileHorizontal => "file-horizontal",
            Self::FileImport => "file-import",
            Self::FileInfinity => "file-infinity",
            Self::FileInfo => "file-info",
            Self::FileInvoice => "file-invoice",
            Self::FileIsr => "file-isr",
            Self::FileLambda => "file-lambda",
            Self::FileLike => "file-like",
            Self::FileMinus => "file-minus",
            Self::FileMusic => "file-music",
            Self::FileNeutral => "file-neutral",
            Self::FileOff => "file-off",
            Self::FileOrientation => "file-orientation",
            Self::FilePencil => "file-pencil",
            Self::FilePercent => "file-percent",
            Self::FilePhone => "file-phone",
            Self::FilePlus => "file-plus",
            Self::FilePower => "file-power",
            Self::FileReport => "file-report",
            Self::FileRss => "file-rss",
            Self::FileSad => "file-sad",
            Self::FileScissors => "file-scissors",
            Self::FileSearch => "file-search",
            Self::FileSettings => "file-settings",
            Self::FileShredder => "file-shredder",
            Self::FileSignal => "file-signal",
            Self::FileSmile => "file-smile",
            Self::FileSpark => "file-spark",
            Self::FileSpreadsheet => "file-spreadsheet",
            Self::FileStack => "file-stack",
            Self::FileStar => "file-star",
            Self::FileSymlink => "file-symlink",
            Self::FileText => "file-text",
            Self::FileTextAi => "file-text-ai",
            Self::FileTextShield => "file-text-shield",
            Self::FileTextSpark => "file-text-spark",
            Self::FileTime => "file-time",
            Self::FileTypeBmp => "file-type-bmp",
            Self::FileTypeCss => "file-type-css",
            Self::FileTypeCsv => "file-type-csv",
            Self::FileTypeDoc => "file-type-doc",
            Self::FileTypeDocx => "file-type-docx",
            Self::FileTypeHtml => "file-type-html",
            Self::FileTypeJpg => "file-type-jpg",
            Self::FileTypeJs => "file-type-js",
            Self::FileTypeJsx => "file-type-jsx",
            Self::FileTypePdf => "file-type-pdf",
            Self::FileTypePhp => "file-type-php",
            Self::FileTypePng => "file-type-png",
            Self::FileTypePpt => "file-type-ppt",
            Self::FileTypeRs => "file-type-rs",
            Self::FileTypeSql => "file-type-sql",
            Self::FileTypeSvg => "file-type-svg",
            Self::FileTypeTs => "file-type-ts",
            Self::FileTypeTsx => "file-type-tsx",
            Self::FileTypeTxt => "file-type-txt",
            Self::FileTypeVue => "file-type-vue",
            Self::FileTypeXls => "file-type-xls",
            Self::FileTypeXml => "file-type-xml",
            Self::FileTypeZip => "file-type-zip",
            Self::FileTypography => "file-typography",
            Self::FileUnknown => "file-unknown",
            Self::FileUpload => "file-upload",
            Self::FileVector => "file-vector",
            Self::FileWord => "file-word",
            Self::FileX => "file-x",
            Self::FileZip => "file-zip",
            Self::Files => "files",
            Self::FilesOff => "files-off",
            Self::FloatCenter => "float-center",
            Self::FloatLeft => "float-left",
            Self::FloatNone => "float-none",
            Self::FloatRight => "float-right",
            Self::Folder => "folder",
            Self::FolderBolt => "folder-bolt",
            Self::FolderCancel => "folder-cancel",
            Self::FolderCheck => "folder-check",
            Self::FolderCode => "folder-code",
            Self::FolderCog => "folder-cog",
            Self::FolderDollar => "folder-dollar",
            Self::FolderDown => "folder-down",
            Self::FolderExclamation => "folder-exclamation",
            Self::FolderHeart => "folder-heart",
            Self::FolderMinus => "folder-minus",
            Self::FolderOff => "folder-off",
            Self::FolderOpen => "folder-open",
            Self::FolderPause => "folder-pause",
            Self::FolderPin => "folder-pin",
            Self::FolderPlus => "folder-plus",
            Self::FolderQuestion => "folder-question",
            Self::FolderRoot => "folder-root",
            Self::FolderSearch => "folder-search",
            Self::FolderShare => "folder-share",
            Self::FolderStar => "folder-star",
            Self::FolderSymlink => "folder-symlink",
            Self::FolderUp => "folder-up",
            Self::FolderX => "folder-x",
            Self::Folders => "folders",
            Self::FoldersOff => "folders-off",
            Self::Forms => "forms",
            Self::H1 => "h-1",
            Self::H2 => "h-2",
            Self::H3 => "h-3",
            Self::H4 => "h-4",
            Self::H5 => "h-5",
            Self::H6 => "h-6",
            Self::Heading => "heading",
            Self::HeadingOff => "heading-off",
            Self::HexagonLetterA => "hexagon-letter-a",
            Self::HexagonLetterB => "hexagon-letter-b",
            Self::HexagonLetterC => "hexagon-letter-c",
            Self::HexagonLetterD => "hexagon-letter-d",
            Self::HexagonLetterE => "hexagon-letter-e",
            Self::HexagonLetterF => "hexagon-letter-f",
            Self::HexagonLetterG => "hexagon-letter-g",
            Self::HexagonLetterH => "hexagon-letter-h",
            Self::HexagonLetterI => "hexagon-letter-i",
            Self::HexagonLetterJ => "hexagon-letter-j",
            Self::HexagonLetterK => "hexagon-letter-k",
            Self::HexagonLetterL => "hexagon-letter-l",
            Self::HexagonLetterM => "hexagon-letter-m",
            Self::HexagonLetterN => "hexagon-letter-n",
            Self::HexagonLetterO => "hexagon-letter-o",
            Self::HexagonLetterP => "hexagon-letter-p",
            Self::HexagonLetterQ => "hexagon-letter-q",
            Self::HexagonLetterR => "hexagon-letter-r",
            Self::HexagonLetterS => "hexagon-letter-s",
            Self::HexagonLetterT => "hexagon-letter-t",
            Self::HexagonLetterU => "hexagon-letter-u",
            Self::HexagonLetterV => "hexagon-letter-v",
            Self::HexagonLetterW => "hexagon-letter-w",
            Self::HexagonLetterX => "hexagon-letter-x",
            Self::HexagonLetterY => "hexagon-letter-y",
            Self::HexagonLetterZ => "hexagon-letter-z",
            Self::HexagonNumber0 => "hexagon-number-0",
            Self::HexagonNumber1 => "hexagon-number-1",
            Self::HexagonNumber2 => "hexagon-number-2",
            Self::HexagonNumber3 => "hexagon-number-3",
            Self::HexagonNumber4 => "hexagon-number-4",
            Self::HexagonNumber5 => "hexagon-number-5",
            Self::HexagonNumber6 => "hexagon-number-6",
            Self::HexagonNumber7 => "hexagon-number-7",
            Self::HexagonNumber8 => "hexagon-number-8",
            Self::HexagonNumber9 => "hexagon-number-9",
            Self::Highlight => "highlight",
            Self::HighlightOff => "highlight-off",
            Self::IndentDecrease => "indent-decrease",
            Self::IndentIncrease => "indent-increase",
            Self::InputSearch => "input-search",
            Self::Invoice => "invoice",
            Self::Italic => "italic",
            Self::Kerning => "kerning",
            Self::Lambda => "lambda",
            Self::Language => "language",
            Self::LanguageHiragana => "language-hiragana",
            Self::LanguageKatakana => "language-katakana",
            Self::LanguageOff => "language-off",
            Self::LetterA => "letter-a",
            Self::LetterASmall => "letter-a-small",
            Self::LetterB => "letter-b",
            Self::LetterBSmall => "letter-b-small",
            Self::LetterC => "letter-c",
            Self::LetterCSmall => "letter-c-small",
            Self::LetterCase => "letter-case",
            Self::LetterCaseLower => "letter-case-lower",
            Self::LetterCaseToggle => "letter-case-toggle",
            Self::LetterCaseUpper => "letter-case-upper",
            Self::LetterD => "letter-d",
            Self::LetterDSmall => "letter-d-small",
            Self::LetterE => "letter-e",
            Self::LetterESmall => "letter-e-small",
            Self::LetterF => "letter-f",
            Self::LetterFSmall => "letter-f-small",
            Self::LetterG => "letter-g",
            Self::LetterGSmall => "letter-g-small",
            Self::LetterH => "letter-h",
            Self::LetterHSmall => "letter-h-small",
            Self::LetterI => "letter-i",
            Self::LetterISmall => "letter-i-small",
            Self::LetterJ => "letter-j",
            Self::LetterJSmall => "letter-j-small",
            Self::LetterK => "letter-k",
            Self::LetterKSmall => "letter-k-small",
            Self::LetterL => "letter-l",
            Self::LetterLSmall => "letter-l-small",
            Self::LetterM => "letter-m",
            Self::LetterMSmall => "letter-m-small",
            Self::LetterN => "letter-n",
            Self::LetterNSmall => "letter-n-small",
            Self::LetterO => "letter-o",
            Self::LetterOSmall => "letter-o-small",
            Self::LetterP => "letter-p",
            Self::LetterPSmall => "letter-p-small",
            Self::LetterQ => "letter-q",
            Self::LetterQSmall => "letter-q-small",
            Self::LetterR => "letter-r",
            Self::LetterRSmall => "letter-r-small",
            Self::LetterS => "letter-s",
            Self::LetterSSmall => "letter-s-small",
            Self::LetterSpacing => "letter-spacing",
            Self::LetterT => "letter-t",
            Self::LetterTSmall => "letter-t-small",
            Self::LetterU => "letter-u",
            Self::LetterUSmall => "letter-u-small",
            Self::LetterV => "letter-v",
            Self::LetterVSmall => "letter-v-small",
            Self::LetterW => "letter-w",
            Self::LetterWSmall => "letter-w-small",
            Self::LetterX => "letter-x",
            Self::LetterXSmall => "letter-x-small",
            Self::LetterY => "letter-y",
            Self::LetterYSmall => "letter-y-small",
            Self::LetterZ => "letter-z",
            Self::LetterZSmall => "letter-z-small",
            Self::License => "license",
            Self::LicenseOff => "license-off",
            Self::LineHeight => "line-height",
            Self::Link => "link",
            Self::LinkMinus => "link-minus",
            Self::LinkOff => "link-off",
            Self::LinkPlus => "link-plus",
            Self::List => "list",
            Self::ListCheck => "list-check",
            Self::ListDetails => "list-details",
            Self::ListNumbers => "list-numbers",
            Self::ListSearch => "list-search",
            Self::ManualGearbox => "manual-gearbox",
            Self::Markdown => "markdown",
            Self::MarkdownOff => "markdown-off",
            Self::News => "news",
            Self::NewsOff => "news-off",
            Self::Notdef => "notdef",
            Self::Note => "note",
            Self::NoteOff => "note-off",
            Self::Notebook => "notebook",
            Self::NotebookOff => "notebook-off",
            Self::Notes => "notes",
            Self::NotesOff => "notes-off",
            Self::Number => "number",
            Self::Number0 => "number-0",
            Self::Number0Small => "number-0-small",
            Self::Number1 => "number-1",
            Self::Number1Small => "number-1-small",
            Self::Number10 => "number-10",
            Self::Number10Small => "number-10-small",
            Self::Number100Small => "number-100-small",
            Self::Number11 => "number-11",
            Self::Number11Small => "number-11-small",
            Self::Number12Small => "number-12-small",
            Self::Number123 => "number-123",
            Self::Number13Small => "number-13-small",
            Self::Number14Small => "number-14-small",
            Self::Number15Small => "number-15-small",
            Self::Number16Small => "number-16-small",
            Self::Number17Small => "number-17-small",
            Self::Number18Small => "number-18-small",
            Self::Number19Small => "number-19-small",
            Self::Number2 => "number-2",
            Self::Number2Small => "number-2-small",
            Self::Number20Small => "number-20-small",
            Self::Number21Small => "number-21-small",
            Self::Number22Small => "number-22-small",
            Self::Number23Small => "number-23-small",
            Self::Number24Small => "number-24-small",
            Self::Number25Small => "number-25-small",
            Self::Number26Small => "number-26-small",
            Self::Number27Small => "number-27-small",
            Self::Number28Small => "number-28-small",
            Self::Number29Small => "number-29-small",
            Self::Number3 => "number-3",
            Self::Number3Small => "number-3-small",
            Self::Number30Small => "number-30-small",
            Self::Number31Small => "number-31-small",
            Self::Number32Small => "number-32-small",
            Self::Number33Small => "number-33-small",
            Self::Number34Small => "number-34-small",
            Self::Number35Small => "number-35-small",
            Self::Number36Small => "number-36-small",
            Self::Number37Small => "number-37-small",
            Self::Number38Small => "number-38-small",
            Self::Number39Small => "number-39-small",
            Self::Number4 => "number-4",
            Self::Number4Small => "number-4-small",
            Self::Number40Small => "number-40-small",
            Self::Number41Small => "number-41-small",
            Self::Number42Small => "number-42-small",
            Self::Number43Small => "number-43-small",
            Self::Number44Small => "number-44-small",
            Self::Number45Small => "number-45-small",
            Self::Number46Small => "number-46-small",
            Self::Number47Small => "number-47-small",
            Self::Number48Small => "number-48-small",
            Self::Number49Small => "number-49-small",
            Self::Number5 => "number-5",
            Self::Number5Small => "number-5-small",
            Self::Number50Small => "number-50-small",
            Self::Number51Small => "number-51-small",
            Self::Number52Small => "number-52-small",
            Self::Number53Small => "number-53-small",
            Self::Number54Small => "number-54-small",
            Self::Number55Small => "number-55-small",
            Self::Number56Small => "number-56-small",
            Self::Number57Small => "number-57-small",
            Self::Number58Small => "number-58-small",
            Self::Number59Small => "number-59-small",
            Self::Number6 => "number-6",
            Self::Number6Small => "number-6-small",
            Self::Number60Small => "number-60-small",
            Self::Number61Small => "number-61-small",
            Self::Number62Small => "number-62-small",
            Self::Number63Small => "number-63-small",
            Self::Number64Small => "number-64-small",
            Self::Number65Small => "number-65-small",
            Self::Number66Small => "number-66-small",
            Self::Number67Small => "number-67-small",
            Self::Number68Small => "number-68-small",
            Self::Number69Small => "number-69-small",
            Self::Number7 => "number-7",
            Self::Number7Small => "number-7-small",
            Self::Number70Small => "number-70-small",
            Self::Number71Small => "number-71-small",
            Self::Number72Small => "number-72-small",
            Self::Number73Small => "number-73-small",
            Self::Number74Small => "number-74-small",
            Self::Number75Small => "number-75-small",
            Self::Number76Small => "number-76-small",
            Self::Number77Small => "number-77-small",
            Self::Number78Small => "number-78-small",
            Self::Number79Small => "number-79-small",
            Self::Number8 => "number-8",
            Self::Number8Small => "number-8-small",
            Self::Number80Small => "number-80-small",
            Self::Number81Small => "number-81-small",
            Self::Number82Small => "number-82-small",
            Self::Number83Small => "number-83-small",
            Self::Number84Small => "number-84-small",
            Self::Number85Small => "number-85-small",
            Self::Number86Small => "number-86-small",
            Self::Number87Small => "number-87-small",
            Self::Number88Small => "number-88-small",
            Self::Number89Small => "number-89-small",
            Self::Number9 => "number-9",
            Self::Number9Small => "number-9-small",
            Self::Number90Small => "number-90-small",
            Self::Number91Small => "number-91-small",
            Self::Number92Small => "number-92-small",
            Self::Number93Small => "number-93-small",
            Self::Number94Small => "number-94-small",
            Self::Number95Small => "number-95-small",
            Self::Number96Small => "number-96-small",
            Self::Number97Small => "number-97-small",
            Self::Number98Small => "number-98-small",
            Self::Number99Small => "number-99-small",
            Self::Numbers => "numbers",
            Self::Overline => "overline",
            Self::PageBreak => "page-break",
            Self::Paperclip => "paperclip",
            Self::PentagonNumber0 => "pentagon-number-0",
            Self::PentagonNumber1 => "pentagon-number-1",
            Self::PentagonNumber2 => "pentagon-number-2",
            Self::PentagonNumber3 => "pentagon-number-3",
            Self::PentagonNumber4 => "pentagon-number-4",
            Self::PentagonNumber5 => "pentagon-number-5",
            Self::PentagonNumber6 => "pentagon-number-6",
            Self::PentagonNumber7 => "pentagon-number-7",
            Self::PentagonNumber8 => "pentagon-number-8",
            Self::PentagonNumber9 => "pentagon-number-9",
            Self::Pilcrow => "pilcrow",
            Self::PilcrowLeft => "pilcrow-left",
            Self::PilcrowRight => "pilcrow-right",
            Self::Presentation => "presentation",
            Self::PresentationAnalytics => "presentation-analytics",
            Self::PresentationOff => "presentation-off",
            Self::Quote => "quote",
            Self::QuoteOff => "quote-off",
            Self::QuoteOpen => "quote-open",
            Self::Quotes => "quotes",
            Self::Receipt => "receipt",
            Self::Receipt2 => "receipt-2",
            Self::ReceiptDollar => "receipt-dollar",
            Self::ReceiptOff => "receipt-off",
            Self::ReceiptRefund => "receipt-refund",
            Self::ReceiptTax => "receipt-tax",
            Self::Regex => "regex",
            Self::RegexOff => "regex-off",
            Self::Report => "report",
            Self::ReportAnalytics => "report-analytics",
            Self::ReportMedical => "report-medical",
            Self::ReportMoney => "report-money",
            Self::ReportOff => "report-off",
            Self::ReportSearch => "report-search",
            Self::RosetteNumber0 => "rosette-number-0",
            Self::RosetteNumber1 => "rosette-number-1",
            Self::RosetteNumber2 => "rosette-number-2",
            Self::RosetteNumber3 => "rosette-number-3",
            Self::RosetteNumber4 => "rosette-number-4",
            Self::RosetteNumber5 => "rosette-number-5",
            Self::RosetteNumber6 => "rosette-number-6",
            Self::RosetteNumber7 => "rosette-number-7",
            Self::RosetteNumber8 => "rosette-number-8",
            Self::RosetteNumber9 => "rosette-number-9",
            Self::RubberStamp => "rubber-stamp",
            Self::RubberStampOff => "rubber-stamp-off",
            Self::ScanLetterA => "scan-letter-a",
            Self::ScanLetterT => "scan-letter-t",
            Self::Scribble => "scribble",
            Self::ScribbleOff => "scribble-off",
            Self::Script => "script",
            Self::ScriptMinus => "script-minus",
            Self::ScriptPlus => "script-plus",
            Self::ScriptX => "script-x",
            Self::SectionSign => "section-sign",
            Self::Separator => "separator",
            Self::SeparatorHorizontal => "separator-horizontal",
            Self::SeparatorVertical => "separator-vertical",
            Self::Signature => "signature",
            Self::SignatureOff => "signature-off",
            Self::Slideshow => "slideshow",
            Self::Sort09 => "sort-0-9",
            Self::Sort90 => "sort-9-0",
            Self::SortAZ => "sort-a-z",
            Self::SortAscending => "sort-ascending",
            Self::SortAscending2 => "sort-ascending-2",
            Self::SortAscendingLetters => "sort-ascending-letters",
            Self::SortAscendingNumbers => "sort-ascending-numbers",
            Self::SortDescending => "sort-descending",
            Self::SortDescending2 => "sort-descending-2",
            Self::SortDescendingLetters => "sort-descending-letters",
            Self::SortDescendingNumbers => "sort-descending-numbers",
            Self::SortZA => "sort-z-a",
            Self::Space => "space",
            Self::SpaceOff => "space-off",
            Self::SpacingHorizontal => "spacing-horizontal",
            Self::SpacingVertical => "spacing-vertical",
            Self::SquareLetterA => "square-letter-a",
            Self::SquareLetterB => "square-letter-b",
            Self::SquareLetterC => "square-letter-c",
            Self::SquareLetterD => "square-letter-d",
            Self::SquareLetterE => "square-letter-e",
            Self::SquareLetterF => "square-letter-f",
            Self::SquareLetterG => "square-letter-g",
            Self::SquareLetterH => "square-letter-h",
            Self::SquareLetterI => "square-letter-i",
            Self::SquareLetterJ => "square-letter-j",
            Self::SquareLetterK => "square-letter-k",
            Self::SquareLetterL => "square-letter-l",
            Self::SquareLetterM => "square-letter-m",
            Self::SquareLetterN => "square-letter-n",
            Self::SquareLetterO => "square-letter-o",
            Self::SquareLetterP => "square-letter-p",
            Self::SquareLetterQ => "square-letter-q",
            Self::SquareLetterR => "square-letter-r",
            Self::SquareLetterS => "square-letter-s",
            Self::SquareLetterT => "square-letter-t",
            Self::SquareLetterU => "square-letter-u",
            Self::SquareLetterV => "square-letter-v",
            Self::SquareLetterW => "square-letter-w",
            Self::SquareLetterX => "square-letter-x",
            Self::SquareLetterY => "square-letter-y",
            Self::SquareLetterZ => "square-letter-z",
            Self::SquareNumber0 => "square-number-0",
            Self::SquareNumber1 => "square-number-1",
            Self::SquareNumber2 => "square-number-2",
            Self::SquareNumber3 => "square-number-3",
            Self::SquareNumber4 => "square-number-4",
            Self::SquareNumber5 => "square-number-5",
            Self::SquareNumber6 => "square-number-6",
            Self::SquareNumber7 => "square-number-7",
            Self::SquareNumber8 => "square-number-8",
            Self::SquareNumber9 => "square-number-9",
            Self::SquareRoundedLetterA => "square-rounded-letter-a",
            Self::SquareRoundedLetterB => "square-rounded-letter-b",
            Self::SquareRoundedLetterC => "square-rounded-letter-c",
            Self::SquareRoundedLetterD => "square-rounded-letter-d",
            Self::SquareRoundedLetterE => "square-rounded-letter-e",
            Self::SquareRoundedLetterF => "square-rounded-letter-f",
            Self::SquareRoundedLetterG => "square-rounded-letter-g",
            Self::SquareRoundedLetterH => "square-rounded-letter-h",
            Self::SquareRoundedLetterI => "square-rounded-letter-i",
            Self::SquareRoundedLetterJ => "square-rounded-letter-j",
            Self::SquareRoundedLetterK => "square-rounded-letter-k",
            Self::SquareRoundedLetterL => "square-rounded-letter-l",
            Self::SquareRoundedLetterM => "square-rounded-letter-m",
            Self::SquareRoundedLetterN => "square-rounded-letter-n",
            Self::SquareRoundedLetterO => "square-rounded-letter-o",
            Self::SquareRoundedLetterP => "square-rounded-letter-p",
            Self::SquareRoundedLetterQ => "square-rounded-letter-q",
            Self::SquareRoundedLetterR => "square-rounded-letter-r",
            Self::SquareRoundedLetterS => "square-rounded-letter-s",
            Self::SquareRoundedLetterT => "square-rounded-letter-t",
            Self::SquareRoundedLetterU => "square-rounded-letter-u",
            Self::SquareRoundedLetterV => "square-rounded-letter-v",
            Self::SquareRoundedLetterW => "square-rounded-letter-w",
            Self::SquareRoundedLetterX => "square-rounded-letter-x",
            Self::SquareRoundedLetterY => "square-rounded-letter-y",
            Self::SquareRoundedLetterZ => "square-rounded-letter-z",
            Self::SquareRoundedNumber0 => "square-rounded-number-0",
            Self::SquareRoundedNumber1 => "square-rounded-number-1",
            Self::SquareRoundedNumber2 => "square-rounded-number-2",
            Self::SquareRoundedNumber3 => "square-rounded-number-3",
            Self::SquareRoundedNumber4 => "square-rounded-number-4",
            Self::SquareRoundedNumber5 => "square-rounded-number-5",
            Self::SquareRoundedNumber6 => "square-rounded-number-6",
            Self::SquareRoundedNumber7 => "square-rounded-number-7",
            Self::SquareRoundedNumber8 => "square-rounded-number-8",
            Self::SquareRoundedNumber9 => "square-rounded-number-9",
            Self::Strikethrough => "strikethrough",
            Self::Subscript => "subscript",
            Self::Superscript => "superscript",
            Self::Tex => "tex",
            Self::TextCaption => "text-caption",
            Self::TextColor => "text-color",
            Self::TextDecrease => "text-decrease",
            Self::TextDirectionLtr => "text-direction-ltr",
            Self::TextDirectionRtl => "text-direction-rtl",
            Self::TextGrammar => "text-grammar",
            Self::TextIncrease => "text-increase",
            Self::TextOrientation => "text-orientation",
            Self::TextPlus => "text-plus",
            Self::TextRecognition => "text-recognition",
            Self::TextSize => "text-size",
            Self::TextSpellcheck => "text-spellcheck",
            Self::TextWrap => "text-wrap",
            Self::TextWrapColumn => "text-wrap-column",
            Self::TextWrapDisabled => "text-wrap-disabled",
            Self::Ticket => "ticket",
            Self::TicketOff => "ticket-off",
            Self::Typography => "typography",
            Self::TypographyOff => "typography-off",
            Self::Underline => "underline",
            Self::Unlink => "unlink",
            Self::Vocabulary => "vocabulary",
            Self::VocabularyOff => "vocabulary-off",
            Self::Writing => "writing",
            Self::WritingOff => "writing-off",
            Self::WritingSign => "writing-sign",
            Self::WritingSignOff => "writing-sign-off",
        }
    }

    fn outline_svg(&self) -> &'static str {
        match self {
            Self::AB2 => A_B_2_SVG,
            Self::ABOff => A_B_OFF_SVG,
            Self::Abc => ABC_SVG,
            Self::AlignBoxBottomCenter => ALIGN_BOX_BOTTOM_CENTER_SVG,
            Self::AlignBoxBottomLeft => ALIGN_BOX_BOTTOM_LEFT_SVG,
            Self::AlignBoxBottomRight => ALIGN_BOX_BOTTOM_RIGHT_SVG,
            Self::AlignBoxCenterBottom => ALIGN_BOX_CENTER_BOTTOM_SVG,
            Self::AlignBoxCenterMiddle => ALIGN_BOX_CENTER_MIDDLE_SVG,
            Self::AlignBoxCenterStretch => ALIGN_BOX_CENTER_STRETCH_SVG,
            Self::AlignBoxCenterTop => ALIGN_BOX_CENTER_TOP_SVG,
            Self::AlignBoxLeftBottom => ALIGN_BOX_LEFT_BOTTOM_SVG,
            Self::AlignBoxLeftMiddle => ALIGN_BOX_LEFT_MIDDLE_SVG,
            Self::AlignBoxLeftStretch => ALIGN_BOX_LEFT_STRETCH_SVG,
            Self::AlignBoxLeftTop => ALIGN_BOX_LEFT_TOP_SVG,
            Self::AlignBoxRightBottom => ALIGN_BOX_RIGHT_BOTTOM_SVG,
            Self::AlignBoxRightMiddle => ALIGN_BOX_RIGHT_MIDDLE_SVG,
            Self::AlignBoxRightStretch => ALIGN_BOX_RIGHT_STRETCH_SVG,
            Self::AlignBoxRightTop => ALIGN_BOX_RIGHT_TOP_SVG,
            Self::AlignBoxTopCenter => ALIGN_BOX_TOP_CENTER_SVG,
            Self::AlignBoxTopLeft => ALIGN_BOX_TOP_LEFT_SVG,
            Self::AlignBoxTopRight => ALIGN_BOX_TOP_RIGHT_SVG,
            Self::AlignCenter => ALIGN_CENTER_SVG,
            Self::AlignJustified => ALIGN_JUSTIFIED_SVG,
            Self::AlignLeft => ALIGN_LEFT_SVG,
            Self::AlignLeft2 => ALIGN_LEFT_2_SVG,
            Self::AlignRight => ALIGN_RIGHT_SVG,
            Self::AlignRight2 => ALIGN_RIGHT_2_SVG,
            Self::Alpha => ALPHA_SVG,
            Self::AlphabetArabic => ALPHABET_ARABIC_SVG,
            Self::AlphabetBangla => ALPHABET_BANGLA_SVG,
            Self::AlphabetCyrillic => ALPHABET_CYRILLIC_SVG,
            Self::AlphabetGreek => ALPHABET_GREEK_SVG,
            Self::AlphabetHebrew => ALPHABET_HEBREW_SVG,
            Self::AlphabetKorean => ALPHABET_KOREAN_SVG,
            Self::AlphabetLatin => ALPHABET_LATIN_SVG,
            Self::AlphabetPolish => ALPHABET_POLISH_SVG,
            Self::AlphabetRunes => ALPHABET_RUNES_SVG,
            Self::AlphabetThai => ALPHABET_THAI_SVG,
            Self::Archive => ARCHIVE_SVG,
            Self::ArchiveOff => ARCHIVE_OFF_SVG,
            Self::Article => ARTICLE_SVG,
            Self::ArticleOff => ARTICLE_OFF_SVG,
            Self::Asterisk => ASTERISK_SVG,
            Self::AsteriskSimple => ASTERISK_SIMPLE_SVG,
            Self::At => AT_SVG,
            Self::AtOff => AT_OFF_SVG,
            Self::Backspace => BACKSPACE_SVG,
            Self::Ballpen => BALLPEN_SVG,
            Self::BallpenOff => BALLPEN_OFF_SVG,
            Self::Baseline => BASELINE_SVG,
            Self::BaselineDensityLarge => BASELINE_DENSITY_LARGE_SVG,
            Self::BaselineDensityMedium => BASELINE_DENSITY_MEDIUM_SVG,
            Self::BaselineDensitySmall => BASELINE_DENSITY_SMALL_SVG,
            Self::Beta => BETA_SVG,
            Self::Bible => BIBLE_SVG,
            Self::Blockquote => BLOCKQUOTE_SVG,
            Self::Bold => BOLD_SVG,
            Self::BoldOff => BOLD_OFF_SVG,
            Self::Book => BOOK_SVG,
            Self::Book2 => BOOK_2_SVG,
            Self::BookDownload => BOOK_DOWNLOAD_SVG,
            Self::BookOff => BOOK_OFF_SVG,
            Self::BookUpload => BOOK_UPLOAD_SVG,
            Self::Bookmark => BOOKMARK_SVG,
            Self::BookmarkAi => BOOKMARK_AI_SVG,
            Self::BookmarkEdit => BOOKMARK_EDIT_SVG,
            Self::BookmarkMinus => BOOKMARK_MINUS_SVG,
            Self::BookmarkOff => BOOKMARK_OFF_SVG,
            Self::BookmarkPlus => BOOKMARK_PLUS_SVG,
            Self::BookmarkQuestion => BOOKMARK_QUESTION_SVG,
            Self::Bookmarks => BOOKMARKS_SVG,
            Self::BookmarksOff => BOOKMARKS_OFF_SVG,
            Self::Books => BOOKS_SVG,
            Self::BooksOff => BOOKS_OFF_SVG,
            Self::BoxMultiple0 => BOX_MULTIPLE_0_SVG,
            Self::BoxMultiple1 => BOX_MULTIPLE_1_SVG,
            Self::BoxMultiple2 => BOX_MULTIPLE_2_SVG,
            Self::BoxMultiple3 => BOX_MULTIPLE_3_SVG,
            Self::BoxMultiple4 => BOX_MULTIPLE_4_SVG,
            Self::BoxMultiple5 => BOX_MULTIPLE_5_SVG,
            Self::BoxMultiple6 => BOX_MULTIPLE_6_SVG,
            Self::BoxMultiple7 => BOX_MULTIPLE_7_SVG,
            Self::BoxMultiple8 => BOX_MULTIPLE_8_SVG,
            Self::BoxMultiple9 => BOX_MULTIPLE_9_SVG,
            Self::Certificate => CERTIFICATE_SVG,
            Self::Certificate2 => CERTIFICATE_2_SVG,
            Self::Certificate2Off => CERTIFICATE_2_OFF_SVG,
            Self::CertificateOff => CERTIFICATE_OFF_SVG,
            Self::Chalkboard => CHALKBOARD_SVG,
            Self::ChalkboardOff => CHALKBOARD_OFF_SVG,
            Self::CircleDashedLetterA => CIRCLE_DASHED_LETTER_A_SVG,
            Self::CircleDashedLetterB => CIRCLE_DASHED_LETTER_B_SVG,
            Self::CircleDashedLetterC => CIRCLE_DASHED_LETTER_C_SVG,
            Self::CircleDashedLetterD => CIRCLE_DASHED_LETTER_D_SVG,
            Self::CircleDashedLetterE => CIRCLE_DASHED_LETTER_E_SVG,
            Self::CircleDashedLetterF => CIRCLE_DASHED_LETTER_F_SVG,
            Self::CircleDashedLetterG => CIRCLE_DASHED_LETTER_G_SVG,
            Self::CircleDashedLetterH => CIRCLE_DASHED_LETTER_H_SVG,
            Self::CircleDashedLetterI => CIRCLE_DASHED_LETTER_I_SVG,
            Self::CircleDashedLetterJ => CIRCLE_DASHED_LETTER_J_SVG,
            Self::CircleDashedLetterK => CIRCLE_DASHED_LETTER_K_SVG,
            Self::CircleDashedLetterL => CIRCLE_DASHED_LETTER_L_SVG,
            Self::CircleDashedLetterM => CIRCLE_DASHED_LETTER_M_SVG,
            Self::CircleDashedLetterN => CIRCLE_DASHED_LETTER_N_SVG,
            Self::CircleDashedLetterO => CIRCLE_DASHED_LETTER_O_SVG,
            Self::CircleDashedLetterP => CIRCLE_DASHED_LETTER_P_SVG,
            Self::CircleDashedLetterQ => CIRCLE_DASHED_LETTER_Q_SVG,
            Self::CircleDashedLetterR => CIRCLE_DASHED_LETTER_R_SVG,
            Self::CircleDashedLetterS => CIRCLE_DASHED_LETTER_S_SVG,
            Self::CircleDashedLetterT => CIRCLE_DASHED_LETTER_T_SVG,
            Self::CircleDashedLetterU => CIRCLE_DASHED_LETTER_U_SVG,
            Self::CircleDashedLetterV => CIRCLE_DASHED_LETTER_V_SVG,
            Self::CircleDashedLetterW => CIRCLE_DASHED_LETTER_W_SVG,
            Self::CircleDashedLetterX => CIRCLE_DASHED_LETTER_X_SVG,
            Self::CircleDashedLetterY => CIRCLE_DASHED_LETTER_Y_SVG,
            Self::CircleDashedLetterZ => CIRCLE_DASHED_LETTER_Z_SVG,
            Self::CircleDashedNumber0 => CIRCLE_DASHED_NUMBER_0_SVG,
            Self::CircleDashedNumber1 => CIRCLE_DASHED_NUMBER_1_SVG,
            Self::CircleDashedNumber2 => CIRCLE_DASHED_NUMBER_2_SVG,
            Self::CircleDashedNumber3 => CIRCLE_DASHED_NUMBER_3_SVG,
            Self::CircleDashedNumber4 => CIRCLE_DASHED_NUMBER_4_SVG,
            Self::CircleDashedNumber5 => CIRCLE_DASHED_NUMBER_5_SVG,
            Self::CircleDashedNumber6 => CIRCLE_DASHED_NUMBER_6_SVG,
            Self::CircleDashedNumber7 => CIRCLE_DASHED_NUMBER_7_SVG,
            Self::CircleDashedNumber8 => CIRCLE_DASHED_NUMBER_8_SVG,
            Self::CircleDashedNumber9 => CIRCLE_DASHED_NUMBER_9_SVG,
            Self::CircleDottedLetterA => CIRCLE_DOTTED_LETTER_A_SVG,
            Self::CircleDottedLetterB => CIRCLE_DOTTED_LETTER_B_SVG,
            Self::CircleDottedLetterC => CIRCLE_DOTTED_LETTER_C_SVG,
            Self::CircleDottedLetterD => CIRCLE_DOTTED_LETTER_D_SVG,
            Self::CircleDottedLetterE => CIRCLE_DOTTED_LETTER_E_SVG,
            Self::CircleDottedLetterF => CIRCLE_DOTTED_LETTER_F_SVG,
            Self::CircleDottedLetterG => CIRCLE_DOTTED_LETTER_G_SVG,
            Self::CircleDottedLetterH => CIRCLE_DOTTED_LETTER_H_SVG,
            Self::CircleDottedLetterI => CIRCLE_DOTTED_LETTER_I_SVG,
            Self::CircleDottedLetterJ => CIRCLE_DOTTED_LETTER_J_SVG,
            Self::CircleDottedLetterK => CIRCLE_DOTTED_LETTER_K_SVG,
            Self::CircleDottedLetterL => CIRCLE_DOTTED_LETTER_L_SVG,
            Self::CircleDottedLetterM => CIRCLE_DOTTED_LETTER_M_SVG,
            Self::CircleDottedLetterN => CIRCLE_DOTTED_LETTER_N_SVG,
            Self::CircleDottedLetterO => CIRCLE_DOTTED_LETTER_O_SVG,
            Self::CircleDottedLetterP => CIRCLE_DOTTED_LETTER_P_SVG,
            Self::CircleDottedLetterQ => CIRCLE_DOTTED_LETTER_Q_SVG,
            Self::CircleDottedLetterR => CIRCLE_DOTTED_LETTER_R_SVG,
            Self::CircleDottedLetterS => CIRCLE_DOTTED_LETTER_S_SVG,
            Self::CircleDottedLetterT => CIRCLE_DOTTED_LETTER_T_SVG,
            Self::CircleDottedLetterU => CIRCLE_DOTTED_LETTER_U_SVG,
            Self::CircleDottedLetterV => CIRCLE_DOTTED_LETTER_V_SVG,
            Self::CircleDottedLetterW => CIRCLE_DOTTED_LETTER_W_SVG,
            Self::CircleDottedLetterX => CIRCLE_DOTTED_LETTER_X_SVG,
            Self::CircleDottedLetterY => CIRCLE_DOTTED_LETTER_Y_SVG,
            Self::CircleDottedLetterZ => CIRCLE_DOTTED_LETTER_Z_SVG,
            Self::CircleLetterA => CIRCLE_LETTER_A_SVG,
            Self::CircleLetterB => CIRCLE_LETTER_B_SVG,
            Self::CircleLetterC => CIRCLE_LETTER_C_SVG,
            Self::CircleLetterD => CIRCLE_LETTER_D_SVG,
            Self::CircleLetterE => CIRCLE_LETTER_E_SVG,
            Self::CircleLetterF => CIRCLE_LETTER_F_SVG,
            Self::CircleLetterG => CIRCLE_LETTER_G_SVG,
            Self::CircleLetterH => CIRCLE_LETTER_H_SVG,
            Self::CircleLetterI => CIRCLE_LETTER_I_SVG,
            Self::CircleLetterJ => CIRCLE_LETTER_J_SVG,
            Self::CircleLetterK => CIRCLE_LETTER_K_SVG,
            Self::CircleLetterL => CIRCLE_LETTER_L_SVG,
            Self::CircleLetterM => CIRCLE_LETTER_M_SVG,
            Self::CircleLetterN => CIRCLE_LETTER_N_SVG,
            Self::CircleLetterO => CIRCLE_LETTER_O_SVG,
            Self::CircleLetterP => CIRCLE_LETTER_P_SVG,
            Self::CircleLetterQ => CIRCLE_LETTER_Q_SVG,
            Self::CircleLetterR => CIRCLE_LETTER_R_SVG,
            Self::CircleLetterS => CIRCLE_LETTER_S_SVG,
            Self::CircleLetterT => CIRCLE_LETTER_T_SVG,
            Self::CircleLetterU => CIRCLE_LETTER_U_SVG,
            Self::CircleLetterV => CIRCLE_LETTER_V_SVG,
            Self::CircleLetterW => CIRCLE_LETTER_W_SVG,
            Self::CircleLetterX => CIRCLE_LETTER_X_SVG,
            Self::CircleLetterY => CIRCLE_LETTER_Y_SVG,
            Self::CircleLetterZ => CIRCLE_LETTER_Z_SVG,
            Self::CircleNumber0 => CIRCLE_NUMBER_0_SVG,
            Self::CircleNumber1 => CIRCLE_NUMBER_1_SVG,
            Self::CircleNumber2 => CIRCLE_NUMBER_2_SVG,
            Self::CircleNumber3 => CIRCLE_NUMBER_3_SVG,
            Self::CircleNumber4 => CIRCLE_NUMBER_4_SVG,
            Self::CircleNumber5 => CIRCLE_NUMBER_5_SVG,
            Self::CircleNumber6 => CIRCLE_NUMBER_6_SVG,
            Self::CircleNumber7 => CIRCLE_NUMBER_7_SVG,
            Self::CircleNumber8 => CIRCLE_NUMBER_8_SVG,
            Self::CircleNumber9 => CIRCLE_NUMBER_9_SVG,
            Self::ClearFormatting => CLEAR_FORMATTING_SVG,
            Self::Clipboard => CLIPBOARD_SVG,
            Self::ClipboardCheck => CLIPBOARD_CHECK_SVG,
            Self::ClipboardCopy => CLIPBOARD_COPY_SVG,
            Self::ClipboardData => CLIPBOARD_DATA_SVG,
            Self::ClipboardHeart => CLIPBOARD_HEART_SVG,
            Self::ClipboardList => CLIPBOARD_LIST_SVG,
            Self::ClipboardOff => CLIPBOARD_OFF_SVG,
            Self::ClipboardPlus => CLIPBOARD_PLUS_SVG,
            Self::ClipboardSearch => CLIPBOARD_SEARCH_SVG,
            Self::ClipboardSmile => CLIPBOARD_SMILE_SVG,
            Self::ClipboardText => CLIPBOARD_TEXT_SVG,
            Self::ClipboardTypography => CLIPBOARD_TYPOGRAPHY_SVG,
            Self::ClipboardX => CLIPBOARD_X_SVG,
            Self::Code => CODE_SVG,
            Self::CodeAsterisk => CODE_ASTERISK_SVG,
            Self::CodeCircle => CODE_CIRCLE_SVG,
            Self::CodeCircle2 => CODE_CIRCLE_2_SVG,
            Self::CodeDots => CODE_DOTS_SVG,
            Self::CodeMinus => CODE_MINUS_SVG,
            Self::CodeOff => CODE_OFF_SVG,
            Self::CodePlus => CODE_PLUS_SVG,
            Self::Columns => COLUMNS_SVG,
            Self::Columns1 => COLUMNS_1_SVG,
            Self::Columns2 => COLUMNS_2_SVG,
            Self::Columns3 => COLUMNS_3_SVG,
            Self::ColumnsOff => COLUMNS_OFF_SVG,
            Self::Contract => CONTRACT_SVG,
            Self::Copy => COPY_SVG,
            Self::CopyCheck => COPY_CHECK_SVG,
            Self::CopyMinus => COPY_MINUS_SVG,
            Self::CopyOff => COPY_OFF_SVG,
            Self::CopyPlus => COPY_PLUS_SVG,
            Self::CopyX => COPY_X_SVG,
            Self::CursorText => CURSOR_TEXT_SVG,
            Self::Delta => DELTA_SVG,
            Self::Emphasis => EMPHASIS_SVG,
            Self::Eraser => ERASER_SVG,
            Self::EraserOff => ERASER_OFF_SVG,
            Self::File => FILE_SVG,
            Self::File3d => FILE_3D_SVG,
            Self::FileAi => FILE_AI_SVG,
            Self::FileAlert => FILE_ALERT_SVG,
            Self::FileAnalytics => FILE_ANALYTICS_SVG,
            Self::FileArrowLeft => FILE_ARROW_LEFT_SVG,
            Self::FileArrowRight => FILE_ARROW_RIGHT_SVG,
            Self::FileBarcode => FILE_BARCODE_SVG,
            Self::FileBitcoin => FILE_BITCOIN_SVG,
            Self::FileBroken => FILE_BROKEN_SVG,
            Self::FileCertificate => FILE_CERTIFICATE_SVG,
            Self::FileChart => FILE_CHART_SVG,
            Self::FileCheck => FILE_CHECK_SVG,
            Self::FileCode => FILE_CODE_SVG,
            Self::FileCode2 => FILE_CODE_2_SVG,
            Self::FileCv => FILE_CV_SVG,
            Self::FileDatabase => FILE_DATABASE_SVG,
            Self::FileDelta => FILE_DELTA_SVG,
            Self::FileDescription => FILE_DESCRIPTION_SVG,
            Self::FileDiff => FILE_DIFF_SVG,
            Self::FileDigit => FILE_DIGIT_SVG,
            Self::FileDislike => FILE_DISLIKE_SVG,
            Self::FileDollar => FILE_DOLLAR_SVG,
            Self::FileDots => FILE_DOTS_SVG,
            Self::FileDownload => FILE_DOWNLOAD_SVG,
            Self::FileEuro => FILE_EURO_SVG,
            Self::FileExcel => FILE_EXCEL_SVG,
            Self::FileExport => FILE_EXPORT_SVG,
            Self::FileFunction => FILE_FUNCTION_SVG,
            Self::FileHorizontal => FILE_HORIZONTAL_SVG,
            Self::FileImport => FILE_IMPORT_SVG,
            Self::FileInfinity => FILE_INFINITY_SVG,
            Self::FileInfo => FILE_INFO_SVG,
            Self::FileInvoice => FILE_INVOICE_SVG,
            Self::FileIsr => FILE_ISR_SVG,
            Self::FileLambda => FILE_LAMBDA_SVG,
            Self::FileLike => FILE_LIKE_SVG,
            Self::FileMinus => FILE_MINUS_SVG,
            Self::FileMusic => FILE_MUSIC_SVG,
            Self::FileNeutral => FILE_NEUTRAL_SVG,
            Self::FileOff => FILE_OFF_SVG,
            Self::FileOrientation => FILE_ORIENTATION_SVG,
            Self::FilePencil => FILE_PENCIL_SVG,
            Self::FilePercent => FILE_PERCENT_SVG,
            Self::FilePhone => FILE_PHONE_SVG,
            Self::FilePlus => FILE_PLUS_SVG,
            Self::FilePower => FILE_POWER_SVG,
            Self::FileReport => FILE_REPORT_SVG,
            Self::FileRss => FILE_RSS_SVG,
            Self::FileSad => FILE_SAD_SVG,
            Self::FileScissors => FILE_SCISSORS_SVG,
            Self::FileSearch => FILE_SEARCH_SVG,
            Self::FileSettings => FILE_SETTINGS_SVG,
            Self::FileShredder => FILE_SHREDDER_SVG,
            Self::FileSignal => FILE_SIGNAL_SVG,
            Self::FileSmile => FILE_SMILE_SVG,
            Self::FileSpark => FILE_SPARK_SVG,
            Self::FileSpreadsheet => FILE_SPREADSHEET_SVG,
            Self::FileStack => FILE_STACK_SVG,
            Self::FileStar => FILE_STAR_SVG,
            Self::FileSymlink => FILE_SYMLINK_SVG,
            Self::FileText => FILE_TEXT_SVG,
            Self::FileTextAi => FILE_TEXT_AI_SVG,
            Self::FileTextShield => FILE_TEXT_SHIELD_SVG,
            Self::FileTextSpark => FILE_TEXT_SPARK_SVG,
            Self::FileTime => FILE_TIME_SVG,
            Self::FileTypeBmp => FILE_TYPE_BMP_SVG,
            Self::FileTypeCss => FILE_TYPE_CSS_SVG,
            Self::FileTypeCsv => FILE_TYPE_CSV_SVG,
            Self::FileTypeDoc => FILE_TYPE_DOC_SVG,
            Self::FileTypeDocx => FILE_TYPE_DOCX_SVG,
            Self::FileTypeHtml => FILE_TYPE_HTML_SVG,
            Self::FileTypeJpg => FILE_TYPE_JPG_SVG,
            Self::FileTypeJs => FILE_TYPE_JS_SVG,
            Self::FileTypeJsx => FILE_TYPE_JSX_SVG,
            Self::FileTypePdf => FILE_TYPE_PDF_SVG,
            Self::FileTypePhp => FILE_TYPE_PHP_SVG,
            Self::FileTypePng => FILE_TYPE_PNG_SVG,
            Self::FileTypePpt => FILE_TYPE_PPT_SVG,
            Self::FileTypeRs => FILE_TYPE_RS_SVG,
            Self::FileTypeSql => FILE_TYPE_SQL_SVG,
            Self::FileTypeSvg => FILE_TYPE_SVG_SVG,
            Self::FileTypeTs => FILE_TYPE_TS_SVG,
            Self::FileTypeTsx => FILE_TYPE_TSX_SVG,
            Self::FileTypeTxt => FILE_TYPE_TXT_SVG,
            Self::FileTypeVue => FILE_TYPE_VUE_SVG,
            Self::FileTypeXls => FILE_TYPE_XLS_SVG,
            Self::FileTypeXml => FILE_TYPE_XML_SVG,
            Self::FileTypeZip => FILE_TYPE_ZIP_SVG,
            Self::FileTypography => FILE_TYPOGRAPHY_SVG,
            Self::FileUnknown => FILE_UNKNOWN_SVG,
            Self::FileUpload => FILE_UPLOAD_SVG,
            Self::FileVector => FILE_VECTOR_SVG,
            Self::FileWord => FILE_WORD_SVG,
            Self::FileX => FILE_X_SVG,
            Self::FileZip => FILE_ZIP_SVG,
            Self::Files => FILES_SVG,
            Self::FilesOff => FILES_OFF_SVG,
            Self::FloatCenter => FLOAT_CENTER_SVG,
            Self::FloatLeft => FLOAT_LEFT_SVG,
            Self::FloatNone => FLOAT_NONE_SVG,
            Self::FloatRight => FLOAT_RIGHT_SVG,
            Self::Folder => FOLDER_SVG,
            Self::FolderBolt => FOLDER_BOLT_SVG,
            Self::FolderCancel => FOLDER_CANCEL_SVG,
            Self::FolderCheck => FOLDER_CHECK_SVG,
            Self::FolderCode => FOLDER_CODE_SVG,
            Self::FolderCog => FOLDER_COG_SVG,
            Self::FolderDollar => FOLDER_DOLLAR_SVG,
            Self::FolderDown => FOLDER_DOWN_SVG,
            Self::FolderExclamation => FOLDER_EXCLAMATION_SVG,
            Self::FolderHeart => FOLDER_HEART_SVG,
            Self::FolderMinus => FOLDER_MINUS_SVG,
            Self::FolderOff => FOLDER_OFF_SVG,
            Self::FolderOpen => FOLDER_OPEN_SVG,
            Self::FolderPause => FOLDER_PAUSE_SVG,
            Self::FolderPin => FOLDER_PIN_SVG,
            Self::FolderPlus => FOLDER_PLUS_SVG,
            Self::FolderQuestion => FOLDER_QUESTION_SVG,
            Self::FolderRoot => FOLDER_ROOT_SVG,
            Self::FolderSearch => FOLDER_SEARCH_SVG,
            Self::FolderShare => FOLDER_SHARE_SVG,
            Self::FolderStar => FOLDER_STAR_SVG,
            Self::FolderSymlink => FOLDER_SYMLINK_SVG,
            Self::FolderUp => FOLDER_UP_SVG,
            Self::FolderX => FOLDER_X_SVG,
            Self::Folders => FOLDERS_SVG,
            Self::FoldersOff => FOLDERS_OFF_SVG,
            Self::Forms => FORMS_SVG,
            Self::H1 => H_1_SVG,
            Self::H2 => H_2_SVG,
            Self::H3 => H_3_SVG,
            Self::H4 => H_4_SVG,
            Self::H5 => H_5_SVG,
            Self::H6 => H_6_SVG,
            Self::Heading => HEADING_SVG,
            Self::HeadingOff => HEADING_OFF_SVG,
            Self::HexagonLetterA => HEXAGON_LETTER_A_SVG,
            Self::HexagonLetterB => HEXAGON_LETTER_B_SVG,
            Self::HexagonLetterC => HEXAGON_LETTER_C_SVG,
            Self::HexagonLetterD => HEXAGON_LETTER_D_SVG,
            Self::HexagonLetterE => HEXAGON_LETTER_E_SVG,
            Self::HexagonLetterF => HEXAGON_LETTER_F_SVG,
            Self::HexagonLetterG => HEXAGON_LETTER_G_SVG,
            Self::HexagonLetterH => HEXAGON_LETTER_H_SVG,
            Self::HexagonLetterI => HEXAGON_LETTER_I_SVG,
            Self::HexagonLetterJ => HEXAGON_LETTER_J_SVG,
            Self::HexagonLetterK => HEXAGON_LETTER_K_SVG,
            Self::HexagonLetterL => HEXAGON_LETTER_L_SVG,
            Self::HexagonLetterM => HEXAGON_LETTER_M_SVG,
            Self::HexagonLetterN => HEXAGON_LETTER_N_SVG,
            Self::HexagonLetterO => HEXAGON_LETTER_O_SVG,
            Self::HexagonLetterP => HEXAGON_LETTER_P_SVG,
            Self::HexagonLetterQ => HEXAGON_LETTER_Q_SVG,
            Self::HexagonLetterR => HEXAGON_LETTER_R_SVG,
            Self::HexagonLetterS => HEXAGON_LETTER_S_SVG,
            Self::HexagonLetterT => HEXAGON_LETTER_T_SVG,
            Self::HexagonLetterU => HEXAGON_LETTER_U_SVG,
            Self::HexagonLetterV => HEXAGON_LETTER_V_SVG,
            Self::HexagonLetterW => HEXAGON_LETTER_W_SVG,
            Self::HexagonLetterX => HEXAGON_LETTER_X_SVG,
            Self::HexagonLetterY => HEXAGON_LETTER_Y_SVG,
            Self::HexagonLetterZ => HEXAGON_LETTER_Z_SVG,
            Self::HexagonNumber0 => HEXAGON_NUMBER_0_SVG,
            Self::HexagonNumber1 => HEXAGON_NUMBER_1_SVG,
            Self::HexagonNumber2 => HEXAGON_NUMBER_2_SVG,
            Self::HexagonNumber3 => HEXAGON_NUMBER_3_SVG,
            Self::HexagonNumber4 => HEXAGON_NUMBER_4_SVG,
            Self::HexagonNumber5 => HEXAGON_NUMBER_5_SVG,
            Self::HexagonNumber6 => HEXAGON_NUMBER_6_SVG,
            Self::HexagonNumber7 => HEXAGON_NUMBER_7_SVG,
            Self::HexagonNumber8 => HEXAGON_NUMBER_8_SVG,
            Self::HexagonNumber9 => HEXAGON_NUMBER_9_SVG,
            Self::Highlight => HIGHLIGHT_SVG,
            Self::HighlightOff => HIGHLIGHT_OFF_SVG,
            Self::IndentDecrease => INDENT_DECREASE_SVG,
            Self::IndentIncrease => INDENT_INCREASE_SVG,
            Self::InputSearch => INPUT_SEARCH_SVG,
            Self::Invoice => INVOICE_SVG,
            Self::Italic => ITALIC_SVG,
            Self::Kerning => KERNING_SVG,
            Self::Lambda => LAMBDA_SVG,
            Self::Language => LANGUAGE_SVG,
            Self::LanguageHiragana => LANGUAGE_HIRAGANA_SVG,
            Self::LanguageKatakana => LANGUAGE_KATAKANA_SVG,
            Self::LanguageOff => LANGUAGE_OFF_SVG,
            Self::LetterA => LETTER_A_SVG,
            Self::LetterASmall => LETTER_A_SMALL_SVG,
            Self::LetterB => LETTER_B_SVG,
            Self::LetterBSmall => LETTER_B_SMALL_SVG,
            Self::LetterC => LETTER_C_SVG,
            Self::LetterCSmall => LETTER_C_SMALL_SVG,
            Self::LetterCase => LETTER_CASE_SVG,
            Self::LetterCaseLower => LETTER_CASE_LOWER_SVG,
            Self::LetterCaseToggle => LETTER_CASE_TOGGLE_SVG,
            Self::LetterCaseUpper => LETTER_CASE_UPPER_SVG,
            Self::LetterD => LETTER_D_SVG,
            Self::LetterDSmall => LETTER_D_SMALL_SVG,
            Self::LetterE => LETTER_E_SVG,
            Self::LetterESmall => LETTER_E_SMALL_SVG,
            Self::LetterF => LETTER_F_SVG,
            Self::LetterFSmall => LETTER_F_SMALL_SVG,
            Self::LetterG => LETTER_G_SVG,
            Self::LetterGSmall => LETTER_G_SMALL_SVG,
            Self::LetterH => LETTER_H_SVG,
            Self::LetterHSmall => LETTER_H_SMALL_SVG,
            Self::LetterI => LETTER_I_SVG,
            Self::LetterISmall => LETTER_I_SMALL_SVG,
            Self::LetterJ => LETTER_J_SVG,
            Self::LetterJSmall => LETTER_J_SMALL_SVG,
            Self::LetterK => LETTER_K_SVG,
            Self::LetterKSmall => LETTER_K_SMALL_SVG,
            Self::LetterL => LETTER_L_SVG,
            Self::LetterLSmall => LETTER_L_SMALL_SVG,
            Self::LetterM => LETTER_M_SVG,
            Self::LetterMSmall => LETTER_M_SMALL_SVG,
            Self::LetterN => LETTER_N_SVG,
            Self::LetterNSmall => LETTER_N_SMALL_SVG,
            Self::LetterO => LETTER_O_SVG,
            Self::LetterOSmall => LETTER_O_SMALL_SVG,
            Self::LetterP => LETTER_P_SVG,
            Self::LetterPSmall => LETTER_P_SMALL_SVG,
            Self::LetterQ => LETTER_Q_SVG,
            Self::LetterQSmall => LETTER_Q_SMALL_SVG,
            Self::LetterR => LETTER_R_SVG,
            Self::LetterRSmall => LETTER_R_SMALL_SVG,
            Self::LetterS => LETTER_S_SVG,
            Self::LetterSSmall => LETTER_S_SMALL_SVG,
            Self::LetterSpacing => LETTER_SPACING_SVG,
            Self::LetterT => LETTER_T_SVG,
            Self::LetterTSmall => LETTER_T_SMALL_SVG,
            Self::LetterU => LETTER_U_SVG,
            Self::LetterUSmall => LETTER_U_SMALL_SVG,
            Self::LetterV => LETTER_V_SVG,
            Self::LetterVSmall => LETTER_V_SMALL_SVG,
            Self::LetterW => LETTER_W_SVG,
            Self::LetterWSmall => LETTER_W_SMALL_SVG,
            Self::LetterX => LETTER_X_SVG,
            Self::LetterXSmall => LETTER_X_SMALL_SVG,
            Self::LetterY => LETTER_Y_SVG,
            Self::LetterYSmall => LETTER_Y_SMALL_SVG,
            Self::LetterZ => LETTER_Z_SVG,
            Self::LetterZSmall => LETTER_Z_SMALL_SVG,
            Self::License => LICENSE_SVG,
            Self::LicenseOff => LICENSE_OFF_SVG,
            Self::LineHeight => LINE_HEIGHT_SVG,
            Self::Link => LINK_SVG,
            Self::LinkMinus => LINK_MINUS_SVG,
            Self::LinkOff => LINK_OFF_SVG,
            Self::LinkPlus => LINK_PLUS_SVG,
            Self::List => LIST_SVG,
            Self::ListCheck => LIST_CHECK_SVG,
            Self::ListDetails => LIST_DETAILS_SVG,
            Self::ListNumbers => LIST_NUMBERS_SVG,
            Self::ListSearch => LIST_SEARCH_SVG,
            Self::ManualGearbox => MANUAL_GEARBOX_SVG,
            Self::Markdown => MARKDOWN_SVG,
            Self::MarkdownOff => MARKDOWN_OFF_SVG,
            Self::News => NEWS_SVG,
            Self::NewsOff => NEWS_OFF_SVG,
            Self::Notdef => NOTDEF_SVG,
            Self::Note => NOTE_SVG,
            Self::NoteOff => NOTE_OFF_SVG,
            Self::Notebook => NOTEBOOK_SVG,
            Self::NotebookOff => NOTEBOOK_OFF_SVG,
            Self::Notes => NOTES_SVG,
            Self::NotesOff => NOTES_OFF_SVG,
            Self::Number => NUMBER_SVG,
            Self::Number0 => NUMBER_0_SVG,
            Self::Number0Small => NUMBER_0_SMALL_SVG,
            Self::Number1 => NUMBER_1_SVG,
            Self::Number1Small => NUMBER_1_SMALL_SVG,
            Self::Number10 => NUMBER_10_SVG,
            Self::Number10Small => NUMBER_10_SMALL_SVG,
            Self::Number100Small => NUMBER_100_SMALL_SVG,
            Self::Number11 => NUMBER_11_SVG,
            Self::Number11Small => NUMBER_11_SMALL_SVG,
            Self::Number12Small => NUMBER_12_SMALL_SVG,
            Self::Number123 => NUMBER_123_SVG,
            Self::Number13Small => NUMBER_13_SMALL_SVG,
            Self::Number14Small => NUMBER_14_SMALL_SVG,
            Self::Number15Small => NUMBER_15_SMALL_SVG,
            Self::Number16Small => NUMBER_16_SMALL_SVG,
            Self::Number17Small => NUMBER_17_SMALL_SVG,
            Self::Number18Small => NUMBER_18_SMALL_SVG,
            Self::Number19Small => NUMBER_19_SMALL_SVG,
            Self::Number2 => NUMBER_2_SVG,
            Self::Number2Small => NUMBER_2_SMALL_SVG,
            Self::Number20Small => NUMBER_20_SMALL_SVG,
            Self::Number21Small => NUMBER_21_SMALL_SVG,
            Self::Number22Small => NUMBER_22_SMALL_SVG,
            Self::Number23Small => NUMBER_23_SMALL_SVG,
            Self::Number24Small => NUMBER_24_SMALL_SVG,
            Self::Number25Small => NUMBER_25_SMALL_SVG,
            Self::Number26Small => NUMBER_26_SMALL_SVG,
            Self::Number27Small => NUMBER_27_SMALL_SVG,
            Self::Number28Small => NUMBER_28_SMALL_SVG,
            Self::Number29Small => NUMBER_29_SMALL_SVG,
            Self::Number3 => NUMBER_3_SVG,
            Self::Number3Small => NUMBER_3_SMALL_SVG,
            Self::Number30Small => NUMBER_30_SMALL_SVG,
            Self::Number31Small => NUMBER_31_SMALL_SVG,
            Self::Number32Small => NUMBER_32_SMALL_SVG,
            Self::Number33Small => NUMBER_33_SMALL_SVG,
            Self::Number34Small => NUMBER_34_SMALL_SVG,
            Self::Number35Small => NUMBER_35_SMALL_SVG,
            Self::Number36Small => NUMBER_36_SMALL_SVG,
            Self::Number37Small => NUMBER_37_SMALL_SVG,
            Self::Number38Small => NUMBER_38_SMALL_SVG,
            Self::Number39Small => NUMBER_39_SMALL_SVG,
            Self::Number4 => NUMBER_4_SVG,
            Self::Number4Small => NUMBER_4_SMALL_SVG,
            Self::Number40Small => NUMBER_40_SMALL_SVG,
            Self::Number41Small => NUMBER_41_SMALL_SVG,
            Self::Number42Small => NUMBER_42_SMALL_SVG,
            Self::Number43Small => NUMBER_43_SMALL_SVG,
            Self::Number44Small => NUMBER_44_SMALL_SVG,
            Self::Number45Small => NUMBER_45_SMALL_SVG,
            Self::Number46Small => NUMBER_46_SMALL_SVG,
            Self::Number47Small => NUMBER_47_SMALL_SVG,
            Self::Number48Small => NUMBER_48_SMALL_SVG,
            Self::Number49Small => NUMBER_49_SMALL_SVG,
            Self::Number5 => NUMBER_5_SVG,
            Self::Number5Small => NUMBER_5_SMALL_SVG,
            Self::Number50Small => NUMBER_50_SMALL_SVG,
            Self::Number51Small => NUMBER_51_SMALL_SVG,
            Self::Number52Small => NUMBER_52_SMALL_SVG,
            Self::Number53Small => NUMBER_53_SMALL_SVG,
            Self::Number54Small => NUMBER_54_SMALL_SVG,
            Self::Number55Small => NUMBER_55_SMALL_SVG,
            Self::Number56Small => NUMBER_56_SMALL_SVG,
            Self::Number57Small => NUMBER_57_SMALL_SVG,
            Self::Number58Small => NUMBER_58_SMALL_SVG,
            Self::Number59Small => NUMBER_59_SMALL_SVG,
            Self::Number6 => NUMBER_6_SVG,
            Self::Number6Small => NUMBER_6_SMALL_SVG,
            Self::Number60Small => NUMBER_60_SMALL_SVG,
            Self::Number61Small => NUMBER_61_SMALL_SVG,
            Self::Number62Small => NUMBER_62_SMALL_SVG,
            Self::Number63Small => NUMBER_63_SMALL_SVG,
            Self::Number64Small => NUMBER_64_SMALL_SVG,
            Self::Number65Small => NUMBER_65_SMALL_SVG,
            Self::Number66Small => NUMBER_66_SMALL_SVG,
            Self::Number67Small => NUMBER_67_SMALL_SVG,
            Self::Number68Small => NUMBER_68_SMALL_SVG,
            Self::Number69Small => NUMBER_69_SMALL_SVG,
            Self::Number7 => NUMBER_7_SVG,
            Self::Number7Small => NUMBER_7_SMALL_SVG,
            Self::Number70Small => NUMBER_70_SMALL_SVG,
            Self::Number71Small => NUMBER_71_SMALL_SVG,
            Self::Number72Small => NUMBER_72_SMALL_SVG,
            Self::Number73Small => NUMBER_73_SMALL_SVG,
            Self::Number74Small => NUMBER_74_SMALL_SVG,
            Self::Number75Small => NUMBER_75_SMALL_SVG,
            Self::Number76Small => NUMBER_76_SMALL_SVG,
            Self::Number77Small => NUMBER_77_SMALL_SVG,
            Self::Number78Small => NUMBER_78_SMALL_SVG,
            Self::Number79Small => NUMBER_79_SMALL_SVG,
            Self::Number8 => NUMBER_8_SVG,
            Self::Number8Small => NUMBER_8_SMALL_SVG,
            Self::Number80Small => NUMBER_80_SMALL_SVG,
            Self::Number81Small => NUMBER_81_SMALL_SVG,
            Self::Number82Small => NUMBER_82_SMALL_SVG,
            Self::Number83Small => NUMBER_83_SMALL_SVG,
            Self::Number84Small => NUMBER_84_SMALL_SVG,
            Self::Number85Small => NUMBER_85_SMALL_SVG,
            Self::Number86Small => NUMBER_86_SMALL_SVG,
            Self::Number87Small => NUMBER_87_SMALL_SVG,
            Self::Number88Small => NUMBER_88_SMALL_SVG,
            Self::Number89Small => NUMBER_89_SMALL_SVG,
            Self::Number9 => NUMBER_9_SVG,
            Self::Number9Small => NUMBER_9_SMALL_SVG,
            Self::Number90Small => NUMBER_90_SMALL_SVG,
            Self::Number91Small => NUMBER_91_SMALL_SVG,
            Self::Number92Small => NUMBER_92_SMALL_SVG,
            Self::Number93Small => NUMBER_93_SMALL_SVG,
            Self::Number94Small => NUMBER_94_SMALL_SVG,
            Self::Number95Small => NUMBER_95_SMALL_SVG,
            Self::Number96Small => NUMBER_96_SMALL_SVG,
            Self::Number97Small => NUMBER_97_SMALL_SVG,
            Self::Number98Small => NUMBER_98_SMALL_SVG,
            Self::Number99Small => NUMBER_99_SMALL_SVG,
            Self::Numbers => NUMBERS_SVG,
            Self::Overline => OVERLINE_SVG,
            Self::PageBreak => PAGE_BREAK_SVG,
            Self::Paperclip => PAPERCLIP_SVG,
            Self::PentagonNumber0 => PENTAGON_NUMBER_0_SVG,
            Self::PentagonNumber1 => PENTAGON_NUMBER_1_SVG,
            Self::PentagonNumber2 => PENTAGON_NUMBER_2_SVG,
            Self::PentagonNumber3 => PENTAGON_NUMBER_3_SVG,
            Self::PentagonNumber4 => PENTAGON_NUMBER_4_SVG,
            Self::PentagonNumber5 => PENTAGON_NUMBER_5_SVG,
            Self::PentagonNumber6 => PENTAGON_NUMBER_6_SVG,
            Self::PentagonNumber7 => PENTAGON_NUMBER_7_SVG,
            Self::PentagonNumber8 => PENTAGON_NUMBER_8_SVG,
            Self::PentagonNumber9 => PENTAGON_NUMBER_9_SVG,
            Self::Pilcrow => PILCROW_SVG,
            Self::PilcrowLeft => PILCROW_LEFT_SVG,
            Self::PilcrowRight => PILCROW_RIGHT_SVG,
            Self::Presentation => PRESENTATION_SVG,
            Self::PresentationAnalytics => PRESENTATION_ANALYTICS_SVG,
            Self::PresentationOff => PRESENTATION_OFF_SVG,
            Self::Quote => QUOTE_SVG,
            Self::QuoteOff => QUOTE_OFF_SVG,
            Self::QuoteOpen => QUOTE_OPEN_SVG,
            Self::Quotes => QUOTES_SVG,
            Self::Receipt => RECEIPT_SVG,
            Self::Receipt2 => RECEIPT_2_SVG,
            Self::ReceiptDollar => RECEIPT_DOLLAR_SVG,
            Self::ReceiptOff => RECEIPT_OFF_SVG,
            Self::ReceiptRefund => RECEIPT_REFUND_SVG,
            Self::ReceiptTax => RECEIPT_TAX_SVG,
            Self::Regex => REGEX_SVG,
            Self::RegexOff => REGEX_OFF_SVG,
            Self::Report => REPORT_SVG,
            Self::ReportAnalytics => REPORT_ANALYTICS_SVG,
            Self::ReportMedical => REPORT_MEDICAL_SVG,
            Self::ReportMoney => REPORT_MONEY_SVG,
            Self::ReportOff => REPORT_OFF_SVG,
            Self::ReportSearch => REPORT_SEARCH_SVG,
            Self::RosetteNumber0 => ROSETTE_NUMBER_0_SVG,
            Self::RosetteNumber1 => ROSETTE_NUMBER_1_SVG,
            Self::RosetteNumber2 => ROSETTE_NUMBER_2_SVG,
            Self::RosetteNumber3 => ROSETTE_NUMBER_3_SVG,
            Self::RosetteNumber4 => ROSETTE_NUMBER_4_SVG,
            Self::RosetteNumber5 => ROSETTE_NUMBER_5_SVG,
            Self::RosetteNumber6 => ROSETTE_NUMBER_6_SVG,
            Self::RosetteNumber7 => ROSETTE_NUMBER_7_SVG,
            Self::RosetteNumber8 => ROSETTE_NUMBER_8_SVG,
            Self::RosetteNumber9 => ROSETTE_NUMBER_9_SVG,
            Self::RubberStamp => RUBBER_STAMP_SVG,
            Self::RubberStampOff => RUBBER_STAMP_OFF_SVG,
            Self::ScanLetterA => SCAN_LETTER_A_SVG,
            Self::ScanLetterT => SCAN_LETTER_T_SVG,
            Self::Scribble => SCRIBBLE_SVG,
            Self::ScribbleOff => SCRIBBLE_OFF_SVG,
            Self::Script => SCRIPT_SVG,
            Self::ScriptMinus => SCRIPT_MINUS_SVG,
            Self::ScriptPlus => SCRIPT_PLUS_SVG,
            Self::ScriptX => SCRIPT_X_SVG,
            Self::SectionSign => SECTION_SIGN_SVG,
            Self::Separator => SEPARATOR_SVG,
            Self::SeparatorHorizontal => SEPARATOR_HORIZONTAL_SVG,
            Self::SeparatorVertical => SEPARATOR_VERTICAL_SVG,
            Self::Signature => SIGNATURE_SVG,
            Self::SignatureOff => SIGNATURE_OFF_SVG,
            Self::Slideshow => SLIDESHOW_SVG,
            Self::Sort09 => SORT_0_9_SVG,
            Self::Sort90 => SORT_9_0_SVG,
            Self::SortAZ => SORT_A_Z_SVG,
            Self::SortAscending => SORT_ASCENDING_SVG,
            Self::SortAscending2 => SORT_ASCENDING_2_SVG,
            Self::SortAscendingLetters => SORT_ASCENDING_LETTERS_SVG,
            Self::SortAscendingNumbers => SORT_ASCENDING_NUMBERS_SVG,
            Self::SortDescending => SORT_DESCENDING_SVG,
            Self::SortDescending2 => SORT_DESCENDING_2_SVG,
            Self::SortDescendingLetters => SORT_DESCENDING_LETTERS_SVG,
            Self::SortDescendingNumbers => SORT_DESCENDING_NUMBERS_SVG,
            Self::SortZA => SORT_Z_A_SVG,
            Self::Space => SPACE_SVG,
            Self::SpaceOff => SPACE_OFF_SVG,
            Self::SpacingHorizontal => SPACING_HORIZONTAL_SVG,
            Self::SpacingVertical => SPACING_VERTICAL_SVG,
            Self::SquareLetterA => SQUARE_LETTER_A_SVG,
            Self::SquareLetterB => SQUARE_LETTER_B_SVG,
            Self::SquareLetterC => SQUARE_LETTER_C_SVG,
            Self::SquareLetterD => SQUARE_LETTER_D_SVG,
            Self::SquareLetterE => SQUARE_LETTER_E_SVG,
            Self::SquareLetterF => SQUARE_LETTER_F_SVG,
            Self::SquareLetterG => SQUARE_LETTER_G_SVG,
            Self::SquareLetterH => SQUARE_LETTER_H_SVG,
            Self::SquareLetterI => SQUARE_LETTER_I_SVG,
            Self::SquareLetterJ => SQUARE_LETTER_J_SVG,
            Self::SquareLetterK => SQUARE_LETTER_K_SVG,
            Self::SquareLetterL => SQUARE_LETTER_L_SVG,
            Self::SquareLetterM => SQUARE_LETTER_M_SVG,
            Self::SquareLetterN => SQUARE_LETTER_N_SVG,
            Self::SquareLetterO => SQUARE_LETTER_O_SVG,
            Self::SquareLetterP => SQUARE_LETTER_P_SVG,
            Self::SquareLetterQ => SQUARE_LETTER_Q_SVG,
            Self::SquareLetterR => SQUARE_LETTER_R_SVG,
            Self::SquareLetterS => SQUARE_LETTER_S_SVG,
            Self::SquareLetterT => SQUARE_LETTER_T_SVG,
            Self::SquareLetterU => SQUARE_LETTER_U_SVG,
            Self::SquareLetterV => SQUARE_LETTER_V_SVG,
            Self::SquareLetterW => SQUARE_LETTER_W_SVG,
            Self::SquareLetterX => SQUARE_LETTER_X_SVG,
            Self::SquareLetterY => SQUARE_LETTER_Y_SVG,
            Self::SquareLetterZ => SQUARE_LETTER_Z_SVG,
            Self::SquareNumber0 => SQUARE_NUMBER_0_SVG,
            Self::SquareNumber1 => SQUARE_NUMBER_1_SVG,
            Self::SquareNumber2 => SQUARE_NUMBER_2_SVG,
            Self::SquareNumber3 => SQUARE_NUMBER_3_SVG,
            Self::SquareNumber4 => SQUARE_NUMBER_4_SVG,
            Self::SquareNumber5 => SQUARE_NUMBER_5_SVG,
            Self::SquareNumber6 => SQUARE_NUMBER_6_SVG,
            Self::SquareNumber7 => SQUARE_NUMBER_7_SVG,
            Self::SquareNumber8 => SQUARE_NUMBER_8_SVG,
            Self::SquareNumber9 => SQUARE_NUMBER_9_SVG,
            Self::SquareRoundedLetterA => SQUARE_ROUNDED_LETTER_A_SVG,
            Self::SquareRoundedLetterB => SQUARE_ROUNDED_LETTER_B_SVG,
            Self::SquareRoundedLetterC => SQUARE_ROUNDED_LETTER_C_SVG,
            Self::SquareRoundedLetterD => SQUARE_ROUNDED_LETTER_D_SVG,
            Self::SquareRoundedLetterE => SQUARE_ROUNDED_LETTER_E_SVG,
            Self::SquareRoundedLetterF => SQUARE_ROUNDED_LETTER_F_SVG,
            Self::SquareRoundedLetterG => SQUARE_ROUNDED_LETTER_G_SVG,
            Self::SquareRoundedLetterH => SQUARE_ROUNDED_LETTER_H_SVG,
            Self::SquareRoundedLetterI => SQUARE_ROUNDED_LETTER_I_SVG,
            Self::SquareRoundedLetterJ => SQUARE_ROUNDED_LETTER_J_SVG,
            Self::SquareRoundedLetterK => SQUARE_ROUNDED_LETTER_K_SVG,
            Self::SquareRoundedLetterL => SQUARE_ROUNDED_LETTER_L_SVG,
            Self::SquareRoundedLetterM => SQUARE_ROUNDED_LETTER_M_SVG,
            Self::SquareRoundedLetterN => SQUARE_ROUNDED_LETTER_N_SVG,
            Self::SquareRoundedLetterO => SQUARE_ROUNDED_LETTER_O_SVG,
            Self::SquareRoundedLetterP => SQUARE_ROUNDED_LETTER_P_SVG,
            Self::SquareRoundedLetterQ => SQUARE_ROUNDED_LETTER_Q_SVG,
            Self::SquareRoundedLetterR => SQUARE_ROUNDED_LETTER_R_SVG,
            Self::SquareRoundedLetterS => SQUARE_ROUNDED_LETTER_S_SVG,
            Self::SquareRoundedLetterT => SQUARE_ROUNDED_LETTER_T_SVG,
            Self::SquareRoundedLetterU => SQUARE_ROUNDED_LETTER_U_SVG,
            Self::SquareRoundedLetterV => SQUARE_ROUNDED_LETTER_V_SVG,
            Self::SquareRoundedLetterW => SQUARE_ROUNDED_LETTER_W_SVG,
            Self::SquareRoundedLetterX => SQUARE_ROUNDED_LETTER_X_SVG,
            Self::SquareRoundedLetterY => SQUARE_ROUNDED_LETTER_Y_SVG,
            Self::SquareRoundedLetterZ => SQUARE_ROUNDED_LETTER_Z_SVG,
            Self::SquareRoundedNumber0 => SQUARE_ROUNDED_NUMBER_0_SVG,
            Self::SquareRoundedNumber1 => SQUARE_ROUNDED_NUMBER_1_SVG,
            Self::SquareRoundedNumber2 => SQUARE_ROUNDED_NUMBER_2_SVG,
            Self::SquareRoundedNumber3 => SQUARE_ROUNDED_NUMBER_3_SVG,
            Self::SquareRoundedNumber4 => SQUARE_ROUNDED_NUMBER_4_SVG,
            Self::SquareRoundedNumber5 => SQUARE_ROUNDED_NUMBER_5_SVG,
            Self::SquareRoundedNumber6 => SQUARE_ROUNDED_NUMBER_6_SVG,
            Self::SquareRoundedNumber7 => SQUARE_ROUNDED_NUMBER_7_SVG,
            Self::SquareRoundedNumber8 => SQUARE_ROUNDED_NUMBER_8_SVG,
            Self::SquareRoundedNumber9 => SQUARE_ROUNDED_NUMBER_9_SVG,
            Self::Strikethrough => STRIKETHROUGH_SVG,
            Self::Subscript => SUBSCRIPT_SVG,
            Self::Superscript => SUPERSCRIPT_SVG,
            Self::Tex => TEX_SVG,
            Self::TextCaption => TEXT_CAPTION_SVG,
            Self::TextColor => TEXT_COLOR_SVG,
            Self::TextDecrease => TEXT_DECREASE_SVG,
            Self::TextDirectionLtr => TEXT_DIRECTION_LTR_SVG,
            Self::TextDirectionRtl => TEXT_DIRECTION_RTL_SVG,
            Self::TextGrammar => TEXT_GRAMMAR_SVG,
            Self::TextIncrease => TEXT_INCREASE_SVG,
            Self::TextOrientation => TEXT_ORIENTATION_SVG,
            Self::TextPlus => TEXT_PLUS_SVG,
            Self::TextRecognition => TEXT_RECOGNITION_SVG,
            Self::TextSize => TEXT_SIZE_SVG,
            Self::TextSpellcheck => TEXT_SPELLCHECK_SVG,
            Self::TextWrap => TEXT_WRAP_SVG,
            Self::TextWrapColumn => TEXT_WRAP_COLUMN_SVG,
            Self::TextWrapDisabled => TEXT_WRAP_DISABLED_SVG,
            Self::Ticket => TICKET_SVG,
            Self::TicketOff => TICKET_OFF_SVG,
            Self::Typography => TYPOGRAPHY_SVG,
            Self::TypographyOff => TYPOGRAPHY_OFF_SVG,
            Self::Underline => UNDERLINE_SVG,
            Self::Unlink => UNLINK_SVG,
            Self::Vocabulary => VOCABULARY_SVG,
            Self::VocabularyOff => VOCABULARY_OFF_SVG,
            Self::Writing => WRITING_SVG,
            Self::WritingOff => WRITING_OFF_SVG,
            Self::WritingSign => WRITING_SIGN_SVG,
            Self::WritingSignOff => WRITING_SIGN_OFF_SVG,
        }
    }

    fn filled_svg(&self) -> Option<&'static str> {
        // Filled variants would be added here
        None
    }
}
