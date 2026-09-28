//! Design icons from Tabler Icons.
//!
//! This module contains 682 icons.

use crate::tabler::TablerIconData;

// SVG Constants
const AD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M7 15v-4a2 2 0 0 1 4 0v4" /> <path d="M7 13l4 0" /> <path d="M17 9v6h-1.5a1.5 1.5 0 1 1 1.5 -1.5" /> </svg>"##;
const AD_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.933 5h-6.933v16h13v-8" /> <path d="M14 17h-5" /> <path d="M9 13h5v-4h-5v4" /> <path d="M15 5v-2" /> <path d="M18 6l2 -2" /> <path d="M19 9h2" /> </svg>"##;
const AD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h10a2 2 0 0 1 2 2v10m-2 2h-14a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2" /> <path d="M7 15v-4a2 2 0 0 1 2 -2m2 2v4" /> <path d="M7 13h4" /> <path d="M17 9v4" /> <path d="M16.115 12.131c.33 .149 .595 .412 .747 .74" /> <path d="M3 3l18 18" /> </svg>"##;
const ANGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 19h-18l9 -15" /> <path d="M20.615 15.171h.015" /> <path d="M19.515 11.771h.015" /> <path d="M17.715 8.671h.015" /> <path d="M15.415 5.971h.015" /> </svg>"##;
const APERTURE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M3.6 15h10.55" /> <path d="M6.551 4.938l3.26 10.034" /> <path d="M17.032 4.636l-8.535 6.201" /> <path d="M20.559 14.51l-8.535 -6.201" /> <path d="M12.257 20.916l3.261 -10.034" /> </svg>"##;
const APERTURE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.6 15h10.55" /> <path d="M5.641 5.631a9 9 0 1 0 12.719 12.738m1.68 -2.318a9 9 0 0 0 -12.074 -12.098" /> <path d="M7.395 7.534l2.416 7.438" /> <path d="M17.032 4.636l-4.852 3.526m-2.334 1.695l-1.349 .98" /> <path d="M20.559 14.51l-8.535 -6.201" /> <path d="M12.257 20.916l2.123 -6.533m.984 -3.028l.154 -.473" /> <path d="M3 3l18 18" /> </svg>"##;
const ARTBOARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1l0 -6" /> <path d="M3 8l1 0" /> <path d="M3 16l1 0" /> <path d="M8 3l0 1" /> <path d="M16 3l0 1" /> <path d="M20 8l1 0" /> <path d="M20 16l1 0" /> <path d="M8 20l0 1" /> <path d="M16 20l0 1" /> </svg>"##;
const ARTBOARD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8h3a1 1 0 0 1 1 1v3" /> <path d="M15.716 15.698a1 1 0 0 1 -.716 .302h-6a1 1 0 0 1 -1 -1v-6c0 -.273 .11 -.52 .287 -.7" /> <path d="M3 8h1" /> <path d="M3 16h1" /> <path d="M8 3v1" /> <path d="M16 3v1" /> <path d="M20 8h1" /> <path d="M20 16h1" /> <path d="M8 20v1" /> <path d="M16 20v1" /> <path d="M3 3l18 18" /> </svg>"##;
const AWARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 9a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" /> <path d="M12 15l3.4 5.89l1.598 -3.233l3.598 .232l-3.4 -5.889" /> <path d="M6.802 12l-3.4 5.89l3.598 -.233l1.598 3.232l3.4 -5.889" /> </svg>"##;
const AWARD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.72 12.704a6 6 0 0 0 -8.433 -8.418m-1.755 2.24a6 6 0 0 0 7.936 7.944" /> <path d="M12 15l3.4 5.89l1.598 -3.233l.707 .046m1.108 -2.902l-1.617 -2.8" /> <path d="M6.802 12l-3.4 5.89l3.598 -.233l1.598 3.232l3.4 -5.889" /> <path d="M3 3l18 18" /> </svg>"##;
const BACKGROUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8l4 -4" /> <path d="M14 4l-10 10" /> <path d="M4 20l16 -16" /> <path d="M20 10l-10 10" /> <path d="M20 16l-4 4" /> </svg>"##;
const BADGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 17v-13l-5 3l-5 -3v13l5 3l5 -3" /> </svg>"##;
const BADGE_2K_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -10" /> <path d="M14 9v6" /> <path d="M17 9l-2 3l2 3" /> <path d="M15 12h-1" /> <path d="M7 9h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2" /> </svg>"##;
const BADGE_3D_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -10" /> <path d="M7 9h1.5a1.5 1.5 0 0 1 0 3h-.5h.5a1.5 1.5 0 0 1 0 3h-1.5" /> <path d="M14 9v6h1a2 2 0 0 0 2 -2v-2a2 2 0 0 0 -2 -2l-1 0" /> </svg>"##;
const BADGE_3K_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -10" /> <path d="M14 9v6" /> <path d="M17 9l-2 3l2 3" /> <path d="M15 12h-1" /> <path d="M7 9.5a.5 .5 0 0 1 .5 -.5h1a1.5 1.5 0 0 1 0 3h-.5h.5a1.5 1.5 0 0 1 0 3h-1a.5 .5 0 0 1 -.5 -.5" /> </svg>"##;
const BADGE_4K_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M7 9v2a1 1 0 0 0 1 1h1" /> <path d="M10 9v6" /> <path d="M14 9v6" /> <path d="M17 9l-2 3l2 3" /> <path d="M15 12h-1" /> </svg>"##;
const BADGE_5K_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -10" /> <path d="M14 9v6" /> <path d="M17 9l-2 3l2 3" /> <path d="M15 12h-1" /> <path d="M7 15h2a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2v-3h3" /> </svg>"##;
const BADGE_8K_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -10" /> <path d="M14 9v6" /> <path d="M17 9l-2 3l2 3" /> <path d="M15 12h-1" /> <path d="M8.5 12h-.5a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h1a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1" /> </svg>"##;
const BADGE_AD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M14 9v6h1a2 2 0 0 0 2 -2v-2a2 2 0 0 0 -2 -2h-1" /> <path d="M7 15v-4.5a1.5 1.5 0 0 1 3 0v4.5" /> <path d="M7 13h3" /> </svg>"##;
const BADGE_AD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h10a2 2 0 0 1 2 2v10m-2 2h-14a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2" /> <path d="M14 14v1h1m2 -2v-2a2 2 0 0 0 -2 -2h-1v1" /> <path d="M7 15v-4.5a1.5 1.5 0 0 1 2.077 -1.385m.788 .762c.087 .19 .135 .4 .135 .623v4.5" /> <path d="M7 13h3" /> <path d="M3 3l18 18" /> </svg>"##;
const BADGE_AR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M7 15v-4.5a1.5 1.5 0 0 1 3 0v4.5" /> <path d="M7 13h3" /> <path d="M14 12h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6m3 0l-2 -3" /> </svg>"##;
const BADGE_CC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M10 10.5a1.5 1.5 0 0 0 -3 0v3a1.5 1.5 0 0 0 3 0" /> <path d="M17 10.5a1.5 1.5 0 0 0 -3 0v3a1.5 1.5 0 0 0 3 0" /> </svg>"##;
const BADGE_HD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M14 9v6h1a2 2 0 0 0 2 -2v-2a2 2 0 0 0 -2 -2h-1" /> <path d="M7 15v-6" /> <path d="M10 15v-6" /> <path d="M7 12h3" /> </svg>"##;
const BADGE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7v10l5 3l5 -3m0 -4v-9l-5 3l-2.496 -1.497" /> <path d="M3 3l18 18" /> </svg>"##;
const BADGE_SD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M14 9v6h1a2 2 0 0 0 2 -2v-2a2 2 0 0 0 -2 -2h-1" /> <path d="M7 14.25c0 .414 .336 .75 .75 .75h1.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h1.25a.75 .75 0 0 1 .75 .75" /> </svg>"##;
const BADGE_TM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M6 9h4" /> <path d="M8 9v6" /> <path d="M13 15v-6l2 3l2 -3v6" /> </svg>"##;
const BADGE_VO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M7 9l2 6l2 -6" /> <path d="M15.5 9a1.5 1.5 0 0 1 1.5 1.5v3a1.5 1.5 0 0 1 -3 0v-3a1.5 1.5 0 0 1 1.5 -1.5" /> </svg>"##;
const BADGE_VR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M14 12h1.5a1.5 1.5 0 0 0 0 -3h-1.5v6m3 0l-2 -3" /> <path d="M7 9l2 6l2 -6" /> </svg>"##;
const BADGE_WC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M6.5 9l.5 6l2 -4l2 4l.5 -6" /> <path d="M17 10.5a1.5 1.5 0 0 0 -3 0v3a1.5 1.5 0 0 0 3 0" /> </svg>"##;
const BADGES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 17v-4l-5 3l-5 -3v4l5 3l5 -3" /> <path d="M17 8v-4l-5 3l-5 -3v4l5 3l5 -3" /> </svg>"##;
const BADGES_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.505 14.497l-2.505 1.503l-5 -3v4l5 3l5 -3" /> <path d="M13.873 9.876l3.127 -1.876v-4l-5 3l-2.492 -1.495m-2.508 1.495v1l2.492 1.495" /> <path d="M3 3l18 18" /> </svg>"##;
const BARREL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.278 4h9.444a2 2 0 0 1 1.841 1.22c.958 2.26 1.437 4.52 1.437 6.78c0 2.26 -.479 4.52 -1.437 6.78a2 2 0 0 1 -1.841 1.22h-9.444a2 2 0 0 1 -1.841 -1.22c-.958 -2.26 -1.437 -4.52 -1.437 -6.78c0 -2.26 .479 -4.52 1.437 -6.78a2 2 0 0 1 1.841 -1.22" /> <path d="M14 4c.667 2.667 1 5.333 1 8s-.333 5.333 -1 8" /> <path d="M10 4c-.667 2.667 -1 5.333 -1 8s.333 5.333 1 8" /> <path d="M4.5 16h15" /> <path d="M19.5 8h-15" /> </svg>"##;
const BARREL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h8.722a2 2 0 0 1 1.841 1.22c.958 2.26 1.437 4.52 1.437 6.78a16.35 16.35 0 0 1 -.407 3.609m-.964 3.013l-.066 .158a2 2 0 0 1 -1.841 1.22h-9.444a2 2 0 0 1 -1.841 -1.22c-.958 -2.26 -1.437 -4.52 -1.437 -6.78c0 -2.21 .458 -4.42 1.374 -6.63" /> <path d="M14 4c.585 2.337 .913 4.674 .985 7.01m-.114 3.86a33.415 33.415 0 0 1 -.871 5.13" /> <path d="M10 4a34.42 34.42 0 0 0 -.366 1.632m-.506 3.501a32.126 32.126 0 0 0 -.128 2.867c0 2.667 .333 5.333 1 8" /> <path d="M4.5 16h11.5" /> <path d="M19.5 8h-7.5m-4 0h-3.5" /> <path d="M3 3l18 18" /> </svg>"##;
const BARRIER_BLOCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1v7a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1l0 -7" /> <path d="M7 16v4" /> <path d="M7.5 16l9 -9" /> <path d="M13.5 16l6.5 -6.5" /> <path d="M4 13.5l6.5 -6.5" /> <path d="M17 16v4" /> <path d="M5 20h4" /> <path d="M15 20h4" /> <path d="M17 7v-2" /> <path d="M7 7v-2" /> </svg>"##;
const BARRIER_BLOCK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 7h8a1 1 0 0 1 1 1v7c0 .27 -.107 .516 -.282 .696" /> <path d="M16 16h-11a1 1 0 0 1 -1 -1v-7a1 1 0 0 1 1 -1h2" /> <path d="M7 16v4" /> <path d="M7.5 16l4.244 -4.244" /> <path d="M13.745 9.755l2.755 -2.755" /> <path d="M13.5 16l1.249 -1.249" /> <path d="M16.741 12.759l3.259 -3.259" /> <path d="M4 13.5l4.752 -4.752" /> <path d="M17 17v3" /> <path d="M5 20h4" /> <path d="M15 20h4" /> <path d="M17 7v-2" /> <path d="M3 3l18 18" /> </svg>"##;
const BINOCULARS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 16a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M14 16a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M16.346 9.17l-.729 -1.261c-.16 -.248 -1.056 -.203 -1.117 .091l-.177 1.38" /> <path d="M19.761 14.813l-2.84 -5.133c-.189 -.31 -.592 -.68 -1.421 -.68c-.828 0 -1.5 .448 -1.5 1v6" /> <path d="M7.654 9.17l.729 -1.261c.16 -.249 1.056 -.203 1.117 .091l.177 1.38" /> <path d="M4.239 14.813l2.84 -5.133c.189 -.31 .592 -.68 1.421 -.68c.828 0 1.5 .448 1.5 1v6" /> <path d="M10 12h4v2h-4l0 -2" /> </svg>"##;
const BLADE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.707 3.707l2.586 2.586a1 1 0 0 1 0 1.414l-.586 .586a1 1 0 0 0 0 1.414l.586 .586a1 1 0 0 1 0 1.414l-8.586 8.586a1 1 0 0 1 -1.414 0l-.586 -.586a1 1 0 0 0 -1.414 0l-.586 .586a1 1 0 0 1 -1.414 0l-2.586 -2.586a1 1 0 0 1 0 -1.414l.586 -.586a1 1 0 0 0 0 -1.414l-.586 -.586a1 1 0 0 1 0 -1.414l8.586 -8.586a1 1 0 0 1 1.414 0l.586 .586a1 1 0 0 0 1.414 0l.586 -.586a1 1 0 0 1 1.414 0" /> <path d="M8 16l3.2 -3.2" /> <path d="M12.8 11.2l3.2 -3.2" /> <path d="M14 8l2 2" /> <path d="M8 14l2 2" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const BLEND_MODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 9.5a6.5 6.5 0 1 0 13 0a6.5 6.5 0 1 0 -13 0" /> <path d="M3 14.5a6.5 6.5 0 1 0 13 0a6.5 6.5 0 1 0 -13 0" /> </svg>"##;
const BLUR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9.01 9.01 0 0 0 2.32 -.302a9 9 0 0 0 1.74 -16.733a9 9 0 1 0 -4.06 17.035" /> <path d="M12 3v17" /> <path d="M12 12h9" /> <path d="M12 9h8" /> <path d="M12 6h6" /> <path d="M12 18h6" /> <path d="M12 15h8" /> </svg>"##;
const BLUR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3v5m0 4v8" /> <path d="M5.641 5.631a9 9 0 1 0 12.719 12.738m1.68 -2.318a9 9 0 0 0 -12.074 -12.098" /> <path d="M16 12h5" /> <path d="M13 9h7" /> <path d="M12 6h6" /> <path d="M12 18h6" /> <path d="M12 15h3m4 0h1" /> <path d="M3 3l18 18" /> </svg>"##;
const BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 3l0 7l6 0l-8 11l0 -7l-6 0l8 -11" /> </svg>"##;
const BOLT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M15.212 15.21l-4.212 5.79v-7h-6l3.79 -5.21m1.685 -2.32l2.525 -3.47v6m1 1h5l-2.104 2.893" /> </svg>"##;
const BOMB_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.349 5.349l3.301 3.301a1.2 1.2 0 0 1 0 1.698l-.972 .972a7.5 7.5 0 1 1 -5 -5l.972 -.972a1.2 1.2 0 0 1 1.698 0l.001 .001" /> <path d="M17 7l1.293 -1.293a2.414 2.414 0 0 0 .707 -1.707a1 1 0 0 1 1 -1h1" /> <path d="M7 13a3 3 0 0 1 3 -3" /> </svg>"##;
const BONG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 3v8.416c.134 .059 .265 .123 .393 .193l3.607 -3.609l2 2l-3.608 3.608a5 5 0 1 1 -6.392 -2.192v-8.416h4" /> <path d="M8 3h6" /> <path d="M6.1 17h9.8" /> </svg>"##;
const BONG_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5v-2h4v6m1.5 1.5l2.5 -2.5l2 2l-2.5 2.5m-.5 3.505a5 5 0 1 1 -7 -4.589v-2.416" /> <path d="M8 3h6" /> <path d="M6.1 17h9.8" /> <path d="M3 3l18 18" /> </svg>"##;
const BOOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9.662c2 2.338 2 4.338 0 6.338c3 .5 4.5 1 5 4c2 -3 6 -4 9 0c0 -3 1 -4 4 -4.004q -3 -2.995 0 -5.996c-3 0 -5 -2 -5 -5c-2 4 -5 3 -7.5 -1c-.5 3 -2.5 5 -5.5 5.662" /> </svg>"##;
const BORDER_ALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M4 12l16 0" /> <path d="M12 4l0 16" /> </svg>"##;
const BORDER_BOTTOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 20l-16 0" /> <path d="M4 4l0 .01" /> <path d="M8 4l0 .01" /> <path d="M12 4l0 .01" /> <path d="M16 4l0 .01" /> <path d="M20 4l0 .01" /> <path d="M4 8l0 .01" /> <path d="M12 8l0 .01" /> <path d="M20 8l0 .01" /> <path d="M4 12l0 .01" /> <path d="M8 12l0 .01" /> <path d="M12 12l0 .01" /> <path d="M16 12l0 .01" /> <path d="M20 12l0 .01" /> <path d="M4 16l0 .01" /> <path d="M12 16l0 .01" /> <path d="M20 16l0 .01" /> </svg>"##;
const BORDER_BOTTOM_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h16" /> <path d="M4 16v-.01" /> <path d="M20 16v-.01" /> <path d="M4 12v-.01" /> <path d="M20 12v-.01" /> <path d="M4 8v-.01" /> <path d="M20 8v-.01" /> <path d="M4 4v-.01" /> <path d="M8 4v-.01" /> <path d="M12 4v-.01" /> <path d="M16 4v-.01" /> <path d="M20 4v-.01" /> <path d="M15 12h-6" /> <path d="M12 9v6" /> </svg>"##;
const BORDER_CORNER_IOS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20c0 -6.559 0 -9.838 1.628 -12.162a9 9 0 0 1 2.21 -2.21c2.324 -1.628 5.602 -1.628 12.162 -1.628" /> </svg>"##;
const BORDER_CORNER_PILL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20v-5c0 -6.075 4.925 -11 11 -11h5" /> </svg>"##;
const BORDER_CORNER_ROUNDED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20v-10a6 6 0 0 1 6 -6h10" /> </svg>"##;
const BORDER_CORNER_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20v-15a1 1 0 0 1 1 -1h15" /> </svg>"##;
const BORDER_CORNERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M20 16v2a2 2 0 0 1 -2 2h-2" /> <path d="M8 20h-2a2 2 0 0 1 -2 -2v-2" /> <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> </svg>"##;
const BORDER_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12l16 0" /> <path d="M4 4l0 .01" /> <path d="M8 4l0 .01" /> <path d="M12 4l0 .01" /> <path d="M16 4l0 .01" /> <path d="M20 4l0 .01" /> <path d="M4 8l0 .01" /> <path d="M12 8l0 .01" /> <path d="M20 8l0 .01" /> <path d="M4 16l0 .01" /> <path d="M12 16l0 .01" /> <path d="M20 16l0 .01" /> <path d="M4 20l0 .01" /> <path d="M8 20l0 .01" /> <path d="M12 20l0 .01" /> <path d="M16 20l0 .01" /> <path d="M20 20l0 .01" /> </svg>"##;
const BORDER_INNER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12l16 0" /> <path d="M12 4l0 16" /> <path d="M4 4l0 .01" /> <path d="M8 4l0 .01" /> <path d="M16 4l0 .01" /> <path d="M20 4l0 .01" /> <path d="M4 8l0 .01" /> <path d="M20 8l0 .01" /> <path d="M4 16l0 .01" /> <path d="M20 16l0 .01" /> <path d="M4 20l0 .01" /> <path d="M8 20l0 .01" /> <path d="M16 20l0 .01" /> <path d="M20 20l0 .01" /> </svg>"##;
const BORDER_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20l0 -16" /> <path d="M8 4l0 .01" /> <path d="M12 4l0 .01" /> <path d="M16 4l0 .01" /> <path d="M20 4l0 .01" /> <path d="M12 8l0 .01" /> <path d="M20 8l0 .01" /> <path d="M8 12l0 .01" /> <path d="M12 12l0 .01" /> <path d="M16 12l0 .01" /> <path d="M20 12l0 .01" /> <path d="M12 16l0 .01" /> <path d="M20 16l0 .01" /> <path d="M8 20l0 .01" /> <path d="M12 20l0 .01" /> <path d="M16 20l0 .01" /> <path d="M20 20l0 .01" /> </svg>"##;
const BORDER_LEFT_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20v-16" /> <path d="M8 4v.01" /> <path d="M12 4v.01" /> <path d="M16 4v.01" /> <path d="M20 4v.01" /> <path d="M20 8v.01" /> <path d="M20 12v.01" /> <path d="M20 16v.01" /> <path d="M8 20v.01" /> <path d="M12 20v.01" /> <path d="M16 20v.01" /> <path d="M20 20v.01" /> <path d="M9 12h6" /> <path d="M12 9v6" /> </svg>"##;
const BORDER_NONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4l0 .01" /> <path d="M8 4l0 .01" /> <path d="M12 4l0 .01" /> <path d="M16 4l0 .01" /> <path d="M20 4l0 .01" /> <path d="M4 8l0 .01" /> <path d="M12 8l0 .01" /> <path d="M20 8l0 .01" /> <path d="M4 12l0 .01" /> <path d="M8 12l0 .01" /> <path d="M12 12l0 .01" /> <path d="M16 12l0 .01" /> <path d="M20 12l0 .01" /> <path d="M4 16l0 .01" /> <path d="M12 16l0 .01" /> <path d="M20 16l0 .01" /> <path d="M4 20l0 .01" /> <path d="M8 20l0 .01" /> <path d="M12 20l0 .01" /> <path d="M16 20l0 .01" /> <path d="M20 20l0 .01" /> </svg>"##;
const BORDER_OUTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M12 8l0 .01" /> <path d="M8 12l0 .01" /> <path d="M12 12l0 .01" /> <path d="M16 12l0 .01" /> <path d="M12 16l0 .01" /> </svg>"##;
const BORDER_RADIUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12v-4a4 4 0 0 1 4 -4h4" /> <path d="M16 4l0 .01" /> <path d="M20 4l0 .01" /> <path d="M20 8l0 .01" /> <path d="M20 12l0 .01" /> <path d="M4 16l0 .01" /> <path d="M20 16l0 .01" /> <path d="M4 20l0 .01" /> <path d="M8 20l0 .01" /> <path d="M12 20l0 .01" /> <path d="M16 20l0 .01" /> <path d="M20 20l0 .01" /> </svg>"##;
const BORDER_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 4l0 16" /> <path d="M4 4l0 .01" /> <path d="M8 4l0 .01" /> <path d="M12 4l0 .01" /> <path d="M16 4l0 .01" /> <path d="M4 8l0 .01" /> <path d="M12 8l0 .01" /> <path d="M4 12l0 .01" /> <path d="M8 12l0 .01" /> <path d="M12 12l0 .01" /> <path d="M16 12l0 .01" /> <path d="M4 16l0 .01" /> <path d="M12 16l0 .01" /> <path d="M4 20l0 .01" /> <path d="M8 20l0 .01" /> <path d="M12 20l0 .01" /> <path d="M16 20l0 .01" /> </svg>"##;
const BORDER_RIGHT_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 20v-16" /> <path d="M16 4v.01" /> <path d="M12 4v.01" /> <path d="M8 4v.01" /> <path d="M4 4v.01" /> <path d="M4 8v.01" /> <path d="M4 12v.01" /> <path d="M4 16v.01" /> <path d="M16 20v.01" /> <path d="M12 20v.01" /> <path d="M8 20v.01" /> <path d="M4 20v.01" /> <path d="M15 12h-6" /> <path d="M12 9v6" /> </svg>"##;
const BORDER_SIDES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v8" /> <path d="M20 16v-8" /> <path d="M8 4h8" /> <path d="M8 20h8" /> </svg>"##;
const BORDER_STYLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20v-14a2 2 0 0 1 2 -2h14" /> <path d="M20 8v.01" /> <path d="M20 12v.01" /> <path d="M20 16v.01" /> <path d="M8 20v.01" /> <path d="M12 20v.01" /> <path d="M16 20v.01" /> <path d="M20 20v.01" /> </svg>"##;
const BORDER_STYLE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18v.01" /> <path d="M8 18v.01" /> <path d="M12 18v.01" /> <path d="M16 18v.01" /> <path d="M20 18v.01" /> <path d="M18 12h2" /> <path d="M11 12h2" /> <path d="M4 12h2" /> <path d="M4 6h16" /> </svg>"##;
const BORDER_TOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4l16 0" /> <path d="M4 8l0 .01" /> <path d="M12 8l0 .01" /> <path d="M20 8l0 .01" /> <path d="M4 12l0 .01" /> <path d="M8 12l0 .01" /> <path d="M12 12l0 .01" /> <path d="M16 12l0 .01" /> <path d="M20 12l0 .01" /> <path d="M4 16l0 .01" /> <path d="M12 16l0 .01" /> <path d="M20 16l0 .01" /> <path d="M4 20l0 .01" /> <path d="M8 20l0 .01" /> <path d="M12 20l0 .01" /> <path d="M16 20l0 .01" /> <path d="M20 20l0 .01" /> </svg>"##;
const BORDER_TOP_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4h16" /> <path d="M4 8v.01" /> <path d="M20 8v.01" /> <path d="M4 12v.01" /> <path d="M20 12v.01" /> <path d="M4 16v.01" /> <path d="M15 12h-6" /> <path d="M12 9v6" /> <path d="M20 16v.01" /> <path d="M4 20v.01" /> <path d="M8 20v.01" /> <path d="M12 20v.01" /> <path d="M16 20v.01" /> <path d="M20 20v.01" /> </svg>"##;
const BORDER_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4l0 16" /> <path d="M4 4l0 .01" /> <path d="M8 4l0 .01" /> <path d="M16 4l0 .01" /> <path d="M20 4l0 .01" /> <path d="M4 8l0 .01" /> <path d="M20 8l0 .01" /> <path d="M4 12l0 .01" /> <path d="M8 12l0 .01" /> <path d="M16 12l0 .01" /> <path d="M20 12l0 .01" /> <path d="M4 16l0 .01" /> <path d="M20 16l0 .01" /> <path d="M4 20l0 .01" /> <path d="M8 20l0 .01" /> <path d="M16 20l0 .01" /> <path d="M20 20l0 .01" /> </svg>"##;
const BOUNCE_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 15.5c-3 -1 -5.5 -.5 -8 4.5c-.5 -3 -1.5 -5.5 -3 -8" /> <path d="M6 9a2 2 0 1 1 0 -4a2 2 0 0 1 0 4" /> </svg>"##;
const BOUNCE_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 15.5c3 -1 5.5 -.5 8 4.5c.5 -3 1.5 -5.5 3 -8" /> <path d="M18 9a2 2 0 1 1 0 -4a2 2 0 0 1 0 4" /> </svg>"##;
const BOX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3l8 4.5l0 9l-8 4.5l-8 -4.5l0 -9l8 -4.5" /> <path d="M12 12l8 -4.5" /> <path d="M12 12l0 9" /> <path d="M12 12l-8 -4.5" /> </svg>"##;
const BOX_ALIGN_BOTTOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 14h16v5a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1v-5" /> <path d="M4 9v.01" /> <path d="M4 4v.01" /> <path d="M9 4v.01" /> <path d="M15 4v.01" /> <path d="M20 4v.01" /> <path d="M20 9v.01" /> </svg>"##;
const BOX_ALIGN_BOTTOM_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 13h5a1 1 0 0 1 1 1v5a1 1 0 0 1 -1 1h-5a1 1 0 0 1 -1 -1v-5a1 1 0 0 1 1 -1" /> <path d="M4 9v.01" /> <path d="M4 4v.01" /> <path d="M9 4v.01" /> <path d="M15 4v.01" /> <path d="M15 20v.01" /> <path d="M20 4v.01" /> <path d="M20 9v.01" /> <path d="M20 15v.01" /> <path d="M20 20v.01" /> </svg>"##;
const BOX_ALIGN_BOTTOM_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 13h-5a1 1 0 0 0 -1 1v5a1 1 0 0 0 1 1h5a1 1 0 0 0 1 -1v-5a1 1 0 0 0 -1 -1" /> <path d="M20 9v.01" /> <path d="M20 4v.01" /> <path d="M15 4v.01" /> <path d="M9 4v.01" /> <path d="M9 20v.01" /> <path d="M4 4v.01" /> <path d="M4 9v.01" /> <path d="M4 15v.01" /> <path d="M4 20v.01" /> </svg>"##;
const BOX_ALIGN_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.002 20.003v-16h-5a1 1 0 0 0 -1 1v14a1 1 0 0 0 1 1h5" /> <path d="M15.002 20.003h-.01" /> <path d="M20.003 20.003h-.011" /> <path d="M20.003 15.002h-.011" /> <path d="M20.003 9.002h-.011" /> <path d="M20.003 4.002h-.011" /> <path d="M15.002 4.002h-.01" /> </svg>"##;
const BOX_ALIGN_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.998 20.003v-16h5a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-5" /> <path d="M8.998 20.003h.01" /> <path d="M3.997 20.003h.011" /> <path d="M3.997 15.002h.011" /> <path d="M3.997 9.002h.011" /> <path d="M3.997 4.002h.011" /> <path d="M8.998 4.002h.01" /> </svg>"##;
const BOX_ALIGN_TOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10.005h16v-5a1 1 0 0 0 -1 -1h-14a1 1 0 0 0 -1 1v5" /> <path d="M4 15.005v-.01" /> <path d="M4 20.005v-.01" /> <path d="M9 20.005v-.01" /> <path d="M15 20.005v-.01" /> <path d="M20 20.005v-.01" /> <path d="M20 15.005v-.01" /> </svg>"##;
const BOX_ALIGN_TOP_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 5v5a1 1 0 0 1 -1 1h-5a1 1 0 0 1 -1 -1v-5a1 1 0 0 1 1 -1h5a1 1 0 0 1 1 1" /> <path d="M15 4h-.01" /> <path d="M20 4h-.01" /> <path d="M20 9h-.01" /> <path d="M20 15h-.01" /> <path d="M4 15h-.01" /> <path d="M20 20h-.01" /> <path d="M15 20h-.01" /> <path d="M9 20h-.01" /> <path d="M4 20h-.01" /> </svg>"##;
const BOX_ALIGN_TOP_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 11.01h-5a1 1 0 0 1 -1 -1v-5a1 1 0 0 1 1 -1h5a1 1 0 0 1 1 1v5a1 1 0 0 1 -1 1" /> <path d="M20 15.01v-.01" /> <path d="M20 20.01v-.01" /> <path d="M15 20.01v-.01" /> <path d="M9 20.01v-.01" /> <path d="M9 4.01v-.01" /> <path d="M4 20.01v-.01" /> <path d="M4 15.01v-.01" /> <path d="M4 9.01v-.01" /> <path d="M4 4.01v-.01" /> </svg>"##;
const BOX_MARGIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h8v8h-8l0 -8" /> <path d="M4 4v.01" /> <path d="M8 4v.01" /> <path d="M12 4v.01" /> <path d="M16 4v.01" /> <path d="M20 4v.01" /> <path d="M4 20v.01" /> <path d="M8 20v.01" /> <path d="M12 20v.01" /> <path d="M16 20v.01" /> <path d="M20 20v.01" /> <path d="M20 16v.01" /> <path d="M20 12v.01" /> <path d="M20 8v.01" /> <path d="M4 16v.01" /> <path d="M4 12v.01" /> <path d="M4 8v.01" /> </svg>"##;
const BOX_MODEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h8v8h-8l0 -8" /> <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M16 16l3.3 3.3" /> <path d="M16 8l3.3 -3.3" /> <path d="M8 8l-3.3 -3.3" /> <path d="M8 16l-3.3 3.3" /> </svg>"##;
const BOX_MODEL_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h8v8h-8l0 -8" /> <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> </svg>"##;
const BOX_MODEL_2_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h10a2 2 0 0 1 2 2v10m-.586 3.414a2 2 0 0 1 -1.414 .586h-12a2 2 0 0 1 -2 -2v-12c0 -.547 .22 -1.043 .576 -1.405" /> <path d="M12 8h4v4m0 4h-8v-8" /> <path d="M3 3l18 18" /> </svg>"##;
const BOX_MODEL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8h4v4m0 4h-8v-8" /> <path d="M8 4h10a2 2 0 0 1 2 2v10m-.586 3.414a2 2 0 0 1 -1.414 .586h-12a2 2 0 0 1 -2 -2v-12c0 -.547 .22 -1.043 .576 -1.405" /> <path d="M16 16l3.3 3.3" /> <path d="M16 8l3.3 -3.3" /> <path d="M8 8l-3.3 -3.3" /> <path d="M8 16l-3.3 3.3" /> <path d="M3 3l18 18" /> </svg>"##;
const BOX_MULTIPLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -10" /> <path d="M17 17v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h2" /> </svg>"##;
const BOX_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.765 17.757l-5.765 3.243l-8 -4.5v-9l2.236 -1.258m2.57 -1.445l3.194 -1.797l8 4.5v8.5" /> <path d="M14.561 10.559l5.439 -3.059" /> <path d="M12 12v9" /> <path d="M12 12l-8 -4.5" /> <path d="M3 3l18 18" /> </svg>"##;
const BOX_PADDING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M8 16v.01" /> <path d="M8 12v.01" /> <path d="M8 8v.01" /> <path d="M16 16v.01" /> <path d="M16 12v.01" /> <path d="M16 8v.01" /> <path d="M12 8v.01" /> <path d="M12 16v.01" /> </svg>"##;
const BRACKETS_ANGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4l-5 8l5 8" /> <path d="M16 4l5 8l-5 8" /> </svg>"##;
const BRACKETS_ANGLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h.01" /> <path d="M6.453 6.474l-3.453 5.526l5 8" /> <path d="M16 4l5 8l-1.917 3.067" /> <path d="M17.535 17.544l-1.535 2.456" /> <path d="M3 3l18 18" /> </svg>"##;
const BRIEFCASE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v9a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -9" /> <path d="M8 7v-2a2 2 0 0 1 2 -2h4a2 2 0 0 1 2 2v2" /> <path d="M12 12l0 .01" /> <path d="M3 13a20 20 0 0 0 18 0" /> </svg>"##;
const BRIEFCASE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v9a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-9" /> <path d="M8 7v-2a2 2 0 0 1 2 -2h4a2 2 0 0 1 2 2v2" /> </svg>"##;
const BRIEFCASE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 7h8a2 2 0 0 1 2 2v8m-1.166 2.818a1.993 1.993 0 0 1 -.834 .182h-14a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h2" /> <path d="M8.185 4.158a2 2 0 0 1 1.815 -1.158h4a2 2 0 0 1 2 2v2" /> <path d="M12 12v.01" /> <path d="M3 13a20 20 0 0 0 11.905 1.928m3.263 -.763a20 20 0 0 0 2.832 -1.165" /> <path d="M3 3l18 18" /> </svg>"##;
const BRIGHTNESS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 3l0 18" /> <path d="M12 9l4.65 -4.65" /> <path d="M12 14.3l7.37 -7.37" /> <path d="M12 19.6l8.85 -8.85" /> </svg>"##;
const BRIGHTNESS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M6 6h3.5l2.5 -2.5l2.5 2.5h3.5v3.5l2.5 2.5l-2.5 2.5v3.5h-3.5l-2.5 2.5l-2.5 -2.5h-3.5v-3.5l-2.5 -2.5l2.5 -2.5l0 -3.5" /> </svg>"##;
const BRIGHTNESS_AUTO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 6h3.5l2.5 -2.5l2.5 2.5h3.5v3.5l2.5 2.5l-2.5 2.5v3.5h-3.5l-2.5 2.5l-2.5 -2.5h-3.5v-3.5l-2.5 -2.5l2.5 -2.5l0 -3.5" /> <path d="M10 14.5v-3.5a2 2 0 1 1 4 0v3.5" /> <path d="M10 13h4" /> </svg>"##;
const BRIGHTNESS_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M12 5l0 .01" /> <path d="M17 7l0 .01" /> <path d="M19 12l0 .01" /> <path d="M17 17l0 .01" /> <path d="M12 19l0 .01" /> <path d="M7 17l0 .01" /> <path d="M5 12l0 .01" /> <path d="M7 7l0 .01" /> </svg>"##;
const BRIGHTNESS_HALF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9a3 3 0 0 0 0 6v-6" /> <path d="M6 6h3.5l2.5 -2.5l2.5 2.5h3.5v3.5l2.5 2.5l-2.5 2.5v3.5h-3.5l-2.5 2.5l-2.5 -2.5h-3.5v-3.5l-2.5 -2.5l2.5 -2.5l0 -3.5" /> </svg>"##;
const BRIGHTNESS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3v5m0 4v9" /> <path d="M5.641 5.631a9 9 0 1 0 12.719 12.738m1.68 -2.318a9 9 0 0 0 -12.074 -12.098" /> <path d="M12.5 8.5l4.15 -4.15" /> <path d="M12 14l1.025 -.983m2.065 -1.981l4.28 -4.106" /> <path d="M12 19.6l3.79 -3.79m2 -2l3.054 -3.054" /> <path d="M3 3l18 18" /> </svg>"##;
const BRIGHTNESS_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M12 5l0 -2" /> <path d="M17 7l1.4 -1.4" /> <path d="M19 12l2 0" /> <path d="M17 17l1.4 1.4" /> <path d="M12 19l0 2" /> <path d="M7 17l-1.4 1.4" /> <path d="M6 12l-2 0" /> <path d="M7 7l-1.4 -1.4" /> </svg>"##;
const BRUSH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21v-4a4 4 0 1 1 4 4h-4" /> <path d="M21 3a16 16 0 0 0 -12.8 10.2" /> <path d="M21 3a16 16 0 0 1 -10.2 12.8" /> <path d="M10.6 9a9 9 0 0 1 4.4 4.4" /> </svg>"##;
const BRUSH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a4 4 0 1 1 4 4h-4v-4" /> <path d="M21 3a16 16 0 0 0 -9.309 4.704m-1.795 2.212a15.993 15.993 0 0 0 -1.696 3.284" /> <path d="M21 3a16 16 0 0 1 -4.697 9.302m-2.195 1.786a15.993 15.993 0 0 1 -3.308 1.712" /> <path d="M3 3l18 18" /> </svg>"##;
const BUCKET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7a8 4 0 1 0 16 0a8 4 0 1 0 -16 0" /> <path d="M4 7c0 .664 .088 1.324 .263 1.965l2.737 10.035c.5 1.5 2.239 2 5 2s4.5 -.5 5 -2c.333 -1 1.246 -4.345 2.737 -10.035a7.45 7.45 0 0 0 .263 -1.965" /> </svg>"##;
const BUCKET_DROPLET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 16l1.465 1.638a2 2 0 1 1 -3.015 .099l1.55 -1.737" /> <path d="M13.737 9.737c2.299 -2.3 3.23 -5.095 2.081 -6.245c-1.15 -1.15 -3.945 -.217 -6.244 2.082c-2.3 2.299 -3.231 5.095 -2.082 6.244c1.15 1.15 3.946 .218 6.245 -2.081" /> <path d="M7.492 11.818c.362 .362 .768 .676 1.208 .934l6.895 4.047c1.078 .557 2.255 -.075 3.692 -1.512c1.437 -1.437 2.07 -2.614 1.512 -3.692c-.372 -.718 -1.72 -3.017 -4.047 -6.895a6.015 6.015 0 0 0 -.934 -1.208" /> </svg>"##;
const BUCKET_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.029 5.036c-.655 .58 -1.029 1.25 -1.029 1.964c0 2.033 3.033 3.712 6.96 3.967m3.788 -.21c3.064 -.559 5.252 -2.029 5.252 -3.757c0 -2.21 -3.582 -4 -8 -4c-1.605 0 -3.1 .236 -4.352 .643" /> <path d="M4 7c0 .664 .088 1.324 .263 1.965l2.737 10.035c.5 1.5 2.239 2 5 2s4.5 -.5 5 -2c.1 -.3 .252 -.812 .457 -1.535m.862 -3.146c.262 -.975 .735 -2.76 1.418 -5.354a7.45 7.45 0 0 0 .263 -1.965" /> <path d="M3 3l18 18" /> </svg>"##;
const BULB_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h1m8 -9v1m8 8h1m-15.4 -6.4l.7 .7m12.1 -.7l-.7 .7" /> <path d="M9 16a5 5 0 1 1 6 0a3.5 3.5 0 0 0 -1 3a2 2 0 0 1 -4 0a3.5 3.5 0 0 0 -1 -3" /> <path d="M9.7 17l4.6 0" /> </svg>"##;
const BULB_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h1m8 -9v1m8 8h1m-15.4 -6.4l.7 .7m12.1 -.7l-.7 .7" /> <path d="M11.089 7.083a5 5 0 0 1 5.826 5.84m-1.378 2.611a5.012 5.012 0 0 1 -.537 .466a3.5 3.5 0 0 0 -1 3a2 2 0 1 1 -4 0a3.5 3.5 0 0 0 -1 -3a5 5 0 0 1 -.528 -7.544" /> <path d="M9.7 17h4.6" /> <path d="M3 3l18 18" /> </svg>"##;
const CAMERA_ROTATE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v9a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2" /> <path d="M11.245 15.904a3 3 0 0 0 3.755 -2.904m-2.25 -2.905a3 3 0 0 0 -3.75 2.905" /> <path d="M14 13h2v2" /> <path d="M10 13h-2v-2" /> </svg>"##;
const CAMERA_SELFIE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v9a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2" /> <path d="M9.5 15a3.5 3.5 0 0 0 5 0" /> <path d="M15 11l.01 0" /> <path d="M9 11l.01 0" /> </svg>"##;
const CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M18.364 5.636l-12.728 12.728" /> </svg>"##;
const CANDLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 21h6v-10a1 1 0 0 0 -1 -1h-4a1 1 0 0 0 -1 1l0 10" /> <path d="M12 2l1.465 1.638a2 2 0 1 1 -3.015 .099l1.55 -1.737" /> </svg>"##;
const CAP_PROJECTING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 6h-13a2 2 0 0 0 -2 2v8a2 2 0 0 0 2 2h13" /> <path d="M13 12a2 2 0 1 1 -4 0a2 2 0 0 1 4 0" /> <path d="M13 12h7" /> </svg>"##;
const CAP_ROUNDED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 6h-9a6 6 0 1 0 0 12h9" /> <path d="M13 12a2 2 0 1 1 -4 0a2 2 0 0 1 4 0" /> <path d="M13 12h7" /> </svg>"##;
const CAP_STRAIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12a2 2 0 1 1 -4 0a2 2 0 0 1 4 0" /> <path d="M8 12h12" /> <path d="M20 6h-12a2 2 0 0 0 -2 2v2" /> <path d="M6 14v2a2 2 0 0 0 2 2h12" /> </svg>"##;
const CAPSULE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 9a6 6 0 0 1 6 -6a6 6 0 0 1 6 6v6a6 6 0 0 1 -6 6a6 6 0 0 1 -6 -6l0 -6" /> </svg>"##;
const CAPSULE_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a6 6 0 0 1 6 -6h6a6 6 0 0 1 6 6a6 6 0 0 1 -6 6h-6a6 6 0 0 1 -6 -6" /> </svg>"##;
const CARDS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.604 7.197l7.138 -3.109a.96 .96 0 0 1 1.27 .527l4.924 11.902a1 1 0 0 1 -.514 1.304l-7.137 3.109a.96 .96 0 0 1 -1.271 -.527l-4.924 -11.903a1 1 0 0 1 .514 -1.304l0 .001" /> <path d="M15 4h1a1 1 0 0 1 1 1v3.5" /> <path d="M20 6c.264 .112 .52 .217 .768 .315a1 1 0 0 1 .53 1.311l-2.298 5.374" /> </svg>"##;
const CARET_LEFT_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 18l6 -6l-6 -6v12" /> <path d="M10 18l-6 -6l6 -6v12" /> </svg>"##;
const CAROUSEL_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 6a1 1 0 0 1 1 -1h8a1 1 0 0 1 1 1v12a1 1 0 0 1 -1 1h-8a1 1 0 0 1 -1 -1l0 -12" /> <path d="M22 17h-1a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h1" /> <path d="M2 17h1a1 1 0 0 0 1 -1v-8a1 1 0 0 0 -1 -1h-1" /> </svg>"##;
const CAROUSEL_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 8v8a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1" /> <path d="M7 22v-1a1 1 0 0 1 1 -1h8a1 1 0 0 1 1 1v1" /> <path d="M17 2v1a1 1 0 0 1 -1 1h-8a1 1 0 0 1 -1 -1v-1" /> </svg>"##;
const CATEGORY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4h6v6h-6l0 -6" /> <path d="M14 4h6v6h-6l0 -6" /> <path d="M4 14h6v6h-6l0 -6" /> <path d="M14 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const CATEGORY_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 4h6v6h-6l0 -6" /> <path d="M4 14h6v6h-6l0 -6" /> <path d="M14 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M4 7a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const CATEGORY_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4h6v6h-6v-6" /> <path d="M14 4h6v6h-6v-6" /> <path d="M4 14h6v6h-6v-6" /> <path d="M14 17h6" /> </svg>"##;
const CATEGORY_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4h6v6h-6v-6" /> <path d="M14 4h6v6h-6v-6" /> <path d="M4 14h6v6h-6v-6" /> <path d="M14 17h6m-3 -3v6" /> </svg>"##;
const CELL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4l-4 2v5l4 2l4 -2v-5l-4 -2" /> <path d="M12 11l4 2l4 -2v-5l-4 -2l-4 2" /> <path d="M8 13v5l4 2l4 -2v-5" /> </svg>"##;
const CHAIR_DIRECTOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 21l12 -9" /> <path d="M6 12l12 9" /> <path d="M5 12h14" /> <path d="M6 3v9" /> <path d="M18 3v9" /> <path d="M6 8h12" /> <path d="M6 5h12" /> </svg>"##;
const CHISEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 14l1.5 1.5" /> <path d="M18.347 15.575l2.08 2.079a1.96 1.96 0 0 1 -2.773 2.772l-2.08 -2.079a1.96 1.96 0 0 1 2.773 -2.772" /> <path d="M3 6l3 -3l7.414 7.414a2 2 0 0 1 .586 1.414v2.172h-2.172a2 2 0 0 1 -1.414 -.586l-7.414 -7.414" /> </svg>"##;
const CHRISTMAS_BALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 13a8 8 0 1 0 16 0a8 8 0 1 0 -16 0" /> <path d="M11 5l1 -2l1 2" /> <path d="M4.512 10.161c2.496 -1.105 4.992 -.825 7.488 .839c2.627 1.752 5.255 1.97 7.882 .653" /> <path d="M4.315 15.252c2.561 -1.21 5.123 -.96 7.685 .748c2.293 1.528 4.585 1.889 6.878 1.081" /> </svg>"##;
const CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CIRCLE_ASTERISK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8.5v7" /> <path d="M9 10l6 4" /> <path d="M9 14l6 -4" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const CIRCLE_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 12l2 2l4 -4" /> </svg>"##;
const CIRCLE_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> </svg>"##;
const CIRCLE_DASHED_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M9 12l2 2l4 -4" /> </svg>"##;
const CIRCLE_DASHED_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M9 12h6" /> </svg>"##;
const CIRCLE_DASHED_PERCENTAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l6 -6" /> <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M9 9.03v.015" /> <path d="M15 15.045v.015" /> </svg>"##;
const CIRCLE_DASHED_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M9 12h6" /> <path d="M12 9v6" /> </svg>"##;
const CIRCLE_DASHED_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M14 14l-4 -4" /> <path d="M10 14l4 -4" /> </svg>"##;
const CIRCLE_DOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CIRCLE_DOTTED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.5 4.21l0 .01" /> <path d="M4.21 7.5l0 .01" /> <path d="M3 12l0 .01" /> <path d="M4.21 16.5l0 .01" /> <path d="M7.5 19.79l0 .01" /> <path d="M12 21l0 .01" /> <path d="M16.5 19.79l0 .01" /> <path d="M19.79 16.5l0 .01" /> <path d="M21 12l0 .01" /> <path d="M19.79 7.5l0 .01" /> <path d="M16.5 4.21l0 .01" /> <path d="M12 3l0 .01" /> </svg>"##;
const CIRCLE_HALF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 3v18" /> </svg>"##;
const CIRCLE_HALF_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 3v18" /> <path d="M12 14l7 -7" /> <path d="M12 19l8.5 -8.5" /> <path d="M12 9l4.5 -4.5" /> </svg>"##;
const CIRCLE_HALF_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M3 12h18" /> </svg>"##;
const CIRCLE_KEY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 10a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M21 12a9 9 0 1 1 -18 0a9 9 0 0 1 18 0" /> <path d="M12.5 11.5l-4 4l1.5 1.5" /> <path d="M12 15l-1.5 -1.5" /> </svg>"##;
const CIRCLE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 12l6 0" /> </svg>"##;
const CIRCLE_MINUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.475 15.029a9 9 0 1 0 -7.962 5.957" /> <path d="M16 19h6" /> </svg>"##;
const CIRCLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.042 16.045a9 9 0 0 0 -12.087 -12.087m-2.318 1.677a9 9 0 1 0 12.725 12.73" /> <path d="M3 3l18 18" /> </svg>"##;
const CIRCLE_PERCENTAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M9 15.075l6 -6" /> <path d="M9 9.105v.015" /> <path d="M15 15.12v.015" /> </svg>"##;
const CIRCLE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M9 12h6" /> <path d="M12 9v6" /> </svg>"##;
const CIRCLE_PLUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.985 12.522a9 9 0 1 0 -8.475 8.464" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const CIRCLE_PLUS_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M9 10h6" /> <path d="M9 16h6" /> <path d="M12 7v6" /> </svg>"##;
const CIRCLE_RECTANGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M7 10h10v4h-10l0 -4" /> </svg>"##;
const CIRCLE_RECTANGLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10h3v3m-3 1h-7v-4h3" /> <path d="M20.042 16.045a9 9 0 0 0 -12.087 -12.087m-2.318 1.677a9 9 0 1 0 12.725 12.73" /> <path d="M3 3l18 18" /> </svg>"##;
const CIRCLE_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9.5a6.5 6.5 0 1 0 13 0a6.5 6.5 0 1 0 -13 0" /> <path d="M10 12a2 2 0 0 1 2 -2h7a2 2 0 0 1 2 2v7a2 2 0 0 1 -2 2h-7a2 2 0 0 1 -2 -2l0 -7" /> </svg>"##;
const CIRCLE_TRIANGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 20l7 -12h-14l7 12" /> </svg>"##;
const CIRCLE_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 10l4 4m0 -4l-4 4" /> </svg>"##;
const CIRCLES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M2.5 17a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M13.5 17a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> </svg>"##;
const CIRCLES_RELATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.183 6.117a6 6 0 1 0 4.511 3.986" /> <path d="M14.813 17.883a6 6 0 1 0 -4.496 -3.954" /> </svg>"##;
const CLUBS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a4 4 0 0 1 3.164 6.447a4 4 0 1 1 -1.164 6.198v1.355l1 4h-6l1 -4l0 -1.355a4 4 0 1 1 -1.164 -6.199a4 4 0 0 1 3.163 -6.446" /> </svg>"##;
const COFFIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3l-2 6l2 12h6l2 -12l-2 -6l-6 0" /> <path d="M10 7v5" /> <path d="M8 9h4" /> <path d="M13 21h4l2 -12l-2 -6h-4" /> </svg>"##;
const COLOR_FILTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.58 13.79c.27 .68 .42 1.43 .42 2.21c0 1.77 -.77 3.37 -2 4.46a5.93 5.93 0 0 1 -4 1.54c-3.31 0 -6 -2.69 -6 -6c0 -2.76 1.88 -5.1 4.42 -5.79" /> <path d="M17.58 10.21c2.54 .69 4.42 3.03 4.42 5.79c0 3.31 -2.69 6 -6 6a5.93 5.93 0 0 1 -4 -1.54" /> <path d="M6 8a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" /> </svg>"##;
const COLOR_PICKER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 7l6 6" /> <path d="M4 16l11.7 -11.7a1 1 0 0 1 1.4 0l2.6 2.6a1 1 0 0 1 0 1.4l-11.7 11.7h-4v-4" /> </svg>"##;
const COLOR_PICKER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 7l6 6" /> <path d="M12 8l3.699 -3.699a1 1 0 0 1 1.4 0l2.6 2.6a1 1 0 0 1 0 1.4l-3.702 3.702m-2 2l-6 6h-4v-4l6 -6" /> <path d="M3 3l18 18" /> </svg>"##;
const COLOR_SWATCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 3h-4a2 2 0 0 0 -2 2v12a4 4 0 0 0 8 0v-12a2 2 0 0 0 -2 -2" /> <path d="M13 7.35l-2 -2a2 2 0 0 0 -2.828 0l-2.828 2.828a2 2 0 0 0 0 2.828l9 9" /> <path d="M7.3 13h-2.3a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h12" /> <path d="M17 17l0 .01" /> </svg>"##;
const COLOR_SWATCH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 13v4a4 4 0 0 0 6.832 2.825m1.168 -2.825v-12a2 2 0 0 0 -2 -2h-4a2 2 0 0 0 -2 2v4" /> <path d="M13 7.35l-2 -2a2 2 0 0 0 -2.11 -.461m-2.13 1.874l-1.416 1.415a2 2 0 0 0 0 2.828l9 9" /> <path d="M7.3 13h-2.3a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h12" /> <path d="M17 17v.01" /> <path d="M3 3l18 18" /> </svg>"##;
const COMPONENTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12l3 3l3 -3l-3 -3l-3 3" /> <path d="M15 12l3 3l3 -3l-3 -3l-3 3" /> <path d="M9 6l3 3l3 -3l-3 -3l-3 3" /> <path d="M9 18l3 3l3 -3l-3 -3l-3 3" /> </svg>"##;
const COMPONENTS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12l3 3l3 -3l-3 -3l-3 3" /> <path d="M18.5 14.5l2.5 -2.5l-3 -3l-2.5 2.5" /> <path d="M12.499 8.501l2.501 -2.501l-3 -3l-2.5 2.5" /> <path d="M9 18l3 3l3 -3l-3 -3l-3 3" /> <path d="M3 3l18 18" /> </svg>"##;
const CONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 17.998v-.5l-8.13 -14.99a1 1 0 0 0 -1.74 0l-8.13 14.989v.5c0 1.659 4.03 3.003 9 3.003s9 -1.344 9 -3.002" /> </svg>"##;
const CONE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 5.002v.5l-8.13 14.99a1 1 0 0 1 -1.74 0l-8.13 -14.989v-.5c0 -1.659 4.03 -3.003 9 -3.003s9 1.344 9 3.002" /> </svg>"##;
const CONE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.396 16.384l-7.526 -13.877a1 1 0 0 0 -1.74 0l-1.626 2.998m-1.407 2.594l-5.097 9.398v.5c0 1.66 4.03 3.003 9 3.003c3.202 0 6.014 -.558 7.609 -1.398" /> <path d="M3 3l18 18" /> </svg>"##;
const CONE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.03 12.022l-5.16 -9.515a1 1 0 0 0 -1.74 0l-8.13 14.99v.5c0 1.66 4.03 3.003 9 3.003c.17 0 .34 -.002 .508 -.005" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const CONFETTI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5h2" /> <path d="M5 4v2" /> <path d="M11.5 4l-.5 2" /> <path d="M18 5h2" /> <path d="M19 4v2" /> <path d="M15 9l-1 1" /> <path d="M18 13l2 -.5" /> <path d="M18 19h2" /> <path d="M19 18v2" /> <path d="M14 16.518l-6.518 -6.518l-4.39 9.58a1 1 0 0 0 1.329 1.329l9.579 -4.39" /> </svg>"##;
const CONFETTI_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5h1" /> <path d="M5 5v1" /> <path d="M11.5 4l-.5 2" /> <path d="M18 5h2" /> <path d="M19 4v2" /> <path d="M15 9l-1 1" /> <path d="M18 13l2 -.5" /> <path d="M18 19h1" /> <path d="M19 19v1" /> <path d="M14 16.518l-6.518 -6.518l-4.39 9.58a1 1 0 0 0 1.329 1.329l9.579 -4.39" /> <path d="M3 3l18 18" /> </svg>"##;
const CONTAINER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 4v.01" /> <path d="M20 20v.01" /> <path d="M20 16v.01" /> <path d="M20 12v.01" /> <path d="M20 8v.01" /> <path d="M8 5a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1l0 -14" /> <path d="M4 4v.01" /> <path d="M4 20v.01" /> <path d="M4 16v.01" /> <path d="M4 12v.01" /> <path d="M4 8v.01" /> </svg>"##;
const CONTAINER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 4v.01" /> <path d="M20 20v.01" /> <path d="M20 16v.01" /> <path d="M20 12v.01" /> <path d="M20 8v.01" /> <path d="M8.297 4.289a1 1 0 0 1 .703 -.289h6a1 1 0 0 1 1 1v7m0 4v3a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1v-11" /> <path d="M4 4v.01" /> <path d="M4 20v.01" /> <path d="M4 16v.01" /> <path d="M4 12v.01" /> <path d="M4 8v.01" /> <path d="M3 3l18 18" /> </svg>"##;
const CONTRAST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 17a5 5 0 0 0 0 -10v10" /> </svg>"##;
const CONTRAST_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -14" /> <path d="M3 19h2.25c3.728 0 6.75 -3.134 6.75 -7s3.022 -7 6.75 -7h2.25" /> </svg>"##;
const CONTRAST_2_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18h2a6 6 0 0 0 6 -6m.878 -3.126a6 6 0 0 1 5.122 -2.874h2" /> <path d="M8 4h10a2 2 0 0 1 2 2v10m-.586 3.414a2 2 0 0 1 -1.414 .586h-12a2 2 0 0 1 -2 -2v-12c0 -.547 .22 -1.043 .576 -1.405" /> <path d="M3 3l18 18" /> </svg>"##;
const CONTRAST_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12v5a4.984 4.984 0 0 0 3.522 -1.45m1.392 -2.623a5 5 0 0 0 -4.914 -5.927v1" /> <path d="M5.641 5.631a9 9 0 1 0 12.719 12.738m1.68 -2.318a9 9 0 0 0 -12.074 -12.098" /> <path d="M3 3l18 18" /> </svg>"##;
const COOKIE_MAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 2a5 5 0 0 1 2.845 9.112l.147 .369l1.755 -.803c.969 -.443 2.12 -.032 2.571 .918a1.88 1.88 0 0 1 -.787 2.447l-.148 .076l-2.383 1.089v2.02l1.426 1.425l.114 .125a1.96 1.96 0 0 1 -2.762 2.762l-.125 -.114l-2.079 -2.08l-.114 -.124a2 2 0 0 1 -.161 -.22h-.599q -.071 .114 -.16 .22l-.115 .125l-2.08 2.079a1.96 1.96 0 0 1 -2.886 -2.648l.114 -.125l1.427 -1.426v-2.019l-2.383 -1.09l-.148 -.075a1.88 1.88 0 0 1 -.787 -2.447c.429 -.902 1.489 -1.318 2.424 -.978l.147 .06l1.755 .803l.147 -.369a5 5 0 0 1 -2.15 -3.895v-.217a5 5 0 0 1 5 -5l-.005 0" /> <path d="M12 16h.01" /> <path d="M12 13h.01" /> <path d="M10 7h.01" /> <path d="M14 7h.01" /> <path d="M12 9h.01" /> </svg>"##;
const CROP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 5v10a1 1 0 0 0 1 1h10" /> <path d="M5 8h10a1 1 0 0 1 1 1v10" /> </svg>"##;
const CROP_1_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> </svg>"##;
const CROP_16_9_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v4a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -4" /> </svg>"##;
const CROP_3_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 9a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -6" /> </svg>"##;
const CROP_5_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -10" /> </svg>"##;
const CROP_7_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -8" /> </svg>"##;
const CROP_LANDSCAPE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -8" /> </svg>"##;
const CROP_PORTRAIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 6a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -12" /> </svg>"##;
const CROWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 6l4 6l5 -4l-2 10h-14l-2 -10l5 4l4 -6" /> </svg>"##;
const CROWN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 18h-13l-1.865 -9.327a.25 .25 0 0 1 .4 -.244l4.465 3.571l1.6 -2.4m1.596 -2.394l.804 -1.206l4 6l4.464 -3.571a.25 .25 0 0 1 .401 .244l-1.363 6.818" /> <path d="M3 3l18 18" /> </svg>"##;
const CUBE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 16.008v-8.018a1.98 1.98 0 0 0 -1 -1.717l-7 -4.008a2.016 2.016 0 0 0 -2 0l-7 4.008c-.619 .355 -1 1.01 -1 1.718v8.018c0 .709 .381 1.363 1 1.717l7 4.008a2.016 2.016 0 0 0 2 0l7 -4.008c.619 -.355 1 -1.01 1 -1.718" /> <path d="M12 22v-10" /> <path d="M12 12l8.73 -5.04" /> <path d="M3.27 6.96l8.73 5.04" /> </svg>"##;
const CUBE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.83 16.809c.11 -.248 .17 -.52 .17 -.801v-8.018a1.98 1.98 0 0 0 -1 -1.717l-7 -4.008a2.016 2.016 0 0 0 -2 0l-3.012 1.725m-2.547 1.458l-1.441 .825c-.619 .355 -1 1.01 -1 1.718v8.018c0 .709 .381 1.363 1 1.717l7 4.008a2.016 2.016 0 0 0 2 0l5.544 -3.174" /> <path d="M12 22v-10" /> <path d="M14.532 10.538l6.198 -3.578" /> <path d="M3.27 6.96l8.73 5.04" /> <path d="M3 3l18 18" /> </svg>"##;
const CUBE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12.5v-4.509a1.98 1.98 0 0 0 -1 -1.717l-7 -4.008a2.016 2.016 0 0 0 -2 0l-7 4.007c-.619 .355 -1 1.01 -1 1.718v8.018c0 .709 .381 1.363 1 1.717l7 4.008a2.016 2.016 0 0 0 2 0" /> <path d="M12 22v-10" /> <path d="M12 12l8.73 -5.04" /> <path d="M3.27 6.96l8.73 5.04" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const CUBE_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12v-4.01a1.98 1.98 0 0 0 -1 -1.717l-7 -4.008a2.02 2.02 0 0 0 -2 0l-7 4.008c-.619 .355 -1 1.01 -1 1.718v8.018c0 .709 .381 1.363 1 1.717l7 4.008c.62 .354 1.38 .354 2 0" /> <path d="M12 22v-10" /> <path d="M12 12l8.73 -5.04" /> <path d="M3.27 6.96l8.73 5.04" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const CUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M14 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M9.15 14.85l8.85 -10.85" /> <path d="M6 4l8.85 10.85" /> </svg>"##;
const CYLINDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 6a7 3 0 1 0 14 0a7 3 0 1 0 -14 0" /> <path d="M5 6v12c0 1.657 3.134 3 7 3s7 -1.343 7 -3v-12" /> </svg>"##;
const CYLINDER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.23 5.233c-.15 .245 -.23 .502 -.23 .767c0 1.131 1.461 2.117 3.62 2.628m4.357 .343c3.404 -.204 6.023 -1.456 6.023 -2.971c0 -1.657 -3.134 -3 -7 -3c-1.645 0 -3.158 .243 -4.353 .65" /> <path d="M5 6v12c0 1.657 3.134 3 7 3c3.245 0 5.974 -.946 6.767 -2.23m.233 -3.77v-9" /> <path d="M3 3l18 18" /> </svg>"##;
const CYLINDER_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 6a7 3 0 1 0 14 0a7 3 0 1 0 -14 0" /> <path d="M5 6v12c0 1.657 3.134 3 7 3c.173 0 .345 -.003 .515 -.008m6.485 -8.992v-6" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const DIABOLO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a8 3 0 1 0 16 0a8 3 0 1 0 -16 0" /> <path d="M4 6v.143a1 1 0 0 0 .048 .307l1.952 5.55l-1.964 5.67a1 1 0 0 0 -.036 .265v.065c0 1.657 3.582 3 8 3s8 -1.343 8 -3v-.065a1 1 0 0 0 -.036 -.265l-1.964 -5.67l1.952 -5.55a1 1 0 0 0 .048 -.307v-.143" /> <path d="M6 12c0 1.105 2.686 2 6 2s6 -.895 6 -2" /> </svg>"##;
const DIABOLO_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.727 4.749c-.467 .38 -.727 .804 -.727 1.251c0 1.217 1.933 2.265 4.71 2.735m4.257 .243c3.962 -.178 7.033 -1.444 7.033 -2.978c0 -1.657 -3.582 -3 -8 -3c-1.66 0 -3.202 .19 -4.48 .514" /> <path d="M4 6v.143a1 1 0 0 0 .048 .307l1.952 5.55l-1.964 5.67a1 1 0 0 0 -.036 .265v.065c0 1.657 3.582 3 8 3c3.218 0 5.992 -.712 7.262 -1.74m-.211 -4.227l-1.051 -3.033l1.952 -5.55a1 1 0 0 0 .048 -.307v-.143" /> <path d="M6 12c0 1.105 2.686 2 6 2c.656 0 1.288 -.035 1.879 -.1m3.198 -.834c.585 -.308 .923 -.674 .923 -1.066" /> <path d="M3 3l18 18" /> </svg>"##;
const DIABOLO_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a8 3 0 1 0 16 0a8 3 0 1 0 -16 0" /> <path d="M4 6v.143a1 1 0 0 0 .048 .307l1.952 5.55l-1.964 5.67a1 1 0 0 0 -.036 .265v.065c0 1.657 3.582 3 8 3c.17 0 .34 -.002 .508 -.006m5.492 -8.994l1.952 -5.55a1 1 0 0 0 .048 -.307v-.143" /> <path d="M6 12c0 1.105 2.686 2 6 2s6 -.895 6 -2" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const DIAMOND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 5h12l3 5l-8.5 9.5a.7 .7 0 0 1 -1 0l-8.5 -9.5l3 -5" /> <path d="M10 12l-2 -2.2l.6 -1" /> </svg>"##;
const DIAMOND_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h9l3 5l-3.308 3.697m-1.883 2.104l-3.309 3.699a.7 .7 0 0 1 -1 0l-8.5 -9.5l2.62 -4.368" /> <path d="M10 12l-2 -2.2l.6 -1" /> <path d="M3 3l18 18" /> </svg>"##;
const DIAMONDS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.831 20.413l-5.375 -6.91c-.608 -.783 -.608 -2.223 0 -3l5.375 -6.911a1.457 1.457 0 0 1 2.338 0l5.375 6.91c.608 .783 .608 2.223 0 3l-5.375 6.911a1.457 1.457 0 0 1 -2.338 0" /> </svg>"##;
const DIMENSIONS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5h11" /> <path d="M12 7l2 -2l-2 -2" /> <path d="M5 3l-2 2l2 2" /> <path d="M19 10v11" /> <path d="M17 19l2 2l2 -2" /> <path d="M21 12l-2 -2l-2 2" /> <path d="M3 12a2 2 0 0 1 2 -2h7a2 2 0 0 1 2 2v7a2 2 0 0 1 -2 2h-7a2 2 0 0 1 -2 -2l0 -7" /> </svg>"##;
const DRAG_DROP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 11v-2a2 2 0 0 0 -2 -2h-8a2 2 0 0 0 -2 2v8a2 2 0 0 0 2 2h2" /> <path d="M13 13l9 3l-4 2l-2 4l-3 -9" /> <path d="M3 3l0 .01" /> <path d="M7 3l0 .01" /> <path d="M11 3l0 .01" /> <path d="M15 3l0 .01" /> <path d="M3 7l0 .01" /> <path d="M3 11l0 .01" /> <path d="M3 15l0 .01" /> </svg>"##;
const DRAG_DROP_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 10a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /> <path d="M4 4l0 .01" /> <path d="M8 4l0 .01" /> <path d="M12 4l0 .01" /> <path d="M16 4l0 .01" /> <path d="M4 8l0 .01" /> <path d="M4 12l0 .01" /> <path d="M4 16l0 .01" /> </svg>"##;
const DROPLET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.502 19.423c2.602 2.105 6.395 2.105 8.996 0c2.602 -2.105 3.262 -5.708 1.566 -8.546l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546" /> </svg>"##;
const DROPLET_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.628 12.076a6.653 6.653 0 0 0 -.564 -1.199l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546c1.7 1.375 3.906 1.852 5.958 1.431" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const DROPLET_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.606 12.014a6.659 6.659 0 0 0 -.542 -1.137l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.154 7.154 0 0 0 4.826 1.572" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const DROPLET_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.967 13.594a6.568 6.568 0 0 0 -.903 -2.717l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.125 7.125 0 0 0 4.04 1.565" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const DROPLET_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.907 13.147a6.586 6.586 0 0 0 -.843 -2.27l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.123 7.123 0 0 0 3.99 1.561" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const DROPLET_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.421 11.56a6.702 6.702 0 0 0 -.357 -.683l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.144 7.144 0 0 0 4.518 1.58" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const DROPLET_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.668 10.29l-4.493 -6.673c-.421 -.625 -1.288 -.803 -1.937 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.175 7.175 0 0 0 5.493 1.51" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const DROPLET_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.602 12.003a6.66 6.66 0 0 0 -.538 -1.126l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.159 7.159 0 0 0 4.972 1.564" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const DROPLET_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.602 12.004a6.66 6.66 0 0 0 -.538 -1.127l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546c2.142 1.734 5.092 2.04 7.519 .919" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const DROPLET_HALF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.502 19.423c2.602 2.105 6.395 2.105 8.996 0c2.602 -2.105 3.262 -5.708 1.566 -8.546l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546" /> <path d="M12 3v18" /> </svg>"##;
const DROPLET_HALF_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.502 19.423c2.602 2.105 6.395 2.105 8.996 0c2.602 -2.105 3.262 -5.708 1.566 -8.546l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546" /> <path d="M5 14h14" /> </svg>"##;
const DROPLET_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.288 11.282a6.734 6.734 0 0 0 -.224 -.405l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.117 7.117 0 0 0 3.824 1.548" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const DROPLET_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.946 15.083a6.538 6.538 0 0 0 -.882 -4.206l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.163 7.163 0 0 0 5.089 1.555" /> <path d="M16 19h6" /> </svg>"##;
const DROPLET_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.963 14.938a6.54 6.54 0 0 0 -.899 -4.06l-4.89 -7.26c-.42 -.626 -1.287 -.804 -1.936 -.398a1.376 1.376 0 0 0 -.41 .397l-1.282 1.9m-1.625 2.415l-1.986 2.946c-1.695 2.837 -1.035 6.44 1.567 8.545c2.602 2.105 6.395 2.105 8.996 0a6.83 6.83 0 0 0 1.376 -1.499" /> <path d="M3 3l18 18" /> </svg>"##;
const DROPLET_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.952 13.456a6.573 6.573 0 0 0 -.888 -2.579l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.176 7.176 0 0 0 5.517 1.507" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const DROPLET_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.064 10.877l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.163 7.163 0 0 0 5.102 1.554" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const DROPLET_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.602 12.004a6.66 6.66 0 0 0 -.538 -1.127l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.16 7.16 0 0 0 5.033 1.56" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const DROPLET_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.064 10.877l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546c2.203 1.782 5.259 2.056 7.723 .82" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const DROPLET_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.064 10.877l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.13 7.13 0 0 0 4.168 1.572" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const DROPLET_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.884 13.025a6.591 6.591 0 0 0 -.82 -2.148l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.125 7.125 0 0 0 4.498 1.58" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const DROPLET_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.496 10.034l-4.321 -6.417c-.421 -.625 -1.288 -.803 -1.937 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.106 7.106 0 0 0 3.547 1.517" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const DROPLET_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.6 11.998a6.66 6.66 0 0 0 -.536 -1.12l-4.89 -7.26c-.42 -.626 -1.287 -.804 -1.936 -.398a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.16 7.16 0 0 0 5.002 1.562" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const DROPLET_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.953 13.467a6.572 6.572 0 0 0 -.889 -2.59l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546a7.179 7.179 0 0 0 5.633 1.49" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const EASE_IN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 20c8 0 18 -16 18 -16" /> </svg>"##;
const EASE_IN_CONTROL_POINT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19c8 0 18 -16 18 -16" /> <path d="M17 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M17 19h-2" /> <path d="M12 19h-2" /> </svg>"##;
const EASE_IN_OUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 20c8 0 10 -16 18 -16" /> </svg>"##;
const EASE_IN_OUT_CONTROL_POINTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 20a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M17 20h-2" /> <path d="M7 4a2 2 0 1 1 -4 0a2 2 0 0 1 4 0" /> <path d="M7 4h2" /> <path d="M14 4h-2" /> <path d="M12 20h-2" /> <path d="M3 20c8 0 10 -16 18 -16" /> </svg>"##;
const EASE_OUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 20s10 -16 18 -16" /> </svg>"##;
const EASE_OUT_CONTROL_POINT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21s10 -16 18 -16" /> <path d="M7 5a2 2 0 1 1 -4 0a2 2 0 0 1 4 0" /> <path d="M7 5h2" /> <path d="M14 5h-2" /> </svg>"##;
const EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7h-1a2 2 0 0 0 -2 2v9a2 2 0 0 0 2 2h9a2 2 0 0 0 2 -2v-1" /> <path d="M20.385 6.585a2.1 2.1 0 0 0 -2.97 -2.97l-8.415 8.385v3h3l8.385 -8.415" /> <path d="M16 5l3 3" /> </svg>"##;
const EDIT_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 15l8.385 -8.415a2.1 2.1 0 0 0 -2.97 -2.97l-8.415 8.385v3h3" /> <path d="M16 5l3 3" /> <path d="M9 7.07a7 7 0 0 0 1 13.93a7 7 0 0 0 6.929 -6" /> </svg>"##;
const EDIT_CIRCLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.507 10.498l-1.507 1.502v3h3l1.493 -1.498m2 -2.01l4.89 -4.907a2.1 2.1 0 0 0 -2.97 -2.97l-4.913 4.896" /> <path d="M16 5l3 3" /> <path d="M7.476 7.471a7 7 0 0 0 2.524 13.529a7 7 0 0 0 6.53 -4.474" /> <path d="M3 3l18 18" /> </svg>"##;
const EDIT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7h-1a2 2 0 0 0 -2 2v9a2 2 0 0 0 2 2h9a2 2 0 0 0 2 -2v-1" /> <path d="M10.507 10.498l-1.507 1.502v3h3l1.493 -1.498m2 -2.01l4.89 -4.907a2.1 2.1 0 0 0 -2.97 -2.97l-4.913 4.896" /> <path d="M16 5l3 3" /> <path d="M3 3l18 18" /> </svg>"##;
const EXPOSURE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.6 20.4l16.8 -16.8" /> <path d="M6 8h4m-2 -2v4" /> <path d="M14 16h4" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -14" /> </svg>"##;
const EXPOSURE_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19a4 4 0 0 0 4 -4v-6a4 4 0 1 0 -8 0v6a4 4 0 0 0 4 4" /> </svg>"##;
const EXPOSURE_MINUS_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h6" /> <path d="M18 19v-14l-4 4" /> </svg>"##;
const EXPOSURE_MINUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9a4 4 0 1 1 8 0c0 1.098 -.564 2.025 -1.159 2.815l-6.841 7.185h8" /> <path d="M3 12h6" /> </svg>"##;
const EXPOSURE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.6 20.4l8.371 -8.371m2.04 -2.04l6.389 -6.389" /> <path d="M6 8h2m0 0v2" /> <path d="M14 16h2" /> <path d="M7 3h12a2 2 0 0 1 2 2v12m-.5 3.5c-.362 .36 -.95 .5 -1.5 .5h-14a2 2 0 0 1 -2 -2v-14c0 -.541 .215 -1.033 .565 -1.393" /> <path d="M3 3l18 18" /> </svg>"##;
const EXPOSURE_PLUS_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h6" /> <path d="M6 9v6" /> <path d="M18 19v-14l-4 4" /> </svg>"##;
const EXPOSURE_PLUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9a4 4 0 1 1 8 0c0 1.098 -.564 2.025 -1.159 2.815l-6.841 7.185h8" /> <path d="M3 12h6" /> <path d="M6 9v6" /> </svg>"##;
const FAVICON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 8a3 3 0 0 1 3 -3h14a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-14a3 3 0 0 1 -3 -3l0 -8" /> <path d="M6 10v4" /> <path d="M11 10a2 2 0 1 0 0 4" /> <path d="M14 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const FLIP_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12l18 0" /> <path d="M7 16l10 0l-10 5l0 -5" /> <path d="M7 8l10 0l-10 -5l0 5" /> </svg>"##;
const FLIP_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3l0 18" /> <path d="M16 7l0 10l5 0l-5 -10" /> <path d="M8 7l0 10l-5 0l5 -10" /> </svg>"##;
const FOCUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 12a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const FOCUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 12a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M5 12a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M12 3l0 2" /> <path d="M3 12l2 0" /> <path d="M12 19l0 2" /> <path d="M19 12l2 0" /> </svg>"##;
const FOCUS_AUTO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> <path d="M10 15v-4a2 2 0 1 1 4 0v4" /> <path d="M10 13h4" /> </svg>"##;
const FRAME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7l16 0" /> <path d="M4 17l16 0" /> <path d="M7 4l0 16" /> <path d="M17 4l0 16" /> </svg>"##;
const FRAME_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7h3m4 0h9" /> <path d="M4 17h13" /> <path d="M7 7v13" /> <path d="M17 4v9m0 4v3" /> <path d="M3 3l18 18" /> </svg>"##;
const FREEZE_COLUMN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 9.5l-6 6" /> <path d="M9 4l-6 6" /> <path d="M9 15l-5 5" /> <path d="M9 3v18" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const FREEZE_ROW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M21 9h-18" /> <path d="M15 3l-6 6" /> <path d="M9.5 3l-6 6" /> <path d="M20 3.5l-5.5 5.5" /> </svg>"##;
const FREEZE_ROW_COLUMN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M15 3l-12 12" /> <path d="M9.5 3l-6 6" /> <path d="M20 3.5l-5.5 5.5" /> <path d="M9 15l-5 5" /> <path d="M21 9h-12v12" /> </svg>"##;
const FRUSTUM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.402 5.508l2.538 10.158a1.99 1.99 0 0 1 -1.064 2.278l-7.036 3.366a1.945 1.945 0 0 1 -1.682 0l-7.035 -3.365a1.99 1.99 0 0 1 -1.064 -2.278l2.539 -10.159a1.98 1.98 0 0 1 1.11 -1.328l4.496 -2.01a1.95 1.95 0 0 1 1.59 0l4.496 2.01c.554 .246 .963 .736 1.112 1.328" /> <path d="M18 4.82l-5.198 2.324a1.963 1.963 0 0 1 -1.602 0l-5.2 -2.325" /> <path d="M12 7.32v14.18" /> </svg>"##;
const FRUSTUM_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.72 3.728l3.484 -1.558a1.95 1.95 0 0 1 1.59 0l4.496 2.01c.554 .246 .963 .736 1.112 1.328l2.538 10.158c.103 .412 .07 .832 -.075 1.206m-2.299 1.699l-5.725 2.738a1.945 1.945 0 0 1 -1.682 0l-7.035 -3.365a1.99 1.99 0 0 1 -1.064 -2.278l2.52 -10.08" /> <path d="M18 4.82l-5.198 2.324a1.963 1.963 0 0 1 -1.602 0" /> <path d="M12 7.32v.68m0 4v9.5" /> <path d="M3 3l18 18" /> </svg>"##;
const FRUSTUM_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.841 21.309a1.945 1.945 0 0 1 -1.682 0l-7.035 -3.365a1.99 1.99 0 0 1 -1.064 -2.278l2.538 -10.158a1.98 1.98 0 0 1 1.11 -1.328l4.496 -2.01a1.95 1.95 0 0 1 1.59 0l4.496 2.01c.554 .246 .963 .736 1.112 1.328l1.67 6.683" /> <path d="M18 4.82l-5.198 2.324a1.963 1.963 0 0 1 -1.602 0l-5.2 -2.325" /> <path d="M12 7.32v14.18" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const GENDER_AGENDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 12a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" /> <path d="M7 12h11" /> </svg>"##;
const GENDER_ANDROGYNE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 11l6 -6" /> <path d="M4 15a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M19 9v-4h-4" /> <path d="M16.5 10.5l-3 -3" /> </svg>"##;
const GENDER_BIGENDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 11a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M19 3l-5 5" /> <path d="M15 3h4v4" /> <path d="M11 16v6" /> <path d="M8 19h6" /> </svg>"##;
const GENDER_DEMIBOY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 14a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M19 5l-5.4 5.4" /> <path d="M19 5h-5" /> </svg>"##;
const GENDER_DEMIGIRL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M12 14v7" /> <path d="M9 18h3" /> </svg>"##;
const GENDER_EPICENE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.536 15.536a5 5 0 1 0 -7.072 -7.072a5 5 0 0 0 7.072 7.072" /> <path d="M15.536 15.535l5.464 -5.535" /> <path d="M3 14l5.464 -5.535" /> <path d="M12 12h.01" /> </svg>"##;
const GENDER_FEMALE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M12 14v7" /> <path d="M9 18h6" /> </svg>"##;
const GENDER_FEMME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M12 14v7" /> <path d="M7 18h10" /> </svg>"##;
const GENDER_GENDERFLUID_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15.464a4 4 0 1 0 4 -6.928a4 4 0 0 0 -4 6.928" /> <path d="M15.464 14l3 -5.196" /> <path d="M5.536 15.195l3 -5.196" /> <path d="M12 12h.01" /> <path d="M9 9l-6 -6" /> <path d="M5.5 8.5l3 -3" /> <path d="M21 21l-6 -6" /> <path d="M17 20l3 -3" /> <path d="M3 7v-4h4" /> </svg>"##;
const GENDER_GENDERLESS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 10a5 5 0 1 1 0 10a5 5 0 0 1 0 -10" /> <path d="M12 10v-7" /> <path d="M7 15h10" /> </svg>"##;
const GENDER_GENDERQUEER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 11a5 5 0 1 1 0 10a5 5 0 0 1 0 -10" /> <path d="M12 11v-8" /> <path d="M14.5 4.5l-5 3" /> <path d="M9.5 4.5l5 3" /> </svg>"##;
const GENDER_HERMAPHRODITE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 14v7" /> <path d="M9 18h6" /> <path d="M12 6a4 4 0 1 1 0 8a4 4 0 0 1 0 -8" /> <path d="M15 3a3 3 0 1 1 -6 0" /> </svg>"##;
const GENDER_INTERGENDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 11.5l6.5 6.5v-4" /> <path d="M11.5 13.5l6.5 6.5" /> <path d="M9 4a5 5 0 1 1 0 10a5 5 0 0 1 0 -10" /> <path d="M14 20l2 -2" /> </svg>"##;
const GENDER_MALE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 14a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M19 5l-5.4 5.4" /> <path d="M19 5h-5" /> <path d="M19 5v5" /> </svg>"##;
const GENDER_NEUTROIS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 10a5 5 0 1 1 0 10a5 5 0 0 1 0 -10" /> <path d="M12 10v-7" /> </svg>"##;
const GENDER_THIRD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12a5 5 0 1 0 10 0a5 5 0 0 0 -10 0" /> <path d="M11 12h-3" /> <path d="M8 12l-5 -4v8l5 -4" /> </svg>"##;
const GENDER_TRANSGENDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M15 9l6 -6" /> <path d="M21 7v-4h-4" /> <path d="M9 9l-6 -6" /> <path d="M3 7v-4h4" /> <path d="M5.5 8.5l3 -3" /> <path d="M12 16v5" /> <path d="M9.5 19h5" /> </svg>"##;
const GENDER_TRASVESTI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 20a5 5 0 1 1 0 -10a5 5 0 0 1 0 10" /> <path d="M6 6l5.4 5.4" /> <path d="M4 8l4 -4" /> </svg>"##;
const GIZMO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 19l-8 -5.5l-8 5.5" /> <path d="M12 4v9.5" /> <path d="M11 4a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3 19a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M19 19a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const GRADIENTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.227 14c.917 4 4.497 7 8.773 7c4.277 0 7.858 -3 8.773 -7" /> <path d="M20.78 10a9 9 0 0 0 -8.78 -7a8.985 8.985 0 0 0 -8.782 7" /> <path d="M10 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const GRAIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.5 9.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M8.5 4.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M8.5 14.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3.5 19.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M13.5 9.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M18.5 4.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M13.5 19.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M18.5 14.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const GRID_3X3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8h18" /> <path d="M3 16h18" /> <path d="M8 3v18" /> <path d="M16 3v18" /> </svg>"##;
const GRID_4X4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6h18" /> <path d="M3 12h18" /> <path d="M3 18h18" /> <path d="M6 3v18" /> <path d="M12 3v18" /> <path d="M18 3v18" /> </svg>"##;
const GRID_GOLDENRATIO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10h18" /> <path d="M3 14h18" /> <path d="M10 3v18" /> <path d="M14 3v18" /> </svg>"##;
const GUITAR_PICK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 18.5c2 -2.5 4 -6.5 4 -10.5c0 -2.946 -2.084 -4.157 -4.204 -4.654c-.864 -.23 -2.13 -.346 -3.796 -.346c-1.667 0 -2.932 .115 -3.796 .346c-2.12 .497 -4.204 1.708 -4.204 4.654c0 3.312 2 8 4 10.5c.297 .37 .618 .731 .963 1.081l.354 .347a3.9 3.9 0 0 0 5.364 0a14.05 14.05 0 0 0 1.319 -1.428" /> </svg>"##;
const HAMMER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.414 10l-7.383 7.418a2.091 2.091 0 0 0 0 2.967a2.11 2.11 0 0 0 2.976 0l7.407 -7.385" /> <path d="M18.121 15.293l2.586 -2.586a1 1 0 0 0 0 -1.414l-7.586 -7.586a1 1 0 0 0 -1.414 0l-2.586 2.586a1 1 0 0 0 0 1.414l7.586 7.586a1 1 0 0 0 1.414 0" /> </svg>"##;
const HAMMER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.698 10.72l-6.668 6.698a2.091 2.091 0 0 0 0 2.967a2.11 2.11 0 0 0 2.976 0l6.696 -6.676" /> <path d="M18.713 14.702l2 -2a1 1 0 0 0 0 -1.414l-7.586 -7.586a1 1 0 0 0 -1.414 0l-2 2" /> <path d="M3 3l18 18" /> </svg>"##;
const HAND_CLICK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13v-8.5a1.5 1.5 0 0 1 3 0v7.5" /> <path d="M11 11.5v-2a1.5 1.5 0 0 1 3 0v2.5" /> <path d="M14 10.5a1.5 1.5 0 0 1 3 0v1.5" /> <path d="M17 11.5a1.5 1.5 0 0 1 3 0v4.5a6 6 0 0 1 -6 6h-2h.208a6 6 0 0 1 -5.012 -2.7l-.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47" /> <path d="M5 3l-1 -1" /> <path d="M4 7h-1" /> <path d="M14 3l1 -1" /> <path d="M15 6h1" /> </svg>"##;
const HAND_CLICK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13v-5" /> <path d="M8.06 4.077a1.5 1.5 0 0 1 2.94 .423v2.5m0 4v1" /> <path d="M12.063 8.065a1.5 1.5 0 0 1 1.937 1.435v.5" /> <path d="M14.06 10.082a1.5 1.5 0 0 1 2.94 .418v1.5" /> <path d="M17 11.5a1.5 1.5 0 0 1 3 0v4.5m-.88 3.129a6 6 0 0 1 -5.12 2.871h-2h.208a6 6 0 0 1 -5.012 -2.7l-.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47" /> <path d="M3 3l18 18" /> <path d="M4 7h-1" /> <path d="M14 3l1 -1" /> <path d="M15 6h1" /> </svg>"##;
const HAND_FINGER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13v-8.5a1.5 1.5 0 0 1 3 0v7.5" /> <path d="M11 11.5v-2a1.5 1.5 0 1 1 3 0v2.5" /> <path d="M14 10.5a1.5 1.5 0 0 1 3 0v1.5" /> <path d="M17 11.5a1.5 1.5 0 0 1 3 0v4.5a6 6 0 0 1 -6 6h-2h.208a6 6 0 0 1 -5.012 -2.7a69.74 69.74 0 0 1 -.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47" /> </svg>"##;
const HAND_FINGER_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12v8.5a1.5 1.5 0 0 0 3 0v-7.5" /> <path d="M11 13.5v2a1.5 1.5 0 0 0 3 0v-2.5" /> <path d="M14 14.5a1.5 1.5 0 0 0 3 0v-1.5" /> <path d="M17 13.5a1.5 1.5 0 0 0 3 0v-4.5a6 6 0 0 0 -6 -6h-2h.208a6 6 0 0 0 -5.012 2.7l-.196 .3q -.468 .718 -3.286 5.728a1.5 1.5 0 0 0 .536 2.022c.734 .44 1.674 .325 2.28 -.28l1.47 -1.47" /> </svg>"##;
const HAND_FINGER_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8h-8.5a1.5 1.5 0 0 0 0 3h7.5" /> <path d="M10.5 11h-2a1.5 1.5 0 1 0 0 3h2.5" /> <path d="M9.5 14a1.5 1.5 0 0 0 0 3h1.5" /> <path d="M10.5 17a1.5 1.5 0 0 0 0 3h4.5a6 6 0 0 0 6 -6v-2v.208a6 6 0 0 0 -2.7 -5.012l-.3 -.196q -.718 -.468 -5.728 -3.286a1.5 1.5 0 0 0 -2.022 .536a1.87 1.87 0 0 0 .28 2.28l1.47 1.47" /> </svg>"##;
const HAND_FINGER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13v-5" /> <path d="M8.06 4.077a1.5 1.5 0 0 1 2.94 .423v2.5m0 4v1" /> <path d="M12.063 8.065a1.5 1.5 0 0 1 1.937 1.435v.5" /> <path d="M14.06 10.082a1.5 1.5 0 0 1 2.94 .418v1.5" /> <path d="M17 11.5a1.5 1.5 0 0 1 3 0v4.5m-.88 3.129a6 6 0 0 1 -5.12 2.871h-2h.208a6 6 0 0 1 -5.012 -2.7l-.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47" /> <path d="M3 3l18 18" /> </svg>"##;
const HAND_FINGER_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8h8.5a1.5 1.5 0 0 1 0 3h-7.5" /> <path d="M13.5 11h2a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M14.5 14a1.5 1.5 0 0 1 0 3h-1.5" /> <path d="M13.5 17a1.5 1.5 0 1 1 0 3h-4.5a6 6 0 0 1 -6 -6v-2v.208a6 6 0 0 1 2.7 -5.012l.3 -.196q .718 -.468 5.728 -3.286a1.5 1.5 0 0 1 2.022 .536c.44 .734 .325 1.674 -.28 2.28l-1.47 1.47" /> </svg>"##;
const HAND_GRAB_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 11v-3.5a1.5 1.5 0 0 1 3 0v2.5" /> <path d="M11 9.5v-3a1.5 1.5 0 0 1 3 0v3.5" /> <path d="M14 7.5a1.5 1.5 0 0 1 3 0v2.5" /> <path d="M17 9.5a1.5 1.5 0 0 1 3 0v4.5a6 6 0 0 1 -6 6h-2h.208a6 6 0 0 1 -5.012 -2.7l-.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47" /> </svg>"##;
const HAND_LITTLE_FINGER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13v-2.5a1.5 1.5 0 0 1 3 0v1.5" /> <path d="M11 11.5v-1a1.5 1.5 0 0 1 3 0v1.5" /> <path d="M17 12v-5.5a1.5 1.5 0 0 1 3 0v9.5a6 6 0 0 1 -6 6h-2h.208a6 6 0 0 1 -5.012 -2.7a69.74 69.74 0 0 1 -.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47" /> <path d="M14 10.5a1.5 1.5 0 0 1 3 0v1.5" /> </svg>"##;
const HAND_LOVE_YOU_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 11.5v-1a1.5 1.5 0 0 1 3 0v1.5" /> <path d="M17 12v-6.5a1.5 1.5 0 0 1 3 0v10.5a6 6 0 0 1 -6 6h-2h.208a6 6 0 0 1 -5.012 -2.7a69.74 69.74 0 0 1 -.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47" /> <path d="M14 10.5a1.5 1.5 0 0 1 3 0v1.5" /> <path d="M8 13v-8.5a1.5 1.5 0 0 1 3 0v7.5" /> </svg>"##;
const HAND_MIDDLE_FINGER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13v-2.5a1.5 1.5 0 0 1 3 0v1.5" /> <path d="M14 10.5a1.5 1.5 0 0 1 3 0v1.5" /> <path d="M17 11.5a1.5 1.5 0 0 1 3 0v4.5a6 6 0 0 1 -6 6h-2h.208a6 6 0 0 1 -5.012 -2.7a69.74 69.74 0 0 1 -.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47" /> <path d="M11 11.5v-8a1.5 1.5 0 1 1 3 0v8.5" /> </svg>"##;
const HAND_MOVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13v-8.5a1.5 1.5 0 0 1 3 0v7.5" /> <path d="M11 11.5v-2a1.5 1.5 0 0 1 3 0v2.5" /> <path d="M14 10.5a1.5 1.5 0 0 1 3 0v1.5" /> <path d="M17 11.5a1.5 1.5 0 0 1 3 0v4.5a6 6 0 0 1 -6 6h-2h.208a6 6 0 0 1 -5.012 -2.7l-.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47" /> <path d="M2.541 5.594a13.487 13.487 0 0 1 2.46 -1.427" /> <path d="M14 3.458c1.32 .354 2.558 .902 3.685 1.612" /> </svg>"##;
const HAND_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M8 13.5v-5.5m.44 -3.562a1.5 1.5 0 0 1 2.56 1.062v1.5m0 4.008v.992m0 -6.5v-2a1.5 1.5 0 1 1 3 0v6.5m0 -4.5a1.5 1.5 0 0 1 3 0v6.5m0 -4.5a1.5 1.5 0 0 1 3 0v8.5a6 6 0 0 1 -6 6h-2c-2.114 -.292 -3.956 -1.397 -5 -3l-2.7 -5.25a1.7 1.7 0 0 1 2.75 -2l.9 1.75" /> </svg>"##;
const HAND_RING_FINGER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13v-2.5a1.5 1.5 0 0 1 3 0v1.5" /> <path d="M17 11.5a1.5 1.5 0 0 1 3 0v4.5a6 6 0 0 1 -6 6h-2h.208a6 6 0 0 1 -5.012 -2.7a69.74 69.74 0 0 1 -.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47" /> <path d="M11 11.5v-2a1.5 1.5 0 1 1 3 0v2.5" /> <path d="M14 12v-6.5a1.5 1.5 0 0 1 3 0v6.5" /> </svg>"##;
const HAND_STOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13v-7.5a1.5 1.5 0 0 1 3 0v6.5" /> <path d="M11 5.5v-2a1.5 1.5 0 1 1 3 0v8.5" /> <path d="M14 5.5a1.5 1.5 0 0 1 3 0v6.5" /> <path d="M17 7.5a1.5 1.5 0 0 1 3 0v8.5a6 6 0 0 1 -6 6h-2h.208a6 6 0 0 1 -5.012 -2.7a69.74 69.74 0 0 1 -.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47" /> </svg>"##;
const HAND_THREE_FINGERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13v-8.5a1.5 1.5 0 0 1 3 0v7.5" /> <path d="M17 11.5a1.5 1.5 0 0 1 3 0v4.5a6 6 0 0 1 -6 6h-2h.208a6 6 0 0 1 -5.012 -2.7a69.74 69.74 0 0 1 -.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47" /> <path d="M11 5.5v-2a1.5 1.5 0 1 1 3 0v8.5" /> <path d="M14 5.5a1.5 1.5 0 0 1 3 0v6.5" /> </svg>"##;
const HAND_TWO_FINGERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13v-8.5a1.5 1.5 0 0 1 3 0v7.5" /> <path d="M17 11.5a1.5 1.5 0 0 1 3 0v4.5a6 6 0 0 1 -6 6h-2h.208a6 6 0 0 1 -5.012 -2.7a69.74 69.74 0 0 1 -.196 -.3c-.312 -.479 -1.407 -2.388 -3.286 -5.728a1.5 1.5 0 0 1 .536 -2.022a1.867 1.867 0 0 1 2.28 .28l1.47 1.47" /> <path d="M14 10.5a1.5 1.5 0 0 1 3 0v1.5" /> <path d="M11 5.5v-2a1.5 1.5 0 1 1 3 0v8.5" /> </svg>"##;
const HASH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 9l14 0" /> <path d="M5 15l14 0" /> <path d="M11 4l-4 16" /> <path d="M17 4l-4 16" /> </svg>"##;
const HDR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 16v-8" /> <path d="M7 8v8" /> <path d="M3 12h4" /> <path d="M10 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2h-2" /> <path d="M17 12h2a2 2 0 1 0 0 -4h-2v8m4 0l-3 -4" /> </svg>"##;
const HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.5 12.572l-7.5 7.428l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 1 1 7.5 6.572" /> </svg>"##;
const HEART_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19l-1 1l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 0 1 8.003 5.997" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const HEART_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 0 1 7.993 6.01" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const HEART_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.5 12.572l-3 2.928m-5.5 3.5a8916.99 8916.99 0 0 0 -6.5 -6.428a5 5 0 1 1 7.5 -6.566a5 5 0 1 1 7.5 6.572" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const HEART_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.5 12.572l-.536 .53m-7.91 5.96l-6.554 -6.489a5 5 0 1 1 7.5 -6.567a5 5 0 1 1 7.5 6.572" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const HEART_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 0 1 8.21 5.697" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const HEART_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19l-1 1l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 0 1 8.785 4.444" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const HEART_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 1 1 7.907 6.12" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const HEART_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.03 17l-3.03 3l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 1 1 7.922 6.102" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const HEART_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.5 12.572l-2.494 2.47m-5.006 4.958l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 1 1 7.5 6.572" /> <path d="M16 19h6" /> </svg>"##;
const HEART_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M19.5 12.572l-1.5 1.428m-2 2l-4 4l-7.5 -7.428a5 5 0 0 1 -1.288 -5.068a4.976 4.976 0 0 1 1.788 -2.504m3 -1c1.56 0 3.05 .727 4 2a5 5 0 1 1 7.5 6.572" /> </svg>"##;
const HEART_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.5 12.572l-.784 .777m-5.725 5.67l-.991 .981l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 1 1 7.5 6.572" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const HEART_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 0 1 8.5 5.179" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const HEART_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 1 1 7.96 6.053" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const HEART_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.105 17.915l-2.105 2.085l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 0 1 8.524 5.127" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const HEART_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-.975 -.966l-6.525 -6.462a5 5 0 1 1 7.5 -6.566a5 5 0 0 1 8.37 5.428" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const HEART_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.5 12.572l-.468 .464m-6.077 6.019l-.955 .945l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 1 1 7.5 6.572" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const HEART_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.537 19.542l-7.037 -6.97a5 5 0 1 1 7.5 -6.566a5 5 0 0 1 8.212 5.693" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const HEART_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.73 17.753l-5.23 -5.181a5 5 0 1 1 7.5 -6.566a5 5 0 0 1 8.563 5.041" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const HEART_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 1 1 7.893 6.139" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const HEART_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.5 12.572l-.5 .428m-6 6l-1 1l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 1 1 7.5 6.572" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const HEARTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.017 18l-2.017 2l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 0 1 8.153 5.784" /> <path d="M15.99 20l4.197 -4.223a2.81 2.81 0 0 0 0 -3.948a2.747 2.747 0 0 0 -3.91 -.007l-.28 .282l-.279 -.283a2.747 2.747 0 0 0 -3.91 -.007a2.81 2.81 0 0 0 -.007 3.948l4.182 4.238l.007 0" /> </svg>"##;
const HEARTS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.017 18l-2.017 2l-7.5 -7.428a5 5 0 0 1 .49 -7.586m3.01 -1a5 5 0 0 1 4 2.018a5 5 0 0 1 8.153 5.784" /> <path d="M11.814 11.814a2.81 2.81 0 0 0 -.007 3.948l4.182 4.238l2.01 -2.021m1.977 -1.99l.211 -.212a2.81 2.81 0 0 0 0 -3.948a2.747 2.747 0 0 0 -3.91 -.007l-.283 .178" /> <path d="M3 3l18 18" /> </svg>"##;
const HEMISPHERE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9a9 3 0 1 0 18 0a9 3 0 1 0 -18 0" /> <path d="M3 9a9 9 0 0 0 18 0" /> </svg>"##;
const HEMISPHERE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.588 6.603c-2.178 .547 -3.588 1.417 -3.588 2.397c0 1.657 4.03 3 9 3m3.72 -.267c3.114 -.473 5.28 -1.518 5.28 -2.733c0 -1.657 -4.03 -3 -9 -3c-.662 0 -1.308 .024 -1.93 .07" /> <path d="M3 9a9 9 0 0 0 13.677 7.69m2.165 -1.843a8.965 8.965 0 0 0 2.158 -5.847" /> <path d="M3 3l18 18" /> </svg>"##;
const HEMISPHERE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9a9 3 0 1 0 18 0a9 3 0 1 0 -18 0" /> <path d="M3 9a9 9 0 0 0 9 9m8.396 -5.752a8.978 8.978 0 0 0 .604 -3.248" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const HEXAGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> </svg>"##;
const HEXAGON_3D_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 6.844a2.007 2.007 0 0 1 1 1.752v6.555c0 .728 -.394 1.399 -1.03 1.753l-6 3.844a2 2 0 0 1 -1.942 0l-6 -3.844a2.007 2.007 0 0 1 -1.029 -1.752v-6.556c0 -.729 .394 -1.4 1.029 -1.753l6 -3.583a2.05 2.05 0 0 1 2 0l6 3.584h-.03l.002 0" /> <path d="M12 16.5v4.5" /> <path d="M4.5 7.5l3.5 2.5" /> <path d="M16 10l4 -2.5" /> <path d="M12 7.5v4.5l-4 2" /> <path d="M12 12l4 2" /> <path d="M12 16.5l4 -2.5v-4l-4 -2.5l-4 2.5v4l4 2.5" /> </svg>"##;
const HEXAGON_ASTERISK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27c.7 .398 1.13 1.143 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.27 2.27 0 0 1 -2.184 0l-6.75 -4.27a2.23 2.23 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98l-.033 0" /> <path d="M12 8.5v7" /> <path d="M9 10l6 4" /> <path d="M9 14l6 -4" /> </svg>"##;
const HEXAGON_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27c.7 .398 1.13 1.143 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M9 12h6" /> </svg>"##;
const HEXAGON_MINUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.092 21.72a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033c.7 .398 1.13 1.143 1.125 1.948v6.782" /> <path d="M16 19h6" /> </svg>"##;
const HEXAGON_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.693 4.69l2.336 -1.39a2.056 2.056 0 0 1 2 0l6 3.573h-.029a2 2 0 0 1 1 1.747v6.536c0 .246 -.045 .485 -.13 .707m-2.16 1.847l-4.739 3.027a2 2 0 0 1 -1.942 0l-6 -3.833a2 2 0 0 1 -1.029 -1.747v-6.537a2 2 0 0 1 1.029 -1.748l1.154 -.687" /> <path d="M3 3l18 18" /> </svg>"##;
const HEXAGON_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 6.27c.7 .398 1.13 1.143 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M9 12h6" /> <path d="M12 9v6" /> </svg>"##;
const HEXAGON_PLUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.092 21.72a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033c.7 .398 1.13 1.143 1.125 1.948v4.282" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const HEXAGONAL_PRISM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.792 6.996l-3.775 2.643a2.005 2.005 0 0 1 -1.147 .361h-7.74c-.41 0 -.81 -.126 -1.146 -.362l-3.774 -2.641" /> <path d="M8 10v11" /> <path d="M16 10v11" /> <path d="M3.853 18.274l3.367 2.363a2 2 0 0 0 1.147 .363h7.265c.41 0 .811 -.126 1.147 -.363l3.367 -2.363c.536 -.375 .854 -.99 .854 -1.643v-9.262c0 -.655 -.318 -1.268 -.853 -1.643l-3.367 -2.363a2 2 0 0 0 -1.147 -.363h-7.266c-.41 0 -.811 .126 -1.147 .363l-3.367 2.363a2.006 2.006 0 0 0 -.853 1.644v9.261c0 .655 .318 1.269 .853 1.644" /> </svg>"##;
const HEXAGONAL_PRISM_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.792 6.996l-3.775 2.643a2.005 2.005 0 0 1 -1.147 .361h-1.87m-4 0h-1.87c-.41 0 -.81 -.126 -1.146 -.362l-3.774 -2.641" /> <path d="M8 10v11" /> <path d="M16 10v2m0 4v5" /> <path d="M20.972 16.968a2.01 2.01 0 0 0 .028 -.337v-9.262c0 -.655 -.318 -1.268 -.853 -1.643l-3.367 -2.363a2 2 0 0 0 -1.147 -.363h-7.266a1.99 1.99 0 0 0 -1.066 .309m-2.345 1.643l-1.103 .774a2.006 2.006 0 0 0 -.853 1.644v9.261c0 .655 .318 1.269 .853 1.644l3.367 2.363a2 2 0 0 0 1.147 .362h7.265c.41 0 .811 -.126 1.147 -.363l2.26 -1.587" /> <path d="M3 3l18 18" /> </svg>"##;
const HEXAGONAL_PRISM_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.792 6.996l-3.775 2.643a2.005 2.005 0 0 1 -1.147 .361h-7.74c-.41 0 -.81 -.126 -1.146 -.362l-3.774 -2.641" /> <path d="M8 10v11" /> <path d="M16 10v3.5" /> <path d="M21 12.5v-5.131c0 -.655 -.318 -1.268 -.853 -1.643l-3.367 -2.363a2 2 0 0 0 -1.147 -.363h-7.266c-.41 0 -.811 .126 -1.147 .363l-3.367 2.363a2.006 2.006 0 0 0 -.853 1.644v9.261c0 .655 .318 1.269 .853 1.644l3.367 2.363a2 2 0 0 0 1.147 .362h4.133" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const HEXAGONAL_PYRAMID_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.162 2.457l-7.846 12.954a1.988 1.988 0 0 0 .267 2.483l2.527 2.523c.374 .373 .88 .583 1.408 .583h8.964c.528 0 1.034 -.21 1.408 -.583l2.527 -2.523a1.988 1.988 0 0 0 .267 -2.483l-7.846 -12.954a.996 .996 0 0 0 -1.676 0" /> <path d="M12 2l-5 18.9" /> <path d="M12 2l5 18.9" /> </svg>"##;
const HEXAGONAL_PYRAMID_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.877 7.88l-4.56 7.53a1.988 1.988 0 0 0 .266 2.484l2.527 2.523c.374 .373 .88 .583 1.408 .583h8.964c.528 0 1.034 -.21 1.408 -.583l1.264 -1.263m1.792 -2.205a1.986 1.986 0 0 0 -.262 -1.538l-7.846 -12.954a.996 .996 0 0 0 -1.676 0l-1.772 2.926" /> <path d="M12 2l-1.254 4.742m-.841 3.177l-2.905 10.981" /> <path d="M12 2l2.153 8.14m1.444 5.457l1.403 5.303" /> <path d="M3 3l18 18" /> </svg>"##;
const HEXAGONAL_PYRAMID_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.642 12.04l-5.804 -9.583a.996 .996 0 0 0 -1.676 0l-7.846 12.954a1.988 1.988 0 0 0 .267 2.483l2.527 2.523c.374 .373 .88 .583 1.408 .583h4.982" /> <path d="M12 2l-5 18.9" /> <path d="M12 2l3.304 12.489" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const HEXAGONS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18v-5l4 -2l4 2v5l-4 2l-4 -2" /> <path d="M8 11v-5l4 -2l4 2v5" /> <path d="M12 13l4 -2l4 2v5l-4 2l-4 -2" /> </svg>"##;
const HEXAGONS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18v-5l4 -2l4 2v5l-4 2l-4 -2" /> <path d="M8 11v-3m1.332 -2.666l2.668 -1.334l4 2v5" /> <path d="M12 13l.661 -.331" /> <path d="M15.345 11.328l.655 -.328l4 2v3m-1.334 2.667l-2.666 1.333l-4 -2" /> <path d="M3 3l18 18" /> </svg>"##;
const HIERARCHY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6.5 17.5l5.5 -4.5l5.5 4.5" /> <path d="M12 7l0 6" /> </svg>"##;
const HIERARCHY_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 3h4v4h-4l0 -4" /> <path d="M3 17h4v4h-4l0 -4" /> <path d="M17 17h4v4h-4l0 -4" /> <path d="M7 17l5 -4l5 4" /> <path d="M12 7l0 6" /> </svg>"##;
const HIERARCHY_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M18 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M2 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M14 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 17l2 -3" /> <path d="M9 10l2 -3" /> <path d="M13 7l2 3" /> <path d="M17 14l2 3" /> <path d="M15 14l-2 3" /> <path d="M9 14l2 3" /> </svg>"##;
const HIERARCHY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17.585 17.587a2 2 0 0 0 2.813 2.843" /> <path d="M6.5 17.5l5.5 -4.5l5.5 4.5" /> <path d="M12 7v1m0 4v1" /> <path d="M3 3l18 18" /> </svg>"##;
const ICONS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6.5a3.5 3.5 0 1 0 7 0a3.5 3.5 0 1 0 -7 0" /> <path d="M2.5 21h8l-4 -7l-4 7" /> <path d="M14 3l7 7" /> <path d="M14 10l7 -7" /> <path d="M14 14h7v7h-7l0 -7" /> </svg>"##;
const ICONS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.01 4.041a3.5 3.5 0 0 0 2.49 5.959c.975 0 1.865 -.357 2.5 -1m.958 -3.044a3.503 3.503 0 0 0 -2.905 -2.912" /> <path d="M2.5 21h8l-4 -7l-4 7" /> <path d="M14 3l7 7" /> <path d="M14 10l7 -7" /> <path d="M18 14h3v3m0 4h-7v-7" /> <path d="M3 3l18 18" /> </svg>"##;
const IKOSAEDR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 8.007v7.986a2 2 0 0 1 -1.006 1.735l-7 4.007a2 2 0 0 1 -1.988 0l-7 -4.007a2 2 0 0 1 -1.006 -1.735v-7.986a2 2 0 0 1 1.006 -1.735l7 -4.007a2 2 0 0 1 1.988 0l7 4.007a2 2 0 0 1 1.006 1.735" /> <path d="M3.29 6.97l4.21 2.03" /> <path d="M20.71 6.97l-4.21 2.03" /> <path d="M20.7 17h-17.4" /> <path d="M11.76 2.03l-4.26 6.97l-4.3 7.84" /> <path d="M12.24 2.03q 2.797 4.44 4.26 6.97t 4.3 7.84" /> <path d="M12 17l-4.5 -8h9l-4.5 8" /> <path d="M12 17v5" /> </svg>"##;
const IMAGE_IN_PICTURE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 15c-2 0 -5 1 -5 5" /> <path d="M4 13a2 2 0 0 1 2 -2h5a2 2 0 0 1 2 2v5a2 2 0 0 1 -2 2h-5a2 2 0 0 1 -2 -2l0 -5" /> <path d="M4 7v-2a1 1 0 0 1 1 -1h2" /> <path d="M11 4h2" /> <path d="M17 4h2a1 1 0 0 1 1 1v2" /> <path d="M20 11v2" /> <path d="M20 17v2a1 1 0 0 1 -1 1h-2" /> </svg>"##;
const INNER_SHADOW_BOTTOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.364 18.364a9 9 0 1 0 -12.728 -12.728a9 9 0 0 0 12.728 12.728" /> <path d="M7.757 16.243a6 6 0 0 0 8.486 0" /> </svg>"##;
const INNER_SHADOW_BOTTOM_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M6 12a6 6 0 0 0 6 6" /> </svg>"##;
const INNER_SHADOW_BOTTOM_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> <path d="M18 12a6 6 0 0 1 -6 6" /> </svg>"##;
const INNER_SHADOW_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.636 5.636a9 9 0 1 1 12.728 12.728a9 9 0 0 1 -12.728 -12.728" /> <path d="M7.757 16.243a6 6 0 0 1 0 -8.486" /> </svg>"##;
const INNER_SHADOW_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.364 18.364a9 9 0 1 1 -12.728 -12.728a9 9 0 0 1 12.728 12.728" /> <path d="M16.243 7.757a6 6 0 0 1 0 8.486" /> </svg>"##;
const INNER_SHADOW_TOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.636 5.636a9 9 0 1 0 12.728 12.728a9 9 0 0 0 -12.728 -12.728" /> <path d="M16.243 7.757a6 6 0 0 0 -8.486 0" /> </svg>"##;
const INNER_SHADOW_TOP_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a9 9 0 1 1 0 18a9 9 0 0 1 0 -18" /> <path d="M6 12a6 6 0 0 1 6 -6" /> </svg>"##;
const INNER_SHADOW_TOP_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a9 9 0 1 0 0 18a9 9 0 0 0 0 -18" /> <path d="M18 12a6 6 0 0 0 -6 -6" /> </svg>"##;
const IRREGULAR_POLYHEDRON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 12l-1.752 6.13a1 1 0 0 0 .592 1.205l6.282 2.503a2.46 2.46 0 0 0 1.756 0l6.282 -2.503a1 1 0 0 0 .592 -1.204l-1.752 -6.131l1.752 -6.13a1 1 0 0 0 -.592 -1.205l-6.282 -2.503a2.46 2.46 0 0 0 -1.756 0l-6.282 2.503a1 1 0 0 0 -.592 1.204l1.752 6.131" /> <path d="M4.5 5.5l6.622 2.33a2.35 2.35 0 0 0 1.756 0l6.622 -2.33" /> <path d="M6 12l5.21 1.862a2.34 2.34 0 0 0 1.58 0l5.21 -1.862" /> <path d="M12 22v-14" /> </svg>"##;
const IRREGULAR_POLYHEDRON_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.706 4.73a1 1 0 0 0 -.458 1.14l1.752 6.13l-1.752 6.13a1 1 0 0 0 .592 1.205l6.282 2.503a2.46 2.46 0 0 0 1.756 0l6.282 -2.503c.04 -.016 .079 -.035 .116 -.055m-.474 -4.474l-.802 -2.806l1.752 -6.13a1 1 0 0 0 -.592 -1.205l-6.282 -2.503a2.46 2.46 0 0 0 -1.756 0l-3.544 1.412" /> <path d="M4.5 5.5c.661 .214 1.161 .38 1.5 .5m6 2c.29 -.003 .603 -.06 .878 -.17l6.622 -2.33" /> <path d="M6 12l5.21 1.862a2.34 2.34 0 0 0 1.58 0l.742 -.265m2.956 -1.057c.312 -.11 .816 -.291 1.512 -.54" /> <path d="M12 22v-10" /> <path d="M3 3l18 18" /> </svg>"##;
const IRREGULAR_POLYHEDRON_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 12l1.752 -6.13a1 1 0 0 0 -.592 -1.205l-6.282 -2.503a2.46 2.46 0 0 0 -1.756 0l-6.282 2.503a1 1 0 0 0 -.592 1.204l1.752 6.131l-1.752 6.13a1 1 0 0 0 .592 1.205l6.282 2.503a2.46 2.46 0 0 0 1.756 0l.221 -.088" /> <path d="M4.5 5.5l6.622 2.33a2.35 2.35 0 0 0 1.756 0l6.622 -2.33" /> <path d="M6 12l5.21 1.862a2.34 2.34 0 0 0 1.58 0l5.21 -1.862" /> <path d="M12 22v-14" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const JEWISH_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 2l3 5h6l-3 5l3 5h-6l-3 5l-3 -5h-6l3 -5l-3 -5h6l3 -5" /> </svg>"##;
const KEY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.555 3.843l3.602 3.602a2.877 2.877 0 0 1 0 4.069l-2.643 2.643a2.877 2.877 0 0 1 -4.069 0l-.301 -.301l-6.558 6.558a2 2 0 0 1 -1.239 .578l-.175 .008h-1.172a1 1 0 0 1 -.993 -.883l-.007 -.117v-1.172a2 2 0 0 1 .467 -1.284l.119 -.13l.414 -.414h2v-2h2v-2l2.144 -2.144l-.301 -.301a2.877 2.877 0 0 1 0 -4.069l2.643 -2.643a2.877 2.877 0 0 1 4.069 0" /> <path d="M15 9h.01" /> </svg>"##;
const KEY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.17 6.159l2.316 -2.316a2.877 2.877 0 0 1 4.069 0l3.602 3.602a2.877 2.877 0 0 1 0 4.069l-2.33 2.33" /> <path d="M14.931 14.948a2.863 2.863 0 0 1 -1.486 -.79l-.301 -.302l-6.558 6.558a2 2 0 0 1 -1.239 .578l-.175 .008h-1.172a1 1 0 0 1 -.993 -.883l-.007 -.117v-1.172a2 2 0 0 1 .467 -1.284l.119 -.13l.414 -.414h2v-2h2v-2l2.144 -2.144l-.301 -.301a2.863 2.863 0 0 1 -.794 -1.504" /> <path d="M15 9h.01" /> <path d="M3 3l18 18" /> </svg>"##;
const LADLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 14v1a6 6 0 1 0 12 0v-9a3 3 0 0 1 6 0" /> <path d="M9 16c-.663 0 -1.3 -.036 -1.896 -.102l-.5 -.064c-2.123 -.308 -3.604 -1.013 -3.604 -1.834c0 -.82 1.482 -1.526 3.603 -1.834l.5 -.064a17.27 17.27 0 0 1 1.897 -.102c.663 0 1.3 .036 1.896 .102l.5 .064c2.123 .308 3.604 1.013 3.604 1.834c0 .82 -1.482 1.526 -3.603 1.834l-.5 .064a17.27 17.27 0 0 1 -1.897 .102" /> </svg>"##;
const LAMP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 20h6" /> <path d="M12 20v-8" /> <path d="M5 12h14l-4 -8h-6l-4 8" /> </svg>"##;
const LAMP_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21h9" /> <path d="M10 21l-7 -8l8.5 -5.5" /> <path d="M13 14c-2.148 -2.148 -2.148 -5.852 0 -8c2.088 -2.088 5.842 -1.972 8 0l-8 8" /> <path d="M11.742 7.574l-1.156 -1.156a2 2 0 0 1 2.828 -2.829l1.144 1.144" /> <path d="M15.5 12l.208 .274a2.527 2.527 0 0 0 3.556 0c.939 -.933 .98 -2.42 .122 -3.4l-.366 -.369" /> </svg>"##;
const LAMP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 20h6" /> <path d="M12 20v-8" /> <path d="M7.325 7.35l-2.325 4.65h7m4 0h3l-4 -8h-6l-.338 .676" /> <path d="M3 3l18 18" /> </svg>"##;
const LASSO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.028 13.252c-.657 -.972 -1.028 -2.078 -1.028 -3.252c0 -3.866 4.03 -7 9 -7s9 3.134 9 7s-4.03 7 -9 7c-1.913 0 -3.686 -.464 -5.144 -1.255" /> <path d="M3 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 17c0 1.42 .316 2.805 1 4" /> </svg>"##;
const LASSO_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.028 13.252c-.657 -.972 -1.028 -2.078 -1.028 -3.252c0 -1.804 .878 -3.449 2.319 -4.69m2.49 -1.506a11.066 11.066 0 0 1 4.191 -.804c4.97 0 9 3.134 9 7c0 1.799 -.873 3.44 -2.307 4.68m-2.503 1.517a11.066 11.066 0 0 1 -4.19 .803c-1.913 0 -3.686 -.464 -5.144 -1.255" /> <path d="M3 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 17c0 1.42 .316 2.805 1 4" /> <path d="M3 3l18 18" /> </svg>"##;
const LASSO_POLYGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.028 13.252l-1.028 -3.252l2 -7l7 5l8 -3l1 9l-9 3l-5.144 -1.255" /> <path d="M3 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 17c0 1.42 .316 2.805 1 4" /> </svg>"##;
const LAUREL_WREATH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.436 8a8.6 8.6 0 0 0 -.436 2.727c0 4.017 2.686 7.273 6 7.273s6 -3.256 6 -7.273a8.6 8.6 0 0 0 -.436 -2.727" /> <path d="M14.5 21s-.682 -3 -2.5 -3s-2.5 3 -2.5 3" /> <path d="M18.52 5.23c.292 1.666 -1.02 2.77 -1.02 2.77s-1.603 -.563 -1.895 -2.23c-.292 -1.666 1.02 -2.77 1.02 -2.77s1.603 .563 1.895 2.23" /> <path d="M21.094 12.14c-1.281 1.266 -3.016 .76 -3.016 .76s-.454 -1.772 .828 -3.04c1.28 -1.266 3.016 -.76 3.016 -.76s.454 1.772 -.828 3.04" /> <path d="M17.734 18.826c-1.5 -.575 -1.734 -2.19 -1.734 -2.19s1.267 -1.038 2.767 -.462c1.5 .575 1.733 2.19 1.733 2.19s-1.267 1.038 -2.767 .462" /> <path d="M6.267 18.826c1.5 -.575 1.733 -2.19 1.733 -2.19s-1.267 -1.038 -2.767 -.462c-1.5 .575 -1.733 2.19 -1.733 2.19s1.267 1.038 2.767 .462" /> <path d="M2.906 12.14c1.281 1.266 3.016 .76 3.016 .76s.454 -1.772 -.828 -3.04c-1.281 -1.265 -3.016 -.76 -3.016 -.76s-.454 1.772 .828 3.04" /> <path d="M5.48 5.23c-.292 1.666 1.02 2.77 1.02 2.77s1.603 -.563 1.895 -2.23c.292 -1.666 -1.02 -2.77 -1.02 -2.77s-1.603 .563 -1.895 2.23" /> </svg>"##;
const LAUREL_WREATH_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.436 8a8.6 8.6 0 0 0 -.436 2.727c0 4.017 2.686 7.273 6 7.273s6 -3.256 6 -7.273a8.6 8.6 0 0 0 -.436 -2.727" /> <path d="M14.5 21s-.682 -3 -2.5 -3s-2.5 3 -2.5 3" /> <path d="M18.52 5.23c.292 1.666 -1.02 2.77 -1.02 2.77s-1.603 -.563 -1.895 -2.23c-.292 -1.666 1.02 -2.77 1.02 -2.77s1.603 .563 1.895 2.23" /> <path d="M21.094 12.14c-1.281 1.266 -3.016 .76 -3.016 .76s-.454 -1.772 .828 -3.04c1.28 -1.266 3.016 -.76 3.016 -.76s.454 1.772 -.828 3.04" /> <path d="M17.734 18.826c-1.5 -.575 -1.734 -2.19 -1.734 -2.19s1.267 -1.038 2.767 -.462c1.5 .575 1.733 2.19 1.733 2.19s-1.267 1.038 -2.767 .462" /> <path d="M6.267 18.826c1.5 -.575 1.733 -2.19 1.733 -2.19s-1.267 -1.038 -2.767 -.462c-1.5 .575 -1.733 2.19 -1.733 2.19s1.267 1.038 2.767 .462" /> <path d="M2.906 12.14c1.281 1.266 3.016 .76 3.016 .76s.454 -1.772 -.828 -3.04c-1.281 -1.265 -3.016 -.76 -3.016 -.76s-.454 1.772 .828 3.04" /> <path d="M5.48 5.23c-.292 1.666 1.02 2.77 1.02 2.77s1.603 -.563 1.895 -2.23c.292 -1.666 -1.02 -2.77 -1.02 -2.77s-1.603 .563 -1.895 2.23" /> <path d="M11 9l1 -1v6" /> </svg>"##;
const LAUREL_WREATH_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.436 8a8.6 8.6 0 0 0 -.436 2.727c0 4.017 2.686 7.273 6 7.273s6 -3.256 6 -7.273a8.6 8.6 0 0 0 -.436 -2.727" /> <path d="M14.5 21s-.682 -3 -2.5 -3s-2.5 3 -2.5 3" /> <path d="M18.52 5.23c.292 1.666 -1.02 2.77 -1.02 2.77s-1.603 -.563 -1.895 -2.23c-.292 -1.666 1.02 -2.77 1.02 -2.77s1.603 .563 1.895 2.23" /> <path d="M21.094 12.14c-1.281 1.266 -3.016 .76 -3.016 .76s-.454 -1.772 .828 -3.04c1.28 -1.266 3.016 -.76 3.016 -.76s.454 1.772 -.828 3.04" /> <path d="M17.734 18.826c-1.5 -.575 -1.734 -2.19 -1.734 -2.19s1.267 -1.038 2.767 -.462c1.5 .575 1.733 2.19 1.733 2.19s-1.267 1.038 -2.767 .462" /> <path d="M6.267 18.826c1.5 -.575 1.733 -2.19 1.733 -2.19s-1.267 -1.038 -2.767 -.462c-1.5 .575 -1.733 2.19 -1.733 2.19s1.267 1.038 2.767 .462" /> <path d="M2.906 12.14c1.281 1.266 3.016 .76 3.016 .76s.454 -1.772 -.828 -3.04c-1.281 -1.265 -3.016 -.76 -3.016 -.76s-.454 1.772 .828 3.04" /> <path d="M5.48 5.23c-.292 1.666 1.02 2.77 1.02 2.77s1.603 -.563 1.895 -2.23c.292 -1.666 -1.02 -2.77 -1.02 -2.77s-1.603 .563 -1.895 2.23" /> <path d="M10.6 8h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2" /> </svg>"##;
const LAUREL_WREATH_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.436 8a8.6 8.6 0 0 0 -.436 2.727c0 4.017 2.686 7.273 6 7.273s6 -3.256 6 -7.273a8.6 8.6 0 0 0 -.436 -2.727" /> <path d="M14.5 21s-.682 -3 -2.5 -3s-2.5 3 -2.5 3" /> <path d="M18.52 5.23c.292 1.666 -1.02 2.77 -1.02 2.77s-1.603 -.563 -1.895 -2.23c-.292 -1.666 1.02 -2.77 1.02 -2.77s1.603 .563 1.895 2.23" /> <path d="M21.094 12.14c-1.281 1.266 -3.016 .76 -3.016 .76s-.454 -1.772 .828 -3.04c1.28 -1.266 3.016 -.76 3.016 -.76s.454 1.772 -.828 3.04" /> <path d="M17.734 18.826c-1.5 -.575 -1.734 -2.19 -1.734 -2.19s1.267 -1.038 2.767 -.462c1.5 .575 1.733 2.19 1.733 2.19s-1.267 1.038 -2.767 .462" /> <path d="M6.267 18.826c1.5 -.575 1.733 -2.19 1.733 -2.19s-1.267 -1.038 -2.767 -.462c-1.5 .575 -1.733 2.19 -1.733 2.19s1.267 1.038 2.767 .462" /> <path d="M2.906 12.14c1.281 1.266 3.016 .76 3.016 .76s.454 -1.772 -.828 -3.04c-1.281 -1.265 -3.016 -.76 -3.016 -.76s-.454 1.772 .828 3.04" /> <path d="M5.48 5.23c-.292 1.666 1.02 2.77 1.02 2.77s1.603 -.563 1.895 -2.23c.292 -1.666 -1.02 -2.77 -1.02 -2.77s-1.603 .563 -1.895 2.23" /> <path d="M10.5 8h1.5a1.5 1.5 0 0 1 0 3h-1h1a1.5 1.5 0 0 1 0 3h-1.5" /> </svg>"##;
const LAYERS_DIFFERENCE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 16v2a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h2v-2a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-2" /> <path d="M10 8l-2 0l0 2" /> <path d="M8 14l0 2l2 0" /> <path d="M14 8l2 0l0 2" /> <path d="M16 14l0 2l-2 0" /> </svg>"##;
const LAYERS_INTERSECT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 6a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /> <path d="M4 10a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /> </svg>"##;
const LAYERS_INTERSECT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 6a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /> <path d="M4 10a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /> <path d="M9 15l6 -6" /> </svg>"##;
const LAYERS_LINKED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 8.268a2 2 0 0 1 1 1.732v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h3" /> <path d="M5 15.734a2 2 0 0 1 -1 -1.734v-8a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-3" /> </svg>"##;
const LAYERS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.59 4.581c.362 -.359 .86 -.581 1.41 -.581h8a2 2 0 0 1 2 2v8c0 .556 -.227 1.06 -.594 1.422m-3.406 .578h-6a2 2 0 0 1 -2 -2v-6" /> <path d="M16 16v2a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h2" /> <path d="M3 3l18 18" /> </svg>"##;
const LAYERS_SELECTED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 10.5l6.492 -6.492" /> <path d="M13.496 16l6.504 -6.504l-6.504 6.504" /> <path d="M8.586 15.414l10.827 -10.827" /> <path d="M8 6a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /> <path d="M16 16v2a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h2" /> </svg>"##;
const LAYERS_SELECTED_BOTTOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 14.5l4 -4" /> <path d="M9.496 20l4.004 -4l-4.004 4" /> <path d="M4.586 19.414l3.914 -3.914" /> <path d="M8 6a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /> <path d="M16 16v2a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h2" /> </svg>"##;
const LAYERS_SUBTRACT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 6a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /> <path d="M16 16v2a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h2" /> </svg>"##;
const LAYERS_UNION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 16v2a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h2v-2a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-2" /> </svg>"##;
const LAYOUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v1a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -1" /> <path d="M4 15a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -3" /> <path d="M14 6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -12" /> </svg>"##;
const LAYOUT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v1a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -1" /> <path d="M4 15a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -3" /> <path d="M14 6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -3" /> <path d="M14 17a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v1a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -1" /> </svg>"##;
const LAYOUT_ALIGN_BOTTOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20l16 0" /> <path d="M9 6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -8" /> </svg>"##;
const LAYOUT_ALIGN_CENTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4l0 5" /> <path d="M12 15l0 5" /> <path d="M6 11a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -2" /> </svg>"##;
const LAYOUT_ALIGN_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4l0 16" /> <path d="M8 11a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -2" /> </svg>"##;
const LAYOUT_ALIGN_MIDDLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12l5 0" /> <path d="M15 12l5 0" /> <path d="M9 8a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -8" /> </svg>"##;
const LAYOUT_ALIGN_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 4l0 16" /> <path d="M4 11a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -2" /> </svg>"##;
const LAYOUT_ALIGN_TOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4l16 0" /> <path d="M9 10a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -8" /> </svg>"##;
const LAYOUT_BOARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M4 9h8" /> <path d="M12 15h8" /> <path d="M12 4v16" /> </svg>"##;
const LAYOUT_BOARD_SPLIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M4 12h8" /> <path d="M12 15h8" /> <path d="M12 9h8" /> <path d="M12 4v16" /> </svg>"##;
const LAYOUT_BOTTOMBAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M4 15l16 0" /> </svg>"##;
const LAYOUT_BOTTOMBAR_COLLAPSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 6v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2" /> <path d="M20 15h-16" /> <path d="M14 8l-2 2l-2 -2" /> </svg>"##;
const LAYOUT_BOTTOMBAR_EXPAND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 6v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2" /> <path d="M20 15h-16" /> <path d="M14 10l-2 -2l-2 2" /> </svg>"##;
const LAYOUT_CARDS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -12" /> <path d="M14 6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v6a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -6" /> </svg>"##;
const LAYOUT_COLLAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M10 4l4 16" /> <path d="M12 12l-8 2" /> </svg>"##;
const LAYOUT_COLUMNS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M12 4l0 16" /> </svg>"##;
const LAYOUT_DASHBOARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h4a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-6a1 1 0 0 1 1 -1" /> <path d="M5 16h4a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1" /> <path d="M15 12h4a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-6a1 1 0 0 1 1 -1" /> <path d="M15 4h4a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1" /> </svg>"##;
const LAYOUT_DISTRIBUTE_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4l16 0" /> <path d="M4 20l16 0" /> <path d="M6 11a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -2" /> </svg>"##;
const LAYOUT_DISTRIBUTE_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4l0 16" /> <path d="M20 4l0 16" /> <path d="M9 8a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -8" /> </svg>"##;
const LAYOUT_GRID_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M14 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M4 15a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M14 15a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> </svg>"##;
const LAYOUT_GRID_ADD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M14 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M4 15a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M14 17h6m-3 -3v6" /> </svg>"##;
const LAYOUT_GRID_REMOVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-4" /> <path d="M14 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-4" /> <path d="M4 15a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-4" /> <path d="M14 17h6" /> </svg>"##;
const LAYOUT_KANBAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4l6 0" /> <path d="M14 4l6 0" /> <path d="M4 10a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -8" /> <path d="M14 10a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -2" /> </svg>"##;
const LAYOUT_LIST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -2" /> <path d="M4 16a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -2" /> </svg>"##;
const LAYOUT_NAVBAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M4 9l16 0" /> </svg>"##;
const LAYOUT_NAVBAR_COLLAPSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2" /> <path d="M4 9h16" /> <path d="M10 16l2 -2l2 2" /> </svg>"##;
const LAYOUT_NAVBAR_EXPAND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2" /> <path d="M4 9h16" /> <path d="M10 14l2 2l2 -2" /> </svg>"##;
const LAYOUT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4a2 2 0 0 1 2 2m-1.162 2.816a1.993 1.993 0 0 1 -.838 .184h-2a2 2 0 0 1 -2 -2v-1c0 -.549 .221 -1.046 .58 -1.407" /> <path d="M4 15a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v3a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -3" /> <path d="M14 10v-4a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v10m-.595 3.423a2 2 0 0 1 -1.405 .577h-2a2 2 0 0 1 -2 -2v-4" /> <path d="M3 3l18 18" /> </svg>"##;
const LAYOUT_ROWS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M4 12l16 0" /> </svg>"##;
const LAYOUT_SIDEBAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M9 4l0 16" /> </svg>"##;
const LAYOUT_SIDEBAR_LEFT_COLLAPSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M9 4v16" /> <path d="M15 10l-2 2l2 2" /> </svg>"##;
const LAYOUT_SIDEBAR_LEFT_EXPAND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M9 4v16" /> <path d="M14 10l2 2l-2 2" /> </svg>"##;
const LAYOUT_SIDEBAR_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M15 4l0 16" /> </svg>"##;
const LAYOUT_SIDEBAR_RIGHT_COLLAPSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M15 4v16" /> <path d="M9 10l2 2l-2 2" /> </svg>"##;
const LAYOUT_SIDEBAR_RIGHT_EXPAND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M15 4v16" /> <path d="M10 10l-2 2l2 2" /> </svg>"##;
const LINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7.5 16.5l9 -9" /> </svg>"##;
const LINE_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h2" /> <path d="M17 12h2" /> <path d="M11 12h2" /> </svg>"##;
const LINE_DOTTED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12v.01" /> <path d="M8 12v.01" /> <path d="M12 12v.01" /> <path d="M16 12v.01" /> <path d="M20 12v.01" /> </svg>"##;
const LIVE_PHOTO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M7 12a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M15.9 20.11l0 .01" /> <path d="M19.04 17.61l0 .01" /> <path d="M20.77 14l0 .01" /> <path d="M20.77 10l0 .01" /> <path d="M19.04 6.39l0 .01" /> <path d="M15.9 3.89l0 .01" /> <path d="M12 3l0 .01" /> <path d="M8.1 3.89l0 .01" /> <path d="M4.96 6.39l0 .01" /> <path d="M3.23 10l0 .01" /> <path d="M3.23 14l0 .01" /> <path d="M4.96 17.61l0 .01" /> <path d="M8.1 20.11l0 .01" /> <path d="M12 21l0 .01" /> </svg>"##;
const LIVE_PHOTO_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.296 11.29a1 1 0 1 0 1.414 1.415" /> <path d="M8.473 8.456a5 5 0 1 0 7.076 7.066m1.365 -2.591a5 5 0 0 0 -5.807 -5.851" /> <path d="M15.9 20.11v.01" /> <path d="M19.04 17.61v.01" /> <path d="M20.77 14v.01" /> <path d="M20.77 10v.01" /> <path d="M19.04 6.39v.01" /> <path d="M15.9 3.89v.01" /> <path d="M12 3v.01" /> <path d="M8.1 3.89v.01" /> <path d="M4.96 6.39v.01" /> <path d="M3.23 10v.01" /> <path d="M3.23 14v.01" /> <path d="M4.96 17.61v.01" /> <path d="M8.1 20.11v.01" /> <path d="M12 21v.01" /> <path d="M3 3l18 18" /> </svg>"##;
const MACRO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 15a6 6 0 1 0 12 0" /> <path d="M18 15a6 6 0 0 0 -6 6" /> <path d="M12 21a6 6 0 0 0 -6 -6" /> <path d="M12 21v-10" /> <path d="M12 11a5 5 0 0 1 -5 -5v-3l3 2l2 -2l2 2l3 -2v3a5 5 0 0 1 -5 5" /> </svg>"##;
const MACRO_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 15a6 6 0 0 0 11.47 2.467" /> <path d="M15.53 15.53a6 6 0 0 0 -3.53 5.47" /> <path d="M12 21a6 6 0 0 0 -6 -6" /> <path d="M12 21v-10" /> <path d="M10.866 10.87a5.007 5.007 0 0 1 -3.734 -3.723m-.132 -4.147l3 2l2 -2l2 2l3 -2v3a5 5 0 0 1 -2.604 4.389" /> <path d="M3 3l18 18" /> </svg>"##;
const MAGNET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 13v-8a2 2 0 0 1 2 -2h1a2 2 0 0 1 2 2v8a2 2 0 0 0 6 0v-8a2 2 0 0 1 2 -2h1a2 2 0 0 1 2 2v8a8 8 0 0 1 -16 0" /> <path d="M4 8l5 0" /> <path d="M15 8l4 0" /> </svg>"##;
const MAGNET_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3a2 2 0 0 1 2 2m0 4v4a3 3 0 0 0 5.552 1.578m.448 -3.578v-6a2 2 0 0 1 2 -2h1a2 2 0 0 1 2 2v8a7.99 7.99 0 0 1 -.424 2.577m-1.463 2.584a8 8 0 0 1 -14.113 -5.161v-8c0 -.297 .065 -.58 .181 -.833" /> <path d="M4 8h4" /> <path d="M15 8h4" /> <path d="M3 3l18 18" /> </svg>"##;
const MAGNETIC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3v18" /> <path d="M18 7c-.633 -1.255 -1.538 -2 -2.5 -2c-1.933 0 -3.5 3.134 -3.5 7s1.567 7 3.5 7s3.5 -3.134 3.5 -7v-1" /> <path d="M6 7c.633 -1.255 1.538 -2 2.5 -2c1.933 0 3.5 3.134 3.5 7s-1.567 7 -3.5 7s-3.5 -3.134 -3.5 -7v-1" /> <path d="M3 13l2 -2l2 2" /> <path d="M17 13l2 -2l2 2" /> </svg>"##;
const MASK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> </svg>"##;
const MASK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.42 19.41a2 2 0 0 1 -1.42 .59h-12a2 2 0 0 1 -2 -2v-12c0 -.554 .225 -1.055 .588 -1.417m3.412 -.583h10a2 2 0 0 1 2 2v10" /> <path d="M9.885 9.872a3 3 0 1 0 4.245 4.24m.582 -3.396a3.012 3.012 0 0 0 -1.438 -1.433" /> <path d="M3 3l18 18" /> </svg>"##;
const MESH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9h18" /> <path d="M3 15h18" /> <path d="M8 4c.485 .445 3.5 3.312 3.5 8c0 .663 -.07 4.848 -3.5 8" /> <path d="M15 4a17 17 0 0 1 2.004 8c0 1.51 -.201 4.628 -2.004 8" /> <path d="M18.778 20h-13.556a2.22 2.22 0 0 1 -2.222 -2.222v-11.556c0 -1.227 .995 -2.222 2.222 -2.222h13.556c1.227 0 2.222 .995 2.222 2.222v11.556a2.22 2.22 0 0 1 -2.222 2.222" /> </svg>"##;
const MICKEY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.5 3a3.5 3.5 0 0 1 3.25 4.8a7.017 7.017 0 0 0 -2.424 2.1a3.5 3.5 0 1 1 -.826 -6.9" /> <path d="M18.5 3a3.5 3.5 0 1 1 -.826 6.902a7.013 7.013 0 0 0 -2.424 -2.103a3.5 3.5 0 0 1 3.25 -4.799" /> <path d="M5 14a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> </svg>"##;
const MOOD_ANGRY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> <path d="M8 9l2 1" /> <path d="M16 9l-2 1" /> <path d="M14.5 16.05a3.5 3.5 0 0 0 -5 0" /> </svg>"##;
const MOOD_ANNOYED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> <path d="M15 14c-2 0 -3 1 -3.5 2.05" /> <path d="M9 10h-.01" /> <path d="M15 10h-.01" /> </svg>"##;
const MOOD_ANNOYED_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> <path d="M15 14c-2 0 -3 1 -3.5 2.05" /> <path d="M10 9.25c-.5 1 -2.5 1 -3 0" /> <path d="M17 9.25c-.5 1 -2.5 1 -3 0" /> </svg>"##;
const MOOD_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 21v-6m2 0v-1.5m0 9v-1.5m-2 -3h3m-1 0h.5a1.5 1.5 0 0 1 0 3h-3.5m3 -3h.5a1.5 1.5 0 0 0 0 -3h-3.5" /> <path d="M20.87 10.48a9 9 0 1 0 -7.876 10.465" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15c.658 .64 1.56 1 2.5 1c.357 0 .709 -.052 1.043 -.151" /> </svg>"##;
const MOOD_BOY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 4.5a9 9 0 0 1 3.864 5.89a2.5 2.5 0 0 1 -.29 4.36a9 9 0 0 1 -17.137 0a2.5 2.5 0 0 1 -.29 -4.36a9 9 0 0 1 3.746 -5.81" /> <path d="M9.5 16a3.5 3.5 0 0 0 5 0" /> <path d="M8.5 2c1.5 1 2.5 3.5 2.5 5" /> <path d="M12.5 2c1.5 2 2 3.5 2 5" /> <path d="M9 12l.01 0" /> <path d="M15 12l.01 0" /> </svg>"##;
const MOOD_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.925 13.163a8.998 8.998 0 0 0 -8.925 -10.163a9 9 0 0 0 0 18" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15c.658 .64 1.56 1 2.5 1s1.842 -.36 2.5 -1" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const MOOD_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -8.983 9" /> <path d="M16.001 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M18.001 14.5v1.5" /> <path d="M18.001 20v1.5" /> <path d="M21.032 16.25l-1.299 .75" /> <path d="M16.27 19l-1.3 .75" /> <path d="M14.97 16.25l1.3 .75" /> <path d="M19.733 19l1.3 .75" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15c.658 .64 1.56 1 2.5 1" /> </svg>"##;
const MOOD_CONFUZED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 10l.01 0" /> <path d="M15 10l.01 0" /> <path d="M9.5 16a10 10 0 0 1 6 -1.5" /> </svg>"##;
const MOOD_CRAZY_HAPPY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M7 8.5l3 3" /> <path d="M7 11.5l3 -3" /> <path d="M14 8.5l3 3" /> <path d="M14 11.5l3 -3" /> <path d="M9.5 15a3.5 3.5 0 0 0 5 0" /> </svg>"##;
const MOOD_CRY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 10l.01 0" /> <path d="M15 10l.01 0" /> <path d="M9.5 15.25a3.5 3.5 0 0 1 5 0" /> <path d="M17.566 17.606a2 2 0 1 0 2.897 .03l-1.463 -1.636l-1.434 1.606" /> <path d="M20.865 13.517a8.937 8.937 0 0 0 .135 -1.517a9 9 0 1 0 -9 9c.69 0 1.36 -.076 2 -.222" /> </svg>"##;
const MOOD_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.87 10.48a9 9 0 1 0 -7.876 10.465" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15c.658 .64 1.56 1 2.5 1c.357 0 .709 -.052 1.043 -.151" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const MOOD_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.955 11.104a9 9 0 1 0 -9.895 9.847" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15c.658 .672 1.56 1 2.5 1c.126 0 .251 -.006 .376 -.018" /> <path d="M18.42 15.61a2.1 2.1 0 0 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const MOOD_EMPTY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 10l.01 0" /> <path d="M15 10l.01 0" /> <path d="M9 15l6 0" /> </svg>"##;
const MOOD_HAPPY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 9l.01 0" /> <path d="M15 9l.01 0" /> <path d="M8 13a4 4 0 1 0 8 0h-8" /> </svg>"##;
const MOOD_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -8.012 8.946" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15a3.59 3.59 0 0 0 2.774 .99" /> <path d="M18.994 21.5l2.518 -2.58a1.74 1.74 0 0 0 .004 -2.413a1.627 1.627 0 0 0 -2.346 -.005l-.168 .172l-.168 -.172a1.627 1.627 0 0 0 -2.346 -.004a1.74 1.74 0 0 0 -.004 2.412l2.51 2.59" /> </svg>"##;
const MOOD_KID_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 10l.01 0" /> <path d="M15 10l.01 0" /> <path d="M9.5 15a3.5 3.5 0 0 0 5 0" /> <path d="M12 3a2 2 0 0 0 0 4" /> </svg>"##;
const MOOD_LOOK_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M9 13h.01" /> <path d="M15 13h.01" /> <path d="M11 17h2" /> </svg>"##;
const MOOD_LOOK_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 9h.01" /> <path d="M4 15h4" /> </svg>"##;
const MOOD_LOOK_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> <path d="M15 9h-.01" /> <path d="M20 15h-4" /> </svg>"##;
const MOOD_LOOK_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M9 8h.01" /> <path d="M15 8h.01" /> <path d="M11 12h2" /> </svg>"##;
const MOOD_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.48 15.014a9 9 0 1 0 -7.956 5.97" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M16 19h6" /> <path d="M9.5 15c.658 .64 1.56 1 2.5 1s1.842 -.36 2.5 -1" /> </svg>"##;
const MOOD_NERD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M6 10a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M14 10a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M9.5 15a3.5 3.5 0 0 0 5 0" /> <path d="M3.5 9h2.5" /> <path d="M18 9h2.5" /> <path d="M10 9.5c1.333 -1.333 2.667 -1.333 4 0" /> </svg>"##;
const MOOD_NERVOUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M8 16l2 -2l2 2l2 -2l2 2" /> </svg>"##;
const MOOD_NEUTRAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 10l.01 0" /> <path d="M15 10l.01 0" /> </svg>"##;
const MOOD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.634 5.638a9 9 0 0 0 12.732 12.724m1.679 -2.322a9 9 0 0 0 -12.08 -12.086" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15a3.5 3.5 0 0 0 5 0" /> <path d="M3 3l18 18" /> </svg>"##;
const MOOD_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -8.352 8.977" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15c.658 .672 1.56 1 2.5 1c.102 0 .203 -.004 .304 -.012" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const MOOD_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.985 12.528a9 9 0 1 0 -8.45 8.456" /> <path d="M16 19h6" /> <path d="M19 16v6" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15c.658 .64 1.56 1 2.5 1s1.842 -.36 2.5 -1" /> </svg>"##;
const MOOD_PUZZLED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.986 3.51a9 9 0 1 0 1.514 16.284c2.489 -1.437 4.181 -3.978 4.5 -6.794" /> <path d="M10 10h.01" /> <path d="M14 8h.01" /> <path d="M12 15c1 -1.333 2 -2 3 -2" /> <path d="M20 9v.01" /> <path d="M20 6a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const MOOD_SAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 10l.01 0" /> <path d="M15 10l.01 0" /> <path d="M9.5 15.25a3.5 3.5 0 0 1 5 0" /> </svg>"##;
const MOOD_SAD_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14.5 16.05a3.5 3.5 0 0 0 -5 0" /> <path d="M10 9.25c-.5 1 -2.5 1 -3 0" /> <path d="M17 9.25c-.5 1 -2.5 1 -3 0" /> </svg>"##;
const MOOD_SAD_DIZZY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14.5 16.05a3.5 3.5 0 0 0 -5 0" /> <path d="M8 9l2 2" /> <path d="M10 9l-2 2" /> <path d="M14 9l2 2" /> <path d="M16 9l-2 2" /> </svg>"##;
const MOOD_SAD_SQUINT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14.5 16.05a3.5 3.5 0 0 0 -5 0" /> <path d="M8.5 11.5l1.5 -1.5l-1.5 -1.5" /> <path d="M15.5 11.5l-1.5 -1.5l1.5 -1.5" /> </svg>"##;
const MOOD_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -9 9" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15c.658 .672 1.56 1 2.5 1" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const MOOD_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.942 13.018a9 9 0 1 0 -8.942 7.982" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15c.658 .672 1.56 1 2.5 1c.213 0 .424 -.017 .63 -.05" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const MOOD_SICK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> <path d="M9 10h-.01" /> <path d="M15 10h-.01" /> <path d="M8 16l1 -1l1.5 1l1.5 -1l1.5 1l1.5 -1l1 1" /> </svg>"##;
const MOOD_SILENCE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> <path d="M9 10h-.01" /> <path d="M15 10h-.01" /> <path d="M8 15h8" /> <path d="M9 14v2" /> <path d="M12 14v2" /> <path d="M15 14v2" /> </svg>"##;
const MOOD_SING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 9h.01" /> <path d="M15 9h.01" /> <path d="M13 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const MOOD_SMILE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 10l.01 0" /> <path d="M15 10l.01 0" /> <path d="M9.5 15a3.5 3.5 0 0 0 5 0" /> </svg>"##;
const MOOD_SMILE_BEAM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> <path d="M10 10c-.5 -1 -2.5 -1 -3 0" /> <path d="M17 10c-.5 -1 -2.5 -1 -3 0" /> <path d="M14.5 15a3.5 3.5 0 0 1 -5 0" /> </svg>"##;
const MOOD_SMILE_DIZZY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14.5 15a3.5 3.5 0 0 1 -5 0" /> <path d="M8 9l2 2" /> <path d="M10 9l-2 2" /> <path d="M14 9l2 2" /> <path d="M16 9l-2 2" /> </svg>"##;
const MOOD_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -8.994 9" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15a3.5 3.5 0 0 0 5 0" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const MOOD_SURPRISED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 9l.01 0" /> <path d="M15 9l.01 0" /> <path d="M10 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const MOOD_TONGUE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 10l.01 0" /> <path d="M15 10l.01 0" /> <path d="M10 14v2a2 2 0 0 0 4 0v-2m1.5 0h-7" /> </svg>"##;
const MOOD_TONGUE_WINK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 10h.01" /> <path d="M10 14v2a2 2 0 0 0 4 0v-2" /> <path d="M15.5 14h-7" /> <path d="M17 10c-.5 -1 -2.5 -1 -3 0" /> </svg>"##;
const MOOD_TONGUE_WINK_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> <path d="M15 10h-.01" /> <path d="M10 14v2a2 2 0 1 0 4 0v-2m1.5 0h-7" /> <path d="M7 10c.5 -1 2.5 -1 3 0" /> </svg>"##;
const MOOD_UNAMUSED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M11 16l4 -1.5" /> <path d="M10 10c-.5 -1 -2.5 -1 -3 0" /> <path d="M17 10c-.5 -1 -2.5 -1 -3 0" /> </svg>"##;
const MOOD_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.984 12.536a9 9 0 1 0 -8.463 8.449" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15c.658 .64 1.56 1 2.5 1s1.842 -.36 2.5 -1" /> </svg>"##;
const MOOD_WINK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M15 10h.01" /> <path d="M9.5 15a3.5 3.5 0 0 0 5 0" /> <path d="M8.5 8.5l1.5 1.5l-1.5 1.5" /> </svg>"##;
const MOOD_WINK_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> <path d="M9 10h-.01" /> <path d="M14.5 15a3.5 3.5 0 0 1 -5 0" /> <path d="M15.5 8.5l-1.5 1.5l1.5 1.5" /> </svg>"##;
const MOOD_WRRR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18" /> <path d="M8 16l1 -1l1.5 1l1.5 -1l1.5 1l1.5 -1l1 1" /> <path d="M8.5 11.5l1.5 -1.5l-1.5 -1.5" /> <path d="M15.5 11.5l-1.5 -1.5l1.5 -1.5" /> </svg>"##;
const MOOD_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.983 12.556a9 9 0 1 0 -8.433 8.427" /> <path d="M9 10h.01" /> <path d="M15 10h.01" /> <path d="M9.5 15c.658 .64 1.56 1 2.5 1c.194 0 .386 -.015 .574 -.045" /> <path d="M21.5 21.5l-5 -5" /> <path d="M16.5 21.5l5 -5" /> </svg>"##;
const MOOD_XD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M9 14h6a3 3 0 0 1 -6 0" /> <path d="M9 8l6 3" /> <path d="M9 11l6 -3" /> </svg>"##;
const MOUSTACHE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 9a3 3 0 0 1 2.599 1.5c.933 1.333 2.133 1.556 3.126 1.556l.291 0l.77 -.044l.213 0c-.963 1.926 -3.163 2.925 -6.6 3l-.4 0l-.165 0a3 3 0 0 1 .165 -6l.001 -.012" /> <path d="M9 9a3 3 0 0 0 -2.599 1.5c-.933 1.333 -2.133 1.556 -3.126 1.556l-.291 0l-.77 -.044l-.213 0c.963 1.926 3.163 2.925 6.6 3l.4 0l.165 0a3 3 0 0 0 -.165 -6l-.001 -.012" /> </svg>"##;
const NEEDLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21c-.667 -.667 3.262 -6.236 11.785 -16.709a3.5 3.5 0 1 1 5.078 4.791c-10.575 8.612 -16.196 12.585 -16.863 11.918" /> <path d="M17.5 6.5l-1 1" /> </svg>"##;
const NEEDLE_THREAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21c-.667 -.667 3.262 -6.236 11.785 -16.709a3.5 3.5 0 1 1 5.078 4.791c-10.575 8.612 -16.196 12.585 -16.863 11.918" /> <path d="M17.5 6.5l-1 1" /> <path d="M17 7c-2.333 -2.667 -3.5 -4 -5 -4s-2 1 -2 2c0 4 8.161 8.406 6 11c-1.056 1.268 -3.363 1.285 -5.75 .808" /> <path d="M5.739 15.425c-1.393 -.565 -3.739 -1.925 -3.739 -3.425" /> <path d="M19.5 9.5l1.5 1.5" /> </svg>"##;
const NOISE_REDUCTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 1 -18 0a9 9 0 0 1 18 0" /> <path d="M10.01 18h-.01" /> <path d="M14.01 14h-.01" /> <path d="M16.01 12h-.01" /> <path d="M18.01 10h-.01" /> <path d="M16.01 16h-.01" /> <path d="M14.01 18h-.01" /> <path d="M18.01 14h-.01" /> <path d="M12.01 16h-.01" /> </svg>"##;
const OCTAGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.802 2.165l5.575 2.389c.48 .206 .863 .589 1.07 1.07l2.388 5.574c.22 .512 .22 1.092 0 1.604l-2.389 5.575c-.206 .48 -.589 .863 -1.07 1.07l-5.574 2.388c-.512 .22 -1.092 .22 -1.604 0l-5.575 -2.389a2.036 2.036 0 0 1 -1.07 -1.07l-2.388 -5.574a2.036 2.036 0 0 1 0 -1.604l2.389 -5.575c.206 -.48 .589 -.863 1.07 -1.07l5.574 -2.388a2.036 2.036 0 0 1 1.604 0" /> </svg>"##;
const OCTAGON_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.802 2.165l5.575 2.389c.48 .206 .863 .589 1.07 1.07l2.388 5.574c.22 .512 .22 1.092 0 1.604l-2.389 5.575c-.206 .48 -.589 .863 -1.07 1.07l-5.574 2.388c-.512 .22 -1.092 .22 -1.604 0l-5.575 -2.389a2.036 2.036 0 0 1 -1.07 -1.07l-2.388 -5.574a2.036 2.036 0 0 1 0 -1.604l2.389 -5.575c.206 -.48 .589 -.863 1.07 -1.07l5.574 -2.388a2.036 2.036 0 0 1 1.604 0" /> <path d="M9 12h6" /> </svg>"##;
const OCTAGON_MINUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.039 21.734l-.237 .101c-.512 .22 -1.092 .22 -1.604 0l-5.575 -2.389a2.036 2.036 0 0 1 -1.07 -1.07l-2.388 -5.574a2.036 2.036 0 0 1 0 -1.604l2.389 -5.575c.206 -.48 .589 -.863 1.07 -1.07l5.574 -2.388a2.036 2.036 0 0 1 1.604 0l5.575 2.389c.48 .206 .863 .589 1.07 1.07l2.388 5.574c.22 .512 .22 1.092 0 1.604l-.94 2.196" /> <path d="M16 19h6" /> </svg>"##;
const OCTAGON_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.027 19.002a2.03 2.03 0 0 1 -.65 .444l-5.575 2.39a2.04 2.04 0 0 1 -1.604 0l-5.575 -2.39a2.036 2.036 0 0 1 -1.07 -1.07l-2.388 -5.574a2.036 2.036 0 0 1 0 -1.604l2.389 -5.575c.103 -.24 .25 -.457 .433 -.639m2.689 -1.31l3.522 -1.51a2.036 2.036 0 0 1 1.604 0l5.575 2.39c.48 .206 .863 .589 1.07 1.07l2.388 5.574c.22 .512 .22 1.092 0 1.604l-1.509 3.522" /> <path d="M3 3l18 18" /> </svg>"##;
const OCTAGON_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.802 2.165l5.575 2.389c.48 .206 .863 .589 1.07 1.07l2.388 5.574c.22 .512 .22 1.092 0 1.604l-2.389 5.575c-.206 .48 -.589 .863 -1.07 1.07l-5.574 2.388c-.512 .22 -1.092 .22 -1.604 0l-5.575 -2.389a2.036 2.036 0 0 1 -1.07 -1.07l-2.388 -5.574a2.036 2.036 0 0 1 0 -1.604l2.389 -5.575c.206 -.48 .589 -.863 1.07 -1.07l5.574 -2.388a2.036 2.036 0 0 1 1.604 0" /> <path d="M9 12h6" /> <path d="M12 9v6" /> </svg>"##;
const OCTAGON_PLUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.023 21.74l-.221 .095c-.512 .22 -1.092 .22 -1.604 0l-5.575 -2.389a2.036 2.036 0 0 1 -1.07 -1.07l-2.388 -5.574a2.036 2.036 0 0 1 0 -1.604l2.389 -5.575c.206 -.48 .589 -.863 1.07 -1.07l5.574 -2.388a2.036 2.036 0 0 1 1.604 0l5.575 2.389c.48 .206 .863 .589 1.07 1.07l2.388 5.574c.22 .512 .22 1.092 0 1.604l-.081 .19" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const OCTAHEDRON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.859 21.652l8.845 -8.949a.984 .984 0 0 0 0 -1.407l-8.845 -8.948a1.233 1.233 0 0 0 -1.718 0l-8.845 8.949a.984 .984 0 0 0 0 1.407l8.845 8.949a1.234 1.234 0 0 0 1.718 -.001" /> <path d="M2 12c.004 .086 .103 .178 .296 .246l8.845 2.632c.459 .163 1.259 .163 1.718 0l8.845 -2.632c.195 -.07 .294 -.156 .296 -.243" /> <path d="M12 2.12v19.76" /> </svg>"##;
const OCTAHEDRON_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.771 6.77l-4.475 4.527a.984 .984 0 0 0 0 1.407l8.845 8.949a1.234 1.234 0 0 0 1.718 -.001l4.36 -4.412m2.002 -2.025l2.483 -2.512a.984 .984 0 0 0 0 -1.407l-8.845 -8.948a1.233 1.233 0 0 0 -1.718 0l-2.375 2.403" /> <path d="M2 12c.004 .086 .103 .178 .296 .246l8.845 2.632c.459 .163 1.259 .163 1.718 0l1.544 -.46m3.094 -.92l4.207 -1.252c.195 -.07 .294 -.156 .296 -.243" /> <path d="M12 2.12v5.88m0 4v9.88" /> <path d="M3 3l18 18" /> </svg>"##;
const OCTAHEDRON_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21.498 12.911l.206 -.208a.984 .984 0 0 0 0 -1.407l-8.845 -8.948a1.233 1.233 0 0 0 -1.718 0l-8.845 8.949a.984 .984 0 0 0 0 1.407l8.845 8.949a1.234 1.234 0 0 0 1.718 -.001l.08 -.081" /> <path d="M2 12c.004 .086 .103 .178 .296 .246l8.845 2.632c.459 .163 1.259 .163 1.718 0l2.634 -.784m5.41 -1.61l.801 -.238c.195 -.07 .294 -.156 .296 -.243" /> <path d="M12 2.12v19.76" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const OVAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 12a6 9 0 1 0 12 0a6 9 0 1 0 -12 0" /> </svg>"##;
const OVAL_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12c0 -3.314 4.03 -6 9 -6s9 2.686 9 6s-4.03 6 -9 6s-9 -2.686 -9 -6" /> </svg>"##;
const PAINT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -2" /> <path d="M19 6h1a2 2 0 0 1 2 2a5 5 0 0 1 -5 5l-5 0v2" /> <path d="M10 16a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -4" /> </svg>"##;
const PAINT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h10a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-4m-4 0h-2a2 2 0 0 1 -2 -2v-2" /> <path d="M19 6h1a2 2 0 0 1 2 2a5 5 0 0 1 -5 5m-4 0h-1v2" /> <path d="M10 16a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -4" /> <path d="M3 3l18 18" /> </svg>"##;
const PALETTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 0 1 0 -18c4.97 0 9 3.582 9 8c0 1.06 -.474 2.078 -1.318 2.828c-.844 .75 -1.989 1.172 -3.182 1.172h-2.5a2 2 0 0 0 -1 3.75a1.3 1.3 0 0 1 -1 2.25" /> <path d="M7.5 10.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11.5 7.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M15.5 10.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const PALETTE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 15h-1a2 2 0 0 0 -1 3.75a1.3 1.3 0 0 1 -1 2.25a9 9 0 0 1 -6.372 -15.356" /> <path d="M8 4c1.236 -.623 2.569 -1 4 -1c4.97 0 9 3.582 9 8c0 1.06 -.474 2.078 -1.318 2.828a4.516 4.516 0 0 1 -1.127 .73" /> <path d="M7.5 10.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11.5 7.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M15.5 10.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3 3l18 18" /> </svg>"##;
const PANORAMA_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.338 5.53c5.106 1.932 10.211 1.932 15.317 0a1 1 0 0 1 1.345 .934v11c0 .692 -.692 1.2 -1.34 .962c-5.107 -1.932 -10.214 -1.932 -15.321 0c-.648 .246 -1.339 -.242 -1.339 -.935v-11.027a1 1 0 0 1 1.338 -.935l0 .001" /> </svg>"##;
const PANORAMA_HORIZONTAL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.95 6.952c2.901 .15 5.803 -.323 8.705 -1.42a1 1 0 0 1 1.345 .934v10.534m-3.212 .806c-4.483 -1.281 -8.966 -1.074 -13.449 .622a.993 .993 0 0 1 -1.339 -.935v-11.027a1 1 0 0 1 1.338 -.935c.588 .221 1.176 .418 1.764 .59" /> <path d="M3 3l18 18" /> </svg>"##;
const PANORAMA_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.463 4.338c-1.932 5.106 -1.932 10.211 0 15.317a1 1 0 0 1 -.934 1.345h-11c-.692 0 -1.208 -.692 -.962 -1.34c1.932 -5.107 1.932 -10.214 0 -15.321c-.246 -.648 .243 -1.339 .935 -1.339h11.028c.693 0 1.18 .691 .935 1.338l-.002 0" /> </svg>"##;
const PANORAMA_VERTICAL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h10.53c.693 0 1.18 .691 .935 1.338c-1.098 2.898 -1.573 5.795 -1.425 8.692m.828 4.847c.172 .592 .37 1.185 .595 1.778a1 1 0 0 1 -.934 1.345h-11c-.692 0 -1.208 -.692 -.962 -1.34c1.697 -4.486 1.903 -8.973 .619 -13.46" /> <path d="M3 3l18 18" /> </svg>"##;
const PENCIL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> </svg>"##;
const PENCIL_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const PENCIL_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const PENCIL_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const PENCIL_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const PENCIL_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const PENCIL_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M16 21l5 -5" /> <path d="M21 21v.01" /> <path d="M16 16v.01" /> </svg>"##;
const PENCIL_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const PENCIL_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const PENCIL_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const PENCIL_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 11l1.5 -1.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4h4l2 -2" /> <path d="M13.5 6.5l4 4" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const PENCIL_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M16 19h6" /> </svg>"##;
const PENCIL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 10l-6 6v4h4l6 -6m1.99 -1.99l2.504 -2.504a2.828 2.828 0 1 0 -4 -4l-2.5 2.5" /> <path d="M13.5 6.5l4 4" /> <path d="M3 3l18 18" /> </svg>"##;
const PENCIL_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const PENCIL_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const PENCIL_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const PENCIL_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 20l6 -6l3 -3l1.5 -1.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4h4" /> <path d="M13.5 6.5l4 4" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const PENCIL_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 11l1.5 -1.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4h4l3 -3" /> <path d="M13.5 6.5l4 4" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const PENCIL_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const PENCIL_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.5 10.5l1 -1a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4h4l2 -2" /> <path d="M13.5 6.5l4 4" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const PENCIL_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const PENCIL_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /> <path d="M13.5 6.5l4 4" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const PENTAGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> </svg>"##;
const PENTAGON_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21q -1.82 0 -5.458 .005a1.98 1.98 0 0 1 -1.881 -1.367l-3.064 -9.43a1.98 1.98 0 0 1 .719 -2.212l8.021 -5.828a1.98 1.98 0 0 1 2.326 0l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-1.559 4.792" /> <path d="M16 19h6" /> </svg>"##;
const PENTAGON_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.133 4.133l2.704 -1.965a1.978 1.978 0 0 1 2.326 0l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-1.887 5.808m-.981 3.02l-.196 .602a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l2.994 -2.176" /> <path d="M3 3l18 18" /> </svg>"##;
const PENTAGON_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21.005h-5.458a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-.78 2.401" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const PENTAGON_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> <path d="M14 14l-4 -4" /> <path d="M10 14l4 -4" /> </svg>"##;
const PENTAGRAM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.636 5.636a9 9 0 1 1 12.728 12.728a9 9 0 0 1 -12.728 -12.728" /> <path d="M15.236 11l5.264 4h-6.5l-2 6l-2 -6h-6.5l5.276 -4l-2.056 -6.28l5.28 3.78l5.28 -3.78l-2.044 6.28" /> </svg>"##;
const PERSPECTIVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.141 4.163l12 1.714a1 1 0 0 1 .859 .99v10.266a1 1 0 0 1 -.859 .99l-12 1.714a1 1 0 0 1 -1.141 -.99v-13.694a1 1 0 0 1 1.141 -.99" /> </svg>"##;
const PERSPECTIVE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.511 4.502l9.63 1.375a1 1 0 0 1 .859 .99v8.133m-.859 3.123l-12 1.714a1 1 0 0 1 -1.141 -.99v-13.694a1 1 0 0 1 .01 -.137" /> <path d="M3 3l18 18" /> </svg>"##;
const PHOTO_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 21v-6m2 0v-1.5m0 9v-1.5m-2 -3h3m-1 0h.5a1.5 1.5 0 0 1 0 3h-3.5m3 -3h.5a1.5 1.5 0 0 0 0 -3h-3.5" /> <path d="M15 8h.01" /> <path d="M13 21h-7a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5" /> <path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l2.5 2.5" /> </svg>"##;
const PHOTO_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M4 15l4 -4c.928 -.893 2.072 -.893 3 0l5 5" /> <path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0l2 2" /> <path d="M3 12a9 9 0 0 0 9 9a9 9 0 0 0 9 -9a9 9 0 0 0 -9 -9a9 9 0 0 0 -9 9" /> </svg>"##;
const PHOTO_HEXAGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M19.875 6.27c.7 .398 1.13 1.143 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033" /> <path d="M3.5 15.5l4.5 -4.5c.928 -.893 2.072 -.893 3 0l5 5" /> <path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0l2.5 2.5" /> </svg>"##;
const PHOTO_PENTAGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.163 2.168l8.021 5.828c.694 .504 .984 1.397 .719 2.212l-3.064 9.43a1.978 1.978 0 0 1 -1.881 1.367h-9.916a1.978 1.978 0 0 1 -1.881 -1.367l-3.064 -9.43a1.978 1.978 0 0 1 .719 -2.212l8.021 -5.828a1.978 1.978 0 0 1 2.326 0" /> <path d="M15 8h.01" /> <path d="M4 15l4 -4c.928 -.893 2.072 -.893 3 0l5 5" /> <path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0l2 2" /> </svg>"##;
const PHOTO_SENSOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 5h2a2 2 0 0 1 2 2v2" /> <path d="M21 15v2a2 2 0 0 1 -2 2h-2" /> <path d="M7 19h-2a2 2 0 0 1 -2 -2v-2" /> <path d="M3 9v-2a2 2 0 0 1 2 -2h2" /> <path d="M7 10a1 1 0 0 1 1 -1h8a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-8a1 1 0 0 1 -1 -1l0 -4" /> </svg>"##;
const PHOTO_SENSOR_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 5h2a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-2" /> <path d="M7 19h-2a2 2 0 0 1 -2 -2v-10a2 2 0 0 1 2 -2h2" /> <path d="M8 12a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> </svg>"##;
const PHOTO_SENSOR_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 4h1a2 2 0 0 1 2 2v1" /> <path d="M20 17v1a2 2 0 0 1 -2 2h-1" /> <path d="M7 20h-1a2 2 0 0 1 -2 -2v-1" /> <path d="M4 7v-1a2 2 0 0 1 2 -2h1" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M12 18v2" /> <path d="M4 12h2" /> <path d="M12 4v2" /> <path d="M20 12h-2" /> </svg>"##;
const PHOTO_SQUARE_ROUNDED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 8h.01" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> <path d="M3.5 15.5l4.5 -4.5c.928 -.893 2.072 -.893 3 0l5 5" /> <path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0l2.5 2.5" /> </svg>"##;
const PHOTO_VIDEO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15h-3a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v3" /> <path d="M9 12a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v6a3 3 0 0 1 -3 3h-6a3 3 0 0 1 -3 -3l0 -6" /> <path d="M3 12l2.296 -2.296a2.41 2.41 0 0 1 3.408 0l.296 .296" /> <path d="M14 13.5v3l2.5 -1.5l-2.5 -1.5" /> <path d="M7 6v.01" /> </svg>"##;
const PLACEHOLDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20.415a8 8 0 1 0 3 -15.415h-3" /> <path d="M13 8l-3 -3l3 -3" /> <path d="M7 17l4 -4l-4 -4l-4 4l4 4" /> </svg>"##;
const POLAROID_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M4 16l16 0" /> <path d="M4 12l3 -3c.928 -.893 2.072 -.893 3 0l4 4" /> <path d="M13 12l2 -2c.928 -.893 2.072 -.893 3 0l2 2" /> <path d="M14 7l.01 0" /> </svg>"##;
const POLYGON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 8a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 11a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M13 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6.5 9.5l3.5 -3" /> <path d="M14 5.5l3 1.5" /> <path d="M18.5 10l-2.5 7" /> <path d="M13.5 17.5l-7 -5" /> </svg>"##;
const POLYGON_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 8a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 11a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M13 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6.5 9.5l1.546 -1.311" /> <path d="M14 5.5l3 1.5" /> <path d="M18.5 10l-1.185 3.318m-1.062 2.972l-.253 .71" /> <path d="M13.5 17.5l-7 -5" /> <path d="M3 3l18 18" /> </svg>"##;
const PRISM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9v13" /> <path d="M19 17.17l-5.98 4.485a1.7 1.7 0 0 1 -2.04 0l-5.98 -4.485a2.5 2.5 0 0 1 -1 -2v-11.17a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1v11.17a2.5 2.5 0 0 1 -1 2" /> <path d="M4.3 3.3l6.655 5.186a1.7 1.7 0 0 0 2.09 0l6.655 -5.186" /> </svg>"##;
const PRISM_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12v10" /> <path d="M17.957 17.952l-4.937 3.703a1.7 1.7 0 0 1 -2.04 0l-5.98 -4.485a2.5 2.5 0 0 1 -1 -2v-11.17m3 -1h12a1 1 0 0 1 1 1v11.17c0 .25 -.037 .495 -.109 .729" /> <path d="M12.688 8.7a1.7 1.7 0 0 0 .357 -.214l6.655 -5.186" /> <path d="M3 3l18 18" /> </svg>"##;
const PRISM_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9v13" /> <path d="M13.02 21.655a1.7 1.7 0 0 1 -2.04 0l-5.98 -4.485a2.5 2.5 0 0 1 -1 -2v-11.17a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1v8" /> <path d="M4.3 3.3l6.655 5.186a1.7 1.7 0 0 0 2.09 0l6.655 -5.186" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const PRONG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.2 10.2l6.3 6.3" /> <path d="M19.347 16.575l1.08 1.079a1.96 1.96 0 0 1 -2.773 2.772l-1.08 -1.079a1.96 1.96 0 0 1 2.773 -2.772" /> <path d="M3 7l3.05 3.15a2.9 2.9 0 0 0 4.1 -4.1l-3.15 -3.05" /> </svg>"##;
const PYRAMID_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.105 21.788a1.994 1.994 0 0 0 1.789 0l8.092 -4.054c.538 -.27 .718 -.951 .385 -1.452l-8.54 -13.836a.999 .999 0 0 0 -1.664 0l-8.54 13.836a1.005 1.005 0 0 0 .386 1.452l8.092 4.054" /> <path d="M12 2v20" /> </svg>"##;
const PYRAMID_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21.384 17.373a1.004 1.004 0 0 0 -.013 -1.091l-8.54 -13.836a.999 .999 0 0 0 -1.664 0l-1.8 2.917m-1.531 2.48l-5.209 8.439a1.005 1.005 0 0 0 .386 1.452l8.092 4.054a1.994 1.994 0 0 0 1.789 0l5.903 -2.958" /> <path d="M12 2v6m0 4v10" /> <path d="M3 3l18 18" /> </svg>"##;
const PYRAMID_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.719 11.985l-5.889 -9.539a.999 .999 0 0 0 -1.664 0l-8.54 13.836a1.005 1.005 0 0 0 .386 1.452l8.092 4.054a1.994 1.994 0 0 0 1.789 0l.149 -.074" /> <path d="M12 2v20" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const RADIUS_BOTTOM_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 19h-6a8 8 0 0 1 -8 -8v-6" /> </svg>"##;
const RADIUS_BOTTOM_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 5v6a8 8 0 0 1 -8 8h-6" /> </svg>"##;
const RADIUS_TOP_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 19v-6a8 8 0 0 1 8 -8h6" /> </svg>"##;
const RADIUS_TOP_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5h6a8 8 0 0 1 8 8v6" /> </svg>"##;
const RECTANGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> </svg>"##;
const RECTANGLE_ROUNDED_BOTTOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 18h6a6 6 0 0 0 6 -6v-5a1 1 0 0 0 -1 -1h-16a1 1 0 0 0 -1 1v5a6 6 0 0 0 6 6" /> </svg>"##;
const RECTANGLE_ROUNDED_TOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 6h6a6 6 0 0 1 6 6v5a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1v-5a6 6 0 0 1 6 -6" /> </svg>"##;
const RECTANGLE_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -14" /> </svg>"##;
const RECTANGULAR_PRISM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 14.008v-5.018a1.98 1.98 0 0 0 -1 -1.717l-4 -2.008a2.016 2.016 0 0 0 -2 0l-10 5.008c-.619 .355 -1 1.01 -1 1.718v5.018c0 .709 .381 1.363 1 1.717l4 2.008a2.016 2.016 0 0 0 2 0l10 -5.008c.619 -.355 1 -1.01 1 -1.718" /> <path d="M9 21v-7.5" /> <path d="M9 13.5l11.5 -5.5" /> <path d="M3.5 11l5.5 2.5" /> </svg>"##;
const RECTANGULAR_PRISM_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.18 8.18l-4.18 2.093c-.619 .355 -1 1.01 -1 1.718v5.018c0 .709 .381 1.363 1 1.717l4 2.008a2.016 2.016 0 0 0 2 0l7.146 -3.578m2.67 -1.337l.184 -.093c.619 -.355 1 -1.01 1 -1.718v-5.018a1.98 1.98 0 0 0 -1 -1.717l-4 -2.008a2.016 2.016 0 0 0 -2 0l-3.146 1.575" /> <path d="M9 21v-7.5" /> <path d="M9 13.5l3.048 -1.458m2.71 -1.296l5.742 -2.746" /> <path d="M3.5 11l5.5 2.5" /> <path d="M3 3l18 18" /> </svg>"##;
const RECTANGULAR_PRISM_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12.5v-3.509a1.98 1.98 0 0 0 -1 -1.717l-4 -2.008a2.016 2.016 0 0 0 -2 0l-10 5.007c-.619 .355 -1 1.01 -1 1.718v5.018c0 .709 .381 1.363 1 1.717l4 2.008a2.016 2.016 0 0 0 2 0l2.062 -1.032" /> <path d="M9 21v-7.5" /> <path d="M9 13.5l11.5 -5.5" /> <path d="M3.5 11l5.5 2.5" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const RESIZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 11v8a1 1 0 0 0 1 1h8m-9 -14v-1a1 1 0 0 1 1 -1h1m5 0h2m5 0h1a1 1 0 0 1 1 1v1m0 5v2m0 5v1a1 1 0 0 1 -1 1h-1" /> <path d="M4 12h7a1 1 0 0 1 1 1v7" /> </svg>"##;
const ROSETTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1" /> </svg>"##;
const ROSETTE_ASTERISK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55l0 -1" /> <path d="M12 8.5v7" /> <path d="M9 10l6 4" /> <path d="M9 14l6 -4" /> </svg>"##;
const RULER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h14a1 1 0 0 1 1 1v5a1 1 0 0 1 -1 1h-7a1 1 0 0 0 -1 1v7a1 1 0 0 1 -1 1h-5a1 1 0 0 1 -1 -1v-14a1 1 0 0 1 1 -1" /> <path d="M4 8l2 0" /> <path d="M4 12l3 0" /> <path d="M4 16l2 0" /> <path d="M8 4l0 2" /> <path d="M12 4l0 3" /> <path d="M16 4l0 2" /> </svg>"##;
const RULER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 3l4 4l-14 14l-4 -4l14 -14" /> <path d="M16 7l-1.5 -1.5" /> <path d="M13 10l-1.5 -1.5" /> <path d="M10 13l-1.5 -1.5" /> <path d="M7 16l-1.5 -1.5" /> </svg>"##;
const RULER_2_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.03 7.97l4.97 -4.97l4 4l-5 5m-2 2l-7 7l-4 -4l7 -7" /> <path d="M16 7l-1.5 -1.5" /> <path d="M10 13l-1.5 -1.5" /> <path d="M7 16l-1.5 -1.5" /> <path d="M3 3l18 18" /> </svg>"##;
const RULER_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 8c.621 0 1.125 .512 1.125 1.143v5.714c0 .631 -.504 1.143 -1.125 1.143h-15.875a1 1 0 0 1 -1 -1v-5.857c0 -.631 .504 -1.143 1.125 -1.143h15.75" /> <path d="M9 8v2" /> <path d="M6 8v3" /> <path d="M12 8v3" /> <path d="M18 8v3" /> <path d="M15 8v2" /> </svg>"##;
const RULER_MEASURE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.875 12c.621 0 1.125 .512 1.125 1.143v5.714c0 .631 -.504 1.143 -1.125 1.143h-15.875a1 1 0 0 1 -1 -1v-5.857c0 -.631 .504 -1.143 1.125 -1.143h15.75" /> <path d="M9 12v2" /> <path d="M6 12v3" /> <path d="M12 12v3" /> <path d="M18 12v3" /> <path d="M15 12v2" /> <path d="M3 3v4" /> <path d="M3 5h18" /> <path d="M21 3v4" /> </svg>"##;
const RULER_MEASURE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19.875c0 .621 -.512 1.125 -1.143 1.125h-5.714a1.134 1.134 0 0 1 -1.143 -1.125v-15.875a1 1 0 0 1 1 -1h5.857c.631 0 1.143 .504 1.143 1.125l0 15.75" /> <path d="M12 9h-2" /> <path d="M12 6h-3" /> <path d="M12 12h-3" /> <path d="M12 18h-3" /> <path d="M12 15h-2" /> <path d="M21 3h-4" /> <path d="M19 3v18" /> <path d="M21 21h-4" /> </svg>"##;
const RULER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h11a1 1 0 0 1 1 1v5a1 1 0 0 1 -1 1h-4m-3.713 .299a1 1 0 0 0 -.287 .701v7a1 1 0 0 1 -1 1h-5a1 1 0 0 1 -1 -1v-14c0 -.284 .118 -.54 .308 -.722" /> <path d="M4 8h2" /> <path d="M4 12h3" /> <path d="M4 16h2" /> <path d="M12 4v3" /> <path d="M16 4v2" /> <path d="M3 3l18 18" /> </svg>"##;
const SCISSORS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M3 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M8.6 8.6l10.4 10.4" /> <path d="M8.6 15.4l10.4 -10.4" /> </svg>"##;
const SCISSORS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.432 4.442a3 3 0 1 0 4.114 4.146" /> <path d="M3 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M8.6 15.4l3.4 -3.4m2 -2l5 -5" /> <path d="M3 3l18 18" /> </svg>"##;
const SCREENSHOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 19a2 2 0 0 1 -2 -2" /> <path d="M5 13v-2" /> <path d="M5 7a2 2 0 0 1 2 -2" /> <path d="M11 5h2" /> <path d="M17 5a2 2 0 0 1 2 2" /> <path d="M19 11v2" /> <path d="M19 17v4" /> <path d="M21 19h-4" /> <path d="M13 19h-2" /> </svg>"##;
const SECTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 20h.01" /> <path d="M4 20h.01" /> <path d="M8 20h.01" /> <path d="M12 20h.01" /> <path d="M16 20h.01" /> <path d="M20 4h.01" /> <path d="M4 4h.01" /> <path d="M8 4h.01" /> <path d="M12 4h.01" /> <path d="M16 4l0 .01" /> <path d="M4 9a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1l0 -6" /> </svg>"##;
const SHADOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M13 12h5" /> <path d="M13 15h4" /> <path d="M13 18h1" /> <path d="M13 9h4" /> <path d="M13 6h1" /> </svg>"##;
const SHADOW_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.634 5.638a9 9 0 0 0 12.728 12.727m1.68 -2.32a9 9 0 0 0 -12.086 -12.088" /> <path d="M16 12h2" /> <path d="M13 15h2" /> <path d="M13 18h1" /> <path d="M13 9h4" /> <path d="M13 6h1" /> <path d="M3 3l18 18" /> </svg>"##;
const SHAPE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 7l0 10" /> <path d="M7 5l10 0" /> <path d="M7 19l10 0" /> <path d="M19 7l0 10" /> </svg>"##;
const SHAPE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6.5 17.5l11 -11m-12.5 .5v10m14 -10v10" /> </svg>"##;
const SHAPE_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 5h10m-12 2v10m14 -10v10" /> </svg>"##;
const SHAPE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.575 3.597a2 2 0 0 0 2.849 2.808" /> <path d="M17 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17.574 17.598a2 2 0 0 0 2.826 2.83" /> <path d="M5 7v10" /> <path d="M9 5h8" /> <path d="M7 19h10" /> <path d="M19 7v8" /> <path d="M3 3l18 18" /> </svg>"##;
const SHOVEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 4l3 3" /> <path d="M18.5 5.5l-8 8" /> <path d="M8.276 11.284l4.44 4.44a.968 .968 0 0 1 0 1.369l-2.704 2.704a4.108 4.108 0 0 1 -5.809 -5.81l2.704 -2.703a.968 .968 0 0 1 1.37 0l-.001 0" /> </svg>"##;
const SHOVEL_PITCHFORKS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 3h4" /> <path d="M7 3v12" /> <path d="M4 15h6v3a3 3 0 0 1 -6 0v-3" /> <path d="M14 21v-3a3 3 0 0 1 6 0v3" /> <path d="M17 21v-18" /> </svg>"##;
const SKETCHING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 15c2 -4.97 7.356 -11 9 -11c4.25 0 -5.5 11.958 -3 13s5.65 -6.678 7.4 -5.902c1.75 .777 -1.05 7.589 -.3 8.63s3.15 -.897 3.9 -2.728" /> </svg>"##;
const SLICE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19l15 -15l3 3l-6 6l2 2a14 14 0 0 1 -14 4" /> </svg>"##;
const SPADE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3l4.919 4.5c.61 .587 1.177 1.177 1.703 1.771a5.527 5.527 0 0 1 .264 6.979c-1.18 1.56 -3.338 1.92 -4.886 .75v1l1 3h-6l1 -3v-1c-1.54 1.07 -3.735 .772 -4.886 -.75a5.527 5.527 0 0 1 .264 -6.979a30.883 30.883 0 0 1 1.703 -1.771a1541.72 1541.72 0 0 1 4.919 -4.5" /> </svg>"##;
const SPARKLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12c-6.597 0 -9 2.403 -9 9c0 -6.597 -2.403 -9 -9 -9c6.597 0 9 -2.403 9 -9c0 6.597 2.403 9 9 9" /> </svg>"##;
const SPARKLE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3c.375 0 .711 .231 .846 .581l1.65 4.29a2.85 2.85 0 0 0 1.632 1.633l4.291 1.65a.906 .906 0 0 1 0 1.692l-4.29 1.65a2.84 2.84 0 0 0 -1.633 1.632l-1.65 4.291a.906 .906 0 0 1 -1.692 0l-1.65 -4.29a2.84 2.84 0 0 0 -1.632 -1.633l-4.291 -1.65a.906 .906 0 0 1 0 -1.692l4.29 -1.65a2.84 2.84 0 0 0 1.633 -1.632l1.65 -4.291a.91 .91 0 0 1 .846 -.581" /> </svg>"##;
const SPARKLE_HIGHLIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.504 8.522l-1.758 -4.032a.814 .814 0 0 0 -1.492 0l-1.759 4.032c-.19 .436 -.537 .784 -.973 .973l-4.032 1.759a.814 .814 0 0 0 0 1.492l4.033 1.758c.436 .19 .784 .538 .973 .974l1.759 4.033a.814 .814 0 0 0 1.492 0l1.758 -4.033c.19 -.436 .538 -.784 .974 -.974l4.033 -1.758a.814 .814 0 0 0 0 -1.492l-4.033 -1.759a1.88 1.88 0 0 1 -.974 -.973" /> <path d="M3 3l2 2" /> <path d="M21 3l-2 2" /> <path d="M3 21l2 -2" /> <path d="M21 21l-2 -2" /> </svg>"##;
const SPARKLES_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 7a9.3 9.3 0 0 0 1.516 -.546c.911 -.438 1.494 -1.015 1.937 -1.932c.207 -.428 .382 -.928 .547 -1.522c.165 .595 .34 1.095 .547 1.521c.443 .918 1.026 1.495 1.937 1.933c.426 .205 .925 .38 1.516 .546a9.3 9.3 0 0 0 -1.516 .547c-.911 .438 -1.494 1.015 -1.937 1.932a9 9 0 0 0 -.547 1.521c-.165 -.594 -.34 -1.095 -.547 -1.521c-.443 -.918 -1.026 -1.494 -1.937 -1.932a9 9 0 0 0 -1.516 -.547" /> <path d="M3 14a21 21 0 0 0 1.652 -.532c2.542 -.953 3.853 -2.238 4.816 -4.806a20 20 0 0 0 .532 -1.662a20 20 0 0 0 .532 1.662c.963 2.567 2.275 3.853 4.816 4.806q .75 .28 1.652 .532a21 21 0 0 0 -1.652 .532c-2.542 .953 -3.854 2.238 -4.816 4.806a20 20 0 0 0 -.532 1.662a20 20 0 0 0 -.532 -1.662c-.963 -2.568 -2.275 -3.853 -4.816 -4.806a21 21 0 0 0 -1.652 -.532" /> </svg>"##;
const SPHERE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12c0 1.657 4.03 3 9 3s9 -1.343 9 -3" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const SPHERE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12c0 1.657 4.03 3 9 3s9 -1.343 9 -3" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M12 3c-1.657 0 -3 4.03 -3 9s1.343 9 3 9" /> </svg>"##;
const SPHERE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12c0 1.657 4.03 3 9 3c.987 0 1.936 -.053 2.825 -.15m3.357 -.67c1.735 -.547 2.818 -1.32 2.818 -2.18" /> <path d="M20.051 16.027a9 9 0 0 0 -12.083 -12.075m-2.34 1.692a9 9 0 0 0 12.74 12.716" /> <path d="M3 3l18 18" /> </svg>"##;
const SPHERE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12c0 1.657 4.03 3 9 3c1.116 0 2.185 -.068 3.172 -.192m5.724 -2.35a1.1 1.1 0 0 0 .104 -.458" /> <path d="M20.984 12.546a9 9 0 1 0 -8.442 8.438" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_ASTERISK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M12 8.5v7" /> <path d="M9 10l6 4" /> <path d="M9 14l6 -4" /> </svg>"##;
const SQUARE_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 12l2 2l4 -4" /> </svg>"##;
const SQUARE_DASHED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -14" /> </svg>"##;
const SQUARE_DOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const SQUARE_FORBID_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 9l6 6" /> </svg>"##;
const SQUARE_FORBID_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 15l6 -6" /> </svg>"##;
const SQUARE_HALF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4v16" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M12 13l7.5 -7.5" /> <path d="M12 18l8 -8" /> <path d="M15 20l5 -5" /> <path d="M12 8l4 -4" /> </svg>"##;
const SQUARE_KEY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 10a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M12.5 11.5l-4 4l1.5 1.5" /> <path d="M12 15l-1.5 -1.5" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12h6" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_MINUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-7.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10" /> <path d="M16 19h6" /> </svg>"##;
const SQUARE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h10a2 2 0 0 1 2 2v10m-.584 3.412a2 2 0 0 1 -1.416 .588h-12a2 2 0 0 1 -2 -2v-12c0 -.552 .224 -1.052 .586 -1.414" /> <path d="M3 3l18 18" /> </svg>"##;
const SQUARE_PERCENTAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 15.037l6 -6" /> <path d="M9 9.068v.014" /> <path d="M15 15.082v.016" /> </svg>"##;
const SQUARE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12h6" /> <path d="M12 9v6" /> <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> </svg>"##;
const SQUARE_PLUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-7.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v7.5" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const SQUARE_ROTATED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.446 2.6l7.955 7.954a2.045 2.045 0 0 1 0 2.892l-7.955 7.955a2.045 2.045 0 0 1 -2.892 0l-7.955 -7.955a2.045 2.045 0 0 1 0 -2.892l7.955 -7.955a2.045 2.045 0 0 1 2.892 0" /> </svg>"##;
const SQUARE_ROTATED_ASTERISK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.446 2.6l7.955 7.954a2.045 2.045 0 0 1 0 2.892l-7.955 7.955a2.045 2.045 0 0 1 -2.892 0l-7.955 -7.955a2.045 2.045 0 0 1 0 -2.892l7.955 -7.955a2.045 2.045 0 0 1 2.892 0" /> <path d="M12 8.5v7" /> <path d="M9 10l6 4" /> <path d="M9 14l6 -4" /> </svg>"##;
const SQUARE_ROTATED_FORBID_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.446 2.6l7.955 7.954a2.045 2.045 0 0 1 0 2.892l-7.955 7.955a2.045 2.045 0 0 1 -2.892 0l-7.955 -7.955a2.045 2.045 0 0 1 0 -2.892l7.955 -7.955a2.045 2.045 0 0 1 2.892 0" /> <path d="M9.5 14.5l5 -5" /> </svg>"##;
const SQUARE_ROTATED_FORBID_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.446 2.6l7.955 7.954a2.045 2.045 0 0 1 0 2.892l-7.955 7.955a2.045 2.045 0 0 1 -2.892 0l-7.955 -7.955a2.045 2.045 0 0 1 0 -2.892l7.955 -7.955a2.045 2.045 0 0 1 2.892 0" /> <path d="M9.5 9.5l5 5" /> </svg>"##;
const SQUARE_ROTATED_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.964 16.952l-3.462 3.461c-.782 .783 -2.222 .783 -3 0l-6.911 -6.91c-.783 -.783 -.783 -2.223 0 -3l3.455 -3.456m2 -2l1.453 -1.452c.782 -.783 2.222 -.783 3 0l6.911 6.91c.783 .783 .783 2.223 0 3l-1.448 1.45" /> <path d="M3 3l18 18" /> </svg>"##;
const SQUARE_ROUNDED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12l2 2l4 -4" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12h6" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_ROUNDED_MINUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21c-.18 .002 -.314 0 -.5 0c-7.2 0 -9 -1.8 -9 -9s1.8 -9 9 -9s9 1.8 9 9c0 1.136 -.046 2.138 -.152 3.02" /> <path d="M16 19h6" /> </svg>"##;
const SQUARE_ROUNDED_PERCENTAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> <path d="M9 15.075l6 -6" /> <path d="M9 9.105v.015" /> <path d="M15 15.12v.015" /> </svg>"##;
const SQUARE_ROUNDED_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> <path d="M15 12h-6" /> <path d="M12 9v6" /> </svg>"##;
const SQUARE_ROUNDED_PLUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.54 20.996c-.176 .004 -.356 .004 -.54 .004c-7.2 0 -9 -1.8 -9 -9s1.8 -9 9 -9s9 1.8 9 9c0 .185 -.001 .366 -.004 .544" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const SQUARE_ROUNDED_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 10l4 4m0 -4l-4 4" /> <path d="M12 3c7.2 0 9 1.8 9 9c0 7.2 -1.8 9 -9 9c-7.2 0 -9 -1.8 -9 -9c0 -7.2 1.8 -9 9 -9" /> </svg>"##;
const SQUARE_TOGGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 2l0 20" /> <path d="M14 20h-8a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h8" /> <path d="M20 6a2 2 0 0 0 -2 -2" /> <path d="M18 20a2 2 0 0 0 2 -2" /> <path d="M20 10l0 4" /> </svg>"##;
const SQUARE_TOGGLE_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12h-20" /> <path d="M4 14v-8a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v8" /> <path d="M18 20a2 2 0 0 0 2 -2" /> <path d="M4 18a2 2 0 0 0 2 2" /> <path d="M14 20l-4 0" /> </svg>"##;
const SQUARE_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 9l6 6m0 -6l-6 6" /> </svg>"##;
const SQUARES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 10a2 2 0 0 1 2 -2h9a2 2 0 0 1 2 2v9a2 2 0 0 1 -2 2h-9a2 2 0 0 1 -2 -2l0 -9" /> <path d="M16 8v-3a2 2 0 0 0 -2 -2h-9a2 2 0 0 0 -2 2v9a2 2 0 0 0 2 2h3" /> </svg>"##;
const SQUARES_DIAGONAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 10a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /> <path d="M16 8v-2a2 2 0 0 0 -2 -2h-8a2 2 0 0 0 -2 2v8a2 2 0 0 0 2 2h2" /> <path d="M8.586 19.414l10.827 -10.827" /> </svg>"##;
const SQUARES_SELECTED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 10a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /> <path d="M8 14.5l6.492 -6.492" /> <path d="M13.496 20l6.504 -6.504l-6.504 6.504" /> <path d="M8.586 19.414l10.827 -10.827" /> <path d="M16 8v-2a2 2 0 0 0 -2 -2h-8a2 2 0 0 0 -2 2v8a2 2 0 0 0 2 2h2" /> </svg>"##;
const STACK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 6l-8 4l8 4l8 -4l-8 -4" /> <path d="M4 14l8 4l8 -4" /> </svg>"##;
const STACK_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 4l-8 4l8 4l8 -4l-8 -4" /> <path d="M4 12l8 4l8 -4" /> <path d="M4 16l8 4l8 -4" /> </svg>"##;
const STACK_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 2l-8 4l8 4l8 -4l-8 -4" /> <path d="M4 10l8 4l8 -4" /> <path d="M4 18l8 4l8 -4" /> <path d="M4 14l8 4l8 -4" /> </svg>"##;
const STACK_POP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9.5l-3 1.5l8 4l8 -4l-3 -1.5" /> <path d="M4 15l8 4l8 -4" /> <path d="M12 11v-7" /> <path d="M9 7l3 -3l3 3" /> </svg>"##;
const STACK_PUSH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 10l-2 1l8 4l8 -4l-2 -1" /> <path d="M4 15l8 4l8 -4" /> <path d="M12 4v7" /> <path d="M15 8l-3 3l-3 -3" /> </svg>"##;
const STICKER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 12l-2 .5a6 6 0 0 1 -6.5 -6.5l.5 -2l8 8" /> <path d="M20 12a8 8 0 1 1 -8 -8" /> </svg>"##;
const STICKER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4h12a2 2 0 0 1 2 2v7h-5a2 2 0 0 0 -2 2v5h-7a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2" /> <path d="M20 13v.172a2 2 0 0 1 -.586 1.414l-4.828 4.828a2 2 0 0 1 -1.414 .586h-.172" /> </svg>"##;
const STROKE_CURVED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19h1.341a7 7 0 0 0 6.845 -5.533l.628 -2.934a7 7 0 0 1 6.846 -5.533h1.34" /> </svg>"##;
const STROKE_DYNAMIC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19h6a2 2 0 0 0 2 -2v-10a2 2 0 0 1 2 -2h7" /> </svg>"##;
const STROKE_STRAIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19l16 -14" /> </svg>"##;
const TEMPLATE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1l0 -2" /> <path d="M4 13a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -6" /> <path d="M14 12l6 0" /> <path d="M14 16l6 0" /> <path d="M14 20l6 0" /> </svg>"##;
const TEMPLATE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h11a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-7m-4 0h-3a1 1 0 0 1 -1 -1v-2c0 -.271 .108 -.517 .283 -.697" /> <path d="M4 13a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -6" /> <path d="M16 12h4" /> <path d="M14 16h2" /> <path d="M14 20h6" /> <path d="M3 3l18 18" /> </svg>"##;
const TEXT_RESIZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 7v10" /> <path d="M7 5h10" /> <path d="M7 19h10" /> <path d="M19 7v10" /> <path d="M10 10h4" /> <path d="M12 14v-4" /> </svg>"##;
const TILT_SHIFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -2.92 1.95" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M10 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const TILT_SHIFT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.56 3.69a9 9 0 0 0 -.577 .263" /> <path d="M3.69 8.56a9 9 0 0 0 -.69 3.44" /> <path d="M3.69 15.44a9 9 0 0 0 1.95 2.92" /> <path d="M8.56 20.31a9 9 0 0 0 3.44 .69" /> <path d="M15.44 20.31a9 9 0 0 0 2.92 -1.95" /> <path d="M20.31 15.44a9 9 0 0 0 .69 -3.44" /> <path d="M20.31 8.56a9 9 0 0 0 -1.95 -2.92" /> <path d="M15.44 3.69a9 9 0 0 0 -3.44 -.69" /> <path d="M10.57 10.602a2 2 0 0 0 2.862 2.795" /> <path d="M3 3l18 18" /> </svg>"##;
const TOOLS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21h4l13 -13a1.5 1.5 0 0 0 -4 -4l-13 13v4" /> <path d="M14.5 5.5l4 4" /> <path d="M12 8l-5 -5l-4 4l5 5" /> <path d="M7 8l-1.5 1.5" /> <path d="M16 12l5 5l-4 4l-5 -5" /> <path d="M16 17l-1.5 1.5" /> </svg>"##;
const TOOLS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 12l4 -4a2.828 2.828 0 1 0 -4 -4l-4 4m-2 2l-7 7v4h4l7 -7" /> <path d="M14.5 5.5l4 4" /> <path d="M12 8l-5 -5m-2 2l-2 2l5 5" /> <path d="M7 8l-1.5 1.5" /> <path d="M16 12l5 5m-2 2l-2 2l-5 -5" /> <path d="M16 17l-1.5 1.5" /> <path d="M3 3l18 18" /> </svg>"##;
const TRIANGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.363 3.591l-8.106 13.534a1.914 1.914 0 0 0 1.636 2.871h16.214a1.914 1.914 0 0 0 1.636 -2.87l-8.106 -13.536a1.914 1.914 0 0 0 -3.274 0" /> </svg>"##;
const TRIANGLE_INVERTED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.363 20.405l-8.106 -13.534a1.914 1.914 0 0 1 1.636 -2.871h16.214a1.914 1.914 0 0 1 1.636 2.871l-8.106 13.534a1.914 1.914 0 0 1 -3.274 0" /> </svg>"##;
const TRIANGLE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.363 3.591l-8.106 13.534a1.914 1.914 0 0 0 1.636 2.871h16.214a1.914 1.914 0 0 0 1.636 -2.87l-8.106 -13.536a1.914 1.914 0 0 0 -3.274 0v.001" /> <path d="M9 13h6" /> </svg>"##;
const TRIANGLE_MINUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.48 15.016l-6.843 -11.426a1.914 1.914 0 0 0 -3.274 0l-8.106 13.535a1.914 1.914 0 0 0 1.636 2.871h8.107" /> <path d="M16 19h6" /> </svg>"##;
const TRIANGLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.825 7.83l-5.568 9.295a1.914 1.914 0 0 0 1.636 2.871h16.107m1.998 -1.99a1.913 1.913 0 0 0 -.255 -.88l-8.106 -13.536a1.914 1.914 0 0 0 -3.274 0l-1.028 1.718" /> <path d="M3 3l18 18" /> </svg>"##;
const TRIANGLE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.363 3.591l-8.106 13.534a1.914 1.914 0 0 0 1.636 2.871h16.214a1.914 1.914 0 0 0 1.636 -2.87l-8.106 -13.536a1.914 1.914 0 0 0 -3.274 0v.001" /> <path d="M9 13h6" /> <path d="M12 10v6" /> </svg>"##;
const TRIANGLE_PLUS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.69 12.027l-5.054 -8.437a1.914 1.914 0 0 0 -3.274 0l-8.105 13.535a1.914 1.914 0 0 0 1.636 2.871h8.107" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const TRIANGLE_SQUARE_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3l-4 7h8l-4 -7" /> <path d="M14 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M4 15a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> </svg>"##;
const TRIANGLES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.974 21h8.052a.975 .975 0 0 0 .81 -1.517l-4.025 -6.048a.973 .973 0 0 0 -1.622 0l-4.025 6.048a.977 .977 0 0 0 .81 1.517" /> <path d="M4.98 16h14.04c.542 0 .98 -.443 .98 -.989a1 1 0 0 0 -.156 -.534l-7.02 -11.023a.974 .974 0 0 0 -1.648 0l-7.02 11.023a1 1 0 0 0 .294 1.366a.973 .973 0 0 0 .53 .157" /> </svg>"##;
const TRIDENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6l2 -2v3a7 7 0 0 0 14 0v-3l2 2" /> <path d="M12 21v-18l-2 2m4 0l-2 -2" /> </svg>"##;
const TYPEFACE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -14" /> <path d="M17 17a2 2 0 0 1 -2 -2v-8h-5a2 2 0 0 0 -2 2" /> <path d="M7 17a2.775 2.775 0 0 0 2.632 -1.897l.368 -1.103a13.4 13.4 0 0 1 3.236 -5.236l1.764 -1.764" /> <path d="M10 14h5" /> </svg>"##;
const UX_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M7 10v2a2 2 0 1 0 4 0v-2" /> <path d="M14 10l3 4" /> <path d="M14 14l3 -4" /> </svg>"##;
const VECTOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M3 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M5 7l0 10" /> <path d="M19 7l0 10" /> <path d="M7 5l10 0" /> <path d="M7 19l10 0" /> </svg>"##;
const VECTOR_BEZIER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 15a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 15a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M10 7a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M10 8.5a6 6 0 0 0 -5 5.5" /> <path d="M14 8.5a6 6 0 0 1 5 5.5" /> <path d="M10 8l-6 0" /> <path d="M20 8l-6 0" /> <path d="M2 8a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M20 8a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const VECTOR_BEZIER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M7 5l7 0" /> <path d="M10 19l7 0" /> <path d="M8 19a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M14 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M7 5.5a5 6.5 0 0 1 5 6.5a5 6.5 0 0 0 5 6.5" /> </svg>"##;
const VECTOR_BEZIER_ARC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 11a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 11a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M10 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M10 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M19 10a5 5 0 0 0 -5 -5" /> <path d="M5 14a5 5 0 0 0 5 5" /> <path d="M5 10a5 5 0 0 1 5 -5" /> </svg>"##;
const VECTOR_BEZIER_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 11a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 11a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M10 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M10 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M19 10a5 5 0 0 0 -5 -5" /> <path d="M19 14a5 5 0 0 1 -5 5" /> <path d="M5 14a5 5 0 0 0 5 5" /> <path d="M5 10a5 5 0 0 1 5 -5" /> </svg>"##;
const VECTOR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.68 6.733a1 1 0 0 1 -.68 .267h-2a1 1 0 0 1 -1 -1v-2c0 -.276 .112 -.527 .293 -.708" /> <path d="M17 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M20.72 20.693a1 1 0 0 1 -.72 .307h-2a1 1 0 0 1 -1 -1v-2c0 -.282 .116 -.536 .304 -.718" /> <path d="M3 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M5 7v10" /> <path d="M19 7v8" /> <path d="M9 5h8" /> <path d="M7 19h10" /> <path d="M3 3l18 18" /> </svg>"##;
const VECTOR_SPLINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M3 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 5c-6.627 0 -12 5.373 -12 12" /> </svg>"##;
const VECTOR_TRIANGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M3 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M17 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M6.5 17.1l5 -9.1" /> <path d="M17.5 17.1l-5 -9.1" /> <path d="M7 19l10 0" /> </svg>"##;
const VECTOR_TRIANGLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 6v-1a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-1" /> <path d="M3 18a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -2" /> <path d="M20.705 20.709a1 1 0 0 1 -.705 .291h-2a1 1 0 0 1 -1 -1v-2c0 -.28 .115 -.532 .3 -.714" /> <path d="M6.5 17.1l3.749 -6.823" /> <path d="M13.158 9.197l-.658 -1.197" /> <path d="M7 19h10" /> <path d="M3 3l18 18" /> </svg>"##;
const VIGNETTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 1 -18 0a9 9 0 0 1 18 0" /> <path d="M7.02 12h-.01" /> <path d="M12.02 7h-.01" /> <path d="M17.02 12h-.01" /> <path d="M12.02 17h-.01" /> <path d="M8.483 8.468l-.007 -.007" /> <path d="M15.554 8.468l-.007 -.007" /> <path d="M15.554 15.539l-.007 -.007" /> <path d="M8.483 15.539l-.007 -.007" /> </svg>"##;
const WHEEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M3 12h6" /> <path d="M15 12h6" /> <path d="M13.6 9.4l3.4 -4.8" /> <path d="M10.4 14.6l-3.4 4.8" /> <path d="M7 4.6l3.4 4.8" /> <path d="M13.6 14.6l3.4 4.8" /> </svg>"##;

/// Design icon variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DesignIcon {
    Ad,
    Ad2,
    AdOff,
    Angle,
    Aperture,
    ApertureOff,
    Artboard,
    ArtboardOff,
    Award,
    AwardOff,
    Background,
    Badge,
    Badge2k,
    Badge3d,
    Badge3k,
    Badge4k,
    Badge5k,
    Badge8k,
    BadgeAd,
    BadgeAdOff,
    BadgeAr,
    BadgeCc,
    BadgeHd,
    BadgeOff,
    BadgeSd,
    BadgeTm,
    BadgeVo,
    BadgeVr,
    BadgeWc,
    Badges,
    BadgesOff,
    Barrel,
    BarrelOff,
    BarrierBlock,
    BarrierBlockOff,
    Binoculars,
    Blade,
    BlendMode,
    Blur,
    BlurOff,
    Bolt,
    BoltOff,
    Bomb,
    Bong,
    BongOff,
    Boom,
    BorderAll,
    BorderBottom,
    BorderBottomPlus,
    BorderCornerIos,
    BorderCornerPill,
    BorderCornerRounded,
    BorderCornerSquare,
    BorderCorners,
    BorderHorizontal,
    BorderInner,
    BorderLeft,
    BorderLeftPlus,
    BorderNone,
    BorderOuter,
    BorderRadius,
    BorderRight,
    BorderRightPlus,
    BorderSides,
    BorderStyle,
    BorderStyle2,
    BorderTop,
    BorderTopPlus,
    BorderVertical,
    BounceLeft,
    BounceRight,
    Box,
    BoxAlignBottom,
    BoxAlignBottomLeft,
    BoxAlignBottomRight,
    BoxAlignLeft,
    BoxAlignRight,
    BoxAlignTop,
    BoxAlignTopLeft,
    BoxAlignTopRight,
    BoxMargin,
    BoxModel,
    BoxModel2,
    BoxModel2Off,
    BoxModelOff,
    BoxMultiple,
    BoxOff,
    BoxPadding,
    BracketsAngle,
    BracketsAngleOff,
    Briefcase,
    Briefcase2,
    BriefcaseOff,
    Brightness,
    Brightness2,
    BrightnessAuto,
    BrightnessDown,
    BrightnessHalf,
    BrightnessOff,
    BrightnessUp,
    Brush,
    BrushOff,
    Bucket,
    BucketDroplet,
    BucketOff,
    Bulb,
    BulbOff,
    CameraRotate,
    CameraSelfie,
    Cancel,
    Candle,
    CapProjecting,
    CapRounded,
    CapStraight,
    Capsule,
    CapsuleHorizontal,
    Cards,
    CaretLeftRight,
    CarouselHorizontal,
    CarouselVertical,
    Category,
    Category2,
    CategoryMinus,
    CategoryPlus,
    Cell,
    ChairDirector,
    Chisel,
    ChristmasBall,
    Circle,
    CircleAsterisk,
    CircleCheck,
    CircleDashed,
    CircleDashedCheck,
    CircleDashedMinus,
    CircleDashedPercentage,
    CircleDashedPlus,
    CircleDashedX,
    CircleDot,
    CircleDotted,
    CircleHalf,
    CircleHalf2,
    CircleHalfVertical,
    CircleKey,
    CircleMinus,
    CircleMinus2,
    CircleOff,
    CirclePercentage,
    CirclePlus,
    CirclePlus2,
    CirclePlusMinus,
    CircleRectangle,
    CircleRectangleOff,
    CircleSquare,
    CircleTriangle,
    CircleX,
    Circles,
    CirclesRelation,
    Clubs,
    Coffin,
    ColorFilter,
    ColorPicker,
    ColorPickerOff,
    ColorSwatch,
    ColorSwatchOff,
    Components,
    ComponentsOff,
    Cone,
    Cone2,
    ConeOff,
    ConePlus,
    Confetti,
    ConfettiOff,
    Container,
    ContainerOff,
    Contrast,
    Contrast2,
    Contrast2Off,
    ContrastOff,
    CookieMan,
    Crop,
    Crop11,
    Crop169,
    Crop32,
    Crop54,
    Crop75,
    CropLandscape,
    CropPortrait,
    Crown,
    CrownOff,
    Cube,
    CubeOff,
    CubePlus,
    CubeSpark,
    Cut,
    Cylinder,
    CylinderOff,
    CylinderPlus,
    Diabolo,
    DiaboloOff,
    DiaboloPlus,
    Diamond,
    DiamondOff,
    Diamonds,
    Dimensions,
    DragDrop,
    DragDrop2,
    Droplet,
    DropletBolt,
    DropletCancel,
    DropletCheck,
    DropletCode,
    DropletCog,
    DropletDollar,
    DropletDown,
    DropletExclamation,
    DropletHalf,
    DropletHalf2,
    DropletHeart,
    DropletMinus,
    DropletOff,
    DropletPause,
    DropletPin,
    DropletPlus,
    DropletQuestion,
    DropletSearch,
    DropletShare,
    DropletStar,
    DropletUp,
    DropletX,
    EaseIn,
    EaseInControlPoint,
    EaseInOut,
    EaseInOutControlPoints,
    EaseOut,
    EaseOutControlPoint,
    Edit,
    EditCircle,
    EditCircleOff,
    EditOff,
    Exposure,
    Exposure0,
    ExposureMinus1,
    ExposureMinus2,
    ExposureOff,
    ExposurePlus1,
    ExposurePlus2,
    Favicon,
    FlipHorizontal,
    FlipVertical,
    Focus,
    Focus2,
    FocusAuto,
    Frame,
    FrameOff,
    FreezeColumn,
    FreezeRow,
    FreezeRowColumn,
    Frustum,
    FrustumOff,
    FrustumPlus,
    GenderAgender,
    GenderAndrogyne,
    GenderBigender,
    GenderDemiboy,
    GenderDemigirl,
    GenderEpicene,
    GenderFemale,
    GenderFemme,
    GenderGenderfluid,
    GenderGenderless,
    GenderGenderqueer,
    GenderHermaphrodite,
    GenderIntergender,
    GenderMale,
    GenderNeutrois,
    GenderThird,
    GenderTransgender,
    GenderTrasvesti,
    Gizmo,
    Gradienter,
    Grain,
    Grid3x3,
    Grid4x4,
    GridGoldenratio,
    GuitarPick,
    Hammer,
    HammerOff,
    HandClick,
    HandClickOff,
    HandFinger,
    HandFingerDown,
    HandFingerLeft,
    HandFingerOff,
    HandFingerRight,
    HandGrab,
    HandLittleFinger,
    HandLoveYou,
    HandMiddleFinger,
    HandMove,
    HandOff,
    HandRingFinger,
    HandStop,
    HandThreeFingers,
    HandTwoFingers,
    Hash,
    Hdr,
    Heart,
    HeartBolt,
    HeartCancel,
    HeartCheck,
    HeartCode,
    HeartCog,
    HeartDollar,
    HeartDown,
    HeartExclamation,
    HeartMinus,
    HeartOff,
    HeartPause,
    HeartPin,
    HeartPlus,
    HeartQuestion,
    HeartSearch,
    HeartShare,
    HeartSpark,
    HeartStar,
    HeartUp,
    HeartX,
    Hearts,
    HeartsOff,
    Hemisphere,
    HemisphereOff,
    HemispherePlus,
    Hexagon,
    Hexagon3d,
    HexagonAsterisk,
    HexagonMinus,
    HexagonMinus2,
    HexagonOff,
    HexagonPlus,
    HexagonPlus2,
    HexagonalPrism,
    HexagonalPrismOff,
    HexagonalPrismPlus,
    HexagonalPyramid,
    HexagonalPyramidOff,
    HexagonalPyramidPlus,
    Hexagons,
    HexagonsOff,
    Hierarchy,
    Hierarchy2,
    Hierarchy3,
    HierarchyOff,
    Icons,
    IconsOff,
    Ikosaedr,
    ImageInPicture,
    InnerShadowBottom,
    InnerShadowBottomLeft,
    InnerShadowBottomRight,
    InnerShadowLeft,
    InnerShadowRight,
    InnerShadowTop,
    InnerShadowTopLeft,
    InnerShadowTopRight,
    IrregularPolyhedron,
    IrregularPolyhedronOff,
    IrregularPolyhedronPlus,
    JewishStar,
    Key,
    KeyOff,
    Ladle,
    Lamp,
    Lamp2,
    LampOff,
    Lasso,
    LassoOff,
    LassoPolygon,
    LaurelWreath,
    LaurelWreath1,
    LaurelWreath2,
    LaurelWreath3,
    LayersDifference,
    LayersIntersect,
    LayersIntersect2,
    LayersLinked,
    LayersOff,
    LayersSelected,
    LayersSelectedBottom,
    LayersSubtract,
    LayersUnion,
    Layout,
    Layout2,
    LayoutAlignBottom,
    LayoutAlignCenter,
    LayoutAlignLeft,
    LayoutAlignMiddle,
    LayoutAlignRight,
    LayoutAlignTop,
    LayoutBoard,
    LayoutBoardSplit,
    LayoutBottombar,
    LayoutBottombarCollapse,
    LayoutBottombarExpand,
    LayoutCards,
    LayoutCollage,
    LayoutColumns,
    LayoutDashboard,
    LayoutDistributeHorizontal,
    LayoutDistributeVertical,
    LayoutGrid,
    LayoutGridAdd,
    LayoutGridRemove,
    LayoutKanban,
    LayoutList,
    LayoutNavbar,
    LayoutNavbarCollapse,
    LayoutNavbarExpand,
    LayoutOff,
    LayoutRows,
    LayoutSidebar,
    LayoutSidebarLeftCollapse,
    LayoutSidebarLeftExpand,
    LayoutSidebarRight,
    LayoutSidebarRightCollapse,
    LayoutSidebarRightExpand,
    Line,
    LineDashed,
    LineDotted,
    LivePhoto,
    LivePhotoOff,
    Macro,
    MacroOff,
    Magnet,
    MagnetOff,
    Magnetic,
    Mask,
    MaskOff,
    Mesh,
    Mickey,
    MoodAngry,
    MoodAnnoyed,
    MoodAnnoyed2,
    MoodBitcoin,
    MoodBoy,
    MoodCheck,
    MoodCog,
    MoodConfuzed,
    MoodCrazyHappy,
    MoodCry,
    MoodDollar,
    MoodEdit,
    MoodEmpty,
    MoodHappy,
    MoodHeart,
    MoodKid,
    MoodLookDown,
    MoodLookLeft,
    MoodLookRight,
    MoodLookUp,
    MoodMinus,
    MoodNerd,
    MoodNervous,
    MoodNeutral,
    MoodOff,
    MoodPin,
    MoodPlus,
    MoodPuzzled,
    MoodSad,
    MoodSad2,
    MoodSadDizzy,
    MoodSadSquint,
    MoodSearch,
    MoodShare,
    MoodSick,
    MoodSilence,
    MoodSing,
    MoodSmile,
    MoodSmileBeam,
    MoodSmileDizzy,
    MoodSpark,
    MoodSurprised,
    MoodTongue,
    MoodTongueWink,
    MoodTongueWink2,
    MoodUnamused,
    MoodUp,
    MoodWink,
    MoodWink2,
    MoodWrrr,
    MoodX,
    MoodXd,
    Moustache,
    Needle,
    NeedleThread,
    NoiseReduction,
    Octagon,
    OctagonMinus,
    OctagonMinus2,
    OctagonOff,
    OctagonPlus,
    OctagonPlus2,
    Octahedron,
    OctahedronOff,
    OctahedronPlus,
    Oval,
    OvalVertical,
    Paint,
    PaintOff,
    Palette,
    PaletteOff,
    PanoramaHorizontal,
    PanoramaHorizontalOff,
    PanoramaVertical,
    PanoramaVerticalOff,
    Pencil,
    PencilBolt,
    PencilCancel,
    PencilCheck,
    PencilCode,
    PencilCog,
    PencilDiscount,
    PencilDollar,
    PencilDown,
    PencilExclamation,
    PencilHeart,
    PencilMinus,
    PencilOff,
    PencilPause,
    PencilPin,
    PencilPlus,
    PencilQuestion,
    PencilSearch,
    PencilShare,
    PencilStar,
    PencilUp,
    PencilX,
    Pentagon,
    PentagonMinus,
    PentagonOff,
    PentagonPlus,
    PentagonX,
    Pentagram,
    Perspective,
    PerspectiveOff,
    PhotoBitcoin,
    PhotoCircle,
    PhotoHexagon,
    PhotoPentagon,
    PhotoSensor,
    PhotoSensor2,
    PhotoSensor3,
    PhotoSquareRounded,
    PhotoVideo,
    Placeholder,
    Polaroid,
    Polygon,
    PolygonOff,
    Prism,
    PrismOff,
    PrismPlus,
    Prong,
    Pyramid,
    PyramidOff,
    PyramidPlus,
    RadiusBottomLeft,
    RadiusBottomRight,
    RadiusTopLeft,
    RadiusTopRight,
    Rectangle,
    RectangleRoundedBottom,
    RectangleRoundedTop,
    RectangleVertical,
    RectangularPrism,
    RectangularPrismOff,
    RectangularPrismPlus,
    Resize,
    Rosette,
    RosetteAsterisk,
    Ruler,
    Ruler2,
    Ruler2Off,
    Ruler3,
    RulerMeasure,
    RulerMeasure2,
    RulerOff,
    Scissors,
    ScissorsOff,
    Screenshot,
    Section,
    Shadow,
    ShadowOff,
    Shape,
    Shape2,
    Shape3,
    ShapeOff,
    Shovel,
    ShovelPitchforks,
    Sketching,
    Slice,
    Spade,
    Sparkle,
    Sparkle2,
    SparkleHighlight,
    Sparkles2,
    Sphere,
    Sphere2,
    SphereOff,
    SpherePlus,
    Square,
    SquareAsterisk,
    SquareCheck,
    SquareDashed,
    SquareDot,
    SquareForbid,
    SquareForbid2,
    SquareHalf,
    SquareKey,
    SquareMinus,
    SquareMinus2,
    SquareOff,
    SquarePercentage,
    SquarePlus,
    SquarePlus2,
    SquareRotated,
    SquareRotatedAsterisk,
    SquareRotatedForbid,
    SquareRotatedForbid2,
    SquareRotatedOff,
    SquareRounded,
    SquareRoundedCheck,
    SquareRoundedMinus,
    SquareRoundedMinus2,
    SquareRoundedPercentage,
    SquareRoundedPlus,
    SquareRoundedPlus2,
    SquareRoundedX,
    SquareToggle,
    SquareToggleHorizontal,
    SquareX,
    Squares,
    SquaresDiagonal,
    SquaresSelected,
    Stack,
    Stack2,
    Stack3,
    StackPop,
    StackPush,
    Sticker,
    Sticker2,
    StrokeCurved,
    StrokeDynamic,
    StrokeStraight,
    Template,
    TemplateOff,
    TextResize,
    TiltShift,
    TiltShiftOff,
    Tools,
    ToolsOff,
    Triangle,
    TriangleInverted,
    TriangleMinus,
    TriangleMinus2,
    TriangleOff,
    TrianglePlus,
    TrianglePlus2,
    TriangleSquareCircle,
    Triangles,
    Trident,
    Typeface,
    UxCircle,
    Vector,
    VectorBezier,
    VectorBezier2,
    VectorBezierArc,
    VectorBezierCircle,
    VectorOff,
    VectorSpline,
    VectorTriangle,
    VectorTriangleOff,
    Vignette,
    Wheel,
}

impl DesignIcon {
    /// Returns all available icons in this category.
    pub fn all() -> &'static [Self] {
        &[Self::Ad, Self::Ad2, Self::AdOff, Self::Angle, Self::Aperture, Self::ApertureOff, Self::Artboard, Self::ArtboardOff, Self::Award, Self::AwardOff, Self::Background, Self::Badge, Self::Badge2k, Self::Badge3d, Self::Badge3k, Self::Badge4k, Self::Badge5k, Self::Badge8k, Self::BadgeAd, Self::BadgeAdOff, Self::BadgeAr, Self::BadgeCc, Self::BadgeHd, Self::BadgeOff, Self::BadgeSd, Self::BadgeTm, Self::BadgeVo, Self::BadgeVr, Self::BadgeWc, Self::Badges, Self::BadgesOff, Self::Barrel, Self::BarrelOff, Self::BarrierBlock, Self::BarrierBlockOff, Self::Binoculars, Self::Blade, Self::BlendMode, Self::Blur, Self::BlurOff, Self::Bolt, Self::BoltOff, Self::Bomb, Self::Bong, Self::BongOff, Self::Boom, Self::BorderAll, Self::BorderBottom, Self::BorderBottomPlus, Self::BorderCornerIos, Self::BorderCornerPill, Self::BorderCornerRounded, Self::BorderCornerSquare, Self::BorderCorners, Self::BorderHorizontal, Self::BorderInner, Self::BorderLeft, Self::BorderLeftPlus, Self::BorderNone, Self::BorderOuter, Self::BorderRadius, Self::BorderRight, Self::BorderRightPlus, Self::BorderSides, Self::BorderStyle, Self::BorderStyle2, Self::BorderTop, Self::BorderTopPlus, Self::BorderVertical, Self::BounceLeft, Self::BounceRight, Self::Box, Self::BoxAlignBottom, Self::BoxAlignBottomLeft, Self::BoxAlignBottomRight, Self::BoxAlignLeft, Self::BoxAlignRight, Self::BoxAlignTop, Self::BoxAlignTopLeft, Self::BoxAlignTopRight, Self::BoxMargin, Self::BoxModel, Self::BoxModel2, Self::BoxModel2Off, Self::BoxModelOff, Self::BoxMultiple, Self::BoxOff, Self::BoxPadding, Self::BracketsAngle, Self::BracketsAngleOff, Self::Briefcase, Self::Briefcase2, Self::BriefcaseOff, Self::Brightness, Self::Brightness2, Self::BrightnessAuto, Self::BrightnessDown, Self::BrightnessHalf, Self::BrightnessOff, Self::BrightnessUp, Self::Brush, Self::BrushOff, Self::Bucket, Self::BucketDroplet, Self::BucketOff, Self::Bulb, Self::BulbOff, Self::CameraRotate, Self::CameraSelfie, Self::Cancel, Self::Candle, Self::CapProjecting, Self::CapRounded, Self::CapStraight, Self::Capsule, Self::CapsuleHorizontal, Self::Cards, Self::CaretLeftRight, Self::CarouselHorizontal, Self::CarouselVertical, Self::Category, Self::Category2, Self::CategoryMinus, Self::CategoryPlus, Self::Cell, Self::ChairDirector, Self::Chisel, Self::ChristmasBall, Self::Circle, Self::CircleAsterisk, Self::CircleCheck, Self::CircleDashed, Self::CircleDashedCheck, Self::CircleDashedMinus, Self::CircleDashedPercentage, Self::CircleDashedPlus, Self::CircleDashedX, Self::CircleDot, Self::CircleDotted, Self::CircleHalf, Self::CircleHalf2, Self::CircleHalfVertical, Self::CircleKey, Self::CircleMinus, Self::CircleMinus2, Self::CircleOff, Self::CirclePercentage, Self::CirclePlus, Self::CirclePlus2, Self::CirclePlusMinus, Self::CircleRectangle, Self::CircleRectangleOff, Self::CircleSquare, Self::CircleTriangle, Self::CircleX, Self::Circles, Self::CirclesRelation, Self::Clubs, Self::Coffin, Self::ColorFilter, Self::ColorPicker, Self::ColorPickerOff, Self::ColorSwatch, Self::ColorSwatchOff, Self::Components, Self::ComponentsOff, Self::Cone, Self::Cone2, Self::ConeOff, Self::ConePlus, Self::Confetti, Self::ConfettiOff, Self::Container, Self::ContainerOff, Self::Contrast, Self::Contrast2, Self::Contrast2Off, Self::ContrastOff, Self::CookieMan, Self::Crop, Self::Crop11, Self::Crop169, Self::Crop32, Self::Crop54, Self::Crop75, Self::CropLandscape, Self::CropPortrait, Self::Crown, Self::CrownOff, Self::Cube, Self::CubeOff, Self::CubePlus, Self::CubeSpark, Self::Cut, Self::Cylinder, Self::CylinderOff, Self::CylinderPlus, Self::Diabolo, Self::DiaboloOff, Self::DiaboloPlus, Self::Diamond, Self::DiamondOff, Self::Diamonds, Self::Dimensions, Self::DragDrop, Self::DragDrop2, Self::Droplet, Self::DropletBolt, Self::DropletCancel, Self::DropletCheck, Self::DropletCode, Self::DropletCog, Self::DropletDollar, Self::DropletDown, Self::DropletExclamation, Self::DropletHalf, Self::DropletHalf2, Self::DropletHeart, Self::DropletMinus, Self::DropletOff, Self::DropletPause, Self::DropletPin, Self::DropletPlus, Self::DropletQuestion, Self::DropletSearch, Self::DropletShare, Self::DropletStar, Self::DropletUp, Self::DropletX, Self::EaseIn, Self::EaseInControlPoint, Self::EaseInOut, Self::EaseInOutControlPoints, Self::EaseOut, Self::EaseOutControlPoint, Self::Edit, Self::EditCircle, Self::EditCircleOff, Self::EditOff, Self::Exposure, Self::Exposure0, Self::ExposureMinus1, Self::ExposureMinus2, Self::ExposureOff, Self::ExposurePlus1, Self::ExposurePlus2, Self::Favicon, Self::FlipHorizontal, Self::FlipVertical, Self::Focus, Self::Focus2, Self::FocusAuto, Self::Frame, Self::FrameOff, Self::FreezeColumn, Self::FreezeRow, Self::FreezeRowColumn, Self::Frustum, Self::FrustumOff, Self::FrustumPlus, Self::GenderAgender, Self::GenderAndrogyne, Self::GenderBigender, Self::GenderDemiboy, Self::GenderDemigirl, Self::GenderEpicene, Self::GenderFemale, Self::GenderFemme, Self::GenderGenderfluid, Self::GenderGenderless, Self::GenderGenderqueer, Self::GenderHermaphrodite, Self::GenderIntergender, Self::GenderMale, Self::GenderNeutrois, Self::GenderThird, Self::GenderTransgender, Self::GenderTrasvesti, Self::Gizmo, Self::Gradienter, Self::Grain, Self::Grid3x3, Self::Grid4x4, Self::GridGoldenratio, Self::GuitarPick, Self::Hammer, Self::HammerOff, Self::HandClick, Self::HandClickOff, Self::HandFinger, Self::HandFingerDown, Self::HandFingerLeft, Self::HandFingerOff, Self::HandFingerRight, Self::HandGrab, Self::HandLittleFinger, Self::HandLoveYou, Self::HandMiddleFinger, Self::HandMove, Self::HandOff, Self::HandRingFinger, Self::HandStop, Self::HandThreeFingers, Self::HandTwoFingers, Self::Hash, Self::Hdr, Self::Heart, Self::HeartBolt, Self::HeartCancel, Self::HeartCheck, Self::HeartCode, Self::HeartCog, Self::HeartDollar, Self::HeartDown, Self::HeartExclamation, Self::HeartMinus, Self::HeartOff, Self::HeartPause, Self::HeartPin, Self::HeartPlus, Self::HeartQuestion, Self::HeartSearch, Self::HeartShare, Self::HeartSpark, Self::HeartStar, Self::HeartUp, Self::HeartX, Self::Hearts, Self::HeartsOff, Self::Hemisphere, Self::HemisphereOff, Self::HemispherePlus, Self::Hexagon, Self::Hexagon3d, Self::HexagonAsterisk, Self::HexagonMinus, Self::HexagonMinus2, Self::HexagonOff, Self::HexagonPlus, Self::HexagonPlus2, Self::HexagonalPrism, Self::HexagonalPrismOff, Self::HexagonalPrismPlus, Self::HexagonalPyramid, Self::HexagonalPyramidOff, Self::HexagonalPyramidPlus, Self::Hexagons, Self::HexagonsOff, Self::Hierarchy, Self::Hierarchy2, Self::Hierarchy3, Self::HierarchyOff, Self::Icons, Self::IconsOff, Self::Ikosaedr, Self::ImageInPicture, Self::InnerShadowBottom, Self::InnerShadowBottomLeft, Self::InnerShadowBottomRight, Self::InnerShadowLeft, Self::InnerShadowRight, Self::InnerShadowTop, Self::InnerShadowTopLeft, Self::InnerShadowTopRight, Self::IrregularPolyhedron, Self::IrregularPolyhedronOff, Self::IrregularPolyhedronPlus, Self::JewishStar, Self::Key, Self::KeyOff, Self::Ladle, Self::Lamp, Self::Lamp2, Self::LampOff, Self::Lasso, Self::LassoOff, Self::LassoPolygon, Self::LaurelWreath, Self::LaurelWreath1, Self::LaurelWreath2, Self::LaurelWreath3, Self::LayersDifference, Self::LayersIntersect, Self::LayersIntersect2, Self::LayersLinked, Self::LayersOff, Self::LayersSelected, Self::LayersSelectedBottom, Self::LayersSubtract, Self::LayersUnion, Self::Layout, Self::Layout2, Self::LayoutAlignBottom, Self::LayoutAlignCenter, Self::LayoutAlignLeft, Self::LayoutAlignMiddle, Self::LayoutAlignRight, Self::LayoutAlignTop, Self::LayoutBoard, Self::LayoutBoardSplit, Self::LayoutBottombar, Self::LayoutBottombarCollapse, Self::LayoutBottombarExpand, Self::LayoutCards, Self::LayoutCollage, Self::LayoutColumns, Self::LayoutDashboard, Self::LayoutDistributeHorizontal, Self::LayoutDistributeVertical, Self::LayoutGrid, Self::LayoutGridAdd, Self::LayoutGridRemove, Self::LayoutKanban, Self::LayoutList, Self::LayoutNavbar, Self::LayoutNavbarCollapse, Self::LayoutNavbarExpand, Self::LayoutOff, Self::LayoutRows, Self::LayoutSidebar, Self::LayoutSidebarLeftCollapse, Self::LayoutSidebarLeftExpand, Self::LayoutSidebarRight, Self::LayoutSidebarRightCollapse, Self::LayoutSidebarRightExpand, Self::Line, Self::LineDashed, Self::LineDotted, Self::LivePhoto, Self::LivePhotoOff, Self::Macro, Self::MacroOff, Self::Magnet, Self::MagnetOff, Self::Magnetic, Self::Mask, Self::MaskOff, Self::Mesh, Self::Mickey, Self::MoodAngry, Self::MoodAnnoyed, Self::MoodAnnoyed2, Self::MoodBitcoin, Self::MoodBoy, Self::MoodCheck, Self::MoodCog, Self::MoodConfuzed, Self::MoodCrazyHappy, Self::MoodCry, Self::MoodDollar, Self::MoodEdit, Self::MoodEmpty, Self::MoodHappy, Self::MoodHeart, Self::MoodKid, Self::MoodLookDown, Self::MoodLookLeft, Self::MoodLookRight, Self::MoodLookUp, Self::MoodMinus, Self::MoodNerd, Self::MoodNervous, Self::MoodNeutral, Self::MoodOff, Self::MoodPin, Self::MoodPlus, Self::MoodPuzzled, Self::MoodSad, Self::MoodSad2, Self::MoodSadDizzy, Self::MoodSadSquint, Self::MoodSearch, Self::MoodShare, Self::MoodSick, Self::MoodSilence, Self::MoodSing, Self::MoodSmile, Self::MoodSmileBeam, Self::MoodSmileDizzy, Self::MoodSpark, Self::MoodSurprised, Self::MoodTongue, Self::MoodTongueWink, Self::MoodTongueWink2, Self::MoodUnamused, Self::MoodUp, Self::MoodWink, Self::MoodWink2, Self::MoodWrrr, Self::MoodX, Self::MoodXd, Self::Moustache, Self::Needle, Self::NeedleThread, Self::NoiseReduction, Self::Octagon, Self::OctagonMinus, Self::OctagonMinus2, Self::OctagonOff, Self::OctagonPlus, Self::OctagonPlus2, Self::Octahedron, Self::OctahedronOff, Self::OctahedronPlus, Self::Oval, Self::OvalVertical, Self::Paint, Self::PaintOff, Self::Palette, Self::PaletteOff, Self::PanoramaHorizontal, Self::PanoramaHorizontalOff, Self::PanoramaVertical, Self::PanoramaVerticalOff, Self::Pencil, Self::PencilBolt, Self::PencilCancel, Self::PencilCheck, Self::PencilCode, Self::PencilCog, Self::PencilDiscount, Self::PencilDollar, Self::PencilDown, Self::PencilExclamation, Self::PencilHeart, Self::PencilMinus, Self::PencilOff, Self::PencilPause, Self::PencilPin, Self::PencilPlus, Self::PencilQuestion, Self::PencilSearch, Self::PencilShare, Self::PencilStar, Self::PencilUp, Self::PencilX, Self::Pentagon, Self::PentagonMinus, Self::PentagonOff, Self::PentagonPlus, Self::PentagonX, Self::Pentagram, Self::Perspective, Self::PerspectiveOff, Self::PhotoBitcoin, Self::PhotoCircle, Self::PhotoHexagon, Self::PhotoPentagon, Self::PhotoSensor, Self::PhotoSensor2, Self::PhotoSensor3, Self::PhotoSquareRounded, Self::PhotoVideo, Self::Placeholder, Self::Polaroid, Self::Polygon, Self::PolygonOff, Self::Prism, Self::PrismOff, Self::PrismPlus, Self::Prong, Self::Pyramid, Self::PyramidOff, Self::PyramidPlus, Self::RadiusBottomLeft, Self::RadiusBottomRight, Self::RadiusTopLeft, Self::RadiusTopRight, Self::Rectangle, Self::RectangleRoundedBottom, Self::RectangleRoundedTop, Self::RectangleVertical, Self::RectangularPrism, Self::RectangularPrismOff, Self::RectangularPrismPlus, Self::Resize, Self::Rosette, Self::RosetteAsterisk, Self::Ruler, Self::Ruler2, Self::Ruler2Off, Self::Ruler3, Self::RulerMeasure, Self::RulerMeasure2, Self::RulerOff, Self::Scissors, Self::ScissorsOff, Self::Screenshot, Self::Section, Self::Shadow, Self::ShadowOff, Self::Shape, Self::Shape2, Self::Shape3, Self::ShapeOff, Self::Shovel, Self::ShovelPitchforks, Self::Sketching, Self::Slice, Self::Spade, Self::Sparkle, Self::Sparkle2, Self::SparkleHighlight, Self::Sparkles2, Self::Sphere, Self::Sphere2, Self::SphereOff, Self::SpherePlus, Self::Square, Self::SquareAsterisk, Self::SquareCheck, Self::SquareDashed, Self::SquareDot, Self::SquareForbid, Self::SquareForbid2, Self::SquareHalf, Self::SquareKey, Self::SquareMinus, Self::SquareMinus2, Self::SquareOff, Self::SquarePercentage, Self::SquarePlus, Self::SquarePlus2, Self::SquareRotated, Self::SquareRotatedAsterisk, Self::SquareRotatedForbid, Self::SquareRotatedForbid2, Self::SquareRotatedOff, Self::SquareRounded, Self::SquareRoundedCheck, Self::SquareRoundedMinus, Self::SquareRoundedMinus2, Self::SquareRoundedPercentage, Self::SquareRoundedPlus, Self::SquareRoundedPlus2, Self::SquareRoundedX, Self::SquareToggle, Self::SquareToggleHorizontal, Self::SquareX, Self::Squares, Self::SquaresDiagonal, Self::SquaresSelected, Self::Stack, Self::Stack2, Self::Stack3, Self::StackPop, Self::StackPush, Self::Sticker, Self::Sticker2, Self::StrokeCurved, Self::StrokeDynamic, Self::StrokeStraight, Self::Template, Self::TemplateOff, Self::TextResize, Self::TiltShift, Self::TiltShiftOff, Self::Tools, Self::ToolsOff, Self::Triangle, Self::TriangleInverted, Self::TriangleMinus, Self::TriangleMinus2, Self::TriangleOff, Self::TrianglePlus, Self::TrianglePlus2, Self::TriangleSquareCircle, Self::Triangles, Self::Trident, Self::Typeface, Self::UxCircle, Self::Vector, Self::VectorBezier, Self::VectorBezier2, Self::VectorBezierArc, Self::VectorBezierCircle, Self::VectorOff, Self::VectorSpline, Self::VectorTriangle, Self::VectorTriangleOff, Self::Vignette, Self::Wheel]
    }

    /// Returns the icon count.
    pub fn count() -> usize {
        682
    }

    /// Creates an icon from its kebab-case name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "ad" => Some(Self::Ad),
            "ad-2" => Some(Self::Ad2),
            "ad-off" => Some(Self::AdOff),
            "angle" => Some(Self::Angle),
            "aperture" => Some(Self::Aperture),
            "aperture-off" => Some(Self::ApertureOff),
            "artboard" => Some(Self::Artboard),
            "artboard-off" => Some(Self::ArtboardOff),
            "award" => Some(Self::Award),
            "award-off" => Some(Self::AwardOff),
            "background" => Some(Self::Background),
            "badge" => Some(Self::Badge),
            "badge-2k" => Some(Self::Badge2k),
            "badge-3d" => Some(Self::Badge3d),
            "badge-3k" => Some(Self::Badge3k),
            "badge-4k" => Some(Self::Badge4k),
            "badge-5k" => Some(Self::Badge5k),
            "badge-8k" => Some(Self::Badge8k),
            "badge-ad" => Some(Self::BadgeAd),
            "badge-ad-off" => Some(Self::BadgeAdOff),
            "badge-ar" => Some(Self::BadgeAr),
            "badge-cc" => Some(Self::BadgeCc),
            "badge-hd" => Some(Self::BadgeHd),
            "badge-off" => Some(Self::BadgeOff),
            "badge-sd" => Some(Self::BadgeSd),
            "badge-tm" => Some(Self::BadgeTm),
            "badge-vo" => Some(Self::BadgeVo),
            "badge-vr" => Some(Self::BadgeVr),
            "badge-wc" => Some(Self::BadgeWc),
            "badges" => Some(Self::Badges),
            "badges-off" => Some(Self::BadgesOff),
            "barrel" => Some(Self::Barrel),
            "barrel-off" => Some(Self::BarrelOff),
            "barrier-block" => Some(Self::BarrierBlock),
            "barrier-block-off" => Some(Self::BarrierBlockOff),
            "binoculars" => Some(Self::Binoculars),
            "blade" => Some(Self::Blade),
            "blend-mode" => Some(Self::BlendMode),
            "blur" => Some(Self::Blur),
            "blur-off" => Some(Self::BlurOff),
            "bolt" => Some(Self::Bolt),
            "bolt-off" => Some(Self::BoltOff),
            "bomb" => Some(Self::Bomb),
            "bong" => Some(Self::Bong),
            "bong-off" => Some(Self::BongOff),
            "boom" => Some(Self::Boom),
            "border-all" => Some(Self::BorderAll),
            "border-bottom" => Some(Self::BorderBottom),
            "border-bottom-plus" => Some(Self::BorderBottomPlus),
            "border-corner-ios" => Some(Self::BorderCornerIos),
            "border-corner-pill" => Some(Self::BorderCornerPill),
            "border-corner-rounded" => Some(Self::BorderCornerRounded),
            "border-corner-square" => Some(Self::BorderCornerSquare),
            "border-corners" => Some(Self::BorderCorners),
            "border-horizontal" => Some(Self::BorderHorizontal),
            "border-inner" => Some(Self::BorderInner),
            "border-left" => Some(Self::BorderLeft),
            "border-left-plus" => Some(Self::BorderLeftPlus),
            "border-none" => Some(Self::BorderNone),
            "border-outer" => Some(Self::BorderOuter),
            "border-radius" => Some(Self::BorderRadius),
            "border-right" => Some(Self::BorderRight),
            "border-right-plus" => Some(Self::BorderRightPlus),
            "border-sides" => Some(Self::BorderSides),
            "border-style" => Some(Self::BorderStyle),
            "border-style-2" => Some(Self::BorderStyle2),
            "border-top" => Some(Self::BorderTop),
            "border-top-plus" => Some(Self::BorderTopPlus),
            "border-vertical" => Some(Self::BorderVertical),
            "bounce-left" => Some(Self::BounceLeft),
            "bounce-right" => Some(Self::BounceRight),
            "box" => Some(Self::Box),
            "box-align-bottom" => Some(Self::BoxAlignBottom),
            "box-align-bottom-left" => Some(Self::BoxAlignBottomLeft),
            "box-align-bottom-right" => Some(Self::BoxAlignBottomRight),
            "box-align-left" => Some(Self::BoxAlignLeft),
            "box-align-right" => Some(Self::BoxAlignRight),
            "box-align-top" => Some(Self::BoxAlignTop),
            "box-align-top-left" => Some(Self::BoxAlignTopLeft),
            "box-align-top-right" => Some(Self::BoxAlignTopRight),
            "box-margin" => Some(Self::BoxMargin),
            "box-model" => Some(Self::BoxModel),
            "box-model-2" => Some(Self::BoxModel2),
            "box-model-2-off" => Some(Self::BoxModel2Off),
            "box-model-off" => Some(Self::BoxModelOff),
            "box-multiple" => Some(Self::BoxMultiple),
            "box-off" => Some(Self::BoxOff),
            "box-padding" => Some(Self::BoxPadding),
            "brackets-angle" => Some(Self::BracketsAngle),
            "brackets-angle-off" => Some(Self::BracketsAngleOff),
            "briefcase" => Some(Self::Briefcase),
            "briefcase-2" => Some(Self::Briefcase2),
            "briefcase-off" => Some(Self::BriefcaseOff),
            "brightness" => Some(Self::Brightness),
            "brightness-2" => Some(Self::Brightness2),
            "brightness-auto" => Some(Self::BrightnessAuto),
            "brightness-down" => Some(Self::BrightnessDown),
            "brightness-half" => Some(Self::BrightnessHalf),
            "brightness-off" => Some(Self::BrightnessOff),
            "brightness-up" => Some(Self::BrightnessUp),
            "brush" => Some(Self::Brush),
            "brush-off" => Some(Self::BrushOff),
            "bucket" => Some(Self::Bucket),
            "bucket-droplet" => Some(Self::BucketDroplet),
            "bucket-off" => Some(Self::BucketOff),
            "bulb" => Some(Self::Bulb),
            "bulb-off" => Some(Self::BulbOff),
            "camera-rotate" => Some(Self::CameraRotate),
            "camera-selfie" => Some(Self::CameraSelfie),
            "cancel" => Some(Self::Cancel),
            "candle" => Some(Self::Candle),
            "cap-projecting" => Some(Self::CapProjecting),
            "cap-rounded" => Some(Self::CapRounded),
            "cap-straight" => Some(Self::CapStraight),
            "capsule" => Some(Self::Capsule),
            "capsule-horizontal" => Some(Self::CapsuleHorizontal),
            "cards" => Some(Self::Cards),
            "caret-left-right" => Some(Self::CaretLeftRight),
            "carousel-horizontal" => Some(Self::CarouselHorizontal),
            "carousel-vertical" => Some(Self::CarouselVertical),
            "category" => Some(Self::Category),
            "category-2" => Some(Self::Category2),
            "category-minus" => Some(Self::CategoryMinus),
            "category-plus" => Some(Self::CategoryPlus),
            "cell" => Some(Self::Cell),
            "chair-director" => Some(Self::ChairDirector),
            "chisel" => Some(Self::Chisel),
            "christmas-ball" => Some(Self::ChristmasBall),
            "circle" => Some(Self::Circle),
            "circle-asterisk" => Some(Self::CircleAsterisk),
            "circle-check" => Some(Self::CircleCheck),
            "circle-dashed" => Some(Self::CircleDashed),
            "circle-dashed-check" => Some(Self::CircleDashedCheck),
            "circle-dashed-minus" => Some(Self::CircleDashedMinus),
            "circle-dashed-percentage" => Some(Self::CircleDashedPercentage),
            "circle-dashed-plus" => Some(Self::CircleDashedPlus),
            "circle-dashed-x" => Some(Self::CircleDashedX),
            "circle-dot" => Some(Self::CircleDot),
            "circle-dotted" => Some(Self::CircleDotted),
            "circle-half" => Some(Self::CircleHalf),
            "circle-half-2" => Some(Self::CircleHalf2),
            "circle-half-vertical" => Some(Self::CircleHalfVertical),
            "circle-key" => Some(Self::CircleKey),
            "circle-minus" => Some(Self::CircleMinus),
            "circle-minus-2" => Some(Self::CircleMinus2),
            "circle-off" => Some(Self::CircleOff),
            "circle-percentage" => Some(Self::CirclePercentage),
            "circle-plus" => Some(Self::CirclePlus),
            "circle-plus-2" => Some(Self::CirclePlus2),
            "circle-plus-minus" => Some(Self::CirclePlusMinus),
            "circle-rectangle" => Some(Self::CircleRectangle),
            "circle-rectangle-off" => Some(Self::CircleRectangleOff),
            "circle-square" => Some(Self::CircleSquare),
            "circle-triangle" => Some(Self::CircleTriangle),
            "circle-x" => Some(Self::CircleX),
            "circles" => Some(Self::Circles),
            "circles-relation" => Some(Self::CirclesRelation),
            "clubs" => Some(Self::Clubs),
            "coffin" => Some(Self::Coffin),
            "color-filter" => Some(Self::ColorFilter),
            "color-picker" => Some(Self::ColorPicker),
            "color-picker-off" => Some(Self::ColorPickerOff),
            "color-swatch" => Some(Self::ColorSwatch),
            "color-swatch-off" => Some(Self::ColorSwatchOff),
            "components" => Some(Self::Components),
            "components-off" => Some(Self::ComponentsOff),
            "cone" => Some(Self::Cone),
            "cone-2" => Some(Self::Cone2),
            "cone-off" => Some(Self::ConeOff),
            "cone-plus" => Some(Self::ConePlus),
            "confetti" => Some(Self::Confetti),
            "confetti-off" => Some(Self::ConfettiOff),
            "container" => Some(Self::Container),
            "container-off" => Some(Self::ContainerOff),
            "contrast" => Some(Self::Contrast),
            "contrast-2" => Some(Self::Contrast2),
            "contrast-2-off" => Some(Self::Contrast2Off),
            "contrast-off" => Some(Self::ContrastOff),
            "cookie-man" => Some(Self::CookieMan),
            "crop" => Some(Self::Crop),
            "crop-1-1" => Some(Self::Crop11),
            "crop-16-9" => Some(Self::Crop169),
            "crop-3-2" => Some(Self::Crop32),
            "crop-5-4" => Some(Self::Crop54),
            "crop-7-5" => Some(Self::Crop75),
            "crop-landscape" => Some(Self::CropLandscape),
            "crop-portrait" => Some(Self::CropPortrait),
            "crown" => Some(Self::Crown),
            "crown-off" => Some(Self::CrownOff),
            "cube" => Some(Self::Cube),
            "cube-off" => Some(Self::CubeOff),
            "cube-plus" => Some(Self::CubePlus),
            "cube-spark" => Some(Self::CubeSpark),
            "cut" => Some(Self::Cut),
            "cylinder" => Some(Self::Cylinder),
            "cylinder-off" => Some(Self::CylinderOff),
            "cylinder-plus" => Some(Self::CylinderPlus),
            "diabolo" => Some(Self::Diabolo),
            "diabolo-off" => Some(Self::DiaboloOff),
            "diabolo-plus" => Some(Self::DiaboloPlus),
            "diamond" => Some(Self::Diamond),
            "diamond-off" => Some(Self::DiamondOff),
            "diamonds" => Some(Self::Diamonds),
            "dimensions" => Some(Self::Dimensions),
            "drag-drop" => Some(Self::DragDrop),
            "drag-drop-2" => Some(Self::DragDrop2),
            "droplet" => Some(Self::Droplet),
            "droplet-bolt" => Some(Self::DropletBolt),
            "droplet-cancel" => Some(Self::DropletCancel),
            "droplet-check" => Some(Self::DropletCheck),
            "droplet-code" => Some(Self::DropletCode),
            "droplet-cog" => Some(Self::DropletCog),
            "droplet-dollar" => Some(Self::DropletDollar),
            "droplet-down" => Some(Self::DropletDown),
            "droplet-exclamation" => Some(Self::DropletExclamation),
            "droplet-half" => Some(Self::DropletHalf),
            "droplet-half-2" => Some(Self::DropletHalf2),
            "droplet-heart" => Some(Self::DropletHeart),
            "droplet-minus" => Some(Self::DropletMinus),
            "droplet-off" => Some(Self::DropletOff),
            "droplet-pause" => Some(Self::DropletPause),
            "droplet-pin" => Some(Self::DropletPin),
            "droplet-plus" => Some(Self::DropletPlus),
            "droplet-question" => Some(Self::DropletQuestion),
            "droplet-search" => Some(Self::DropletSearch),
            "droplet-share" => Some(Self::DropletShare),
            "droplet-star" => Some(Self::DropletStar),
            "droplet-up" => Some(Self::DropletUp),
            "droplet-x" => Some(Self::DropletX),
            "ease-in" => Some(Self::EaseIn),
            "ease-in-control-point" => Some(Self::EaseInControlPoint),
            "ease-in-out" => Some(Self::EaseInOut),
            "ease-in-out-control-points" => Some(Self::EaseInOutControlPoints),
            "ease-out" => Some(Self::EaseOut),
            "ease-out-control-point" => Some(Self::EaseOutControlPoint),
            "edit" => Some(Self::Edit),
            "edit-circle" => Some(Self::EditCircle),
            "edit-circle-off" => Some(Self::EditCircleOff),
            "edit-off" => Some(Self::EditOff),
            "exposure" => Some(Self::Exposure),
            "exposure-0" => Some(Self::Exposure0),
            "exposure-minus-1" => Some(Self::ExposureMinus1),
            "exposure-minus-2" => Some(Self::ExposureMinus2),
            "exposure-off" => Some(Self::ExposureOff),
            "exposure-plus-1" => Some(Self::ExposurePlus1),
            "exposure-plus-2" => Some(Self::ExposurePlus2),
            "favicon" => Some(Self::Favicon),
            "flip-horizontal" => Some(Self::FlipHorizontal),
            "flip-vertical" => Some(Self::FlipVertical),
            "focus" => Some(Self::Focus),
            "focus-2" => Some(Self::Focus2),
            "focus-auto" => Some(Self::FocusAuto),
            "frame" => Some(Self::Frame),
            "frame-off" => Some(Self::FrameOff),
            "freeze-column" => Some(Self::FreezeColumn),
            "freeze-row" => Some(Self::FreezeRow),
            "freeze-row-column" => Some(Self::FreezeRowColumn),
            "frustum" => Some(Self::Frustum),
            "frustum-off" => Some(Self::FrustumOff),
            "frustum-plus" => Some(Self::FrustumPlus),
            "gender-agender" => Some(Self::GenderAgender),
            "gender-androgyne" => Some(Self::GenderAndrogyne),
            "gender-bigender" => Some(Self::GenderBigender),
            "gender-demiboy" => Some(Self::GenderDemiboy),
            "gender-demigirl" => Some(Self::GenderDemigirl),
            "gender-epicene" => Some(Self::GenderEpicene),
            "gender-female" => Some(Self::GenderFemale),
            "gender-femme" => Some(Self::GenderFemme),
            "gender-genderfluid" => Some(Self::GenderGenderfluid),
            "gender-genderless" => Some(Self::GenderGenderless),
            "gender-genderqueer" => Some(Self::GenderGenderqueer),
            "gender-hermaphrodite" => Some(Self::GenderHermaphrodite),
            "gender-intergender" => Some(Self::GenderIntergender),
            "gender-male" => Some(Self::GenderMale),
            "gender-neutrois" => Some(Self::GenderNeutrois),
            "gender-third" => Some(Self::GenderThird),
            "gender-transgender" => Some(Self::GenderTransgender),
            "gender-trasvesti" => Some(Self::GenderTrasvesti),
            "gizmo" => Some(Self::Gizmo),
            "gradienter" => Some(Self::Gradienter),
            "grain" => Some(Self::Grain),
            "grid-3x3" => Some(Self::Grid3x3),
            "grid-4x4" => Some(Self::Grid4x4),
            "grid-goldenratio" => Some(Self::GridGoldenratio),
            "guitar-pick" => Some(Self::GuitarPick),
            "hammer" => Some(Self::Hammer),
            "hammer-off" => Some(Self::HammerOff),
            "hand-click" => Some(Self::HandClick),
            "hand-click-off" => Some(Self::HandClickOff),
            "hand-finger" => Some(Self::HandFinger),
            "hand-finger-down" => Some(Self::HandFingerDown),
            "hand-finger-left" => Some(Self::HandFingerLeft),
            "hand-finger-off" => Some(Self::HandFingerOff),
            "hand-finger-right" => Some(Self::HandFingerRight),
            "hand-grab" => Some(Self::HandGrab),
            "hand-little-finger" => Some(Self::HandLittleFinger),
            "hand-love-you" => Some(Self::HandLoveYou),
            "hand-middle-finger" => Some(Self::HandMiddleFinger),
            "hand-move" => Some(Self::HandMove),
            "hand-off" => Some(Self::HandOff),
            "hand-ring-finger" => Some(Self::HandRingFinger),
            "hand-stop" => Some(Self::HandStop),
            "hand-three-fingers" => Some(Self::HandThreeFingers),
            "hand-two-fingers" => Some(Self::HandTwoFingers),
            "hash" => Some(Self::Hash),
            "hdr" => Some(Self::Hdr),
            "heart" => Some(Self::Heart),
            "heart-bolt" => Some(Self::HeartBolt),
            "heart-cancel" => Some(Self::HeartCancel),
            "heart-check" => Some(Self::HeartCheck),
            "heart-code" => Some(Self::HeartCode),
            "heart-cog" => Some(Self::HeartCog),
            "heart-dollar" => Some(Self::HeartDollar),
            "heart-down" => Some(Self::HeartDown),
            "heart-exclamation" => Some(Self::HeartExclamation),
            "heart-minus" => Some(Self::HeartMinus),
            "heart-off" => Some(Self::HeartOff),
            "heart-pause" => Some(Self::HeartPause),
            "heart-pin" => Some(Self::HeartPin),
            "heart-plus" => Some(Self::HeartPlus),
            "heart-question" => Some(Self::HeartQuestion),
            "heart-search" => Some(Self::HeartSearch),
            "heart-share" => Some(Self::HeartShare),
            "heart-spark" => Some(Self::HeartSpark),
            "heart-star" => Some(Self::HeartStar),
            "heart-up" => Some(Self::HeartUp),
            "heart-x" => Some(Self::HeartX),
            "hearts" => Some(Self::Hearts),
            "hearts-off" => Some(Self::HeartsOff),
            "hemisphere" => Some(Self::Hemisphere),
            "hemisphere-off" => Some(Self::HemisphereOff),
            "hemisphere-plus" => Some(Self::HemispherePlus),
            "hexagon" => Some(Self::Hexagon),
            "hexagon-3d" => Some(Self::Hexagon3d),
            "hexagon-asterisk" => Some(Self::HexagonAsterisk),
            "hexagon-minus" => Some(Self::HexagonMinus),
            "hexagon-minus-2" => Some(Self::HexagonMinus2),
            "hexagon-off" => Some(Self::HexagonOff),
            "hexagon-plus" => Some(Self::HexagonPlus),
            "hexagon-plus-2" => Some(Self::HexagonPlus2),
            "hexagonal-prism" => Some(Self::HexagonalPrism),
            "hexagonal-prism-off" => Some(Self::HexagonalPrismOff),
            "hexagonal-prism-plus" => Some(Self::HexagonalPrismPlus),
            "hexagonal-pyramid" => Some(Self::HexagonalPyramid),
            "hexagonal-pyramid-off" => Some(Self::HexagonalPyramidOff),
            "hexagonal-pyramid-plus" => Some(Self::HexagonalPyramidPlus),
            "hexagons" => Some(Self::Hexagons),
            "hexagons-off" => Some(Self::HexagonsOff),
            "hierarchy" => Some(Self::Hierarchy),
            "hierarchy-2" => Some(Self::Hierarchy2),
            "hierarchy-3" => Some(Self::Hierarchy3),
            "hierarchy-off" => Some(Self::HierarchyOff),
            "icons" => Some(Self::Icons),
            "icons-off" => Some(Self::IconsOff),
            "ikosaedr" => Some(Self::Ikosaedr),
            "image-in-picture" => Some(Self::ImageInPicture),
            "inner-shadow-bottom" => Some(Self::InnerShadowBottom),
            "inner-shadow-bottom-left" => Some(Self::InnerShadowBottomLeft),
            "inner-shadow-bottom-right" => Some(Self::InnerShadowBottomRight),
            "inner-shadow-left" => Some(Self::InnerShadowLeft),
            "inner-shadow-right" => Some(Self::InnerShadowRight),
            "inner-shadow-top" => Some(Self::InnerShadowTop),
            "inner-shadow-top-left" => Some(Self::InnerShadowTopLeft),
            "inner-shadow-top-right" => Some(Self::InnerShadowTopRight),
            "irregular-polyhedron" => Some(Self::IrregularPolyhedron),
            "irregular-polyhedron-off" => Some(Self::IrregularPolyhedronOff),
            "irregular-polyhedron-plus" => Some(Self::IrregularPolyhedronPlus),
            "jewish-star" => Some(Self::JewishStar),
            "key" => Some(Self::Key),
            "key-off" => Some(Self::KeyOff),
            "ladle" => Some(Self::Ladle),
            "lamp" => Some(Self::Lamp),
            "lamp-2" => Some(Self::Lamp2),
            "lamp-off" => Some(Self::LampOff),
            "lasso" => Some(Self::Lasso),
            "lasso-off" => Some(Self::LassoOff),
            "lasso-polygon" => Some(Self::LassoPolygon),
            "laurel-wreath" => Some(Self::LaurelWreath),
            "laurel-wreath-1" => Some(Self::LaurelWreath1),
            "laurel-wreath-2" => Some(Self::LaurelWreath2),
            "laurel-wreath-3" => Some(Self::LaurelWreath3),
            "layers-difference" => Some(Self::LayersDifference),
            "layers-intersect" => Some(Self::LayersIntersect),
            "layers-intersect-2" => Some(Self::LayersIntersect2),
            "layers-linked" => Some(Self::LayersLinked),
            "layers-off" => Some(Self::LayersOff),
            "layers-selected" => Some(Self::LayersSelected),
            "layers-selected-bottom" => Some(Self::LayersSelectedBottom),
            "layers-subtract" => Some(Self::LayersSubtract),
            "layers-union" => Some(Self::LayersUnion),
            "layout" => Some(Self::Layout),
            "layout-2" => Some(Self::Layout2),
            "layout-align-bottom" => Some(Self::LayoutAlignBottom),
            "layout-align-center" => Some(Self::LayoutAlignCenter),
            "layout-align-left" => Some(Self::LayoutAlignLeft),
            "layout-align-middle" => Some(Self::LayoutAlignMiddle),
            "layout-align-right" => Some(Self::LayoutAlignRight),
            "layout-align-top" => Some(Self::LayoutAlignTop),
            "layout-board" => Some(Self::LayoutBoard),
            "layout-board-split" => Some(Self::LayoutBoardSplit),
            "layout-bottombar" => Some(Self::LayoutBottombar),
            "layout-bottombar-collapse" => Some(Self::LayoutBottombarCollapse),
            "layout-bottombar-expand" => Some(Self::LayoutBottombarExpand),
            "layout-cards" => Some(Self::LayoutCards),
            "layout-collage" => Some(Self::LayoutCollage),
            "layout-columns" => Some(Self::LayoutColumns),
            "layout-dashboard" => Some(Self::LayoutDashboard),
            "layout-distribute-horizontal" => Some(Self::LayoutDistributeHorizontal),
            "layout-distribute-vertical" => Some(Self::LayoutDistributeVertical),
            "layout-grid" => Some(Self::LayoutGrid),
            "layout-grid-add" => Some(Self::LayoutGridAdd),
            "layout-grid-remove" => Some(Self::LayoutGridRemove),
            "layout-kanban" => Some(Self::LayoutKanban),
            "layout-list" => Some(Self::LayoutList),
            "layout-navbar" => Some(Self::LayoutNavbar),
            "layout-navbar-collapse" => Some(Self::LayoutNavbarCollapse),
            "layout-navbar-expand" => Some(Self::LayoutNavbarExpand),
            "layout-off" => Some(Self::LayoutOff),
            "layout-rows" => Some(Self::LayoutRows),
            "layout-sidebar" => Some(Self::LayoutSidebar),
            "layout-sidebar-left-collapse" => Some(Self::LayoutSidebarLeftCollapse),
            "layout-sidebar-left-expand" => Some(Self::LayoutSidebarLeftExpand),
            "layout-sidebar-right" => Some(Self::LayoutSidebarRight),
            "layout-sidebar-right-collapse" => Some(Self::LayoutSidebarRightCollapse),
            "layout-sidebar-right-expand" => Some(Self::LayoutSidebarRightExpand),
            "line" => Some(Self::Line),
            "line-dashed" => Some(Self::LineDashed),
            "line-dotted" => Some(Self::LineDotted),
            "live-photo" => Some(Self::LivePhoto),
            "live-photo-off" => Some(Self::LivePhotoOff),
            "macro" => Some(Self::Macro),
            "macro-off" => Some(Self::MacroOff),
            "magnet" => Some(Self::Magnet),
            "magnet-off" => Some(Self::MagnetOff),
            "magnetic" => Some(Self::Magnetic),
            "mask" => Some(Self::Mask),
            "mask-off" => Some(Self::MaskOff),
            "mesh" => Some(Self::Mesh),
            "mickey" => Some(Self::Mickey),
            "mood-angry" => Some(Self::MoodAngry),
            "mood-annoyed" => Some(Self::MoodAnnoyed),
            "mood-annoyed-2" => Some(Self::MoodAnnoyed2),
            "mood-bitcoin" => Some(Self::MoodBitcoin),
            "mood-boy" => Some(Self::MoodBoy),
            "mood-check" => Some(Self::MoodCheck),
            "mood-cog" => Some(Self::MoodCog),
            "mood-confuzed" => Some(Self::MoodConfuzed),
            "mood-crazy-happy" => Some(Self::MoodCrazyHappy),
            "mood-cry" => Some(Self::MoodCry),
            "mood-dollar" => Some(Self::MoodDollar),
            "mood-edit" => Some(Self::MoodEdit),
            "mood-empty" => Some(Self::MoodEmpty),
            "mood-happy" => Some(Self::MoodHappy),
            "mood-heart" => Some(Self::MoodHeart),
            "mood-kid" => Some(Self::MoodKid),
            "mood-look-down" => Some(Self::MoodLookDown),
            "mood-look-left" => Some(Self::MoodLookLeft),
            "mood-look-right" => Some(Self::MoodLookRight),
            "mood-look-up" => Some(Self::MoodLookUp),
            "mood-minus" => Some(Self::MoodMinus),
            "mood-nerd" => Some(Self::MoodNerd),
            "mood-nervous" => Some(Self::MoodNervous),
            "mood-neutral" => Some(Self::MoodNeutral),
            "mood-off" => Some(Self::MoodOff),
            "mood-pin" => Some(Self::MoodPin),
            "mood-plus" => Some(Self::MoodPlus),
            "mood-puzzled" => Some(Self::MoodPuzzled),
            "mood-sad" => Some(Self::MoodSad),
            "mood-sad-2" => Some(Self::MoodSad2),
            "mood-sad-dizzy" => Some(Self::MoodSadDizzy),
            "mood-sad-squint" => Some(Self::MoodSadSquint),
            "mood-search" => Some(Self::MoodSearch),
            "mood-share" => Some(Self::MoodShare),
            "mood-sick" => Some(Self::MoodSick),
            "mood-silence" => Some(Self::MoodSilence),
            "mood-sing" => Some(Self::MoodSing),
            "mood-smile" => Some(Self::MoodSmile),
            "mood-smile-beam" => Some(Self::MoodSmileBeam),
            "mood-smile-dizzy" => Some(Self::MoodSmileDizzy),
            "mood-spark" => Some(Self::MoodSpark),
            "mood-surprised" => Some(Self::MoodSurprised),
            "mood-tongue" => Some(Self::MoodTongue),
            "mood-tongue-wink" => Some(Self::MoodTongueWink),
            "mood-tongue-wink-2" => Some(Self::MoodTongueWink2),
            "mood-unamused" => Some(Self::MoodUnamused),
            "mood-up" => Some(Self::MoodUp),
            "mood-wink" => Some(Self::MoodWink),
            "mood-wink-2" => Some(Self::MoodWink2),
            "mood-wrrr" => Some(Self::MoodWrrr),
            "mood-x" => Some(Self::MoodX),
            "mood-xd" => Some(Self::MoodXd),
            "moustache" => Some(Self::Moustache),
            "needle" => Some(Self::Needle),
            "needle-thread" => Some(Self::NeedleThread),
            "noise-reduction" => Some(Self::NoiseReduction),
            "octagon" => Some(Self::Octagon),
            "octagon-minus" => Some(Self::OctagonMinus),
            "octagon-minus-2" => Some(Self::OctagonMinus2),
            "octagon-off" => Some(Self::OctagonOff),
            "octagon-plus" => Some(Self::OctagonPlus),
            "octagon-plus-2" => Some(Self::OctagonPlus2),
            "octahedron" => Some(Self::Octahedron),
            "octahedron-off" => Some(Self::OctahedronOff),
            "octahedron-plus" => Some(Self::OctahedronPlus),
            "oval" => Some(Self::Oval),
            "oval-vertical" => Some(Self::OvalVertical),
            "paint" => Some(Self::Paint),
            "paint-off" => Some(Self::PaintOff),
            "palette" => Some(Self::Palette),
            "palette-off" => Some(Self::PaletteOff),
            "panorama-horizontal" => Some(Self::PanoramaHorizontal),
            "panorama-horizontal-off" => Some(Self::PanoramaHorizontalOff),
            "panorama-vertical" => Some(Self::PanoramaVertical),
            "panorama-vertical-off" => Some(Self::PanoramaVerticalOff),
            "pencil" => Some(Self::Pencil),
            "pencil-bolt" => Some(Self::PencilBolt),
            "pencil-cancel" => Some(Self::PencilCancel),
            "pencil-check" => Some(Self::PencilCheck),
            "pencil-code" => Some(Self::PencilCode),
            "pencil-cog" => Some(Self::PencilCog),
            "pencil-discount" => Some(Self::PencilDiscount),
            "pencil-dollar" => Some(Self::PencilDollar),
            "pencil-down" => Some(Self::PencilDown),
            "pencil-exclamation" => Some(Self::PencilExclamation),
            "pencil-heart" => Some(Self::PencilHeart),
            "pencil-minus" => Some(Self::PencilMinus),
            "pencil-off" => Some(Self::PencilOff),
            "pencil-pause" => Some(Self::PencilPause),
            "pencil-pin" => Some(Self::PencilPin),
            "pencil-plus" => Some(Self::PencilPlus),
            "pencil-question" => Some(Self::PencilQuestion),
            "pencil-search" => Some(Self::PencilSearch),
            "pencil-share" => Some(Self::PencilShare),
            "pencil-star" => Some(Self::PencilStar),
            "pencil-up" => Some(Self::PencilUp),
            "pencil-x" => Some(Self::PencilX),
            "pentagon" => Some(Self::Pentagon),
            "pentagon-minus" => Some(Self::PentagonMinus),
            "pentagon-off" => Some(Self::PentagonOff),
            "pentagon-plus" => Some(Self::PentagonPlus),
            "pentagon-x" => Some(Self::PentagonX),
            "pentagram" => Some(Self::Pentagram),
            "perspective" => Some(Self::Perspective),
            "perspective-off" => Some(Self::PerspectiveOff),
            "photo-bitcoin" => Some(Self::PhotoBitcoin),
            "photo-circle" => Some(Self::PhotoCircle),
            "photo-hexagon" => Some(Self::PhotoHexagon),
            "photo-pentagon" => Some(Self::PhotoPentagon),
            "photo-sensor" => Some(Self::PhotoSensor),
            "photo-sensor-2" => Some(Self::PhotoSensor2),
            "photo-sensor-3" => Some(Self::PhotoSensor3),
            "photo-square-rounded" => Some(Self::PhotoSquareRounded),
            "photo-video" => Some(Self::PhotoVideo),
            "placeholder" => Some(Self::Placeholder),
            "polaroid" => Some(Self::Polaroid),
            "polygon" => Some(Self::Polygon),
            "polygon-off" => Some(Self::PolygonOff),
            "prism" => Some(Self::Prism),
            "prism-off" => Some(Self::PrismOff),
            "prism-plus" => Some(Self::PrismPlus),
            "prong" => Some(Self::Prong),
            "pyramid" => Some(Self::Pyramid),
            "pyramid-off" => Some(Self::PyramidOff),
            "pyramid-plus" => Some(Self::PyramidPlus),
            "radius-bottom-left" => Some(Self::RadiusBottomLeft),
            "radius-bottom-right" => Some(Self::RadiusBottomRight),
            "radius-top-left" => Some(Self::RadiusTopLeft),
            "radius-top-right" => Some(Self::RadiusTopRight),
            "rectangle" => Some(Self::Rectangle),
            "rectangle-rounded-bottom" => Some(Self::RectangleRoundedBottom),
            "rectangle-rounded-top" => Some(Self::RectangleRoundedTop),
            "rectangle-vertical" => Some(Self::RectangleVertical),
            "rectangular-prism" => Some(Self::RectangularPrism),
            "rectangular-prism-off" => Some(Self::RectangularPrismOff),
            "rectangular-prism-plus" => Some(Self::RectangularPrismPlus),
            "resize" => Some(Self::Resize),
            "rosette" => Some(Self::Rosette),
            "rosette-asterisk" => Some(Self::RosetteAsterisk),
            "ruler" => Some(Self::Ruler),
            "ruler-2" => Some(Self::Ruler2),
            "ruler-2-off" => Some(Self::Ruler2Off),
            "ruler-3" => Some(Self::Ruler3),
            "ruler-measure" => Some(Self::RulerMeasure),
            "ruler-measure-2" => Some(Self::RulerMeasure2),
            "ruler-off" => Some(Self::RulerOff),
            "scissors" => Some(Self::Scissors),
            "scissors-off" => Some(Self::ScissorsOff),
            "screenshot" => Some(Self::Screenshot),
            "section" => Some(Self::Section),
            "shadow" => Some(Self::Shadow),
            "shadow-off" => Some(Self::ShadowOff),
            "shape" => Some(Self::Shape),
            "shape-2" => Some(Self::Shape2),
            "shape-3" => Some(Self::Shape3),
            "shape-off" => Some(Self::ShapeOff),
            "shovel" => Some(Self::Shovel),
            "shovel-pitchforks" => Some(Self::ShovelPitchforks),
            "sketching" => Some(Self::Sketching),
            "slice" => Some(Self::Slice),
            "spade" => Some(Self::Spade),
            "sparkle" => Some(Self::Sparkle),
            "sparkle-2" => Some(Self::Sparkle2),
            "sparkle-highlight" => Some(Self::SparkleHighlight),
            "sparkles-2" => Some(Self::Sparkles2),
            "sphere" => Some(Self::Sphere),
            "sphere-2" => Some(Self::Sphere2),
            "sphere-off" => Some(Self::SphereOff),
            "sphere-plus" => Some(Self::SpherePlus),
            "square" => Some(Self::Square),
            "square-asterisk" => Some(Self::SquareAsterisk),
            "square-check" => Some(Self::SquareCheck),
            "square-dashed" => Some(Self::SquareDashed),
            "square-dot" => Some(Self::SquareDot),
            "square-forbid" => Some(Self::SquareForbid),
            "square-forbid-2" => Some(Self::SquareForbid2),
            "square-half" => Some(Self::SquareHalf),
            "square-key" => Some(Self::SquareKey),
            "square-minus" => Some(Self::SquareMinus),
            "square-minus-2" => Some(Self::SquareMinus2),
            "square-off" => Some(Self::SquareOff),
            "square-percentage" => Some(Self::SquarePercentage),
            "square-plus" => Some(Self::SquarePlus),
            "square-plus-2" => Some(Self::SquarePlus2),
            "square-rotated" => Some(Self::SquareRotated),
            "square-rotated-asterisk" => Some(Self::SquareRotatedAsterisk),
            "square-rotated-forbid" => Some(Self::SquareRotatedForbid),
            "square-rotated-forbid-2" => Some(Self::SquareRotatedForbid2),
            "square-rotated-off" => Some(Self::SquareRotatedOff),
            "square-rounded" => Some(Self::SquareRounded),
            "square-rounded-check" => Some(Self::SquareRoundedCheck),
            "square-rounded-minus" => Some(Self::SquareRoundedMinus),
            "square-rounded-minus-2" => Some(Self::SquareRoundedMinus2),
            "square-rounded-percentage" => Some(Self::SquareRoundedPercentage),
            "square-rounded-plus" => Some(Self::SquareRoundedPlus),
            "square-rounded-plus-2" => Some(Self::SquareRoundedPlus2),
            "square-rounded-x" => Some(Self::SquareRoundedX),
            "square-toggle" => Some(Self::SquareToggle),
            "square-toggle-horizontal" => Some(Self::SquareToggleHorizontal),
            "square-x" => Some(Self::SquareX),
            "squares" => Some(Self::Squares),
            "squares-diagonal" => Some(Self::SquaresDiagonal),
            "squares-selected" => Some(Self::SquaresSelected),
            "stack" => Some(Self::Stack),
            "stack-2" => Some(Self::Stack2),
            "stack-3" => Some(Self::Stack3),
            "stack-pop" => Some(Self::StackPop),
            "stack-push" => Some(Self::StackPush),
            "sticker" => Some(Self::Sticker),
            "sticker-2" => Some(Self::Sticker2),
            "stroke-curved" => Some(Self::StrokeCurved),
            "stroke-dynamic" => Some(Self::StrokeDynamic),
            "stroke-straight" => Some(Self::StrokeStraight),
            "template" => Some(Self::Template),
            "template-off" => Some(Self::TemplateOff),
            "text-resize" => Some(Self::TextResize),
            "tilt-shift" => Some(Self::TiltShift),
            "tilt-shift-off" => Some(Self::TiltShiftOff),
            "tools" => Some(Self::Tools),
            "tools-off" => Some(Self::ToolsOff),
            "triangle" => Some(Self::Triangle),
            "triangle-inverted" => Some(Self::TriangleInverted),
            "triangle-minus" => Some(Self::TriangleMinus),
            "triangle-minus-2" => Some(Self::TriangleMinus2),
            "triangle-off" => Some(Self::TriangleOff),
            "triangle-plus" => Some(Self::TrianglePlus),
            "triangle-plus-2" => Some(Self::TrianglePlus2),
            "triangle-square-circle" => Some(Self::TriangleSquareCircle),
            "triangles" => Some(Self::Triangles),
            "trident" => Some(Self::Trident),
            "typeface" => Some(Self::Typeface),
            "ux-circle" => Some(Self::UxCircle),
            "vector" => Some(Self::Vector),
            "vector-bezier" => Some(Self::VectorBezier),
            "vector-bezier-2" => Some(Self::VectorBezier2),
            "vector-bezier-arc" => Some(Self::VectorBezierArc),
            "vector-bezier-circle" => Some(Self::VectorBezierCircle),
            "vector-off" => Some(Self::VectorOff),
            "vector-spline" => Some(Self::VectorSpline),
            "vector-triangle" => Some(Self::VectorTriangle),
            "vector-triangle-off" => Some(Self::VectorTriangleOff),
            "vignette" => Some(Self::Vignette),
            "wheel" => Some(Self::Wheel),
            _ => None,
        }
    }
}

impl TablerIconData for DesignIcon {
    fn name(&self) -> &'static str {
        match self {
            Self::Ad => "ad",
            Self::Ad2 => "ad-2",
            Self::AdOff => "ad-off",
            Self::Angle => "angle",
            Self::Aperture => "aperture",
            Self::ApertureOff => "aperture-off",
            Self::Artboard => "artboard",
            Self::ArtboardOff => "artboard-off",
            Self::Award => "award",
            Self::AwardOff => "award-off",
            Self::Background => "background",
            Self::Badge => "badge",
            Self::Badge2k => "badge-2k",
            Self::Badge3d => "badge-3d",
            Self::Badge3k => "badge-3k",
            Self::Badge4k => "badge-4k",
            Self::Badge5k => "badge-5k",
            Self::Badge8k => "badge-8k",
            Self::BadgeAd => "badge-ad",
            Self::BadgeAdOff => "badge-ad-off",
            Self::BadgeAr => "badge-ar",
            Self::BadgeCc => "badge-cc",
            Self::BadgeHd => "badge-hd",
            Self::BadgeOff => "badge-off",
            Self::BadgeSd => "badge-sd",
            Self::BadgeTm => "badge-tm",
            Self::BadgeVo => "badge-vo",
            Self::BadgeVr => "badge-vr",
            Self::BadgeWc => "badge-wc",
            Self::Badges => "badges",
            Self::BadgesOff => "badges-off",
            Self::Barrel => "barrel",
            Self::BarrelOff => "barrel-off",
            Self::BarrierBlock => "barrier-block",
            Self::BarrierBlockOff => "barrier-block-off",
            Self::Binoculars => "binoculars",
            Self::Blade => "blade",
            Self::BlendMode => "blend-mode",
            Self::Blur => "blur",
            Self::BlurOff => "blur-off",
            Self::Bolt => "bolt",
            Self::BoltOff => "bolt-off",
            Self::Bomb => "bomb",
            Self::Bong => "bong",
            Self::BongOff => "bong-off",
            Self::Boom => "boom",
            Self::BorderAll => "border-all",
            Self::BorderBottom => "border-bottom",
            Self::BorderBottomPlus => "border-bottom-plus",
            Self::BorderCornerIos => "border-corner-ios",
            Self::BorderCornerPill => "border-corner-pill",
            Self::BorderCornerRounded => "border-corner-rounded",
            Self::BorderCornerSquare => "border-corner-square",
            Self::BorderCorners => "border-corners",
            Self::BorderHorizontal => "border-horizontal",
            Self::BorderInner => "border-inner",
            Self::BorderLeft => "border-left",
            Self::BorderLeftPlus => "border-left-plus",
            Self::BorderNone => "border-none",
            Self::BorderOuter => "border-outer",
            Self::BorderRadius => "border-radius",
            Self::BorderRight => "border-right",
            Self::BorderRightPlus => "border-right-plus",
            Self::BorderSides => "border-sides",
            Self::BorderStyle => "border-style",
            Self::BorderStyle2 => "border-style-2",
            Self::BorderTop => "border-top",
            Self::BorderTopPlus => "border-top-plus",
            Self::BorderVertical => "border-vertical",
            Self::BounceLeft => "bounce-left",
            Self::BounceRight => "bounce-right",
            Self::Box => "box",
            Self::BoxAlignBottom => "box-align-bottom",
            Self::BoxAlignBottomLeft => "box-align-bottom-left",
            Self::BoxAlignBottomRight => "box-align-bottom-right",
            Self::BoxAlignLeft => "box-align-left",
            Self::BoxAlignRight => "box-align-right",
            Self::BoxAlignTop => "box-align-top",
            Self::BoxAlignTopLeft => "box-align-top-left",
            Self::BoxAlignTopRight => "box-align-top-right",
            Self::BoxMargin => "box-margin",
            Self::BoxModel => "box-model",
            Self::BoxModel2 => "box-model-2",
            Self::BoxModel2Off => "box-model-2-off",
            Self::BoxModelOff => "box-model-off",
            Self::BoxMultiple => "box-multiple",
            Self::BoxOff => "box-off",
            Self::BoxPadding => "box-padding",
            Self::BracketsAngle => "brackets-angle",
            Self::BracketsAngleOff => "brackets-angle-off",
            Self::Briefcase => "briefcase",
            Self::Briefcase2 => "briefcase-2",
            Self::BriefcaseOff => "briefcase-off",
            Self::Brightness => "brightness",
            Self::Brightness2 => "brightness-2",
            Self::BrightnessAuto => "brightness-auto",
            Self::BrightnessDown => "brightness-down",
            Self::BrightnessHalf => "brightness-half",
            Self::BrightnessOff => "brightness-off",
            Self::BrightnessUp => "brightness-up",
            Self::Brush => "brush",
            Self::BrushOff => "brush-off",
            Self::Bucket => "bucket",
            Self::BucketDroplet => "bucket-droplet",
            Self::BucketOff => "bucket-off",
            Self::Bulb => "bulb",
            Self::BulbOff => "bulb-off",
            Self::CameraRotate => "camera-rotate",
            Self::CameraSelfie => "camera-selfie",
            Self::Cancel => "cancel",
            Self::Candle => "candle",
            Self::CapProjecting => "cap-projecting",
            Self::CapRounded => "cap-rounded",
            Self::CapStraight => "cap-straight",
            Self::Capsule => "capsule",
            Self::CapsuleHorizontal => "capsule-horizontal",
            Self::Cards => "cards",
            Self::CaretLeftRight => "caret-left-right",
            Self::CarouselHorizontal => "carousel-horizontal",
            Self::CarouselVertical => "carousel-vertical",
            Self::Category => "category",
            Self::Category2 => "category-2",
            Self::CategoryMinus => "category-minus",
            Self::CategoryPlus => "category-plus",
            Self::Cell => "cell",
            Self::ChairDirector => "chair-director",
            Self::Chisel => "chisel",
            Self::ChristmasBall => "christmas-ball",
            Self::Circle => "circle",
            Self::CircleAsterisk => "circle-asterisk",
            Self::CircleCheck => "circle-check",
            Self::CircleDashed => "circle-dashed",
            Self::CircleDashedCheck => "circle-dashed-check",
            Self::CircleDashedMinus => "circle-dashed-minus",
            Self::CircleDashedPercentage => "circle-dashed-percentage",
            Self::CircleDashedPlus => "circle-dashed-plus",
            Self::CircleDashedX => "circle-dashed-x",
            Self::CircleDot => "circle-dot",
            Self::CircleDotted => "circle-dotted",
            Self::CircleHalf => "circle-half",
            Self::CircleHalf2 => "circle-half-2",
            Self::CircleHalfVertical => "circle-half-vertical",
            Self::CircleKey => "circle-key",
            Self::CircleMinus => "circle-minus",
            Self::CircleMinus2 => "circle-minus-2",
            Self::CircleOff => "circle-off",
            Self::CirclePercentage => "circle-percentage",
            Self::CirclePlus => "circle-plus",
            Self::CirclePlus2 => "circle-plus-2",
            Self::CirclePlusMinus => "circle-plus-minus",
            Self::CircleRectangle => "circle-rectangle",
            Self::CircleRectangleOff => "circle-rectangle-off",
            Self::CircleSquare => "circle-square",
            Self::CircleTriangle => "circle-triangle",
            Self::CircleX => "circle-x",
            Self::Circles => "circles",
            Self::CirclesRelation => "circles-relation",
            Self::Clubs => "clubs",
            Self::Coffin => "coffin",
            Self::ColorFilter => "color-filter",
            Self::ColorPicker => "color-picker",
            Self::ColorPickerOff => "color-picker-off",
            Self::ColorSwatch => "color-swatch",
            Self::ColorSwatchOff => "color-swatch-off",
            Self::Components => "components",
            Self::ComponentsOff => "components-off",
            Self::Cone => "cone",
            Self::Cone2 => "cone-2",
            Self::ConeOff => "cone-off",
            Self::ConePlus => "cone-plus",
            Self::Confetti => "confetti",
            Self::ConfettiOff => "confetti-off",
            Self::Container => "container",
            Self::ContainerOff => "container-off",
            Self::Contrast => "contrast",
            Self::Contrast2 => "contrast-2",
            Self::Contrast2Off => "contrast-2-off",
            Self::ContrastOff => "contrast-off",
            Self::CookieMan => "cookie-man",
            Self::Crop => "crop",
            Self::Crop11 => "crop-1-1",
            Self::Crop169 => "crop-16-9",
            Self::Crop32 => "crop-3-2",
            Self::Crop54 => "crop-5-4",
            Self::Crop75 => "crop-7-5",
            Self::CropLandscape => "crop-landscape",
            Self::CropPortrait => "crop-portrait",
            Self::Crown => "crown",
            Self::CrownOff => "crown-off",
            Self::Cube => "cube",
            Self::CubeOff => "cube-off",
            Self::CubePlus => "cube-plus",
            Self::CubeSpark => "cube-spark",
            Self::Cut => "cut",
            Self::Cylinder => "cylinder",
            Self::CylinderOff => "cylinder-off",
            Self::CylinderPlus => "cylinder-plus",
            Self::Diabolo => "diabolo",
            Self::DiaboloOff => "diabolo-off",
            Self::DiaboloPlus => "diabolo-plus",
            Self::Diamond => "diamond",
            Self::DiamondOff => "diamond-off",
            Self::Diamonds => "diamonds",
            Self::Dimensions => "dimensions",
            Self::DragDrop => "drag-drop",
            Self::DragDrop2 => "drag-drop-2",
            Self::Droplet => "droplet",
            Self::DropletBolt => "droplet-bolt",
            Self::DropletCancel => "droplet-cancel",
            Self::DropletCheck => "droplet-check",
            Self::DropletCode => "droplet-code",
            Self::DropletCog => "droplet-cog",
            Self::DropletDollar => "droplet-dollar",
            Self::DropletDown => "droplet-down",
            Self::DropletExclamation => "droplet-exclamation",
            Self::DropletHalf => "droplet-half",
            Self::DropletHalf2 => "droplet-half-2",
            Self::DropletHeart => "droplet-heart",
            Self::DropletMinus => "droplet-minus",
            Self::DropletOff => "droplet-off",
            Self::DropletPause => "droplet-pause",
            Self::DropletPin => "droplet-pin",
            Self::DropletPlus => "droplet-plus",
            Self::DropletQuestion => "droplet-question",
            Self::DropletSearch => "droplet-search",
            Self::DropletShare => "droplet-share",
            Self::DropletStar => "droplet-star",
            Self::DropletUp => "droplet-up",
            Self::DropletX => "droplet-x",
            Self::EaseIn => "ease-in",
            Self::EaseInControlPoint => "ease-in-control-point",
            Self::EaseInOut => "ease-in-out",
            Self::EaseInOutControlPoints => "ease-in-out-control-points",
            Self::EaseOut => "ease-out",
            Self::EaseOutControlPoint => "ease-out-control-point",
            Self::Edit => "edit",
            Self::EditCircle => "edit-circle",
            Self::EditCircleOff => "edit-circle-off",
            Self::EditOff => "edit-off",
            Self::Exposure => "exposure",
            Self::Exposure0 => "exposure-0",
            Self::ExposureMinus1 => "exposure-minus-1",
            Self::ExposureMinus2 => "exposure-minus-2",
            Self::ExposureOff => "exposure-off",
            Self::ExposurePlus1 => "exposure-plus-1",
            Self::ExposurePlus2 => "exposure-plus-2",
            Self::Favicon => "favicon",
            Self::FlipHorizontal => "flip-horizontal",
            Self::FlipVertical => "flip-vertical",
            Self::Focus => "focus",
            Self::Focus2 => "focus-2",
            Self::FocusAuto => "focus-auto",
            Self::Frame => "frame",
            Self::FrameOff => "frame-off",
            Self::FreezeColumn => "freeze-column",
            Self::FreezeRow => "freeze-row",
            Self::FreezeRowColumn => "freeze-row-column",
            Self::Frustum => "frustum",
            Self::FrustumOff => "frustum-off",
            Self::FrustumPlus => "frustum-plus",
            Self::GenderAgender => "gender-agender",
            Self::GenderAndrogyne => "gender-androgyne",
            Self::GenderBigender => "gender-bigender",
            Self::GenderDemiboy => "gender-demiboy",
            Self::GenderDemigirl => "gender-demigirl",
            Self::GenderEpicene => "gender-epicene",
            Self::GenderFemale => "gender-female",
            Self::GenderFemme => "gender-femme",
            Self::GenderGenderfluid => "gender-genderfluid",
            Self::GenderGenderless => "gender-genderless",
            Self::GenderGenderqueer => "gender-genderqueer",
            Self::GenderHermaphrodite => "gender-hermaphrodite",
            Self::GenderIntergender => "gender-intergender",
            Self::GenderMale => "gender-male",
            Self::GenderNeutrois => "gender-neutrois",
            Self::GenderThird => "gender-third",
            Self::GenderTransgender => "gender-transgender",
            Self::GenderTrasvesti => "gender-trasvesti",
            Self::Gizmo => "gizmo",
            Self::Gradienter => "gradienter",
            Self::Grain => "grain",
            Self::Grid3x3 => "grid-3x3",
            Self::Grid4x4 => "grid-4x4",
            Self::GridGoldenratio => "grid-goldenratio",
            Self::GuitarPick => "guitar-pick",
            Self::Hammer => "hammer",
            Self::HammerOff => "hammer-off",
            Self::HandClick => "hand-click",
            Self::HandClickOff => "hand-click-off",
            Self::HandFinger => "hand-finger",
            Self::HandFingerDown => "hand-finger-down",
            Self::HandFingerLeft => "hand-finger-left",
            Self::HandFingerOff => "hand-finger-off",
            Self::HandFingerRight => "hand-finger-right",
            Self::HandGrab => "hand-grab",
            Self::HandLittleFinger => "hand-little-finger",
            Self::HandLoveYou => "hand-love-you",
            Self::HandMiddleFinger => "hand-middle-finger",
            Self::HandMove => "hand-move",
            Self::HandOff => "hand-off",
            Self::HandRingFinger => "hand-ring-finger",
            Self::HandStop => "hand-stop",
            Self::HandThreeFingers => "hand-three-fingers",
            Self::HandTwoFingers => "hand-two-fingers",
            Self::Hash => "hash",
            Self::Hdr => "hdr",
            Self::Heart => "heart",
            Self::HeartBolt => "heart-bolt",
            Self::HeartCancel => "heart-cancel",
            Self::HeartCheck => "heart-check",
            Self::HeartCode => "heart-code",
            Self::HeartCog => "heart-cog",
            Self::HeartDollar => "heart-dollar",
            Self::HeartDown => "heart-down",
            Self::HeartExclamation => "heart-exclamation",
            Self::HeartMinus => "heart-minus",
            Self::HeartOff => "heart-off",
            Self::HeartPause => "heart-pause",
            Self::HeartPin => "heart-pin",
            Self::HeartPlus => "heart-plus",
            Self::HeartQuestion => "heart-question",
            Self::HeartSearch => "heart-search",
            Self::HeartShare => "heart-share",
            Self::HeartSpark => "heart-spark",
            Self::HeartStar => "heart-star",
            Self::HeartUp => "heart-up",
            Self::HeartX => "heart-x",
            Self::Hearts => "hearts",
            Self::HeartsOff => "hearts-off",
            Self::Hemisphere => "hemisphere",
            Self::HemisphereOff => "hemisphere-off",
            Self::HemispherePlus => "hemisphere-plus",
            Self::Hexagon => "hexagon",
            Self::Hexagon3d => "hexagon-3d",
            Self::HexagonAsterisk => "hexagon-asterisk",
            Self::HexagonMinus => "hexagon-minus",
            Self::HexagonMinus2 => "hexagon-minus-2",
            Self::HexagonOff => "hexagon-off",
            Self::HexagonPlus => "hexagon-plus",
            Self::HexagonPlus2 => "hexagon-plus-2",
            Self::HexagonalPrism => "hexagonal-prism",
            Self::HexagonalPrismOff => "hexagonal-prism-off",
            Self::HexagonalPrismPlus => "hexagonal-prism-plus",
            Self::HexagonalPyramid => "hexagonal-pyramid",
            Self::HexagonalPyramidOff => "hexagonal-pyramid-off",
            Self::HexagonalPyramidPlus => "hexagonal-pyramid-plus",
            Self::Hexagons => "hexagons",
            Self::HexagonsOff => "hexagons-off",
            Self::Hierarchy => "hierarchy",
            Self::Hierarchy2 => "hierarchy-2",
            Self::Hierarchy3 => "hierarchy-3",
            Self::HierarchyOff => "hierarchy-off",
            Self::Icons => "icons",
            Self::IconsOff => "icons-off",
            Self::Ikosaedr => "ikosaedr",
            Self::ImageInPicture => "image-in-picture",
            Self::InnerShadowBottom => "inner-shadow-bottom",
            Self::InnerShadowBottomLeft => "inner-shadow-bottom-left",
            Self::InnerShadowBottomRight => "inner-shadow-bottom-right",
            Self::InnerShadowLeft => "inner-shadow-left",
            Self::InnerShadowRight => "inner-shadow-right",
            Self::InnerShadowTop => "inner-shadow-top",
            Self::InnerShadowTopLeft => "inner-shadow-top-left",
            Self::InnerShadowTopRight => "inner-shadow-top-right",
            Self::IrregularPolyhedron => "irregular-polyhedron",
            Self::IrregularPolyhedronOff => "irregular-polyhedron-off",
            Self::IrregularPolyhedronPlus => "irregular-polyhedron-plus",
            Self::JewishStar => "jewish-star",
            Self::Key => "key",
            Self::KeyOff => "key-off",
            Self::Ladle => "ladle",
            Self::Lamp => "lamp",
            Self::Lamp2 => "lamp-2",
            Self::LampOff => "lamp-off",
            Self::Lasso => "lasso",
            Self::LassoOff => "lasso-off",
            Self::LassoPolygon => "lasso-polygon",
            Self::LaurelWreath => "laurel-wreath",
            Self::LaurelWreath1 => "laurel-wreath-1",
            Self::LaurelWreath2 => "laurel-wreath-2",
            Self::LaurelWreath3 => "laurel-wreath-3",
            Self::LayersDifference => "layers-difference",
            Self::LayersIntersect => "layers-intersect",
            Self::LayersIntersect2 => "layers-intersect-2",
            Self::LayersLinked => "layers-linked",
            Self::LayersOff => "layers-off",
            Self::LayersSelected => "layers-selected",
            Self::LayersSelectedBottom => "layers-selected-bottom",
            Self::LayersSubtract => "layers-subtract",
            Self::LayersUnion => "layers-union",
            Self::Layout => "layout",
            Self::Layout2 => "layout-2",
            Self::LayoutAlignBottom => "layout-align-bottom",
            Self::LayoutAlignCenter => "layout-align-center",
            Self::LayoutAlignLeft => "layout-align-left",
            Self::LayoutAlignMiddle => "layout-align-middle",
            Self::LayoutAlignRight => "layout-align-right",
            Self::LayoutAlignTop => "layout-align-top",
            Self::LayoutBoard => "layout-board",
            Self::LayoutBoardSplit => "layout-board-split",
            Self::LayoutBottombar => "layout-bottombar",
            Self::LayoutBottombarCollapse => "layout-bottombar-collapse",
            Self::LayoutBottombarExpand => "layout-bottombar-expand",
            Self::LayoutCards => "layout-cards",
            Self::LayoutCollage => "layout-collage",
            Self::LayoutColumns => "layout-columns",
            Self::LayoutDashboard => "layout-dashboard",
            Self::LayoutDistributeHorizontal => "layout-distribute-horizontal",
            Self::LayoutDistributeVertical => "layout-distribute-vertical",
            Self::LayoutGrid => "layout-grid",
            Self::LayoutGridAdd => "layout-grid-add",
            Self::LayoutGridRemove => "layout-grid-remove",
            Self::LayoutKanban => "layout-kanban",
            Self::LayoutList => "layout-list",
            Self::LayoutNavbar => "layout-navbar",
            Self::LayoutNavbarCollapse => "layout-navbar-collapse",
            Self::LayoutNavbarExpand => "layout-navbar-expand",
            Self::LayoutOff => "layout-off",
            Self::LayoutRows => "layout-rows",
            Self::LayoutSidebar => "layout-sidebar",
            Self::LayoutSidebarLeftCollapse => "layout-sidebar-left-collapse",
            Self::LayoutSidebarLeftExpand => "layout-sidebar-left-expand",
            Self::LayoutSidebarRight => "layout-sidebar-right",
            Self::LayoutSidebarRightCollapse => "layout-sidebar-right-collapse",
            Self::LayoutSidebarRightExpand => "layout-sidebar-right-expand",
            Self::Line => "line",
            Self::LineDashed => "line-dashed",
            Self::LineDotted => "line-dotted",
            Self::LivePhoto => "live-photo",
            Self::LivePhotoOff => "live-photo-off",
            Self::Macro => "macro",
            Self::MacroOff => "macro-off",
            Self::Magnet => "magnet",
            Self::MagnetOff => "magnet-off",
            Self::Magnetic => "magnetic",
            Self::Mask => "mask",
            Self::MaskOff => "mask-off",
            Self::Mesh => "mesh",
            Self::Mickey => "mickey",
            Self::MoodAngry => "mood-angry",
            Self::MoodAnnoyed => "mood-annoyed",
            Self::MoodAnnoyed2 => "mood-annoyed-2",
            Self::MoodBitcoin => "mood-bitcoin",
            Self::MoodBoy => "mood-boy",
            Self::MoodCheck => "mood-check",
            Self::MoodCog => "mood-cog",
            Self::MoodConfuzed => "mood-confuzed",
            Self::MoodCrazyHappy => "mood-crazy-happy",
            Self::MoodCry => "mood-cry",
            Self::MoodDollar => "mood-dollar",
            Self::MoodEdit => "mood-edit",
            Self::MoodEmpty => "mood-empty",
            Self::MoodHappy => "mood-happy",
            Self::MoodHeart => "mood-heart",
            Self::MoodKid => "mood-kid",
            Self::MoodLookDown => "mood-look-down",
            Self::MoodLookLeft => "mood-look-left",
            Self::MoodLookRight => "mood-look-right",
            Self::MoodLookUp => "mood-look-up",
            Self::MoodMinus => "mood-minus",
            Self::MoodNerd => "mood-nerd",
            Self::MoodNervous => "mood-nervous",
            Self::MoodNeutral => "mood-neutral",
            Self::MoodOff => "mood-off",
            Self::MoodPin => "mood-pin",
            Self::MoodPlus => "mood-plus",
            Self::MoodPuzzled => "mood-puzzled",
            Self::MoodSad => "mood-sad",
            Self::MoodSad2 => "mood-sad-2",
            Self::MoodSadDizzy => "mood-sad-dizzy",
            Self::MoodSadSquint => "mood-sad-squint",
            Self::MoodSearch => "mood-search",
            Self::MoodShare => "mood-share",
            Self::MoodSick => "mood-sick",
            Self::MoodSilence => "mood-silence",
            Self::MoodSing => "mood-sing",
            Self::MoodSmile => "mood-smile",
            Self::MoodSmileBeam => "mood-smile-beam",
            Self::MoodSmileDizzy => "mood-smile-dizzy",
            Self::MoodSpark => "mood-spark",
            Self::MoodSurprised => "mood-surprised",
            Self::MoodTongue => "mood-tongue",
            Self::MoodTongueWink => "mood-tongue-wink",
            Self::MoodTongueWink2 => "mood-tongue-wink-2",
            Self::MoodUnamused => "mood-unamused",
            Self::MoodUp => "mood-up",
            Self::MoodWink => "mood-wink",
            Self::MoodWink2 => "mood-wink-2",
            Self::MoodWrrr => "mood-wrrr",
            Self::MoodX => "mood-x",
            Self::MoodXd => "mood-xd",
            Self::Moustache => "moustache",
            Self::Needle => "needle",
            Self::NeedleThread => "needle-thread",
            Self::NoiseReduction => "noise-reduction",
            Self::Octagon => "octagon",
            Self::OctagonMinus => "octagon-minus",
            Self::OctagonMinus2 => "octagon-minus-2",
            Self::OctagonOff => "octagon-off",
            Self::OctagonPlus => "octagon-plus",
            Self::OctagonPlus2 => "octagon-plus-2",
            Self::Octahedron => "octahedron",
            Self::OctahedronOff => "octahedron-off",
            Self::OctahedronPlus => "octahedron-plus",
            Self::Oval => "oval",
            Self::OvalVertical => "oval-vertical",
            Self::Paint => "paint",
            Self::PaintOff => "paint-off",
            Self::Palette => "palette",
            Self::PaletteOff => "palette-off",
            Self::PanoramaHorizontal => "panorama-horizontal",
            Self::PanoramaHorizontalOff => "panorama-horizontal-off",
            Self::PanoramaVertical => "panorama-vertical",
            Self::PanoramaVerticalOff => "panorama-vertical-off",
            Self::Pencil => "pencil",
            Self::PencilBolt => "pencil-bolt",
            Self::PencilCancel => "pencil-cancel",
            Self::PencilCheck => "pencil-check",
            Self::PencilCode => "pencil-code",
            Self::PencilCog => "pencil-cog",
            Self::PencilDiscount => "pencil-discount",
            Self::PencilDollar => "pencil-dollar",
            Self::PencilDown => "pencil-down",
            Self::PencilExclamation => "pencil-exclamation",
            Self::PencilHeart => "pencil-heart",
            Self::PencilMinus => "pencil-minus",
            Self::PencilOff => "pencil-off",
            Self::PencilPause => "pencil-pause",
            Self::PencilPin => "pencil-pin",
            Self::PencilPlus => "pencil-plus",
            Self::PencilQuestion => "pencil-question",
            Self::PencilSearch => "pencil-search",
            Self::PencilShare => "pencil-share",
            Self::PencilStar => "pencil-star",
            Self::PencilUp => "pencil-up",
            Self::PencilX => "pencil-x",
            Self::Pentagon => "pentagon",
            Self::PentagonMinus => "pentagon-minus",
            Self::PentagonOff => "pentagon-off",
            Self::PentagonPlus => "pentagon-plus",
            Self::PentagonX => "pentagon-x",
            Self::Pentagram => "pentagram",
            Self::Perspective => "perspective",
            Self::PerspectiveOff => "perspective-off",
            Self::PhotoBitcoin => "photo-bitcoin",
            Self::PhotoCircle => "photo-circle",
            Self::PhotoHexagon => "photo-hexagon",
            Self::PhotoPentagon => "photo-pentagon",
            Self::PhotoSensor => "photo-sensor",
            Self::PhotoSensor2 => "photo-sensor-2",
            Self::PhotoSensor3 => "photo-sensor-3",
            Self::PhotoSquareRounded => "photo-square-rounded",
            Self::PhotoVideo => "photo-video",
            Self::Placeholder => "placeholder",
            Self::Polaroid => "polaroid",
            Self::Polygon => "polygon",
            Self::PolygonOff => "polygon-off",
            Self::Prism => "prism",
            Self::PrismOff => "prism-off",
            Self::PrismPlus => "prism-plus",
            Self::Prong => "prong",
            Self::Pyramid => "pyramid",
            Self::PyramidOff => "pyramid-off",
            Self::PyramidPlus => "pyramid-plus",
            Self::RadiusBottomLeft => "radius-bottom-left",
            Self::RadiusBottomRight => "radius-bottom-right",
            Self::RadiusTopLeft => "radius-top-left",
            Self::RadiusTopRight => "radius-top-right",
            Self::Rectangle => "rectangle",
            Self::RectangleRoundedBottom => "rectangle-rounded-bottom",
            Self::RectangleRoundedTop => "rectangle-rounded-top",
            Self::RectangleVertical => "rectangle-vertical",
            Self::RectangularPrism => "rectangular-prism",
            Self::RectangularPrismOff => "rectangular-prism-off",
            Self::RectangularPrismPlus => "rectangular-prism-plus",
            Self::Resize => "resize",
            Self::Rosette => "rosette",
            Self::RosetteAsterisk => "rosette-asterisk",
            Self::Ruler => "ruler",
            Self::Ruler2 => "ruler-2",
            Self::Ruler2Off => "ruler-2-off",
            Self::Ruler3 => "ruler-3",
            Self::RulerMeasure => "ruler-measure",
            Self::RulerMeasure2 => "ruler-measure-2",
            Self::RulerOff => "ruler-off",
            Self::Scissors => "scissors",
            Self::ScissorsOff => "scissors-off",
            Self::Screenshot => "screenshot",
            Self::Section => "section",
            Self::Shadow => "shadow",
            Self::ShadowOff => "shadow-off",
            Self::Shape => "shape",
            Self::Shape2 => "shape-2",
            Self::Shape3 => "shape-3",
            Self::ShapeOff => "shape-off",
            Self::Shovel => "shovel",
            Self::ShovelPitchforks => "shovel-pitchforks",
            Self::Sketching => "sketching",
            Self::Slice => "slice",
            Self::Spade => "spade",
            Self::Sparkle => "sparkle",
            Self::Sparkle2 => "sparkle-2",
            Self::SparkleHighlight => "sparkle-highlight",
            Self::Sparkles2 => "sparkles-2",
            Self::Sphere => "sphere",
            Self::Sphere2 => "sphere-2",
            Self::SphereOff => "sphere-off",
            Self::SpherePlus => "sphere-plus",
            Self::Square => "square",
            Self::SquareAsterisk => "square-asterisk",
            Self::SquareCheck => "square-check",
            Self::SquareDashed => "square-dashed",
            Self::SquareDot => "square-dot",
            Self::SquareForbid => "square-forbid",
            Self::SquareForbid2 => "square-forbid-2",
            Self::SquareHalf => "square-half",
            Self::SquareKey => "square-key",
            Self::SquareMinus => "square-minus",
            Self::SquareMinus2 => "square-minus-2",
            Self::SquareOff => "square-off",
            Self::SquarePercentage => "square-percentage",
            Self::SquarePlus => "square-plus",
            Self::SquarePlus2 => "square-plus-2",
            Self::SquareRotated => "square-rotated",
            Self::SquareRotatedAsterisk => "square-rotated-asterisk",
            Self::SquareRotatedForbid => "square-rotated-forbid",
            Self::SquareRotatedForbid2 => "square-rotated-forbid-2",
            Self::SquareRotatedOff => "square-rotated-off",
            Self::SquareRounded => "square-rounded",
            Self::SquareRoundedCheck => "square-rounded-check",
            Self::SquareRoundedMinus => "square-rounded-minus",
            Self::SquareRoundedMinus2 => "square-rounded-minus-2",
            Self::SquareRoundedPercentage => "square-rounded-percentage",
            Self::SquareRoundedPlus => "square-rounded-plus",
            Self::SquareRoundedPlus2 => "square-rounded-plus-2",
            Self::SquareRoundedX => "square-rounded-x",
            Self::SquareToggle => "square-toggle",
            Self::SquareToggleHorizontal => "square-toggle-horizontal",
            Self::SquareX => "square-x",
            Self::Squares => "squares",
            Self::SquaresDiagonal => "squares-diagonal",
            Self::SquaresSelected => "squares-selected",
            Self::Stack => "stack",
            Self::Stack2 => "stack-2",
            Self::Stack3 => "stack-3",
            Self::StackPop => "stack-pop",
            Self::StackPush => "stack-push",
            Self::Sticker => "sticker",
            Self::Sticker2 => "sticker-2",
            Self::StrokeCurved => "stroke-curved",
            Self::StrokeDynamic => "stroke-dynamic",
            Self::StrokeStraight => "stroke-straight",
            Self::Template => "template",
            Self::TemplateOff => "template-off",
            Self::TextResize => "text-resize",
            Self::TiltShift => "tilt-shift",
            Self::TiltShiftOff => "tilt-shift-off",
            Self::Tools => "tools",
            Self::ToolsOff => "tools-off",
            Self::Triangle => "triangle",
            Self::TriangleInverted => "triangle-inverted",
            Self::TriangleMinus => "triangle-minus",
            Self::TriangleMinus2 => "triangle-minus-2",
            Self::TriangleOff => "triangle-off",
            Self::TrianglePlus => "triangle-plus",
            Self::TrianglePlus2 => "triangle-plus-2",
            Self::TriangleSquareCircle => "triangle-square-circle",
            Self::Triangles => "triangles",
            Self::Trident => "trident",
            Self::Typeface => "typeface",
            Self::UxCircle => "ux-circle",
            Self::Vector => "vector",
            Self::VectorBezier => "vector-bezier",
            Self::VectorBezier2 => "vector-bezier-2",
            Self::VectorBezierArc => "vector-bezier-arc",
            Self::VectorBezierCircle => "vector-bezier-circle",
            Self::VectorOff => "vector-off",
            Self::VectorSpline => "vector-spline",
            Self::VectorTriangle => "vector-triangle",
            Self::VectorTriangleOff => "vector-triangle-off",
            Self::Vignette => "vignette",
            Self::Wheel => "wheel",
        }
    }

    fn outline_svg(&self) -> &'static str {
        match self {
            Self::Ad => AD_SVG,
            Self::Ad2 => AD_2_SVG,
            Self::AdOff => AD_OFF_SVG,
            Self::Angle => ANGLE_SVG,
            Self::Aperture => APERTURE_SVG,
            Self::ApertureOff => APERTURE_OFF_SVG,
            Self::Artboard => ARTBOARD_SVG,
            Self::ArtboardOff => ARTBOARD_OFF_SVG,
            Self::Award => AWARD_SVG,
            Self::AwardOff => AWARD_OFF_SVG,
            Self::Background => BACKGROUND_SVG,
            Self::Badge => BADGE_SVG,
            Self::Badge2k => BADGE_2K_SVG,
            Self::Badge3d => BADGE_3D_SVG,
            Self::Badge3k => BADGE_3K_SVG,
            Self::Badge4k => BADGE_4K_SVG,
            Self::Badge5k => BADGE_5K_SVG,
            Self::Badge8k => BADGE_8K_SVG,
            Self::BadgeAd => BADGE_AD_SVG,
            Self::BadgeAdOff => BADGE_AD_OFF_SVG,
            Self::BadgeAr => BADGE_AR_SVG,
            Self::BadgeCc => BADGE_CC_SVG,
            Self::BadgeHd => BADGE_HD_SVG,
            Self::BadgeOff => BADGE_OFF_SVG,
            Self::BadgeSd => BADGE_SD_SVG,
            Self::BadgeTm => BADGE_TM_SVG,
            Self::BadgeVo => BADGE_VO_SVG,
            Self::BadgeVr => BADGE_VR_SVG,
            Self::BadgeWc => BADGE_WC_SVG,
            Self::Badges => BADGES_SVG,
            Self::BadgesOff => BADGES_OFF_SVG,
            Self::Barrel => BARREL_SVG,
            Self::BarrelOff => BARREL_OFF_SVG,
            Self::BarrierBlock => BARRIER_BLOCK_SVG,
            Self::BarrierBlockOff => BARRIER_BLOCK_OFF_SVG,
            Self::Binoculars => BINOCULARS_SVG,
            Self::Blade => BLADE_SVG,
            Self::BlendMode => BLEND_MODE_SVG,
            Self::Blur => BLUR_SVG,
            Self::BlurOff => BLUR_OFF_SVG,
            Self::Bolt => BOLT_SVG,
            Self::BoltOff => BOLT_OFF_SVG,
            Self::Bomb => BOMB_SVG,
            Self::Bong => BONG_SVG,
            Self::BongOff => BONG_OFF_SVG,
            Self::Boom => BOOM_SVG,
            Self::BorderAll => BORDER_ALL_SVG,
            Self::BorderBottom => BORDER_BOTTOM_SVG,
            Self::BorderBottomPlus => BORDER_BOTTOM_PLUS_SVG,
            Self::BorderCornerIos => BORDER_CORNER_IOS_SVG,
            Self::BorderCornerPill => BORDER_CORNER_PILL_SVG,
            Self::BorderCornerRounded => BORDER_CORNER_ROUNDED_SVG,
            Self::BorderCornerSquare => BORDER_CORNER_SQUARE_SVG,
            Self::BorderCorners => BORDER_CORNERS_SVG,
            Self::BorderHorizontal => BORDER_HORIZONTAL_SVG,
            Self::BorderInner => BORDER_INNER_SVG,
            Self::BorderLeft => BORDER_LEFT_SVG,
            Self::BorderLeftPlus => BORDER_LEFT_PLUS_SVG,
            Self::BorderNone => BORDER_NONE_SVG,
            Self::BorderOuter => BORDER_OUTER_SVG,
            Self::BorderRadius => BORDER_RADIUS_SVG,
            Self::BorderRight => BORDER_RIGHT_SVG,
            Self::BorderRightPlus => BORDER_RIGHT_PLUS_SVG,
            Self::BorderSides => BORDER_SIDES_SVG,
            Self::BorderStyle => BORDER_STYLE_SVG,
            Self::BorderStyle2 => BORDER_STYLE_2_SVG,
            Self::BorderTop => BORDER_TOP_SVG,
            Self::BorderTopPlus => BORDER_TOP_PLUS_SVG,
            Self::BorderVertical => BORDER_VERTICAL_SVG,
            Self::BounceLeft => BOUNCE_LEFT_SVG,
            Self::BounceRight => BOUNCE_RIGHT_SVG,
            Self::Box => BOX_SVG,
            Self::BoxAlignBottom => BOX_ALIGN_BOTTOM_SVG,
            Self::BoxAlignBottomLeft => BOX_ALIGN_BOTTOM_LEFT_SVG,
            Self::BoxAlignBottomRight => BOX_ALIGN_BOTTOM_RIGHT_SVG,
            Self::BoxAlignLeft => BOX_ALIGN_LEFT_SVG,
            Self::BoxAlignRight => BOX_ALIGN_RIGHT_SVG,
            Self::BoxAlignTop => BOX_ALIGN_TOP_SVG,
            Self::BoxAlignTopLeft => BOX_ALIGN_TOP_LEFT_SVG,
            Self::BoxAlignTopRight => BOX_ALIGN_TOP_RIGHT_SVG,
            Self::BoxMargin => BOX_MARGIN_SVG,
            Self::BoxModel => BOX_MODEL_SVG,
            Self::BoxModel2 => BOX_MODEL_2_SVG,
            Self::BoxModel2Off => BOX_MODEL_2_OFF_SVG,
            Self::BoxModelOff => BOX_MODEL_OFF_SVG,
            Self::BoxMultiple => BOX_MULTIPLE_SVG,
            Self::BoxOff => BOX_OFF_SVG,
            Self::BoxPadding => BOX_PADDING_SVG,
            Self::BracketsAngle => BRACKETS_ANGLE_SVG,
            Self::BracketsAngleOff => BRACKETS_ANGLE_OFF_SVG,
            Self::Briefcase => BRIEFCASE_SVG,
            Self::Briefcase2 => BRIEFCASE_2_SVG,
            Self::BriefcaseOff => BRIEFCASE_OFF_SVG,
            Self::Brightness => BRIGHTNESS_SVG,
            Self::Brightness2 => BRIGHTNESS_2_SVG,
            Self::BrightnessAuto => BRIGHTNESS_AUTO_SVG,
            Self::BrightnessDown => BRIGHTNESS_DOWN_SVG,
            Self::BrightnessHalf => BRIGHTNESS_HALF_SVG,
            Self::BrightnessOff => BRIGHTNESS_OFF_SVG,
            Self::BrightnessUp => BRIGHTNESS_UP_SVG,
            Self::Brush => BRUSH_SVG,
            Self::BrushOff => BRUSH_OFF_SVG,
            Self::Bucket => BUCKET_SVG,
            Self::BucketDroplet => BUCKET_DROPLET_SVG,
            Self::BucketOff => BUCKET_OFF_SVG,
            Self::Bulb => BULB_SVG,
            Self::BulbOff => BULB_OFF_SVG,
            Self::CameraRotate => CAMERA_ROTATE_SVG,
            Self::CameraSelfie => CAMERA_SELFIE_SVG,
            Self::Cancel => CANCEL_SVG,
            Self::Candle => CANDLE_SVG,
            Self::CapProjecting => CAP_PROJECTING_SVG,
            Self::CapRounded => CAP_ROUNDED_SVG,
            Self::CapStraight => CAP_STRAIGHT_SVG,
            Self::Capsule => CAPSULE_SVG,
            Self::CapsuleHorizontal => CAPSULE_HORIZONTAL_SVG,
            Self::Cards => CARDS_SVG,
            Self::CaretLeftRight => CARET_LEFT_RIGHT_SVG,
            Self::CarouselHorizontal => CAROUSEL_HORIZONTAL_SVG,
            Self::CarouselVertical => CAROUSEL_VERTICAL_SVG,
            Self::Category => CATEGORY_SVG,
            Self::Category2 => CATEGORY_2_SVG,
            Self::CategoryMinus => CATEGORY_MINUS_SVG,
            Self::CategoryPlus => CATEGORY_PLUS_SVG,
            Self::Cell => CELL_SVG,
            Self::ChairDirector => CHAIR_DIRECTOR_SVG,
            Self::Chisel => CHISEL_SVG,
            Self::ChristmasBall => CHRISTMAS_BALL_SVG,
            Self::Circle => CIRCLE_SVG,
            Self::CircleAsterisk => CIRCLE_ASTERISK_SVG,
            Self::CircleCheck => CIRCLE_CHECK_SVG,
            Self::CircleDashed => CIRCLE_DASHED_SVG,
            Self::CircleDashedCheck => CIRCLE_DASHED_CHECK_SVG,
            Self::CircleDashedMinus => CIRCLE_DASHED_MINUS_SVG,
            Self::CircleDashedPercentage => CIRCLE_DASHED_PERCENTAGE_SVG,
            Self::CircleDashedPlus => CIRCLE_DASHED_PLUS_SVG,
            Self::CircleDashedX => CIRCLE_DASHED_X_SVG,
            Self::CircleDot => CIRCLE_DOT_SVG,
            Self::CircleDotted => CIRCLE_DOTTED_SVG,
            Self::CircleHalf => CIRCLE_HALF_SVG,
            Self::CircleHalf2 => CIRCLE_HALF_2_SVG,
            Self::CircleHalfVertical => CIRCLE_HALF_VERTICAL_SVG,
            Self::CircleKey => CIRCLE_KEY_SVG,
            Self::CircleMinus => CIRCLE_MINUS_SVG,
            Self::CircleMinus2 => CIRCLE_MINUS_2_SVG,
            Self::CircleOff => CIRCLE_OFF_SVG,
            Self::CirclePercentage => CIRCLE_PERCENTAGE_SVG,
            Self::CirclePlus => CIRCLE_PLUS_SVG,
            Self::CirclePlus2 => CIRCLE_PLUS_2_SVG,
            Self::CirclePlusMinus => CIRCLE_PLUS_MINUS_SVG,
            Self::CircleRectangle => CIRCLE_RECTANGLE_SVG,
            Self::CircleRectangleOff => CIRCLE_RECTANGLE_OFF_SVG,
            Self::CircleSquare => CIRCLE_SQUARE_SVG,
            Self::CircleTriangle => CIRCLE_TRIANGLE_SVG,
            Self::CircleX => CIRCLE_X_SVG,
            Self::Circles => CIRCLES_SVG,
            Self::CirclesRelation => CIRCLES_RELATION_SVG,
            Self::Clubs => CLUBS_SVG,
            Self::Coffin => COFFIN_SVG,
            Self::ColorFilter => COLOR_FILTER_SVG,
            Self::ColorPicker => COLOR_PICKER_SVG,
            Self::ColorPickerOff => COLOR_PICKER_OFF_SVG,
            Self::ColorSwatch => COLOR_SWATCH_SVG,
            Self::ColorSwatchOff => COLOR_SWATCH_OFF_SVG,
            Self::Components => COMPONENTS_SVG,
            Self::ComponentsOff => COMPONENTS_OFF_SVG,
            Self::Cone => CONE_SVG,
            Self::Cone2 => CONE_2_SVG,
            Self::ConeOff => CONE_OFF_SVG,
            Self::ConePlus => CONE_PLUS_SVG,
            Self::Confetti => CONFETTI_SVG,
            Self::ConfettiOff => CONFETTI_OFF_SVG,
            Self::Container => CONTAINER_SVG,
            Self::ContainerOff => CONTAINER_OFF_SVG,
            Self::Contrast => CONTRAST_SVG,
            Self::Contrast2 => CONTRAST_2_SVG,
            Self::Contrast2Off => CONTRAST_2_OFF_SVG,
            Self::ContrastOff => CONTRAST_OFF_SVG,
            Self::CookieMan => COOKIE_MAN_SVG,
            Self::Crop => CROP_SVG,
            Self::Crop11 => CROP_1_1_SVG,
            Self::Crop169 => CROP_16_9_SVG,
            Self::Crop32 => CROP_3_2_SVG,
            Self::Crop54 => CROP_5_4_SVG,
            Self::Crop75 => CROP_7_5_SVG,
            Self::CropLandscape => CROP_LANDSCAPE_SVG,
            Self::CropPortrait => CROP_PORTRAIT_SVG,
            Self::Crown => CROWN_SVG,
            Self::CrownOff => CROWN_OFF_SVG,
            Self::Cube => CUBE_SVG,
            Self::CubeOff => CUBE_OFF_SVG,
            Self::CubePlus => CUBE_PLUS_SVG,
            Self::CubeSpark => CUBE_SPARK_SVG,
            Self::Cut => CUT_SVG,
            Self::Cylinder => CYLINDER_SVG,
            Self::CylinderOff => CYLINDER_OFF_SVG,
            Self::CylinderPlus => CYLINDER_PLUS_SVG,
            Self::Diabolo => DIABOLO_SVG,
            Self::DiaboloOff => DIABOLO_OFF_SVG,
            Self::DiaboloPlus => DIABOLO_PLUS_SVG,
            Self::Diamond => DIAMOND_SVG,
            Self::DiamondOff => DIAMOND_OFF_SVG,
            Self::Diamonds => DIAMONDS_SVG,
            Self::Dimensions => DIMENSIONS_SVG,
            Self::DragDrop => DRAG_DROP_SVG,
            Self::DragDrop2 => DRAG_DROP_2_SVG,
            Self::Droplet => DROPLET_SVG,
            Self::DropletBolt => DROPLET_BOLT_SVG,
            Self::DropletCancel => DROPLET_CANCEL_SVG,
            Self::DropletCheck => DROPLET_CHECK_SVG,
            Self::DropletCode => DROPLET_CODE_SVG,
            Self::DropletCog => DROPLET_COG_SVG,
            Self::DropletDollar => DROPLET_DOLLAR_SVG,
            Self::DropletDown => DROPLET_DOWN_SVG,
            Self::DropletExclamation => DROPLET_EXCLAMATION_SVG,
            Self::DropletHalf => DROPLET_HALF_SVG,
            Self::DropletHalf2 => DROPLET_HALF_2_SVG,
            Self::DropletHeart => DROPLET_HEART_SVG,
            Self::DropletMinus => DROPLET_MINUS_SVG,
            Self::DropletOff => DROPLET_OFF_SVG,
            Self::DropletPause => DROPLET_PAUSE_SVG,
            Self::DropletPin => DROPLET_PIN_SVG,
            Self::DropletPlus => DROPLET_PLUS_SVG,
            Self::DropletQuestion => DROPLET_QUESTION_SVG,
            Self::DropletSearch => DROPLET_SEARCH_SVG,
            Self::DropletShare => DROPLET_SHARE_SVG,
            Self::DropletStar => DROPLET_STAR_SVG,
            Self::DropletUp => DROPLET_UP_SVG,
            Self::DropletX => DROPLET_X_SVG,
            Self::EaseIn => EASE_IN_SVG,
            Self::EaseInControlPoint => EASE_IN_CONTROL_POINT_SVG,
            Self::EaseInOut => EASE_IN_OUT_SVG,
            Self::EaseInOutControlPoints => EASE_IN_OUT_CONTROL_POINTS_SVG,
            Self::EaseOut => EASE_OUT_SVG,
            Self::EaseOutControlPoint => EASE_OUT_CONTROL_POINT_SVG,
            Self::Edit => EDIT_SVG,
            Self::EditCircle => EDIT_CIRCLE_SVG,
            Self::EditCircleOff => EDIT_CIRCLE_OFF_SVG,
            Self::EditOff => EDIT_OFF_SVG,
            Self::Exposure => EXPOSURE_SVG,
            Self::Exposure0 => EXPOSURE_0_SVG,
            Self::ExposureMinus1 => EXPOSURE_MINUS_1_SVG,
            Self::ExposureMinus2 => EXPOSURE_MINUS_2_SVG,
            Self::ExposureOff => EXPOSURE_OFF_SVG,
            Self::ExposurePlus1 => EXPOSURE_PLUS_1_SVG,
            Self::ExposurePlus2 => EXPOSURE_PLUS_2_SVG,
            Self::Favicon => FAVICON_SVG,
            Self::FlipHorizontal => FLIP_HORIZONTAL_SVG,
            Self::FlipVertical => FLIP_VERTICAL_SVG,
            Self::Focus => FOCUS_SVG,
            Self::Focus2 => FOCUS_2_SVG,
            Self::FocusAuto => FOCUS_AUTO_SVG,
            Self::Frame => FRAME_SVG,
            Self::FrameOff => FRAME_OFF_SVG,
            Self::FreezeColumn => FREEZE_COLUMN_SVG,
            Self::FreezeRow => FREEZE_ROW_SVG,
            Self::FreezeRowColumn => FREEZE_ROW_COLUMN_SVG,
            Self::Frustum => FRUSTUM_SVG,
            Self::FrustumOff => FRUSTUM_OFF_SVG,
            Self::FrustumPlus => FRUSTUM_PLUS_SVG,
            Self::GenderAgender => GENDER_AGENDER_SVG,
            Self::GenderAndrogyne => GENDER_ANDROGYNE_SVG,
            Self::GenderBigender => GENDER_BIGENDER_SVG,
            Self::GenderDemiboy => GENDER_DEMIBOY_SVG,
            Self::GenderDemigirl => GENDER_DEMIGIRL_SVG,
            Self::GenderEpicene => GENDER_EPICENE_SVG,
            Self::GenderFemale => GENDER_FEMALE_SVG,
            Self::GenderFemme => GENDER_FEMME_SVG,
            Self::GenderGenderfluid => GENDER_GENDERFLUID_SVG,
            Self::GenderGenderless => GENDER_GENDERLESS_SVG,
            Self::GenderGenderqueer => GENDER_GENDERQUEER_SVG,
            Self::GenderHermaphrodite => GENDER_HERMAPHRODITE_SVG,
            Self::GenderIntergender => GENDER_INTERGENDER_SVG,
            Self::GenderMale => GENDER_MALE_SVG,
            Self::GenderNeutrois => GENDER_NEUTROIS_SVG,
            Self::GenderThird => GENDER_THIRD_SVG,
            Self::GenderTransgender => GENDER_TRANSGENDER_SVG,
            Self::GenderTrasvesti => GENDER_TRASVESTI_SVG,
            Self::Gizmo => GIZMO_SVG,
            Self::Gradienter => GRADIENTER_SVG,
            Self::Grain => GRAIN_SVG,
            Self::Grid3x3 => GRID_3X3_SVG,
            Self::Grid4x4 => GRID_4X4_SVG,
            Self::GridGoldenratio => GRID_GOLDENRATIO_SVG,
            Self::GuitarPick => GUITAR_PICK_SVG,
            Self::Hammer => HAMMER_SVG,
            Self::HammerOff => HAMMER_OFF_SVG,
            Self::HandClick => HAND_CLICK_SVG,
            Self::HandClickOff => HAND_CLICK_OFF_SVG,
            Self::HandFinger => HAND_FINGER_SVG,
            Self::HandFingerDown => HAND_FINGER_DOWN_SVG,
            Self::HandFingerLeft => HAND_FINGER_LEFT_SVG,
            Self::HandFingerOff => HAND_FINGER_OFF_SVG,
            Self::HandFingerRight => HAND_FINGER_RIGHT_SVG,
            Self::HandGrab => HAND_GRAB_SVG,
            Self::HandLittleFinger => HAND_LITTLE_FINGER_SVG,
            Self::HandLoveYou => HAND_LOVE_YOU_SVG,
            Self::HandMiddleFinger => HAND_MIDDLE_FINGER_SVG,
            Self::HandMove => HAND_MOVE_SVG,
            Self::HandOff => HAND_OFF_SVG,
            Self::HandRingFinger => HAND_RING_FINGER_SVG,
            Self::HandStop => HAND_STOP_SVG,
            Self::HandThreeFingers => HAND_THREE_FINGERS_SVG,
            Self::HandTwoFingers => HAND_TWO_FINGERS_SVG,
            Self::Hash => HASH_SVG,
            Self::Hdr => HDR_SVG,
            Self::Heart => HEART_SVG,
            Self::HeartBolt => HEART_BOLT_SVG,
            Self::HeartCancel => HEART_CANCEL_SVG,
            Self::HeartCheck => HEART_CHECK_SVG,
            Self::HeartCode => HEART_CODE_SVG,
            Self::HeartCog => HEART_COG_SVG,
            Self::HeartDollar => HEART_DOLLAR_SVG,
            Self::HeartDown => HEART_DOWN_SVG,
            Self::HeartExclamation => HEART_EXCLAMATION_SVG,
            Self::HeartMinus => HEART_MINUS_SVG,
            Self::HeartOff => HEART_OFF_SVG,
            Self::HeartPause => HEART_PAUSE_SVG,
            Self::HeartPin => HEART_PIN_SVG,
            Self::HeartPlus => HEART_PLUS_SVG,
            Self::HeartQuestion => HEART_QUESTION_SVG,
            Self::HeartSearch => HEART_SEARCH_SVG,
            Self::HeartShare => HEART_SHARE_SVG,
            Self::HeartSpark => HEART_SPARK_SVG,
            Self::HeartStar => HEART_STAR_SVG,
            Self::HeartUp => HEART_UP_SVG,
            Self::HeartX => HEART_X_SVG,
            Self::Hearts => HEARTS_SVG,
            Self::HeartsOff => HEARTS_OFF_SVG,
            Self::Hemisphere => HEMISPHERE_SVG,
            Self::HemisphereOff => HEMISPHERE_OFF_SVG,
            Self::HemispherePlus => HEMISPHERE_PLUS_SVG,
            Self::Hexagon => HEXAGON_SVG,
            Self::Hexagon3d => HEXAGON_3D_SVG,
            Self::HexagonAsterisk => HEXAGON_ASTERISK_SVG,
            Self::HexagonMinus => HEXAGON_MINUS_SVG,
            Self::HexagonMinus2 => HEXAGON_MINUS_2_SVG,
            Self::HexagonOff => HEXAGON_OFF_SVG,
            Self::HexagonPlus => HEXAGON_PLUS_SVG,
            Self::HexagonPlus2 => HEXAGON_PLUS_2_SVG,
            Self::HexagonalPrism => HEXAGONAL_PRISM_SVG,
            Self::HexagonalPrismOff => HEXAGONAL_PRISM_OFF_SVG,
            Self::HexagonalPrismPlus => HEXAGONAL_PRISM_PLUS_SVG,
            Self::HexagonalPyramid => HEXAGONAL_PYRAMID_SVG,
            Self::HexagonalPyramidOff => HEXAGONAL_PYRAMID_OFF_SVG,
            Self::HexagonalPyramidPlus => HEXAGONAL_PYRAMID_PLUS_SVG,
            Self::Hexagons => HEXAGONS_SVG,
            Self::HexagonsOff => HEXAGONS_OFF_SVG,
            Self::Hierarchy => HIERARCHY_SVG,
            Self::Hierarchy2 => HIERARCHY_2_SVG,
            Self::Hierarchy3 => HIERARCHY_3_SVG,
            Self::HierarchyOff => HIERARCHY_OFF_SVG,
            Self::Icons => ICONS_SVG,
            Self::IconsOff => ICONS_OFF_SVG,
            Self::Ikosaedr => IKOSAEDR_SVG,
            Self::ImageInPicture => IMAGE_IN_PICTURE_SVG,
            Self::InnerShadowBottom => INNER_SHADOW_BOTTOM_SVG,
            Self::InnerShadowBottomLeft => INNER_SHADOW_BOTTOM_LEFT_SVG,
            Self::InnerShadowBottomRight => INNER_SHADOW_BOTTOM_RIGHT_SVG,
            Self::InnerShadowLeft => INNER_SHADOW_LEFT_SVG,
            Self::InnerShadowRight => INNER_SHADOW_RIGHT_SVG,
            Self::InnerShadowTop => INNER_SHADOW_TOP_SVG,
            Self::InnerShadowTopLeft => INNER_SHADOW_TOP_LEFT_SVG,
            Self::InnerShadowTopRight => INNER_SHADOW_TOP_RIGHT_SVG,
            Self::IrregularPolyhedron => IRREGULAR_POLYHEDRON_SVG,
            Self::IrregularPolyhedronOff => IRREGULAR_POLYHEDRON_OFF_SVG,
            Self::IrregularPolyhedronPlus => IRREGULAR_POLYHEDRON_PLUS_SVG,
            Self::JewishStar => JEWISH_STAR_SVG,
            Self::Key => KEY_SVG,
            Self::KeyOff => KEY_OFF_SVG,
            Self::Ladle => LADLE_SVG,
            Self::Lamp => LAMP_SVG,
            Self::Lamp2 => LAMP_2_SVG,
            Self::LampOff => LAMP_OFF_SVG,
            Self::Lasso => LASSO_SVG,
            Self::LassoOff => LASSO_OFF_SVG,
            Self::LassoPolygon => LASSO_POLYGON_SVG,
            Self::LaurelWreath => LAUREL_WREATH_SVG,
            Self::LaurelWreath1 => LAUREL_WREATH_1_SVG,
            Self::LaurelWreath2 => LAUREL_WREATH_2_SVG,
            Self::LaurelWreath3 => LAUREL_WREATH_3_SVG,
            Self::LayersDifference => LAYERS_DIFFERENCE_SVG,
            Self::LayersIntersect => LAYERS_INTERSECT_SVG,
            Self::LayersIntersect2 => LAYERS_INTERSECT_2_SVG,
            Self::LayersLinked => LAYERS_LINKED_SVG,
            Self::LayersOff => LAYERS_OFF_SVG,
            Self::LayersSelected => LAYERS_SELECTED_SVG,
            Self::LayersSelectedBottom => LAYERS_SELECTED_BOTTOM_SVG,
            Self::LayersSubtract => LAYERS_SUBTRACT_SVG,
            Self::LayersUnion => LAYERS_UNION_SVG,
            Self::Layout => LAYOUT_SVG,
            Self::Layout2 => LAYOUT_2_SVG,
            Self::LayoutAlignBottom => LAYOUT_ALIGN_BOTTOM_SVG,
            Self::LayoutAlignCenter => LAYOUT_ALIGN_CENTER_SVG,
            Self::LayoutAlignLeft => LAYOUT_ALIGN_LEFT_SVG,
            Self::LayoutAlignMiddle => LAYOUT_ALIGN_MIDDLE_SVG,
            Self::LayoutAlignRight => LAYOUT_ALIGN_RIGHT_SVG,
            Self::LayoutAlignTop => LAYOUT_ALIGN_TOP_SVG,
            Self::LayoutBoard => LAYOUT_BOARD_SVG,
            Self::LayoutBoardSplit => LAYOUT_BOARD_SPLIT_SVG,
            Self::LayoutBottombar => LAYOUT_BOTTOMBAR_SVG,
            Self::LayoutBottombarCollapse => LAYOUT_BOTTOMBAR_COLLAPSE_SVG,
            Self::LayoutBottombarExpand => LAYOUT_BOTTOMBAR_EXPAND_SVG,
            Self::LayoutCards => LAYOUT_CARDS_SVG,
            Self::LayoutCollage => LAYOUT_COLLAGE_SVG,
            Self::LayoutColumns => LAYOUT_COLUMNS_SVG,
            Self::LayoutDashboard => LAYOUT_DASHBOARD_SVG,
            Self::LayoutDistributeHorizontal => LAYOUT_DISTRIBUTE_HORIZONTAL_SVG,
            Self::LayoutDistributeVertical => LAYOUT_DISTRIBUTE_VERTICAL_SVG,
            Self::LayoutGrid => LAYOUT_GRID_SVG,
            Self::LayoutGridAdd => LAYOUT_GRID_ADD_SVG,
            Self::LayoutGridRemove => LAYOUT_GRID_REMOVE_SVG,
            Self::LayoutKanban => LAYOUT_KANBAN_SVG,
            Self::LayoutList => LAYOUT_LIST_SVG,
            Self::LayoutNavbar => LAYOUT_NAVBAR_SVG,
            Self::LayoutNavbarCollapse => LAYOUT_NAVBAR_COLLAPSE_SVG,
            Self::LayoutNavbarExpand => LAYOUT_NAVBAR_EXPAND_SVG,
            Self::LayoutOff => LAYOUT_OFF_SVG,
            Self::LayoutRows => LAYOUT_ROWS_SVG,
            Self::LayoutSidebar => LAYOUT_SIDEBAR_SVG,
            Self::LayoutSidebarLeftCollapse => LAYOUT_SIDEBAR_LEFT_COLLAPSE_SVG,
            Self::LayoutSidebarLeftExpand => LAYOUT_SIDEBAR_LEFT_EXPAND_SVG,
            Self::LayoutSidebarRight => LAYOUT_SIDEBAR_RIGHT_SVG,
            Self::LayoutSidebarRightCollapse => LAYOUT_SIDEBAR_RIGHT_COLLAPSE_SVG,
            Self::LayoutSidebarRightExpand => LAYOUT_SIDEBAR_RIGHT_EXPAND_SVG,
            Self::Line => LINE_SVG,
            Self::LineDashed => LINE_DASHED_SVG,
            Self::LineDotted => LINE_DOTTED_SVG,
            Self::LivePhoto => LIVE_PHOTO_SVG,
            Self::LivePhotoOff => LIVE_PHOTO_OFF_SVG,
            Self::Macro => MACRO_SVG,
            Self::MacroOff => MACRO_OFF_SVG,
            Self::Magnet => MAGNET_SVG,
            Self::MagnetOff => MAGNET_OFF_SVG,
            Self::Magnetic => MAGNETIC_SVG,
            Self::Mask => MASK_SVG,
            Self::MaskOff => MASK_OFF_SVG,
            Self::Mesh => MESH_SVG,
            Self::Mickey => MICKEY_SVG,
            Self::MoodAngry => MOOD_ANGRY_SVG,
            Self::MoodAnnoyed => MOOD_ANNOYED_SVG,
            Self::MoodAnnoyed2 => MOOD_ANNOYED_2_SVG,
            Self::MoodBitcoin => MOOD_BITCOIN_SVG,
            Self::MoodBoy => MOOD_BOY_SVG,
            Self::MoodCheck => MOOD_CHECK_SVG,
            Self::MoodCog => MOOD_COG_SVG,
            Self::MoodConfuzed => MOOD_CONFUZED_SVG,
            Self::MoodCrazyHappy => MOOD_CRAZY_HAPPY_SVG,
            Self::MoodCry => MOOD_CRY_SVG,
            Self::MoodDollar => MOOD_DOLLAR_SVG,
            Self::MoodEdit => MOOD_EDIT_SVG,
            Self::MoodEmpty => MOOD_EMPTY_SVG,
            Self::MoodHappy => MOOD_HAPPY_SVG,
            Self::MoodHeart => MOOD_HEART_SVG,
            Self::MoodKid => MOOD_KID_SVG,
            Self::MoodLookDown => MOOD_LOOK_DOWN_SVG,
            Self::MoodLookLeft => MOOD_LOOK_LEFT_SVG,
            Self::MoodLookRight => MOOD_LOOK_RIGHT_SVG,
            Self::MoodLookUp => MOOD_LOOK_UP_SVG,
            Self::MoodMinus => MOOD_MINUS_SVG,
            Self::MoodNerd => MOOD_NERD_SVG,
            Self::MoodNervous => MOOD_NERVOUS_SVG,
            Self::MoodNeutral => MOOD_NEUTRAL_SVG,
            Self::MoodOff => MOOD_OFF_SVG,
            Self::MoodPin => MOOD_PIN_SVG,
            Self::MoodPlus => MOOD_PLUS_SVG,
            Self::MoodPuzzled => MOOD_PUZZLED_SVG,
            Self::MoodSad => MOOD_SAD_SVG,
            Self::MoodSad2 => MOOD_SAD_2_SVG,
            Self::MoodSadDizzy => MOOD_SAD_DIZZY_SVG,
            Self::MoodSadSquint => MOOD_SAD_SQUINT_SVG,
            Self::MoodSearch => MOOD_SEARCH_SVG,
            Self::MoodShare => MOOD_SHARE_SVG,
            Self::MoodSick => MOOD_SICK_SVG,
            Self::MoodSilence => MOOD_SILENCE_SVG,
            Self::MoodSing => MOOD_SING_SVG,
            Self::MoodSmile => MOOD_SMILE_SVG,
            Self::MoodSmileBeam => MOOD_SMILE_BEAM_SVG,
            Self::MoodSmileDizzy => MOOD_SMILE_DIZZY_SVG,
            Self::MoodSpark => MOOD_SPARK_SVG,
            Self::MoodSurprised => MOOD_SURPRISED_SVG,
            Self::MoodTongue => MOOD_TONGUE_SVG,
            Self::MoodTongueWink => MOOD_TONGUE_WINK_SVG,
            Self::MoodTongueWink2 => MOOD_TONGUE_WINK_2_SVG,
            Self::MoodUnamused => MOOD_UNAMUSED_SVG,
            Self::MoodUp => MOOD_UP_SVG,
            Self::MoodWink => MOOD_WINK_SVG,
            Self::MoodWink2 => MOOD_WINK_2_SVG,
            Self::MoodWrrr => MOOD_WRRR_SVG,
            Self::MoodX => MOOD_X_SVG,
            Self::MoodXd => MOOD_XD_SVG,
            Self::Moustache => MOUSTACHE_SVG,
            Self::Needle => NEEDLE_SVG,
            Self::NeedleThread => NEEDLE_THREAD_SVG,
            Self::NoiseReduction => NOISE_REDUCTION_SVG,
            Self::Octagon => OCTAGON_SVG,
            Self::OctagonMinus => OCTAGON_MINUS_SVG,
            Self::OctagonMinus2 => OCTAGON_MINUS_2_SVG,
            Self::OctagonOff => OCTAGON_OFF_SVG,
            Self::OctagonPlus => OCTAGON_PLUS_SVG,
            Self::OctagonPlus2 => OCTAGON_PLUS_2_SVG,
            Self::Octahedron => OCTAHEDRON_SVG,
            Self::OctahedronOff => OCTAHEDRON_OFF_SVG,
            Self::OctahedronPlus => OCTAHEDRON_PLUS_SVG,
            Self::Oval => OVAL_SVG,
            Self::OvalVertical => OVAL_VERTICAL_SVG,
            Self::Paint => PAINT_SVG,
            Self::PaintOff => PAINT_OFF_SVG,
            Self::Palette => PALETTE_SVG,
            Self::PaletteOff => PALETTE_OFF_SVG,
            Self::PanoramaHorizontal => PANORAMA_HORIZONTAL_SVG,
            Self::PanoramaHorizontalOff => PANORAMA_HORIZONTAL_OFF_SVG,
            Self::PanoramaVertical => PANORAMA_VERTICAL_SVG,
            Self::PanoramaVerticalOff => PANORAMA_VERTICAL_OFF_SVG,
            Self::Pencil => PENCIL_SVG,
            Self::PencilBolt => PENCIL_BOLT_SVG,
            Self::PencilCancel => PENCIL_CANCEL_SVG,
            Self::PencilCheck => PENCIL_CHECK_SVG,
            Self::PencilCode => PENCIL_CODE_SVG,
            Self::PencilCog => PENCIL_COG_SVG,
            Self::PencilDiscount => PENCIL_DISCOUNT_SVG,
            Self::PencilDollar => PENCIL_DOLLAR_SVG,
            Self::PencilDown => PENCIL_DOWN_SVG,
            Self::PencilExclamation => PENCIL_EXCLAMATION_SVG,
            Self::PencilHeart => PENCIL_HEART_SVG,
            Self::PencilMinus => PENCIL_MINUS_SVG,
            Self::PencilOff => PENCIL_OFF_SVG,
            Self::PencilPause => PENCIL_PAUSE_SVG,
            Self::PencilPin => PENCIL_PIN_SVG,
            Self::PencilPlus => PENCIL_PLUS_SVG,
            Self::PencilQuestion => PENCIL_QUESTION_SVG,
            Self::PencilSearch => PENCIL_SEARCH_SVG,
            Self::PencilShare => PENCIL_SHARE_SVG,
            Self::PencilStar => PENCIL_STAR_SVG,
            Self::PencilUp => PENCIL_UP_SVG,
            Self::PencilX => PENCIL_X_SVG,
            Self::Pentagon => PENTAGON_SVG,
            Self::PentagonMinus => PENTAGON_MINUS_SVG,
            Self::PentagonOff => PENTAGON_OFF_SVG,
            Self::PentagonPlus => PENTAGON_PLUS_SVG,
            Self::PentagonX => PENTAGON_X_SVG,
            Self::Pentagram => PENTAGRAM_SVG,
            Self::Perspective => PERSPECTIVE_SVG,
            Self::PerspectiveOff => PERSPECTIVE_OFF_SVG,
            Self::PhotoBitcoin => PHOTO_BITCOIN_SVG,
            Self::PhotoCircle => PHOTO_CIRCLE_SVG,
            Self::PhotoHexagon => PHOTO_HEXAGON_SVG,
            Self::PhotoPentagon => PHOTO_PENTAGON_SVG,
            Self::PhotoSensor => PHOTO_SENSOR_SVG,
            Self::PhotoSensor2 => PHOTO_SENSOR_2_SVG,
            Self::PhotoSensor3 => PHOTO_SENSOR_3_SVG,
            Self::PhotoSquareRounded => PHOTO_SQUARE_ROUNDED_SVG,
            Self::PhotoVideo => PHOTO_VIDEO_SVG,
            Self::Placeholder => PLACEHOLDER_SVG,
            Self::Polaroid => POLAROID_SVG,
            Self::Polygon => POLYGON_SVG,
            Self::PolygonOff => POLYGON_OFF_SVG,
            Self::Prism => PRISM_SVG,
            Self::PrismOff => PRISM_OFF_SVG,
            Self::PrismPlus => PRISM_PLUS_SVG,
            Self::Prong => PRONG_SVG,
            Self::Pyramid => PYRAMID_SVG,
            Self::PyramidOff => PYRAMID_OFF_SVG,
            Self::PyramidPlus => PYRAMID_PLUS_SVG,
            Self::RadiusBottomLeft => RADIUS_BOTTOM_LEFT_SVG,
            Self::RadiusBottomRight => RADIUS_BOTTOM_RIGHT_SVG,
            Self::RadiusTopLeft => RADIUS_TOP_LEFT_SVG,
            Self::RadiusTopRight => RADIUS_TOP_RIGHT_SVG,
            Self::Rectangle => RECTANGLE_SVG,
            Self::RectangleRoundedBottom => RECTANGLE_ROUNDED_BOTTOM_SVG,
            Self::RectangleRoundedTop => RECTANGLE_ROUNDED_TOP_SVG,
            Self::RectangleVertical => RECTANGLE_VERTICAL_SVG,
            Self::RectangularPrism => RECTANGULAR_PRISM_SVG,
            Self::RectangularPrismOff => RECTANGULAR_PRISM_OFF_SVG,
            Self::RectangularPrismPlus => RECTANGULAR_PRISM_PLUS_SVG,
            Self::Resize => RESIZE_SVG,
            Self::Rosette => ROSETTE_SVG,
            Self::RosetteAsterisk => ROSETTE_ASTERISK_SVG,
            Self::Ruler => RULER_SVG,
            Self::Ruler2 => RULER_2_SVG,
            Self::Ruler2Off => RULER_2_OFF_SVG,
            Self::Ruler3 => RULER_3_SVG,
            Self::RulerMeasure => RULER_MEASURE_SVG,
            Self::RulerMeasure2 => RULER_MEASURE_2_SVG,
            Self::RulerOff => RULER_OFF_SVG,
            Self::Scissors => SCISSORS_SVG,
            Self::ScissorsOff => SCISSORS_OFF_SVG,
            Self::Screenshot => SCREENSHOT_SVG,
            Self::Section => SECTION_SVG,
            Self::Shadow => SHADOW_SVG,
            Self::ShadowOff => SHADOW_OFF_SVG,
            Self::Shape => SHAPE_SVG,
            Self::Shape2 => SHAPE_2_SVG,
            Self::Shape3 => SHAPE_3_SVG,
            Self::ShapeOff => SHAPE_OFF_SVG,
            Self::Shovel => SHOVEL_SVG,
            Self::ShovelPitchforks => SHOVEL_PITCHFORKS_SVG,
            Self::Sketching => SKETCHING_SVG,
            Self::Slice => SLICE_SVG,
            Self::Spade => SPADE_SVG,
            Self::Sparkle => SPARKLE_SVG,
            Self::Sparkle2 => SPARKLE_2_SVG,
            Self::SparkleHighlight => SPARKLE_HIGHLIGHT_SVG,
            Self::Sparkles2 => SPARKLES_2_SVG,
            Self::Sphere => SPHERE_SVG,
            Self::Sphere2 => SPHERE_2_SVG,
            Self::SphereOff => SPHERE_OFF_SVG,
            Self::SpherePlus => SPHERE_PLUS_SVG,
            Self::Square => SQUARE_SVG,
            Self::SquareAsterisk => SQUARE_ASTERISK_SVG,
            Self::SquareCheck => SQUARE_CHECK_SVG,
            Self::SquareDashed => SQUARE_DASHED_SVG,
            Self::SquareDot => SQUARE_DOT_SVG,
            Self::SquareForbid => SQUARE_FORBID_SVG,
            Self::SquareForbid2 => SQUARE_FORBID_2_SVG,
            Self::SquareHalf => SQUARE_HALF_SVG,
            Self::SquareKey => SQUARE_KEY_SVG,
            Self::SquareMinus => SQUARE_MINUS_SVG,
            Self::SquareMinus2 => SQUARE_MINUS_2_SVG,
            Self::SquareOff => SQUARE_OFF_SVG,
            Self::SquarePercentage => SQUARE_PERCENTAGE_SVG,
            Self::SquarePlus => SQUARE_PLUS_SVG,
            Self::SquarePlus2 => SQUARE_PLUS_2_SVG,
            Self::SquareRotated => SQUARE_ROTATED_SVG,
            Self::SquareRotatedAsterisk => SQUARE_ROTATED_ASTERISK_SVG,
            Self::SquareRotatedForbid => SQUARE_ROTATED_FORBID_SVG,
            Self::SquareRotatedForbid2 => SQUARE_ROTATED_FORBID_2_SVG,
            Self::SquareRotatedOff => SQUARE_ROTATED_OFF_SVG,
            Self::SquareRounded => SQUARE_ROUNDED_SVG,
            Self::SquareRoundedCheck => SQUARE_ROUNDED_CHECK_SVG,
            Self::SquareRoundedMinus => SQUARE_ROUNDED_MINUS_SVG,
            Self::SquareRoundedMinus2 => SQUARE_ROUNDED_MINUS_2_SVG,
            Self::SquareRoundedPercentage => SQUARE_ROUNDED_PERCENTAGE_SVG,
            Self::SquareRoundedPlus => SQUARE_ROUNDED_PLUS_SVG,
            Self::SquareRoundedPlus2 => SQUARE_ROUNDED_PLUS_2_SVG,
            Self::SquareRoundedX => SQUARE_ROUNDED_X_SVG,
            Self::SquareToggle => SQUARE_TOGGLE_SVG,
            Self::SquareToggleHorizontal => SQUARE_TOGGLE_HORIZONTAL_SVG,
            Self::SquareX => SQUARE_X_SVG,
            Self::Squares => SQUARES_SVG,
            Self::SquaresDiagonal => SQUARES_DIAGONAL_SVG,
            Self::SquaresSelected => SQUARES_SELECTED_SVG,
            Self::Stack => STACK_SVG,
            Self::Stack2 => STACK_2_SVG,
            Self::Stack3 => STACK_3_SVG,
            Self::StackPop => STACK_POP_SVG,
            Self::StackPush => STACK_PUSH_SVG,
            Self::Sticker => STICKER_SVG,
            Self::Sticker2 => STICKER_2_SVG,
            Self::StrokeCurved => STROKE_CURVED_SVG,
            Self::StrokeDynamic => STROKE_DYNAMIC_SVG,
            Self::StrokeStraight => STROKE_STRAIGHT_SVG,
            Self::Template => TEMPLATE_SVG,
            Self::TemplateOff => TEMPLATE_OFF_SVG,
            Self::TextResize => TEXT_RESIZE_SVG,
            Self::TiltShift => TILT_SHIFT_SVG,
            Self::TiltShiftOff => TILT_SHIFT_OFF_SVG,
            Self::Tools => TOOLS_SVG,
            Self::ToolsOff => TOOLS_OFF_SVG,
            Self::Triangle => TRIANGLE_SVG,
            Self::TriangleInverted => TRIANGLE_INVERTED_SVG,
            Self::TriangleMinus => TRIANGLE_MINUS_SVG,
            Self::TriangleMinus2 => TRIANGLE_MINUS_2_SVG,
            Self::TriangleOff => TRIANGLE_OFF_SVG,
            Self::TrianglePlus => TRIANGLE_PLUS_SVG,
            Self::TrianglePlus2 => TRIANGLE_PLUS_2_SVG,
            Self::TriangleSquareCircle => TRIANGLE_SQUARE_CIRCLE_SVG,
            Self::Triangles => TRIANGLES_SVG,
            Self::Trident => TRIDENT_SVG,
            Self::Typeface => TYPEFACE_SVG,
            Self::UxCircle => UX_CIRCLE_SVG,
            Self::Vector => VECTOR_SVG,
            Self::VectorBezier => VECTOR_BEZIER_SVG,
            Self::VectorBezier2 => VECTOR_BEZIER_2_SVG,
            Self::VectorBezierArc => VECTOR_BEZIER_ARC_SVG,
            Self::VectorBezierCircle => VECTOR_BEZIER_CIRCLE_SVG,
            Self::VectorOff => VECTOR_OFF_SVG,
            Self::VectorSpline => VECTOR_SPLINE_SVG,
            Self::VectorTriangle => VECTOR_TRIANGLE_SVG,
            Self::VectorTriangleOff => VECTOR_TRIANGLE_OFF_SVG,
            Self::Vignette => VIGNETTE_SVG,
            Self::Wheel => WHEEL_SVG,
        }
    }

    fn filled_svg(&self) -> Option<&'static str> {
        // Filled variants would be added here
        None
    }
}
