//! Arrows icons from Tabler Icons.
//!
//! This module contains 332 icons.

use crate::tabler::TablerIconData;

// SVG Constants
const ARROW_AUTOFIT_CONTENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4l-3 3l3 3" /> <path d="M18 4l3 3l-3 3" /> <path d="M4 16a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -2" /> <path d="M10 7h-7" /> <path d="M21 7h-7" /> </svg>"##;
const ARROW_AUTOFIT_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-6a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h8" /> <path d="M18 4v17" /> <path d="M15 18l3 3l3 -3" /> </svg>"##;
const ARROW_AUTOFIT_HEIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-6a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h6" /> <path d="M18 14v7" /> <path d="M18 3v7" /> <path d="M15 18l3 3l3 -3" /> <path d="M15 6l3 -3l3 3" /> </svg>"##;
const ARROW_AUTOFIT_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12v-6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v8" /> <path d="M20 18h-17" /> <path d="M6 15l-3 3l3 3" /> </svg>"##;
const ARROW_AUTOFIT_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 12v-6a2 2 0 0 0 -2 -2h-12a2 2 0 0 0 -2 2v8" /> <path d="M4 18h17" /> <path d="M18 15l3 3l-3 3" /> </svg>"##;
const ARROW_AUTOFIT_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4h-6a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h8" /> <path d="M18 20v-17" /> <path d="M15 6l3 -3l3 3" /> </svg>"##;
const ARROW_AUTOFIT_WIDTH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12v-6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6" /> <path d="M10 18h-7" /> <path d="M21 18h-7" /> <path d="M6 15l-3 3l3 3" /> <path d="M18 15l3 3l-3 3" /> </svg>"##;
const ARROW_BACK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11l-4 4l4 4m-4 -4h11a4 4 0 0 0 0 -8h-1" /> </svg>"##;
const ARROW_BACK_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 14l-4 -4l4 -4" /> <path d="M5 10h11a4 4 0 1 1 0 8h-1" /> </svg>"##;
const ARROW_BACK_UP_DOUBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 14l-4 -4l4 -4" /> <path d="M8 14l-4 -4l4 -4" /> <path d="M9 10h7a4 4 0 1 1 0 8h-1" /> </svg>"##;
const ARROW_BADGE_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 13v-6l-5 4l-5 -4v6l5 4l5 -4" /> </svg>"##;
const ARROW_BADGE_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 17h6l-4 -5l4 -5h-6l-4 5l4 5" /> </svg>"##;
const ARROW_BADGE_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 7h-6l4 5l-4 5h6l4 -5l-4 -5" /> </svg>"##;
const ARROW_BADGE_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 11v6l-5 -4l-5 4v-6l5 -4l5 4" /> </svg>"##;
const ARROW_BAR_BOTH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12h-6" /> <path d="M5 15l-3 -3l3 -3" /> <path d="M22 12h-6" /> <path d="M19 15l3 -3l-3 -3" /> <path d="M12 4v16" /> </svg>"##;
const ARROW_BAR_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l0 -10" /> <path d="M12 20l4 -4" /> <path d="M12 20l-4 -4" /> <path d="M4 4l16 0" /> </svg>"##;
const ARROW_BAR_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12l10 0" /> <path d="M4 12l4 4" /> <path d="M4 12l4 -4" /> <path d="M20 4l0 16" /> </svg>"##;
const ARROW_BAR_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 12l-10 0" /> <path d="M20 12l-4 4" /> <path d="M20 12l-4 -4" /> <path d="M4 4l0 16" /> </svg>"##;
const ARROW_BAR_TO_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20l16 0" /> <path d="M12 14l0 -10" /> <path d="M12 14l4 -4" /> <path d="M12 14l-4 -4" /> </svg>"##;
const ARROW_BAR_TO_DOWN_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 14v-10" /> <path d="M12 14l4 -4" /> <path d="M12 14l-4 -4" /> <path d="M4 20h3m13 0h-3m-3.5 0h-3" /> </svg>"##;
const ARROW_BAR_TO_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12l10 0" /> <path d="M10 12l4 4" /> <path d="M10 12l4 -4" /> <path d="M4 4l0 16" /> </svg>"##;
const ARROW_BAR_TO_LEFT_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12l10 0" /> <path d="M10 12l4 -4" /> <path d="M10 12l4 4" /> <path d="M4 20l0 -3m0 -13l0 3m0 3.5l0 3" /> </svg>"##;
const ARROW_BAR_TO_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 12l-10 0" /> <path d="M14 12l-4 4" /> <path d="M14 12l-4 -4" /> <path d="M20 4l0 16" /> </svg>"##;
const ARROW_BAR_TO_RIGHT_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 12l-10 0" /> <path d="M14 12l-4 4" /> <path d="M14 12l-4 -4" /> <path d="M20 4l0 3m0 13l0 -3m0 -3.5l0 -3" /> </svg>"##;
const ARROW_BAR_TO_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 10l0 10" /> <path d="M12 10l4 4" /> <path d="M12 10l-4 4" /> <path d="M4 4l16 0" /> </svg>"##;
const ARROW_BAR_TO_UP_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 10v10" /> <path d="M12 10l4 4" /> <path d="M12 10l-4 4" /> <path d="M4 4h3m13 0h-3m-3.5 0h-3" /> </svg>"##;
const ARROW_BAR_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4l0 10" /> <path d="M12 4l4 4" /> <path d="M12 4l-4 4" /> <path d="M4 20l16 0" /> </svg>"##;
const ARROW_BEAR_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 3h-5v5" /> <path d="M8 3l7.536 7.536a5 5 0 0 1 1.464 3.534v6.93" /> </svg>"##;
const ARROW_BEAR_LEFT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 3h-5v5" /> <path d="M4 3l7.536 7.536a5 5 0 0 1 1.464 3.534v6.93" /> <path d="M20 5l-4.5 4.5" /> </svg>"##;
const ARROW_BEAR_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3h5v5" /> <path d="M17 3l-7.536 7.536a5 5 0 0 0 -1.464 3.534v6.93" /> </svg>"##;
const ARROW_BEAR_RIGHT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 3h5v5" /> <path d="M20 3l-7.536 7.536a5 5 0 0 0 -1.464 3.534v6.93" /> <path d="M4 5l4.5 4.5" /> </svg>"##;
const ARROW_BIG_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 4v8h3.586a1 1 0 0 1 .707 1.707l-6.586 6.586a1 1 0 0 1 -1.414 0l-6.586 -6.586a1 1 0 0 1 .707 -1.707h3.586v-8a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1" /> </svg>"##;
const ARROW_BIG_DOWN_LINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 12h3.586a1 1 0 0 1 .707 1.707l-6.586 6.586a1 1 0 0 1 -1.414 0l-6.586 -6.586a1 1 0 0 1 .707 -1.707h3.586v-6h6v6" /> <path d="M15 3h-6" /> </svg>"##;
const ARROW_BIG_DOWN_LINES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 12h3.586a1 1 0 0 1 .707 1.707l-6.586 6.586a1 1 0 0 1 -1.414 0l-6.586 -6.586a1 1 0 0 1 .707 -1.707h3.586v-3h6v3" /> <path d="M15 3h-6" /> <path d="M15 6h-6" /> </svg>"##;
const ARROW_BIG_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 15h-8v3.586a1 1 0 0 1 -1.707 .707l-6.586 -6.586a1 1 0 0 1 0 -1.414l6.586 -6.586a1 1 0 0 1 1.707 .707v3.586h8a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1" /> </svg>"##;
const ARROW_BIG_LEFT_LINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 15v3.586a1 1 0 0 1 -1.707 .707l-6.586 -6.586a1 1 0 0 1 0 -1.414l6.586 -6.586a1 1 0 0 1 1.707 .707v3.586h6v6h-6" /> <path d="M21 15v-6" /> </svg>"##;
const ARROW_BIG_LEFT_LINES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 15v3.586a1 1 0 0 1 -1.707 .707l-6.586 -6.586a1 1 0 0 1 0 -1.414l6.586 -6.586a1 1 0 0 1 1.707 .707v3.586h3v6h-3" /> <path d="M21 15v-6" /> <path d="M18 15v-6" /> </svg>"##;
const ARROW_BIG_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 9h8v-3.586a1 1 0 0 1 1.707 -.707l6.586 6.586a1 1 0 0 1 0 1.414l-6.586 6.586a1 1 0 0 1 -1.707 -.707v-3.586h-8a1 1 0 0 1 -1 -1v-4a1 1 0 0 1 1 -1" /> </svg>"##;
const ARROW_BIG_RIGHT_LINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9v-3.586a1 1 0 0 1 1.707 -.707l6.586 6.586a1 1 0 0 1 0 1.414l-6.586 6.586a1 1 0 0 1 -1.707 -.707v-3.586h-6v-6h6" /> <path d="M3 9v6" /> </svg>"##;
const ARROW_BIG_RIGHT_LINES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9v-3.586a1 1 0 0 1 1.707 -.707l6.586 6.586a1 1 0 0 1 0 1.414l-6.586 6.586a1 1 0 0 1 -1.707 -.707v-3.586h-3v-6h3" /> <path d="M3 9v6" /> <path d="M6 9v6" /> </svg>"##;
const ARROW_BIG_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 20v-8h-3.586a1 1 0 0 1 -.707 -1.707l6.586 -6.586a1 1 0 0 1 1.414 0l6.586 6.586a1 1 0 0 1 -.707 1.707h-3.586v8a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1" /> </svg>"##;
const ARROW_BIG_UP_LINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12h-3.586a1 1 0 0 1 -.707 -1.707l6.586 -6.586a1 1 0 0 1 1.414 0l6.586 6.586a1 1 0 0 1 -.707 1.707h-3.586v6h-6v-6" /> <path d="M9 21h6" /> </svg>"##;
const ARROW_BIG_UP_LINES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12h-3.586a1 1 0 0 1 -.707 -1.707l6.586 -6.586a1 1 0 0 1 1.414 0l6.586 6.586a1 1 0 0 1 -.707 1.707h-3.586v3h-6v-3" /> <path d="M9 21h6" /> <path d="M9 18h6" /> </svg>"##;
const ARROW_BOUNCE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 18h4" /> <path d="M3 8a9 9 0 0 1 9 9v1l1.428 -4.285a12 12 0 0 1 6.018 -6.938l.554 -.277" /> <path d="M15 6h5v5" /> </svg>"##;
const ARROW_CAPSULE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 15a6 6 0 1 1 -12 0v-6a6 6 0 1 1 12 0v2" /> <path d="M15 8l3 3l3 -3" /> </svg>"##;
const ARROW_CURVE_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 7l-4 -4l-4 4" /> <path d="M10 3v4.394a6.737 6.737 0 0 0 3 5.606a6.737 6.737 0 0 1 3 5.606v2.394" /> </svg>"##;
const ARROW_CURVE_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 7l4 -4l4 4" /> <path d="M14 3v4.394a6.737 6.737 0 0 1 -3 5.606a6.737 6.737 0 0 0 -3 5.606v2.394" /> </svg>"##;
const ARROW_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5l0 14" /> <path d="M18 13l-6 6" /> <path d="M6 13l6 6" /> </svg>"##;
const ARROW_DOWN_BAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3v18" /> <path d="M9 18l3 3l3 -3" /> <path d="M9 3h6" /> </svg>"##;
const ARROW_DOWN_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 7v14" /> <path d="M9 18l3 3l3 -3" /> <path d="M12 7a2 2 0 1 0 0 -4a2 2 0 0 0 0 4" /> </svg>"##;
const ARROW_DOWN_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5v.5m0 3v1.5m0 3v6" /> <path d="M18 13l-6 6" /> <path d="M6 13l6 6" /> </svg>"##;
const ARROW_DOWN_FROM_ARC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 15v-12" /> <path d="M16 7l-4 -4l-4 4" /> <path d="M3 12a9 9 0 0 0 18 0" /> </svg>"##;
const ARROW_DOWN_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 7l-10 10" /> <path d="M16 17l-9 0l0 -9" /> </svg>"##;
const ARROW_DOWN_LEFT_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.536 8.464l-9.536 9.536" /> <path d="M6 14v4h4" /> <path d="M15.586 8.414a2 2 0 1 0 2.828 -2.828a2 2 0 0 0 -2.828 2.828" /> </svg>"##;
const ARROW_DOWN_RHOMBUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8v13" /> <path d="M15 18l-3 3l-3 -3" /> <path d="M14.5 5.5l-2.5 -2.5l-2.5 2.5l2.5 2.5l2.5 -2.5" /> </svg>"##;
const ARROW_DOWN_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7l10 10" /> <path d="M17 8l0 9l-9 0" /> </svg>"##;
const ARROW_DOWN_RIGHT_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.464 8.464l9.536 9.536" /> <path d="M14 18h4v-4" /> <path d="M8.414 8.414a2 2 0 1 0 -2.828 -2.828a2 2 0 0 0 2.828 2.828" /> </svg>"##;
const ARROW_DOWN_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 7v14" /> <path d="M9 18l3 3l3 -3" /> <path d="M14 3v4h-4v-4l4 0" /> </svg>"##;
const ARROW_DOWN_TAIL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 6v15" /> <path d="M9 18l3 3l3 -3" /> <path d="M9 3l3 3l3 -3" /> </svg>"##;
const ARROW_DOWN_TO_ARC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3v12" /> <path d="M16 11l-4 4l-4 -4" /> <path d="M3 12a9 9 0 0 0 18 0" /> </svg>"##;
const ARROW_ELBOW_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 14v-6h6" /> <path d="M3 8l9 9l9 -9" /> </svg>"##;
const ARROW_ELBOW_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 14v-6h-6" /> <path d="M21 8l-9 9l-9 -9" /> </svg>"##;
const ARROW_FORK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 3h5v5" /> <path d="M8 3h-5v5" /> <path d="M21 3l-7.536 7.536a5 5 0 0 0 -1.464 3.534v6.93" /> <path d="M3 3l7.536 7.536a5 5 0 0 1 1.464 3.534v.93" /> </svg>"##;
const ARROW_FORWARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11l4 4l-4 4m4 -4h-11a4 4 0 0 1 0 -8h1" /> </svg>"##;
const ARROW_FORWARD_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 14l4 -4l-4 -4" /> <path d="M19 10h-11a4 4 0 1 0 0 8h1" /> </svg>"##;
const ARROW_FORWARD_UP_DOUBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 14l4 -4l-4 -4" /> <path d="M16 14l4 -4l-4 -4" /> <path d="M15 10h-7a4 4 0 1 0 0 8h1" /> </svg>"##;
const ARROW_GUIDE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 19h3a2 2 0 0 0 2 -2v-8a2 2 0 0 1 2 -2h7" /> <path d="M18 4l3 3l-3 3" /> </svg>"##;
const ARROW_ITERATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.5 16a5.5 5.5 0 1 0 -5.5 -5.5v.5" /> <path d="M3 16h18" /> <path d="M18 13l3 3l-3 3" /> </svg>"##;
const ARROW_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12l14 0" /> <path d="M5 12l6 6" /> <path d="M5 12l6 -6" /> </svg>"##;
const ARROW_LEFT_BAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12h-18" /> <path d="M6 9l-3 3l3 3" /> <path d="M21 9v6" /> </svg>"##;
const ARROW_LEFT_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 12h-14" /> <path d="M6 9l-3 3l3 3" /> <path d="M17 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const ARROW_LEFT_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h6m3 0h1.5m3 0h.5" /> <path d="M5 12l6 6" /> <path d="M5 12l6 -6" /> </svg>"##;
const ARROW_LEFT_FROM_ARC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12h12" /> <path d="M17 16l4 -4l-4 -4" /> <path d="M12 3a9 9 0 1 0 0 18" /> </svg>"##;
const ARROW_LEFT_RHOMBUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12h-13" /> <path d="M6 9l-3 3l3 3" /> <path d="M18.5 9.5l2.5 2.5l-2.5 2.5l-2.5 -2.5l2.5 -2.5" /> </svg>"##;
const ARROW_LEFT_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 13l4 -4l-4 -4" /> <path d="M7 13l-4 -4l4 -4" /> <path d="M12 14a5 5 0 0 1 5 -5h4" /> <path d="M12 19v-5a5 5 0 0 0 -5 -5h-4" /> </svg>"##;
const ARROW_LEFT_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 12h-14" /> <path d="M6 9l-3 3l3 3" /> <path d="M21 14h-4v-4h4l0 4" /> </svg>"##;
const ARROW_LEFT_TAIL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 12h-15" /> <path d="M6 9l-3 3l3 3" /> <path d="M21 9l-3 3l3 3" /> </svg>"##;
const ARROW_LEFT_TO_ARC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12h-12" /> <path d="M13 16l-4 -4l4 -4" /> <path d="M12 3a9 9 0 1 0 0 18" /> </svg>"##;
const ARROW_LOOP_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21v-13a4 4 0 1 1 4 4h-13" /> <path d="M8 16l-4 -4l4 -4" /> </svg>"##;
const ARROW_LOOP_LEFT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21v-6m0 -6v-1a4 4 0 1 1 4 4h-13" /> <path d="M8 16l-4 -4l4 -4" /> </svg>"##;
const ARROW_LOOP_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21v-13a4 4 0 1 0 -4 4h13" /> <path d="M17 16l4 -4l-4 -4" /> </svg>"##;
const ARROW_LOOP_RIGHT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21v-6m0 -6v-1a4 4 0 1 0 -4 4h13" /> <path d="M17 16l4 -4l-4 -4" /> </svg>"##;
const ARROW_MERGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7l4 -4l4 4" /> <path d="M12 3v5.394a6.737 6.737 0 0 1 -3 5.606a6.737 6.737 0 0 0 -3 5.606v1.394" /> <path d="M12 3v5.394a6.737 6.737 0 0 0 3 5.606a6.737 6.737 0 0 1 3 5.606v1.394" /> </svg>"##;
const ARROW_MERGE_ALT_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7l4 -4l4 4" /> <path d="M18 21v.01" /> <path d="M18 18.01v.01" /> <path d="M17 15.02v.01" /> <path d="M14 13.03v.01" /> <path d="M12 3v5.394a6.737 6.737 0 0 1 -3 5.606a6.737 6.737 0 0 0 -3 5.606v1.394" /> </svg>"##;
const ARROW_MERGE_ALT_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 7l-4 -4l-4 4" /> <path d="M6 21v.01" /> <path d="M6 18.01v.01" /> <path d="M7 15.02v.01" /> <path d="M10 13.03v.01" /> <path d="M12 3v5.394a6.737 6.737 0 0 0 3 5.606a6.737 6.737 0 0 1 3 5.606v1.394" /> </svg>"##;
const ARROW_MERGE_BOTH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 8l-4 -4l-4 4" /> <path d="M12 20v-16" /> <path d="M18 18c-4 -1.333 -6 -4.667 -6 -10" /> <path d="M6 18c4 -1.333 6 -4.667 6 -10" /> </svg>"##;
const ARROW_MERGE_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8l4 -4l4 4" /> <path d="M12 20v-16" /> <path d="M6 18c4 -1.333 6 -4.667 6 -10" /> </svg>"##;
const ARROW_MERGE_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 8l-4 -4l-4 4" /> <path d="M12 20v-16" /> <path d="M18 18c-4 -1.333 -6 -4.667 -6 -10" /> </svg>"##;
const ARROW_MOVE_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 11v10" /> <path d="M9 18l3 3l3 -3" /> <path d="M10 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const ARROW_MOVE_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 12h-10" /> <path d="M6 15l-3 -3l3 -3" /> <path d="M17 12a2 2 0 1 1 4 0a2 2 0 0 1 -4 0" /> </svg>"##;
const ARROW_MOVE_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12h10" /> <path d="M18 9l3 3l-3 3" /> <path d="M7 12a2 2 0 1 1 -4 0a2 2 0 0 1 4 0" /> </svg>"##;
const ARROW_MOVE_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 13v-10" /> <path d="M9 6l3 -3l3 3" /> <path d="M12 17a2 2 0 1 1 0 4a2 2 0 0 1 0 -4" /> </svg>"##;
const ARROW_NARROW_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5l0 14" /> <path d="M16 15l-4 4" /> <path d="M8 15l4 4" /> </svg>"##;
const ARROW_NARROW_DOWN_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5v.5m0 3v1.5m0 3v6" /> <path d="M16 15l-4 4" /> <path d="M8 15l4 4" /> </svg>"##;
const ARROW_NARROW_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12l14 0" /> <path d="M5 12l4 4" /> <path d="M5 12l4 -4" /> </svg>"##;
const ARROW_NARROW_LEFT_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h6m3 0h1.5m3 0h.5" /> <path d="M5 12l4 4" /> <path d="M5 12l4 -4" /> </svg>"##;
const ARROW_NARROW_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12l14 0" /> <path d="M15 16l4 -4" /> <path d="M15 8l4 4" /> </svg>"##;
const ARROW_NARROW_RIGHT_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h.5m3 0h1.5m3 0h6" /> <path d="M15 16l4 -4" /> <path d="M15 8l4 4" /> </svg>"##;
const ARROW_NARROW_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5l0 14" /> <path d="M16 9l-4 -4" /> <path d="M8 9l4 -4" /> </svg>"##;
const ARROW_NARROW_UP_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5v6m0 3v1.5m0 3v.5" /> <path d="M16 9l-4 -4" /> <path d="M8 9l4 -4" /> </svg>"##;
const ARROW_RAMP_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 3l0 8.707" /> <path d="M13 7l4 -4l4 4" /> <path d="M7 14l-4 -4l4 -4" /> <path d="M17 21a11 11 0 0 0 -11 -11h-3" /> </svg>"##;
const ARROW_RAMP_LEFT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 3v8.707" /> <path d="M8 14l-4 -4l4 -4" /> <path d="M18 21c0 -6.075 -4.925 -11 -11 -11h-3" /> </svg>"##;
const ARROW_RAMP_LEFT_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 3v6" /> <path d="M8 16l-4 -4l4 -4" /> <path d="M18 21v-6a3 3 0 0 0 -3 -3h-11" /> </svg>"##;
const ARROW_RAMP_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3l0 8.707" /> <path d="M11 7l-4 -4l-4 4" /> <path d="M17 14l4 -4l-4 -4" /> <path d="M7 21a11 11 0 0 1 11 -11h3" /> </svg>"##;
const ARROW_RAMP_RIGHT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 3v8.707" /> <path d="M16 14l4 -4l-4 -4" /> <path d="M6 21c0 -6.075 4.925 -11 11 -11h3" /> </svg>"##;
const ARROW_RAMP_RIGHT_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 3v6" /> <path d="M16 16l4 -4l-4 -4" /> <path d="M6 21v-6a3 3 0 0 1 3 -3h11" /> </svg>"##;
const ARROW_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12l14 0" /> <path d="M13 18l6 -6" /> <path d="M13 6l6 6" /> </svg>"##;
const ARROW_RIGHT_BAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 15l3 -3l-3 -3" /> <path d="M3 12h18" /> <path d="M3 9v6" /> </svg>"##;
const ARROW_RIGHT_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 15l3 -3l-3 -3" /> <path d="M3 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 12h14" /> </svg>"##;
const ARROW_RIGHT_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h.5m3 0h1.5m3 0h6" /> <path d="M13 18l6 -6" /> <path d="M13 6l6 6" /> </svg>"##;
const ARROW_RIGHT_FROM_ARC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 12h-12" /> <path d="M7 8l-4 4l4 4" /> <path d="M12 21a9 9 0 0 0 0 -18" /> </svg>"##;
const ARROW_RIGHT_RHOMBUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12h13" /> <path d="M18 9l3 3l-3 3" /> <path d="M5.5 9.5l-2.5 2.5l2.5 2.5l2.5 -2.5l-2.5 -2.5" /> </svg>"##;
const ARROW_RIGHT_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 12l14 0" /> <path d="M18 15l3 -3l-3 -3" /> <path d="M3 10h4v4h-4l0 -4" /> </svg>"##;
const ARROW_RIGHT_TAIL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 15l3 -3l-3 -3" /> <path d="M3 15l3 -3l-3 -3" /> <path d="M6 12l15 0" /> </svg>"##;
const ARROW_RIGHT_TO_ARC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h12" /> <path d="M11 8l4 4l-4 4" /> <path d="M12 21a9 9 0 0 0 0 -18" /> </svg>"##;
const ARROW_ROTARY_FIRST_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 10a3 3 0 1 1 0 -6a3 3 0 0 1 0 6" /> <path d="M16 10v10" /> <path d="M13.5 9.5l-8.5 8.5" /> <path d="M10 18h-5v-5" /> </svg>"##;
const ARROW_ROTARY_FIRST_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M8 10v10" /> <path d="M10.5 9.5l8.5 8.5" /> <path d="M14 18h5v-5" /> </svg>"##;
const ARROW_ROTARY_LAST_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 15a3 3 0 1 1 0 -6a3 3 0 0 1 0 6" /> <path d="M15 15v6" /> <path d="M12.5 9.5l-6.5 -6.5" /> <path d="M11 3h-5v5" /> </svg>"##;
const ARROW_ROTARY_LAST_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M9 15v6" /> <path d="M11.5 9.5l6.5 -6.5" /> <path d="M13 3h5v5" /> </svg>"##;
const ARROW_ROTARY_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 10a3 3 0 1 1 0 -6a3 3 0 0 1 0 6" /> <path d="M16 10v10" /> <path d="M13 7h-10" /> <path d="M7 11l-4 -4l4 -4" /> </svg>"##;
const ARROW_ROTARY_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M8 10v10" /> <path d="M17 11l4 -4l-4 -4" /> <path d="M11 7h10" /> </svg>"##;
const ARROW_ROTARY_STRAIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 13a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M13 16v5" /> <path d="M13 3v7" /> <path d="M9 7l4 -4l4 4" /> </svg>"##;
const ARROW_ROUNDABOUT_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9h8a5 5 0 1 1 5 5v7" /> <path d="M7 5l-4 4l4 4" /> </svg>"##;
const ARROW_ROUNDABOUT_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 9h-8a5 5 0 1 0 -5 5v7" /> <path d="M17 5l4 4l-4 4" /> </svg>"##;
const ARROW_SHARP_TURN_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 18v-11.31a.7 .7 0 0 0 -1.195 -.495l-9.805 9.805" /> <path d="M11 16h-5v-5" /> </svg>"##;
const ARROW_SHARP_TURN_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18v-11.31a.7 .7 0 0 1 1.195 -.495l9.805 9.805" /> <path d="M13 16h5v-5" /> </svg>"##;
const ARROW_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5l0 14" /> <path d="M18 11l-6 -6" /> <path d="M6 11l6 -6" /> </svg>"##;
const ARROW_UP_BAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21l0 -18" /> <path d="M15 6l-3 -3l-3 3" /> <path d="M9 21l6 0" /> </svg>"##;
const ARROW_UP_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17v-14" /> <path d="M15 6l-3 -3l-3 3" /> <path d="M12 17a2 2 0 1 0 0 4a2 2 0 0 0 0 -4" /> </svg>"##;
const ARROW_UP_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5v6m0 3v1.5m0 3v.5" /> <path d="M18 11l-6 -6" /> <path d="M6 11l6 -6" /> </svg>"##;
const ARROW_UP_FROM_ARC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9v12" /> <path d="M8 17l4 4l4 -4" /> <path d="M21 12a9 9 0 0 0 -18 0" /> </svg>"##;
const ARROW_UP_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7l10 10" /> <path d="M16 7l-9 0l0 9" /> </svg>"##;
const ARROW_UP_LEFT_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.536 15.536l-9.536 -9.536" /> <path d="M10 6h-4v4" /> <path d="M15.586 15.586a2 2 0 1 0 2.828 2.828a2 2 0 0 0 -2.828 -2.828" /> </svg>"##;
const ARROW_UP_RHOMBUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 16v-13" /> <path d="M15 6l-3 -3l-3 3" /> <path d="M14.5 18.5l-2.5 2.5l-2.5 -2.5l2.5 -2.5l2.5 2.5" /> </svg>"##;
const ARROW_UP_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 7l-10 10" /> <path d="M8 7l9 0l0 9" /> </svg>"##;
const ARROW_UP_RIGHT_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.464 15.536l9.536 -9.536" /> <path d="M18 10v-4h-4" /> <path d="M8.414 15.586a2 2 0 1 0 -2.828 2.828a2 2 0 0 0 2.828 -2.828" /> </svg>"##;
const ARROW_UP_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17l0 -14" /> <path d="M15 6l-3 -3l-3 3" /> <path d="M10 21v-4h4v4l-4 0" /> </svg>"##;
const ARROW_UP_TAIL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l0 -15" /> <path d="M15 6l-3 -3l-3 3" /> <path d="M15 21l-3 -3l-3 3" /> </svg>"##;
const ARROW_UP_TO_ARC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21v-12" /> <path d="M8 13l4 -4l4 4" /> <path d="M21 12a9 9 0 0 0 -18 0" /> </svg>"##;
const ARROW_WAVE_LEFT_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 14h-4v-4" /> <path d="M21 12c-.887 1.284 -2.48 2.033 -4 2c-1.52 .033 -3.113 -.716 -4 -2s-2.48 -2.033 -4 -2c-1.52 -.033 -3 1 -4 2l-2 2" /> </svg>"##;
const ARROW_WAVE_LEFT_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 10h-4v4" /> <path d="M21 12c-.887 -1.285 -2.48 -2.033 -4 -2c-1.52 -.033 -3.113 .715 -4 2c-.887 1.284 -2.48 2.033 -4 2c-1.52 .033 -3 -1 -4 -2l-2 -2" /> </svg>"##;
const ARROW_WAVE_RIGHT_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 14h4v-4" /> <path d="M3 12c.887 1.284 2.48 2.033 4 2c1.52 .033 3.113 -.716 4 -2s2.48 -2.033 4 -2c1.52 -.033 3 1 4 2l2 2" /> </svg>"##;
const ARROW_WAVE_RIGHT_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10h4v4" /> <path d="M3 12c.887 -1.284 2.48 -2.033 4 -2c1.52 -.033 3.113 .716 4 2s2.48 2.033 4 2c1.52 .033 3 -1 4 -2l2 -2" /> </svg>"##;
const ARROW_ZIG_ZAG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 20v-10l10 6v-12" /> <path d="M13 7l3 -3l3 3" /> </svg>"##;
const ARROWS_CROSS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 4h4v4" /> <path d="M15 9l5 -5" /> <path d="M4 20l5 -5" /> <path d="M16 20h4v-4" /> <path d="M4 4l16 16" /> </svg>"##;
const ARROWS_DIAGONAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 4l4 0l0 4" /> <path d="M14 10l6 -6" /> <path d="M8 20l-4 0l0 -4" /> <path d="M4 20l6 -6" /> </svg>"##;
const ARROWS_DIAGONAL_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 20l4 0l0 -4" /> <path d="M14 14l6 6" /> <path d="M8 4l-4 0l0 4" /> <path d="M4 4l6 6" /> </svg>"##;
const ARROWS_DIAGONAL_MINIMIZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 10h4v-4" /> <path d="M4 4l6 6" /> <path d="M18 14h-4v4" /> <path d="M14 14l6 6" /> </svg>"##;
const ARROWS_DIAGONAL_MINIMIZE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 10h-4v-4" /> <path d="M20 4l-6 6" /> <path d="M6 14h4v4" /> <path d="M10 14l-6 6" /> </svg>"##;
const ARROWS_DIFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 16h10" /> <path d="M11 16l4 4" /> <path d="M11 16l4 -4" /> <path d="M13 8h-10" /> <path d="M13 8l-4 4" /> <path d="M13 8l-4 -4" /> </svg>"##;
const ARROWS_DOUBLE_NE_SW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 14l11 -11" /> <path d="M10 3h4v4" /> <path d="M10 17v4h4" /> <path d="M21 10l-11 11" /> </svg>"##;
const ARROWS_DOUBLE_NW_SE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 21l-11 -11" /> <path d="M3 14v-4h4" /> <path d="M17 14h4v-4" /> <path d="M10 3l11 11" /> </svg>"##;
const ARROWS_DOUBLE_SE_NW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10l11 11" /> <path d="M14 17v4h-4" /> <path d="M14 3h-4v4" /> <path d="M21 14l-11 -11" /> </svg>"##;
const ARROWS_DOUBLE_SW_NE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3l-11 11" /> <path d="M3 10v4h4" /> <path d="M17 10h4v4" /> <path d="M10 21l11 -11" /> </svg>"##;
const ARROWS_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 21l0 -18" /> <path d="M20 18l-3 3l-3 -3" /> <path d="M4 18l3 3l3 -3" /> <path d="M17 21l0 -18" /> </svg>"##;
const ARROWS_DOWN_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 3l0 18" /> <path d="M10 18l-3 3l-3 -3" /> <path d="M7 21l0 -18" /> <path d="M20 6l-3 -3l-3 3" /> </svg>"##;
const ARROWS_EXCHANGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 10h14l-4 -4" /> <path d="M17 14h-14l4 4" /> </svg>"##;
const ARROWS_EXCHANGE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10h-14l4 -4" /> <path d="M7 14h14l-4 4" /> </svg>"##;
const ARROWS_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8l-4 4l4 4" /> <path d="M17 8l4 4l-4 4" /> <path d="M3 12l18 0" /> </svg>"##;
const ARROWS_JOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7h5l3.5 5h9.5" /> <path d="M3 17h5l3.495 -5" /> <path d="M18 15l3 -3l-3 -3" /> </svg>"##;
const ARROWS_JOIN_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7h1.948c1.913 0 3.705 .933 4.802 2.5a5.861 5.861 0 0 0 4.802 2.5h6.448" /> <path d="M3 17h1.95a5.854 5.854 0 0 0 4.798 -2.5a5.854 5.854 0 0 1 4.798 -2.5h5.454" /> <path d="M18 15l3 -3l-3 -3" /> </svg>"##;
const ARROWS_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7l18 0" /> <path d="M6 20l-3 -3l3 -3" /> <path d="M6 4l-3 3l3 3" /> <path d="M3 17l18 0" /> </svg>"##;
const ARROWS_LEFT_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3l-4 4l4 4" /> <path d="M3 7h11a3 3 0 0 1 3 3v11" /> <path d="M13 17l4 4l4 -4" /> </svg>"##;
const ARROWS_LEFT_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 17l-18 0" /> <path d="M6 10l-3 -3l3 -3" /> <path d="M3 7l18 0" /> <path d="M18 20l3 -3l-3 -3" /> </svg>"##;
const ARROWS_MAXIMIZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 4l4 0l0 4" /> <path d="M14 10l6 -6" /> <path d="M8 20l-4 0l0 -4" /> <path d="M4 20l6 -6" /> <path d="M16 20l4 0l0 -4" /> <path d="M14 14l6 6" /> <path d="M8 4l-4 0l0 4" /> <path d="M4 4l6 6" /> </svg>"##;
const ARROWS_MINIMIZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 9l4 0l0 -4" /> <path d="M3 3l6 6" /> <path d="M5 15l4 0l0 4" /> <path d="M3 21l6 -6" /> <path d="M19 9l-4 0l0 -4" /> <path d="M15 9l6 -6" /> <path d="M19 15l-4 0l0 4" /> <path d="M15 15l6 6" /> </svg>"##;
const ARROWS_MOVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9l3 3l-3 3" /> <path d="M15 12h6" /> <path d="M6 9l-3 3l3 3" /> <path d="M3 12h6" /> <path d="M9 18l3 3l3 -3" /> <path d="M12 15v6" /> <path d="M15 6l-3 -3l-3 3" /> <path d="M12 3v6" /> </svg>"##;
const ARROWS_MOVE_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 9l3 3l-3 3" /> <path d="M15 12h6" /> <path d="M6 9l-3 3l3 3" /> <path d="M3 12h6" /> </svg>"##;
const ARROWS_MOVE_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 18l3 3l3 -3" /> <path d="M12 15v6" /> <path d="M15 6l-3 -3l-3 3" /> <path d="M12 3v6" /> </svg>"##;
const ARROWS_RANDOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 21h-4v-4" /> <path d="M16 21l5 -5" /> <path d="M6.5 9.504l-3.5 -2l2 -3.504" /> <path d="M3 7.504l6.83 -1.87" /> <path d="M4 16l4 -1l1 4" /> <path d="M8 15l-3.5 6" /> <path d="M21 5l-.5 4l-4 -.5" /> <path d="M20.5 9l-4.5 -5.5" /> </svg>"##;
const ARROWS_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 17l-18 0" /> <path d="M18 4l3 3l-3 3" /> <path d="M18 20l3 -3l-3 -3" /> <path d="M21 7l-18 0" /> </svg>"##;
const ARROWS_RIGHT_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17l4 4l4 -4" /> <path d="M7 21v-11a3 3 0 0 1 3 -3h11" /> <path d="M17 11l4 -4l-4 -4" /> </svg>"##;
const ARROWS_RIGHT_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 7l-18 0" /> <path d="M18 10l3 -3l-3 -3" /> <path d="M6 20l-3 -3l3 -3" /> <path d="M3 17l18 0" /> </svg>"##;
const ARROWS_SHUFFLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 4l3 3l-3 3" /> <path d="M18 20l3 -3l-3 -3" /> <path d="M3 7h3a5 5 0 0 1 5 5a5 5 0 0 0 5 5h5" /> <path d="M21 7h-5a4.978 4.978 0 0 0 -3 1m-4 8a4.984 4.984 0 0 1 -3 1h-3" /> </svg>"##;
const ARROWS_SHUFFLE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 4l3 3l-3 3" /> <path d="M18 20l3 -3l-3 -3" /> <path d="M3 7h3a5 5 0 0 1 5 5a5 5 0 0 0 5 5h5" /> <path d="M3 17h3a5 5 0 0 0 5 -5a5 5 0 0 1 5 -5h5" /> </svg>"##;
const ARROWS_SORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9l4 -4l4 4m-4 -4v14" /> <path d="M21 15l-4 4l-4 -4m4 4v-14" /> </svg>"##;
const ARROWS_SPLIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 17h-8l-3.5 -5h-6.5" /> <path d="M21 7h-8l-3.495 5" /> <path d="M18 10l3 -3l-3 -3" /> <path d="M18 20l3 -3l-3 -3" /> </svg>"##;
const ARROWS_SPLIT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 17h-5.397a5 5 0 0 1 -4.096 -2.133l-.514 -.734a5 5 0 0 0 -4.096 -2.133h-3.897" /> <path d="M21 7h-5.395a5 5 0 0 0 -4.098 2.135l-.51 .73a5 5 0 0 1 -4.097 2.135h-3.9" /> <path d="M18 10l3 -3l-3 -3" /> <path d="M18 20l3 -3l-3 -3" /> </svg>"##;
const ARROWS_TRANSFER_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 3v6" /> <path d="M10 18l-3 3l-3 -3" /> <path d="M7 21v-18" /> <path d="M20 6l-3 -3l-3 3" /> <path d="M17 21v-2" /> <path d="M17 15v-2" /> </svg>"##;
const ARROWS_TRANSFER_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 21v-6" /> <path d="M20 6l-3 -3l-3 3" /> <path d="M17 3v18" /> <path d="M10 18l-3 3l-3 -3" /> <path d="M7 3v2" /> <path d="M7 9v2" /> </svg>"##;
const ARROWS_TRANSFER_UP_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 21v-6" /> <path d="M20 6l-3 -3l-3 3" /> <path d="M10 18l-3 3l-3 -3" /> <path d="M7 3v2" /> <path d="M7 9v2" /> <path d="M17 3v6" /> <path d="M17 21v-2" /> <path d="M17 15v-2" /> </svg>"##;
const ARROWS_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 3l0 18" /> <path d="M4 6l3 -3l3 3" /> <path d="M20 6l-3 -3l-3 3" /> <path d="M7 3l0 18" /> </svg>"##;
const ARROWS_UP_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3l0 18" /> <path d="M10 6l-3 -3l-3 3" /> <path d="M20 18l-3 3l-3 -3" /> <path d="M17 21l0 -18" /> </svg>"##;
const ARROWS_UP_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 7l-4 -4l-4 4" /> <path d="M17 3v11a3 3 0 0 1 -3 3h-11" /> <path d="M7 13l-4 4l4 4" /> </svg>"##;
const ARROWS_UP_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 21l4 -4l-4 -4" /> <path d="M21 17h-11a3 3 0 0 1 -3 -3v-11" /> <path d="M11 7l-4 -4l-4 4" /> </svg>"##;
const ARROWS_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7l4 -4l4 4" /> <path d="M8 17l4 4l4 -4" /> <path d="M12 3l0 18" /> </svg>"##;
const AXIS_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 13v.01" /> <path d="M4 9v.01" /> <path d="M4 5v.01" /> <path d="M17 20l3 -3l-3 -3" /> <path d="M4 17h16" /> </svg>"##;
const AXIS_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 20h-.01" /> <path d="M15 20h-.01" /> <path d="M19 20h-.01" /> <path d="M4 7l3 -3l3 3" /> <path d="M7 20v-16" /> </svg>"##;
const CARET_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 10l6 6l6 -6h-12" /> </svg>"##;
const CARET_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 6l-6 6l6 6v-12" /> </svg>"##;
const CARET_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 18l6 -6l-6 -6v12" /> </svg>"##;
const CARET_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 14l-6 -6l-6 6h12" /> </svg>"##;
const CARET_UP_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 10l-6 -6l-6 6h12" /> <path d="M18 14l-6 6l-6 -6h12" /> </svg>"##;
const CHEVRON_COMPACT_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 11l8 3l8 -3" /> </svg>"##;
const CHEVRON_COMPACT_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 20l-3 -8l3 -8" /> </svg>"##;
const CHEVRON_COMPACT_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 4l3 8l-3 8" /> </svg>"##;
const CHEVRON_COMPACT_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 13l8 -3l8 3" /> </svg>"##;
const CHEVRON_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 9l6 6l6 -6" /> </svg>"##;
const CHEVRON_DOWN_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8v8h8" /> </svg>"##;
const CHEVRON_DOWN_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 8v8h-8" /> </svg>"##;
const CHEVRON_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 6l-6 6l6 6" /> </svg>"##;
const CHEVRON_LEFT_PIPE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 6v12" /> <path d="M18 6l-6 6l6 6" /> </svg>"##;
const CHEVRON_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 6l6 6l-6 6" /> </svg>"##;
const CHEVRON_RIGHT_PIPE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 6l6 6l-6 6" /> <path d="M17 5v13" /> </svg>"##;
const CHEVRON_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 15l6 -6l6 6" /> </svg>"##;
const CHEVRON_UP_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16v-8h8" /> </svg>"##;
const CHEVRON_UP_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h8v8" /> </svg>"##;
const CHEVRONS_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7l5 5l5 -5" /> <path d="M7 13l5 5l5 -5" /> </svg>"##;
const CHEVRONS_DOWN_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 5v8h8" /> <path d="M7 9v8h8" /> </svg>"##;
const CHEVRONS_DOWN_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 5v8h-8" /> <path d="M17 9v8h-8" /> </svg>"##;
const CHEVRONS_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 7l-5 5l5 5" /> <path d="M17 7l-5 5l5 5" /> </svg>"##;
const CHEVRONS_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7l5 5l-5 5" /> <path d="M13 7l5 5l-5 5" /> </svg>"##;
const CHEVRONS_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 11l5 -5l5 5" /> <path d="M7 17l5 -5l5 5" /> </svg>"##;
const CHEVRONS_UP_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 15v-8h8" /> <path d="M11 19v-8h8" /> </svg>"##;
const CHEVRONS_UP_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 7h8v8" /> <path d="M5 11h8v8" /> </svg>"##;
const CIRCLE_ARROW_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M8 12l4 4" /> <path d="M12 8v8" /> <path d="M16 12l-4 4" /> </svg>"##;
const CIRCLE_ARROW_DOWN_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M15 9l-6 6" /> <path d="M15 15h-6v-6" /> </svg>"##;
const CIRCLE_ARROW_DOWN_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M15 15h-6" /> <path d="M15 9v6l-6 -6" /> </svg>"##;
const CIRCLE_ARROW_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 1 0 0 -18a9 9 0 0 0 0 18" /> <path d="M8 12l4 4" /> <path d="M8 12h8" /> <path d="M12 8l-4 4" /> </svg>"##;
const CIRCLE_ARROW_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a9 9 0 1 0 0 18a9 9 0 0 0 0 -18" /> <path d="M16 12l-4 -4" /> <path d="M16 12h-8" /> <path d="M12 16l4 -4" /> </svg>"##;
const CIRCLE_ARROW_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M12 8l-4 4" /> <path d="M12 8v8" /> <path d="M16 12l-4 -4" /> </svg>"##;
const CIRCLE_ARROW_UP_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M9 9l6 6" /> <path d="M15 9h-6v6" /> </svg>"##;
const CIRCLE_ARROW_UP_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M15 9l-6 6" /> <path d="M15 15v-6h-6" /> </svg>"##;
const CIRCLE_CARET_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 15l-4 -4h8l-4 4" /> </svg>"##;
const CIRCLE_CARET_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12l4 -4v8l-4 -4" /> <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> </svg>"##;
const CIRCLE_CARET_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 12l-4 -4v8l4 -4" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CIRCLE_CARET_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9l4 4h-8l4 -4" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CIRCLE_CHEVRON_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11l-3 3l-3 -3" /> <path d="M12 3a9 9 0 1 0 0 18a9 9 0 0 0 0 -18" /> </svg>"##;
const CIRCLE_CHEVRON_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 15l-3 -3l3 -3" /> <path d="M21 12a9 9 0 1 0 -18 0a9 9 0 0 0 18 0" /> </svg>"##;
const CIRCLE_CHEVRON_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 9l3 3l-3 3" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const CIRCLE_CHEVRON_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 13l3 -3l3 3" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CIRCLE_CHEVRONS_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 9l-3 3l-3 -3" /> <path d="M15 13l-3 3l-3 -3" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CIRCLE_CHEVRONS_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 15l-3 -3l3 -3" /> <path d="M11 15l-3 -3l3 -3" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CIRCLE_CHEVRONS_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 9l3 3l-3 3" /> <path d="M13 9l3 3l-3 3" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CIRCLE_CHEVRONS_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l3 -3l3 3" /> <path d="M9 11l3 -3l3 3" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CIRCLE_OPEN_ARROW_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.998 3.934a9 9 0 1 1 -3.998 -.934v13" /> <path d="M16 12l-4 4l-4 -4" /> </svg>"##;
const CIRCLE_OPEN_ARROW_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.066 8.002a9 9 0 1 0 .934 3.998h-13" /> <path d="M12 8l-4 4l4 4" /> </svg>"##;
const CIRCLE_OPEN_ARROW_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.934 8.002a9 9 0 1 1 -.934 3.998h13" /> <path d="M12 8l4 4l-4 4" /> </svg>"##;
const CIRCLE_OPEN_ARROW_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.998 20.066a9 9 0 1 0 -3.998 .934v-13" /> <path d="M16 12l-4 -4l-4 4" /> </svg>"##;
const CORNER_DOWN_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 6v6a3 3 0 0 1 -3 3h-10l4 -4m0 8l-4 -4" /> </svg>"##;
const CORNER_DOWN_LEFT_DOUBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v6a3 3 0 0 1 -3 3h-7" /> <path d="M13 10l-4 4l4 4m-5 -8l-4 4l4 4" /> </svg>"##;
const CORNER_DOWN_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 6v6a3 3 0 0 0 3 3h10l-4 -4m0 8l4 -4" /> </svg>"##;
const CORNER_DOWN_RIGHT_DOUBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5v6a3 3 0 0 0 3 3h7" /> <path d="M10 10l4 4l-4 4m5 -8l4 4l-4 4" /> </svg>"##;
const CORNER_LEFT_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 6h-6a3 3 0 0 0 -3 3v10l-4 -4m8 0l-4 4" /> </svg>"##;
const CORNER_LEFT_DOWN_DOUBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 4h-6a3 3 0 0 0 -3 3v7" /> <path d="M13 10l-4 4l-4 -4m8 5l-4 4l-4 -4" /> </svg>"##;
const CORNER_LEFT_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 18h-6a3 3 0 0 1 -3 -3v-10l-4 4m8 0l-4 -4" /> </svg>"##;
const CORNER_LEFT_UP_DOUBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 19h-6a3 3 0 0 1 -3 -3v-7" /> <path d="M13 13l-4 -4l-4 4m8 -5l-4 -4l-4 4" /> </svg>"##;
const CORNER_RIGHT_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 6h6a3 3 0 0 1 3 3v10l-4 -4m8 0l-4 4" /> </svg>"##;
const CORNER_RIGHT_DOWN_DOUBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h6a3 3 0 0 1 3 3v7" /> <path d="M10 10l4 4l4 -4m-8 5l4 4l4 -4" /> </svg>"##;
const CORNER_RIGHT_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 18h6a3 3 0 0 0 3 -3v-10l-4 4m8 0l-4 -4" /> </svg>"##;
const CORNER_RIGHT_UP_DOUBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 19h6a3 3 0 0 0 3 -3v-7" /> <path d="M10 13l4 -4l4 4m-8 -5l4 -4l4 4" /> </svg>"##;
const CORNER_UP_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 18v-6a3 3 0 0 0 -3 -3h-10l4 -4m0 8l-4 -4" /> </svg>"##;
const CORNER_UP_LEFT_DOUBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 18v-6a3 3 0 0 0 -3 -3h-7" /> <path d="M13 13l-4 -4l4 -4m-5 8l-4 -4l4 -4" /> </svg>"##;
const CORNER_UP_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 18v-6a3 3 0 0 1 3 -3h10l-4 -4m0 8l4 -4" /> </svg>"##;
const CORNER_UP_RIGHT_DOUBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18v-6a3 3 0 0 1 3 -3h7" /> <path d="M10 13l4 -4l-4 -4m5 8l4 -4l-4 -4" /> </svg>"##;
const DOWNLOAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 17v2a2 2 0 0 0 2 2h12a2 2 0 0 0 2 -2v-2" /> <path d="M7 11l5 5l5 -5" /> <path d="M12 4l0 12" /> </svg>"##;
const DOWNLOAD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 17v2a2 2 0 0 0 2 2h12a2 2 0 0 0 1.83 -1.19" /> <path d="M7 11l5 5l2 -2m2 -2l1 -1" /> <path d="M12 4v4m0 4v4" /> <path d="M3 3l18 18" /> </svg>"##;
const FOLD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3v6l3 -3m-6 0l3 3" /> <path d="M12 21v-6l3 3m-6 0l3 -3" /> <path d="M4 12l1 0" /> <path d="M9 12l1 0" /> <path d="M14 12l1 0" /> <path d="M19 12l1 0" /> </svg>"##;
const FOLD_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 11v8l3 -3m-6 0l3 3" /> <path d="M9 7l1 0" /> <path d="M14 7l1 0" /> <path d="M19 7l1 0" /> <path d="M4 7l1 0" /> </svg>"##;
const FOLD_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 13v-8l-3 3m6 0l-3 -3" /> <path d="M9 17l1 0" /> <path d="M14 17l1 0" /> <path d="M19 17l1 0" /> <path d="M4 17l1 0" /> </svg>"##;
const LOGIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8v-2a2 2 0 0 0 -2 -2h-7a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h7a2 2 0 0 0 2 -2v-2" /> <path d="M21 12h-13l3 -3" /> <path d="M11 15l-3 -3" /> </svg>"##;
const LOGIN_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 8v-2a2 2 0 0 1 2 -2h7a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-7a2 2 0 0 1 -2 -2v-2" /> <path d="M3 12h13l-3 -3" /> <path d="M13 15l3 -3" /> </svg>"##;
const REFRESH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -4v4h4" /> <path d="M4 13a8.1 8.1 0 0 0 15.5 2m.5 4v-4h-4" /> </svg>"##;
const REFRESH_ALERT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -4v4h4" /> <path d="M4 13a8.1 8.1 0 0 0 15.5 2m.5 4v-4h-4" /> <path d="M12 9l0 3" /> <path d="M12 15l.01 0" /> </svg>"##;
const REFRESH_DOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -4v4h4" /> <path d="M4 13a8.1 8.1 0 0 0 15.5 2m.5 4v-4h-4" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const REFRESH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 11a8.1 8.1 0 0 0 -11.271 -6.305m-2.41 1.624a8.083 8.083 0 0 0 -1.819 2.681m-.5 -4v4h4" /> <path d="M4 13a8.1 8.1 0 0 0 13.671 4.691m2.329 -1.691v-1h-1" /> <path d="M3 3l18 18" /> </svg>"##;
const RELOAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.933 13.041a8 8 0 1 1 -9.925 -8.788c3.899 -1 7.935 1.007 9.425 4.747" /> <path d="M20 4v5h-5" /> </svg>"##;
const RIPPLE_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7q 4.5 -3 9 0t 9 0" /> <path d="M3 17q 4.5 -3 9 0q .213 .142 .427 .27" /> <path d="M3 12q 4.5 -3 9 0q 2.006 1.338 4.012 1.482" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const RIPPLE_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7q 4.5 -3 9 0t 9 0" /> <path d="M3 17q 4.5 -3 9 0q .218 .144 .434 .275" /> <path d="M3 12q 4.5 -3 9 0q 1.941 1.294 3.882 1.472" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const ROTATE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.95 11a8 8 0 1 0 -.5 4m.5 5v-5h-5" /> </svg>"##;
const ROTATE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 4.55a8 8 0 0 0 -6 14.9m0 -4.45v5h-5" /> <path d="M18.37 7.16l0 .01" /> <path d="M13 19.94l0 .01" /> <path d="M16.84 18.37l0 .01" /> <path d="M19.37 15.1l0 .01" /> <path d="M19.94 11l0 .01" /> </svg>"##;
const ROTATE_360_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 16h4v4" /> <path d="M19.458 11.042c.86 -2.366 .722 -4.58 -.6 -5.9c-2.272 -2.274 -7.185 -1.045 -10.973 2.743c-3.788 3.788 -5.017 8.701 -2.744 10.974c2.227 2.226 6.987 1.093 10.74 -2.515" /> </svg>"##;
const ROTATE_CLOCKWISE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.05 11a8 8 0 1 1 .5 4m-.5 5v-5h5" /> </svg>"##;
const ROTATE_CLOCKWISE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 4.55a8 8 0 0 1 6 14.9m0 -4.45v5h5" /> <path d="M5.63 7.16l0 .01" /> <path d="M4.06 11l0 .01" /> <path d="M4.63 15.1l0 .01" /> <path d="M7.16 18.37l0 .01" /> <path d="M11 19.94l0 .01" /> </svg>"##;
const ROTATE_DOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.95 11a8 8 0 1 0 -.5 4m.5 5v-5h-5" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const ROTATE_RECTANGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.09 4.01l.496 -.495a2 2 0 0 1 2.828 0l7.071 7.07a2 2 0 0 1 0 2.83l-7.07 7.07a2 2 0 0 1 -2.83 0l-7.07 -7.07a2 2 0 0 1 0 -2.83l3.535 -3.535h-3.988" /> <path d="M7.05 11.038v-3.988" /> </svg>"##;
const S_TURN_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5a2 2 0 1 1 -4 0a2 2 0 0 1 4 0" /> <path d="M5 7v9.5a3.5 3.5 0 0 0 7 0v-9a3.5 3.5 0 0 1 7 0v13.5" /> <path d="M16 18l3 3l3 -3" /> </svg>"##;
const S_TURN_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 7a2 2 0 1 1 0 -4a2 2 0 0 1 0 4" /> <path d="M17 5h-9.5a3.5 3.5 0 0 0 0 7h9a3.5 3.5 0 0 1 0 7h-13.5" /> <path d="M6 16l-3 3l3 3" /> </svg>"##;
const S_TURN_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 5h9.5a3.5 3.5 0 0 1 0 7h-9a3.5 3.5 0 0 0 0 7h13.5" /> <path d="M18 16l3 3l-3 3" /> </svg>"##;
const S_TURN_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 19a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M5 17v-9.5a3.5 3.5 0 0 1 7 0v9a3.5 3.5 0 0 0 7 0v-13.5" /> <path d="M16 6l3 -3l3 3" /> </svg>"##;
const SELECT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 11l3 3l3 -3" /> </svg>"##;
const SELECTOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9l4 -4l4 4" /> <path d="M16 15l-4 4l-4 -4" /> </svg>"##;
const SHARE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9h-1a2 2 0 0 0 -2 2v8a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-8a2 2 0 0 0 -2 -2h-1" /> <path d="M12 14v-11" /> <path d="M9 6l3 -3l3 3" /> </svg>"##;
const SHARE_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 4v4c-6.575 1.028 -9.02 6.788 -10 12c-.037 .206 5.384 -5.962 10 -6v4l8 -7l-8 -7" /> </svg>"##;
const SQUARE_ARROW_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12l4 4l4 -4" /> <path d="M12 8v8" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_ARROW_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8l-4 4l4 4" /> <path d="M16 12h-8" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_ARROW_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 16l4 -4l-4 -4" /> <path d="M8 12h8" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_ARROW_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12l-4 -4l-4 4" /> <path d="M12 16v-8" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_CHEVRON_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11l-3 3l-3 -3" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_CHEVRON_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 15l-3 -3l3 -3" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_CHEVRON_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 9l3 3l-3 3" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_CHEVRON_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 13l3 -3l3 3" /> </svg>"##;
const SQUARE_CHEVRONS_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8l-3 3l-3 -3" /> <path d="M15 13l-3 3l-3 -3" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_CHEVRONS_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 15l-3 -3l3 -3" /> <path d="M11 15l-3 -3l3 -3" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_CHEVRONS_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9l3 3l-3 3" /> <path d="M13 9l3 3l-3 3" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_CHEVRONS_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 16l3 -3l3 3" /> <path d="M9 11l3 -3l3 3" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_ROUNDED_ARROW_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12l4 4l4 -4" /> <path d="M12 8v8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_ARROW_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8l-4 4l4 4" /> <path d="M16 12h-8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_ARROW_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 16l4 -4l-4 -4" /> <path d="M8 12h8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_ARROW_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12l-4 -4l-4 4" /> <path d="M12 16v-8" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_CHEVRON_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11l-3 3l-3 -3" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_CHEVRON_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 15l-3 -3l3 -3" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_CHEVRON_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 9l3 3l-3 3" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_CHEVRON_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 13l3 -3l3 3" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_CHEVRONS_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 9l-3 3l-3 -3" /> <path d="M15 13l-3 3l-3 -3" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_CHEVRONS_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 15l-3 -3l3 -3" /> <path d="M11 15l-3 -3l3 -3" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_CHEVRONS_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 9l3 3l-3 3" /> <path d="M13 9l3 3l-3 3" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_CHEVRONS_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l3 -3l3 3" /> <path d="M9 11l3 -3l3 3" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const STEP_INTO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3l0 12" /> <path d="M16 11l-4 4" /> <path d="M8 11l4 4" /> <path d="M11 20a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const STEP_OUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3l0 12" /> <path d="M16 7l-4 -4" /> <path d="M8 7l4 -4" /> <path d="M11 20a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const SWIPE_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4a4 4 0 1 1 0 8a4 4 0 0 1 0 -8" /> <path d="M12 12v8" /> <path d="M9 17l3 3l3 -3" /> </svg>"##;
const SWIPE_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 12a4 4 0 1 0 -8 0a4 4 0 0 0 8 0" /> <path d="M12 12h-8" /> <path d="M7 15l-3 -3l3 -3" /> </svg>"##;
const SWIPE_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12a4 4 0 1 1 8 0a4 4 0 0 1 -8 0" /> <path d="M12 12h8" /> <path d="M17 15l3 -3l-3 -3" /> </svg>"##;
const SWIPE_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M12 12v-8" /> <path d="M9 7l3 -3l3 3" /> </svg>"##;
const SWITCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 4l4 0l0 4" /> <path d="M14.75 9.25l4.25 -5.25" /> <path d="M5 19l4 -4" /> <path d="M15 19l4 0l0 -4" /> <path d="M5 5l14 14" /> </svg>"##;
const SWITCH_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17h5l1.67 -2.386m3.66 -5.227l1.67 -2.387h6" /> <path d="M18 4l3 3l-3 3" /> <path d="M3 7h5l7 10h6" /> <path d="M18 20l3 -3l-3 -3" /> </svg>"##;
const SWITCH_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17h2.397a5 5 0 0 0 4.096 -2.133l.177 -.253m3.66 -5.227l.177 -.254a5 5 0 0 1 4.096 -2.133h3.397" /> <path d="M18 4l3 3l-3 3" /> <path d="M3 7h2.397a5 5 0 0 1 4.096 2.133l4.014 5.734a5 5 0 0 0 4.096 2.133h3.397" /> <path d="M18 20l3 -3l-3 -3" /> </svg>"##;
const SWITCH_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 3l4 4l-4 4" /> <path d="M10 7l10 0" /> <path d="M8 13l-4 4l4 4" /> <path d="M4 17l9 0" /> </svg>"##;
const SWITCH_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8l4 -4l4 4" /> <path d="M7 4l0 9" /> <path d="M13 16l4 4l4 -4" /> <path d="M17 10l0 10" /> </svg>"##;
const TRANSITION_BOTTOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 18a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3" /> <path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3" /> <path d="M12 9v8" /> <path d="M9 14l3 3l3 -3" /> </svg>"##;
const TRANSITION_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 21a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3" /> <path d="M21 6v12a3 3 0 0 1 -6 0v-12a3 3 0 0 1 6 0" /> <path d="M15 12h-8" /> <path d="M10 9l-3 3l3 3" /> </svg>"##;
const TRANSITION_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 3a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3" /> <path d="M3 18v-12a3 3 0 1 1 6 0v12a3 3 0 0 1 -6 0" /> <path d="M9 12h8" /> <path d="M14 15l3 -3l-3 -3" /> </svg>"##;
const TRANSITION_TOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 6a3 3 0 0 0 -3 -3h-12a3 3 0 0 0 -3 3" /> <path d="M6 21h12a3 3 0 0 0 0 -6h-12a3 3 0 0 0 0 6" /> <path d="M12 15v-8" /> <path d="M9 10l3 -3l3 3" /> </svg>"##;
const TRENDING_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7l6 6l4 -4l8 8" /> <path d="M21 10l0 7l-7 0" /> </svg>"##;
const TRENDING_DOWN_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6h5l7 10h6" /> <path d="M18 19l3 -3l-3 -3" /> </svg>"##;
const TRENDING_DOWN_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6h2.397a5 5 0 0 1 4.096 2.133l4.014 5.734a5 5 0 0 0 4.096 2.133h3.397" /> <path d="M18 19l3 -3l-3 -3" /> </svg>"##;
const TRENDING_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17l6 -6l4 4l8 -8" /> <path d="M14 7l7 0l0 7" /> </svg>"##;
const TRENDING_UP_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 5l3 3l-3 3" /> <path d="M3 18h5l7 -10h6" /> </svg>"##;
const TRENDING_UP_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 5l3 3l-3 3" /> <path d="M3 18h2.397a5 5 0 0 0 4.096 -2.133l4.014 -5.734a5 5 0 0 1 4.096 -2.133h3.397" /> </svg>"##;
const U_TURN_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 20v-11.5a4.5 4.5 0 1 0 -9 0v8.5" /> <path d="M11 14l-3 3l-3 -3" /> </svg>"##;
const U_TURN_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 20v-11.5a4.5 4.5 0 0 1 9 0v8.5" /> <path d="M13 14l3 3l3 -3" /> </svg>"##;
const UPLOAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 17v2a2 2 0 0 0 2 2h12a2 2 0 0 0 2 -2v-2" /> <path d="M7 9l5 -5l5 5" /> <path d="M12 4l0 12" /> </svg>"##;

