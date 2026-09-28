//! Media icons from Tabler Icons.
//!
//! This module contains 277 icons.

use crate::tabler::TablerIconData;

// SVG Constants
const ACROBATIC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.207 3l-6.735 2.462a1 1 0 0 0 -.364 1.646l1.892 1.892" /> <path d="M10.5 8.25l1.5 -.25h3.174a2 2 0 0 1 1.411 .583l1.422 1.417" /> <path d="M8 9c0 4.5 1.781 5.14 3 5.5" /> <path d="M13.007 21h-1a1 1 0 0 1 -1 -1l-.007 -5.5" /> <path d="M12.007 14a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const ALBUM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M12 4v7l2 -2l2 2v-7" /> </svg>"##;
const ALBUM_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h10a2 2 0 0 1 2 2v10m-.581 3.41c-.362 .364 -.864 .59 -1.419 .59h-12a2 2 0 0 1 -2 -2v-12c0 -.552 .224 -1.052 .585 -1.413" /> <path d="M12 4v4m1.503 1.497l.497 -.497l2 2v-7" /> <path d="M3 3l18 18" /> </svg>"##;
const ARCHERY_ARROW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 7v3h3l3 -3h-3v-3l-3 3" /> <path d="M14 10l-9 9" /> <path d="M5 15v4h4" /> </svg>"##;
const ASPECT_RATIO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M7 12v-3h3" /> <path d="M17 12v3h-3" /> </svg>"##;
const ASPECT_RATIO_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h10a2 2 0 0 1 2 2v10m-2 2h-14a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2" /> <path d="M7 12v-3h2" /> <path d="M17 12v1m-2 2h-1" /> <path d="M3 3l18 18" /> </svg>"##;
const AXE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 9l7.383 7.418c.823 .82 .823 2.148 0 2.967a2.11 2.11 0 0 1 -2.976 0l-7.407 -7.385" /> <path d="M6.66 15.66l-3.32 -3.32a1.25 1.25 0 0 1 .42 -2.044l3.24 -1.296l6 -6l3 3l-6 6l-1.296 3.24a1.25 1.25 0 0 1 -2.044 .42" /> </svg>"##;
const BALL_AMERICAN_FOOTBALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 9l-6 6" /> <path d="M10 12l2 2" /> <path d="M12 10l2 2" /> <path d="M8 21a5 5 0 0 0 -5 -5" /> <path d="M16 3c-7.18 0 -13 5.82 -13 13a5 5 0 0 0 5 5c7.18 0 13 -5.82 13 -13a5 5 0 0 0 -5 -5" /> <path d="M16 3a5 5 0 0 0 5 5" /> </svg>"##;
const BALL_AMERICAN_FOOTBALL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 9l-1 1m-2 2l-3 3" /> <path d="M10 12l2 2" /> <path d="M8 21a5 5 0 0 0 -5 -5" /> <path d="M6.813 6.802a12.96 12.96 0 0 0 -3.813 9.198a5 5 0 0 0 5 5a12.96 12.96 0 0 0 9.186 -3.801m1.789 -2.227a12.94 12.94 0 0 0 2.025 -6.972a5 5 0 0 0 -5 -5a12.94 12.94 0 0 0 -6.967 2.022" /> <path d="M16 3a5 5 0 0 0 5 5" /> <path d="M3 3l18 18" /> </svg>"##;
const BALL_BASEBALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.636 18.364a9 9 0 1 0 12.728 -12.728a9 9 0 0 0 -12.728 12.728" /> <path d="M12.495 3.02a9 9 0 0 1 -9.475 9.475" /> <path d="M20.98 11.505a9 9 0 0 0 -9.475 9.475" /> <path d="M9 9l2 2" /> <path d="M13 13l2 2" /> <path d="M11 7l2 1" /> <path d="M7 11l1 2" /> <path d="M16 11l1 2" /> <path d="M11 16l2 1" /> </svg>"##;
const BALL_BASKETBALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M5.65 5.65l12.7 12.7" /> <path d="M5.65 18.35l12.7 -12.7" /> <path d="M12 3a9 9 0 0 0 9 9" /> <path d="M3 12a9 9 0 0 1 9 9" /> </svg>"##;
const BALL_BOWLING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M11 9l0 .01" /> <path d="M15 8l0 .01" /> <path d="M14 12l0 .01" /> </svg>"##;
const BALL_FOOTBALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 7l4.76 3.45l-1.76 5.55h-6l-1.76 -5.55l4.76 -3.45" /> <path d="M12 7v-4m3 13l2.5 3m-.74 -8.55l3.74 -1.45m-11.44 7.05l-2.56 2.95m.74 -8.55l-3.74 -1.45" /> </svg>"##;
const BALL_FOOTBALL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.041 16.046a9 9 0 0 0 -12.084 -12.09m-2.323 1.683a9 9 0 0 0 12.726 12.73" /> <path d="M12 7l4.755 3.455l-.566 1.743l-.98 3.014l-.209 .788h-6l-1.755 -5.545l1.86 -1.351l2.313 -1.681l.582 -.423" /> <path d="M12 7v-4" /> <path d="M15 16l2.5 3" /> <path d="M16.755 10.455l3.745 -1.455" /> <path d="M9.061 16.045l-2.561 2.955" /> <path d="M7.245 10.455l-3.745 -1.455" /> <path d="M3 3l18 18" /> </svg>"##;
const BALL_TENNIS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M6 5.3a9 9 0 0 1 0 13.4" /> <path d="M18 5.3a9 9 0 0 0 0 13.4" /> </svg>"##;
const BALL_VOLLEYBALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 12a8 8 0 0 0 8 4" /> <path d="M7.5 13.5a12 12 0 0 0 8.5 6.5" /> <path d="M12 12a8 8 0 0 0 -7.464 4.928" /> <path d="M12.951 7.353a12 12 0 0 0 -9.88 4.111" /> <path d="M12 12a8 8 0 0 0 -.536 -8.928" /> <path d="M15.549 15.147a12 12 0 0 0 1.38 -10.611" /> </svg>"##;
const BARBELL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 12h1" /> <path d="M6 8h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2" /> <path d="M6 7v10a1 1 0 0 0 1 1h1a1 1 0 0 0 1 -1v-10a1 1 0 0 0 -1 -1h-1a1 1 0 0 0 -1 1" /> <path d="M9 12h6" /> <path d="M15 7v10a1 1 0 0 0 1 1h1a1 1 0 0 0 1 -1v-10a1 1 0 0 0 -1 -1h-1a1 1 0 0 0 -1 1" /> <path d="M18 8h2a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-2" /> <path d="M22 12h-1" /> </svg>"##;
const BARBELL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 12h1" /> <path d="M6 8h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2" /> <path d="M6.298 6.288a1 1 0 0 0 -.298 .712v10a1 1 0 0 0 1 1h1a1 1 0 0 0 1 -1v-8" /> <path d="M9 12h3" /> <path d="M15 15v2a1 1 0 0 0 1 1h1c.275 0 .523 -.11 .704 -.29m.296 -3.71v-7a1 1 0 0 0 -1 -1h-1a1 1 0 0 0 -1 1v4" /> <path d="M18 8h2a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1" /> <path d="M22 12h-1" /> <path d="M3 3l18 18" /> </svg>"##;
const BOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 3h4v4" /> <path d="M21 3l-15 15" /> <path d="M3 18h3v3" /> <path d="M16.5 20c1.576 -1.576 2.5 -4.095 2.5 -6.5c0 -4.81 -3.69 -8.5 -8.5 -8.5c-2.415 0 -4.922 .913 -6.5 2.5l12.5 12.5" /> </svg>"##;
const BOWLING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 11v.01" /> <path d="M11 10v.01" /> <path d="M10 14v.01" /> <path d="M11.059 6.07a8 8 0 1 0 .32 15.81" /> <path d="M15.969 9h4" /> <path d="M14.969 5c0 1.5 1 2 1 4c0 2.5 -2 4.5 -2 7c0 2.6 1.9 6 1.9 6h4.1s2 -3.4 2 -6c0 -2.5 -2 -4.5 -2 -7c0 -2 1 -2.5 1 -4a3 3 0 1 0 -6 0" /> </svg>"##;
const CAMERA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v9a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2" /> <path d="M9 13a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> </svg>"##;
const CAMERA_AI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20h-5a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v2" /> <path d="M14.362 11.15a3 3 0 1 0 -4.144 4.263" /> <path d="M14 21v-4a2 2 0 1 1 4 0v4" /> <path d="M14 19h4" /> <path d="M21 15v6" /> </svg>"##;
const CAMERA_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v1" /> <path d="M14.477 11.307a3 3 0 1 0 -2.477 4.693" /> <path d="M17 21v-6" /> <path d="M19 15v-1.5" /> <path d="M19 22.5v-1.5" /> <path d="M17 18h3" /> <path d="M19 18h.5a1.5 1.5 0 0 1 0 3h-3.5" /> <path d="M19 18h.5a1.5 1.5 0 0 0 0 -3h-3.5" /> </svg>"##;
const CAMERA_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 20h-8a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v3.5" /> <path d="M9 13a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const CAMERA_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v3.5" /> <path d="M14.984 13.307a3 3 0 1 0 -2.32 2.62" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const CAMERA_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 20h-6a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v4" /> <path d="M9 13a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const CAMERA_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 20h-6a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v4" /> <path d="M14.948 13.559a3 3 0 1 0 -2.58 2.419" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const CAMERA_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v3" /> <path d="M14.973 13.406a3 3 0 1 0 -2.973 2.594" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const CAMERA_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 20h-8a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v1.5" /> <path d="M14.935 12.375a3.001 3.001 0 1 0 -1.902 3.442" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const CAMERA_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v3.5" /> <path d="M9 13a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const CAMERA_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 20h-10a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v3.5" /> <path d="M9 13a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const CAMERA_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.5 20h-5.5a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v2" /> <path d="M14.41 11.212a3 3 0 1 0 -4.15 4.231" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const CAMERA_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v6" /> <path d="M16 19h6" /> <path d="M9 13a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> </svg>"##;
const CAMERA_MOON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 20h-6.5a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v3.5" /> <path d="M14.815 11.96a3.001 3.001 0 1 0 -3.398 3.983" /> <path d="M18.62 22c-2 0 -3.62 -1.58 -3.62 -3.53c0 -1.727 1.273 -3.165 2.954 -3.47a3.4 3.4 0 0 0 -.24 1.264c0 1.95 1.621 3.53 3.62 3.53q .342 0 .666 -.06c-.521 1.326 -1.838 2.266 -3.38 2.266" /> </svg>"##;
const CAMERA_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.297 4.289a.997 .997 0 0 1 .703 -.289h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v8m-1.187 2.828c-.249 .11 -.524 .172 -.813 .172h-14a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1c.298 0 .58 -.065 .834 -.181" /> <path d="M10.422 10.448a3 3 0 1 0 4.15 4.098" /> <path d="M3 3l18 18" /> </svg>"##;
const CAMERA_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 20h-8a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v4" /> <path d="M14.958 13.506a3 3 0 1 0 -1.735 2.235" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const CAMERA_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 20h-7.5a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v2" /> <path d="M14.933 12.366a3.001 3.001 0 1 0 -2.933 3.634" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const CAMERA_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v3.5" /> <path d="M16 19h6" /> <path d="M19 16v6" /> <path d="M9 13a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> </svg>"##;
const CAMERA_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 20h-10a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v2.5" /> <path d="M14.975 12.612a3 3 0 1 0 -1.507 3.005" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const CAMERA_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 20h-6.5a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v2.5" /> <path d="M14.757 11.815a3 3 0 1 0 -3.431 4.109" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const CAMERA_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 20h-7.5a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v4" /> <path d="M14.98 13.347a3 3 0 1 0 -2.39 2.595" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const CAMERA_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 20h-6.5a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v3" /> <path d="M9 13a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const CAMERA_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.5 20h-5.5a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v2.5" /> <path d="M14.569 11.45a3 3 0 1 0 -4.518 3.83" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const CAMERA_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v3.5" /> <path d="M12 16a3 3 0 1 0 0 -6a3 3 0 0 0 0 6" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const CAMERA_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 20h-8.5a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v4" /> <path d="M9 13a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const CAPTURE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const CAPTURE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2c.554 0 1.055 -.225 1.417 -.589" /> <path d="M9.87 9.887a3 3 0 0 0 4.255 4.23m.58 -3.416a3.012 3.012 0 0 0 -1.4 -1.403" /> <path d="M4 8v-2c0 -.548 .22 -1.044 .577 -1.405" /> <path d="M3 3l18 18" /> </svg>"##;
const CAST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19l.01 0" /> <path d="M7 19a4 4 0 0 0 -4 -4" /> <path d="M11 19a8 8 0 0 0 -8 -8" /> <path d="M15 19h3a3 3 0 0 0 3 -3v-8a3 3 0 0 0 -3 -3h-12a3 3 0 0 0 -2.8 2" /> </svg>"##;
const CAST_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19h.01" /> <path d="M7 19a4 4 0 0 0 -4 -4" /> <path d="M11 19a8 8 0 0 0 -8 -8" /> <path d="M15 19h3a3 3 0 0 0 .875 -.13m2 -2a3 3 0 0 0 .128 -.868v-8a3 3 0 0 0 -3 -3h-9m-3.865 .136a3 3 0 0 0 -1.935 1.864" /> <path d="M3 3l18 18" /> </svg>"##;
const CHESS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a3 3 0 0 1 3 3c0 1.113 -.6 2.482 -1.5 3l1.5 7h-6l1.5 -7c-.9 -.518 -1.5 -1.887 -1.5 -3a3 3 0 0 1 3 -3" /> <path d="M8 9h8" /> <path d="M6.684 16.772a1 1 0 0 0 -.684 .949v1.279a1 1 0 0 0 1 1h10a1 1 0 0 0 1 -1v-1.28a1 1 0 0 0 -.684 -.948l-2.316 -.772h-6l-2.316 .772" /> </svg>"##;
const CHESS_BISHOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16l-1.447 .724a1 1 0 0 0 -.553 .894v2.382h12v-2.382a1 1 0 0 0 -.553 -.894l-1.447 -.724h-8" /> <path d="M11 4a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M9.5 16c-1.667 0 -2.5 -1.669 -2.5 -3c0 -3.667 1.667 -6 5 -7c3.333 1 5 3.427 5 7c0 1.284 -.775 2.881 -2.325 3l-.175 0h-5" /> <path d="M15 8l-3 3" /> <path d="M12 5v1" /> </svg>"##;
const CHESS_KING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16l-1.447 .724a1 1 0 0 0 -.553 .894v2.382h12v-2.382a1 1 0 0 0 -.553 -.894l-1.447 -.724h-8" /> <path d="M8.5 16a3.5 3.5 0 1 1 3.163 -5h.674a3.5 3.5 0 1 1 3.163 5l-7 0" /> <path d="M9 6h6" /> <path d="M12 3v8" /> </svg>"##;
const CHESS_KNIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16l-1.447 .724a1 1 0 0 0 -.553 .894v2.382h12v-2.382a1 1 0 0 0 -.553 -.894l-1.447 -.724h-8" /> <path d="M9 3l1 3l-3.491 2.148a1 1 0 0 0 .524 1.852h2.967l-2.073 6h7.961l.112 -5c0 -3 -1.09 -5.983 -4 -7c-1.94 -.678 -2.94 -1.011 -3 -1" /> </svg>"##;
const CHESS_QUEEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 16l2 -11l-4 4l-2 -5l-2 5l-4 -4l2 11" /> <path d="M8 16l-1.447 .724a1 1 0 0 0 -.553 .894v2.382h12v-2.382a1 1 0 0 0 -.553 -.894l-1.447 -.724h-8" /> <path d="M11 4a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M5 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M17 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const CHESS_ROOK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16l-1.447 .724a1 1 0 0 0 -.553 .894v2.382h12v-2.382a1 1 0 0 0 -.553 -.894l-1.447 -.724h-8" /> <path d="M8 16l1 -9h6l1 9" /> <path d="M6 4l.5 3h11l.5 -3" /> <path d="M10 4v3" /> <path d="M14 4v3" /> </svg>"##;
const CLEF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12a4.16 4.16 0 0 1 -5.62 3.89a3.78 3.78 0 0 1 -2.38 -3.39a3.42 3.42 0 0 1 2.34 -3.38l3.79 -1.42a2.89 2.89 0 0 0 1.87 -2.7a2 2 0 1 0 -4 0v14a2 2 0 1 1 -4 0" /> </svg>"##;
const CLEF_STAFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 15h6" /> <path d="M15 11h6" /> <path d="M11 19h10" /> <path d="M11 12a4.16 4.16 0 0 1 -5.62 3.89a3.78 3.78 0 0 1 -2.38 -3.39a3.42 3.42 0 0 1 2.34 -3.38l3.79 -1.42a2.89 2.89 0 0 0 1.87 -2.7a2 2 0 0 0 -2 -2a2 2 0 0 0 -2 2v14a2 2 0 0 1 -2 2a2 2 0 0 1 -2 -2" /> </svg>"##;
const CLIFF_JUMPING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.5 18l2.5 2l2 -2" /> <path d="M18 21l3 -3l-4 -2l-2 -5" /> <path d="M9 8l3 3l3 1l4 -2l3 -2" /> <path d="M3 21v-1l2 -3l.5 -2.5l1.5 -2.5l-1 -5l1 -3l-1 -1l-2 .5l-2 -.5" /> <path d="M13.007 8a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const CRICKET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.105 18.79l-1 .992a4.159 4.159 0 0 1 -6.038 -5.715l.157 -.166l8.282 -8.401l1.5 1.5l3.45 -3.391a2.08 2.08 0 0 1 3.057 2.815l-.116 .126l-3.391 3.45l1.5 1.5l-3.668 3.617" /> <path d="M10.5 7.5l6 6" /> <path d="M11 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const CURLING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 13a4 4 0 0 1 4 -4h8a4 4 0 0 1 4 4v2a4 4 0 0 1 -4 4h-8a4 4 0 0 1 -4 -4l0 -2" /> <path d="M4 14h16" /> <path d="M8 5h6a2 2 0 0 1 2 2v2" /> </svg>"##;
const DICE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M8 8.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M15 8.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M15 15.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M8 15.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> </svg>"##;
const DICE_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M11.5 12a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> </svg>"##;
const DICE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 9.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M14 14.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> </svg>"##;
const DICE_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M8 8.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M15 15.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M11.5 12a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> </svg>"##;
const DICE_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M8 8.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M15 8.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M15 15.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M8 15.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> </svg>"##;
const DICE_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M8 8.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M15 8.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M15 15.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M8 15.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M11.5 12a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> </svg>"##;
const DICE_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M8 7.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M15 7.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M8 12a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M15 12a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M15 16.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M8 16.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> </svg>"##;
const DISC_GOLF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5h14" /> <path d="M6 5c.32 6.744 2.74 9.246 6 10" /> <path d="M18 5c-.32 6.744 -2.74 9.246 -6 10" /> <path d="M10 5c0 4.915 .552 7.082 2 10" /> <path d="M14 5c0 4.915 -.552 7.082 -2 10" /> <path d="M12 15v6" /> <path d="M12 3v2" /> <path d="M7 16c.64 .64 1.509 1 2.414 1h5.172c.905 0 1.774 -.36 2.414 -1" /> <path d="M11 21h2" /> </svg>"##;
const DUMBBELL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.026 9.61l-.95 -4.18a2 2 0 0 1 1.95 -2.43h8a2 2 0 0 1 2 2.43l-1 4.2" /> <path d="M9.026 17.001h6" /> <path d="M18.906 20.06a7.92 7.92 0 0 0 1 -5.33a8 8 0 1 0 -14.77 5.33a2 2 0 0 0 1.71 .94h10.36a2 2 0 0 0 1.7 -.94" /> </svg>"##;
const EXERCISE_BALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.59 18.31a15.57 15.57 0 0 1 4.51 -9.21a15.9 15.9 0 0 1 7.43 -4.19" /> <path d="M11.55 21a9.34 9.34 0 0 1 2.79 -7.65a9.5 9.5 0 0 1 6.54 -2.85" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const GHOST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 11a7 7 0 0 1 14 0v7a1.78 1.78 0 0 1 -3.1 1.4a1.65 1.65 0 0 0 -2.6 0a1.65 1.65 0 0 1 -2.6 0a1.65 1.65 0 0 0 -2.6 0a1.78 1.78 0 0 1 -3.1 -1.4v-7" /> <path d="M10 10l.01 0" /> <path d="M14 10l.01 0" /> <path d="M10 14a3.5 3.5 0 0 0 4 0" /> </svg>"##;
const GHOST_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 9h.01" /> <path d="M14 9h.01" /> <path d="M12 3a7 7 0 0 1 7 7v1l1 0a2 2 0 1 1 0 4l-1 0v3l2 3h-10a6 6 0 0 1 -6 -5.775l0 -.226l-1 0a2 2 0 0 1 0 -4l1 0v-1a7 7 0 0 1 7 -7l0 .001" /> <path d="M11 14h2a1 1 0 0 0 -2 0" /> </svg>"##;
const GHOST_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 11a7 7 0 0 1 14 0v7a1.78 1.78 0 0 1 -3.1 1.4a1.65 1.65 0 0 0 -2.6 0a1.65 1.65 0 0 1 -2.6 0a1.65 1.65 0 0 0 -2.6 0a1.78 1.78 0 0 1 -3.1 -1.4v-7" /> <path d="M10 10h.01" /> <path d="M14 10h.01" /> </svg>"##;
const GHOST_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.794 4.776a7 7 0 0 1 10.206 6.224v4m-.12 3.898a1.779 1.779 0 0 1 -2.98 .502a1.65 1.65 0 0 0 -2.6 0a1.65 1.65 0 0 1 -2.6 0a1.65 1.65 0 0 0 -2.6 0a1.78 1.78 0 0 1 -3.1 -1.4v-7c0 -1.683 .594 -3.227 1.583 -4.434" /> <path d="M14 10h.01" /> <path d="M10 14a3.5 3.5 0 0 0 4 0" /> <path d="M3 3l18 18" /> </svg>"##;
const GO_GAME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 12h7m4 0h7" /> <path d="M3 6h1m4 0h13" /> <path d="M3 18h1m4 0h8m4 0h1" /> <path d="M6 3v1m0 4v8m0 4v1" /> <path d="M12 3v7m0 4v7" /> <path d="M18 3v13m0 4v1" /> </svg>"##;
const GOLF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18v-15l7 4l-7 4" /> <path d="M9 17.67c-.62 .36 -1 .82 -1 1.33c0 1.1 1.8 2 4 2s4 -.9 4 -2c0 -.5 -.38 -.97 -1 -1.33" /> </svg>"##;
const GOLF_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18v-6m0 -4v-5l7 4l-5.07 2.897" /> <path d="M9 17.67c-.62 .36 -1 .82 -1 1.33c0 1.1 1.8 2 4 2s4 -.9 4 -2c0 -.5 -.38 -.97 -1 -1.33" /> <path d="M3 3l18 18" /> </svg>"##;
const HEADPHONES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 15a2 2 0 0 1 2 -2h1a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-1a2 2 0 0 1 -2 -2l0 -3" /> <path d="M15 15a2 2 0 0 1 2 -2h1a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-1a2 2 0 0 1 -2 -2l0 -3" /> <path d="M4 15v-3a8 8 0 0 1 16 0v3" /> </svg>"##;
const HEADPHONES_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M4 15a2 2 0 0 1 2 -2h1a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-1a2 2 0 0 1 -2 -2l0 -3" /> <path d="M17 13h1a2 2 0 0 1 2 2v1m-.589 3.417c-.361 .36 -.86 .583 -1.411 .583h-1a2 2 0 0 1 -2 -2v-3" /> <path d="M4 15v-3c0 -2.21 .896 -4.21 2.344 -5.658m2.369 -1.638a8 8 0 0 1 11.287 7.296v3" /> </svg>"##;
const HEADSET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 14v-3a8 8 0 1 1 16 0v3" /> <path d="M18 19c0 1.657 -2.686 3 -6 3" /> <path d="M4 14a2 2 0 0 1 2 -2h1a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-1a2 2 0 0 1 -2 -2v-3" /> <path d="M15 14a2 2 0 0 1 2 -2h1a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-1a2 2 0 0 1 -2 -2v-3" /> </svg>"##;
const HEADSET_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 14v-3c0 -1.953 .7 -3.742 1.862 -5.13m2.182 -1.825a8 8 0 0 1 11.956 6.955v3" /> <path d="M18 19c0 1.657 -2.686 3 -6 3" /> <path d="M4 14a2 2 0 0 1 2 -2h1a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-1a2 2 0 0 1 -2 -2v-3" /> <path d="M16.169 12.18c.253 -.115 .534 -.18 .831 -.18h1a2 2 0 0 1 2 2v2m-1.183 2.826c-.25 .112 -.526 .174 -.817 .174h-1a2 2 0 0 1 -2 -2v-2" /> <path d="M3 3l18 18" /> </svg>"##;
const HELMET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4a9 9 0 0 1 5.656 16h-11.312a9 9 0 0 1 5.656 -16" /> <path d="M20 9h-8.8a1 1 0 0 0 -.968 1.246c.507 2 1.596 3.418 3.268 4.254c2 1 4.333 1.5 7 1.5" /> </svg>"##;
const HELMET_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.633 4.654a9 9 0 0 1 11.718 11.7m-1.503 2.486a9.008 9.008 0 0 1 -1.192 1.16h-11.312a9 9 0 0 1 -.185 -13.847" /> <path d="M20 9h-7m-2.768 1.246c.507 2 1.596 3.418 3.268 4.254c.524 .262 1.07 .49 1.64 .683" /> <path d="M3 3l18 18" /> </svg>"##;
const HORSE_TOY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.5 17.5c5.667 4.667 11.333 4.667 17 0" /> <path d="M19 18.5l-2 -8.5l1 -2l2 1l1.5 -1.5l-2.5 -4.5c-5.052 .218 -5.99 3.133 -7 6h-6a3 3 0 0 0 -3 3" /> <path d="M5 18.5l2 -9.5" /> <path d="M8 20l2 -5h4l2 5" /> </svg>"##;
const HULA_HOOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 6l2 1.5l6 .5l6 -.5l2 -1.5" /> <path d="M16 21l-4 -8v-5" /> <path d="M8 21l4 -8" /> <path d="M9.007 10.999c-2.37 .32 -4.007 1.201 -4.007 2.001c0 1.105 3.134 2 7 2s7 -.895 7 -2c0 -.798 -1.636 -1.679 -4 -2" /> </svg>"##;
const ICE_SKATING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.905 5h3.418a1 1 0 0 1 .928 .629l1.143 2.856a3 3 0 0 0 2.207 1.83l4.717 .926a2.084 2.084 0 0 1 1.682 2.045v.714a1 1 0 0 1 -1 1h-13.895a1 1 0 0 1 -1 -1.1l.8 -8a1 1 0 0 1 1 -.9" /> <path d="M3 19h17a1 1 0 0 0 1 -1" /> <path d="M9 15v4" /> <path d="M15 15v4" /> </svg>"##;
const JOKER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 17.5a1.5 1.5 0 0 1 1.5 -1.5h11a1.5 1.5 0 0 1 1.5 1.5a1.5 1.5 0 0 1 -1.5 1.5h-11a1.5 1.5 0 0 1 -1.5 -1.5" /> <path d="M12 16q -2.5 -8 -6 -8q -2.5 0 -3 2c2.953 .31 3.308 3.33 4 6" /> <path d="M12 16q 2.5 -8 6 -8q 2.5 0 3 2c-2.953 .31 -3.308 3.33 -4 6" /> <path d="M9 9.5q 2 -3.5 3 -3.5t 3 3.5" /> </svg>"##;
const JUMP_ROPE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 14v-6a3 3 0 1 1 6 0v8a3 3 0 0 0 6 0v-6" /> <path d="M16 5a2 2 0 0 1 2 -2a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2a2 2 0 0 1 -2 -2l0 -3" /> <path d="M4 16a2 2 0 0 1 2 -2a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2a2 2 0 0 1 -2 -2l0 -3" /> </svg>"##;
const KARATE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9l4.5 1l3 2.5" /> <path d="M13 21v-8l3 -5.5" /> <path d="M8 4.5l4 2l4 1l4 3.5l-2 3.5" /> <path d="M15.007 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const KAYAK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.414 6.414a2 2 0 0 0 0 -2.828l-1.414 -1.414l-2.828 2.828l1.414 1.414a2 2 0 0 0 2.828 0" /> <path d="M17.586 17.586a2 2 0 0 0 0 2.828l1.414 1.414l2.828 -2.828l-1.414 -1.414a2 2 0 0 0 -2.828 0" /> <path d="M6.5 6.5l11 11" /> <path d="M22 2.5c-9.983 2.601 -17.627 7.952 -20 19.5c9.983 -2.601 17.627 -7.952 20 -19.5" /> <path d="M6.5 12.5l5 5" /> <path d="M12.5 6.5l5 5" /> </svg>"##;
const KEYFRAME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.225 18.412a1.595 1.595 0 0 1 -1.225 .588c-.468 0 -.914 -.214 -1.225 -.588l-4.361 -5.248a1.844 1.844 0 0 1 0 -2.328l4.361 -5.248a1.595 1.595 0 0 1 1.225 -.588c.468 0 .914 .214 1.225 .588l4.361 5.248a1.844 1.844 0 0 1 0 2.328l-4.361 5.248" /> </svg>"##;
const KEYFRAME_ALIGN_CENTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20v2" /> <path d="M12.816 16.58c-.207 .267 -.504 .42 -.816 .42c-.312 0 -.61 -.153 -.816 -.42l-2.908 -3.748a1.39 1.39 0 0 1 0 -1.664l2.908 -3.748c.207 -.267 .504 -.42 .816 -.42c.312 0 .61 .153 .816 .42l2.908 3.748a1.39 1.39 0 0 1 0 1.664l-2.908 3.748" /> <path d="M12 2v2" /> <path d="M3 12h2" /> <path d="M19 12h2" /> </svg>"##;
const KEYFRAME_ALIGN_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.816 16.58c-.207 .267 -.504 .42 -.816 .42c-.312 0 -.61 -.153 -.816 -.42l-2.908 -3.748a1.39 1.39 0 0 1 0 -1.664l2.908 -3.748c.207 -.267 .504 -.42 .816 -.42c.312 0 .61 .153 .816 .42l2.908 3.748a1.39 1.39 0 0 1 0 1.664l-2.908 3.748" /> <path d="M3 12h2" /> <path d="M19 12h2" /> </svg>"##;
const KEYFRAME_ALIGN_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 2v2" /> <path d="M12.816 16.58c-.207 .267 -.504 .42 -.816 .42c-.312 0 -.61 -.153 -.816 -.42l-2.908 -3.748a1.39 1.39 0 0 1 0 -1.664l2.908 -3.748c.207 -.267 .504 -.42 .816 -.42c.312 0 .61 .153 .816 .42l2.908 3.748a1.39 1.39 0 0 1 0 1.664l-2.908 3.748" /> <path d="M12 20v2" /> </svg>"##;
const KEYFRAMES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.225 18.412a1.595 1.595 0 0 1 -1.225 .588c-.468 0 -.914 -.214 -1.225 -.588l-4.361 -5.248a1.844 1.844 0 0 1 0 -2.328l4.361 -5.248a1.595 1.595 0 0 1 1.225 -.588c.468 0 .914 .214 1.225 .588l4.361 5.248a1.844 1.844 0 0 1 0 2.328l-4.361 5.248" /> <path d="M17 5l4.586 5.836a1.844 1.844 0 0 1 0 2.328l-4.586 5.836" /> <path d="M13 5l4.586 5.836a1.844 1.844 0 0 1 0 2.328l-4.586 5.836" /> </svg>"##;
const MAXIMIZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const MAXIMIZE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2c0 -.551 .223 -1.05 .584 -1.412" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2c.545 0 1.04 -.218 1.4 -.572" /> <path d="M3 3l18 18" /> </svg>"##;
const MEDAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4v3m-4 -3v6m8 -6v6" /> <path d="M12 18.5l-3 1.5l.5 -3.5l-2 -2l3 -.5l1.5 -3l1.5 3l3 .5l-2 2l.5 3.5l-3 -1.5" /> </svg>"##;
const MEDAL_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 3h6l3 7l-6 2l-6 -2l3 -7" /> <path d="M12 12l-3 -9" /> <path d="M15 11l-3 -8" /> <path d="M12 19.5l-3 1.5l.5 -3.5l-2 -2l3 -.5l1.5 -3l1.5 3l3 .5l-2 2l.5 3.5l-3 -1.5" /> </svg>"##;
const MEEPLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 20h-5a1 1 0 0 1 -1 -1c0 -2 3.378 -4.907 4 -6c-1 0 -4 -.5 -4 -2c0 -2 4 -3.5 6 -4c0 -1.5 .5 -4 3 -4s3 2.5 3 4c2 .5 6 2 6 4c0 1.5 -3 2 -4 2c.622 1.093 4 4 4 6a1 1 0 0 1 -1 1h-5c-1 0 -2 -4 -3 -4s-2 4 -3 4" /> </svg>"##;
const MICROPHONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5a3 3 0 0 1 3 -3a3 3 0 0 1 3 3v5a3 3 0 0 1 -3 3a3 3 0 0 1 -3 -3l0 -5" /> <path d="M5 10a7 7 0 0 0 14 0" /> <path d="M8 21l8 0" /> <path d="M12 17l0 4" /> </svg>"##;
const MICROPHONE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 12.9a5 5 0 1 0 -3.902 -3.9" /> <path d="M15 12.9l-3.902 -3.899l-7.513 8.584a2 2 0 1 0 2.827 2.83l8.588 -7.515" /> </svg>"##;
const MICROPHONE_2_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.908 12.917a5 5 0 1 0 -5.827 -5.819" /> <path d="M10.116 10.125l-6.529 7.46a2 2 0 1 0 2.827 2.83l7.461 -6.529" /> <path d="M3 3l18 18" /> </svg>"##;
const MICROPHONE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M9 5a3 3 0 0 1 6 0v5a3 3 0 0 1 -.13 .874m-2 2a3 3 0 0 1 -3.87 -2.872v-1" /> <path d="M5 10a7 7 0 0 0 10.846 5.85m2 -2a6.967 6.967 0 0 0 1.152 -3.85" /> <path d="M8 21l8 0" /> <path d="M12 17l0 4" /> </svg>"##;
const MINIMIZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 19v-2a2 2 0 0 1 2 -2h2" /> <path d="M15 5v2a2 2 0 0 0 2 2h2" /> <path d="M5 15h2a2 2 0 0 1 2 2v2" /> <path d="M5 9h2a2 2 0 0 0 2 -2v-2" /> </svg>"##;
const MOVIE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M8 4l0 16" /> <path d="M16 4l0 16" /> <path d="M4 8l4 0" /> <path d="M4 16l4 0" /> <path d="M4 12l16 0" /> <path d="M16 8l4 0" /> <path d="M16 16l4 0" /> </svg>"##;
const MOVIE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h10a2 2 0 0 1 2 2v10m-.592 3.42c-.362 .359 -.859 .58 -1.408 .58h-12a2 2 0 0 1 -2 -2v-12c0 -.539 .213 -1.028 .56 -1.388" /> <path d="M8 8v12" /> <path d="M16 4v8m0 4v4" /> <path d="M4 8h4" /> <path d="M4 16h4" /> <path d="M4 12h8m4 0h4" /> <path d="M16 8h4" /> <path d="M3 3l18 18" /> </svg>"##;
const MUSIC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M13 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v13" /> <path d="M9 8h10" /> </svg>"##;
const MUSIC_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v8" /> <path d="M9 8h10" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const MUSIC_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v8" /> <path d="M9 8h10" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const MUSIC_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v9.5" /> <path d="M9 8h10" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const MUSIC_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v9" /> <path d="M9 8h10" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const MUSIC_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v7.5" /> <path d="M9 8h10" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const MUSIC_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v6" /> <path d="M9 8h10" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const MUSIC_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v8" /> <path d="M9 8h10" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const MUSIC_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v8" /> <path d="M9 8h10" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const MUSIC_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v7" /> <path d="M9 8h10" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const MUSIC_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v11" /> <path d="M9 8h10" /> <path d="M16 19h6" /> </svg>"##;
const MUSIC_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M14.42 14.45a3 3 0 1 0 4.138 4.119" /> <path d="M9 17v-8m0 -4v-1h10v11" /> <path d="M12 8h7" /> <path d="M3 3l18 18" /> </svg>"##;
const MUSIC_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v9" /> <path d="M9 8h10" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const MUSIC_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v7" /> <path d="M9 8h10" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const MUSIC_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v8" /> <path d="M9 8h10" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const MUSIC_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v7" /> <path d="M9 8h10" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const MUSIC_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v7" /> <path d="M9 8h10" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const MUSIC_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v9" /> <path d="M9 8h10" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const MUSIC_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v6" /> <path d="M9 8h10" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const MUSIC_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v8" /> <path d="M9 8h10" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const MUSIC_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M9 17v-13h10v9" /> <path d="M9 8h10" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const OLYMPIC_TORCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 21h-4c0 -4.364 -1 -7 -2 -11q 4 2 8 0c-1 4 -2 6.636 -2 11" /> <path d="M11 2c0 2.5 -1 2.66 -1 4a1.9 1.9 0 0 0 2 2a1.87 1.87 0 0 0 2 -2c0 -1.41 -1 -3 -3 -4" /> </svg>"##;
const OLYMPICS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M15 9a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M9 9a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M6 15a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M12 15a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const OLYMPICS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 6a3 3 0 1 0 3 3" /> <path d="M15 9a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M9 9a3 3 0 0 0 3 3m2.566 -1.445a3 3 0 0 0 -4.135 -4.113" /> <path d="M6 15a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M12.878 12.88a3 3 0 0 0 4.239 4.247m.586 -3.431a3.012 3.012 0 0 0 -1.43 -1.414" /> <path d="M3 3l18 18" /> </svg>"##;
const PACMAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.636 5.636a9 9 0 0 1 13.397 .747l-5.619 5.617l5.619 5.617a9 9 0 1 1 -13.397 -11.981" /> <path d="M11.5 7.5a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> </svg>"##;
const PHOTO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-12" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l5 5" /> <path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0l3 3" /> </svg>"##;
const PHOTO_AI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M10 21h-4a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l1 1" /> <path d="M14 21v-4a2 2 0 1 1 4 0v4" /> <path d="M14 19h4" /> <path d="M21 15v6" /> </svg>"##;
const PHOTO_ALT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 18h5" /> <path d="M14 18h4" /> <path d="M15 7h.01" /> <path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-12" /> <path d="M3 15l5 -5c.928 -.893 2.072 -.893 3 0l5 5" /> <path d="M14 13l1 -1c.928 -.893 2.072 -.893 3 0l3 3" /> <path d="M3 15h18" /> </svg>"##;
const PHOTO_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M13.5 21h-7.5a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6.5" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l4 4" /> <path d="M14 14l1 -1c.669 -.643 1.45 -.823 2.18 -.54" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const PHOTO_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M12.5 21h-6.5a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6.5" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l3 3" /> <path d="M14 14l1 -1c.616 -.593 1.328 -.792 2.008 -.598" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const PHOTO_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M11.5 21h-5.5a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v7" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l4 4" /> <path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0l.5 .5" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const PHOTO_CIRCLE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M20.475 15.035a9 9 0 0 0 -8.475 -12.035a9 9 0 0 0 -9 9a9 9 0 0 0 9.525 8.985" /> <path d="M4 15l4 -4c.928 -.893 2.072 -.893 3 0l4 4" /> <path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0l2 2" /> <path d="M16 19h6" /> </svg>"##;
const PHOTO_CIRCLE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M20.964 12.806a9 9 0 0 0 -8.964 -9.806a9 9 0 0 0 -9 9a9 9 0 0 0 9.397 8.991" /> <path d="M4 15l4 -4c.928 -.893 2.072 -.893 3 0l4 4" /> <path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0" /> <path d="M16 19.33h6" /> <path d="M19 16.33v6" /> </svg>"##;
const PHOTO_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M11.5 21h-5.5a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v7" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l3 3" /> <path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const PHOTO_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M12 21h-6a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l3 3" /> <path d="M14 14l1 -1c.48 -.461 1.016 -.684 1.551 -.67" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const PHOTO_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M13 21h-7a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4.5" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l2.5 2.5" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const PHOTO_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M12.5 21h-6.5a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6.5" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l4 4" /> <path d="M14 14l1 -1c.653 -.629 1.413 -.815 2.13 -.559" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const PHOTO_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M11 20h-4a3 3 0 0 1 -3 -3v-10a3 3 0 0 1 3 -3h10a3 3 0 0 1 3 3v4" /> <path d="M4 15l4 -4c.928 -.893 2.072 -.893 3 0l3 3" /> <path d="M14 14l1 -1c.31 -.298 .644 -.497 .987 -.596" /> <path d="M18.42 15.61a2.1 2.1 0 0 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const PHOTO_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M15 21h-9a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l4 4" /> <path d="M14 14l1 -1c.665 -.64 1.44 -.821 2.167 -.545" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const PHOTO_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M11.5 21h-5.5a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l1.5 1.5" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const PHOTO_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M12.5 21h-6.5a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v9" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l4 4" /> <path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0l2 2" /> <path d="M16 19h6" /> </svg>"##;
const PHOTO_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M7 3h11a3 3 0 0 1 3 3v11m-.856 3.099a2.991 2.991 0 0 1 -2.144 .901h-12a3 3 0 0 1 -3 -3v-12c0 -.845 .349 -1.608 .91 -2.153" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l5 5" /> <path d="M16.33 12.338c.574 -.054 1.155 .166 1.67 .662l3 3" /> <path d="M3 3l18 18" /> </svg>"##;
const PHOTO_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M13 21h-7a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v7" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l3 3" /> <path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const PHOTO_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M12.5 21h-6.5a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l2.5 2.5" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const PHOTO_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M12.5 21h-6.5a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6.5" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l4 4" /> <path d="M14 14l1 -1c.67 -.644 1.45 -.824 2.182 -.54" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const PHOTO_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M15 21h-9a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l3 3" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const PHOTO_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M11.5 21h-5.5a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l2 2" /> </svg>"##;
const PHOTO_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M12 21h-6a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v7" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l3 3" /> <path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const PHOTO_SHIELD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M11.5 20h-4.5a3 3 0 0 1 -3 -3v-10a3 3 0 0 1 3 -3h10a3 3 0 0 1 3 3v4" /> <path d="M4 15l4 -4c.928 -.893 2.072 -.893 3 0l1.5 1.5" /> <path d="M22 16c0 4 -2.5 6 -3.5 6s-3.5 -2 -3.5 -6c1 0 2.5 -.5 3.5 -1.5c1 1 2.5 1.5 3.5 1.5" /> </svg>"##;
const PHOTO_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M12 21h-6a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l3.993 3.993" /> <path d="M14 14l1 -1c.47 -.452 .995 -.675 1.52 -.67" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const PHOTO_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M11 21h-5a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l2 2" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const PHOTO_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M12.5 21h-6.5a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v6.5" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l3.5 3.5" /> <path d="M14 14l1 -1c.679 -.653 1.473 -.829 2.214 -.526" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const PHOTO_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M13 21h-7a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v7" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l3 3" /> <path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const PIANO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M9 19v-6" /> <path d="M8 5v8h2v-8" /> <path d="M15 19v-6" /> <path d="M14 5v8h2v-8" /> </svg>"##;
const PICK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 8l-9.383 9.418a2.091 2.091 0 0 0 0 2.967a2.11 2.11 0 0 0 2.976 0l9.407 -9.385" /> <path d="M9 3h4.586a1 1 0 0 1 .707 .293l6.414 6.414a1 1 0 0 1 .293 .707v4.586a2 2 0 1 1 -4 0v-3l-5 -5h-3a2 2 0 1 1 0 -4" /> </svg>"##;
const PICTURE_IN_PICTURE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 19h-6a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4" /> <path d="M14 15a1 1 0 0 1 1 -1h5a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-5a1 1 0 0 1 -1 -1l0 -3" /> </svg>"##;
const PICTURE_IN_PICTURE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 19h-6a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4" /> <path d="M14 15a1 1 0 0 1 1 -1h5a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-5a1 1 0 0 1 -1 -1l0 -3" /> <path d="M7 9l4 4" /> <path d="M7 12v-3h3" /> </svg>"##;
const PICTURE_IN_PICTURE_ON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 19h-6a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4" /> <path d="M14 15a1 1 0 0 1 1 -1h5a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-5a1 1 0 0 1 -1 -1l0 -3" /> <path d="M7 9l4 4" /> <path d="M8 13h3v-3" /> </svg>"##;
const PICTURE_IN_PICTURE_TOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 5h-6a2 2 0 0 0 -2 2v10a2 2 0 0 0 2 2h14a2 2 0 0 0 2 -2v-4" /> <path d="M15 10h5a1 1 0 0 0 1 -1v-3a1 1 0 0 0 -1 -1h-5a1 1 0 0 0 -1 1v3a1 1 0 0 0 1 1" /> </svg>"##;
const PING_PONG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.718 20.713a7.64 7.64 0 0 1 -7.48 -12.755l.72 -.72a7.643 7.643 0 0 1 9.105 -1.283l2.387 -2.345a2.08 2.08 0 0 1 3.057 2.815l-.116 .126l-2.346 2.387a7.644 7.644 0 0 1 -1.052 8.864" /> <path d="M11 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M9.3 5.3l9.4 9.4" /> </svg>"##;
const PLAY_BASKETBALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.007 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 21l3 -3l.75 -1.5" /> <path d="M14 21v-4l-4 -3l.5 -6" /> <path d="M5 12l1 -3l4.5 -1l3.5 3l4 -.5" /> <path d="M18.007 15.5a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> </svg>"##;
const PLAY_CARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M12 16l-3 -4l3 -4l3 4l-3 4" /> </svg>"##;
const PLAY_CARD_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M12 9v6" /> </svg>"##;
const PLAY_CARD_10_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M9 9v6" /> <path d="M12 13c0 1.105 .672 2 1.5 2s1.5 -.895 1.5 -2v-2c0 -1.105 -.672 -2 -1.5 -2s-1.5 .895 -1.5 2l0 2" /> </svg>"##;
const PLAY_CARD_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M10 9h3a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h3" /> </svg>"##;
const PLAY_CARD_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M10 9h2.5a1.5 1.5 0 0 1 0 3h-1.5h1.5a1.5 1.5 0 0 1 0 3h-2.5" /> </svg>"##;
const PLAY_CARD_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M10 9v2a1 1 0 0 0 1 1h3" /> <path d="M14 9v6" /> </svg>"##;
const PLAY_CARD_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M10 15h3a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-3v-3h4" /> </svg>"##;
const PLAY_CARD_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M14 9h-3a1 1 0 0 0 -1 1v4a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const PLAY_CARD_7_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M10 9h4l-2 6" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> </svg>"##;
const PLAY_CARD_8_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M12 12h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1l-1 0" /> <path d="M12 12h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1" /> </svg>"##;
const PLAY_CARD_9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M10 15h3a1 1 0 0 0 1 -1v-4a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h3" /> </svg>"##;
const PLAY_CARD_A_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M10 15v-4a2 2 0 1 1 4 0v4" /> <path d="M10 13h4" /> </svg>"##;
const PLAY_CARD_J_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M10 9h4v4a2 2 0 1 1 -4 0" /> </svg>"##;
const PLAY_CARD_K_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M10 9v6" /> <path d="M14 9l-3 3l3 3" /> <path d="M10 12h1" /> </svg>"##;
const PLAY_CARD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h10a2 2 0 0 1 2 2v10m0 4a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14" /> <path d="M16 18h.01" /> <path d="M13.716 13.712l-1.716 2.288l-3 -4l1.29 -1.72" /> <path d="M3 3l18 18" /> </svg>"##;
const PLAY_CARD_Q_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M12 9a2 2 0 0 1 2 2v2a2 2 0 1 1 -4 0v-2a2 2 0 0 1 2 -2" /> <path d="M13 14l1.5 1.5" /> </svg>"##;
const PLAY_CARD_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2" /> <path d="M8 6h.01" /> <path d="M16 18h.01" /> <path d="M11.75 14.112l-1.63 .853a.294 .294 0 0 1 -.425 -.307l.31 -1.808l-1.317 -1.28a.292 .292 0 0 1 .163 -.499l1.82 -.264l.815 -1.644a.294 .294 0 0 1 .527 0l.814 1.644l1.82 .264a.292 .292 0 0 1 .164 .499l-1.318 1.28l.31 1.807a.292 .292 0 0 1 -.425 .308l-1.628 -.853" /> </svg>"##;
const PLAY_FOOTBALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17l5 1l.75 -1.5" /> <path d="M14 21v-4l-4 -3l1 -6" /> <path d="M6 12v-3l5 -1l3 3l3 1" /> <path d="M18.007 19.5a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> <path d="M10.007 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const PLAY_HANDBALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21l3.5 -2l-4.5 -4l2 -4.5" /> <path d="M5 7l4 3l5 .5l4 2.5l2.5 3" /> <path d="M4 20l5 -1l1.5 -2" /> <path d="M13.007 8a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6.007 3.5a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> </svg>"##;
const PLAY_VOLLEYBALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.007 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.007 9.5a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> <path d="M2 16l5 1l.5 -2.5" /> <path d="M11.5 21l2.5 -5.5l-5.5 -3.5l3.5 -4l3 4l4 2" /> </svg>"##;
const PLAYER_EJECT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14l-7 -8l-7 8" /> <path d="M5 17a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1l0 -2" /> </svg>"##;
const PLAYER_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 6a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v12a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -12" /> <path d="M14 6a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v12a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -12" /> </svg>"##;
const PLAYER_PLAY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 4v16l13 -8l-13 -8" /> </svg>"##;
const PLAYER_RECORD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> </svg>"##;
const PLAYER_SKIP_BACK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 5v14l-12 -7l12 -7" /> <path d="M4 5l0 14" /> </svg>"##;
const PLAYER_SKIP_FORWARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5v14l12 -7l-12 -7" /> <path d="M20 5l0 14" /> </svg>"##;
const PLAYER_STOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -10" /> </svg>"##;
const PLAYER_TRACK_NEXT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5v14l8 -7l-8 -7" /> <path d="M14 5v14l8 -7l-8 -7" /> </svg>"##;
const PLAYER_TRACK_PREV_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 5v14l-8 -7l8 -7" /> <path d="M10 5v14l-8 -7l8 -7" /> </svg>"##;
const PLAYLIST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 17v-13h4" /> <path d="M13 5h-10" /> <path d="M3 9l10 0" /> <path d="M9 13h-6" /> </svg>"##;
const PLAYLIST_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 14a3 3 0 1 0 3 3" /> <path d="M17 13v-9h4" /> <path d="M13 5h-4m-4 0h-2" /> <path d="M3 9h6" /> <path d="M9 13h-6" /> <path d="M3 3l18 18" /> </svg>"##;
const PLAYLIST_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 8h-14" /> <path d="M5 12h7" /> <path d="M12 16h-7" /> <path d="M16 14l4 4" /> <path d="M20 14l-4 4" /> </svg>"##;
const POKER_CHIP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M7 12a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M12 3v4" /> <path d="M12 17v4" /> <path d="M3 12h4" /> <path d="M17 12h4" /> <path d="M18.364 5.636l-2.828 2.828" /> <path d="M8.464 15.536l-2.828 2.828" /> <path d="M5.636 5.636l2.828 2.828" /> <path d="M15.536 15.536l2.828 2.828" /> </svg>"##;
const POOL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 20a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1" /> <path d="M2 16a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1" /> <path d="M15 12v-7.5a1.5 1.5 0 0 1 3 0" /> <path d="M9 12v-7.5a1.5 1.5 0 0 0 -3 0" /> <path d="M15 5l-6 0" /> <path d="M9 10l6 0" /> </svg>"##;
const POOL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 20a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1c.303 0 .6 -.045 .876 -.146" /> <path d="M2 16a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 1.13 -.856m5.727 1.717a2.4 2.4 0 0 0 1.143 -.861" /> <path d="M15 11v-6.5a1.5 1.5 0 0 1 3 0" /> <path d="M9 12v-3m0 -4v-.5a1.5 1.5 0 0 0 -1.936 -1.436" /> <path d="M15 5h-6" /> <path d="M9 10h1m4 0h1" /> <path d="M3 3l18 18" /> </svg>"##;
const PUMPKIN_SCARY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l1.5 1l1.5 -1l1.5 1l1.5 -1" /> <path d="M10 11h.01" /> <path d="M14 11h.01" /> <path d="M17 6.082c2.609 .588 3.627 4.162 2.723 7.983c-.903 3.82 -2.75 6.44 -5.359 5.853a3.355 3.355 0 0 1 -.774 -.279a3.728 3.728 0 0 1 -1.59 .361c-.556 0 -1.09 -.127 -1.59 -.362a3.296 3.296 0 0 1 -.774 .28c-2.609 .588 -4.456 -2.033 -5.36 -5.853c-.903 -3.82 .115 -7.395 2.724 -7.983c1.085 -.244 1.575 .066 2.585 .787c.716 -.554 1.54 -.869 2.415 -.869c.876 0 1.699 .315 2.415 .87c1.01 -.722 1.5 -1.032 2.585 -.788" /> <path d="M12 6c0 -1.226 .693 -2.346 1.789 -2.894l.211 -.106" /> </svg>"##;
const PUZZLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7h3a1 1 0 0 0 1 -1v-1a2 2 0 0 1 4 0v1a1 1 0 0 0 1 1h3a1 1 0 0 1 1 1v3a1 1 0 0 0 1 1h1a2 2 0 0 1 0 4h-1a1 1 0 0 0 -1 1v3a1 1 0 0 1 -1 1h-3a1 1 0 0 1 -1 -1v-1a2 2 0 0 0 -4 0v1a1 1 0 0 1 -1 1h-3a1 1 0 0 1 -1 -1v-3a1 1 0 0 1 1 -1h1a2 2 0 0 0 0 -4h-1a1 1 0 0 1 -1 -1v-3a1 1 0 0 1 1 -1" /> </svg>"##;
const PUZZLE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M12 4v2.5a.5 .5 0 0 1 -.5 .5a1.5 1.5 0 0 0 0 3a.5 .5 0 0 1 .5 .5v1.5" /> <path d="M12 12v1.5a.5 .5 0 0 0 .5 .5a1.5 1.5 0 0 1 0 3a.5 .5 0 0 0 -.5 .5v2.5" /> <path d="M20 12h-2.5a.5 .5 0 0 1 -.5 -.5a1.5 1.5 0 0 0 -3 0a.5 .5 0 0 1 -.5 .5h-1.5" /> <path d="M12 12h-1.5a.5 .5 0 0 0 -.5 .5a1.5 1.5 0 0 1 -3 0a.5 .5 0 0 0 -.5 -.5h-2.5" /> </svg>"##;
const PUZZLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.18 4.171a2 2 0 0 1 3.82 .829v1a1 1 0 0 0 1 1h3a1 1 0 0 1 1 1v3a1 1 0 0 0 1 1h1a2 2 0 0 1 .819 3.825m-2.819 1.175v3a1 1 0 0 1 -1 1h-3a1 1 0 0 1 -1 -1v-1a2 2 0 1 0 -4 0v1a1 1 0 0 1 -1 1h-3a1 1 0 0 1 -1 -1v-3a1 1 0 0 1 1 -1h1a2 2 0 1 0 0 -4h-1a1 1 0 0 1 -1 -1v-3a1 1 0 0 1 1 -1h3" /> <path d="M3 3l18 18" /> </svg>"##;
const RADIO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3l-9.371 3.749a1 1 0 0 0 -.629 .928v11.323a1 1 0 0 0 1 1h14a1 1 0 0 0 1 -1v-11a1 1 0 0 0 -1 -1h-14.5" /> <path d="M4 12h16" /> <path d="M7 12v-2" /> <path d="M17 16v.01" /> <path d="M13 16v.01" /> </svg>"##;
const RADIO_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3l-4.986 2m-2.875 1.15l-1.51 .604a1 1 0 0 0 -.629 .928v11.323a1 1 0 0 0 1 1h14a1 1 0 0 0 .708 -.294m.292 -3.706v-8a1 1 0 0 0 -1 -1h-8m-4 0h-2.5" /> <path d="M4 12h8m4 0h4" /> <path d="M7 12v-2" /> <path d="M13 16v.01" /> <path d="M3 3l18 18" /> </svg>"##;
const REPEAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12v-3a3 3 0 0 1 3 -3h13m-3 -3l3 3l-3 3" /> <path d="M20 12v3a3 3 0 0 1 -3 3h-13m3 3l-3 -3l3 -3" /> </svg>"##;
const REPEAT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12v-3c0 -1.336 .873 -2.468 2.08 -2.856m3.92 -.144h10m-3 -3l3 3l-3 3" /> <path d="M20 12v3a3 3 0 0 1 -.133 .886m-1.99 1.984a3 3 0 0 1 -.877 .13h-13m3 3l-3 -3l3 -3" /> <path d="M3 3l18 18" /> </svg>"##;
const REPEAT_ONCE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12v-3a3 3 0 0 1 3 -3h13m-3 -3l3 3l-3 3" /> <path d="M20 12v3a3 3 0 0 1 -3 3h-13m3 3l-3 -3l3 -3" /> <path d="M11 11l1 -1v4" /> </svg>"##;
const REWIND_BACKWARD_10_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9l-3 -3l3 -3" /> <path d="M15.997 17.918a6.002 6.002 0 0 0 -.997 -11.918h-11" /> <path d="M6 14v6" /> <path d="M9 15.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> </svg>"##;
const REWIND_BACKWARD_15_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 20h2a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2v-3h3" /> <path d="M15 18a6 6 0 1 0 0 -12h-11" /> <path d="M5 14v6" /> <path d="M7 9l-3 -3l3 -3" /> </svg>"##;
const REWIND_BACKWARD_20_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.007 16.466a6 6 0 0 0 -4.007 -10.466h-11" /> <path d="M7 9l-3 -3l3 -3" /> <path d="M12 15.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M6 14h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2" /> </svg>"##;
const REWIND_BACKWARD_30_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.007 16.466a6 6 0 0 0 -4.007 -10.466h-11" /> <path d="M12 15.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M6 14h1.5a1.5 1.5 0 0 1 0 3h-.5h.5a1.5 1.5 0 0 1 0 3h-1.5" /> <path d="M7 9l-3 -3l3 -3" /> </svg>"##;
const REWIND_BACKWARD_40_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.007 16.466a6 6 0 0 0 -4.007 -10.466h-11" /> <path d="M12 15.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M6 14v2a1 1 0 0 0 1 1h1" /> <path d="M9 14v6" /> <path d="M7 9l-3 -3l3 -3" /> </svg>"##;
const REWIND_BACKWARD_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 18a6 6 0 1 0 0 -12h-11" /> <path d="M7 9l-3 -3l3 -3" /> <path d="M8 20h2a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2v-3h3" /> </svg>"##;
const REWIND_BACKWARD_50_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.007 16.466a6 6 0 0 0 -4.007 -10.466h-11" /> <path d="M12 15.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M6 20h1.5a1.5 1.5 0 0 0 0 -3h-1.5v-3h3" /> <path d="M7 9l-3 -3l3 -3" /> </svg>"##;
const REWIND_BACKWARD_60_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.007 16.466a6 6 0 0 0 -4.007 -10.466h-11" /> <path d="M7 9l-3 -3l3 -3" /> <path d="M12 15.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M9 14h-2a1 1 0 0 0 -1 1v4a1 1 0 0 0 1 1h1a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2" /> </svg>"##;
const REWIND_FORWARD_10_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 9l3 -3l-3 -3" /> <path d="M8 17.918a5.997 5.997 0 0 1 -5 -5.918a6 6 0 0 1 6 -6h11" /> <path d="M12 14v6" /> <path d="M15 15.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> </svg>"##;
const REWIND_FORWARD_15_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 9l3 -3l-3 -3" /> <path d="M9 18a6 6 0 1 1 0 -12h11" /> <path d="M16 20h2a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2v-3h3" /> <path d="M13 14v6" /> </svg>"##;
const REWIND_FORWARD_20_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.007 16.478a6 6 0 0 1 3.993 -10.478h11" /> <path d="M15 15.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M17 9l3 -3l-3 -3" /> <path d="M9 14h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2" /> </svg>"##;
const REWIND_FORWARD_30_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.007 16.478a6 6 0 0 1 3.993 -10.478h11" /> <path d="M15 15.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M17 9l3 -3l-3 -3" /> <path d="M9 14h1.5a1.5 1.5 0 0 1 0 3h-.5h.5a1.5 1.5 0 0 1 0 3h-1.5" /> </svg>"##;
const REWIND_FORWARD_40_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.007 16.478a6 6 0 0 1 3.993 -10.478h11" /> <path d="M15 15.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M17 9l3 -3l-3 -3" /> <path d="M9 14v2a1 1 0 0 0 1 1h1" /> <path d="M12 14v6" /> </svg>"##;
const REWIND_FORWARD_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 18a6 6 0 1 1 0 -12h11" /> <path d="M13 20h2a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2v-3h3" /> <path d="M17 9l3 -3l-3 -3" /> </svg>"##;
const REWIND_FORWARD_50_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.007 16.478a6 6 0 0 1 3.993 -10.478h11" /> <path d="M15 15.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M17 9l3 -3l-3 -3" /> <path d="M9 20h1.5a1.5 1.5 0 0 0 0 -3h-1.5v-3h3" /> </svg>"##;
const REWIND_FORWARD_60_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.007 16.478a6 6 0 0 1 3.993 -10.478h11" /> <path d="M15 15.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M17 9l3 -3l-3 -3" /> <path d="M12 14h-2a1 1 0 0 0 -1 1v4a1 1 0 0 0 1 1h1a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2" /> </svg>"##;
const RINGS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M14 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M7 15v-11" /> <path d="M17 15v-11" /> <path d="M3 4h18" /> </svg>"##;
const ROBOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 6a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v4a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -4" /> <path d="M12 2v2" /> <path d="M9 12v9" /> <path d="M15 12v9" /> <path d="M5 16l4 -2" /> <path d="M15 14l4 2" /> <path d="M9 18h6" /> <path d="M10 8v.01" /> <path d="M14 8v.01" /> </svg>"##;
const ROLLER_SKATING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.905 5h3.418a1 1 0 0 1 .928 .629l1.143 2.856a3 3 0 0 0 2.207 1.83l4.717 .926a2.084 2.084 0 0 1 1.682 2.045v.714a1 1 0 0 1 -1 1h-13.895a1 1 0 0 1 -1 -1.1l.8 -8a1 1 0 0 1 1 -.9" /> <path d="M6 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M14 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const ROULETTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.586 10.586l-1.586 -1.586" /> <path d="M13.414 10.586l1.586 -1.586" /> <path d="M13.414 13.414l1.586 1.586" /> <path d="M10.586 13.414l-1.586 1.586" /> <path d="M14 12a2 2 0 1 1 -4 0a2 2 0 0 1 4 0" /> <path d="M16.5 4.206l-.5 .866" /> <path d="M7.5 19.794l.5 -.866" /> <path d="M19.794 7.5l-.866 .5" /> <path d="M4.206 16.5l.866 -.5" /> <path d="M7.5 4.206l.5 .866" /> <path d="M16.5 19.794l-.5 -.866" /> <path d="M4.206 7.5l.866 .5" /> <path d="M19.794 16.5l-.866 -.5" /> <path d="M12 3v1" /> <path d="M12 21v-1" /> <path d="M21 12h-1" /> <path d="M3 12h1" /> <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> </svg>"##;
const RUGBY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 15h-4v6h4v-6" /> <path d="M12 15v-4" /> <path d="M8 21h8" /> <path d="M19 3v8h-14v-8" /> </svg>"##;
const RUN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.007 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 17l5 1l.75 -1.5" /> <path d="M15 21v-4l-4 -3l1 -6" /> <path d="M7 12v-3l5 -1l3 3l3 1" /> </svg>"##;
const SCOREBOARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M12 5v2" /> <path d="M12 10v1" /> <path d="M12 14v1" /> <path d="M12 18v1" /> <path d="M7 3v2" /> <path d="M17 3v2" /> <path d="M15 10.5v3a1.5 1.5 0 0 0 3 0v-3a1.5 1.5 0 0 0 -3 0" /> <path d="M6 9h1.5a1.5 1.5 0 0 1 0 3h-.5h.5a1.5 1.5 0 0 1 0 3h-1.5" /> </svg>"##;
const SCUBA_DIVING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 12a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M2 2l3 3l1.5 4l3.5 2l6 2l1 4l2.5 3" /> <path d="M11 8l4.5 1.5" /> </svg>"##;
const SCUBA_DIVING_TANK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 11a4 4 0 1 1 8 0v5h-8l0 -5" /> <path d="M8 16v3a2 2 0 0 0 2 2h4a2 2 0 0 0 2 -2v-3" /> <path d="M9 4h6" /> <path d="M12 7v-3" /> <path d="M7 4a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11.5 4a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> </svg>"##;
const SCUBA_MASK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7h12a1 1 0 0 1 1 1v4.5a2.5 2.5 0 0 1 -2.5 2.5h-.5a2 2 0 0 1 -2 -2a2 2 0 1 0 -4 0a2 2 0 0 1 -2 2h-.5a2.5 2.5 0 0 1 -2.5 -2.5v-4.5a1 1 0 0 1 1 -1" /> <path d="M10 17a2 2 0 0 0 2 2h3.5a5.5 5.5 0 0 0 5.5 -5.5v-9.5" /> </svg>"##;
const SCUBA_MASK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 7h5a1 1 0 0 1 1 1v4.5c0 .154 -.014 .304 -.04 .45m-2 2.007c-.15 .028 -.305 .043 -.463 .043h-.5a2 2 0 0 1 -2 -2a2 2 0 1 0 -4 0a2 2 0 0 1 -2 2h-.5a2.5 2.5 0 0 1 -2.5 -2.5v-4.5a1 1 0 0 1 1 -1h3" /> <path d="M10 17a2 2 0 0 0 2 2h3.5a5.475 5.475 0 0 0 2.765 -.744m2 -2c.47 -.81 .739 -1.752 .739 -2.756v-9.5" /> <path d="M3 3l18 18" /> </svg>"##;
const SHAREPLAY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 18a3 3 0 0 0 3 -3v-8a3 3 0 0 0 -3 -3h-12a3 3 0 0 0 -3 3v8a3 3 0 0 0 3 3" /> <path d="M9 20h6l-3 -5l-3 5" /> </svg>"##;
const SKATEBOARDING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.5 15h3.5l.75 -1.5" /> <path d="M14 19v-5l-2.5 -3l2.5 -4" /> <path d="M8 8l3 -1h4l1 3h3" /> <path d="M17.5 21a.5 .5 0 1 0 0 -1a.5 .5 0 0 0 0 1" /> <path d="M3 18c0 .552 .895 1 2 1h14c1.105 0 2 -.448 2 -1" /> <path d="M6.5 21a.5 .5 0 1 0 0 -1a.5 .5 0 0 0 0 1" /> <path d="M14.007 4a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const SKI_JUMPING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 17.5l-5 -4.5v-6l5 4" /> <path d="M7 17.5l5 -4.5" /> <path d="M15.103 21.58l6.762 -14.502a2 2 0 0 0 -.968 -2.657" /> <path d="M8.897 21.58l-6.762 -14.503a2 2 0 0 1 .968 -2.657" /> <path d="M7 11l5 -4" /> <path d="M10.007 4a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const SOCCER_FIELD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M3 9h3v6h-3l0 -6" /> <path d="M18 9h3v6h-3l0 -6" /> <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M12 5l0 14" /> </svg>"##;
const SPEAKERPHONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 8a3 3 0 0 1 0 6" /> <path d="M10 8v11a1 1 0 0 1 -1 1h-1a1 1 0 0 1 -1 -1v-5" /> <path d="M12 8l4.524 -3.77a.9 .9 0 0 1 1.476 .692v12.156a.9 .9 0 0 1 -1.476 .692l-4.524 -3.77h-8a1 1 0 0 1 -1 -1v-4a1 1 0 0 1 1 -1h8" /> </svg>"##;
const SPORT_BILLARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 10a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 12a8 8 0 1 0 16 0a8 8 0 1 0 -16 0" /> </svg>"##;
const STRETCHING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M5 20l5 -.5l1 -2" /> <path d="M18 20v-5h-5.5l2.5 -6.5l-5.5 1l1.5 2" /> </svg>"##;
const STRETCHING_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.5 21l3.5 -5" /> <path d="M5 11l7 -2" /> <path d="M16 21l-4 -7v-5l7 -4" /> <path d="M9.007 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const SUBTITLES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 5a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3l12 0" /> <path d="M7 15h5" /> <path d="M15 15h2" /> <path d="M17 12h-3" /> <path d="M11 12h-1" /> </svg>"##;
const SUBTITLES_AI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 19h-5.5a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4" /> <path d="M7 15h5" /> <path d="M17 12h-3" /> <path d="M11 12h-1" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const SUBTITLES_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 19h-5.5a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v3" /> <path d="M7 15h5" /> <path d="M17 12h-3" /> <path d="M11 12h-1" /> <path d="M18.42 15.61a2.1 2.1 0 0 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const SUBTITLES_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h9a3 3 0 0 1 3 3v8a3 3 0 0 1 -.13 .874m-2.006 2a3 3 0 0 1 -.864 .126h-12a3 3 0 0 1 -3 -3v-8c0 -1.35 .893 -2.493 2.12 -2.869" /> <path d="M7 15h5" /> <path d="M17 12h-1" /> <path d="M12 12h-2" /> <path d="M3 3l18 18" /> </svg>"##;
const SWIMMING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 9a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M6 11l4 -2l3.5 3l-1.5 2" /> <path d="M3 16.75a2.4 2.4 0 0 0 1 .25a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 1 -.25" /> </svg>"##;
const SWORD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 4v5l-9 7l-4 4l-3 -3l4 -4l7 -9l5 0" /> <path d="M6.5 11.5l6 6" /> </svg>"##;
const SWORD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.938 7.937l3.062 -3.937h5v5l-3.928 3.055m-2.259 1.757l-2.813 2.188l-4 4l-3 -3l4 -4l2.19 -2.815" /> <path d="M6.5 11.5l6 6" /> <path d="M3 3l18 18" /> </svg>"##;
const SWORDS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 3v5l-11 9l-4 4l-3 -3l4 -4l9 -11l5 0" /> <path d="M5 13l6 6" /> <path d="M14.32 17.32l3.68 3.68l3 -3l-3.365 -3.365" /> <path d="M10 5.5l-2 -2.5h-5v5l3 2.5" /> </svg>"##;
const TARGET_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 21l-1.74 -6" /> <path d="M7 21l1.74 -6" /> <path d="M12 4v-1" /> <path d="M14 10a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M18 10a6 6 0 1 0 -12 0a6 6 0 0 0 12 0" /> </svg>"##;
const TARGET_ARROW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M12 7a5 5 0 1 0 5 5" /> <path d="M13 3.055a9 9 0 1 0 7.941 7.945" /> <path d="M15 6v3h3l3 -3h-3v-3l-3 3" /> <path d="M15 9l-3 3" /> </svg>"##;
const TIC_TAC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 12h18" /> <path d="M12 3v18" /> <path d="M4 16l4 4" /> <path d="M4 20l4 -4" /> <path d="M16 4l4 4" /> <path d="M16 8l4 -4" /> <path d="M16 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const TOURNAMENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 4a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M18 10a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M2 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M2 20a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6 12h3a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-3" /> <path d="M6 4h7a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-2" /> <path d="M14 10h4" /> </svg>"##;
const TREADMILL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 3a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M3 14l4 1l.5 -.5" /> <path d="M12 18v-3l-3 -2.923l.75 -5.077" /> <path d="M6 10v-2l4 -1l2.5 2.5l2.5 .5" /> <path d="M21 22a1 1 0 0 0 -1 -1h-16a1 1 0 0 0 -1 1" /> <path d="M18 21l1 -11l2 -1" /> </svg>"##;
const TREKKING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 4a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M7 21l2 -4" /> <path d="M13 21v-4l-3 -3l1 -6l3 4l3 2" /> <path d="M10 14l-1.827 -1.218a2 2 0 0 1 -.831 -2.15l.28 -1.117a2 2 0 0 1 1.939 -1.515h1.439l4 1l3 -2" /> <path d="M17 12v9" /> <path d="M16 20h2" /> </svg>"##;
const TROPHY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 21l8 0" /> <path d="M12 17l0 4" /> <path d="M7 4l10 0" /> <path d="M17 4v8a5 5 0 0 1 -10 0v-8" /> <path d="M3 9a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 9a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const TROPHY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 21h8" /> <path d="M12 17v4" /> <path d="M8 4h9" /> <path d="M17 4v8c0 .31 -.028 .612 -.082 .905m-1.384 2.632a5 5 0 0 1 -8.534 -3.537v-5" /> <path d="M3 9a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 9a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 3l18 18" /> </svg>"##;
const UFO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.95 9.01c3.02 .739 5.05 2.123 5.05 3.714c0 2.367 -4.48 4.276 -10 4.276s-10 -1.909 -10 -4.276c0 -1.59 2.04 -2.985 5.07 -3.724" /> <path d="M7 9c0 1.105 2.239 2 5 2s5 -.895 5 -2v-.035c0 -2.742 -2.239 -4.965 -5 -4.965s-5 2.223 -5 4.965v.035" /> <path d="M15 17l2 3" /> <path d="M8.5 17l-1.5 3" /> <path d="M12 14h.01" /> <path d="M7 13h.01" /> <path d="M17 13h.01" /> </svg>"##;
const USER_SCREEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.03 17.818a3 3 0 0 0 1.97 -2.818v-8a3 3 0 0 0 -3 -3h-12a3 3 0 0 0 -3 3v8c0 1.317 .85 2.436 2.03 2.84" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M8 21a2 2 0 0 1 2 -2h4a2 2 0 0 1 2 2" /> </svg>"##;
const VIDEO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 10l4.553 -2.276a1 1 0 0 1 1.447 .894v6.764a1 1 0 0 1 -1.447 .894l-4.553 -2.276v-4" /> <path d="M3 8a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /> </svg>"##;
const VIDEO_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 10l4.553 -2.276a1 1 0 0 1 1.447 .894v6.764a1 1 0 0 1 -1.447 .894l-4.553 -2.276v-4" /> <path d="M3 8a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /> <path d="M7 12l4 0" /> </svg>"##;
const VIDEO_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M15 11v-1l4.553 -2.276a1 1 0 0 1 1.447 .894v6.764a1 1 0 0 1 -.675 .946" /> <path d="M10 6h3a2 2 0 0 1 2 2v3m0 4v1a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h1" /> </svg>"##;
const VIDEO_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 10l4.553 -2.276a1 1 0 0 1 1.447 .894v6.764a1 1 0 0 1 -1.447 .894l-4.553 -2.276v-4" /> <path d="M3 8a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /> <path d="M7 12l4 0" /> <path d="M9 10l0 4" /> </svg>"##;
const VOLUME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8a5 5 0 0 1 0 8" /> <path d="M17.7 5a9 9 0 0 1 0 14" /> <path d="M6 15h-2a1 1 0 0 1 -1 -1v-4a1 1 0 0 1 1 -1h2l3.5 -4.5a.8 .8 0 0 1 1.5 .5v14a.8 .8 0 0 1 -1.5 .5l-3.5 -4.5" /> </svg>"##;
const VOLUME_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8a5 5 0 0 1 0 8" /> <path d="M6 15h-2a1 1 0 0 1 -1 -1v-4a1 1 0 0 1 1 -1h2l3.5 -4.5a.8 .8 0 0 1 1.5 .5v14a.8 .8 0 0 1 -1.5 .5l-3.5 -4.5" /> </svg>"##;
const VOLUME_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 15h-2a1 1 0 0 1 -1 -1v-4a1 1 0 0 1 1 -1h2l3.5 -4.5a.8 .8 0 0 1 1.5 .5v14a.8 .8 0 0 1 -1.5 .5l-3.5 -4.5" /> <path d="M16 10l4 4m0 -4l-4 4" /> </svg>"##;
const VOLUME_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 15h-2a1 1 0 0 1 -1 -1v-4a1 1 0 0 1 1 -1h2l3.5 -4.5a.8 .8 0 0 1 1.5 .5v14a.8 .8 0 0 1 -1.5 .5l-3.5 -4.5" /> </svg>"##;
const VOLUME_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8a5 5 0 0 1 1.912 4.934m-1.377 2.602a5 5 0 0 1 -.535 .464" /> <path d="M17.7 5a9 9 0 0 1 2.362 11.086m-1.676 2.299a9 9 0 0 1 -.686 .615" /> <path d="M9.069 5.054l.431 -.554a.8 .8 0 0 1 1.5 .5v2m0 4v8a.8 .8 0 0 1 -1.5 .5l-3.5 -4.5h-2a1 1 0 0 1 -1 -1v-4a1 1 0 0 1 1 -1h2l1.294 -1.664" /> <path d="M3 3l18 18" /> </svg>"##;
const WALK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M7 21l3 -4" /> <path d="M16 21l-2 -4l-3 -3l1 -6" /> <path d="M6 12l2 -3l4 -1l3 3l3 1" /> </svg>"##;
const WATERPOLO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 8l3 4l5 1l7 -1" /> <path d="M3 18.75a2.4 2.4 0 0 0 1 .25a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 1 -.25" /> <path d="M12 16l1 -3" /> <path d="M11.007 9a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5.007 3.5a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> </svg>"##;
const YOGA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l1.5 -3" /> <path d="M17 20l-1 -5h-5l1 -7" /> <path d="M4 10l4 -1l4 -1l4 1.5l4 1.5" /> <path d="M10.007 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;

