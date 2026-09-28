//! Communication icons from Tabler Icons.
//!
//! This module contains 117 icons.

use crate::tabler::TablerIconData;

// SVG Constants
const ADDRESS_BOOK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 6v12a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M10 16h6" /> <path d="M11 11a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 8h3" /> <path d="M4 12h3" /> <path d="M4 16h3" /> </svg>"##;
const ADDRESS_BOOK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h10a2 2 0 0 1 2 2v10m-.57 3.399c-.363 .37 -.87 .601 -1.43 .601h-10a2 2 0 0 1 -2 -2v-12" /> <path d="M10 16h6" /> <path d="M11 11a2 2 0 0 0 2 2m2 -2a2 2 0 0 0 -2 -2" /> <path d="M4 8h3" /> <path d="M4 12h3" /> <path d="M4 16h3" /> <path d="M3 3l18 18" /> </svg>"##;
const BUBBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.4 3a5.34 5.34 0 0 1 4.906 3.239a5.333 5.333 0 0 1 -1.195 10.6a4.26 4.26 0 0 1 -5.28 1.863l-3.831 2.298v-3.134a2.668 2.668 0 0 1 -1.795 -3.773a4.8 4.8 0 0 1 2.908 -8.933a5.33 5.33 0 0 1 4.287 -2.16" /> </svg>"##;
const BUBBLE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.4 19a4.2 4.2 0 0 1 -1.57 -.298l-3.83 2.298v-3.134a2.668 2.668 0 0 1 -1.795 -3.773a4.8 4.8 0 0 1 2.908 -8.933a5.335 5.335 0 0 1 9.194 1.078a5.333 5.333 0 0 1 3.404 8.771" /> <path d="M16 19h6" /> </svg>"##;
const BUBBLE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.4 19a4.2 4.2 0 0 1 -1.57 -.298l-3.83 2.298v-3.134a2.668 2.668 0 0 1 -1.795 -3.773a4.8 4.8 0 0 1 2.908 -8.933a5.335 5.335 0 0 1 9.194 1.078a5.333 5.333 0 0 1 4.45 6.89" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const BUBBLE_TEXT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 10h10" /> <path d="M9 14h5" /> <path d="M12.4 3a5.34 5.34 0 0 1 4.906 3.239a5.333 5.333 0 0 1 -1.195 10.6a4.26 4.26 0 0 1 -5.28 1.863l-3.831 2.298v-3.134a2.668 2.668 0 0 1 -1.795 -3.773a4.8 4.8 0 0 1 2.908 -8.933a5.33 5.33 0 0 1 4.287 -2.16" /> </svg>"##;
const BUBBLE_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 18.75c-.345 .09 -.727 .25 -1.1 .25a4.3 4.3 0 0 1 -1.57 -.298l-3.83 2.298v-3.134a2.668 2.668 0 0 1 -1.795 -3.773a4.8 4.8 0 0 1 2.908 -8.933a5.335 5.335 0 0 1 9.194 1.078a5.333 5.333 0 0 1 4.484 6.778" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const EMAIL_STAMP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.586 4.586a2 2 0 0 0 -1.414 -.586h-.172a2 2 0 0 0 -2 2v.172a2 2 0 0 0 .586 1.414a2 2 0 0 1 0 2.828a2 2 0 0 0 -.586 1.414v.344a2 2 0 0 0 .586 1.414c.4 .4 .595 .928 .585 1.452c-.01 .5 -.204 .995 -.585 1.376a2 2 0 0 0 -.586 1.414v.172a2 2 0 0 0 2 2h.172a2 2 0 0 0 1.414 -.586a2 2 0 0 1 2.828 0a2 2 0 0 0 1.414 .586h.344a2 2 0 0 0 1.414 -.586a2 2 0 0 1 2.828 0a2 2 0 0 0 1.414 .586h.172a2 2 0 0 0 2 -2v-.172a2 2 0 0 0 -.586 -1.414a1.996 1.996 0 0 1 0 -2.828a2 2 0 0 0 .586 -1.414v-.344a2 2 0 0 0 -.586 -1.414a2 2 0 0 1 0 -2.828a2 2 0 0 0 .586 -1.414v-.172a2 2 0 0 0 -2 -2h-.172a2 2 0 0 0 -1.414 .586a2 2 0 0 1 -2.828 0a2 2 0 0 0 -1.414 -.586h-.344a2 2 0 0 0 -1.414 .586a2 2 0 0 1 -2.828 0" /> <path d="M10 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M9 15c0 -1.105 .672 -2 1.5 -2h3c.828 0 1.5 .895 1.5 2" /> </svg>"##;
const MAIL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M3 7l9 6l9 -6" /> </svg>"##;
const MAIL_AI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 19h-5a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4" /> <path d="M3 7l8 5.345m4 -1.345l6 -4" /> <path d="M14 21v-4a2 2 0 1 1 4 0v4" /> <path d="M14 19h4" /> <path d="M21 15v6" /> </svg>"##;
const MAIL_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 21v-6m2 0v-1.5m0 9v-1.5m-2 -3h3m-1 0h.5a1.5 1.5 0 0 1 0 3h-3.5m3 -3h.5a1.5 1.5 0 0 0 0 -3h-3.5" /> <path d="M13.5 19h-8.5a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4" /> <path d="M3 7l9 6l9 -6" /> </svg>"##;
const MAIL_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19h-8a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v5.5" /> <path d="M3 7l9 6l9 -6" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const MAIL_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-7a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v5" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> <path d="M3 7l9 6l9 -6" /> </svg>"##;
const MAIL_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 19h-6a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v6" /> <path d="M3 7l9 6l9 -6" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const MAIL_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 19h-6a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v6" /> <path d="M3 7l9 6l9 -6" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const MAIL_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-7a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v5" /> <path d="M3 7l9 6l9 -6" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const MAIL_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 19h-8.5a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v3.5" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> <path d="M3 7l9 6l9 -6" /> </svg>"##;
const MAIL_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-7a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v5.5" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> <path d="M3 7l9 6l9 -6" /> </svg>"##;
const MAIL_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 19h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v5.5" /> <path d="M3 7l9 6l9 -6" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const MAIL_FAST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7h3" /> <path d="M3 11h2" /> <path d="M9.02 8.801l-.6 6a2 2 0 0 0 1.99 2.199h7.98a2 2 0 0 0 1.99 -1.801l.6 -6a2 2 0 0 0 -1.99 -2.199h-7.98a2 2 0 0 0 -1.99 1.801" /> <path d="M9.8 7.5l2.982 3.28a3 3 0 0 0 4.238 .202l3.28 -2.982" /> </svg>"##;
const MAIL_FORWARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18h-7a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v7.5" /> <path d="M3 6l9 6l9 -6" /> <path d="M15 18h6" /> <path d="M18 15l3 3l-3 3" /> </svg>"##;
const MAIL_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.5 19h-5.5a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4" /> <path d="M3 7l9 6l2.983 -1.989l6.017 -4.011" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const MAIL_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-7a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v8" /> <path d="M16 19h6" /> <path d="M3 7l9 6l9 -6" /> </svg>"##;
const MAIL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h10a2 2 0 0 1 2 2v10m-2 2h-14a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2" /> <path d="M3 7l9 6l.565 -.377m2.435 -1.623l6 -4" /> <path d="M3 3l18 18" /> </svg>"##;
const MAIL_OPENED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9l9 6l9 -6l-9 -6l-9 6" /> <path d="M21 9v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M3 19l6 -6" /> <path d="M15 13l6 6" /> </svg>"##;
const MAIL_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19h-8a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v6" /> <path d="M3 7l9 6l9 -6" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const MAIL_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-7a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4.5" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> <path d="M3 7l9 6l9 -6" /> </svg>"##;
const MAIL_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-7a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v5.5" /> <path d="M16 19h6" /> <path d="M19 16v6" /> <path d="M3 7l9 6l9 -6" /> </svg>"##;
const MAIL_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 19h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4.5" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> <path d="M3 7l9 6l9 -6" /> </svg>"##;
const MAIL_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 19h-6a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4.5" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> <path d="M3 7l9 6l9 -6" /> </svg>"##;
const MAIL_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19h-8a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v6" /> <path d="M3 7l9 6l9 -6" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const MAIL_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> <path d="M11.5 19h-6.5a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v5" /> <path d="M3 7l9 6l9 -6" /> </svg>"##;
const MAIL_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 19h-5a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4.5" /> <path d="M3 7l9 6l9 -6" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const MAIL_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-7a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v5.5" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> <path d="M3 7l9 6l9 -6" /> </svg>"##;
const MAIL_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 19h-8.5a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v6" /> <path d="M3 7l9 6l9 -6" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const MAILBOX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 21v-6.5a3.5 3.5 0 0 0 -7 0v6.5h18v-6a4 4 0 0 0 -4 -4h-10.5" /> <path d="M12 11v-8h4l2 2l-2 2h-4" /> <path d="M6 15h1" /> </svg>"##;
const MAILBOX_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 21v-6.5a3.5 3.5 0 0 0 -7 0v6.5h18m0 -4v-2a4 4 0 0 0 -4 -4h-2m-4 0h-4.5" /> <path d="M12 8v-5h4l2 2l-2 2h-4" /> <path d="M6 15h1" /> <path d="M3 3l18 18" /> </svg>"##;
const MESSAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M18 4a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-5l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12" /> </svg>"##;
const MESSAGE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M9 18h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-3l-3 3l-3 -3" /> </svg>"##;
const MESSAGE_2_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M13 20l-1 1l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const MESSAGE_2_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M12 21l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const MESSAGE_2_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M12 21l-1 -1l-2 -2h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const MESSAGE_2_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M12 21l-1 -1l-2 -2h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const MESSAGE_2_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M12 21l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const MESSAGE_2_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M13.5 19.5l-1.5 1.5l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v3.5" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const MESSAGE_2_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M12.5 20.5l-.5 .5l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const MESSAGE_2_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M15 18l-3 3l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const MESSAGE_2_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h3.5" /> <path d="M10.5 19.5l-1.5 -1.5h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const MESSAGE_2_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M12 21l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v8" /> <path d="M16 19h6" /> </svg>"##;
const MESSAGE_2_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h1m4 0h3" /> <path d="M8 13h5" /> <path d="M8 4h10a3 3 0 0 1 3 3v8c0 .57 -.16 1.104 -.436 1.558m-2.564 1.442h-3l-3 3l-3 -3h-3a3 3 0 0 1 -3 -3v-8c0 -1.084 .575 -2.034 1.437 -2.561" /> <path d="M3 3l18 18" /> </svg>"##;
const MESSAGE_2_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M13 20l-1 1l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const MESSAGE_2_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M12.5 20.5l-.5 .5l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const MESSAGE_2_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M12.5 20.5l-.5 .5l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const MESSAGE_2_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M14.5 18.5l-2.5 2.5l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4.5" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const MESSAGE_2_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h5" /> <path d="M12 21l-.5 -.5l-2.5 -2.5h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4.5" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const MESSAGE_2_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M12 21l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const MESSAGE_2_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h4.5" /> <path d="M10 19l-1 -1h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4.5" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const MESSAGE_2_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M12.354 20.646l-.354 .354l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const MESSAGE_2_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M13.5 19.5l-1.5 1.5l-3 -3h-3a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const MESSAGE_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M13 18l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const MESSAGE_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M11.995 18.603l-3.995 2.397v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const MESSAGE_CHATBOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 4a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-5l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12" /> <path d="M9.5 9h.01" /> <path d="M14.5 9h.01" /> <path d="M9.5 13a3.5 3.5 0 0 0 5 0" /> </svg>"##;
const MESSAGE_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M10.99 19.206l-2.99 1.794v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const MESSAGE_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 20l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c3.255 2.777 3.695 7.266 1.029 10.501c-2.666 3.235 -7.615 4.215 -11.574 2.293l-4.7 1" /> </svg>"##;
const MESSAGE_CIRCLE_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.038 19.927a9.933 9.933 0 0 1 -5.338 -.927l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.993 1.7 2.93 4.043 2.746 6.346" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const MESSAGE_CIRCLE_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.015 19.98a9.87 9.87 0 0 1 -4.315 -.98l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.927 1.644 2.867 3.887 2.761 6.114" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const MESSAGE_CIRCLE_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.042 19.933a9.798 9.798 0 0 1 -3.342 -.933l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c2.127 1.814 3.052 4.36 2.694 6.808" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const MESSAGE_CIRCLE_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.036 19.933a9.798 9.798 0 0 1 -3.336 -.933l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c2.128 1.815 3.053 4.361 2.694 6.81" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const MESSAGE_CIRCLE_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.996 19.98a9.868 9.868 0 0 1 -4.296 -.98l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.842 1.572 2.783 3.691 2.77 5.821" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const MESSAGE_CIRCLE_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.16 19.914a9.94 9.94 0 0 1 -5.46 -.914l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.384 1.181 2.26 2.672 2.603 4.243" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const MESSAGE_CIRCLE_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.006 19.98a9.869 9.869 0 0 1 -4.306 -.98l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.993 1.7 2.93 4.041 2.746 6.344" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const MESSAGE_CIRCLE_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.02 19.52c-2.34 .736 -5 .606 -7.32 -.52l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.96 1.671 2.898 3.963 2.755 6.227" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const MESSAGE_CIRCLE_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.59 19.88a9.763 9.763 0 0 1 -2.89 -.88l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.565 1.335 2.479 3.065 2.71 4.861" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const MESSAGE_CIRCLE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.023 19.98a9.87 9.87 0 0 1 -4.323 -.98l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c2.718 2.319 3.473 5.832 2.096 8.811" /> <path d="M16 19h6" /> </svg>"##;
const MESSAGE_CIRCLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.595 4.577c3.223 -1.176 7.025 -.61 9.65 1.63c2.982 2.543 3.601 6.523 1.636 9.66m-1.908 2.109c-2.787 2.19 -6.89 2.666 -10.273 1.024l-4.7 1l1.3 -3.9c-2.229 -3.296 -1.494 -7.511 1.68 -10.057" /> <path d="M3 3l18 18" /> </svg>"##;
const MESSAGE_CIRCLE_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.989 19.932a9.93 9.93 0 0 1 -5.289 -.932l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c2.131 1.818 3.056 4.37 2.692 6.824" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const MESSAGE_CIRCLE_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.337 19.974a9.891 9.891 0 0 1 -4.637 -.974l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.63 1.39 2.554 3.21 2.736 5.085" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const MESSAGE_CIRCLE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.007 19.98a9.869 9.869 0 0 1 -4.307 -.98l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.992 1.7 2.93 4.04 2.747 6.34" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const MESSAGE_CIRCLE_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.02 19.52c-2.341 .736 -5 .606 -7.32 -.52l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.649 1.407 2.575 3.253 2.742 5.152" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const MESSAGE_CIRCLE_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.303 19.955a9.818 9.818 0 0 1 -3.603 -.955l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.73 1.476 2.665 3.435 2.76 5.433" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const MESSAGE_CIRCLE_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.58 19.963a9.906 9.906 0 0 1 -4.88 -.963l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c2.13 1.817 3.055 4.368 2.692 6.82" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const MESSAGE_CIRCLE_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.517 19.869a9.757 9.757 0 0 1 -2.817 -.869l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.666 1.421 2.594 3.29 2.747 5.21" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const MESSAGE_CIRCLE_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.004 19.98a9.869 9.869 0 0 1 -4.304 -.98l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.994 1.701 2.932 4.045 2.746 6.349" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const MESSAGE_CIRCLE_USER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M22 22a2 2 0 0 0 -2 -2h-2a2 2 0 0 0 -2 2" /> <path d="M12.454 19.97a9.9 9.9 0 0 1 -4.754 -.97l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c1.667 1.423 2.596 3.294 2.747 5.216" /> </svg>"##;
const MESSAGE_CIRCLE_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.593 19.855a9.96 9.96 0 0 1 -5.893 -.855l-4.7 1l1.3 -3.9c-2.324 -3.437 -1.426 -7.872 2.1 -10.374c3.526 -2.501 8.59 -2.296 11.845 .48c2.128 1.816 3.053 4.363 2.693 6.813" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const MESSAGE_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M11.012 19.193l-3.012 1.807v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const MESSAGE_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M12.031 18.581l-4.031 2.419v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const MESSAGE_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M13 18l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v3.5" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const MESSAGE_DOTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 11v.01" /> <path d="M8 11v.01" /> <path d="M16 11v.01" /> <path d="M18 4a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-5l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3l12 0" /> </svg>"##;
const MESSAGE_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M11.998 18.601l-3.998 2.399v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const MESSAGE_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M15 18h-2l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const MESSAGE_FORWARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 4a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-5l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12" /> <path d="M13 8l3 3l-3 3" /> <path d="M16 11h-8" /> </svg>"##;
const MESSAGE_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h3.5" /> <path d="M10.48 19.512l-2.48 1.488v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const MESSAGE_LANGUAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 21v-13a3 3 0 0 1 3 -3h10a3 3 0 0 1 3 3v6a3 3 0 0 1 -3 3h-9l-4 4" /> <path d="M10 14v-4a2 2 0 1 1 4 0v4" /> <path d="M14 12h-4" /> </svg>"##;
const MESSAGE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M11.976 18.614l-3.976 2.386v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v8" /> <path d="M16 19h6" /> </svg>"##;
const MESSAGE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h1m4 0h3" /> <path d="M8 13h5" /> <path d="M8 4h10a3 3 0 0 1 3 3v8c0 .577 -.163 1.116 -.445 1.573m-2.555 1.427h-5l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8c0 -1.085 .576 -2.036 1.439 -2.562" /> <path d="M3 3l18 18" /> </svg>"##;
const MESSAGE_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M13 18l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const MESSAGE_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M12.007 18.596l-4.007 2.404v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4.5" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const MESSAGE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M12.01 18.594l-4.01 2.406v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const MESSAGE_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M14 18h-1l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4.5" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const MESSAGE_REPLY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 4a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-5l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12" /> <path d="M11 8l-3 3l3 3" /> <path d="M16 11h-8" /> </svg>"##;
const MESSAGE_REPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 4a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-5l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12" /> <path d="M12 8v3" /> <path d="M12 14v.01" /> </svg>"##;
const MESSAGE_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h5" /> <path d="M11.008 19.195l-3.008 1.805v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4.5" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const MESSAGE_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M13 18l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const MESSAGE_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h4.5" /> <path d="M10.325 19.605l-2.325 1.395v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4.5" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const MESSAGE_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M11.99 18.606l-3.99 2.394v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const MESSAGE_USER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 18l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4.5" /> <path d="M17 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M22 22a2 2 0 0 0 -2 -2h-2a2 2 0 0 0 -2 2" /> </svg>"##;
const MESSAGE_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h8" /> <path d="M8 13h6" /> <path d="M13 18l-5 3v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const MESSAGES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 14l-3 -3h-7a1 1 0 0 1 -1 -1v-6a1 1 0 0 1 1 -1h9a1 1 0 0 1 1 1v10" /> <path d="M14 15v2a1 1 0 0 1 -1 1h-7l-3 3v-10a1 1 0 0 1 1 -1h2" /> </svg>"##;
const MESSAGES_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M11 11a1 1 0 0 1 -1 -1m0 -3.968v-2.032a1 1 0 0 1 1 -1h9a1 1 0 0 1 1 1v10l-3 -3h-3" /> <path d="M14 15v2a1 1 0 0 1 -1 1h-7l-3 3v-10a1 1 0 0 1 1 -1h2" /> </svg>"##;
const PHONE_DONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2c-8.072 -.49 -14.51 -6.928 -15 -15a2 2 0 0 1 2 -2" /> <path d="M15 5l2 2l4 -4" /> </svg>"##;
const PHONE_END_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2c-8.072 -.49 -14.51 -6.928 -15 -15a2 2 0 0 1 2 -2" /> <path d="M17 3l4 4" /> <path d="M21 3l-4 4" /> </svg>"##;
const PHONE_RINGING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 4l-2 2" /> <path d="M22 10.5l-2.5 -.5" /> <path d="M13.5 2l.5 2.5" /> <path d="M5 4h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2c-8.072 -.49 -14.51 -6.928 -15 -15a2 2 0 0 1 2 -2" /> </svg>"##;
const RSS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M4 4a16 16 0 0 1 16 16" /> <path d="M4 11a9 9 0 0 1 9 9" /> </svg>"##;
const SEND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 14l11 -11" /> <path d="M21 3l-6.5 18a.55 .55 0 0 1 -1 0l-3.5 -7l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5" /> </svg>"##;
const SEND_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.698 4.034l16.302 7.966l-16.302 7.966a.503 .503 0 0 1 -.546 -.124a.555 .555 0 0 1 -.12 -.568l2.468 -7.274l-2.468 -7.274a.555 .555 0 0 1 .12 -.568a.503 .503 0 0 1 .546 -.124" /> <path d="M6.5 12h14.5" /> </svg>"##;
const SEND_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 14l2 -2m2 -2l7 -7" /> <path d="M10.718 6.713l10.282 -3.713l-3.715 10.289m-1.063 2.941l-1.722 4.77a.55 .55 0 0 1 -1 0l-3.5 -7l-7 -3.5a.55 .55 0 0 1 0 -1l4.772 -1.723" /> <path d="M3 3l18 18" /> </svg>"##;