/// Arrows icon variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArrowsIcon {
    ArrowAutofitContent,
    ArrowAutofitDown,
    ArrowAutofitHeight,
    ArrowAutofitLeft,
    ArrowAutofitRight,
    ArrowAutofitUp,
    ArrowAutofitWidth,
    ArrowBack,
    ArrowBackUp,
    ArrowBackUpDouble,
    ArrowBadgeDown,
    ArrowBadgeLeft,
    ArrowBadgeRight,
    ArrowBadgeUp,
    ArrowBarBoth,
    ArrowBarDown,
    ArrowBarLeft,
    ArrowBarRight,
    ArrowBarToDown,
    ArrowBarToDownDashed,
    ArrowBarToLeft,
    ArrowBarToLeftDashed,
    ArrowBarToRight,
    ArrowBarToRightDashed,
    ArrowBarToUp,
    ArrowBarToUpDashed,
    ArrowBarUp,
    ArrowBearLeft,
    ArrowBearLeft2,
    ArrowBearRight,
    ArrowBearRight2,
    ArrowBigDown,
    ArrowBigDownLine,
    ArrowBigDownLines,
    ArrowBigLeft,
    ArrowBigLeftLine,
    ArrowBigLeftLines,
    ArrowBigRight,
    ArrowBigRightLine,
    ArrowBigRightLines,
    ArrowBigUp,
    ArrowBigUpLine,
    ArrowBigUpLines,
    ArrowBounce,
    ArrowCapsule,
    ArrowCurveLeft,
    ArrowCurveRight,
    ArrowDown,
    ArrowDownBar,
    ArrowDownCircle,
    ArrowDownDashed,
    ArrowDownFromArc,
    ArrowDownLeft,
    ArrowDownLeftCircle,
    ArrowDownRhombus,
    ArrowDownRight,
    ArrowDownRightCircle,
    ArrowDownSquare,
    ArrowDownTail,
    ArrowDownToArc,
    ArrowElbowLeft,
    ArrowElbowRight,
    ArrowFork,
    ArrowForward,
    ArrowForwardUp,
    ArrowForwardUpDouble,
    ArrowGuide,
    ArrowIteration,
    ArrowLeft,
    ArrowLeftBar,
    ArrowLeftCircle,
    ArrowLeftDashed,
    ArrowLeftFromArc,
    ArrowLeftRhombus,
    ArrowLeftRight,
    ArrowLeftSquare,
    ArrowLeftTail,
    ArrowLeftToArc,
    ArrowLoopLeft,
    ArrowLoopLeft2,
    ArrowLoopRight,
    ArrowLoopRight2,
    ArrowMerge,
    ArrowMergeAltLeft,
    ArrowMergeAltRight,
    ArrowMergeBoth,
    ArrowMergeLeft,
    ArrowMergeRight,
    ArrowMoveDown,
    ArrowMoveLeft,
    ArrowMoveRight,
    ArrowMoveUp,
    ArrowNarrowDown,
    ArrowNarrowDownDashed,
    ArrowNarrowLeft,
    ArrowNarrowLeftDashed,
    ArrowNarrowRight,
    ArrowNarrowRightDashed,
    ArrowNarrowUp,
    ArrowNarrowUpDashed,
    ArrowRampLeft,
    ArrowRampLeft2,
    ArrowRampLeft3,
    ArrowRampRight,
    ArrowRampRight2,
    ArrowRampRight3,
    ArrowRight,
    ArrowRightBar,
    ArrowRightCircle,
    ArrowRightDashed,
    ArrowRightFromArc,
    ArrowRightRhombus,
    ArrowRightSquare,
    ArrowRightTail,
    ArrowRightToArc,
    ArrowRotaryFirstLeft,
    ArrowRotaryFirstRight,
    ArrowRotaryLastLeft,
    ArrowRotaryLastRight,
    ArrowRotaryLeft,
    ArrowRotaryRight,
    ArrowRotaryStraight,
    ArrowRoundaboutLeft,
    ArrowRoundaboutRight,
    ArrowSharpTurnLeft,
    ArrowSharpTurnRight,
    ArrowUp,
    ArrowUpBar,
    ArrowUpCircle,
    ArrowUpDashed,
    ArrowUpFromArc,
    ArrowUpLeft,
    ArrowUpLeftCircle,
    ArrowUpRhombus,
    ArrowUpRight,
    ArrowUpRightCircle,
    ArrowUpSquare,
    ArrowUpTail,
    ArrowUpToArc,
    ArrowWaveLeftDown,
    ArrowWaveLeftUp,
    ArrowWaveRightDown,
    ArrowWaveRightUp,
    ArrowZigZag,
    ArrowsCross,
    ArrowsDiagonal,
    ArrowsDiagonal2,
    ArrowsDiagonalMinimize,
    ArrowsDiagonalMinimize2,
    ArrowsDiff,
    ArrowsDoubleNeSw,
    ArrowsDoubleNwSe,
    ArrowsDoubleSeNw,
    ArrowsDoubleSwNe,
    ArrowsDown,
    ArrowsDownUp,
    ArrowsExchange,
    ArrowsExchange2,
    ArrowsHorizontal,
    ArrowsJoin,
    ArrowsJoin2,
    ArrowsLeft,
    ArrowsLeftDown,
    ArrowsLeftRight,
    ArrowsMaximize,
    ArrowsMinimize,
    ArrowsMove,
    ArrowsMoveHorizontal,
    ArrowsMoveVertical,
    ArrowsRandom,
    ArrowsRight,
    ArrowsRightDown,
    ArrowsRightLeft,
    ArrowsShuffle,
    ArrowsShuffle2,
    ArrowsSort,
    ArrowsSplit,
    ArrowsSplit2,
    ArrowsTransferDown,
    ArrowsTransferUp,
    ArrowsTransferUpDown,
    ArrowsUp,
    ArrowsUpDown,
    ArrowsUpLeft,
    ArrowsUpRight,
    ArrowsVertical,
    AxisX,
    AxisY,
    CaretDown,
    CaretLeft,
    CaretRight,
    CaretUp,
    CaretUpDown,
    ChevronCompactDown,
    ChevronCompactLeft,
    ChevronCompactRight,
    ChevronCompactUp,
    ChevronDown,
    ChevronDownLeft,
    ChevronDownRight,
    ChevronLeft,
    ChevronLeftPipe,
    ChevronRight,
    ChevronRightPipe,
    ChevronUp,
    ChevronUpLeft,
    ChevronUpRight,
    ChevronsDown,
    ChevronsDownLeft,
    ChevronsDownRight,
    ChevronsLeft,
    ChevronsRight,
    ChevronsUp,
    ChevronsUpLeft,
    ChevronsUpRight,
    CircleArrowDown,
    CircleArrowDownLeft,
    CircleArrowDownRight,
    CircleArrowLeft,
    CircleArrowRight,
    CircleArrowUp,
    CircleArrowUpLeft,
    CircleArrowUpRight,
    CircleCaretDown,
    CircleCaretLeft,
    CircleCaretRight,
    CircleCaretUp,
    CircleChevronDown,
    CircleChevronLeft,
    CircleChevronRight,
    CircleChevronUp,
    CircleChevronsDown,
    CircleChevronsLeft,
    CircleChevronsRight,
    CircleChevronsUp,
    CircleOpenArrowDown,
    CircleOpenArrowLeft,
    CircleOpenArrowRight,
    CircleOpenArrowUp,
    CornerDownLeft,
    CornerDownLeftDouble,
    CornerDownRight,
    CornerDownRightDouble,
    CornerLeftDown,
    CornerLeftDownDouble,
    CornerLeftUp,
    CornerLeftUpDouble,
    CornerRightDown,
    CornerRightDownDouble,
    CornerRightUp,
    CornerRightUpDouble,
    CornerUpLeft,
    CornerUpLeftDouble,
    CornerUpRight,
    CornerUpRightDouble,
    Download,
    DownloadOff,
    Fold,
    FoldDown,
    FoldUp,
    Login,
    Login2,
    Refresh,
    RefreshAlert,
    RefreshDot,
    RefreshOff,
    Reload,
    RippleDown,
    RippleUp,
    Rotate,
    Rotate2,
    Rotate360,
    RotateClockwise,
    RotateClockwise2,
    RotateDot,
    RotateRectangle,
    STurnDown,
    STurnLeft,
    STurnRight,
    STurnUp,
    Select,
    Selector,
    Share2,
    Share3,
    SquareArrowDown,
    SquareArrowLeft,
    SquareArrowRight,
    SquareArrowUp,
    SquareChevronDown,
    SquareChevronLeft,
    SquareChevronRight,
    SquareChevronUp,
    SquareChevronsDown,
    SquareChevronsLeft,
    SquareChevronsRight,
    SquareChevronsUp,
    SquareRoundedArrowDown,
    SquareRoundedArrowLeft,
    SquareRoundedArrowRight,
    SquareRoundedArrowUp,
    SquareRoundedChevronDown,
    SquareRoundedChevronLeft,
    SquareRoundedChevronRight,
    SquareRoundedChevronUp,
    SquareRoundedChevronsDown,
    SquareRoundedChevronsLeft,
    SquareRoundedChevronsRight,
    SquareRoundedChevronsUp,
    StepInto,
    StepOut,
    SwipeDown,
    SwipeLeft,
    SwipeRight,
    SwipeUp,
    Switch,
    Switch2,
    Switch3,
    SwitchHorizontal,
    SwitchVertical,
    TransitionBottom,
    TransitionLeft,
    TransitionRight,
    TransitionTop,
    TrendingDown,
    TrendingDown2,
    TrendingDown3,
    TrendingUp,
    TrendingUp2,
    TrendingUp3,
    UTurnLeft,
    UTurnRight,
    Upload,
}