/// Media icon variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MediaIcon {
    Acrobatic,
    Album,
    AlbumOff,
    ArcheryArrow,
    AspectRatio,
    AspectRatioOff,
    Axe,
    BallAmericanFootball,
    BallAmericanFootballOff,
    BallBaseball,
    BallBasketball,
    BallBowling,
    BallFootball,
    BallFootballOff,
    BallTennis,
    BallVolleyball,
    Barbell,
    BarbellOff,
    Bow,
    Bowling,
    Camera,
    CameraAi,
    CameraBitcoin,
    CameraBolt,
    CameraCancel,
    CameraCheck,
    CameraCode,
    CameraCog,
    CameraDollar,
    CameraDown,
    CameraExclamation,
    CameraHeart,
    CameraMinus,
    CameraMoon,
    CameraOff,
    CameraPause,
    CameraPin,
    CameraPlus,
    CameraQuestion,
    CameraSearch,
    CameraShare,
    CameraSpark,
    CameraStar,
    CameraUp,
    CameraX,
    Capture,
    CaptureOff,
    Cast,
    CastOff,
    Chess,
    ChessBishop,
    ChessKing,
    ChessKnight,
    ChessQueen,
    ChessRook,
    Clef,
    ClefStaff,
    CliffJumping,
    Cricket,
    Curling,
    Dice,
    Dice1,
    Dice2,
    Dice3,
    Dice4,
    Dice5,
    Dice6,
    DiscGolf,
    Dumbbell,
    ExerciseBall,
    Ghost,
    Ghost2,
    Ghost3,
    GhostOff,
    GoGame,
    Golf,
    GolfOff,
    Headphones,
    HeadphonesOff,
    Headset,
    HeadsetOff,
    Helmet,
    HelmetOff,
    HorseToy,
    HulaHoop,
    IceSkating,
    Joker,
    JumpRope,
    Karate,
    Kayak,
    Keyframe,
    KeyframeAlignCenter,
    KeyframeAlignHorizontal,
    KeyframeAlignVertical,
    Keyframes,
    Maximize,
    MaximizeOff,
    Medal,
    Medal2,
    Meeple,
    Microphone,
    Microphone2,
    Microphone2Off,
    MicrophoneOff,
    Minimize,
    Movie,
    MovieOff,
    Music,
    MusicBolt,
    MusicCancel,
    MusicCheck,
    MusicCode,
    MusicCog,
    MusicDollar,
    MusicDown,
    MusicExclamation,
    MusicHeart,
    MusicMinus,
    MusicOff,
    MusicPause,
    MusicPin,
    MusicPlus,
    MusicQuestion,
    MusicSearch,
    MusicShare,
    MusicStar,
    MusicUp,
    MusicX,
    OlympicTorch,
    Olympics,
    OlympicsOff,
    Pacman,
    Photo,
    PhotoAi,
    PhotoAlt,
    PhotoBolt,
    PhotoCancel,
    PhotoCheck,
    PhotoCircleMinus,
    PhotoCirclePlus,
    PhotoCode,
    PhotoCog,
    PhotoDollar,
    PhotoDown,
    PhotoEdit,
    PhotoExclamation,
    PhotoHeart,
    PhotoMinus,
    PhotoOff,
    PhotoPause,
    PhotoPin,
    PhotoPlus,
    PhotoQuestion,
    PhotoSearch,
    PhotoShare,
    PhotoShield,
    PhotoSpark,
    PhotoStar,
    PhotoUp,
    PhotoX,
    Piano,
    Pick,
    PictureInPicture,
    PictureInPictureOff,
    PictureInPictureOn,
    PictureInPictureTop,
    PingPong,
    PlayBasketball,
    PlayCard,
    PlayCard1,
    PlayCard10,
    PlayCard2,
    PlayCard3,
    PlayCard4,
    PlayCard5,
    PlayCard6,
    PlayCard7,
    PlayCard8,
    PlayCard9,
    PlayCardA,
    PlayCardJ,
    PlayCardK,
    PlayCardOff,
    PlayCardQ,
    PlayCardStar,
    PlayFootball,
    PlayHandball,
    PlayVolleyball,
    PlayerEject,
    PlayerPause,
    PlayerPlay,
    PlayerRecord,
    PlayerSkipBack,
    PlayerSkipForward,
    PlayerStop,
    PlayerTrackNext,
    PlayerTrackPrev,
    Playlist,
    PlaylistOff,
    PlaylistX,
    PokerChip,
    Pool,
    PoolOff,
    PumpkinScary,
    Puzzle,
    Puzzle2,
    PuzzleOff,
    Radio,
    RadioOff,
    Repeat,
    RepeatOff,
    RepeatOnce,
    RewindBackward10,
    RewindBackward15,
    RewindBackward20,
    RewindBackward30,
    RewindBackward40,
    RewindBackward5,
    RewindBackward50,
    RewindBackward60,
    RewindForward10,
    RewindForward15,
    RewindForward20,
    RewindForward30,
    RewindForward40,
    RewindForward5,
    RewindForward50,
    RewindForward60,
    Rings,
    Robot,
    RollerSkating,
    Roulette,
    Rugby,
    Run,
    Scoreboard,
    ScubaDiving,
    ScubaDivingTank,
    ScubaMask,
    ScubaMaskOff,
    Shareplay,
    Skateboarding,
    SkiJumping,
    SoccerField,
    Speakerphone,
    SportBillard,
    Stretching,
    Stretching2,
    Subtitles,
    SubtitlesAi,
    SubtitlesEdit,
    SubtitlesOff,
    Swimming,
    Sword,
    SwordOff,
    Swords,
    Target2,
    TargetArrow,
    TicTac,
    Tournament,
    Treadmill,
    Trekking,
    Trophy,
    TrophyOff,
    Ufo,
    UserScreen,
    Video,
    VideoMinus,
    VideoOff,
    VideoPlus,
    Volume,
    Volume2,
    Volume3,
    Volume4,
    VolumeOff,
    Walk,
    Waterpolo,
    Yoga,
}