/// Communication icon variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommunicationIcon {
    AddressBook,
    AddressBookOff,
    Bubble,
    BubbleMinus,
    BubblePlus,
    BubbleText,
    BubbleX,
    EmailStamp,
    Mail,
    MailAi,
    MailBitcoin,
    MailBolt,
    MailCancel,
    MailCheck,
    MailCode,
    MailCog,
    MailDollar,
    MailDown,
    MailExclamation,
    MailFast,
    MailForward,
    MailHeart,
    MailMinus,
    MailOff,
    MailOpened,
    MailPause,
    MailPin,
    MailPlus,
    MailQuestion,
    MailSearch,
    MailShare,
    MailSpark,
    MailStar,
    MailUp,
    MailX,
    Mailbox,
    MailboxOff,
    Message,
    Message2,
    Message2Bolt,
    Message2Cancel,
    Message2Check,
    Message2Code,
    Message2Cog,
    Message2Dollar,
    Message2Down,
    Message2Exclamation,
    Message2Heart,
    Message2Minus,
    Message2Off,
    Message2Pause,
    Message2Pin,
    Message2Plus,
    Message2Question,
    Message2Search,
    Message2Share,
    Message2Star,
    Message2Up,
    Message2X,
    MessageBolt,
    MessageCancel,
    MessageChatbot,
    MessageCheck,
    MessageCircle,
    MessageCircleBolt,
    MessageCircleCancel,
    MessageCircleCheck,
    MessageCircleCode,
    MessageCircleCog,
    MessageCircleDollar,
    MessageCircleDown,
    MessageCircleExclamation,
    MessageCircleHeart,
    MessageCircleMinus,
    MessageCircleOff,
    MessageCirclePause,
    MessageCirclePin,
    MessageCirclePlus,
    MessageCircleQuestion,
    MessageCircleSearch,
    MessageCircleShare,
    MessageCircleStar,
    MessageCircleUp,
    MessageCircleUser,
    MessageCircleX,
    MessageCode,
    MessageCog,
    MessageDollar,
    MessageDots,
    MessageDown,
    MessageExclamation,
    MessageForward,
    MessageHeart,
    MessageLanguage,
    MessageMinus,
    MessageOff,
    MessagePause,
    MessagePin,
    MessagePlus,
    MessageQuestion,
    MessageReply,
    MessageReport,
    MessageSearch,
    MessageShare,
    MessageStar,
    MessageUp,
    MessageUser,
    MessageX,
    Messages,
    MessagesOff,
    PhoneDone,
    PhoneEnd,
    PhoneRinging,
    Rss,
    Send,
    Send2,
    SendOff,
}