impl ArrowsIcon {
    /// Returns all available icons in this category.
    pub fn all() -> &'static [Self] {
        &[Self::ArrowAutofitContent, Self::ArrowAutofitDown, Self::ArrowAutofitHeight, Self::ArrowAutofitLeft, Self::ArrowAutofitRight, Self::ArrowAutofitUp, Self::ArrowAutofitWidth, Self::ArrowBack, Self::ArrowBackUp, Self::ArrowBackUpDouble, Self::ArrowBadgeDown, Self::ArrowBadgeLeft, Self::ArrowBadgeRight, Self::ArrowBadgeUp, Self::ArrowBarBoth, Self::ArrowBarDown, Self::ArrowBarLeft, Self::ArrowBarRight, Self::ArrowBarToDown, Self::ArrowBarToDownDashed, Self::ArrowBarToLeft, Self::ArrowBarToLeftDashed, Self::ArrowBarToRight, Self::ArrowBarToRightDashed, Self::ArrowBarToUp, Self::ArrowBarToUpDashed, Self::ArrowBarUp, Self::ArrowBearLeft, Self::ArrowBearLeft2, Self::ArrowBearRight, Self::ArrowBearRight2, Self::ArrowBigDown, Self::ArrowBigDownLine, Self::ArrowBigDownLines, Self::ArrowBigLeft, Self::ArrowBigLeftLine, Self::ArrowBigLeftLines, Self::ArrowBigRight, Self::ArrowBigRightLine, Self::ArrowBigRightLines, Self::ArrowBigUp, Self::ArrowBigUpLine, Self::ArrowBigUpLines, Self::ArrowBounce, Self::ArrowCapsule, Self::ArrowCurveLeft, Self::ArrowCurveRight, Self::ArrowDown, Self::ArrowDownBar, Self::ArrowDownCircle, Self::ArrowDownDashed, Self::ArrowDownFromArc, Self::ArrowDownLeft, Self::ArrowDownLeftCircle, Self::ArrowDownRhombus, Self::ArrowDownRight, Self::ArrowDownRightCircle, Self::ArrowDownSquare, Self::ArrowDownTail, Self::ArrowDownToArc, Self::ArrowElbowLeft, Self::ArrowElbowRight, Self::ArrowFork, Self::ArrowForward, Self::ArrowForwardUp, Self::ArrowForwardUpDouble, Self::ArrowGuide, Self::ArrowIteration, Self::ArrowLeft, Self::ArrowLeftBar, Self::ArrowLeftCircle, Self::ArrowLeftDashed, Self::ArrowLeftFromArc, Self::ArrowLeftRhombus, Self::ArrowLeftRight, Self::ArrowLeftSquare, Self::ArrowLeftTail, Self::ArrowLeftToArc, Self::ArrowLoopLeft, Self::ArrowLoopLeft2, Self::ArrowLoopRight, Self::ArrowLoopRight2, Self::ArrowMerge, Self::ArrowMergeAltLeft, Self::ArrowMergeAltRight, Self::ArrowMergeBoth, Self::ArrowMergeLeft, Self::ArrowMergeRight, Self::ArrowMoveDown, Self::ArrowMoveLeft, Self::ArrowMoveRight, Self::ArrowMoveUp, Self::ArrowNarrowDown, Self::ArrowNarrowDownDashed, Self::ArrowNarrowLeft, Self::ArrowNarrowLeftDashed, Self::ArrowNarrowRight, Self::ArrowNarrowRightDashed, Self::ArrowNarrowUp, Self::ArrowNarrowUpDashed, Self::ArrowRampLeft, Self::ArrowRampLeft2, Self::ArrowRampLeft3, Self::ArrowRampRight, Self::ArrowRampRight2, Self::ArrowRampRight3, Self::ArrowRight, Self::ArrowRightBar, Self::ArrowRightCircle, Self::ArrowRightDashed, Self::ArrowRightFromArc, Self::ArrowRightRhombus, Self::ArrowRightSquare, Self::ArrowRightTail, Self::ArrowRightToArc, Self::ArrowRotaryFirstLeft, Self::ArrowRotaryFirstRight, Self::ArrowRotaryLastLeft, Self::ArrowRotaryLastRight, Self::ArrowRotaryLeft, Self::ArrowRotaryRight, Self::ArrowRotaryStraight, Self::ArrowRoundaboutLeft, Self::ArrowRoundaboutRight, Self::ArrowSharpTurnLeft, Self::ArrowSharpTurnRight, Self::ArrowUp, Self::ArrowUpBar, Self::ArrowUpCircle, Self::ArrowUpDashed, Self::ArrowUpFromArc, Self::ArrowUpLeft, Self::ArrowUpLeftCircle, Self::ArrowUpRhombus, Self::ArrowUpRight, Self::ArrowUpRightCircle, Self::ArrowUpSquare, Self::ArrowUpTail, Self::ArrowUpToArc, Self::ArrowWaveLeftDown, Self::ArrowWaveLeftUp, Self::ArrowWaveRightDown, Self::ArrowWaveRightUp, Self::ArrowZigZag, Self::ArrowsCross, Self::ArrowsDiagonal, Self::ArrowsDiagonal2, Self::ArrowsDiagonalMinimize, Self::ArrowsDiagonalMinimize2, Self::ArrowsDiff, Self::ArrowsDoubleNeSw, Self::ArrowsDoubleNwSe, Self::ArrowsDoubleSeNw, Self::ArrowsDoubleSwNe, Self::ArrowsDown, Self::ArrowsDownUp, Self::ArrowsExchange, Self::ArrowsExchange2, Self::ArrowsHorizontal, Self::ArrowsJoin, Self::ArrowsJoin2, Self::ArrowsLeft, Self::ArrowsLeftDown, Self::ArrowsLeftRight, Self::ArrowsMaximize, Self::ArrowsMinimize, Self::ArrowsMove, Self::ArrowsMoveHorizontal, Self::ArrowsMoveVertical, Self::ArrowsRandom, Self::ArrowsRight, Self::ArrowsRightDown, Self::ArrowsRightLeft, Self::ArrowsShuffle, Self::ArrowsShuffle2, Self::ArrowsSort, Self::ArrowsSplit, Self::ArrowsSplit2, Self::ArrowsTransferDown, Self::ArrowsTransferUp, Self::ArrowsTransferUpDown, Self::ArrowsUp, Self::ArrowsUpDown, Self::ArrowsUpLeft, Self::ArrowsUpRight, Self::ArrowsVertical, Self::AxisX, Self::AxisY, Self::CaretDown, Self::CaretLeft, Self::CaretRight, Self::CaretUp, Self::CaretUpDown, Self::ChevronCompactDown, Self::ChevronCompactLeft, Self::ChevronCompactRight, Self::ChevronCompactUp, Self::ChevronDown, Self::ChevronDownLeft, Self::ChevronDownRight, Self::ChevronLeft, Self::ChevronLeftPipe, Self::ChevronRight, Self::ChevronRightPipe, Self::ChevronUp, Self::ChevronUpLeft, Self::ChevronUpRight, Self::ChevronsDown, Self::ChevronsDownLeft, Self::ChevronsDownRight, Self::ChevronsLeft, Self::ChevronsRight, Self::ChevronsUp, Self::ChevronsUpLeft, Self::ChevronsUpRight, Self::CircleArrowDown, Self::CircleArrowDownLeft, Self::CircleArrowDownRight, Self::CircleArrowLeft, Self::CircleArrowRight, Self::CircleArrowUp, Self::CircleArrowUpLeft, Self::CircleArrowUpRight, Self::CircleCaretDown, Self::CircleCaretLeft, Self::CircleCaretRight, Self::CircleCaretUp, Self::CircleChevronDown, Self::CircleChevronLeft, Self::CircleChevronRight, Self::CircleChevronUp, Self::CircleChevronsDown, Self::CircleChevronsLeft, Self::CircleChevronsRight, Self::CircleChevronsUp, Self::CircleOpenArrowDown, Self::CircleOpenArrowLeft, Self::CircleOpenArrowRight, Self::CircleOpenArrowUp, Self::CornerDownLeft, Self::CornerDownLeftDouble, Self::CornerDownRight, Self::CornerDownRightDouble, Self::CornerLeftDown, Self::CornerLeftDownDouble, Self::CornerLeftUp, Self::CornerLeftUpDouble, Self::CornerRightDown, Self::CornerRightDownDouble, Self::CornerRightUp, Self::CornerRightUpDouble, Self::CornerUpLeft, Self::CornerUpLeftDouble, Self::CornerUpRight, Self::CornerUpRightDouble, Self::Download, Self::DownloadOff, Self::Fold, Self::FoldDown, Self::FoldUp, Self::Login, Self::Login2, Self::Refresh, Self::RefreshAlert, Self::RefreshDot, Self::RefreshOff, Self::Reload, Self::RippleDown, Self::RippleUp, Self::Rotate, Self::Rotate2, Self::Rotate360, Self::RotateClockwise, Self::RotateClockwise2, Self::RotateDot, Self::RotateRectangle, Self::STurnDown, Self::STurnLeft, Self::STurnRight, Self::STurnUp, Self::Select, Self::Selector, Self::Share2, Self::Share3, Self::SquareArrowDown, Self::SquareArrowLeft, Self::SquareArrowRight, Self::SquareArrowUp, Self::SquareChevronDown, Self::SquareChevronLeft, Self::SquareChevronRight, Self::SquareChevronUp, Self::SquareChevronsDown, Self::SquareChevronsLeft, Self::SquareChevronsRight, Self::SquareChevronsUp, Self::SquareRoundedArrowDown, Self::SquareRoundedArrowLeft, Self::SquareRoundedArrowRight, Self::SquareRoundedArrowUp, Self::SquareRoundedChevronDown, Self::SquareRoundedChevronLeft, Self::SquareRoundedChevronRight, Self::SquareRoundedChevronUp, Self::SquareRoundedChevronsDown, Self::SquareRoundedChevronsLeft, Self::SquareRoundedChevronsRight, Self::SquareRoundedChevronsUp, Self::StepInto, Self::StepOut, Self::SwipeDown, Self::SwipeLeft, Self::SwipeRight, Self::SwipeUp, Self::Switch, Self::Switch2, Self::Switch3, Self::SwitchHorizontal, Self::SwitchVertical, Self::TransitionBottom, Self::TransitionLeft, Self::TransitionRight, Self::TransitionTop, Self::TrendingDown, Self::TrendingDown2, Self::TrendingDown3, Self::TrendingUp, Self::TrendingUp2, Self::TrendingUp3, Self::UTurnLeft, Self::UTurnRight, Self::Upload]
    }

    /// Returns the icon count.
    pub fn count() -> usize {
        332
    }

    /// Creates an icon from its kebab-case name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "arrow-autofit-content" => Some(Self::ArrowAutofitContent),
            "arrow-autofit-down" => Some(Self::ArrowAutofitDown),
            "arrow-autofit-height" => Some(Self::ArrowAutofitHeight),
            "arrow-autofit-left" => Some(Self::ArrowAutofitLeft),
            "arrow-autofit-right" => Some(Self::ArrowAutofitRight),
            "arrow-autofit-up" => Some(Self::ArrowAutofitUp),
            "arrow-autofit-width" => Some(Self::ArrowAutofitWidth),
            "arrow-back" => Some(Self::ArrowBack),
            "arrow-back-up" => Some(Self::ArrowBackUp),
            "arrow-back-up-double" => Some(Self::ArrowBackUpDouble),
            "arrow-badge-down" => Some(Self::ArrowBadgeDown),
            "arrow-badge-left" => Some(Self::ArrowBadgeLeft),
            "arrow-badge-right" => Some(Self::ArrowBadgeRight),
            "arrow-badge-up" => Some(Self::ArrowBadgeUp),
            "arrow-bar-both" => Some(Self::ArrowBarBoth),
            "arrow-bar-down" => Some(Self::ArrowBarDown),
            "arrow-bar-left" => Some(Self::ArrowBarLeft),
            "arrow-bar-right" => Some(Self::ArrowBarRight),
            "arrow-bar-to-down" => Some(Self::ArrowBarToDown),
            "arrow-bar-to-down-dashed" => Some(Self::ArrowBarToDownDashed),
            "arrow-bar-to-left" => Some(Self::ArrowBarToLeft),
            "arrow-bar-to-left-dashed" => Some(Self::ArrowBarToLeftDashed),
            "arrow-bar-to-right" => Some(Self::ArrowBarToRight),
            "arrow-bar-to-right-dashed" => Some(Self::ArrowBarToRightDashed),
            "arrow-bar-to-up" => Some(Self::ArrowBarToUp),
            "arrow-bar-to-up-dashed" => Some(Self::ArrowBarToUpDashed),
            "arrow-bar-up" => Some(Self::ArrowBarUp),
            "arrow-bear-left" => Some(Self::ArrowBearLeft),
            "arrow-bear-left-2" => Some(Self::ArrowBearLeft2),
            "arrow-bear-right" => Some(Self::ArrowBearRight),
            "arrow-bear-right-2" => Some(Self::ArrowBearRight2),
            "arrow-big-down" => Some(Self::ArrowBigDown),
            "arrow-big-down-line" => Some(Self::ArrowBigDownLine),
            "arrow-big-down-lines" => Some(Self::ArrowBigDownLines),
            "arrow-big-left" => Some(Self::ArrowBigLeft),
            "arrow-big-left-line" => Some(Self::ArrowBigLeftLine),
            "arrow-big-left-lines" => Some(Self::ArrowBigLeftLines),
            "arrow-big-right" => Some(Self::ArrowBigRight),
            "arrow-big-right-line" => Some(Self::ArrowBigRightLine),
            "arrow-big-right-lines" => Some(Self::ArrowBigRightLines),
            "arrow-big-up" => Some(Self::ArrowBigUp),
            "arrow-big-up-line" => Some(Self::ArrowBigUpLine),
            "arrow-big-up-lines" => Some(Self::ArrowBigUpLines),
            "arrow-bounce" => Some(Self::ArrowBounce),
            "arrow-capsule" => Some(Self::ArrowCapsule),
            "arrow-curve-left" => Some(Self::ArrowCurveLeft),
            "arrow-curve-right" => Some(Self::ArrowCurveRight),
            "arrow-down" => Some(Self::ArrowDown),
            "arrow-down-bar" => Some(Self::ArrowDownBar),
            "arrow-down-circle" => Some(Self::ArrowDownCircle),
            "arrow-down-dashed" => Some(Self::ArrowDownDashed),
            "arrow-down-from-arc" => Some(Self::ArrowDownFromArc),
            "arrow-down-left" => Some(Self::ArrowDownLeft),
            "arrow-down-left-circle" => Some(Self::ArrowDownLeftCircle),
            "arrow-down-rhombus" => Some(Self::ArrowDownRhombus),
            "arrow-down-right" => Some(Self::ArrowDownRight),
            "arrow-down-right-circle" => Some(Self::ArrowDownRightCircle),
            "arrow-down-square" => Some(Self::ArrowDownSquare),
            "arrow-down-tail" => Some(Self::ArrowDownTail),
            "arrow-down-to-arc" => Some(Self::ArrowDownToArc),
            "arrow-elbow-left" => Some(Self::ArrowElbowLeft),
            "arrow-elbow-right" => Some(Self::ArrowElbowRight),
            "arrow-fork" => Some(Self::ArrowFork),
            "arrow-forward" => Some(Self::ArrowForward),
            "arrow-forward-up" => Some(Self::ArrowForwardUp),
            "arrow-forward-up-double" => Some(Self::ArrowForwardUpDouble),
            "arrow-guide" => Some(Self::ArrowGuide),
            "arrow-iteration" => Some(Self::ArrowIteration),
            "arrow-left" => Some(Self::ArrowLeft),
            "arrow-left-bar" => Some(Self::ArrowLeftBar),
            "arrow-left-circle" => Some(Self::ArrowLeftCircle),
            "arrow-left-dashed" => Some(Self::ArrowLeftDashed),
            "arrow-left-from-arc" => Some(Self::ArrowLeftFromArc),
            "arrow-left-rhombus" => Some(Self::ArrowLeftRhombus),
            "arrow-left-right" => Some(Self::ArrowLeftRight),
            "arrow-left-square" => Some(Self::ArrowLeftSquare),
            "arrow-left-tail" => Some(Self::ArrowLeftTail),
            "arrow-left-to-arc" => Some(Self::ArrowLeftToArc),
            "arrow-loop-left" => Some(Self::ArrowLoopLeft),
            "arrow-loop-left-2" => Some(Self::ArrowLoopLeft2),
            "arrow-loop-right" => Some(Self::ArrowLoopRight),
            "arrow-loop-right-2" => Some(Self::ArrowLoopRight2),
            "arrow-merge" => Some(Self::ArrowMerge),
            "arrow-merge-alt-left" => Some(Self::ArrowMergeAltLeft),
            "arrow-merge-alt-right" => Some(Self::ArrowMergeAltRight),
            "arrow-merge-both" => Some(Self::ArrowMergeBoth),
            "arrow-merge-left" => Some(Self::ArrowMergeLeft),
            "arrow-merge-right" => Some(Self::ArrowMergeRight),
            "arrow-move-down" => Some(Self::ArrowMoveDown),
            "arrow-move-left" => Some(Self::ArrowMoveLeft),
            "arrow-move-right" => Some(Self::ArrowMoveRight),
            "arrow-move-up" => Some(Self::ArrowMoveUp),
            "arrow-narrow-down" => Some(Self::ArrowNarrowDown),
            "arrow-narrow-down-dashed" => Some(Self::ArrowNarrowDownDashed),
            "arrow-narrow-left" => Some(Self::ArrowNarrowLeft),
            "arrow-narrow-left-dashed" => Some(Self::ArrowNarrowLeftDashed),
            "arrow-narrow-right" => Some(Self::ArrowNarrowRight),
            "arrow-narrow-right-dashed" => Some(Self::ArrowNarrowRightDashed),
            "arrow-narrow-up" => Some(Self::ArrowNarrowUp),
            "arrow-narrow-up-dashed" => Some(Self::ArrowNarrowUpDashed),
            "arrow-ramp-left" => Some(Self::ArrowRampLeft),
            "arrow-ramp-left-2" => Some(Self::ArrowRampLeft2),
            "arrow-ramp-left-3" => Some(Self::ArrowRampLeft3),
            "arrow-ramp-right" => Some(Self::ArrowRampRight),
            "arrow-ramp-right-2" => Some(Self::ArrowRampRight2),
            "arrow-ramp-right-3" => Some(Self::ArrowRampRight3),
            "arrow-right" => Some(Self::ArrowRight),
            "arrow-right-bar" => Some(Self::ArrowRightBar),
            "arrow-right-circle" => Some(Self::ArrowRightCircle),
            "arrow-right-dashed" => Some(Self::ArrowRightDashed),
            "arrow-right-from-arc" => Some(Self::ArrowRightFromArc),
            "arrow-right-rhombus" => Some(Self::ArrowRightRhombus),
            "arrow-right-square" => Some(Self::ArrowRightSquare),
            "arrow-right-tail" => Some(Self::ArrowRightTail),
            "arrow-right-to-arc" => Some(Self::ArrowRightToArc),
            "arrow-rotary-first-left" => Some(Self::ArrowRotaryFirstLeft),
            "arrow-rotary-first-right" => Some(Self::ArrowRotaryFirstRight),
            "arrow-rotary-last-left" => Some(Self::ArrowRotaryLastLeft),
            "arrow-rotary-last-right" => Some(Self::ArrowRotaryLastRight),
            "arrow-rotary-left" => Some(Self::ArrowRotaryLeft),
            "arrow-rotary-right" => Some(Self::ArrowRotaryRight),
            "arrow-rotary-straight" => Some(Self::ArrowRotaryStraight),
            "arrow-roundabout-left" => Some(Self::ArrowRoundaboutLeft),
            "arrow-roundabout-right" => Some(Self::ArrowRoundaboutRight),
            "arrow-sharp-turn-left" => Some(Self::ArrowSharpTurnLeft),
            "arrow-sharp-turn-right" => Some(Self::ArrowSharpTurnRight),
            "arrow-up" => Some(Self::ArrowUp),
            "arrow-up-bar" => Some(Self::ArrowUpBar),
            "arrow-up-circle" => Some(Self::ArrowUpCircle),
            "arrow-up-dashed" => Some(Self::ArrowUpDashed),
            "arrow-up-from-arc" => Some(Self::ArrowUpFromArc),
            "arrow-up-left" => Some(Self::ArrowUpLeft),
            "arrow-up-left-circle" => Some(Self::ArrowUpLeftCircle),
            "arrow-up-rhombus" => Some(Self::ArrowUpRhombus),
            "arrow-up-right" => Some(Self::ArrowUpRight),
            "arrow-up-right-circle" => Some(Self::ArrowUpRightCircle),
            "arrow-up-square" => Some(Self::ArrowUpSquare),
            "arrow-up-tail" => Some(Self::ArrowUpTail),
            "arrow-up-to-arc" => Some(Self::ArrowUpToArc),
            "arrow-wave-left-down" => Some(Self::ArrowWaveLeftDown),
            "arrow-wave-left-up" => Some(Self::ArrowWaveLeftUp),
            "arrow-wave-right-down" => Some(Self::ArrowWaveRightDown),
            "arrow-wave-right-up" => Some(Self::ArrowWaveRightUp),
            "arrow-zig-zag" => Some(Self::ArrowZigZag),
            "arrows-cross" => Some(Self::ArrowsCross),
            "arrows-diagonal" => Some(Self::ArrowsDiagonal),
            "arrows-diagonal-2" => Some(Self::ArrowsDiagonal2),
            "arrows-diagonal-minimize" => Some(Self::ArrowsDiagonalMinimize),
            "arrows-diagonal-minimize-2" => Some(Self::ArrowsDiagonalMinimize2),
            "arrows-diff" => Some(Self::ArrowsDiff),
            "arrows-double-ne-sw" => Some(Self::ArrowsDoubleNeSw),
            "arrows-double-nw-se" => Some(Self::ArrowsDoubleNwSe),
            "arrows-double-se-nw" => Some(Self::ArrowsDoubleSeNw),
            "arrows-double-sw-ne" => Some(Self::ArrowsDoubleSwNe),
            "arrows-down" => Some(Self::ArrowsDown),
            "arrows-down-up" => Some(Self::ArrowsDownUp),
            "arrows-exchange" => Some(Self::ArrowsExchange),
            "arrows-exchange-2" => Some(Self::ArrowsExchange2),
            "arrows-horizontal" => Some(Self::ArrowsHorizontal),
            "arrows-join" => Some(Self::ArrowsJoin),
            "arrows-join-2" => Some(Self::ArrowsJoin2),
            "arrows-left" => Some(Self::ArrowsLeft),
            "arrows-left-down" => Some(Self::ArrowsLeftDown),
            "arrows-left-right" => Some(Self::ArrowsLeftRight),
            "arrows-maximize" => Some(Self::ArrowsMaximize),
            "arrows-minimize" => Some(Self::ArrowsMinimize),
            "arrows-move" => Some(Self::ArrowsMove),
            "arrows-move-horizontal" => Some(Self::ArrowsMoveHorizontal),
            "arrows-move-vertical" => Some(Self::ArrowsMoveVertical),
            "arrows-random" => Some(Self::ArrowsRandom),
            "arrows-right" => Some(Self::ArrowsRight),
            "arrows-right-down" => Some(Self::ArrowsRightDown),
            "arrows-right-left" => Some(Self::ArrowsRightLeft),
            "arrows-shuffle" => Some(Self::ArrowsShuffle),
            "arrows-shuffle-2" => Some(Self::ArrowsShuffle2),
            "arrows-sort" => Some(Self::ArrowsSort),
            "arrows-split" => Some(Self::ArrowsSplit),
            "arrows-split-2" => Some(Self::ArrowsSplit2),
            "arrows-transfer-down" => Some(Self::ArrowsTransferDown),
            "arrows-transfer-up" => Some(Self::ArrowsTransferUp),
            "arrows-transfer-up-down" => Some(Self::ArrowsTransferUpDown),
            "arrows-up" => Some(Self::ArrowsUp),
            "arrows-up-down" => Some(Self::ArrowsUpDown),
            "arrows-up-left" => Some(Self::ArrowsUpLeft),
            "arrows-up-right" => Some(Self::ArrowsUpRight),
            "arrows-vertical" => Some(Self::ArrowsVertical),
            "axis-x" => Some(Self::AxisX),
            "axis-y" => Some(Self::AxisY),
            "caret-down" => Some(Self::CaretDown),
            "caret-left" => Some(Self::CaretLeft),
            "caret-right" => Some(Self::CaretRight),
            "caret-up" => Some(Self::CaretUp),
            "caret-up-down" => Some(Self::CaretUpDown),
            "chevron-compact-down" => Some(Self::ChevronCompactDown),
            "chevron-compact-left" => Some(Self::ChevronCompactLeft),
            "chevron-compact-right" => Some(Self::ChevronCompactRight),
            "chevron-compact-up" => Some(Self::ChevronCompactUp),
            "chevron-down" => Some(Self::ChevronDown),
            "chevron-down-left" => Some(Self::ChevronDownLeft),
            "chevron-down-right" => Some(Self::ChevronDownRight),
            "chevron-left" => Some(Self::ChevronLeft),
            "chevron-left-pipe" => Some(Self::ChevronLeftPipe),
            "chevron-right" => Some(Self::ChevronRight),
            "chevron-right-pipe" => Some(Self::ChevronRightPipe),
            "chevron-up" => Some(Self::ChevronUp),
            "chevron-up-left" => Some(Self::ChevronUpLeft),
            "chevron-up-right" => Some(Self::ChevronUpRight),
            "chevrons-down" => Some(Self::ChevronsDown),
            "chevrons-down-left" => Some(Self::ChevronsDownLeft),
            "chevrons-down-right" => Some(Self::ChevronsDownRight),
            "chevrons-left" => Some(Self::ChevronsLeft),
            "chevrons-right" => Some(Self::ChevronsRight),
            "chevrons-up" => Some(Self::ChevronsUp),
            "chevrons-up-left" => Some(Self::ChevronsUpLeft),
            "chevrons-up-right" => Some(Self::ChevronsUpRight),
            "circle-arrow-down" => Some(Self::CircleArrowDown),
            "circle-arrow-down-left" => Some(Self::CircleArrowDownLeft),
            "circle-arrow-down-right" => Some(Self::CircleArrowDownRight),
            "circle-arrow-left" => Some(Self::CircleArrowLeft),
            "circle-arrow-right" => Some(Self::CircleArrowRight),
            "circle-arrow-up" => Some(Self::CircleArrowUp),
            "circle-arrow-up-left" => Some(Self::CircleArrowUpLeft),
            "circle-arrow-up-right" => Some(Self::CircleArrowUpRight),
            "circle-caret-down" => Some(Self::CircleCaretDown),
            "circle-caret-left" => Some(Self::CircleCaretLeft),
            "circle-caret-right" => Some(Self::CircleCaretRight),
            "circle-caret-up" => Some(Self::CircleCaretUp),
            "circle-chevron-down" => Some(Self::CircleChevronDown),
            "circle-chevron-left" => Some(Self::CircleChevronLeft),
            "circle-chevron-right" => Some(Self::CircleChevronRight),
            "circle-chevron-up" => Some(Self::CircleChevronUp),
            "circle-chevrons-down" => Some(Self::CircleChevronsDown),
            "circle-chevrons-left" => Some(Self::CircleChevronsLeft),
            "circle-chevrons-right" => Some(Self::CircleChevronsRight),
            "circle-chevrons-up" => Some(Self::CircleChevronsUp),
            "circle-open-arrow-down" => Some(Self::CircleOpenArrowDown),
            "circle-open-arrow-left" => Some(Self::CircleOpenArrowLeft),
            "circle-open-arrow-right" => Some(Self::CircleOpenArrowRight),
            "circle-open-arrow-up" => Some(Self::CircleOpenArrowUp),
            "corner-down-left" => Some(Self::CornerDownLeft),
            "corner-down-left-double" => Some(Self::CornerDownLeftDouble),
            "corner-down-right" => Some(Self::CornerDownRight),
            "corner-down-right-double" => Some(Self::CornerDownRightDouble),
            "corner-left-down" => Some(Self::CornerLeftDown),
            "corner-left-down-double" => Some(Self::CornerLeftDownDouble),
            "corner-left-up" => Some(Self::CornerLeftUp),
            "corner-left-up-double" => Some(Self::CornerLeftUpDouble),
            "corner-right-down" => Some(Self::CornerRightDown),
            "corner-right-down-double" => Some(Self::CornerRightDownDouble),
            "corner-right-up" => Some(Self::CornerRightUp),
            "corner-right-up-double" => Some(Self::CornerRightUpDouble),
            "corner-up-left" => Some(Self::CornerUpLeft),
            "corner-up-left-double" => Some(Self::CornerUpLeftDouble),
            "corner-up-right" => Some(Self::CornerUpRight),
            "corner-up-right-double" => Some(Self::CornerUpRightDouble),
            "download" => Some(Self::Download),
            "download-off" => Some(Self::DownloadOff),
            "fold" => Some(Self::Fold),
            "fold-down" => Some(Self::FoldDown),
            "fold-up" => Some(Self::FoldUp),
            "login" => Some(Self::Login),
            "login-2" => Some(Self::Login2),
            "refresh" => Some(Self::Refresh),
            "refresh-alert" => Some(Self::RefreshAlert),
            "refresh-dot" => Some(Self::RefreshDot),
            "refresh-off" => Some(Self::RefreshOff),
            "reload" => Some(Self::Reload),
            "ripple-down" => Some(Self::RippleDown),
            "ripple-up" => Some(Self::RippleUp),
            "rotate" => Some(Self::Rotate),
            "rotate-2" => Some(Self::Rotate2),
            "rotate-360" => Some(Self::Rotate360),
            "rotate-clockwise" => Some(Self::RotateClockwise),
            "rotate-clockwise-2" => Some(Self::RotateClockwise2),
            "rotate-dot" => Some(Self::RotateDot),
            "rotate-rectangle" => Some(Self::RotateRectangle),
            "s-turn-down" => Some(Self::STurnDown),
            "s-turn-left" => Some(Self::STurnLeft),
            "s-turn-right" => Some(Self::STurnRight),
            "s-turn-up" => Some(Self::STurnUp),
            "select" => Some(Self::Select),
            "selector" => Some(Self::Selector),
            "share-2" => Some(Self::Share2),
            "share-3" => Some(Self::Share3),
            "square-arrow-down" => Some(Self::SquareArrowDown),
            "square-arrow-left" => Some(Self::SquareArrowLeft),
            "square-arrow-right" => Some(Self::SquareArrowRight),
            "square-arrow-up" => Some(Self::SquareArrowUp),
            "square-chevron-down" => Some(Self::SquareChevronDown),
            "square-chevron-left" => Some(Self::SquareChevronLeft),
            "square-chevron-right" => Some(Self::SquareChevronRight),
            "square-chevron-up" => Some(Self::SquareChevronUp),
            "square-chevrons-down" => Some(Self::SquareChevronsDown),
            "square-chevrons-left" => Some(Self::SquareChevronsLeft),
            "square-chevrons-right" => Some(Self::SquareChevronsRight),
            "square-chevrons-up" => Some(Self::SquareChevronsUp),
            "square-rounded-arrow-down" => Some(Self::SquareRoundedArrowDown),
            "square-rounded-arrow-left" => Some(Self::SquareRoundedArrowLeft),
            "square-rounded-arrow-right" => Some(Self::SquareRoundedArrowRight),
            "square-rounded-arrow-up" => Some(Self::SquareRoundedArrowUp),
            "square-rounded-chevron-down" => Some(Self::SquareRoundedChevronDown),
            "square-rounded-chevron-left" => Some(Self::SquareRoundedChevronLeft),
            "square-rounded-chevron-right" => Some(Self::SquareRoundedChevronRight),
            "square-rounded-chevron-up" => Some(Self::SquareRoundedChevronUp),
            "square-rounded-chevrons-down" => Some(Self::SquareRoundedChevronsDown),
            "square-rounded-chevrons-left" => Some(Self::SquareRoundedChevronsLeft),
            "square-rounded-chevrons-right" => Some(Self::SquareRoundedChevronsRight),
            "square-rounded-chevrons-up" => Some(Self::SquareRoundedChevronsUp),
            "step-into" => Some(Self::StepInto),
            "step-out" => Some(Self::StepOut),
            "swipe-down" => Some(Self::SwipeDown),
            "swipe-left" => Some(Self::SwipeLeft),
            "swipe-right" => Some(Self::SwipeRight),
            "swipe-up" => Some(Self::SwipeUp),
            "switch" => Some(Self::Switch),
            "switch-2" => Some(Self::Switch2),
            "switch-3" => Some(Self::Switch3),
            "switch-horizontal" => Some(Self::SwitchHorizontal),
            "switch-vertical" => Some(Self::SwitchVertical),
            "transition-bottom" => Some(Self::TransitionBottom),
            "transition-left" => Some(Self::TransitionLeft),
            "transition-right" => Some(Self::TransitionRight),
            "transition-top" => Some(Self::TransitionTop),
            "trending-down" => Some(Self::TrendingDown),
            "trending-down-2" => Some(Self::TrendingDown2),
            "trending-down-3" => Some(Self::TrendingDown3),
            "trending-up" => Some(Self::TrendingUp),
            "trending-up-2" => Some(Self::TrendingUp2),
            "trending-up-3" => Some(Self::TrendingUp3),
            "u-turn-left" => Some(Self::UTurnLeft),
            "u-turn-right" => Some(Self::UTurnRight),
            "upload" => Some(Self::Upload),
            _ => None,
        }
    }
}