impl MediaIcon {
    /// Returns all available icons in this category.
    pub fn all() -> &'static [Self] {
        &[Self::Acrobatic, Self::Album, Self::AlbumOff, Self::ArcheryArrow, Self::AspectRatio, Self::AspectRatioOff, Self::Axe, Self::BallAmericanFootball, Self::BallAmericanFootballOff, Self::BallBaseball, Self::BallBasketball, Self::BallBowling, Self::BallFootball, Self::BallFootballOff, Self::BallTennis, Self::BallVolleyball, Self::Barbell, Self::BarbellOff, Self::Bow, Self::Bowling, Self::Camera, Self::CameraAi, Self::CameraBitcoin, Self::CameraBolt, Self::CameraCancel, Self::CameraCheck, Self::CameraCode, Self::CameraCog, Self::CameraDollar, Self::CameraDown, Self::CameraExclamation, Self::CameraHeart, Self::CameraMinus, Self::CameraMoon, Self::CameraOff, Self::CameraPause, Self::CameraPin, Self::CameraPlus, Self::CameraQuestion, Self::CameraSearch, Self::CameraShare, Self::CameraSpark, Self::CameraStar, Self::CameraUp, Self::CameraX, Self::Capture, Self::CaptureOff, Self::Cast, Self::CastOff, Self::Chess, Self::ChessBishop, Self::ChessKing, Self::ChessKnight, Self::ChessQueen, Self::ChessRook, Self::Clef, Self::ClefStaff, Self::CliffJumping, Self::Cricket, Self::Curling, Self::Dice, Self::Dice1, Self::Dice2, Self::Dice3, Self::Dice4, Self::Dice5, Self::Dice6, Self::DiscGolf, Self::Dumbbell, Self::ExerciseBall, Self::Ghost, Self::Ghost2, Self::Ghost3, Self::GhostOff, Self::GoGame, Self::Golf, Self::GolfOff, Self::Headphones, Self::HeadphonesOff, Self::Headset, Self::HeadsetOff, Self::Helmet, Self::HelmetOff, Self::HorseToy, Self::HulaHoop, Self::IceSkating, Self::Joker, Self::JumpRope, Self::Karate, Self::Kayak, Self::Keyframe, Self::KeyframeAlignCenter, Self::KeyframeAlignHorizontal, Self::KeyframeAlignVertical, Self::Keyframes, Self::Maximize, Self::MaximizeOff, Self::Medal, Self::Medal2, Self::Meeple, Self::Microphone, Self::Microphone2, Self::Microphone2Off, Self::MicrophoneOff, Self::Minimize, Self::Movie, Self::MovieOff, Self::Music, Self::MusicBolt, Self::MusicCancel, Self::MusicCheck, Self::MusicCode, Self::MusicCog, Self::MusicDollar, Self::MusicDown, Self::MusicExclamation, Self::MusicHeart, Self::MusicMinus, Self::MusicOff, Self::MusicPause, Self::MusicPin, Self::MusicPlus, Self::MusicQuestion, Self::MusicSearch, Self::MusicShare, Self::MusicStar, Self::MusicUp, Self::MusicX, Self::OlympicTorch, Self::Olympics, Self::OlympicsOff, Self::Pacman, Self::Photo, Self::PhotoAi, Self::PhotoAlt, Self::PhotoBolt, Self::PhotoCancel, Self::PhotoCheck, Self::PhotoCircleMinus, Self::PhotoCirclePlus, Self::PhotoCode, Self::PhotoCog, Self::PhotoDollar, Self::PhotoDown, Self::PhotoEdit, Self::PhotoExclamation, Self::PhotoHeart, Self::PhotoMinus, Self::PhotoOff, Self::PhotoPause, Self::PhotoPin, Self::PhotoPlus, Self::PhotoQuestion, Self::PhotoSearch, Self::PhotoShare, Self::PhotoShield, Self::PhotoSpark, Self::PhotoStar, Self::PhotoUp, Self::PhotoX, Self::Piano, Self::Pick, Self::PictureInPicture, Self::PictureInPictureOff, Self::PictureInPictureOn, Self::PictureInPictureTop, Self::PingPong, Self::PlayBasketball, Self::PlayCard, Self::PlayCard1, Self::PlayCard10, Self::PlayCard2, Self::PlayCard3, Self::PlayCard4, Self::PlayCard5, Self::PlayCard6, Self::PlayCard7, Self::PlayCard8, Self::PlayCard9, Self::PlayCardA, Self::PlayCardJ, Self::PlayCardK, Self::PlayCardOff, Self::PlayCardQ, Self::PlayCardStar, Self::PlayFootball, Self::PlayHandball, Self::PlayVolleyball, Self::PlayerEject, Self::PlayerPause, Self::PlayerPlay, Self::PlayerRecord, Self::PlayerSkipBack, Self::PlayerSkipForward, Self::PlayerStop, Self::PlayerTrackNext, Self::PlayerTrackPrev, Self::Playlist, Self::PlaylistOff, Self::PlaylistX, Self::PokerChip, Self::Pool, Self::PoolOff, Self::PumpkinScary, Self::Puzzle, Self::Puzzle2, Self::PuzzleOff, Self::Radio, Self::RadioOff, Self::Repeat, Self::RepeatOff, Self::RepeatOnce, Self::RewindBackward10, Self::RewindBackward15, Self::RewindBackward20, Self::RewindBackward30, Self::RewindBackward40, Self::RewindBackward5, Self::RewindBackward50, Self::RewindBackward60, Self::RewindForward10, Self::RewindForward15, Self::RewindForward20, Self::RewindForward30, Self::RewindForward40, Self::RewindForward5, Self::RewindForward50, Self::RewindForward60, Self::Rings, Self::Robot, Self::RollerSkating, Self::Roulette, Self::Rugby, Self::Run, Self::Scoreboard, Self::ScubaDiving, Self::ScubaDivingTank, Self::ScubaMask, Self::ScubaMaskOff, Self::Shareplay, Self::Skateboarding, Self::SkiJumping, Self::SoccerField, Self::Speakerphone, Self::SportBillard, Self::Stretching, Self::Stretching2, Self::Subtitles, Self::SubtitlesAi, Self::SubtitlesEdit, Self::SubtitlesOff, Self::Swimming, Self::Sword, Self::SwordOff, Self::Swords, Self::Target2, Self::TargetArrow, Self::TicTac, Self::Tournament, Self::Treadmill, Self::Trekking, Self::Trophy, Self::TrophyOff, Self::Ufo, Self::UserScreen, Self::Video, Self::VideoMinus, Self::VideoOff, Self::VideoPlus, Self::Volume, Self::Volume2, Self::Volume3, Self::Volume4, Self::VolumeOff, Self::Walk, Self::Waterpolo, Self::Yoga]
    }

    /// Returns the icon count.
    pub fn count() -> usize {
        277
    }

    /// Creates an icon from its kebab-case name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "acrobatic" => Some(Self::Acrobatic),
            "album" => Some(Self::Album),
            "album-off" => Some(Self::AlbumOff),
            "archery-arrow" => Some(Self::ArcheryArrow),
            "aspect-ratio" => Some(Self::AspectRatio),
            "aspect-ratio-off" => Some(Self::AspectRatioOff),
            "axe" => Some(Self::Axe),
            "ball-american-football" => Some(Self::BallAmericanFootball),
            "ball-american-football-off" => Some(Self::BallAmericanFootballOff),
            "ball-baseball" => Some(Self::BallBaseball),
            "ball-basketball" => Some(Self::BallBasketball),
            "ball-bowling" => Some(Self::BallBowling),
            "ball-football" => Some(Self::BallFootball),
            "ball-football-off" => Some(Self::BallFootballOff),
            "ball-tennis" => Some(Self::BallTennis),
            "ball-volleyball" => Some(Self::BallVolleyball),
            "barbell" => Some(Self::Barbell),
            "barbell-off" => Some(Self::BarbellOff),
            "bow" => Some(Self::Bow),
            "bowling" => Some(Self::Bowling),
            "camera" => Some(Self::Camera),
            "camera-ai" => Some(Self::CameraAi),
            "camera-bitcoin" => Some(Self::CameraBitcoin),
            "camera-bolt" => Some(Self::CameraBolt),
            "camera-cancel" => Some(Self::CameraCancel),
            "camera-check" => Some(Self::CameraCheck),
            "camera-code" => Some(Self::CameraCode),
            "camera-cog" => Some(Self::CameraCog),
            "camera-dollar" => Some(Self::CameraDollar),
            "camera-down" => Some(Self::CameraDown),
            "camera-exclamation" => Some(Self::CameraExclamation),
            "camera-heart" => Some(Self::CameraHeart),
            "camera-minus" => Some(Self::CameraMinus),
            "camera-moon" => Some(Self::CameraMoon),
            "camera-off" => Some(Self::CameraOff),
            "camera-pause" => Some(Self::CameraPause),
            "camera-pin" => Some(Self::CameraPin),
            "camera-plus" => Some(Self::CameraPlus),
            "camera-question" => Some(Self::CameraQuestion),
            "camera-search" => Some(Self::CameraSearch),
            "camera-share" => Some(Self::CameraShare),
            "camera-spark" => Some(Self::CameraSpark),
            "camera-star" => Some(Self::CameraStar),
            "camera-up" => Some(Self::CameraUp),
            "camera-x" => Some(Self::CameraX),
            "capture" => Some(Self::Capture),
            "capture-off" => Some(Self::CaptureOff),
            "cast" => Some(Self::Cast),
            "cast-off" => Some(Self::CastOff),
            "chess" => Some(Self::Chess),
            "chess-bishop" => Some(Self::ChessBishop),
            "chess-king" => Some(Self::ChessKing),
            "chess-knight" => Some(Self::ChessKnight),
            "chess-queen" => Some(Self::ChessQueen),
            "chess-rook" => Some(Self::ChessRook),
            "clef" => Some(Self::Clef),
            "clef-staff" => Some(Self::ClefStaff),
            "cliff-jumping" => Some(Self::CliffJumping),
            "cricket" => Some(Self::Cricket),
            "curling" => Some(Self::Curling),
            "dice" => Some(Self::Dice),
            "dice-1" => Some(Self::Dice1),
            "dice-2" => Some(Self::Dice2),
            "dice-3" => Some(Self::Dice3),
            "dice-4" => Some(Self::Dice4),
            "dice-5" => Some(Self::Dice5),
            "dice-6" => Some(Self::Dice6),
            "disc-golf" => Some(Self::DiscGolf),
            "dumbbell" => Some(Self::Dumbbell),
            "exercise-ball" => Some(Self::ExerciseBall),
            "ghost" => Some(Self::Ghost),
            "ghost-2" => Some(Self::Ghost2),
            "ghost-3" => Some(Self::Ghost3),
            "ghost-off" => Some(Self::GhostOff),
            "go-game" => Some(Self::GoGame),
            "golf" => Some(Self::Golf),
            "golf-off" => Some(Self::GolfOff),
            "headphones" => Some(Self::Headphones),
            "headphones-off" => Some(Self::HeadphonesOff),
            "headset" => Some(Self::Headset),
            "headset-off" => Some(Self::HeadsetOff),
            "helmet" => Some(Self::Helmet),
            "helmet-off" => Some(Self::HelmetOff),
            "horse-toy" => Some(Self::HorseToy),
            "hula-hoop" => Some(Self::HulaHoop),
            "ice-skating" => Some(Self::IceSkating),
            "joker" => Some(Self::Joker),
            "jump-rope" => Some(Self::JumpRope),
            "karate" => Some(Self::Karate),
            "kayak" => Some(Self::Kayak),
            "keyframe" => Some(Self::Keyframe),
            "keyframe-align-center" => Some(Self::KeyframeAlignCenter),
            "keyframe-align-horizontal" => Some(Self::KeyframeAlignHorizontal),
            "keyframe-align-vertical" => Some(Self::KeyframeAlignVertical),
            "keyframes" => Some(Self::Keyframes),
            "maximize" => Some(Self::Maximize),
            "maximize-off" => Some(Self::MaximizeOff),
            "medal" => Some(Self::Medal),
            "medal-2" => Some(Self::Medal2),
            "meeple" => Some(Self::Meeple),
            "microphone" => Some(Self::Microphone),
            "microphone-2" => Some(Self::Microphone2),
            "microphone-2-off" => Some(Self::Microphone2Off),
            "microphone-off" => Some(Self::MicrophoneOff),
            "minimize" => Some(Self::Minimize),
            "movie" => Some(Self::Movie),
            "movie-off" => Some(Self::MovieOff),
            "music" => Some(Self::Music),
            "music-bolt" => Some(Self::MusicBolt),
            "music-cancel" => Some(Self::MusicCancel),
            "music-check" => Some(Self::MusicCheck),
            "music-code" => Some(Self::MusicCode),
            "music-cog" => Some(Self::MusicCog),
            "music-dollar" => Some(Self::MusicDollar),
            "music-down" => Some(Self::MusicDown),
            "music-exclamation" => Some(Self::MusicExclamation),
            "music-heart" => Some(Self::MusicHeart),
            "music-minus" => Some(Self::MusicMinus),
            "music-off" => Some(Self::MusicOff),
            "music-pause" => Some(Self::MusicPause),
            "music-pin" => Some(Self::MusicPin),
            "music-plus" => Some(Self::MusicPlus),
            "music-question" => Some(Self::MusicQuestion),
            "music-search" => Some(Self::MusicSearch),
            "music-share" => Some(Self::MusicShare),
            "music-star" => Some(Self::MusicStar),
            "music-up" => Some(Self::MusicUp),
            "music-x" => Some(Self::MusicX),
            "olympic-torch" => Some(Self::OlympicTorch),
            "olympics" => Some(Self::Olympics),
            "olympics-off" => Some(Self::OlympicsOff),
            "pacman" => Some(Self::Pacman),
            "photo" => Some(Self::Photo),
            "photo-ai" => Some(Self::PhotoAi),
            "photo-alt" => Some(Self::PhotoAlt),
            "photo-bolt" => Some(Self::PhotoBolt),
            "photo-cancel" => Some(Self::PhotoCancel),
            "photo-check" => Some(Self::PhotoCheck),
            "photo-circle-minus" => Some(Self::PhotoCircleMinus),
            "photo-circle-plus" => Some(Self::PhotoCirclePlus),
            "photo-code" => Some(Self::PhotoCode),
            "photo-cog" => Some(Self::PhotoCog),
            "photo-dollar" => Some(Self::PhotoDollar),
            "photo-down" => Some(Self::PhotoDown),
            "photo-edit" => Some(Self::PhotoEdit),
            "photo-exclamation" => Some(Self::PhotoExclamation),
            "photo-heart" => Some(Self::PhotoHeart),
            "photo-minus" => Some(Self::PhotoMinus),
            "photo-off" => Some(Self::PhotoOff),
            "photo-pause" => Some(Self::PhotoPause),
            "photo-pin" => Some(Self::PhotoPin),
            "photo-plus" => Some(Self::PhotoPlus),
            "photo-question" => Some(Self::PhotoQuestion),
            "photo-search" => Some(Self::PhotoSearch),
            "photo-share" => Some(Self::PhotoShare),
            "photo-shield" => Some(Self::PhotoShield),
            "photo-spark" => Some(Self::PhotoSpark),
            "photo-star" => Some(Self::PhotoStar),
            "photo-up" => Some(Self::PhotoUp),
            "photo-x" => Some(Self::PhotoX),
            "piano" => Some(Self::Piano),
            "pick" => Some(Self::Pick),
            "picture-in-picture" => Some(Self::PictureInPicture),
            "picture-in-picture-off" => Some(Self::PictureInPictureOff),
            "picture-in-picture-on" => Some(Self::PictureInPictureOn),
            "picture-in-picture-top" => Some(Self::PictureInPictureTop),
            "ping-pong" => Some(Self::PingPong),
            "play-basketball" => Some(Self::PlayBasketball),
            "play-card" => Some(Self::PlayCard),
            "play-card-1" => Some(Self::PlayCard1),
            "play-card-10" => Some(Self::PlayCard10),
            "play-card-2" => Some(Self::PlayCard2),
            "play-card-3" => Some(Self::PlayCard3),
            "play-card-4" => Some(Self::PlayCard4),
            "play-card-5" => Some(Self::PlayCard5),
            "play-card-6" => Some(Self::PlayCard6),
            "play-card-7" => Some(Self::PlayCard7),
            "play-card-8" => Some(Self::PlayCard8),
            "play-card-9" => Some(Self::PlayCard9),
            "play-card-a" => Some(Self::PlayCardA),
            "play-card-j" => Some(Self::PlayCardJ),
            "play-card-k" => Some(Self::PlayCardK),
            "play-card-off" => Some(Self::PlayCardOff),
            "play-card-q" => Some(Self::PlayCardQ),
            "play-card-star" => Some(Self::PlayCardStar),
            "play-football" => Some(Self::PlayFootball),
            "play-handball" => Some(Self::PlayHandball),
            "play-volleyball" => Some(Self::PlayVolleyball),
            "player-eject" => Some(Self::PlayerEject),
            "player-pause" => Some(Self::PlayerPause),
            "player-play" => Some(Self::PlayerPlay),
            "player-record" => Some(Self::PlayerRecord),
            "player-skip-back" => Some(Self::PlayerSkipBack),
            "player-skip-forward" => Some(Self::PlayerSkipForward),
            "player-stop" => Some(Self::PlayerStop),
            "player-track-next" => Some(Self::PlayerTrackNext),
            "player-track-prev" => Some(Self::PlayerTrackPrev),
            "playlist" => Some(Self::Playlist),
            "playlist-off" => Some(Self::PlaylistOff),
            "playlist-x" => Some(Self::PlaylistX),
            "poker-chip" => Some(Self::PokerChip),
            "pool" => Some(Self::Pool),
            "pool-off" => Some(Self::PoolOff),
            "pumpkin-scary" => Some(Self::PumpkinScary),
            "puzzle" => Some(Self::Puzzle),
            "puzzle-2" => Some(Self::Puzzle2),
            "puzzle-off" => Some(Self::PuzzleOff),
            "radio" => Some(Self::Radio),
            "radio-off" => Some(Self::RadioOff),
            "repeat" => Some(Self::Repeat),
            "repeat-off" => Some(Self::RepeatOff),
            "repeat-once" => Some(Self::RepeatOnce),
            "rewind-backward-10" => Some(Self::RewindBackward10),
            "rewind-backward-15" => Some(Self::RewindBackward15),
            "rewind-backward-20" => Some(Self::RewindBackward20),
            "rewind-backward-30" => Some(Self::RewindBackward30),
            "rewind-backward-40" => Some(Self::RewindBackward40),
            "rewind-backward-5" => Some(Self::RewindBackward5),
            "rewind-backward-50" => Some(Self::RewindBackward50),
            "rewind-backward-60" => Some(Self::RewindBackward60),
            "rewind-forward-10" => Some(Self::RewindForward10),
            "rewind-forward-15" => Some(Self::RewindForward15),
            "rewind-forward-20" => Some(Self::RewindForward20),
            "rewind-forward-30" => Some(Self::RewindForward30),
            "rewind-forward-40" => Some(Self::RewindForward40),
            "rewind-forward-5" => Some(Self::RewindForward5),
            "rewind-forward-50" => Some(Self::RewindForward50),
            "rewind-forward-60" => Some(Self::RewindForward60),
            "rings" => Some(Self::Rings),
            "robot" => Some(Self::Robot),
            "roller-skating" => Some(Self::RollerSkating),
            "roulette" => Some(Self::Roulette),
            "rugby" => Some(Self::Rugby),
            "run" => Some(Self::Run),
            "scoreboard" => Some(Self::Scoreboard),
            "scuba-diving" => Some(Self::ScubaDiving),
            "scuba-diving-tank" => Some(Self::ScubaDivingTank),
            "scuba-mask" => Some(Self::ScubaMask),
            "scuba-mask-off" => Some(Self::ScubaMaskOff),
            "shareplay" => Some(Self::Shareplay),
            "skateboarding" => Some(Self::Skateboarding),
            "ski-jumping" => Some(Self::SkiJumping),
            "soccer-field" => Some(Self::SoccerField),
            "speakerphone" => Some(Self::Speakerphone),
            "sport-billard" => Some(Self::SportBillard),
            "stretching" => Some(Self::Stretching),
            "stretching-2" => Some(Self::Stretching2),
            "subtitles" => Some(Self::Subtitles),
            "subtitles-ai" => Some(Self::SubtitlesAi),
            "subtitles-edit" => Some(Self::SubtitlesEdit),
            "subtitles-off" => Some(Self::SubtitlesOff),
            "swimming" => Some(Self::Swimming),
            "sword" => Some(Self::Sword),
            "sword-off" => Some(Self::SwordOff),
            "swords" => Some(Self::Swords),
            "target-2" => Some(Self::Target2),
            "target-arrow" => Some(Self::TargetArrow),
            "tic-tac" => Some(Self::TicTac),
            "tournament" => Some(Self::Tournament),
            "treadmill" => Some(Self::Treadmill),
            "trekking" => Some(Self::Trekking),
            "trophy" => Some(Self::Trophy),
            "trophy-off" => Some(Self::TrophyOff),
            "ufo" => Some(Self::Ufo),
            "user-screen" => Some(Self::UserScreen),
            "video" => Some(Self::Video),
            "video-minus" => Some(Self::VideoMinus),
            "video-off" => Some(Self::VideoOff),
            "video-plus" => Some(Self::VideoPlus),
            "volume" => Some(Self::Volume),
            "volume-2" => Some(Self::Volume2),
            "volume-3" => Some(Self::Volume3),
            "volume-4" => Some(Self::Volume4),
            "volume-off" => Some(Self::VolumeOff),
            "walk" => Some(Self::Walk),
            "waterpolo" => Some(Self::Waterpolo),
            "yoga" => Some(Self::Yoga),
            _ => None,
        }
    }
}