impl CommunicationIcon {
    /// Returns all available icons in this category.
    pub fn all() -> &'static [Self] {
        &[Self::AddressBook, Self::AddressBookOff, Self::Bubble, Self::BubbleMinus, Self::BubblePlus, Self::BubbleText, Self::BubbleX, Self::EmailStamp, Self::Mail, Self::MailAi, Self::MailBitcoin, Self::MailBolt, Self::MailCancel, Self::MailCheck, Self::MailCode, Self::MailCog, Self::MailDollar, Self::MailDown, Self::MailExclamation, Self::MailFast, Self::MailForward, Self::MailHeart, Self::MailMinus, Self::MailOff, Self::MailOpened, Self::MailPause, Self::MailPin, Self::MailPlus, Self::MailQuestion, Self::MailSearch, Self::MailShare, Self::MailSpark, Self::MailStar, Self::MailUp, Self::MailX, Self::Mailbox, Self::MailboxOff, Self::Message, Self::Message2, Self::Message2Bolt, Self::Message2Cancel, Self::Message2Check, Self::Message2Code, Self::Message2Cog, Self::Message2Dollar, Self::Message2Down, Self::Message2Exclamation, Self::Message2Heart, Self::Message2Minus, Self::Message2Off, Self::Message2Pause, Self::Message2Pin, Self::Message2Plus, Self::Message2Question, Self::Message2Search, Self::Message2Share, Self::Message2Star, Self::Message2Up, Self::Message2X, Self::MessageBolt, Self::MessageCancel, Self::MessageChatbot, Self::MessageCheck, Self::MessageCircle, Self::MessageCircleBolt, Self::MessageCircleCancel, Self::MessageCircleCheck, Self::MessageCircleCode, Self::MessageCircleCog, Self::MessageCircleDollar, Self::MessageCircleDown, Self::MessageCircleExclamation, Self::MessageCircleHeart, Self::MessageCircleMinus, Self::MessageCircleOff, Self::MessageCirclePause, Self::MessageCirclePin, Self::MessageCirclePlus, Self::MessageCircleQuestion, Self::MessageCircleSearch, Self::MessageCircleShare, Self::MessageCircleStar, Self::MessageCircleUp, Self::MessageCircleUser, Self::MessageCircleX, Self::MessageCode, Self::MessageCog, Self::MessageDollar, Self::MessageDots, Self::MessageDown, Self::MessageExclamation, Self::MessageForward, Self::MessageHeart, Self::MessageLanguage, Self::MessageMinus, Self::MessageOff, Self::MessagePause, Self::MessagePin, Self::MessagePlus, Self::MessageQuestion, Self::MessageReply, Self::MessageReport, Self::MessageSearch, Self::MessageShare, Self::MessageStar, Self::MessageUp, Self::MessageUser, Self::MessageX, Self::Messages, Self::MessagesOff, Self::PhoneDone, Self::PhoneEnd, Self::PhoneRinging, Self::Rss, Self::Send, Self::Send2, Self::SendOff]
    }

    /// Returns the icon count.
    pub fn count() -> usize {
        117
    }

    /// Creates an icon from its kebab-case name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "address-book" => Some(Self::AddressBook),
            "address-book-off" => Some(Self::AddressBookOff),
            "bubble" => Some(Self::Bubble),
            "bubble-minus" => Some(Self::BubbleMinus),
            "bubble-plus" => Some(Self::BubblePlus),
            "bubble-text" => Some(Self::BubbleText),
            "bubble-x" => Some(Self::BubbleX),
            "email-stamp" => Some(Self::EmailStamp),
            "mail" => Some(Self::Mail),
            "mail-ai" => Some(Self::MailAi),
            "mail-bitcoin" => Some(Self::MailBitcoin),
            "mail-bolt" => Some(Self::MailBolt),
            "mail-cancel" => Some(Self::MailCancel),
            "mail-check" => Some(Self::MailCheck),
            "mail-code" => Some(Self::MailCode),
            "mail-cog" => Some(Self::MailCog),
            "mail-dollar" => Some(Self::MailDollar),
            "mail-down" => Some(Self::MailDown),
            "mail-exclamation" => Some(Self::MailExclamation),
            "mail-fast" => Some(Self::MailFast),
            "mail-forward" => Some(Self::MailForward),
            "mail-heart" => Some(Self::MailHeart),
            "mail-minus" => Some(Self::MailMinus),
            "mail-off" => Some(Self::MailOff),
            "mail-opened" => Some(Self::MailOpened),
            "mail-pause" => Some(Self::MailPause),
            "mail-pin" => Some(Self::MailPin),
            "mail-plus" => Some(Self::MailPlus),
            "mail-question" => Some(Self::MailQuestion),
            "mail-search" => Some(Self::MailSearch),
            "mail-share" => Some(Self::MailShare),
            "mail-spark" => Some(Self::MailSpark),
            "mail-star" => Some(Self::MailStar),
            "mail-up" => Some(Self::MailUp),
            "mail-x" => Some(Self::MailX),
            "mailbox" => Some(Self::Mailbox),
            "mailbox-off" => Some(Self::MailboxOff),
            "message" => Some(Self::Message),
            "message-2" => Some(Self::Message2),
            "message-2-bolt" => Some(Self::Message2Bolt),
            "message-2-cancel" => Some(Self::Message2Cancel),
            "message-2-check" => Some(Self::Message2Check),
            "message-2-code" => Some(Self::Message2Code),
            "message-2-cog" => Some(Self::Message2Cog),
            "message-2-dollar" => Some(Self::Message2Dollar),
            "message-2-down" => Some(Self::Message2Down),
            "message-2-exclamation" => Some(Self::Message2Exclamation),
            "message-2-heart" => Some(Self::Message2Heart),
            "message-2-minus" => Some(Self::Message2Minus),
            "message-2-off" => Some(Self::Message2Off),
            "message-2-pause" => Some(Self::Message2Pause),
            "message-2-pin" => Some(Self::Message2Pin),
            "message-2-plus" => Some(Self::Message2Plus),
            "message-2-question" => Some(Self::Message2Question),
            "message-2-search" => Some(Self::Message2Search),
            "message-2-share" => Some(Self::Message2Share),
            "message-2-star" => Some(Self::Message2Star),
            "message-2-up" => Some(Self::Message2Up),
            "message-2-x" => Some(Self::Message2X),
            "message-bolt" => Some(Self::MessageBolt),
            "message-cancel" => Some(Self::MessageCancel),
            "message-chatbot" => Some(Self::MessageChatbot),
            "message-check" => Some(Self::MessageCheck),
            "message-circle" => Some(Self::MessageCircle),
            "message-circle-bolt" => Some(Self::MessageCircleBolt),
            "message-circle-cancel" => Some(Self::MessageCircleCancel),
            "message-circle-check" => Some(Self::MessageCircleCheck),
            "message-circle-code" => Some(Self::MessageCircleCode),
            "message-circle-cog" => Some(Self::MessageCircleCog),
            "message-circle-dollar" => Some(Self::MessageCircleDollar),
            "message-circle-down" => Some(Self::MessageCircleDown),
            "message-circle-exclamation" => Some(Self::MessageCircleExclamation),
            "message-circle-heart" => Some(Self::MessageCircleHeart),
            "message-circle-minus" => Some(Self::MessageCircleMinus),
            "message-circle-off" => Some(Self::MessageCircleOff),
            "message-circle-pause" => Some(Self::MessageCirclePause),
            "message-circle-pin" => Some(Self::MessageCirclePin),
            "message-circle-plus" => Some(Self::MessageCirclePlus),
            "message-circle-question" => Some(Self::MessageCircleQuestion),
            "message-circle-search" => Some(Self::MessageCircleSearch),
            "message-circle-share" => Some(Self::MessageCircleShare),
            "message-circle-star" => Some(Self::MessageCircleStar),
            "message-circle-up" => Some(Self::MessageCircleUp),
            "message-circle-user" => Some(Self::MessageCircleUser),
            "message-circle-x" => Some(Self::MessageCircleX),
            "message-code" => Some(Self::MessageCode),
            "message-cog" => Some(Self::MessageCog),
            "message-dollar" => Some(Self::MessageDollar),
            "message-dots" => Some(Self::MessageDots),
            "message-down" => Some(Self::MessageDown),
            "message-exclamation" => Some(Self::MessageExclamation),
            "message-forward" => Some(Self::MessageForward),
            "message-heart" => Some(Self::MessageHeart),
            "message-language" => Some(Self::MessageLanguage),
            "message-minus" => Some(Self::MessageMinus),
            "message-off" => Some(Self::MessageOff),
            "message-pause" => Some(Self::MessagePause),
            "message-pin" => Some(Self::MessagePin),
            "message-plus" => Some(Self::MessagePlus),
            "message-question" => Some(Self::MessageQuestion),
            "message-reply" => Some(Self::MessageReply),
            "message-report" => Some(Self::MessageReport),
            "message-search" => Some(Self::MessageSearch),
            "message-share" => Some(Self::MessageShare),
            "message-star" => Some(Self::MessageStar),
            "message-up" => Some(Self::MessageUp),
            "message-user" => Some(Self::MessageUser),
            "message-x" => Some(Self::MessageX),
            "messages" => Some(Self::Messages),
            "messages-off" => Some(Self::MessagesOff),
            "phone-done" => Some(Self::PhoneDone),
            "phone-end" => Some(Self::PhoneEnd),
            "phone-ringing" => Some(Self::PhoneRinging),
            "rss" => Some(Self::Rss),
            "send" => Some(Self::Send),
            "send-2" => Some(Self::Send2),
            "send-off" => Some(Self::SendOff),
            _ => None,
        }
    }
}