impl TablerIconData for ArrowsIcon {
    fn name(&self) -> &'static str {
        match self {
            Self::ArrowAutofitContent => "arrow-autofit-content",
            Self::ArrowAutofitDown => "arrow-autofit-down",
            Self::ArrowAutofitHeight => "arrow-autofit-height",
            Self::ArrowAutofitLeft => "arrow-autofit-left",
            Self::ArrowAutofitRight => "arrow-autofit-right",
            Self::ArrowAutofitUp => "arrow-autofit-up",
            Self::ArrowAutofitWidth => "arrow-autofit-width",
            Self::ArrowBack => "arrow-back",
            Self::ArrowBackUp => "arrow-back-up",
            Self::ArrowBackUpDouble => "arrow-back-up-double",
            Self::ArrowBadgeDown => "arrow-badge-down",
            Self::ArrowBadgeLeft => "arrow-badge-left",
            Self::ArrowBadgeRight => "arrow-badge-right",
            Self::ArrowBadgeUp => "arrow-badge-up",
            Self::ArrowBarBoth => "arrow-bar-both",
            Self::ArrowBarDown => "arrow-bar-down",
            Self::ArrowBarLeft => "arrow-bar-left",
            Self::ArrowBarRight => "arrow-bar-right",
            Self::ArrowBarToDown => "arrow-bar-to-down",
            Self::ArrowBarToDownDashed => "arrow-bar-to-down-dashed",
            Self::ArrowBarToLeft => "arrow-bar-to-left",
            Self::ArrowBarToLeftDashed => "arrow-bar-to-left-dashed",
            Self::ArrowBarToRight => "arrow-bar-to-right",
            Self::ArrowBarToRightDashed => "arrow-bar-to-right-dashed",
            Self::ArrowBarToUp => "arrow-bar-to-up",
            Self::ArrowBarToUpDashed => "arrow-bar-to-up-dashed",
            Self::ArrowBarUp => "arrow-bar-up",
            Self::ArrowBearLeft => "arrow-bear-left",
            Self::ArrowBearLeft2 => "arrow-bear-left-2",
            Self::ArrowBearRight => "arrow-bear-right",
            Self::ArrowBearRight2 => "arrow-bear-right-2",
            Self::ArrowBigDown => "arrow-big-down",
            Self::ArrowBigDownLine => "arrow-big-down-line",
            Self::ArrowBigDownLines => "arrow-big-down-lines",
            Self::ArrowBigLeft => "arrow-big-left",
            Self::ArrowBigLeftLine => "arrow-big-left-line",
            Self::ArrowBigLeftLines => "arrow-big-left-lines",
            Self::ArrowBigRight => "arrow-big-right",
            Self::ArrowBigRightLine => "arrow-big-right-line",
            Self::ArrowBigRightLines => "arrow-big-right-lines",
            Self::ArrowBigUp => "arrow-big-up",
            Self::ArrowBigUpLine => "arrow-big-up-line",
            Self::ArrowBigUpLines => "arrow-big-up-lines",
            Self::ArrowBounce => "arrow-bounce",
            Self::ArrowCapsule => "arrow-capsule",
            Self::ArrowCurveLeft => "arrow-curve-left",
            Self::ArrowCurveRight => "arrow-curve-right",
            Self::ArrowDown => "arrow-down",
            Self::ArrowDownBar => "arrow-down-bar",
            Self::ArrowDownCircle => "arrow-down-circle",
            Self::ArrowDownDashed => "arrow-down-dashed",
            Self::ArrowDownFromArc => "arrow-down-from-arc",
            Self::ArrowDownLeft => "arrow-down-left",
            Self::ArrowDownLeftCircle => "arrow-down-left-circle",
            Self::ArrowDownRhombus => "arrow-down-rhombus",
            Self::ArrowDownRight => "arrow-down-right",
            Self::ArrowDownRightCircle => "arrow-down-right-circle",
            Self::ArrowDownSquare => "arrow-down-square",
            Self::ArrowDownTail => "arrow-down-tail",
            Self::ArrowDownToArc => "arrow-down-to-arc",
            Self::ArrowElbowLeft => "arrow-elbow-left",
            Self::ArrowElbowRight => "arrow-elbow-right",
            Self::ArrowFork => "arrow-fork",
            Self::ArrowForward => "arrow-forward",
            Self::ArrowForwardUp => "arrow-forward-up",
            Self::ArrowForwardUpDouble => "arrow-forward-up-double",
            Self::ArrowGuide => "arrow-guide",
            Self::ArrowIteration => "arrow-iteration",
            Self::ArrowLeft => "arrow-left",
            Self::ArrowLeftBar => "arrow-left-bar",
            Self::ArrowLeftCircle => "arrow-left-circle",
            Self::ArrowLeftDashed => "arrow-left-dashed",
            Self::ArrowLeftFromArc => "arrow-left-from-arc",
            Self::ArrowLeftRhombus => "arrow-left-rhombus",
            Self::ArrowLeftRight => "arrow-left-right",
            Self::ArrowLeftSquare => "arrow-left-square",
            Self::ArrowLeftTail => "arrow-left-tail",
            Self::ArrowLeftToArc => "arrow-left-to-arc",
            Self::ArrowLoopLeft => "arrow-loop-left",
            Self::ArrowLoopLeft2 => "arrow-loop-left-2",
            Self::ArrowLoopRight => "arrow-loop-right",
            Self::ArrowLoopRight2 => "arrow-loop-right-2",
            Self::ArrowMerge => "arrow-merge",
            Self::ArrowMergeAltLeft => "arrow-merge-alt-left",
            Self::ArrowMergeAltRight => "arrow-merge-alt-right",
            Self::ArrowMergeBoth => "arrow-merge-both",
            Self::ArrowMergeLeft => "arrow-merge-left",
            Self::ArrowMergeRight => "arrow-merge-right",
            Self::ArrowMoveDown => "arrow-move-down",
            Self::ArrowMoveLeft => "arrow-move-left",
            Self::ArrowMoveRight => "arrow-move-right",
            Self::ArrowMoveUp => "arrow-move-up",
            Self::ArrowNarrowDown => "arrow-narrow-down",
            Self::ArrowNarrowDownDashed => "arrow-narrow-down-dashed",
            Self::ArrowNarrowLeft => "arrow-narrow-left",
            Self::ArrowNarrowLeftDashed => "arrow-narrow-left-dashed",
            Self::ArrowNarrowRight => "arrow-narrow-right",
            Self::ArrowNarrowRightDashed => "arrow-narrow-right-dashed",
            Self::ArrowNarrowUp => "arrow-narrow-up",
            Self::ArrowNarrowUpDashed => "arrow-narrow-up-dashed",
            Self::ArrowRampLeft => "arrow-ramp-left",
            Self::ArrowRampLeft2 => "arrow-ramp-left-2",
            Self::ArrowRampLeft3 => "arrow-ramp-left-3",
            Self::ArrowRampRight => "arrow-ramp-right",
            Self::ArrowRampRight2 => "arrow-ramp-right-2",
            Self::ArrowRampRight3 => "arrow-ramp-right-3",
            Self::ArrowRight => "arrow-right",
            Self::ArrowRightBar => "arrow-right-bar",
            Self::ArrowRightCircle => "arrow-right-circle",
            Self::ArrowRightDashed => "arrow-right-dashed",
            Self::ArrowRightFromArc => "arrow-right-from-arc",
            Self::ArrowRightRhombus => "arrow-right-rhombus",
            Self::ArrowRightSquare => "arrow-right-square",
            Self::ArrowRightTail => "arrow-right-tail",
            Self::ArrowRightToArc => "arrow-right-to-arc",
            Self::ArrowRotaryFirstLeft => "arrow-rotary-first-left",
            Self::ArrowRotaryFirstRight => "arrow-rotary-first-right",
            Self::ArrowRotaryLastLeft => "arrow-rotary-last-left",
            Self::ArrowRotaryLastRight => "arrow-rotary-last-right",
            Self::ArrowRotaryLeft => "arrow-rotary-left",
            Self::ArrowRotaryRight => "arrow-rotary-right",
            Self::ArrowRotaryStraight => "arrow-rotary-straight",
            Self::ArrowRoundaboutLeft => "arrow-roundabout-left",
            Self::ArrowRoundaboutRight => "arrow-roundabout-right",
            Self::ArrowSharpTurnLeft => "arrow-sharp-turn-left",
            Self::ArrowSharpTurnRight => "arrow-sharp-turn-right",
            Self::ArrowUp => "arrow-up",
            Self::ArrowUpBar => "arrow-up-bar",
            Self::ArrowUpCircle => "arrow-up-circle",
            Self::ArrowUpDashed => "arrow-up-dashed",
            Self::ArrowUpFromArc => "arrow-up-from-arc",
            Self::ArrowUpLeft => "arrow-up-left",
            Self::ArrowUpLeftCircle => "arrow-up-left-circle",
            Self::ArrowUpRhombus => "arrow-up-rhombus",
            Self::ArrowUpRight => "arrow-up-right",
            Self::ArrowUpRightCircle => "arrow-up-right-circle",
            Self::ArrowUpSquare => "arrow-up-square",
            Self::ArrowUpTail => "arrow-up-tail",
            Self::ArrowUpToArc => "arrow-up-to-arc",
            Self::ArrowWaveLeftDown => "arrow-wave-left-down",
            Self::ArrowWaveLeftUp => "arrow-wave-left-up",
            Self::ArrowWaveRightDown => "arrow-wave-right-down",
            Self::ArrowWaveRightUp => "arrow-wave-right-up",
            Self::ArrowZigZag => "arrow-zig-zag",
            Self::ArrowsCross => "arrows-cross",
            Self::ArrowsDiagonal => "arrows-diagonal",
            Self::ArrowsDiagonal2 => "arrows-diagonal-2",
            Self::ArrowsDiagonalMinimize => "arrows-diagonal-minimize",
            Self::ArrowsDiagonalMinimize2 => "arrows-diagonal-minimize-2",
            Self::ArrowsDiff => "arrows-diff",
            Self::ArrowsDoubleNeSw => "arrows-double-ne-sw",
            Self::ArrowsDoubleNwSe => "arrows-double-nw-se",
            Self::ArrowsDoubleSeNw => "arrows-double-se-nw",
            Self::ArrowsDoubleSwNe => "arrows-double-sw-ne",
            Self::ArrowsDown => "arrows-down",
            Self::ArrowsDownUp => "arrows-down-up",
            Self::ArrowsExchange => "arrows-exchange",
            Self::ArrowsExchange2 => "arrows-exchange-2",
            Self::ArrowsHorizontal => "arrows-horizontal",
            Self::ArrowsJoin => "arrows-join",
            Self::ArrowsJoin2 => "arrows-join-2",
            Self::ArrowsLeft => "arrows-left",
            Self::ArrowsLeftDown => "arrows-left-down",
            Self::ArrowsLeftRight => "arrows-left-right",
            Self::ArrowsMaximize => "arrows-maximize",
            Self::ArrowsMinimize => "arrows-minimize",
            Self::ArrowsMove => "arrows-move",
            Self::ArrowsMoveHorizontal => "arrows-move-horizontal",
            Self::ArrowsMoveVertical => "arrows-move-vertical",
            Self::ArrowsRandom => "arrows-random",
            Self::ArrowsRight => "arrows-right",
            Self::ArrowsRightDown => "arrows-right-down",
            Self::ArrowsRightLeft => "arrows-right-left",
            Self::ArrowsShuffle => "arrows-shuffle",
            Self::ArrowsShuffle2 => "arrows-shuffle-2",
            Self::ArrowsSort => "arrows-sort",
            Self::ArrowsSplit => "arrows-split",
            Self::ArrowsSplit2 => "arrows-split-2",
            Self::ArrowsTransferDown => "arrows-transfer-down",
            Self::ArrowsTransferUp => "arrows-transfer-up",
            Self::ArrowsTransferUpDown => "arrows-transfer-up-down",
            Self::ArrowsUp => "arrows-up",
            Self::ArrowsUpDown => "arrows-up-down",
            Self::ArrowsUpLeft => "arrows-up-left",
            Self::ArrowsUpRight => "arrows-up-right",
            Self::ArrowsVertical => "arrows-vertical",
            Self::AxisX => "axis-x",
            Self::AxisY => "axis-y",
            Self::CaretDown => "caret-down",
            Self::CaretLeft => "caret-left",
            Self::CaretRight => "caret-right",
            Self::CaretUp => "caret-up",
            Self::CaretUpDown => "caret-up-down",
            Self::ChevronCompactDown => "chevron-compact-down",
            Self::ChevronCompactLeft => "chevron-compact-left",
            Self::ChevronCompactRight => "chevron-compact-right",
            Self::ChevronCompactUp => "chevron-compact-up",
            Self::ChevronDown => "chevron-down",
            Self::ChevronDownLeft => "chevron-down-left",
            Self::ChevronDownRight => "chevron-down-right",
            Self::ChevronLeft => "chevron-left",
            Self::ChevronLeftPipe => "chevron-left-pipe",
            Self::ChevronRight => "chevron-right",
            Self::ChevronRightPipe => "chevron-right-pipe",
            Self::ChevronUp => "chevron-up",
            Self::ChevronUpLeft => "chevron-up-left",
            Self::ChevronUpRight => "chevron-up-right",
            Self::ChevronsDown => "chevrons-down",
            Self::ChevronsDownLeft => "chevrons-down-left",
            Self::ChevronsDownRight => "chevrons-down-right",
            Self::ChevronsLeft => "chevrons-left",
            Self::ChevronsRight => "chevrons-right",
            Self::ChevronsUp => "chevrons-up",
            Self::ChevronsUpLeft => "chevrons-up-left",
            Self::ChevronsUpRight => "chevrons-up-right",
            Self::CircleArrowDown => "circle-arrow-down",
            Self::CircleArrowDownLeft => "circle-arrow-down-left",
            Self::CircleArrowDownRight => "circle-arrow-down-right",
            Self::CircleArrowLeft => "circle-arrow-left",
            Self::CircleArrowRight => "circle-arrow-right",
            Self::CircleArrowUp => "circle-arrow-up",
            Self::CircleArrowUpLeft => "circle-arrow-up-left",
            Self::CircleArrowUpRight => "circle-arrow-up-right",
            Self::CircleCaretDown => "circle-caret-down",
            Self::CircleCaretLeft => "circle-caret-left",
            Self::CircleCaretRight => "circle-caret-right",
            Self::CircleCaretUp => "circle-caret-up",
            Self::CircleChevronDown => "circle-chevron-down",
            Self::CircleChevronLeft => "circle-chevron-left",
            Self::CircleChevronRight => "circle-chevron-right",
            Self::CircleChevronUp => "circle-chevron-up",
            Self::CircleChevronsDown => "circle-chevrons-down",
            Self::CircleChevronsLeft => "circle-chevrons-left",
            Self::CircleChevronsRight => "circle-chevrons-right",
            Self::CircleChevronsUp => "circle-chevrons-up",
            Self::CircleOpenArrowDown => "circle-open-arrow-down",
            Self::CircleOpenArrowLeft => "circle-open-arrow-left",
            Self::CircleOpenArrowRight => "circle-open-arrow-right",
            Self::CircleOpenArrowUp => "circle-open-arrow-up",
            Self::CornerDownLeft => "corner-down-left",
            Self::CornerDownLeftDouble => "corner-down-left-double",
            Self::CornerDownRight => "corner-down-right",
            Self::CornerDownRightDouble => "corner-down-right-double",
            Self::CornerLeftDown => "corner-left-down",
            Self::CornerLeftDownDouble => "corner-left-down-double",
            Self::CornerLeftUp => "corner-left-up",
            Self::CornerLeftUpDouble => "corner-left-up-double",
            Self::CornerRightDown => "corner-right-down",
            Self::CornerRightDownDouble => "corner-right-down-double",
            Self::CornerRightUp => "corner-right-up",
            Self::CornerRightUpDouble => "corner-right-up-double",
            Self::CornerUpLeft => "corner-up-left",
            Self::CornerUpLeftDouble => "corner-up-left-double",
            Self::CornerUpRight => "corner-up-right",
            Self::CornerUpRightDouble => "corner-up-right-double",
            Self::Download => "download",
            Self::DownloadOff => "download-off",
            Self::Fold => "fold",
            Self::FoldDown => "fold-down",
            Self::FoldUp => "fold-up",
            Self::Login => "login",
            Self::Login2 => "login-2",
            Self::Refresh => "refresh",
            Self::RefreshAlert => "refresh-alert",
            Self::RefreshDot => "refresh-dot",
            Self::RefreshOff => "refresh-off",
            Self::Reload => "reload",
            Self::RippleDown => "ripple-down",
            Self::RippleUp => "ripple-up",
            Self::Rotate => "rotate",
            Self::Rotate2 => "rotate-2",
            Self::Rotate360 => "rotate-360",
            Self::RotateClockwise => "rotate-clockwise",
            Self::RotateClockwise2 => "rotate-clockwise-2",
            Self::RotateDot => "rotate-dot",
            Self::RotateRectangle => "rotate-rectangle",
            Self::STurnDown => "s-turn-down",
            Self::STurnLeft => "s-turn-left",
            Self::STurnRight => "s-turn-right",
            Self::STurnUp => "s-turn-up",
            Self::Select => "select",
            Self::Selector => "selector",
            Self::Share2 => "share-2",
            Self::Share3 => "share-3",
            Self::SquareArrowDown => "square-arrow-down",
            Self::SquareArrowLeft => "square-arrow-left",
            Self::SquareArrowRight => "square-arrow-right",
            Self::SquareArrowUp => "square-arrow-up",
            Self::SquareChevronDown => "square-chevron-down",
            Self::SquareChevronLeft => "square-chevron-left",
            Self::SquareChevronRight => "square-chevron-right",
            Self::SquareChevronUp => "square-chevron-up",
            Self::SquareChevronsDown => "square-chevrons-down",
            Self::SquareChevronsLeft => "square-chevrons-left",
            Self::SquareChevronsRight => "square-chevrons-right",
            Self::SquareChevronsUp => "square-chevrons-up",
            Self::SquareRoundedArrowDown => "square-rounded-arrow-down",
            Self::SquareRoundedArrowLeft => "square-rounded-arrow-left",
            Self::SquareRoundedArrowRight => "square-rounded-arrow-right",
            Self::SquareRoundedArrowUp => "square-rounded-arrow-up",
            Self::SquareRoundedChevronDown => "square-rounded-chevron-down",
            Self::SquareRoundedChevronLeft => "square-rounded-chevron-left",
            Self::SquareRoundedChevronRight => "square-rounded-chevron-right",
            Self::SquareRoundedChevronUp => "square-rounded-chevron-up",
            Self::SquareRoundedChevronsDown => "square-rounded-chevrons-down",
            Self::SquareRoundedChevronsLeft => "square-rounded-chevrons-left",
            Self::SquareRoundedChevronsRight => "square-rounded-chevrons-right",
            Self::SquareRoundedChevronsUp => "square-rounded-chevrons-up",
            Self::StepInto => "step-into",
            Self::StepOut => "step-out",
            Self::SwipeDown => "swipe-down",
            Self::SwipeLeft => "swipe-left",
            Self::SwipeRight => "swipe-right",
            Self::SwipeUp => "swipe-up",
            Self::Switch => "switch",
            Self::Switch2 => "switch-2",
            Self::Switch3 => "switch-3",
            Self::SwitchHorizontal => "switch-horizontal",
            Self::SwitchVertical => "switch-vertical",
            Self::TransitionBottom => "transition-bottom",
            Self::TransitionLeft => "transition-left",
            Self::TransitionRight => "transition-right",
            Self::TransitionTop => "transition-top",
            Self::TrendingDown => "trending-down",
            Self::TrendingDown2 => "trending-down-2",
            Self::TrendingDown3 => "trending-down-3",
            Self::TrendingUp => "trending-up",
            Self::TrendingUp2 => "trending-up-2",
            Self::TrendingUp3 => "trending-up-3",
            Self::UTurnLeft => "u-turn-left",
            Self::UTurnRight => "u-turn-right",
            Self::Upload => "upload",
        }
    }

    fn outline_svg(&self) -> &'static str {
        match self {
            Self::ArrowAutofitContent => ARROW_AUTOFIT_CONTENT_SVG,
            Self::ArrowAutofitDown => ARROW_AUTOFIT_DOWN_SVG,
            Self::ArrowAutofitHeight => ARROW_AUTOFIT_HEIGHT_SVG,
            Self::ArrowAutofitLeft => ARROW_AUTOFIT_LEFT_SVG,
            Self::ArrowAutofitRight => ARROW_AUTOFIT_RIGHT_SVG,
            Self::ArrowAutofitUp => ARROW_AUTOFIT_UP_SVG,
            Self::ArrowAutofitWidth => ARROW_AUTOFIT_WIDTH_SVG,
            Self::ArrowBack => ARROW_BACK_SVG,
            Self::ArrowBackUp => ARROW_BACK_UP_SVG,
            Self::ArrowBackUpDouble => ARROW_BACK_UP_DOUBLE_SVG,
            Self::ArrowBadgeDown => ARROW_BADGE_DOWN_SVG,
            Self::ArrowBadgeLeft => ARROW_BADGE_LEFT_SVG,
            Self::ArrowBadgeRight => ARROW_BADGE_RIGHT_SVG,
            Self::ArrowBadgeUp => ARROW_BADGE_UP_SVG,
            Self::ArrowBarBoth => ARROW_BAR_BOTH_SVG,
            Self::ArrowBarDown => ARROW_BAR_DOWN_SVG,
            Self::ArrowBarLeft => ARROW_BAR_LEFT_SVG,
            Self::ArrowBarRight => ARROW_BAR_RIGHT_SVG,
            Self::ArrowBarToDown => ARROW_BAR_TO_DOWN_SVG,
            Self::ArrowBarToDownDashed => ARROW_BAR_TO_DOWN_DASHED_SVG,
            Self::ArrowBarToLeft => ARROW_BAR_TO_LEFT_SVG,
            Self::ArrowBarToLeftDashed => ARROW_BAR_TO_LEFT_DASHED_SVG,
            Self::ArrowBarToRight => ARROW_BAR_TO_RIGHT_SVG,
            Self::ArrowBarToRightDashed => ARROW_BAR_TO_RIGHT_DASHED_SVG,
            Self::ArrowBarToUp => ARROW_BAR_TO_UP_SVG,
            Self::ArrowBarToUpDashed => ARROW_BAR_TO_UP_DASHED_SVG,
            Self::ArrowBarUp => ARROW_BAR_UP_SVG,
            Self::ArrowBearLeft => ARROW_BEAR_LEFT_SVG,
            Self::ArrowBearLeft2 => ARROW_BEAR_LEFT_2_SVG,
            Self::ArrowBearRight => ARROW_BEAR_RIGHT_SVG,
            Self::ArrowBearRight2 => ARROW_BEAR_RIGHT_2_SVG,
            Self::ArrowBigDown => ARROW_BIG_DOWN_SVG,
            Self::ArrowBigDownLine => ARROW_BIG_DOWN_LINE_SVG,
            Self::ArrowBigDownLines => ARROW_BIG_DOWN_LINES_SVG,
            Self::ArrowBigLeft => ARROW_BIG_LEFT_SVG,
            Self::ArrowBigLeftLine => ARROW_BIG_LEFT_LINE_SVG,
            Self::ArrowBigLeftLines => ARROW_BIG_LEFT_LINES_SVG,
            Self::ArrowBigRight => ARROW_BIG_RIGHT_SVG,
            Self::ArrowBigRightLine => ARROW_BIG_RIGHT_LINE_SVG,
            Self::ArrowBigRightLines => ARROW_BIG_RIGHT_LINES_SVG,
            Self::ArrowBigUp => ARROW_BIG_UP_SVG,
            Self::ArrowBigUpLine => ARROW_BIG_UP_LINE_SVG,
            Self::ArrowBigUpLines => ARROW_BIG_UP_LINES_SVG,
            Self::ArrowBounce => ARROW_BOUNCE_SVG,
            Self::ArrowCapsule => ARROW_CAPSULE_SVG,
            Self::ArrowCurveLeft => ARROW_CURVE_LEFT_SVG,
            Self::ArrowCurveRight => ARROW_CURVE_RIGHT_SVG,
            Self::ArrowDown => ARROW_DOWN_SVG,
            Self::ArrowDownBar => ARROW_DOWN_BAR_SVG,
            Self::ArrowDownCircle => ARROW_DOWN_CIRCLE_SVG,
            Self::ArrowDownDashed => ARROW_DOWN_DASHED_SVG,
            Self::ArrowDownFromArc => ARROW_DOWN_FROM_ARC_SVG,
            Self::ArrowDownLeft => ARROW_DOWN_LEFT_SVG,
            Self::ArrowDownLeftCircle => ARROW_DOWN_LEFT_CIRCLE_SVG,
            Self::ArrowDownRhombus => ARROW_DOWN_RHOMBUS_SVG,
            Self::ArrowDownRight => ARROW_DOWN_RIGHT_SVG,
            Self::ArrowDownRightCircle => ARROW_DOWN_RIGHT_CIRCLE_SVG,
            Self::ArrowDownSquare => ARROW_DOWN_SQUARE_SVG,
            Self::ArrowDownTail => ARROW_DOWN_TAIL_SVG,
            Self::ArrowDownToArc => ARROW_DOWN_TO_ARC_SVG,
            Self::ArrowElbowLeft => ARROW_ELBOW_LEFT_SVG,
            Self::ArrowElbowRight => ARROW_ELBOW_RIGHT_SVG,
            Self::ArrowFork => ARROW_FORK_SVG,
            Self::ArrowForward => ARROW_FORWARD_SVG,
            Self::ArrowForwardUp => ARROW_FORWARD_UP_SVG,
            Self::ArrowForwardUpDouble => ARROW_FORWARD_UP_DOUBLE_SVG,
            Self::ArrowGuide => ARROW_GUIDE_SVG,
            Self::ArrowIteration => ARROW_ITERATION_SVG,
            Self::ArrowLeft => ARROW_LEFT_SVG,
            Self::ArrowLeftBar => ARROW_LEFT_BAR_SVG,
            Self::ArrowLeftCircle => ARROW_LEFT_CIRCLE_SVG,
            Self::ArrowLeftDashed => ARROW_LEFT_DASHED_SVG,
            Self::ArrowLeftFromArc => ARROW_LEFT_FROM_ARC_SVG,
            Self::ArrowLeftRhombus => ARROW_LEFT_RHOMBUS_SVG,
            Self::ArrowLeftRight => ARROW_LEFT_RIGHT_SVG,
            Self::ArrowLeftSquare => ARROW_LEFT_SQUARE_SVG,
            Self::ArrowLeftTail => ARROW_LEFT_TAIL_SVG,
            Self::ArrowLeftToArc => ARROW_LEFT_TO_ARC_SVG,
            Self::ArrowLoopLeft => ARROW_LOOP_LEFT_SVG,
            Self::ArrowLoopLeft2 => ARROW_LOOP_LEFT_2_SVG,
            Self::ArrowLoopRight => ARROW_LOOP_RIGHT_SVG,
            Self::ArrowLoopRight2 => ARROW_LOOP_RIGHT_2_SVG,
            Self::ArrowMerge => ARROW_MERGE_SVG,
            Self::ArrowMergeAltLeft => ARROW_MERGE_ALT_LEFT_SVG,
            Self::ArrowMergeAltRight => ARROW_MERGE_ALT_RIGHT_SVG,
            Self::ArrowMergeBoth => ARROW_MERGE_BOTH_SVG,
            Self::ArrowMergeLeft => ARROW_MERGE_LEFT_SVG,
            Self::ArrowMergeRight => ARROW_MERGE_RIGHT_SVG,
            Self::ArrowMoveDown => ARROW_MOVE_DOWN_SVG,
            Self::ArrowMoveLeft => ARROW_MOVE_LEFT_SVG,
            Self::ArrowMoveRight => ARROW_MOVE_RIGHT_SVG,
            Self::ArrowMoveUp => ARROW_MOVE_UP_SVG,
            Self::ArrowNarrowDown => ARROW_NARROW_DOWN_SVG,
            Self::ArrowNarrowDownDashed => ARROW_NARROW_DOWN_DASHED_SVG,
            Self::ArrowNarrowLeft => ARROW_NARROW_LEFT_SVG,
            Self::ArrowNarrowLeftDashed => ARROW_NARROW_LEFT_DASHED_SVG,
            Self::ArrowNarrowRight => ARROW_NARROW_RIGHT_SVG,
            Self::ArrowNarrowRightDashed => ARROW_NARROW_RIGHT_DASHED_SVG,
            Self::ArrowNarrowUp => ARROW_NARROW_UP_SVG,
            Self::ArrowNarrowUpDashed => ARROW_NARROW_UP_DASHED_SVG,
            Self::ArrowRampLeft => ARROW_RAMP_LEFT_SVG,
            Self::ArrowRampLeft2 => ARROW_RAMP_LEFT_2_SVG,
            Self::ArrowRampLeft3 => ARROW_RAMP_LEFT_3_SVG,
            Self::ArrowRampRight => ARROW_RAMP_RIGHT_SVG,
            Self::ArrowRampRight2 => ARROW_RAMP_RIGHT_2_SVG,
            Self::ArrowRampRight3 => ARROW_RAMP_RIGHT_3_SVG,
            Self::ArrowRight => ARROW_RIGHT_SVG,
            Self::ArrowRightBar => ARROW_RIGHT_BAR_SVG,
            Self::ArrowRightCircle => ARROW_RIGHT_CIRCLE_SVG,
            Self::ArrowRightDashed => ARROW_RIGHT_DASHED_SVG,
            Self::ArrowRightFromArc => ARROW_RIGHT_FROM_ARC_SVG,
            Self::ArrowRightRhombus => ARROW_RIGHT_RHOMBUS_SVG,
            Self::ArrowRightSquare => ARROW_RIGHT_SQUARE_SVG,
            Self::ArrowRightTail => ARROW_RIGHT_TAIL_SVG,
            Self::ArrowRightToArc => ARROW_RIGHT_TO_ARC_SVG,
            Self::ArrowRotaryFirstLeft => ARROW_ROTARY_FIRST_LEFT_SVG,
            Self::ArrowRotaryFirstRight => ARROW_ROTARY_FIRST_RIGHT_SVG,
            Self::ArrowRotaryLastLeft => ARROW_ROTARY_LAST_LEFT_SVG,
            Self::ArrowRotaryLastRight => ARROW_ROTARY_LAST_RIGHT_SVG,
            Self::ArrowRotaryLeft => ARROW_ROTARY_LEFT_SVG,
            Self::ArrowRotaryRight => ARROW_ROTARY_RIGHT_SVG,
            Self::ArrowRotaryStraight => ARROW_ROTARY_STRAIGHT_SVG,
            Self::ArrowRoundaboutLeft => ARROW_ROUNDABOUT_LEFT_SVG,
            Self::ArrowRoundaboutRight => ARROW_ROUNDABOUT_RIGHT_SVG,
            Self::ArrowSharpTurnLeft => ARROW_SHARP_TURN_LEFT_SVG,
            Self::ArrowSharpTurnRight => ARROW_SHARP_TURN_RIGHT_SVG,
            Self::ArrowUp => ARROW_UP_SVG,
            Self::ArrowUpBar => ARROW_UP_BAR_SVG,
            Self::ArrowUpCircle => ARROW_UP_CIRCLE_SVG,
            Self::ArrowUpDashed => ARROW_UP_DASHED_SVG,
            Self::ArrowUpFromArc => ARROW_UP_FROM_ARC_SVG,
            Self::ArrowUpLeft => ARROW_UP_LEFT_SVG,
            Self::ArrowUpLeftCircle => ARROW_UP_LEFT_CIRCLE_SVG,
            Self::ArrowUpRhombus => ARROW_UP_RHOMBUS_SVG,
            Self::ArrowUpRight => ARROW_UP_RIGHT_SVG,
            Self::ArrowUpRightCircle => ARROW_UP_RIGHT_CIRCLE_SVG,
            Self::ArrowUpSquare => ARROW_UP_SQUARE_SVG,
            Self::ArrowUpTail => ARROW_UP_TAIL_SVG,
            Self::ArrowUpToArc => ARROW_UP_TO_ARC_SVG,
            Self::ArrowWaveLeftDown => ARROW_WAVE_LEFT_DOWN_SVG,
            Self::ArrowWaveLeftUp => ARROW_WAVE_LEFT_UP_SVG,
            Self::ArrowWaveRightDown => ARROW_WAVE_RIGHT_DOWN_SVG,
            Self::ArrowWaveRightUp => ARROW_WAVE_RIGHT_UP_SVG,
            Self::ArrowZigZag => ARROW_ZIG_ZAG_SVG,
            Self::ArrowsCross => ARROWS_CROSS_SVG,
            Self::ArrowsDiagonal => ARROWS_DIAGONAL_SVG,
            Self::ArrowsDiagonal2 => ARROWS_DIAGONAL_2_SVG,
            Self::ArrowsDiagonalMinimize => ARROWS_DIAGONAL_MINIMIZE_SVG,
            Self::ArrowsDiagonalMinimize2 => ARROWS_DIAGONAL_MINIMIZE_2_SVG,
            Self::ArrowsDiff => ARROWS_DIFF_SVG,
            Self::ArrowsDoubleNeSw => ARROWS_DOUBLE_NE_SW_SVG,
            Self::ArrowsDoubleNwSe => ARROWS_DOUBLE_NW_SE_SVG,
            Self::ArrowsDoubleSeNw => ARROWS_DOUBLE_SE_NW_SVG,
            Self::ArrowsDoubleSwNe => ARROWS_DOUBLE_SW_NE_SVG,
            Self::ArrowsDown => ARROWS_DOWN_SVG,
            Self::ArrowsDownUp => ARROWS_DOWN_UP_SVG,
            Self::ArrowsExchange => ARROWS_EXCHANGE_SVG,
            Self::ArrowsExchange2 => ARROWS_EXCHANGE_2_SVG,
            Self::ArrowsHorizontal => ARROWS_HORIZONTAL_SVG,
            Self::ArrowsJoin => ARROWS_JOIN_SVG,
            Self::ArrowsJoin2 => ARROWS_JOIN_2_SVG,
            Self::ArrowsLeft => ARROWS_LEFT_SVG,
            Self::ArrowsLeftDown => ARROWS_LEFT_DOWN_SVG,
            Self::ArrowsLeftRight => ARROWS_LEFT_RIGHT_SVG,
            Self::ArrowsMaximize => ARROWS_MAXIMIZE_SVG,
            Self::ArrowsMinimize => ARROWS_MINIMIZE_SVG,
            Self::ArrowsMove => ARROWS_MOVE_SVG,
            Self::ArrowsMoveHorizontal => ARROWS_MOVE_HORIZONTAL_SVG,
            Self::ArrowsMoveVertical => ARROWS_MOVE_VERTICAL_SVG,
            Self::ArrowsRandom => ARROWS_RANDOM_SVG,
            Self::ArrowsRight => ARROWS_RIGHT_SVG,
            Self::ArrowsRightDown => ARROWS_RIGHT_DOWN_SVG,
            Self::ArrowsRightLeft => ARROWS_RIGHT_LEFT_SVG,
            Self::ArrowsShuffle => ARROWS_SHUFFLE_SVG,
            Self::ArrowsShuffle2 => ARROWS_SHUFFLE_2_SVG,
            Self::ArrowsSort => ARROWS_SORT_SVG,
            Self::ArrowsSplit => ARROWS_SPLIT_SVG,
            Self::ArrowsSplit2 => ARROWS_SPLIT_2_SVG,
            Self::ArrowsTransferDown => ARROWS_TRANSFER_DOWN_SVG,
            Self::ArrowsTransferUp => ARROWS_TRANSFER_UP_SVG,
            Self::ArrowsTransferUpDown => ARROWS_TRANSFER_UP_DOWN_SVG,
            Self::ArrowsUp => ARROWS_UP_SVG,
            Self::ArrowsUpDown => ARROWS_UP_DOWN_SVG,
            Self::ArrowsUpLeft => ARROWS_UP_LEFT_SVG,
            Self::ArrowsUpRight => ARROWS_UP_RIGHT_SVG,
            Self::ArrowsVertical => ARROWS_VERTICAL_SVG,
            Self::AxisX => AXIS_X_SVG,
            Self::AxisY => AXIS_Y_SVG,
            Self::CaretDown => CARET_DOWN_SVG,
            Self::CaretLeft => CARET_LEFT_SVG,
            Self::CaretRight => CARET_RIGHT_SVG,
            Self::CaretUp => CARET_UP_SVG,
            Self::CaretUpDown => CARET_UP_DOWN_SVG,
            Self::ChevronCompactDown => CHEVRON_COMPACT_DOWN_SVG,
            Self::ChevronCompactLeft => CHEVRON_COMPACT_LEFT_SVG,
            Self::ChevronCompactRight => CHEVRON_COMPACT_RIGHT_SVG,
            Self::ChevronCompactUp => CHEVRON_COMPACT_UP_SVG,
            Self::ChevronDown => CHEVRON_DOWN_SVG,
            Self::ChevronDownLeft => CHEVRON_DOWN_LEFT_SVG,
            Self::ChevronDownRight => CHEVRON_DOWN_RIGHT_SVG,
            Self::ChevronLeft => CHEVRON_LEFT_SVG,
            Self::ChevronLeftPipe => CHEVRON_LEFT_PIPE_SVG,
            Self::ChevronRight => CHEVRON_RIGHT_SVG,
            Self::ChevronRightPipe => CHEVRON_RIGHT_PIPE_SVG,
            Self::ChevronUp => CHEVRON_UP_SVG,
            Self::ChevronUpLeft => CHEVRON_UP_LEFT_SVG,
            Self::ChevronUpRight => CHEVRON_UP_RIGHT_SVG,
            Self::ChevronsDown => CHEVRONS_DOWN_SVG,
            Self::ChevronsDownLeft => CHEVRONS_DOWN_LEFT_SVG,
            Self::ChevronsDownRight => CHEVRONS_DOWN_RIGHT_SVG,
            Self::ChevronsLeft => CHEVRONS_LEFT_SVG,
            Self::ChevronsRight => CHEVRONS_RIGHT_SVG,
            Self::ChevronsUp => CHEVRONS_UP_SVG,
            Self::ChevronsUpLeft => CHEVRONS_UP_LEFT_SVG,
            Self::ChevronsUpRight => CHEVRONS_UP_RIGHT_SVG,
            Self::CircleArrowDown => CIRCLE_ARROW_DOWN_SVG,
            Self::CircleArrowDownLeft => CIRCLE_ARROW_DOWN_LEFT_SVG,
            Self::CircleArrowDownRight => CIRCLE_ARROW_DOWN_RIGHT_SVG,
            Self::CircleArrowLeft => CIRCLE_ARROW_LEFT_SVG,
            Self::CircleArrowRight => CIRCLE_ARROW_RIGHT_SVG,
            Self::CircleArrowUp => CIRCLE_ARROW_UP_SVG,
            Self::CircleArrowUpLeft => CIRCLE_ARROW_UP_LEFT_SVG,
            Self::CircleArrowUpRight => CIRCLE_ARROW_UP_RIGHT_SVG,
            Self::CircleCaretDown => CIRCLE_CARET_DOWN_SVG,
            Self::CircleCaretLeft => CIRCLE_CARET_LEFT_SVG,
            Self::CircleCaretRight => CIRCLE_CARET_RIGHT_SVG,
            Self::CircleCaretUp => CIRCLE_CARET_UP_SVG,
            Self::CircleChevronDown => CIRCLE_CHEVRON_DOWN_SVG,
            Self::CircleChevronLeft => CIRCLE_CHEVRON_LEFT_SVG,
            Self::CircleChevronRight => CIRCLE_CHEVRON_RIGHT_SVG,
            Self::CircleChevronUp => CIRCLE_CHEVRON_UP_SVG,
            Self::CircleChevronsDown => CIRCLE_CHEVRONS_DOWN_SVG,
            Self::CircleChevronsLeft => CIRCLE_CHEVRONS_LEFT_SVG,
            Self::CircleChevronsRight => CIRCLE_CHEVRONS_RIGHT_SVG,
            Self::CircleChevronsUp => CIRCLE_CHEVRONS_UP_SVG,
            Self::CircleOpenArrowDown => CIRCLE_OPEN_ARROW_DOWN_SVG,
            Self::CircleOpenArrowLeft => CIRCLE_OPEN_ARROW_LEFT_SVG,
            Self::CircleOpenArrowRight => CIRCLE_OPEN_ARROW_RIGHT_SVG,
            Self::CircleOpenArrowUp => CIRCLE_OPEN_ARROW_UP_SVG,
            Self::CornerDownLeft => CORNER_DOWN_LEFT_SVG,
            Self::CornerDownLeftDouble => CORNER_DOWN_LEFT_DOUBLE_SVG,
            Self::CornerDownRight => CORNER_DOWN_RIGHT_SVG,
            Self::CornerDownRightDouble => CORNER_DOWN_RIGHT_DOUBLE_SVG,
            Self::CornerLeftDown => CORNER_LEFT_DOWN_SVG,
            Self::CornerLeftDownDouble => CORNER_LEFT_DOWN_DOUBLE_SVG,
            Self::CornerLeftUp => CORNER_LEFT_UP_SVG,
            Self::CornerLeftUpDouble => CORNER_LEFT_UP_DOUBLE_SVG,
            Self::CornerRightDown => CORNER_RIGHT_DOWN_SVG,
            Self::CornerRightDownDouble => CORNER_RIGHT_DOWN_DOUBLE_SVG,
            Self::CornerRightUp => CORNER_RIGHT_UP_SVG,
            Self::CornerRightUpDouble => CORNER_RIGHT_UP_DOUBLE_SVG,
            Self::CornerUpLeft => CORNER_UP_LEFT_SVG,
            Self::CornerUpLeftDouble => CORNER_UP_LEFT_DOUBLE_SVG,
            Self::CornerUpRight => CORNER_UP_RIGHT_SVG,
            Self::CornerUpRightDouble => CORNER_UP_RIGHT_DOUBLE_SVG,
            Self::Download => DOWNLOAD_SVG,
            Self::DownloadOff => DOWNLOAD_OFF_SVG,
            Self::Fold => FOLD_SVG,
            Self::FoldDown => FOLD_DOWN_SVG,
            Self::FoldUp => FOLD_UP_SVG,
            Self::Login => LOGIN_SVG,
            Self::Login2 => LOGIN_2_SVG,
            Self::Refresh => REFRESH_SVG,
            Self::RefreshAlert => REFRESH_ALERT_SVG,
            Self::RefreshDot => REFRESH_DOT_SVG,
            Self::RefreshOff => REFRESH_OFF_SVG,
            Self::Reload => RELOAD_SVG,
            Self::RippleDown => RIPPLE_DOWN_SVG,
            Self::RippleUp => RIPPLE_UP_SVG,
            Self::Rotate => ROTATE_SVG,
            Self::Rotate2 => ROTATE_2_SVG,
            Self::Rotate360 => ROTATE_360_SVG,
            Self::RotateClockwise => ROTATE_CLOCKWISE_SVG,
            Self::RotateClockwise2 => ROTATE_CLOCKWISE_2_SVG,
            Self::RotateDot => ROTATE_DOT_SVG,
            Self::RotateRectangle => ROTATE_RECTANGLE_SVG,
            Self::STurnDown => S_TURN_DOWN_SVG,
            Self::STurnLeft => S_TURN_LEFT_SVG,
            Self::STurnRight => S_TURN_RIGHT_SVG,
            Self::STurnUp => S_TURN_UP_SVG,
            Self::Select => SELECT_SVG,
            Self::Selector => SELECTOR_SVG,
            Self::Share2 => SHARE_2_SVG,
            Self::Share3 => SHARE_3_SVG,
            Self::SquareArrowDown => SQUARE_ARROW_DOWN_SVG,
            Self::SquareArrowLeft => SQUARE_ARROW_LEFT_SVG,
            Self::SquareArrowRight => SQUARE_ARROW_RIGHT_SVG,
            Self::SquareArrowUp => SQUARE_ARROW_UP_SVG,
            Self::SquareChevronDown => SQUARE_CHEVRON_DOWN_SVG,
            Self::SquareChevronLeft => SQUARE_CHEVRON_LEFT_SVG,
            Self::SquareChevronRight => SQUARE_CHEVRON_RIGHT_SVG,
            Self::SquareChevronUp => SQUARE_CHEVRON_UP_SVG,
            Self::SquareChevronsDown => SQUARE_CHEVRONS_DOWN_SVG,
            Self::SquareChevronsLeft => SQUARE_CHEVRONS_LEFT_SVG,
            Self::SquareChevronsRight => SQUARE_CHEVRONS_RIGHT_SVG,
            Self::SquareChevronsUp => SQUARE_CHEVRONS_UP_SVG,
            Self::SquareRoundedArrowDown => SQUARE_ROUNDED_ARROW_DOWN_SVG,
            Self::SquareRoundedArrowLeft => SQUARE_ROUNDED_ARROW_LEFT_SVG,
            Self::SquareRoundedArrowRight => SQUARE_ROUNDED_ARROW_RIGHT_SVG,
            Self::SquareRoundedArrowUp => SQUARE_ROUNDED_ARROW_UP_SVG,
            Self::SquareRoundedChevronDown => SQUARE_ROUNDED_CHEVRON_DOWN_SVG,
            Self::SquareRoundedChevronLeft => SQUARE_ROUNDED_CHEVRON_LEFT_SVG,
            Self::SquareRoundedChevronRight => SQUARE_ROUNDED_CHEVRON_RIGHT_SVG,
            Self::SquareRoundedChevronUp => SQUARE_ROUNDED_CHEVRON_UP_SVG,
            Self::SquareRoundedChevronsDown => SQUARE_ROUNDED_CHEVRONS_DOWN_SVG,
            Self::SquareRoundedChevronsLeft => SQUARE_ROUNDED_CHEVRONS_LEFT_SVG,
            Self::SquareRoundedChevronsRight => SQUARE_ROUNDED_CHEVRONS_RIGHT_SVG,
            Self::SquareRoundedChevronsUp => SQUARE_ROUNDED_CHEVRONS_UP_SVG,
            Self::StepInto => STEP_INTO_SVG,
            Self::StepOut => STEP_OUT_SVG,
            Self::SwipeDown => SWIPE_DOWN_SVG,
            Self::SwipeLeft => SWIPE_LEFT_SVG,
            Self::SwipeRight => SWIPE_RIGHT_SVG,
            Self::SwipeUp => SWIPE_UP_SVG,
            Self::Switch => SWITCH_SVG,
            Self::Switch2 => SWITCH_2_SVG,
            Self::Switch3 => SWITCH_3_SVG,
            Self::SwitchHorizontal => SWITCH_HORIZONTAL_SVG,
            Self::SwitchVertical => SWITCH_VERTICAL_SVG,
            Self::TransitionBottom => TRANSITION_BOTTOM_SVG,
            Self::TransitionLeft => TRANSITION_LEFT_SVG,
            Self::TransitionRight => TRANSITION_RIGHT_SVG,
            Self::TransitionTop => TRANSITION_TOP_SVG,
            Self::TrendingDown => TRENDING_DOWN_SVG,
            Self::TrendingDown2 => TRENDING_DOWN_2_SVG,
            Self::TrendingDown3 => TRENDING_DOWN_3_SVG,
            Self::TrendingUp => TRENDING_UP_SVG,
            Self::TrendingUp2 => TRENDING_UP_2_SVG,
            Self::TrendingUp3 => TRENDING_UP_3_SVG,
            Self::UTurnLeft => U_TURN_LEFT_SVG,
            Self::UTurnRight => U_TURN_RIGHT_SVG,
            Self::Upload => UPLOAD_SVG,
        }
    }

    fn filled_svg(&self) -> Option<&'static str> {
        // Filled variants would be added here
        None
    }
}