impl TablerIconData for MediaIcon {
    fn name(&self) -> &'static str {
        match self {
            Self::Acrobatic => "acrobatic",
            Self::Album => "album",
            Self::AlbumOff => "album-off",
            Self::ArcheryArrow => "archery-arrow",
            Self::AspectRatio => "aspect-ratio",
            Self::AspectRatioOff => "aspect-ratio-off",
            Self::Axe => "axe",
            Self::BallAmericanFootball => "ball-american-football",
            Self::BallAmericanFootballOff => "ball-american-football-off",
            Self::BallBaseball => "ball-baseball",
            Self::BallBasketball => "ball-basketball",
            Self::BallBowling => "ball-bowling",
            Self::BallFootball => "ball-football",
            Self::BallFootballOff => "ball-football-off",
            Self::BallTennis => "ball-tennis",
            Self::BallVolleyball => "ball-volleyball",
            Self::Barbell => "barbell",
            Self::BarbellOff => "barbell-off",
            Self::Bow => "bow",
            Self::Bowling => "bowling",
            Self::Camera => "camera",
            Self::CameraAi => "camera-ai",
            Self::CameraBitcoin => "camera-bitcoin",
            Self::CameraBolt => "camera-bolt",
            Self::CameraCancel => "camera-cancel",
            Self::CameraCheck => "camera-check",
            Self::CameraCode => "camera-code",
            Self::CameraCog => "camera-cog",
            Self::CameraDollar => "camera-dollar",
            Self::CameraDown => "camera-down",
            Self::CameraExclamation => "camera-exclamation",
            Self::CameraHeart => "camera-heart",
            Self::CameraMinus => "camera-minus",
            Self::CameraMoon => "camera-moon",
            Self::CameraOff => "camera-off",
            Self::CameraPause => "camera-pause",
            Self::CameraPin => "camera-pin",
            Self::CameraPlus => "camera-plus",
            Self::CameraQuestion => "camera-question",
            Self::CameraSearch => "camera-search",
            Self::CameraShare => "camera-share",
            Self::CameraSpark => "camera-spark",
            Self::CameraStar => "camera-star",
            Self::CameraUp => "camera-up",
            Self::CameraX => "camera-x",
            Self::Capture => "capture",
            Self::CaptureOff => "capture-off",
            Self::Cast => "cast",
            Self::CastOff => "cast-off",
            Self::Chess => "chess",
            Self::ChessBishop => "chess-bishop",
            Self::ChessKing => "chess-king",
            Self::ChessKnight => "chess-knight",
            Self::ChessQueen => "chess-queen",
            Self::ChessRook => "chess-rook",
            Self::Clef => "clef",
            Self::ClefStaff => "clef-staff",
            Self::CliffJumping => "cliff-jumping",
            Self::Cricket => "cricket",
            Self::Curling => "curling",
            Self::Dice => "dice",
            Self::Dice1 => "dice-1",
            Self::Dice2 => "dice-2",
            Self::Dice3 => "dice-3",
            Self::Dice4 => "dice-4",
            Self::Dice5 => "dice-5",
            Self::Dice6 => "dice-6",
            Self::DiscGolf => "disc-golf",
            Self::Dumbbell => "dumbbell",
            Self::ExerciseBall => "exercise-ball",
            Self::Ghost => "ghost",
            Self::Ghost2 => "ghost-2",
            Self::Ghost3 => "ghost-3",
            Self::GhostOff => "ghost-off",
            Self::GoGame => "go-game",
            Self::Golf => "golf",
            Self::GolfOff => "golf-off",
            Self::Headphones => "headphones",
            Self::HeadphonesOff => "headphones-off",
            Self::Headset => "headset",
            Self::HeadsetOff => "headset-off",
            Self::Helmet => "helmet",
            Self::HelmetOff => "helmet-off",
            Self::HorseToy => "horse-toy",
            Self::HulaHoop => "hula-hoop",
            Self::IceSkating => "ice-skating",
            Self::Joker => "joker",
            Self::JumpRope => "jump-rope",
            Self::Karate => "karate",
            Self::Kayak => "kayak",
            Self::Keyframe => "keyframe",
            Self::KeyframeAlignCenter => "keyframe-align-center",
            Self::KeyframeAlignHorizontal => "keyframe-align-horizontal",
            Self::KeyframeAlignVertical => "keyframe-align-vertical",
            Self::Keyframes => "keyframes",
            Self::Maximize => "maximize",
            Self::MaximizeOff => "maximize-off",
            Self::Medal => "medal",
            Self::Medal2 => "medal-2",
            Self::Meeple => "meeple",
            Self::Microphone => "microphone",
            Self::Microphone2 => "microphone-2",
            Self::Microphone2Off => "microphone-2-off",
            Self::MicrophoneOff => "microphone-off",
            Self::Minimize => "minimize",
            Self::Movie => "movie",
            Self::MovieOff => "movie-off",
            Self::Music => "music",
            Self::MusicBolt => "music-bolt",
            Self::MusicCancel => "music-cancel",
            Self::MusicCheck => "music-check",
            Self::MusicCode => "music-code",
            Self::MusicCog => "music-cog",
            Self::MusicDollar => "music-dollar",
            Self::MusicDown => "music-down",
            Self::MusicExclamation => "music-exclamation",
            Self::MusicHeart => "music-heart",
            Self::MusicMinus => "music-minus",
            Self::MusicOff => "music-off",
            Self::MusicPause => "music-pause",
            Self::MusicPin => "music-pin",
            Self::MusicPlus => "music-plus",
            Self::MusicQuestion => "music-question",
            Self::MusicSearch => "music-search",
            Self::MusicShare => "music-share",
            Self::MusicStar => "music-star",
            Self::MusicUp => "music-up",
            Self::MusicX => "music-x",
            Self::OlympicTorch => "olympic-torch",
            Self::Olympics => "olympics",
            Self::OlympicsOff => "olympics-off",
            Self::Pacman => "pacman",
            Self::Photo => "photo",
            Self::PhotoAi => "photo-ai",
            Self::PhotoAlt => "photo-alt",
            Self::PhotoBolt => "photo-bolt",
            Self::PhotoCancel => "photo-cancel",
            Self::PhotoCheck => "photo-check",
            Self::PhotoCircleMinus => "photo-circle-minus",
            Self::PhotoCirclePlus => "photo-circle-plus",
            Self::PhotoCode => "photo-code",
            Self::PhotoCog => "photo-cog",
            Self::PhotoDollar => "photo-dollar",
            Self::PhotoDown => "photo-down",
            Self::PhotoEdit => "photo-edit",
            Self::PhotoExclamation => "photo-exclamation",
            Self::PhotoHeart => "photo-heart",
            Self::PhotoMinus => "photo-minus",
            Self::PhotoOff => "photo-off",
            Self::PhotoPause => "photo-pause",
            Self::PhotoPin => "photo-pin",
            Self::PhotoPlus => "photo-plus",
            Self::PhotoQuestion => "photo-question",
            Self::PhotoSearch => "photo-search",
            Self::PhotoShare => "photo-share",
            Self::PhotoShield => "photo-shield",
            Self::PhotoSpark => "photo-spark",
            Self::PhotoStar => "photo-star",
            Self::PhotoUp => "photo-up",
            Self::PhotoX => "photo-x",
            Self::Piano => "piano",
            Self::Pick => "pick",
            Self::PictureInPicture => "picture-in-picture",
            Self::PictureInPictureOff => "picture-in-picture-off",
            Self::PictureInPictureOn => "picture-in-picture-on",
            Self::PictureInPictureTop => "picture-in-picture-top",
            Self::PingPong => "ping-pong",
            Self::PlayBasketball => "play-basketball",
            Self::PlayCard => "play-card",
            Self::PlayCard1 => "play-card-1",
            Self::PlayCard10 => "play-card-10",
            Self::PlayCard2 => "play-card-2",
            Self::PlayCard3 => "play-card-3",
            Self::PlayCard4 => "play-card-4",
            Self::PlayCard5 => "play-card-5",
            Self::PlayCard6 => "play-card-6",
            Self::PlayCard7 => "play-card-7",
            Self::PlayCard8 => "play-card-8",
            Self::PlayCard9 => "play-card-9",
            Self::PlayCardA => "play-card-a",
            Self::PlayCardJ => "play-card-j",
            Self::PlayCardK => "play-card-k",
            Self::PlayCardOff => "play-card-off",
            Self::PlayCardQ => "play-card-q",
            Self::PlayCardStar => "play-card-star",
            Self::PlayFootball => "play-football",
            Self::PlayHandball => "play-handball",
            Self::PlayVolleyball => "play-volleyball",
            Self::PlayerEject => "player-eject",
            Self::PlayerPause => "player-pause",
            Self::PlayerPlay => "player-play",
            Self::PlayerRecord => "player-record",
            Self::PlayerSkipBack => "player-skip-back",
            Self::PlayerSkipForward => "player-skip-forward",
            Self::PlayerStop => "player-stop",
            Self::PlayerTrackNext => "player-track-next",
            Self::PlayerTrackPrev => "player-track-prev",
            Self::Playlist => "playlist",
            Self::PlaylistOff => "playlist-off",
            Self::PlaylistX => "playlist-x",
            Self::PokerChip => "poker-chip",
            Self::Pool => "pool",
            Self::PoolOff => "pool-off",
            Self::PumpkinScary => "pumpkin-scary",
            Self::Puzzle => "puzzle",
            Self::Puzzle2 => "puzzle-2",
            Self::PuzzleOff => "puzzle-off",
            Self::Radio => "radio",
            Self::RadioOff => "radio-off",
            Self::Repeat => "repeat",
            Self::RepeatOff => "repeat-off",
            Self::RepeatOnce => "repeat-once",
            Self::RewindBackward10 => "rewind-backward-10",
            Self::RewindBackward15 => "rewind-backward-15",
            Self::RewindBackward20 => "rewind-backward-20",
            Self::RewindBackward30 => "rewind-backward-30",
            Self::RewindBackward40 => "rewind-backward-40",
            Self::RewindBackward5 => "rewind-backward-5",
            Self::RewindBackward50 => "rewind-backward-50",
            Self::RewindBackward60 => "rewind-backward-60",
            Self::RewindForward10 => "rewind-forward-10",
            Self::RewindForward15 => "rewind-forward-15",
            Self::RewindForward20 => "rewind-forward-20",
            Self::RewindForward30 => "rewind-forward-30",
            Self::RewindForward40 => "rewind-forward-40",
            Self::RewindForward5 => "rewind-forward-5",
            Self::RewindForward50 => "rewind-forward-50",
            Self::RewindForward60 => "rewind-forward-60",
            Self::Rings => "rings",
            Self::Robot => "robot",
            Self::RollerSkating => "roller-skating",
            Self::Roulette => "roulette",
            Self::Rugby => "rugby",
            Self::Run => "run",
            Self::Scoreboard => "scoreboard",
            Self::ScubaDiving => "scuba-diving",
            Self::ScubaDivingTank => "scuba-diving-tank",
            Self::ScubaMask => "scuba-mask",
            Self::ScubaMaskOff => "scuba-mask-off",
            Self::Shareplay => "shareplay",
            Self::Skateboarding => "skateboarding",
            Self::SkiJumping => "ski-jumping",
            Self::SoccerField => "soccer-field",
            Self::Speakerphone => "speakerphone",
            Self::SportBillard => "sport-billard",
            Self::Stretching => "stretching",
            Self::Stretching2 => "stretching-2",
            Self::Subtitles => "subtitles",
            Self::SubtitlesAi => "subtitles-ai",
            Self::SubtitlesEdit => "subtitles-edit",
            Self::SubtitlesOff => "subtitles-off",
            Self::Swimming => "swimming",
            Self::Sword => "sword",
            Self::SwordOff => "sword-off",
            Self::Swords => "swords",
            Self::Target2 => "target-2",
            Self::TargetArrow => "target-arrow",
            Self::TicTac => "tic-tac",
            Self::Tournament => "tournament",
            Self::Treadmill => "treadmill",
            Self::Trekking => "trekking",
            Self::Trophy => "trophy",
            Self::TrophyOff => "trophy-off",
            Self::Ufo => "ufo",
            Self::UserScreen => "user-screen",
            Self::Video => "video",
            Self::VideoMinus => "video-minus",
            Self::VideoOff => "video-off",
            Self::VideoPlus => "video-plus",
            Self::Volume => "volume",
            Self::Volume2 => "volume-2",
            Self::Volume3 => "volume-3",
            Self::Volume4 => "volume-4",
            Self::VolumeOff => "volume-off",
            Self::Walk => "walk",
            Self::Waterpolo => "waterpolo",
            Self::Yoga => "yoga",
        }
    }

    fn outline_svg(&self) -> &'static str {
        match self {
            Self::Acrobatic => ACROBATIC_SVG,
            Self::Album => ALBUM_SVG,
            Self::AlbumOff => ALBUM_OFF_SVG,
            Self::ArcheryArrow => ARCHERY_ARROW_SVG,
            Self::AspectRatio => ASPECT_RATIO_SVG,
            Self::AspectRatioOff => ASPECT_RATIO_OFF_SVG,
            Self::Axe => AXE_SVG,
            Self::BallAmericanFootball => BALL_AMERICAN_FOOTBALL_SVG,
            Self::BallAmericanFootballOff => BALL_AMERICAN_FOOTBALL_OFF_SVG,
            Self::BallBaseball => BALL_BASEBALL_SVG,
            Self::BallBasketball => BALL_BASKETBALL_SVG,
            Self::BallBowling => BALL_BOWLING_SVG,
            Self::BallFootball => BALL_FOOTBALL_SVG,
            Self::BallFootballOff => BALL_FOOTBALL_OFF_SVG,
            Self::BallTennis => BALL_TENNIS_SVG,
            Self::BallVolleyball => BALL_VOLLEYBALL_SVG,
            Self::Barbell => BARBELL_SVG,
            Self::BarbellOff => BARBELL_OFF_SVG,
            Self::Bow => BOW_SVG,
            Self::Bowling => BOWLING_SVG,
            Self::Camera => CAMERA_SVG,
            Self::CameraAi => CAMERA_AI_SVG,
            Self::CameraBitcoin => CAMERA_BITCOIN_SVG,
            Self::CameraBolt => CAMERA_BOLT_SVG,
            Self::CameraCancel => CAMERA_CANCEL_SVG,
            Self::CameraCheck => CAMERA_CHECK_SVG,
            Self::CameraCode => CAMERA_CODE_SVG,
            Self::CameraCog => CAMERA_COG_SVG,
            Self::CameraDollar => CAMERA_DOLLAR_SVG,
            Self::CameraDown => CAMERA_DOWN_SVG,
            Self::CameraExclamation => CAMERA_EXCLAMATION_SVG,
            Self::CameraHeart => CAMERA_HEART_SVG,
            Self::CameraMinus => CAMERA_MINUS_SVG,
            Self::CameraMoon => CAMERA_MOON_SVG,
            Self::CameraOff => CAMERA_OFF_SVG,
            Self::CameraPause => CAMERA_PAUSE_SVG,
            Self::CameraPin => CAMERA_PIN_SVG,
            Self::CameraPlus => CAMERA_PLUS_SVG,
            Self::CameraQuestion => CAMERA_QUESTION_SVG,
            Self::CameraSearch => CAMERA_SEARCH_SVG,
            Self::CameraShare => CAMERA_SHARE_SVG,
            Self::CameraSpark => CAMERA_SPARK_SVG,
            Self::CameraStar => CAMERA_STAR_SVG,
            Self::CameraUp => CAMERA_UP_SVG,
            Self::CameraX => CAMERA_X_SVG,
            Self::Capture => CAPTURE_SVG,
            Self::CaptureOff => CAPTURE_OFF_SVG,
            Self::Cast => CAST_SVG,
            Self::CastOff => CAST_OFF_SVG,
            Self::Chess => CHESS_SVG,
            Self::ChessBishop => CHESS_BISHOP_SVG,
            Self::ChessKing => CHESS_KING_SVG,
            Self::ChessKnight => CHESS_KNIGHT_SVG,
            Self::ChessQueen => CHESS_QUEEN_SVG,
            Self::ChessRook => CHESS_ROOK_SVG,
            Self::Clef => CLEF_SVG,
            Self::ClefStaff => CLEF_STAFF_SVG,
            Self::CliffJumping => CLIFF_JUMPING_SVG,
            Self::Cricket => CRICKET_SVG,
            Self::Curling => CURLING_SVG,
            Self::Dice => DICE_SVG,
            Self::Dice1 => DICE_1_SVG,
            Self::Dice2 => DICE_2_SVG,
            Self::Dice3 => DICE_3_SVG,
            Self::Dice4 => DICE_4_SVG,
            Self::Dice5 => DICE_5_SVG,
            Self::Dice6 => DICE_6_SVG,
            Self::DiscGolf => DISC_GOLF_SVG,
            Self::Dumbbell => DUMBBELL_SVG,
            Self::ExerciseBall => EXERCISE_BALL_SVG,
            Self::Ghost => GHOST_SVG,
            Self::Ghost2 => GHOST_2_SVG,
            Self::Ghost3 => GHOST_3_SVG,
            Self::GhostOff => GHOST_OFF_SVG,
            Self::GoGame => GO_GAME_SVG,
            Self::Golf => GOLF_SVG,
            Self::GolfOff => GOLF_OFF_SVG,
            Self::Headphones => HEADPHONES_SVG,
            Self::HeadphonesOff => HEADPHONES_OFF_SVG,
            Self::Headset => HEADSET_SVG,
            Self::HeadsetOff => HEADSET_OFF_SVG,
            Self::Helmet => HELMET_SVG,
            Self::HelmetOff => HELMET_OFF_SVG,
            Self::HorseToy => HORSE_TOY_SVG,
            Self::HulaHoop => HULA_HOOP_SVG,
            Self::IceSkating => ICE_SKATING_SVG,
            Self::Joker => JOKER_SVG,
            Self::JumpRope => JUMP_ROPE_SVG,
            Self::Karate => KARATE_SVG,
            Self::Kayak => KAYAK_SVG,
            Self::Keyframe => KEYFRAME_SVG,
            Self::KeyframeAlignCenter => KEYFRAME_ALIGN_CENTER_SVG,
            Self::KeyframeAlignHorizontal => KEYFRAME_ALIGN_HORIZONTAL_SVG,
            Self::KeyframeAlignVertical => KEYFRAME_ALIGN_VERTICAL_SVG,
            Self::Keyframes => KEYFRAMES_SVG,
            Self::Maximize => MAXIMIZE_SVG,
            Self::MaximizeOff => MAXIMIZE_OFF_SVG,
            Self::Medal => MEDAL_SVG,
            Self::Medal2 => MEDAL_2_SVG,
            Self::Meeple => MEEPLE_SVG,
            Self::Microphone => MICROPHONE_SVG,
            Self::Microphone2 => MICROPHONE_2_SVG,
            Self::Microphone2Off => MICROPHONE_2_OFF_SVG,
            Self::MicrophoneOff => MICROPHONE_OFF_SVG,
            Self::Minimize => MINIMIZE_SVG,
            Self::Movie => MOVIE_SVG,
            Self::MovieOff => MOVIE_OFF_SVG,
            Self::Music => MUSIC_SVG,
            Self::MusicBolt => MUSIC_BOLT_SVG,
            Self::MusicCancel => MUSIC_CANCEL_SVG,
            Self::MusicCheck => MUSIC_CHECK_SVG,
            Self::MusicCode => MUSIC_CODE_SVG,
            Self::MusicCog => MUSIC_COG_SVG,
            Self::MusicDollar => MUSIC_DOLLAR_SVG,
            Self::MusicDown => MUSIC_DOWN_SVG,
            Self::MusicExclamation => MUSIC_EXCLAMATION_SVG,
            Self::MusicHeart => MUSIC_HEART_SVG,
            Self::MusicMinus => MUSIC_MINUS_SVG,
            Self::MusicOff => MUSIC_OFF_SVG,
            Self::MusicPause => MUSIC_PAUSE_SVG,
            Self::MusicPin => MUSIC_PIN_SVG,
            Self::MusicPlus => MUSIC_PLUS_SVG,
            Self::MusicQuestion => MUSIC_QUESTION_SVG,
            Self::MusicSearch => MUSIC_SEARCH_SVG,
            Self::MusicShare => MUSIC_SHARE_SVG,
            Self::MusicStar => MUSIC_STAR_SVG,
            Self::MusicUp => MUSIC_UP_SVG,
            Self::MusicX => MUSIC_X_SVG,
            Self::OlympicTorch => OLYMPIC_TORCH_SVG,
            Self::Olympics => OLYMPICS_SVG,
            Self::OlympicsOff => OLYMPICS_OFF_SVG,
            Self::Pacman => PACMAN_SVG,
            Self::Photo => PHOTO_SVG,
            Self::PhotoAi => PHOTO_AI_SVG,
            Self::PhotoAlt => PHOTO_ALT_SVG,
            Self::PhotoBolt => PHOTO_BOLT_SVG,
            Self::PhotoCancel => PHOTO_CANCEL_SVG,
            Self::PhotoCheck => PHOTO_CHECK_SVG,
            Self::PhotoCircleMinus => PHOTO_CIRCLE_MINUS_SVG,
            Self::PhotoCirclePlus => PHOTO_CIRCLE_PLUS_SVG,
            Self::PhotoCode => PHOTO_CODE_SVG,
            Self::PhotoCog => PHOTO_COG_SVG,
            Self::PhotoDollar => PHOTO_DOLLAR_SVG,
            Self::PhotoDown => PHOTO_DOWN_SVG,
            Self::PhotoEdit => PHOTO_EDIT_SVG,
            Self::PhotoExclamation => PHOTO_EXCLAMATION_SVG,
            Self::PhotoHeart => PHOTO_HEART_SVG,
            Self::PhotoMinus => PHOTO_MINUS_SVG,
            Self::PhotoOff => PHOTO_OFF_SVG,
            Self::PhotoPause => PHOTO_PAUSE_SVG,
            Self::PhotoPin => PHOTO_PIN_SVG,
            Self::PhotoPlus => PHOTO_PLUS_SVG,
            Self::PhotoQuestion => PHOTO_QUESTION_SVG,
            Self::PhotoSearch => PHOTO_SEARCH_SVG,
            Self::PhotoShare => PHOTO_SHARE_SVG,
            Self::PhotoShield => PHOTO_SHIELD_SVG,
            Self::PhotoSpark => PHOTO_SPARK_SVG,
            Self::PhotoStar => PHOTO_STAR_SVG,
            Self::PhotoUp => PHOTO_UP_SVG,
            Self::PhotoX => PHOTO_X_SVG,
            Self::Piano => PIANO_SVG,
            Self::Pick => PICK_SVG,
            Self::PictureInPicture => PICTURE_IN_PICTURE_SVG,
            Self::PictureInPictureOff => PICTURE_IN_PICTURE_OFF_SVG,
            Self::PictureInPictureOn => PICTURE_IN_PICTURE_ON_SVG,
            Self::PictureInPictureTop => PICTURE_IN_PICTURE_TOP_SVG,
            Self::PingPong => PING_PONG_SVG,
            Self::PlayBasketball => PLAY_BASKETBALL_SVG,
            Self::PlayCard => PLAY_CARD_SVG,
            Self::PlayCard1 => PLAY_CARD_1_SVG,
            Self::PlayCard10 => PLAY_CARD_10_SVG,
            Self::PlayCard2 => PLAY_CARD_2_SVG,
            Self::PlayCard3 => PLAY_CARD_3_SVG,
            Self::PlayCard4 => PLAY_CARD_4_SVG,
            Self::PlayCard5 => PLAY_CARD_5_SVG,
            Self::PlayCard6 => PLAY_CARD_6_SVG,
            Self::PlayCard7 => PLAY_CARD_7_SVG,
            Self::PlayCard8 => PLAY_CARD_8_SVG,
            Self::PlayCard9 => PLAY_CARD_9_SVG,
            Self::PlayCardA => PLAY_CARD_A_SVG,
            Self::PlayCardJ => PLAY_CARD_J_SVG,
            Self::PlayCardK => PLAY_CARD_K_SVG,
            Self::PlayCardOff => PLAY_CARD_OFF_SVG,
            Self::PlayCardQ => PLAY_CARD_Q_SVG,
            Self::PlayCardStar => PLAY_CARD_STAR_SVG,
            Self::PlayFootball => PLAY_FOOTBALL_SVG,
            Self::PlayHandball => PLAY_HANDBALL_SVG,
            Self::PlayVolleyball => PLAY_VOLLEYBALL_SVG,
            Self::PlayerEject => PLAYER_EJECT_SVG,
            Self::PlayerPause => PLAYER_PAUSE_SVG,
            Self::PlayerPlay => PLAYER_PLAY_SVG,
            Self::PlayerRecord => PLAYER_RECORD_SVG,
            Self::PlayerSkipBack => PLAYER_SKIP_BACK_SVG,
            Self::PlayerSkipForward => PLAYER_SKIP_FORWARD_SVG,
            Self::PlayerStop => PLAYER_STOP_SVG,
            Self::PlayerTrackNext => PLAYER_TRACK_NEXT_SVG,
            Self::PlayerTrackPrev => PLAYER_TRACK_PREV_SVG,
            Self::Playlist => PLAYLIST_SVG,
            Self::PlaylistOff => PLAYLIST_OFF_SVG,
            Self::PlaylistX => PLAYLIST_X_SVG,
            Self::PokerChip => POKER_CHIP_SVG,
            Self::Pool => POOL_SVG,
            Self::PoolOff => POOL_OFF_SVG,
            Self::PumpkinScary => PUMPKIN_SCARY_SVG,
            Self::Puzzle => PUZZLE_SVG,
            Self::Puzzle2 => PUZZLE_2_SVG,
            Self::PuzzleOff => PUZZLE_OFF_SVG,
            Self::Radio => RADIO_SVG,
            Self::RadioOff => RADIO_OFF_SVG,
            Self::Repeat => REPEAT_SVG,
            Self::RepeatOff => REPEAT_OFF_SVG,
            Self::RepeatOnce => REPEAT_ONCE_SVG,
            Self::RewindBackward10 => REWIND_BACKWARD_10_SVG,
            Self::RewindBackward15 => REWIND_BACKWARD_15_SVG,
            Self::RewindBackward20 => REWIND_BACKWARD_20_SVG,
            Self::RewindBackward30 => REWIND_BACKWARD_30_SVG,
            Self::RewindBackward40 => REWIND_BACKWARD_40_SVG,
            Self::RewindBackward5 => REWIND_BACKWARD_5_SVG,
            Self::RewindBackward50 => REWIND_BACKWARD_50_SVG,
            Self::RewindBackward60 => REWIND_BACKWARD_60_SVG,
            Self::RewindForward10 => REWIND_FORWARD_10_SVG,
            Self::RewindForward15 => REWIND_FORWARD_15_SVG,
            Self::RewindForward20 => REWIND_FORWARD_20_SVG,
            Self::RewindForward30 => REWIND_FORWARD_30_SVG,
            Self::RewindForward40 => REWIND_FORWARD_40_SVG,
            Self::RewindForward5 => REWIND_FORWARD_5_SVG,
            Self::RewindForward50 => REWIND_FORWARD_50_SVG,
            Self::RewindForward60 => REWIND_FORWARD_60_SVG,
            Self::Rings => RINGS_SVG,
            Self::Robot => ROBOT_SVG,
            Self::RollerSkating => ROLLER_SKATING_SVG,
            Self::Roulette => ROULETTE_SVG,
            Self::Rugby => RUGBY_SVG,
            Self::Run => RUN_SVG,
            Self::Scoreboard => SCOREBOARD_SVG,
            Self::ScubaDiving => SCUBA_DIVING_SVG,
            Self::ScubaDivingTank => SCUBA_DIVING_TANK_SVG,
            Self::ScubaMask => SCUBA_MASK_SVG,
            Self::ScubaMaskOff => SCUBA_MASK_OFF_SVG,
            Self::Shareplay => SHAREPLAY_SVG,
            Self::Skateboarding => SKATEBOARDING_SVG,
            Self::SkiJumping => SKI_JUMPING_SVG,
            Self::SoccerField => SOCCER_FIELD_SVG,
            Self::Speakerphone => SPEAKERPHONE_SVG,
            Self::SportBillard => SPORT_BILLARD_SVG,
            Self::Stretching => STRETCHING_SVG,
            Self::Stretching2 => STRETCHING_2_SVG,
            Self::Subtitles => SUBTITLES_SVG,
            Self::SubtitlesAi => SUBTITLES_AI_SVG,
            Self::SubtitlesEdit => SUBTITLES_EDIT_SVG,
            Self::SubtitlesOff => SUBTITLES_OFF_SVG,
            Self::Swimming => SWIMMING_SVG,
            Self::Sword => SWORD_SVG,
            Self::SwordOff => SWORD_OFF_SVG,
            Self::Swords => SWORDS_SVG,
            Self::Target2 => TARGET_2_SVG,
            Self::TargetArrow => TARGET_ARROW_SVG,
            Self::TicTac => TIC_TAC_SVG,
            Self::Tournament => TOURNAMENT_SVG,
            Self::Treadmill => TREADMILL_SVG,
            Self::Trekking => TREKKING_SVG,
            Self::Trophy => TROPHY_SVG,
            Self::TrophyOff => TROPHY_OFF_SVG,
            Self::Ufo => UFO_SVG,
            Self::UserScreen => USER_SCREEN_SVG,
            Self::Video => VIDEO_SVG,
            Self::VideoMinus => VIDEO_MINUS_SVG,
            Self::VideoOff => VIDEO_OFF_SVG,
            Self::VideoPlus => VIDEO_PLUS_SVG,
            Self::Volume => VOLUME_SVG,
            Self::Volume2 => VOLUME_2_SVG,
            Self::Volume3 => VOLUME_3_SVG,
            Self::Volume4 => VOLUME_4_SVG,
            Self::VolumeOff => VOLUME_OFF_SVG,
            Self::Walk => WALK_SVG,
            Self::Waterpolo => WATERPOLO_SVG,
            Self::Yoga => YOGA_SVG,
        }
    }

    fn filled_svg(&self) -> Option<&'static str> {
        // Filled variants would be added here
        None
    }
}