impl TablerIconData for CommunicationIcon {
    fn name(&self) -> &'static str {
        match self {
            Self::AddressBook => "address-book",
            Self::AddressBookOff => "address-book-off",
            Self::Bubble => "bubble",
            Self::BubbleMinus => "bubble-minus",
            Self::BubblePlus => "bubble-plus",
            Self::BubbleText => "bubble-text",
            Self::BubbleX => "bubble-x",
            Self::EmailStamp => "email-stamp",
            Self::Mail => "mail",
            Self::MailAi => "mail-ai",
            Self::MailBitcoin => "mail-bitcoin",
            Self::MailBolt => "mail-bolt",
            Self::MailCancel => "mail-cancel",
            Self::MailCheck => "mail-check",
            Self::MailCode => "mail-code",
            Self::MailCog => "mail-cog",
            Self::MailDollar => "mail-dollar",
            Self::MailDown => "mail-down",
            Self::MailExclamation => "mail-exclamation",
            Self::MailFast => "mail-fast",
            Self::MailForward => "mail-forward",
            Self::MailHeart => "mail-heart",
            Self::MailMinus => "mail-minus",
            Self::MailOff => "mail-off",
            Self::MailOpened => "mail-opened",
            Self::MailPause => "mail-pause",
            Self::MailPin => "mail-pin",
            Self::MailPlus => "mail-plus",
            Self::MailQuestion => "mail-question",
            Self::MailSearch => "mail-search",
            Self::MailShare => "mail-share",
            Self::MailSpark => "mail-spark",
            Self::MailStar => "mail-star",
            Self::MailUp => "mail-up",
            Self::MailX => "mail-x",
            Self::Mailbox => "mailbox",
            Self::MailboxOff => "mailbox-off",
            Self::Message => "message",
            Self::Message2 => "message-2",
            Self::Message2Bolt => "message-2-bolt",
            Self::Message2Cancel => "message-2-cancel",
            Self::Message2Check => "message-2-check",
            Self::Message2Code => "message-2-code",
            Self::Message2Cog => "message-2-cog",
            Self::Message2Dollar => "message-2-dollar",
            Self::Message2Down => "message-2-down",
            Self::Message2Exclamation => "message-2-exclamation",
            Self::Message2Heart => "message-2-heart",
            Self::Message2Minus => "message-2-minus",
            Self::Message2Off => "message-2-off",
            Self::Message2Pause => "message-2-pause",
            Self::Message2Pin => "message-2-pin",
            Self::Message2Plus => "message-2-plus",
            Self::Message2Question => "message-2-question",
            Self::Message2Search => "message-2-search",
            Self::Message2Share => "message-2-share",
            Self::Message2Star => "message-2-star",
            Self::Message2Up => "message-2-up",
            Self::Message2X => "message-2-x",
            Self::MessageBolt => "message-bolt",
            Self::MessageCancel => "message-cancel",
            Self::MessageChatbot => "message-chatbot",
            Self::MessageCheck => "message-check",
            Self::MessageCircle => "message-circle",
            Self::MessageCircleBolt => "message-circle-bolt",
            Self::MessageCircleCancel => "message-circle-cancel",
            Self::MessageCircleCheck => "message-circle-check",
            Self::MessageCircleCode => "message-circle-code",
            Self::MessageCircleCog => "message-circle-cog",
            Self::MessageCircleDollar => "message-circle-dollar",
            Self::MessageCircleDown => "message-circle-down",
            Self::MessageCircleExclamation => "message-circle-exclamation",
            Self::MessageCircleHeart => "message-circle-heart",
            Self::MessageCircleMinus => "message-circle-minus",
            Self::MessageCircleOff => "message-circle-off",
            Self::MessageCirclePause => "message-circle-pause",
            Self::MessageCirclePin => "message-circle-pin",
            Self::MessageCirclePlus => "message-circle-plus",
            Self::MessageCircleQuestion => "message-circle-question",
            Self::MessageCircleSearch => "message-circle-search",
            Self::MessageCircleShare => "message-circle-share",
            Self::MessageCircleStar => "message-circle-star",
            Self::MessageCircleUp => "message-circle-up",
            Self::MessageCircleUser => "message-circle-user",
            Self::MessageCircleX => "message-circle-x",
            Self::MessageCode => "message-code",
            Self::MessageCog => "message-cog",
            Self::MessageDollar => "message-dollar",
            Self::MessageDots => "message-dots",
            Self::MessageDown => "message-down",
            Self::MessageExclamation => "message-exclamation",
            Self::MessageForward => "message-forward",
            Self::MessageHeart => "message-heart",
            Self::MessageLanguage => "message-language",
            Self::MessageMinus => "message-minus",
            Self::MessageOff => "message-off",
            Self::MessagePause => "message-pause",
            Self::MessagePin => "message-pin",
            Self::MessagePlus => "message-plus",
            Self::MessageQuestion => "message-question",
            Self::MessageReply => "message-reply",
            Self::MessageReport => "message-report",
            Self::MessageSearch => "message-search",
            Self::MessageShare => "message-share",
            Self::MessageStar => "message-star",
            Self::MessageUp => "message-up",
            Self::MessageUser => "message-user",
            Self::MessageX => "message-x",
            Self::Messages => "messages",
            Self::MessagesOff => "messages-off",
            Self::PhoneDone => "phone-done",
            Self::PhoneEnd => "phone-end",
            Self::PhoneRinging => "phone-ringing",
            Self::Rss => "rss",
            Self::Send => "send",
            Self::Send2 => "send-2",
            Self::SendOff => "send-off",
        }
    }

    fn outline_svg(&self) -> &'static str {
        match self {
            Self::AddressBook => ADDRESS_BOOK_SVG,
            Self::AddressBookOff => ADDRESS_BOOK_OFF_SVG,
            Self::Bubble => BUBBLE_SVG,
            Self::BubbleMinus => BUBBLE_MINUS_SVG,
            Self::BubblePlus => BUBBLE_PLUS_SVG,
            Self::BubbleText => BUBBLE_TEXT_SVG,
            Self::BubbleX => BUBBLE_X_SVG,
            Self::EmailStamp => EMAIL_STAMP_SVG,
            Self::Mail => MAIL_SVG,
            Self::MailAi => MAIL_AI_SVG,
            Self::MailBitcoin => MAIL_BITCOIN_SVG,
            Self::MailBolt => MAIL_BOLT_SVG,
            Self::MailCancel => MAIL_CANCEL_SVG,
            Self::MailCheck => MAIL_CHECK_SVG,
            Self::MailCode => MAIL_CODE_SVG,
            Self::MailCog => MAIL_COG_SVG,
            Self::MailDollar => MAIL_DOLLAR_SVG,
            Self::MailDown => MAIL_DOWN_SVG,
            Self::MailExclamation => MAIL_EXCLAMATION_SVG,
            Self::MailFast => MAIL_FAST_SVG,
            Self::MailForward => MAIL_FORWARD_SVG,
            Self::MailHeart => MAIL_HEART_SVG,
            Self::MailMinus => MAIL_MINUS_SVG,
            Self::MailOff => MAIL_OFF_SVG,
            Self::MailOpened => MAIL_OPENED_SVG,
            Self::MailPause => MAIL_PAUSE_SVG,
            Self::MailPin => MAIL_PIN_SVG,
            Self::MailPlus => MAIL_PLUS_SVG,
            Self::MailQuestion => MAIL_QUESTION_SVG,
            Self::MailSearch => MAIL_SEARCH_SVG,
            Self::MailShare => MAIL_SHARE_SVG,
            Self::MailSpark => MAIL_SPARK_SVG,
            Self::MailStar => MAIL_STAR_SVG,
            Self::MailUp => MAIL_UP_SVG,
            Self::MailX => MAIL_X_SVG,
            Self::Mailbox => MAILBOX_SVG,
            Self::MailboxOff => MAILBOX_OFF_SVG,
            Self::Message => MESSAGE_SVG,
            Self::Message2 => MESSAGE_2_SVG,
            Self::Message2Bolt => MESSAGE_2_BOLT_SVG,
            Self::Message2Cancel => MESSAGE_2_CANCEL_SVG,
            Self::Message2Check => MESSAGE_2_CHECK_SVG,
            Self::Message2Code => MESSAGE_2_CODE_SVG,
            Self::Message2Cog => MESSAGE_2_COG_SVG,
            Self::Message2Dollar => MESSAGE_2_DOLLAR_SVG,
            Self::Message2Down => MESSAGE_2_DOWN_SVG,
            Self::Message2Exclamation => MESSAGE_2_EXCLAMATION_SVG,
            Self::Message2Heart => MESSAGE_2_HEART_SVG,
            Self::Message2Minus => MESSAGE_2_MINUS_SVG,
            Self::Message2Off => MESSAGE_2_OFF_SVG,
            Self::Message2Pause => MESSAGE_2_PAUSE_SVG,
            Self::Message2Pin => MESSAGE_2_PIN_SVG,
            Self::Message2Plus => MESSAGE_2_PLUS_SVG,
            Self::Message2Question => MESSAGE_2_QUESTION_SVG,
            Self::Message2Search => MESSAGE_2_SEARCH_SVG,
            Self::Message2Share => MESSAGE_2_SHARE_SVG,
            Self::Message2Star => MESSAGE_2_STAR_SVG,
            Self::Message2Up => MESSAGE_2_UP_SVG,
            Self::Message2X => MESSAGE_2_X_SVG,
            Self::MessageBolt => MESSAGE_BOLT_SVG,
            Self::MessageCancel => MESSAGE_CANCEL_SVG,
            Self::MessageChatbot => MESSAGE_CHATBOT_SVG,
            Self::MessageCheck => MESSAGE_CHECK_SVG,
            Self::MessageCircle => MESSAGE_CIRCLE_SVG,
            Self::MessageCircleBolt => MESSAGE_CIRCLE_BOLT_SVG,
            Self::MessageCircleCancel => MESSAGE_CIRCLE_CANCEL_SVG,
            Self::MessageCircleCheck => MESSAGE_CIRCLE_CHECK_SVG,
            Self::MessageCircleCode => MESSAGE_CIRCLE_CODE_SVG,
            Self::MessageCircleCog => MESSAGE_CIRCLE_COG_SVG,
            Self::MessageCircleDollar => MESSAGE_CIRCLE_DOLLAR_SVG,
            Self::MessageCircleDown => MESSAGE_CIRCLE_DOWN_SVG,
            Self::MessageCircleExclamation => MESSAGE_CIRCLE_EXCLAMATION_SVG,
            Self::MessageCircleHeart => MESSAGE_CIRCLE_HEART_SVG,
            Self::MessageCircleMinus => MESSAGE_CIRCLE_MINUS_SVG,
            Self::MessageCircleOff => MESSAGE_CIRCLE_OFF_SVG,
            Self::MessageCirclePause => MESSAGE_CIRCLE_PAUSE_SVG,
            Self::MessageCirclePin => MESSAGE_CIRCLE_PIN_SVG,
            Self::MessageCirclePlus => MESSAGE_CIRCLE_PLUS_SVG,
            Self::MessageCircleQuestion => MESSAGE_CIRCLE_QUESTION_SVG,
            Self::MessageCircleSearch => MESSAGE_CIRCLE_SEARCH_SVG,
            Self::MessageCircleShare => MESSAGE_CIRCLE_SHARE_SVG,
            Self::MessageCircleStar => MESSAGE_CIRCLE_STAR_SVG,
            Self::MessageCircleUp => MESSAGE_CIRCLE_UP_SVG,
            Self::MessageCircleUser => MESSAGE_CIRCLE_USER_SVG,
            Self::MessageCircleX => MESSAGE_CIRCLE_X_SVG,
            Self::MessageCode => MESSAGE_CODE_SVG,
            Self::MessageCog => MESSAGE_COG_SVG,
            Self::MessageDollar => MESSAGE_DOLLAR_SVG,
            Self::MessageDots => MESSAGE_DOTS_SVG,
            Self::MessageDown => MESSAGE_DOWN_SVG,
            Self::MessageExclamation => MESSAGE_EXCLAMATION_SVG,
            Self::MessageForward => MESSAGE_FORWARD_SVG,
            Self::MessageHeart => MESSAGE_HEART_SVG,
            Self::MessageLanguage => MESSAGE_LANGUAGE_SVG,
            Self::MessageMinus => MESSAGE_MINUS_SVG,
            Self::MessageOff => MESSAGE_OFF_SVG,
            Self::MessagePause => MESSAGE_PAUSE_SVG,
            Self::MessagePin => MESSAGE_PIN_SVG,
            Self::MessagePlus => MESSAGE_PLUS_SVG,
            Self::MessageQuestion => MESSAGE_QUESTION_SVG,
            Self::MessageReply => MESSAGE_REPLY_SVG,
            Self::MessageReport => MESSAGE_REPORT_SVG,
            Self::MessageSearch => MESSAGE_SEARCH_SVG,
            Self::MessageShare => MESSAGE_SHARE_SVG,
            Self::MessageStar => MESSAGE_STAR_SVG,
            Self::MessageUp => MESSAGE_UP_SVG,
            Self::MessageUser => MESSAGE_USER_SVG,
            Self::MessageX => MESSAGE_X_SVG,
            Self::Messages => MESSAGES_SVG,
            Self::MessagesOff => MESSAGES_OFF_SVG,
            Self::PhoneDone => PHONE_DONE_SVG,
            Self::PhoneEnd => PHONE_END_SVG,
            Self::PhoneRinging => PHONE_RINGING_SVG,
            Self::Rss => RSS_SVG,
            Self::Send => SEND_SVG,
            Self::Send2 => SEND_2_SVG,
            Self::SendOff => SEND_OFF_SVG,
        }
    }

    fn filled_svg(&self) -> Option<&'static str> {
        // Filled variants would be added here
        None
    }
}
