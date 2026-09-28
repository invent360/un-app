//! System icons from Tabler Icons.
//!
//! This module contains 1185 icons.

use crate::tabler::TablerIconData;

// SVG Constants
const A_B_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 16v-5.5a2.5 2.5 0 0 1 5 0v5.5m0 -4h-5" /> <path d="M12 6l0 12" /> <path d="M16 16v-8h3a2 2 0 0 1 0 4h-3m3 0a2 2 0 0 1 0 4h-3" /> </svg>"##;
const ABACUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 3v18" /> <path d="M19 21v-18" /> <path d="M5 7h14" /> <path d="M5 15h14" /> <path d="M8 13v4" /> <path d="M11 13v4" /> <path d="M16 13v4" /> <path d="M14 5v4" /> <path d="M11 5v4" /> <path d="M8 5v4" /> <path d="M3 21h18" /> </svg>"##;
const ABACUS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5v16" /> <path d="M19 21v-2m0 -4v-12" /> <path d="M5 7h2m4 0h8" /> <path d="M5 15h10" /> <path d="M8 13v4" /> <path d="M11 13v4" /> <path d="M16 16v1" /> <path d="M14 5v4" /> <path d="M11 5v2" /> <path d="M8 8v1" /> <path d="M3 21h18" /> <path d="M3 3l18 18" /> </svg>"##;
const ACCESSIBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 16.5l2 -3l2 3m-2 -3v-2l3 -1m-6 0l3 1" /> <path d="M11.5 7.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> </svg>"##;
const ACCESSIBLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16.5l2 -3l2 3m-2 -3v-1.5m2.627 -1.376l.373 -.124m-6 0l2.231 .744" /> <path d="M20.042 16.045a9 9 0 0 0 -12.087 -12.087m-2.318 1.677a9 9 0 1 0 12.725 12.73" /> <path d="M12 8a.5 .5 0 1 0 -.5 -.5" /> <path d="M3 3l18 18" /> </svg>"##;
const ACTIVITY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h4l3 8l4 -16l3 8h4" /> </svg>"##;
const ACTIVITY_HEARTBEAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h4.5l1.5 -6l4 12l2 -9l1.5 3h4.5" /> </svg>"##;
const AD_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 12a10 10 0 1 0 20 0a10 10 0 1 0 -20 0" /> <path d="M7 15v-4.5a1.5 1.5 0 0 1 3 0v4.5" /> <path d="M7 13h3" /> <path d="M14 9v6h1a2 2 0 0 0 2 -2v-2a2 2 0 0 0 -2 -2h-1" /> </svg>"##;
const AD_CIRCLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.91 4.949a9.968 9.968 0 0 0 -2.91 7.051c0 5.523 4.477 10 10 10a9.968 9.968 0 0 0 7.05 -2.909" /> <path d="M20.778 16.793a9.955 9.955 0 0 0 1.222 -4.793c0 -5.523 -4.477 -10 -10 -10c-1.74 0 -3.376 .444 -4.8 1.225" /> <path d="M7 15v-4.5a1.5 1.5 0 0 1 2.138 -1.358" /> <path d="M9.854 9.853c.094 .196 .146 .415 .146 .647v4.5" /> <path d="M7 13h3" /> <path d="M14 14v1h1" /> <path d="M17 13v-2a2 2 0 0 0 -2 -2h-1v1" /> <path d="M3 3l18 18" /> </svg>"##;
const ADJUSTMENTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M10 16a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v11" /> </svg>"##;
const ADJUSTMENTS_ALT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8h4v4h-4l0 -4" /> <path d="M6 4l0 4" /> <path d="M6 12l0 8" /> <path d="M10 14h4v4h-4l0 -4" /> <path d="M12 4l0 10" /> <path d="M12 18l0 2" /> <path d="M16 5h4v4h-4l0 -4" /> <path d="M18 4l0 1" /> <path d="M18 9l0 11" /> </svg>"##;
const ADJUSTMENTS_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M10 16a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 4v10" /> <path d="M19 16l-2 3h4l-2 3" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v3" /> </svg>"##;
const ADJUSTMENTS_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.499 14.675a2 2 0 1 0 -1.499 3.325" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v3" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const ADJUSTMENTS_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.823 15.176a2 2 0 1 0 -2.638 2.651" /> <path d="M12 4v10" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v5" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const ADJUSTMENTS_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.557 14.745a2 2 0 1 0 -1.557 3.255" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v4" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const ADJUSTMENTS_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.199 14.399a2 2 0 1 0 -1.199 3.601" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v2.5" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const ADJUSTMENTS_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.366 14.54a2 2 0 1 0 -.216 3.097" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v1" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const ADJUSTMENTS_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.945 15.53a2 2 0 1 0 -1.945 2.47" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v3" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const ADJUSTMENTS_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M10 16a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v3" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const ADJUSTMENTS_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M12 4v8.5" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v2.5" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const ADJUSTMENTS_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 6l8 0" /> <path d="M16 6l4 0" /> <path d="M6 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 12l2 0" /> <path d="M10 12l10 0" /> <path d="M15 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 18l11 0" /> <path d="M19 18l1 0" /> </svg>"##;
const ADJUSTMENTS_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.954 15.574a2 2 0 1 0 -1.954 2.426" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v6" /> <path d="M16 19h6" /> </svg>"##;
const ADJUSTMENTS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 6v2" /> <path d="M6 12v8" /> <path d="M10 16a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 4v4m0 4v2" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v5m0 4v2" /> <path d="M3 3l18 18" /> </svg>"##;
const ADJUSTMENTS_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.627 14.836a2 2 0 1 0 -.62 2.892" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M17 17v5" /> <path d="M21 17v5" /> <path d="M18 9v4.5" /> </svg>"##;
const ADJUSTMENTS_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.071 14.31a2 2 0 1 0 -1.071 3.69" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v2.5" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const ADJUSTMENTS_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.958 15.592a2 2 0 1 0 -1.958 2.408" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v3" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const ADJUSTMENTS_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.577 14.77a2 2 0 1 0 .117 2.295" /> <path d="M12 4v10" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v2" /> </svg>"##;
const ADJUSTMENTS_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M12 14a2 2 0 0 0 -1.042 3.707" /> <path d="M12 4v10" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v2" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const ADJUSTMENTS_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.387 14.56a2 2 0 1 0 -.798 3.352" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> <path d="M18 9v4" /> </svg>"##;
const ADJUSTMENTS_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.879 15.312a2 2 0 1 0 -2.26 2.652" /> <path d="M12 4v10" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v2.5" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const ADJUSTMENTS_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M12 4v9.5" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> <path d="M18 9v1" /> </svg>"##;
const ADJUSTMENTS_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.927 15.462a2 2 0 1 0 -1.927 2.538" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v3" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const ADJUSTMENTS_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 4v4" /> <path d="M6 12v8" /> <path d="M13.653 14.874a2 2 0 1 0 -.586 2.818" /> <path d="M12 4v10" /> <path d="M12 18v2" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M18 4v1" /> <path d="M18 9v4" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const AFFILIATE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.931 6.936l1.275 4.249m5.607 5.609l4.251 1.275" /> <path d="M11.683 12.317l5.759 -5.759" /> <path d="M4 5.5a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> <path d="M17 5.5a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> <path d="M17 18.5a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> <path d="M4 15.5a4.5 4.5 0 1 0 9 0a4.5 4.5 0 1 0 -9 0" /> </svg>"##;
const AI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16v-6a2 2 0 1 1 4 0v6" /> <path d="M8 13h4" /> <path d="M16 8v8" /> </svg>"##;
const AI_AGENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 14a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M6 14a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M16 14a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M13.5 9.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M8.5 9.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M13.5 18.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M8.5 18.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3.5 18.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M18.5 18.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const AI_AGENTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6 5a1 1 0 1 0 -2 0a1 1 0 0 0 2 0" /> <path d="M18 5a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M4 12a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M18 12a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M11 19a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> </svg>"##;
const AI_GATEWAY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> <path d="M15 6.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> <path d="M15 17.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> <path d="M4 17.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> <path d="M8.5 15.5l7 -7" /> </svg>"##;
const ALARM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 13a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M12 10l0 3l2 0" /> <path d="M7 4l-2.75 2" /> <path d="M17 4l2.75 2" /> </svg>"##;
const ALARM_AVERAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 13a7 7 0 1 0 14 0a7 7 0 0 0 -14 0" /> <path d="M7 4l-2.75 2" /> <path d="M17 4l2.75 2" /> <path d="M8 13h1l2 3l2 -6l2 3h1" /> </svg>"##;
const ALARM_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 13a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M7 4l-2.75 2" /> <path d="M17 4l2.75 2" /> <path d="M10 13h4" /> </svg>"##;
const ALARM_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.587 7.566a7 7 0 1 0 9.833 9.864m1.35 -2.645a7 7 0 0 0 -8.536 -8.56" /> <path d="M12 12v1h1" /> <path d="M5.261 5.265l-1.011 .735" /> <path d="M17 4l2.75 2" /> <path d="M3 3l18 18" /> </svg>"##;
const ALARM_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 13a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M7 4l-2.75 2" /> <path d="M17 4l2.75 2" /> <path d="M10 13h4" /> <path d="M12 11v4" /> </svg>"##;
const ALARM_SNOOZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 13a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M10 11h4l-4 4h4" /> <path d="M7 4l-2.75 2" /> <path d="M17 4l2.75 2" /> </svg>"##;
const ALERT_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M12 8v4" /> <path d="M12 16h.01" /> </svg>"##;
const ALERT_CIRCLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.644 5.629a9 9 0 1 0 12.715 12.741m1.693 -2.349a9 9 0 0 0 -12.087 -12.068" /> <path d="M12 7v1" /> <path d="M12 16h.01" /> <path d="M3 3l18 18" /> </svg>"##;
const ALERT_HEXAGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27c.7 .398 1.13 1.143 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M12 8v4" /> <path d="M12 16h.01" /> </svg>"##;
const ALERT_HEXAGON_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.36 18.387l-5.268 3.333a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l1.317 -.777m2.535 -1.493l2.898 -1.709a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033c.7 .398 1.13 1.143 1.125 1.948v7.284c0 .414 -.116 .812 -.326 1.155" /> <path d="M12 7v1" /> <path d="M12 8v.01" /> <path d="M3 3l18 18" /> <path d="M12 16h.01" /> </svg>"##;
const ALERT_OCTAGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.802 2.165l5.575 2.389c.48 .206 .863 .589 1.07 1.07l2.388 5.574c.22 .512 .22 1.092 0 1.604l-2.389 5.575c-.206 .48 -.589 .863 -1.07 1.07l-5.574 2.388c-.512 .22 -1.092 .22 -1.604 0l-5.575 -2.389a2.036 2.036 0 0 1 -1.07 -1.07l-2.388 -5.574a2.036 2.036 0 0 1 0 -1.604l2.389 -5.575c.206 -.48 .589 -.863 1.07 -1.07l5.574 -2.388a2.036 2.036 0 0 1 1.604 0" /> <path d="M12 8v4" /> <path d="M12 16h.01" /> </svg>"##;
const ALERT_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8v4" /> <path d="M12 16h.01" /> </svg>"##;
const ALERT_SMALL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 16h.01" /> <path d="M12 7v1" /> <path d="M3 3l18 18" /> </svg>"##;
const ALERT_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M12 8v4" /> <path d="M12 16h.01" /> </svg>"##;
const ALERT_SQUARE_ROUNDED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> <path d="M12 8v4" /> <path d="M12 16h.01" /> </svg>"##;
const ALERT_SQUARE_ROUNDED_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.201 19.199c-1.35 1.35 -3.6 1.801 -7.201 1.801c-7.2 0 -9 -1.8 -9 -9c0 -3.598 .45 -5.847 1.797 -7.197m2.626 -1.376c1.204 -.307 2.709 -.427 4.577 -.427c7.2 0 9 1.8 9 9c0 1.865 -.12 3.367 -.425 4.57" /> <path d="M12 7v1" /> <path d="M12 16h.01" /> <path d="M3 3l18 18" /> </svg>"##;
const ALERT_TRIANGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9v4" /> <path d="M10.363 3.591l-8.106 13.534a1.914 1.914 0 0 0 1.636 2.871h16.214a1.914 1.914 0 0 0 1.636 -2.87l-8.106 -13.536a1.914 1.914 0 0 0 -3.274 0" /> <path d="M12 16h.01" /> </svg>"##;
const ALERT_TRIANGLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21.998 17.997a1.913 1.913 0 0 0 -.255 -.872l-8.106 -13.534a1.914 1.914 0 0 0 -3.274 0l-1.04 1.736m-1.493 2.493l-5.573 9.304a1.914 1.914 0 0 0 1.636 2.871h16.107" /> <path d="M12 16h.01" /> <path d="M3 3l18 18" /> <path d="M12 7v1" /> </svg>"##;
const ALT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 16v-6a2 2 0 1 1 4 0v6" /> <path d="M4 13h4" /> <path d="M11 8v8h4" /> <path d="M16 8h4" /> <path d="M18 8v8" /> </svg>"##;
const AMPERSAND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 20l-10.403 -10.972a2.948 2.948 0 0 1 0 -4.165a2.94 2.94 0 0 1 4.161 0a2.948 2.948 0 0 1 0 4.165l-4.68 4.687a3.685 3.685 0 0 0 0 5.207a3.675 3.675 0 0 0 5.2 0l5.722 -5.922" /> </svg>"##;
const ANALYZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 11a8.1 8.1 0 0 0 -6.986 -6.918a8.095 8.095 0 0 0 -8.019 3.918" /> <path d="M4 13a8.1 8.1 0 0 0 15 3" /> <path d="M18 16a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M4 8a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const ANALYZE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 11a8.1 8.1 0 0 0 -6.986 -6.918a8.086 8.086 0 0 0 -4.31 .62m-2.383 1.608a8.089 8.089 0 0 0 -1.326 1.69" /> <path d="M4 13a8.1 8.1 0 0 0 13.687 4.676" /> <path d="M20 16a1 1 0 0 0 -1 -1" /> <path d="M4 8a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M9.888 9.87a3 3 0 1 0 4.233 4.252m.595 -3.397a3.012 3.012 0 0 0 -1.426 -1.435" /> <path d="M3 3l18 18" /> </svg>"##;
const ANKH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 13h12" /> <path d="M12 21v-8l-.422 -.211a6.472 6.472 0 0 1 -3.578 -5.789a4 4 0 1 1 8 0a6.472 6.472 0 0 1 -3.578 5.789l-.422 .211" /> </svg>"##;
const API_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 13h5" /> <path d="M12 16v-8h3a2 2 0 0 1 2 2v1a2 2 0 0 1 -2 2h-3" /> <path d="M20 8v8" /> <path d="M9 16v-5.5a2.5 2.5 0 0 0 -5 0v5.5" /> </svg>"##;
const API_APP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 15h-6.5a2.5 2.5 0 1 1 0 -5h.5" /> <path d="M15 12v6.5a2.5 2.5 0 1 1 -5 0v-.5" /> <path d="M12 9h6.5a2.5 2.5 0 1 1 0 5h-.5" /> <path d="M9 12v-6.5a2.5 2.5 0 0 1 5 0v.5" /> </svg>"##;
const API_APP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 15h-6.5a2.5 2.5 0 1 1 0 -5h.5" /> <path d="M15 15v3.5a2.5 2.5 0 1 1 -5 0v-.5" /> <path d="M13 9h5.5a2.5 2.5 0 1 1 0 5h-.5" /> <path d="M9 12v-3m.042 -3.957a2.5 2.5 0 0 1 4.958 .457v.5" /> <path d="M3 3l18 18" /> </svg>"##;
const API_BOOK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19a9 9 0 0 1 9 0a9 9 0 0 1 1.006 -.5" /> <path d="M3 6a9 9 0 0 1 9 0a9 9 0 0 1 9 0" /> <path d="M3 6v13" /> <path d="M12 6v13" /> <path d="M21 6v6" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const API_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 13h5" /> <path d="M12 16v-4m0 -4h3a2 2 0 0 1 2 2v1c0 .554 -.225 1.055 -.589 1.417m-3.411 .583h-1" /> <path d="M20 8v8" /> <path d="M9 16v-5.5a2.5 2.5 0 0 0 -5 0v5.5" /> <path d="M3 3l18 18" /> </svg>"##;
const APP_WINDOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M6 8h.01" /> <path d="M9 8h.01" /> </svg>"##;
const APPS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M4 15a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M14 15a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M14 7l6 0" /> <path d="M17 4l0 6" /> </svg>"##;
const APPS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h1a1 1 0 0 1 1 1v1m-.29 3.704a1 1 0 0 1 -.71 .296h-4a1 1 0 0 1 -1 -1v-4c0 -.276 .111 -.525 .292 -.706" /> <path d="M18 14h1a1 1 0 0 1 1 1v1m-.29 3.704a1 1 0 0 1 -.71 .296h-4a1 1 0 0 1 -1 -1v-4c0 -.276 .111 -.525 .292 -.706" /> <path d="M4 15a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M14 7h6" /> <path d="M17 4v6" /> <path d="M3 3l18 18" /> </svg>"##;
const ARMCHAIR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 11a2 2 0 0 1 2 2v2h10v-2a2 2 0 1 1 4 0v4a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-4a2 2 0 0 1 2 -2" /> <path d="M5 11v-5a3 3 0 0 1 3 -3h8a3 3 0 0 1 3 3v5" /> <path d="M6 19v2" /> <path d="M18 19v2" /> </svg>"##;
const ARMCHAIR_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 10v-4a3 3 0 0 1 3 -3h8a3 3 0 0 1 3 3v4" /> <path d="M16 15v-2a3 3 0 1 1 3 3v3h-14v-3a3 3 0 1 1 3 -3v2" /> <path d="M8 12h8" /> <path d="M7 19v2" /> <path d="M17 19v2" /> </svg>"##;
const ARMCHAIR_2_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 10v-4a3 3 0 0 1 .128 -.869m2.038 -2.013c.264 -.078 .544 -.118 .834 -.118h8a3 3 0 0 1 3 3v4" /> <path d="M16.124 12.145a3 3 0 1 1 3.756 3.724m-.88 3.131h-14v-3a3 3 0 1 1 3 -3v2" /> <path d="M8 12h4" /> <path d="M7 19v2" /> <path d="M17 19v2" /> <path d="M3 3l18 18" /> </svg>"##;
const ARMCHAIR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 13a2 2 0 1 1 4 0v4m-2 2h-14a2 2 0 0 1 -2 -2v-4a2 2 0 1 1 4 0v2h8.036" /> <path d="M5 11v-5a3 3 0 0 1 .134 -.89m1.987 -1.98a3 3 0 0 1 .879 -.13h8a3 3 0 0 1 3 3v5" /> <path d="M6 19v2" /> <path d="M18 19v2" /> <path d="M3 3l18 18" /> </svg>"##;
const ASSEMBLY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27c.7 .398 1.13 1.143 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.27 2.27 0 0 1 -2.184 0l-6.75 -4.27a2.23 2.23 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98l-.033 0" /> <path d="M15.5 9.422c.312 .18 .503 .515 .5 .876v3.277c0 .364 -.197 .7 -.515 .877l-3 1.922a1 1 0 0 1 -.97 0l-3 -1.922a1 1 0 0 1 -.515 -.876v-3.278c0 -.364 .197 -.7 .514 -.877l3 -1.79c.311 -.174 .69 -.174 1 0l3 1.79h-.014l0 .001" /> </svg>"##;
const ASSEMBLY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.376 18.377l-5.284 3.343a2.27 2.27 0 0 1 -2.184 0l-6.75 -4.27a2.23 2.23 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l1.328 -.783m2.514 -1.487l2.908 -1.71a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033c.7 .398 1.13 1.143 1.125 1.948v7.284c0 .417 -.118 .817 -.33 1.16" /> <path d="M14.855 14.855l-2.37 1.519a1 1 0 0 1 -.97 0l-3 -1.922a1 1 0 0 1 -.515 -.876v-3.278c0 -.364 .197 -.7 .514 -.877l.563 -.336m2.437 -1.454a1.03 1.03 0 0 1 1 0l3 1.79h-.014c.312 .181 .503 .516 .5 .877v1.702" /> <path d="M3 3l18 18" /> </svg>"##;
const ASSET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 15a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" /> <path d="M7 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M14.218 17.975l6.619 -12.174" /> <path d="M6.079 9.756l12.217 -6.631" /> <path d="M7 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const AUGMENTED_REALITY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> <path d="M12 12.5l4 -2.5" /> <path d="M8 10l4 2.5v4.5l4 -2.5v-4.5l-4 -2.5l-4 2.5" /> <path d="M8 10v4.5l4 2.5" /> </svg>"##;
const AUGMENTED_REALITY_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 21h-2a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v3.5" /> <path d="M17 17l-4 -2.5l4 -2.5l4 2.5v4.5l-4 2.5l0 -4.5" /> <path d="M13 14.5v4.5l4 2.5" /> <path d="M17 17l4 -2.5" /> <path d="M11 4h2" /> </svg>"##;
const AUGMENTED_REALITY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2c0 -.557 .228 -1.061 .595 -1.424" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2c.558 0 1.062 -.228 1.425 -.596" /> <path d="M12 12.5l.312 -.195m2.457 -1.536l1.231 -.769" /> <path d="M9.225 9.235l-1.225 .765l4 2.5v4.5l3.076 -1.923m.924 -3.077v-2l-4 -2.5l-.302 .189" /> <path d="M8 10v4.5l4 2.5" /> <path d="M3 3l18 18" /> </svg>"##;
const AUTH_2FA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 16h-4l3.47 -4.66a2 2 0 1 0 -3.47 -1.54" /> <path d="M10 16v-8h4" /> <path d="M10 12l3 0" /> <path d="M17 16v-6a2 2 0 0 1 4 0v6" /> <path d="M17 13l4 0" /> </svg>"##;
const AUTOMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 20.693c-.905 .628 -2.36 .292 -2.675 -1.01a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.492 .362 1.716 2.219 .674 3.03" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M17 22l5 -3l-5 -3l0 6" /> </svg>"##;
const BABY_BOTTLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 10h14" /> <path d="M12 2v2" /> <path d="M12 4a5 5 0 0 1 5 5v11a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2v-11a5 5 0 0 1 5 -5" /> </svg>"##;
const BABY_CARRIAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M2 5h2.5l1.632 4.897a6 6 0 0 0 5.693 4.103h2.675a5.5 5.5 0 0 0 0 -11h-.5v6" /> <path d="M6 9h14" /> <path d="M9 17l1 -3" /> <path d="M16 14l1 3" /> </svg>"##;
const BACKSLASH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5l10 14" /> </svg>"##;
const BAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M5.7 5.7l12.6 12.6" /> </svg>"##;
const BANDAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 12l0 .01" /> <path d="M10 12l0 .01" /> <path d="M12 10l0 .01" /> <path d="M12 14l0 .01" /> <path d="M4.5 12.5l8 -8a4.94 4.94 0 0 1 7 7l-8 8a4.94 4.94 0 0 1 -7 -7" /> </svg>"##;
const BANDAGE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12v.01" /> <path d="M12 14v.01" /> <path d="M10.513 6.487l1.987 -1.987a4.95 4.95 0 0 1 7 7l-2.018 2.018m-1.982 1.982l-4 4a4.95 4.95 0 0 1 -7 -7l4 -4" /> <path d="M3 3l18 18" /> </svg>"##;
const BARCODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7v-1a2 2 0 0 1 2 -2h2" /> <path d="M4 17v1a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v1" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-1" /> <path d="M5 11h1v2h-1l0 -2" /> <path d="M10 11l0 2" /> <path d="M14 11h1v2h-1l0 -2" /> <path d="M19 11l0 2" /> </svg>"##;
const BARCODE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7v-1c0 -.552 .224 -1.052 .586 -1.414" /> <path d="M4 17v1a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v1" /> <path d="M16 20h2c.551 0 1.05 -.223 1.412 -.584" /> <path d="M5 11h1v2h-1l0 -2" /> <path d="M10 11v2" /> <path d="M15 11v.01" /> <path d="M19 11v2" /> <path d="M3 3l18 18" /> </svg>"##;
const BATH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12h16a1 1 0 0 1 1 1v3a4 4 0 0 1 -4 4h-10a4 4 0 0 1 -4 -4v-3a1 1 0 0 1 1 -1" /> <path d="M6 12v-7a2 2 0 0 1 2 -2h3v2.25" /> <path d="M4 21l1 -1.5" /> <path d="M20 21l-1 -1.5" /> </svg>"##;
const BATH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12h4a1 1 0 0 1 1 1v3c0 .311 -.036 .614 -.103 .904m-1.61 2.378a3.982 3.982 0 0 1 -2.287 .718h-10a4 4 0 0 1 -4 -4v-3a1 1 0 0 1 1 -1h8" /> <path d="M6 12v-6m1.178 -2.824c.252 -.113 .53 -.176 .822 -.176h3v2.25" /> <path d="M4 21l1 -1.5" /> <path d="M20 21l-1 -1.5" /> <path d="M3 3l18 18" /> </svg>"##;
const BED_FLAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 11a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 13h11v-2a3 3 0 0 0 -3 -3h-8v5" /> <path d="M3 16h18" /> </svg>"##;
const BELL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a2 2 0 1 1 4 0a7 7 0 0 1 4 6v3a4 4 0 0 0 2 3h-16a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6" /> <path d="M9 17v1a3 3 0 0 0 6 0v-1" /> </svg>"##;
const BELL_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 17h-9.5a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6v1" /> <path d="M9 17v1a3 3 0 0 0 4.368 2.67" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const BELL_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 17h-8.5a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6v1" /> <path d="M9 17v1a3 3 0 0 0 3 3" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const BELL_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 17h-7.5a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6v3c.016 .129 .037 .256 .065 .382" /> <path d="M9 17v1a3 3 0 0 0 2.502 2.959" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const BELL_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 17h-7.5a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6v2" /> <path d="M9 17v1a3 3 0 0 0 2.498 2.958" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const BELL_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17h-8a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6v.5" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> <path d="M9 17v1a3 3 0 0 0 3 3" /> </svg>"##;
const BELL_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 17h-9a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 3.911 5.17" /> <path d="M9 17v1a3 3 0 0 0 4.02 2.822" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const BELL_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 17h-8.5a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6v1" /> <path d="M9 17v1a3 3 0 0 0 3.518 2.955" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const BELL_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 17h-11a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6v1.5" /> <path d="M9 17v1a3 3 0 0 0 6 0v-1" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const BELL_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 17h-6a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6" /> <path d="M9 17v1c0 1.408 .97 2.59 2.28 2.913" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const BELL_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 17h-8.5a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6v3c.047 .386 .149 .758 .3 1.107" /> <path d="M9 17v1a3 3 0 0 0 3.504 2.958" /> <path d="M16 19h6" /> </svg>"##;
const BELL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.346 5.353c.21 -.129 .428 -.246 .654 -.353a2 2 0 1 1 4 0a7 7 0 0 1 4 6v3m-1 3h-13a4 4 0 0 0 2 -3v-3a6.996 6.996 0 0 1 1.273 -3.707" /> <path d="M9 17v1a3 3 0 0 0 6 0v-1" /> <path d="M3 3l18 18" /> </svg>"##;
const BELL_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 17h-9a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6v2" /> <path d="M9 17v1a3 3 0 0 0 4.022 2.821" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const BELL_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17h-8a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6" /> <path d="M9 17v1a3 3 0 0 0 3.64 2.931" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const BELL_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 17h-8.5a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6v1" /> <path d="M9 17v1a3 3 0 0 0 3.51 2.957" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const BELL_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 17h-9.5a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6" /> <path d="M9 17v1a3 3 0 0 0 5.914 .716" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const BELL_RINGING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a2 2 0 0 1 4 0a7 7 0 0 1 4 6v3a4 4 0 0 0 2 3h-16a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6" /> <path d="M9 17v1a3 3 0 0 0 6 0v-1" /> <path d="M21 6.727a11.05 11.05 0 0 0 -2.794 -3.727" /> <path d="M3 6.727a11.05 11.05 0 0 1 2.792 -3.727" /> </svg>"##;
const BELL_RINGING_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.364 4.636a2 2 0 0 1 0 2.828a7 7 0 0 1 -1.414 7.072l-2.122 2.12a4 4 0 0 0 -.707 3.536l-11.313 -11.312a4 4 0 0 0 3.535 -.707l2.121 -2.123a7 7 0 0 1 7.072 -1.414a2 2 0 0 1 2.828 0" /> <path d="M7.343 12.414l-.707 .707a3 3 0 0 0 4.243 4.243l.707 -.707" /> </svg>"##;
const BELL_SCHOOL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" /> <path d="M13.5 15h.5a2 2 0 0 1 2 2v1a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-1a2 2 0 0 1 2 -2h.5" /> <path d="M16 17a5.698 5.698 0 0 0 4.467 -7.932l-.467 -1.068" /> <path d="M10 10v.01" /> <path d="M19 8a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const BELL_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 17h-7a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6" /> <path d="M9 17v1a3 3 0 0 0 2.685 2.984" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const BELL_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 17h-8.5a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6v2" /> <path d="M9 17v1a3 3 0 0 0 3 3" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const BELL_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 17h-5.5a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 3.88 5" /> <path d="M9 17v1a3 3 0 0 0 2.15 2.878" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const BELL_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 17h-8.5a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6v1" /> <path d="M9 17v1a3 3 0 0 0 3.49 2.96" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const BELL_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 17h-9a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6a2 2 0 1 1 4 0a7 7 0 0 1 4 6v2" /> <path d="M9 17v1a3 3 0 0 0 4.194 2.753" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const BELL_Z_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a2 2 0 1 1 4 0a7 7 0 0 1 4 6v3a4 4 0 0 0 2 3h-16a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6" /> <path d="M9 17v1a3 3 0 0 0 6 0v-1" /> <path d="M10 9h4l-4 4h4" /> </svg>"##;
const BIOHAZARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M11.939 14c0 .173 .048 .351 .056 .533l0 .217a4.75 4.75 0 0 1 -4.533 4.745l-.217 0m-4.75 -4.75a4.75 4.75 0 0 1 7.737 -3.693m6.513 8.443a4.75 4.75 0 0 1 -4.69 -5.503l-.06 0m1.764 -2.944a4.75 4.75 0 0 1 7.731 3.477l0 .217m-11.195 -3.813a4.75 4.75 0 0 1 -1.828 -7.624l.164 -.172m6.718 0a4.75 4.75 0 0 1 -1.665 7.798" /> </svg>"##;
const BIOHAZARD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.586 10.586a2 2 0 1 0 2.836 2.82" /> <path d="M11.939 14c0 .173 .048 .351 .056 .533v.217a4.75 4.75 0 0 1 -4.533 4.745h-.217" /> <path d="M2.495 14.745a4.75 4.75 0 0 1 7.737 -3.693" /> <path d="M16.745 19.495a4.75 4.75 0 0 1 -4.69 -5.503h-.06" /> <path d="M14.533 10.538a4.75 4.75 0 0 1 6.957 3.987v.217" /> <path d="M10.295 10.929a4.75 4.75 0 0 1 -2.988 -3.64m.66 -3.324a4.75 4.75 0 0 1 .5 -.66l.164 -.172" /> <path d="M15.349 3.133a4.75 4.75 0 0 1 -.836 7.385" /> <path d="M3 3l18 18" /> </svg>"##;
const BLIND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 4a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M4 21l3 -4" /> <path d="M13 21l-2 -4l-3 -3l1 -6" /> <path d="M3 12l2 -3l4 -1l6 4" /> <path d="M16.5 14l3.5 7" /> </svg>"##;
const BLOB_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.897 20.188c1.67 .752 3.896 .812 6.103 .812s4.434 -.059 6.104 -.812c.868 -.392 1.614 -.982 2.133 -1.856c.514 -.865 .763 -1.94 .763 -3.234c0 -2.577 -.983 -5.315 -2.557 -7.416c-1.57 -2.094 -3.833 -3.682 -6.443 -3.682s-4.873 1.588 -6.443 3.682c-1.574 2.101 -2.557 4.84 -2.557 7.416c0 1.295 .249 2.369 .763 3.234c.519 .874 1.265 1.464 2.134 1.856" /> </svg>"##;
const BLOCKS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 4a1 1 0 0 1 1 -1h5a1 1 0 0 1 1 1v5a1 1 0 0 1 -1 1h-5a1 1 0 0 1 -1 -1l0 -5" /> <path d="M3 14h12a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h3a2 2 0 0 1 2 2v12" /> </svg>"##;
const BODY_SCAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 8a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M10 17v-1a2 2 0 1 1 4 0v1" /> <path d="M8 10q 1 1 2 1h4q 1 0 2 -1" /> <path d="M12 11v3" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const BOT_ID_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 10.5c0 -.828 .746 -1.5 1.667 -1.5h6.666c.92 0 1.667 .672 1.667 1.5v3c0 .828 -.746 1.5 -1.667 1.5h-6.666c-.92 0 -1.667 -.672 -1.667 -1.5v-3" /> <path d="M12 7v2" /> <path d="M10 12v.01" /> <path d="M14 12v.01" /> <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const BRACES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 4a2 2 0 0 0 -2 2v3a2 3 0 0 1 -2 3a2 3 0 0 1 2 3v3a2 2 0 0 0 2 2" /> <path d="M17 4a2 2 0 0 1 2 2v3a2 3 0 0 0 2 3a2 3 0 0 0 -2 3v3a2 2 0 0 1 -2 2" /> </svg>"##;
const BRACES_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.176 5.177c-.113 .251 -.176 .53 -.176 .823v3c0 1.657 -.895 3 -2 3c1.105 0 2 1.343 2 3v3a2 2 0 0 0 2 2" /> <path d="M17 4a2 2 0 0 1 2 2v3c0 1.657 .895 3 2 3c-1.105 0 -2 1.343 -2 3m-.176 3.821a2 2 0 0 1 -1.824 1.179" /> <path d="M3 3l18 18" /> </svg>"##;
const BRACKETS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h-3v16h3" /> <path d="M16 4h3v16h-3" /> </svg>"##;
const BRACKETS_CONTAIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 4h-4v16h4" /> <path d="M17 4h4v16h-4" /> <path d="M8 16h.01" /> <path d="M12 16h.01" /> <path d="M16 16h.01" /> </svg>"##;
const BRACKETS_CONTAIN_END_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 4h4v16h-4" /> <path d="M5 16h.01" /> <path d="M9 16h.01" /> <path d="M13 16h.01" /> </svg>"##;
const BRACKETS_CONTAIN_START_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 4h-4v16h4" /> <path d="M18 16h-.01" /> <path d="M14 16h-.01" /> <path d="M10 16h-.01" /> </svg>"##;
const BRACKETS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5v15h3" /> <path d="M16 4h3v11m0 4v1h-3" /> <path d="M3 3l18 18" /> </svg>"##;
const BRAILLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 5a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M7 5a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M7 19a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M16 12h.01" /> <path d="M8 12h.01" /> <path d="M16 19h.01" /> </svg>"##;
const BRAIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.5 13a3.5 3.5 0 0 0 -3.5 3.5v1a3.5 3.5 0 0 0 7 0v-1.8" /> <path d="M8.5 13a3.5 3.5 0 0 1 3.5 3.5v1a3.5 3.5 0 0 1 -7 0v-1.8" /> <path d="M17.5 16a3.5 3.5 0 0 0 0 -7h-.5" /> <path d="M19 9.3v-2.8a3.5 3.5 0 0 0 -7 0" /> <path d="M6.5 16a3.5 3.5 0 0 1 0 -7h.5" /> <path d="M5 9.3v-2.8a3.5 3.5 0 0 1 7 0v10" /> </svg>"##;
const BUG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 9v-1a3 3 0 0 1 6 0v1" /> <path d="M8 9h8a6 6 0 0 1 1 3v3a5 5 0 0 1 -10 0v-3a6 6 0 0 1 1 -3" /> <path d="M3 13l4 0" /> <path d="M17 13l4 0" /> <path d="M12 20l0 -6" /> <path d="M4 19l3.35 -2" /> <path d="M20 19l-3.35 -2" /> <path d="M4 7l3.75 2.4" /> <path d="M20 7l-3.75 2.4" /> </svg>"##;
const BUG_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.884 5.873a3 3 0 0 1 5.116 2.127v1" /> <path d="M13 9h3a6 6 0 0 1 1 3v1m-.298 3.705a5 5 0 0 1 -9.702 -1.705v-3a6 6 0 0 1 1 -3h1" /> <path d="M3 13h4" /> <path d="M17 13h4" /> <path d="M12 20v-6" /> <path d="M4 19l3.35 -2" /> <path d="M4 7l3.75 2.4" /> <path d="M20 7l-3.75 2.4" /> <path d="M3 3l18 18" /> </svg>"##;
const BUILDING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21l18 0" /> <path d="M9 8l1 0" /> <path d="M9 12l1 0" /> <path d="M9 16l1 0" /> <path d="M14 8l1 0" /> <path d="M14 12l1 0" /> <path d="M14 16l1 0" /> <path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16" /> </svg>"##;
const BUILDING_AIRPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.59 7h8.82a1 1 0 0 1 .902 1.433l-1.44 3a1 1 0 0 1 -.901 .567h-5.942a1 1 0 0 1 -.901 -.567l-1.44 -3a1 1 0 0 1 .901 -1.433" /> <path d="M6 7l-.78 -2.342a.5 .5 0 0 1 .473 -.658h4.612a.5 .5 0 0 1 .475 .658l-.78 2.342" /> <path d="M8 2v2" /> <path d="M6 12v9h4v-9" /> <path d="M3 21h18" /> <path d="M22 5h-6l-1 -1" /> <path d="M18 3l2 2l-2 2" /> <path d="M10 17h7a2 2 0 0 1 2 2v2" /> </svg>"##;
const BUILDING_ARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21l18 0" /> <path d="M4 21v-15a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v15" /> <path d="M9 21v-8a3 3 0 0 1 6 0v8" /> </svg>"##;
const BUILDING_BANK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21l18 0" /> <path d="M3 10l18 0" /> <path d="M5 6l7 -3l7 3" /> <path d="M4 10l0 11" /> <path d="M20 10l0 11" /> <path d="M8 14l0 3" /> <path d="M12 14l0 3" /> <path d="M16 14l0 3" /> </svg>"##;
const BUILDING_BRIDGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 5l0 14" /> <path d="M18 5l0 14" /> <path d="M2 15l20 0" /> <path d="M3 8a7.5 7.5 0 0 0 3 -2a6.5 6.5 0 0 0 12 0a7.5 7.5 0 0 0 3 2" /> <path d="M12 10l0 5" /> </svg>"##;
const BUILDING_BRIDGE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 7h12a2 2 0 0 1 2 2v9a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-2a4 4 0 0 0 -8 0v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-9a2 2 0 0 1 2 -2" /> </svg>"##;
const BUILDING_BROADCAST_TOWER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M16.616 13.924a5 5 0 1 0 -9.23 0" /> <path d="M20.307 15.469a9 9 0 1 0 -16.615 0" /> <path d="M9 21l3 -9l3 9" /> <path d="M10 19h4" /> </svg>"##;
const BUILDING_BURJ_AL_ARAB_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21h18" /> <path d="M7 21v-18" /> <path d="M7 4c5.675 .908 10 5.613 10 11.28a11 11 0 0 1 -1.605 5.72" /> <path d="M5 9h12" /> <path d="M7 13h4" /> <path d="M7 17h4" /> </svg>"##;
const BUILDING_CAROUSEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 12a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" /> <path d="M3 8a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 4a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 8a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 16a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 16a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M8 22l4 -10l4 10" /> </svg>"##;
const BUILDING_CASTLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 19v-2a3 3 0 0 0 -6 0v2a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-14h4v3h3v-3h4v3h3v-3h4v14a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1" /> <path d="M3 11l18 0" /> </svg>"##;
const BUILDING_CHURCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21l18 0" /> <path d="M10 21v-4a2 2 0 0 1 4 0v4" /> <path d="M10 5l4 0" /> <path d="M12 3l0 5" /> <path d="M6 21v-7m-2 2l8 -8l8 8m-2 -2v7" /> </svg>"##;
const BUILDING_CIRCUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 11h16" /> <path d="M12 6.5c0 1 -5 4.5 -8 4.5" /> <path d="M12 6.5c0 1 5 4.5 8 4.5" /> <path d="M6 11c-.333 5.333 -1 8.667 -2 10h4c1 0 4 -4 4 -9v-1" /> <path d="M18 11c.333 5.333 1 8.667 2 10h-4c-1 0 -4 -4 -4 -9v-1" /> <path d="M12 7v-4l2 1h-2" /> </svg>"##;
const BUILDING_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21h9" /> <path d="M9 8h1" /> <path d="M9 12h1" /> <path d="M9 16h1" /> <path d="M14 8h1" /> <path d="M14 12h1" /> <path d="M5 21v-16c0 -.53 .211 -1.039 .586 -1.414c.375 -.375 .884 -.586 1.414 -.586h10c.53 0 1.039 .211 1.414 .586c.375 .375 .586 .884 .586 1.414v7" /> <path d="M16 18c0 .53 .211 1.039 .586 1.414c.375 .375 .884 .586 1.414 .586c.53 0 1.039 -.211 1.414 -.586c.375 -.375 .586 -.884 .586 -1.414c0 -.53 -.211 -1.039 -.586 -1.414c-.375 -.375 -.884 -.586 -1.414 -.586c-.53 0 -1.039 .211 -1.414 .586c-.375 .375 -.586 .884 -.586 1.414" /> <path d="M18 14.5v1.5" /> <path d="M18 20v1.5" /> <path d="M21.032 16.25l-1.299 .75" /> <path d="M16.27 19l-1.3 .75" /> <path d="M14.97 16.25l1.3 .75" /> <path d="M19.733 19l1.3 .75" /> </svg>"##;
const BUILDING_COMMUNITY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9l5 5v7h-5v-4m0 4h-5v-7l5 -5m1 1v-6a1 1 0 0 1 1 -1h10a1 1 0 0 1 1 1v17h-8" /> <path d="M13 7l0 .01" /> <path d="M17 7l0 .01" /> <path d="M17 11l0 .01" /> <path d="M17 15l0 .01" /> </svg>"##;
const BUILDING_COTTAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21l18 0" /> <path d="M4 21v-11l2.5 -4.5l5.5 -2.5l5.5 2.5l2.5 4.5v11" /> <path d="M10 9a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M9 21v-5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v5" /> </svg>"##;
const BUILDING_EIFFEL_TOWER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 4c0 4.889 -2.292 12.111 -5 17" /> <path d="M13 4c0 4.889 2.292 12.111 5 17" /> <path d="M3 21h18" /> <path d="M8 14h8" /> <path d="M9 10h6" /> <path d="M10 4h4" /> <path d="M12 2v1.778" /> <path d="M10 21s.27 -1.406 .667 -2c.333 -.5 .666 -1 1.333 -1s1 .5 1.333 1c.448 .672 .667 2 .667 2" /> </svg>"##;
const BUILDING_ESTATE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21h18" /> <path d="M19 21v-4" /> <path d="M19 17a2 2 0 0 0 2 -2v-2a2 2 0 1 0 -4 0v2a2 2 0 0 0 2 2" /> <path d="M14 21v-14a3 3 0 0 0 -3 -3h-4a3 3 0 0 0 -3 3v14" /> <path d="M9 17v4" /> <path d="M8 13h2" /> <path d="M8 9h2" /> </svg>"##;
const BUILDING_FACTORY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 21c1.147 -4.02 1.983 -8.027 2 -12h6c.017 3.973 .853 7.98 2 12" /> <path d="M12.5 13h4.5c.025 2.612 .894 5.296 2 8" /> <path d="M9 5a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1" /> <path d="M3 21l19 0" /> </svg>"##;
const BUILDING_FACTORY_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21h18" /> <path d="M5 21v-12l5 4v-4l5 4h4" /> <path d="M19 21v-8l-1.436 -9.574a.5 .5 0 0 0 -.495 -.426h-1.145a.5 .5 0 0 0 -.494 .418l-1.43 8.582" /> <path d="M9 17h1" /> <path d="M14 17h1" /> </svg>"##;
const BUILDING_FORTRESS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 21h1a1 1 0 0 0 1 -1v-1a3 3 0 0 1 6 0m3 2h1a1 1 0 0 0 1 -1v-15l-3 -2l-3 2v6h-4v-6l-3 -2l-3 2v15a1 1 0 0 0 1 1h2m8 -2v1a1 1 0 0 0 1 1h2" /> <path d="M7 7v.01" /> <path d="M7 10v.01" /> <path d="M7 13v.01" /> <path d="M17 7v.01" /> <path d="M17 10v.01" /> <path d="M17 13v.01" /> </svg>"##;
const BUILDING_HOSPITAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21l18 0" /> <path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16" /> <path d="M9 21v-4a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v4" /> <path d="M10 9l4 0" /> <path d="M12 7l0 4" /> </svg>"##;
const BUILDING_LIGHTHOUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3l2 3l2 15h-8l2 -15l2 -3" /> <path d="M8 9l8 0" /> <path d="M3 11l2 -2l-2 -2" /> <path d="M21 11l-2 -2l2 -2" /> </svg>"##;
const BUILDING_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21h9" /> <path d="M9 8h1" /> <path d="M9 12h1" /> <path d="M9 16h1" /> <path d="M14 8h1" /> <path d="M14 12h1" /> <path d="M5 21v-16c0 -.53 .211 -1.039 .586 -1.414c.375 -.375 .884 -.586 1.414 -.586h10c.53 0 1.039 .211 1.414 .586c.375 .375 .586 .884 .586 1.414v7" /> <path d="M16 19h6" /> </svg>"##;
const BUILDING_MONUMENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 18l2 -13l2 -2l2 2l2 13" /> <path d="M5 21v-3h14v3" /> <path d="M3 21l18 0" /> </svg>"##;
const BUILDING_MOSQUE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21h7v-2a2 2 0 1 1 4 0v2h7" /> <path d="M4 21v-10" /> <path d="M20 21v-10" /> <path d="M4 16h3v-3h10v3h3" /> <path d="M17 13a5 5 0 0 0 -10 0" /> <path d="M21 10.5c0 -.329 -.077 -.653 -.224 -.947l-.776 -1.553l-.776 1.553a2.118 2.118 0 0 0 -.224 .947a.5 .5 0 0 0 .5 .5h1a.5 .5 0 0 0 .5 -.5" /> <path d="M5 10.5c0 -.329 -.077 -.653 -.224 -.947l-.776 -1.553l-.776 1.553a2.118 2.118 0 0 0 -.224 .947a.5 .5 0 0 0 .5 .5h1a.5 .5 0 0 0 .5 -.5" /> <path d="M12 2a2 2 0 1 0 2 2" /> <path d="M12 6v2" /> </svg>"##;
const BUILDING_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21h18" /> <path d="M9 12h1" /> <path d="M9 16h1" /> <path d="M14 8h1" /> <path d="M14 16h1" /> <path d="M5 21v-16" /> <path d="M7 3h10c1 0 2 1 2 2v10" /> <path d="M19 19v2" /> <path d="M3 3l18 18" /> </svg>"##;
const BUILDING_PAVILION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21h7v-3a2 2 0 0 1 4 0v3h7" /> <path d="M6 21l0 -9" /> <path d="M18 21l0 -9" /> <path d="M6 12h12a3 3 0 0 0 3 -3a9 8 0 0 1 -9 -6a9 8 0 0 1 -9 6a3 3 0 0 0 3 3" /> </svg>"##;
const BUILDING_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21h9" /> <path d="M9 8h1" /> <path d="M9 12h1" /> <path d="M9 16h1" /> <path d="M14 8h1" /> <path d="M14 12h1" /> <path d="M5 21v-16c0 -.53 .211 -1.039 .586 -1.414c.375 -.375 .884 -.586 1.414 -.586h10c.53 0 1.039 .211 1.414 .586c.375 .375 .586 .884 .586 1.414v7" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const BUILDING_SKYSCRAPER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21l18 0" /> <path d="M5 21v-14l8 -4v18" /> <path d="M19 21v-10l-6 -4" /> <path d="M9 9l0 .01" /> <path d="M9 12l0 .01" /> <path d="M9 15l0 .01" /> <path d="M9 18l0 .01" /> </svg>"##;
const BUILDING_STADIUM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12a8 2 0 1 0 16 0a8 2 0 1 0 -16 0" /> <path d="M4 12v7c0 .94 2.51 1.785 6 2v-3h4v3c3.435 -.225 6 -1.07 6 -2v-7" /> <path d="M15 6h4v-3h-4v7" /> <path d="M7 6h4v-3h-4v7" /> </svg>"##;
const BUILDING_STORE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21l18 0" /> <path d="M3 7v1a3 3 0 0 0 6 0v-1m0 1a3 3 0 0 0 6 0v-1m0 1a3 3 0 0 0 6 0v-1h-18l2 -4h14l2 4" /> <path d="M5 21l0 -10.15" /> <path d="M19 21l0 -10.15" /> <path d="M9 21v-4a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v4" /> </svg>"##;
const BUILDING_TUNNEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21h14a2 2 0 0 0 2 -2v-7a9 9 0 0 0 -18 0v7a2 2 0 0 0 2 2" /> <path d="M8 21v-9a4 4 0 1 1 8 0v9" /> <path d="M3 17h4" /> <path d="M17 17h4" /> <path d="M21 12h-4" /> <path d="M7 12h-4" /> <path d="M12 3v5" /> <path d="M6 6l3 3" /> <path d="M15 9l3 -3l-3 3" /> </svg>"##;
const BUILDING_WAREHOUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21v-13l9 -4l9 4v13" /> <path d="M13 13h4v8h-10v-6h6" /> <path d="M13 21v-9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v3" /> </svg>"##;
const BUILDING_WIND_TURBINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 11a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 11v-2.573c0 -.18 .013 -.358 .04 -.536l.716 -4.828c.064 -.597 .597 -1.063 1.244 -1.063s1.18 .466 1.244 1.063l.716 4.828c.027 .178 .04 .357 .04 .536v2.573" /> <path d="M13.01 9.28l2.235 1.276c.156 .09 .305 .19 .446 .3l3.836 2.911c.487 .352 .624 1.04 .3 1.596c-.325 .556 -1 .782 -1.548 .541l-4.555 -1.68a3.624 3.624 0 0 1 -.486 -.231l-2.235 -1.277" /> <path d="M13 12.716l-2.236 1.277a3.624 3.624 0 0 1 -.485 .23l-4.555 1.681c-.551 .241 -1.223 .015 -1.548 -.54c-.324 -.557 -.187 -1.245 .3 -1.597l3.836 -2.91a3.41 3.41 0 0 1 .446 -.3l2.235 -1.277" /> <path d="M7 21h10" /> <path d="M10 21l1 -7" /> <path d="M13 14l1 7" /> </svg>"##;
const BUILDINGS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 21v-15c0 -1 1 -2 2 -2h5c1 0 2 1 2 2v15" /> <path d="M16 8h2c1 0 2 1 2 2v11" /> <path d="M3 21h18" /> <path d="M10 12v.01" /> <path d="M10 16v.01" /> <path d="M10 8v.01" /> <path d="M7 12v.01" /> <path d="M7 16v.01" /> <path d="M7 8v.01" /> <path d="M17 12v.01" /> <path d="M17 16v.01" /> </svg>"##;
const CALENDAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M11 15h1" /> <path d="M12 15v3" /> </svg>"##;
const CALENDAR_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 21h-7.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v5" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const CALENDAR_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v5" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const CALENDAR_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-5.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const CALENDAR_CLOCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.5 21h-4.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v3" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h10" /> <path d="M14 18a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M18 16.5v1.5l.5 .5" /> </svg>"##;
const CALENDAR_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-5.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const CALENDAR_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-6a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v5" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const CALENDAR_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-7a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v3" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h12.5" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const CALENDAR_DOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v5" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const CALENDAR_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v5" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> </svg>"##;
const CALENDAR_DUE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const CALENDAR_EVENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M16 3l0 4" /> <path d="M8 3l0 4" /> <path d="M4 11l16 0" /> <path d="M8 15h2v2h-2l0 -2" /> </svg>"##;
const CALENDAR_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21h-9a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v5" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M11 15h1" /> <path d="M12 15v3" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const CALENDAR_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-5.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v4" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const CALENDAR_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v8" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M16 19h6" /> </svg>"##;
const CALENDAR_MONTH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M8 14v4" /> <path d="M12 14v4" /> <path d="M16 14v4" /> </svg>"##;
const CALENDAR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h9a2 2 0 0 1 2 2v9m-.184 3.839a2 2 0 0 1 -1.816 1.161h-12a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 1.158 -1.815" /> <path d="M16 3v4" /> <path d="M8 3v1" /> <path d="M4 11h7m4 0h5" /> <path d="M3 3l18 18" /> </svg>"##;
const CALENDAR_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-7a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const CALENDAR_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v4" /> <path d="M16 3v4" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> <path d="M8 3v4" /> <path d="M4 11h16" /> </svg>"##;
const CALENDAR_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v5" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const CALENDAR_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21h-9a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v4" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const CALENDAR_REPEAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v3" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h12" /> <path d="M20 14l2 2h-3" /> <path d="M20 18l2 -2" /> <path d="M19 16a3 3 0 1 0 2 5.236" /> </svg>"##;
const CALENDAR_SAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12m12 -4v4m-8 -4v4m-4 4h16m-9.995 3h.01m3.99 0h.01" /> <path d="M10 18a3.5 3.5 0 0 1 4 0" /> </svg>"##;
const CALENDAR_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-5.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v4.5" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const CALENDAR_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-6a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const CALENDAR_SMILE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12m12 -4v4m-8 -4v4m-4 4h16m-9.995 3h.01m3.99 0h.01" /> <path d="M10.005 17a3.5 3.5 0 0 0 4 0" /> </svg>"##;
const CALENDAR_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 21h-5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v3.5" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h11" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const CALENDAR_STATS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.795 21h-6.795a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v4" /> <path d="M18 14v4h4" /> <path d="M14 18a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M15 3v4" /> <path d="M7 3v4" /> <path d="M3 11h16" /> </svg>"##;
const CALENDAR_TIME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.795 21h-6.795a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v4" /> <path d="M14 18a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M15 3v4" /> <path d="M7 3v4" /> <path d="M3 11h16" /> <path d="M18 16.496v1.504l1 1" /> </svg>"##;
const CALENDAR_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v5" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const CALENDAR_USER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-6a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v4.5" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M17 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M22 22a2 2 0 0 0 -2 -2h-2a2 2 0 0 0 -2 2" /> </svg>"##;
const CALENDAR_WEEK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M7 14h.013" /> <path d="M10.01 14h.005" /> <path d="M13.01 14h.005" /> <path d="M16.015 14h.005" /> <path d="M13.015 17h.005" /> <path d="M7.01 17h.005" /> <path d="M10.01 17h.005" /> </svg>"##;
const CALENDAR_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-7a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6.5" /> <path d="M16 3v4" /> <path d="M8 3v4" /> <path d="M4 11h16" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const CANARY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20v-2" /> <path d="M15 8.01v.01" /> <path d="M3 17l8 -8v-1a4 4 0 1 1 8 0h2l-2 2v1a7 7 0 0 1 -13.215 3.223" /> </svg>"##;
const CANE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 21l6.324 -11.69c.54 -.974 1.756 -4.104 -1.499 -5.762c-3.255 -1.657 -5.175 .863 -5.825 2.032" /> </svg>"##;
const CAR_GARAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 20a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M15 20a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M5 20h-2v-6l2 -5h9l4 5h1a2 2 0 0 1 2 2v4h-2m-4 0h-6m-6 -6h15m-6 0v-5" /> <path d="M3 6l9 -4l9 4" /> </svg>"##;
const CE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 6a6 6 0 1 0 0 12" /> <path d="M21 6a6 6 0 1 0 0 12" /> <path d="M15 12h6" /> </svg>"##;
const CE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.53 6.53a6.001 6.001 0 0 0 2.47 11.47" /> <path d="M21 6a6 6 0 0 0 -5.927 5.061l.927 .939" /> <path d="M16 12h5" /> <path d="M3 3l18 18" /> </svg>"##;
const CHALKBOARD_TEACHER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 19h-3a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v11a1 1 0 0 1 -1 1" /> <path d="M12 14a2 2 0 1 0 4.001 -.001a2 2 0 0 0 -4.001 .001" /> <path d="M17 19a2 2 0 0 0 -2 -2h-2a2 2 0 0 0 -2 2" /> </svg>"##;
const CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12l5 5l10 -10" /> </svg>"##;
const CHECKBOX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11l3 3l8 -8" /> <path d="M20 12v6a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h9" /> </svg>"##;
const CHECKLIST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.615 20h-2.615a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8" /> <path d="M14 19l2 2l4 -4" /> <path d="M9 8h4" /> <path d="M9 12h2" /> </svg>"##;
const CHECKS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 12l5 5l10 -10" /> <path d="M2 12l5 5m5 -5l5 -5" /> </svg>"##;
const CHECKUP_LIST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-12a2 2 0 0 0 -2 -2h-2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2" /> <path d="M9 14h.01" /> <path d="M9 17h.01" /> <path d="M12 16l1 1l3 -3" /> </svg>"##;
const CLEAR_ALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 6h12" /> <path d="M6 12h12" /> <path d="M4 18h12" /> </svg>"##;
const CLICK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12l3 0" /> <path d="M12 3l0 3" /> <path d="M7.8 7.8l-2.2 -2.2" /> <path d="M16.2 7.8l2.2 -2.2" /> <path d="M7.8 16.2l-2.2 2.2" /> <path d="M12 12l9 3l-4 2l-2 4l-3 -9" /> </svg>"##;
const CLOCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M12 7v5l3 3" /> </svg>"##;
const CLOCK_12_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 0 0 9 9m9 -9a9 9 0 1 0 -18 0" /> <path d="M12 7v5l.5 .5" /> <path d="M18 15h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2" /> <path d="M15 21v-6" /> </svg>"##;
const CLOCK_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1l0 -14" /> <path d="M12 7v5l3 3" /> <path d="M4 12h1" /> <path d="M19 12h1" /> <path d="M12 19v1" /> </svg>"##;
const CLOCK_24_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 0 0 5.998 8.485m12.002 -8.485a9 9 0 1 0 -18 0" /> <path d="M12 7v5" /> <path d="M12 15h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2" /> <path d="M18 15v2a1 1 0 0 0 1 1h1" /> <path d="M21 15v6" /> </svg>"##;
const CLOCK_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 21v-6m2 0v-1.5m0 9v-1.5m-2 -3h3m-1 0h.5a1.5 1.5 0 0 1 0 3h-3.5m3 -3h.5a1.5 1.5 0 0 0 0 -3h-3.5" /> <path d="M20.866 10.45a9 9 0 1 0 -7.815 10.488" /> <path d="M12 7v5l1.5 1.5" /> </svg>"##;
const CLOCK_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.984 12.53a9 9 0 1 0 -7.552 8.355" /> <path d="M12 7v5l3 3" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const CLOCK_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.997 12.25a9 9 0 1 0 -8.718 8.745" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> <path d="M12 7v5l2 2" /> </svg>"##;
const CLOCK_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.942 13.021a9 9 0 1 0 -9.407 7.967" /> <path d="M12 7v5l3 3" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const CLOCK_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.931 13.111a9 9 0 1 0 -9.453 7.874" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> <path d="M12 7v5l2 2" /> </svg>"##;
const CLOCK_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -9.002 9" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> <path d="M12 7v5l2 2" /> </svg>"##;
const CLOCK_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.866 10.45a9 9 0 1 0 -7.815 10.488" /> <path d="M12 7v5l1.5 1.5" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const CLOCK_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.984 12.535a9 9 0 1 0 -8.431 8.448" /> <path d="M12 7v5l3 3" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const CLOCK_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -9.972 8.948c.32 .034 .644 .052 .972 .052" /> <path d="M12 7v5l2 2" /> <path d="M18.42 15.61a2.1 2.1 0 0 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const CLOCK_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.986 12.502a9 9 0 1 0 -5.973 7.98" /> <path d="M12 7v5l3 3" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const CLOCK_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.956 11.107a9 9 0 1 0 -9.579 9.871" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> <path d="M12 7v5l.5 .5" /> </svg>"##;
const CLOCK_HOUR_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 7v5" /> <path d="M12 12l2 -3" /> </svg>"##;
const CLOCK_HOUR_10_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 12l-3 -2" /> <path d="M12 7v5" /> </svg>"##;
const CLOCK_HOUR_11_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 12l-2 -3" /> <path d="M12 7v5" /> </svg>"##;
const CLOCK_HOUR_12_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 7v5" /> </svg>"##;
const CLOCK_HOUR_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 12l3 -2" /> <path d="M12 7v5" /> </svg>"##;
const CLOCK_HOUR_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 12h3.5" /> <path d="M12 7v5" /> </svg>"##;
const CLOCK_HOUR_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 12l3 2" /> <path d="M12 7v5" /> </svg>"##;
const CLOCK_HOUR_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 12l2 3" /> <path d="M12 7v5" /> </svg>"##;
const CLOCK_HOUR_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 12v3.5" /> <path d="M12 7v5" /> </svg>"##;
const CLOCK_HOUR_7_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 12l-2 3" /> <path d="M12 7v5" /> </svg>"##;
const CLOCK_HOUR_8_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 12l-3 2" /> <path d="M12 7v5" /> </svg>"##;
const CLOCK_HOUR_9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 12h-3.5" /> <path d="M12 7v5" /> </svg>"##;
const CLOCK_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.477 15.022a9 9 0 1 0 -7.998 5.965" /> <path d="M12 7v5l3 3" /> <path d="M16 19h6" /> </svg>"##;
const CLOCK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.633 5.64a9 9 0 1 0 12.735 12.72m1.674 -2.32a9 9 0 0 0 -12.082 -12.082" /> <path d="M12 7v1" /> <path d="M3 3l18 18" /> </svg>"##;
const CLOCK_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.942 13.018a9 9 0 1 0 -7.909 7.922" /> <path d="M12 7v5l2 2" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const CLOCK_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.971 11.278a9 9 0 1 0 -8.313 9.698" /> <path d="M12 7v5l1.5 1.5" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const CLOCK_PLAY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 7v5l2 2" /> <path d="M17 22l5 -3l-5 -3l0 6" /> <path d="M13.017 20.943a9 9 0 1 1 7.831 -7.292" /> </svg>"##;
const CLOCK_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.984 12.535a9 9 0 1 0 -8.468 8.45" /> <path d="M16 19h6" /> <path d="M19 16v6" /> <path d="M12 7v5l3 3" /> </svg>"##;
const CLOCK_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.975 11.33a9 9 0 1 0 -5.717 9.06" /> <path d="M12 7v5l2 2" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const CLOCK_RECORD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12.3a9 9 0 1 0 -8.683 8.694" /> <path d="M12 7v5l2 2" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const CLOCK_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.993 11.646a9 9 0 1 0 -9.318 9.348" /> <path d="M12 7v5l1 1" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const CLOCK_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.943 13.016a9 9 0 1 0 -8.915 7.984" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> <path d="M12 7v5l2 2" /> </svg>"##;
const CLOCK_SHIELD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -8.98 9" /> <path d="M12 7v5l1 1" /> <path d="M22 16c0 4 -2.5 6 -3.5 6s-3.5 -2 -3.5 -6c1 0 2.5 -.5 3.5 -1.5c1 1 2.5 1.5 3.5 1.5" /> </svg>"##;
const CLOCK_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.982 11.436a9 9 0 1 0 -9.966 9.51" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> <path d="M12 7v5l1 1" /> </svg>"##;
const CLOCK_STOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -9 9" /> <path d="M12 7v5l1 1" /> <path d="M16 16h6v6h-6l0 -6" /> </svg>"##;
const CLOCK_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.983 12.548a9 9 0 1 0 -8.45 8.436" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> <path d="M12 7v5l2.5 2.5" /> </svg>"##;
const CLOCK_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.926 13.15a9 9 0 1 0 -7.835 7.784" /> <path d="M12 7v5l2 2" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const CLOUD_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 21v-6m2 0v-1.5m0 9v-1.5m-2 -3h3m-1 0h.5a1.5 1.5 0 0 1 0 3h-3.5m3 -3h.5a1.5 1.5 0 0 0 0 -3h-3.5" /> <path d="M13.5 18.004h-6.843c-2.572 -.004 -4.657 -2.011 -4.657 -4.487s2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.28 1.023 1.957 2.51 1.873 4.027" /> </svg>"##;
const CLOUD_DOWNLOAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 18a3.5 3.5 0 0 0 0 -7h-1a5 4.5 0 0 0 -11 -2a4.6 4.4 0 0 0 -2.1 8.4" /> <path d="M12 13l0 9" /> <path d="M9 19l3 3l3 -3" /> </svg>"##;
const CLOUD_LOCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 18a3.5 3.5 0 0 0 0 -7h-1c.397 -1.768 -.285 -3.593 -1.788 -4.787c-1.503 -1.193 -3.6 -1.575 -5.5 -1s-3.315 2.019 -3.712 3.787c-2.199 -.088 -4.155 1.326 -4.666 3.373c-.512 2.047 .564 4.154 2.566 5.027" /> <path d="M8 16a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1l0 -3" /> <path d="M10 15v-2a2 2 0 1 1 4 0v2" /> </svg>"##;
const CLOUD_LOCK_OPEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 18a3.5 3.5 0 0 0 0 -7h-1c.397 -1.768 -.285 -3.593 -1.788 -4.787c-1.503 -1.193 -3.6 -1.575 -5.5 -1s-3.315 2.019 -3.712 3.787c-2.199 -.088 -4.155 1.326 -4.666 3.373c-.512 2.047 .564 4.154 2.566 5.027" /> <path d="M8 16a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1l0 -3" /> <path d="M10 15v-2a2 2 0 0 1 3.736 -1" /> </svg>"##;
const CLOUD_NETWORK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 20h7" /> <path d="M14 20h7" /> <path d="M10 20a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 16v2" /> <path d="M8 16.004h-1.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.913 0 3.464 1.56 3.464 3.486c0 1.927 -1.551 3.487 -3.465 3.487h-2.535" /> </svg>"##;
const CLOUD_UPLOAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18a4.6 4.4 0 0 1 0 -9a5 4.5 0 0 1 11 2h1a3.5 3.5 0 0 1 0 7h-1" /> <path d="M9 15l3 -3l3 3" /> <path d="M12 12l0 9" /> </svg>"##;
const CODE_AI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8l-4 4l4 4" /> <path d="M17 8l3.111 3.111" /> <path d="M14 4l-2.175 8.7" /> <path d="M14 21v-4a2 2 0 1 1 4 0v4" /> <path d="M14 19h4" /> <path d="M21 15v6" /> </svg>"##;
const CODE_VARIABLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v4a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -4" /> </svg>"##;
const CODE_VARIABLE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 16h-6.5a2 2 0 0 1 -2 -2v-4a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v4" /> <path d="M16 18h6" /> </svg>"##;
const CODE_VARIABLE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 16h-7a2 2 0 0 1 -2 -2v-4a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v1" /> <path d="M16 18h6" /> <path d="M19 15v6" /> </svg>"##;
const CODEBLOCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4l-2 2l2 2" /> <path d="M12 4l2 2l-2 2" /> <path d="M8 8l1 -4" /> <path d="M17 6a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-7" /> </svg>"##;
const COLUMN_INSERT_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 4h4a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-14a1 1 0 0 1 1 -1" /> <path d="M5 12l4 0" /> <path d="M7 10l0 4" /> </svg>"##;
const COLUMN_INSERT_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4h4a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-14a1 1 0 0 1 1 -1" /> <path d="M15 12l4 0" /> <path d="M17 10l0 4" /> </svg>"##;
const COLUMN_REMOVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4h4a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-14a1 1 0 0 1 1 -1" /> <path d="M16 10l4 4" /> <path d="M16 14l4 -4" /> </svg>"##;
const COMMAND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9a2 2 0 1 1 2 -2v10a2 2 0 1 1 -2 -2h10a2 2 0 1 1 -2 2v-10a2 2 0 1 1 2 2h-10" /> </svg>"##;
const COMMAND_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 9v8a2 2 0 1 1 -2 -2h8m3.411 3.417a2 2 0 0 1 -3.411 -1.417v-2m0 -4v-4a2 2 0 1 1 2 2h-4m-4 0h-2a2 2 0 0 1 -1.417 -3.411" /> <path d="M3 3l18 18" /> </svg>"##;
const CONFUCIUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 19l3 2v-18" /> <path d="M4 10l8 -2" /> <path d="M4 18l8 -10" /> <path d="M20 18l-8 -8l8 -4" /> </svg>"##;
const CONGRUENT_TO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 13h14" /> <path d="M5 17h14" /> <path d="M5 7.686c2.333 -2.624 4.667 -1.856 7 .064s4.667 2.688 7 .064" /> </svg>"##;
const CONNECTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 6.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> <path d="M4 17.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> <path d="M8.5 15.5l7 -7" /> </svg>"##;
const COPYLEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 9.75a3.016 3.016 0 0 1 4.163 .173a2.993 2.993 0 0 1 0 4.154a3.016 3.016 0 0 1 -4.163 .173" /> </svg>"##;
const COPYLEFT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.303 9.3a3.01 3.01 0 0 1 1.405 1.406m-.586 3.413a3.016 3.016 0 0 1 -4.122 .131" /> <path d="M20.042 16.045a9 9 0 0 0 -12.087 -12.087m-2.318 1.677a9 9 0 1 0 12.725 12.73" /> <path d="M3 3l18 18" /> </svg>"##;
const COPYRIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14 9.75a3.016 3.016 0 0 0 -4.163 .173a2.993 2.993 0 0 0 0 4.154a3.016 3.016 0 0 0 4.163 .173" /> </svg>"##;
const COPYRIGHT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 9.75a3.016 3.016 0 0 0 -.711 -.466m-3.41 .596a2.993 2.993 0 0 0 -.042 4.197a3.016 3.016 0 0 0 4.163 .173" /> <path d="M20.042 16.045a9 9 0 0 0 -12.087 -12.087m-2.318 1.677a9 9 0 1 0 12.725 12.73" /> <path d="M3 3l18 18" /> </svg>"##;
const CREATIVE_COMMONS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10.5 10.5c-.847 -.71 -2.132 -.658 -2.914 .116a1.928 1.928 0 0 0 0 2.768c.782 .774 2.067 .825 2.914 .116" /> <path d="M16.5 10.5c-.847 -.71 -2.132 -.658 -2.914 .116a1.928 1.928 0 0 0 0 2.768c.782 .774 2.067 .825 2.914 .116" /> </svg>"##;
const CREATIVE_COMMONS_BY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M11 7a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M9 13v-1a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-.5l-.5 4h-2l-.5 -4h-.5a1 1 0 0 1 -1 -1" /> </svg>"##;
const CREATIVE_COMMONS_NC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M15 9h-4.5a1.5 1.5 0 0 0 0 3h3a1.5 1.5 0 0 1 0 3h-4.5" /> <path d="M12 7v2" /> <path d="M12 15v2" /> <path d="M6 6l3 3" /> <path d="M15 15l3 3" /> </svg>"##;
const CREATIVE_COMMONS_ND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 10h6" /> <path d="M9 14h6" /> </svg>"##;
const CREATIVE_COMMONS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.638 5.634a9 9 0 1 0 12.723 12.733m1.686 -2.332a9 9 0 0 0 -12.093 -12.077" /> <path d="M10.5 10.5a2.187 2.187 0 0 0 -2.914 .116a1.928 1.928 0 0 0 0 2.768a2.188 2.188 0 0 0 2.914 .116" /> <path d="M16.5 10.5a2.194 2.194 0 0 0 -2.309 -.302" /> <path d="M3 3l18 18" /> </svg>"##;
const CREATIVE_COMMONS_SA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 16a4 4 0 1 0 -4 -4v1" /> <path d="M6 12l2 2l2 -2" /> </svg>"##;
const CREATIVE_COMMONS_ZERO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 12a3 4 0 1 0 6 0a3 4 0 1 0 -6 0" /> <path d="M14 9l-4 6" /> </svg>"##;
const CREDITS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 14a6 2 0 1 0 12 0a6 2 0 1 0 -12 0" /> <path d="M3 14v5c0 1.105 2.686 2 6 2s6 -.895 6 -2v-5" /> <path d="M9 5a6 2 0 1 0 12 0a6 2 0 1 0 -12 0" /> <path d="M9 5v3" /> <path d="M18.365 11.656c1.59 -.36 2.635 -.966 2.635 -1.656v-5" /> </svg>"##;
const CROSS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 21h4v-9h5v-4h-5v-5h-4v5h-5v4h5l0 9" /> </svg>"##;
const CROSS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12h3v-4h-5v-5h-4v3m-2 2h-3v4h5v9h4v-7" /> <path d="M3 3l18 18" /> </svg>"##;
const CROSSHAIR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> <path d="M9 12l6 0" /> <path d="M12 9l0 6" /> </svg>"##;
const CRUTCHES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 5a2 2 0 0 1 2 -2h4a2 2 0 0 1 2 2a2 2 0 0 1 -2 2h-4a2 2 0 0 1 -2 -2" /> <path d="M11 21h2" /> <path d="M12 21v-4.092a3 3 0 0 1 .504 -1.664l.992 -1.488a3 3 0 0 0 .504 -1.664v-5.092" /> <path d="M12 21v-4.092a3 3 0 0 0 -.504 -1.664l-.992 -1.488a3 3 0 0 1 -.504 -1.664v-5.092" /> <path d="M10 11h4" /> </svg>"##;
const CRUTCHES_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.178 4.174a2 2 0 0 1 1.822 -1.174h4a2 2 0 1 1 0 4h-3" /> <path d="M11 21h2" /> <path d="M12 21v-4.092a3 3 0 0 1 .504 -1.664l.992 -1.488a3 3 0 0 0 .097 -.155m.407 -3.601v-3" /> <path d="M12 21v-4.092a3 3 0 0 0 -.504 -1.664l-.992 -1.488a3 3 0 0 1 -.504 -1.664v-2.092" /> <path d="M10 11h1" /> <path d="M3 3l18 18" /> </svg>"##;
const CUBE_3D_SPHERE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 17.6l-2 -1.1v-2.5" /> <path d="M4 10v-2.5l2 -1.1" /> <path d="M10 4.1l2 -1.1l2 1.1" /> <path d="M18 6.4l2 1.1v2.5" /> <path d="M20 14v2.5l-2 1.12" /> <path d="M14 19.9l-2 1.1l-2 -1.1" /> <path d="M12 12l2 -1.1" /> <path d="M18 8.6l2 -1.1" /> <path d="M12 12l0 2.5" /> <path d="M12 18.5l0 2.5" /> <path d="M12 12l-2 -1.12" /> <path d="M6 8.6l-2 -1.1" /> </svg>"##;
const CUBE_3D_SPHERE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 17.6l-2 -1.1v-2.5" /> <path d="M4 10v-2.5l2 -1.1" /> <path d="M10 4.1l2 -1.1l2 1.1" /> <path d="M18 6.4l2 1.1v2.5" /> <path d="M20 14v2" /> <path d="M14 19.9l-2 1.1l-2 -1.1" /> <path d="M18 8.6l2 -1.1" /> <path d="M12 12v2.5" /> <path d="M12 18.5v2.5" /> <path d="M12 12l-2 -1.12" /> <path d="M6 8.6l-2 -1.1" /> <path d="M3 3l18 18" /> </svg>"##;
const CUBE_SEND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12.5l-5 -3l5 -3l5 3v5.5l-5 3l0 -5.5" /> <path d="M11 9.5v5.5l5 3" /> <path d="M16 12.545l5 -3.03" /> <path d="M7 9h-5" /> <path d="M7 12h-3" /> <path d="M7 15h-1" /> </svg>"##;
const CUBE_UNFOLDED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 15h10v5h5v-5h5v-5h-10v-5h-5v5h-5l0 5" /> <path d="M7 15v-5h5v5h5v-5" /> </svg>"##;
const CURLY_LOOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 8c-4 0 -7 2 -7 5a3 3 0 0 0 6 0c0 -3 -2.5 -5 -8 -5s-8 2 -8 5a3 3 0 0 0 6 0c0 -3 -3 -5 -7 -5" /> </svg>"##;
const CURSOR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 4a3 3 0 0 1 3 3v1m0 9a3 3 0 0 1 -3 3" /> <path d="M15 4a3 3 0 0 0 -3 3v1m0 4v5a3 3 0 0 0 3 3" /> <path d="M3 3l18 18" /> </svg>"##;
const DASHBOARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 13a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M13.45 11.55l2.05 -2.05" /> <path d="M6.4 20a9 9 0 1 1 11.2 0l-11.2 0" /> </svg>"##;
const DASHBOARD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.175 11.178a2 2 0 1 0 2.653 2.634" /> <path d="M14.5 10.5l1 -1" /> <path d="M8.621 4.612a9 9 0 0 1 11.721 11.72m-1.516 2.488a9.008 9.008 0 0 1 -1.226 1.18h-11.2a9 9 0 0 1 -.268 -13.87" /> <path d="M3 3l18 18" /> </svg>"##;
const DATABASE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a8 3 0 1 0 16 0a8 3 0 1 0 -16 0" /> <path d="M4 6v6a8 3 0 0 0 16 0v-6" /> <path d="M4 12v6a8 3 0 0 0 16 0v-6" /> </svg>"##;
const DATABASE_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v6c0 1.657 3.582 3 8 3c.21 0 .42 -.003 .626 -.01" /> <path d="M20 11.5v-5.5" /> <path d="M4 12v6c0 1.657 3.582 3 8 3" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const DATABASE_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v6c0 1.657 3.582 3 8 3c.415 0 .822 -.012 1.22 -.035" /> <path d="M20 10v-4" /> <path d="M4 12v6c0 1.657 3.582 3 8 3c.352 0 .698 -.009 1.037 -.025" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const DATABASE_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v6c0 1.657 3.582 3 8 3c.478 0 .947 -.016 1.402 -.046" /> <path d="M20 12v-6" /> <path d="M4 12v6c0 1.526 3.04 2.786 6.972 2.975" /> <path d="M18.42 15.61a2.1 2.1 0 0 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const DATABASE_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v6c0 1.657 3.582 3 8 3c1.118 0 2.182 -.086 3.148 -.241m4.852 -2.759v-6" /> <path d="M4 12v6c0 1.657 3.582 3 8 3c1.064 0 2.079 -.078 3.007 -.22" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const DATABASE_EXPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v6c0 1.657 3.582 3 8 3c1.118 0 2.183 -.086 3.15 -.241" /> <path d="M20 12v-6" /> <path d="M4 12v6c0 1.657 3.582 3 8 3c.157 0 .312 -.002 .466 -.005" /> <path d="M16 19h6" /> <path d="M19 16l3 3l-3 3" /> </svg>"##;
const DATABASE_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v6c0 1.453 2.755 2.665 6.414 2.941" /> <path d="M20 11v-5" /> <path d="M4 12v6c0 1.579 3.253 2.873 7.383 2.991" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const DATABASE_IMPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v6c0 1.657 3.582 3 8 3c.856 0 1.68 -.05 2.454 -.144m5.546 -2.856v-6" /> <path d="M4 12v6c0 1.657 3.582 3 8 3c.171 0 .341 -.002 .51 -.006" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const DATABASE_LEAK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v12c0 1.657 3.582 3 8 3s8 -1.343 8 -3v-12" /> <path d="M4 15a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1" /> </svg>"##;
const DATABASE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v6c0 1.657 3.582 3 8 3s8 -1.343 8 -3v-6" /> <path d="M4 12v6c0 1.657 3.582 3 8 3c.164 0 .328 -.002 .49 -.006" /> <path d="M20 15v-3" /> <path d="M16 19h6" /> </svg>"##;
const DATABASE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.983 8.978c3.955 -.182 7.017 -1.446 7.017 -2.978c0 -1.657 -3.582 -3 -8 -3c-1.661 0 -3.204 .19 -4.483 .515m-2.783 1.228c-.471 .382 -.734 .808 -.734 1.257c0 1.22 1.944 2.271 4.734 2.74" /> <path d="M4 6v6c0 1.657 3.582 3 8 3c.986 0 1.93 -.067 2.802 -.19m3.187 -.82c1.251 -.53 2.011 -1.228 2.011 -1.99v-6" /> <path d="M4 12v6c0 1.657 3.582 3 8 3c3.217 0 5.991 -.712 7.261 -1.74m.739 -3.26v-4" /> <path d="M3 3l18 18" /> </svg>"##;
const DATABASE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v6c0 1.657 3.582 3 8 3c1.075 0 2.1 -.08 3.037 -.224" /> <path d="M20 12v-6" /> <path d="M4 12v6c0 1.657 3.582 3 8 3c.166 0 .331 -.002 .495 -.006" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const DATABASE_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v6c0 1.657 3.582 3 8 3m8 -3.5v-5.5" /> <path d="M4 12v6c0 1.657 3.582 3 8 3" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const DATABASE_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v6c0 1.657 3.582 3 8 3c.361 0 .716 -.009 1.065 -.026" /> <path d="M20 13v-7" /> <path d="M4 12v6c0 1.657 3.582 3 8 3" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const DATABASE_SMILE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 14h.01" /> <path d="M14 14h.01" /> <path d="M10 17a3.5 3.5 0 0 0 4 0" /> <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v12c0 1.657 3.582 3 8 3s8 -1.343 8 -3v-12" /> </svg>"##;
const DATABASE_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v6c0 1.43 2.67 2.627 6.243 2.927" /> <path d="M20 10.5v-4.5" /> <path d="M4 12v6c0 1.546 3.12 2.82 7.128 2.982" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const DATABASE_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 1.657 3.582 3 8 3s8 -1.343 8 -3s-3.582 -3 -8 -3s-8 1.343 -8 3" /> <path d="M4 6v6c0 1.657 3.582 3 8 3c.537 0 1.062 -.02 1.57 -.058" /> <path d="M20 13.5v-7.5" /> <path d="M4 12v6c0 1.657 3.582 3 8 3c.384 0 .762 -.01 1.132 -.03" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const DEAF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 10a7 7 0 1 1 13 3.6a10 10 0 0 1 -2 2a8 8 0 0 0 -2 3a4.5 4.5 0 0 1 -6.8 1.4" /> <path d="M10 10a3 3 0 1 1 5 2.2" /> <path d="M5 13l4 4" /> <path d="M9 13l-4 4" /> </svg>"##;
const DECIMAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M10 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M5 16h.01" /> </svg>"##;
const DENTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5.5c-1.074 -.586 -2.583 -1.5 -4 -1.5c-2.1 0 -4 1.247 -4 5c0 4.899 1.056 8.41 2.671 10.537c.573 .756 1.97 .521 2.567 -.236c.398 -.505 .819 -1.439 1.262 -2.801c.292 -.771 .892 -1.504 1.5 -1.5c.602 0 1.21 .737 1.5 1.5c.443 1.362 .864 2.295 1.262 2.8c.597 .759 2 .993 2.567 .237c1.615 -2.127 2.671 -5.637 2.671 -10.537c0 -3.74 -1.908 -5 -4 -5c-1.423 0 -2.92 .911 -4 1.5" /> <path d="M12 5.5l3 1.5" /> </svg>"##;
const DENTAL_BROKEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5.5c-1.074 -.586 -2.583 -1.5 -4 -1.5c-2.1 0 -4 1.247 -4 5c0 4.899 1.056 8.41 2.671 10.537c.573 .756 1.97 .521 2.567 -.236c.398 -.505 .819 -1.439 1.262 -2.801c.292 -.771 .892 -1.504 1.5 -1.5c.602 0 1.21 .737 1.5 1.5c.443 1.362 .864 2.295 1.262 2.8c.597 .759 2 .993 2.567 .237c1.615 -2.127 2.671 -5.637 2.671 -10.537c0 -3.74 -1.908 -5 -4 -5c-1.423 0 -2.92 .911 -4 1.5" /> <path d="M12 5.5l1 2.5l-2 2l2 2" /> </svg>"##;
const DENTAL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.277 15.281c.463 -1.75 .723 -3.844 .723 -6.281c0 -3.74 -1.908 -5 -4 -5c-1.423 0 -2.92 .911 -4 1.5c-1.074 -.586 -2.583 -1.5 -4 -1.5m-2.843 1.153c-.707 .784 -1.157 2.017 -1.157 3.847c0 4.899 1.056 8.41 2.671 10.537c.573 .756 1.97 .521 2.567 -.236c.398 -.505 .819 -1.439 1.262 -2.801c.292 -.771 .892 -1.504 1.5 -1.5c.602 0 1.21 .737 1.5 1.5c.443 1.362 .864 2.295 1.262 2.8c.597 .759 2 .993 2.567 .237c.305 -.402 .59 -.853 .852 -1.353" /> <path d="M12 5.5l3 1.5" /> <path d="M3 3l18 18" /> </svg>"##;
const DESELECT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8h3a1 1 0 0 1 1 1v3" /> <path d="M16 16h-7a1 1 0 0 1 -1 -1v-7" /> <path d="M12 20v.01" /> <path d="M16 20v.01" /> <path d="M8 20v.01" /> <path d="M4 20v.01" /> <path d="M4 16v.01" /> <path d="M4 12v.01" /> <path d="M4 8v.01" /> <path d="M8 4v.01" /> <path d="M12 4v.01" /> <path d="M16 4v.01" /> <path d="M20 4v.01" /> <path d="M20 8v.01" /> <path d="M20 12v.01" /> <path d="M20 16v.01" /> <path d="M3 3l18 18" /> </svg>"##;
const DESK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6h18" /> <path d="M4 6v13" /> <path d="M20 19v-13" /> <path d="M4 10h16" /> <path d="M15 6v8a2 2 0 0 0 2 2h3" /> </svg>"##;
const DETAILS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.999 3l.001 17" /> <path d="M10.363 3.591l-8.106 13.534a1.914 1.914 0 0 0 1.636 2.871h16.214a1.914 1.914 0 0 0 1.636 -2.87l-8.106 -13.536a1.914 1.914 0 0 0 -3.274 0" /> </svg>"##;
const DETAILS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 19h14" /> <path d="M20.986 16.984a2 2 0 0 0 -.146 -.734l-7.1 -12.25a2 2 0 0 0 -3.5 0l-.821 1.417m-1.469 2.534l-4.81 8.299a2 2 0 0 0 1.75 2.75" /> <path d="M12 3v5m0 4v7" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICE_PROJECTOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9a5 5 0 1 0 10 0a5 5 0 0 0 -10 0" /> <path d="M9 6h-4a2 2 0 0 0 -2 2v8a2 2 0 0 0 2 2h14a2 2 0 0 0 2 -2v-8a2 2 0 0 0 -2 -2h-2" /> <path d="M6 15h1" /> <path d="M7 18l-1 2" /> <path d="M18 18l1 2" /> </svg>"##;
const DEVICE_UNKNOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -14" /> <path d="M12 16v.01" /> <path d="M12 13a2 2 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const DIRECTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 10l3 -3l3 3" /> <path d="M9 14l3 3l3 -3" /> </svg>"##;
const DIRECTION_ARROWS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M8 11l-1 1l1 1" /> <path d="M11 8l1 -1l1 1" /> <path d="M16 11l1 1l-1 1" /> <path d="M11 16l1 1l1 -1" /> </svg>"##;
const DIRECTION_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 9l-3 3l3 3" /> <path d="M14 9l3 3l-3 3" /> </svg>"##;
const DIRECTION_SIGN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.32 12.774l7.906 7.905c.427 .428 1.12 .428 1.548 0l7.905 -7.905a1.095 1.095 0 0 0 0 -1.548l-7.905 -7.905a1.095 1.095 0 0 0 -1.548 0l-7.905 7.905a1.095 1.095 0 0 0 0 1.548" /> <path d="M8 12h7.5" /> <path d="M12 8.5l3.5 3.5l-3.5 3.5" /> </svg>"##;
const DIRECTION_SIGN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.73 14.724l1.949 -1.95a1.095 1.095 0 0 0 0 -1.548l-7.905 -7.905a1.095 1.095 0 0 0 -1.548 0l-1.95 1.95m-2.01 2.01l-3.945 3.945a1.095 1.095 0 0 0 0 1.548l7.905 7.905c.427 .428 1.12 .428 1.548 0l3.95 -3.95" /> <path d="M8 12h4" /> <path d="M13.748 13.752l-1.748 1.748" /> <path d="M3 3l18 18" /> </svg>"##;
const DISABLED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M11 7l0 8l4 0l4 5" /> <path d="M11 11l5 0" /> <path d="M7 11.5a5 5 0 1 0 6 7.5" /> </svg>"##;
const DISABLED_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M9 11a5 5 0 1 0 3.95 7.95" /> <path d="M19 20l-4 -5h-4l3 -5l-4 -3l-4 1" /> </svg>"##;
const DISABLED_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 7a2 2 0 1 0 -2 -2" /> <path d="M11 11v4h4l4 5" /> <path d="M15 11h1" /> <path d="M7 11.5a5 5 0 1 0 6 7.5" /> <path d="M3 3l18 18" /> </svg>"##;
const DIVIDE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 6a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" fill="currentColor" /> <path d="M11 18a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" fill="currentColor" /> <path d="M5 12l14 0" /> </svg>"##;
const DNA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.828 14.828a4 4 0 1 0 -5.656 -5.656a4 4 0 0 0 5.656 5.656" /> <path d="M9.172 20.485a4 4 0 1 0 -5.657 -5.657" /> <path d="M14.828 3.515a4 4 0 0 0 5.657 5.657" /> </svg>"##;
const DNA_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 3v1c-.01 3.352 -1.68 6.023 -5.008 8.014c-3.328 1.99 3.336 -2 .008 -.014c-3.328 1.99 -5 4.662 -5.008 8.014v1" /> <path d="M17 21.014v-1c-.01 -3.352 -1.68 -6.023 -5.008 -8.014c-3.328 -1.99 3.336 2 .008 .014c-3.328 -1.991 -5 -4.662 -5.008 -8.014v-1" /> <path d="M7 4h10" /> <path d="M7 20h10" /> <path d="M8 8h8" /> <path d="M8 16h8" /> </svg>"##;
const DNA_2_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 3v1c-.007 2.46 -.91 4.554 -2.705 6.281m-2.295 1.719c-3.328 1.99 -5 4.662 -5.008 8.014v1" /> <path d="M17 21.014v-1c0 -1.44 -.315 -2.755 -.932 -3.944m-4.068 -4.07c-1.903 -1.138 -3.263 -2.485 -4.082 -4.068" /> <path d="M8 4h9" /> <path d="M7 20h10" /> <path d="M12 8h4" /> <path d="M8 16h8" /> <path d="M3 3l18 18" /> </svg>"##;
const DNA_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12a3.898 3.898 0 0 0 -1.172 -2.828a4.027 4.027 0 0 0 -2.828 -1.172m-2.828 1.172a4 4 0 1 0 5.656 5.656" /> <path d="M9.172 20.485a4 4 0 1 0 -5.657 -5.657" /> <path d="M14.828 3.515a4 4 0 1 0 5.657 5.657" /> <path d="M3 3l18 18" /> </svg>"##;
const DOOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 12v.01" /> <path d="M3 21h18" /> <path d="M6 21v-16a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v16" /> </svg>"##;
const DOOR_ENTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 12v.01" /> <path d="M3 21h18" /> <path d="M5 21v-16a2 2 0 0 1 2 -2h6m4 10.5v7.5" /> <path d="M21 7h-7m3 -3l-3 3l3 3" /> </svg>"##;
const DOOR_EXIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 12v.01" /> <path d="M3 21h18" /> <path d="M5 21v-16a2 2 0 0 1 2 -2h7.5m2.5 10.5v7.5" /> <path d="M14 7h7m-3 -3l3 3l-3 3" /> </svg>"##;
const DOOR_HANGER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a7 7 0 0 0 -5.48 2.64a1 1 0 0 0 .31 1.49l1.76 1a1 1 0 0 0 1.22 -.13a3 3 0 0 1 2.82 -.88a3.09 3.09 0 0 1 2.37 3.01v.87l-9.2 1.84a1 1 0 0 0 -.8 1v6.16a1 1 0 0 0 1 1h12a1 1 0 0 0 1 -1v-9.72a7.18 7.18 0 0 0 -7 -7.28" /> <path d="M12 17v.01" /> </svg>"##;
const DOOR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21h18" /> <path d="M6 21v-15" /> <path d="M7.18 3.175c.25 -.112 .528 -.175 .82 -.175h8a2 2 0 0 1 2 2v9" /> <path d="M18 18v3" /> <path d="M3 3l18 18" /> </svg>"##;
const DOTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M18 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const DOTS_CIRCLE_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M8 12l0 .01" /> <path d="M12 12l0 .01" /> <path d="M16 12l0 .01" /> </svg>"##;
const DOTS_DIAGONAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 17a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M16 7a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const DOTS_DIAGONAL_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 7a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M16 17a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const DOTS_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 19a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const EAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 10a7 7 0 1 1 13 3.6a10 10 0 0 1 -2 2a8 8 0 0 0 -2 3a4.5 4.5 0 0 1 -6.8 1.4" /> <path d="M10 10a3 3 0 1 1 5 2.2" /> </svg>"##;
const EAR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 10c0 -1.146 .277 -2.245 .78 -3.219m1.792 -2.208a7 7 0 0 1 10.428 9.027a10 10 0 0 1 -.633 .762m-2.045 1.96a8 8 0 0 0 -1.322 2.278a4.5 4.5 0 0 1 -6.8 1.4" /> <path d="M11.42 7.414a3 3 0 0 1 4.131 4.13" /> <path d="M3 3l18 18" /> </svg>"##;
const EAR_SCAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 15a2 2 0 0 1 -2 2c-.732 0 -1.555 -.247 -1.72 -.98c-.634 -2.8 -3.17 -2.628 -3.28 -5.02v-.5a3.5 3.5 0 0 1 6.671 -1.483" /> <path d="M13 12v.01" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const ELEVATOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1l0 -14" /> <path d="M10 10l2 -2l2 2" /> <path d="M10 14l2 2l2 -2" /> </svg>"##;
const ELEVATOR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h10a1 1 0 0 1 1 1v10m0 4a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1v-14" /> <path d="M12 8l2 2" /> <path d="M10 14l2 2l2 -2" /> <path d="M3 3l18 18" /> </svg>"##;
const EMERGENCY_BED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 8l2.1 2.8a3 3 0 0 0 2.4 1.2h11.5" /> <path d="M10 6h4" /> <path d="M12 4v4" /> <path d="M12 12v2l-2.5 2.5" /> <path d="M14.5 16.5l-2.5 -2.5" /> </svg>"##;
const EMPATHIZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 5.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> <path d="M12 21.368l5.095 -5.096a3.088 3.088 0 1 0 -4.367 -4.367l-.728 .727l-.728 -.727a3.088 3.088 0 1 0 -4.367 4.367l5.095 5.096" /> </svg>"##;
const EMPATHIZE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8a2.5 2.5 0 1 0 -2.5 -2.5" /> <path d="M12.317 12.315l-.317 .317l-.728 -.727a3.088 3.088 0 1 0 -4.367 4.367l5.095 5.096l4.689 -4.69m1.324 -2.673a3.087 3.087 0 0 0 -3.021 -3.018" /> <path d="M3 3l18 18" /> </svg>"##;
const EQUAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 10h14" /> <path d="M5 14h14" /> </svg>"##;
const EQUAL_DOUBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10h7" /> <path d="M3 14h7" /> <path d="M14 10h7" /> <path d="M14 14h7" /> </svg>"##;
const EQUAL_NOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 10h14" /> <path d="M5 14h14" /> <path d="M5 19l14 -14" /> </svg>"##;
const EXCHANGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19 8v5a5 5 0 0 1 -5 5h-3l3 -3m0 6l-3 -3" /> <path d="M5 16v-5a5 5 0 0 1 5 -5h3l-3 -3m0 6l3 -3" /> </svg>"##;
const EXCHANGE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19 8v5c0 .594 -.104 1.164 -.294 1.692m-1.692 2.298a4.978 4.978 0 0 1 -3.014 1.01h-3l3 -3" /> <path d="M14 21l-3 -3" /> <path d="M5 16v-5c0 -1.632 .782 -3.082 1.992 -4m3.008 -1h3l-3 -3" /> <path d="M11.501 7.499l1.499 -1.499" /> <path d="M3 3l18 18" /> </svg>"##;
const EXCLAMATION_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 9v4" /> <path d="M12 16v.01" /> </svg>"##;
const EXCLAMATION_MARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19v.01" /> <path d="M12 15v-10" /> </svg>"##;
const EXCLAMATION_MARK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19v.01" /> <path d="M12 15v-3m0 -4v-3" /> <path d="M3 3l18 18" /> </svg>"##;
const EXPLICIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1l0 -14" /> <path d="M14 8h-4v8h4" /> <path d="M14 12h-4" /> </svg>"##;
const EXPLICIT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h-2m-2 2v6h4" /> <path d="M8 4h10a2 2 0 0 1 2 2v10m-.586 3.414a2 2 0 0 1 -1.414 .586h-12a2 2 0 0 1 -2 -2v-12c0 -.547 .22 -1.043 .576 -1.405" /> <path d="M12 12h-2" /> <path d="M3 3l18 18" /> </svg>"##;
const EXTERNAL_LINK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 6h-6a2 2 0 0 0 -2 2v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-6" /> <path d="M11 13l9 -9" /> <path d="M15 4h5v5" /> </svg>"##;
const EXTERNAL_LINK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7h-1a2 2 0 0 0 -2 2v9a2 2 0 0 0 2 2h9a2 2 0 0 0 2 -2v-1" /> <path d="M10 14l2 -2m2.007 -2.007l6 -6" /> <path d="M15 4h5v5" /> <path d="M3 3l18 18" /> </svg>"##;
const EYE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M21 12c-2.4 4 -5.4 6 -9 6c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6" /> </svg>"##;
const EYE_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M13.193 17.924q -.585 .075 -1.193 .076q -5.4 0 -9 -6q 3.6 -6 9 -6q 4.508 0 7.761 4.181" /> <path d="M17 21v-6m2 0v-1.5m0 9v-1.5m-2 -3h3m-1 0h.5a1.5 1.5 0 0 1 0 3h-3.5m3 -3h.5a1.5 1.5 0 0 0 0 -3h-3.5" /> </svg>"##;
const EYE_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M13.1 17.936a9.28 9.28 0 0 1 -1.1 .064c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const EYE_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 18c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const EYE_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M11.102 17.957c-3.204 -.307 -5.904 -2.294 -8.102 -5.957c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6a19.5 19.5 0 0 1 -.663 1.032" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const EYE_CLOSED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 9c-2.4 2.667 -5.4 4 -9 4c-3.6 0 -6.6 -1.333 -9 -4" /> <path d="M3 15l2.5 -3.8" /> <path d="M21 14.976l-2.492 -3.776" /> <path d="M9 17l.5 -4" /> <path d="M15 17l-.5 -4" /> </svg>"##;
const EYE_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M11.11 17.958c-3.209 -.307 -5.91 -2.293 -8.11 -5.958c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6c-.21 .352 -.427 .688 -.647 1.008" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const EYE_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 18c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const EYE_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 18c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6" /> <path d="M16 21l5 -5" /> <path d="M21 21v.01" /> <path d="M16 16v.01" /> </svg>"##;
const EYE_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M13.193 17.924c-.39 .05 -.788 .076 -1.193 .076c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.005 0 5.592 1.394 7.761 4.181" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const EYE_DOTTED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M21 12h.01" /> <path d="M3 12h.01" /> <path d="M5 15h.01" /> <path d="M5 9h.01" /> <path d="M19 15h.01" /> <path d="M12 18h.01" /> <path d="M12 6h.01" /> <path d="M8 17h.01" /> <path d="M8 7h.01" /> <path d="M16 17h.01" /> <path d="M16 7h.01" /> <path d="M19 9h.01" /> </svg>"##;
const EYE_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 18c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const EYE_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M11.192 17.966c-3.242 -.28 -5.972 -2.269 -8.192 -5.966c2.4 -4 5.4 -6 9 -6c3.326 0 6.14 1.707 8.442 5.122" /> <path d="M18.42 15.61a2.1 2.1 0 0 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const EYE_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M15.03 17.478a8.797 8.797 0 0 1 -3.03 .522c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6a20.48 20.48 0 0 1 -.258 .419" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const EYE_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.905 11.39a2 2 0 1 0 -2.855 2.37" /> <path d="M9.992 17.779c-2.722 -.621 -5.053 -2.547 -6.992 -5.779c2.4 -4 5.4 -6 9 -6c3.332 0 6.15 1.714 8.454 5.14" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const EYE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 18c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6c-.713 1.188 -1.478 2.199 -2.296 3.034" /> <path d="M16 19h6" /> </svg>"##;
const EYE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.585 10.587a2 2 0 0 0 2.829 2.828" /> <path d="M16.681 16.673a8.717 8.717 0 0 1 -4.681 1.327c-3.6 0 -6.6 -2 -9 -6c1.272 -2.12 2.712 -3.678 4.32 -4.674m2.86 -1.146a9.055 9.055 0 0 1 1.82 -.18c3.6 0 6.6 2 9 6c-.666 1.11 -1.379 2.067 -2.138 2.87" /> <path d="M3 3l18 18" /> </svg>"##;
const EYE_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M13.022 17.945a9.308 9.308 0 0 1 -1.022 .055c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6c-.195 .325 -.394 .636 -.596 .935" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const EYE_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 18c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.362 0 6.202 1.745 8.517 5.234" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const EYE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 18c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const EYE_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M14.071 17.764a8.989 8.989 0 0 1 -2.071 .236c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.346 0 6.173 1.727 8.482 5.182" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const EYE_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 18c-.328 0 -.652 -.017 -.97 -.05c-3.172 -.332 -5.85 -2.315 -8.03 -5.95c2.4 -4 5.4 -6 9 -6c3.465 0 6.374 1.853 8.727 5.558" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const EYE_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12.597 17.981a9.467 9.467 0 0 1 -.597 .019c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6c-.205 .342 -.415 .67 -.63 .983" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const EYE_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M11.669 17.994q -5.18 -.18 -8.669 -5.994q 3.6 -6 9 -6t 9 6" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const EYE_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M9.608 17.682c-2.558 -.71 -4.76 -2.603 -6.608 -5.682c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const EYE_TABLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 18h-.011" /> <path d="M12 18h-.011" /> <path d="M16 18h-.011" /> <path d="M4 3h16" /> <path d="M5 3v17a1 1 0 0 0 1 1h12a1 1 0 0 0 1 -1v-17" /> <path d="M14 7h-4" /> <path d="M9 15h1" /> <path d="M14 15h1" /> <path d="M12 11v-4" /> </svg>"##;
const EYE_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 18c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6c-.09 .15 -.18 .295 -.27 .439" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const EYE_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M13.048 17.942a9.298 9.298 0 0 1 -1.048 .058c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6a17.986 17.986 0 0 1 -1.362 1.975" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const EYEGLASS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h-2l-3 10" /> <path d="M16 4h2l3 10" /> <path d="M10 16l4 0" /> <path d="M21 16.5a3.5 3.5 0 0 1 -7 0v-2.5h7v2.5" /> <path d="M10 16.5a3.5 3.5 0 0 1 -7 0v-2.5h7v2.5" /> </svg>"##;
const EYEGLASS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h-2l-3 10v2.5" /> <path d="M16 4h2l3 10v2.5" /> <path d="M10 16l4 0" /> <path d="M14 16.5a3.5 3.5 0 1 0 7 0a3.5 3.5 0 1 0 -7 0" /> <path d="M3 16.5a3.5 3.5 0 1 0 7 0a3.5 3.5 0 1 0 -7 0" /> </svg>"##;
const EYEGLASS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.536 5.546l-2.536 8.454" /> <path d="M16 4h2l3 10" /> <path d="M10 16h4" /> <path d="M19.426 19.423a3.5 3.5 0 0 1 -5.426 -2.923v-2.5m4 0h3v2.5c0 .157 -.01 .312 -.03 .463" /> <path d="M10 16.5a3.5 3.5 0 0 1 -7 0v-2.5h7v2.5" /> <path d="M3 3l18 18" /> </svg>"##;
const FACE_ID_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> <path d="M9 10l.01 0" /> <path d="M15 10l.01 0" /> <path d="M9.5 15a3.5 3.5 0 0 0 5 0" /> </svg>"##;
const FACE_ID_ERROR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15.05a3.5 3.5 0 0 1 5 0" /> </svg>"##;
const FACE_MASK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 14.5h-.222c-1.535 0 -2.778 -1.12 -2.778 -2.5s1.243 -2.5 2.778 -2.5h.222" /> <path d="M19 14.5h.222c1.534 0 2.778 -1.12 2.778 -2.5s-1.244 -2.5 -2.778 -2.5h-.222" /> <path d="M9 10h6" /> <path d="M9 14h6" /> <path d="M12.55 18.843l5 -1.429a2 2 0 0 0 1.45 -1.923v-6.981a2 2 0 0 0 -1.45 -1.923l-5 -1.429a2 2 0 0 0 -1.1 0l-5 1.429a2 2 0 0 0 -1.45 1.922v6.982a2 2 0 0 0 1.45 1.923l5 1.429a2 2 0 0 0 1.1 0" /> </svg>"##;
const FACE_MASK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 14.5h-.222c-1.535 0 -2.778 -1.12 -2.778 -2.5s1.243 -2.5 2.778 -2.5h.222" /> <path d="M19 14.5h.222c1.534 0 2.778 -1.12 2.778 -2.5s-1.244 -2.5 -2.778 -2.5h-.222" /> <path d="M9 10h1m4 0h1" /> <path d="M9 14h5" /> <path d="M19 15v-6.49a2 2 0 0 0 -1.45 -1.923l-5 -1.429a2 2 0 0 0 -1.1 0l-1.788 .511m-3.118 .891l-.094 .027a2 2 0 0 0 -1.45 1.922v6.982a2 2 0 0 0 1.45 1.923l5 1.429a2 2 0 0 0 1.1 0l4.899 -1.4" /> <path d="M3 3l18 18" /> </svg>"##;
const FALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 21l1 -5l-1 -4l-3 -4h4l3 -3" /> <path d="M6 16l-1 -4l3 -4" /> <path d="M5 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M13.5 12h2.5l4 2" /> </svg>"##;
const FENCE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12v4h16v-4l-16 0" /> <path d="M6 16v4h4v-4m0 -4v-6l-2 -2l-2 2v6" /> <path d="M14 16v4h4v-4m0 -4v-6l-2 -2l-2 2v6" /> </svg>"##;
const FENCE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12h-8v4h12m4 0v-4h-4" /> <path d="M6 16v4h4v-4" /> <path d="M10 12v-2m0 -4l-2 -2m-2 2v6" /> <path d="M14 16v4h4v-2" /> <path d="M18 12v-6l-2 -2l-2 2v4" /> <path d="M3 3l18 18" /> </svg>"##;
const FIDGET_SPINNER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 16v.01" /> <path d="M6 16v.01" /> <path d="M12 5v.01" /> <path d="M12 12v.01" /> <path d="M12 1a4 4 0 0 1 2.001 7.464l.001 .072a3.998 3.998 0 0 1 1.987 3.758l.22 .128a3.978 3.978 0 0 1 1.591 -.417l.2 -.005a4 4 0 1 1 -3.994 3.77l-.28 -.16c-.522 .25 -1.108 .39 -1.726 .39c-.619 0 -1.205 -.14 -1.728 -.391l-.279 .16l.007 .231a4 4 0 1 1 -2.212 -3.579l.222 -.129a3.998 3.998 0 0 1 1.988 -3.756l.002 -.071a4 4 0 0 1 -1.995 -3.265l-.005 -.2a4 4 0 0 1 4 -4" /> </svg>"##;
const FILTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v7l-6 2v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227" /> </svg>"##;
const FILTER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h6" /> </svg>"##;
const FILTER_2_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h4" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const FILTER_2_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h3" /> <path d="M17 21l4 -4m-5 2c0 .796 .316 1.559 .879 2.121c.563 .563 1.326 .879 2.121 .879c.796 0 1.559 -.316 2.121 -.879c.563 -.563 .879 -1.326 .879 -2.121c0 -.796 -.316 -1.559 -.879 -2.121c-.563 -.563 -1.326 -.879 -2.121 -.879c-.796 0 -1.559 .316 -2.121 .879c-.563 .563 -.879 1.326 -.879 2.121l1 2" /> </svg>"##;
const FILTER_2_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h2" /> <path d="M15 18l2 2l4 -4" /> </svg>"##;
const FILTER_2_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h2" /> <path d="M17 17l-2 2l2 2" /> <path d="M20 21l2 -2l-2 -2" /> </svg>"##;
const FILTER_2_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h3" /> <path d="M19.001 21c-.53 0 -1.039 -.211 -1.414 -.586c-.375 -.375 -.586 -.884 -.586 -1.414c0 -.53 .211 -1.039 .586 -1.414c.375 -.375 .884 -.586 1.414 -.586m0 4c.53 0 1.039 -.211 1.414 -.586c.375 -.375 .586 -.884 .586 -1.414c0 -.53 -.211 -1.039 -.586 -1.414c-.375 -.375 -.884 -.586 -1.414 -.586m0 4v1.5m0 -5.5v-1.5m3.031 1.75l-1.299 .75m-3.463 2l-1.3 .75m0 -3.5l1.3 .75m3.463 2l1.3 .75" /> </svg>"##;
const FILTER_2_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h3" /> <path d="M16 16v.01" /> <path d="M16 21l5 -5" /> <path d="M21 21v.01" /> </svg>"##;
const FILTER_2_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h10" /> <path d="M9 18h4" /> <path d="M21 15h-2m-2 6h2m0 0v1m0 -1h.5c.398 0 .779 -.158 1.061 -.439c.281 -.281 .439 -.663 .439 -1.061c0 -.398 -.158 -.779 -.439 -1.061c-.281 -.281 -.663 -.439 -1.061 -.439h-1c-.398 0 -.779 -.158 -1.061 -.439c-.281 -.281 -.439 -.663 -.439 -1.061c0 -.398 .158 -.779 .439 -1.061c.281 -.281 .663 -.439 1.061 -.439h.5m0 -1v1" /> </svg>"##;
const FILTER_2_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h3" /> <path d="M19 16v6" /> <path d="M19 22l3 -3" /> <path d="M19 22l-3 -3" /> </svg>"##;
const FILTER_2_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h11" /> <path d="M9 18h2" /> <path d="M18.42 15.61c.195 -.195 .426 -.35 .681 -.455c.255 -.106 .528 -.16 .804 -.16c.276 0 .549 .054 .804 .16c.255 .106 .486 .26 .681 .455c.195 .195 .35 .427 .455 .681c.106 .255 .16 .528 .16 .804c0 .276 -.054 .549 -.16 .804c-.105 .255 -.26 .486 -.455 .681l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const FILTER_2_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h6" /> <path d="M19 16v3m0 3v.01" /> </svg>"##;
const FILTER_2_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h3" /> <path d="M16 19h6" /> </svg>"##;
const FILTER_2_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h4" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const FILTER_2_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h10" /> <path d="M9 18h3" /> <path d="M19 18v.01m2.121 2.111c.42 -.419 .706 -.954 .821 -1.536c.116 -.582 .056 -1.185 -.17 -1.733c-.227 -.548 -.611 -1.017 -1.105 -1.347c-.493 -.33 -1.073 -.506 -1.667 -.506c-.593 0 -1.173 .176 -1.667 .506c-.493 .33 -.878 .798 -1.105 1.347c-.227 .548 -.286 1.151 -.17 1.733c.116 .582 .402 1.116 .821 1.536c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879l-2.121 -2.121" /> </svg>"##;
const FILTER_2_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h10" /> <path d="M9 18h3" /> <path d="M19 15v6" /> <path d="M16 18h6" /> </svg>"##;
const FILTER_2_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h10.5" /> <path d="M9 18h5" /> <path d="M19 22v.01" /> <path d="M19 19c.448 -.001 .883 -.153 1.235 -.431c.352 -.278 .6 -.666 .706 -1.101c.105 -.436 .061 -.894 -.125 -1.302c-.186 -.408 -.504 -.742 -.902 -.948c-.398 -.204 -.853 -.267 -1.291 -.179c-.438 .088 -.834 .321 -1.123 .662" /> </svg>"##;
const FILTER_2_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h8.5" /> <path d="M9 18h2" /> <path d="M15 18c0 .796 .316 1.559 .879 2.121c.563 .563 1.326 .879 2.121 .879c.796 0 1.559 -.316 2.121 -.879c.563 -.563 .879 -1.326 .879 -2.121c0 -.796 -.316 -1.559 -.879 -2.121c-.563 -.563 -1.326 -.879 -2.121 -.879c-.796 0 -1.559 .316 -2.121 .879c-.563 .563 -.879 1.326 -.879 2.121" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const FILTER_2_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h3.5" /> <path d="M16 22l5 -5" /> <path d="M21 17v4.5" /> <path d="M21 17h-4.5" /> </svg>"##;
const FILTER_2_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h9.5" /> <path d="M9 18h2.5" /> <path d="M19 21.5c.205 -.849 .641 -1.625 1.258 -2.242c.617 -.617 1.393 -1.053 2.242 -1.258c-.849 -.205 -1.625 -.641 -2.242 -1.258c-.617 -.617 -1.053 -1.393 -1.258 -2.242c-.205 .849 -.641 1.625 -1.258 2.242c-.617 .617 -1.393 1.053 -2.242 1.258c.849 .205 1.625 .641 2.242 1.258c.617 .617 1.053 1.393 1.258 2.242" /> </svg>"##;
const FILTER_2_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h3" /> <path d="M19 22v-6m0 0l3 3m-3 -3l-3 3" /> </svg>"##;
const FILTER_2_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M6 12h12" /> <path d="M9 18h4" /> <path d="M22 22l-5 -5m0 5l5 -5" /> </svg>"##;
const FILTER_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.991 19.67l-3.991 1.33v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v3" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const FILTER_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-3 1v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v1.5" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const FILTER_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.18 20.274l-2.18 .726v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v3" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const FILTER_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.19 20.27l-2.19 .73v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v1.5" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const FILTER_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-3 1v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v1.5" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const FILTER_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.705 19.765l-3.705 1.235v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v.5" /> <path d="M16 21l5 -5" /> <path d="M21 21v.01" /> <path d="M16 16v.01" /> </svg>"##;
const FILTER_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.25 19.583l-4.25 1.417v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const FILTER_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-3 1v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v3" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const FILTER_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.97 20.344l-1.97 .656v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v1.5" /> <path d="M18.42 15.61a2.1 2.1 0 0 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const FILTER_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v7l-6 2v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const FILTER_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.888 20.37l-1.888 .63v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-3.503 3.503" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const FILTER_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-3 1v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v3" /> <path d="M16 19h6" /> </svg>"##;
const FILTER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h12v2.172a2 2 0 0 1 -.586 1.414l-3.914 3.914m-.5 3.5v4l-6 2v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227" /> <path d="M3 3l18 18" /> </svg>"##;
const FILTER_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.97 19.677l-3.97 1.323v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v1.5" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const FILTER_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-3 1v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const FILTER_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-3 1v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v3" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const FILTER_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 19l-6 2v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const FILTER_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.36 20.213l-2.36 .787v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const FILTER_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.713 19.762l-3.713 1.238v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v1" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const FILTER_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 12.5v-.5l4.414 -4.414a2 2 0 0 0 .586 -1.414v-2.172h-16v2.227c0 .497 .185 .977 .52 1.345l4.48 4.928v8.5l2 -.667" /> <path d="M18.5 22a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const FILTER_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.042 20.32l-2.042 .68v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const FILTER_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-3 1v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v2" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const FILTER_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.758 19.414l-4.758 1.586v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v1.5" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const FILTERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M8 11a5 5 0 1 0 3.998 1.997" /> <path d="M12.002 19.003a5 5 0 1 0 3.998 -8.003" /> </svg>"##;
const FINGERPRINT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.9 7a8 8 0 0 1 1.1 5v1a6 6 0 0 0 .8 3" /> <path d="M8 11a4 4 0 0 1 8 0v1a10 10 0 0 0 2 6" /> <path d="M12 11v2a14 14 0 0 0 2.5 8" /> <path d="M8 15a18 18 0 0 0 1.8 6" /> <path d="M4.9 19a22 22 0 0 1 -.9 -7v-1a8 8 0 0 1 12 -6.95" /> </svg>"##;
const FINGERPRINT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.9 7a8 8 0 0 1 1.1 5v1a6 6 0 0 0 .8 3" /> <path d="M8 11c0 -.848 .264 -1.634 .713 -2.28m2.4 -1.621a4 4 0 0 1 4.887 3.901l0 1" /> <path d="M12 12v1a14 14 0 0 0 2.5 8" /> <path d="M8 15a18 18 0 0 0 1.8 6" /> <path d="M4.9 19a22 22 0 0 1 -.9 -7v-1a8 8 0 0 1 1.854 -5.143m2.176 -1.825a8 8 0 0 1 7.97 .018" /> <path d="M3 3l18 18" /> </svg>"##;
const FINGERPRINT_SCAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 0 1 6 0c0 1.657 .612 3.082 1 4" /> <path d="M12 11v1.75c-.001 1.11 .661 2.206 1 3.25" /> <path d="M9 14.25c.068 .58 .358 1.186 .5 1.75" /> <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const FIRE_EXTINGUISHER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 7a4 4 0 0 1 4 4v9a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1v-9a4 4 0 0 1 4 -4" /> <path d="M9 16h6" /> <path d="M12 7v-3" /> <path d="M16 5l-4 -1l4 -1" /> <path d="M12 4h-3a3 3 0 0 0 -3 3" /> </svg>"##;
const FIREWALL_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 3v18" /> <path d="M3 9h18" /> <path d="M3 15h10" /> <path d="M15 3v10" /> <path d="M11 21h-6a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v8" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const FIREWALL_FLAME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.5 16q 2.5 1.5 2.5 -1v-2s4 1.06 4 5c0 1.664 -.649 3.338 -2 4v-.25c0 -.957 -1.053 -1.75 -2 -1.75s-2 .793 -2 1.75v.25c-1.351 -.662 -2 -2 -2 -3.5s1.5 -2.5 1.5 -2.5" /> <path d="M9 3v13" /> <path d="M3 9h18" /> <path d="M6 21h-1a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4" /> <path d="M3 15h7" /> <path d="M15 3v7" /> </svg>"##;
const FIRST_AID_KIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8v-2a2 2 0 0 1 2 -2h4a2 2 0 0 1 2 2v2" /> <path d="M4 10a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -8" /> <path d="M10 14h4" /> <path d="M12 12v4" /> </svg>"##;
const FIRST_AID_KIT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.595 4.577a2 2 0 0 1 1.405 -.577h4a2 2 0 0 1 2 2v2" /> <path d="M12 8h6a2 2 0 0 1 2 2v6m-.576 3.405a2 2 0 0 1 -1.424 .595h-12a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h2" /> <path d="M10 14h4" /> <path d="M12 12v4" /> <path d="M3 3l18 18" /> </svg>"##;
const FISH_CHRISTIANITY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 7s-5.646 10 -12.308 10c-3.226 .025 -6.194 -1.905 -7.692 -5c1.498 -3.095 4.466 -5.025 7.692 -5c6.662 0 12.308 10 12.308 10" /> </svg>"##;
const FLAG_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 21v-6m2 0v-1.5m0 9v-1.5m-2 -3h3m-1 0h.5a1.5 1.5 0 0 1 0 3h-3.5m3 -3h.5a1.5 1.5 0 0 0 0 -3h-3.5" /> <path d="M13.222 14.882a5 5 0 0 1 -1.222 -.882a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v5" /> <path d="M5 21v-7" /> </svg>"##;
const FLAG_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.804 14.641a5.02 5.02 0 0 1 -.804 -.641a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v8" /> <path d="M5 21v-7" /> <path d="M16 21l5 -5" /> <path d="M21 21v.01" /> <path d="M16 16v.01" /> </svg>"##;
const FLASK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 3l6 0" /> <path d="M10 9l4 0" /> <path d="M10 3v6l-4 11a.7 .7 0 0 0 .5 1h11a.7 .7 0 0 0 .5 -1l-4 -11v-6" /> </svg>"##;
const FLASK_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.1 15h11.8" /> <path d="M14 3v7.342a6 6 0 0 1 1.318 10.658h-6.635a6 6 0 0 1 1.317 -10.66v-7.34h4" /> <path d="M9 3h6" /> </svg>"##;
const FLASK_2_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.1 15h8.9" /> <path d="M17.742 17.741a6 6 0 0 1 -2.424 3.259h-6.635a6 6 0 0 1 1.317 -10.66v-.326m0 -4.014v-3h4v7m.613 .598a6 6 0 0 1 2.801 2.817" /> <path d="M9 3h6" /> <path d="M3 3l18 18" /> </svg>"##;
const FLASK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 3h6" /> <path d="M13 9h1" /> <path d="M10 3v3m-.268 3.736l-3.732 10.264a.7 .7 0 0 0 .5 1h11a.7 .7 0 0 0 .5 -1l-1.143 -3.142m-2.288 -6.294l-.569 -1.564v-6" /> <path d="M3 3l18 18" /> </svg>"##;
const FOCUS_CENTERED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const FOODSTEPS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 16.5a2.5 2.5 0 0 0 5 0a1.5 1.5 0 0 0 -1.5 -1.5h-2a1.5 1.5 0 0 0 -1.5 1.5" /> <path d="M15 18.5a2.5 2.5 0 0 0 5 0a1.5 1.5 0 0 0 -1.5 -1.5h-2a1.5 1.5 0 0 0 -1.5 1.5" /> <path d="M8.52 12h-4.04c-.348 0 -.678 -.179 -.823 -.496c-1.326 -2.904 -.774 -8.504 2.843 -8.504s4.17 5.6 2.843 8.504c-.145 .317 -.475 .496 -.824 .496" /> <path d="M19.52 14h-4.04c-.348 0 -.678 -.179 -.823 -.496c-1.326 -2.904 -.774 -8.504 2.843 -8.504s4.17 5.6 2.843 8.504c-.145 .317 -.475 .496 -.824 .496" /> </svg>"##;
const FORBID_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 9l6 6" /> </svg>"##;
const FORBID_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 15l6 -6" /> </svg>"##;
const FREE_RIGHTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M13.867 9.75c-.246 -.48 -.708 -.769 -1.2 -.75h-1.334c-.736 0 -1.333 .67 -1.333 1.5c0 .827 .597 1.499 1.333 1.499h1.334c.736 0 1.333 .671 1.333 1.5c0 .828 -.597 1.499 -1.333 1.499h-1.334c-.492 .019 -.954 -.27 -1.2 -.75" /> <path d="M12 7v2" /> <path d="M12 15v2" /> <path d="M6 6l1.5 1.5" /> <path d="M16.5 16.5l1.5 1.5" /> </svg>"##;
const FRIENDS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 22v-5l-1 -1v-4a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4l-1 1v5" /> <path d="M15 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 22v-4h-2l2 -6a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1l2 6h-2v4" /> </svg>"##;
const FRIENDS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a2 2 0 0 0 2 2m2 -2a2 2 0 0 0 -2 -2" /> <path d="M5 22v-5l-1 -1v-4a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4l-1 1v5" /> <path d="M15 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 22v-4h-2l1.254 -3.763m1.036 -2.942a1 1 0 0 1 .71 -.295h2a1 1 0 0 1 1 1l1.503 4.508m-1.503 2.492v3" /> <path d="M3 3l18 18" /> </svg>"##;
const FUNCTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6.667a2.667 2.667 0 0 1 2.667 -2.667h10.666a2.667 2.667 0 0 1 2.667 2.667v10.666a2.667 2.667 0 0 1 -2.667 2.667h-10.666a2.667 2.667 0 0 1 -2.667 -2.667l0 -10.666" /> <path d="M9 15.5v.25c0 .69 .56 1.25 1.25 1.25c.71 0 1.304 -.538 1.374 -1.244l.752 -7.512a1.381 1.381 0 0 1 1.374 -1.244c.69 0 1.25 .56 1.25 1.25v.25" /> <path d="M9 12h6" /> </svg>"##;
const FUNCTION_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15.5v.25c0 .69 .56 1.25 1.25 1.25a1.38 1.38 0 0 0 1.374 -1.244l.376 -3.756m.363 -3.63l.013 -.126a1.38 1.38 0 0 1 1.374 -1.244c.69 0 1.25 .56 1.25 1.25v.25" /> <path d="M8 4h10a2 2 0 0 1 2 2v10m-.586 3.414a2 2 0 0 1 -1.414 .586h-12a2 2 0 0 1 -2 -2v-12c0 -.547 .22 -1.043 .576 -1.405" /> <path d="M9 12h3" /> <path d="M3 3l18 18" /> </svg>"##;
const GALAXY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3c-1.333 1 -2 2.5 -2 4.5c0 3 2 4.5 2 4.5s2 1.5 2 4.5c0 2 -.667 3.5 -2 4.5" /> <path d="M19.794 16.5c-.2 -1.655 -1.165 -2.982 -2.897 -3.982c-2.597 -1.5 -4.897 -.518 -4.897 -.518s-2.299 .982 -4.897 -.518c-1.732 -1 -2.698 -2.327 -2.897 -3.982" /> <path d="M19.794 7.5c-1.532 -.655 -3.165 -.482 -4.897 .518c-2.597 1.5 -2.897 3.982 -2.897 3.982s-.299 2.482 -2.897 3.982c-1.732 1 -3.365 1.173 -4.897 .518" /> </svg>"##;
const GAUGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M13.41 10.59l2.59 -2.59" /> <path d="M7 12a5 5 0 0 1 5 -5" /> </svg>"##;
const GAUGE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.038 16.052a9 9 0 0 0 -12.067 -12.102m-2.333 1.686a9 9 0 1 0 12.73 12.726" /> <path d="M11.283 11.303a1 1 0 0 0 1.419 1.41" /> <path d="M14 10l2 -2" /> <path d="M7 12c0 -1.386 .564 -2.64 1.475 -3.546m2.619 -1.372c.294 -.054 .597 -.082 .906 -.082" /> <path d="M3 3l18 18" /> </svg>"##;
const GAVEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 10l7.383 7.418c.823 .82 .823 2.148 0 2.967a2.11 2.11 0 0 1 -2.976 0l-7.407 -7.385" /> <path d="M6 9l4 4" /> <path d="M13 10l-4 -4" /> <path d="M3 21h7" /> <path d="M6.793 15.793l-3.586 -3.586a1 1 0 0 1 0 -1.414l2.293 -2.293l.5 .5l3 -3l-.5 -.5l2.293 -2.293a1 1 0 0 1 1.414 0l3.586 3.586a1 1 0 0 1 0 1.414l-2.293 2.293l-.5 -.5l-3 3l.5 .5l-2.293 2.293a1 1 0 0 1 -1.414 0" /> </svg>"##;
const GRAPH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2" /> <path d="M7 14l3 -3l2 2l3 -3l2 2" /> </svg>"##;
const GRAPH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h10a2 2 0 0 1 2 2v10m-.586 3.414a2 2 0 0 1 -1.414 .586h-12a2 2 0 0 1 -2 -2v-12c0 -.547 .22 -1.043 .576 -1.405" /> <path d="M7 14l3 -3l2 2l.5 -.5m2 -2l.5 -.5l2 2" /> <path d="M3 3l18 18" /> </svg>"##;
const GRID_DOTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M18 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M4 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M18 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M4 19a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 19a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M18 19a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const GRID_PATTERN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M10 8v8" /> <path d="M14 8v8" /> <path d="M8 10h8" /> <path d="M8 14h8" /> </svg>"##;
const GRID_SCAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v8" /> <path d="M14 8v8" /> <path d="M8 10h8" /> <path d="M8 14h8" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const GRIP_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 9a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M4 15a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 9a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 15a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M18 9a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M18 15a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const GRIP_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M8 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M8 19a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M14 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M14 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M14 19a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const GYMNASTICS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M13 21l1 -9l7 -6" /> <path d="M3 11h6l5 1" /> <path d="M11.5 8.5l4.5 -3.5" /> </svg>"##;
const HAND_SANITIZER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 21h10v-10a3 3 0 0 0 -3 -3h-4a3 3 0 0 0 -3 3v10" /> <path d="M15 3h-6a2 2 0 0 0 -2 2" /> <path d="M12 3v5" /> <path d="M12 11v4" /> <path d="M10 13h4" /> </svg>"##;
const HANGER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9l-7.971 4.428a2 2 0 0 0 -1.029 1.749v.823a2 2 0 0 0 2 2h1" /> <path d="M18 18h1a2 2 0 0 0 2 -2v-.823a2 2 0 0 0 -1.029 -1.749l-7.971 -4.428c-1.457 -.81 -1.993 -2.333 -2 -4a2 2 0 1 1 4 0" /> <path d="M6 18a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v1a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -1" /> </svg>"##;
const HAZE_MOON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 16h18" /> <path d="M3 20h18" /> <path d="M8.296 16c-2.268 -1.4 -3.598 -4.087 -3.237 -6.916c.443 -3.48 3.308 -6.083 6.698 -6.084v.006h.296c-1.991 1.916 -2.377 5.03 -.918 7.405c1.459 2.374 4.346 3.33 6.865 2.275a6.888 6.888 0 0 1 -2.777 3.314" /> </svg>"##;
const HEALTH_RECOGNITION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> <path d="M8.603 9.61a2.04 2.04 0 0 1 2.912 0l.485 .39l.5 -.396a2.035 2.035 0 0 1 2.897 .007a2.104 2.104 0 0 1 0 2.949l-3.397 3.44l-3.397 -3.44a2.104 2.104 0 0 1 0 -2.95" /> </svg>"##;
const HEART_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 21v-6m2 0v-1.5m0 9v-1.5m-2 -3h3m-1 0h.5a1.5 1.5 0 0 1 0 3h-3.5m3 -3h.5a1.5 1.5 0 0 0 0 -3h-3.5" /> <path d="M13 19l-1 1l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 0 1 8.785 4.444" /> </svg>"##;
const HEART_BROKEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.5 12.572l-7.5 7.428l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 1 1 7.5 6.572" /> <path d="M12 6l-2 4l4 3l-2 4v3" /> </svg>"##;
const HEART_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19l-1 1l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 1 1 7.5 6.572" /> <path d="M16 21l5 -5" /> <path d="M21 21v.01" /> <path d="M16 16v.01" /> </svg>"##;
const HEART_HANDSHAKE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.5 12.572l-7.5 7.428l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 1 1 7.5 6.572" /> <path d="M12 6l-3.293 3.293a1 1 0 0 0 0 1.414l.543 .543c.69 .69 1.81 .69 2.5 0l1 -1a3.182 3.182 0 0 1 4.5 0l2.25 2.25" /> <path d="M12.5 15.5l2 2" /> <path d="M15 13l2 2" /> </svg>"##;
const HEART_RATE_MONITOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1l0 -10" /> <path d="M7 20h10" /> <path d="M9 16v4" /> <path d="M15 16v4" /> <path d="M7 10h2l2 3l2 -6l1 3h3" /> </svg>"##;
const HEARTBEAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.5 13.572l-7.5 7.428l-2.896 -2.868m-6.117 -8.104a5 5 0 0 1 9.013 -3.022a5 5 0 1 1 7.5 6.572" /> <path d="M3 13h2l2 3l2 -6l1 3h3" /> </svg>"##;
const HELP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 17l0 .01" /> <path d="M12 13.5a1.5 1.5 0 0 1 1 -1.5a2.6 2.6 0 1 0 -3 -4" /> </svg>"##;
const HELP_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M12 16v.01" /> <path d="M12 13a2 2 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const HELP_HEXAGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27c.7 .398 1.13 1.143 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M12 16v.01" /> <path d="M12 13a2 2 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const HELP_OCTAGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.802 2.165l5.575 2.389c.48 .206 .863 .589 1.07 1.07l2.388 5.574c.22 .512 .22 1.092 0 1.604l-2.389 5.575c-.206 .48 -.589 .863 -1.07 1.07l-5.574 2.388c-.512 .22 -1.092 .22 -1.604 0l-5.575 -2.389a2.036 2.036 0 0 1 -1.07 -1.07l-2.388 -5.574a2.036 2.036 0 0 1 0 -1.604l2.389 -5.575c.206 -.48 .589 -.863 1.07 -1.07l5.574 -2.388a2.036 2.036 0 0 1 1.604 0" /> <path d="M12 16v.01" /> <path d="M12 13a2 2 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const HELP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.641 5.631a9 9 0 1 0 12.719 12.738m1.68 -2.318a9 9 0 0 0 -12.074 -12.098" /> <path d="M12 17v.01" /> <path d="M12 13.5a1.5 1.5 0 0 1 .394 -1.1m2.106 -1.9a2.6 2.6 0 0 0 -3.347 -3.361" /> <path d="M3 3l18 18" /> </svg>"##;
const HELP_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 16v.01" /> <path d="M12 13a2 2 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const HELP_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M12 16v.01" /> <path d="M12 13a2 2 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const HELP_SQUARE_ROUNDED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> <path d="M12 16v.01" /> <path d="M12 13a2 2 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const HELP_TRIANGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 16v.01" /> <path d="M10.363 3.591l-8.106 13.534a1.914 1.914 0 0 0 1.636 2.871h16.214a1.914 1.914 0 0 0 1.636 -2.87l-8.106 -13.536a1.914 1.914 0 0 0 -3.274 0" /> <path d="M12 13a2 2 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const HISTORY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8l0 4l2 2" /> <path d="M3.05 11a9 9 0 1 1 .5 4m-.5 5v-5h5" /> </svg>"##;
const HISTORY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.05 11a8.975 8.975 0 0 1 2.54 -5.403m2.314 -1.697a9 9 0 0 1 12.113 12.112m-1.695 2.312a9 9 0 0 1 -14.772 -3.324m-.5 5v-5h5" /> <path d="M3 3l18 18" /> </svg>"##;
const HISTORY_TOGGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20.777a8.942 8.942 0 0 1 -2.48 -.969" /> <path d="M14 3.223a9.003 9.003 0 0 1 0 17.554" /> <path d="M4.579 17.093a8.961 8.961 0 0 1 -1.227 -2.592" /> <path d="M3.124 10.5c.16 -.95 .468 -1.85 .9 -2.675l.169 -.305" /> <path d="M6.907 4.579a8.954 8.954 0 0 1 3.093 -1.356" /> <path d="M12 8v4l3 3" /> </svg>"##;
const HOME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12l-2 0l9 -9l9 9l-2 0" /> <path d="M5 12v7a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-7" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v6" /> </svg>"##;
const HOME_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12l-2 0l9 -9l9 9l-2 0" /> <path d="M5 12v7a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-7" /> <path d="M10 12h4v4h-4l0 -4" /> </svg>"##;
const HOME_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 21v-6m2 0v-1.5m0 9v-1.5m-2 -3h3m-1 0h.5a1.5 1.5 0 0 1 0 3h-3.5m3 -3h.5a1.5 1.5 0 0 0 0 -3h-3.5" /> <path d="M19.5 10.5l-7.5 -7.5l-9 9h2v7a2 2 0 0 0 2 2h6" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2c.387 0 .748 .11 1.054 .3" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> </svg>"##;
const HOME_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 10l-7 -7l-9 9h2v7a2 2 0 0 0 2 2h7.5" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2c.661 0 1.248 .32 1.612 .815" /> <path d="M19 14l-2 4h4l-2 4" /> </svg>"##;
const HOME_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> <path d="M19 12h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h5.5" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2c.58 0 1.103 .247 1.468 .642" /> </svg>"##;
const HOME_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2" /> <path d="M19 13.488v-1.488h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h4.525" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const HOME_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 21v-6a2 2 0 0 1 2 -2h1.6" /> <path d="M20 11l-8 -8l-9 9h2v7a2 2 0 0 0 2 2h4.159" /> <path d="M16 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M18 14.5v1.5" /> <path d="M18 20v1.5" /> <path d="M21.032 16.25l-1.299 .75" /> <path d="M16.27 19l-1.3 .75" /> <path d="M14.97 16.25l1.3 .75" /> <path d="M19.733 19l1.3 .75" /> </svg>"##;
const HOME_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 10l-7 -7l-9 9h2v7a2 2 0 0 0 2 2h6" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2c.387 0 .748 .11 1.054 .3" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const HOME_DOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 12h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h5" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2c.641 0 1.212 .302 1.578 .771" /> </svg>"##;
const HOME_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 12h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h5.5" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const HOME_ECO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 11l-8 -8l-9 9h2v7a2 2 0 0 0 2 2h5" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2c.325 0 .631 .077 .902 .215" /> <path d="M16 22s0 -2 3 -4" /> <path d="M19 21a3 3 0 0 1 0 -6h3v3a3 3 0 0 1 -3 3" /> </svg>"##;
const HOME_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 21v-6a2 2 0 0 1 2 -2h2c.645 0 1.218 .305 1.584 .78" /> <path d="M20 11l-8 -8l-9 9h2v7a2 2 0 0 0 2 2h4" /> <path d="M18.42 15.61a2.1 2.1 0 0 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const HOME_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h8" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 1.857 1.257" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const HOME_HAND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9l-6 -6l-9 9h2v7a2 2 0 0 0 2 2h3.5" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2" /> <path d="M16 17.5l-.585 -.578a1.516 1.516 0 0 0 -2 0c-.477 .433 -.551 1.112 -.177 1.622l1.762 2.456c.37 .506 1.331 1 2 1h3c1.009 0 1.497 -.683 1.622 -1.593c.252 -.938 .378 -1.74 .378 -2.407c0 -1 -.939 -1.843 -2 -2h-1v-2.636c0 -.754 -.672 -1.364 -1.5 -1.364s-1.5 .61 -1.5 1.364v4.136" /> </svg>"##;
const HOME_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h6" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2c.39 0 .754 .112 1.061 .304" /> <path d="M19 21.5l2.518 -2.58a1.74 1.74 0 0 0 0 -2.413a1.627 1.627 0 0 0 -2.346 0l-.168 .172l-.168 -.172a1.627 1.627 0 0 0 -2.346 0a1.74 1.74 0 0 0 0 2.412l2.51 2.59l0 -.009" /> </svg>"##;
const HOME_INFINITY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 14v-2h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h2.5" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 1.75 1.032" /> <path d="M15.536 17.586a2.123 2.123 0 0 0 -2.929 0a1.951 1.951 0 0 0 0 2.828c.809 .781 2.12 .781 2.929 0c.809 -.781 -.805 .778 0 0l1.46 -1.41l1.46 -1.419" /> <path d="M15.54 17.582l1.46 1.42l1.46 1.41c.809 .78 -.805 -.779 0 0s2.12 .781 2.929 0a1.951 1.951 0 0 0 0 -2.828a2.123 2.123 0 0 0 -2.929 0" /> </svg>"##;
const HOME_LINK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.085 11.085l-8.085 -8.085l-9 9h2v7a2 2 0 0 0 2 2h4.5" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 1.807 1.143" /> <path d="M20 21a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M20 16a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M15 19a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M21 16l-5 3l5 2" /> </svg>"##;
const HOME_LOCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h-2l9 -9l8 8" /> <path d="M5 12v7a2 2 0 0 0 2 2h6" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2c.688 0 1.294 .347 1.654 .875" /> <path d="M17 19a1 1 0 0 1 1 -1h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-3a1 1 0 0 1 -1 -1v-2" /> <path d="M18 18v-1.5a1.5 1.5 0 1 1 3 0v1.5" /> </svg>"##;
const HOME_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 15v-3h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h5.5" /> <path d="M16 19h6" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2" /> </svg>"##;
const HOME_MOVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2" /> <path d="M19 12h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h5.5" /> <path d="M16 19h6" /> <path d="M19 16l3 3l-3 3" /> </svg>"##;
const HOME_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h-2l4.497 -4.497m2 -2l2.504 -2.504l9 9h-2" /> <path d="M5 12v7a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2m0 -4v-3" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2m2 2v6" /> <path d="M3 3l18 18" /> </svg>"##;
const HOME_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 12h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h5.5" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const HOME_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.136 11.136l-8.136 -8.136l-9 9h2v7a2 2 0 0 0 2 2h7" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2c.467 0 .896 .16 1.236 .428" /> <path d="M19 22v.01" /> <path d="M19 19a2 2 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const HOME_RIBBON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 15h5v7l-2.5 -1.5l-2.5 1.5l0 -7" /> <path d="M20 11l-8 -8l-9 9h2v7a2 2 0 0 0 2 2h5" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h1.5" /> </svg>"##;
const HOME_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h4.7" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const HOME_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 21v-6a2 2 0 0 1 2 -2h2c.247 0 .484 .045 .702 .127" /> <path d="M19 12h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h5" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const HOME_SHIELD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h-2l9 -9l7.636 7.636" /> <path d="M5 12v7a2 2 0 0 0 2 2h5" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h1.5" /> <path d="M22 16c0 4 -2.5 6 -3.5 6s-3.5 -2 -3.5 -6c1 0 2.5 -.5 3.5 -1.5c1 1 2.5 1.5 3.5 1.5" /> </svg>"##;
const HOME_SIGNAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 22v-2" /> <path d="M18 22v-4" /> <path d="M21 22v-6" /> <path d="M19 12.494v-.494h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h4" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v.5" /> </svg>"##;
const HOME_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h-2l9 -9l9 9h-2" /> <path d="M5 12v7a2 2 0 0 0 2 2h5" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const HOME_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.258 10.258l-7.258 -7.258l-9 9h2v7a2 2 0 0 0 2 2h4" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h1.5" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const HOME_STATS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 13v-1h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h2.5" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2" /> <path d="M13 22l3 -3l2 2l4 -4" /> <path d="M19 17h3v3" /> </svg>"##;
const HOME_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 21v-6a2 2 0 0 1 2 -2h2c.641 0 1.212 .302 1.578 .771" /> <path d="M20.136 11.136l-8.136 -8.136l-9 9h2v7a2 2 0 0 0 2 2h6.344" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const HOME_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 13.4v-1.4h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h5.5" /> <path d="M9 21v-6a2 2 0 0 1 2 -2h2c.402 0 .777 .119 1.091 .323" /> <path d="M21.5 21.5l-5 -5" /> <path d="M16.5 21.5l5 -5" /> </svg>"##;
const HOSPITAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 16v-8" /> <path d="M14 16v-8" /> <path d="M10 12h4" /> </svg>"##;
const HOSPITAL_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-8" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M14 16v-8" /> <path d="M10 12h4" /> </svg>"##;
const HOTEL_SERVICE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.5 10a1.5 1.5 0 0 1 -1.5 -1.5a5.5 5.5 0 0 1 11 0v10.5a2 2 0 0 1 -2 2h-7a2 2 0 0 1 -2 -2v-2c0 -1.38 .71 -2.444 1.88 -3.175l4.424 -2.765c1.055 -.66 1.696 -1.316 1.696 -2.56a2.5 2.5 0 1 0 -5 0a1.5 1.5 0 0 1 -1.5 1.5" /> </svg>"##;
const HOURGLASS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.5 7h11" /> <path d="M6.5 17h11" /> <path d="M6 20v-2a6 6 0 1 1 12 0v2a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1" /> <path d="M6 4v2a6 6 0 1 0 12 0v-2a1 1 0 0 0 -1 -1h-10a1 1 0 0 0 -1 1" /> </svg>"##;
const HOURGLASS_EMPTY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 20v-2a6 6 0 1 1 12 0v2a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1" /> <path d="M6 4v2a6 6 0 1 0 12 0v-2a1 1 0 0 0 -1 -1h-10a1 1 0 0 0 -1 1" /> </svg>"##;
const HOURGLASS_HIGH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.5 7h11" /> <path d="M6 20v-2a6 6 0 1 1 12 0v2a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1" /> <path d="M6 4v2a6 6 0 1 0 12 0v-2a1 1 0 0 0 -1 -1h-10a1 1 0 0 0 -1 1" /> </svg>"##;
const HOURGLASS_LOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.5 17h11" /> <path d="M6 20v-2a6 6 0 1 1 12 0v2a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1" /> <path d="M6 4v2a6 6 0 1 0 12 0v-2a1 1 0 0 0 -1 -1h-10a1 1 0 0 0 -1 1" /> </svg>"##;
const HOURGLASS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 18v2a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1v-2a6 6 0 0 1 6 -6" /> <path d="M6 6a6 6 0 0 0 6 6m3.13 -.88a6 6 0 0 0 2.87 -5.12v-2a1 1 0 0 0 -1 -1h-10" /> <path d="M3 3l18 18" /> </svg>"##;
const HOURS_12_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -4v4h4" /> <path d="M4 13c.468 3.6 3.384 6.546 7 7" /> <path d="M18 15h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2" /> <path d="M15 21v-6" /> </svg>"##;
const HOURS_24_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 13c.325 2.532 1.881 4.781 4 6" /> <path d="M20 11a8.1 8.1 0 0 0 -15.5 -2" /> <path d="M4 5v4h4" /> <path d="M12 15h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2" /> <path d="M18 15v2a1 1 0 0 0 1 1h1" /> <path d="M21 15v6" /> </svg>"##;
const ID_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v10a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3l0 -10" /> <path d="M7 10a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 8l2 0" /> <path d="M15 12l2 0" /> <path d="M7 16l10 0" /> </svg>"##;
const ID_BADGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 6a3 3 0 0 1 3 -3h8a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-8a3 3 0 0 1 -3 -3l0 -12" /> <path d="M10 13a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 6h4" /> <path d="M9 18h6" /> </svg>"##;
const ID_BADGE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 12h3v4h-3l0 -4" /> <path d="M10 6h-6a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h16a1 1 0 0 0 1 -1v-12a1 1 0 0 0 -1 -1h-6" /> <path d="M10 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -3" /> <path d="M14 16h2" /> <path d="M14 12h4" /> </svg>"##;
const ID_BADGE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.141 3.125a3 3 0 0 1 .859 -.125h8a3 3 0 0 1 3 3v9m-.13 3.874a3 3 0 0 1 -2.87 2.126h-8a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 .128 -.869" /> <path d="M11.179 11.176a2 2 0 1 0 2.635 2.667" /> <path d="M10 6h4" /> <path d="M9 18h6" /> <path d="M3 3l18 18" /> </svg>"##;
const ID_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h10a3 3 0 0 1 3 3v10m-1.437 2.561c-.455 .279 -.99 .439 -1.563 .439h-12a3 3 0 0 1 -3 -3v-10c0 -1.083 .573 -2.031 1.433 -2.559" /> <path d="M8.175 8.178a2 2 0 1 0 2.646 2.65" /> <path d="M15 8h2" /> <path d="M16 12h1" /> <path d="M7 16h9" /> <path d="M3 3l18 18" /> </svg>"##;
const IMAGE_GENERATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21v-4a4 4 0 1 1 4 4h-4" /> <path d="M21 3a16 16 0 0 0 -12.8 10.2" /> <path d="M21 3a16 16 0 0 1 -10.2 12.8" /> <path d="M10.6 9a9 9 0 0 1 4.4 4.4" /> <path d="M17 19a2 2 0 0 1 2 2a2 2 0 0 1 2 -2a2 2 0 0 1 -2 -2a2 2 0 0 1 -2 2" /> <path d="M3 5a2 2 0 0 1 2 2a2 2 0 0 1 2 -2a2 2 0 0 1 -2 -2a2 2 0 0 1 -2 2" /> </svg>"##;
const INBOX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M4 13h3l3 3h4l3 -3h3" /> </svg>"##;
const INBOX_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h10a2 2 0 0 1 2 2v10m-.593 3.422a2 2 0 0 1 -1.407 .578h-12a2 2 0 0 1 -2 -2v-12c0 -.554 .225 -1.056 .59 -1.418" /> <path d="M4 13h3l3 3h4l.987 -.987m2.013 -2.013h3" /> <path d="M3 3l18 18" /> </svg>"##;
const INFINITY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.828 9.172a4 4 0 1 0 0 5.656a10 10 0 0 0 2.172 -2.828a10 10 0 0 1 2.172 -2.828a4 4 0 1 1 0 5.656a10 10 0 0 1 -2.172 -2.828a10 10 0 0 0 -2.172 -2.828" /> </svg>"##;
const INFINITY_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.94 9.39a10 10 0 0 1 .232 -.218a4 4 0 1 1 0 5.656a10 10 0 0 1 -2.172 -2.828a10 10 0 0 0 -2.172 -2.828a4 4 0 1 0 0 5.656a10 10 0 0 0 .234 -.219" /> </svg>"##;
const INFINITY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.165 8.174a4 4 0 0 0 -5.166 3.826a4 4 0 0 0 6.829 2.828a10 10 0 0 0 2.172 -2.828m1.677 -2.347a10 10 0 0 1 .495 -.481a4 4 0 1 1 5.129 6.1m-3.521 .537a4 4 0 0 1 -1.608 -.981a10 10 0 0 1 -2.172 -2.828" /> <path d="M3 3l18 18" /> </svg>"##;
const INFO_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M12 9h.01" /> <path d="M11 12h1v4h1" /> </svg>"##;
const INFO_HEXAGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27c.7 .398 1.13 1.143 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M12 9h.01" /> <path d="M11 12h1v4h1" /> </svg>"##;
const INFO_OCTAGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.802 2.165l5.575 2.389c.48 .206 .863 .589 1.07 1.07l2.388 5.574c.22 .512 .22 1.092 0 1.604l-2.389 5.575c-.206 .48 -.589 .863 -1.07 1.07l-5.574 2.388c-.512 .22 -1.092 .22 -1.604 0l-5.575 -2.389a2.036 2.036 0 0 1 -1.07 -1.07l-2.388 -5.574a2.036 2.036 0 0 1 0 -1.604l2.389 -5.575c.206 -.48 .589 -.863 1.07 -1.07l5.574 -2.388a2.036 2.036 0 0 1 1.604 0" /> <path d="M12 9h.01" /> <path d="M11 12h1v4h1" /> </svg>"##;
const INFO_SMALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9h.01" /> <path d="M11 12h1v4h1" /> </svg>"##;
const INFO_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9h.01" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M11 12h1v4h1" /> </svg>"##;
const INFO_SQUARE_ROUNDED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9h.01" /> <path d="M11 12h1v4h1" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const INFO_TRIANGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.363 3.591l-8.106 13.534a1.914 1.914 0 0 0 1.636 2.871h16.214a1.914 1.914 0 0 0 1.636 -2.87l-8.106 -13.536a1.914 1.914 0 0 0 -3.274 0" /> <path d="M12 9h.01" /> <path d="M11 12h1v4h1" /> </svg>"##;
const INPUT_AI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 11v-2a2 2 0 0 0 -2 -2h-12a2 2 0 0 0 -2 2v5a2 2 0 0 0 2 2h4" /> <path d="M14 21v-4a2 2 0 1 1 4 0v4" /> <path d="M14 19h4" /> <path d="M21 15v6" /> </svg>"##;
const INPUT_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 13v-4a2 2 0 0 0 -2 -2h-12a2 2 0 0 0 -2 2v5a2 2 0 0 0 2 2h6" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const INPUT_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> <path d="M20 11.5v-2.5a2 2 0 0 0 -2 -2h-12a2 2 0 0 0 -2 2v5a2 2 0 0 0 2 2h7" /> </svg>"##;
const INPUT_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 13v-4a2 2 0 0 0 -2 -2h-12a2 2 0 0 0 -2 2v5a2 2 0 0 0 2 2h7" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const JETPACK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 6a3 3 0 1 0 -6 0v7h6v-7" /> <path d="M14 13h6v-7a3 3 0 0 0 -6 0v7" /> <path d="M5 16c0 2.333 .667 4 2 5c1.333 -1 2 -2.667 2 -5" /> <path d="M15 16c0 2.333 .667 4 2 5c1.333 -1 2 -2.667 2 -5" /> <path d="M10 8h4" /> <path d="M10 11h4" /> </svg>"##;
const JOIN_BEVEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4h3a2 2 0 0 1 2 2v6a1 1 0 0 0 1 1h6a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-6.586a1 1 0 0 1 -.707 -.293l-6.414 -6.414a1 1 0 0 1 -.293 -.707v-6.586a2 2 0 0 1 2 -2" /> </svg>"##;
const JOIN_ROUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4h3a2 2 0 0 1 2 2v6a1 1 0 0 0 1 1h6a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-6a8 8 0 0 1 -8 -8v-6a2 2 0 0 1 2 -2" /> </svg>"##;
const JOIN_STRAIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4h3a2 2 0 0 1 2 2v6a1 1 0 0 0 1 1h6a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2" /> </svg>"##;
const LABEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.52 7h-10.52a2 2 0 0 0 -2 2v6a2 2 0 0 0 2 2h10.52a1 1 0 0 0 .78 -.375l3.7 -4.625l-3.7 -4.625a1 1 0 0 0 -.78 -.375" /> </svg>"##;
const LABEL_IMPORTANT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.52 7h-12.52l4 5l-4 5h12.52a1 1 0 0 0 .78 -.375l3.7 -4.625l-3.7 -4.625a1 1 0 0 0 -.78 -.375" /> </svg>"##;
const LABEL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7h-1a2 2 0 0 0 -2 2v6a2 2 0 0 0 2 2h10.52a1 1 0 0 0 .394 -.081m1.86 -2.137l2.226 -2.782l-3.7 -4.625a1 1 0 0 0 -.78 -.375h-5.52" /> <path d="M3 3l18 18" /> </svg>"##;
const LADDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 3v18" /> <path d="M16 3v18" /> <path d="M8 14h8" /> <path d="M8 10h8" /> <path d="M8 6h8" /> <path d="M8 18h8" /> </svg>"##;
const LADDER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 3v1m0 4v13" /> <path d="M16 3v9m0 4v5" /> <path d="M8 14h6" /> <path d="M8 10h2m4 0h2" /> <path d="M10 6h6" /> <path d="M8 18h8" /> <path d="M3 3l18 18" /> </svg>"##;
const LANE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6v13" /> <path d="M20 6v13" /> </svg>"##;
const LAYOUT_BOTTOMBAR_INACTIVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12" /> <path d="M4 15h1" /> <path d="M19 15h1" /> <path d="M9 15h1" /> <path d="M14 15h1" /> </svg>"##;
const LAYOUT_NAVBAR_INACTIVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12" /> <path d="M4 9h1" /> <path d="M19 9h1" /> <path d="M9 9h1" /> <path d="M14 9h1" /> </svg>"##;
const LAYOUT_SIDEBAR_INACTIVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12" /> <path d="M9 4v1" /> <path d="M9 9v1" /> <path d="M9 14v1" /> <path d="M9 19v1" /> </svg>"##;
const LAYOUT_SIDEBAR_RIGHT_INACTIVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12" /> <path d="M15 4v1" /> <path d="M15 9v1" /> <path d="M15 14v1" /> <path d="M15 19v1" /> </svg>"##;
const LEGO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 11l.01 0" /> <path d="M14.5 11l.01 0" /> <path d="M9.5 15a3.5 3.5 0 0 0 5 0" /> <path d="M7 5h1v-2h8v2h1a3 3 0 0 1 3 3v9a3 3 0 0 1 -3 3v1h-10v-1a3 3 0 0 1 -3 -3v-9a3 3 0 0 1 3 -3" /> </svg>"##;
const LEGO_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 11h.01" /> <path d="M9.5 15a3.5 3.5 0 0 0 5 0" /> <path d="M8 4v-1h8v2h1a3 3 0 0 1 3 3v8m-.884 3.127a2.99 2.99 0 0 1 -2.116 .873v1h-10v-1a3 3 0 0 1 -3 -3v-9c0 -1.083 .574 -2.032 1.435 -2.56" /> <path d="M3 3l18 18" /> </svg>"##;
const LIBRARY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5.667a2.667 2.667 0 0 1 2.667 -2.667h8.666a2.667 2.667 0 0 1 2.667 2.667v8.666a2.667 2.667 0 0 1 -2.667 2.667h-8.666a2.667 2.667 0 0 1 -2.667 -2.667l0 -8.666" /> <path d="M4.012 7.26a2.005 2.005 0 0 0 -1.012 1.737v10c0 1.1 .9 2 2 2h10c.75 0 1.158 -.385 1.5 -1" /> <path d="M11 7h5" /> <path d="M11 10h6" /> <path d="M11 13h3" /> </svg>"##;
const LIBRARY_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5.667a2.667 2.667 0 0 1 2.667 -2.667h8.666a2.667 2.667 0 0 1 2.667 2.667v8.666a2.667 2.667 0 0 1 -2.667 2.667h-8.666a2.667 2.667 0 0 1 -2.667 -2.667l0 -8.666" /> <path d="M4.012 7.26a2.005 2.005 0 0 0 -1.012 1.737v10c0 1.1 .9 2 2 2h10c.75 0 1.158 -.385 1.5 -1" /> <path d="M11 10h6" /> </svg>"##;
const LIBRARY_PHOTO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5.667a2.667 2.667 0 0 1 2.667 -2.667h8.666a2.667 2.667 0 0 1 2.667 2.667v8.666a2.667 2.667 0 0 1 -2.667 2.667h-8.666a2.667 2.667 0 0 1 -2.667 -2.667l0 -8.666" /> <path d="M4.012 7.26a2.005 2.005 0 0 0 -1.012 1.737v10c0 1.1 .9 2 2 2h10c.75 0 1.158 -.385 1.5 -1" /> <path d="M17 7h.01" /> <path d="M7 13l3.644 -3.644a1.21 1.21 0 0 1 1.712 0l3.644 3.644" /> <path d="M15 12l1.644 -1.644a1.21 1.21 0 0 1 1.712 0l2.644 2.644" /> </svg>"##;
const LIBRARY_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5.667a2.667 2.667 0 0 1 2.667 -2.667h8.666a2.667 2.667 0 0 1 2.667 2.667v8.666a2.667 2.667 0 0 1 -2.667 2.667h-8.666a2.667 2.667 0 0 1 -2.667 -2.667l0 -8.666" /> <path d="M4.012 7.26a2.005 2.005 0 0 0 -1.012 1.737v10c0 1.1 .9 2 2 2h10c.75 0 1.158 -.385 1.5 -1" /> <path d="M11 10h6" /> <path d="M14 7v6" /> </svg>"##;
const LIFEBUOY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M15 15l3.35 3.35" /> <path d="M9 15l-3.35 3.35" /> <path d="M5.65 5.65l3.35 3.35" /> <path d="M18.35 5.65l-3.35 3.35" /> </svg>"##;
const LIFEBUOY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.171 9.172a4 4 0 0 0 5.65 5.663m1.179 -2.835a4 4 0 0 0 -4 -4" /> <path d="M5.64 5.632a9 9 0 1 0 12.73 12.725m1.667 -2.301a9 9 0 0 0 -12.077 -12.1" /> <path d="M15 15l3.35 3.35" /> <path d="M9 15l-3.35 3.35" /> <path d="M5.65 5.65l3.35 3.35" /> <path d="M18.35 5.65l-3.35 3.35" /> <path d="M3 3l18 18" /> </svg>"##;
const LIGHTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 3v16a2 2 0 0 0 2 2h5a2 2 0 0 0 2 -2v-7h-12a2 2 0 0 1 -2 -2v-5a2 2 0 0 1 2 -2h3" /> <path d="M16 4l1.465 1.638a2 2 0 1 1 -3.015 .099l1.55 -1.737" /> </svg>"##;
const LINE_SCAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 12h10" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const LIST_LETTERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 6h9" /> <path d="M11 12h9" /> <path d="M11 18h9" /> <path d="M4 10v-4.5a1.5 1.5 0 0 1 3 0v4.5" /> <path d="M4 8h3" /> <path d="M4 20h1.5a1.5 1.5 0 0 0 0 -3h-1.5h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6" /> </svg>"##;
const LIST_TREE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 6h11" /> <path d="M12 12h8" /> <path d="M15 18h5" /> <path d="M5 6v.01" /> <path d="M8 12v.01" /> <path d="M11 18v.01" /> </svg>"##;
const LOADER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 6l0 -3" /> <path d="M16.25 7.75l2.15 -2.15" /> <path d="M18 12l3 0" /> <path d="M16.25 16.25l2.15 2.15" /> <path d="M12 18l0 3" /> <path d="M7.75 16.25l-2.15 2.15" /> <path d="M6 12l-3 0" /> <path d="M7.75 7.75l-2.15 -2.15" /> </svg>"##;
const LOADER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a9 9 0 1 0 9 9" /> </svg>"##;
const LOADER_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 0 0 9 9a9 9 0 0 0 9 -9a9 9 0 0 0 -9 -9" /> <path d="M17 12a5 5 0 1 0 -5 5" /> </svg>"##;
const LOADER_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21v-3m6.36 .36l-2.12 -2.12m4.76 -4.24h-3m.36 -6.36l-2.12 2.12m-4.24 -4.76v3m-6.36 -.36l2.12 2.12m-3.76 4.24h2m1 4.95l.71 -.71" /> </svg>"##;
const LOADER_QUARTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 6l0 -3" /> <path d="M6 12l-3 0" /> <path d="M7.75 7.75l-2.15 -2.15" /> </svg>"##;
const LOCATION_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.797 19.595l-2.797 -5.595l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.548 9.826" /> <path d="M16 21l5 -5" /> <path d="M21 21v.01" /> <path d="M16 16v.01" /> </svg>"##;
const LOCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 13a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v6a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-6" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> </svg>"##;
const LOCK_ACCESS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> <path d="M8 12a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1l0 -3" /> <path d="M10 11v-2a2 2 0 1 1 4 0v2" /> </svg>"##;
const LOCK_ACCESS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2c0 -.554 .225 -1.055 .588 -1.417" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2c.55 0 1.05 -.222 1.41 -.582" /> <path d="M15 11a1 1 0 0 1 1 1m-.29 3.704a1 1 0 0 1 -.71 .296h-6a1 1 0 0 1 -1 -1v-3a1 1 0 0 1 1 -1h2" /> <path d="M10 11v-1m1.182 -2.826a2 2 0 0 1 2.818 1.826v1" /> <path d="M3 3l18 18" /> </svg>"##;
const LOCK_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 21v-6m2 0v-1.5m0 9v-1.5m-2 -3h3m-1 0h.5a1.5 1.5 0 0 1 0 3h-3.5m3 -3h.5a1.5 1.5 0 0 0 0 -3h-3.5" /> <path d="M13 21h-6a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> </svg>"##;
const LOCK_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 21h-6.5a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10a2 2 0 0 1 1.74 1.012" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const LOCK_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-5.5a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10a2 2 0 0 1 1.749 1.028" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const LOCK_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-4.5a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v.5" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const LOCK_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-4.5a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const LOCK_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-5a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10c.564 0 1.074 .234 1.437 .61" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const LOCK_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-6a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const LOCK_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-5.5a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10a2 2 0 0 1 1.74 1.015" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const LOCK_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21h-8a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10a2 2 0 0 1 1.734 1.002" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const LOCK_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-4.5a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10c.38 0 .734 .106 1.037 .29" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const LOCK_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-5.5a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v2" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M16 19h6" /> </svg>"##;
const LOCK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11h2a2 2 0 0 1 2 2v2m0 4a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h4" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-3m.719 -3.289a4 4 0 0 1 7.281 2.289v4" /> <path d="M3 3l18 18" /> </svg>"##;
const LOCK_OPEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 13a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v6a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -6" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M8 11v-5a4 4 0 0 1 8 0" /> </svg>"##;
const LOCK_OPEN_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 13a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v6a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -6" /> <path d="M9 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M13 11v-4a4 4 0 1 1 8 0v4" /> </svg>"##;
const LOCK_OPEN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11h2a2 2 0 0 1 2 2v2m0 4a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h4" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M8 11v-3m.347 -3.631a4 4 0 0 1 7.653 1.631" /> <path d="M3 3l18 18" /> </svg>"##;
const LOCK_PASSWORD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 13a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v6a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -6" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M15 16h.01" /> <path d="M12.01 16h.01" /> <path d="M9.02 16h.01" /> </svg>"##;
const LOCK_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-6a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v.5" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const LOCK_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-5.5a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10c.24 0 .47 .042 .683 .12" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const LOCK_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-5.5a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10a2 2 0 0 1 1.74 1.012" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const LOCK_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21h-8a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10c.265 0 .518 .052 .75 .145" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const LOCK_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-4.5a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const LOCK_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M12 21h-5a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const LOCK_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1l0 -3" /> <path d="M10 11v-2a2 2 0 1 1 4 0v2" /> <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> </svg>"##;
const LOCK_SQUARE_ROUNDED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> <path d="M8 12a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1l0 -3" /> <path d="M10 11v-2a2 2 0 1 1 4 0v2" /> </svg>"##;
const LOCK_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 21h-4a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h9" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const LOCK_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-5.5a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10a2 2 0 0 1 1.739 1.01" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const LOCK_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-6a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v.5" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const LOGIC_AND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12h-5" /> <path d="M2 9h5" /> <path d="M2 15h5" /> <path d="M9 5c6 0 8 3.5 8 7s-2 7 -8 7h-2v-14h2" /> </svg>"##;
const LOGIC_BUFFER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12h-5" /> <path d="M2 9h5" /> <path d="M2 15h5" /> <path d="M7 5l10 7l-10 7l0 -14" /> </svg>"##;
const LOGIC_NAND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12h-3" /> <path d="M2 9h3" /> <path d="M2 15h3" /> <path d="M7 5c6 0 8 3.5 8 7s-2 7 -8 7h-2v-14h2" /> <path d="M15 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const LOGIC_NOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12h-4" /> <path d="M2 9h5" /> <path d="M2 15h5" /> <path d="M6 5c10.667 2.1 10.667 12.6 0 14c1.806 -4.667 1.806 -9.333 0 -14" /> <path d="M14 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const LOGIC_NOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12h-3" /> <path d="M2 9h3" /> <path d="M2 15h3" /> <path d="M5 5l10 7l-10 7l0 -14" /> <path d="M15 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const LOGIC_OR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12h-6" /> <path d="M2 9h7" /> <path d="M2 15h7" /> <path d="M8 5c10.667 2.1 10.667 12.6 0 14c1.806 -4.667 1.806 -9.333 0 -14" /> </svg>"##;
const LOGIC_XNOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12h-2" /> <path d="M2 9h4" /> <path d="M2 15h4" /> <path d="M5 19c1.778 -4.667 1.778 -9.333 0 -14" /> <path d="M8 5c10.667 2.1 10.667 12.6 0 14c1.806 -4.667 1.806 -9.333 0 -14" /> <path d="M16 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const LOGIC_XOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12h-4" /> <path d="M2 9h6" /> <path d="M2 15h6" /> <path d="M7 19c1.778 -4.667 1.778 -9.333 0 -14" /> <path d="M10 5c10.667 2.1 10.667 12.6 0 14c1.806 -4.667 1.806 -9.333 0 -14" /> </svg>"##;
const LOGOUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8v-2a2 2 0 0 0 -2 -2h-7a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h7a2 2 0 0 0 2 -2v-2" /> <path d="M9 12h12l-3 -3" /> <path d="M18 15l3 -3" /> </svg>"##;
const LOGOUT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v-2a2 2 0 0 1 2 -2h7a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-7a2 2 0 0 1 -2 -2v-2" /> <path d="M15 12h-12l3 -3" /> <path d="M6 15l-3 -3" /> </svg>"##;
const LOGS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12h.01" /> <path d="M4 6h.01" /> <path d="M4 18h.01" /> <path d="M8 18h2" /> <path d="M8 12h2" /> <path d="M8 6h2" /> <path d="M14 6h6" /> <path d="M14 12h6" /> <path d="M14 18h6" /> </svg>"##;
const LUGGAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 8a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -10" /> <path d="M9 6v-1a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v1" /> <path d="M6 10h12" /> <path d="M6 16h12" /> <path d="M9 20v1" /> <path d="M15 20v1" /> </svg>"##;
const LUGGAGE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 6h6a2 2 0 0 1 2 2v6m0 4a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-10c0 -.546 .218 -1.04 .573 -1.4" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v1" /> <path d="M6 10h4m4 0h4" /> <path d="M6 16h10" /> <path d="M9 20v1" /> <path d="M15 20v1" /> <path d="M3 3l18 18" /> </svg>"##;
const LUNGS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.081 20c1.612 0 2.919 -1.335 2.919 -2.98v-9.763c0 -.694 -.552 -1.257 -1.232 -1.257c-.205 0 -.405 .052 -.584 .15l-.13 .083c-1.46 1.059 -2.432 2.647 -3.404 5.824c-.42 1.37 -.636 2.962 -.648 4.775c-.012 1.675 1.261 3.054 2.877 3.161l.203 .007" /> <path d="M17.92 20c-1.613 0 -2.92 -1.335 -2.92 -2.98v-9.763c0 -.694 .552 -1.257 1.233 -1.257c.204 0 .405 .052 .584 .15l.13 .083c1.46 1.059 2.432 2.647 3.405 5.824c.42 1.37 .636 2.962 .648 4.775c.012 1.675 -1.261 3.054 -2.878 3.161l-.202 .007" /> <path d="M9 12a3 3 0 0 0 3 -3a3 3 0 0 0 3 3" /> <path d="M12 4v5" /> </svg>"##;
const LUNGS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.583 6.608c-1.206 1.058 -2.07 2.626 -2.933 5.449c-.42 1.37 -.636 2.962 -.648 4.775c-.012 1.675 1.261 3.054 2.877 3.161l.203 .007c1.611 0 2.918 -1.335 2.918 -2.98v-8.02" /> <path d="M15 11v-3.743c0 -.694 .552 -1.257 1.233 -1.257c.204 0 .405 .052 .584 .15l.13 .083c1.46 1.059 2.432 2.647 3.405 5.824c.42 1.37 .636 2.962 .648 4.775c0 .063 0 .125 0 .187m-1.455 2.51c-.417 .265 -.9 .43 -1.419 .464l-.202 .007c-1.613 0 -2.92 -1.335 -2.92 -2.98v-2.02" /> <path d="M9 12a2.99 2.99 0 0 0 2.132 -.89" /> <path d="M12 4v4" /> <path d="M3 3l18 18" /> </svg>"##;
const MAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v5" /> <path d="M14 16v5" /> <path d="M9 9h6l-1 7h-4l-1 -7" /> <path d="M5 11c1.333 -1.333 2.667 -2 4 -2" /> <path d="M19 11c-1.333 -1.333 -2.667 -2 -4 -2" /> <path d="M10 4a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const MARQUEE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2m3 0h1.5m3 0h1.5m3 0a2 2 0 0 1 2 2m0 3v1.5m0 3v1.5m0 3a2 2 0 0 1 -2 2m-3 0h-1.5m-3 0h-1.5m-3 0a2 2 0 0 1 -2 -2m0 -3v-1.5m0 -3v-1.5m0 -3" /> </svg>"##;
const MARQUEE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6v-1a1 1 0 0 1 1 -1h1m5 0h2m5 0h1a1 1 0 0 1 1 1v1m0 5v2m0 5v1a1 1 0 0 1 -1 1h-1m-5 0h-2m-5 0h-1a1 1 0 0 1 -1 -1v-1m0 -5v-2" /> </svg>"##;
const MARQUEE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c0 -.556 .227 -1.059 .593 -1.421" /> <path d="M9 4h1.5" /> <path d="M13.5 4h1.5" /> <path d="M18 4a2 2 0 0 1 2 2" /> <path d="M20 9v1.5" /> <path d="M20 13.5v1.5" /> <path d="M19.402 19.426a1.993 1.993 0 0 1 -1.402 .574" /> <path d="M15 20h-1.5" /> <path d="M10.5 20h-1.5" /> <path d="M6 20a2 2 0 0 1 -2 -2" /> <path d="M4 15v-1.5" /> <path d="M4 10.5v-1.5" /> <path d="M3 3l18 18" /> </svg>"##;
const MARS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 14a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M19 5l-5.4 5.4" /> <path d="M19 5l-5 0" /> <path d="M19 5l0 5" /> </svg>"##;
const MASSAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M8 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M4 22l4 -2v-3h12" /> <path d="M11 20h9" /> <path d="M8 14l3 -2l1 -4c3 1 3 4 3 6" /> </svg>"##;
const MATCHSTICK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21l14 -9" /> <path d="M16 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M17 3l3.62 7.29a4.007 4.007 0 0 1 -.764 4.51a4 4 0 0 1 -6.493 -4.464l3.637 -7.336" /> </svg>"##;
const MATH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5h-7l-4 14l-3 -6h-2" /> <path d="M14 13l6 6" /> <path d="M14 19l6 -6" /> </svg>"##;
const MATH_1_DIVIDE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14" /> <path d="M10 15h3a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h3" /> <path d="M10 5l2 -2v6" /> </svg>"##;
const MATH_1_DIVIDE_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15.5a.5 .5 0 0 1 .5 -.5h2a1.5 1.5 0 0 1 0 3h-1.167h1.167a1.5 1.5 0 0 1 0 3h-2a.5 .5 0 0 1 -.5 -.5" /> <path d="M5 12h14" /> <path d="M10 5l2 -2v6" /> </svg>"##;
const MATH_AVG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21l18 -18" /> <path d="M4 12a8 8 0 1 0 16 0a8 8 0 1 0 -16 0" /> </svg>"##;
const MATH_COS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 10a2 2 0 1 0 -4 0v4a2 2 0 1 0 4 0" /> <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M17 15c.345 .6 1.258 1 2 1a2 2 0 1 0 0 -4a2 2 0 1 1 0 -4c.746 0 1.656 .394 2 1" /> </svg>"##;
const MATH_CTG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8h4" /> <path d="M21 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> <path d="M12 8v8" /> <path d="M7 10a2 2 0 1 0 -4 0v4a2 2 0 1 0 4 0" /> </svg>"##;
const MATH_EQUAL_GREATER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 18l14 -4" /> <path d="M5 14l14 -4l-14 -4" /> </svg>"##;
const MATH_EQUAL_LOWER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 18l-14 -4" /> <path d="M19 14l-14 -4l14 -4" /> </svg>"##;
const MATH_FUNCTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19a2 2 0 0 0 2 2c2 0 2 -4 3 -9s1 -9 3 -9a2 2 0 0 1 2 2" /> <path d="M5 12h6" /> <path d="M15 12l6 6" /> <path d="M15 18l6 -6" /> </svg>"##;
const MATH_FUNCTION_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10h1c.882 0 .986 .777 1.694 2.692" /> <path d="M13 17c.864 0 1.727 -.663 2.495 -1.512m1.717 -2.302c.993 -1.45 2.39 -3.186 3.788 -3.186" /> <path d="M3 19c0 1.5 .5 2 2 2s2 -4 3 -9c.237 -1.186 .446 -2.317 .647 -3.35m.727 -3.248c.423 -1.492 .91 -2.402 1.626 -2.402c1.5 0 2 .5 2 2" /> <path d="M5 12h6" /> <path d="M3 3l18 18" /> </svg>"##;
const MATH_FUNCTION_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19a2 2 0 0 0 2 2c2 0 2 -4 3 -9s1 -9 3 -9a2 2 0 0 1 2 2" /> <path d="M5 12h6" /> <path d="M15 12l3 5.063" /> <path d="M21 12l-4.8 9" /> </svg>"##;
const MATH_GREATER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 18l14 -6l-14 -6" /> </svg>"##;
const MATH_INTEGRAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 19a2 2 0 0 0 2 2c2 0 2 -4 3 -9s1 -9 3 -9a2 2 0 0 1 2 2" /> </svg>"##;
const MATH_INTEGRAL_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19a2 2 0 0 0 2 2c2 0 2 -4 3 -9s1 -9 3 -9a2 2 0 0 1 2 2" /> <path d="M14 12l6 6" /> <path d="M14 18l6 -6" /> </svg>"##;
const MATH_INTEGRALS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19a2 2 0 0 0 2 2c2 0 2 -4 3 -9s1 -9 3 -9a2 2 0 0 1 2 2" /> <path d="M11 19a2 2 0 0 0 2 2c2 0 2 -4 3 -9s1 -9 3 -9a2 2 0 0 1 2 2" /> </svg>"##;
const MATH_LOWER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 18l-14 -6l14 -6" /> </svg>"##;
const MATH_MAX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 6a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M3 15s.616 -5.544 2.332 -7.93" /> <path d="M8.637 7.112c2.717 3.313 5.882 12.888 8.363 12.888c2 0 3.333 -3 4 -9" /> </svg>"##;
const MATH_MAX_MIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M5 5a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M3 14s.605 -5.44 2.284 -7.862m3.395 .026c2.137 2.652 4.547 9.113 6.68 11.719" /> <path d="M18.748 18.038c.702 -.88 1.452 -3.56 2.252 -8.038" /> </svg>"##;
const MATH_MIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 18a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M3 13s1 -9 4 -9c2.48 0 5.643 9.565 8.36 12.883" /> <path d="M18.748 17.038c.702 -.88 1.452 -3.56 2.252 -8.038" /> </svg>"##;
const MATH_NOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14v4" /> </svg>"##;
const MATH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 19l2.5 -2.5" /> <path d="M18.5 14.5l1.5 -1.5" /> <path d="M3 3l18 18" /> <path d="M19 5h-7l-.646 2.262" /> <path d="M10.448 10.431l-2.448 8.569l-3 -6h-2" /> </svg>"##;
const MATH_PI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 20v-16" /> <path d="M17 4v16" /> <path d="M20 4h-16" /> </svg>"##;
const MATH_PI_DIVIDE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15h3a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h3" /> <path d="M5 12h14" /> <path d="M10 9v-6" /> <path d="M14 3v6" /> <path d="M15 3h-6" /> </svg>"##;
const MATH_SEC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 15c.345 .6 1.258 1 2 1a2 2 0 1 0 0 -4a2 2 0 1 1 0 -4c.746 0 1.656 .394 2 1" /> <path d="M21 10a2 2 0 1 0 -4 0v4a2 2 0 1 0 4 0" /> <path d="M14 8h-4v8h4" /> <path d="M10 12h2.5" /> </svg>"##;
const MATH_SIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 15c.345 .6 1.258 1 2 1a2 2 0 1 0 0 -4a2 2 0 1 1 0 -4c.746 0 1.656 .394 2 1" /> <path d="M12 8v8" /> <path d="M16 16v-8l4 8v-8" /> </svg>"##;
const MATH_SYMBOLS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12l18 0" /> <path d="M12 3l0 18" /> <path d="M16.5 4.5l3 3" /> <path d="M19.5 4.5l-3 3" /> <path d="M6 4l0 4" /> <path d="M4 6l4 0" /> <path d="M18 16l.01 0" /> <path d="M18 20l.01 0" /> <path d="M4 18l4 0" /> </svg>"##;
const MATH_TG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8h4" /> <path d="M9 8v8" /> <path d="M18 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> </svg>"##;
const MATH_X_DIVIDE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15h3a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h3" /> <path d="M5 12h14" /> <path d="M9 3l6 6" /> <path d="M9 9l6 -6" /> </svg>"##;
const MATH_X_DIVIDE_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 3l6 6" /> <path d="M9 9l6 -6" /> <path d="M9 15l3 4.5" /> <path d="M15 15l-4.5 7" /> <path d="M5 12h14" /> </svg>"##;
const MATH_X_DIVIDE_Y_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21l18 -18" /> <path d="M15 14l3 4.5" /> <path d="M21 14l-4.5 7" /> <path d="M3 4l6 6" /> <path d="M3 10l6 -6" /> </svg>"##;
const MATH_X_FLOOR_DIVIDE_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M1.5 19l18 -18" /> <path d="M4.5 22l18 -18" /> <path d="M18 15l3 4" /> <path d="M23 15l-4.5 8" /> <path d="M1 1l6 6" /> <path d="M1 7l6 -6" /> </svg>"##;
const MATH_X_MINUS_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 9l6 6" /> <path d="M2 15l6 -6" /> <path d="M16 9l6 6" /> <path d="M16 15l6 -6" /> <path d="M10 12h4" /> </svg>"##;
const MATH_X_MINUS_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 9l6 6" /> <path d="M2 15l6 -6" /> <path d="M16 9l3 5.063" /> <path d="M22 9l-4.8 9" /> <path d="M10 12h4" /> </svg>"##;
const MATH_X_PLUS_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 9l6 6" /> <path d="M2 15l6 -6" /> <path d="M16 9l6 6" /> <path d="M16 15l6 -6" /> <path d="M10 12h4" /> <path d="M12 10v4" /> </svg>"##;
const MATH_X_PLUS_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 9l3 5.063" /> <path d="M2 9l6 6" /> <path d="M2 15l6 -6" /> <path d="M22 9l-4.8 9" /> <path d="M10 12h4" /> <path d="M12 10v4" /> </svg>"##;
const MATH_XY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 9l3 5.063" /> <path d="M4 9l6 6" /> <path d="M4 15l6 -6" /> <path d="M20 9l-4.8 9" /> </svg>"##;
const MATH_Y_MINUS_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 9l3 5.063" /> <path d="M8 9l-4.8 9" /> <path d="M16 9l3 5.063" /> <path d="M22 9l-4.8 9" /> <path d="M10 12h4" /> </svg>"##;
const MATH_Y_PLUS_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 9l3 5.063" /> <path d="M8 9l-4.8 9" /> <path d="M16 9l3 5.063" /> <path d="M22 9l-4.8 9" /> <path d="M10 12h4" /> <path d="M12 10v4" /> </svg>"##;
const MATRIX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16h.013" /> <path d="M12.01 16h.005" /> <path d="M16.015 16h.005" /> <path d="M16.015 12h.005" /> <path d="M8.01 12h.005" /> <path d="M12.01 12h.005" /> <path d="M16.02 8h.005" /> <path d="M8.015 8h.005" /> <path d="M12.015 8h.005" /> <path d="M7 4h-1a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h1" /> <path d="M17 4h1a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-1" /> </svg>"##;
const MEDICINE_SYRUP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 21h8a1 1 0 0 0 1 -1v-10a3 3 0 0 0 -3 -3h-4a3 3 0 0 0 -3 3v10a1 1 0 0 0 1 1" /> <path d="M10 14h4" /> <path d="M12 12v4" /> <path d="M10 7v-3a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v3" /> </svg>"##;
const MENORAH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4v16" /> <path d="M8 4v2a4 4 0 1 0 8 0v-2" /> <path d="M4 4v2a8 8 0 1 0 16 0v-2" /> <path d="M10 20h4" /> </svg>"##;
const MENU_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8l16 0" /> <path d="M4 16l16 0" /> </svg>"##;
const MENU_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6l16 0" /> <path d="M4 12l16 0" /> <path d="M4 18l16 0" /> </svg>"##;
const MENU_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 6h10" /> <path d="M4 12h16" /> <path d="M7 12h13" /> <path d="M4 18h10" /> </svg>"##;
const MENU_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 6h10" /> <path d="M4 12h16" /> <path d="M7 12h13" /> <path d="M7 18h10" /> </svg>"##;
const MENU_DEEP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h16" /> <path d="M7 12h13" /> <path d="M10 18h10" /> </svg>"##;
const MENU_ORDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10h16" /> <path d="M4 14h16" /> <path d="M9 18l3 3l3 -3" /> <path d="M9 6l3 -3l3 3" /> </svg>"##;
const METER_CUBE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 5h1.5a1.5 1.5 0 0 1 0 3h-.5h.5a1.5 1.5 0 0 1 0 3h-1.5" /> <path d="M4 12v6" /> <path d="M4 14a2 2 0 0 1 2 -2h.5a2.5 2.5 0 0 1 2.5 2.5v3.5" /> <path d="M9 15.5v-1a2.5 2.5 0 1 1 5 0v3.5" /> </svg>"##;
const METER_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 5h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2" /> <path d="M4 12v6" /> <path d="M4 14a2 2 0 0 1 2 -2h.5a2.5 2.5 0 0 1 2.5 2.5v3.5" /> <path d="M9 15.5v-1a2.5 2.5 0 1 1 5 0v3.5" /> </svg>"##;
const METRONOME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.153 8.188l-.72 -3.236a2.493 2.493 0 0 0 -4.867 0l-3.025 13.614a2 2 0 0 0 1.952 2.434h7.014a2 2 0 0 0 1.952 -2.434l-.524 -2.357m-4.935 1.791l9 -13" /> <path d="M19 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const MICROFRONTENDS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.5 7.5l4.5 4.5l4.5 -4.5" /> <path d="M6 16v-4" /> <path d="M18 16v-4" /> <path d="M16 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const MICROSCOPE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21h14" /> <path d="M6 18h2" /> <path d="M7 18v3" /> <path d="M9 11l3 3l6 -6l-3 -3l-6 6" /> <path d="M10.5 12.5l-1.5 1.5" /> <path d="M17 3l3 3" /> <path d="M12 21a6 6 0 0 0 3.715 -10.712" /> </svg>"##;
const MICROSCOPE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21h14" /> <path d="M6 18h2" /> <path d="M7 18v3" /> <path d="M10 10l-1 1l3 3l1 -1m2 -2l3 -3l-3 -3l-3 3" /> <path d="M10.5 12.5l-1.5 1.5" /> <path d="M17 3l3 3" /> <path d="M12 21a6 6 0 0 0 5.457 -3.505m.441 -3.599a6 6 0 0 0 -2.183 -3.608" /> <path d="M3 3l18 18" /> </svg>"##;
const MIDDLEWARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20l2.25 -2.25" /> <path d="M20 20l-2.25 -2.25" /> <path d="M20 4l-2.25 2.25" /> <path d="M4 4l2.25 2.25" /> <path d="M10 19.748a8.01 8.01 0 0 1 -5.747 -5.748" /> <path d="M19.748 14a8.01 8.01 0 0 1 -5.748 5.748" /> <path d="M4.252 10a8.02 8.02 0 0 1 5.478 -5.672l.27 -.075" /> <path d="M14 4.252a8.01 8.01 0 0 1 5.748 5.749" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> </svg>"##;
const MILITARY_AWARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M8.5 10.5l-1 -2.5h-5.5l2.48 5.788a2 2 0 0 0 1.84 1.212h2.18" /> <path d="M15.5 10.5l1 -2.5h5.5l-2.48 5.788a2 2 0 0 1 -1.84 1.212h-2.18" /> </svg>"##;
const MILITARY_RANK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 7v12a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-12l6 -4l6 4" /> <path d="M10 13l2 -1l2 1" /> <path d="M10 17l2 -1l2 1" /> <path d="M10 9l2 -1l2 1" /> </svg>"##;
const MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12l14 0" /> </svg>"##;
const MINUS_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5v14" /> </svg>"##;
const MOBILEDATA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12v-8" /> <path d="M8 20v-8" /> <path d="M13 7l3 -3l3 3" /> <path d="M5 17l3 3l3 -3" /> </svg>"##;
const MOBILEDATA_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12v-8" /> <path d="M8 20v-8" /> <path d="M13 7l3 -3l3 3" /> <path d="M5 17l3 3l3 -3" /> <path d="M3 3l18 18" /> </svg>"##;
const MOSQUE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 5.49a1.764 1.764 0 0 1 -2.5 -2.49" /> <path d="M12 6v3" /> <path d="M19 21a8.9 8.9 0 0 0 1 -3.67c0 -2 -.92 -3.25 -3.24 -4.51a17.4 17.4 0 0 1 -4.76 -3.82a17.4 17.4 0 0 1 -4.76 3.82c-2.32 1.26 -3.24 2.55 -3.24 4.51a8.9 8.9 0 0 0 1 3.67h14" /> </svg>"##;
const MULTIPLIER_0_5X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16h2a2 2 0 1 0 0 -4h-2v-4h4" /> <path d="M5 16v.01" /> <path d="M15 16l4 -4" /> <path d="M19 16l-4 -4" /> </svg>"##;
const MULTIPLIER_1_5X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 16v-8l-2 2" /> <path d="M10 16h2a2 2 0 1 0 0 -4h-2v-4h4" /> <path d="M7 16v.01" /> <path d="M17 16l4 -4" /> <path d="M21 16l-4 -4" /> </svg>"##;
const MULTIPLIER_1X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 16v-8l-2 2" /> <path d="M13 16l4 -4" /> <path d="M17 16l-4 -4" /> </svg>"##;
const MULTIPLIER_2X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 16l4 -4" /> <path d="M18 16l-4 -4" /> <path d="M6 10a2 2 0 1 1 4 0c0 .591 -.417 1.318 -.816 1.858l-3.184 4.143l4 0" /> </svg>"##;
const MUSIC_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v9" /> <path d="M9 8h10" /> <path d="M16 21l5 -5" /> <path d="M21 21v.01" /> <path d="M16 16v.01" /> </svg>"##;
const NEW_SECTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12l6 0" /> <path d="M12 9l0 6" /> <path d="M4 6v-1a1 1 0 0 1 1 -1h1m5 0h2m5 0h1a1 1 0 0 1 1 1v1m0 5v2m0 5v1a1 1 0 0 1 -1 1h-1m-5 0h-2m-5 0h-1a1 1 0 0 1 -1 -1v-1m0 -5v-2m0 -5" /> </svg>"##;
const NO_COPYRIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14 9.75a3.016 3.016 0 0 0 -4.163 .173a2.993 2.993 0 0 0 0 4.154a3.016 3.016 0 0 0 4.163 .173" /> <path d="M6 6l1.5 1.5" /> <path d="M16.5 16.5l1.5 1.5" /> </svg>"##;
const NO_CREATIVE_COMMONS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10.5 10.5c-.847 -.71 -2.132 -.658 -2.914 .116a1.928 1.928 0 0 0 0 2.768c.782 .774 2.067 .825 2.914 .116" /> <path d="M16.5 10.5c-.847 -.71 -2.132 -.658 -2.914 .116a1.928 1.928 0 0 0 0 2.768c.782 .774 2.067 .825 2.914 .116" /> <path d="M6 6l1.5 1.5" /> <path d="M16.5 16.5l1.5 1.5" /> </svg>"##;
const NO_DERIVATIVES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 10h6" /> <path d="M9 14h6" /> </svg>"##;
const NOTIFICATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 6h-3a2 2 0 0 0 -2 2v9a2 2 0 0 0 2 2h9a2 2 0 0 0 2 -2v-3" /> <path d="M14 7a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const NOTIFICATION_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.154 6.187a2 2 0 0 0 -1.154 1.813v9a2 2 0 0 0 2 2h9a2 2 0 0 0 1.811 -1.151" /> <path d="M14 7a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M3 3l18 18" /> </svg>"##;
const NURSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5c2.941 0 6.685 1.537 9 3l-2 11h-14l-2 -11c2.394 -1.513 6.168 -3.005 9 -3" /> <path d="M10 12h4" /> <path d="M12 10v4" /> </svg>"##;
const OBJECT_SCAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 10a2 2 0 0 1 2 -2h4a2 2 0 0 1 2 2v4a2 2 0 0 1 -2 2h-4a2 2 0 0 1 -2 -2v-4" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const OLD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 21l-1 -4l-2 -3v-6" /> <path d="M5 14l-1 -3l4 -3l3 2l3 .5" /> <path d="M7 4a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M7 17l-2 4" /> <path d="M16 21v-8.5a1.5 1.5 0 0 1 3 0v.5" /> </svg>"##;
const OM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 12c2.21 0 4 -1.567 4 -3.5s-1.79 -3.5 -4 -3.5c-1.594 0 -2.97 .816 -3.613 2" /> <path d="M3.423 14.483a4.944 4.944 0 0 0 -.423 2.017c0 2.485 1.79 4.5 4 4.5s4 -2.015 4 -4.5s-1.79 -4.5 -4 -4.5" /> <path d="M14.071 17.01c.327 2.277 1.739 3.99 3.429 3.99c1.933 0 3.5 -2.239 3.5 -5s-1.567 -5 -3.5 -5c-.96 0 -1.868 .606 -2.5 1.5c-.717 1.049 -1.76 1.7 -2.936 1.7c-.92 0 -1.766 -.406 -2.434 -1.087" /> <path d="M17 3l2 2" /> <path d="M12 3c1.667 3.667 4.667 5.333 9 5" /> </svg>"##;
const OMEGA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19h5v-1a7.35 7.35 0 1 1 6 0v1h5" /> </svg>"##;
const OPTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 6h5m0 12h-5l-5 -12h-4" /> </svg>"##;
const OUTBOUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 15l6 -6" /> <path d="M11 9h4v4" /> </svg>"##;
const OUTLET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M8.5 12a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M14.5 12a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> </svg>"##;
const PACKAGE_EXPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21l-8 -4.5v-9l8 -4.5l8 4.5v4.5" /> <path d="M12 12l8 -4.5" /> <path d="M12 12v9" /> <path d="M12 12l-8 -4.5" /> <path d="M15 18h7" /> <path d="M19 15l3 3l-3 3" /> </svg>"##;
const PACKAGE_IMPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21l-8 -4.5v-9l8 -4.5l8 4.5v4.5" /> <path d="M12 12l8 -4.5" /> <path d="M12 12v9" /> <path d="M12 12l-8 -4.5" /> <path d="M22 18h-7" /> <path d="M18 15l-3 3l3 3" /> </svg>"##;
const PACKAGES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 16.5l-5 -3l5 -3l5 3v5.5l-5 3l0 -5.5" /> <path d="M2 13.5v5.5l5 3" /> <path d="M7 16.545l5 -3.03" /> <path d="M17 16.5l-5 -3l5 -3l5 3v5.5l-5 3l0 -5.5" /> <path d="M12 19l5 3" /> <path d="M17 16.5l5 -3" /> <path d="M12 13.5v-5.5l-5 -3l5 -3l5 3v5.5" /> <path d="M7 5.03v5.455" /> <path d="M12 8l5 -3" /> </svg>"##;
const PARENTHESES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 4a12.25 12.25 0 0 0 0 16" /> <path d="M17 4a12.25 12.25 0 0 1 0 16" /> </svg>"##;
const PARENTHESES_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.743 5.745a12.253 12.253 0 0 0 1.257 14.255" /> <path d="M17 4a12.25 12.25 0 0 1 2.474 11.467m-1.22 2.794a12.291 12.291 0 0 1 -1.254 1.739" /> <path d="M3 3l18 18" /> </svg>"##;
const PARKING_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-8h3.334c.92 0 1.666 .895 1.666 2s-.746 2 -1.666 2h-3.334" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PASSWORD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 10v4" /> <path d="M10 13l4 -2" /> <path d="M10 11l4 2" /> <path d="M5 10v4" /> <path d="M3 13l4 -2" /> <path d="M3 11l4 2" /> <path d="M19 10v4" /> <path d="M17 13l4 -2" /> <path d="M17 11l4 2" /> </svg>"##;
const PASSWORD_FINGERPRINT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 8c.788 1 1 2 1 3v1" /> <path d="M9 11c0 -1.578 1.343 -3 3 -3s3 1.422 3 3v2" /> <path d="M12 11v2" /> <path d="M6 12v-1.397c-.006 -1.999 1.136 -3.849 2.993 -4.85a6.385 6.385 0 0 1 6.007 -.005" /> <path d="M12 17v4" /> <path d="M10 20l4 -2" /> <path d="M10 18l4 2" /> <path d="M5 17v4" /> <path d="M3 20l4 -2" /> <path d="M3 18l4 2" /> <path d="M19 17v4" /> <path d="M17 20l4 -2" /> <path d="M17 18l4 2" /> </svg>"##;
const PASSWORD_MOBILE_PHONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17v4" /> <path d="M10 20l4 -2" /> <path d="M10 18l4 2" /> <path d="M5 17v4" /> <path d="M3 20l4 -2" /> <path d="M3 18l4 2" /> <path d="M19 17v4" /> <path d="M17 20l4 -2" /> <path d="M17 18l4 2" /> <path d="M7 14v-8a2 2 0 0 1 2 -2h6a2 2 0 0 1 2 2v8" /> <path d="M11 5h2" /> <path d="M12 17v.01" /> </svg>"##;
const PASSWORD_USER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17v4" /> <path d="M10 20l4 -2" /> <path d="M10 18l4 2" /> <path d="M5 17v4" /> <path d="M3 20l4 -2" /> <path d="M3 18l4 2" /> <path d="M19 17v4" /> <path d="M17 20l4 -2" /> <path d="M17 18l4 2" /> <path d="M9 6a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M7 14a2 2 0 0 1 2 -2h6a2 2 0 0 1 2 2" /> </svg>"##;
const PAYWALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-6a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h10" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M8 11v-4a4 4 0 1 1 8 0v4" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1" /> <path d="M19 14v1" /> </svg>"##;
const PEACE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 3l0 18" /> <path d="M12 12l6.3 6.3" /> <path d="M12 12l-6.3 6.3" /> </svg>"##;
const PENDULUM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 19a2 2 0 1 1 4 0a2 2 0 0 1 -4 0" /> <path d="M12 3l-4.4 14.09" /> <path d="M19 3h-14" /> </svg>"##;
const PERCENTAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 17a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M6 7a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M6 18l12 -12" /> </svg>"##;
const PERCENTAGE_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PERCENTAGE_10_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3c1.92 0 3.7 .601 5.16 1.626l-5.16 7.374v-9" fill="currentColor" stroke="none" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PERCENTAGE_100_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" fill="currentColor" /> </svg>"##;
const PERCENTAGE_20_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a9 9 0 0 1 8.497 6.025l-8.497 2.975v-9" fill="currentColor" stroke="none" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PERCENTAGE_25_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 0 0 -9 -9m0 0v9h9" fill="currentColor" stroke="none" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PERCENTAGE_30_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a9 9 0 0 1 8.495 11.973l-8.495 -2.973v-9" fill="currentColor" stroke="none" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PERCENTAGE_33_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a9 9 0 0 1 7.794 13.5l-7.79 -4.497v-9" fill="currentColor" stroke="none" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PERCENTAGE_40_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a9 9 0 0 1 5.162 16.372l-5.162 -7.372v-9" fill="currentColor" stroke="none" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PERCENTAGE_50_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 0 0 0 -18m0 0v18" fill="currentColor" stroke="none" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PERCENTAGE_60_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a9 9 0 1 1 -5.162 16.373l5.162 -7.373v-9" fill="currentColor" stroke="none" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PERCENTAGE_66_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a9 9 0 1 1 -7.795 13.498l7.795 -4.498v-9" fill="currentColor" stroke="none" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PERCENTAGE_70_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a9 9 0 1 1 -8.495 11.973l8.495 -2.973v-9" fill="currentColor" stroke="none" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PERCENTAGE_75_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 9 -9m0 0v9h-9" fill="currentColor" stroke="none" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PERCENTAGE_80_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a9 9 0 1 1 -8.497 6.025l8.497 2.975v-9" fill="currentColor" stroke="none" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PERCENTAGE_90_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a9 9 0 1 1 -5.16 1.626l5.16 7.374v-9" fill="currentColor" stroke="none" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const PHOTO_SCAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M6 13l2.644 -2.644a1.21 1.21 0 0 1 1.712 0l3.644 3.644" /> <path d="M13 13l1.644 -1.644a1.21 1.21 0 0 1 1.712 0l1.644 1.644" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const PHYSOTHERAPIST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l-1 -3l4 -2l4 1h3.5" /> <path d="M3 19a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 6a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M12 17v-7" /> <path d="M8 20h7l1 -4l4 -2" /> <path d="M18 20h3" /> </svg>"##;
const PILL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.5 12.5l8 -8a4.94 4.94 0 0 1 7 7l-8 8a4.94 4.94 0 0 1 -7 -7" /> <path d="M8.5 8.5l7 7" /> </svg>"##;
const PILL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.495 6.505l2 -2a4.95 4.95 0 0 1 7 7l-2 2m-2 2l-4 4a4.95 4.95 0 0 1 -7 -7l4 -4" /> <path d="M8.5 8.5l7 7" /> <path d="M3 3l18 18" /> </svg>"##;
const PILLOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 9a9.34 9.34 0 0 1 0 6" /> <path d="M21.699 16.607c.481 .934 .28 2.088 -.486 2.79c-.767 .703 -1.9 .77 -2.74 .165a48 48 0 0 1 -12.946 0a2.16 2.16 0 0 1 -2.74 -.165a2.345 2.345 0 0 1 -.486 -2.79a41.7 41.7 0 0 1 0 -9.163a2.346 2.346 0 0 1 .433 -2.856a2.16 2.16 0 0 1 2.793 -.145a48 48 0 0 1 12.946 0a2.16 2.16 0 0 1 2.793 .145c.78 .726 .961 1.918 .433 2.856a41.7 41.7 0 0 1 0 9.163" /> </svg>"##;
const PILLS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M13 17a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M4.5 4.5l7 7" /> <path d="M19.5 14.5l-5 5" /> </svg>"##;
const PIN_END_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 11v-5a1 1 0 0 0 -1 -1h-16a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h9" /> <path d="M17 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 13v-4h4" /> <path d="M14 13l-4 -4" /> </svg>"##;
const PIN_INVOKE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 13v5a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h9" /> <path d="M17 7a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 11h4v4" /> <path d="M10 15l4 -4" /> </svg>"##;
const PIPELINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4h8" /> <path d="M4 4v5a6 6 0 0 0 6 6h3a1 1 0 0 1 1 1v4" /> <path d="M10 4v4a1 1 0 0 0 1 1h3a6 6 0 0 1 6 6v5" /> <path d="M13 20h8" /> <path d="M12 9v6" /> </svg>"##;
const PLAYLIST_ADD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 8h-14" /> <path d="M5 12h9" /> <path d="M11 16h-6" /> <path d="M15 16h6" /> <path d="M18 13v6" /> </svg>"##;
const PLUNGER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.71 14.12l7.81 -7.82a2 2 0 0 0 -2.82 -2.82l-7.82 7.81" /> <path d="M3.71 13.22l.7 -.71a5 5 0 0 1 7.08 0a5 5 0 0 1 0 7.08l-.71 .7" /> <path d="M3 12.5l8.5 8.5" /> </svg>"##;
const PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5l0 14" /> <path d="M5 12l14 0" /> </svg>"##;
const PLUS_EQUAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7h6" /> <path d="M7 4v6" /> <path d="M20 16h-6" /> <path d="M20 19h-6" /> <path d="M5 19l14 -14" /> </svg>"##;
const PLUS_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7h6" /> <path d="M7 4v6" /> <path d="M20 18h-6" /> <path d="M5 19l14 -14" /> </svg>"##;
const PODIUM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 8h14l-.621 2.485a2 2 0 0 1 -1.94 1.515h-8.878a2 2 0 0 1 -1.94 -1.515l-.621 -2.485" /> <path d="M7 8v-2a3 3 0 0 1 3 -3" /> <path d="M8 12l1 9" /> <path d="M16 12l-1 9" /> <path d="M7 21h10" /> </svg>"##;
const PODIUM_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8h7l-.621 2.485a2 2 0 0 1 -1.94 1.515h-.439m-4 0h-4.439a2 2 0 0 1 -1.94 -1.515l-.621 -2.485h3" /> <path d="M7 8v-1m.864 -3.106a2.99 2.99 0 0 1 2.136 -.894" /> <path d="M8 12l1 9" /> <path d="M15.599 15.613l-.599 5.387" /> <path d="M7 21h10" /> <path d="M3 3l18 18" /> </svg>"##;
const POINT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> </svg>"##;
const POINT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.15 9.194a4 4 0 0 0 5.697 5.617m1.153 -2.811a4 4 0 0 0 -4 -4" /> <path d="M3 3l18 18" /> </svg>"##;
const POINTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.904 17.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l4.907 4.907a1.067 1.067 0 0 0 1.509 0l1.047 -1.047a1.067 1.067 0 0 0 0 -1.509l-4.907 -4.907l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563" /> </svg>"##;
const POINTER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.185 13.14l5.644 -2.202c1.625 -.634 1.538 -2.962 -.13 -3.473l-14.319 -4.382c-1.41 -.431 -2.73 .888 -2.298 2.298l4.382 14.318c.51 1.668 2.84 1.755 3.473 .13l2.202 -5.644a1.84 1.84 0 0 1 1.045 -1.045" /> </svg>"##;
const POINTER_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.044 13.488l-1.266 -1.266l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l1.678 1.678" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const POINTER_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.526 12.97l-.748 -.748l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l.714 .714" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const POINTER_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.487 14.93l-2.709 -2.708l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l.785 .785" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const POINTER_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.76 13.203l-.982 -.981l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l.67 .67" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const POINTER_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.774 13.218l-.996 -.996l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l.343 .343" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const POINTER_COLLABORATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.943 13.013l5.016 -1.957c1.445 -.563 1.367 -2.633 -.116 -3.087l-12.727 -3.895c-1.253 -.384 -2.426 .79 -2.042 2.042l3.895 12.727c.454 1.483 2.524 1.56 3.087 .116l1.957 -5.017c.166 -.426 .503 -.763 .93 -.929" /> <path d="M20 15l-3.151 1.064a1.25 1.25 0 0 0 -.785 .785l-1.064 3.151" /> </svg>"##;
const POINTER_COLLABORATION_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.987 13.943l1.957 5.016c.563 1.445 2.633 1.367 3.087 -.116l3.895 -12.727c.384 -1.253 -.79 -2.426 -2.042 -2.042l-12.727 3.895c-1.483 .454 -1.56 2.524 -.116 3.087l5.017 1.957c.426 .166 .763 .503 .929 .93" /> <path d="M9 20l-1.064 -3.151a1.25 1.25 0 0 0 -.785 -.785l-3.151 -1.064" /> </svg>"##;
const POINTER_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.778 12.222l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l.787 .787" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const POINTER_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.992 13.436l-1.214 -1.214l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l1.171 1.171" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const POINTER_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.97 13.414l-1.192 -1.192l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l2.778 2.778" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const POINTER_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.571 11.018l1.32 -.886a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const POINTER_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.6 15.043l-2.822 -2.821l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l1.188 1.188" /> <path d="M16 19h6" /> </svg>"##;
const POINTER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.662 11.628l2.229 -1.496a1.2 1.2 0 0 0 -.309 -2.228l-8.013 -2.303m-5.569 -1.601l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l4.907 4.907a1.067 1.067 0 0 0 1.509 0l.524 -.524" /> <path d="M3 3l18 18" /> </svg>"##;
const POINTER_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.72 13.163l-.942 -.941l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l.969 .969" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const POINTER_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.778 12.222l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l.381 .381" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const POINTER_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.941 13.385l-1.163 -1.163l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l1.23 1.23" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const POINTER_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.062 12.506l-.284 -.284l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l1.278 1.278" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const POINTER_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.778 12.222l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const POINTER_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.646 13.09l-.868 -.868l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l.607 .607" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const POINTER_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.891 10.132a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const POINTER_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.984 13.428l-1.206 -1.206l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l1.217 1.217" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const POINTER_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.768 13.212l-.99 -.99l3.113 -2.09a1.2 1.2 0 0 0 -.309 -2.228l-13.582 -3.904l3.904 13.563a1.2 1.2 0 0 0 2.228 .308l2.09 -3.093l.908 .908" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const POO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h.01" /> <path d="M14 12h.01" /> <path d="M10 16a3.5 3.5 0 0 0 4 0" /> <path d="M11 4c2 0 3.5 1.5 3.5 4l.164 0a2.5 2.5 0 0 1 2.196 3.32a3 3 0 0 1 1.615 3.063a3 3 0 0 1 -1.299 5.607l-.176 0h-10a3 3 0 0 1 -1.474 -5.613a3 3 0 0 1 1.615 -3.062a2.5 2.5 0 0 1 2.195 -3.32l.164 0c1.5 0 2.5 -2 1.5 -4l0 .005" /> </svg>"##;
const PRAY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M7 20h8l-4 -4v-7l4 3l2 -2" /> </svg>"##;
const PREMIUM_RIGHTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M13.867 9.75c-.246 -.48 -.708 -.769 -1.2 -.75h-1.334c-.736 0 -1.333 .67 -1.333 1.5c0 .827 .597 1.499 1.333 1.499h1.334c.736 0 1.333 .671 1.333 1.5c0 .828 -.597 1.499 -1.333 1.499h-1.334c-.492 .019 -.954 -.27 -1.2 -.75" /> <path d="M12 7v2" /> <path d="M12 15v2" /> </svg>"##;
const PRESCRIPTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 19v-16h4.5a4.5 4.5 0 1 1 0 9h-4.5" /> <path d="M19 21l-9 -9" /> <path d="M13 21l6 -6" /> </svg>"##;
const PRISM_LIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.731 19h11.539a1 1 0 0 0 .866 -1.5l-5.769 -10a1 1 0 0 0 -1.732 0l-5.769 10a1 1 0 0 0 .865 1.5" /> <path d="M2 13h4.45" /> <path d="M18 5l-4.5 6" /> <path d="M22 9l-7.75 3.25" /> <path d="M22 15l-7 -1.5" /> </svg>"##;
const PROGRESS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20.777a8.942 8.942 0 0 1 -2.48 -.969" /> <path d="M14 3.223a9.003 9.003 0 0 1 0 17.554" /> <path d="M4.579 17.093a8.961 8.961 0 0 1 -1.227 -2.592" /> <path d="M3.124 10.5c.16 -.95 .468 -1.85 .9 -2.675l.169 -.305" /> <path d="M6.907 4.579a8.954 8.954 0 0 1 3.093 -1.356" /> </svg>"##;
const PROGRESS_ALERT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20.777a8.942 8.942 0 0 1 -2.48 -.969" /> <path d="M14 3.223a9.003 9.003 0 0 1 0 17.554" /> <path d="M4.579 17.093a8.961 8.961 0 0 1 -1.227 -2.592" /> <path d="M3.124 10.5c.16 -.95 .468 -1.85 .9 -2.675l.169 -.305" /> <path d="M6.907 4.579a8.954 8.954 0 0 1 3.093 -1.356" /> <path d="M12 8v4" /> <path d="M12 16v.01" /> </svg>"##;
const PROGRESS_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20.777a8.942 8.942 0 0 1 -2.48 -.969" /> <path d="M14 3.223a9.003 9.003 0 0 1 0 17.554" /> <path d="M4.579 17.093a8.961 8.961 0 0 1 -1.227 -2.592" /> <path d="M3.124 10.5c.16 -.95 .468 -1.85 .9 -2.675l.169 -.305" /> <path d="M6.907 4.579a8.954 8.954 0 0 1 3.093 -1.356" /> <path d="M12 9l-2 3h4l-2 3" /> </svg>"##;
const PROGRESS_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20.777a8.942 8.942 0 0 1 -2.48 -.969" /> <path d="M14 3.223a9.003 9.003 0 0 1 0 17.554" /> <path d="M4.579 17.093a8.961 8.961 0 0 1 -1.227 -2.592" /> <path d="M3.124 10.5c.16 -.95 .468 -1.85 .9 -2.675l.169 -.305" /> <path d="M6.907 4.579a8.954 8.954 0 0 1 3.093 -1.356" /> <path d="M9 12l2 2l4 -4" /> </svg>"##;
const PROGRESS_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20.777a8.942 8.942 0 0 1 -2.48 -.969" /> <path d="M14 3.223a9.003 9.003 0 0 1 0 17.554" /> <path d="M4.579 17.093a8.961 8.961 0 0 1 -1.227 -2.592" /> <path d="M3.124 10.5c.16 -.95 .468 -1.85 .9 -2.675l.169 -.305" /> <path d="M6.907 4.579a8.954 8.954 0 0 1 3.093 -1.356" /> <path d="M12 9v6" /> <path d="M15 12l-3 3l-3 -3" /> </svg>"##;
const PROGRESS_HELP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 16v.01" /> <path d="M12 13a2 2 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> <path d="M10 20.777a8.942 8.942 0 0 1 -2.48 -.969" /> <path d="M14 3.223a9.003 9.003 0 0 1 0 17.554" /> <path d="M4.579 17.093a8.961 8.961 0 0 1 -1.227 -2.592" /> <path d="M3.124 10.5c.16 -.95 .468 -1.85 .9 -2.675l.169 -.305" /> <path d="M6.907 4.579a8.954 8.954 0 0 1 3.093 -1.356" /> </svg>"##;
const PROGRESS_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20.777a8.942 8.942 0 0 1 -2.48 -.969" /> <path d="M14 3.223a9.003 9.003 0 0 1 0 17.554" /> <path d="M4.579 17.093a8.961 8.961 0 0 1 -1.227 -2.592" /> <path d="M3.124 10.5c.16 -.95 .468 -1.85 .9 -2.675l.169 -.305" /> <path d="M6.907 4.579a8.954 8.954 0 0 1 3.093 -1.356" /> <path d="M14 14l-4 -4" /> <path d="M10 14l4 -4" /> </svg>"##;
const PROMPT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7l5 5l-5 5" /> <path d="M13 17l6 0" /> </svg>"##;
const PROPELLER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 13a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M14.167 10.5c.722 -1.538 1.156 -3.043 1.303 -4.514c.22 -1.63 -.762 -2.986 -3.47 -2.986s-3.69 1.357 -3.47 2.986c.147 1.471 .581 2.976 1.303 4.514" /> <path d="M13.169 16.751c.97 1.395 2.057 2.523 3.257 3.386c1.3 1 2.967 .833 4.321 -1.512c1.354 -2.345 .67 -3.874 -.85 -4.498c-1.348 -.608 -2.868 -.985 -4.562 -1.128" /> <path d="M8.664 13c-1.693 .143 -3.213 .52 -4.56 1.128c-1.522 .623 -2.206 2.153 -.852 4.498s3.02 2.517 4.321 1.512c1.2 -.863 2.287 -1.991 3.258 -3.386" /> </svg>"##;
const PROPELLER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.448 10.432a3 3 0 1 0 4.106 4.143" /> <path d="M14.272 10.272c.66 -1.459 1.058 -2.888 1.198 -4.286c.22 -1.63 -.762 -2.986 -3.47 -2.986c-1.94 0 -3 .696 -3.355 1.69m.697 4.653c.145 .384 .309 .77 .491 1.157" /> <path d="M13.169 16.751c.97 1.395 2.057 2.523 3.257 3.386c1.02 .789 2.265 .853 3.408 -.288m1.479 -2.493c.492 -1.634 -.19 -2.726 -1.416 -3.229c-.82 -.37 -1.703 -.654 -2.65 -.852" /> <path d="M8.664 13c-1.693 .143 -3.213 .52 -4.56 1.128c-1.522 .623 -2.206 2.153 -.852 4.498s3.02 2.517 4.321 1.512c1.2 -.863 2.287 -1.991 3.258 -3.386" /> <path d="M3 3l18 18" /> </svg>"##;
const PROTOCOL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 6l-7 12" /> <path d="M20 6l-7 12" /> <path d="M5 14v.015" /> <path d="M5 10.015v.015" /> </svg>"##;
const QUESTION_MARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8a3.5 3 0 0 1 3.5 -3h1a3.5 3 0 0 1 3.5 3a3 3 0 0 1 -2 3a3 4 0 0 0 -2 4" /> <path d="M12 19l0 .01" /> </svg>"##;
const QUEUE_POP_IN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 6h-3a2 2 0 0 0 -2 2v11a2 2 0 0 0 2 2h11a2 2 0 0 0 2 -2v-3.357" /> <path d="M13 5a2 2 0 0 1 2 -2h4a2 2 0 0 1 2 2v4a2 2 0 0 1 -2 2h-4a2 2 0 0 1 -2 -2v-4" /> <path d="M13.5 10.5l-5.5 5.5" /> <path d="M8 11v5h5" /> </svg>"##;
const QUEUE_POP_OUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 5h-6a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2 -2v-6" /> <path d="M3 13h6a2 2 0 0 1 2 2v6" /> <path d="M16 3h5v5" /> <path d="M21 3l-6 6" /> </svg>"##;
const RADIOACTIVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 14.6l3 5.19a9 9 0 0 0 4.5 -7.79h-6a3 3 0 0 1 -1.5 2.6" /> <path d="M13.5 9.4l3 -5.19a9 9 0 0 0 -9 0l3 5.19a3 3 0 0 1 3 0" /> <path d="M10.5 14.6l-3 5.19a9 9 0 0 1 -4.5 -7.79h6a3 3 0 0 0 1.5 2.6" /> </svg>"##;
const RADIOACTIVE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.118 14.127c-.182 .181 -.39 .341 -.618 .473l3 5.19a9 9 0 0 0 1.856 -1.423m1.68 -2.32a8.993 8.993 0 0 0 .964 -4.047h-5" /> <path d="M13.5 9.4l3 -5.19a9 9 0 0 0 -8.536 -.25" /> <path d="M10.5 14.6l-3 5.19a9 9 0 0 1 -4.5 -7.79h6a3 3 0 0 0 1.5 2.6" /> <path d="M3 3l18 18" /> </svg>"##;
const RATING_12_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M7 15v-6" /> <path d="M15.5 12h3" /> <path d="M17 10.5v3" /> <path d="M10 10.5a1.5 1.5 0 0 1 3 0c0 .443 -.313 .989 -.612 1.393l-2.388 3.107h3" /> </svg>"##;
const RATING_14_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M7 15v-6" /> <path d="M15.5 12h3" /> <path d="M17 10.5v3" /> <path d="M12.5 15v-6m-2.5 0v4h3" /> </svg>"##;
const RATING_16_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 13.5a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> <path d="M7 15v-6" /> <path d="M15.5 12h3" /> <path d="M17 10.5v3" /> <path d="M10 13.5v-3a1.5 1.5 0 0 1 1.5 -1.5h1" /> </svg>"##;
const RATING_18_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 10.5a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> <path d="M10 13.5a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> <path d="M7 15v-6" /> <path d="M15.5 12h3" /> <path d="M17 10.5v3" /> </svg>"##;
const RATING_21_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M13 15v-6" /> <path d="M15.5 12h3" /> <path d="M17 10.5v3" /> <path d="M7 10.5a1.5 1.5 0 0 1 3 0c0 .443 -.313 .989 -.612 1.393l-2.388 3.107h3" /> </svg>"##;
const RAZOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h10v4h-10l0 -4" /> <path d="M12 7v4" /> <path d="M12 11a2 2 0 0 1 2 2v6a2 2 0 1 1 -4 0v-6a2 2 0 0 1 2 -2" /> </svg>"##;
const RAZOR_ELECTRIC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 3v2" /> <path d="M12 3v2" /> <path d="M16 3v2" /> <path d="M9 12v6a3 3 0 0 0 6 0v-6h-6" /> <path d="M8 5h8l-1 4h-6l-1 -4" /> <path d="M12 17v1" /> </svg>"##;
const RECHARGING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.038 4.5a9 9 0 0 0 -2.495 2.47" /> <path d="M3.186 10.209a9 9 0 0 0 0 3.508" /> <path d="M4.5 16.962a9 9 0 0 0 2.47 2.495" /> <path d="M10.209 20.814a9 9 0 0 0 3.5 0" /> <path d="M16.962 19.5a9 9 0 0 0 2.495 -2.47" /> <path d="M20.814 13.791a9 9 0 0 0 0 -3.508" /> <path d="M19.5 7.038a9 9 0 0 0 -2.47 -2.495" /> <path d="M13.791 3.186a9 9 0 0 0 -3.508 -.02" /> <path d="M12 8l-2 4h4l-2 4" /> <path d="M12 21a9 9 0 0 0 0 -18" /> </svg>"##;
const RECORD_MAIL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M14 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M7 15l10 0" /> </svg>"##;
const RECORD_MAIL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M18.569 14.557a3 3 0 1 0 -4.113 -4.149" /> <path d="M7 15h8" /> <path d="M3 3l18 18" /> </svg>"##;
const RECYCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17l-2 2l2 2" /> <path d="M10 19h9a2 2 0 0 0 1.75 -2.75l-.55 -1" /> <path d="M8.536 11l-.732 -2.732l-2.732 .732" /> <path d="M7.804 8.268l-4.5 7.794a2 2 0 0 0 1.506 2.89l1.141 .024" /> <path d="M15.464 11l2.732 .732l.732 -2.732" /> <path d="M18.196 11.732l-4.5 -7.794a2 2 0 0 0 -3.256 -.14l-.591 .976" /> </svg>"##;
const RECYCLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17l-2 2l2 2m-2 -2h9m1.896 -2.071a2 2 0 0 0 -.146 -.679l-.55 -1" /> <path d="M8.536 11l-.732 -2.732l-2.732 .732m2.732 -.732l-4.5 7.794a2 2 0 0 0 1.506 2.89l1.141 .024" /> <path d="M15.464 11l2.732 .732l.732 -2.732m-.732 2.732l-4.5 -7.794a2 2 0 0 0 -3.256 -.14l-.591 .976" /> <path d="M3 3l18 18" /> </svg>"##;
const REGISTERED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 15v-6h2a2 2 0 1 1 0 4h-2" /> <path d="M14 15l-2 -2" /> </svg>"##;
const RELATION_MANY_TO_MANY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M15 14v-4l3 4v-4" /> <path d="M6 14v-4l3 4v-4" /> <path d="M12 10.5l0 .01" /> <path d="M12 13.5l0 .01" /> </svg>"##;
const RELATION_ONE_TO_MANY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M7 10h1v4" /> <path d="M14 14v-4l3 4v-4" /> <path d="M11 10.5l0 .01" /> <path d="M11 13.5l0 .01" /> </svg>"##;
const RELATION_ONE_TO_ONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M8 10h1v4" /> <path d="M15 10h1v4" /> <path d="M12 10.5l0 .01" /> <path d="M12 13.5l0 .01" /> </svg>"##;
const REORDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 16a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M10 16a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 16a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M5 11v-3a3 3 0 0 1 3 -3h8a3 3 0 0 1 3 3v3" /> <path d="M16.5 8.5l2.5 2.5l2.5 -2.5" /> </svg>"##;
const REPLACE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M15 16a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M21 11v-3a2 2 0 0 0 -2 -2h-6l3 3m0 -6l-3 3" /> <path d="M3 13v3a2 2 0 0 0 2 2h6l-3 -3m0 6l3 -3" /> </svg>"##;
const REPLACE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h1a1 1 0 0 1 1 1v1m-.303 3.717a1 1 0 0 1 -.697 .283h-4a1 1 0 0 1 -1 -1v-4c0 -.28 .115 -.532 .3 -.714" /> <path d="M19 15h1a1 1 0 0 1 1 1v1m-.303 3.717a1 1 0 0 1 -.697 .283h-4a1 1 0 0 1 -1 -1v-4c0 -.28 .115 -.532 .3 -.714" /> <path d="M21 11v-3a2 2 0 0 0 -2 -2h-6l3 3m0 -6l-3 3" /> <path d="M3 13v3a2 2 0 0 0 2 2h6l-3 -3m0 6l3 -3" /> <path d="M3 3l18 18" /> </svg>"##;
const REPLACE_USER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 11v-3c0 -.53 -.211 -1.039 -.586 -1.414c-.375 -.375 -.884 -.586 -1.414 -.586h-6m0 0l3 3m-3 -3l3 -3" /> <path d="M3 13.013v3c0 .53 .211 1.039 .586 1.414c.375 .375 .884 .586 1.414 .586h6m0 0l-3 -3m3 3l-3 3" /> <path d="M16 16.502c0 .53 .211 1.039 .586 1.414c.375 .375 .884 .586 1.414 .586c.53 0 1.039 -.211 1.414 -.586c.375 -.375 .586 -.884 .586 -1.414c0 -.53 -.211 -1.039 -.586 -1.414c-.375 -.375 -.884 -.586 -1.414 -.586c-.53 0 -1.039 .211 -1.414 .586c-.375 .375 -.586 .884 -.586 1.414" /> <path d="M4 4.502c0 .53 .211 1.039 .586 1.414c.375 .375 .884 .586 1.414 .586c.53 0 1.039 -.211 1.414 -.586c.375 -.375 .586 -.884 .586 -1.414c0 -.53 -.211 -1.039 -.586 -1.414c-.375 -.375 -.884 -.586 -1.414 -.586c-.53 0 -1.039 .211 -1.414 .586c-.375 .375 -.586 .884 -.586 1.414" /> <path d="M21 21.499c0 -.53 -.211 -1.039 -.586 -1.414c-.375 -.375 -.884 -.586 -1.414 -.586h-2c-.53 0 -1.039 .211 -1.414 .586c-.375 .375 -.586 .884 -.586 1.414" /> <path d="M9 9.499c0 -.53 -.211 -1.039 -.586 -1.414c-.375 -.375 -.884 -.586 -1.414 -.586h-2c-.53 0 -1.039 .211 -1.414 .586c-.375 .375 -.586 .884 -.586 1.414" /> </svg>"##;
const RESERVED_LINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 20h6" /> <path d="M12 14v6" /> <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -6" /> <path d="M9 9h6" /> </svg>"##;
const RESTORE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.06 13a9 9 0 1 0 .49 -4.087" /> <path d="M3 4.001v5h5" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const RIBBON_HEALTH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 21s9.286 -9.841 9.286 -13.841a3.864 3.864 0 0 0 -1.182 -3.008a4.13 4.13 0 0 0 -3.104 -1.144a4.13 4.13 0 0 0 -3.104 1.143a3.864 3.864 0 0 0 -1.182 3.01c0 4 9.286 13.84 9.286 13.84" /> </svg>"##;
const ROBOT_FACE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 5h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2" /> <path d="M9 16c1 .667 2 1 3 1s2 -.333 3 -1" /> <path d="M9 7l-1 -4" /> <path d="M15 7l1 -4" /> <path d="M9 12v-1" /> <path d="M15 12v-1" /> </svg>"##;
const ROBOT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h8a2 2 0 0 1 2 2v4a2 2 0 0 1 -2 2m-4 0h-4a2 2 0 0 1 -2 -2v-4" /> <path d="M12 2v2" /> <path d="M9 12v9" /> <path d="M15 15v6" /> <path d="M5 16l4 -2" /> <path d="M9 18h6" /> <path d="M14 8v.01" /> <path d="M3 3l18 18" /> </svg>"##;
const ROTATE_3D_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a7 7 0 0 1 7 7v4l-3 -3" /> <path d="M22 11l-3 3" /> <path d="M8 15.5l-5 -3l5 -3l5 3v5.5l-5 3l0 -5.5" /> <path d="M3 12.5v5.5l5 3" /> <path d="M8 15.545l5 -3.03" /> </svg>"##;
const ROUTE_SCAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> <path d="M7 12v-3h3" /> <path d="M14 9h3v3" /> <path d="M7 9l4.414 4.414a2 2 0 0 1 .586 1.414v2.172" /> <path d="M17 9l-4.414 4.414a2 2 0 0 0 -.586 1.414v2.172" /> </svg>"##;
const ROW_INSERT_BOTTOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 6v4a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1v-4a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1" /> <path d="M12 15l0 4" /> <path d="M14 17l-4 0" /> </svg>"##;
const ROW_INSERT_TOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18v-4a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1" /> <path d="M12 9v-4" /> <path d="M10 7l4 0" /> </svg>"##;
const ROW_REMOVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 6v4a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1v-4a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1" /> <path d="M10 16l4 4" /> <path d="M10 20l4 -4" /> </svg>"##;
const RV_TRUCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M9 17h6" /> <path d="M19 17h1a1 1 0 0 0 1 -1v-4.528a2 2 0 0 0 -.211 -.894l-.96 -1.92a3 3 0 0 0 -2.683 -1.658h-11.146a3 3 0 0 0 -3 3v6a1 1 0 0 0 1 1h1" /> <path d="M3 12h18" /> <path d="M15 12v-5" /> <path d="M6 5.5a1.5 1.5 0 0 1 1.5 -1.5h7a1.5 1.5 0 0 1 1.5 1.5a1.5 1.5 0 0 1 -1.5 1.5h-7a1.5 1.5 0 0 1 -1.5 -1.5" /> </svg>"##;
const SANDBOX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.953 8.017l1.047 6.983v2a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-2l1.245 -8.297a2 2 0 0 1 1.977 -1.703h3.778" /> <path d="M3 15h18" /> <path d="M13 3l5.5 1.5" /> <path d="M15.75 3.75l-2 7" /> <path d="M7 10.5c1.667 -.667 3.333 -.667 5 0c1.667 .667 3.333 .667 5 0" /> </svg>"##;
const SCALE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 20l10 0" /> <path d="M6 6l6 -1l6 1" /> <path d="M12 3l0 17" /> <path d="M9 12l-3 -6l-3 6a3 3 0 0 0 6 0" /> <path d="M21 12l-3 -6l-3 6a3 3 0 0 0 6 0" /> </svg>"##;
const SCALE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 20h10" /> <path d="M9.452 5.425l2.548 -.425l6 1" /> <path d="M12 3v5m0 4v8" /> <path d="M9 12l-3 -6l-3 6a3 3 0 0 0 6 0" /> <path d="M18.873 14.871a3 3 0 0 0 2.127 -2.871l-3 -6l-2.677 5.355" /> <path d="M3 3l18 18" /> </svg>"##;
const SCALE_OUTLINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a4 4 0 0 1 4 -4h10a4 4 0 0 1 4 4v10a4 4 0 0 1 -4 4h-10a4 4 0 0 1 -4 -4v-10" /> <path d="M12 7c1.956 0 3.724 .802 5 2.095l-2.956 2.904a3 3 0 0 0 -2.038 -.799a3 3 0 0 0 -2.038 .798l-2.956 -2.903a6.979 6.979 0 0 1 5 -2.095" /> </svg>"##;
const SCALE_OUTLINE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h10a4 4 0 0 1 4 4v10m-1.173 2.83a3.987 3.987 0 0 1 -2.827 1.17h-10a4 4 0 0 1 -4 -4v-10c0 -1.104 .447 -2.103 1.17 -2.827" /> <path d="M11.062 7.062c.31 -.041 .622 -.062 .938 -.062c1.956 0 3.724 .802 5 2.095a142.85 142.85 0 0 0 -2 1.905m-3.723 .288a3 3 0 0 0 -1.315 .71l-2.956 -2.903a6.977 6.977 0 0 1 1.142 -.942" /> <path d="M3 3l18 18" /> </svg>"##;
const SCAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const SCAN_CUBE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.504 9.426l3 -1.714a1 1 0 0 1 .992 0l3 1.714a1 1 0 0 1 .504 .868v3.411a1 1 0 0 1 -.504 .868l-3 1.715a1 1 0 0 1 -.992 0l-3 -1.715a1 1 0 0 1 -.504 -.868v-3.41a1 1 0 0 1 .504 -.869" /> <path d="M15.75 9.964l-3.75 2.036" /> <path d="M12 12l-3.75 -2.036" /> <path d="M12 12v4.071" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const SCAN_EYE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 12q 5 -7 10 0" /> <path d="M7 12q 5 7 10 0" /> <path d="M12 12h-.01" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const SCAN_POSITION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17l3 -8l-8 3l3.5 1.5l1.5 3.5" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const SCAN_TRACES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12h6" /> <path d="M8 8h5" /> <path d="M9 16h5" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const SCHEMA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 2h5v4h-5l0 -4" /> <path d="M15 10h5v4h-5l0 -4" /> <path d="M5 18h5v4h-5l0 -4" /> <path d="M5 10h5v4h-5l0 -4" /> <path d="M10 12h5" /> <path d="M7.5 6v4" /> <path d="M7.5 14v4" /> </svg>"##;
const SCHEMA_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 2h4v4m-4 0h-1v-1" /> <path d="M15 11v-1h5v4h-2" /> <path d="M5 18h5v4h-5l0 -4" /> <path d="M5 10h5v4h-5l0 -4" /> <path d="M10 12h2" /> <path d="M7.5 7.5v2.5" /> <path d="M7.5 14v4" /> <path d="M3 3l18 18" /> </svg>"##;
const SCHOOL_BELL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 17a3 3 0 0 0 3 3" /> <path d="M14.805 6.37l2.783 -2.784a2 2 0 1 1 2.829 2.828l-2.784 2.786" /> <path d="M16.505 7.495a5.105 5.105 0 0 1 .176 7.035l-.176 .184l-1.867 1.867a3.48 3.48 0 0 0 -1.013 2.234l-.008 .23v.934c0 .327 -.13 .64 -.36 .871a.51 .51 0 0 1 -.652 .06l-.07 -.06l-9.385 -9.384a.51 .51 0 0 1 0 -.722c.198 -.198 .456 -.322 .732 -.353l.139 -.008h.933c.848 0 1.663 -.309 2.297 -.864l.168 -.157l1.867 -1.867l.16 -.153a5.105 5.105 0 0 1 7.059 .153" /> </svg>"##;
const SDK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8h-3a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-3" /> <path d="M17 8v8" /> <path d="M21 8l-3 4l3 4" /> <path d="M17 12h1" /> <path d="M10 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2h-2" /> </svg>"##;
const SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M21 21l-6 -6" /> </svg>"##;
const SEARCH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.039 5.062a7 7 0 0 0 9.91 9.89m1.584 -2.434a7 7 0 0 0 -9.038 -9.057" /> <path d="M3 3l18 18" /> </svg>"##;
const SELECT_ALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1l0 -6" /> <path d="M12 20v.01" /> <path d="M16 20v.01" /> <path d="M8 20v.01" /> <path d="M4 20v.01" /> <path d="M4 16v.01" /> <path d="M4 12v.01" /> <path d="M4 8v.01" /> <path d="M4 4v.01" /> <path d="M8 4v.01" /> <path d="M12 4v.01" /> <path d="M16 4v.01" /> <path d="M20 4v.01" /> <path d="M20 8v.01" /> <path d="M20 12v.01" /> <path d="M20 16v.01" /> <path d="M20 20v.01" /> </svg>"##;
const SEO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8h-3a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-3" /> <path d="M14 16h-4v-8h4" /> <path d="M11 12h2" /> <path d="M17 9a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -6" /> </svg>"##;
const SERVERLESS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 19l3.03 1.748a2 2 0 0 0 1.94 0l6 -3.843a2 2 0 0 0 1.03 -1.753v-6.152l-8 3l-8 3v-6.405c0 -.728 .394 -1.399 1.03 -1.752l6 -3.582a2.05 2.05 0 0 1 2 0l2.97 1.739" /> </svg>"##;
const SERVICEMARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 9h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M13 15v-6l3 4l3 -4v6" /> </svg>"##;
const SETTINGS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.325 4.317c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756 .426 1.756 2.924 0 3.35a1.724 1.724 0 0 0 -1.066 2.573c.94 1.543 -.826 3.31 -2.37 2.37a1.724 1.724 0 0 0 -2.572 1.065c-.426 1.756 -2.924 1.756 -3.35 0a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> </svg>"##;
const SETTINGS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const SETTINGS_AI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.325 4.317c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756 .426 1.756 2.924 0 3.35a1.724 1.724 0 0 0 -1.066 2.573c.94 1.543 -.826 3.31 -2.37 2.37a1.724 1.724 0 0 0 -2.572 1.065c-.426 1.756 -2.924 1.756 -3.35 0a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065" /> <path d="M9 14v-2.5a1.5 1.5 0 0 1 3 0v2.5" /> <path d="M9 13h3" /> <path d="M15 10v4" /> </svg>"##;
const SETTINGS_AUTOMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.325 4.317c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756 .426 1.756 2.924 0 3.35a1.724 1.724 0 0 0 -1.066 2.573c.94 1.543 -.826 3.31 -2.37 2.37a1.724 1.724 0 0 0 -2.572 1.065c-.426 1.756 -2.924 1.756 -3.35 0a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065" /> <path d="M10 9v6l5 -3l-5 -3" /> </svg>"##;
const SETTINGS_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.256 20.473c-.855 .907 -2.583 .643 -2.931 -.79a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.07 .26 1.488 1.29 1.254 2.15" /> <path d="M19 16l-2 3h4l-2 3" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> </svg>"##;
const SETTINGS_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.29 20.977c-.818 .132 -1.724 -.3 -1.965 -1.294a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c.983 .238 1.416 1.126 1.298 1.937" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> </svg>"##;
const SETTINGS_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.445 20.913a1.665 1.665 0 0 1 -1.12 -1.23a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.31 .318 1.643 1.79 .997 2.694" /> <path d="M15 19l2 2l4 -4" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> </svg>"##;
const SETTINGS_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.482 20.924a1.666 1.666 0 0 1 -1.157 -1.241a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.312 .318 1.644 1.794 .995 2.697" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const SETTINGS_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.003 21c-.732 .001 -1.465 -.438 -1.678 -1.317a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c.886 .215 1.325 .957 1.318 1.694" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const SETTINGS_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.038 20.666c-.902 .665 -2.393 .337 -2.713 -.983a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 .402 2.248" /> <path d="M15 12a3 3 0 1 0 -1.724 2.716" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const SETTINGS_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.52 20.924c-.87 .262 -1.93 -.152 -2.195 -1.241a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.088 .264 1.502 1.323 1.242 2.192" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> </svg>"##;
const SETTINGS_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.004 18.401a1.724 1.724 0 0 0 -1.329 1.282c-.426 1.756 -2.924 1.756 -3.35 0a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.079 .262 1.495 1.305 1.248 2.17" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const SETTINGS_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.231 20.828a1.668 1.668 0 0 1 -.906 -1.145a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c.509 .123 .87 .421 1.084 .792" /> <path d="M14.882 11.165a3.001 3.001 0 1 0 -4.31 3.474" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const SETTINGS_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.488 20.933c-.863 .243 -1.902 -.174 -2.163 -1.25a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756 .426 1.756 2.924 0 3.35c-.535 .13 -.976 .507 -1.187 1.016c-.049 .118 -.084 .185 -.106 .309" /> <path d="M16 19h6" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> </svg>"##;
const SETTINGS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.451 5.437c.418 -.218 .75 -.609 .874 -1.12c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756 .426 1.756 2.924 0 3.35c-.486 .118 -.894 .44 -1.123 .878m-.188 3.803c-.517 .523 -1.349 .734 -2.125 .262a1.724 1.724 0 0 0 -2.572 1.065c-.426 1.756 -2.924 1.756 -3.35 0a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.472 -.774 -.262 -1.604 .259 -2.121" /> <path d="M9.889 9.869a3 3 0 1 0 4.226 4.26m.592 -3.424a3.012 3.012 0 0 0 -1.419 -1.415" /> <path d="M3 3l18 18" /> </svg>"##;
const SETTINGS_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.004 20.69c-.905 .632 -2.363 .296 -2.679 -1.007a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.314 .319 1.645 1.798 .992 2.701" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const SETTINGS_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.578 20.905c-.88 .299 -1.983 -.109 -2.253 -1.222a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c.574 .14 .96 .5 1.16 .937" /> <path d="M14.99 12.256a3 3 0 1 0 -2.33 2.671" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const SETTINGS_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.483 20.935c-.862 .239 -1.898 -.178 -2.158 -1.252a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.08 .262 1.496 1.308 1.247 2.173" /> <path d="M16 19h6" /> <path d="M19 16v6" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> </svg>"##;
const SETTINGS_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.69 18.498c-.508 .21 -.885 .65 -1.015 1.185c-.426 1.756 -2.924 1.756 -3.35 0a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572a1.67 1.67 0 0 1 1.179 .982" /> <path d="M14.95 12.553a3 3 0 1 0 -1.211 1.892" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const SETTINGS_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.646 20.965a1.67 1.67 0 0 1 -1.321 -1.282a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c.728 .177 1.154 .71 1.279 1.303" /> <path d="M14.985 11.694a3 3 0 1 0 -3.29 3.29" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const SETTINGS_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.004 21c-.732 .002 -1.466 -.437 -1.679 -1.317a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.306 .317 1.64 1.78 1.004 2.684" /> <path d="M12 15a3 3 0 1 0 0 -6a3 3 0 0 0 0 6" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const SETTINGS_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.992 21c-.728 -.003 -1.455 -.442 -1.667 -1.317a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c.882 .214 1.32 .95 1.317 1.684" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const SETTINGS_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.325 19.683a1.723 1.723 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572a1.67 1.67 0 0 1 1.106 .831" /> <path d="M14.89 11.195a3.001 3.001 0 1 0 -4.457 3.364" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const SETTINGS_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.501 20.93c-.866 .25 -1.914 -.166 -2.176 -1.247a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.074 .26 1.49 1.296 1.252 2.158" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> </svg>"##;
const SETTINGS_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.675 19.683c-.426 1.756 -2.924 1.756 -3.35 0a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756 .426 1.756 2.924 0 3.35a1.66 1.66 0 0 0 -.324 .114" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M15 6a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M8.7 10.7l6.6 -3.4" /> <path d="M8.7 13.3l6.6 3.4" /> </svg>"##;
const SHARE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M15 6a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M15.861 15.896a3 3 0 0 0 4.265 4.22m.578 -3.417a3.012 3.012 0 0 0 -1.507 -1.45" /> <path d="M8.7 10.7l1.336 -.688m2.624 -1.352l2.64 -1.36" /> <path d="M8.7 13.3l6.6 3.4" /> <path d="M3 3l18 18" /> </svg>"##;
const SHIELD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a12 12 0 0 0 8.5 3a12 12 0 0 1 -8.5 15a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3" /> </svg>"##;
const SHIELD_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.342 20.566c-.436 .17 -.884 .315 -1.342 .434a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3a12 12 0 0 1 .117 6.34" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const SHIELD_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.277 20.925c-.092 .026 -.184 .051 -.277 .075a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3a12 12 0 0 1 .145 6.232" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const SHIELD_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.46 20.846a12 12 0 0 1 -7.96 -14.846a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3a12 12 0 0 1 -.09 7.06" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const SHIELD_CHECKERED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a12 12 0 0 0 8.5 3a12 12 0 0 1 -8.5 15a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3" /> <path d="M12 3v18" /> <path d="M3.5 12h17" /> </svg>"##;
const SHIELD_CHEVRON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a12 12 0 0 0 8.5 3a12 12 0 0 1 -8.5 15a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3" /> <path d="M4 14l8 -3l8 3" /> </svg>"##;
const SHIELD_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3a12 12 0 0 1 -.078 7.024" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const SHIELD_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3c.568 1.933 .635 3.957 .223 5.89" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const SHIELD_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.018 20.687c-.333 .119 -.673 .223 -1.018 .313a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3c.433 1.472 .575 2.998 .436 4.495" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const SHIELD_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.444 20.876c-.147 .044 -.295 .085 -.444 .124a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3a12 12 0 0 1 .117 6.343" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const SHIELD_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.04 19.745c-.942 .551 -1.964 .976 -3.04 1.255a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3a12 12 0 0 1 .195 6.015" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const SHIELD_HALF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a12 12 0 0 0 8.5 3a12 12 0 0 1 -8.5 15a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3" /> <path d="M12 3v18" /> </svg>"##;
const SHIELD_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3a12.01 12.01 0 0 1 .378 5" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const SHIELD_LOCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a12 12 0 0 0 8.5 3a12 12 0 0 1 -8.5 15a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3" /> <path d="M11 11a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M12 12l0 2.5" /> </svg>"##;
const SHIELD_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.46 20.871c-.153 .046 -.306 .089 -.46 .129a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3a12 12 0 0 1 -.916 9.015" /> <path d="M16 19h6" /> </svg>"##;
const SHIELD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.67 17.667a12 12 0 0 1 -5.67 3.333a12 12 0 0 1 -8.5 -15c.794 .036 1.583 -.006 2.357 -.124m3.128 -.926a11.997 11.997 0 0 0 3.015 -1.95a12 12 0 0 0 8.5 3a12 12 0 0 1 -1.116 9.376" /> <path d="M3 3l18 18" /> </svg>"##;
const SHIELD_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.004 20.692c-.329 .117 -.664 .22 -1.004 .308a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3a12 12 0 0 1 -.081 7.034" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const SHIELD_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.597 20.829a12 12 0 0 1 -.597 .171a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3c.506 1.72 .614 3.512 .342 5.248" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const SHIELD_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.462 20.87c-.153 .047 -.307 .09 -.462 .13a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3a12 12 0 0 1 .11 6.37" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const SHIELD_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.065 19.732c-.95 .557 -1.98 .986 -3.065 1.268a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3c.51 1.738 .617 3.55 .333 5.303" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const SHIELD_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3c.539 1.832 .627 3.747 .283 5.588" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const SHIELD_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3a12 12 0 0 1 .193 6.025" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const SHIELD_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.143 20.743a12 12 0 0 1 -7.643 -14.743a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3c.504 1.716 .614 3.505 .343 5.237" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const SHIELD_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.442 20.876a13.12 13.12 0 0 1 -.442 .124a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3a12 12 0 0 1 .119 6.336" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const SHIELD_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.252 20.601c-.408 .155 -.826 .288 -1.252 .399a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3a12 12 0 0 0 8.5 3a12 12 0 0 1 -.19 7.357" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const SITEMAP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -2" /> <path d="M15 17a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -2" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -2" /> <path d="M6 15v-1a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v1" /> <path d="M12 9l0 3" /> </svg>"##;
const SITEMAP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -2" /> <path d="M19 15a2 2 0 0 1 2 2m-.591 3.42c-.362 .358 -.86 .58 -1.409 .58h-2a2 2 0 0 1 -2 -2v-2c0 -.549 .221 -1.046 .579 -1.407" /> <path d="M9 5a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2" /> <path d="M6 15v-1a2 2 0 0 1 2 -2h4m4 0a2 2 0 0 1 2 2" /> <path d="M3 3l18 18" /> </svg>"##;
const SKEW_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5.205v13.59a1 1 0 0 0 1.184 .983l14 -2.625a1 1 0 0 0 .816 -.983v-8.34a1 1 0 0 0 -.816 -.983l-14 -2.625a1 1 0 0 0 -1.184 .983" /> </svg>"##;
const SKEW_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.326 19h15.348a1 1 0 0 0 .962 -1.275l-3.429 -12a1 1 0 0 0 -.961 -.725h-8.492a1 1 0 0 0 -.961 .725l-3.429 12a1 1 0 0 0 .962 1.275" /> </svg>"##;
const SKULL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4c4.418 0 8 3.358 8 7.5c0 1.901 -.755 3.637 -2 4.96l0 2.54a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1v-2.54c-1.245 -1.322 -2 -3.058 -2 -4.96c0 -4.142 3.582 -7.5 8 -7.5" /> <path d="M10 17v3" /> <path d="M14 17v3" /> <path d="M8 11a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M14 11a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const SLASH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 5l-10 14" /> </svg>"##;
const SLASHES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 5l-10 14" /> <path d="M20 5l-10 14" /> </svg>"##;
const SMART_HOME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 8.71l-5.333 -4.148a2.666 2.666 0 0 0 -3.274 0l-5.334 4.148a2.665 2.665 0 0 0 -1.029 2.105v7.2a2 2 0 0 0 2 2h12a2 2 0 0 0 2 -2v-7.2c0 -.823 -.38 -1.6 -1.03 -2.105" /> <path d="M16 15c-2.21 1.333 -5.792 1.333 -8 0" /> </svg>"##;
const SMART_HOME_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.097 7.125l-2.037 1.585a2.665 2.665 0 0 0 -1.029 2.105v7.2a2 2 0 0 0 2 2h12c.559 0 1.064 -.229 1.427 -.598m.572 -3.417v-5.185c0 -.823 -.38 -1.6 -1.03 -2.105l-5.333 -4.148a2.666 2.666 0 0 0 -3.274 0l-1.029 .8" /> <path d="M15.332 15.345c-2.213 .976 -5.335 .86 -7.332 -.345" /> <path d="M3 3l18 18" /> </svg>"##;
const SMOKING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 14a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1l0 -2" /> <path d="M8 13l0 4" /> <path d="M16 5v.5a2 2 0 0 0 2 2a2 2 0 0 1 2 2v.5" /> </svg>"##;
const SMOKING_NO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13l0 4" /> <path d="M16 5v.5a2 2 0 0 0 2 2a2 2 0 0 1 2 2v.5" /> <path d="M3 3l18 18" /> <path d="M17 13h3a1 1 0 0 1 1 1v2c0 .28 -.115 .533 -.3 .714m-3.7 .286h-13a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h9" /> </svg>"##;
const SNOWBOARDING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 3a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M7 19l4 -2.5l-.5 -1.5" /> <path d="M16 21l-1 -6l-4.5 -3l3.5 -6" /> <path d="M7 9l1.5 -3h5.5l2 4l3 1" /> <path d="M3 17c.399 1.154 .899 1.805 1.5 1.951c6 1.464 10.772 2.262 13.5 2.927c1.333 .325 2.333 0 3 -.976" /> </svg>"##;
const SOCIAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M9 14a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M12 7l0 4" /> <path d="M6.7 17.8l2.8 -2" /> <path d="M17.3 17.8l-2.8 -2" /> </svg>"##;
const SOCIAL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17.57 17.602a2 2 0 0 0 2.83 2.827" /> <path d="M11.113 11.133a3 3 0 1 0 3.765 3.715" /> <path d="M12 7v1" /> <path d="M6.7 17.8l2.8 -2" /> <path d="M17.3 17.8l-2.8 -2" /> <path d="M3 3l18 18" /> </svg>"##;
const SOFA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 11a2 2 0 0 1 2 2v1h12v-1a2 2 0 1 1 4 0v5a1 1 0 0 1 -1 1h-18a1 1 0 0 1 -1 -1v-5a2 2 0 0 1 2 -2" /> <path d="M4 11v-3a3 3 0 0 1 3 -3h10a3 3 0 0 1 3 3v3" /> <path d="M12 5v9" /> </svg>"##;
const SOFA_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 14v-1a2 2 0 1 1 4 0v5m-3 1h-16a1 1 0 0 1 -1 -1v-5a2 2 0 1 1 4 0v1h8" /> <path d="M4 11v-3c0 -1.082 .573 -2.03 1.432 -2.558m3.568 -.442h8a3 3 0 0 1 3 3v3" /> <path d="M12 5v3m0 4v2" /> <path d="M3 3l18 18" /> </svg>"##;
const SOLAR_ELECTRICITY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6.28v11.44a1 1 0 0 0 1.243 .97l6 -1.5a1 1 0 0 0 .757 -.97v-8.44a1 1 0 0 0 -.757 -.97l-6 -1.5a1 1 0 0 0 -1.243 .97" /> <path d="M8 6v12" /> <path d="M12 12h-8" /> <path d="M20 7l-3 5h4l-3 5" /> </svg>"##;
const SOLAR_PANEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.28 14h15.44a1 1 0 0 0 .97 -1.243l-1.5 -6a1 1 0 0 0 -.97 -.757h-12.44a1 1 0 0 0 -.97 .757l-1.5 6a1 1 0 0 0 .97 1.243" /> <path d="M4 10h16" /> <path d="M10 6l-1 8" /> <path d="M14 6l1 8" /> <path d="M12 14v4" /> <path d="M7 18h10" /> </svg>"##;
const SOLAR_PANEL_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 2a4 4 0 1 0 8 0" /> <path d="M4 3h1" /> <path d="M19 3h1" /> <path d="M12 9v1" /> <path d="M17.2 7.2l.707 .707" /> <path d="M6.8 7.2l-.7 .7" /> <path d="M4.28 21h15.44a1 1 0 0 0 .97 -1.243l-1.5 -6a1 1 0 0 0 -.97 -.757h-12.44a1 1 0 0 0 -.97 .757l-1.5 6a1 1 0 0 0 .97 1.243" /> <path d="M4 17h16" /> <path d="M10 13l-1 8" /> <path d="M14 13l1 8" /> </svg>"##;
const SORT_ASCENDING_SHAPES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 15l3 3l3 -3" /> <path d="M7 6v12" /> <path d="M14 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-4" /> <path d="M17 14l-3.5 6h7l-3.5 -6" /> </svg>"##;
const SORT_ASCENDING_SMALL_BIG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 15l3 3l3 -3" /> <path d="M7 6v12" /> <path d="M14 5.667c0 -.369 .298 -.667 .667 -.667h2.666c.369 0 .667 .298 .667 .667v2.666a.667 .667 0 0 1 -.667 .667h-2.666a.667 .667 0 0 1 -.667 -.667v-2.666" /> <path d="M14 13.167c0 -.645 .522 -1.167 1.167 -1.167h4.666c.645 0 1.167 .522 1.167 1.167v4.666c0 .645 -.522 1.167 -1.167 1.167h-4.666a1.167 1.167 0 0 1 -1.167 -1.167v-4.666" /> </svg>"##;
const SORT_DESCENDING_SHAPES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 15l3 3l3 -3" /> <path d="M7 6v12" /> <path d="M14 15a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-4" /> <path d="M17 4l-3.5 6h7l-3.5 -6" /> </svg>"##;
const SORT_DESCENDING_SMALL_BIG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15l-3 3l-3 -3" /> <path d="M7 6v12" /> <path d="M14 18.333c0 .369 .298 .667 .667 .667h2.666a.667 .667 0 0 0 .667 -.667v-2.666a.667 .667 0 0 0 -.667 -.667h-2.666a.667 .667 0 0 0 -.667 .667v2.666" /> <path d="M14 10.833c0 .645 .522 1.167 1.167 1.167h4.666c.645 0 1.167 -.522 1.167 -1.167v-4.666c0 -.645 -.522 -1.167 -1.167 -1.167h-4.666c-.645 0 -1.167 .522 -1.167 1.167v4.666" /> </svg>"##;
const SOS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M17 15c.345 .6 1.258 1 2 1a2 2 0 1 0 0 -4a2 2 0 1 1 0 -4c.746 0 1.656 .394 2 1" /> <path d="M3 15c.345 .6 1.258 1 2 1a2 2 0 1 0 0 -4a2 2 0 1 1 0 -4c.746 0 1.656 .394 2 1" /> </svg>"##;
const SOURCE_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.5 4h2.5a3 3 0 0 1 3 3v10a3 3 0 0 1 -3 3h-10a3 3 0 0 1 -3 -3v-5" /> <path d="M6 5l-2 2l2 2" /> <path d="M10 9l2 -2l-2 -2" /> </svg>"##;
const SPACES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.045 9.777a6 6 0 1 0 5.951 .023" /> <path d="M11.997 20.196a6 6 0 1 0 -2.948 -5.97" /> <path d="M17.95 9.785q .05 -.386 .05 -.785a6 6 0 1 0 -3.056 5.23" /> </svg>"##;
const SPARKLES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 18a2 2 0 0 1 2 2a2 2 0 0 1 2 -2a2 2 0 0 1 -2 -2a2 2 0 0 1 -2 2m0 -12a2 2 0 0 1 2 2a2 2 0 0 1 2 -2a2 2 0 0 1 -2 -2a2 2 0 0 1 -2 2m-7 12a6 6 0 0 1 6 -6a6 6 0 0 1 -6 -6a6 6 0 0 1 -6 6a6 6 0 0 1 6 6" /> </svg>"##;
const SPIRAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12.057a1.9 1.9 0 0 0 .614 .743c1.06 .713 2.472 .112 3.043 -.919c.839 -1.513 -.022 -3.368 -1.525 -4.08c-2 -.95 -4.371 .154 -5.24 2.086c-1.095 2.432 .29 5.248 2.71 6.246c2.931 1.208 6.283 -.418 7.438 -3.255c1.36 -3.343 -.557 -7.134 -3.896 -8.41c-3.855 -1.474 -8.2 .68 -9.636 4.422c-1.63 4.253 .823 9.024 5.082 10.576c4.778 1.74 10.118 -.941 11.833 -5.59a9.354 9.354 0 0 0 .577 -2.813" /> </svg>"##;
const SPIRAL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12.057a1.9 1.9 0 0 0 .614 .743c.682 .459 1.509 .374 2.164 -.02m1.103 -2.92a3.298 3.298 0 0 0 -1.749 -2.059a3.6 3.6 0 0 0 -.507 -.195m-3.385 .634a4.154 4.154 0 0 0 -1.347 1.646c-1.095 2.432 .29 5.248 2.71 6.246c1.955 .806 4.097 .35 5.65 -.884m1.745 -2.268l.043 -.103c1.36 -3.343 -.557 -7.134 -3.896 -8.41c-1.593 -.61 -3.27 -.599 -4.79 -.113m-2.579 1.408a7.574 7.574 0 0 0 -2.268 3.128c-1.63 4.253 .823 9.024 5.082 10.576c3.211 1.17 6.676 .342 9.124 -1.738m1.869 -2.149a9.354 9.354 0 0 0 1.417 -4.516" /> <path d="M3 3l18 18" /> </svg>"##;
const SPRAY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12a2 2 0 0 1 2 -2h4a2 2 0 0 1 2 2v7a2 2 0 0 1 -2 2h-4a2 2 0 0 1 -2 -2l0 -7" /> <path d="M6 10v-4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v4" /> <path d="M15 7h.01" /> <path d="M18 9h.01" /> <path d="M18 5h.01" /> <path d="M21 3h.01" /> <path d="M21 7h.01" /> <path d="M21 11h.01" /> <path d="M10 7h1" /> </svg>"##;
const SPY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 11h18" /> <path d="M5 11v-4a3 3 0 0 1 3 -3h8a3 3 0 0 1 3 3v4" /> <path d="M4 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M14 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M10 17h4" /> </svg>"##;
const SPY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 11h8m4 0h6" /> <path d="M5 11v-4c0 -.571 .16 -1.105 .437 -1.56m2.563 -1.44h8a3 3 0 0 1 3 3v4" /> <path d="M4 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M14.88 14.877a3 3 0 1 0 4.239 4.247m.59 -3.414a3.012 3.012 0 0 0 -1.425 -1.422" /> <path d="M10 17h4" /> <path d="M3 3l18 18" /> </svg>"##;
const SQUARE_F0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M13 10.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M8 12h2" /> <path d="M10 9h-2v6" /> </svg>"##;
const SQUARE_F1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M13 11l2 -2v6" /> <path d="M8 12h2" /> <path d="M10 9h-2v6" /> </svg>"##;
const SQUARE_F2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M13 9h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2" /> <path d="M8 12h2" /> <path d="M10 9h-2v6" /> </svg>"##;
const SQUARE_F3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M13 9.5a.5 .5 0 0 1 .5 -.5h1a1.5 1.5 0 0 1 0 3h-.5h.5a1.5 1.5 0 0 1 0 3h-1a.5 .5 0 0 1 -.5 -.5" /> <path d="M8 12h2" /> <path d="M10 9h-2v6" /> </svg>"##;
const SQUARE_F4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M13 9v2a1 1 0 0 0 1 1h1" /> <path d="M16 9v6" /> <path d="M8 12h2" /> <path d="M10 9h-2v6" /> </svg>"##;
const SQUARE_F5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M13 14.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2v-3h3" /> <path d="M8 12h2" /> <path d="M10 9h-2v6" /> </svg>"##;
const SQUARE_F6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M16 9.75a.75 .75 0 0 0 -.75 -.75h-1.25a1 1 0 0 0 -1 1v4a1 1 0 0 0 1 1h1a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2" /> <path d="M8 12h2" /> <path d="M10 9h-2v6" /> </svg>"##;
const SQUARE_F7_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M13 9h3l-1.5 6" /> <path d="M8 12h2" /> <path d="M10 9h-2v6" /> </svg>"##;
const SQUARE_F8_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M14.5 12h-.5a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h1a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1" /> <path d="M8 12h2" /> <path d="M10 9h-2v6" /> </svg>"##;
const SQUARE_F9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M13 14.25c0 .414 .336 .75 .75 .75h1.5a.75 .75 0 0 0 .75 -.75v-4.5a.75 .75 0 0 0 -.75 -.75h-1.5a.75 .75 0 0 0 -.75 .75v1.5c0 .414 .336 .75 .75 .75h2.25" /> <path d="M8 12h2" /> <path d="M10 9h-2v6" /> </svg>"##;
const SQUARE_ROOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h2l4 8l4 -16h8" /> </svg>"##;
const SQUARE_ROOT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 12h1c1 0 1 1 2.016 3.527c.984 2.473 .984 3.473 1.984 3.473h1" /> <path d="M12 19c1.5 0 3 -2 4 -3.5s2.5 -3.5 4 -3.5" /> <path d="M3 12h1l3 8l3 -16h10" /> </svg>"##;
const STACK_BACK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8l8 4l8 -4l-8 -4l-8 4" /> <path d="M12 16l-4 -2l-4 2l8 4l8 -4l-4 -2l-4 2" fill="currentColor" /> <path d="M8 10l-4 2l4 2m8 0l4 -2l-4 -2" /> </svg>"##;
const STACK_BACKWARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 12l6 -3l-8 -4l-8 4l6 3" /> <path d="M10 12l-6 3l8 4l8 -4l-6 -3l-2 1l-2 -1" fill="currentColor" /> </svg>"##;
const STACK_FORWARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5l-8 4l8 4l8 -4l-8 -4" fill="currentColor" /> <path d="M10 12l-6 3l8 4l8 -4l-6 -3" /> </svg>"##;
const STACK_FRONT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4l-8 4l8 4l8 -4l-8 -4" fill="currentColor" /> <path d="M8 14l-4 2l8 4l8 -4l-4 -2" /> <path d="M8 10l-4 2l8 4l8 -4l-4 -2" /> </svg>"##;
const STACK_MIDDLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 10l4 -2l-8 -4l-8 4l4 2" /> <path d="M12 12l-4 -2l-4 2l8 4l8 -4l-4 -2l-4 2" fill="currentColor" /> <path d="M8 14l-4 2l8 4l8 -4l-4 -2" /> </svg>"##;
const STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873l-6.158 -3.245" /> </svg>"##;
const STAR_HALF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l.007 15.748" /> </svg>"##;
const STAR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M10.012 6.016l1.981 -4.014l3.086 6.253l6.9 1l-4.421 4.304m.012 4.01l.588 3.426l-6.158 -3.245l-6.172 3.245l1.179 -6.873l-5 -4.867l6.327 -.917" /> </svg>"##;
const STARS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.8 19.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> <path d="M6.2 19.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> <path d="M12 9.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const STARS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.373 13.371l.076 -.154a.392 .392 0 0 1 .702 0l.907 1.831m.367 .39c.498 .071 1.245 .18 2.24 .324a.39 .39 0 0 1 .217 .665c-.326 .316 -.57 .553 -.732 .712m-.611 3.405a.39 .39 0 0 1 -.567 .411l-2.172 -1.138l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l1.601 -.232" /> <path d="M6.2 19.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> <path d="M9.557 5.556l1 -.146l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.014 .187m-4.153 -.166l-.744 .39a.392 .392 0 0 1 -.568 -.41l.188 -1.093" /> <path d="M3 3l18 18" /> </svg>"##;
const STATUS_CHANGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6 12v-2a6 6 0 1 1 12 0v2" /> <path d="M15 9l3 3l3 -3" /> </svg>"##;
const STEAM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 4a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M19 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 20a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M5.5 5.5l3 3" /> <path d="M15.5 15.5l3 3" /> <path d="M18.5 5.5l-3 3" /> <path d="M8.5 15.5l-3 3" /> </svg>"##;
const STEREO_GLASSES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 3h-2l-3 9" /> <path d="M16 3h2l3 9" /> <path d="M3 12v7a1 1 0 0 0 1 1h4.586a1 1 0 0 0 .707 -.293l2 -2a1 1 0 0 1 1.414 0l2 2a1 1 0 0 0 .707 .293h4.586a1 1 0 0 0 1 -1v-7h-18" /> <path d="M7 16h1" /> <path d="M16 16h1" /> </svg>"##;
const STETHOSCOPE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4h-1a2 2 0 0 0 -2 2v3.5a5.5 5.5 0 0 0 11 0v-3.5a2 2 0 0 0 -2 -2h-1" /> <path d="M8 15a6 6 0 1 0 12 0v-3" /> <path d="M11 3v2" /> <path d="M6 3v2" /> <path d="M18 10a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const STETHOSCOPE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.172 4.179a2 2 0 0 0 -1.172 1.821v3.5a5.5 5.5 0 0 0 9.856 3.358m1.144 -2.858v-4a2 2 0 0 0 -2 -2h-1" /> <path d="M8 15a6 6 0 0 0 10.714 3.712m1.216 -2.798c.046 -.3 .07 -.605 .07 -.914v-3" /> <path d="M11 3v2" /> <path d="M18 10a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 3l18 18" /> </svg>"##;
const STOPWATCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 13a7 7 0 1 0 14 0a7 7 0 0 0 -14 0" /> <path d="M14.5 10.5l-2.5 2.5" /> <path d="M17 8l1 -1" /> <path d="M14 3h-4" /> </svg>"##;
const SUBTASK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 9l6 0" /> <path d="M4 5l4 0" /> <path d="M6 5v11a1 1 0 0 0 1 1h5" /> <path d="M12 8a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1l0 -2" /> <path d="M12 16a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1l0 -2" /> </svg>"##;
const SUM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 16v2a1 1 0 0 1 -1 1h-11l6 -7l-6 -7h11a1 1 0 0 1 1 1v2" /> </svg>"##;
const SUM_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 18a1 1 0 0 1 -1 1h-11l6 -7m-3 -7h8a1 1 0 0 1 1 1v2" /> <path d="M3 3l18 18" /> </svg>"##;
const SUNGLASSES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h-2l-3 10" /> <path d="M16 4h2l3 10" /> <path d="M10 16h4" /> <path d="M21 16.5a3.5 3.5 0 0 1 -7 0v-2.5h7v2.5" /> <path d="M10 16.5a3.5 3.5 0 0 1 -7 0v-2.5h7v2.5" /> <path d="M4 14l4.5 4.5" /> <path d="M15 14l4.5 4.5" /> </svg>"##;
const SWIPE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 16.572v2.42a2.01 2.01 0 0 1 -2.009 2.008h-7.981a2.01 2.01 0 0 1 -2.01 -2.009v-7.981a2.01 2.01 0 0 1 2.009 -2.01h2.954" /> <path d="M9.167 4.511a2.04 2.04 0 0 1 2.496 -1.441l7.826 2.097a2.04 2.04 0 0 1 1.441 2.496l-2.097 7.826a2.04 2.04 0 0 1 -2.496 1.441l-7.827 -2.097a2.04 2.04 0 0 1 -1.441 -2.496l2.098 -7.827l0 .001" /> </svg>"##;
const TABLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M3 10h18" /> <path d="M10 3v18" /> </svg>"##;
const TABLE_ALIAS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12v-7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-7" /> <path d="M3 10h18" /> <path d="M10 3v10" /> <path d="M2 17a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-4" /> </svg>"##;
const TABLE_COLUMN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 10h11" /> <path d="M10 3v18" /> <path d="M9 3l-6 6" /> <path d="M10 7l-7 7" /> <path d="M10 12l-7 7" /> <path d="M10 17l-4 4" /> </svg>"##;
const TABLE_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -14" /> <path d="M3 10h18" /> <path d="M10 3v18" /> </svg>"##;
const TABLE_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-7.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v7.5" /> <path d="M3 10h18" /> <path d="M10 3v18" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const TABLE_EXPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-7.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v7.5" /> <path d="M3 10h18" /> <path d="M10 3v18" /> <path d="M16 19h6" /> <path d="M19 16l3 3l-3 3" /> </svg>"##;
const TABLE_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-6.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v6" /> <path d="M3 10h18" /> <path d="M10 3v18" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const TABLE_IMPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-7a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v8" /> <path d="M3 10h18" /> <path d="M10 3v18" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const TABLE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-7.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10" /> <path d="M3 10h18" /> <path d="M10 3v18" /> <path d="M16 19h6" /> </svg>"##;
const TABLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h12a2 2 0 0 1 2 2v12m-.585 3.413a1.994 1.994 0 0 1 -1.415 .587h-14a2 2 0 0 1 -2 -2v-14c0 -.55 .223 -1.05 .583 -1.412" /> <path d="M3 10h7m4 0h7" /> <path d="M10 3v3m0 4v11" /> <path d="M3 3l18 18" /> </svg>"##;
const TABLE_OPTIONS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-7a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v7" /> <path d="M3 10h18" /> <path d="M10 3v18" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const TABLE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-7.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v7.5" /> <path d="M3 10h18" /> <path d="M10 3v18" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const TABLE_ROW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 3l-6 6" /> <path d="M14 3l-7 7" /> <path d="M19 3l-7 7" /> <path d="M21 6l-4 4" /> <path d="M3 10h18" /> <path d="M10 10v11" /> </svg>"##;
const TABLE_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-7a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v8" /> <path d="M3 10h18" /> <path d="M10 3v18" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const TABLE_SHORTCUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 13v-8a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-8" /> <path d="M3 10h18" /> <path d="M10 3v11" /> <path d="M2 22l5 -5" /> <path d="M7 21.5v-4.5h-4.5" /> </svg>"##;
const TABLE_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> <path d="M12 21h-7a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v7" /> <path d="M3 10h18" /> <path d="M10 3v18" /> </svg>"##;
const TALLYMARK_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5l0 14" /> </svg>"##;
const TALLYMARK_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5l0 14" /> <path d="M14 5l0 14" /> </svg>"##;
const TALLYMARK_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 5l0 14" /> <path d="M12 5l0 14" /> <path d="M16 5l0 14" /> </svg>"##;
const TALLYMARK_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 5l0 14" /> <path d="M10 5l0 14" /> <path d="M14 5l0 14" /> <path d="M18 5l0 14" /> </svg>"##;
const TALLYMARKS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 5l0 14" /> <path d="M10 5l0 14" /> <path d="M14 5l0 14" /> <path d="M18 5l0 14" /> <path d="M3 17l18 -10" /> </svg>"##;
const TELESCOPE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 21l6 -5l6 5" /> <path d="M12 13v8" /> <path d="M3.294 13.678l.166 .281c.52 .88 1.624 1.265 2.605 .91l14.242 -5.165a1.023 1.023 0 0 0 .565 -1.456l-2.62 -4.705a1.087 1.087 0 0 0 -1.447 -.42l-.056 .032l-12.694 7.618c-1.02 .613 -1.357 1.897 -.76 2.905l-.001 0" /> <path d="M14 5l3 5.5" /> </svg>"##;
const TELESCOPE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 21l6 -5l6 5" /> <path d="M12 13v8" /> <path d="M8.238 8.264l-4.183 2.51c-1.02 .614 -1.357 1.898 -.76 2.906l.165 .28c.52 .88 1.624 1.266 2.605 .91l6.457 -2.34m2.907 -1.055l4.878 -1.77a1.023 1.023 0 0 0 .565 -1.455l-2.62 -4.705a1.087 1.087 0 0 0 -1.447 -.42l-.056 .032l-6.016 3.61" /> <path d="M14 5l3 5.5" /> <path d="M3 3l18 18" /> </svg>"##;
const TEMPERATURE_SNOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 13.5a4 4 0 1 0 4 0v-8.5a2 2 0 1 0 -4 0v8.5" /> <path d="M4 9h4" /> <path d="M14.75 4l1 2h2.25" /> <path d="M17 4l-3 5l2 3" /> <path d="M20.25 10l-1.25 2l1.25 2" /> <path d="M22 12h-6l-2 3" /> <path d="M18 18h-2.25l-1 2" /> <path d="M17 20l-3 -5h-1" /> <path d="M12 9l2.088 .008" /> </svg>"##;
const TEMPERATURE_SUN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 13.5a4 4 0 1 0 4 0v-8.5a2 2 0 1 0 -4 0v8.5" /> <path d="M4 9h4" /> <path d="M13 16a4 4 0 1 0 0 -8a4.07 4.07 0 0 0 -1 .124" /> <path d="M13 3v1" /> <path d="M21 12h1" /> <path d="M13 20v1" /> <path d="M19.4 5.6l-.7 .7" /> <path d="M18.7 17.7l.7 .7" /> </svg>"##;
const TERMINAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7l5 5l-5 5" /> <path d="M12 19l7 0" /> </svg>"##;
const TERMINAL_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9l3 3l-3 3" /> <path d="M13 15l3 0" /> <path d="M3 6a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -12" /> </svg>"##;
const TEST_PIPE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 8.04l-12.122 12.124a2.857 2.857 0 1 1 -4.041 -4.04l12.122 -12.124" /> <path d="M7 13h8" /> <path d="M19 15l1.5 1.6a2 2 0 1 1 -3 0l1.5 -1.6" /> <path d="M15 3l6 6" /> </svg>"##;
const TEST_PIPE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 3v15a3 3 0 0 1 -6 0v-15" /> <path d="M9 12h6" /> <path d="M8 3h8" /> </svg>"##;
const TEST_PIPE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 8.04a803.533 803.533 0 0 0 -4 3.96m-2 2c-1.085 1.085 -3.125 3.14 -6.122 6.164a2.857 2.857 0 0 1 -4.041 -4.04c3.018 -3 5.073 -5.037 6.163 -6.124m2 -2c.872 -.872 2.191 -2.205 3.959 -4" /> <path d="M7 13h6" /> <path d="M19 15l1.5 1.6m-.74 3.173a2 2 0 0 1 -2.612 -2.608" /> <path d="M15 3l6 6" /> <path d="M3 3l18 18" /> </svg>"##;
const TEXT_SCAN_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12h8" /> <path d="M8 9h6" /> <path d="M8 15h4" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const TEXT_SCAN_AI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12h4.5" /> <path d="M8 8h6" /> <path d="M8 16h2" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M14 21v-4a2 2 0 1 1 4 0v4" /> <path d="M14 19h4" /> <path d="M21 15v6" /> </svg>"##;
const TEXTURE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 3l-3 3" /> <path d="M21 18l-3 3" /> <path d="M11 3l-8 8" /> <path d="M16 3l-13 13" /> <path d="M21 3l-18 18" /> <path d="M21 8l-13 13" /> <path d="M21 13l-8 8" /> </svg>"##;
const THERMOMETER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5a2.828 2.828 0 0 1 0 4l-8 8h-4v-4l8 -8a2.828 2.828 0 0 1 4 0" /> <path d="M16 7l-1.5 -1.5" /> <path d="M13 10l-1.5 -1.5" /> <path d="M10 13l-1.5 -1.5" /> <path d="M7 17l-3 3" /> </svg>"##;
const THUMB_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 13v-8a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v7a1 1 0 0 0 1 1h3a4 4 0 0 1 4 4v1a2 2 0 0 0 4 0v-5h3a2 2 0 0 0 2 -2l-1 -5a2 3 0 0 0 -2 -2h-7a3 3 0 0 0 -3 3" /> </svg>"##;
const THUMB_DOWN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 13v-6m-3 -3a1 1 0 0 0 -1 1v7a1 1 0 0 0 1 1h3a4 4 0 0 1 4 4v1a2 2 0 1 0 4 0v-3m2 -2h1a2 2 0 0 0 2 -2l-1 -5c-.295 -1.26 -1.11 -2.076 -2 -2h-7c-.57 0 -1.102 .159 -1.556 .434" /> <path d="M3 3l18 18" /> </svg>"##;
const THUMB_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 11v8a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-7a1 1 0 0 1 1 -1h3a4 4 0 0 0 4 -4v-1a2 2 0 0 1 4 0v5h3a2 2 0 0 1 2 2l-1 5a2 3 0 0 1 -2 2h-7a3 3 0 0 1 -3 -3" /> </svg>"##;
const THUMB_UP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 11v8a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-7a1 1 0 0 1 1 -1h3a3.987 3.987 0 0 0 2.828 -1.172m1.172 -2.828v-1a2 2 0 1 1 4 0v5h3a2 2 0 0 1 2 2c-.222 1.112 -.39 1.947 -.5 2.503m-.758 3.244c-.392 .823 -1.044 1.312 -1.742 1.253h-7a3 3 0 0 1 -3 -3" /> <path d="M3 3l18 18" /> </svg>"##;
const TILDE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12c0 -1.657 1.592 -3 3.556 -3c1.963 0 3.11 1.5 4.444 3c1.333 1.5 2.48 3 4.444 3s3.556 -1.343 3.556 -3" /> </svg>"##;
const TIME_DURATION_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12v.01" /> <path d="M21 12v.01" /> <path d="M12 21v.01" /> <path d="M12 3v.01" /> <path d="M7.5 4.2v.01" /> <path d="M16.5 4.2v.01" /> <path d="M16.5 19.8v.01" /> <path d="M7.5 19.8v.01" /> <path d="M4.2 16.5v.01" /> <path d="M19.8 16.5v.01" /> <path d="M19.8 7.5v.01" /> <path d="M4.2 7.5v.01" /> <path d="M10 11v2a2 2 0 1 0 4 0v-2a2 2 0 1 0 -4 0" /> </svg>"##;
const TIME_DURATION_10_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 9v6" /> <path d="M12 11v2a2 2 0 1 0 4 0v-2a2 2 0 1 0 -4 0" /> <path d="M3 12v.01" /> <path d="M21 12v.01" /> <path d="M12 21v.01" /> <path d="M7.5 4.2v.01" /> <path d="M16.5 19.8v.01" /> <path d="M7.5 19.8v.01" /> <path d="M4.2 16.5v.01" /> <path d="M19.8 16.5v.01" /> <path d="M4.2 7.5v.01" /> <path d="M19.81 7.527a8.994 8.994 0 0 0 -7.81 -4.527" /> </svg>"##;
const TIME_DURATION_15_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 15h2a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2v-3h3" /> <path d="M9 9v6" /> <path d="M3 12v.01" /> <path d="M12 21v.01" /> <path d="M7.5 4.2v.01" /> <path d="M16.5 19.8v.01" /> <path d="M7.5 19.8v.01" /> <path d="M4.2 16.5v.01" /> <path d="M19.8 16.5v.01" /> <path d="M4.2 7.5v.01" /> <path d="M21 12a9 9 0 0 0 -9 -9" /> </svg>"##;
const TIME_DURATION_30_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M8 9h1.5a1.5 1.5 0 0 1 0 3h-.5h.5a1.5 1.5 0 0 1 0 3h-1.5" /> <path d="M3 12v.01" /> <path d="M7.5 4.2v.01" /> <path d="M7.5 19.8v.01" /> <path d="M4.2 16.5v.01" /> <path d="M4.2 7.5v.01" /> <path d="M12 21a9 9 0 0 0 0 -18" /> </svg>"##;
const TIME_DURATION_45_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 15h2a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2v-3h3" /> <path d="M7 9v2a1 1 0 0 0 1 1h1" /> <path d="M10 9v6" /> <path d="M7.5 4.2v.01" /> <path d="M4.2 7.5v.01" /> <path d="M3 12a9 9 0 1 0 9 -9" /> </svg>"##;
const TIME_DURATION_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15h2a1.5 1.5 0 0 0 0 -3h-2v-3h3.5" /> <path d="M3 12v.01" /> <path d="M21 12v.01" /> <path d="M12 21v.01" /> <path d="M7.5 4.2v.01" /> <path d="M16.5 19.8v.01" /> <path d="M7.5 19.8v.01" /> <path d="M4.2 16.5v.01" /> <path d="M19.8 16.5v.01" /> <path d="M19.8 7.5v.01" /> <path d="M4.2 7.5v.01" /> <path d="M16.5 4.206a9.042 9.042 0 0 0 -4.5 -1.206" /> </svg>"##;
const TIME_DURATION_60_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M11 9h-2a1 1 0 0 0 -1 1v4a1 1 0 0 0 1 1h1a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const TIME_DURATION_90_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 14.25c0 .414 .336 .75 .75 .75h1.5a.75 .75 0 0 0 .75 -.75v-4.5a.75 .75 0 0 0 -.75 -.75h-1.5a.75 .75 0 0 0 -.75 .75v1.5c0 .414 .336 .75 .75 .75h2.25" /> <path d="M14 10.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const TIME_DURATION_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12v.01" /> <path d="M7.5 19.8v.01" /> <path d="M4.2 16.5v.01" /> <path d="M4.2 7.5v.01" /> <path d="M12 21a8.994 8.994 0 0 0 6.362 -2.634m1.685 -2.336a9 9 0 0 0 -8.047 -13.03" /> <path d="M3 3l18 18" /> </svg>"##;
const TIMELINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 16l6 -7l5 5l5 -6" /> <path d="M14 14a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M9 9a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3 16a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M19 8a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const TIMELINE_EVENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 20h-6" /> <path d="M14 20h6" /> <path d="M12 15l-2 -2h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h10a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-3l-2 2" /> </svg>"##;
const TIMELINE_EVENT_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 20h-6" /> <path d="M14 20h6" /> <path d="M12 15l-2 -2h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h10a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-3l-2 2" /> <path d="M12 6v2" /> <path d="M12 11v.01" /> </svg>"##;
const TIMELINE_EVENT_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 20h-6" /> <path d="M14 20h6" /> <path d="M12 15l-2 -2h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h10a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-3l-2 2" /> <path d="M10 8h4" /> </svg>"##;
const TIMELINE_EVENT_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 20h-6" /> <path d="M14 20h6" /> <path d="M12 15l-2 -2h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h10a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-3l-2 2" /> <path d="M10 8h4" /> <path d="M12 6v4" /> </svg>"##;
const TIMELINE_EVENT_TEXT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 20h-6" /> <path d="M14 20h6" /> <path d="M12 15l-2 -2h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h10a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-3l-2 2" /> <path d="M9 6h6" /> <path d="M9 9h3" /> </svg>"##;
const TIMELINE_EVENT_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 20h-6" /> <path d="M14 20h6" /> <path d="M12 15l-2 -2h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h10a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-3l-2 2" /> <path d="M13.5 9.5l-3 -3" /> <path d="M10.5 9.5l3 -3" /> </svg>"##;
const TIMEZONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.884 10.554a9 9 0 1 0 -10.337 10.328" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h6.9" /> <path d="M11.5 3a17 17 0 0 0 -1.502 14.954" /> <path d="M12.5 3a17 17 0 0 1 2.52 7.603" /> <path d="M14 18a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M18 16.5v1.5l.5 .5" /> </svg>"##;
const TIP_JAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M12 9v1" /> <path d="M12 16v1" /> <path d="M17 4v1.882c0 .685 .387 1.312 1 1.618s1 .933 1 1.618v8.882a3 3 0 0 1 -3 3h-8a3 3 0 0 1 -3 -3v-8.882c0 -.685 .387 -1.312 1 -1.618s1 -.933 1 -1.618v-1.882" /> <path d="M6 4h12l-12 0" /> </svg>"##;
const TIP_JAR_EURO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 4v1.882c0 .685 .387 1.312 1 1.618s1 .933 1 1.618v8.882a3 3 0 0 1 -3 3h-8a3 3 0 0 1 -3 -3v-8.882c0 -.685 .387 -1.312 1 -1.618s1 -.933 1 -1.618v-1.882" /> <path d="M6 4h12l-12 0" /> <path d="M12 13h-3" /> <path d="M14 10.172a3 3 0 1 0 0 5.656" /> </svg>"##;
const TIP_JAR_POUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 4v1.882c0 .685 .387 1.312 1 1.618s1 .933 1 1.618v8.882a3 3 0 0 1 -3 3h-8a3 3 0 0 1 -3 -3v-8.882c0 -.685 .387 -1.312 1 -1.618s1 -.933 1 -1.618v-1.882" /> <path d="M6 4h12l-12 0" /> <path d="M14 10h-1a2 2 0 0 0 -2 2v2c0 1.105 -.395 2 -1.5 2h4.5" /> <path d="M10 13h3" /> </svg>"##;
const TOGGLE_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M2 12a6 6 0 0 1 6 -6h8a6 6 0 0 1 6 6a6 6 0 0 1 -6 6h-8a6 6 0 0 1 -6 -6" /> </svg>"##;
const TOGGLE_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M2 12a6 6 0 0 1 6 -6h8a6 6 0 0 1 6 6a6 6 0 0 1 -6 6h-8a6 6 0 0 1 -6 -6" /> </svg>"##;
const TOILET_PAPER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10a3 7 0 1 0 6 0a3 7 0 1 0 -6 0" /> <path d="M21 10c0 -3.866 -1.343 -7 -3 -7" /> <path d="M6 3h12" /> <path d="M21 10v10l-3 -1l-3 2l-3 -3l-3 2v-10" /> <path d="M6 10h.01" /> </svg>"##;
const TOILET_PAPER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.27 4.28c-.768 1.27 -1.27 3.359 -1.27 5.72c0 3.866 1.343 7 3 7s3 -3.134 3 -7c0 -.34 -.01 -.672 -.03 -1" /> <path d="M21 10c0 -3.866 -1.343 -7 -3 -7" /> <path d="M7 3h11" /> <path d="M21 10v7m-1.513 2.496l-1.487 -.496l-3 2l-3 -3l-3 2v-10" /> <path d="M6 10h.01" /> <path d="M3 3l18 18" /> </svg>"##;
const TOOL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 10h3v-3l-3.5 -3.5a6 6 0 0 1 8 8l6 6a2 2 0 0 1 -3 3l-6 -6a6 6 0 0 1 -8 -8l3.5 3.5" /> </svg>"##;
const TOOLTIP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M12 13l-1.707 -1.707a1 1 0 0 0 -.707 -.293h-2.586a2 2 0 0 1 -2 -2v-3a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-2.586a1 1 0 0 0 -.707 .293l-1.707 1.707" /> </svg>"##;
const TORII_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4c5.333 1.333 10.667 1.333 16 0" /> <path d="M4 8h16" /> <path d="M12 5v3" /> <path d="M18 4.5v15.5" /> <path d="M6 4.5v15.5" /> </svg>"##;
const TOWER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 3h1a1 1 0 0 1 1 1v2h3v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2h3v-2a1 1 0 0 1 1 -1h1a1 1 0 0 1 1 1v4.394a2 2 0 0 1 -.336 1.11l-1.328 1.992a2 2 0 0 0 -.336 1.11v7.394a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1v-7.394a2 2 0 0 0 -.336 -1.11l-1.328 -1.992a2 2 0 0 1 -.336 -1.11v-4.394a1 1 0 0 1 1 -1" /> <path d="M10 21v-5a2 2 0 1 1 4 0v5" /> </svg>"##;
const TOWER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 6v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2h3v-2a1 1 0 0 1 1 -1h1a1 1 0 0 1 1 1v4.394a2 2 0 0 1 -.336 1.11l-1.328 1.992a2 2 0 0 0 -.336 1.11v1.394m0 4v2a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1v-7.394a2 2 0 0 0 -.336 -1.11l-1.328 -1.992a2 2 0 0 1 -.336 -1.11v-4.394" /> <path d="M10 21v-5a2 2 0 1 1 4 0v5" /> <path d="M3 3l18 18" /> </svg>"##;
const TRADEMARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.5 9h5m-2.5 0v6" /> <path d="M13 15v-6l3 4l3 -4v6" /> </svg>"##;
const TRANSFER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 10h-16l5.5 -6" /> <path d="M4 14h16l-5.5 6" /> </svg>"##;
const TRANSFER_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 4v16l-6 -5.5" /> <path d="M14 20v-16l6 5.5" /> </svg>"##;
const TRANSFORM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M21 11v-3a2 2 0 0 0 -2 -2h-6l3 3m0 -6l-3 3" /> <path d="M3 13v3a2 2 0 0 0 2 2h6l-3 -3m0 6l3 -3" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> </svg>"##;
const TRANSFORM_POINT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M3 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M11 5h2" /> <path d="M5 11v2" /> <path d="M19 11v2" /> <path d="M11 19h2" /> </svg>"##;
const TRANSFORM_POINT_BOTTOM_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M3 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" fill="currentColor" /> <path d="M17 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M11 5h2" /> <path d="M5 11v2" /> <path d="M19 11v2" /> <path d="M11 19h2" /> </svg>"##;
const TRANSFORM_POINT_BOTTOM_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M3 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" fill="currentColor" /> <path d="M11 5h2" /> <path d="M5 11v2" /> <path d="M19 11v2" /> <path d="M11 19h2" /> </svg>"##;
const TRANSFORM_POINT_TOP_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" fill="currentColor" /> <path d="M3 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M11 5h2" /> <path d="M5 11v2" /> <path d="M19 11v2" /> <path d="M11 19h2" /> </svg>"##;
const TRANSFORM_POINT_TOP_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M3 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" fill="currentColor" /> <path d="M17 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M11 5h2" /> <path d="M5 11v2" /> <path d="M19 11v2" /> <path d="M11 19h2" /> </svg>"##;
const TRASH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7l16 0" /> <path d="M10 11l0 6" /> <path d="M14 11l0 6" /> <path d="M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l1 -12" /> <path d="M9 7v-3a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3" /> </svg>"##;
const TRASH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M4 7h3m4 0h9" /> <path d="M10 11l0 6" /> <path d="M14 14l0 3" /> <path d="M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l.077 -.923" /> <path d="M18.384 14.373l.616 -7.373" /> <path d="M9 5v-1a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3" /> </svg>"##;
const TRASH_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7h16" /> <path d="M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l1 -12" /> <path d="M9 7v-3a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3" /> <path d="M10 12l4 4m0 -4l-4 4" /> </svg>"##;
const TROWEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.42 9.058l-5.362 5.363a1.978 1.978 0 0 1 -3.275 -.773l-2.682 -8.044a1.978 1.978 0 0 1 2.502 -2.502l8.045 2.682a1.978 1.978 0 0 1 .773 3.274" /> <path d="M10 10l6.5 6.5" /> <path d="M19.347 16.575l1.08 1.079a1.96 1.96 0 0 1 -2.773 2.772l-1.08 -1.079a1.96 1.96 0 0 1 2.773 -2.772" /> </svg>"##;
const UFO_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.95 9.01c3.02 .739 5.05 2.123 5.05 3.714c0 1.08 -.931 2.063 -2.468 2.814m-3 1c-1.36 .295 -2.9 .462 -4.531 .462c-5.52 0 -10 -1.909 -10 -4.276c0 -1.59 2.04 -2.985 5.07 -3.724" /> <path d="M14.69 10.686c1.388 -.355 2.31 -.976 2.31 -1.686v-.035c0 -2.742 -2.239 -4.965 -5 -4.965c-1.125 0 -2.164 .37 -3 .992m-1.707 2.297a4.925 4.925 0 0 0 -.293 1.676v.035c0 .961 1.696 1.764 3.956 1.956" /> <path d="M15 17l2 3" /> <path d="M8.5 17l-1.5 3" /> <path d="M12 14h.01" /> <path d="M7 13h.01" /> <path d="M17 13h.01" /> <path d="M3 3l18 18" /> </svg>"##;
const UHD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-8" /> <path d="M10 12h4" /> <path d="M14 8v8" /> <path d="M17 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2l-2 0" /> <path d="M3 8v6a2 2 0 1 0 4 0v-6" /> </svg>"##;
const UMBRELLA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12a8 8 0 0 1 16 0l-16 0" /> <path d="M12 12v6a2 2 0 0 0 4 0" /> </svg>"##;
const UMBRELLA_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.343 7.343a8 8 0 1 1 11.314 11.314l-11.314 -11.314" /> <path d="M10.828 13.34l-4.242 4.243a2 2 0 1 0 2.828 2.828" /> </svg>"##;
const UMBRELLA_CLOSED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 16l3 -13l3 13l-6 0" /> <path d="M12 16v3c0 2.667 4 2.667 4 0" /> </svg>"##;
const UMBRELLA_CLOSED_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.697 12.071l11.313 -7.071l-7.07 11.314l-4.243 -4.243" /> <path d="M8.743 14.475l-2.121 2.121c-1.886 1.886 .943 4.715 2.828 2.829" /> </svg>"##;
const UMBRELLA_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12h-8c0 -2.209 .895 -4.208 2.342 -5.656m2.382 -1.645a8 8 0 0 1 11.276 7.301l-4 0" /> <path d="M12 12v6a2 2 0 1 0 4 0" /> <path d="M3 3l18 18" /> </svg>"##;
const UNIVERSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.027 11.477a5 5 0 1 0 5.496 -4.45a4.951 4.951 0 0 0 -3.088 .681" /> <path d="M5.636 5.636a9 9 0 1 0 3.555 -2.188" /> <path d="M17 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M8 16a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const URGENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16v-4a4 4 0 0 1 8 0v4" /> <path d="M3 12h1m8 -9v1m8 8h1m-15.4 -6.4l.7 .7m12.1 -.7l-.7 .7" /> <path d="M6 17a1 1 0 0 1 1 -1h10a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1l0 -2" /> </svg>"##;
const USB_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M12 17v-11.5" /> <path d="M7 10v3l5 3" /> <path d="M12 14.5l5 -2v-2.5" /> <path d="M16 10h2v-2h-2l0 2" /> <path d="M6 9a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M10 5.5h4l-2 -2.5l-2 2.5" /> </svg>"##;
const USER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v2" /> </svg>"##;
const USER_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 21v-6m2 0v-1.5m0 9v-1.5m-2 -3h3m-1 0h.5a1.5 1.5 0 0 1 0 3h-3.5m3 -3h.5a1.5 1.5 0 0 0 0 -3h-3.5" /> <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h3" /> </svg>"##;
const USER_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h4c.267 0 .529 .026 .781 .076" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const USER_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h3.5" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const USER_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h4" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const USER_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 10a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M6.168 18.849a4 4 0 0 1 3.832 -2.849h4a4 4 0 0 1 3.834 2.855" /> </svg>"##;
const USER_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h3.5" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const USER_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h2.5" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const USER_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h3" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const USER_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h4c.342 0 .674 .043 .99 .124" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const USER_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h3.5" /> <path d="M18.42 15.61a2.1 2.1 0 0 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const USER_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h4c.348 0 .686 .045 1.008 .128" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const USER_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h.5" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const USER_HEXAGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 13a3 3 0 1 0 0 -6a3 3 0 0 0 0 6" /> <path d="M6.201 18.744a4 4 0 0 1 3.799 -2.744h4a4 4 0 0 1 3.798 2.741" /> <path d="M19.875 6.27c.7 .398 1.13 1.143 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> </svg>"##;
const USER_KEY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h5" /> <path d="M18.5 18.5l-3.5 3.5l-1.5 -1.5" /> <path d="M18.554 18.414a2 2 0 1 1 2.828 -2.828a2 2 0 0 1 -2.828 2.828" /> <path d="M16 19l1 1" /> </svg>"##;
const USER_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h4c.348 0 .686 .045 1.009 .128" /> <path d="M16 19h6" /> </svg>"##;
const USER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.18 8.189a4.01 4.01 0 0 0 2.616 2.627m3.507 -.545a4 4 0 1 0 -5.59 -5.552" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h4c.412 0 .81 .062 1.183 .178m2.633 2.618c.12 .38 .184 .785 .184 1.204v2" /> <path d="M3 3l18 18" /> </svg>"##;
const USER_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h3.5" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const USER_PENTAGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> <path d="M12 13a3 3 0 1 0 0 -6a3 3 0 0 0 0 6" /> <path d="M6 20.703v-.703a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v.707" /> </svg>"##;
const USER_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h2.5" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const USER_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M16 19h6" /> <path d="M19 16v6" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h4" /> </svg>"##;
const USER_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h3.5" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const USER_SCAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 9a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M8 16a2 2 0 0 1 2 -2h4a2 2 0 0 1 2 2" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const USER_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h1.5" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const USER_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h3" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const USER_SHIELD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 21v-2a4 4 0 0 1 4 -4h2" /> <path d="M22 16c0 4 -2.5 6 -3.5 6s-3.5 -2 -3.5 -6c1 0 2.5 -.5 3.5 -1.5c1 1 2.5 1.5 3.5 1.5" /> <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> </svg>"##;
const USER_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 10a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M6 21v-1a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v1" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const USER_SQUARE_ROUNDED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 13a3 3 0 1 0 0 -6a3 3 0 0 0 0 6" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> <path d="M6 20.05v-.05a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v.05" /> </svg>"##;
const USER_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h.5" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const USER_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h4" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const USER_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M6 21v-2a4 4 0 0 1 4 -4h3.5" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const USERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M3 21v-2a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v2" /> <path d="M16 3.13a4 4 0 0 1 0 7.75" /> <path d="M21 21v-2a4 4 0 0 0 -3 -3.85" /> </svg>"##;
const USERS_GROUP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 13a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M8 21v-1a2 2 0 0 1 2 -2h4a2 2 0 0 1 2 2v1" /> <path d="M15 5a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M17 10h2a2 2 0 0 1 2 2v1" /> <path d="M5 5a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M3 13v-1a2 2 0 0 1 2 -2h2" /> </svg>"##;
const USERS_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M3 21v-2a4 4 0 0 1 4 -4h4c.948 0 1.818 .33 2.504 .88" /> <path d="M16 3.13a4 4 0 0 1 0 7.75" /> <path d="M16 19h6" /> </svg>"##;
const USERS_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M3 21v-2a4 4 0 0 1 4 -4h4c.96 0 1.84 .338 2.53 .901" /> <path d="M16 3.13a4 4 0 0 1 0 7.75" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const VACCINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 3l4 4" /> <path d="M19 5l-4.5 4.5" /> <path d="M11.5 6.5l6 6" /> <path d="M16.5 11.5l-6.5 6.5h-4v-4l6.5 -6.5" /> <path d="M7.5 12.5l1.5 1.5" /> <path d="M10.5 9.5l1.5 1.5" /> <path d="M3 21l3 -3" /> </svg>"##;
const VACCINE_BOTTLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 4a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -1" /> <path d="M10 6v.98c0 .877 -.634 1.626 -1.5 1.77c-.866 .144 -1.5 .893 -1.5 1.77v8.48a2 2 0 0 0 2 2h6a2 2 0 0 0 2 -2v-8.48c0 -.877 -.634 -1.626 -1.5 -1.77a1.795 1.795 0 0 1 -1.5 -1.77v-.98" /> <path d="M7 12h10" /> <path d="M7 18h10" /> <path d="M11 15h2" /> </svg>"##;
const VACCINE_BOTTLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5v-1a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-4" /> <path d="M8.7 8.705a1.806 1.806 0 0 1 -.2 .045c-.866 .144 -1.5 .893 -1.5 1.77v8.48a2 2 0 0 0 2 2h6a2 2 0 0 0 2 -2v-2m0 -4v-2.48c0 -.877 -.634 -1.626 -1.5 -1.77a1.795 1.795 0 0 1 -1.5 -1.77v-.98" /> <path d="M7 12h5m4 0h1" /> <path d="M7 18h10" /> <path d="M11 15h2" /> <path d="M3 3l18 18" /> </svg>"##;
const VACCINE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 3l4 4" /> <path d="M19 5l-4.5 4.5" /> <path d="M11.5 6.5l6 6" /> <path d="M16.5 11.5l-.5 .5m-2 2l-4 4h-4v-4l4 -4m2 -2l.5 -.5" /> <path d="M7.5 12.5l1.5 1.5" /> <path d="M3 21l3 -3" /> <path d="M3 3l18 18" /> </svg>"##;
const VACUUM_CLEANER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 1 -18 0a9 9 0 0 1 18 0" /> <path d="M14 9a2 2 0 1 1 -4 0a2 2 0 0 1 4 0" /> <path d="M12 16h.01" /> </svg>"##;
const VARIABLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4c-2.5 5 -2.5 10 0 16m14 -16c2.5 5 2.5 10 0 16m-10 -11h1c1 0 1 1 2.016 3.527c.984 2.473 .984 3.473 1.984 3.473h1" /> <path d="M8 16c1.5 0 3 -2 4 -3.5s2.5 -3.5 4 -3.5" /> </svg>"##;
const VARIABLE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16c1.5 0 3 -2 4 -3.5s2.5 -3.5 4 -3.5" /> <path d="M5 4c-2.5 5 -2.5 10 0 16m14 -16c1.775 3.55 2.29 7.102 1.544 11.01m-11.544 -6.01h1c1 0 1 1 2.016 3.527c.782 1.966 .943 3 1.478 3.343" /> <path d="M8 16c1.5 0 3 -2 4 -3.5s2.5 -3.5 4 -3.5" /> <path d="M16 19h6" /> </svg>"##;
const VARIABLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.675 4.68c-2.17 4.776 -2.062 9.592 .325 15.32" /> <path d="M19 4c1.959 3.917 2.383 7.834 1.272 12.232m-.983 3.051c-.093 .238 -.189 .477 -.289 .717" /> <path d="M11.696 11.696c.095 .257 .2 .533 .32 .831c.984 2.473 .984 3.473 1.984 3.473h1" /> <path d="M8 16c1.5 0 3 -2 4 -3.5m2.022 -2.514c.629 -.582 1.304 -.986 1.978 -.986" /> <path d="M3 3l18 18" /> </svg>"##;
const VARIABLE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4c-2.5 5 -2.5 10 0 16m14 -16c1.38 2.76 2 5.52 1.855 8.448m-11.855 -3.448h1c1 0 1 1 2.016 3.527c.785 1.972 .944 3.008 1.483 3.346" /> <path d="M8 16c1.5 0 3 -2 4 -3.5s2.5 -3.5 4 -3.5" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const VENUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M12 14l0 7" /> <path d="M9 18l6 0" /> </svg>"##;
const VERSIONS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 7a2 2 0 0 1 2 -2h6a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2l0 -10" /> <path d="M7 7l0 10" /> <path d="M4 8l0 8" /> </svg>"##;
const VERSIONS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.184 6.162a2 2 0 0 1 1.816 -1.162h6a2 2 0 0 1 2 2v9m-1.185 2.827a1.993 1.993 0 0 1 -.815 .173h-6a2 2 0 0 1 -2 -2v-7" /> <path d="M7 7v10" /> <path d="M4 8v8" /> <path d="M3 3l18 18" /> </svg>"##;
const VIEW_360_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M8 12a4 9 0 1 0 8 0a4 9 0 1 0 -8 0" /> <path d="M3 12c0 2.21 4.03 4 9 4s9 -1.79 9 -4s-4.03 -4 -9 -4s-9 1.79 -9 4" /> </svg>"##;
const VIEW_360_ARROW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 15.328c2.414 -.718 4 -1.94 4 -3.328c0 -2.21 -4.03 -4 -9 -4s-9 1.79 -9 4s4.03 4 9 4" /> <path d="M9 13l3 3l-3 3" /> </svg>"##;
const VIEW_360_NUMBER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 6a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> <path d="M3 5h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> <path d="M17 7v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> <path d="M3 16c0 1.657 4.03 3 9 3s9 -1.343 9 -3" /> </svg>"##;
const VIEW_360_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.335 8.388a19 19 0 0 0 -.335 3.612c0 4.97 1.79 9 4 9c1.622 0 3.018 -2.172 3.646 -5.294m.354 -3.706c0 -4.97 -1.79 -9 -4 -9c-1.035 0 -1.979 .885 -2.689 2.337" /> <path d="M5.65 5.623a9 9 0 1 0 12.71 12.745m1.684 -2.328a9 9 0 0 0 -12.094 -12.08" /> <path d="M8.32 8.349c-3.136 .625 -5.32 2.025 -5.32 3.651c0 2.21 4.03 4 9 4c1.286 0 2.51 -.12 3.616 -.336m3.059 -.98c1.445 -.711 2.325 -1.653 2.325 -2.684c0 -2.21 -4.03 -4 -9 -4" /> <path d="M3 3l18 18" /> </svg>"##;
const VIEWPORT_SHORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3v7l3 -3" /> <path d="M9 7l3 3" /> <path d="M12 21v-7l3 3" /> <path d="M9 17l3 -3" /> <path d="M18 9h1a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-1" /> <path d="M6 9h-1a2 2 0 0 0 -2 2v2a2 2 0 0 0 2 2h1" /> </svg>"##;
const VIEWPORT_TALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 10v-7l3 3" /> <path d="M9 6l3 -3" /> <path d="M12 14v7l3 -3" /> <path d="M9 18l3 3" /> <path d="M18 3h1a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-1" /> <path d="M6 3h-1a2 2 0 0 0 -2 2v14a2 2 0 0 0 2 2h1" /> </svg>"##;
const VIP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5h18" /> <path d="M3 19h18" /> <path d="M4 9l2 6h1l2 -6" /> <path d="M12 9v6" /> <path d="M16 15v-6h2a2 2 0 1 1 0 4h-2" /> </svg>"##;
const VIP_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5h4" /> <path d="M17 5h4" /> <path d="M3 19h18" /> <path d="M4 9l2 6h1l2 -6" /> <path d="M12 10v5" /> <path d="M16 15v-6h2a2 2 0 1 1 0 4h-2" /> <path d="M10 5a2 2 0 0 1 2 2a2 2 0 0 1 2 -2a2 2 0 0 1 -2 -2a2 2 0 0 1 -2 2" /> </svg>"##;
const VIP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5h2m4 0h12" /> <path d="M3 19h16" /> <path d="M4 9l2 6h1l2 -6" /> <path d="M12 12v3" /> <path d="M16 12v-3h2a2 2 0 1 1 0 4h-1" /> <path d="M3 3l18 18" /> </svg>"##;
const VIRUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 12a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M12 7v-4" /> <path d="M11 3h2" /> <path d="M15.536 8.464l2.828 -2.828" /> <path d="M17.657 4.929l1.414 1.414" /> <path d="M17 12h4" /> <path d="M21 11v2" /> <path d="M15.535 15.536l2.829 2.828" /> <path d="M19.071 17.657l-1.414 1.414" /> <path d="M12 17v4" /> <path d="M13 21h-2" /> <path d="M8.465 15.536l-2.829 2.828" /> <path d="M6.343 19.071l-1.413 -1.414" /> <path d="M7 12h-4" /> <path d="M3 13v-2" /> <path d="M8.464 8.464l-2.828 -2.828" /> <path d="M4.929 6.343l1.414 -1.413" /> </svg>"##;
const VIRUS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M8.469 8.46a5 5 0 0 0 7.058 7.084" /> <path d="M16.913 12.936a5 5 0 0 0 -5.826 -5.853" /> <path d="M12 7v-4" /> <path d="M11 3h2" /> <path d="M15.536 8.464l2.828 -2.828" /> <path d="M17.657 4.929l1.414 1.414" /> <path d="M17 12h4" /> <path d="M21 11v2" /> <path d="M18.364 18.363l-.707 .707" /> <path d="M12 17v4" /> <path d="M13 21h-2" /> <path d="M8.465 15.536l-2.829 2.828" /> <path d="M6.343 19.071l-1.413 -1.414" /> <path d="M7 12h-4" /> <path d="M3 13v-2" /> <path d="M5.636 5.637l-.707 .707" /> </svg>"##;
const VIRUS_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 12a5 5 0 1 0 -5 5" /> <path d="M12 7v-4" /> <path d="M11 3h2" /> <path d="M15.536 8.464l2.828 -2.828" /> <path d="M17.657 4.929l1.414 1.414" /> <path d="M17 12h4" /> <path d="M21 11v2" /> <path d="M12 17v4" /> <path d="M13 21h-2" /> <path d="M8.465 15.536l-2.829 2.828" /> <path d="M6.343 19.071l-1.413 -1.414" /> <path d="M7 12h-4" /> <path d="M3 13v-2" /> <path d="M8.464 8.464l-2.828 -2.828" /> <path d="M4.929 6.343l1.414 -1.413" /> <path d="M15 17.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> <path d="M19.5 19.5l2.5 2.5" /> </svg>"##;
const VS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 12c0 5.523 4.477 10 10 10s10 -4.477 10 -10s-4.477 -10 -10 -10s-10 4.477 -10 10" /> <path d="M14 14.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1.25a.75 .75 0 0 1 .75 .75" /> <path d="M7 9l2 6l2 -6" /> </svg>"##;
const WALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M4 8h16" /> <path d="M20 12h-16" /> <path d="M4 16h16" /> <path d="M9 4v4" /> <path d="M14 8v4" /> <path d="M8 12v4" /> <path d="M16 12v4" /> <path d="M11 16v4" /> </svg>"##;
const WALL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h10a2 2 0 0 1 2 2v10m-.589 3.417c-.361 .36 -.86 .583 -1.411 .583h-12a2 2 0 0 1 -2 -2v-12c0 -.55 .222 -1.047 .58 -1.409" /> <path d="M4 8h4m4 0h8" /> <path d="M20 12h-4m-4 0h-8" /> <path d="M4 16h12" /> <path d="M9 4v1" /> <path d="M14 8v2" /> <path d="M8 12v4" /> <path d="M11 16v4" /> <path d="M3 3l18 18" /> </svg>"##;
const WALLPAPER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 6h10a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-12" /> <path d="M4 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M8 18v-12a2 2 0 1 0 -4 0v12" /> </svg>"##;
const WALLPAPER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 6h8a2 2 0 0 1 2 2v8m-.58 3.409a2 2 0 0 1 -1.42 .591h-12" /> <path d="M4 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M8 18v-10m-3.427 -3.402c-.353 .362 -.573 .856 -.573 1.402v12" /> <path d="M3 3l18 18" /> </svg>"##;
const WAND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 21l15 -15l-3 -3l-15 15l3 3" /> <path d="M15 6l3 3" /> <path d="M9 3a2 2 0 0 0 2 2a2 2 0 0 0 -2 2a2 2 0 0 0 -2 -2a2 2 0 0 0 2 -2" /> <path d="M19 13a2 2 0 0 0 2 2a2 2 0 0 0 -2 2a2 2 0 0 0 -2 -2a2 2 0 0 0 2 -2" /> </svg>"##;
const WAND_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.5 10.5l-7.5 7.5l3 3l7.5 -7.5m2 -2l5.5 -5.5l-3 -3l-5.5 5.5" /> <path d="M15 6l3 3" /> <path d="M8.433 4.395c.35 -.36 .567 -.852 .567 -1.395a2 2 0 0 0 2 2c-.554 0 -1.055 .225 -1.417 .589" /> <path d="M18.418 14.41c.36 -.36 .582 -.86 .582 -1.41a2 2 0 0 0 2 2c-.555 0 -1.056 .226 -1.419 .59" /> <path d="M3 3l18 18" /> </svg>"##;
const WAVE_SAW_TOOL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h5l4 8v-16l4 8h5" /> </svg>"##;
const WAVE_SINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12h-2c-.894 0 -1.662 -.857 -1.761 -2c-.296 -3.45 -.749 -6 -2.749 -6s-2.5 3.582 -2.5 8s-.5 8 -2.5 8s-2.452 -2.547 -2.749 -6c-.1 -1.147 -.867 -2 -1.763 -2h-2" /> </svg>"##;
const WAVE_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h5v8h4v-16h4v8h5" /> </svg>"##;
const WAVES_ELECTRICITY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12c.576 -.643 1.512 -1.017 2.5 -1c.988 -.017 1.924 .357 2.5 1c.576 .643 1.512 1.017 2.5 1c.988 .017 1.924 -.357 2.5 -1" /> <path d="M3 16c.576 -.643 1.512 -1.017 2.5 -1c.988 -.017 1.924 .357 2.5 1c.576 .643 1.512 1.017 2.5 1c.988 .017 1.924 -.357 2.5 -1" /> <path d="M3 8c.576 -.643 1.512 -1.017 2.5 -1c.988 -.017 1.924 .357 2.5 1c.576 .643 1.512 1.017 2.5 1c.988 .017 1.924 -.357 2.5 -1" /> <path d="M20 7l-3 5h4l-3 5" /> </svg>"##;
const WEBHOOK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.876 13.61a4 4 0 1 0 6.124 3.39h6" /> <path d="M15.066 20.502a4 4 0 1 0 1.934 -7.502c-.706 0 -1.424 .179 -2 .5l-3 -5.5" /> <path d="M16 8a4 4 0 1 0 -8 0c0 1.506 .77 2.818 2 3.5l-3 5.5" /> </svg>"##;
const WEBHOOK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.876 13.61a4 4 0 1 0 6.124 3.39h6" /> <path d="M15.066 20.502a4 4 0 0 0 4.763 -.675m1.171 -2.827a4 4 0 0 0 -4 -4" /> <path d="M16 8a4 4 0 0 0 -6.824 -2.833m-1.176 2.833c0 1.506 .77 2.818 2 3.5l-3 5.5" /> <path d="M3 3l18 18" /> </svg>"##;
const WEIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 6a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M6.835 9h10.33a1 1 0 0 1 .984 .821l1.637 9a1 1 0 0 1 -.984 1.179h-13.604a1 1 0 0 1 -.984 -1.179l1.637 -9a1 1 0 0 1 .984 -.821" /> </svg>"##;
const WINDOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3c-3.866 0 -7 3.272 -7 7v10a1 1 0 0 0 1 1h12a1 1 0 0 0 1 -1v-10c0 -3.728 -3.134 -7 -7 -7" /> <path d="M5 13l14 0" /> <path d="M12 3l0 18" /> </svg>"##;
const WINDOW_MAXIMIZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a1 1 0 0 1 1 -1h3a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-3a1 1 0 0 1 -1 -1l0 -3" /> <path d="M4 12v-6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-6" /> <path d="M12 8h4v4" /> <path d="M16 8l-5 5" /> </svg>"##;
const WINDOW_MINIMIZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a1 1 0 0 1 1 -1h3a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-3a1 1 0 0 1 -1 -1l0 -3" /> <path d="M4 12v-6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-6" /> <path d="M15 13h-4v-4" /> <path d="M11 13l5 -5" /> </svg>"##;
const WINDOW_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.166 6.19a6.903 6.903 0 0 0 -1.166 3.81v10a1 1 0 0 0 1 1h12a1 1 0 0 0 1 -1v-1m0 -4v-5c0 -3.728 -3.134 -7 -7 -7a6.86 6.86 0 0 0 -3.804 1.158" /> <path d="M5 13h8m4 0h2" /> <path d="M12 3v5m0 4v9" /> <path d="M3 3l18 18" /> </svg>"##;
const WOMAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v5" /> <path d="M14 16v5" /> <path d="M8 16h8l-2 -7h-4l-2 7" /> <path d="M5 11c1.667 -1.333 3.333 -2 5 -2" /> <path d="M19 11c-1.667 -1.333 -3.333 -2 -5 -2" /> <path d="M10 4a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const WOOD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 5.5a6 2.5 0 1 0 12 0a6 2.5 0 1 0 -12 0" /> <path d="M18 5.5v4.626a1.415 1.415 0 0 1 1.683 2.18l-.097 .108l-1.586 1.586v4c0 1.61 -2.54 2.925 -5.725 3l-.275 0c-3.314 0 -6 -1.343 -6 -3v-2l-1.586 -1.586a1.414 1.414 0 0 1 1.586 -2.287v-6.627" /> <path d="M10 12.5v1.5" /> <path d="M14 16v1" /> </svg>"##;
const X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 6l-12 12" /> <path d="M6 6l12 12" /> </svg>"##;
const X_MARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 16l3.644 3.644a1.21 1.21 0 0 0 1.712 0l2.288 -2.288a1.21 1.21 0 0 0 0 -1.712l-3.644 -3.644l3.644 -3.644a1.21 1.21 0 0 0 0 -1.712l-2.288 -2.288a1.21 1.21 0 0 0 -1.712 0l-3.644 3.644l-3.644 -3.644a1.21 1.21 0 0 0 -1.712 0l-2.288 2.288a1.21 1.21 0 0 0 0 1.712l3.644 3.644l-3.644 3.644a1.21 1.21 0 0 0 0 1.712l2.288 2.288a1.21 1.21 0 0 0 1.712 0l3.644 -3.644" /> </svg>"##;
const X_POWER_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 3l3 5.063" /> <path d="M5 12l6 6" /> <path d="M5 18l6 -6" /> <path d="M21 3l-4.8 9" /> </svg>"##;
const XD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 8l4 8" /> <path d="M6 16l4 -8" /> <path d="M14 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2h-2" /> </svg>"##;
const XXX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8l4 8" /> <path d="M10 16l4 -8" /> <path d="M17 8l4 8" /> <path d="M17 16l4 -8" /> <path d="M3 8l4 8" /> <path d="M3 16l4 -8" /> </svg>"##;
const YIN_YANG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 3a4.5 4.5 0 0 0 0 9a4.5 4.5 0 0 1 0 9" /> <path d="M11.5 7.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M11.5 16.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> </svg>"##;
const ZERO_CONFIG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12a8 8 0 1 0 16 0a8 8 0 1 0 -16 0" /> <path d="M3 21l18 -18" /> </svg>"##;
const ZOOM_SCAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M16 16l-2.5 -2.5" /> <path d="M3 7v-2a2 2 0 0 1 2 -2h2" /> <path d="M3 17v2a2 2 0 0 0 2 2h2" /> <path d="M17 3h2a2 2 0 0 1 2 2v2" /> <path d="M17 21h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const ZZZ_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12h6l-6 8h6" /> <path d="M14 4h6l-6 8h6" /> </svg>"##;
const ZZZ_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12h6l-6 8h6" /> <path d="M14 4h6l-5.146 6.862m1.146 1.138h4" /> <path d="M3 3l18 18" /> </svg>"##;

/// System icon variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SystemIcon {
    AB,
    Abacus,
    AbacusOff,
    Accessible,
    AccessibleOff,
    Activity,
    ActivityHeartbeat,
    AdCircle,
    AdCircleOff,
    Adjustments,
    AdjustmentsAlt,
    AdjustmentsBolt,
    AdjustmentsCancel,
    AdjustmentsCheck,
    AdjustmentsCode,
    AdjustmentsCog,
    AdjustmentsDollar,
    AdjustmentsDown,
    AdjustmentsExclamation,
    AdjustmentsHeart,
    AdjustmentsHorizontal,
    AdjustmentsMinus,
    AdjustmentsOff,
    AdjustmentsPause,
    AdjustmentsPin,
    AdjustmentsPlus,
    AdjustmentsQuestion,
    AdjustmentsSearch,
    AdjustmentsShare,
    AdjustmentsSpark,
    AdjustmentsStar,
    AdjustmentsUp,
    AdjustmentsX,
    Affiliate,
    Ai,
    AiAgent,
    AiAgents,
    AiGateway,
    Alarm,
    AlarmAverage,
    AlarmMinus,
    AlarmOff,
    AlarmPlus,
    AlarmSnooze,
    AlertCircle,
    AlertCircleOff,
    AlertHexagon,
    AlertHexagonOff,
    AlertOctagon,
    AlertSmall,
    AlertSmallOff,
    AlertSquare,
    AlertSquareRounded,
    AlertSquareRoundedOff,
    AlertTriangle,
    AlertTriangleOff,
    Alt,
    Ampersand,
    Analyze,
    AnalyzeOff,
    Ankh,
    Api,
    ApiApp,
    ApiAppOff,
    ApiBook,
    ApiOff,
    AppWindow,
    Apps,
    AppsOff,
    Armchair,
    Armchair2,
    Armchair2Off,
    ArmchairOff,
    Assembly,
    AssemblyOff,
    Asset,
    AugmentedReality,
    AugmentedReality2,
    AugmentedRealityOff,
    Auth2fa,
    Automation,
    BabyBottle,
    BabyCarriage,
    Backslash,
    Ban,
    Bandage,
    BandageOff,
    Barcode,
    BarcodeOff,
    Bath,
    BathOff,
    BedFlat,
    Bell,
    BellBolt,
    BellCancel,
    BellCheck,
    BellCode,
    BellCog,
    BellDollar,
    BellDown,
    BellExclamation,
    BellHeart,
    BellMinus,
    BellOff,
    BellPause,
    BellPin,
    BellPlus,
    BellQuestion,
    BellRinging,
    BellRinging2,
    BellSchool,
    BellSearch,
    BellShare,
    BellStar,
    BellUp,
    BellX,
    BellZ,
    Biohazard,
    BiohazardOff,
    Blind,
    Blob,
    Blocks,
    BodyScan,
    BotId,
    Braces,
    BracesOff,
    Brackets,
    BracketsContain,
    BracketsContainEnd,
    BracketsContainStart,
    BracketsOff,
    Braille,
    Brain,
    Bug,
    BugOff,
    Building,
    BuildingAirport,
    BuildingArch,
    BuildingBank,
    BuildingBridge,
    BuildingBridge2,
    BuildingBroadcastTower,
    BuildingBurjAlArab,
    BuildingCarousel,
    BuildingCastle,
    BuildingChurch,
    BuildingCircus,
    BuildingCog,
    BuildingCommunity,
    BuildingCottage,
    BuildingEiffelTower,
    BuildingEstate,
    BuildingFactory,
    BuildingFactory2,
    BuildingFortress,
    BuildingHospital,
    BuildingLighthouse,
    BuildingMinus,
    BuildingMonument,
    BuildingMosque,
    BuildingOff,
    BuildingPavilion,
    BuildingPlus,
    BuildingSkyscraper,
    BuildingStadium,
    BuildingStore,
    BuildingTunnel,
    BuildingWarehouse,
    BuildingWindTurbine,
    Buildings,
    Calendar,
    CalendarBolt,
    CalendarCancel,
    CalendarCheck,
    CalendarClock,
    CalendarCode,
    CalendarCog,
    CalendarDollar,
    CalendarDot,
    CalendarDown,
    CalendarDue,
    CalendarEvent,
    CalendarExclamation,
    CalendarHeart,
    CalendarMinus,
    CalendarMonth,
    CalendarOff,
    CalendarPause,
    CalendarPin,
    CalendarPlus,
    CalendarQuestion,
    CalendarRepeat,
    CalendarSad,
    CalendarSearch,
    CalendarShare,
    CalendarSmile,
    CalendarStar,
    CalendarStats,
    CalendarTime,
    CalendarUp,
    CalendarUser,
    CalendarWeek,
    CalendarX,
    Canary,
    Cane,
    CarGarage,
    Ce,
    CeOff,
    ChalkboardTeacher,
    Check,
    Checkbox,
    Checklist,
    Checks,
    CheckupList,
    ClearAll,
    Click,
    Clock,
    Clock12,
    Clock2,
    Clock24,
    ClockBitcoin,
    ClockBolt,
    ClockCancel,
    ClockCheck,
    ClockCode,
    ClockCog,
    ClockDollar,
    ClockDown,
    ClockEdit,
    ClockExclamation,
    ClockHeart,
    ClockHour1,
    ClockHour10,
    ClockHour11,
    ClockHour12,
    ClockHour2,
    ClockHour3,
    ClockHour4,
    ClockHour5,
    ClockHour6,
    ClockHour7,
    ClockHour8,
    ClockHour9,
    ClockMinus,
    ClockOff,
    ClockPause,
    ClockPin,
    ClockPlay,
    ClockPlus,
    ClockQuestion,
    ClockRecord,
    ClockSearch,
    ClockShare,
    ClockShield,
    ClockStar,
    ClockStop,
    ClockUp,
    ClockX,
    CloudBitcoin,
    CloudDownload,
    CloudLock,
    CloudLockOpen,
    CloudNetwork,
    CloudUpload,
    CodeAi,
    CodeVariable,
    CodeVariableMinus,
    CodeVariablePlus,
    Codeblock,
    ColumnInsertLeft,
    ColumnInsertRight,
    ColumnRemove,
    Command,
    CommandOff,
    Confucius,
    CongruentTo,
    Connection,
    Copyleft,
    CopyleftOff,
    Copyright,
    CopyrightOff,
    CreativeCommons,
    CreativeCommonsBy,
    CreativeCommonsNc,
    CreativeCommonsNd,
    CreativeCommonsOff,
    CreativeCommonsSa,
    CreativeCommonsZero,
    Credits,
    Cross,
    CrossOff,
    Crosshair,
    Crutches,
    CrutchesOff,
    Cube3dSphere,
    Cube3dSphereOff,
    CubeSend,
    CubeUnfolded,
    CurlyLoop,
    CursorOff,
    Dashboard,
    DashboardOff,
    Database,
    DatabaseCog,
    DatabaseDollar,
    DatabaseEdit,
    DatabaseExclamation,
    DatabaseExport,
    DatabaseHeart,
    DatabaseImport,
    DatabaseLeak,
    DatabaseMinus,
    DatabaseOff,
    DatabasePlus,
    DatabaseSearch,
    DatabaseShare,
    DatabaseSmile,
    DatabaseStar,
    DatabaseX,
    Deaf,
    Decimal,
    Dental,
    DentalBroken,
    DentalOff,
    Deselect,
    Desk,
    Details,
    DetailsOff,
    DeviceProjector,
    DeviceUnknown,
    Direction,
    DirectionArrows,
    DirectionHorizontal,
    DirectionSign,
    DirectionSignOff,
    Disabled,
    Disabled2,
    DisabledOff,
    Divide,
    Dna,
    Dna2,
    Dna2Off,
    DnaOff,
    Door,
    DoorEnter,
    DoorExit,
    DoorHanger,
    DoorOff,
    Dots,
    DotsCircleHorizontal,
    DotsDiagonal,
    DotsDiagonal2,
    DotsVertical,
    Ear,
    EarOff,
    EarScan,
    Elevator,
    ElevatorOff,
    EmergencyBed,
    Empathize,
    EmpathizeOff,
    Equal,
    EqualDouble,
    EqualNot,
    Exchange,
    ExchangeOff,
    ExclamationCircle,
    ExclamationMark,
    ExclamationMarkOff,
    Explicit,
    ExplicitOff,
    ExternalLink,
    ExternalLinkOff,
    Eye,
    EyeBitcoin,
    EyeBolt,
    EyeCancel,
    EyeCheck,
    EyeClosed,
    EyeCode,
    EyeCog,
    EyeDiscount,
    EyeDollar,
    EyeDotted,
    EyeDown,
    EyeEdit,
    EyeExclamation,
    EyeHeart,
    EyeMinus,
    EyeOff,
    EyePause,
    EyePin,
    EyePlus,
    EyeQuestion,
    EyeSearch,
    EyeShare,
    EyeSpark,
    EyeStar,
    EyeTable,
    EyeUp,
    EyeX,
    Eyeglass,
    Eyeglass2,
    EyeglassOff,
    FaceId,
    FaceIdError,
    FaceMask,
    FaceMaskOff,
    Fall,
    Fence,
    FenceOff,
    FidgetSpinner,
    Filter,
    Filter2,
    Filter2Bolt,
    Filter2Cancel,
    Filter2Check,
    Filter2Code,
    Filter2Cog,
    Filter2Discount,
    Filter2Dollar,
    Filter2Down,
    Filter2Edit,
    Filter2Exclamation,
    Filter2Minus,
    Filter2Pause,
    Filter2Pin,
    Filter2Plus,
    Filter2Question,
    Filter2Search,
    Filter2Share,
    Filter2Spark,
    Filter2Up,
    Filter2X,
    FilterBolt,
    FilterCancel,
    FilterCheck,
    FilterCode,
    FilterCog,
    FilterDiscount,
    FilterDollar,
    FilterDown,
    FilterEdit,
    FilterExclamation,
    FilterHeart,
    FilterMinus,
    FilterOff,
    FilterPause,
    FilterPin,
    FilterPlus,
    FilterQuestion,
    FilterSearch,
    FilterShare,
    FilterSpark,
    FilterStar,
    FilterUp,
    FilterX,
    Filters,
    Fingerprint,
    FingerprintOff,
    FingerprintScan,
    FireExtinguisher,
    FirewallCheck,
    FirewallFlame,
    FirstAidKit,
    FirstAidKitOff,
    FishChristianity,
    FlagBitcoin,
    FlagDiscount,
    Flask,
    Flask2,
    Flask2Off,
    FlaskOff,
    FocusCentered,
    Foodsteps,
    Forbid,
    Forbid2,
    FreeRights,
    Friends,
    FriendsOff,
    Function,
    FunctionOff,
    Galaxy,
    Gauge,
    GaugeOff,
    Gavel,
    Graph,
    GraphOff,
    GridDots,
    GridPattern,
    GridScan,
    GripHorizontal,
    GripVertical,
    Gymnastics,
    HandSanitizer,
    Hanger2,
    HazeMoon,
    HealthRecognition,
    HeartBitcoin,
    HeartBroken,
    HeartDiscount,
    HeartHandshake,
    HeartRateMonitor,
    Heartbeat,
    Help,
    HelpCircle,
    HelpHexagon,
    HelpOctagon,
    HelpOff,
    HelpSmall,
    HelpSquare,
    HelpSquareRounded,
    HelpTriangle,
    History,
    HistoryOff,
    HistoryToggle,
    Home,
    Home2,
    HomeBitcoin,
    HomeBolt,
    HomeCancel,
    HomeCheck,
    HomeCog,
    HomeDollar,
    HomeDot,
    HomeDown,
    HomeEco,
    HomeEdit,
    HomeExclamation,
    HomeHand,
    HomeHeart,
    HomeInfinity,
    HomeLink,
    HomeLock,
    HomeMinus,
    HomeMove,
    HomeOff,
    HomePlus,
    HomeQuestion,
    HomeRibbon,
    HomeSearch,
    HomeShare,
    HomeShield,
    HomeSignal,
    HomeSpark,
    HomeStar,
    HomeStats,
    HomeUp,
    HomeX,
    Hospital,
    HospitalCircle,
    HotelService,
    Hourglass,
    HourglassEmpty,
    HourglassHigh,
    HourglassLow,
    HourglassOff,
    Hours12,
    Hours24,
    Id,
    IdBadge,
    IdBadge2,
    IdBadgeOff,
    IdOff,
    ImageGeneration,
    Inbox,
    InboxOff,
    Infinity,
    Infinity2,
    InfinityOff,
    InfoCircle,
    InfoHexagon,
    InfoOctagon,
    InfoSmall,
    InfoSquare,
    InfoSquareRounded,
    InfoTriangle,
    InputAi,
    InputCheck,
    InputSpark,
    InputX,
    Jetpack,
    JoinBevel,
    JoinRound,
    JoinStraight,
    Label,
    LabelImportant,
    LabelOff,
    Ladder,
    LadderOff,
    Lane,
    LayoutBottombarInactive,
    LayoutNavbarInactive,
    LayoutSidebarInactive,
    LayoutSidebarRightInactive,
    Lego,
    LegoOff,
    Library,
    LibraryMinus,
    LibraryPhoto,
    LibraryPlus,
    Lifebuoy,
    LifebuoyOff,
    Lighter,
    LineScan,
    ListLetters,
    ListTree,
    Loader,
    Loader2,
    Loader3,
    Loader4,
    LoaderQuarter,
    LocationDiscount,
    Lock,
    LockAccess,
    LockAccessOff,
    LockBitcoin,
    LockBolt,
    LockCancel,
    LockCheck,
    LockCode,
    LockCog,
    LockDollar,
    LockDown,
    LockExclamation,
    LockHeart,
    LockMinus,
    LockOff,
    LockOpen,
    LockOpen2,
    LockOpenOff,
    LockPassword,
    LockPause,
    LockPin,
    LockPlus,
    LockQuestion,
    LockSearch,
    LockShare,
    LockSquare,
    LockSquareRounded,
    LockStar,
    LockUp,
    LockX,
    LogicAnd,
    LogicBuffer,
    LogicNand,
    LogicNor,
    LogicNot,
    LogicOr,
    LogicXnor,
    LogicXor,
    Logout,
    Logout2,
    Logs,
    Luggage,
    LuggageOff,
    Lungs,
    LungsOff,
    Man,
    Marquee,
    Marquee2,
    MarqueeOff,
    Mars,
    Massage,
    Matchstick,
    Math,
    Math1Divide2,
    Math1Divide3,
    MathAvg,
    MathCos,
    MathCtg,
    MathEqualGreater,
    MathEqualLower,
    MathFunction,
    MathFunctionOff,
    MathFunctionY,
    MathGreater,
    MathIntegral,
    MathIntegralX,
    MathIntegrals,
    MathLower,
    MathMax,
    MathMaxMin,
    MathMin,
    MathNot,
    MathOff,
    MathPi,
    MathPiDivide2,
    MathSec,
    MathSin,
    MathSymbols,
    MathTg,
    MathXDivide2,
    MathXDivideY,
    MathXDivideY2,
    MathXFloorDivideY,
    MathXMinusX,
    MathXMinusY,
    MathXPlusX,
    MathXPlusY,
    MathXy,
    MathYMinusY,
    MathYPlusY,
    Matrix,
    MedicineSyrup,
    Menorah,
    Menu,
    Menu2,
    Menu3,
    Menu4,
    MenuDeep,
    MenuOrder,
    MeterCube,
    MeterSquare,
    Metronome,
    Microfrontends,
    Microscope,
    MicroscopeOff,
    Middleware,
    MilitaryAward,
    MilitaryRank,
    Minus,
    MinusVertical,
    Mobiledata,
    MobiledataOff,
    Mosque,
    Multiplier05x,
    Multiplier15x,
    Multiplier1x,
    Multiplier2x,
    MusicDiscount,
    NewSection,
    NoCopyright,
    NoCreativeCommons,
    NoDerivatives,
    Notification,
    NotificationOff,
    Nurse,
    ObjectScan,
    Old,
    Om,
    Omega,
    Option,
    Outbound,
    Outlet,
    PackageExport,
    PackageImport,
    Packages,
    Parentheses,
    ParenthesesOff,
    ParkingCircle,
    Password,
    PasswordFingerprint,
    PasswordMobilePhone,
    PasswordUser,
    Paywall,
    Peace,
    Pendulum,
    Percentage,
    Percentage0,
    Percentage10,
    Percentage100,
    Percentage20,
    Percentage25,
    Percentage30,
    Percentage33,
    Percentage40,
    Percentage50,
    Percentage60,
    Percentage66,
    Percentage70,
    Percentage75,
    Percentage80,
    Percentage90,
    PhotoScan,
    Physotherapist,
    Pill,
    PillOff,
    Pillow,
    Pills,
    PinEnd,
    PinInvoke,
    Pipeline,
    PlaylistAdd,
    Plunger,
    Plus,
    PlusEqual,
    PlusMinus,
    Podium,
    PodiumOff,
    Point,
    PointOff,
    Pointer,
    Pointer2,
    PointerBolt,
    PointerCancel,
    PointerCheck,
    PointerCode,
    PointerCog,
    PointerCollaboration,
    PointerCollaboration2,
    PointerDollar,
    PointerDown,
    PointerExclamation,
    PointerHeart,
    PointerMinus,
    PointerOff,
    PointerPause,
    PointerPin,
    PointerPlus,
    PointerQuestion,
    PointerSearch,
    PointerShare,
    PointerStar,
    PointerUp,
    PointerX,
    Poo,
    Pray,
    PremiumRights,
    Prescription,
    PrismLight,
    Progress,
    ProgressAlert,
    ProgressBolt,
    ProgressCheck,
    ProgressDown,
    ProgressHelp,
    ProgressX,
    Prompt,
    Propeller,
    PropellerOff,
    Protocol,
    QuestionMark,
    QueuePopIn,
    QueuePopOut,
    Radioactive,
    RadioactiveOff,
    Rating12Plus,
    Rating14Plus,
    Rating16Plus,
    Rating18Plus,
    Rating21Plus,
    Razor,
    RazorElectric,
    Recharging,
    RecordMail,
    RecordMailOff,
    Recycle,
    RecycleOff,
    Registered,
    RelationManyToMany,
    RelationOneToMany,
    RelationOneToOne,
    Reorder,
    Replace,
    ReplaceOff,
    ReplaceUser,
    ReservedLine,
    Restore,
    RibbonHealth,
    RobotFace,
    RobotOff,
    Rotate3d,
    RouteScan,
    RowInsertBottom,
    RowInsertTop,
    RowRemove,
    RvTruck,
    Sandbox,
    Scale,
    ScaleOff,
    ScaleOutline,
    ScaleOutlineOff,
    Scan,
    ScanCube,
    ScanEye,
    ScanPosition,
    ScanTraces,
    Schema,
    SchemaOff,
    SchoolBell,
    Sdk,
    Search,
    SearchOff,
    SelectAll,
    Seo,
    Serverless,
    Servicemark,
    Settings,
    Settings2,
    SettingsAi,
    SettingsAutomation,
    SettingsBolt,
    SettingsCancel,
    SettingsCheck,
    SettingsCode,
    SettingsCog,
    SettingsDollar,
    SettingsDown,
    SettingsExclamation,
    SettingsHeart,
    SettingsMinus,
    SettingsOff,
    SettingsPause,
    SettingsPin,
    SettingsPlus,
    SettingsQuestion,
    SettingsSearch,
    SettingsShare,
    SettingsSpark,
    SettingsStar,
    SettingsUp,
    SettingsX,
    Share,
    ShareOff,
    Shield,
    ShieldBolt,
    ShieldCancel,
    ShieldCheck,
    ShieldCheckered,
    ShieldChevron,
    ShieldCode,
    ShieldCog,
    ShieldDollar,
    ShieldDown,
    ShieldExclamation,
    ShieldHalf,
    ShieldHeart,
    ShieldLock,
    ShieldMinus,
    ShieldOff,
    ShieldPause,
    ShieldPin,
    ShieldPlus,
    ShieldQuestion,
    ShieldSearch,
    ShieldShare,
    ShieldStar,
    ShieldUp,
    ShieldX,
    Sitemap,
    SitemapOff,
    SkewX,
    SkewY,
    Skull,
    Slash,
    Slashes,
    SmartHome,
    SmartHomeOff,
    Smoking,
    SmokingNo,
    Snowboarding,
    Social,
    SocialOff,
    Sofa,
    SofaOff,
    SolarElectricity,
    SolarPanel,
    SolarPanel2,
    SortAscendingShapes,
    SortAscendingSmallBig,
    SortDescendingShapes,
    SortDescendingSmallBig,
    Sos,
    SourceCode,
    Spaces,
    Sparkles,
    Spiral,
    SpiralOff,
    Spray,
    Spy,
    SpyOff,
    SquareF0,
    SquareF1,
    SquareF2,
    SquareF3,
    SquareF4,
    SquareF5,
    SquareF6,
    SquareF7,
    SquareF8,
    SquareF9,
    SquareRoot,
    SquareRoot2,
    StackBack,
    StackBackward,
    StackForward,
    StackFront,
    StackMiddle,
    Star,
    StarHalf,
    StarOff,
    Stars,
    StarsOff,
    StatusChange,
    Steam,
    StereoGlasses,
    Stethoscope,
    StethoscopeOff,
    Stopwatch,
    Subtask,
    Sum,
    SumOff,
    Sunglasses,
    Swipe,
    Table,
    TableAlias,
    TableColumn,
    TableDashed,
    TableDown,
    TableExport,
    TableHeart,
    TableImport,
    TableMinus,
    TableOff,
    TableOptions,
    TablePlus,
    TableRow,
    TableShare,
    TableShortcut,
    TableSpark,
    Tallymark1,
    Tallymark2,
    Tallymark3,
    Tallymark4,
    Tallymarks,
    Telescope,
    TelescopeOff,
    TemperatureSnow,
    TemperatureSun,
    Terminal,
    Terminal2,
    TestPipe,
    TestPipe2,
    TestPipeOff,
    TextScan2,
    TextScanAi,
    Texture,
    Thermometer,
    ThumbDown,
    ThumbDownOff,
    ThumbUp,
    ThumbUpOff,
    Tilde,
    TimeDuration0,
    TimeDuration10,
    TimeDuration15,
    TimeDuration30,
    TimeDuration45,
    TimeDuration5,
    TimeDuration60,
    TimeDuration90,
    TimeDurationOff,
    Timeline,
    TimelineEvent,
    TimelineEventExclamation,
    TimelineEventMinus,
    TimelineEventPlus,
    TimelineEventText,
    TimelineEventX,
    Timezone,
    TipJar,
    TipJarEuro,
    TipJarPound,
    ToggleLeft,
    ToggleRight,
    ToiletPaper,
    ToiletPaperOff,
    Tool,
    Tooltip,
    Torii,
    Tower,
    TowerOff,
    Trademark,
    Transfer,
    TransferVertical,
    Transform,
    TransformPoint,
    TransformPointBottomLeft,
    TransformPointBottomRight,
    TransformPointTopLeft,
    TransformPointTopRight,
    Trash,
    TrashOff,
    TrashX,
    Trowel,
    UfoOff,
    Uhd,
    Umbrella,
    Umbrella2,
    UmbrellaClosed,
    UmbrellaClosed2,
    UmbrellaOff,
    Universe,
    Urgent,
    Usb,
    User,
    UserBitcoin,
    UserBolt,
    UserCancel,
    UserCheck,
    UserCircle,
    UserCode,
    UserCog,
    UserDollar,
    UserDown,
    UserEdit,
    UserExclamation,
    UserHeart,
    UserHexagon,
    UserKey,
    UserMinus,
    UserOff,
    UserPause,
    UserPentagon,
    UserPin,
    UserPlus,
    UserQuestion,
    UserScan,
    UserSearch,
    UserShare,
    UserShield,
    UserSquare,
    UserSquareRounded,
    UserStar,
    UserUp,
    UserX,
    Users,
    UsersGroup,
    UsersMinus,
    UsersPlus,
    Vaccine,
    VaccineBottle,
    VaccineBottleOff,
    VaccineOff,
    VacuumCleaner,
    Variable,
    VariableMinus,
    VariableOff,
    VariablePlus,
    Venus,
    Versions,
    VersionsOff,
    View360,
    View360Arrow,
    View360Number,
    View360Off,
    ViewportShort,
    ViewportTall,
    Vip,
    Vip2,
    VipOff,
    Virus,
    VirusOff,
    VirusSearch,
    Vs,
    Wall,
    WallOff,
    Wallpaper,
    WallpaperOff,
    Wand,
    WandOff,
    WaveSawTool,
    WaveSine,
    WaveSquare,
    WavesElectricity,
    Webhook,
    WebhookOff,
    Weight,
    Window,
    WindowMaximize,
    WindowMinimize,
    WindowOff,
    Woman,
    Wood,
    X,
    XMark,
    XPowerY,
    Xd,
    Xxx,
    YinYang,
    ZeroConfig,
    ZoomScan,
    Zzz,
    ZzzOff,
}

impl SystemIcon {
    /// Returns all available icons in this category.
    pub fn all() -> &'static [Self] {
        &[Self::AB, Self::Abacus, Self::AbacusOff, Self::Accessible, Self::AccessibleOff, Self::Activity, Self::ActivityHeartbeat, Self::AdCircle, Self::AdCircleOff, Self::Adjustments, Self::AdjustmentsAlt, Self::AdjustmentsBolt, Self::AdjustmentsCancel, Self::AdjustmentsCheck, Self::AdjustmentsCode, Self::AdjustmentsCog, Self::AdjustmentsDollar, Self::AdjustmentsDown, Self::AdjustmentsExclamation, Self::AdjustmentsHeart, Self::AdjustmentsHorizontal, Self::AdjustmentsMinus, Self::AdjustmentsOff, Self::AdjustmentsPause, Self::AdjustmentsPin, Self::AdjustmentsPlus, Self::AdjustmentsQuestion, Self::AdjustmentsSearch, Self::AdjustmentsShare, Self::AdjustmentsSpark, Self::AdjustmentsStar, Self::AdjustmentsUp, Self::AdjustmentsX, Self::Affiliate, Self::Ai, Self::AiAgent, Self::AiAgents, Self::AiGateway, Self::Alarm, Self::AlarmAverage, Self::AlarmMinus, Self::AlarmOff, Self::AlarmPlus, Self::AlarmSnooze, Self::AlertCircle, Self::AlertCircleOff, Self::AlertHexagon, Self::AlertHexagonOff, Self::AlertOctagon, Self::AlertSmall, Self::AlertSmallOff, Self::AlertSquare, Self::AlertSquareRounded, Self::AlertSquareRoundedOff, Self::AlertTriangle, Self::AlertTriangleOff, Self::Alt, Self::Ampersand, Self::Analyze, Self::AnalyzeOff, Self::Ankh, Self::Api, Self::ApiApp, Self::ApiAppOff, Self::ApiBook, Self::ApiOff, Self::AppWindow, Self::Apps, Self::AppsOff, Self::Armchair, Self::Armchair2, Self::Armchair2Off, Self::ArmchairOff, Self::Assembly, Self::AssemblyOff, Self::Asset, Self::AugmentedReality, Self::AugmentedReality2, Self::AugmentedRealityOff, Self::Auth2fa, Self::Automation, Self::BabyBottle, Self::BabyCarriage, Self::Backslash, Self::Ban, Self::Bandage, Self::BandageOff, Self::Barcode, Self::BarcodeOff, Self::Bath, Self::BathOff, Self::BedFlat, Self::Bell, Self::BellBolt, Self::BellCancel, Self::BellCheck, Self::BellCode, Self::BellCog, Self::BellDollar, Self::BellDown, Self::BellExclamation, Self::BellHeart, Self::BellMinus, Self::BellOff, Self::BellPause, Self::BellPin, Self::BellPlus, Self::BellQuestion, Self::BellRinging, Self::BellRinging2, Self::BellSchool, Self::BellSearch, Self::BellShare, Self::BellStar, Self::BellUp, Self::BellX, Self::BellZ, Self::Biohazard, Self::BiohazardOff, Self::Blind, Self::Blob, Self::Blocks, Self::BodyScan, Self::BotId, Self::Braces, Self::BracesOff, Self::Brackets, Self::BracketsContain, Self::BracketsContainEnd, Self::BracketsContainStart, Self::BracketsOff, Self::Braille, Self::Brain, Self::Bug, Self::BugOff, Self::Building, Self::BuildingAirport, Self::BuildingArch, Self::BuildingBank, Self::BuildingBridge, Self::BuildingBridge2, Self::BuildingBroadcastTower, Self::BuildingBurjAlArab, Self::BuildingCarousel, Self::BuildingCastle, Self::BuildingChurch, Self::BuildingCircus, Self::BuildingCog, Self::BuildingCommunity, Self::BuildingCottage, Self::BuildingEiffelTower, Self::BuildingEstate, Self::BuildingFactory, Self::BuildingFactory2, Self::BuildingFortress, Self::BuildingHospital, Self::BuildingLighthouse, Self::BuildingMinus, Self::BuildingMonument, Self::BuildingMosque, Self::BuildingOff, Self::BuildingPavilion, Self::BuildingPlus, Self::BuildingSkyscraper, Self::BuildingStadium, Self::BuildingStore, Self::BuildingTunnel, Self::BuildingWarehouse, Self::BuildingWindTurbine, Self::Buildings, Self::Calendar, Self::CalendarBolt, Self::CalendarCancel, Self::CalendarCheck, Self::CalendarClock, Self::CalendarCode, Self::CalendarCog, Self::CalendarDollar, Self::CalendarDot, Self::CalendarDown, Self::CalendarDue, Self::CalendarEvent, Self::CalendarExclamation, Self::CalendarHeart, Self::CalendarMinus, Self::CalendarMonth, Self::CalendarOff, Self::CalendarPause, Self::CalendarPin, Self::CalendarPlus, Self::CalendarQuestion, Self::CalendarRepeat, Self::CalendarSad, Self::CalendarSearch, Self::CalendarShare, Self::CalendarSmile, Self::CalendarStar, Self::CalendarStats, Self::CalendarTime, Self::CalendarUp, Self::CalendarUser, Self::CalendarWeek, Self::CalendarX, Self::Canary, Self::Cane, Self::CarGarage, Self::Ce, Self::CeOff, Self::ChalkboardTeacher, Self::Check, Self::Checkbox, Self::Checklist, Self::Checks, Self::CheckupList, Self::ClearAll, Self::Click, Self::Clock, Self::Clock12, Self::Clock2, Self::Clock24, Self::ClockBitcoin, Self::ClockBolt, Self::ClockCancel, Self::ClockCheck, Self::ClockCode, Self::ClockCog, Self::ClockDollar, Self::ClockDown, Self::ClockEdit, Self::ClockExclamation, Self::ClockHeart, Self::ClockHour1, Self::ClockHour10, Self::ClockHour11, Self::ClockHour12, Self::ClockHour2, Self::ClockHour3, Self::ClockHour4, Self::ClockHour5, Self::ClockHour6, Self::ClockHour7, Self::ClockHour8, Self::ClockHour9, Self::ClockMinus, Self::ClockOff, Self::ClockPause, Self::ClockPin, Self::ClockPlay, Self::ClockPlus, Self::ClockQuestion, Self::ClockRecord, Self::ClockSearch, Self::ClockShare, Self::ClockShield, Self::ClockStar, Self::ClockStop, Self::ClockUp, Self::ClockX, Self::CloudBitcoin, Self::CloudDownload, Self::CloudLock, Self::CloudLockOpen, Self::CloudNetwork, Self::CloudUpload, Self::CodeAi, Self::CodeVariable, Self::CodeVariableMinus, Self::CodeVariablePlus, Self::Codeblock, Self::ColumnInsertLeft, Self::ColumnInsertRight, Self::ColumnRemove, Self::Command, Self::CommandOff, Self::Confucius, Self::CongruentTo, Self::Connection, Self::Copyleft, Self::CopyleftOff, Self::Copyright, Self::CopyrightOff, Self::CreativeCommons, Self::CreativeCommonsBy, Self::CreativeCommonsNc, Self::CreativeCommonsNd, Self::CreativeCommonsOff, Self::CreativeCommonsSa, Self::CreativeCommonsZero, Self::Credits, Self::Cross, Self::CrossOff, Self::Crosshair, Self::Crutches, Self::CrutchesOff, Self::Cube3dSphere, Self::Cube3dSphereOff, Self::CubeSend, Self::CubeUnfolded, Self::CurlyLoop, Self::CursorOff, Self::Dashboard, Self::DashboardOff, Self::Database, Self::DatabaseCog, Self::DatabaseDollar, Self::DatabaseEdit, Self::DatabaseExclamation, Self::DatabaseExport, Self::DatabaseHeart, Self::DatabaseImport, Self::DatabaseLeak, Self::DatabaseMinus, Self::DatabaseOff, Self::DatabasePlus, Self::DatabaseSearch, Self::DatabaseShare, Self::DatabaseSmile, Self::DatabaseStar, Self::DatabaseX, Self::Deaf, Self::Decimal, Self::Dental, Self::DentalBroken, Self::DentalOff, Self::Deselect, Self::Desk, Self::Details, Self::DetailsOff, Self::DeviceProjector, Self::DeviceUnknown, Self::Direction, Self::DirectionArrows, Self::DirectionHorizontal, Self::DirectionSign, Self::DirectionSignOff, Self::Disabled, Self::Disabled2, Self::DisabledOff, Self::Divide, Self::Dna, Self::Dna2, Self::Dna2Off, Self::DnaOff, Self::Door, Self::DoorEnter, Self::DoorExit, Self::DoorHanger, Self::DoorOff, Self::Dots, Self::DotsCircleHorizontal, Self::DotsDiagonal, Self::DotsDiagonal2, Self::DotsVertical, Self::Ear, Self::EarOff, Self::EarScan, Self::Elevator, Self::ElevatorOff, Self::EmergencyBed, Self::Empathize, Self::EmpathizeOff, Self::Equal, Self::EqualDouble, Self::EqualNot, Self::Exchange, Self::ExchangeOff, Self::ExclamationCircle, Self::ExclamationMark, Self::ExclamationMarkOff, Self::Explicit, Self::ExplicitOff, Self::ExternalLink, Self::ExternalLinkOff, Self::Eye, Self::EyeBitcoin, Self::EyeBolt, Self::EyeCancel, Self::EyeCheck, Self::EyeClosed, Self::EyeCode, Self::EyeCog, Self::EyeDiscount, Self::EyeDollar, Self::EyeDotted, Self::EyeDown, Self::EyeEdit, Self::EyeExclamation, Self::EyeHeart, Self::EyeMinus, Self::EyeOff, Self::EyePause, Self::EyePin, Self::EyePlus, Self::EyeQuestion, Self::EyeSearch, Self::EyeShare, Self::EyeSpark, Self::EyeStar, Self::EyeTable, Self::EyeUp, Self::EyeX, Self::Eyeglass, Self::Eyeglass2, Self::EyeglassOff, Self::FaceId, Self::FaceIdError, Self::FaceMask, Self::FaceMaskOff, Self::Fall, Self::Fence, Self::FenceOff, Self::FidgetSpinner, Self::Filter, Self::Filter2, Self::Filter2Bolt, Self::Filter2Cancel, Self::Filter2Check, Self::Filter2Code, Self::Filter2Cog, Self::Filter2Discount, Self::Filter2Dollar, Self::Filter2Down, Self::Filter2Edit, Self::Filter2Exclamation, Self::Filter2Minus, Self::Filter2Pause, Self::Filter2Pin, Self::Filter2Plus, Self::Filter2Question, Self::Filter2Search, Self::Filter2Share, Self::Filter2Spark, Self::Filter2Up, Self::Filter2X, Self::FilterBolt, Self::FilterCancel, Self::FilterCheck, Self::FilterCode, Self::FilterCog, Self::FilterDiscount, Self::FilterDollar, Self::FilterDown, Self::FilterEdit, Self::FilterExclamation, Self::FilterHeart, Self::FilterMinus, Self::FilterOff, Self::FilterPause, Self::FilterPin, Self::FilterPlus, Self::FilterQuestion, Self::FilterSearch, Self::FilterShare, Self::FilterSpark, Self::FilterStar, Self::FilterUp, Self::FilterX, Self::Filters, Self::Fingerprint, Self::FingerprintOff, Self::FingerprintScan, Self::FireExtinguisher, Self::FirewallCheck, Self::FirewallFlame, Self::FirstAidKit, Self::FirstAidKitOff, Self::FishChristianity, Self::FlagBitcoin, Self::FlagDiscount, Self::Flask, Self::Flask2, Self::Flask2Off, Self::FlaskOff, Self::FocusCentered, Self::Foodsteps, Self::Forbid, Self::Forbid2, Self::FreeRights, Self::Friends, Self::FriendsOff, Self::Function, Self::FunctionOff, Self::Galaxy, Self::Gauge, Self::GaugeOff, Self::Gavel, Self::Graph, Self::GraphOff, Self::GridDots, Self::GridPattern, Self::GridScan, Self::GripHorizontal, Self::GripVertical, Self::Gymnastics, Self::HandSanitizer, Self::Hanger2, Self::HazeMoon, Self::HealthRecognition, Self::HeartBitcoin, Self::HeartBroken, Self::HeartDiscount, Self::HeartHandshake, Self::HeartRateMonitor, Self::Heartbeat, Self::Help, Self::HelpCircle, Self::HelpHexagon, Self::HelpOctagon, Self::HelpOff, Self::HelpSmall, Self::HelpSquare, Self::HelpSquareRounded, Self::HelpTriangle, Self::History, Self::HistoryOff, Self::HistoryToggle, Self::Home, Self::Home2, Self::HomeBitcoin, Self::HomeBolt, Self::HomeCancel, Self::HomeCheck, Self::HomeCog, Self::HomeDollar, Self::HomeDot, Self::HomeDown, Self::HomeEco, Self::HomeEdit, Self::HomeExclamation, Self::HomeHand, Self::HomeHeart, Self::HomeInfinity, Self::HomeLink, Self::HomeLock, Self::HomeMinus, Self::HomeMove, Self::HomeOff, Self::HomePlus, Self::HomeQuestion, Self::HomeRibbon, Self::HomeSearch, Self::HomeShare, Self::HomeShield, Self::HomeSignal, Self::HomeSpark, Self::HomeStar, Self::HomeStats, Self::HomeUp, Self::HomeX, Self::Hospital, Self::HospitalCircle, Self::HotelService, Self::Hourglass, Self::HourglassEmpty, Self::HourglassHigh, Self::HourglassLow, Self::HourglassOff, Self::Hours12, Self::Hours24, Self::Id, Self::IdBadge, Self::IdBadge2, Self::IdBadgeOff, Self::IdOff, Self::ImageGeneration, Self::Inbox, Self::InboxOff, Self::Infinity, Self::Infinity2, Self::InfinityOff, Self::InfoCircle, Self::InfoHexagon, Self::InfoOctagon, Self::InfoSmall, Self::InfoSquare, Self::InfoSquareRounded, Self::InfoTriangle, Self::InputAi, Self::InputCheck, Self::InputSpark, Self::InputX, Self::Jetpack, Self::JoinBevel, Self::JoinRound, Self::JoinStraight, Self::Label, Self::LabelImportant, Self::LabelOff, Self::Ladder, Self::LadderOff, Self::Lane, Self::LayoutBottombarInactive, Self::LayoutNavbarInactive, Self::LayoutSidebarInactive, Self::LayoutSidebarRightInactive, Self::Lego, Self::LegoOff, Self::Library, Self::LibraryMinus, Self::LibraryPhoto, Self::LibraryPlus, Self::Lifebuoy, Self::LifebuoyOff, Self::Lighter, Self::LineScan, Self::ListLetters, Self::ListTree, Self::Loader, Self::Loader2, Self::Loader3, Self::Loader4, Self::LoaderQuarter, Self::LocationDiscount, Self::Lock, Self::LockAccess, Self::LockAccessOff, Self::LockBitcoin, Self::LockBolt, Self::LockCancel, Self::LockCheck, Self::LockCode, Self::LockCog, Self::LockDollar, Self::LockDown, Self::LockExclamation, Self::LockHeart, Self::LockMinus, Self::LockOff, Self::LockOpen, Self::LockOpen2, Self::LockOpenOff, Self::LockPassword, Self::LockPause, Self::LockPin, Self::LockPlus, Self::LockQuestion, Self::LockSearch, Self::LockShare, Self::LockSquare, Self::LockSquareRounded, Self::LockStar, Self::LockUp, Self::LockX, Self::LogicAnd, Self::LogicBuffer, Self::LogicNand, Self::LogicNor, Self::LogicNot, Self::LogicOr, Self::LogicXnor, Self::LogicXor, Self::Logout, Self::Logout2, Self::Logs, Self::Luggage, Self::LuggageOff, Self::Lungs, Self::LungsOff, Self::Man, Self::Marquee, Self::Marquee2, Self::MarqueeOff, Self::Mars, Self::Massage, Self::Matchstick, Self::Math, Self::Math1Divide2, Self::Math1Divide3, Self::MathAvg, Self::MathCos, Self::MathCtg, Self::MathEqualGreater, Self::MathEqualLower, Self::MathFunction, Self::MathFunctionOff, Self::MathFunctionY, Self::MathGreater, Self::MathIntegral, Self::MathIntegralX, Self::MathIntegrals, Self::MathLower, Self::MathMax, Self::MathMaxMin, Self::MathMin, Self::MathNot, Self::MathOff, Self::MathPi, Self::MathPiDivide2, Self::MathSec, Self::MathSin, Self::MathSymbols, Self::MathTg, Self::MathXDivide2, Self::MathXDivideY, Self::MathXDivideY2, Self::MathXFloorDivideY, Self::MathXMinusX, Self::MathXMinusY, Self::MathXPlusX, Self::MathXPlusY, Self::MathXy, Self::MathYMinusY, Self::MathYPlusY, Self::Matrix, Self::MedicineSyrup, Self::Menorah, Self::Menu, Self::Menu2, Self::Menu3, Self::Menu4, Self::MenuDeep, Self::MenuOrder, Self::MeterCube, Self::MeterSquare, Self::Metronome, Self::Microfrontends, Self::Microscope, Self::MicroscopeOff, Self::Middleware, Self::MilitaryAward, Self::MilitaryRank, Self::Minus, Self::MinusVertical, Self::Mobiledata, Self::MobiledataOff, Self::Mosque, Self::Multiplier05x, Self::Multiplier15x, Self::Multiplier1x, Self::Multiplier2x, Self::MusicDiscount, Self::NewSection, Self::NoCopyright, Self::NoCreativeCommons, Self::NoDerivatives, Self::Notification, Self::NotificationOff, Self::Nurse, Self::ObjectScan, Self::Old, Self::Om, Self::Omega, Self::Option, Self::Outbound, Self::Outlet, Self::PackageExport, Self::PackageImport, Self::Packages, Self::Parentheses, Self::ParenthesesOff, Self::ParkingCircle, Self::Password, Self::PasswordFingerprint, Self::PasswordMobilePhone, Self::PasswordUser, Self::Paywall, Self::Peace, Self::Pendulum, Self::Percentage, Self::Percentage0, Self::Percentage10, Self::Percentage100, Self::Percentage20, Self::Percentage25, Self::Percentage30, Self::Percentage33, Self::Percentage40, Self::Percentage50, Self::Percentage60, Self::Percentage66, Self::Percentage70, Self::Percentage75, Self::Percentage80, Self::Percentage90, Self::PhotoScan, Self::Physotherapist, Self::Pill, Self::PillOff, Self::Pillow, Self::Pills, Self::PinEnd, Self::PinInvoke, Self::Pipeline, Self::PlaylistAdd, Self::Plunger, Self::Plus, Self::PlusEqual, Self::PlusMinus, Self::Podium, Self::PodiumOff, Self::Point, Self::PointOff, Self::Pointer, Self::Pointer2, Self::PointerBolt, Self::PointerCancel, Self::PointerCheck, Self::PointerCode, Self::PointerCog, Self::PointerCollaboration, Self::PointerCollaboration2, Self::PointerDollar, Self::PointerDown, Self::PointerExclamation, Self::PointerHeart, Self::PointerMinus, Self::PointerOff, Self::PointerPause, Self::PointerPin, Self::PointerPlus, Self::PointerQuestion, Self::PointerSearch, Self::PointerShare, Self::PointerStar, Self::PointerUp, Self::PointerX, Self::Poo, Self::Pray, Self::PremiumRights, Self::Prescription, Self::PrismLight, Self::Progress, Self::ProgressAlert, Self::ProgressBolt, Self::ProgressCheck, Self::ProgressDown, Self::ProgressHelp, Self::ProgressX, Self::Prompt, Self::Propeller, Self::PropellerOff, Self::Protocol, Self::QuestionMark, Self::QueuePopIn, Self::QueuePopOut, Self::Radioactive, Self::RadioactiveOff, Self::Rating12Plus, Self::Rating14Plus, Self::Rating16Plus, Self::Rating18Plus, Self::Rating21Plus, Self::Razor, Self::RazorElectric, Self::Recharging, Self::RecordMail, Self::RecordMailOff, Self::Recycle, Self::RecycleOff, Self::Registered, Self::RelationManyToMany, Self::RelationOneToMany, Self::RelationOneToOne, Self::Reorder, Self::Replace, Self::ReplaceOff, Self::ReplaceUser, Self::ReservedLine, Self::Restore, Self::RibbonHealth, Self::RobotFace, Self::RobotOff, Self::Rotate3d, Self::RouteScan, Self::RowInsertBottom, Self::RowInsertTop, Self::RowRemove, Self::RvTruck, Self::Sandbox, Self::Scale, Self::ScaleOff, Self::ScaleOutline, Self::ScaleOutlineOff, Self::Scan, Self::ScanCube, Self::ScanEye, Self::ScanPosition, Self::ScanTraces, Self::Schema, Self::SchemaOff, Self::SchoolBell, Self::Sdk, Self::Search, Self::SearchOff, Self::SelectAll, Self::Seo, Self::Serverless, Self::Servicemark, Self::Settings, Self::Settings2, Self::SettingsAi, Self::SettingsAutomation, Self::SettingsBolt, Self::SettingsCancel, Self::SettingsCheck, Self::SettingsCode, Self::SettingsCog, Self::SettingsDollar, Self::SettingsDown, Self::SettingsExclamation, Self::SettingsHeart, Self::SettingsMinus, Self::SettingsOff, Self::SettingsPause, Self::SettingsPin, Self::SettingsPlus, Self::SettingsQuestion, Self::SettingsSearch, Self::SettingsShare, Self::SettingsSpark, Self::SettingsStar, Self::SettingsUp, Self::SettingsX, Self::Share, Self::ShareOff, Self::Shield, Self::ShieldBolt, Self::ShieldCancel, Self::ShieldCheck, Self::ShieldCheckered, Self::ShieldChevron, Self::ShieldCode, Self::ShieldCog, Self::ShieldDollar, Self::ShieldDown, Self::ShieldExclamation, Self::ShieldHalf, Self::ShieldHeart, Self::ShieldLock, Self::ShieldMinus, Self::ShieldOff, Self::ShieldPause, Self::ShieldPin, Self::ShieldPlus, Self::ShieldQuestion, Self::ShieldSearch, Self::ShieldShare, Self::ShieldStar, Self::ShieldUp, Self::ShieldX, Self::Sitemap, Self::SitemapOff, Self::SkewX, Self::SkewY, Self::Skull, Self::Slash, Self::Slashes, Self::SmartHome, Self::SmartHomeOff, Self::Smoking, Self::SmokingNo, Self::Snowboarding, Self::Social, Self::SocialOff, Self::Sofa, Self::SofaOff, Self::SolarElectricity, Self::SolarPanel, Self::SolarPanel2, Self::SortAscendingShapes, Self::SortAscendingSmallBig, Self::SortDescendingShapes, Self::SortDescendingSmallBig, Self::Sos, Self::SourceCode, Self::Spaces, Self::Sparkles, Self::Spiral, Self::SpiralOff, Self::Spray, Self::Spy, Self::SpyOff, Self::SquareF0, Self::SquareF1, Self::SquareF2, Self::SquareF3, Self::SquareF4, Self::SquareF5, Self::SquareF6, Self::SquareF7, Self::SquareF8, Self::SquareF9, Self::SquareRoot, Self::SquareRoot2, Self::StackBack, Self::StackBackward, Self::StackForward, Self::StackFront, Self::StackMiddle, Self::Star, Self::StarHalf, Self::StarOff, Self::Stars, Self::StarsOff, Self::StatusChange, Self::Steam, Self::StereoGlasses, Self::Stethoscope, Self::StethoscopeOff, Self::Stopwatch, Self::Subtask, Self::Sum, Self::SumOff, Self::Sunglasses, Self::Swipe, Self::Table, Self::TableAlias, Self::TableColumn, Self::TableDashed, Self::TableDown, Self::TableExport, Self::TableHeart, Self::TableImport, Self::TableMinus, Self::TableOff, Self::TableOptions, Self::TablePlus, Self::TableRow, Self::TableShare, Self::TableShortcut, Self::TableSpark, Self::Tallymark1, Self::Tallymark2, Self::Tallymark3, Self::Tallymark4, Self::Tallymarks, Self::Telescope, Self::TelescopeOff, Self::TemperatureSnow, Self::TemperatureSun, Self::Terminal, Self::Terminal2, Self::TestPipe, Self::TestPipe2, Self::TestPipeOff, Self::TextScan2, Self::TextScanAi, Self::Texture, Self::Thermometer, Self::ThumbDown, Self::ThumbDownOff, Self::ThumbUp, Self::ThumbUpOff, Self::Tilde, Self::TimeDuration0, Self::TimeDuration10, Self::TimeDuration15, Self::TimeDuration30, Self::TimeDuration45, Self::TimeDuration5, Self::TimeDuration60, Self::TimeDuration90, Self::TimeDurationOff, Self::Timeline, Self::TimelineEvent, Self::TimelineEventExclamation, Self::TimelineEventMinus, Self::TimelineEventPlus, Self::TimelineEventText, Self::TimelineEventX, Self::Timezone, Self::TipJar, Self::TipJarEuro, Self::TipJarPound, Self::ToggleLeft, Self::ToggleRight, Self::ToiletPaper, Self::ToiletPaperOff, Self::Tool, Self::Tooltip, Self::Torii, Self::Tower, Self::TowerOff, Self::Trademark, Self::Transfer, Self::TransferVertical, Self::Transform, Self::TransformPoint, Self::TransformPointBottomLeft, Self::TransformPointBottomRight, Self::TransformPointTopLeft, Self::TransformPointTopRight, Self::Trash, Self::TrashOff, Self::TrashX, Self::Trowel, Self::UfoOff, Self::Uhd, Self::Umbrella, Self::Umbrella2, Self::UmbrellaClosed, Self::UmbrellaClosed2, Self::UmbrellaOff, Self::Universe, Self::Urgent, Self::Usb, Self::User, Self::UserBitcoin, Self::UserBolt, Self::UserCancel, Self::UserCheck, Self::UserCircle, Self::UserCode, Self::UserCog, Self::UserDollar, Self::UserDown, Self::UserEdit, Self::UserExclamation, Self::UserHeart, Self::UserHexagon, Self::UserKey, Self::UserMinus, Self::UserOff, Self::UserPause, Self::UserPentagon, Self::UserPin, Self::UserPlus, Self::UserQuestion, Self::UserScan, Self::UserSearch, Self::UserShare, Self::UserShield, Self::UserSquare, Self::UserSquareRounded, Self::UserStar, Self::UserUp, Self::UserX, Self::Users, Self::UsersGroup, Self::UsersMinus, Self::UsersPlus, Self::Vaccine, Self::VaccineBottle, Self::VaccineBottleOff, Self::VaccineOff, Self::VacuumCleaner, Self::Variable, Self::VariableMinus, Self::VariableOff, Self::VariablePlus, Self::Venus, Self::Versions, Self::VersionsOff, Self::View360, Self::View360Arrow, Self::View360Number, Self::View360Off, Self::ViewportShort, Self::ViewportTall, Self::Vip, Self::Vip2, Self::VipOff, Self::Virus, Self::VirusOff, Self::VirusSearch, Self::Vs, Self::Wall, Self::WallOff, Self::Wallpaper, Self::WallpaperOff, Self::Wand, Self::WandOff, Self::WaveSawTool, Self::WaveSine, Self::WaveSquare, Self::WavesElectricity, Self::Webhook, Self::WebhookOff, Self::Weight, Self::Window, Self::WindowMaximize, Self::WindowMinimize, Self::WindowOff, Self::Woman, Self::Wood, Self::X, Self::XMark, Self::XPowerY, Self::Xd, Self::Xxx, Self::YinYang, Self::ZeroConfig, Self::ZoomScan, Self::Zzz, Self::ZzzOff]
    }

    /// Returns the icon count.
    pub fn count() -> usize {
        1185
    }

    /// Creates an icon from its kebab-case name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "a-b" => Some(Self::AB),
            "abacus" => Some(Self::Abacus),
            "abacus-off" => Some(Self::AbacusOff),
            "accessible" => Some(Self::Accessible),
            "accessible-off" => Some(Self::AccessibleOff),
            "activity" => Some(Self::Activity),
            "activity-heartbeat" => Some(Self::ActivityHeartbeat),
            "ad-circle" => Some(Self::AdCircle),
            "ad-circle-off" => Some(Self::AdCircleOff),
            "adjustments" => Some(Self::Adjustments),
            "adjustments-alt" => Some(Self::AdjustmentsAlt),
            "adjustments-bolt" => Some(Self::AdjustmentsBolt),
            "adjustments-cancel" => Some(Self::AdjustmentsCancel),
            "adjustments-check" => Some(Self::AdjustmentsCheck),
            "adjustments-code" => Some(Self::AdjustmentsCode),
            "adjustments-cog" => Some(Self::AdjustmentsCog),
            "adjustments-dollar" => Some(Self::AdjustmentsDollar),
            "adjustments-down" => Some(Self::AdjustmentsDown),
            "adjustments-exclamation" => Some(Self::AdjustmentsExclamation),
            "adjustments-heart" => Some(Self::AdjustmentsHeart),
            "adjustments-horizontal" => Some(Self::AdjustmentsHorizontal),
            "adjustments-minus" => Some(Self::AdjustmentsMinus),
            "adjustments-off" => Some(Self::AdjustmentsOff),
            "adjustments-pause" => Some(Self::AdjustmentsPause),
            "adjustments-pin" => Some(Self::AdjustmentsPin),
            "adjustments-plus" => Some(Self::AdjustmentsPlus),
            "adjustments-question" => Some(Self::AdjustmentsQuestion),
            "adjustments-search" => Some(Self::AdjustmentsSearch),
            "adjustments-share" => Some(Self::AdjustmentsShare),
            "adjustments-spark" => Some(Self::AdjustmentsSpark),
            "adjustments-star" => Some(Self::AdjustmentsStar),
            "adjustments-up" => Some(Self::AdjustmentsUp),
            "adjustments-x" => Some(Self::AdjustmentsX),
            "affiliate" => Some(Self::Affiliate),
            "ai" => Some(Self::Ai),
            "ai-agent" => Some(Self::AiAgent),
            "ai-agents" => Some(Self::AiAgents),
            "ai-gateway" => Some(Self::AiGateway),
            "alarm" => Some(Self::Alarm),
            "alarm-average" => Some(Self::AlarmAverage),
            "alarm-minus" => Some(Self::AlarmMinus),
            "alarm-off" => Some(Self::AlarmOff),
            "alarm-plus" => Some(Self::AlarmPlus),
            "alarm-snooze" => Some(Self::AlarmSnooze),
            "alert-circle" => Some(Self::AlertCircle),
            "alert-circle-off" => Some(Self::AlertCircleOff),
            "alert-hexagon" => Some(Self::AlertHexagon),
            "alert-hexagon-off" => Some(Self::AlertHexagonOff),
            "alert-octagon" => Some(Self::AlertOctagon),
            "alert-small" => Some(Self::AlertSmall),
            "alert-small-off" => Some(Self::AlertSmallOff),
            "alert-square" => Some(Self::AlertSquare),
            "alert-square-rounded" => Some(Self::AlertSquareRounded),
            "alert-square-rounded-off" => Some(Self::AlertSquareRoundedOff),
            "alert-triangle" => Some(Self::AlertTriangle),
            "alert-triangle-off" => Some(Self::AlertTriangleOff),
            "alt" => Some(Self::Alt),
            "ampersand" => Some(Self::Ampersand),
            "analyze" => Some(Self::Analyze),
            "analyze-off" => Some(Self::AnalyzeOff),
            "ankh" => Some(Self::Ankh),
            "api" => Some(Self::Api),
            "api-app" => Some(Self::ApiApp),
            "api-app-off" => Some(Self::ApiAppOff),
            "api-book" => Some(Self::ApiBook),
            "api-off" => Some(Self::ApiOff),
            "app-window" => Some(Self::AppWindow),
            "apps" => Some(Self::Apps),
            "apps-off" => Some(Self::AppsOff),
            "armchair" => Some(Self::Armchair),
            "armchair-2" => Some(Self::Armchair2),
            "armchair-2-off" => Some(Self::Armchair2Off),
            "armchair-off" => Some(Self::ArmchairOff),
            "assembly" => Some(Self::Assembly),
            "assembly-off" => Some(Self::AssemblyOff),
            "asset" => Some(Self::Asset),
            "augmented-reality" => Some(Self::AugmentedReality),
            "augmented-reality-2" => Some(Self::AugmentedReality2),
            "augmented-reality-off" => Some(Self::AugmentedRealityOff),
            "auth-2fa" => Some(Self::Auth2fa),
            "automation" => Some(Self::Automation),
            "baby-bottle" => Some(Self::BabyBottle),
            "baby-carriage" => Some(Self::BabyCarriage),
            "backslash" => Some(Self::Backslash),
            "ban" => Some(Self::Ban),
            "bandage" => Some(Self::Bandage),
            "bandage-off" => Some(Self::BandageOff),
            "barcode" => Some(Self::Barcode),
            "barcode-off" => Some(Self::BarcodeOff),
            "bath" => Some(Self::Bath),
            "bath-off" => Some(Self::BathOff),
            "bed-flat" => Some(Self::BedFlat),
            "bell" => Some(Self::Bell),
            "bell-bolt" => Some(Self::BellBolt),
            "bell-cancel" => Some(Self::BellCancel),
            "bell-check" => Some(Self::BellCheck),
            "bell-code" => Some(Self::BellCode),
            "bell-cog" => Some(Self::BellCog),
            "bell-dollar" => Some(Self::BellDollar),
            "bell-down" => Some(Self::BellDown),
            "bell-exclamation" => Some(Self::BellExclamation),
            "bell-heart" => Some(Self::BellHeart),
            "bell-minus" => Some(Self::BellMinus),
            "bell-off" => Some(Self::BellOff),
            "bell-pause" => Some(Self::BellPause),
            "bell-pin" => Some(Self::BellPin),
            "bell-plus" => Some(Self::BellPlus),
            "bell-question" => Some(Self::BellQuestion),
            "bell-ringing" => Some(Self::BellRinging),
            "bell-ringing-2" => Some(Self::BellRinging2),
            "bell-school" => Some(Self::BellSchool),
            "bell-search" => Some(Self::BellSearch),
            "bell-share" => Some(Self::BellShare),
            "bell-star" => Some(Self::BellStar),
            "bell-up" => Some(Self::BellUp),
            "bell-x" => Some(Self::BellX),
            "bell-z" => Some(Self::BellZ),
            "biohazard" => Some(Self::Biohazard),
            "biohazard-off" => Some(Self::BiohazardOff),
            "blind" => Some(Self::Blind),
            "blob" => Some(Self::Blob),
            "blocks" => Some(Self::Blocks),
            "body-scan" => Some(Self::BodyScan),
            "bot-id" => Some(Self::BotId),
            "braces" => Some(Self::Braces),
            "braces-off" => Some(Self::BracesOff),
            "brackets" => Some(Self::Brackets),
            "brackets-contain" => Some(Self::BracketsContain),
            "brackets-contain-end" => Some(Self::BracketsContainEnd),
            "brackets-contain-start" => Some(Self::BracketsContainStart),
            "brackets-off" => Some(Self::BracketsOff),
            "braille" => Some(Self::Braille),
            "brain" => Some(Self::Brain),
            "bug" => Some(Self::Bug),
            "bug-off" => Some(Self::BugOff),
            "building" => Some(Self::Building),
            "building-airport" => Some(Self::BuildingAirport),
            "building-arch" => Some(Self::BuildingArch),
            "building-bank" => Some(Self::BuildingBank),
            "building-bridge" => Some(Self::BuildingBridge),
            "building-bridge-2" => Some(Self::BuildingBridge2),
            "building-broadcast-tower" => Some(Self::BuildingBroadcastTower),
            "building-burj-al-arab" => Some(Self::BuildingBurjAlArab),
            "building-carousel" => Some(Self::BuildingCarousel),
            "building-castle" => Some(Self::BuildingCastle),
            "building-church" => Some(Self::BuildingChurch),
            "building-circus" => Some(Self::BuildingCircus),
            "building-cog" => Some(Self::BuildingCog),
            "building-community" => Some(Self::BuildingCommunity),
            "building-cottage" => Some(Self::BuildingCottage),
            "building-eiffel-tower" => Some(Self::BuildingEiffelTower),
            "building-estate" => Some(Self::BuildingEstate),
            "building-factory" => Some(Self::BuildingFactory),
            "building-factory-2" => Some(Self::BuildingFactory2),
            "building-fortress" => Some(Self::BuildingFortress),
            "building-hospital" => Some(Self::BuildingHospital),
            "building-lighthouse" => Some(Self::BuildingLighthouse),
            "building-minus" => Some(Self::BuildingMinus),
            "building-monument" => Some(Self::BuildingMonument),
            "building-mosque" => Some(Self::BuildingMosque),
            "building-off" => Some(Self::BuildingOff),
            "building-pavilion" => Some(Self::BuildingPavilion),
            "building-plus" => Some(Self::BuildingPlus),
            "building-skyscraper" => Some(Self::BuildingSkyscraper),
            "building-stadium" => Some(Self::BuildingStadium),
            "building-store" => Some(Self::BuildingStore),
            "building-tunnel" => Some(Self::BuildingTunnel),
            "building-warehouse" => Some(Self::BuildingWarehouse),
            "building-wind-turbine" => Some(Self::BuildingWindTurbine),
            "buildings" => Some(Self::Buildings),
            "calendar" => Some(Self::Calendar),
            "calendar-bolt" => Some(Self::CalendarBolt),
            "calendar-cancel" => Some(Self::CalendarCancel),
            "calendar-check" => Some(Self::CalendarCheck),
            "calendar-clock" => Some(Self::CalendarClock),
            "calendar-code" => Some(Self::CalendarCode),
            "calendar-cog" => Some(Self::CalendarCog),
            "calendar-dollar" => Some(Self::CalendarDollar),
            "calendar-dot" => Some(Self::CalendarDot),
            "calendar-down" => Some(Self::CalendarDown),
            "calendar-due" => Some(Self::CalendarDue),
            "calendar-event" => Some(Self::CalendarEvent),
            "calendar-exclamation" => Some(Self::CalendarExclamation),
            "calendar-heart" => Some(Self::CalendarHeart),
            "calendar-minus" => Some(Self::CalendarMinus),
            "calendar-month" => Some(Self::CalendarMonth),
            "calendar-off" => Some(Self::CalendarOff),
            "calendar-pause" => Some(Self::CalendarPause),
            "calendar-pin" => Some(Self::CalendarPin),
            "calendar-plus" => Some(Self::CalendarPlus),
            "calendar-question" => Some(Self::CalendarQuestion),
            "calendar-repeat" => Some(Self::CalendarRepeat),
            "calendar-sad" => Some(Self::CalendarSad),
            "calendar-search" => Some(Self::CalendarSearch),
            "calendar-share" => Some(Self::CalendarShare),
            "calendar-smile" => Some(Self::CalendarSmile),
            "calendar-star" => Some(Self::CalendarStar),
            "calendar-stats" => Some(Self::CalendarStats),
            "calendar-time" => Some(Self::CalendarTime),
            "calendar-up" => Some(Self::CalendarUp),
            "calendar-user" => Some(Self::CalendarUser),
            "calendar-week" => Some(Self::CalendarWeek),
            "calendar-x" => Some(Self::CalendarX),
            "canary" => Some(Self::Canary),
            "cane" => Some(Self::Cane),
            "car-garage" => Some(Self::CarGarage),
            "ce" => Some(Self::Ce),
            "ce-off" => Some(Self::CeOff),
            "chalkboard-teacher" => Some(Self::ChalkboardTeacher),
            "check" => Some(Self::Check),
            "checkbox" => Some(Self::Checkbox),
            "checklist" => Some(Self::Checklist),
            "checks" => Some(Self::Checks),
            "checkup-list" => Some(Self::CheckupList),
            "clear-all" => Some(Self::ClearAll),
            "click" => Some(Self::Click),
            "clock" => Some(Self::Clock),
            "clock-12" => Some(Self::Clock12),
            "clock-2" => Some(Self::Clock2),
            "clock-24" => Some(Self::Clock24),
            "clock-bitcoin" => Some(Self::ClockBitcoin),
            "clock-bolt" => Some(Self::ClockBolt),
            "clock-cancel" => Some(Self::ClockCancel),
            "clock-check" => Some(Self::ClockCheck),
            "clock-code" => Some(Self::ClockCode),
            "clock-cog" => Some(Self::ClockCog),
            "clock-dollar" => Some(Self::ClockDollar),
            "clock-down" => Some(Self::ClockDown),
            "clock-edit" => Some(Self::ClockEdit),
            "clock-exclamation" => Some(Self::ClockExclamation),
            "clock-heart" => Some(Self::ClockHeart),
            "clock-hour-1" => Some(Self::ClockHour1),
            "clock-hour-10" => Some(Self::ClockHour10),
            "clock-hour-11" => Some(Self::ClockHour11),
            "clock-hour-12" => Some(Self::ClockHour12),
            "clock-hour-2" => Some(Self::ClockHour2),
            "clock-hour-3" => Some(Self::ClockHour3),
            "clock-hour-4" => Some(Self::ClockHour4),
            "clock-hour-5" => Some(Self::ClockHour5),
            "clock-hour-6" => Some(Self::ClockHour6),
            "clock-hour-7" => Some(Self::ClockHour7),
            "clock-hour-8" => Some(Self::ClockHour8),
            "clock-hour-9" => Some(Self::ClockHour9),
            "clock-minus" => Some(Self::ClockMinus),
            "clock-off" => Some(Self::ClockOff),
            "clock-pause" => Some(Self::ClockPause),
            "clock-pin" => Some(Self::ClockPin),
            "clock-play" => Some(Self::ClockPlay),
            "clock-plus" => Some(Self::ClockPlus),
            "clock-question" => Some(Self::ClockQuestion),
            "clock-record" => Some(Self::ClockRecord),
            "clock-search" => Some(Self::ClockSearch),
            "clock-share" => Some(Self::ClockShare),
            "clock-shield" => Some(Self::ClockShield),
            "clock-star" => Some(Self::ClockStar),
            "clock-stop" => Some(Self::ClockStop),
            "clock-up" => Some(Self::ClockUp),
            "clock-x" => Some(Self::ClockX),
            "cloud-bitcoin" => Some(Self::CloudBitcoin),
            "cloud-download" => Some(Self::CloudDownload),
            "cloud-lock" => Some(Self::CloudLock),
            "cloud-lock-open" => Some(Self::CloudLockOpen),
            "cloud-network" => Some(Self::CloudNetwork),
            "cloud-upload" => Some(Self::CloudUpload),
            "code-ai" => Some(Self::CodeAi),
            "code-variable" => Some(Self::CodeVariable),
            "code-variable-minus" => Some(Self::CodeVariableMinus),
            "code-variable-plus" => Some(Self::CodeVariablePlus),
            "codeblock" => Some(Self::Codeblock),
            "column-insert-left" => Some(Self::ColumnInsertLeft),
            "column-insert-right" => Some(Self::ColumnInsertRight),
            "column-remove" => Some(Self::ColumnRemove),
            "command" => Some(Self::Command),
            "command-off" => Some(Self::CommandOff),
            "confucius" => Some(Self::Confucius),
            "congruent-to" => Some(Self::CongruentTo),
            "connection" => Some(Self::Connection),
            "copyleft" => Some(Self::Copyleft),
            "copyleft-off" => Some(Self::CopyleftOff),
            "copyright" => Some(Self::Copyright),
            "copyright-off" => Some(Self::CopyrightOff),
            "creative-commons" => Some(Self::CreativeCommons),
            "creative-commons-by" => Some(Self::CreativeCommonsBy),
            "creative-commons-nc" => Some(Self::CreativeCommonsNc),
            "creative-commons-nd" => Some(Self::CreativeCommonsNd),
            "creative-commons-off" => Some(Self::CreativeCommonsOff),
            "creative-commons-sa" => Some(Self::CreativeCommonsSa),
            "creative-commons-zero" => Some(Self::CreativeCommonsZero),
            "credits" => Some(Self::Credits),
            "cross" => Some(Self::Cross),
            "cross-off" => Some(Self::CrossOff),
            "crosshair" => Some(Self::Crosshair),
            "crutches" => Some(Self::Crutches),
            "crutches-off" => Some(Self::CrutchesOff),
            "cube-3d-sphere" => Some(Self::Cube3dSphere),
            "cube-3d-sphere-off" => Some(Self::Cube3dSphereOff),
            "cube-send" => Some(Self::CubeSend),
            "cube-unfolded" => Some(Self::CubeUnfolded),
            "curly-loop" => Some(Self::CurlyLoop),
            "cursor-off" => Some(Self::CursorOff),
            "dashboard" => Some(Self::Dashboard),
            "dashboard-off" => Some(Self::DashboardOff),
            "database" => Some(Self::Database),
            "database-cog" => Some(Self::DatabaseCog),
            "database-dollar" => Some(Self::DatabaseDollar),
            "database-edit" => Some(Self::DatabaseEdit),
            "database-exclamation" => Some(Self::DatabaseExclamation),
            "database-export" => Some(Self::DatabaseExport),
            "database-heart" => Some(Self::DatabaseHeart),
            "database-import" => Some(Self::DatabaseImport),
            "database-leak" => Some(Self::DatabaseLeak),
            "database-minus" => Some(Self::DatabaseMinus),
            "database-off" => Some(Self::DatabaseOff),
            "database-plus" => Some(Self::DatabasePlus),
            "database-search" => Some(Self::DatabaseSearch),
            "database-share" => Some(Self::DatabaseShare),
            "database-smile" => Some(Self::DatabaseSmile),
            "database-star" => Some(Self::DatabaseStar),
            "database-x" => Some(Self::DatabaseX),
            "deaf" => Some(Self::Deaf),
            "decimal" => Some(Self::Decimal),
            "dental" => Some(Self::Dental),
            "dental-broken" => Some(Self::DentalBroken),
            "dental-off" => Some(Self::DentalOff),
            "deselect" => Some(Self::Deselect),
            "desk" => Some(Self::Desk),
            "details" => Some(Self::Details),
            "details-off" => Some(Self::DetailsOff),
            "device-projector" => Some(Self::DeviceProjector),
            "device-unknown" => Some(Self::DeviceUnknown),
            "direction" => Some(Self::Direction),
            "direction-arrows" => Some(Self::DirectionArrows),
            "direction-horizontal" => Some(Self::DirectionHorizontal),
            "direction-sign" => Some(Self::DirectionSign),
            "direction-sign-off" => Some(Self::DirectionSignOff),
            "disabled" => Some(Self::Disabled),
            "disabled-2" => Some(Self::Disabled2),
            "disabled-off" => Some(Self::DisabledOff),
            "divide" => Some(Self::Divide),
            "dna" => Some(Self::Dna),
            "dna-2" => Some(Self::Dna2),
            "dna-2-off" => Some(Self::Dna2Off),
            "dna-off" => Some(Self::DnaOff),
            "door" => Some(Self::Door),
            "door-enter" => Some(Self::DoorEnter),
            "door-exit" => Some(Self::DoorExit),
            "door-hanger" => Some(Self::DoorHanger),
            "door-off" => Some(Self::DoorOff),
            "dots" => Some(Self::Dots),
            "dots-circle-horizontal" => Some(Self::DotsCircleHorizontal),
            "dots-diagonal" => Some(Self::DotsDiagonal),
            "dots-diagonal-2" => Some(Self::DotsDiagonal2),
            "dots-vertical" => Some(Self::DotsVertical),
            "ear" => Some(Self::Ear),
            "ear-off" => Some(Self::EarOff),
            "ear-scan" => Some(Self::EarScan),
            "elevator" => Some(Self::Elevator),
            "elevator-off" => Some(Self::ElevatorOff),
            "emergency-bed" => Some(Self::EmergencyBed),
            "empathize" => Some(Self::Empathize),
            "empathize-off" => Some(Self::EmpathizeOff),
            "equal" => Some(Self::Equal),
            "equal-double" => Some(Self::EqualDouble),
            "equal-not" => Some(Self::EqualNot),
            "exchange" => Some(Self::Exchange),
            "exchange-off" => Some(Self::ExchangeOff),
            "exclamation-circle" => Some(Self::ExclamationCircle),
            "exclamation-mark" => Some(Self::ExclamationMark),
            "exclamation-mark-off" => Some(Self::ExclamationMarkOff),
            "explicit" => Some(Self::Explicit),
            "explicit-off" => Some(Self::ExplicitOff),
            "external-link" => Some(Self::ExternalLink),
            "external-link-off" => Some(Self::ExternalLinkOff),
            "eye" => Some(Self::Eye),
            "eye-bitcoin" => Some(Self::EyeBitcoin),
            "eye-bolt" => Some(Self::EyeBolt),
            "eye-cancel" => Some(Self::EyeCancel),
            "eye-check" => Some(Self::EyeCheck),
            "eye-closed" => Some(Self::EyeClosed),
            "eye-code" => Some(Self::EyeCode),
            "eye-cog" => Some(Self::EyeCog),
            "eye-discount" => Some(Self::EyeDiscount),
            "eye-dollar" => Some(Self::EyeDollar),
            "eye-dotted" => Some(Self::EyeDotted),
            "eye-down" => Some(Self::EyeDown),
            "eye-edit" => Some(Self::EyeEdit),
            "eye-exclamation" => Some(Self::EyeExclamation),
            "eye-heart" => Some(Self::EyeHeart),
            "eye-minus" => Some(Self::EyeMinus),
            "eye-off" => Some(Self::EyeOff),
            "eye-pause" => Some(Self::EyePause),
            "eye-pin" => Some(Self::EyePin),
            "eye-plus" => Some(Self::EyePlus),
            "eye-question" => Some(Self::EyeQuestion),
            "eye-search" => Some(Self::EyeSearch),
            "eye-share" => Some(Self::EyeShare),
            "eye-spark" => Some(Self::EyeSpark),
            "eye-star" => Some(Self::EyeStar),
            "eye-table" => Some(Self::EyeTable),
            "eye-up" => Some(Self::EyeUp),
            "eye-x" => Some(Self::EyeX),
            "eyeglass" => Some(Self::Eyeglass),
            "eyeglass-2" => Some(Self::Eyeglass2),
            "eyeglass-off" => Some(Self::EyeglassOff),
            "face-id" => Some(Self::FaceId),
            "face-id-error" => Some(Self::FaceIdError),
            "face-mask" => Some(Self::FaceMask),
            "face-mask-off" => Some(Self::FaceMaskOff),
            "fall" => Some(Self::Fall),
            "fence" => Some(Self::Fence),
            "fence-off" => Some(Self::FenceOff),
            "fidget-spinner" => Some(Self::FidgetSpinner),
            "filter" => Some(Self::Filter),
            "filter-2" => Some(Self::Filter2),
            "filter-2-bolt" => Some(Self::Filter2Bolt),
            "filter-2-cancel" => Some(Self::Filter2Cancel),
            "filter-2-check" => Some(Self::Filter2Check),
            "filter-2-code" => Some(Self::Filter2Code),
            "filter-2-cog" => Some(Self::Filter2Cog),
            "filter-2-discount" => Some(Self::Filter2Discount),
            "filter-2-dollar" => Some(Self::Filter2Dollar),
            "filter-2-down" => Some(Self::Filter2Down),
            "filter-2-edit" => Some(Self::Filter2Edit),
            "filter-2-exclamation" => Some(Self::Filter2Exclamation),
            "filter-2-minus" => Some(Self::Filter2Minus),
            "filter-2-pause" => Some(Self::Filter2Pause),
            "filter-2-pin" => Some(Self::Filter2Pin),
            "filter-2-plus" => Some(Self::Filter2Plus),
            "filter-2-question" => Some(Self::Filter2Question),
            "filter-2-search" => Some(Self::Filter2Search),
            "filter-2-share" => Some(Self::Filter2Share),
            "filter-2-spark" => Some(Self::Filter2Spark),
            "filter-2-up" => Some(Self::Filter2Up),
            "filter-2-x" => Some(Self::Filter2X),
            "filter-bolt" => Some(Self::FilterBolt),
            "filter-cancel" => Some(Self::FilterCancel),
            "filter-check" => Some(Self::FilterCheck),
            "filter-code" => Some(Self::FilterCode),
            "filter-cog" => Some(Self::FilterCog),
            "filter-discount" => Some(Self::FilterDiscount),
            "filter-dollar" => Some(Self::FilterDollar),
            "filter-down" => Some(Self::FilterDown),
            "filter-edit" => Some(Self::FilterEdit),
            "filter-exclamation" => Some(Self::FilterExclamation),
            "filter-heart" => Some(Self::FilterHeart),
            "filter-minus" => Some(Self::FilterMinus),
            "filter-off" => Some(Self::FilterOff),
            "filter-pause" => Some(Self::FilterPause),
            "filter-pin" => Some(Self::FilterPin),
            "filter-plus" => Some(Self::FilterPlus),
            "filter-question" => Some(Self::FilterQuestion),
            "filter-search" => Some(Self::FilterSearch),
            "filter-share" => Some(Self::FilterShare),
            "filter-spark" => Some(Self::FilterSpark),
            "filter-star" => Some(Self::FilterStar),
            "filter-up" => Some(Self::FilterUp),
            "filter-x" => Some(Self::FilterX),
            "filters" => Some(Self::Filters),
            "fingerprint" => Some(Self::Fingerprint),
            "fingerprint-off" => Some(Self::FingerprintOff),
            "fingerprint-scan" => Some(Self::FingerprintScan),
            "fire-extinguisher" => Some(Self::FireExtinguisher),
            "firewall-check" => Some(Self::FirewallCheck),
            "firewall-flame" => Some(Self::FirewallFlame),
            "first-aid-kit" => Some(Self::FirstAidKit),
            "first-aid-kit-off" => Some(Self::FirstAidKitOff),
            "fish-christianity" => Some(Self::FishChristianity),
            "flag-bitcoin" => Some(Self::FlagBitcoin),
            "flag-discount" => Some(Self::FlagDiscount),
            "flask" => Some(Self::Flask),
            "flask-2" => Some(Self::Flask2),
            "flask-2-off" => Some(Self::Flask2Off),
            "flask-off" => Some(Self::FlaskOff),
            "focus-centered" => Some(Self::FocusCentered),
            "foodsteps" => Some(Self::Foodsteps),
            "forbid" => Some(Self::Forbid),
            "forbid-2" => Some(Self::Forbid2),
            "free-rights" => Some(Self::FreeRights),
            "friends" => Some(Self::Friends),
            "friends-off" => Some(Self::FriendsOff),
            "function" => Some(Self::Function),
            "function-off" => Some(Self::FunctionOff),
            "galaxy" => Some(Self::Galaxy),
            "gauge" => Some(Self::Gauge),
            "gauge-off" => Some(Self::GaugeOff),
            "gavel" => Some(Self::Gavel),
            "graph" => Some(Self::Graph),
            "graph-off" => Some(Self::GraphOff),
            "grid-dots" => Some(Self::GridDots),
            "grid-pattern" => Some(Self::GridPattern),
            "grid-scan" => Some(Self::GridScan),
            "grip-horizontal" => Some(Self::GripHorizontal),
            "grip-vertical" => Some(Self::GripVertical),
            "gymnastics" => Some(Self::Gymnastics),
            "hand-sanitizer" => Some(Self::HandSanitizer),
            "hanger-2" => Some(Self::Hanger2),
            "haze-moon" => Some(Self::HazeMoon),
            "health-recognition" => Some(Self::HealthRecognition),
            "heart-bitcoin" => Some(Self::HeartBitcoin),
            "heart-broken" => Some(Self::HeartBroken),
            "heart-discount" => Some(Self::HeartDiscount),
            "heart-handshake" => Some(Self::HeartHandshake),
            "heart-rate-monitor" => Some(Self::HeartRateMonitor),
            "heartbeat" => Some(Self::Heartbeat),
            "help" => Some(Self::Help),
            "help-circle" => Some(Self::HelpCircle),
            "help-hexagon" => Some(Self::HelpHexagon),
            "help-octagon" => Some(Self::HelpOctagon),
            "help-off" => Some(Self::HelpOff),
            "help-small" => Some(Self::HelpSmall),
            "help-square" => Some(Self::HelpSquare),
            "help-square-rounded" => Some(Self::HelpSquareRounded),
            "help-triangle" => Some(Self::HelpTriangle),
            "history" => Some(Self::History),
            "history-off" => Some(Self::HistoryOff),
            "history-toggle" => Some(Self::HistoryToggle),
            "home" => Some(Self::Home),
            "home-2" => Some(Self::Home2),
            "home-bitcoin" => Some(Self::HomeBitcoin),
            "home-bolt" => Some(Self::HomeBolt),
            "home-cancel" => Some(Self::HomeCancel),
            "home-check" => Some(Self::HomeCheck),
            "home-cog" => Some(Self::HomeCog),
            "home-dollar" => Some(Self::HomeDollar),
            "home-dot" => Some(Self::HomeDot),
            "home-down" => Some(Self::HomeDown),
            "home-eco" => Some(Self::HomeEco),
            "home-edit" => Some(Self::HomeEdit),
            "home-exclamation" => Some(Self::HomeExclamation),
            "home-hand" => Some(Self::HomeHand),
            "home-heart" => Some(Self::HomeHeart),
            "home-infinity" => Some(Self::HomeInfinity),
            "home-link" => Some(Self::HomeLink),
            "home-lock" => Some(Self::HomeLock),
            "home-minus" => Some(Self::HomeMinus),
            "home-move" => Some(Self::HomeMove),
            "home-off" => Some(Self::HomeOff),
            "home-plus" => Some(Self::HomePlus),
            "home-question" => Some(Self::HomeQuestion),
            "home-ribbon" => Some(Self::HomeRibbon),
            "home-search" => Some(Self::HomeSearch),
            "home-share" => Some(Self::HomeShare),
            "home-shield" => Some(Self::HomeShield),
            "home-signal" => Some(Self::HomeSignal),
            "home-spark" => Some(Self::HomeSpark),
            "home-star" => Some(Self::HomeStar),
            "home-stats" => Some(Self::HomeStats),
            "home-up" => Some(Self::HomeUp),
            "home-x" => Some(Self::HomeX),
            "hospital" => Some(Self::Hospital),
            "hospital-circle" => Some(Self::HospitalCircle),
            "hotel-service" => Some(Self::HotelService),
            "hourglass" => Some(Self::Hourglass),
            "hourglass-empty" => Some(Self::HourglassEmpty),
            "hourglass-high" => Some(Self::HourglassHigh),
            "hourglass-low" => Some(Self::HourglassLow),
            "hourglass-off" => Some(Self::HourglassOff),
            "hours-12" => Some(Self::Hours12),
            "hours-24" => Some(Self::Hours24),
            "id" => Some(Self::Id),
            "id-badge" => Some(Self::IdBadge),
            "id-badge-2" => Some(Self::IdBadge2),
            "id-badge-off" => Some(Self::IdBadgeOff),
            "id-off" => Some(Self::IdOff),
            "image-generation" => Some(Self::ImageGeneration),
            "inbox" => Some(Self::Inbox),
            "inbox-off" => Some(Self::InboxOff),
            "infinity" => Some(Self::Infinity),
            "infinity-2" => Some(Self::Infinity2),
            "infinity-off" => Some(Self::InfinityOff),
            "info-circle" => Some(Self::InfoCircle),
            "info-hexagon" => Some(Self::InfoHexagon),
            "info-octagon" => Some(Self::InfoOctagon),
            "info-small" => Some(Self::InfoSmall),
            "info-square" => Some(Self::InfoSquare),
            "info-square-rounded" => Some(Self::InfoSquareRounded),
            "info-triangle" => Some(Self::InfoTriangle),
            "input-ai" => Some(Self::InputAi),
            "input-check" => Some(Self::InputCheck),
            "input-spark" => Some(Self::InputSpark),
            "input-x" => Some(Self::InputX),
            "jetpack" => Some(Self::Jetpack),
            "join-bevel" => Some(Self::JoinBevel),
            "join-round" => Some(Self::JoinRound),
            "join-straight" => Some(Self::JoinStraight),
            "label" => Some(Self::Label),
            "label-important" => Some(Self::LabelImportant),
            "label-off" => Some(Self::LabelOff),
            "ladder" => Some(Self::Ladder),
            "ladder-off" => Some(Self::LadderOff),
            "lane" => Some(Self::Lane),
            "layout-bottombar-inactive" => Some(Self::LayoutBottombarInactive),
            "layout-navbar-inactive" => Some(Self::LayoutNavbarInactive),
            "layout-sidebar-inactive" => Some(Self::LayoutSidebarInactive),
            "layout-sidebar-right-inactive" => Some(Self::LayoutSidebarRightInactive),
            "lego" => Some(Self::Lego),
            "lego-off" => Some(Self::LegoOff),
            "library" => Some(Self::Library),
            "library-minus" => Some(Self::LibraryMinus),
            "library-photo" => Some(Self::LibraryPhoto),
            "library-plus" => Some(Self::LibraryPlus),
            "lifebuoy" => Some(Self::Lifebuoy),
            "lifebuoy-off" => Some(Self::LifebuoyOff),
            "lighter" => Some(Self::Lighter),
            "line-scan" => Some(Self::LineScan),
            "list-letters" => Some(Self::ListLetters),
            "list-tree" => Some(Self::ListTree),
            "loader" => Some(Self::Loader),
            "loader-2" => Some(Self::Loader2),
            "loader-3" => Some(Self::Loader3),
            "loader-4" => Some(Self::Loader4),
            "loader-quarter" => Some(Self::LoaderQuarter),
            "location-discount" => Some(Self::LocationDiscount),
            "lock" => Some(Self::Lock),
            "lock-access" => Some(Self::LockAccess),
            "lock-access-off" => Some(Self::LockAccessOff),
            "lock-bitcoin" => Some(Self::LockBitcoin),
            "lock-bolt" => Some(Self::LockBolt),
            "lock-cancel" => Some(Self::LockCancel),
            "lock-check" => Some(Self::LockCheck),
            "lock-code" => Some(Self::LockCode),
            "lock-cog" => Some(Self::LockCog),
            "lock-dollar" => Some(Self::LockDollar),
            "lock-down" => Some(Self::LockDown),
            "lock-exclamation" => Some(Self::LockExclamation),
            "lock-heart" => Some(Self::LockHeart),
            "lock-minus" => Some(Self::LockMinus),
            "lock-off" => Some(Self::LockOff),
            "lock-open" => Some(Self::LockOpen),
            "lock-open-2" => Some(Self::LockOpen2),
            "lock-open-off" => Some(Self::LockOpenOff),
            "lock-password" => Some(Self::LockPassword),
            "lock-pause" => Some(Self::LockPause),
            "lock-pin" => Some(Self::LockPin),
            "lock-plus" => Some(Self::LockPlus),
            "lock-question" => Some(Self::LockQuestion),
            "lock-search" => Some(Self::LockSearch),
            "lock-share" => Some(Self::LockShare),
            "lock-square" => Some(Self::LockSquare),
            "lock-square-rounded" => Some(Self::LockSquareRounded),
            "lock-star" => Some(Self::LockStar),
            "lock-up" => Some(Self::LockUp),
            "lock-x" => Some(Self::LockX),
            "logic-and" => Some(Self::LogicAnd),
            "logic-buffer" => Some(Self::LogicBuffer),
            "logic-nand" => Some(Self::LogicNand),
            "logic-nor" => Some(Self::LogicNor),
            "logic-not" => Some(Self::LogicNot),
            "logic-or" => Some(Self::LogicOr),
            "logic-xnor" => Some(Self::LogicXnor),
            "logic-xor" => Some(Self::LogicXor),
            "logout" => Some(Self::Logout),
            "logout-2" => Some(Self::Logout2),
            "logs" => Some(Self::Logs),
            "luggage" => Some(Self::Luggage),
            "luggage-off" => Some(Self::LuggageOff),
            "lungs" => Some(Self::Lungs),
            "lungs-off" => Some(Self::LungsOff),
            "man" => Some(Self::Man),
            "marquee" => Some(Self::Marquee),
            "marquee-2" => Some(Self::Marquee2),
            "marquee-off" => Some(Self::MarqueeOff),
            "mars" => Some(Self::Mars),
            "massage" => Some(Self::Massage),
            "matchstick" => Some(Self::Matchstick),
            "math" => Some(Self::Math),
            "math-1-divide-2" => Some(Self::Math1Divide2),
            "math-1-divide-3" => Some(Self::Math1Divide3),
            "math-avg" => Some(Self::MathAvg),
            "math-cos" => Some(Self::MathCos),
            "math-ctg" => Some(Self::MathCtg),
            "math-equal-greater" => Some(Self::MathEqualGreater),
            "math-equal-lower" => Some(Self::MathEqualLower),
            "math-function" => Some(Self::MathFunction),
            "math-function-off" => Some(Self::MathFunctionOff),
            "math-function-y" => Some(Self::MathFunctionY),
            "math-greater" => Some(Self::MathGreater),
            "math-integral" => Some(Self::MathIntegral),
            "math-integral-x" => Some(Self::MathIntegralX),
            "math-integrals" => Some(Self::MathIntegrals),
            "math-lower" => Some(Self::MathLower),
            "math-max" => Some(Self::MathMax),
            "math-max-min" => Some(Self::MathMaxMin),
            "math-min" => Some(Self::MathMin),
            "math-not" => Some(Self::MathNot),
            "math-off" => Some(Self::MathOff),
            "math-pi" => Some(Self::MathPi),
            "math-pi-divide-2" => Some(Self::MathPiDivide2),
            "math-sec" => Some(Self::MathSec),
            "math-sin" => Some(Self::MathSin),
            "math-symbols" => Some(Self::MathSymbols),
            "math-tg" => Some(Self::MathTg),
            "math-x-divide-2" => Some(Self::MathXDivide2),
            "math-x-divide-y" => Some(Self::MathXDivideY),
            "math-x-divide-y-2" => Some(Self::MathXDivideY2),
            "math-x-floor-divide-y" => Some(Self::MathXFloorDivideY),
            "math-x-minus-x" => Some(Self::MathXMinusX),
            "math-x-minus-y" => Some(Self::MathXMinusY),
            "math-x-plus-x" => Some(Self::MathXPlusX),
            "math-x-plus-y" => Some(Self::MathXPlusY),
            "math-xy" => Some(Self::MathXy),
            "math-y-minus-y" => Some(Self::MathYMinusY),
            "math-y-plus-y" => Some(Self::MathYPlusY),
            "matrix" => Some(Self::Matrix),
            "medicine-syrup" => Some(Self::MedicineSyrup),
            "menorah" => Some(Self::Menorah),
            "menu" => Some(Self::Menu),
            "menu-2" => Some(Self::Menu2),
            "menu-3" => Some(Self::Menu3),
            "menu-4" => Some(Self::Menu4),
            "menu-deep" => Some(Self::MenuDeep),
            "menu-order" => Some(Self::MenuOrder),
            "meter-cube" => Some(Self::MeterCube),
            "meter-square" => Some(Self::MeterSquare),
            "metronome" => Some(Self::Metronome),
            "microfrontends" => Some(Self::Microfrontends),
            "microscope" => Some(Self::Microscope),
            "microscope-off" => Some(Self::MicroscopeOff),
            "middleware" => Some(Self::Middleware),
            "military-award" => Some(Self::MilitaryAward),
            "military-rank" => Some(Self::MilitaryRank),
            "minus" => Some(Self::Minus),
            "minus-vertical" => Some(Self::MinusVertical),
            "mobiledata" => Some(Self::Mobiledata),
            "mobiledata-off" => Some(Self::MobiledataOff),
            "mosque" => Some(Self::Mosque),
            "multiplier-0-5x" => Some(Self::Multiplier05x),
            "multiplier-1-5x" => Some(Self::Multiplier15x),
            "multiplier-1x" => Some(Self::Multiplier1x),
            "multiplier-2x" => Some(Self::Multiplier2x),
            "music-discount" => Some(Self::MusicDiscount),
            "new-section" => Some(Self::NewSection),
            "no-copyright" => Some(Self::NoCopyright),
            "no-creative-commons" => Some(Self::NoCreativeCommons),
            "no-derivatives" => Some(Self::NoDerivatives),
            "notification" => Some(Self::Notification),
            "notification-off" => Some(Self::NotificationOff),
            "nurse" => Some(Self::Nurse),
            "object-scan" => Some(Self::ObjectScan),
            "old" => Some(Self::Old),
            "om" => Some(Self::Om),
            "omega" => Some(Self::Omega),
            "option" => Some(Self::Option),
            "outbound" => Some(Self::Outbound),
            "outlet" => Some(Self::Outlet),
            "package-export" => Some(Self::PackageExport),
            "package-import" => Some(Self::PackageImport),
            "packages" => Some(Self::Packages),
            "parentheses" => Some(Self::Parentheses),
            "parentheses-off" => Some(Self::ParenthesesOff),
            "parking-circle" => Some(Self::ParkingCircle),
            "password" => Some(Self::Password),
            "password-fingerprint" => Some(Self::PasswordFingerprint),
            "password-mobile-phone" => Some(Self::PasswordMobilePhone),
            "password-user" => Some(Self::PasswordUser),
            "paywall" => Some(Self::Paywall),
            "peace" => Some(Self::Peace),
            "pendulum" => Some(Self::Pendulum),
            "percentage" => Some(Self::Percentage),
            "percentage-0" => Some(Self::Percentage0),
            "percentage-10" => Some(Self::Percentage10),
            "percentage-100" => Some(Self::Percentage100),
            "percentage-20" => Some(Self::Percentage20),
            "percentage-25" => Some(Self::Percentage25),
            "percentage-30" => Some(Self::Percentage30),
            "percentage-33" => Some(Self::Percentage33),
            "percentage-40" => Some(Self::Percentage40),
            "percentage-50" => Some(Self::Percentage50),
            "percentage-60" => Some(Self::Percentage60),
            "percentage-66" => Some(Self::Percentage66),
            "percentage-70" => Some(Self::Percentage70),
            "percentage-75" => Some(Self::Percentage75),
            "percentage-80" => Some(Self::Percentage80),
            "percentage-90" => Some(Self::Percentage90),
            "photo-scan" => Some(Self::PhotoScan),
            "physotherapist" => Some(Self::Physotherapist),
            "pill" => Some(Self::Pill),
            "pill-off" => Some(Self::PillOff),
            "pillow" => Some(Self::Pillow),
            "pills" => Some(Self::Pills),
            "pin-end" => Some(Self::PinEnd),
            "pin-invoke" => Some(Self::PinInvoke),
            "pipeline" => Some(Self::Pipeline),
            "playlist-add" => Some(Self::PlaylistAdd),
            "plunger" => Some(Self::Plunger),
            "plus" => Some(Self::Plus),
            "plus-equal" => Some(Self::PlusEqual),
            "plus-minus" => Some(Self::PlusMinus),
            "podium" => Some(Self::Podium),
            "podium-off" => Some(Self::PodiumOff),
            "point" => Some(Self::Point),
            "point-off" => Some(Self::PointOff),
            "pointer" => Some(Self::Pointer),
            "pointer-2" => Some(Self::Pointer2),
            "pointer-bolt" => Some(Self::PointerBolt),
            "pointer-cancel" => Some(Self::PointerCancel),
            "pointer-check" => Some(Self::PointerCheck),
            "pointer-code" => Some(Self::PointerCode),
            "pointer-cog" => Some(Self::PointerCog),
            "pointer-collaboration" => Some(Self::PointerCollaboration),
            "pointer-collaboration-2" => Some(Self::PointerCollaboration2),
            "pointer-dollar" => Some(Self::PointerDollar),
            "pointer-down" => Some(Self::PointerDown),
            "pointer-exclamation" => Some(Self::PointerExclamation),
            "pointer-heart" => Some(Self::PointerHeart),
            "pointer-minus" => Some(Self::PointerMinus),
            "pointer-off" => Some(Self::PointerOff),
            "pointer-pause" => Some(Self::PointerPause),
            "pointer-pin" => Some(Self::PointerPin),
            "pointer-plus" => Some(Self::PointerPlus),
            "pointer-question" => Some(Self::PointerQuestion),
            "pointer-search" => Some(Self::PointerSearch),
            "pointer-share" => Some(Self::PointerShare),
            "pointer-star" => Some(Self::PointerStar),
            "pointer-up" => Some(Self::PointerUp),
            "pointer-x" => Some(Self::PointerX),
            "poo" => Some(Self::Poo),
            "pray" => Some(Self::Pray),
            "premium-rights" => Some(Self::PremiumRights),
            "prescription" => Some(Self::Prescription),
            "prism-light" => Some(Self::PrismLight),
            "progress" => Some(Self::Progress),
            "progress-alert" => Some(Self::ProgressAlert),
            "progress-bolt" => Some(Self::ProgressBolt),
            "progress-check" => Some(Self::ProgressCheck),
            "progress-down" => Some(Self::ProgressDown),
            "progress-help" => Some(Self::ProgressHelp),
            "progress-x" => Some(Self::ProgressX),
            "prompt" => Some(Self::Prompt),
            "propeller" => Some(Self::Propeller),
            "propeller-off" => Some(Self::PropellerOff),
            "protocol" => Some(Self::Protocol),
            "question-mark" => Some(Self::QuestionMark),
            "queue-pop-in" => Some(Self::QueuePopIn),
            "queue-pop-out" => Some(Self::QueuePopOut),
            "radioactive" => Some(Self::Radioactive),
            "radioactive-off" => Some(Self::RadioactiveOff),
            "rating-12-plus" => Some(Self::Rating12Plus),
            "rating-14-plus" => Some(Self::Rating14Plus),
            "rating-16-plus" => Some(Self::Rating16Plus),
            "rating-18-plus" => Some(Self::Rating18Plus),
            "rating-21-plus" => Some(Self::Rating21Plus),
            "razor" => Some(Self::Razor),
            "razor-electric" => Some(Self::RazorElectric),
            "recharging" => Some(Self::Recharging),
            "record-mail" => Some(Self::RecordMail),
            "record-mail-off" => Some(Self::RecordMailOff),
            "recycle" => Some(Self::Recycle),
            "recycle-off" => Some(Self::RecycleOff),
            "registered" => Some(Self::Registered),
            "relation-many-to-many" => Some(Self::RelationManyToMany),
            "relation-one-to-many" => Some(Self::RelationOneToMany),
            "relation-one-to-one" => Some(Self::RelationOneToOne),
            "reorder" => Some(Self::Reorder),
            "replace" => Some(Self::Replace),
            "replace-off" => Some(Self::ReplaceOff),
            "replace-user" => Some(Self::ReplaceUser),
            "reserved-line" => Some(Self::ReservedLine),
            "restore" => Some(Self::Restore),
            "ribbon-health" => Some(Self::RibbonHealth),
            "robot-face" => Some(Self::RobotFace),
            "robot-off" => Some(Self::RobotOff),
            "rotate-3d" => Some(Self::Rotate3d),
            "route-scan" => Some(Self::RouteScan),
            "row-insert-bottom" => Some(Self::RowInsertBottom),
            "row-insert-top" => Some(Self::RowInsertTop),
            "row-remove" => Some(Self::RowRemove),
            "rv-truck" => Some(Self::RvTruck),
            "sandbox" => Some(Self::Sandbox),
            "scale" => Some(Self::Scale),
            "scale-off" => Some(Self::ScaleOff),
            "scale-outline" => Some(Self::ScaleOutline),
            "scale-outline-off" => Some(Self::ScaleOutlineOff),
            "scan" => Some(Self::Scan),
            "scan-cube" => Some(Self::ScanCube),
            "scan-eye" => Some(Self::ScanEye),
            "scan-position" => Some(Self::ScanPosition),
            "scan-traces" => Some(Self::ScanTraces),
            "schema" => Some(Self::Schema),
            "schema-off" => Some(Self::SchemaOff),
            "school-bell" => Some(Self::SchoolBell),
            "sdk" => Some(Self::Sdk),
            "search" => Some(Self::Search),
            "search-off" => Some(Self::SearchOff),
            "select-all" => Some(Self::SelectAll),
            "seo" => Some(Self::Seo),
            "serverless" => Some(Self::Serverless),
            "servicemark" => Some(Self::Servicemark),
            "settings" => Some(Self::Settings),
            "settings-2" => Some(Self::Settings2),
            "settings-ai" => Some(Self::SettingsAi),
            "settings-automation" => Some(Self::SettingsAutomation),
            "settings-bolt" => Some(Self::SettingsBolt),
            "settings-cancel" => Some(Self::SettingsCancel),
            "settings-check" => Some(Self::SettingsCheck),
            "settings-code" => Some(Self::SettingsCode),
            "settings-cog" => Some(Self::SettingsCog),
            "settings-dollar" => Some(Self::SettingsDollar),
            "settings-down" => Some(Self::SettingsDown),
            "settings-exclamation" => Some(Self::SettingsExclamation),
            "settings-heart" => Some(Self::SettingsHeart),
            "settings-minus" => Some(Self::SettingsMinus),
            "settings-off" => Some(Self::SettingsOff),
            "settings-pause" => Some(Self::SettingsPause),
            "settings-pin" => Some(Self::SettingsPin),
            "settings-plus" => Some(Self::SettingsPlus),
            "settings-question" => Some(Self::SettingsQuestion),
            "settings-search" => Some(Self::SettingsSearch),
            "settings-share" => Some(Self::SettingsShare),
            "settings-spark" => Some(Self::SettingsSpark),
            "settings-star" => Some(Self::SettingsStar),
            "settings-up" => Some(Self::SettingsUp),
            "settings-x" => Some(Self::SettingsX),
            "share" => Some(Self::Share),
            "share-off" => Some(Self::ShareOff),
            "shield" => Some(Self::Shield),
            "shield-bolt" => Some(Self::ShieldBolt),
            "shield-cancel" => Some(Self::ShieldCancel),
            "shield-check" => Some(Self::ShieldCheck),
            "shield-checkered" => Some(Self::ShieldCheckered),
            "shield-chevron" => Some(Self::ShieldChevron),
            "shield-code" => Some(Self::ShieldCode),
            "shield-cog" => Some(Self::ShieldCog),
            "shield-dollar" => Some(Self::ShieldDollar),
            "shield-down" => Some(Self::ShieldDown),
            "shield-exclamation" => Some(Self::ShieldExclamation),
            "shield-half" => Some(Self::ShieldHalf),
            "shield-heart" => Some(Self::ShieldHeart),
            "shield-lock" => Some(Self::ShieldLock),
            "shield-minus" => Some(Self::ShieldMinus),
            "shield-off" => Some(Self::ShieldOff),
            "shield-pause" => Some(Self::ShieldPause),
            "shield-pin" => Some(Self::ShieldPin),
            "shield-plus" => Some(Self::ShieldPlus),
            "shield-question" => Some(Self::ShieldQuestion),
            "shield-search" => Some(Self::ShieldSearch),
            "shield-share" => Some(Self::ShieldShare),
            "shield-star" => Some(Self::ShieldStar),
            "shield-up" => Some(Self::ShieldUp),
            "shield-x" => Some(Self::ShieldX),
            "sitemap" => Some(Self::Sitemap),
            "sitemap-off" => Some(Self::SitemapOff),
            "skew-x" => Some(Self::SkewX),
            "skew-y" => Some(Self::SkewY),
            "skull" => Some(Self::Skull),
            "slash" => Some(Self::Slash),
            "slashes" => Some(Self::Slashes),
            "smart-home" => Some(Self::SmartHome),
            "smart-home-off" => Some(Self::SmartHomeOff),
            "smoking" => Some(Self::Smoking),
            "smoking-no" => Some(Self::SmokingNo),
            "snowboarding" => Some(Self::Snowboarding),
            "social" => Some(Self::Social),
            "social-off" => Some(Self::SocialOff),
            "sofa" => Some(Self::Sofa),
            "sofa-off" => Some(Self::SofaOff),
            "solar-electricity" => Some(Self::SolarElectricity),
            "solar-panel" => Some(Self::SolarPanel),
            "solar-panel-2" => Some(Self::SolarPanel2),
            "sort-ascending-shapes" => Some(Self::SortAscendingShapes),
            "sort-ascending-small-big" => Some(Self::SortAscendingSmallBig),
            "sort-descending-shapes" => Some(Self::SortDescendingShapes),
            "sort-descending-small-big" => Some(Self::SortDescendingSmallBig),
            "sos" => Some(Self::Sos),
            "source-code" => Some(Self::SourceCode),
            "spaces" => Some(Self::Spaces),
            "sparkles" => Some(Self::Sparkles),
            "spiral" => Some(Self::Spiral),
            "spiral-off" => Some(Self::SpiralOff),
            "spray" => Some(Self::Spray),
            "spy" => Some(Self::Spy),
            "spy-off" => Some(Self::SpyOff),
            "square-f0" => Some(Self::SquareF0),
            "square-f1" => Some(Self::SquareF1),
            "square-f2" => Some(Self::SquareF2),
            "square-f3" => Some(Self::SquareF3),
            "square-f4" => Some(Self::SquareF4),
            "square-f5" => Some(Self::SquareF5),
            "square-f6" => Some(Self::SquareF6),
            "square-f7" => Some(Self::SquareF7),
            "square-f8" => Some(Self::SquareF8),
            "square-f9" => Some(Self::SquareF9),
            "square-root" => Some(Self::SquareRoot),
            "square-root-2" => Some(Self::SquareRoot2),
            "stack-back" => Some(Self::StackBack),
            "stack-backward" => Some(Self::StackBackward),
            "stack-forward" => Some(Self::StackForward),
            "stack-front" => Some(Self::StackFront),
            "stack-middle" => Some(Self::StackMiddle),
            "star" => Some(Self::Star),
            "star-half" => Some(Self::StarHalf),
            "star-off" => Some(Self::StarOff),
            "stars" => Some(Self::Stars),
            "stars-off" => Some(Self::StarsOff),
            "status-change" => Some(Self::StatusChange),
            "steam" => Some(Self::Steam),
            "stereo-glasses" => Some(Self::StereoGlasses),
            "stethoscope" => Some(Self::Stethoscope),
            "stethoscope-off" => Some(Self::StethoscopeOff),
            "stopwatch" => Some(Self::Stopwatch),
            "subtask" => Some(Self::Subtask),
            "sum" => Some(Self::Sum),
            "sum-off" => Some(Self::SumOff),
            "sunglasses" => Some(Self::Sunglasses),
            "swipe" => Some(Self::Swipe),
            "table" => Some(Self::Table),
            "table-alias" => Some(Self::TableAlias),
            "table-column" => Some(Self::TableColumn),
            "table-dashed" => Some(Self::TableDashed),
            "table-down" => Some(Self::TableDown),
            "table-export" => Some(Self::TableExport),
            "table-heart" => Some(Self::TableHeart),
            "table-import" => Some(Self::TableImport),
            "table-minus" => Some(Self::TableMinus),
            "table-off" => Some(Self::TableOff),
            "table-options" => Some(Self::TableOptions),
            "table-plus" => Some(Self::TablePlus),
            "table-row" => Some(Self::TableRow),
            "table-share" => Some(Self::TableShare),
            "table-shortcut" => Some(Self::TableShortcut),
            "table-spark" => Some(Self::TableSpark),
            "tallymark-1" => Some(Self::Tallymark1),
            "tallymark-2" => Some(Self::Tallymark2),
            "tallymark-3" => Some(Self::Tallymark3),
            "tallymark-4" => Some(Self::Tallymark4),
            "tallymarks" => Some(Self::Tallymarks),
            "telescope" => Some(Self::Telescope),
            "telescope-off" => Some(Self::TelescopeOff),
            "temperature-snow" => Some(Self::TemperatureSnow),
            "temperature-sun" => Some(Self::TemperatureSun),
            "terminal" => Some(Self::Terminal),
            "terminal-2" => Some(Self::Terminal2),
            "test-pipe" => Some(Self::TestPipe),
            "test-pipe-2" => Some(Self::TestPipe2),
            "test-pipe-off" => Some(Self::TestPipeOff),
            "text-scan-2" => Some(Self::TextScan2),
            "text-scan-ai" => Some(Self::TextScanAi),
            "texture" => Some(Self::Texture),
            "thermometer" => Some(Self::Thermometer),
            "thumb-down" => Some(Self::ThumbDown),
            "thumb-down-off" => Some(Self::ThumbDownOff),
            "thumb-up" => Some(Self::ThumbUp),
            "thumb-up-off" => Some(Self::ThumbUpOff),
            "tilde" => Some(Self::Tilde),
            "time-duration-0" => Some(Self::TimeDuration0),
            "time-duration-10" => Some(Self::TimeDuration10),
            "time-duration-15" => Some(Self::TimeDuration15),
            "time-duration-30" => Some(Self::TimeDuration30),
            "time-duration-45" => Some(Self::TimeDuration45),
            "time-duration-5" => Some(Self::TimeDuration5),
            "time-duration-60" => Some(Self::TimeDuration60),
            "time-duration-90" => Some(Self::TimeDuration90),
            "time-duration-off" => Some(Self::TimeDurationOff),
            "timeline" => Some(Self::Timeline),
            "timeline-event" => Some(Self::TimelineEvent),
            "timeline-event-exclamation" => Some(Self::TimelineEventExclamation),
            "timeline-event-minus" => Some(Self::TimelineEventMinus),
            "timeline-event-plus" => Some(Self::TimelineEventPlus),
            "timeline-event-text" => Some(Self::TimelineEventText),
            "timeline-event-x" => Some(Self::TimelineEventX),
            "timezone" => Some(Self::Timezone),
            "tip-jar" => Some(Self::TipJar),
            "tip-jar-euro" => Some(Self::TipJarEuro),
            "tip-jar-pound" => Some(Self::TipJarPound),
            "toggle-left" => Some(Self::ToggleLeft),
            "toggle-right" => Some(Self::ToggleRight),
            "toilet-paper" => Some(Self::ToiletPaper),
            "toilet-paper-off" => Some(Self::ToiletPaperOff),
            "tool" => Some(Self::Tool),
            "tooltip" => Some(Self::Tooltip),
            "torii" => Some(Self::Torii),
            "tower" => Some(Self::Tower),
            "tower-off" => Some(Self::TowerOff),
            "trademark" => Some(Self::Trademark),
            "transfer" => Some(Self::Transfer),
            "transfer-vertical" => Some(Self::TransferVertical),
            "transform" => Some(Self::Transform),
            "transform-point" => Some(Self::TransformPoint),
            "transform-point-bottom-left" => Some(Self::TransformPointBottomLeft),
            "transform-point-bottom-right" => Some(Self::TransformPointBottomRight),
            "transform-point-top-left" => Some(Self::TransformPointTopLeft),
            "transform-point-top-right" => Some(Self::TransformPointTopRight),
            "trash" => Some(Self::Trash),
            "trash-off" => Some(Self::TrashOff),
            "trash-x" => Some(Self::TrashX),
            "trowel" => Some(Self::Trowel),
            "ufo-off" => Some(Self::UfoOff),
            "uhd" => Some(Self::Uhd),
            "umbrella" => Some(Self::Umbrella),
            "umbrella-2" => Some(Self::Umbrella2),
            "umbrella-closed" => Some(Self::UmbrellaClosed),
            "umbrella-closed-2" => Some(Self::UmbrellaClosed2),
            "umbrella-off" => Some(Self::UmbrellaOff),
            "universe" => Some(Self::Universe),
            "urgent" => Some(Self::Urgent),
            "usb" => Some(Self::Usb),
            "user" => Some(Self::User),
            "user-bitcoin" => Some(Self::UserBitcoin),
            "user-bolt" => Some(Self::UserBolt),
            "user-cancel" => Some(Self::UserCancel),
            "user-check" => Some(Self::UserCheck),
            "user-circle" => Some(Self::UserCircle),
            "user-code" => Some(Self::UserCode),
            "user-cog" => Some(Self::UserCog),
            "user-dollar" => Some(Self::UserDollar),
            "user-down" => Some(Self::UserDown),
            "user-edit" => Some(Self::UserEdit),
            "user-exclamation" => Some(Self::UserExclamation),
            "user-heart" => Some(Self::UserHeart),
            "user-hexagon" => Some(Self::UserHexagon),
            "user-key" => Some(Self::UserKey),
            "user-minus" => Some(Self::UserMinus),
            "user-off" => Some(Self::UserOff),
            "user-pause" => Some(Self::UserPause),
            "user-pentagon" => Some(Self::UserPentagon),
            "user-pin" => Some(Self::UserPin),
            "user-plus" => Some(Self::UserPlus),
            "user-question" => Some(Self::UserQuestion),
            "user-scan" => Some(Self::UserScan),
            "user-search" => Some(Self::UserSearch),
            "user-share" => Some(Self::UserShare),
            "user-shield" => Some(Self::UserShield),
            "user-square" => Some(Self::UserSquare),
            "user-square-rounded" => Some(Self::UserSquareRounded),
            "user-star" => Some(Self::UserStar),
            "user-up" => Some(Self::UserUp),
            "user-x" => Some(Self::UserX),
            "users" => Some(Self::Users),
            "users-group" => Some(Self::UsersGroup),
            "users-minus" => Some(Self::UsersMinus),
            "users-plus" => Some(Self::UsersPlus),
            "vaccine" => Some(Self::Vaccine),
            "vaccine-bottle" => Some(Self::VaccineBottle),
            "vaccine-bottle-off" => Some(Self::VaccineBottleOff),
            "vaccine-off" => Some(Self::VaccineOff),
            "vacuum-cleaner" => Some(Self::VacuumCleaner),
            "variable" => Some(Self::Variable),
            "variable-minus" => Some(Self::VariableMinus),
            "variable-off" => Some(Self::VariableOff),
            "variable-plus" => Some(Self::VariablePlus),
            "venus" => Some(Self::Venus),
            "versions" => Some(Self::Versions),
            "versions-off" => Some(Self::VersionsOff),
            "view-360" => Some(Self::View360),
            "view-360-arrow" => Some(Self::View360Arrow),
            "view-360-number" => Some(Self::View360Number),
            "view-360-off" => Some(Self::View360Off),
            "viewport-short" => Some(Self::ViewportShort),
            "viewport-tall" => Some(Self::ViewportTall),
            "vip" => Some(Self::Vip),
            "vip-2" => Some(Self::Vip2),
            "vip-off" => Some(Self::VipOff),
            "virus" => Some(Self::Virus),
            "virus-off" => Some(Self::VirusOff),
            "virus-search" => Some(Self::VirusSearch),
            "vs" => Some(Self::Vs),
            "wall" => Some(Self::Wall),
            "wall-off" => Some(Self::WallOff),
            "wallpaper" => Some(Self::Wallpaper),
            "wallpaper-off" => Some(Self::WallpaperOff),
            "wand" => Some(Self::Wand),
            "wand-off" => Some(Self::WandOff),
            "wave-saw-tool" => Some(Self::WaveSawTool),
            "wave-sine" => Some(Self::WaveSine),
            "wave-square" => Some(Self::WaveSquare),
            "waves-electricity" => Some(Self::WavesElectricity),
            "webhook" => Some(Self::Webhook),
            "webhook-off" => Some(Self::WebhookOff),
            "weight" => Some(Self::Weight),
            "window" => Some(Self::Window),
            "window-maximize" => Some(Self::WindowMaximize),
            "window-minimize" => Some(Self::WindowMinimize),
            "window-off" => Some(Self::WindowOff),
            "woman" => Some(Self::Woman),
            "wood" => Some(Self::Wood),
            "x" => Some(Self::X),
            "x-mark" => Some(Self::XMark),
            "x-power-y" => Some(Self::XPowerY),
            "xd" => Some(Self::Xd),
            "xxx" => Some(Self::Xxx),
            "yin-yang" => Some(Self::YinYang),
            "zero-config" => Some(Self::ZeroConfig),
            "zoom-scan" => Some(Self::ZoomScan),
            "zzz" => Some(Self::Zzz),
            "zzz-off" => Some(Self::ZzzOff),
            _ => None,
        }
    }
}

impl TablerIconData for SystemIcon {
    fn name(&self) -> &'static str {
        match self {
            Self::AB => "a-b",
            Self::Abacus => "abacus",
            Self::AbacusOff => "abacus-off",
            Self::Accessible => "accessible",
            Self::AccessibleOff => "accessible-off",
            Self::Activity => "activity",
            Self::ActivityHeartbeat => "activity-heartbeat",
            Self::AdCircle => "ad-circle",
            Self::AdCircleOff => "ad-circle-off",
            Self::Adjustments => "adjustments",
            Self::AdjustmentsAlt => "adjustments-alt",
            Self::AdjustmentsBolt => "adjustments-bolt",
            Self::AdjustmentsCancel => "adjustments-cancel",
            Self::AdjustmentsCheck => "adjustments-check",
            Self::AdjustmentsCode => "adjustments-code",
            Self::AdjustmentsCog => "adjustments-cog",
            Self::AdjustmentsDollar => "adjustments-dollar",
            Self::AdjustmentsDown => "adjustments-down",
            Self::AdjustmentsExclamation => "adjustments-exclamation",
            Self::AdjustmentsHeart => "adjustments-heart",
            Self::AdjustmentsHorizontal => "adjustments-horizontal",
            Self::AdjustmentsMinus => "adjustments-minus",
            Self::AdjustmentsOff => "adjustments-off",
            Self::AdjustmentsPause => "adjustments-pause",
            Self::AdjustmentsPin => "adjustments-pin",
            Self::AdjustmentsPlus => "adjustments-plus",
            Self::AdjustmentsQuestion => "adjustments-question",
            Self::AdjustmentsSearch => "adjustments-search",
            Self::AdjustmentsShare => "adjustments-share",
            Self::AdjustmentsSpark => "adjustments-spark",
            Self::AdjustmentsStar => "adjustments-star",
            Self::AdjustmentsUp => "adjustments-up",
            Self::AdjustmentsX => "adjustments-x",
            Self::Affiliate => "affiliate",
            Self::Ai => "ai",
            Self::AiAgent => "ai-agent",
            Self::AiAgents => "ai-agents",
            Self::AiGateway => "ai-gateway",
            Self::Alarm => "alarm",
            Self::AlarmAverage => "alarm-average",
            Self::AlarmMinus => "alarm-minus",
            Self::AlarmOff => "alarm-off",
            Self::AlarmPlus => "alarm-plus",
            Self::AlarmSnooze => "alarm-snooze",
            Self::AlertCircle => "alert-circle",
            Self::AlertCircleOff => "alert-circle-off",
            Self::AlertHexagon => "alert-hexagon",
            Self::AlertHexagonOff => "alert-hexagon-off",
            Self::AlertOctagon => "alert-octagon",
            Self::AlertSmall => "alert-small",
            Self::AlertSmallOff => "alert-small-off",
            Self::AlertSquare => "alert-square",
            Self::AlertSquareRounded => "alert-square-rounded",
            Self::AlertSquareRoundedOff => "alert-square-rounded-off",
            Self::AlertTriangle => "alert-triangle",
            Self::AlertTriangleOff => "alert-triangle-off",
            Self::Alt => "alt",
            Self::Ampersand => "ampersand",
            Self::Analyze => "analyze",
            Self::AnalyzeOff => "analyze-off",
            Self::Ankh => "ankh",
            Self::Api => "api",
            Self::ApiApp => "api-app",
            Self::ApiAppOff => "api-app-off",
            Self::ApiBook => "api-book",
            Self::ApiOff => "api-off",
            Self::AppWindow => "app-window",
            Self::Apps => "apps",
            Self::AppsOff => "apps-off",
            Self::Armchair => "armchair",
            Self::Armchair2 => "armchair-2",
            Self::Armchair2Off => "armchair-2-off",
            Self::ArmchairOff => "armchair-off",
            Self::Assembly => "assembly",
            Self::AssemblyOff => "assembly-off",
            Self::Asset => "asset",
            Self::AugmentedReality => "augmented-reality",
            Self::AugmentedReality2 => "augmented-reality-2",
            Self::AugmentedRealityOff => "augmented-reality-off",
            Self::Auth2fa => "auth-2fa",
            Self::Automation => "automation",
            Self::BabyBottle => "baby-bottle",
            Self::BabyCarriage => "baby-carriage",
            Self::Backslash => "backslash",
            Self::Ban => "ban",
            Self::Bandage => "bandage",
            Self::BandageOff => "bandage-off",
            Self::Barcode => "barcode",
            Self::BarcodeOff => "barcode-off",
            Self::Bath => "bath",
            Self::BathOff => "bath-off",
            Self::BedFlat => "bed-flat",
            Self::Bell => "bell",
            Self::BellBolt => "bell-bolt",
            Self::BellCancel => "bell-cancel",
            Self::BellCheck => "bell-check",
            Self::BellCode => "bell-code",
            Self::BellCog => "bell-cog",
            Self::BellDollar => "bell-dollar",
            Self::BellDown => "bell-down",
            Self::BellExclamation => "bell-exclamation",
            Self::BellHeart => "bell-heart",
            Self::BellMinus => "bell-minus",
            Self::BellOff => "bell-off",
            Self::BellPause => "bell-pause",
            Self::BellPin => "bell-pin",
            Self::BellPlus => "bell-plus",
            Self::BellQuestion => "bell-question",
            Self::BellRinging => "bell-ringing",
            Self::BellRinging2 => "bell-ringing-2",
            Self::BellSchool => "bell-school",
            Self::BellSearch => "bell-search",
            Self::BellShare => "bell-share",
            Self::BellStar => "bell-star",
            Self::BellUp => "bell-up",
            Self::BellX => "bell-x",
            Self::BellZ => "bell-z",
            Self::Biohazard => "biohazard",
            Self::BiohazardOff => "biohazard-off",
            Self::Blind => "blind",
            Self::Blob => "blob",
            Self::Blocks => "blocks",
            Self::BodyScan => "body-scan",
            Self::BotId => "bot-id",
            Self::Braces => "braces",
            Self::BracesOff => "braces-off",
            Self::Brackets => "brackets",
            Self::BracketsContain => "brackets-contain",
            Self::BracketsContainEnd => "brackets-contain-end",
            Self::BracketsContainStart => "brackets-contain-start",
            Self::BracketsOff => "brackets-off",
            Self::Braille => "braille",
            Self::Brain => "brain",
            Self::Bug => "bug",
            Self::BugOff => "bug-off",
            Self::Building => "building",
            Self::BuildingAirport => "building-airport",
            Self::BuildingArch => "building-arch",
            Self::BuildingBank => "building-bank",
            Self::BuildingBridge => "building-bridge",
            Self::BuildingBridge2 => "building-bridge-2",
            Self::BuildingBroadcastTower => "building-broadcast-tower",
            Self::BuildingBurjAlArab => "building-burj-al-arab",
            Self::BuildingCarousel => "building-carousel",
            Self::BuildingCastle => "building-castle",
            Self::BuildingChurch => "building-church",
            Self::BuildingCircus => "building-circus",
            Self::BuildingCog => "building-cog",
            Self::BuildingCommunity => "building-community",
            Self::BuildingCottage => "building-cottage",
            Self::BuildingEiffelTower => "building-eiffel-tower",
            Self::BuildingEstate => "building-estate",
            Self::BuildingFactory => "building-factory",
            Self::BuildingFactory2 => "building-factory-2",
            Self::BuildingFortress => "building-fortress",
            Self::BuildingHospital => "building-hospital",
            Self::BuildingLighthouse => "building-lighthouse",
            Self::BuildingMinus => "building-minus",
            Self::BuildingMonument => "building-monument",
            Self::BuildingMosque => "building-mosque",
            Self::BuildingOff => "building-off",
            Self::BuildingPavilion => "building-pavilion",
            Self::BuildingPlus => "building-plus",
            Self::BuildingSkyscraper => "building-skyscraper",
            Self::BuildingStadium => "building-stadium",
            Self::BuildingStore => "building-store",
            Self::BuildingTunnel => "building-tunnel",
            Self::BuildingWarehouse => "building-warehouse",
            Self::BuildingWindTurbine => "building-wind-turbine",
            Self::Buildings => "buildings",
            Self::Calendar => "calendar",
            Self::CalendarBolt => "calendar-bolt",
            Self::CalendarCancel => "calendar-cancel",
            Self::CalendarCheck => "calendar-check",
            Self::CalendarClock => "calendar-clock",
            Self::CalendarCode => "calendar-code",
            Self::CalendarCog => "calendar-cog",
            Self::CalendarDollar => "calendar-dollar",
            Self::CalendarDot => "calendar-dot",
            Self::CalendarDown => "calendar-down",
            Self::CalendarDue => "calendar-due",
            Self::CalendarEvent => "calendar-event",
            Self::CalendarExclamation => "calendar-exclamation",
            Self::CalendarHeart => "calendar-heart",
            Self::CalendarMinus => "calendar-minus",
            Self::CalendarMonth => "calendar-month",
            Self::CalendarOff => "calendar-off",
            Self::CalendarPause => "calendar-pause",
            Self::CalendarPin => "calendar-pin",
            Self::CalendarPlus => "calendar-plus",
            Self::CalendarQuestion => "calendar-question",
            Self::CalendarRepeat => "calendar-repeat",
            Self::CalendarSad => "calendar-sad",
            Self::CalendarSearch => "calendar-search",
            Self::CalendarShare => "calendar-share",
            Self::CalendarSmile => "calendar-smile",
            Self::CalendarStar => "calendar-star",
            Self::CalendarStats => "calendar-stats",
            Self::CalendarTime => "calendar-time",
            Self::CalendarUp => "calendar-up",
            Self::CalendarUser => "calendar-user",
            Self::CalendarWeek => "calendar-week",
            Self::CalendarX => "calendar-x",
            Self::Canary => "canary",
            Self::Cane => "cane",
            Self::CarGarage => "car-garage",
            Self::Ce => "ce",
            Self::CeOff => "ce-off",
            Self::ChalkboardTeacher => "chalkboard-teacher",
            Self::Check => "check",
            Self::Checkbox => "checkbox",
            Self::Checklist => "checklist",
            Self::Checks => "checks",
            Self::CheckupList => "checkup-list",
            Self::ClearAll => "clear-all",
            Self::Click => "click",
            Self::Clock => "clock",
            Self::Clock12 => "clock-12",
            Self::Clock2 => "clock-2",
            Self::Clock24 => "clock-24",
            Self::ClockBitcoin => "clock-bitcoin",
            Self::ClockBolt => "clock-bolt",
            Self::ClockCancel => "clock-cancel",
            Self::ClockCheck => "clock-check",
            Self::ClockCode => "clock-code",
            Self::ClockCog => "clock-cog",
            Self::ClockDollar => "clock-dollar",
            Self::ClockDown => "clock-down",
            Self::ClockEdit => "clock-edit",
            Self::ClockExclamation => "clock-exclamation",
            Self::ClockHeart => "clock-heart",
            Self::ClockHour1 => "clock-hour-1",
            Self::ClockHour10 => "clock-hour-10",
            Self::ClockHour11 => "clock-hour-11",
            Self::ClockHour12 => "clock-hour-12",
            Self::ClockHour2 => "clock-hour-2",
            Self::ClockHour3 => "clock-hour-3",
            Self::ClockHour4 => "clock-hour-4",
            Self::ClockHour5 => "clock-hour-5",
            Self::ClockHour6 => "clock-hour-6",
            Self::ClockHour7 => "clock-hour-7",
            Self::ClockHour8 => "clock-hour-8",
            Self::ClockHour9 => "clock-hour-9",
            Self::ClockMinus => "clock-minus",
            Self::ClockOff => "clock-off",
            Self::ClockPause => "clock-pause",
            Self::ClockPin => "clock-pin",
            Self::ClockPlay => "clock-play",
            Self::ClockPlus => "clock-plus",
            Self::ClockQuestion => "clock-question",
            Self::ClockRecord => "clock-record",
            Self::ClockSearch => "clock-search",
            Self::ClockShare => "clock-share",
            Self::ClockShield => "clock-shield",
            Self::ClockStar => "clock-star",
            Self::ClockStop => "clock-stop",
            Self::ClockUp => "clock-up",
            Self::ClockX => "clock-x",
            Self::CloudBitcoin => "cloud-bitcoin",
            Self::CloudDownload => "cloud-download",
            Self::CloudLock => "cloud-lock",
            Self::CloudLockOpen => "cloud-lock-open",
            Self::CloudNetwork => "cloud-network",
            Self::CloudUpload => "cloud-upload",
            Self::CodeAi => "code-ai",
            Self::CodeVariable => "code-variable",
            Self::CodeVariableMinus => "code-variable-minus",
            Self::CodeVariablePlus => "code-variable-plus",
            Self::Codeblock => "codeblock",
            Self::ColumnInsertLeft => "column-insert-left",
            Self::ColumnInsertRight => "column-insert-right",
            Self::ColumnRemove => "column-remove",
            Self::Command => "command",
            Self::CommandOff => "command-off",
            Self::Confucius => "confucius",
            Self::CongruentTo => "congruent-to",
            Self::Connection => "connection",
            Self::Copyleft => "copyleft",
            Self::CopyleftOff => "copyleft-off",
            Self::Copyright => "copyright",
            Self::CopyrightOff => "copyright-off",
            Self::CreativeCommons => "creative-commons",
            Self::CreativeCommonsBy => "creative-commons-by",
            Self::CreativeCommonsNc => "creative-commons-nc",
            Self::CreativeCommonsNd => "creative-commons-nd",
            Self::CreativeCommonsOff => "creative-commons-off",
            Self::CreativeCommonsSa => "creative-commons-sa",
            Self::CreativeCommonsZero => "creative-commons-zero",
            Self::Credits => "credits",
            Self::Cross => "cross",
            Self::CrossOff => "cross-off",
            Self::Crosshair => "crosshair",
            Self::Crutches => "crutches",
            Self::CrutchesOff => "crutches-off",
            Self::Cube3dSphere => "cube-3d-sphere",
            Self::Cube3dSphereOff => "cube-3d-sphere-off",
            Self::CubeSend => "cube-send",
            Self::CubeUnfolded => "cube-unfolded",
            Self::CurlyLoop => "curly-loop",
            Self::CursorOff => "cursor-off",
            Self::Dashboard => "dashboard",
            Self::DashboardOff => "dashboard-off",
            Self::Database => "database",
            Self::DatabaseCog => "database-cog",
            Self::DatabaseDollar => "database-dollar",
            Self::DatabaseEdit => "database-edit",
            Self::DatabaseExclamation => "database-exclamation",
            Self::DatabaseExport => "database-export",
            Self::DatabaseHeart => "database-heart",
            Self::DatabaseImport => "database-import",
            Self::DatabaseLeak => "database-leak",
            Self::DatabaseMinus => "database-minus",
            Self::DatabaseOff => "database-off",
            Self::DatabasePlus => "database-plus",
            Self::DatabaseSearch => "database-search",
            Self::DatabaseShare => "database-share",
            Self::DatabaseSmile => "database-smile",
            Self::DatabaseStar => "database-star",
            Self::DatabaseX => "database-x",
            Self::Deaf => "deaf",
            Self::Decimal => "decimal",
            Self::Dental => "dental",
            Self::DentalBroken => "dental-broken",
            Self::DentalOff => "dental-off",
            Self::Deselect => "deselect",
            Self::Desk => "desk",
            Self::Details => "details",
            Self::DetailsOff => "details-off",
            Self::DeviceProjector => "device-projector",
            Self::DeviceUnknown => "device-unknown",
            Self::Direction => "direction",
            Self::DirectionArrows => "direction-arrows",
            Self::DirectionHorizontal => "direction-horizontal",
            Self::DirectionSign => "direction-sign",
            Self::DirectionSignOff => "direction-sign-off",
            Self::Disabled => "disabled",
            Self::Disabled2 => "disabled-2",
            Self::DisabledOff => "disabled-off",
            Self::Divide => "divide",
            Self::Dna => "dna",
            Self::Dna2 => "dna-2",
            Self::Dna2Off => "dna-2-off",
            Self::DnaOff => "dna-off",
            Self::Door => "door",
            Self::DoorEnter => "door-enter",
            Self::DoorExit => "door-exit",
            Self::DoorHanger => "door-hanger",
            Self::DoorOff => "door-off",
            Self::Dots => "dots",
            Self::DotsCircleHorizontal => "dots-circle-horizontal",
            Self::DotsDiagonal => "dots-diagonal",
            Self::DotsDiagonal2 => "dots-diagonal-2",
            Self::DotsVertical => "dots-vertical",
            Self::Ear => "ear",
            Self::EarOff => "ear-off",
            Self::EarScan => "ear-scan",
            Self::Elevator => "elevator",
            Self::ElevatorOff => "elevator-off",
            Self::EmergencyBed => "emergency-bed",
            Self::Empathize => "empathize",
            Self::EmpathizeOff => "empathize-off",
            Self::Equal => "equal",
            Self::EqualDouble => "equal-double",
            Self::EqualNot => "equal-not",
            Self::Exchange => "exchange",
            Self::ExchangeOff => "exchange-off",
            Self::ExclamationCircle => "exclamation-circle",
            Self::ExclamationMark => "exclamation-mark",
            Self::ExclamationMarkOff => "exclamation-mark-off",
            Self::Explicit => "explicit",
            Self::ExplicitOff => "explicit-off",
            Self::ExternalLink => "external-link",
            Self::ExternalLinkOff => "external-link-off",
            Self::Eye => "eye",
            Self::EyeBitcoin => "eye-bitcoin",
            Self::EyeBolt => "eye-bolt",
            Self::EyeCancel => "eye-cancel",
            Self::EyeCheck => "eye-check",
            Self::EyeClosed => "eye-closed",
            Self::EyeCode => "eye-code",
            Self::EyeCog => "eye-cog",
            Self::EyeDiscount => "eye-discount",
            Self::EyeDollar => "eye-dollar",
            Self::EyeDotted => "eye-dotted",
            Self::EyeDown => "eye-down",
            Self::EyeEdit => "eye-edit",
            Self::EyeExclamation => "eye-exclamation",
            Self::EyeHeart => "eye-heart",
            Self::EyeMinus => "eye-minus",
            Self::EyeOff => "eye-off",
            Self::EyePause => "eye-pause",
            Self::EyePin => "eye-pin",
            Self::EyePlus => "eye-plus",
            Self::EyeQuestion => "eye-question",
            Self::EyeSearch => "eye-search",
            Self::EyeShare => "eye-share",
            Self::EyeSpark => "eye-spark",
            Self::EyeStar => "eye-star",
            Self::EyeTable => "eye-table",
            Self::EyeUp => "eye-up",
            Self::EyeX => "eye-x",
            Self::Eyeglass => "eyeglass",
            Self::Eyeglass2 => "eyeglass-2",
            Self::EyeglassOff => "eyeglass-off",
            Self::FaceId => "face-id",
            Self::FaceIdError => "face-id-error",
            Self::FaceMask => "face-mask",
            Self::FaceMaskOff => "face-mask-off",
            Self::Fall => "fall",
            Self::Fence => "fence",
            Self::FenceOff => "fence-off",
            Self::FidgetSpinner => "fidget-spinner",
            Self::Filter => "filter",
            Self::Filter2 => "filter-2",
            Self::Filter2Bolt => "filter-2-bolt",
            Self::Filter2Cancel => "filter-2-cancel",
            Self::Filter2Check => "filter-2-check",
            Self::Filter2Code => "filter-2-code",
            Self::Filter2Cog => "filter-2-cog",
            Self::Filter2Discount => "filter-2-discount",
            Self::Filter2Dollar => "filter-2-dollar",
            Self::Filter2Down => "filter-2-down",
            Self::Filter2Edit => "filter-2-edit",
            Self::Filter2Exclamation => "filter-2-exclamation",
            Self::Filter2Minus => "filter-2-minus",
            Self::Filter2Pause => "filter-2-pause",
            Self::Filter2Pin => "filter-2-pin",
            Self::Filter2Plus => "filter-2-plus",
            Self::Filter2Question => "filter-2-question",
            Self::Filter2Search => "filter-2-search",
            Self::Filter2Share => "filter-2-share",
            Self::Filter2Spark => "filter-2-spark",
            Self::Filter2Up => "filter-2-up",
            Self::Filter2X => "filter-2-x",
            Self::FilterBolt => "filter-bolt",
            Self::FilterCancel => "filter-cancel",
            Self::FilterCheck => "filter-check",
            Self::FilterCode => "filter-code",
            Self::FilterCog => "filter-cog",
            Self::FilterDiscount => "filter-discount",
            Self::FilterDollar => "filter-dollar",
            Self::FilterDown => "filter-down",
            Self::FilterEdit => "filter-edit",
            Self::FilterExclamation => "filter-exclamation",
            Self::FilterHeart => "filter-heart",
            Self::FilterMinus => "filter-minus",
            Self::FilterOff => "filter-off",
            Self::FilterPause => "filter-pause",
            Self::FilterPin => "filter-pin",
            Self::FilterPlus => "filter-plus",
            Self::FilterQuestion => "filter-question",
            Self::FilterSearch => "filter-search",
            Self::FilterShare => "filter-share",
            Self::FilterSpark => "filter-spark",
            Self::FilterStar => "filter-star",
            Self::FilterUp => "filter-up",
            Self::FilterX => "filter-x",
            Self::Filters => "filters",
            Self::Fingerprint => "fingerprint",
            Self::FingerprintOff => "fingerprint-off",
            Self::FingerprintScan => "fingerprint-scan",
            Self::FireExtinguisher => "fire-extinguisher",
            Self::FirewallCheck => "firewall-check",
            Self::FirewallFlame => "firewall-flame",
            Self::FirstAidKit => "first-aid-kit",
            Self::FirstAidKitOff => "first-aid-kit-off",
            Self::FishChristianity => "fish-christianity",
            Self::FlagBitcoin => "flag-bitcoin",
            Self::FlagDiscount => "flag-discount",
            Self::Flask => "flask",
            Self::Flask2 => "flask-2",
            Self::Flask2Off => "flask-2-off",
            Self::FlaskOff => "flask-off",
            Self::FocusCentered => "focus-centered",
            Self::Foodsteps => "foodsteps",
            Self::Forbid => "forbid",
            Self::Forbid2 => "forbid-2",
            Self::FreeRights => "free-rights",
            Self::Friends => "friends",
            Self::FriendsOff => "friends-off",
            Self::Function => "function",
            Self::FunctionOff => "function-off",
            Self::Galaxy => "galaxy",
            Self::Gauge => "gauge",
            Self::GaugeOff => "gauge-off",
            Self::Gavel => "gavel",
            Self::Graph => "graph",
            Self::GraphOff => "graph-off",
            Self::GridDots => "grid-dots",
            Self::GridPattern => "grid-pattern",
            Self::GridScan => "grid-scan",
            Self::GripHorizontal => "grip-horizontal",
            Self::GripVertical => "grip-vertical",
            Self::Gymnastics => "gymnastics",
            Self::HandSanitizer => "hand-sanitizer",
            Self::Hanger2 => "hanger-2",
            Self::HazeMoon => "haze-moon",
            Self::HealthRecognition => "health-recognition",
            Self::HeartBitcoin => "heart-bitcoin",
            Self::HeartBroken => "heart-broken",
            Self::HeartDiscount => "heart-discount",
            Self::HeartHandshake => "heart-handshake",
            Self::HeartRateMonitor => "heart-rate-monitor",
            Self::Heartbeat => "heartbeat",
            Self::Help => "help",
            Self::HelpCircle => "help-circle",
            Self::HelpHexagon => "help-hexagon",
            Self::HelpOctagon => "help-octagon",
            Self::HelpOff => "help-off",
            Self::HelpSmall => "help-small",
            Self::HelpSquare => "help-square",
            Self::HelpSquareRounded => "help-square-rounded",
            Self::HelpTriangle => "help-triangle",
            Self::History => "history",
            Self::HistoryOff => "history-off",
            Self::HistoryToggle => "history-toggle",
            Self::Home => "home",
            Self::Home2 => "home-2",
            Self::HomeBitcoin => "home-bitcoin",
            Self::HomeBolt => "home-bolt",
            Self::HomeCancel => "home-cancel",
            Self::HomeCheck => "home-check",
            Self::HomeCog => "home-cog",
            Self::HomeDollar => "home-dollar",
            Self::HomeDot => "home-dot",
            Self::HomeDown => "home-down",
            Self::HomeEco => "home-eco",
            Self::HomeEdit => "home-edit",
            Self::HomeExclamation => "home-exclamation",
            Self::HomeHand => "home-hand",
            Self::HomeHeart => "home-heart",
            Self::HomeInfinity => "home-infinity",
            Self::HomeLink => "home-link",
            Self::HomeLock => "home-lock",
            Self::HomeMinus => "home-minus",
            Self::HomeMove => "home-move",
            Self::HomeOff => "home-off",
            Self::HomePlus => "home-plus",
            Self::HomeQuestion => "home-question",
            Self::HomeRibbon => "home-ribbon",
            Self::HomeSearch => "home-search",
            Self::HomeShare => "home-share",
            Self::HomeShield => "home-shield",
            Self::HomeSignal => "home-signal",
            Self::HomeSpark => "home-spark",
            Self::HomeStar => "home-star",
            Self::HomeStats => "home-stats",
            Self::HomeUp => "home-up",
            Self::HomeX => "home-x",
            Self::Hospital => "hospital",
            Self::HospitalCircle => "hospital-circle",
            Self::HotelService => "hotel-service",
            Self::Hourglass => "hourglass",
            Self::HourglassEmpty => "hourglass-empty",
            Self::HourglassHigh => "hourglass-high",
            Self::HourglassLow => "hourglass-low",
            Self::HourglassOff => "hourglass-off",
            Self::Hours12 => "hours-12",
            Self::Hours24 => "hours-24",
            Self::Id => "id",
            Self::IdBadge => "id-badge",
            Self::IdBadge2 => "id-badge-2",
            Self::IdBadgeOff => "id-badge-off",
            Self::IdOff => "id-off",
            Self::ImageGeneration => "image-generation",
            Self::Inbox => "inbox",
            Self::InboxOff => "inbox-off",
            Self::Infinity => "infinity",
            Self::Infinity2 => "infinity-2",
            Self::InfinityOff => "infinity-off",
            Self::InfoCircle => "info-circle",
            Self::InfoHexagon => "info-hexagon",
            Self::InfoOctagon => "info-octagon",
            Self::InfoSmall => "info-small",
            Self::InfoSquare => "info-square",
            Self::InfoSquareRounded => "info-square-rounded",
            Self::InfoTriangle => "info-triangle",
            Self::InputAi => "input-ai",
            Self::InputCheck => "input-check",
            Self::InputSpark => "input-spark",
            Self::InputX => "input-x",
            Self::Jetpack => "jetpack",
            Self::JoinBevel => "join-bevel",
            Self::JoinRound => "join-round",
            Self::JoinStraight => "join-straight",
            Self::Label => "label",
            Self::LabelImportant => "label-important",
            Self::LabelOff => "label-off",
            Self::Ladder => "ladder",
            Self::LadderOff => "ladder-off",
            Self::Lane => "lane",
            Self::LayoutBottombarInactive => "layout-bottombar-inactive",
            Self::LayoutNavbarInactive => "layout-navbar-inactive",
            Self::LayoutSidebarInactive => "layout-sidebar-inactive",
            Self::LayoutSidebarRightInactive => "layout-sidebar-right-inactive",
            Self::Lego => "lego",
            Self::LegoOff => "lego-off",
            Self::Library => "library",
            Self::LibraryMinus => "library-minus",
            Self::LibraryPhoto => "library-photo",
            Self::LibraryPlus => "library-plus",
            Self::Lifebuoy => "lifebuoy",
            Self::LifebuoyOff => "lifebuoy-off",
            Self::Lighter => "lighter",
            Self::LineScan => "line-scan",
            Self::ListLetters => "list-letters",
            Self::ListTree => "list-tree",
            Self::Loader => "loader",
            Self::Loader2 => "loader-2",
            Self::Loader3 => "loader-3",
            Self::Loader4 => "loader-4",
            Self::LoaderQuarter => "loader-quarter",
            Self::LocationDiscount => "location-discount",
            Self::Lock => "lock",
            Self::LockAccess => "lock-access",
            Self::LockAccessOff => "lock-access-off",
            Self::LockBitcoin => "lock-bitcoin",
            Self::LockBolt => "lock-bolt",
            Self::LockCancel => "lock-cancel",
            Self::LockCheck => "lock-check",
            Self::LockCode => "lock-code",
            Self::LockCog => "lock-cog",
            Self::LockDollar => "lock-dollar",
            Self::LockDown => "lock-down",
            Self::LockExclamation => "lock-exclamation",
            Self::LockHeart => "lock-heart",
            Self::LockMinus => "lock-minus",
            Self::LockOff => "lock-off",
            Self::LockOpen => "lock-open",
            Self::LockOpen2 => "lock-open-2",
            Self::LockOpenOff => "lock-open-off",
            Self::LockPassword => "lock-password",
            Self::LockPause => "lock-pause",
            Self::LockPin => "lock-pin",
            Self::LockPlus => "lock-plus",
            Self::LockQuestion => "lock-question",
            Self::LockSearch => "lock-search",
            Self::LockShare => "lock-share",
            Self::LockSquare => "lock-square",
            Self::LockSquareRounded => "lock-square-rounded",
            Self::LockStar => "lock-star",
            Self::LockUp => "lock-up",
            Self::LockX => "lock-x",
            Self::LogicAnd => "logic-and",
            Self::LogicBuffer => "logic-buffer",
            Self::LogicNand => "logic-nand",
            Self::LogicNor => "logic-nor",
            Self::LogicNot => "logic-not",
            Self::LogicOr => "logic-or",
            Self::LogicXnor => "logic-xnor",
            Self::LogicXor => "logic-xor",
            Self::Logout => "logout",
            Self::Logout2 => "logout-2",
            Self::Logs => "logs",
            Self::Luggage => "luggage",
            Self::LuggageOff => "luggage-off",
            Self::Lungs => "lungs",
            Self::LungsOff => "lungs-off",
            Self::Man => "man",
            Self::Marquee => "marquee",
            Self::Marquee2 => "marquee-2",
            Self::MarqueeOff => "marquee-off",
            Self::Mars => "mars",
            Self::Massage => "massage",
            Self::Matchstick => "matchstick",
            Self::Math => "math",
            Self::Math1Divide2 => "math-1-divide-2",
            Self::Math1Divide3 => "math-1-divide-3",
            Self::MathAvg => "math-avg",
            Self::MathCos => "math-cos",
            Self::MathCtg => "math-ctg",
            Self::MathEqualGreater => "math-equal-greater",
            Self::MathEqualLower => "math-equal-lower",
            Self::MathFunction => "math-function",
            Self::MathFunctionOff => "math-function-off",
            Self::MathFunctionY => "math-function-y",
            Self::MathGreater => "math-greater",
            Self::MathIntegral => "math-integral",
            Self::MathIntegralX => "math-integral-x",
            Self::MathIntegrals => "math-integrals",
            Self::MathLower => "math-lower",
            Self::MathMax => "math-max",
            Self::MathMaxMin => "math-max-min",
            Self::MathMin => "math-min",
            Self::MathNot => "math-not",
            Self::MathOff => "math-off",
            Self::MathPi => "math-pi",
            Self::MathPiDivide2 => "math-pi-divide-2",
            Self::MathSec => "math-sec",
            Self::MathSin => "math-sin",
            Self::MathSymbols => "math-symbols",
            Self::MathTg => "math-tg",
            Self::MathXDivide2 => "math-x-divide-2",
            Self::MathXDivideY => "math-x-divide-y",
            Self::MathXDivideY2 => "math-x-divide-y-2",
            Self::MathXFloorDivideY => "math-x-floor-divide-y",
            Self::MathXMinusX => "math-x-minus-x",
            Self::MathXMinusY => "math-x-minus-y",
            Self::MathXPlusX => "math-x-plus-x",
            Self::MathXPlusY => "math-x-plus-y",
            Self::MathXy => "math-xy",
            Self::MathYMinusY => "math-y-minus-y",
            Self::MathYPlusY => "math-y-plus-y",
            Self::Matrix => "matrix",
            Self::MedicineSyrup => "medicine-syrup",
            Self::Menorah => "menorah",
            Self::Menu => "menu",
            Self::Menu2 => "menu-2",
            Self::Menu3 => "menu-3",
            Self::Menu4 => "menu-4",
            Self::MenuDeep => "menu-deep",
            Self::MenuOrder => "menu-order",
            Self::MeterCube => "meter-cube",
            Self::MeterSquare => "meter-square",
            Self::Metronome => "metronome",
            Self::Microfrontends => "microfrontends",
            Self::Microscope => "microscope",
            Self::MicroscopeOff => "microscope-off",
            Self::Middleware => "middleware",
            Self::MilitaryAward => "military-award",
            Self::MilitaryRank => "military-rank",
            Self::Minus => "minus",
            Self::MinusVertical => "minus-vertical",
            Self::Mobiledata => "mobiledata",
            Self::MobiledataOff => "mobiledata-off",
            Self::Mosque => "mosque",
            Self::Multiplier05x => "multiplier-0-5x",
            Self::Multiplier15x => "multiplier-1-5x",
            Self::Multiplier1x => "multiplier-1x",
            Self::Multiplier2x => "multiplier-2x",
            Self::MusicDiscount => "music-discount",
            Self::NewSection => "new-section",
            Self::NoCopyright => "no-copyright",
            Self::NoCreativeCommons => "no-creative-commons",
            Self::NoDerivatives => "no-derivatives",
            Self::Notification => "notification",
            Self::NotificationOff => "notification-off",
            Self::Nurse => "nurse",
            Self::ObjectScan => "object-scan",
            Self::Old => "old",
            Self::Om => "om",
            Self::Omega => "omega",
            Self::Option => "option",
            Self::Outbound => "outbound",
            Self::Outlet => "outlet",
            Self::PackageExport => "package-export",
            Self::PackageImport => "package-import",
            Self::Packages => "packages",
            Self::Parentheses => "parentheses",
            Self::ParenthesesOff => "parentheses-off",
            Self::ParkingCircle => "parking-circle",
            Self::Password => "password",
            Self::PasswordFingerprint => "password-fingerprint",
            Self::PasswordMobilePhone => "password-mobile-phone",
            Self::PasswordUser => "password-user",
            Self::Paywall => "paywall",
            Self::Peace => "peace",
            Self::Pendulum => "pendulum",
            Self::Percentage => "percentage",
            Self::Percentage0 => "percentage-0",
            Self::Percentage10 => "percentage-10",
            Self::Percentage100 => "percentage-100",
            Self::Percentage20 => "percentage-20",
            Self::Percentage25 => "percentage-25",
            Self::Percentage30 => "percentage-30",
            Self::Percentage33 => "percentage-33",
            Self::Percentage40 => "percentage-40",
            Self::Percentage50 => "percentage-50",
            Self::Percentage60 => "percentage-60",
            Self::Percentage66 => "percentage-66",
            Self::Percentage70 => "percentage-70",
            Self::Percentage75 => "percentage-75",
            Self::Percentage80 => "percentage-80",
            Self::Percentage90 => "percentage-90",
            Self::PhotoScan => "photo-scan",
            Self::Physotherapist => "physotherapist",
            Self::Pill => "pill",
            Self::PillOff => "pill-off",
            Self::Pillow => "pillow",
            Self::Pills => "pills",
            Self::PinEnd => "pin-end",
            Self::PinInvoke => "pin-invoke",
            Self::Pipeline => "pipeline",
            Self::PlaylistAdd => "playlist-add",
            Self::Plunger => "plunger",
            Self::Plus => "plus",
            Self::PlusEqual => "plus-equal",
            Self::PlusMinus => "plus-minus",
            Self::Podium => "podium",
            Self::PodiumOff => "podium-off",
            Self::Point => "point",
            Self::PointOff => "point-off",
            Self::Pointer => "pointer",
            Self::Pointer2 => "pointer-2",
            Self::PointerBolt => "pointer-bolt",
            Self::PointerCancel => "pointer-cancel",
            Self::PointerCheck => "pointer-check",
            Self::PointerCode => "pointer-code",
            Self::PointerCog => "pointer-cog",
            Self::PointerCollaboration => "pointer-collaboration",
            Self::PointerCollaboration2 => "pointer-collaboration-2",
            Self::PointerDollar => "pointer-dollar",
            Self::PointerDown => "pointer-down",
            Self::PointerExclamation => "pointer-exclamation",
            Self::PointerHeart => "pointer-heart",
            Self::PointerMinus => "pointer-minus",
            Self::PointerOff => "pointer-off",
            Self::PointerPause => "pointer-pause",
            Self::PointerPin => "pointer-pin",
            Self::PointerPlus => "pointer-plus",
            Self::PointerQuestion => "pointer-question",
            Self::PointerSearch => "pointer-search",
            Self::PointerShare => "pointer-share",
            Self::PointerStar => "pointer-star",
            Self::PointerUp => "pointer-up",
            Self::PointerX => "pointer-x",
            Self::Poo => "poo",
            Self::Pray => "pray",
            Self::PremiumRights => "premium-rights",
            Self::Prescription => "prescription",
            Self::PrismLight => "prism-light",
            Self::Progress => "progress",
            Self::ProgressAlert => "progress-alert",
            Self::ProgressBolt => "progress-bolt",
            Self::ProgressCheck => "progress-check",
            Self::ProgressDown => "progress-down",
            Self::ProgressHelp => "progress-help",
            Self::ProgressX => "progress-x",
            Self::Prompt => "prompt",
            Self::Propeller => "propeller",
            Self::PropellerOff => "propeller-off",
            Self::Protocol => "protocol",
            Self::QuestionMark => "question-mark",
            Self::QueuePopIn => "queue-pop-in",
            Self::QueuePopOut => "queue-pop-out",
            Self::Radioactive => "radioactive",
            Self::RadioactiveOff => "radioactive-off",
            Self::Rating12Plus => "rating-12-plus",
            Self::Rating14Plus => "rating-14-plus",
            Self::Rating16Plus => "rating-16-plus",
            Self::Rating18Plus => "rating-18-plus",
            Self::Rating21Plus => "rating-21-plus",
            Self::Razor => "razor",
            Self::RazorElectric => "razor-electric",
            Self::Recharging => "recharging",
            Self::RecordMail => "record-mail",
            Self::RecordMailOff => "record-mail-off",
            Self::Recycle => "recycle",
            Self::RecycleOff => "recycle-off",
            Self::Registered => "registered",
            Self::RelationManyToMany => "relation-many-to-many",
            Self::RelationOneToMany => "relation-one-to-many",
            Self::RelationOneToOne => "relation-one-to-one",
            Self::Reorder => "reorder",
            Self::Replace => "replace",
            Self::ReplaceOff => "replace-off",
            Self::ReplaceUser => "replace-user",
            Self::ReservedLine => "reserved-line",
            Self::Restore => "restore",
            Self::RibbonHealth => "ribbon-health",
            Self::RobotFace => "robot-face",
            Self::RobotOff => "robot-off",
            Self::Rotate3d => "rotate-3d",
            Self::RouteScan => "route-scan",
            Self::RowInsertBottom => "row-insert-bottom",
            Self::RowInsertTop => "row-insert-top",
            Self::RowRemove => "row-remove",
            Self::RvTruck => "rv-truck",
            Self::Sandbox => "sandbox",
            Self::Scale => "scale",
            Self::ScaleOff => "scale-off",
            Self::ScaleOutline => "scale-outline",
            Self::ScaleOutlineOff => "scale-outline-off",
            Self::Scan => "scan",
            Self::ScanCube => "scan-cube",
            Self::ScanEye => "scan-eye",
            Self::ScanPosition => "scan-position",
            Self::ScanTraces => "scan-traces",
            Self::Schema => "schema",
            Self::SchemaOff => "schema-off",
            Self::SchoolBell => "school-bell",
            Self::Sdk => "sdk",
            Self::Search => "search",
            Self::SearchOff => "search-off",
            Self::SelectAll => "select-all",
            Self::Seo => "seo",
            Self::Serverless => "serverless",
            Self::Servicemark => "servicemark",
            Self::Settings => "settings",
            Self::Settings2 => "settings-2",
            Self::SettingsAi => "settings-ai",
            Self::SettingsAutomation => "settings-automation",
            Self::SettingsBolt => "settings-bolt",
            Self::SettingsCancel => "settings-cancel",
            Self::SettingsCheck => "settings-check",
            Self::SettingsCode => "settings-code",
            Self::SettingsCog => "settings-cog",
            Self::SettingsDollar => "settings-dollar",
            Self::SettingsDown => "settings-down",
            Self::SettingsExclamation => "settings-exclamation",
            Self::SettingsHeart => "settings-heart",
            Self::SettingsMinus => "settings-minus",
            Self::SettingsOff => "settings-off",
            Self::SettingsPause => "settings-pause",
            Self::SettingsPin => "settings-pin",
            Self::SettingsPlus => "settings-plus",
            Self::SettingsQuestion => "settings-question",
            Self::SettingsSearch => "settings-search",
            Self::SettingsShare => "settings-share",
            Self::SettingsSpark => "settings-spark",
            Self::SettingsStar => "settings-star",
            Self::SettingsUp => "settings-up",
            Self::SettingsX => "settings-x",
            Self::Share => "share",
            Self::ShareOff => "share-off",
            Self::Shield => "shield",
            Self::ShieldBolt => "shield-bolt",
            Self::ShieldCancel => "shield-cancel",
            Self::ShieldCheck => "shield-check",
            Self::ShieldCheckered => "shield-checkered",
            Self::ShieldChevron => "shield-chevron",
            Self::ShieldCode => "shield-code",
            Self::ShieldCog => "shield-cog",
            Self::ShieldDollar => "shield-dollar",
            Self::ShieldDown => "shield-down",
            Self::ShieldExclamation => "shield-exclamation",
            Self::ShieldHalf => "shield-half",
            Self::ShieldHeart => "shield-heart",
            Self::ShieldLock => "shield-lock",
            Self::ShieldMinus => "shield-minus",
            Self::ShieldOff => "shield-off",
            Self::ShieldPause => "shield-pause",
            Self::ShieldPin => "shield-pin",
            Self::ShieldPlus => "shield-plus",
            Self::ShieldQuestion => "shield-question",
            Self::ShieldSearch => "shield-search",
            Self::ShieldShare => "shield-share",
            Self::ShieldStar => "shield-star",
            Self::ShieldUp => "shield-up",
            Self::ShieldX => "shield-x",
            Self::Sitemap => "sitemap",
            Self::SitemapOff => "sitemap-off",
            Self::SkewX => "skew-x",
            Self::SkewY => "skew-y",
            Self::Skull => "skull",
            Self::Slash => "slash",
            Self::Slashes => "slashes",
            Self::SmartHome => "smart-home",
            Self::SmartHomeOff => "smart-home-off",
            Self::Smoking => "smoking",
            Self::SmokingNo => "smoking-no",
            Self::Snowboarding => "snowboarding",
            Self::Social => "social",
            Self::SocialOff => "social-off",
            Self::Sofa => "sofa",
            Self::SofaOff => "sofa-off",
            Self::SolarElectricity => "solar-electricity",
            Self::SolarPanel => "solar-panel",
            Self::SolarPanel2 => "solar-panel-2",
            Self::SortAscendingShapes => "sort-ascending-shapes",
            Self::SortAscendingSmallBig => "sort-ascending-small-big",
            Self::SortDescendingShapes => "sort-descending-shapes",
            Self::SortDescendingSmallBig => "sort-descending-small-big",
            Self::Sos => "sos",
            Self::SourceCode => "source-code",
            Self::Spaces => "spaces",
            Self::Sparkles => "sparkles",
            Self::Spiral => "spiral",
            Self::SpiralOff => "spiral-off",
            Self::Spray => "spray",
            Self::Spy => "spy",
            Self::SpyOff => "spy-off",
            Self::SquareF0 => "square-f0",
            Self::SquareF1 => "square-f1",
            Self::SquareF2 => "square-f2",
            Self::SquareF3 => "square-f3",
            Self::SquareF4 => "square-f4",
            Self::SquareF5 => "square-f5",
            Self::SquareF6 => "square-f6",
            Self::SquareF7 => "square-f7",
            Self::SquareF8 => "square-f8",
            Self::SquareF9 => "square-f9",
            Self::SquareRoot => "square-root",
            Self::SquareRoot2 => "square-root-2",
            Self::StackBack => "stack-back",
            Self::StackBackward => "stack-backward",
            Self::StackForward => "stack-forward",
            Self::StackFront => "stack-front",
            Self::StackMiddle => "stack-middle",
            Self::Star => "star",
            Self::StarHalf => "star-half",
            Self::StarOff => "star-off",
            Self::Stars => "stars",
            Self::StarsOff => "stars-off",
            Self::StatusChange => "status-change",
            Self::Steam => "steam",
            Self::StereoGlasses => "stereo-glasses",
            Self::Stethoscope => "stethoscope",
            Self::StethoscopeOff => "stethoscope-off",
            Self::Stopwatch => "stopwatch",
            Self::Subtask => "subtask",
            Self::Sum => "sum",
            Self::SumOff => "sum-off",
            Self::Sunglasses => "sunglasses",
            Self::Swipe => "swipe",
            Self::Table => "table",
            Self::TableAlias => "table-alias",
            Self::TableColumn => "table-column",
            Self::TableDashed => "table-dashed",
            Self::TableDown => "table-down",
            Self::TableExport => "table-export",
            Self::TableHeart => "table-heart",
            Self::TableImport => "table-import",
            Self::TableMinus => "table-minus",
            Self::TableOff => "table-off",
            Self::TableOptions => "table-options",
            Self::TablePlus => "table-plus",
            Self::TableRow => "table-row",
            Self::TableShare => "table-share",
            Self::TableShortcut => "table-shortcut",
            Self::TableSpark => "table-spark",
            Self::Tallymark1 => "tallymark-1",
            Self::Tallymark2 => "tallymark-2",
            Self::Tallymark3 => "tallymark-3",
            Self::Tallymark4 => "tallymark-4",
            Self::Tallymarks => "tallymarks",
            Self::Telescope => "telescope",
            Self::TelescopeOff => "telescope-off",
            Self::TemperatureSnow => "temperature-snow",
            Self::TemperatureSun => "temperature-sun",
            Self::Terminal => "terminal",
            Self::Terminal2 => "terminal-2",
            Self::TestPipe => "test-pipe",
            Self::TestPipe2 => "test-pipe-2",
            Self::TestPipeOff => "test-pipe-off",
            Self::TextScan2 => "text-scan-2",
            Self::TextScanAi => "text-scan-ai",
            Self::Texture => "texture",
            Self::Thermometer => "thermometer",
            Self::ThumbDown => "thumb-down",
            Self::ThumbDownOff => "thumb-down-off",
            Self::ThumbUp => "thumb-up",
            Self::ThumbUpOff => "thumb-up-off",
            Self::Tilde => "tilde",
            Self::TimeDuration0 => "time-duration-0",
            Self::TimeDuration10 => "time-duration-10",
            Self::TimeDuration15 => "time-duration-15",
            Self::TimeDuration30 => "time-duration-30",
            Self::TimeDuration45 => "time-duration-45",
            Self::TimeDuration5 => "time-duration-5",
            Self::TimeDuration60 => "time-duration-60",
            Self::TimeDuration90 => "time-duration-90",
            Self::TimeDurationOff => "time-duration-off",
            Self::Timeline => "timeline",
            Self::TimelineEvent => "timeline-event",
            Self::TimelineEventExclamation => "timeline-event-exclamation",
            Self::TimelineEventMinus => "timeline-event-minus",
            Self::TimelineEventPlus => "timeline-event-plus",
            Self::TimelineEventText => "timeline-event-text",
            Self::TimelineEventX => "timeline-event-x",
            Self::Timezone => "timezone",
            Self::TipJar => "tip-jar",
            Self::TipJarEuro => "tip-jar-euro",
            Self::TipJarPound => "tip-jar-pound",
            Self::ToggleLeft => "toggle-left",
            Self::ToggleRight => "toggle-right",
            Self::ToiletPaper => "toilet-paper",
            Self::ToiletPaperOff => "toilet-paper-off",
            Self::Tool => "tool",
            Self::Tooltip => "tooltip",
            Self::Torii => "torii",
            Self::Tower => "tower",
            Self::TowerOff => "tower-off",
            Self::Trademark => "trademark",
            Self::Transfer => "transfer",
            Self::TransferVertical => "transfer-vertical",
            Self::Transform => "transform",
            Self::TransformPoint => "transform-point",
            Self::TransformPointBottomLeft => "transform-point-bottom-left",
            Self::TransformPointBottomRight => "transform-point-bottom-right",
            Self::TransformPointTopLeft => "transform-point-top-left",
            Self::TransformPointTopRight => "transform-point-top-right",
            Self::Trash => "trash",
            Self::TrashOff => "trash-off",
            Self::TrashX => "trash-x",
            Self::Trowel => "trowel",
            Self::UfoOff => "ufo-off",
            Self::Uhd => "uhd",
            Self::Umbrella => "umbrella",
            Self::Umbrella2 => "umbrella-2",
            Self::UmbrellaClosed => "umbrella-closed",
            Self::UmbrellaClosed2 => "umbrella-closed-2",
            Self::UmbrellaOff => "umbrella-off",
            Self::Universe => "universe",
            Self::Urgent => "urgent",
            Self::Usb => "usb",
            Self::User => "user",
            Self::UserBitcoin => "user-bitcoin",
            Self::UserBolt => "user-bolt",
            Self::UserCancel => "user-cancel",
            Self::UserCheck => "user-check",
            Self::UserCircle => "user-circle",
            Self::UserCode => "user-code",
            Self::UserCog => "user-cog",
            Self::UserDollar => "user-dollar",
            Self::UserDown => "user-down",
            Self::UserEdit => "user-edit",
            Self::UserExclamation => "user-exclamation",
            Self::UserHeart => "user-heart",
            Self::UserHexagon => "user-hexagon",
            Self::UserKey => "user-key",
            Self::UserMinus => "user-minus",
            Self::UserOff => "user-off",
            Self::UserPause => "user-pause",
            Self::UserPentagon => "user-pentagon",
            Self::UserPin => "user-pin",
            Self::UserPlus => "user-plus",
            Self::UserQuestion => "user-question",
            Self::UserScan => "user-scan",
            Self::UserSearch => "user-search",
            Self::UserShare => "user-share",
            Self::UserShield => "user-shield",
            Self::UserSquare => "user-square",
            Self::UserSquareRounded => "user-square-rounded",
            Self::UserStar => "user-star",
            Self::UserUp => "user-up",
            Self::UserX => "user-x",
            Self::Users => "users",
            Self::UsersGroup => "users-group",
            Self::UsersMinus => "users-minus",
            Self::UsersPlus => "users-plus",
            Self::Vaccine => "vaccine",
            Self::VaccineBottle => "vaccine-bottle",
            Self::VaccineBottleOff => "vaccine-bottle-off",
            Self::VaccineOff => "vaccine-off",
            Self::VacuumCleaner => "vacuum-cleaner",
            Self::Variable => "variable",
            Self::VariableMinus => "variable-minus",
            Self::VariableOff => "variable-off",
            Self::VariablePlus => "variable-plus",
            Self::Venus => "venus",
            Self::Versions => "versions",
            Self::VersionsOff => "versions-off",
            Self::View360 => "view-360",
            Self::View360Arrow => "view-360-arrow",
            Self::View360Number => "view-360-number",
            Self::View360Off => "view-360-off",
            Self::ViewportShort => "viewport-short",
            Self::ViewportTall => "viewport-tall",
            Self::Vip => "vip",
            Self::Vip2 => "vip-2",
            Self::VipOff => "vip-off",
            Self::Virus => "virus",
            Self::VirusOff => "virus-off",
            Self::VirusSearch => "virus-search",
            Self::Vs => "vs",
            Self::Wall => "wall",
            Self::WallOff => "wall-off",
            Self::Wallpaper => "wallpaper",
            Self::WallpaperOff => "wallpaper-off",
            Self::Wand => "wand",
            Self::WandOff => "wand-off",
            Self::WaveSawTool => "wave-saw-tool",
            Self::WaveSine => "wave-sine",
            Self::WaveSquare => "wave-square",
            Self::WavesElectricity => "waves-electricity",
            Self::Webhook => "webhook",
            Self::WebhookOff => "webhook-off",
            Self::Weight => "weight",
            Self::Window => "window",
            Self::WindowMaximize => "window-maximize",
            Self::WindowMinimize => "window-minimize",
            Self::WindowOff => "window-off",
            Self::Woman => "woman",
            Self::Wood => "wood",
            Self::X => "x",
            Self::XMark => "x-mark",
            Self::XPowerY => "x-power-y",
            Self::Xd => "xd",
            Self::Xxx => "xxx",
            Self::YinYang => "yin-yang",
            Self::ZeroConfig => "zero-config",
            Self::ZoomScan => "zoom-scan",
            Self::Zzz => "zzz",
            Self::ZzzOff => "zzz-off",
        }
    }

    fn outline_svg(&self) -> &'static str {
        match self {
            Self::AB => A_B_SVG,
            Self::Abacus => ABACUS_SVG,
            Self::AbacusOff => ABACUS_OFF_SVG,
            Self::Accessible => ACCESSIBLE_SVG,
            Self::AccessibleOff => ACCESSIBLE_OFF_SVG,
            Self::Activity => ACTIVITY_SVG,
            Self::ActivityHeartbeat => ACTIVITY_HEARTBEAT_SVG,
            Self::AdCircle => AD_CIRCLE_SVG,
            Self::AdCircleOff => AD_CIRCLE_OFF_SVG,
            Self::Adjustments => ADJUSTMENTS_SVG,
            Self::AdjustmentsAlt => ADJUSTMENTS_ALT_SVG,
            Self::AdjustmentsBolt => ADJUSTMENTS_BOLT_SVG,
            Self::AdjustmentsCancel => ADJUSTMENTS_CANCEL_SVG,
            Self::AdjustmentsCheck => ADJUSTMENTS_CHECK_SVG,
            Self::AdjustmentsCode => ADJUSTMENTS_CODE_SVG,
            Self::AdjustmentsCog => ADJUSTMENTS_COG_SVG,
            Self::AdjustmentsDollar => ADJUSTMENTS_DOLLAR_SVG,
            Self::AdjustmentsDown => ADJUSTMENTS_DOWN_SVG,
            Self::AdjustmentsExclamation => ADJUSTMENTS_EXCLAMATION_SVG,
            Self::AdjustmentsHeart => ADJUSTMENTS_HEART_SVG,
            Self::AdjustmentsHorizontal => ADJUSTMENTS_HORIZONTAL_SVG,
            Self::AdjustmentsMinus => ADJUSTMENTS_MINUS_SVG,
            Self::AdjustmentsOff => ADJUSTMENTS_OFF_SVG,
            Self::AdjustmentsPause => ADJUSTMENTS_PAUSE_SVG,
            Self::AdjustmentsPin => ADJUSTMENTS_PIN_SVG,
            Self::AdjustmentsPlus => ADJUSTMENTS_PLUS_SVG,
            Self::AdjustmentsQuestion => ADJUSTMENTS_QUESTION_SVG,
            Self::AdjustmentsSearch => ADJUSTMENTS_SEARCH_SVG,
            Self::AdjustmentsShare => ADJUSTMENTS_SHARE_SVG,
            Self::AdjustmentsSpark => ADJUSTMENTS_SPARK_SVG,
            Self::AdjustmentsStar => ADJUSTMENTS_STAR_SVG,
            Self::AdjustmentsUp => ADJUSTMENTS_UP_SVG,
            Self::AdjustmentsX => ADJUSTMENTS_X_SVG,
            Self::Affiliate => AFFILIATE_SVG,
            Self::Ai => AI_SVG,
            Self::AiAgent => AI_AGENT_SVG,
            Self::AiAgents => AI_AGENTS_SVG,
            Self::AiGateway => AI_GATEWAY_SVG,
            Self::Alarm => ALARM_SVG,
            Self::AlarmAverage => ALARM_AVERAGE_SVG,
            Self::AlarmMinus => ALARM_MINUS_SVG,
            Self::AlarmOff => ALARM_OFF_SVG,
            Self::AlarmPlus => ALARM_PLUS_SVG,
            Self::AlarmSnooze => ALARM_SNOOZE_SVG,
            Self::AlertCircle => ALERT_CIRCLE_SVG,
            Self::AlertCircleOff => ALERT_CIRCLE_OFF_SVG,
            Self::AlertHexagon => ALERT_HEXAGON_SVG,
            Self::AlertHexagonOff => ALERT_HEXAGON_OFF_SVG,
            Self::AlertOctagon => ALERT_OCTAGON_SVG,
            Self::AlertSmall => ALERT_SMALL_SVG,
            Self::AlertSmallOff => ALERT_SMALL_OFF_SVG,
            Self::AlertSquare => ALERT_SQUARE_SVG,
            Self::AlertSquareRounded => ALERT_SQUARE_ROUNDED_SVG,
            Self::AlertSquareRoundedOff => ALERT_SQUARE_ROUNDED_OFF_SVG,
            Self::AlertTriangle => ALERT_TRIANGLE_SVG,
            Self::AlertTriangleOff => ALERT_TRIANGLE_OFF_SVG,
            Self::Alt => ALT_SVG,
            Self::Ampersand => AMPERSAND_SVG,
            Self::Analyze => ANALYZE_SVG,
            Self::AnalyzeOff => ANALYZE_OFF_SVG,
            Self::Ankh => ANKH_SVG,
            Self::Api => API_SVG,
            Self::ApiApp => API_APP_SVG,
            Self::ApiAppOff => API_APP_OFF_SVG,
            Self::ApiBook => API_BOOK_SVG,
            Self::ApiOff => API_OFF_SVG,
            Self::AppWindow => APP_WINDOW_SVG,
            Self::Apps => APPS_SVG,
            Self::AppsOff => APPS_OFF_SVG,
            Self::Armchair => ARMCHAIR_SVG,
            Self::Armchair2 => ARMCHAIR_2_SVG,
            Self::Armchair2Off => ARMCHAIR_2_OFF_SVG,
            Self::ArmchairOff => ARMCHAIR_OFF_SVG,
            Self::Assembly => ASSEMBLY_SVG,
            Self::AssemblyOff => ASSEMBLY_OFF_SVG,
            Self::Asset => ASSET_SVG,
            Self::AugmentedReality => AUGMENTED_REALITY_SVG,
            Self::AugmentedReality2 => AUGMENTED_REALITY_2_SVG,
            Self::AugmentedRealityOff => AUGMENTED_REALITY_OFF_SVG,
            Self::Auth2fa => AUTH_2FA_SVG,
            Self::Automation => AUTOMATION_SVG,
            Self::BabyBottle => BABY_BOTTLE_SVG,
            Self::BabyCarriage => BABY_CARRIAGE_SVG,
            Self::Backslash => BACKSLASH_SVG,
            Self::Ban => BAN_SVG,
            Self::Bandage => BANDAGE_SVG,
            Self::BandageOff => BANDAGE_OFF_SVG,
            Self::Barcode => BARCODE_SVG,
            Self::BarcodeOff => BARCODE_OFF_SVG,
            Self::Bath => BATH_SVG,
            Self::BathOff => BATH_OFF_SVG,
            Self::BedFlat => BED_FLAT_SVG,
            Self::Bell => BELL_SVG,
            Self::BellBolt => BELL_BOLT_SVG,
            Self::BellCancel => BELL_CANCEL_SVG,
            Self::BellCheck => BELL_CHECK_SVG,
            Self::BellCode => BELL_CODE_SVG,
            Self::BellCog => BELL_COG_SVG,
            Self::BellDollar => BELL_DOLLAR_SVG,
            Self::BellDown => BELL_DOWN_SVG,
            Self::BellExclamation => BELL_EXCLAMATION_SVG,
            Self::BellHeart => BELL_HEART_SVG,
            Self::BellMinus => BELL_MINUS_SVG,
            Self::BellOff => BELL_OFF_SVG,
            Self::BellPause => BELL_PAUSE_SVG,
            Self::BellPin => BELL_PIN_SVG,
            Self::BellPlus => BELL_PLUS_SVG,
            Self::BellQuestion => BELL_QUESTION_SVG,
            Self::BellRinging => BELL_RINGING_SVG,
            Self::BellRinging2 => BELL_RINGING_2_SVG,
            Self::BellSchool => BELL_SCHOOL_SVG,
            Self::BellSearch => BELL_SEARCH_SVG,
            Self::BellShare => BELL_SHARE_SVG,
            Self::BellStar => BELL_STAR_SVG,
            Self::BellUp => BELL_UP_SVG,
            Self::BellX => BELL_X_SVG,
            Self::BellZ => BELL_Z_SVG,
            Self::Biohazard => BIOHAZARD_SVG,
            Self::BiohazardOff => BIOHAZARD_OFF_SVG,
            Self::Blind => BLIND_SVG,
            Self::Blob => BLOB_SVG,
            Self::Blocks => BLOCKS_SVG,
            Self::BodyScan => BODY_SCAN_SVG,
            Self::BotId => BOT_ID_SVG,
            Self::Braces => BRACES_SVG,
            Self::BracesOff => BRACES_OFF_SVG,
            Self::Brackets => BRACKETS_SVG,
            Self::BracketsContain => BRACKETS_CONTAIN_SVG,
            Self::BracketsContainEnd => BRACKETS_CONTAIN_END_SVG,
            Self::BracketsContainStart => BRACKETS_CONTAIN_START_SVG,
            Self::BracketsOff => BRACKETS_OFF_SVG,
            Self::Braille => BRAILLE_SVG,
            Self::Brain => BRAIN_SVG,
            Self::Bug => BUG_SVG,
            Self::BugOff => BUG_OFF_SVG,
            Self::Building => BUILDING_SVG,
            Self::BuildingAirport => BUILDING_AIRPORT_SVG,
            Self::BuildingArch => BUILDING_ARCH_SVG,
            Self::BuildingBank => BUILDING_BANK_SVG,
            Self::BuildingBridge => BUILDING_BRIDGE_SVG,
            Self::BuildingBridge2 => BUILDING_BRIDGE_2_SVG,
            Self::BuildingBroadcastTower => BUILDING_BROADCAST_TOWER_SVG,
            Self::BuildingBurjAlArab => BUILDING_BURJ_AL_ARAB_SVG,
            Self::BuildingCarousel => BUILDING_CAROUSEL_SVG,
            Self::BuildingCastle => BUILDING_CASTLE_SVG,
            Self::BuildingChurch => BUILDING_CHURCH_SVG,
            Self::BuildingCircus => BUILDING_CIRCUS_SVG,
            Self::BuildingCog => BUILDING_COG_SVG,
            Self::BuildingCommunity => BUILDING_COMMUNITY_SVG,
            Self::BuildingCottage => BUILDING_COTTAGE_SVG,
            Self::BuildingEiffelTower => BUILDING_EIFFEL_TOWER_SVG,
            Self::BuildingEstate => BUILDING_ESTATE_SVG,
            Self::BuildingFactory => BUILDING_FACTORY_SVG,
            Self::BuildingFactory2 => BUILDING_FACTORY_2_SVG,
            Self::BuildingFortress => BUILDING_FORTRESS_SVG,
            Self::BuildingHospital => BUILDING_HOSPITAL_SVG,
            Self::BuildingLighthouse => BUILDING_LIGHTHOUSE_SVG,
            Self::BuildingMinus => BUILDING_MINUS_SVG,
            Self::BuildingMonument => BUILDING_MONUMENT_SVG,
            Self::BuildingMosque => BUILDING_MOSQUE_SVG,
            Self::BuildingOff => BUILDING_OFF_SVG,
            Self::BuildingPavilion => BUILDING_PAVILION_SVG,
            Self::BuildingPlus => BUILDING_PLUS_SVG,
            Self::BuildingSkyscraper => BUILDING_SKYSCRAPER_SVG,
            Self::BuildingStadium => BUILDING_STADIUM_SVG,
            Self::BuildingStore => BUILDING_STORE_SVG,
            Self::BuildingTunnel => BUILDING_TUNNEL_SVG,
            Self::BuildingWarehouse => BUILDING_WAREHOUSE_SVG,
            Self::BuildingWindTurbine => BUILDING_WIND_TURBINE_SVG,
            Self::Buildings => BUILDINGS_SVG,
            Self::Calendar => CALENDAR_SVG,
            Self::CalendarBolt => CALENDAR_BOLT_SVG,
            Self::CalendarCancel => CALENDAR_CANCEL_SVG,
            Self::CalendarCheck => CALENDAR_CHECK_SVG,
            Self::CalendarClock => CALENDAR_CLOCK_SVG,
            Self::CalendarCode => CALENDAR_CODE_SVG,
            Self::CalendarCog => CALENDAR_COG_SVG,
            Self::CalendarDollar => CALENDAR_DOLLAR_SVG,
            Self::CalendarDot => CALENDAR_DOT_SVG,
            Self::CalendarDown => CALENDAR_DOWN_SVG,
            Self::CalendarDue => CALENDAR_DUE_SVG,
            Self::CalendarEvent => CALENDAR_EVENT_SVG,
            Self::CalendarExclamation => CALENDAR_EXCLAMATION_SVG,
            Self::CalendarHeart => CALENDAR_HEART_SVG,
            Self::CalendarMinus => CALENDAR_MINUS_SVG,
            Self::CalendarMonth => CALENDAR_MONTH_SVG,
            Self::CalendarOff => CALENDAR_OFF_SVG,
            Self::CalendarPause => CALENDAR_PAUSE_SVG,
            Self::CalendarPin => CALENDAR_PIN_SVG,
            Self::CalendarPlus => CALENDAR_PLUS_SVG,
            Self::CalendarQuestion => CALENDAR_QUESTION_SVG,
            Self::CalendarRepeat => CALENDAR_REPEAT_SVG,
            Self::CalendarSad => CALENDAR_SAD_SVG,
            Self::CalendarSearch => CALENDAR_SEARCH_SVG,
            Self::CalendarShare => CALENDAR_SHARE_SVG,
            Self::CalendarSmile => CALENDAR_SMILE_SVG,
            Self::CalendarStar => CALENDAR_STAR_SVG,
            Self::CalendarStats => CALENDAR_STATS_SVG,
            Self::CalendarTime => CALENDAR_TIME_SVG,
            Self::CalendarUp => CALENDAR_UP_SVG,
            Self::CalendarUser => CALENDAR_USER_SVG,
            Self::CalendarWeek => CALENDAR_WEEK_SVG,
            Self::CalendarX => CALENDAR_X_SVG,
            Self::Canary => CANARY_SVG,
            Self::Cane => CANE_SVG,
            Self::CarGarage => CAR_GARAGE_SVG,
            Self::Ce => CE_SVG,
            Self::CeOff => CE_OFF_SVG,
            Self::ChalkboardTeacher => CHALKBOARD_TEACHER_SVG,
            Self::Check => CHECK_SVG,
            Self::Checkbox => CHECKBOX_SVG,
            Self::Checklist => CHECKLIST_SVG,
            Self::Checks => CHECKS_SVG,
            Self::CheckupList => CHECKUP_LIST_SVG,
            Self::ClearAll => CLEAR_ALL_SVG,
            Self::Click => CLICK_SVG,
            Self::Clock => CLOCK_SVG,
            Self::Clock12 => CLOCK_12_SVG,
            Self::Clock2 => CLOCK_2_SVG,
            Self::Clock24 => CLOCK_24_SVG,
            Self::ClockBitcoin => CLOCK_BITCOIN_SVG,
            Self::ClockBolt => CLOCK_BOLT_SVG,
            Self::ClockCancel => CLOCK_CANCEL_SVG,
            Self::ClockCheck => CLOCK_CHECK_SVG,
            Self::ClockCode => CLOCK_CODE_SVG,
            Self::ClockCog => CLOCK_COG_SVG,
            Self::ClockDollar => CLOCK_DOLLAR_SVG,
            Self::ClockDown => CLOCK_DOWN_SVG,
            Self::ClockEdit => CLOCK_EDIT_SVG,
            Self::ClockExclamation => CLOCK_EXCLAMATION_SVG,
            Self::ClockHeart => CLOCK_HEART_SVG,
            Self::ClockHour1 => CLOCK_HOUR_1_SVG,
            Self::ClockHour10 => CLOCK_HOUR_10_SVG,
            Self::ClockHour11 => CLOCK_HOUR_11_SVG,
            Self::ClockHour12 => CLOCK_HOUR_12_SVG,
            Self::ClockHour2 => CLOCK_HOUR_2_SVG,
            Self::ClockHour3 => CLOCK_HOUR_3_SVG,
            Self::ClockHour4 => CLOCK_HOUR_4_SVG,
            Self::ClockHour5 => CLOCK_HOUR_5_SVG,
            Self::ClockHour6 => CLOCK_HOUR_6_SVG,
            Self::ClockHour7 => CLOCK_HOUR_7_SVG,
            Self::ClockHour8 => CLOCK_HOUR_8_SVG,
            Self::ClockHour9 => CLOCK_HOUR_9_SVG,
            Self::ClockMinus => CLOCK_MINUS_SVG,
            Self::ClockOff => CLOCK_OFF_SVG,
            Self::ClockPause => CLOCK_PAUSE_SVG,
            Self::ClockPin => CLOCK_PIN_SVG,
            Self::ClockPlay => CLOCK_PLAY_SVG,
            Self::ClockPlus => CLOCK_PLUS_SVG,
            Self::ClockQuestion => CLOCK_QUESTION_SVG,
            Self::ClockRecord => CLOCK_RECORD_SVG,
            Self::ClockSearch => CLOCK_SEARCH_SVG,
            Self::ClockShare => CLOCK_SHARE_SVG,
            Self::ClockShield => CLOCK_SHIELD_SVG,
            Self::ClockStar => CLOCK_STAR_SVG,
            Self::ClockStop => CLOCK_STOP_SVG,
            Self::ClockUp => CLOCK_UP_SVG,
            Self::ClockX => CLOCK_X_SVG,
            Self::CloudBitcoin => CLOUD_BITCOIN_SVG,
            Self::CloudDownload => CLOUD_DOWNLOAD_SVG,
            Self::CloudLock => CLOUD_LOCK_SVG,
            Self::CloudLockOpen => CLOUD_LOCK_OPEN_SVG,
            Self::CloudNetwork => CLOUD_NETWORK_SVG,
            Self::CloudUpload => CLOUD_UPLOAD_SVG,
            Self::CodeAi => CODE_AI_SVG,
            Self::CodeVariable => CODE_VARIABLE_SVG,
            Self::CodeVariableMinus => CODE_VARIABLE_MINUS_SVG,
            Self::CodeVariablePlus => CODE_VARIABLE_PLUS_SVG,
            Self::Codeblock => CODEBLOCK_SVG,
            Self::ColumnInsertLeft => COLUMN_INSERT_LEFT_SVG,
            Self::ColumnInsertRight => COLUMN_INSERT_RIGHT_SVG,
            Self::ColumnRemove => COLUMN_REMOVE_SVG,
            Self::Command => COMMAND_SVG,
            Self::CommandOff => COMMAND_OFF_SVG,
            Self::Confucius => CONFUCIUS_SVG,
            Self::CongruentTo => CONGRUENT_TO_SVG,
            Self::Connection => CONNECTION_SVG,
            Self::Copyleft => COPYLEFT_SVG,
            Self::CopyleftOff => COPYLEFT_OFF_SVG,
            Self::Copyright => COPYRIGHT_SVG,
            Self::CopyrightOff => COPYRIGHT_OFF_SVG,
            Self::CreativeCommons => CREATIVE_COMMONS_SVG,
            Self::CreativeCommonsBy => CREATIVE_COMMONS_BY_SVG,
            Self::CreativeCommonsNc => CREATIVE_COMMONS_NC_SVG,
            Self::CreativeCommonsNd => CREATIVE_COMMONS_ND_SVG,
            Self::CreativeCommonsOff => CREATIVE_COMMONS_OFF_SVG,
            Self::CreativeCommonsSa => CREATIVE_COMMONS_SA_SVG,
            Self::CreativeCommonsZero => CREATIVE_COMMONS_ZERO_SVG,
            Self::Credits => CREDITS_SVG,
            Self::Cross => CROSS_SVG,
            Self::CrossOff => CROSS_OFF_SVG,
            Self::Crosshair => CROSSHAIR_SVG,
            Self::Crutches => CRUTCHES_SVG,
            Self::CrutchesOff => CRUTCHES_OFF_SVG,
            Self::Cube3dSphere => CUBE_3D_SPHERE_SVG,
            Self::Cube3dSphereOff => CUBE_3D_SPHERE_OFF_SVG,
            Self::CubeSend => CUBE_SEND_SVG,
            Self::CubeUnfolded => CUBE_UNFOLDED_SVG,
            Self::CurlyLoop => CURLY_LOOP_SVG,
            Self::CursorOff => CURSOR_OFF_SVG,
            Self::Dashboard => DASHBOARD_SVG,
            Self::DashboardOff => DASHBOARD_OFF_SVG,
            Self::Database => DATABASE_SVG,
            Self::DatabaseCog => DATABASE_COG_SVG,
            Self::DatabaseDollar => DATABASE_DOLLAR_SVG,
            Self::DatabaseEdit => DATABASE_EDIT_SVG,
            Self::DatabaseExclamation => DATABASE_EXCLAMATION_SVG,
            Self::DatabaseExport => DATABASE_EXPORT_SVG,
            Self::DatabaseHeart => DATABASE_HEART_SVG,
            Self::DatabaseImport => DATABASE_IMPORT_SVG,
            Self::DatabaseLeak => DATABASE_LEAK_SVG,
            Self::DatabaseMinus => DATABASE_MINUS_SVG,
            Self::DatabaseOff => DATABASE_OFF_SVG,
            Self::DatabasePlus => DATABASE_PLUS_SVG,
            Self::DatabaseSearch => DATABASE_SEARCH_SVG,
            Self::DatabaseShare => DATABASE_SHARE_SVG,
            Self::DatabaseSmile => DATABASE_SMILE_SVG,
            Self::DatabaseStar => DATABASE_STAR_SVG,
            Self::DatabaseX => DATABASE_X_SVG,
            Self::Deaf => DEAF_SVG,
            Self::Decimal => DECIMAL_SVG,
            Self::Dental => DENTAL_SVG,
            Self::DentalBroken => DENTAL_BROKEN_SVG,
            Self::DentalOff => DENTAL_OFF_SVG,
            Self::Deselect => DESELECT_SVG,
            Self::Desk => DESK_SVG,
            Self::Details => DETAILS_SVG,
            Self::DetailsOff => DETAILS_OFF_SVG,
            Self::DeviceProjector => DEVICE_PROJECTOR_SVG,
            Self::DeviceUnknown => DEVICE_UNKNOWN_SVG,
            Self::Direction => DIRECTION_SVG,
            Self::DirectionArrows => DIRECTION_ARROWS_SVG,
            Self::DirectionHorizontal => DIRECTION_HORIZONTAL_SVG,
            Self::DirectionSign => DIRECTION_SIGN_SVG,
            Self::DirectionSignOff => DIRECTION_SIGN_OFF_SVG,
            Self::Disabled => DISABLED_SVG,
            Self::Disabled2 => DISABLED_2_SVG,
            Self::DisabledOff => DISABLED_OFF_SVG,
            Self::Divide => DIVIDE_SVG,
            Self::Dna => DNA_SVG,
            Self::Dna2 => DNA_2_SVG,
            Self::Dna2Off => DNA_2_OFF_SVG,
            Self::DnaOff => DNA_OFF_SVG,
            Self::Door => DOOR_SVG,
            Self::DoorEnter => DOOR_ENTER_SVG,
            Self::DoorExit => DOOR_EXIT_SVG,
            Self::DoorHanger => DOOR_HANGER_SVG,
            Self::DoorOff => DOOR_OFF_SVG,
            Self::Dots => DOTS_SVG,
            Self::DotsCircleHorizontal => DOTS_CIRCLE_HORIZONTAL_SVG,
            Self::DotsDiagonal => DOTS_DIAGONAL_SVG,
            Self::DotsDiagonal2 => DOTS_DIAGONAL_2_SVG,
            Self::DotsVertical => DOTS_VERTICAL_SVG,
            Self::Ear => EAR_SVG,
            Self::EarOff => EAR_OFF_SVG,
            Self::EarScan => EAR_SCAN_SVG,
            Self::Elevator => ELEVATOR_SVG,
            Self::ElevatorOff => ELEVATOR_OFF_SVG,
            Self::EmergencyBed => EMERGENCY_BED_SVG,
            Self::Empathize => EMPATHIZE_SVG,
            Self::EmpathizeOff => EMPATHIZE_OFF_SVG,
            Self::Equal => EQUAL_SVG,
            Self::EqualDouble => EQUAL_DOUBLE_SVG,
            Self::EqualNot => EQUAL_NOT_SVG,
            Self::Exchange => EXCHANGE_SVG,
            Self::ExchangeOff => EXCHANGE_OFF_SVG,
            Self::ExclamationCircle => EXCLAMATION_CIRCLE_SVG,
            Self::ExclamationMark => EXCLAMATION_MARK_SVG,
            Self::ExclamationMarkOff => EXCLAMATION_MARK_OFF_SVG,
            Self::Explicit => EXPLICIT_SVG,
            Self::ExplicitOff => EXPLICIT_OFF_SVG,
            Self::ExternalLink => EXTERNAL_LINK_SVG,
            Self::ExternalLinkOff => EXTERNAL_LINK_OFF_SVG,
            Self::Eye => EYE_SVG,
            Self::EyeBitcoin => EYE_BITCOIN_SVG,
            Self::EyeBolt => EYE_BOLT_SVG,
            Self::EyeCancel => EYE_CANCEL_SVG,
            Self::EyeCheck => EYE_CHECK_SVG,
            Self::EyeClosed => EYE_CLOSED_SVG,
            Self::EyeCode => EYE_CODE_SVG,
            Self::EyeCog => EYE_COG_SVG,
            Self::EyeDiscount => EYE_DISCOUNT_SVG,
            Self::EyeDollar => EYE_DOLLAR_SVG,
            Self::EyeDotted => EYE_DOTTED_SVG,
            Self::EyeDown => EYE_DOWN_SVG,
            Self::EyeEdit => EYE_EDIT_SVG,
            Self::EyeExclamation => EYE_EXCLAMATION_SVG,
            Self::EyeHeart => EYE_HEART_SVG,
            Self::EyeMinus => EYE_MINUS_SVG,
            Self::EyeOff => EYE_OFF_SVG,
            Self::EyePause => EYE_PAUSE_SVG,
            Self::EyePin => EYE_PIN_SVG,
            Self::EyePlus => EYE_PLUS_SVG,
            Self::EyeQuestion => EYE_QUESTION_SVG,
            Self::EyeSearch => EYE_SEARCH_SVG,
            Self::EyeShare => EYE_SHARE_SVG,
            Self::EyeSpark => EYE_SPARK_SVG,
            Self::EyeStar => EYE_STAR_SVG,
            Self::EyeTable => EYE_TABLE_SVG,
            Self::EyeUp => EYE_UP_SVG,
            Self::EyeX => EYE_X_SVG,
            Self::Eyeglass => EYEGLASS_SVG,
            Self::Eyeglass2 => EYEGLASS_2_SVG,
            Self::EyeglassOff => EYEGLASS_OFF_SVG,
            Self::FaceId => FACE_ID_SVG,
            Self::FaceIdError => FACE_ID_ERROR_SVG,
            Self::FaceMask => FACE_MASK_SVG,
            Self::FaceMaskOff => FACE_MASK_OFF_SVG,
            Self::Fall => FALL_SVG,
            Self::Fence => FENCE_SVG,
            Self::FenceOff => FENCE_OFF_SVG,
            Self::FidgetSpinner => FIDGET_SPINNER_SVG,
            Self::Filter => FILTER_SVG,
            Self::Filter2 => FILTER_2_SVG,
            Self::Filter2Bolt => FILTER_2_BOLT_SVG,
            Self::Filter2Cancel => FILTER_2_CANCEL_SVG,
            Self::Filter2Check => FILTER_2_CHECK_SVG,
            Self::Filter2Code => FILTER_2_CODE_SVG,
            Self::Filter2Cog => FILTER_2_COG_SVG,
            Self::Filter2Discount => FILTER_2_DISCOUNT_SVG,
            Self::Filter2Dollar => FILTER_2_DOLLAR_SVG,
            Self::Filter2Down => FILTER_2_DOWN_SVG,
            Self::Filter2Edit => FILTER_2_EDIT_SVG,
            Self::Filter2Exclamation => FILTER_2_EXCLAMATION_SVG,
            Self::Filter2Minus => FILTER_2_MINUS_SVG,
            Self::Filter2Pause => FILTER_2_PAUSE_SVG,
            Self::Filter2Pin => FILTER_2_PIN_SVG,
            Self::Filter2Plus => FILTER_2_PLUS_SVG,
            Self::Filter2Question => FILTER_2_QUESTION_SVG,
            Self::Filter2Search => FILTER_2_SEARCH_SVG,
            Self::Filter2Share => FILTER_2_SHARE_SVG,
            Self::Filter2Spark => FILTER_2_SPARK_SVG,
            Self::Filter2Up => FILTER_2_UP_SVG,
            Self::Filter2X => FILTER_2_X_SVG,
            Self::FilterBolt => FILTER_BOLT_SVG,
            Self::FilterCancel => FILTER_CANCEL_SVG,
            Self::FilterCheck => FILTER_CHECK_SVG,
            Self::FilterCode => FILTER_CODE_SVG,
            Self::FilterCog => FILTER_COG_SVG,
            Self::FilterDiscount => FILTER_DISCOUNT_SVG,
            Self::FilterDollar => FILTER_DOLLAR_SVG,
            Self::FilterDown => FILTER_DOWN_SVG,
            Self::FilterEdit => FILTER_EDIT_SVG,
            Self::FilterExclamation => FILTER_EXCLAMATION_SVG,
            Self::FilterHeart => FILTER_HEART_SVG,
            Self::FilterMinus => FILTER_MINUS_SVG,
            Self::FilterOff => FILTER_OFF_SVG,
            Self::FilterPause => FILTER_PAUSE_SVG,
            Self::FilterPin => FILTER_PIN_SVG,
            Self::FilterPlus => FILTER_PLUS_SVG,
            Self::FilterQuestion => FILTER_QUESTION_SVG,
            Self::FilterSearch => FILTER_SEARCH_SVG,
            Self::FilterShare => FILTER_SHARE_SVG,
            Self::FilterSpark => FILTER_SPARK_SVG,
            Self::FilterStar => FILTER_STAR_SVG,
            Self::FilterUp => FILTER_UP_SVG,
            Self::FilterX => FILTER_X_SVG,
            Self::Filters => FILTERS_SVG,
            Self::Fingerprint => FINGERPRINT_SVG,
            Self::FingerprintOff => FINGERPRINT_OFF_SVG,
            Self::FingerprintScan => FINGERPRINT_SCAN_SVG,
            Self::FireExtinguisher => FIRE_EXTINGUISHER_SVG,
            Self::FirewallCheck => FIREWALL_CHECK_SVG,
            Self::FirewallFlame => FIREWALL_FLAME_SVG,
            Self::FirstAidKit => FIRST_AID_KIT_SVG,
            Self::FirstAidKitOff => FIRST_AID_KIT_OFF_SVG,
            Self::FishChristianity => FISH_CHRISTIANITY_SVG,
            Self::FlagBitcoin => FLAG_BITCOIN_SVG,
            Self::FlagDiscount => FLAG_DISCOUNT_SVG,
            Self::Flask => FLASK_SVG,
            Self::Flask2 => FLASK_2_SVG,
            Self::Flask2Off => FLASK_2_OFF_SVG,
            Self::FlaskOff => FLASK_OFF_SVG,
            Self::FocusCentered => FOCUS_CENTERED_SVG,
            Self::Foodsteps => FOODSTEPS_SVG,
            Self::Forbid => FORBID_SVG,
            Self::Forbid2 => FORBID_2_SVG,
            Self::FreeRights => FREE_RIGHTS_SVG,
            Self::Friends => FRIENDS_SVG,
            Self::FriendsOff => FRIENDS_OFF_SVG,
            Self::Function => FUNCTION_SVG,
            Self::FunctionOff => FUNCTION_OFF_SVG,
            Self::Galaxy => GALAXY_SVG,
            Self::Gauge => GAUGE_SVG,
            Self::GaugeOff => GAUGE_OFF_SVG,
            Self::Gavel => GAVEL_SVG,
            Self::Graph => GRAPH_SVG,
            Self::GraphOff => GRAPH_OFF_SVG,
            Self::GridDots => GRID_DOTS_SVG,
            Self::GridPattern => GRID_PATTERN_SVG,
            Self::GridScan => GRID_SCAN_SVG,
            Self::GripHorizontal => GRIP_HORIZONTAL_SVG,
            Self::GripVertical => GRIP_VERTICAL_SVG,
            Self::Gymnastics => GYMNASTICS_SVG,
            Self::HandSanitizer => HAND_SANITIZER_SVG,
            Self::Hanger2 => HANGER_2_SVG,
            Self::HazeMoon => HAZE_MOON_SVG,
            Self::HealthRecognition => HEALTH_RECOGNITION_SVG,
            Self::HeartBitcoin => HEART_BITCOIN_SVG,
            Self::HeartBroken => HEART_BROKEN_SVG,
            Self::HeartDiscount => HEART_DISCOUNT_SVG,
            Self::HeartHandshake => HEART_HANDSHAKE_SVG,
            Self::HeartRateMonitor => HEART_RATE_MONITOR_SVG,
            Self::Heartbeat => HEARTBEAT_SVG,
            Self::Help => HELP_SVG,
            Self::HelpCircle => HELP_CIRCLE_SVG,
            Self::HelpHexagon => HELP_HEXAGON_SVG,
            Self::HelpOctagon => HELP_OCTAGON_SVG,
            Self::HelpOff => HELP_OFF_SVG,
            Self::HelpSmall => HELP_SMALL_SVG,
            Self::HelpSquare => HELP_SQUARE_SVG,
            Self::HelpSquareRounded => HELP_SQUARE_ROUNDED_SVG,
            Self::HelpTriangle => HELP_TRIANGLE_SVG,
            Self::History => HISTORY_SVG,
            Self::HistoryOff => HISTORY_OFF_SVG,
            Self::HistoryToggle => HISTORY_TOGGLE_SVG,
            Self::Home => HOME_SVG,
            Self::Home2 => HOME_2_SVG,
            Self::HomeBitcoin => HOME_BITCOIN_SVG,
            Self::HomeBolt => HOME_BOLT_SVG,
            Self::HomeCancel => HOME_CANCEL_SVG,
            Self::HomeCheck => HOME_CHECK_SVG,
            Self::HomeCog => HOME_COG_SVG,
            Self::HomeDollar => HOME_DOLLAR_SVG,
            Self::HomeDot => HOME_DOT_SVG,
            Self::HomeDown => HOME_DOWN_SVG,
            Self::HomeEco => HOME_ECO_SVG,
            Self::HomeEdit => HOME_EDIT_SVG,
            Self::HomeExclamation => HOME_EXCLAMATION_SVG,
            Self::HomeHand => HOME_HAND_SVG,
            Self::HomeHeart => HOME_HEART_SVG,
            Self::HomeInfinity => HOME_INFINITY_SVG,
            Self::HomeLink => HOME_LINK_SVG,
            Self::HomeLock => HOME_LOCK_SVG,
            Self::HomeMinus => HOME_MINUS_SVG,
            Self::HomeMove => HOME_MOVE_SVG,
            Self::HomeOff => HOME_OFF_SVG,
            Self::HomePlus => HOME_PLUS_SVG,
            Self::HomeQuestion => HOME_QUESTION_SVG,
            Self::HomeRibbon => HOME_RIBBON_SVG,
            Self::HomeSearch => HOME_SEARCH_SVG,
            Self::HomeShare => HOME_SHARE_SVG,
            Self::HomeShield => HOME_SHIELD_SVG,
            Self::HomeSignal => HOME_SIGNAL_SVG,
            Self::HomeSpark => HOME_SPARK_SVG,
            Self::HomeStar => HOME_STAR_SVG,
            Self::HomeStats => HOME_STATS_SVG,
            Self::HomeUp => HOME_UP_SVG,
            Self::HomeX => HOME_X_SVG,
            Self::Hospital => HOSPITAL_SVG,
            Self::HospitalCircle => HOSPITAL_CIRCLE_SVG,
            Self::HotelService => HOTEL_SERVICE_SVG,
            Self::Hourglass => HOURGLASS_SVG,
            Self::HourglassEmpty => HOURGLASS_EMPTY_SVG,
            Self::HourglassHigh => HOURGLASS_HIGH_SVG,
            Self::HourglassLow => HOURGLASS_LOW_SVG,
            Self::HourglassOff => HOURGLASS_OFF_SVG,
            Self::Hours12 => HOURS_12_SVG,
            Self::Hours24 => HOURS_24_SVG,
            Self::Id => ID_SVG,
            Self::IdBadge => ID_BADGE_SVG,
            Self::IdBadge2 => ID_BADGE_2_SVG,
            Self::IdBadgeOff => ID_BADGE_OFF_SVG,
            Self::IdOff => ID_OFF_SVG,
            Self::ImageGeneration => IMAGE_GENERATION_SVG,
            Self::Inbox => INBOX_SVG,
            Self::InboxOff => INBOX_OFF_SVG,
            Self::Infinity => INFINITY_SVG,
            Self::Infinity2 => INFINITY_2_SVG,
            Self::InfinityOff => INFINITY_OFF_SVG,
            Self::InfoCircle => INFO_CIRCLE_SVG,
            Self::InfoHexagon => INFO_HEXAGON_SVG,
            Self::InfoOctagon => INFO_OCTAGON_SVG,
            Self::InfoSmall => INFO_SMALL_SVG,
            Self::InfoSquare => INFO_SQUARE_SVG,
            Self::InfoSquareRounded => INFO_SQUARE_ROUNDED_SVG,
            Self::InfoTriangle => INFO_TRIANGLE_SVG,
            Self::InputAi => INPUT_AI_SVG,
            Self::InputCheck => INPUT_CHECK_SVG,
            Self::InputSpark => INPUT_SPARK_SVG,
            Self::InputX => INPUT_X_SVG,
            Self::Jetpack => JETPACK_SVG,
            Self::JoinBevel => JOIN_BEVEL_SVG,
            Self::JoinRound => JOIN_ROUND_SVG,
            Self::JoinStraight => JOIN_STRAIGHT_SVG,
            Self::Label => LABEL_SVG,
            Self::LabelImportant => LABEL_IMPORTANT_SVG,
            Self::LabelOff => LABEL_OFF_SVG,
            Self::Ladder => LADDER_SVG,
            Self::LadderOff => LADDER_OFF_SVG,
            Self::Lane => LANE_SVG,
            Self::LayoutBottombarInactive => LAYOUT_BOTTOMBAR_INACTIVE_SVG,
            Self::LayoutNavbarInactive => LAYOUT_NAVBAR_INACTIVE_SVG,
            Self::LayoutSidebarInactive => LAYOUT_SIDEBAR_INACTIVE_SVG,
            Self::LayoutSidebarRightInactive => LAYOUT_SIDEBAR_RIGHT_INACTIVE_SVG,
            Self::Lego => LEGO_SVG,
            Self::LegoOff => LEGO_OFF_SVG,
            Self::Library => LIBRARY_SVG,
            Self::LibraryMinus => LIBRARY_MINUS_SVG,
            Self::LibraryPhoto => LIBRARY_PHOTO_SVG,
            Self::LibraryPlus => LIBRARY_PLUS_SVG,
            Self::Lifebuoy => LIFEBUOY_SVG,
            Self::LifebuoyOff => LIFEBUOY_OFF_SVG,
            Self::Lighter => LIGHTER_SVG,
            Self::LineScan => LINE_SCAN_SVG,
            Self::ListLetters => LIST_LETTERS_SVG,
            Self::ListTree => LIST_TREE_SVG,
            Self::Loader => LOADER_SVG,
            Self::Loader2 => LOADER_2_SVG,
            Self::Loader3 => LOADER_3_SVG,
            Self::Loader4 => LOADER_4_SVG,
            Self::LoaderQuarter => LOADER_QUARTER_SVG,
            Self::LocationDiscount => LOCATION_DISCOUNT_SVG,
            Self::Lock => LOCK_SVG,
            Self::LockAccess => LOCK_ACCESS_SVG,
            Self::LockAccessOff => LOCK_ACCESS_OFF_SVG,
            Self::LockBitcoin => LOCK_BITCOIN_SVG,
            Self::LockBolt => LOCK_BOLT_SVG,
            Self::LockCancel => LOCK_CANCEL_SVG,
            Self::LockCheck => LOCK_CHECK_SVG,
            Self::LockCode => LOCK_CODE_SVG,
            Self::LockCog => LOCK_COG_SVG,
            Self::LockDollar => LOCK_DOLLAR_SVG,
            Self::LockDown => LOCK_DOWN_SVG,
            Self::LockExclamation => LOCK_EXCLAMATION_SVG,
            Self::LockHeart => LOCK_HEART_SVG,
            Self::LockMinus => LOCK_MINUS_SVG,
            Self::LockOff => LOCK_OFF_SVG,
            Self::LockOpen => LOCK_OPEN_SVG,
            Self::LockOpen2 => LOCK_OPEN_2_SVG,
            Self::LockOpenOff => LOCK_OPEN_OFF_SVG,
            Self::LockPassword => LOCK_PASSWORD_SVG,
            Self::LockPause => LOCK_PAUSE_SVG,
            Self::LockPin => LOCK_PIN_SVG,
            Self::LockPlus => LOCK_PLUS_SVG,
            Self::LockQuestion => LOCK_QUESTION_SVG,
            Self::LockSearch => LOCK_SEARCH_SVG,
            Self::LockShare => LOCK_SHARE_SVG,
            Self::LockSquare => LOCK_SQUARE_SVG,
            Self::LockSquareRounded => LOCK_SQUARE_ROUNDED_SVG,
            Self::LockStar => LOCK_STAR_SVG,
            Self::LockUp => LOCK_UP_SVG,
            Self::LockX => LOCK_X_SVG,
            Self::LogicAnd => LOGIC_AND_SVG,
            Self::LogicBuffer => LOGIC_BUFFER_SVG,
            Self::LogicNand => LOGIC_NAND_SVG,
            Self::LogicNor => LOGIC_NOR_SVG,
            Self::LogicNot => LOGIC_NOT_SVG,
            Self::LogicOr => LOGIC_OR_SVG,
            Self::LogicXnor => LOGIC_XNOR_SVG,
            Self::LogicXor => LOGIC_XOR_SVG,
            Self::Logout => LOGOUT_SVG,
            Self::Logout2 => LOGOUT_2_SVG,
            Self::Logs => LOGS_SVG,
            Self::Luggage => LUGGAGE_SVG,
            Self::LuggageOff => LUGGAGE_OFF_SVG,
            Self::Lungs => LUNGS_SVG,
            Self::LungsOff => LUNGS_OFF_SVG,
            Self::Man => MAN_SVG,
            Self::Marquee => MARQUEE_SVG,
            Self::Marquee2 => MARQUEE_2_SVG,
            Self::MarqueeOff => MARQUEE_OFF_SVG,
            Self::Mars => MARS_SVG,
            Self::Massage => MASSAGE_SVG,
            Self::Matchstick => MATCHSTICK_SVG,
            Self::Math => MATH_SVG,
            Self::Math1Divide2 => MATH_1_DIVIDE_2_SVG,
            Self::Math1Divide3 => MATH_1_DIVIDE_3_SVG,
            Self::MathAvg => MATH_AVG_SVG,
            Self::MathCos => MATH_COS_SVG,
            Self::MathCtg => MATH_CTG_SVG,
            Self::MathEqualGreater => MATH_EQUAL_GREATER_SVG,
            Self::MathEqualLower => MATH_EQUAL_LOWER_SVG,
            Self::MathFunction => MATH_FUNCTION_SVG,
            Self::MathFunctionOff => MATH_FUNCTION_OFF_SVG,
            Self::MathFunctionY => MATH_FUNCTION_Y_SVG,
            Self::MathGreater => MATH_GREATER_SVG,
            Self::MathIntegral => MATH_INTEGRAL_SVG,
            Self::MathIntegralX => MATH_INTEGRAL_X_SVG,
            Self::MathIntegrals => MATH_INTEGRALS_SVG,
            Self::MathLower => MATH_LOWER_SVG,
            Self::MathMax => MATH_MAX_SVG,
            Self::MathMaxMin => MATH_MAX_MIN_SVG,
            Self::MathMin => MATH_MIN_SVG,
            Self::MathNot => MATH_NOT_SVG,
            Self::MathOff => MATH_OFF_SVG,
            Self::MathPi => MATH_PI_SVG,
            Self::MathPiDivide2 => MATH_PI_DIVIDE_2_SVG,
            Self::MathSec => MATH_SEC_SVG,
            Self::MathSin => MATH_SIN_SVG,
            Self::MathSymbols => MATH_SYMBOLS_SVG,
            Self::MathTg => MATH_TG_SVG,
            Self::MathXDivide2 => MATH_X_DIVIDE_2_SVG,
            Self::MathXDivideY => MATH_X_DIVIDE_Y_SVG,
            Self::MathXDivideY2 => MATH_X_DIVIDE_Y_2_SVG,
            Self::MathXFloorDivideY => MATH_X_FLOOR_DIVIDE_Y_SVG,
            Self::MathXMinusX => MATH_X_MINUS_X_SVG,
            Self::MathXMinusY => MATH_X_MINUS_Y_SVG,
            Self::MathXPlusX => MATH_X_PLUS_X_SVG,
            Self::MathXPlusY => MATH_X_PLUS_Y_SVG,
            Self::MathXy => MATH_XY_SVG,
            Self::MathYMinusY => MATH_Y_MINUS_Y_SVG,
            Self::MathYPlusY => MATH_Y_PLUS_Y_SVG,
            Self::Matrix => MATRIX_SVG,
            Self::MedicineSyrup => MEDICINE_SYRUP_SVG,
            Self::Menorah => MENORAH_SVG,
            Self::Menu => MENU_SVG,
            Self::Menu2 => MENU_2_SVG,
            Self::Menu3 => MENU_3_SVG,
            Self::Menu4 => MENU_4_SVG,
            Self::MenuDeep => MENU_DEEP_SVG,
            Self::MenuOrder => MENU_ORDER_SVG,
            Self::MeterCube => METER_CUBE_SVG,
            Self::MeterSquare => METER_SQUARE_SVG,
            Self::Metronome => METRONOME_SVG,
            Self::Microfrontends => MICROFRONTENDS_SVG,
            Self::Microscope => MICROSCOPE_SVG,
            Self::MicroscopeOff => MICROSCOPE_OFF_SVG,
            Self::Middleware => MIDDLEWARE_SVG,
            Self::MilitaryAward => MILITARY_AWARD_SVG,
            Self::MilitaryRank => MILITARY_RANK_SVG,
            Self::Minus => MINUS_SVG,
            Self::MinusVertical => MINUS_VERTICAL_SVG,
            Self::Mobiledata => MOBILEDATA_SVG,
            Self::MobiledataOff => MOBILEDATA_OFF_SVG,
            Self::Mosque => MOSQUE_SVG,
            Self::Multiplier05x => MULTIPLIER_0_5X_SVG,
            Self::Multiplier15x => MULTIPLIER_1_5X_SVG,
            Self::Multiplier1x => MULTIPLIER_1X_SVG,
            Self::Multiplier2x => MULTIPLIER_2X_SVG,
            Self::MusicDiscount => MUSIC_DISCOUNT_SVG,
            Self::NewSection => NEW_SECTION_SVG,
            Self::NoCopyright => NO_COPYRIGHT_SVG,
            Self::NoCreativeCommons => NO_CREATIVE_COMMONS_SVG,
            Self::NoDerivatives => NO_DERIVATIVES_SVG,
            Self::Notification => NOTIFICATION_SVG,
            Self::NotificationOff => NOTIFICATION_OFF_SVG,
            Self::Nurse => NURSE_SVG,
            Self::ObjectScan => OBJECT_SCAN_SVG,
            Self::Old => OLD_SVG,
            Self::Om => OM_SVG,
            Self::Omega => OMEGA_SVG,
            Self::Option => OPTION_SVG,
            Self::Outbound => OUTBOUND_SVG,
            Self::Outlet => OUTLET_SVG,
            Self::PackageExport => PACKAGE_EXPORT_SVG,
            Self::PackageImport => PACKAGE_IMPORT_SVG,
            Self::Packages => PACKAGES_SVG,
            Self::Parentheses => PARENTHESES_SVG,
            Self::ParenthesesOff => PARENTHESES_OFF_SVG,
            Self::ParkingCircle => PARKING_CIRCLE_SVG,
            Self::Password => PASSWORD_SVG,
            Self::PasswordFingerprint => PASSWORD_FINGERPRINT_SVG,
            Self::PasswordMobilePhone => PASSWORD_MOBILE_PHONE_SVG,
            Self::PasswordUser => PASSWORD_USER_SVG,
            Self::Paywall => PAYWALL_SVG,
            Self::Peace => PEACE_SVG,
            Self::Pendulum => PENDULUM_SVG,
            Self::Percentage => PERCENTAGE_SVG,
            Self::Percentage0 => PERCENTAGE_0_SVG,
            Self::Percentage10 => PERCENTAGE_10_SVG,
            Self::Percentage100 => PERCENTAGE_100_SVG,
            Self::Percentage20 => PERCENTAGE_20_SVG,
            Self::Percentage25 => PERCENTAGE_25_SVG,
            Self::Percentage30 => PERCENTAGE_30_SVG,
            Self::Percentage33 => PERCENTAGE_33_SVG,
            Self::Percentage40 => PERCENTAGE_40_SVG,
            Self::Percentage50 => PERCENTAGE_50_SVG,
            Self::Percentage60 => PERCENTAGE_60_SVG,
            Self::Percentage66 => PERCENTAGE_66_SVG,
            Self::Percentage70 => PERCENTAGE_70_SVG,
            Self::Percentage75 => PERCENTAGE_75_SVG,
            Self::Percentage80 => PERCENTAGE_80_SVG,
            Self::Percentage90 => PERCENTAGE_90_SVG,
            Self::PhotoScan => PHOTO_SCAN_SVG,
            Self::Physotherapist => PHYSOTHERAPIST_SVG,
            Self::Pill => PILL_SVG,
            Self::PillOff => PILL_OFF_SVG,
            Self::Pillow => PILLOW_SVG,
            Self::Pills => PILLS_SVG,
            Self::PinEnd => PIN_END_SVG,
            Self::PinInvoke => PIN_INVOKE_SVG,
            Self::Pipeline => PIPELINE_SVG,
            Self::PlaylistAdd => PLAYLIST_ADD_SVG,
            Self::Plunger => PLUNGER_SVG,
            Self::Plus => PLUS_SVG,
            Self::PlusEqual => PLUS_EQUAL_SVG,
            Self::PlusMinus => PLUS_MINUS_SVG,
            Self::Podium => PODIUM_SVG,
            Self::PodiumOff => PODIUM_OFF_SVG,
            Self::Point => POINT_SVG,
            Self::PointOff => POINT_OFF_SVG,
            Self::Pointer => POINTER_SVG,
            Self::Pointer2 => POINTER_2_SVG,
            Self::PointerBolt => POINTER_BOLT_SVG,
            Self::PointerCancel => POINTER_CANCEL_SVG,
            Self::PointerCheck => POINTER_CHECK_SVG,
            Self::PointerCode => POINTER_CODE_SVG,
            Self::PointerCog => POINTER_COG_SVG,
            Self::PointerCollaboration => POINTER_COLLABORATION_SVG,
            Self::PointerCollaboration2 => POINTER_COLLABORATION_2_SVG,
            Self::PointerDollar => POINTER_DOLLAR_SVG,
            Self::PointerDown => POINTER_DOWN_SVG,
            Self::PointerExclamation => POINTER_EXCLAMATION_SVG,
            Self::PointerHeart => POINTER_HEART_SVG,
            Self::PointerMinus => POINTER_MINUS_SVG,
            Self::PointerOff => POINTER_OFF_SVG,
            Self::PointerPause => POINTER_PAUSE_SVG,
            Self::PointerPin => POINTER_PIN_SVG,
            Self::PointerPlus => POINTER_PLUS_SVG,
            Self::PointerQuestion => POINTER_QUESTION_SVG,
            Self::PointerSearch => POINTER_SEARCH_SVG,
            Self::PointerShare => POINTER_SHARE_SVG,
            Self::PointerStar => POINTER_STAR_SVG,
            Self::PointerUp => POINTER_UP_SVG,
            Self::PointerX => POINTER_X_SVG,
            Self::Poo => POO_SVG,
            Self::Pray => PRAY_SVG,
            Self::PremiumRights => PREMIUM_RIGHTS_SVG,
            Self::Prescription => PRESCRIPTION_SVG,
            Self::PrismLight => PRISM_LIGHT_SVG,
            Self::Progress => PROGRESS_SVG,
            Self::ProgressAlert => PROGRESS_ALERT_SVG,
            Self::ProgressBolt => PROGRESS_BOLT_SVG,
            Self::ProgressCheck => PROGRESS_CHECK_SVG,
            Self::ProgressDown => PROGRESS_DOWN_SVG,
            Self::ProgressHelp => PROGRESS_HELP_SVG,
            Self::ProgressX => PROGRESS_X_SVG,
            Self::Prompt => PROMPT_SVG,
            Self::Propeller => PROPELLER_SVG,
            Self::PropellerOff => PROPELLER_OFF_SVG,
            Self::Protocol => PROTOCOL_SVG,
            Self::QuestionMark => QUESTION_MARK_SVG,
            Self::QueuePopIn => QUEUE_POP_IN_SVG,
            Self::QueuePopOut => QUEUE_POP_OUT_SVG,
            Self::Radioactive => RADIOACTIVE_SVG,
            Self::RadioactiveOff => RADIOACTIVE_OFF_SVG,
            Self::Rating12Plus => RATING_12_PLUS_SVG,
            Self::Rating14Plus => RATING_14_PLUS_SVG,
            Self::Rating16Plus => RATING_16_PLUS_SVG,
            Self::Rating18Plus => RATING_18_PLUS_SVG,
            Self::Rating21Plus => RATING_21_PLUS_SVG,
            Self::Razor => RAZOR_SVG,
            Self::RazorElectric => RAZOR_ELECTRIC_SVG,
            Self::Recharging => RECHARGING_SVG,
            Self::RecordMail => RECORD_MAIL_SVG,
            Self::RecordMailOff => RECORD_MAIL_OFF_SVG,
            Self::Recycle => RECYCLE_SVG,
            Self::RecycleOff => RECYCLE_OFF_SVG,
            Self::Registered => REGISTERED_SVG,
            Self::RelationManyToMany => RELATION_MANY_TO_MANY_SVG,
            Self::RelationOneToMany => RELATION_ONE_TO_MANY_SVG,
            Self::RelationOneToOne => RELATION_ONE_TO_ONE_SVG,
            Self::Reorder => REORDER_SVG,
            Self::Replace => REPLACE_SVG,
            Self::ReplaceOff => REPLACE_OFF_SVG,
            Self::ReplaceUser => REPLACE_USER_SVG,
            Self::ReservedLine => RESERVED_LINE_SVG,
            Self::Restore => RESTORE_SVG,
            Self::RibbonHealth => RIBBON_HEALTH_SVG,
            Self::RobotFace => ROBOT_FACE_SVG,
            Self::RobotOff => ROBOT_OFF_SVG,
            Self::Rotate3d => ROTATE_3D_SVG,
            Self::RouteScan => ROUTE_SCAN_SVG,
            Self::RowInsertBottom => ROW_INSERT_BOTTOM_SVG,
            Self::RowInsertTop => ROW_INSERT_TOP_SVG,
            Self::RowRemove => ROW_REMOVE_SVG,
            Self::RvTruck => RV_TRUCK_SVG,
            Self::Sandbox => SANDBOX_SVG,
            Self::Scale => SCALE_SVG,
            Self::ScaleOff => SCALE_OFF_SVG,
            Self::ScaleOutline => SCALE_OUTLINE_SVG,
            Self::ScaleOutlineOff => SCALE_OUTLINE_OFF_SVG,
            Self::Scan => SCAN_SVG,
            Self::ScanCube => SCAN_CUBE_SVG,
            Self::ScanEye => SCAN_EYE_SVG,
            Self::ScanPosition => SCAN_POSITION_SVG,
            Self::ScanTraces => SCAN_TRACES_SVG,
            Self::Schema => SCHEMA_SVG,
            Self::SchemaOff => SCHEMA_OFF_SVG,
            Self::SchoolBell => SCHOOL_BELL_SVG,
            Self::Sdk => SDK_SVG,
            Self::Search => SEARCH_SVG,
            Self::SearchOff => SEARCH_OFF_SVG,
            Self::SelectAll => SELECT_ALL_SVG,
            Self::Seo => SEO_SVG,
            Self::Serverless => SERVERLESS_SVG,
            Self::Servicemark => SERVICEMARK_SVG,
            Self::Settings => SETTINGS_SVG,
            Self::Settings2 => SETTINGS_2_SVG,
            Self::SettingsAi => SETTINGS_AI_SVG,
            Self::SettingsAutomation => SETTINGS_AUTOMATION_SVG,
            Self::SettingsBolt => SETTINGS_BOLT_SVG,
            Self::SettingsCancel => SETTINGS_CANCEL_SVG,
            Self::SettingsCheck => SETTINGS_CHECK_SVG,
            Self::SettingsCode => SETTINGS_CODE_SVG,
            Self::SettingsCog => SETTINGS_COG_SVG,
            Self::SettingsDollar => SETTINGS_DOLLAR_SVG,
            Self::SettingsDown => SETTINGS_DOWN_SVG,
            Self::SettingsExclamation => SETTINGS_EXCLAMATION_SVG,
            Self::SettingsHeart => SETTINGS_HEART_SVG,
            Self::SettingsMinus => SETTINGS_MINUS_SVG,
            Self::SettingsOff => SETTINGS_OFF_SVG,
            Self::SettingsPause => SETTINGS_PAUSE_SVG,
            Self::SettingsPin => SETTINGS_PIN_SVG,
            Self::SettingsPlus => SETTINGS_PLUS_SVG,
            Self::SettingsQuestion => SETTINGS_QUESTION_SVG,
            Self::SettingsSearch => SETTINGS_SEARCH_SVG,
            Self::SettingsShare => SETTINGS_SHARE_SVG,
            Self::SettingsSpark => SETTINGS_SPARK_SVG,
            Self::SettingsStar => SETTINGS_STAR_SVG,
            Self::SettingsUp => SETTINGS_UP_SVG,
            Self::SettingsX => SETTINGS_X_SVG,
            Self::Share => SHARE_SVG,
            Self::ShareOff => SHARE_OFF_SVG,
            Self::Shield => SHIELD_SVG,
            Self::ShieldBolt => SHIELD_BOLT_SVG,
            Self::ShieldCancel => SHIELD_CANCEL_SVG,
            Self::ShieldCheck => SHIELD_CHECK_SVG,
            Self::ShieldCheckered => SHIELD_CHECKERED_SVG,
            Self::ShieldChevron => SHIELD_CHEVRON_SVG,
            Self::ShieldCode => SHIELD_CODE_SVG,
            Self::ShieldCog => SHIELD_COG_SVG,
            Self::ShieldDollar => SHIELD_DOLLAR_SVG,
            Self::ShieldDown => SHIELD_DOWN_SVG,
            Self::ShieldExclamation => SHIELD_EXCLAMATION_SVG,
            Self::ShieldHalf => SHIELD_HALF_SVG,
            Self::ShieldHeart => SHIELD_HEART_SVG,
            Self::ShieldLock => SHIELD_LOCK_SVG,
            Self::ShieldMinus => SHIELD_MINUS_SVG,
            Self::ShieldOff => SHIELD_OFF_SVG,
            Self::ShieldPause => SHIELD_PAUSE_SVG,
            Self::ShieldPin => SHIELD_PIN_SVG,
            Self::ShieldPlus => SHIELD_PLUS_SVG,
            Self::ShieldQuestion => SHIELD_QUESTION_SVG,
            Self::ShieldSearch => SHIELD_SEARCH_SVG,
            Self::ShieldShare => SHIELD_SHARE_SVG,
            Self::ShieldStar => SHIELD_STAR_SVG,
            Self::ShieldUp => SHIELD_UP_SVG,
            Self::ShieldX => SHIELD_X_SVG,
            Self::Sitemap => SITEMAP_SVG,
            Self::SitemapOff => SITEMAP_OFF_SVG,
            Self::SkewX => SKEW_X_SVG,
            Self::SkewY => SKEW_Y_SVG,
            Self::Skull => SKULL_SVG,
            Self::Slash => SLASH_SVG,
            Self::Slashes => SLASHES_SVG,
            Self::SmartHome => SMART_HOME_SVG,
            Self::SmartHomeOff => SMART_HOME_OFF_SVG,
            Self::Smoking => SMOKING_SVG,
            Self::SmokingNo => SMOKING_NO_SVG,
            Self::Snowboarding => SNOWBOARDING_SVG,
            Self::Social => SOCIAL_SVG,
            Self::SocialOff => SOCIAL_OFF_SVG,
            Self::Sofa => SOFA_SVG,
            Self::SofaOff => SOFA_OFF_SVG,
            Self::SolarElectricity => SOLAR_ELECTRICITY_SVG,
            Self::SolarPanel => SOLAR_PANEL_SVG,
            Self::SolarPanel2 => SOLAR_PANEL_2_SVG,
            Self::SortAscendingShapes => SORT_ASCENDING_SHAPES_SVG,
            Self::SortAscendingSmallBig => SORT_ASCENDING_SMALL_BIG_SVG,
            Self::SortDescendingShapes => SORT_DESCENDING_SHAPES_SVG,
            Self::SortDescendingSmallBig => SORT_DESCENDING_SMALL_BIG_SVG,
            Self::Sos => SOS_SVG,
            Self::SourceCode => SOURCE_CODE_SVG,
            Self::Spaces => SPACES_SVG,
            Self::Sparkles => SPARKLES_SVG,
            Self::Spiral => SPIRAL_SVG,
            Self::SpiralOff => SPIRAL_OFF_SVG,
            Self::Spray => SPRAY_SVG,
            Self::Spy => SPY_SVG,
            Self::SpyOff => SPY_OFF_SVG,
            Self::SquareF0 => SQUARE_F0_SVG,
            Self::SquareF1 => SQUARE_F1_SVG,
            Self::SquareF2 => SQUARE_F2_SVG,
            Self::SquareF3 => SQUARE_F3_SVG,
            Self::SquareF4 => SQUARE_F4_SVG,
            Self::SquareF5 => SQUARE_F5_SVG,
            Self::SquareF6 => SQUARE_F6_SVG,
            Self::SquareF7 => SQUARE_F7_SVG,
            Self::SquareF8 => SQUARE_F8_SVG,
            Self::SquareF9 => SQUARE_F9_SVG,
            Self::SquareRoot => SQUARE_ROOT_SVG,
            Self::SquareRoot2 => SQUARE_ROOT_2_SVG,
            Self::StackBack => STACK_BACK_SVG,
            Self::StackBackward => STACK_BACKWARD_SVG,
            Self::StackForward => STACK_FORWARD_SVG,
            Self::StackFront => STACK_FRONT_SVG,
            Self::StackMiddle => STACK_MIDDLE_SVG,
            Self::Star => STAR_SVG,
            Self::StarHalf => STAR_HALF_SVG,
            Self::StarOff => STAR_OFF_SVG,
            Self::Stars => STARS_SVG,
            Self::StarsOff => STARS_OFF_SVG,
            Self::StatusChange => STATUS_CHANGE_SVG,
            Self::Steam => STEAM_SVG,
            Self::StereoGlasses => STEREO_GLASSES_SVG,
            Self::Stethoscope => STETHOSCOPE_SVG,
            Self::StethoscopeOff => STETHOSCOPE_OFF_SVG,
            Self::Stopwatch => STOPWATCH_SVG,
            Self::Subtask => SUBTASK_SVG,
            Self::Sum => SUM_SVG,
            Self::SumOff => SUM_OFF_SVG,
            Self::Sunglasses => SUNGLASSES_SVG,
            Self::Swipe => SWIPE_SVG,
            Self::Table => TABLE_SVG,
            Self::TableAlias => TABLE_ALIAS_SVG,
            Self::TableColumn => TABLE_COLUMN_SVG,
            Self::TableDashed => TABLE_DASHED_SVG,
            Self::TableDown => TABLE_DOWN_SVG,
            Self::TableExport => TABLE_EXPORT_SVG,
            Self::TableHeart => TABLE_HEART_SVG,
            Self::TableImport => TABLE_IMPORT_SVG,
            Self::TableMinus => TABLE_MINUS_SVG,
            Self::TableOff => TABLE_OFF_SVG,
            Self::TableOptions => TABLE_OPTIONS_SVG,
            Self::TablePlus => TABLE_PLUS_SVG,
            Self::TableRow => TABLE_ROW_SVG,
            Self::TableShare => TABLE_SHARE_SVG,
            Self::TableShortcut => TABLE_SHORTCUT_SVG,
            Self::TableSpark => TABLE_SPARK_SVG,
            Self::Tallymark1 => TALLYMARK_1_SVG,
            Self::Tallymark2 => TALLYMARK_2_SVG,
            Self::Tallymark3 => TALLYMARK_3_SVG,
            Self::Tallymark4 => TALLYMARK_4_SVG,
            Self::Tallymarks => TALLYMARKS_SVG,
            Self::Telescope => TELESCOPE_SVG,
            Self::TelescopeOff => TELESCOPE_OFF_SVG,
            Self::TemperatureSnow => TEMPERATURE_SNOW_SVG,
            Self::TemperatureSun => TEMPERATURE_SUN_SVG,
            Self::Terminal => TERMINAL_SVG,
            Self::Terminal2 => TERMINAL_2_SVG,
            Self::TestPipe => TEST_PIPE_SVG,
            Self::TestPipe2 => TEST_PIPE_2_SVG,
            Self::TestPipeOff => TEST_PIPE_OFF_SVG,
            Self::TextScan2 => TEXT_SCAN_2_SVG,
            Self::TextScanAi => TEXT_SCAN_AI_SVG,
            Self::Texture => TEXTURE_SVG,
            Self::Thermometer => THERMOMETER_SVG,
            Self::ThumbDown => THUMB_DOWN_SVG,
            Self::ThumbDownOff => THUMB_DOWN_OFF_SVG,
            Self::ThumbUp => THUMB_UP_SVG,
            Self::ThumbUpOff => THUMB_UP_OFF_SVG,
            Self::Tilde => TILDE_SVG,
            Self::TimeDuration0 => TIME_DURATION_0_SVG,
            Self::TimeDuration10 => TIME_DURATION_10_SVG,
            Self::TimeDuration15 => TIME_DURATION_15_SVG,
            Self::TimeDuration30 => TIME_DURATION_30_SVG,
            Self::TimeDuration45 => TIME_DURATION_45_SVG,
            Self::TimeDuration5 => TIME_DURATION_5_SVG,
            Self::TimeDuration60 => TIME_DURATION_60_SVG,
            Self::TimeDuration90 => TIME_DURATION_90_SVG,
            Self::TimeDurationOff => TIME_DURATION_OFF_SVG,
            Self::Timeline => TIMELINE_SVG,
            Self::TimelineEvent => TIMELINE_EVENT_SVG,
            Self::TimelineEventExclamation => TIMELINE_EVENT_EXCLAMATION_SVG,
            Self::TimelineEventMinus => TIMELINE_EVENT_MINUS_SVG,
            Self::TimelineEventPlus => TIMELINE_EVENT_PLUS_SVG,
            Self::TimelineEventText => TIMELINE_EVENT_TEXT_SVG,
            Self::TimelineEventX => TIMELINE_EVENT_X_SVG,
            Self::Timezone => TIMEZONE_SVG,
            Self::TipJar => TIP_JAR_SVG,
            Self::TipJarEuro => TIP_JAR_EURO_SVG,
            Self::TipJarPound => TIP_JAR_POUND_SVG,
            Self::ToggleLeft => TOGGLE_LEFT_SVG,
            Self::ToggleRight => TOGGLE_RIGHT_SVG,
            Self::ToiletPaper => TOILET_PAPER_SVG,
            Self::ToiletPaperOff => TOILET_PAPER_OFF_SVG,
            Self::Tool => TOOL_SVG,
            Self::Tooltip => TOOLTIP_SVG,
            Self::Torii => TORII_SVG,
            Self::Tower => TOWER_SVG,
            Self::TowerOff => TOWER_OFF_SVG,
            Self::Trademark => TRADEMARK_SVG,
            Self::Transfer => TRANSFER_SVG,
            Self::TransferVertical => TRANSFER_VERTICAL_SVG,
            Self::Transform => TRANSFORM_SVG,
            Self::TransformPoint => TRANSFORM_POINT_SVG,
            Self::TransformPointBottomLeft => TRANSFORM_POINT_BOTTOM_LEFT_SVG,
            Self::TransformPointBottomRight => TRANSFORM_POINT_BOTTOM_RIGHT_SVG,
            Self::TransformPointTopLeft => TRANSFORM_POINT_TOP_LEFT_SVG,
            Self::TransformPointTopRight => TRANSFORM_POINT_TOP_RIGHT_SVG,
            Self::Trash => TRASH_SVG,
            Self::TrashOff => TRASH_OFF_SVG,
            Self::TrashX => TRASH_X_SVG,
            Self::Trowel => TROWEL_SVG,
            Self::UfoOff => UFO_OFF_SVG,
            Self::Uhd => UHD_SVG,
            Self::Umbrella => UMBRELLA_SVG,
            Self::Umbrella2 => UMBRELLA_2_SVG,
            Self::UmbrellaClosed => UMBRELLA_CLOSED_SVG,
            Self::UmbrellaClosed2 => UMBRELLA_CLOSED_2_SVG,
            Self::UmbrellaOff => UMBRELLA_OFF_SVG,
            Self::Universe => UNIVERSE_SVG,
            Self::Urgent => URGENT_SVG,
            Self::Usb => USB_SVG,
            Self::User => USER_SVG,
            Self::UserBitcoin => USER_BITCOIN_SVG,
            Self::UserBolt => USER_BOLT_SVG,
            Self::UserCancel => USER_CANCEL_SVG,
            Self::UserCheck => USER_CHECK_SVG,
            Self::UserCircle => USER_CIRCLE_SVG,
            Self::UserCode => USER_CODE_SVG,
            Self::UserCog => USER_COG_SVG,
            Self::UserDollar => USER_DOLLAR_SVG,
            Self::UserDown => USER_DOWN_SVG,
            Self::UserEdit => USER_EDIT_SVG,
            Self::UserExclamation => USER_EXCLAMATION_SVG,
            Self::UserHeart => USER_HEART_SVG,
            Self::UserHexagon => USER_HEXAGON_SVG,
            Self::UserKey => USER_KEY_SVG,
            Self::UserMinus => USER_MINUS_SVG,
            Self::UserOff => USER_OFF_SVG,
            Self::UserPause => USER_PAUSE_SVG,
            Self::UserPentagon => USER_PENTAGON_SVG,
            Self::UserPin => USER_PIN_SVG,
            Self::UserPlus => USER_PLUS_SVG,
            Self::UserQuestion => USER_QUESTION_SVG,
            Self::UserScan => USER_SCAN_SVG,
            Self::UserSearch => USER_SEARCH_SVG,
            Self::UserShare => USER_SHARE_SVG,
            Self::UserShield => USER_SHIELD_SVG,
            Self::UserSquare => USER_SQUARE_SVG,
            Self::UserSquareRounded => USER_SQUARE_ROUNDED_SVG,
            Self::UserStar => USER_STAR_SVG,
            Self::UserUp => USER_UP_SVG,
            Self::UserX => USER_X_SVG,
            Self::Users => USERS_SVG,
            Self::UsersGroup => USERS_GROUP_SVG,
            Self::UsersMinus => USERS_MINUS_SVG,
            Self::UsersPlus => USERS_PLUS_SVG,
            Self::Vaccine => VACCINE_SVG,
            Self::VaccineBottle => VACCINE_BOTTLE_SVG,
            Self::VaccineBottleOff => VACCINE_BOTTLE_OFF_SVG,
            Self::VaccineOff => VACCINE_OFF_SVG,
            Self::VacuumCleaner => VACUUM_CLEANER_SVG,
            Self::Variable => VARIABLE_SVG,
            Self::VariableMinus => VARIABLE_MINUS_SVG,
            Self::VariableOff => VARIABLE_OFF_SVG,
            Self::VariablePlus => VARIABLE_PLUS_SVG,
            Self::Venus => VENUS_SVG,
            Self::Versions => VERSIONS_SVG,
            Self::VersionsOff => VERSIONS_OFF_SVG,
            Self::View360 => VIEW_360_SVG,
            Self::View360Arrow => VIEW_360_ARROW_SVG,
            Self::View360Number => VIEW_360_NUMBER_SVG,
            Self::View360Off => VIEW_360_OFF_SVG,
            Self::ViewportShort => VIEWPORT_SHORT_SVG,
            Self::ViewportTall => VIEWPORT_TALL_SVG,
            Self::Vip => VIP_SVG,
            Self::Vip2 => VIP_2_SVG,
            Self::VipOff => VIP_OFF_SVG,
            Self::Virus => VIRUS_SVG,
            Self::VirusOff => VIRUS_OFF_SVG,
            Self::VirusSearch => VIRUS_SEARCH_SVG,
            Self::Vs => VS_SVG,
            Self::Wall => WALL_SVG,
            Self::WallOff => WALL_OFF_SVG,
            Self::Wallpaper => WALLPAPER_SVG,
            Self::WallpaperOff => WALLPAPER_OFF_SVG,
            Self::Wand => WAND_SVG,
            Self::WandOff => WAND_OFF_SVG,
            Self::WaveSawTool => WAVE_SAW_TOOL_SVG,
            Self::WaveSine => WAVE_SINE_SVG,
            Self::WaveSquare => WAVE_SQUARE_SVG,
            Self::WavesElectricity => WAVES_ELECTRICITY_SVG,
            Self::Webhook => WEBHOOK_SVG,
            Self::WebhookOff => WEBHOOK_OFF_SVG,
            Self::Weight => WEIGHT_SVG,
            Self::Window => WINDOW_SVG,
            Self::WindowMaximize => WINDOW_MAXIMIZE_SVG,
            Self::WindowMinimize => WINDOW_MINIMIZE_SVG,
            Self::WindowOff => WINDOW_OFF_SVG,
            Self::Woman => WOMAN_SVG,
            Self::Wood => WOOD_SVG,
            Self::X => X_SVG,
            Self::XMark => X_MARK_SVG,
            Self::XPowerY => X_POWER_Y_SVG,
            Self::Xd => XD_SVG,
            Self::Xxx => XXX_SVG,
            Self::YinYang => YIN_YANG_SVG,
            Self::ZeroConfig => ZERO_CONFIG_SVG,
            Self::ZoomScan => ZOOM_SCAN_SVG,
            Self::Zzz => ZZZ_SVG,
            Self::ZzzOff => ZZZ_OFF_SVG,
        }
    }

    fn filled_svg(&self) -> Option<&'static str> {
        // Filled variants would be added here
        None
    }
}
