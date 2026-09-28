//! Devices icons from Tabler Icons.
//!
//! This module contains 443 icons.

use crate::tabler::TablerIconData;

// SVG Constants
const ACCESS_POINT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12l0 .01" /> <path d="M14.828 9.172a4 4 0 0 1 0 5.656" /> <path d="M17.657 6.343a8 8 0 0 1 0 11.314" /> <path d="M9.168 14.828a4 4 0 0 1 0 -5.656" /> <path d="M6.337 17.657a8 8 0 0 1 0 -11.314" /> </svg>"##;
const ACCESS_POINT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M14.828 9.172a4 4 0 0 1 1.172 2.828" /> <path d="M17.657 6.343a8 8 0 0 1 1.635 8.952" /> <path d="M9.168 14.828a4 4 0 0 1 0 -5.656" /> <path d="M6.337 17.657a8 8 0 0 1 0 -11.314" /> </svg>"##;
const AIR_CONDITIONING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16a3 3 0 0 1 -3 3" /> <path d="M16 16a3 3 0 0 0 3 3" /> <path d="M12 16v4" /> <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -4" /> <path d="M7 13v-3a1 1 0 0 1 1 -1h8a1 1 0 0 1 1 1v3" /> </svg>"##;
const AIR_CONDITIONING_DISABLED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -4" /> <path d="M7 16v-3a1 1 0 0 1 1 -1h8a1 1 0 0 1 1 1v3" /> </svg>"##;
const ALARM_SMOKE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 8l-.8 3a1.25 1.25 0 0 1 -1.2 1h-8a1.25 1.25 0 0 1 -1.2 -1l-.8 -3" /> <path d="M3 5a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1l0 -2" /> <path d="M12 16c.643 .288 1.017 .756 1 1.25c.017 .494 -.357 .962 -1 1.25s-1.017 .756 -1 1.25c-.017 .494 .357 .962 1 1.25" /> <path d="M7 16c.643 .288 1.017 .756 1 1.25c.017 .494 -.357 .962 -1 1.25s-1.017 .756 -1 1.25c-.017 .494 .357 .962 1 1.25" /> <path d="M17 16c.643 .288 1.017 .756 1 1.25c.017 .494 -.357 .962 -1 1.25s-1.017 .756 -1 1.25c-.017 .494 .357 .962 1 1.25" /> </svg>"##;
const ANTENNA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 4v8" /> <path d="M16 4.5v7" /> <path d="M12 5v16" /> <path d="M8 5.5v5" /> <path d="M4 6v4" /> <path d="M20 8h-16" /> </svg>"##;
const ANTENNA_BARS_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 18l0 .01" /> <path d="M10 18l0 .01" /> <path d="M14 18l0 .01" /> <path d="M18 18l0 .01" /> </svg>"##;
const ANTENNA_BARS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 18l0 -3" /> <path d="M10 18l0 .01" /> <path d="M14 18l0 .01" /> <path d="M18 18l0 .01" /> </svg>"##;
const ANTENNA_BARS_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 18l0 -3" /> <path d="M10 18l0 -6" /> <path d="M14 18l0 .01" /> <path d="M18 18l0 .01" /> </svg>"##;
const ANTENNA_BARS_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 18l0 -3" /> <path d="M10 18l0 -6" /> <path d="M14 18l0 -9" /> <path d="M18 18l0 .01" /> </svg>"##;
const ANTENNA_BARS_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 18l0 -3" /> <path d="M10 18l0 -6" /> <path d="M14 18l0 -9" /> <path d="M18 18l0 -12" /> </svg>"##;
const ANTENNA_BARS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 18v-3" /> <path d="M10 18v-6" /> <path d="M14 18v-4" /> <path d="M14 10v-1" /> <path d="M18 14v-8" /> <path d="M3 3l18 18" /> </svg>"##;
const ANTENNA_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 4v8" /> <path d="M16 4.5v7" /> <path d="M12 5v3m0 4v9" /> <path d="M8 8v2.5" /> <path d="M4 6v4" /> <path d="M20 8h-8m-4 0h-4" /> <path d="M3 3l18 18" /> </svg>"##;
const BATTERY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 7h11a2 2 0 0 1 2 2v.5a.5 .5 0 0 0 .5 .5a.5 .5 0 0 1 .5 .5v3a.5 .5 0 0 1 -.5 .5a.5 .5 0 0 0 -.5 .5v.5a2 2 0 0 1 -2 2h-11a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2" /> </svg>"##;
const BATTERY_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 7h11a2 2 0 0 1 2 2v.5a.5 .5 0 0 0 .5 .5a.5 .5 0 0 1 .5 .5v3a.5 .5 0 0 1 -.5 .5a.5 .5 0 0 0 -.5 .5v.5a2 2 0 0 1 -2 2h-11a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2" /> <path d="M7 10l0 4" /> </svg>"##;
const BATTERY_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 7h11a2 2 0 0 1 2 2v.5a.5 .5 0 0 0 .5 .5a.5 .5 0 0 1 .5 .5v3a.5 .5 0 0 1 -.5 .5a.5 .5 0 0 0 -.5 .5v.5a2 2 0 0 1 -2 2h-11a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2" /> <path d="M7 10l0 4" /> <path d="M10 10l0 4" /> </svg>"##;
const BATTERY_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 7h11a2 2 0 0 1 2 2v.5a.5 .5 0 0 0 .5 .5a.5 .5 0 0 1 .5 .5v3a.5 .5 0 0 1 -.5 .5a.5 .5 0 0 0 -.5 .5v.5a2 2 0 0 1 -2 2h-11a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2" /> <path d="M7 10l0 4" /> <path d="M10 10l0 4" /> <path d="M13 10l0 4" /> </svg>"##;
const BATTERY_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 7h11a2 2 0 0 1 2 2v.5a.5 .5 0 0 0 .5 .5a.5 .5 0 0 1 .5 .5v3a.5 .5 0 0 1 -.5 .5a.5 .5 0 0 0 -.5 .5v.5a2 2 0 0 1 -2 2h-11a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2" /> <path d="M7 10l0 4" /> <path d="M10 10l0 4" /> <path d="M13 10l0 4" /> <path d="M16 10l0 4" /> </svg>"##;
const BATTERY_CHARGING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 7h1a2 2 0 0 1 2 2v.5a.5 .5 0 0 0 .5 .5a.5 .5 0 0 1 .5 .5v3a.5 .5 0 0 1 -.5 .5a.5 .5 0 0 0 -.5 .5v.5a2 2 0 0 1 -2 2h-2" /> <path d="M8 7h-2a2 2 0 0 0 -2 2v6a2 2 0 0 0 2 2h1" /> <path d="M12 8l-2 4h3l-2 4" /> </svg>"##;
const BATTERY_CHARGING_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 9a2 2 0 0 1 2 -2h11a2 2 0 0 1 2 2v.5a.5 .5 0 0 0 .5 .5a.5 .5 0 0 1 .5 .5v3a.5 .5 0 0 1 -.5 .5a.5 .5 0 0 0 -.5 .5v.5a2 2 0 0 1 -2 2h-4.5" /> <path d="M3 15h6v2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2v-2" /> <path d="M6 22v-3" /> <path d="M4 15v-2.5" /> <path d="M8 15v-2.5" /> </svg>"##;
const BATTERY_ECO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 9a2 2 0 0 1 2 -2h11a2 2 0 0 1 2 2v.5a.5 .5 0 0 0 .5 .5a.5 .5 0 0 1 .5 .5v3a.5 .5 0 0 1 -.5 .5a.5 .5 0 0 0 -.5 .5v.5a2 2 0 0 1 -2 2h-5.5" /> <path d="M3 16.143c0 -2.84 2.09 -5.143 4.667 -5.143h2.333v.857c0 2.84 -2.09 5.143 -4.667 5.143h-2.333v-.857" /> <path d="M3 20v-3" /> </svg>"##;
const BATTERY_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 17h8c1.105 0 2 -.895 2 -2v-.5c0 -.276 .224 -.5 .5 -.5s.5 -.224 .5 -.5v-3c0 -.276 -.224 -.5 -.5 -.5s-.5 -.224 -.5 -.5v-.5c0 -1.105 -.895 -2 -2 -2h-11c-1.105 0 -2 .895 -2 2v3" /> <path d="M5 16v3" /> <path d="M5 22v.01" /> </svg>"##;
const BATTERY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M11 7h6a2 2 0 0 1 2 2v.5a.5 .5 0 0 0 .5 .5a.5 .5 0 0 1 .5 .5v3a.5 .5 0 0 1 -.5 .5a.5 .5 0 0 0 -.5 .5v.5m-2 2h-11a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h1" /> </svg>"##;
const BATTERY_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17h-6a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2h11a2 2 0 0 1 2 2v.5a.5 .5 0 0 0 .5 .5a.5 .5 0 0 1 .5 .5v1" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const BATTERY_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18v-11a2 2 0 0 1 2 -2h.5a.5 .5 0 0 0 .5 -.5a.5 .5 0 0 1 .5 -.5h3a.5 .5 0 0 1 .5 .5a.5 .5 0 0 0 .5 .5h.5a2 2 0 0 1 2 2v11a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2" /> </svg>"##;
const BATTERY_VERTICAL_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18v-11a2 2 0 0 1 2 -2h.5a.5 .5 0 0 0 .5 -.5a.5 .5 0 0 1 .5 -.5h3a.5 .5 0 0 1 .5 .5a.5 .5 0 0 0 .5 .5h.5a2 2 0 0 1 2 2v11a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2" /> <path d="M10 17h4" /> </svg>"##;
const BATTERY_VERTICAL_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18v-11a2 2 0 0 1 2 -2h.5a.5 .5 0 0 0 .5 -.5a.5 .5 0 0 1 .5 -.5h3a.5 .5 0 0 1 .5 .5a.5 .5 0 0 0 .5 .5h.5a2 2 0 0 1 2 2v11a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2" /> <path d="M10 17h4" /> <path d="M10 14h4" /> </svg>"##;
const BATTERY_VERTICAL_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18v-11a2 2 0 0 1 2 -2h.5a.5 .5 0 0 0 .5 -.5a.5 .5 0 0 1 .5 -.5h3a.5 .5 0 0 1 .5 .5a.5 .5 0 0 0 .5 .5h.5a2 2 0 0 1 2 2v11a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2" /> <path d="M10 17h4" /> <path d="M10 14h4" /> <path d="M10 11h4" /> </svg>"##;
const BATTERY_VERTICAL_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18v-11a2 2 0 0 1 2 -2h.5a.5 .5 0 0 0 .5 -.5a.5 .5 0 0 1 .5 -.5h3a.5 .5 0 0 1 .5 .5a.5 .5 0 0 0 .5 .5h.5a2 2 0 0 1 2 2v11a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2" /> <path d="M10 17h4" /> <path d="M10 14h4" /> <path d="M10 11h4" /> <path d="M10 8h4" /> </svg>"##;
const BATTERY_VERTICAL_CHARGING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18v-11a2 2 0 0 1 2 -2h.5a.5 .5 0 0 0 .5 -.5a.5 .5 0 0 1 .5 -.5h3a.5 .5 0 0 1 .5 .5a.5 .5 0 0 0 .5 .5h.5a2 2 0 0 1 2 2v11a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2" /> <path d="M12.667 8l-2.667 4h4l-2.667 4" /> </svg>"##;
const BATTERY_VERTICAL_CHARGING_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18v-11c0 -1.105 .895 -2 2 -2h.5c.276 0 .5 -.224 .5 -.5s.224 -.5 .5 -.5h3c.276 0 .5 .224 .5 .5s.224 .5 .5 .5h.5c1.105 0 2 .895 2 2v1m-10 10c0 1.105 .895 2 2 2h1" /> <path d="M12 14h6v2a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2l0 -2" /> <path d="M15 21v-3" /> <path d="M13 14v-2.5" /> <path d="M17 14v-2.5" /> </svg>"##;
const BATTERY_VERTICAL_ECO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18v-11c0 -1.105 .895 -2 2 -2h.5c.276 0 .5 -.224 .5 -.5s.224 -.5 .5 -.5h3c.276 0 .5 .224 .5 .5s.224 .5 .5 .5h.5c1.105 0 2 .895 2 2v1m-8 12c-1.105 0 -2 -.895 -2 -2" /> <path d="M13 17.143c0 -2.84 2.09 -5.143 4.667 -5.143h2.333v.857c0 2.84 -2.09 5.143 -4.667 5.143h-2.333l0 -.857" /> <path d="M13 21v-3" /> </svg>"##;
const BATTERY_VERTICAL_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 12v-5c0 -1.105 -.895 -2 -2 -2h-.5c-.276 0 -.5 -.224 -.5 -.5s-.224 -.5 -.5 -.5h-3c-.276 0 -.5 .224 -.5 .5s-.224 .5 -.5 .5h-.5c-1.105 0 -2 .895 -2 2v11c0 1.105 .895 2 2 2h6" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const BATTERY_VERTICAL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M17 13v-6a2 2 0 0 0 -2 -2h-.5a.5 .5 0 0 1 -.5 -.5a.5 .5 0 0 0 -.5 -.5h-3a.5 .5 0 0 0 -.5 .5a.5 .5 0 0 1 -.5 .5h-.5m-2 2v11a2 2 0 0 0 2 2h6a2 2 0 0 0 2 -2v-1" /> </svg>"##;
const BINARY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 10v-5h-1m8 14v-5h-1" /> <path d="M15 5.5a.5 .5 0 0 1 .5 -.5h2a.5 .5 0 0 1 .5 .5v4a.5 .5 0 0 1 -.5 .5h-2a.5 .5 0 0 1 -.5 -.5l0 -4" /> <path d="M10 14.5a.5 .5 0 0 1 .5 -.5h2a.5 .5 0 0 1 .5 .5v4a.5 .5 0 0 1 -.5 .5h-2a.5 .5 0 0 1 -.5 -.5l0 -4" /> <path d="M6 10h.01m-.01 9h.01" /> </svg>"##;
const BINARY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 7v-2h-1" /> <path d="M18 19v-1" /> <path d="M15.5 5h2a.5 .5 0 0 1 .5 .5v4a.5 .5 0 0 1 -.5 .5h-2a.5 .5 0 0 1 -.5 -.5v-4a.5 .5 0 0 1 .5 -.5" /> <path d="M10.5 14h2a.5 .5 0 0 1 .5 .5v4a.5 .5 0 0 1 -.5 .5h-2a.5 .5 0 0 1 -.5 -.5v-4a.5 .5 0 0 1 .5 -.5" /> <path d="M6 10v.01" /> <path d="M6 19v.01" /> <path d="M3 3l18 18" /> </svg>"##;
const BINARY_TREE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 20a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M16 4a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M16 20a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M11 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M21 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M5.058 18.306l2.88 -4.606" /> <path d="M10.061 10.303l2.877 -4.604" /> <path d="M10.065 13.705l2.876 4.6" /> <path d="M15.063 5.7l2.881 4.61" /> </svg>"##;
const BINARY_TREE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M7 14a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M21 14a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M12 8v8" /> <path d="M6.316 12.496l4.368 -4.992" /> <path d="M17.684 12.496l-4.366 -4.99" /> </svg>"##;
const BLENDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 10h-3a1 1 0 0 1 -1 -1v-4a1 1 0 0 1 1 -1h10.802a1 1 0 0 1 .984 1.179l-1.786 9.821" /> <path d="M8 4l2 11" /> <path d="M11 15h4a3 3 0 0 1 3 3v2a1 1 0 0 1 -1 1h-8a1 1 0 0 1 -1 -1v-2a3 3 0 0 1 3 -3" /> <path d="M12 4v-1h2v1" /> <path d="M13 18v.01" /> </svg>"##;
const BLUETOOTH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8l10 8l-5 4l0 -16l5 4l-10 8" /> </svg>"##;
const BLUETOOTH_CONNECTED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8l10 8l-5 4l0 -16l5 4l-10 8" /> <path d="M4 12l1 0" /> <path d="M18 12l1 0" /> </svg>"##;
const BLUETOOTH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M16.438 16.45l-4.438 3.55v-8m0 -4v-4l5 4l-2.776 2.22m-2.222 1.779l-5 4" /> </svg>"##;
const BLUETOOTH_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8l10 8l-5 4v-16l1 .802m0 6.396l-6 4.802" /> <path d="M16 6l4 4" /> <path d="M20 6l-4 4" /> </svg>"##;
const BMP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 16v-8h2a2 2 0 1 1 0 4h-2" /> <path d="M6 14a2 2 0 0 1 -2 2h-2v-8h2a2 2 0 1 1 0 4h-2h2a2 2 0 0 1 2 2" /> <path d="M9 16v-8l3 6l3 -6v8" /> </svg>"##;
const BROADCAST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.364 19.364a9 9 0 1 0 -12.728 0" /> <path d="M15.536 16.536a5 5 0 1 0 -7.072 0" /> <path d="M11 13a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const BROADCAST_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.364 19.364a9 9 0 0 0 -9.721 -14.717m-2.488 1.509a9 9 0 0 0 -.519 13.208" /> <path d="M15.536 16.536a5 5 0 0 0 -3.536 -8.536m-3 1a5 5 0 0 0 -.535 7.536" /> <path d="M12 12a1 1 0 1 0 1 1" /> <path d="M3 3l18 18" /> </svg>"##;
const BROWSER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8h16" /> <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M8 4v4" /> </svg>"##;
const BROWSER_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1l0 -14" /> <path d="M4 8h16" /> <path d="M8 4v4" /> <path d="M9.5 14.5l1.5 1.5l3 -3" /> </svg>"##;
const BROWSER_MAXIMIZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8h8" /> <path d="M20 11.5v6.5a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h6.5" /> <path d="M8 4v4" /> <path d="M16 8l5 -5" /> <path d="M21 7.5v-4.5h-4.5" /> </svg>"##;
const BROWSER_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8h16" /> <path d="M12 20h-6a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v9" /> <path d="M8 4v4" /> <path d="M16 19h6" /> </svg>"##;
const BROWSER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h11a1 1 0 0 1 1 1v11m-.288 3.702a1 1 0 0 1 -.712 .298h-14a1 1 0 0 1 -1 -1v-14c0 -.276 .112 -.526 .293 -.707" /> <path d="M4 8h4m4 0h8" /> <path d="M3 3l18 18" /> </svg>"##;
const BROWSER_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8h16" /> <path d="M12 20h-6a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6" /> <path d="M8 4v4" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const BROWSER_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8h16" /> <path d="M12.5 20h-6.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v7" /> <path d="M8 4v4" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const BROWSER_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1l0 -14" /> <path d="M4 8h16" /> <path d="M8 4v4" /> <path d="M10 16l4 -4" /> <path d="M14 16l-4 -4" /> </svg>"##;
const CALCULATOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -14" /> <path d="M8 8a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1l0 -1" /> <path d="M8 14l0 .01" /> <path d="M12 14l0 .01" /> <path d="M16 14l0 .01" /> <path d="M8 17l0 .01" /> <path d="M12 17l0 .01" /> <path d="M16 17l0 .01" /> </svg>"##;
const CALCULATOR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.823 19.824a2 2 0 0 1 -1.823 1.176h-12a2 2 0 0 1 -2 -2v-14c0 -.295 .064 -.575 .178 -.827m2.822 -1.173h11a2 2 0 0 1 2 2v11" /> <path d="M10 10h-1a1 1 0 0 1 -1 -1v-1m3 -1h4a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1" /> <path d="M8 14v.01" /> <path d="M12 14v.01" /> <path d="M8 17v.01" /> <path d="M12 17v.01" /> <path d="M16 17v.01" /> <path d="M3 3l18 18" /> </svg>"##;
const CARDBOARDS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8v8.5a2.5 2.5 0 0 0 2.5 2.5h1.06a3 3 0 0 0 2.34 -1.13l1.54 -1.92a2 2 0 0 1 3.12 0l1.54 1.92a3 3 0 0 0 2.34 1.13h1.06a2.5 2.5 0 0 0 2.5 -2.5v-8.5a2 2 0 0 0 -2 -2h-14a2 2 0 0 0 -2 2" /> <path d="M7 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M15 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const CARDBOARDS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.96 16.953c.026 -.147 .04 -.298 .04 -.453v-8.5a2 2 0 0 0 -2 -2h-9m-4 0h-1a2 2 0 0 0 -2 2v8.5a2.5 2.5 0 0 0 2.5 2.5h1.06a3 3 0 0 0 2.34 -1.13l1.54 -1.92a2 2 0 0 1 3.12 0l1.54 1.92a3 3 0 0 0 2.34 1.13h1.06c.155 0 .307 -.014 .454 -.041" /> <path d="M7 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M16.714 12.7a1 1 0 0 0 -1.417 -1.411l1.417 1.41" /> <path d="M3 3l18 18" /> </svg>"##;
const CELL_SIGNAL_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 20h-15.269a.731 .731 0 0 1 -.517 -1.249l14.537 -14.537a.731 .731 0 0 1 1.249 .517v15.269" /> </svg>"##;
const CELL_SIGNAL_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 20h-15.269a.731 .731 0 0 1 -.517 -1.249l14.537 -14.537a.731 .731 0 0 1 1.249 .517v15.269" /> <path d="M8 20v-5" /> </svg>"##;
const CELL_SIGNAL_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 20h-15.269a.731 .731 0 0 1 -.517 -1.249l14.537 -14.537a.731 .731 0 0 1 1.249 .517v15.269" /> <path d="M12 20v-9" /> </svg>"##;
const CELL_SIGNAL_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 20h-15.269a.731 .731 0 0 1 -.517 -1.249l14.537 -14.537a.731 .731 0 0 1 1.249 .517v15.269" /> <path d="M16 7v13" /> </svg>"##;
const CELL_SIGNAL_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 20h-15.269a.731 .731 0 0 1 -.517 -1.249l14.537 -14.537a.731 .731 0 0 1 1.249 .517v15.269" /> <path d="M16 7v13" /> <path d="M12 20v-9" /> <path d="M8 20v-5" /> </svg>"##;
const CELL_SIGNAL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 20h-15.269a.731 .731 0 0 1 -.517 -1.249l7.265 -7.264m2 -2l5.272 -5.272a.731 .731 0 0 1 1.249 .517v11.269" /> <path d="M3 3l18 18" /> </svg>"##;
const CIRCUIT_AMMETER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M5 12h-3" /> <path d="M19 12h3" /> <path d="M10 14v-3c0 -1.036 .895 -2 2 -2s2 .964 2 2v3" /> <path d="M14 12h-4" /> </svg>"##;
const CIRCUIT_BATTERY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 12h4" /> <path d="M18 12h4" /> <path d="M18 5v14" /> <path d="M14 9v6" /> <path d="M10 5v14" /> <path d="M6 9v6" /> </svg>"##;
const CIRCUIT_BULB_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 12h5" /> <path d="M17 12h5" /> <path d="M7 12a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M8.5 8.5l7 7" /> <path d="M15.5 8.5l-7 7" /> </svg>"##;
const CIRCUIT_CAPACITOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12h-8" /> <path d="M2 12h8" /> <path d="M10 7v10" /> <path d="M14 7v10" /> </svg>"##;
const CIRCUIT_CAPACITOR_POLARIZED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12h-8" /> <path d="M2 12h8" /> <path d="M10 7v10" /> <path d="M14 7v10" /> <path d="M17 5h4" /> <path d="M19 3v4" /> </svg>"##;
const CIRCUIT_CELL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 12h8" /> <path d="M14 12h8" /> <path d="M10 5v14" /> <path d="M14 9v6" /> </svg>"##;
const CIRCUIT_CELL_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 12h9" /> <path d="M15 12h7" /> <path d="M11 5v14" /> <path d="M15 9v6" /> <path d="M3 5h4" /> <path d="M5 3v4" /> </svg>"##;
const CIRCUIT_CHANGEOVER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 12h2" /> <path d="M20 7h2" /> <path d="M4 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 7a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M20 17h2" /> <path d="M16 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7.5 10.5l8.5 -3.5" /> </svg>"##;
const CIRCUIT_DIODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12h-6" /> <path d="M2 12h6" /> <path d="M8 7l8 5l-8 5l0 -10" /> <path d="M16 7v10" /> </svg>"##;
const CIRCUIT_DIODE_ZENER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12h-6" /> <path d="M2 12h6" /> <path d="M8 7l8 5l-8 5l0 -10" /> <path d="M14 7h2v10h2" /> </svg>"##;
const CIRCUIT_GROUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 13v-8" /> <path d="M4 13h16" /> <path d="M7 16h10" /> <path d="M10 19h4" /> </svg>"##;
const CIRCUIT_GROUND_DIGITAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 13v-10" /> <path d="M12 21l-6 -8h12l-6 8" /> </svg>"##;
const CIRCUIT_INDUCTOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 14h3v-2a2 2 0 1 1 4 0v2v-1.5a2.5 2.5 0 1 1 5 0v1.5v-1.5a2.5 2.5 0 1 1 5 0v1.5h3" /> </svg>"##;
const CIRCUIT_MOTOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M5 12h-3" /> <path d="M19 12h3" /> <path d="M10 14v-4l2 2l2 -2v4" /> </svg>"##;
const CIRCUIT_PUSHBUTTON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 17h2" /> <path d="M20 17h2" /> <path d="M4 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6 11h12" /> <path d="M12 11v-6" /> </svg>"##;
const CIRCUIT_RESISTOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 12h2l2 -5l3 10l3 -10l3 10l3 -10l1.5 5h2.5" /> </svg>"##;
const CIRCUIT_SWITCH_CLOSED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 12h2" /> <path d="M20 12h2" /> <path d="M4 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M8 12h8" /> </svg>"##;
const CIRCUIT_SWITCH_OPEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 12h2" /> <path d="M20 12h2" /> <path d="M4 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7.5 10.5l7.5 -5.5" /> </svg>"##;
const CIRCUIT_VOLTMETER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M5 12h-3" /> <path d="M19 12h3" /> <path d="M10 10l2 4l2 -4" /> </svg>"##;
const CLOUD_COMPUTING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.657 16c-2.572 0 -4.657 -2.007 -4.657 -4.483c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.913 0 3.464 1.56 3.464 3.486c0 1.927 -1.551 3.487 -3.465 3.487h-11.878" /> <path d="M12 16v5" /> <path d="M16 16v4a1 1 0 0 0 1 1h4" /> <path d="M8 16v4a1 1 0 0 1 -1 1h-4" /> </svg>"##;
const CLOUD_DATA_CONNECTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 9.897c0 -1.714 1.46 -3.104 3.26 -3.104c.275 -1.22 1.255 -2.215 2.572 -2.611c1.317 -.397 2.77 -.134 3.811 .69c1.042 .822 1.514 2.08 1.239 3.3h.693a2.42 2.42 0 0 1 2.425 2.414a2.42 2.42 0 0 1 -2.425 2.414h-8.315c-1.8 0 -3.26 -1.39 -3.26 -3.103" /> <path d="M12 13v3" /> <path d="M10 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M14 18h7" /> <path d="M3 18h7" /> </svg>"##;
const CPU_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 6a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v12a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1l0 -12" /> <path d="M9 9h6v6h-6l0 -6" /> <path d="M3 10h2" /> <path d="M3 14h2" /> <path d="M10 3v2" /> <path d="M14 3v2" /> <path d="M21 10h-2" /> <path d="M21 14h-2" /> <path d="M14 21v-2" /> <path d="M10 21v-2" /> </svg>"##;
const CPU_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 6a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v12a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1l0 -12" /> <path d="M8 10v-2h2m6 6v2h-2m-4 0h-2v-2m8 -4v-2h-2" /> <path d="M3 10h2" /> <path d="M3 14h2" /> <path d="M10 3v2" /> <path d="M14 3v2" /> <path d="M21 10h-2" /> <path d="M21 14h-2" /> <path d="M14 21v-2" /> <path d="M10 21v-2" /> </svg>"##;
const CPU_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h9a1 1 0 0 1 1 1v9m-.292 3.706a1 1 0 0 1 -.708 .294h-12a1 1 0 0 1 -1 -1v-12c0 -.272 .108 -.518 .284 -.698" /> <path d="M13 9h2v2m0 4h-6v-6" /> <path d="M3 10h2" /> <path d="M3 14h2" /> <path d="M10 3v2" /> <path d="M14 3v2" /> <path d="M21 10h-2" /> <path d="M21 14h-2" /> <path d="M14 21v-2" /> <path d="M10 21v-2" /> <path d="M3 3l18 18" /> </svg>"##;
const CSV_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1" /> <path d="M17 8l2 8l2 -8" /> <path d="M7 10a2 2 0 1 0 -4 0v4a2 2 0 1 0 4 0" /> </svg>"##;
const DEVICE_3D_CAMERA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 8a2 2 0 0 1 2 -2a2 2 0 0 1 2 2a2 2 0 0 1 -2 2a2 2 0 0 1 -2 -2" /> <path d="M8 6a3 3 0 0 1 3 -3h4a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-4a3 3 0 0 1 -3 -3v-12" /> <path d="M13 14v2" /> </svg>"##;
const DEVICE_3D_LENS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.005 14.64a3.98 3.98 0 0 0 .995 -2.64" /> <path d="M12 4v16" /> <path d="M15 5v14a7 7 0 0 0 0 -14" /> <path d="M9 5v14a7 7 0 0 1 0 -14" /> </svg>"##;
const DEVICE_AIRPODS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4a4 4 0 0 1 4 3.8l0 .2v10.5a1.5 1.5 0 0 1 -3 0v-6.5h-1a4 4 0 0 1 -4 -3.8l0 -.2a4 4 0 0 1 4 -4" /> <path d="M18 4a4 4 0 0 0 -4 3.8l0 .2v10.5a1.5 1.5 0 0 0 3 0v-6.5h1a4 4 0 0 0 4 -3.8l0 -.2a4 4 0 0 0 -4 -4" /> </svg>"##;
const DEVICE_AIRPODS_CASE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 10h-18" /> <path d="M3 8a4 4 0 0 1 4 -4h10a4 4 0 0 1 4 4v8a4 4 0 0 1 -4 4h-10a4 4 0 0 1 -4 -4l0 -8" /> <path d="M7 10v1.5a1.5 1.5 0 0 0 1.5 1.5h7a1.5 1.5 0 0 0 1.5 -1.5v-1.5" /> </svg>"##;
const DEVICE_AIRTAG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12a8 8 0 1 0 16 0a8 8 0 0 0 -16 0" /> <path d="M9 15v.01" /> <path d="M15 15a6 6 0 0 0 -6 -6" /> <path d="M12 15a3 3 0 0 0 -3 -3" /> </svg>"##;
const DEVICE_ANALYTICS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1l0 -10" /> <path d="M7 20l10 0" /> <path d="M9 16l0 4" /> <path d="M15 16l0 4" /> <path d="M8 12l3 -3l2 2l3 -3" /> </svg>"##;
const DEVICE_AUDIO_TAPE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10" /> <path d="M3 17l4 -3h10l4 3" /> <path d="M7 9.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M16 9.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> </svg>"##;
const DEVICE_CAMERA_PHONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 8.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> <path d="M13 7h-8a2 2 0 0 0 -2 2v7a2 2 0 0 0 2 2h13a2 2 0 0 0 2 -2v-2" /> <path d="M17 15v-1" /> </svg>"##;
const DEVICE_CCTV_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1l0 -2" /> <path d="M8 14a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M19 7v7a7 7 0 0 1 -14 0v-7" /> <path d="M12 14l.01 0" /> </svg>"##;
const DEVICE_CCTV_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7h-3a1 1 0 0 1 -1 -1v-2c0 -.275 .11 -.523 .29 -.704m3.71 -.296h13a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-9" /> <path d="M10.36 10.35a4 4 0 1 0 5.285 5.3" /> <path d="M19 7v7c0 .321 -.022 .637 -.064 .947m-1.095 2.913a7 7 0 0 1 -12.841 -3.86l0 -7" /> <path d="M12 14h.01" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICE_COMPUTER_CAMERA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M9 10a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M8 16l-2.091 3.486a1 1 0 0 0 .857 1.514h10.468a1 1 0 0 0 .857 -1.514l-2.091 -3.486" /> </svg>"##;
const DEVICE_COMPUTER_CAMERA_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 14h-10a4 4 0 0 1 -4 -4a4 4 0 0 1 4 -4h10a4 4 0 0 1 4 4a4 4 0 0 1 -4 4" /> <path d="M15 14h-6v4h6v-4" /> <path d="M17 18h-10" /> <path d="M12 10.02v.01" /> </svg>"##;
const DEVICE_COMPUTER_CAMERA_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.15 6.153a7 7 0 0 0 9.696 9.696m2 -2a7 7 0 0 0 -9.699 -9.695" /> <path d="M9.13 9.122a3 3 0 0 0 3.743 3.749m2 -2a3 3 0 0 0 -3.737 -3.736" /> <path d="M8 16l-2.091 3.486a1 1 0 0 0 .857 1.514h10.468a1 1 0 0 0 .857 -1.514l-2.091 -3.486" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICE_DESKTOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1v-10" /> <path d="M7 20h10" /> <path d="M9 16v4" /> <path d="M15 16v4" /> </svg>"##;
const DEVICE_DESKTOP_ANALYTICS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1l0 -10" /> <path d="M7 20h10" /> <path d="M9 16v4" /> <path d="M15 16v4" /> <path d="M9 12v-4" /> <path d="M12 12v-1" /> <path d="M15 12v-2" /> <path d="M12 12v-1" /> </svg>"##;
const DEVICE_DESKTOP_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.5 16h-10.5a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v7.5" /> <path d="M7 20h6" /> <path d="M9 16v4" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const DEVICE_DESKTOP_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 16h-8.5a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v7.5" /> <path d="M7 20h5" /> <path d="M9 16v4" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const DEVICE_DESKTOP_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 16h-8a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8" /> <path d="M15 19l2 2l4 -4" /> <path d="M7 20h4" /> <path d="M9 16v4" /> </svg>"##;
const DEVICE_DESKTOP_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 16h-8.5a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8" /> <path d="M7 20h4" /> <path d="M9 16v4" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const DEVICE_DESKTOP_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 16h-8a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v7" /> <path d="M7 20h5" /> <path d="M9 16v4" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const DEVICE_DESKTOP_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 16h-9a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v5.5" /> <path d="M7 20h6.5" /> <path d="M9 16v4" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const DEVICE_DESKTOP_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 16h-9.5a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v7.5" /> <path d="M7 20h5" /> <path d="M9 16v4" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const DEVICE_DESKTOP_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 16h-11a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v7" /> <path d="M7 20h8" /> <path d="M9 16v4" /> <path d="M15 16v4" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const DEVICE_DESKTOP_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16h-6a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v6" /> <path d="M7 20h3.5" /> <path d="M9 16v4" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const DEVICE_DESKTOP_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 16h-9.5a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v10" /> <path d="M7 20h5" /> <path d="M9 16v4" /> <path d="M16 19h6" /> </svg>"##;
const DEVICE_DESKTOP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h12a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1m-4 0h-12a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1" /> <path d="M7 20h10" /> <path d="M9 16v4" /> <path d="M15 16v4" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICE_DESKTOP_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 16h-9a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8" /> <path d="M17 17v5" /> <path d="M21 17v5" /> <path d="M7 20h6" /> <path d="M9 16v4" /> </svg>"##;
const DEVICE_DESKTOP_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 16h-8.5a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v6" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> <path d="M7 20h5" /> <path d="M9 16v4" /> </svg>"##;
const DEVICE_DESKTOP_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 16h-9.5a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v7.5" /> <path d="M7 20h5" /> <path d="M9 16v4" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const DEVICE_DESKTOP_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 16h-9.5a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v6.5" /> <path d="M7 20h8" /> <path d="M9 16v4" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const DEVICE_DESKTOP_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 16h-7.5a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v6.5" /> <path d="M7 20h4" /> <path d="M9 16v4" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const DEVICE_DESKTOP_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 16h-8.5a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8" /> <path d="M7 20h5.5" /> <path d="M9 16v4" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const DEVICE_DESKTOP_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16h-6a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v6.5" /> <path d="M7 20h3.5" /> <path d="M9 16v4" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const DEVICE_DESKTOP_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 16h-9.5a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v7.5" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> <path d="M7 20h5" /> <path d="M9 16v4" /> </svg>"##;
const DEVICE_DESKTOP_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 16h-9a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8" /> <path d="M7 20h6.5" /> <path d="M9 16v4" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const DEVICE_FLOPPY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 4h10l4 4v10a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M14 4l0 4l-6 0l0 -4" /> </svg>"##;
const DEVICE_GAMEPAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 8a2 2 0 0 1 2 -2h16a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-16a2 2 0 0 1 -2 -2l0 -8" /> <path d="M6 12h4m-2 -2v4" /> <path d="M15 11l0 .01" /> <path d="M18 13l0 .01" /> </svg>"##;
const DEVICE_GAMEPAD_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5h3.5a5 5 0 0 1 0 10h-5.5l-4.015 4.227a2.3 2.3 0 0 1 -3.923 -2.035l1.634 -8.173a5 5 0 0 1 4.904 -4.019h3.4" /> <path d="M14 15l4.07 4.284a2.3 2.3 0 0 0 3.925 -2.023l-1.6 -8.232" /> <path d="M8 9v2" /> <path d="M7 10h2" /> <path d="M14 10h2" /> </svg>"##;
const DEVICE_GAMEPAD_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12l-3 -3h-2a1 1 0 0 0 -1 1v4a1 1 0 0 0 1 1h2l3 -3" /> <path d="M15 12l3 -3h2a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-2l-3 -3" /> <path d="M12 15l-3 3v2a1 1 0 0 0 1 1h4a1 1 0 0 0 1 -1v-2l-3 -3" /> <path d="M12 9l-3 -3v-2a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v2l-3 3" /> </svg>"##;
const DEVICE_HEART_MONITOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M4 9h6l1 -2l2 4l1 -2h6" /> <path d="M4 14h16" /> <path d="M14 17v.01" /> <path d="M17 17v.01" /> </svg>"##;
const DEVICE_IMAC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v12a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1v-12" /> <path d="M3 13h18" /> <path d="M8 21h8" /> <path d="M10 17l-.5 4" /> <path d="M14 17l.5 4" /> </svg>"##;
const DEVICE_IMAC_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 17h-9.5a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8.5" /> <path d="M3 13h13" /> <path d="M8 21h5.5" /> <path d="M10 17l-.5 4" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const DEVICE_IMAC_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 17h-8.5a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8" /> <path d="M3 13h12.5" /> <path d="M8 21h4.5" /> <path d="M10 17l-.5 4" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const DEVICE_IMAC_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 17h-7.5a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v9" /> <path d="M3 13h18" /> <path d="M8 21h3.5" /> <path d="M10 17l-.5 4" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const DEVICE_IMAC_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 17h-7.5a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v9" /> <path d="M3 13h18" /> <path d="M8 21h3.5" /> <path d="M10 17l-.5 4" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const DEVICE_IMAC_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17h-8a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8" /> <path d="M3 13h13" /> <path d="M8 21h4" /> <path d="M10 17l-.5 4" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const DEVICE_IMAC_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 17h-9a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v6.5" /> <path d="M3 13h11" /> <path d="M8 21h5" /> <path d="M10 17l-.5 4" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const DEVICE_IMAC_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 17h-8.5a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8.5" /> <path d="M3 13h13" /> <path d="M8 21h4.5" /> <path d="M10 17l-.5 4" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const DEVICE_IMAC_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 17h-11a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8.5" /> <path d="M3 13h13" /> <path d="M8 21h7" /> <path d="M10 17l-.5 4" /> <path d="M14 17l.5 4" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const DEVICE_IMAC_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 17h-6a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v7" /> <path d="M3 13h9" /> <path d="M8 21h3.5" /> <path d="M10 17l-.5 4" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const DEVICE_IMAC_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 17h-8.5a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v11" /> <path d="M3 13h18" /> <path d="M8 21h4.5" /> <path d="M10 17l-.5 4" /> <path d="M16 19h6" /> </svg>"##;
const DEVICE_IMAC_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h13a1 1 0 0 1 1 1v12c0 .28 -.115 .532 -.3 .713m-3.7 .287h-13a1 1 0 0 1 -1 -1v-12c0 -.276 .112 -.526 .293 -.707" /> <path d="M3 13h10m4 0h4" /> <path d="M8 21h8" /> <path d="M10 17l-.5 4" /> <path d="M14 17l.5 4" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICE_IMAC_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 17h-9a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v9" /> <path d="M3 13h18" /> <path d="M8 21h5" /> <path d="M10 17l-.5 4" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const DEVICE_IMAC_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 17h-8a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v7.5" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> <path d="M3 13h11" /> <path d="M8 21h4.5" /> <path d="M10 17l-.5 4" /> </svg>"##;
const DEVICE_IMAC_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 17h-8.5a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8.5" /> <path d="M3 13h13.5" /> <path d="M8 21h4.5" /> <path d="M10 17l-.5 4" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const DEVICE_IMAC_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 17h-10a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v7.5" /> <path d="M3 13h11.5" /> <path d="M8 21h7" /> <path d="M10 17l-.5 4" /> <path d="M14 17l.5 4" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const DEVICE_IMAC_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 17h-7a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8" /> <path d="M3 13h10" /> <path d="M8 21h4" /> <path d="M10 17l-.5 4" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const DEVICE_IMAC_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 17h-8.5a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v9" /> <path d="M3 13h18" /> <path d="M8 21h4" /> <path d="M10 17l-.5 4" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const DEVICE_IMAC_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 17h-6a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v7.5" /> <path d="M3 13h10" /> <path d="M8 21h3" /> <path d="M10 17l-.5 4" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const DEVICE_IMAC_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 17h-8.5a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8.5" /> <path d="M3 13h13" /> <path d="M8 21h4.5" /> <path d="M10 17l-.5 4" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const DEVICE_IMAC_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 17h-9a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v9" /> <path d="M3 13h18" /> <path d="M8 21h5" /> <path d="M10 17l-.5 4" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const DEVICE_IPAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 3a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2l12 0" /> <path d="M9 18h6" /> </svg>"##;
const DEVICE_IPAD_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 21h-7.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v7" /> <path d="M9 18h4" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const DEVICE_IPAD_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v7" /> <path d="M9 18h3" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const DEVICE_IPAD_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-5.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v8" /> <path d="M9 18h2" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const DEVICE_IPAD_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-5.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v8" /> <path d="M9 18h2" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const DEVICE_IPAD_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-6a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6.5" /> <path d="M9 18h3" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const DEVICE_IPAD_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-7a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v5" /> <path d="M9 18h4" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const DEVICE_IPAD_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v7" /> <path d="M9 18h3" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const DEVICE_IPAD_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21h-9a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v7" /> <path d="M9 18h6" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const DEVICE_IPAD_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-5.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6" /> <path d="M9 18h1" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-12" /> <path d="M9 17h6" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 20h-8a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v6.5" /> <path d="M9 17h4.5" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v6.5" /> <path d="M9 17h3.5" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 20h-6a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v7" /> <path d="M15 19l2 2l4 -4" /> <path d="M9 17h2.5" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 20h-6a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v7" /> <path d="M9 17h2.5" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v6" /> <path d="M9 17h3" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 20h-8a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4.5" /> <path d="M9 17h4" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v6.5" /> <path d="M9 17h3.5" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 20h-10a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v6" /> <path d="M9 17h6" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.5 20h-5.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v5" /> <path d="M9 17h1" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v9" /> <path d="M9 17h3.5" /> <path d="M16 19h6" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h12a2 2 0 0 1 2 2v12m-2 2h-16a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2" /> <path d="M9 17h6" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 20h-8a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v7" /> <path d="M9 17h4" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v5" /> <path d="M9 17h3" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v6.5" /> <path d="M9 17h3.5" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 20h-10a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v5" /> <path d="M9 17h4.5" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 20h-6.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v5.5" /> <path d="M9 17h2" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 20h-7.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v7" /> <path d="M9 17h3" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.5 20h-5.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v5.5" /> <path d="M9 17h1" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 20h-7a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v6.5" /> <path d="M9 17h3.5" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const DEVICE_IPAD_HORIZONTAL_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 20h-8.5a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v7" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> <path d="M9 17h4" /> </svg>"##;
const DEVICE_IPAD_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v10" /> <path d="M9 18h3" /> <path d="M16 19h6" /> </svg>"##;
const DEVICE_IPAD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 2h12a2 2 0 0 1 2 2v12m0 4a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-16" /> <path d="M9 19h6" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICE_IPAD_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-7a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v8" /> <path d="M9 18h4" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const DEVICE_IPAD_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6" /> <path d="M9 18h3" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const DEVICE_IPAD_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v7" /> <path d="M9 18h3" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const DEVICE_IPAD_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21h-9a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6" /> <path d="M9 18h5" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const DEVICE_IPAD_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-5.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v6" /> <path d="M9 18h2" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const DEVICE_IPAD_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-6a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v8" /> <path d="M9 18h3.5" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const DEVICE_IPAD_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 21h-5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v5.5" /> <path d="M9 18h1" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const DEVICE_IPAD_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 18h3" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> <path d="M13.5 21h-6.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v7" /> </svg>"##;
const DEVICE_IPAD_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> <path d="M13 21h-7a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v9" /> <path d="M9 18h4" /> </svg>"##;
const DEVICE_LANDLINE_PHONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 3h-2a2 2 0 0 0 -2 2v14a2 2 0 0 0 2 2h2a2 2 0 0 0 2 -2v-14a2 2 0 0 0 -2 -2" /> <path d="M16 4h-11a3 3 0 0 0 -3 3v10a3 3 0 0 0 3 3h11" /> <path d="M12 8h-6v3h6l0 -3" /> <path d="M12 14v.01" /> <path d="M9 14v.01" /> <path d="M6 14v.01" /> <path d="M12 17v.01" /> <path d="M9 17v.01" /> <path d="M6 17v.01" /> </svg>"##;
const DEVICE_LAPTOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19l18 0" /> <path d="M5 7a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1l0 -8" /> </svg>"##;
const DEVICE_LAPTOP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19h16" /> <path d="M10 6h8a1 1 0 0 1 1 1v8m-3 1h-10a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICE_MOBILE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 5a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-14" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> </svg>"##;
const DEVICE_MOBILE_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 21h-5.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v7" /> <path d="M19 16l-2 3h4l-2 3" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> </svg>"##;
const DEVICE_MOBILE_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-4a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v7" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> </svg>"##;
const DEVICE_MOBILE_CHARGING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 5a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -14" /> <path d="M11 4h2" /> <path d="M12 9.5l-1 2.5h2l-1 2.5" /> </svg>"##;
const DEVICE_MOBILE_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-3.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v9.5" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const DEVICE_MOBILE_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-3.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> </svg>"##;
const DEVICE_MOBILE_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-4a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v6.5" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const DEVICE_MOBILE_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v5" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const DEVICE_MOBILE_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-4.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v7" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const DEVICE_MOBILE_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21h-7a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v7" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const DEVICE_MOBILE_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-3.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v6" /> <path d="M11 4h2" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const DEVICE_MOBILE_MESSAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 3h10v8h-3l-4 2v-2h-3l0 -8" /> <path d="M15 16v4a1 1 0 0 1 -1 1h-8a1 1 0 0 1 -1 -1v-14a1 1 0 0 1 1 -1h2" /> <path d="M10 18v.01" /> </svg>"##;
const DEVICE_MOBILE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-4.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v10" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> <path d="M16 19h6" /> </svg>"##;
const DEVICE_MOBILE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.159 3.185c.256 -.119 .54 -.185 .841 -.185h8a2 2 0 0 1 2 2v9m0 4v1a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-13" /> <path d="M11 4h2" /> <path d="M3 3l18 18" /> <path d="M12 17v.01" /> </svg>"##;
const DEVICE_MOBILE_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8" /> <path d="M17 17v5" /> <path d="M21 17v5" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> </svg>"##;
const DEVICE_MOBILE_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-4.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v6" /> <path d="M11 4h2" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> <path d="M12 17v.01" /> </svg>"##;
const DEVICE_MOBILE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-4.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v7" /> <path d="M16 19h6" /> <path d="M19 16v6" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> </svg>"##;
const DEVICE_MOBILE_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21h-7a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v6" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> </svg>"##;
const DEVICE_MOBILE_ROTATED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -8" /> <path d="M20 11v2" /> <path d="M7 12h-.01" /> </svg>"##;
const DEVICE_MOBILE_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-4a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v6" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> </svg>"##;
const DEVICE_MOBILE_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-4a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8" /> <path d="M11 4h2" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> <path d="M12 17v.01" /> </svg>"##;
const DEVICE_MOBILE_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 21h-3a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v5" /> <path d="M11 4h2" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const DEVICE_MOBILE_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-4.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v7" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> </svg>"##;
const DEVICE_MOBILE_VIBRATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -14" /> <path d="M8 4l2 0" /> <path d="M9 17l0 .01" /> <path d="M21 6l-2 3l2 3l-2 3l2 3" /> </svg>"##;
const DEVICE_MOBILE_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> <path d="M11 4h2" /> <path d="M12 17v.01" /> </svg>"##;
const DEVICE_NINTENDO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 20v-16h-3a4 4 0 0 0 -4 4v8a4 4 0 0 0 4 4h3" /> <path d="M14 20v-16h3a4 4 0 0 1 4 4v8a4 4 0 0 1 -4 4h-3" /> <path d="M16.5 15.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" fill="currentColor" /> <path d="M5.5 8.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" fill="currentColor" /> </svg>"##;
const DEVICE_NINTENDO_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.713 4.718a4 4 0 0 0 -1.713 3.282v8a4 4 0 0 0 4 4h3v-10m0 -4v-2h-2" /> <path d="M14 10v-6h3a4 4 0 0 1 4 4v8c0 .308 -.035 .608 -.1 .896m-1.62 2.39a3.982 3.982 0 0 1 -2.28 .714h-3v-6" /> <path d="M5.5 8.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICE_REMOTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 10a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 5a2 2 0 0 1 2 -2h6a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2l0 -14" /> <path d="M12 3v2" /> <path d="M10 15v.01" /> <path d="M10 18v.01" /> <path d="M14 18v.01" /> <path d="M14 15v.01" /> </svg>"##;
const DEVICE_SCREEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14" /> <path d="M15 7l-6 10" /> <path d="M15 14l-1.75 3" /> <path d="M10.75 7l-1.75 3" /> </svg>"##;
const DEVICE_SD_CARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 21h10a2 2 0 0 0 2 -2v-14a2 2 0 0 0 -2 -2h-6.172a2 2 0 0 0 -1.414 .586l-3.828 3.828a2 2 0 0 0 -.586 1.414v10.172a2 2 0 0 0 2 2" /> <path d="M13 6v2" /> <path d="M16 6v2" /> <path d="M10 7v1" /> </svg>"##;
const DEVICE_SIM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 3h8.5l4.5 4.5v12.5a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1" /> <path d="M9 11h3v6" /> <path d="M15 17v.01" /> <path d="M15 14v.01" /> <path d="M15 11v.01" /> <path d="M9 14v.01" /> <path d="M9 17v.01" /> </svg>"##;
const DEVICE_SIM_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 3h8.5l4.5 4.5v12.5a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1" /> <path d="M10 11l2 -2v8" /> </svg>"##;
const DEVICE_SIM_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 3h8.5l4.5 4.5v12.5a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1" /> <path d="M10 9h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> </svg>"##;
const DEVICE_SIM_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 3h8.5l4.5 4.5v12.5a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1" /> <path d="M10 9h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const DEVICE_SPEAKER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -14" /> <path d="M9 14a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M12 7l0 .01" /> </svg>"##;
const DEVICE_SPEAKER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h10a2 2 0 0 1 2 2v10m0 4a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14" /> <path d="M11.114 11.133a3 3 0 1 0 3.754 3.751" /> <path d="M12 7v.01" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICE_TABLET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v16a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1v-16" /> <path d="M11 17a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> </svg>"##;
const DEVICE_TABLET_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 21h-7.5a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v8" /> <path d="M19 16l-2 3h4l-2 3" /> <path d="M11 17a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> </svg>"##;
const DEVICE_TABLET_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v8" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> <path d="M11 17a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> </svg>"##;
const DEVICE_TABLET_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-5.5a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v9.5" /> <path d="M12.314 16.05a1 1 0 0 0 -1.042 1.635" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const DEVICE_TABLET_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-5.5a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v9" /> <path d="M12.344 16.06a1 1 0 0 0 -1.07 1.627" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const DEVICE_TABLET_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-6a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v7.5" /> <path d="M12 16a1 1 0 0 0 0 2" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const DEVICE_TABLET_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-7a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v6" /> <path d="M11 17a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const DEVICE_TABLET_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v8" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> <path d="M11 17a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> </svg>"##;
const DEVICE_TABLET_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21h-9a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v8" /> <path d="M11 17a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const DEVICE_TABLET_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-5.5a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v7" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const DEVICE_TABLET_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v11" /> <path d="M12.872 16.51a1 1 0 1 0 -.872 1.49" /> <path d="M16 19h6" /> </svg>"##;
const DEVICE_TABLET_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h11a1 1 0 0 1 1 1v11m0 4v1a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1v-15" /> <path d="M11 17a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICE_TABLET_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-7a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v9.5" /> <path d="M17 17v5" /> <path d="M21 17v5" /> <path d="M11 17a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> </svg>"##;
const DEVICE_TABLET_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v7" /> <path d="M12 16a1 1 0 0 0 0 2" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const DEVICE_TABLET_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v8" /> <path d="M16 19h6" /> <path d="M19 16v6" /> <path d="M11 17a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> </svg>"##;
const DEVICE_TABLET_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21h-9a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v7" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> <path d="M11 17a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> </svg>"##;
const DEVICE_TABLET_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-5.5a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v7" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const DEVICE_TABLET_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21h-6a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v9" /> <path d="M12.57 16.178a1 1 0 1 0 .016 1.633" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const DEVICE_TABLET_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 21h-5a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v6" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const DEVICE_TABLET_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-6.5a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v8" /> <path d="M12.906 16.576a1 1 0 1 0 -.906 1.424" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const DEVICE_TABLET_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-7a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v9.5" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> <path d="M11 17a1 1 0 1 0 2 0a1 1 0 0 0 -2 0" /> </svg>"##;
const DEVICE_TV_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v9a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -9" /> <path d="M16 3l-4 4l-4 -4" /> </svg>"##;
const DEVICE_TV_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 7h8a2 2 0 0 1 2 2v8m-1.178 2.824c-.25 .113 -.529 .176 -.822 .176h-14a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h2" /> <path d="M16 3l-4 4l-4 -4" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICE_TV_OLD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v9a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -9" /> <path d="M16 3l-4 4l-4 -4" /> <path d="M15 7v13" /> <path d="M18 15v.01" /> <path d="M18 12v.01" /> </svg>"##;
const DEVICE_USB_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8h10v8a5 5 0 0 1 -10 0l0 -8" /> <path d="M9 8v-5h6v5" /> </svg>"##;
const DEVICE_VISION_PRO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 7c1.143 0 2.235 .035 3.275 .104c1.017 .068 1.95 .207 2.798 .42c.813 .203 1.52 .505 2.119 .909a3.903 3.903 0 0 1 1.328 1.531c.326 .657 .48 1.48 .48 2.466c0 1.006 -.189 1.91 -.574 2.707c-.375 .779 -.886 1.396 -1.537 1.848a3.696 3.696 0 0 1 -2.16 .66c-.509 0 -.97 -.068 -1.382 -.21a5.84 5.84 0 0 1 -1.17 -.548a18.45 18.45 0 0 1 -1.045 -.695a9.104 9.104 0 0 0 -1.001 -.63a2.376 2.376 0 0 0 -1.13 -.301c-.373 0 -.75 .097 -1.132 .3c-.316 .17 -.65 .38 -1 .63c-.322 .23 -.67 .462 -1.047 .695a5.78 5.78 0 0 1 -1.168 .548c-.413 .142 -.872 .21 -1.378 .21a3.706 3.706 0 0 1 -2.165 -.659c-.651 -.452 -1.162 -1.07 -1.537 -1.848c-.385 -.798 -.574 -1.7 -.574 -2.709c-.004 -.98 .15 -1.802 .477 -2.46a3.897 3.897 0 0 1 1.33 -1.531c.6 -.403 1.307 -.704 2.12 -.907a16.088 16.088 0 0 1 2.8 -.423c1.04 -.071 2.13 -.107 3.273 -.107" /> </svg>"##;
const DEVICE_WATCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 9a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v6a3 3 0 0 1 -3 3h-6a3 3 0 0 1 -3 -3v-6" /> <path d="M9 18v3h6v-3" /> <path d="M9 6v-3h6v3" /> </svg>"##;
const DEVICE_WATCH_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 18h-4a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v3" /> <path d="M9 18v3h4.5" /> <path d="M9 6v-3h6v3" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const DEVICE_WATCH_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18h-3a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v3" /> <path d="M9 18v3h3" /> <path d="M9 6v-3h6v3" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const DEVICE_WATCH_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 18h-2a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v5.5" /> <path d="M9 18v3h2.5" /> <path d="M9 6v-3h6v3" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const DEVICE_WATCH_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 18h-2a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v4" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> <path d="M9 18v3h3" /> <path d="M9 6v-3h6v3" /> </svg>"##;
const DEVICE_WATCH_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18h-3a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v2.5" /> <path d="M9 18v3h3" /> <path d="M9 6v-3h6v3" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const DEVICE_WATCH_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 18h-4a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v1" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> <path d="M9 18v3h4" /> <path d="M9 6v-3h6v3" /> </svg>"##;
const DEVICE_WATCH_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18h-3a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v3" /> <path d="M9 18v3h3.5" /> <path d="M9 6v-3h6v3" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const DEVICE_WATCH_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 18h-6a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v3" /> <path d="M9 18v3h6v-3" /> <path d="M9 6v-3h6v3" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const DEVICE_WATCH_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 18h-1a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v2" /> <path d="M9 18v3h2.5" /> <path d="M9 6v-3h6v3" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const DEVICE_WATCH_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18h-3a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v6" /> <path d="M9 18v3h3.5" /> <path d="M9 6v-3h6v3" /> <path d="M16 19h6" /> </svg>"##;
const DEVICE_WATCH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 6h5a3 3 0 0 1 3 3v5m-.89 3.132a2.99 2.99 0 0 1 -2.11 .868h-6a3 3 0 0 1 -3 -3v-6c0 -.817 .327 -1.559 .857 -2.1" /> <path d="M9 18v3h6v-3" /> <path d="M9 5v-2h6v3" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICE_WATCH_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 18h-4a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v4" /> <path d="M9 18v3h4" /> <path d="M9 6v-3h6v3" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const DEVICE_WATCH_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18h-3a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v2" /> <path d="M9 18v3h3.5" /> <path d="M9 6v-3h6v3" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const DEVICE_WATCH_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18h-3a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v3" /> <path d="M16 19h6" /> <path d="M19 16v6" /> <path d="M9 18v3h3.5" /> <path d="M9 6v-3h6v3" /> </svg>"##;
const DEVICE_WATCH_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 18h-5a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v2" /> <path d="M9 18v3h6v-2" /> <path d="M9 6v-3h6v3" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const DEVICE_WATCH_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 18h-2a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v2" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> <path d="M9 18v3h3" /> <path d="M9 6v-3h6v3" /> </svg>"##;
const DEVICE_WATCH_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 18h-3.5a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v4" /> <path d="M9 18v3h3" /> <path d="M9 6v-3h6v3" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const DEVICE_WATCH_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 18h-1a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v1" /> <path d="M9 18v3h2" /> <path d="M9 6v-3h6v3" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const DEVICE_WATCH_STATS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 9a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v6a3 3 0 0 1 -3 3h-6a3 3 0 0 1 -3 -3l0 -6" /> <path d="M9 18v3h6v-3" /> <path d="M9 6v-3h6v3" /> <path d="M9 14v-4" /> <path d="M12 14v-1" /> <path d="M15 14v-3" /> </svg>"##;
const DEVICE_WATCH_STATS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 9a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v6a3 3 0 0 1 -3 3h-6a3 3 0 0 1 -3 -3l0 -6" /> <path d="M9 18v3h6v-3" /> <path d="M9 6v-3h6v3" /> <path d="M12 10a2 2 0 1 0 2 2" /> </svg>"##;
const DEVICE_WATCH_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18h-3a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v3" /> <path d="M9 18v3h3.5" /> <path d="M9 6v-3h6v3" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const DEVICE_WATCH_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 18h-4a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v4" /> <path d="M9 18v3h4" /> <path d="M9 6v-3h6v3" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const DEVICES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 9a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1v-10" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h9" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15h-6a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h6" /> <path d="M13 5a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1l0 -14" /> <path d="M7 19l3 0" /> <path d="M17 8l0 .01" /> <path d="M16 16a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M9 15l0 4" /> </svg>"##;
const DEVICES_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19v-10a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v3.5" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h9" /> <path d="M19 16l-2 3h4l-2 3" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 15.5v-6.5a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v3.5" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h8" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 15.5v-6.5a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v4" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h7" /> <path d="M16 9h2" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const DEVICES_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 15.5v-6.5a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v4m0 6a1 1 0 0 1 -1 1" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h7" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 14.5v-5.5a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v3" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h8" /> <path d="M16 9h2" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const DEVICES_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19v-10a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v1.5" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h9" /> <path d="M16 9h2" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const DEVICES_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 16.5v-7.5a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v3.5" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h8" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 20h-1a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v3.5" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h9" /> <path d="M16 9h2" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const DEVICES_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 12v-3a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v2" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h6" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 16.5v-7.5a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v6" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h8" /> <path d="M16 19h6" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 9a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v8m-1 3h-6a1 1 0 0 1 -1 -1v-6" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-9m-4 0a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h9" /> <path d="M16 9h2" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICES_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19v-10a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v4" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h9" /> <path d="M17 17v5" /> <path d="M21 17v5" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_PC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5h6v14h-6l0 -14" /> <path d="M12 9h10v7h-10l0 -7" /> <path d="M14 19h6" /> <path d="M17 16v3" /> <path d="M6 13v.01" /> <path d="M6 16v.01" /> </svg>"##;
const DEVICES_PC_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 9v10h-6v-14h2" /> <path d="M13 9h9v7h-2m-4 0h-4v-4" /> <path d="M14 19h5" /> <path d="M17 17v2" /> <path d="M6 13v.01" /> <path d="M6 16v.01" /> <path d="M3 3l18 18" /> </svg>"##;
const DEVICES_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 14v-5a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v2" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h8" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 16.5v-7.5a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v3.5" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h8" /> <path d="M16 9h2" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const DEVICES_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 20h-1a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v2" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h9" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 13v-4a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v2.5" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h7" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 15v-6a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v4" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h9" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 13v-4a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v2.5" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h5.5" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 16.5v-7.5a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v3.5" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h8" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> <path d="M16 9h2" /> </svg>"##;
const DEVICES_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 20a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v4" /> <path d="M18 8v-3a1 1 0 0 0 -1 -1h-13a1 1 0 0 0 -1 1v12a1 1 0 0 0 1 1h9" /> <path d="M16 9h2" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const DIALPAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 3h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1" /> <path d="M18 3h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1" /> <path d="M11 3h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1" /> <path d="M4 10h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1" /> <path d="M18 10h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1" /> <path d="M11 10h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1" /> <path d="M11 17h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1" /> </svg>"##;
const DIALPAD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7h-4v-4" /> <path d="M17 3h4v4h-4l0 -4" /> <path d="M10 6v-3h4v4h-3" /> <path d="M3 10h4v4h-4l0 -4" /> <path d="M17 13v-3h4v4h-3" /> <path d="M14 14h-4v-4" /> <path d="M10 17h4v4h-4l0 -4" /> <path d="M3 3l18 18" /> </svg>"##;
const DISC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M7 12a5 5 0 0 1 5 -5" /> <path d="M12 17a5 5 0 0 0 5 -5" /> </svg>"##;
const DISC_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.044 16.04a9 9 0 0 0 -12.082 -12.085m-2.333 1.688a9 9 0 0 0 6.371 15.357c2.491 0 4.73 -1 6.36 -2.631" /> <path d="M11.298 11.288a1 1 0 1 0 1.402 1.427" /> <path d="M7 12c0 -1.38 .559 -2.629 1.462 -3.534m2.607 -1.38c.302 -.056 .613 -.086 .931 -.086" /> <path d="M12 17a4.985 4.985 0 0 0 3.551 -1.48m1.362 -2.587c.057 -.302 .087 -.614 .087 -.933" /> <path d="M3 3l18 18" /> </svg>"##;
const DUAL_SCREEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4l8 3v15l-8 -3l0 -15" /> <path d="M13 19h6v-15h-14" /> </svg>"##;
const EARPHONE_BLUETOOTH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.57 12.77a6.9 6.9 0 0 1 -.57 -2.77a7 7 0 0 1 14 0" /> <path d="M9 16l-1 1" /> <path d="M10.83 19.83l6.36 -6.37a1 1 0 0 0 0 -1.41l-4.19 -4.24a1 1 0 0 0 -1.41 0l-6.42 6.36a4 4 0 0 0 0 5.66a4 4 0 0 0 5.66 0" /> </svg>"##;
const ERROR_404_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8v3a1 1 0 0 0 1 1h3" /> <path d="M7 8v8" /> <path d="M17 8v3a1 1 0 0 0 1 1h3" /> <path d="M21 8v8" /> <path d="M10 10v4a2 2 0 1 0 4 0v-4a2 2 0 1 0 -4 0" /> </svg>"##;
const ERROR_404_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8v3a1 1 0 0 0 1 1h3" /> <path d="M7 8v8" /> <path d="M17 8v3a1 1 0 0 0 1 1h3" /> <path d="M21 8v8" /> <path d="M10 10v4a2 2 0 1 0 4 0m0 -4a2 2 0 0 0 -2 -2" /> <path d="M3 3l18 18" /> </svg>"##;
const FRIDGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -14" /> <path d="M5 10h14" /> <path d="M9 13v3" /> <path d="M9 6v1" /> </svg>"##;
const FRIDGE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h10a2 2 0 0 1 2 2v10m0 4a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-14" /> <path d="M5 10h5m4 0h5" /> <path d="M9 13v3" /> <path d="M3 3l18 18" /> </svg>"##;
const GIF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> <path d="M12 8v8" /> <path d="M16 12h3" /> <path d="M20 8h-4v8" /> </svg>"##;
const GIT_BRANCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 8l0 8" /> <path d="M9 18h6a2 2 0 0 0 2 -2v-5" /> <path d="M14 14l3 -3l3 3" /> </svg>"##;
const GIT_BRANCH_DELETED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 8v8" /> <path d="M9 18h6a2 2 0 0 0 2 -2v-5" /> <path d="M14 14l3 -3l3 3" /> <path d="M15 4l4 4" /> <path d="M15 8l4 -4" /> </svg>"##;
const GIT_CHERRY_PICK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M7 3v6" /> <path d="M7 15v6" /> <path d="M13 7h2.5l1.5 5l-1.5 5h-2.5" /> <path d="M17 12h3" /> </svg>"##;
const GIT_COMMIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M12 3l0 6" /> <path d="M12 15l0 6" /> </svg>"##;
const GIT_COMPARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M11 6h5a2 2 0 0 1 2 2v8" /> <path d="M14 9l-3 -3l3 -3" /> <path d="M13 18h-5a2 2 0 0 1 -2 -2v-8" /> <path d="M10 15l3 3l-3 3" /> </svg>"##;
const GIT_FORK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 8v2a2 2 0 0 0 2 2h6a2 2 0 0 0 2 -2v-2" /> <path d="M12 12l0 4" /> </svg>"##;
const GIT_MERGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 8l0 8" /> <path d="M7 8a4 4 0 0 0 4 4h4" /> </svg>"##;
const GIT_PULL_REQUEST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6 8l0 8" /> <path d="M11 6h5a2 2 0 0 1 2 2v8" /> <path d="M14 9l-3 -3l3 -3" /> </svg>"##;
const GIT_PULL_REQUEST_CLOSED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6 8v8" /> <path d="M18 11v5" /> <path d="M16 4l4 4m0 -4l-4 4" /> </svg>"##;
const GIT_PULL_REQUEST_CONFLICT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 18a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M11 6h5a2 2 0 0 1 2 2v8" /> <path d="M4 18a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M6 11v5" /> <path d="M4 4l4 4" /> <path d="M8 4l-4 4" /> </svg>"##;
const GIT_PULL_REQUEST_DRAFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6 8v8" /> <path d="M18 11h.01" /> <path d="M18 6h.01" /> </svg>"##;
const HAMMER_DRILL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 15v6" /> <path d="M16 5h4" /> <path d="M8 5h-4" /> <path d="M15 11h-6a1 1 0 0 1 -1 -1v-6a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1" /> <path d="M14 11h-4v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-3" /> </svg>"##;
const HTML_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 16v-8l2 5l2 -5v8" /> <path d="M1 16v-8" /> <path d="M5 8v8" /> <path d="M1 12h4" /> <path d="M7 8h4" /> <path d="M9 8v8" /> <path d="M20 8v8h3" /> </svg>"##;
const HTTP_CONNECT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 10a2 2 0 1 0 -4 0v4a2 2 0 1 0 4 0" /> <path d="M17 16v-8l4 8v-8" /> <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> </svg>"##;
const HTTP_CONNECT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 10a2 2 0 1 0 -4 0v4a2 2 0 1 0 4 0" /> <path d="M17 13v-5l4 8v-8" /> <path d="M14 14a2 2 0 1 1 -4 0v-4m2 -2a2 2 0 0 1 2 2" /> <path d="M3 3l18 18" /> </svg>"##;
const HTTP_DELETE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2l-2 0" /> <path d="M14 8h-4v8h4" /> <path d="M10 12h2.5" /> <path d="M17 8v8h4" /> </svg>"##;
const HTTP_DELETE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2l-2 0" /> <path d="M14 8h-2m-2 2v6h4" /> <path d="M10 12h2" /> <path d="M17 8v5m3 3h1" /> <path d="M3 3l18 18" /> </svg>"##;
const HTTP_GET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> <path d="M14 8h-4v8h4" /> <path d="M10 12h2.5" /> <path d="M17 8h4" /> <path d="M19 8v8" /> </svg>"##;
const HTTP_GET_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> <path d="M14 8h-2m-2 2v6h4" /> <path d="M10 12h2" /> <path d="M17 8h4" /> <path d="M19 8v7" /> <path d="M3 3l18 18" /> </svg>"##;
const HTTP_HEAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 16v-8" /> <path d="M7 8v8" /> <path d="M3 12h4" /> <path d="M14 8h-4v8h4" /> <path d="M10 12h2.5" /> <path d="M17 16v-6a2 2 0 1 1 4 0v6" /> <path d="M17 13h4" /> </svg>"##;
const HTTP_HEAD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 16v-8" /> <path d="M7 8v8" /> <path d="M3 12h4" /> <path d="M14 8h-2m-2 2v6h4" /> <path d="M10 12h2" /> <path d="M17 13v-3a2 2 0 1 1 4 0v6" /> <path d="M17 13h4" /> <path d="M3 3l18 18" /> </svg>"##;
const HTTP_OPTIONS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8" /> <path d="M17 8h4" /> <path d="M19 8v8" /> </svg>"##;
const HTTP_OPTIONS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M10 12h2m2 -2a2 2 0 0 0 -2 -2m-2 2v6" /> <path d="M17 8h4" /> <path d="M19 8v7" /> <path d="M3 3l18 18" /> </svg>"##;
const HTTP_PATCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h2a2 2 0 1 0 0 -4h-2v8" /> <path d="M10 16v-6a2 2 0 1 1 4 0v6" /> <path d="M10 13h4" /> <path d="M17 8h4" /> <path d="M19 8v8" /> </svg>"##;
const HTTP_PATCH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h2a2 2 0 1 0 0 -4h-2v8" /> <path d="M10 16v-6m2 -2a2 2 0 0 1 2 2m0 4v2" /> <path d="M10 13h3" /> <path d="M17 8h4" /> <path d="M19 8v7" /> <path d="M3 3l18 18" /> </svg>"##;
const HTTP_POST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h2a2 2 0 1 0 0 -4h-2v8" /> <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M17 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1" /> </svg>"##;
const HTTP_POST_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h2a2 2 0 1 0 0 -4h-2v8" /> <path d="M12 8a2 2 0 0 1 2 2m0 4a2 2 0 1 1 -4 0v-4" /> <path d="M20 16a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1" /> <path d="M3 3l18 18" /> </svg>"##;
const HTTP_PUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h2a2 2 0 1 0 0 -4h-2v8" /> <path d="M17 8h4" /> <path d="M19 8v8" /> <path d="M10 8v6a2 2 0 1 0 4 0v-6" /> </svg>"##;
const HTTP_PUT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h2a2 2 0 1 0 0 -4h-2v8" /> <path d="M17 8h4" /> <path d="M19 8v8" /> <path d="M10 10v4a2 2 0 1 0 4 0m0 -4v-2" /> <path d="M3 3l18 18" /> </svg>"##;
const HTTP_QUE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M6 15l1 1" /> <path d="M21 8h-4v8h4" /> <path d="M17 12h2.5" /> <path d="M10 8v6a2 2 0 1 0 4 0v-6" /> </svg>"##;
const HTTP_QUE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M6 15l1 1" /> <path d="M21 8h-4v8h4" /> <path d="M17 12h2.5" /> <path d="M10 10v4a2 2 0 1 0 4 0m0 -4v-2" /> <path d="M3 3l18 18" /> </svg>"##;
const HTTP_TRACE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8h4" /> <path d="M5 8v8" /> <path d="M10 12h2a2 2 0 1 0 0 -4h-2v8" /> <path d="M14 16l-3 -4" /> <path d="M17 16v-6a2 2 0 1 1 4 0v6" /> <path d="M17 13h4" /> </svg>"##;
const HTTP_TRACE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8h4" /> <path d="M5 8v8" /> <path d="M10 12h2m2 -2a2 2 0 0 0 -2 -2m-2 2v6" /> <path d="M14 16l-3 -4" /> <path d="M17 13v-3a2 2 0 1 1 4 0v6" /> <path d="M17 13h4" /> <path d="M3 3l18 18" /> </svg>"##;
const JPG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> <path d="M10 16v-8h2a2 2 0 1 1 0 4h-2" /> <path d="M3 8h4v6a2 2 0 0 1 -2 2h-1.5a.5 .5 0 0 1 -.5 -.5v-.5" /> </svg>"##;
const JSON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 16v-8l3 8v-8" /> <path d="M15 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M1 8h3v6.5a1.5 1.5 0 0 1 -3 0v-.5" /> <path d="M7 15a1 1 0 0 0 1 1h1a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-1a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h1a1 1 0 0 1 1 1" /> </svg>"##;
const KEYBOARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 8a2 2 0 0 1 2 -2h16a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-16a2 2 0 0 1 -2 -2l0 -8" /> <path d="M6 10l0 .01" /> <path d="M10 10l0 .01" /> <path d="M14 10l0 .01" /> <path d="M18 10l0 .01" /> <path d="M6 14l0 .01" /> <path d="M18 14l0 .01" /> <path d="M10 14l4 .01" /> </svg>"##;
const KEYBOARD_HIDE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 5a2 2 0 0 1 2 -2h16a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-16a2 2 0 0 1 -2 -2l0 -8" /> <path d="M6 7l0 .01" /> <path d="M10 7l0 .01" /> <path d="M14 7l0 .01" /> <path d="M18 7l0 .01" /> <path d="M6 11l0 .01" /> <path d="M18 11l0 .01" /> <path d="M10 11l4 0" /> <path d="M10 21l2 -2l2 2" /> </svg>"##;
const KEYBOARD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 18h-14a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h2m4 0h10a2 2 0 0 1 2 2v8c0 .554 -.226 1.056 -.59 1.418" /> <path d="M6 10l0 .01" /> <path d="M10 10l0 .01" /> <path d="M14 10l0 .01" /> <path d="M18 10l0 .01" /> <path d="M6 14l0 .01" /> <path d="M18 14l0 .01" /> <path d="M10 14l4 0" /> <path d="M3 3l18 18" /> </svg>"##;
const KEYBOARD_SHOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 5a2 2 0 0 1 2 -2h16a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-16a2 2 0 0 1 -2 -2l0 -8" /> <path d="M6 7l0 .01" /> <path d="M10 7l0 .01" /> <path d="M14 7l0 .01" /> <path d="M18 7l0 .01" /> <path d="M6 11l0 .01" /> <path d="M18 11l0 .01" /> <path d="M10 11l4 0" /> <path d="M10 19l2 2l2 -2" /> </svg>"##;
const LAWN_MOWER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 11h5.38a1 1 0 0 1 .9 .55l.72 1.45h5a1 1 0 0 1 1 1v2" /> <path d="M3 4h1.13a1 1 0 0 1 1 .86l1.59 11.14" /> <path d="M17 18h-8" /> <path d="M9 18a2 2 0 1 1 -4 0a2 2 0 0 1 4 0" /> <path d="M21 18a2 2 0 1 1 -4 0a2 2 0 0 1 4 0" /> </svg>"##;
const LOAD_BALANCER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 13a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M11 20a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M12 16v3" /> <path d="M12 10v-7" /> <path d="M9 6l3 -3l3 3" /> <path d="M12 10v-7" /> <path d="M9 6l3 -3l3 3" /> <path d="M14.894 12.227l6.11 -2.224" /> <path d="M17.159 8.21l3.845 1.793l-1.793 3.845" /> <path d="M9.101 12.214l-6.075 -2.211" /> <path d="M6.871 8.21l-3.845 1.793l1.793 3.845" /> </svg>"##;
const MOUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 7a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v10a4 4 0 0 1 -4 4h-4a4 4 0 0 1 -4 -4l0 -10" /> <path d="M12 7l0 4" /> </svg>"##;
const MOUSE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 7a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v10a4 4 0 0 1 -4 4h-4a4 4 0 0 1 -4 -4l0 -10" /> <path d="M12 3v7" /> <path d="M6 10h12" /> </svg>"##;
const MOUSE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.733 3.704a3.982 3.982 0 0 1 2.267 -.704h4a4 4 0 0 1 4 4v7m-.1 3.895a4 4 0 0 1 -3.9 3.105h-4a4 4 0 0 1 -4 -4v-10c0 -.3 .033 -.593 .096 -.874" /> <path d="M12 7v1" /> <path d="M3 3l18 18" /> </svg>"##;
const NETWORK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 9a6 6 0 1 0 12 0a6 6 0 0 0 -12 0" /> <path d="M12 3c1.333 .333 2 2.333 2 6s-.667 5.667 -2 6" /> <path d="M12 3c-1.333 .333 -2 2.333 -2 6s.667 5.667 2 6" /> <path d="M6 9h12" /> <path d="M3 20h7" /> <path d="M14 20h7" /> <path d="M10 20a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 15v3" /> </svg>"##;
const NETWORK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.528 6.536a6 6 0 0 0 7.942 7.933m2.247 -1.76a6 6 0 0 0 -8.427 -8.425" /> <path d="M12 3c1.333 .333 2 2.333 2 6c0 .337 -.006 .66 -.017 .968m-.55 3.473c-.333 .884 -.81 1.403 -1.433 1.559" /> <path d="M12 3c-.936 .234 -1.544 1.29 -1.822 3.167m-.16 3.838c.116 3.029 .776 4.695 1.982 4.995" /> <path d="M6 9h3m4 0h5" /> <path d="M3 20h7" /> <path d="M14 20h7" /> <path d="M10 20a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 15v3" /> <path d="M3 3l18 18" /> </svg>"##;
const NFC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 20a3 3 0 0 1 -3 -3v-11l5 5" /> <path d="M13 4a3 3 0 0 1 3 3v11l-5 -5" /> <path d="M4 7a3 3 0 0 1 3 -3h10a3 3 0 0 1 3 3v10a3 3 0 0 1 -3 3h-10a3 3 0 0 1 -3 -3l0 -10" /> </svg>"##;
const NFC_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 20a3 3 0 0 1 -3 -3v-9" /> <path d="M13 4a3 3 0 0 1 3 3v5m0 4v2l-5 -5" /> <path d="M8 4h9a3 3 0 0 1 3 3v9m-.873 3.116a2.99 2.99 0 0 1 -2.127 .884h-10a3 3 0 0 1 -3 -3v-10c0 -.83 .337 -1.582 .882 -2.125" /> <path d="M3 3l18 18" /> </svg>"##;
const PDF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8v8h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2h-2" /> <path d="M3 12h2a2 2 0 1 0 0 -4h-2v8" /> <path d="M17 12h3" /> <path d="M21 8h-4v8" /> </svg>"##;
const PHONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2a16 16 0 0 1 -15 -15a2 2 0 0 1 2 -2" /> </svg>"##;
const PHONE_CALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2a16 16 0 0 1 -15 -15a2 2 0 0 1 2 -2" /> <path d="M15 7a2 2 0 0 1 2 2" /> <path d="M15 3a6 6 0 0 1 6 6" /> </svg>"##;
const PHONE_CALLING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2a16 16 0 0 1 -15 -15a2 2 0 0 1 2 -2" /> <path d="M15 7l0 .01" /> <path d="M18 7l0 .01" /> <path d="M21 7l0 .01" /> </svg>"##;
const PHONE_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2a16 16 0 0 1 -15 -15a2 2 0 0 1 2 -2" /> <path d="M15 6l2 2l4 -4" /> </svg>"##;
const PHONE_INCOMING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2a16 16 0 0 1 -15 -15a2 2 0 0 1 2 -2" /> <path d="M15 9l5 -5" /> <path d="M15 5l0 4l4 0" /> </svg>"##;
const PHONE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21l18 -18" /> <path d="M5.831 14.161a15.946 15.946 0 0 1 -2.831 -8.161a2 2 0 0 1 2 -2h4l2 5l-2.5 1.5c.108 .22 .223 .435 .345 .645m1.751 2.277c.843 .84 1.822 1.544 2.904 2.078l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2a15.963 15.963 0 0 1 -10.344 -4.657" /> </svg>"##;
const PHONE_OUTGOING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2c-8.072 -.49 -14.51 -6.928 -15 -15a2 2 0 0 1 2 -2" /> <path d="M15 5h6" /> <path d="M18.5 7.5l2.5 -2.5l-2.5 -2.5" /> </svg>"##;
const PHONE_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2c-8.072 -.49 -14.51 -6.928 -15 -15a2 2 0 0 1 2 -2" /> <path d="M17 3v5" /> <path d="M21 3v5" /> </svg>"##;
const PHONE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2a16 16 0 0 1 -15 -15a2 2 0 0 1 2 -2" /> <path d="M15 6h6m-3 -3v6" /> </svg>"##;
const PHONE_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.584 19.225a16 16 0 0 1 -8.584 -13.225a2 2 0 0 1 2 -2h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l.65 .26" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const PHONE_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2a16 16 0 0 1 -15 -15a2 2 0 0 1 2 -2" /> <path d="M16 4l4 4m0 -4l-4 4" /> </svg>"##;
const PLAYSTATION_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 0 0 9 -9a9 9 0 0 0 -9 -9a9 9 0 0 0 -9 9a9 9 0 0 0 9 9" /> <path d="M7.5 12a4.5 4.5 0 1 0 9 0a4.5 4.5 0 1 0 -9 0" /> </svg>"##;
const PLAYSTATION_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 0 0 9 -9a9 9 0 0 0 -9 -9a9 9 0 0 0 -9 9a9 9 0 0 0 9 9" /> <path d="M8 9a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1l0 -6" /> </svg>"##;
const PLAYSTATION_TRIANGLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 0 0 9 -9a9 9 0 0 0 -9 -9a9 9 0 0 0 -9 9a9 9 0 0 0 9 9" /> <path d="M7.5 15h9l-4.5 -8l-4.5 8" /> </svg>"##;
const PLAYSTATION_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 0 0 9 -9a9 9 0 0 0 -9 -9a9 9 0 0 0 -9 9a9 9 0 0 0 9 9" /> <path d="M8.5 8.5l7 7" /> <path d="M8.5 15.5l7 -7" /> </svg>"##;
const PLUG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.785 6l8.215 8.215l-2.054 2.054a5.81 5.81 0 1 1 -8.215 -8.215l2.054 -2.054" /> <path d="M4 20l3.5 -3.5" /> <path d="M15 4l-3.5 3.5" /> <path d="M20 9l-3.5 3.5" /> </svg>"##;
const PLUG_CONNECTED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 12l5 5l-1.5 1.5a3.536 3.536 0 1 1 -5 -5l1.5 -1.5" /> <path d="M17 12l-5 -5l1.5 -1.5a3.536 3.536 0 1 1 5 5l-1.5 1.5" /> <path d="M3 21l2.5 -2.5" /> <path d="M18.5 5.5l2.5 -2.5" /> <path d="M10 11l-2 2" /> <path d="M13 14l-2 2" /> </svg>"##;
const PLUG_CONNECTED_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 16l-4 4" /> <path d="M7 12l5 5l-1.5 1.5a3.536 3.536 0 1 1 -5 -5l1.5 -1.5" /> <path d="M17 12l-5 -5l1.5 -1.5a3.536 3.536 0 1 1 5 5l-1.5 1.5" /> <path d="M3 21l2.5 -2.5" /> <path d="M18.5 5.5l2.5 -2.5" /> <path d="M10 11l-2 2" /> <path d="M13 14l-2 2" /> <path d="M16 16l4 4" /> </svg>"##;
const PLUG_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.123 16.092l-.177 .177a5.81 5.81 0 1 1 -8.215 -8.215l.159 -.159" /> <path d="M4 20l3.5 -3.5" /> <path d="M15 4l-3.5 3.5" /> <path d="M20 9l-3.5 3.5" /> <path d="M3 3l18 18" /> </svg>"##;
const PLUG_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.55 17.733a5.806 5.806 0 0 1 -7.356 -4.052a5.81 5.81 0 0 1 1.537 -5.627l2.054 -2.054l7.165 7.165" /> <path d="M4 20l3.5 -3.5" /> <path d="M15 4l-3.5 3.5" /> <path d="M20 9l-3.5 3.5" /> <path d="M16 16l4 4" /> <path d="M20 16l-4 4" /> </svg>"##;
const PNG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> <path d="M3 16v-8h2a2 2 0 1 1 0 4h-2" /> <path d="M10 16v-8l4 8v-8" /> </svg>"##;
const POWER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 6a7.75 7.75 0 1 0 10 0" /> <path d="M12 4l0 8" /> </svg>"##;
const PRINTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 17h2a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2h-14a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2" /> <path d="M17 9v-4a2 2 0 0 0 -2 -2h-6a2 2 0 0 0 -2 2v4" /> <path d="M7 15a2 2 0 0 1 2 -2h6a2 2 0 0 1 2 2v4a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2l0 -4" /> </svg>"##;
const PRINTER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.412 16.416c.363 -.362 .588 -.863 .588 -1.416v-4a2 2 0 0 0 -2 -2h-6m-4 0h-4a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2" /> <path d="M17 9v-4a2 2 0 0 0 -2 -2h-6c-.551 0 -1.05 .223 -1.412 .584m-.588 3.416v2" /> <path d="M17 17v2a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2v-4a2 2 0 0 1 2 -2h4" /> <path d="M3 3l18 18" /> </svg>"##;
const QRCODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M7 17l0 .01" /> <path d="M14 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M7 7l0 .01" /> <path d="M4 15a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M17 7l0 .01" /> <path d="M14 14l3 0" /> <path d="M20 14l0 .01" /> <path d="M14 14l0 3" /> <path d="M14 20l3 0" /> <path d="M17 17l3 0" /> <path d="M20 17l0 3" /> </svg>"##;
const QRCODE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h1a1 1 0 0 1 1 1v1m-.297 3.711a1 1 0 0 1 -.703 .289h-4a1 1 0 0 1 -1 -1v-4c0 -.275 .11 -.524 .29 -.705" /> <path d="M7 17v.01" /> <path d="M14 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M7 7v.01" /> <path d="M4 15a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -4" /> <path d="M17 7v.01" /> <path d="M20 14v.01" /> <path d="M14 14v3" /> <path d="M14 20h3" /> <path d="M3 3l18 18" /> </svg>"##;
const ROUTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 15a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -4" /> <path d="M17 17l0 .01" /> <path d="M13 17l0 .01" /> <path d="M15 13l0 -2" /> <path d="M11.75 8.75a4 4 0 0 1 6.5 0" /> <path d="M8.5 6.5a8 8 0 0 1 13 0" /> </svg>"##;
const ROUTER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 13h2a2 2 0 0 1 2 2v2m-.588 3.417c-.362 .36 -.861 .583 -1.412 .583h-14a2 2 0 0 1 -2 -2v-4a2 2 0 0 1 2 -2h8" /> <path d="M17 17v.01" /> <path d="M13 17v.01" /> <path d="M12.226 8.2a4 4 0 0 1 6.024 .55" /> <path d="M9.445 5.407a8 8 0 0 1 12.055 1.093" /> <path d="M3 3l18 18" /> </svg>"##;
const SCREEN_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12v3a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h9" /> <path d="M7 20l10 0" /> <path d="M9 16l0 4" /> <path d="M15 16l0 4" /> <path d="M17 4h4v4" /> <path d="M16 9l5 -5" /> </svg>"##;
const SCREEN_SHARE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12v3a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h9" /> <path d="M7 20l10 0" /> <path d="M9 16l0 4" /> <path d="M15 16l0 4" /> <path d="M17 8l4 -4m-4 0l4 4" /> </svg>"##;
const SERVER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v2a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3" /> <path d="M3 15a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v2a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3l0 -2" /> <path d="M7 8l0 .01" /> <path d="M7 16l0 .01" /> </svg>"##;
const SERVER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v2a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-2" /> <path d="M3 15a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v2a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3l0 -2" /> <path d="M7 8l0 .01" /> <path d="M7 16l0 .01" /> <path d="M11 8h6" /> <path d="M11 16h6" /> </svg>"##;
const SERVER_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v2a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3" /> <path d="M15 20h-9a3 3 0 0 1 -3 -3v-2a3 3 0 0 1 3 -3h12" /> <path d="M7 8v.01" /> <path d="M7 16v.01" /> <path d="M20 15l-2 3h3l-2 3" /> </svg>"##;
const SERVER_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v2a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-2" /> <path d="M12 20h-6a3 3 0 0 1 -3 -3v-2a3 3 0 0 1 3 -3h10.5" /> <path d="M16 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M18 14.5v1.5" /> <path d="M18 20v1.5" /> <path d="M21.032 16.25l-1.299 .75" /> <path d="M16.27 19l-1.3 .75" /> <path d="M14.97 16.25l1.3 .75" /> <path d="M19.733 19l1.3 .75" /> <path d="M7 8v.01" /> <path d="M7 16v.01" /> </svg>"##;
const SERVER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12h-6a3 3 0 0 1 -3 -3v-2c0 -1.083 .574 -2.033 1.435 -2.56m3.565 -.44h10a3 3 0 0 1 3 3v2a3 3 0 0 1 -3 3h-2" /> <path d="M16 12h2a3 3 0 0 1 3 3v2m-1.448 2.568a2.986 2.986 0 0 1 -1.552 .432h-12a3 3 0 0 1 -3 -3v-2a3 3 0 0 1 3 -3h6" /> <path d="M7 8v.01" /> <path d="M7 16v.01" /> <path d="M3 3l18 18" /> </svg>"##;
const SERVER_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> <path d="M3 7a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v2a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3l0 -2" /> <path d="M12 20h-6a3 3 0 0 1 -3 -3v-2a3 3 0 0 1 3 -3h10.5" /> <path d="M7 8v.01" /> <path d="M7 16v.01" /> </svg>"##;
const SHREDDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 11a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1l0 -3" /> <path d="M17 10v-4a2 2 0 0 0 -2 -2h-6a2 2 0 0 0 -2 2v4m5 5v5m4 -5v2m-8 -2v3" /> </svg>"##;
const SIGNAL_2G_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 8h-3a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h3v-4h-1" /> <path d="M5 8h4a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-3a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h4" /> </svg>"##;
const SIGNAL_3G_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> <path d="M6 8h2.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-1.5h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1 -1.5 1.5h-2.5" /> </svg>"##;
const SIGNAL_4G_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 8v3a1 1 0 0 0 1 1h3" /> <path d="M10 8v8" /> <path d="M17 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> </svg>"##;
const SIGNAL_4G_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 12h4" /> <path d="M3 8v3a1 1 0 0 0 1 1h3" /> <path d="M7 8v8" /> <path d="M19 10v4" /> <path d="M14 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> </svg>"##;
const SIGNAL_5G_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> <path d="M6 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3v-4h4" /> </svg>"##;
const SIGNAL_6G_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> <path d="M10 9a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> </svg>"##;
const SIGNAL_E_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h-4v8h4" /> <path d="M10 12h2.5" /> </svg>"##;
const SIGNAL_G_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> </svg>"##;
const SIGNAL_H_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 16v-8" /> <path d="M14 8v8" /> <path d="M10 12h4" /> </svg>"##;
const SIGNAL_H_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 16v-8" /> <path d="M11 8v8" /> <path d="M7 12h4" /> <path d="M14 12h4" /> <path d="M16 10v4" /> </svg>"##;
const SIGNAL_LTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 8h-4v8h4" /> <path d="M17 12h2.5" /> <path d="M4 8v8h4" /> <path d="M10 8h4" /> <path d="M12 8v8" /> </svg>"##;
const SQL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8a2 2 0 0 1 2 2v4a2 2 0 1 1 -4 0v-4a2 2 0 0 1 2 -2" /> <path d="M17 8v8h4" /> <path d="M13 15l1 1" /> <path d="M3 15a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1" /> </svg>"##;
const SVG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 8h-2a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h2v-4h-1" /> <path d="M7 8h-3a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h2a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-3" /> <path d="M10 8l1.5 8h1l1.5 -8" /> </svg>"##;
const TOML_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M1.499 8h3" /> <path d="M2.999 8v8" /> <path d="M8.5 8a1.5 1.5 0 0 1 1.5 1.5v5a1.5 1.5 0 0 1 -3 0v-5a1.5 1.5 0 0 1 1.5 -1.5" /> <path d="M13 16v-8l2 5l2 -5v8" /> <path d="M20 8v8h2.5" /> </svg>"##;
const TOPOLOGY_BUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 10a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 10a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M22 10a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M2 16h20" /> <path d="M4 12v4" /> <path d="M12 12v4" /> <path d="M20 12v4" /> </svg>"##;
const TOPOLOGY_COMPLEX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M8 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M8 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M20 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M7.5 7.5l3 3" /> <path d="M6 8v8" /> <path d="M18 16v-8" /> <path d="M8 6h8" /> <path d="M16 18h-8" /> </svg>"##;
const TOPOLOGY_FULL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M8 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M8 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M20 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 8v8" /> <path d="M18 16v-8" /> <path d="M8 6h8" /> <path d="M16 18h-8" /> <path d="M7.5 7.5l9 9" /> <path d="M7.5 16.5l9 -9" /> </svg>"##;
const TOPOLOGY_FULL_HIERARCHY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M8 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M8 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M20 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 8v8" /> <path d="M18 16v-8" /> <path d="M8 6h8" /> <path d="M16 18h-8" /> <path d="M7.5 7.5l3 3" /> <path d="M13.5 13.5l3 3" /> <path d="M16.5 7.5l-3 3" /> <path d="M10.5 13.5l-3 3" /> </svg>"##;
const TOPOLOGY_RING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 20a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 4a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M22 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M13.5 5.5l5 5" /> <path d="M5.5 13.5l5 5" /> <path d="M13.5 18.5l5 -5" /> <path d="M10.5 5.5l-5 5" /> </svg>"##;
const TOPOLOGY_RING_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M7 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M21 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M7 18h10" /> <path d="M18 16l-5 -8" /> <path d="M11 8l-5 8" /> </svg>"##;
const TOPOLOGY_RING_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M20 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M20 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M8 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 8v8" /> <path d="M18 16v-8" /> <path d="M8 6h8" /> <path d="M16 18h-8" /> </svg>"##;
const TOPOLOGY_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M20 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M8 6a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M20 18a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M7.5 7.5l3 3" /> <path d="M7.5 16.5l3 -3" /> <path d="M13.5 13.5l3 3" /> <path d="M16.5 7.5l-3 3" /> </svg>"##;
const TOPOLOGY_STAR_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 20a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 4a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M22 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 12h4" /> <path d="M14 12h4" /> <path d="M12 6v4" /> <path d="M12 14v4" /> </svg>"##;
const TOPOLOGY_STAR_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 19a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M18 5a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M10 5a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M18 19a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M22 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 12h4" /> <path d="M14 12h4" /> <path d="M15 7l-2 3" /> <path d="M9 7l2 3" /> <path d="M11 14l-2 3" /> <path d="M13 14l2 3" /> </svg>"##;
const TOPOLOGY_STAR_RING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 20a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 4a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M22 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 12h4" /> <path d="M14 12h4" /> <path d="M13.5 5.5l5 5" /> <path d="M5.5 13.5l5 5" /> <path d="M13.5 18.5l5 -5" /> <path d="M10.5 5.5l-5 5" /> <path d="M12 6v4" /> <path d="M12 14v4" /> </svg>"##;
const TOPOLOGY_STAR_RING_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 20a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 4a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M22 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 12h4" /> <path d="M14 12h4" /> <path d="M12 6v4" /> <path d="M12 14v4" /> <path d="M5.5 10.5l5 -5" /> <path d="M13.5 5.5l5 5" /> <path d="M18.5 13.5l-5 5" /> <path d="M10.5 18.5l-5 -5" /> </svg>"##;
const TOPOLOGY_STAR_RING_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 19a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M18 5a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M10 5a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M18 19a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M22 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M6 12h4" /> <path d="M14 12h4" /> <path d="M15 7l-2 3" /> <path d="M9 7l2 3" /> <path d="M11 14l-2 3" /> <path d="M13 14l2 3" /> <path d="M10 5h4" /> <path d="M10 19h4" /> <path d="M17 17l2 -3" /> <path d="M19 10l-2 -3" /> <path d="M7 7l-2 3" /> <path d="M5 14l2 3" /> </svg>"##;
const TXT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8h4" /> <path d="M5 8v8" /> <path d="M17 8h4" /> <path d="M19 8v8" /> <path d="M10 8l4 8" /> <path d="M10 16l4 -8" /> </svg>"##;
const VIEWPORT_NARROW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h7l-3 -3" /> <path d="M7 15l3 -3" /> <path d="M21 12h-7l3 -3" /> <path d="M17 15l-3 -3" /> <path d="M9 6v-1a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v1" /> <path d="M9 18v1a2 2 0 0 0 2 2h2a2 2 0 0 0 2 -2v-1" /> </svg>"##;
const VIEWPORT_WIDE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 12h-7l3 -3" /> <path d="M6 15l-3 -3" /> <path d="M14 12h7l-3 -3" /> <path d="M18 15l3 -3" /> <path d="M3 6v-1a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v1" /> <path d="M3 18v1a2 2 0 0 0 2 2h14a2 2 0 0 0 2 -2v-1" /> </svg>"##;
const VINYL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 3.937a9 9 0 1 0 5 8.063" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M19 4a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M20 4l-3.5 10l-2.5 2" /> </svg>"##;
const WASH_MACHINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -14" /> <path d="M8 14a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M8 6h.01" /> <path d="M11 6h.01" /> <path d="M14 6h2" /> <path d="M8 14c1.333 -.667 2.667 -.667 4 0c1.333 .667 2.667 .667 4 0" /> </svg>"##;
const WIFI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l.01 0" /> <path d="M9.172 15.172a4 4 0 0 1 5.656 0" /> <path d="M6.343 12.343a8 8 0 0 1 11.314 0" /> <path d="M3.515 9.515c4.686 -4.687 12.284 -4.687 17 0" /> </svg>"##;
const WIFI_0_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l.01 0" /> </svg>"##;
const WIFI_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l.01 0" /> <path d="M9.172 15.172a4 4 0 0 1 5.656 0" /> </svg>"##;
const WIFI_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l.01 0" /> <path d="M9.172 15.172a4 4 0 0 1 5.656 0" /> <path d="M6.343 12.343a8 8 0 0 1 11.314 0" /> </svg>"##;
const WIFI_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l.01 0" /> <path d="M9.172 15.172a4 4 0 0 1 5.656 0" /> <path d="M6.343 12.343a7.963 7.963 0 0 1 3.864 -2.14m4.163 .155a7.965 7.965 0 0 1 3.287 2" /> <path d="M3.515 9.515a12 12 0 0 1 3.544 -2.455m3.101 -.92a12 12 0 0 1 10.325 3.374" /> <path d="M3 3l18 18" /> </svg>"##;
const XBOX_A_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 0 0 9 -9a9 9 0 0 0 -9 -9a9 9 0 0 0 -9 9a9 9 0 0 0 9 9" /> <path d="M15 16l-3 -8l-3 8" /> <path d="M14 14h-4" /> </svg>"##;
const XBOX_B_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 0 0 9 -9a9 9 0 0 0 -9 -9a9 9 0 0 0 -9 9a9 9 0 0 0 9 9" /> <path d="M13 12a2 2 0 1 1 0 4h-3v-4" /> <path d="M13 12h-3" /> <path d="M13 12a2 2 0 1 0 0 -4h-3v4" /> </svg>"##;
const XBOX_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 0 0 9 -9a9 9 0 0 0 -9 -9a9 9 0 0 0 -9 9a9 9 0 0 0 9 9" /> <path d="M9 8l6 8" /> <path d="M15 8l-6 8" /> </svg>"##;
const XBOX_Y_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21a9 9 0 0 0 9 -9a9 9 0 0 0 -9 -9a9 9 0 0 0 -9 9a9 9 0 0 0 9 9" /> <path d="M9 8l3 4" /> <path d="M15 8l-2.988 3.984l-.012 4.016" /> </svg>"##;
const ZIP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 16v-8h2a2 2 0 1 1 0 4h-2" /> <path d="M12 8v8" /> <path d="M4 8h4l-4 8h4" /> </svg>"##;

/// Devices icon variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DevicesIcon {
    AccessPoint,
    AccessPointOff,
    AirConditioning,
    AirConditioningDisabled,
    AlarmSmoke,
    Antenna,
    AntennaBars1,
    AntennaBars2,
    AntennaBars3,
    AntennaBars4,
    AntennaBars5,
    AntennaBarsOff,
    AntennaOff,
    Battery,
    Battery1,
    Battery2,
    Battery3,
    Battery4,
    BatteryCharging,
    BatteryCharging2,
    BatteryEco,
    BatteryExclamation,
    BatteryOff,
    BatterySpark,
    BatteryVertical,
    BatteryVertical1,
    BatteryVertical2,
    BatteryVertical3,
    BatteryVertical4,
    BatteryVerticalCharging,
    BatteryVerticalCharging2,
    BatteryVerticalEco,
    BatteryVerticalExclamation,
    BatteryVerticalOff,
    Binary,
    BinaryOff,
    BinaryTree,
    BinaryTree2,
    Blender,
    Bluetooth,
    BluetoothConnected,
    BluetoothOff,
    BluetoothX,
    Bmp,
    Broadcast,
    BroadcastOff,
    Browser,
    BrowserCheck,
    BrowserMaximize,
    BrowserMinus,
    BrowserOff,
    BrowserPlus,
    BrowserShare,
    BrowserX,
    Calculator,
    CalculatorOff,
    Cardboards,
    CardboardsOff,
    CellSignal1,
    CellSignal2,
    CellSignal3,
    CellSignal4,
    CellSignal5,
    CellSignalOff,
    CircuitAmmeter,
    CircuitBattery,
    CircuitBulb,
    CircuitCapacitor,
    CircuitCapacitorPolarized,
    CircuitCell,
    CircuitCellPlus,
    CircuitChangeover,
    CircuitDiode,
    CircuitDiodeZener,
    CircuitGround,
    CircuitGroundDigital,
    CircuitInductor,
    CircuitMotor,
    CircuitPushbutton,
    CircuitResistor,
    CircuitSwitchClosed,
    CircuitSwitchOpen,
    CircuitVoltmeter,
    CloudComputing,
    CloudDataConnection,
    Cpu,
    Cpu2,
    CpuOff,
    Csv,
    Device3dCamera,
    Device3dLens,
    DeviceAirpods,
    DeviceAirpodsCase,
    DeviceAirtag,
    DeviceAnalytics,
    DeviceAudioTape,
    DeviceCameraPhone,
    DeviceCctv,
    DeviceCctvOff,
    DeviceComputerCamera,
    DeviceComputerCamera2,
    DeviceComputerCameraOff,
    DeviceDesktop,
    DeviceDesktopAnalytics,
    DeviceDesktopBolt,
    DeviceDesktopCancel,
    DeviceDesktopCheck,
    DeviceDesktopCode,
    DeviceDesktopCog,
    DeviceDesktopDollar,
    DeviceDesktopDown,
    DeviceDesktopExclamation,
    DeviceDesktopHeart,
    DeviceDesktopMinus,
    DeviceDesktopOff,
    DeviceDesktopPause,
    DeviceDesktopPin,
    DeviceDesktopPlus,
    DeviceDesktopQuestion,
    DeviceDesktopSearch,
    DeviceDesktopShare,
    DeviceDesktopStar,
    DeviceDesktopUp,
    DeviceDesktopX,
    DeviceFloppy,
    DeviceGamepad,
    DeviceGamepad2,
    DeviceGamepad3,
    DeviceHeartMonitor,
    DeviceImac,
    DeviceImacBolt,
    DeviceImacCancel,
    DeviceImacCheck,
    DeviceImacCode,
    DeviceImacCog,
    DeviceImacDollar,
    DeviceImacDown,
    DeviceImacExclamation,
    DeviceImacHeart,
    DeviceImacMinus,
    DeviceImacOff,
    DeviceImacPause,
    DeviceImacPin,
    DeviceImacPlus,
    DeviceImacQuestion,
    DeviceImacSearch,
    DeviceImacShare,
    DeviceImacStar,
    DeviceImacUp,
    DeviceImacX,
    DeviceIpad,
    DeviceIpadBolt,
    DeviceIpadCancel,
    DeviceIpadCheck,
    DeviceIpadCode,
    DeviceIpadCog,
    DeviceIpadDollar,
    DeviceIpadDown,
    DeviceIpadExclamation,
    DeviceIpadHeart,
    DeviceIpadHorizontal,
    DeviceIpadHorizontalBolt,
    DeviceIpadHorizontalCancel,
    DeviceIpadHorizontalCheck,
    DeviceIpadHorizontalCode,
    DeviceIpadHorizontalCog,
    DeviceIpadHorizontalDollar,
    DeviceIpadHorizontalDown,
    DeviceIpadHorizontalExclamation,
    DeviceIpadHorizontalHeart,
    DeviceIpadHorizontalMinus,
    DeviceIpadHorizontalOff,
    DeviceIpadHorizontalPause,
    DeviceIpadHorizontalPin,
    DeviceIpadHorizontalPlus,
    DeviceIpadHorizontalQuestion,
    DeviceIpadHorizontalSearch,
    DeviceIpadHorizontalShare,
    DeviceIpadHorizontalStar,
    DeviceIpadHorizontalUp,
    DeviceIpadHorizontalX,
    DeviceIpadMinus,
    DeviceIpadOff,
    DeviceIpadPause,
    DeviceIpadPin,
    DeviceIpadPlus,
    DeviceIpadQuestion,
    DeviceIpadSearch,
    DeviceIpadShare,
    DeviceIpadStar,
    DeviceIpadUp,
    DeviceIpadX,
    DeviceLandlinePhone,
    DeviceLaptop,
    DeviceLaptopOff,
    DeviceMobile,
    DeviceMobileBolt,
    DeviceMobileCancel,
    DeviceMobileCharging,
    DeviceMobileCheck,
    DeviceMobileCode,
    DeviceMobileCog,
    DeviceMobileDollar,
    DeviceMobileDown,
    DeviceMobileExclamation,
    DeviceMobileHeart,
    DeviceMobileMessage,
    DeviceMobileMinus,
    DeviceMobileOff,
    DeviceMobilePause,
    DeviceMobilePin,
    DeviceMobilePlus,
    DeviceMobileQuestion,
    DeviceMobileRotated,
    DeviceMobileSearch,
    DeviceMobileShare,
    DeviceMobileStar,
    DeviceMobileUp,
    DeviceMobileVibration,
    DeviceMobileX,
    DeviceNintendo,
    DeviceNintendoOff,
    DeviceRemote,
    DeviceScreen,
    DeviceSdCard,
    DeviceSim,
    DeviceSim1,
    DeviceSim2,
    DeviceSim3,
    DeviceSpeaker,
    DeviceSpeakerOff,
    DeviceTablet,
    DeviceTabletBolt,
    DeviceTabletCancel,
    DeviceTabletCheck,
    DeviceTabletCode,
    DeviceTabletCog,
    DeviceTabletDollar,
    DeviceTabletDown,
    DeviceTabletExclamation,
    DeviceTabletHeart,
    DeviceTabletMinus,
    DeviceTabletOff,
    DeviceTabletPause,
    DeviceTabletPin,
    DeviceTabletPlus,
    DeviceTabletQuestion,
    DeviceTabletSearch,
    DeviceTabletShare,
    DeviceTabletStar,
    DeviceTabletUp,
    DeviceTabletX,
    DeviceTv,
    DeviceTvOff,
    DeviceTvOld,
    DeviceUsb,
    DeviceVisionPro,
    DeviceWatch,
    DeviceWatchBolt,
    DeviceWatchCancel,
    DeviceWatchCheck,
    DeviceWatchCode,
    DeviceWatchCog,
    DeviceWatchDollar,
    DeviceWatchDown,
    DeviceWatchExclamation,
    DeviceWatchHeart,
    DeviceWatchMinus,
    DeviceWatchOff,
    DeviceWatchPause,
    DeviceWatchPin,
    DeviceWatchPlus,
    DeviceWatchQuestion,
    DeviceWatchSearch,
    DeviceWatchShare,
    DeviceWatchStar,
    DeviceWatchStats,
    DeviceWatchStats2,
    DeviceWatchUp,
    DeviceWatchX,
    Devices,
    Devices2,
    DevicesBolt,
    DevicesCancel,
    DevicesCheck,
    DevicesCode,
    DevicesCog,
    DevicesDollar,
    DevicesDown,
    DevicesExclamation,
    DevicesHeart,
    DevicesMinus,
    DevicesOff,
    DevicesPause,
    DevicesPc,
    DevicesPcOff,
    DevicesPin,
    DevicesPlus,
    DevicesQuestion,
    DevicesSearch,
    DevicesShare,
    DevicesStar,
    DevicesUp,
    DevicesX,
    Dialpad,
    DialpadOff,
    Disc,
    DiscOff,
    DualScreen,
    EarphoneBluetooth,
    Error404,
    Error404Off,
    Fridge,
    FridgeOff,
    Gif,
    GitBranch,
    GitBranchDeleted,
    GitCherryPick,
    GitCommit,
    GitCompare,
    GitFork,
    GitMerge,
    GitPullRequest,
    GitPullRequestClosed,
    GitPullRequestConflict,
    GitPullRequestDraft,
    HammerDrill,
    Html,
    HttpConnect,
    HttpConnectOff,
    HttpDelete,
    HttpDeleteOff,
    HttpGet,
    HttpGetOff,
    HttpHead,
    HttpHeadOff,
    HttpOptions,
    HttpOptionsOff,
    HttpPatch,
    HttpPatchOff,
    HttpPost,
    HttpPostOff,
    HttpPut,
    HttpPutOff,
    HttpQue,
    HttpQueOff,
    HttpTrace,
    HttpTraceOff,
    Jpg,
    Json,
    Keyboard,
    KeyboardHide,
    KeyboardOff,
    KeyboardShow,
    LawnMower,
    LoadBalancer,
    Mouse,
    Mouse2,
    MouseOff,
    Network,
    NetworkOff,
    Nfc,
    NfcOff,
    Pdf,
    Phone,
    PhoneCall,
    PhoneCalling,
    PhoneCheck,
    PhoneIncoming,
    PhoneOff,
    PhoneOutgoing,
    PhonePause,
    PhonePlus,
    PhoneSpark,
    PhoneX,
    PlaystationCircle,
    PlaystationSquare,
    PlaystationTriangle,
    PlaystationX,
    Plug,
    PlugConnected,
    PlugConnectedX,
    PlugOff,
    PlugX,
    Png,
    Power,
    Printer,
    PrinterOff,
    Qrcode,
    QrcodeOff,
    Router,
    RouterOff,
    ScreenShare,
    ScreenShareOff,
    Server,
    Server2,
    ServerBolt,
    ServerCog,
    ServerOff,
    ServerSpark,
    Shredder,
    Signal2g,
    Signal3g,
    Signal4g,
    Signal4gPlus,
    Signal5g,
    Signal6g,
    SignalE,
    SignalG,
    SignalH,
    SignalHPlus,
    SignalLte,
    Sql,
    Svg,
    Toml,
    TopologyBus,
    TopologyComplex,
    TopologyFull,
    TopologyFullHierarchy,
    TopologyRing,
    TopologyRing2,
    TopologyRing3,
    TopologyStar,
    TopologyStar2,
    TopologyStar3,
    TopologyStarRing,
    TopologyStarRing2,
    TopologyStarRing3,
    Txt,
    ViewportNarrow,
    ViewportWide,
    Vinyl,
    WashMachine,
    Wifi,
    Wifi0,
    Wifi1,
    Wifi2,
    WifiOff,
    XboxA,
    XboxB,
    XboxX,
    XboxY,
    Zip,
}

impl DevicesIcon {
    /// Returns all available icons in this category.
    pub fn all() -> &'static [Self] {
        &[Self::AccessPoint, Self::AccessPointOff, Self::AirConditioning, Self::AirConditioningDisabled, Self::AlarmSmoke, Self::Antenna, Self::AntennaBars1, Self::AntennaBars2, Self::AntennaBars3, Self::AntennaBars4, Self::AntennaBars5, Self::AntennaBarsOff, Self::AntennaOff, Self::Battery, Self::Battery1, Self::Battery2, Self::Battery3, Self::Battery4, Self::BatteryCharging, Self::BatteryCharging2, Self::BatteryEco, Self::BatteryExclamation, Self::BatteryOff, Self::BatterySpark, Self::BatteryVertical, Self::BatteryVertical1, Self::BatteryVertical2, Self::BatteryVertical3, Self::BatteryVertical4, Self::BatteryVerticalCharging, Self::BatteryVerticalCharging2, Self::BatteryVerticalEco, Self::BatteryVerticalExclamation, Self::BatteryVerticalOff, Self::Binary, Self::BinaryOff, Self::BinaryTree, Self::BinaryTree2, Self::Blender, Self::Bluetooth, Self::BluetoothConnected, Self::BluetoothOff, Self::BluetoothX, Self::Bmp, Self::Broadcast, Self::BroadcastOff, Self::Browser, Self::BrowserCheck, Self::BrowserMaximize, Self::BrowserMinus, Self::BrowserOff, Self::BrowserPlus, Self::BrowserShare, Self::BrowserX, Self::Calculator, Self::CalculatorOff, Self::Cardboards, Self::CardboardsOff, Self::CellSignal1, Self::CellSignal2, Self::CellSignal3, Self::CellSignal4, Self::CellSignal5, Self::CellSignalOff, Self::CircuitAmmeter, Self::CircuitBattery, Self::CircuitBulb, Self::CircuitCapacitor, Self::CircuitCapacitorPolarized, Self::CircuitCell, Self::CircuitCellPlus, Self::CircuitChangeover, Self::CircuitDiode, Self::CircuitDiodeZener, Self::CircuitGround, Self::CircuitGroundDigital, Self::CircuitInductor, Self::CircuitMotor, Self::CircuitPushbutton, Self::CircuitResistor, Self::CircuitSwitchClosed, Self::CircuitSwitchOpen, Self::CircuitVoltmeter, Self::CloudComputing, Self::CloudDataConnection, Self::Cpu, Self::Cpu2, Self::CpuOff, Self::Csv, Self::Device3dCamera, Self::Device3dLens, Self::DeviceAirpods, Self::DeviceAirpodsCase, Self::DeviceAirtag, Self::DeviceAnalytics, Self::DeviceAudioTape, Self::DeviceCameraPhone, Self::DeviceCctv, Self::DeviceCctvOff, Self::DeviceComputerCamera, Self::DeviceComputerCamera2, Self::DeviceComputerCameraOff, Self::DeviceDesktop, Self::DeviceDesktopAnalytics, Self::DeviceDesktopBolt, Self::DeviceDesktopCancel, Self::DeviceDesktopCheck, Self::DeviceDesktopCode, Self::DeviceDesktopCog, Self::DeviceDesktopDollar, Self::DeviceDesktopDown, Self::DeviceDesktopExclamation, Self::DeviceDesktopHeart, Self::DeviceDesktopMinus, Self::DeviceDesktopOff, Self::DeviceDesktopPause, Self::DeviceDesktopPin, Self::DeviceDesktopPlus, Self::DeviceDesktopQuestion, Self::DeviceDesktopSearch, Self::DeviceDesktopShare, Self::DeviceDesktopStar, Self::DeviceDesktopUp, Self::DeviceDesktopX, Self::DeviceFloppy, Self::DeviceGamepad, Self::DeviceGamepad2, Self::DeviceGamepad3, Self::DeviceHeartMonitor, Self::DeviceImac, Self::DeviceImacBolt, Self::DeviceImacCancel, Self::DeviceImacCheck, Self::DeviceImacCode, Self::DeviceImacCog, Self::DeviceImacDollar, Self::DeviceImacDown, Self::DeviceImacExclamation, Self::DeviceImacHeart, Self::DeviceImacMinus, Self::DeviceImacOff, Self::DeviceImacPause, Self::DeviceImacPin, Self::DeviceImacPlus, Self::DeviceImacQuestion, Self::DeviceImacSearch, Self::DeviceImacShare, Self::DeviceImacStar, Self::DeviceImacUp, Self::DeviceImacX, Self::DeviceIpad, Self::DeviceIpadBolt, Self::DeviceIpadCancel, Self::DeviceIpadCheck, Self::DeviceIpadCode, Self::DeviceIpadCog, Self::DeviceIpadDollar, Self::DeviceIpadDown, Self::DeviceIpadExclamation, Self::DeviceIpadHeart, Self::DeviceIpadHorizontal, Self::DeviceIpadHorizontalBolt, Self::DeviceIpadHorizontalCancel, Self::DeviceIpadHorizontalCheck, Self::DeviceIpadHorizontalCode, Self::DeviceIpadHorizontalCog, Self::DeviceIpadHorizontalDollar, Self::DeviceIpadHorizontalDown, Self::DeviceIpadHorizontalExclamation, Self::DeviceIpadHorizontalHeart, Self::DeviceIpadHorizontalMinus, Self::DeviceIpadHorizontalOff, Self::DeviceIpadHorizontalPause, Self::DeviceIpadHorizontalPin, Self::DeviceIpadHorizontalPlus, Self::DeviceIpadHorizontalQuestion, Self::DeviceIpadHorizontalSearch, Self::DeviceIpadHorizontalShare, Self::DeviceIpadHorizontalStar, Self::DeviceIpadHorizontalUp, Self::DeviceIpadHorizontalX, Self::DeviceIpadMinus, Self::DeviceIpadOff, Self::DeviceIpadPause, Self::DeviceIpadPin, Self::DeviceIpadPlus, Self::DeviceIpadQuestion, Self::DeviceIpadSearch, Self::DeviceIpadShare, Self::DeviceIpadStar, Self::DeviceIpadUp, Self::DeviceIpadX, Self::DeviceLandlinePhone, Self::DeviceLaptop, Self::DeviceLaptopOff, Self::DeviceMobile, Self::DeviceMobileBolt, Self::DeviceMobileCancel, Self::DeviceMobileCharging, Self::DeviceMobileCheck, Self::DeviceMobileCode, Self::DeviceMobileCog, Self::DeviceMobileDollar, Self::DeviceMobileDown, Self::DeviceMobileExclamation, Self::DeviceMobileHeart, Self::DeviceMobileMessage, Self::DeviceMobileMinus, Self::DeviceMobileOff, Self::DeviceMobilePause, Self::DeviceMobilePin, Self::DeviceMobilePlus, Self::DeviceMobileQuestion, Self::DeviceMobileRotated, Self::DeviceMobileSearch, Self::DeviceMobileShare, Self::DeviceMobileStar, Self::DeviceMobileUp, Self::DeviceMobileVibration, Self::DeviceMobileX, Self::DeviceNintendo, Self::DeviceNintendoOff, Self::DeviceRemote, Self::DeviceScreen, Self::DeviceSdCard, Self::DeviceSim, Self::DeviceSim1, Self::DeviceSim2, Self::DeviceSim3, Self::DeviceSpeaker, Self::DeviceSpeakerOff, Self::DeviceTablet, Self::DeviceTabletBolt, Self::DeviceTabletCancel, Self::DeviceTabletCheck, Self::DeviceTabletCode, Self::DeviceTabletCog, Self::DeviceTabletDollar, Self::DeviceTabletDown, Self::DeviceTabletExclamation, Self::DeviceTabletHeart, Self::DeviceTabletMinus, Self::DeviceTabletOff, Self::DeviceTabletPause, Self::DeviceTabletPin, Self::DeviceTabletPlus, Self::DeviceTabletQuestion, Self::DeviceTabletSearch, Self::DeviceTabletShare, Self::DeviceTabletStar, Self::DeviceTabletUp, Self::DeviceTabletX, Self::DeviceTv, Self::DeviceTvOff, Self::DeviceTvOld, Self::DeviceUsb, Self::DeviceVisionPro, Self::DeviceWatch, Self::DeviceWatchBolt, Self::DeviceWatchCancel, Self::DeviceWatchCheck, Self::DeviceWatchCode, Self::DeviceWatchCog, Self::DeviceWatchDollar, Self::DeviceWatchDown, Self::DeviceWatchExclamation, Self::DeviceWatchHeart, Self::DeviceWatchMinus, Self::DeviceWatchOff, Self::DeviceWatchPause, Self::DeviceWatchPin, Self::DeviceWatchPlus, Self::DeviceWatchQuestion, Self::DeviceWatchSearch, Self::DeviceWatchShare, Self::DeviceWatchStar, Self::DeviceWatchStats, Self::DeviceWatchStats2, Self::DeviceWatchUp, Self::DeviceWatchX, Self::Devices, Self::Devices2, Self::DevicesBolt, Self::DevicesCancel, Self::DevicesCheck, Self::DevicesCode, Self::DevicesCog, Self::DevicesDollar, Self::DevicesDown, Self::DevicesExclamation, Self::DevicesHeart, Self::DevicesMinus, Self::DevicesOff, Self::DevicesPause, Self::DevicesPc, Self::DevicesPcOff, Self::DevicesPin, Self::DevicesPlus, Self::DevicesQuestion, Self::DevicesSearch, Self::DevicesShare, Self::DevicesStar, Self::DevicesUp, Self::DevicesX, Self::Dialpad, Self::DialpadOff, Self::Disc, Self::DiscOff, Self::DualScreen, Self::EarphoneBluetooth, Self::Error404, Self::Error404Off, Self::Fridge, Self::FridgeOff, Self::Gif, Self::GitBranch, Self::GitBranchDeleted, Self::GitCherryPick, Self::GitCommit, Self::GitCompare, Self::GitFork, Self::GitMerge, Self::GitPullRequest, Self::GitPullRequestClosed, Self::GitPullRequestConflict, Self::GitPullRequestDraft, Self::HammerDrill, Self::Html, Self::HttpConnect, Self::HttpConnectOff, Self::HttpDelete, Self::HttpDeleteOff, Self::HttpGet, Self::HttpGetOff, Self::HttpHead, Self::HttpHeadOff, Self::HttpOptions, Self::HttpOptionsOff, Self::HttpPatch, Self::HttpPatchOff, Self::HttpPost, Self::HttpPostOff, Self::HttpPut, Self::HttpPutOff, Self::HttpQue, Self::HttpQueOff, Self::HttpTrace, Self::HttpTraceOff, Self::Jpg, Self::Json, Self::Keyboard, Self::KeyboardHide, Self::KeyboardOff, Self::KeyboardShow, Self::LawnMower, Self::LoadBalancer, Self::Mouse, Self::Mouse2, Self::MouseOff, Self::Network, Self::NetworkOff, Self::Nfc, Self::NfcOff, Self::Pdf, Self::Phone, Self::PhoneCall, Self::PhoneCalling, Self::PhoneCheck, Self::PhoneIncoming, Self::PhoneOff, Self::PhoneOutgoing, Self::PhonePause, Self::PhonePlus, Self::PhoneSpark, Self::PhoneX, Self::PlaystationCircle, Self::PlaystationSquare, Self::PlaystationTriangle, Self::PlaystationX, Self::Plug, Self::PlugConnected, Self::PlugConnectedX, Self::PlugOff, Self::PlugX, Self::Png, Self::Power, Self::Printer, Self::PrinterOff, Self::Qrcode, Self::QrcodeOff, Self::Router, Self::RouterOff, Self::ScreenShare, Self::ScreenShareOff, Self::Server, Self::Server2, Self::ServerBolt, Self::ServerCog, Self::ServerOff, Self::ServerSpark, Self::Shredder, Self::Signal2g, Self::Signal3g, Self::Signal4g, Self::Signal4gPlus, Self::Signal5g, Self::Signal6g, Self::SignalE, Self::SignalG, Self::SignalH, Self::SignalHPlus, Self::SignalLte, Self::Sql, Self::Svg, Self::Toml, Self::TopologyBus, Self::TopologyComplex, Self::TopologyFull, Self::TopologyFullHierarchy, Self::TopologyRing, Self::TopologyRing2, Self::TopologyRing3, Self::TopologyStar, Self::TopologyStar2, Self::TopologyStar3, Self::TopologyStarRing, Self::TopologyStarRing2, Self::TopologyStarRing3, Self::Txt, Self::ViewportNarrow, Self::ViewportWide, Self::Vinyl, Self::WashMachine, Self::Wifi, Self::Wifi0, Self::Wifi1, Self::Wifi2, Self::WifiOff, Self::XboxA, Self::XboxB, Self::XboxX, Self::XboxY, Self::Zip]
    }

    /// Returns the icon count.
    pub fn count() -> usize {
        443
    }

    /// Creates an icon from its kebab-case name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "access-point" => Some(Self::AccessPoint),
            "access-point-off" => Some(Self::AccessPointOff),
            "air-conditioning" => Some(Self::AirConditioning),
            "air-conditioning-disabled" => Some(Self::AirConditioningDisabled),
            "alarm-smoke" => Some(Self::AlarmSmoke),
            "antenna" => Some(Self::Antenna),
            "antenna-bars-1" => Some(Self::AntennaBars1),
            "antenna-bars-2" => Some(Self::AntennaBars2),
            "antenna-bars-3" => Some(Self::AntennaBars3),
            "antenna-bars-4" => Some(Self::AntennaBars4),
            "antenna-bars-5" => Some(Self::AntennaBars5),
            "antenna-bars-off" => Some(Self::AntennaBarsOff),
            "antenna-off" => Some(Self::AntennaOff),
            "battery" => Some(Self::Battery),
            "battery-1" => Some(Self::Battery1),
            "battery-2" => Some(Self::Battery2),
            "battery-3" => Some(Self::Battery3),
            "battery-4" => Some(Self::Battery4),
            "battery-charging" => Some(Self::BatteryCharging),
            "battery-charging-2" => Some(Self::BatteryCharging2),
            "battery-eco" => Some(Self::BatteryEco),
            "battery-exclamation" => Some(Self::BatteryExclamation),
            "battery-off" => Some(Self::BatteryOff),
            "battery-spark" => Some(Self::BatterySpark),
            "battery-vertical" => Some(Self::BatteryVertical),
            "battery-vertical-1" => Some(Self::BatteryVertical1),
            "battery-vertical-2" => Some(Self::BatteryVertical2),
            "battery-vertical-3" => Some(Self::BatteryVertical3),
            "battery-vertical-4" => Some(Self::BatteryVertical4),
            "battery-vertical-charging" => Some(Self::BatteryVerticalCharging),
            "battery-vertical-charging-2" => Some(Self::BatteryVerticalCharging2),
            "battery-vertical-eco" => Some(Self::BatteryVerticalEco),
            "battery-vertical-exclamation" => Some(Self::BatteryVerticalExclamation),
            "battery-vertical-off" => Some(Self::BatteryVerticalOff),
            "binary" => Some(Self::Binary),
            "binary-off" => Some(Self::BinaryOff),
            "binary-tree" => Some(Self::BinaryTree),
            "binary-tree-2" => Some(Self::BinaryTree2),
            "blender" => Some(Self::Blender),
            "bluetooth" => Some(Self::Bluetooth),
            "bluetooth-connected" => Some(Self::BluetoothConnected),
            "bluetooth-off" => Some(Self::BluetoothOff),
            "bluetooth-x" => Some(Self::BluetoothX),
            "bmp" => Some(Self::Bmp),
            "broadcast" => Some(Self::Broadcast),
            "broadcast-off" => Some(Self::BroadcastOff),
            "browser" => Some(Self::Browser),
            "browser-check" => Some(Self::BrowserCheck),
            "browser-maximize" => Some(Self::BrowserMaximize),
            "browser-minus" => Some(Self::BrowserMinus),
            "browser-off" => Some(Self::BrowserOff),
            "browser-plus" => Some(Self::BrowserPlus),
            "browser-share" => Some(Self::BrowserShare),
            "browser-x" => Some(Self::BrowserX),
            "calculator" => Some(Self::Calculator),
            "calculator-off" => Some(Self::CalculatorOff),
            "cardboards" => Some(Self::Cardboards),
            "cardboards-off" => Some(Self::CardboardsOff),
            "cell-signal-1" => Some(Self::CellSignal1),
            "cell-signal-2" => Some(Self::CellSignal2),
            "cell-signal-3" => Some(Self::CellSignal3),
            "cell-signal-4" => Some(Self::CellSignal4),
            "cell-signal-5" => Some(Self::CellSignal5),
            "cell-signal-off" => Some(Self::CellSignalOff),
            "circuit-ammeter" => Some(Self::CircuitAmmeter),
            "circuit-battery" => Some(Self::CircuitBattery),
            "circuit-bulb" => Some(Self::CircuitBulb),
            "circuit-capacitor" => Some(Self::CircuitCapacitor),
            "circuit-capacitor-polarized" => Some(Self::CircuitCapacitorPolarized),
            "circuit-cell" => Some(Self::CircuitCell),
            "circuit-cell-plus" => Some(Self::CircuitCellPlus),
            "circuit-changeover" => Some(Self::CircuitChangeover),
            "circuit-diode" => Some(Self::CircuitDiode),
            "circuit-diode-zener" => Some(Self::CircuitDiodeZener),
            "circuit-ground" => Some(Self::CircuitGround),
            "circuit-ground-digital" => Some(Self::CircuitGroundDigital),
            "circuit-inductor" => Some(Self::CircuitInductor),
            "circuit-motor" => Some(Self::CircuitMotor),
            "circuit-pushbutton" => Some(Self::CircuitPushbutton),
            "circuit-resistor" => Some(Self::CircuitResistor),
            "circuit-switch-closed" => Some(Self::CircuitSwitchClosed),
            "circuit-switch-open" => Some(Self::CircuitSwitchOpen),
            "circuit-voltmeter" => Some(Self::CircuitVoltmeter),
            "cloud-computing" => Some(Self::CloudComputing),
            "cloud-data-connection" => Some(Self::CloudDataConnection),
            "cpu" => Some(Self::Cpu),
            "cpu-2" => Some(Self::Cpu2),
            "cpu-off" => Some(Self::CpuOff),
            "csv" => Some(Self::Csv),
            "device-3d-camera" => Some(Self::Device3dCamera),
            "device-3d-lens" => Some(Self::Device3dLens),
            "device-airpods" => Some(Self::DeviceAirpods),
            "device-airpods-case" => Some(Self::DeviceAirpodsCase),
            "device-airtag" => Some(Self::DeviceAirtag),
            "device-analytics" => Some(Self::DeviceAnalytics),
            "device-audio-tape" => Some(Self::DeviceAudioTape),
            "device-camera-phone" => Some(Self::DeviceCameraPhone),
            "device-cctv" => Some(Self::DeviceCctv),
            "device-cctv-off" => Some(Self::DeviceCctvOff),
            "device-computer-camera" => Some(Self::DeviceComputerCamera),
            "device-computer-camera-2" => Some(Self::DeviceComputerCamera2),
            "device-computer-camera-off" => Some(Self::DeviceComputerCameraOff),
            "device-desktop" => Some(Self::DeviceDesktop),
            "device-desktop-analytics" => Some(Self::DeviceDesktopAnalytics),
            "device-desktop-bolt" => Some(Self::DeviceDesktopBolt),
            "device-desktop-cancel" => Some(Self::DeviceDesktopCancel),
            "device-desktop-check" => Some(Self::DeviceDesktopCheck),
            "device-desktop-code" => Some(Self::DeviceDesktopCode),
            "device-desktop-cog" => Some(Self::DeviceDesktopCog),
            "device-desktop-dollar" => Some(Self::DeviceDesktopDollar),
            "device-desktop-down" => Some(Self::DeviceDesktopDown),
            "device-desktop-exclamation" => Some(Self::DeviceDesktopExclamation),
            "device-desktop-heart" => Some(Self::DeviceDesktopHeart),
            "device-desktop-minus" => Some(Self::DeviceDesktopMinus),
            "device-desktop-off" => Some(Self::DeviceDesktopOff),
            "device-desktop-pause" => Some(Self::DeviceDesktopPause),
            "device-desktop-pin" => Some(Self::DeviceDesktopPin),
            "device-desktop-plus" => Some(Self::DeviceDesktopPlus),
            "device-desktop-question" => Some(Self::DeviceDesktopQuestion),
            "device-desktop-search" => Some(Self::DeviceDesktopSearch),
            "device-desktop-share" => Some(Self::DeviceDesktopShare),
            "device-desktop-star" => Some(Self::DeviceDesktopStar),
            "device-desktop-up" => Some(Self::DeviceDesktopUp),
            "device-desktop-x" => Some(Self::DeviceDesktopX),
            "device-floppy" => Some(Self::DeviceFloppy),
            "device-gamepad" => Some(Self::DeviceGamepad),
            "device-gamepad-2" => Some(Self::DeviceGamepad2),
            "device-gamepad-3" => Some(Self::DeviceGamepad3),
            "device-heart-monitor" => Some(Self::DeviceHeartMonitor),
            "device-imac" => Some(Self::DeviceImac),
            "device-imac-bolt" => Some(Self::DeviceImacBolt),
            "device-imac-cancel" => Some(Self::DeviceImacCancel),
            "device-imac-check" => Some(Self::DeviceImacCheck),
            "device-imac-code" => Some(Self::DeviceImacCode),
            "device-imac-cog" => Some(Self::DeviceImacCog),
            "device-imac-dollar" => Some(Self::DeviceImacDollar),
            "device-imac-down" => Some(Self::DeviceImacDown),
            "device-imac-exclamation" => Some(Self::DeviceImacExclamation),
            "device-imac-heart" => Some(Self::DeviceImacHeart),
            "device-imac-minus" => Some(Self::DeviceImacMinus),
            "device-imac-off" => Some(Self::DeviceImacOff),
            "device-imac-pause" => Some(Self::DeviceImacPause),
            "device-imac-pin" => Some(Self::DeviceImacPin),
            "device-imac-plus" => Some(Self::DeviceImacPlus),
            "device-imac-question" => Some(Self::DeviceImacQuestion),
            "device-imac-search" => Some(Self::DeviceImacSearch),
            "device-imac-share" => Some(Self::DeviceImacShare),
            "device-imac-star" => Some(Self::DeviceImacStar),
            "device-imac-up" => Some(Self::DeviceImacUp),
            "device-imac-x" => Some(Self::DeviceImacX),
            "device-ipad" => Some(Self::DeviceIpad),
            "device-ipad-bolt" => Some(Self::DeviceIpadBolt),
            "device-ipad-cancel" => Some(Self::DeviceIpadCancel),
            "device-ipad-check" => Some(Self::DeviceIpadCheck),
            "device-ipad-code" => Some(Self::DeviceIpadCode),
            "device-ipad-cog" => Some(Self::DeviceIpadCog),
            "device-ipad-dollar" => Some(Self::DeviceIpadDollar),
            "device-ipad-down" => Some(Self::DeviceIpadDown),
            "device-ipad-exclamation" => Some(Self::DeviceIpadExclamation),
            "device-ipad-heart" => Some(Self::DeviceIpadHeart),
            "device-ipad-horizontal" => Some(Self::DeviceIpadHorizontal),
            "device-ipad-horizontal-bolt" => Some(Self::DeviceIpadHorizontalBolt),
            "device-ipad-horizontal-cancel" => Some(Self::DeviceIpadHorizontalCancel),
            "device-ipad-horizontal-check" => Some(Self::DeviceIpadHorizontalCheck),
            "device-ipad-horizontal-code" => Some(Self::DeviceIpadHorizontalCode),
            "device-ipad-horizontal-cog" => Some(Self::DeviceIpadHorizontalCog),
            "device-ipad-horizontal-dollar" => Some(Self::DeviceIpadHorizontalDollar),
            "device-ipad-horizontal-down" => Some(Self::DeviceIpadHorizontalDown),
            "device-ipad-horizontal-exclamation" => Some(Self::DeviceIpadHorizontalExclamation),
            "device-ipad-horizontal-heart" => Some(Self::DeviceIpadHorizontalHeart),
            "device-ipad-horizontal-minus" => Some(Self::DeviceIpadHorizontalMinus),
            "device-ipad-horizontal-off" => Some(Self::DeviceIpadHorizontalOff),
            "device-ipad-horizontal-pause" => Some(Self::DeviceIpadHorizontalPause),
            "device-ipad-horizontal-pin" => Some(Self::DeviceIpadHorizontalPin),
            "device-ipad-horizontal-plus" => Some(Self::DeviceIpadHorizontalPlus),
            "device-ipad-horizontal-question" => Some(Self::DeviceIpadHorizontalQuestion),
            "device-ipad-horizontal-search" => Some(Self::DeviceIpadHorizontalSearch),
            "device-ipad-horizontal-share" => Some(Self::DeviceIpadHorizontalShare),
            "device-ipad-horizontal-star" => Some(Self::DeviceIpadHorizontalStar),
            "device-ipad-horizontal-up" => Some(Self::DeviceIpadHorizontalUp),
            "device-ipad-horizontal-x" => Some(Self::DeviceIpadHorizontalX),
            "device-ipad-minus" => Some(Self::DeviceIpadMinus),
            "device-ipad-off" => Some(Self::DeviceIpadOff),
            "device-ipad-pause" => Some(Self::DeviceIpadPause),
            "device-ipad-pin" => Some(Self::DeviceIpadPin),
            "device-ipad-plus" => Some(Self::DeviceIpadPlus),
            "device-ipad-question" => Some(Self::DeviceIpadQuestion),
            "device-ipad-search" => Some(Self::DeviceIpadSearch),
            "device-ipad-share" => Some(Self::DeviceIpadShare),
            "device-ipad-star" => Some(Self::DeviceIpadStar),
            "device-ipad-up" => Some(Self::DeviceIpadUp),
            "device-ipad-x" => Some(Self::DeviceIpadX),
            "device-landline-phone" => Some(Self::DeviceLandlinePhone),
            "device-laptop" => Some(Self::DeviceLaptop),
            "device-laptop-off" => Some(Self::DeviceLaptopOff),
            "device-mobile" => Some(Self::DeviceMobile),
            "device-mobile-bolt" => Some(Self::DeviceMobileBolt),
            "device-mobile-cancel" => Some(Self::DeviceMobileCancel),
            "device-mobile-charging" => Some(Self::DeviceMobileCharging),
            "device-mobile-check" => Some(Self::DeviceMobileCheck),
            "device-mobile-code" => Some(Self::DeviceMobileCode),
            "device-mobile-cog" => Some(Self::DeviceMobileCog),
            "device-mobile-dollar" => Some(Self::DeviceMobileDollar),
            "device-mobile-down" => Some(Self::DeviceMobileDown),
            "device-mobile-exclamation" => Some(Self::DeviceMobileExclamation),
            "device-mobile-heart" => Some(Self::DeviceMobileHeart),
            "device-mobile-message" => Some(Self::DeviceMobileMessage),
            "device-mobile-minus" => Some(Self::DeviceMobileMinus),
            "device-mobile-off" => Some(Self::DeviceMobileOff),
            "device-mobile-pause" => Some(Self::DeviceMobilePause),
            "device-mobile-pin" => Some(Self::DeviceMobilePin),
            "device-mobile-plus" => Some(Self::DeviceMobilePlus),
            "device-mobile-question" => Some(Self::DeviceMobileQuestion),
            "device-mobile-rotated" => Some(Self::DeviceMobileRotated),
            "device-mobile-search" => Some(Self::DeviceMobileSearch),
            "device-mobile-share" => Some(Self::DeviceMobileShare),
            "device-mobile-star" => Some(Self::DeviceMobileStar),
            "device-mobile-up" => Some(Self::DeviceMobileUp),
            "device-mobile-vibration" => Some(Self::DeviceMobileVibration),
            "device-mobile-x" => Some(Self::DeviceMobileX),
            "device-nintendo" => Some(Self::DeviceNintendo),
            "device-nintendo-off" => Some(Self::DeviceNintendoOff),
            "device-remote" => Some(Self::DeviceRemote),
            "device-screen" => Some(Self::DeviceScreen),
            "device-sd-card" => Some(Self::DeviceSdCard),
            "device-sim" => Some(Self::DeviceSim),
            "device-sim-1" => Some(Self::DeviceSim1),
            "device-sim-2" => Some(Self::DeviceSim2),
            "device-sim-3" => Some(Self::DeviceSim3),
            "device-speaker" => Some(Self::DeviceSpeaker),
            "device-speaker-off" => Some(Self::DeviceSpeakerOff),
            "device-tablet" => Some(Self::DeviceTablet),
            "device-tablet-bolt" => Some(Self::DeviceTabletBolt),
            "device-tablet-cancel" => Some(Self::DeviceTabletCancel),
            "device-tablet-check" => Some(Self::DeviceTabletCheck),
            "device-tablet-code" => Some(Self::DeviceTabletCode),
            "device-tablet-cog" => Some(Self::DeviceTabletCog),
            "device-tablet-dollar" => Some(Self::DeviceTabletDollar),
            "device-tablet-down" => Some(Self::DeviceTabletDown),
            "device-tablet-exclamation" => Some(Self::DeviceTabletExclamation),
            "device-tablet-heart" => Some(Self::DeviceTabletHeart),
            "device-tablet-minus" => Some(Self::DeviceTabletMinus),
            "device-tablet-off" => Some(Self::DeviceTabletOff),
            "device-tablet-pause" => Some(Self::DeviceTabletPause),
            "device-tablet-pin" => Some(Self::DeviceTabletPin),
            "device-tablet-plus" => Some(Self::DeviceTabletPlus),
            "device-tablet-question" => Some(Self::DeviceTabletQuestion),
            "device-tablet-search" => Some(Self::DeviceTabletSearch),
            "device-tablet-share" => Some(Self::DeviceTabletShare),
            "device-tablet-star" => Some(Self::DeviceTabletStar),
            "device-tablet-up" => Some(Self::DeviceTabletUp),
            "device-tablet-x" => Some(Self::DeviceTabletX),
            "device-tv" => Some(Self::DeviceTv),
            "device-tv-off" => Some(Self::DeviceTvOff),
            "device-tv-old" => Some(Self::DeviceTvOld),
            "device-usb" => Some(Self::DeviceUsb),
            "device-vision-pro" => Some(Self::DeviceVisionPro),
            "device-watch" => Some(Self::DeviceWatch),
            "device-watch-bolt" => Some(Self::DeviceWatchBolt),
            "device-watch-cancel" => Some(Self::DeviceWatchCancel),
            "device-watch-check" => Some(Self::DeviceWatchCheck),
            "device-watch-code" => Some(Self::DeviceWatchCode),
            "device-watch-cog" => Some(Self::DeviceWatchCog),
            "device-watch-dollar" => Some(Self::DeviceWatchDollar),
            "device-watch-down" => Some(Self::DeviceWatchDown),
            "device-watch-exclamation" => Some(Self::DeviceWatchExclamation),
            "device-watch-heart" => Some(Self::DeviceWatchHeart),
            "device-watch-minus" => Some(Self::DeviceWatchMinus),
            "device-watch-off" => Some(Self::DeviceWatchOff),
            "device-watch-pause" => Some(Self::DeviceWatchPause),
            "device-watch-pin" => Some(Self::DeviceWatchPin),
            "device-watch-plus" => Some(Self::DeviceWatchPlus),
            "device-watch-question" => Some(Self::DeviceWatchQuestion),
            "device-watch-search" => Some(Self::DeviceWatchSearch),
            "device-watch-share" => Some(Self::DeviceWatchShare),
            "device-watch-star" => Some(Self::DeviceWatchStar),
            "device-watch-stats" => Some(Self::DeviceWatchStats),
            "device-watch-stats-2" => Some(Self::DeviceWatchStats2),
            "device-watch-up" => Some(Self::DeviceWatchUp),
            "device-watch-x" => Some(Self::DeviceWatchX),
            "devices" => Some(Self::Devices),
            "devices-2" => Some(Self::Devices2),
            "devices-bolt" => Some(Self::DevicesBolt),
            "devices-cancel" => Some(Self::DevicesCancel),
            "devices-check" => Some(Self::DevicesCheck),
            "devices-code" => Some(Self::DevicesCode),
            "devices-cog" => Some(Self::DevicesCog),
            "devices-dollar" => Some(Self::DevicesDollar),
            "devices-down" => Some(Self::DevicesDown),
            "devices-exclamation" => Some(Self::DevicesExclamation),
            "devices-heart" => Some(Self::DevicesHeart),
            "devices-minus" => Some(Self::DevicesMinus),
            "devices-off" => Some(Self::DevicesOff),
            "devices-pause" => Some(Self::DevicesPause),
            "devices-pc" => Some(Self::DevicesPc),
            "devices-pc-off" => Some(Self::DevicesPcOff),
            "devices-pin" => Some(Self::DevicesPin),
            "devices-plus" => Some(Self::DevicesPlus),
            "devices-question" => Some(Self::DevicesQuestion),
            "devices-search" => Some(Self::DevicesSearch),
            "devices-share" => Some(Self::DevicesShare),
            "devices-star" => Some(Self::DevicesStar),
            "devices-up" => Some(Self::DevicesUp),
            "devices-x" => Some(Self::DevicesX),
            "dialpad" => Some(Self::Dialpad),
            "dialpad-off" => Some(Self::DialpadOff),
            "disc" => Some(Self::Disc),
            "disc-off" => Some(Self::DiscOff),
            "dual-screen" => Some(Self::DualScreen),
            "earphone-bluetooth" => Some(Self::EarphoneBluetooth),
            "error-404" => Some(Self::Error404),
            "error-404-off" => Some(Self::Error404Off),
            "fridge" => Some(Self::Fridge),
            "fridge-off" => Some(Self::FridgeOff),
            "gif" => Some(Self::Gif),
            "git-branch" => Some(Self::GitBranch),
            "git-branch-deleted" => Some(Self::GitBranchDeleted),
            "git-cherry-pick" => Some(Self::GitCherryPick),
            "git-commit" => Some(Self::GitCommit),
            "git-compare" => Some(Self::GitCompare),
            "git-fork" => Some(Self::GitFork),
            "git-merge" => Some(Self::GitMerge),
            "git-pull-request" => Some(Self::GitPullRequest),
            "git-pull-request-closed" => Some(Self::GitPullRequestClosed),
            "git-pull-request-conflict" => Some(Self::GitPullRequestConflict),
            "git-pull-request-draft" => Some(Self::GitPullRequestDraft),
            "hammer-drill" => Some(Self::HammerDrill),
            "html" => Some(Self::Html),
            "http-connect" => Some(Self::HttpConnect),
            "http-connect-off" => Some(Self::HttpConnectOff),
            "http-delete" => Some(Self::HttpDelete),
            "http-delete-off" => Some(Self::HttpDeleteOff),
            "http-get" => Some(Self::HttpGet),
            "http-get-off" => Some(Self::HttpGetOff),
            "http-head" => Some(Self::HttpHead),
            "http-head-off" => Some(Self::HttpHeadOff),
            "http-options" => Some(Self::HttpOptions),
            "http-options-off" => Some(Self::HttpOptionsOff),
            "http-patch" => Some(Self::HttpPatch),
            "http-patch-off" => Some(Self::HttpPatchOff),
            "http-post" => Some(Self::HttpPost),
            "http-post-off" => Some(Self::HttpPostOff),
            "http-put" => Some(Self::HttpPut),
            "http-put-off" => Some(Self::HttpPutOff),
            "http-que" => Some(Self::HttpQue),
            "http-que-off" => Some(Self::HttpQueOff),
            "http-trace" => Some(Self::HttpTrace),
            "http-trace-off" => Some(Self::HttpTraceOff),
            "jpg" => Some(Self::Jpg),
            "json" => Some(Self::Json),
            "keyboard" => Some(Self::Keyboard),
            "keyboard-hide" => Some(Self::KeyboardHide),
            "keyboard-off" => Some(Self::KeyboardOff),
            "keyboard-show" => Some(Self::KeyboardShow),
            "lawn-mower" => Some(Self::LawnMower),
            "load-balancer" => Some(Self::LoadBalancer),
            "mouse" => Some(Self::Mouse),
            "mouse-2" => Some(Self::Mouse2),
            "mouse-off" => Some(Self::MouseOff),
            "network" => Some(Self::Network),
            "network-off" => Some(Self::NetworkOff),
            "nfc" => Some(Self::Nfc),
            "nfc-off" => Some(Self::NfcOff),
            "pdf" => Some(Self::Pdf),
            "phone" => Some(Self::Phone),
            "phone-call" => Some(Self::PhoneCall),
            "phone-calling" => Some(Self::PhoneCalling),
            "phone-check" => Some(Self::PhoneCheck),
            "phone-incoming" => Some(Self::PhoneIncoming),
            "phone-off" => Some(Self::PhoneOff),
            "phone-outgoing" => Some(Self::PhoneOutgoing),
            "phone-pause" => Some(Self::PhonePause),
            "phone-plus" => Some(Self::PhonePlus),
            "phone-spark" => Some(Self::PhoneSpark),
            "phone-x" => Some(Self::PhoneX),
            "playstation-circle" => Some(Self::PlaystationCircle),
            "playstation-square" => Some(Self::PlaystationSquare),
            "playstation-triangle" => Some(Self::PlaystationTriangle),
            "playstation-x" => Some(Self::PlaystationX),
            "plug" => Some(Self::Plug),
            "plug-connected" => Some(Self::PlugConnected),
            "plug-connected-x" => Some(Self::PlugConnectedX),
            "plug-off" => Some(Self::PlugOff),
            "plug-x" => Some(Self::PlugX),
            "png" => Some(Self::Png),
            "power" => Some(Self::Power),
            "printer" => Some(Self::Printer),
            "printer-off" => Some(Self::PrinterOff),
            "qrcode" => Some(Self::Qrcode),
            "qrcode-off" => Some(Self::QrcodeOff),
            "router" => Some(Self::Router),
            "router-off" => Some(Self::RouterOff),
            "screen-share" => Some(Self::ScreenShare),
            "screen-share-off" => Some(Self::ScreenShareOff),
            "server" => Some(Self::Server),
            "server-2" => Some(Self::Server2),
            "server-bolt" => Some(Self::ServerBolt),
            "server-cog" => Some(Self::ServerCog),
            "server-off" => Some(Self::ServerOff),
            "server-spark" => Some(Self::ServerSpark),
            "shredder" => Some(Self::Shredder),
            "signal-2g" => Some(Self::Signal2g),
            "signal-3g" => Some(Self::Signal3g),
            "signal-4g" => Some(Self::Signal4g),
            "signal-4g-plus" => Some(Self::Signal4gPlus),
            "signal-5g" => Some(Self::Signal5g),
            "signal-6g" => Some(Self::Signal6g),
            "signal-e" => Some(Self::SignalE),
            "signal-g" => Some(Self::SignalG),
            "signal-h" => Some(Self::SignalH),
            "signal-h-plus" => Some(Self::SignalHPlus),
            "signal-lte" => Some(Self::SignalLte),
            "sql" => Some(Self::Sql),
            "svg" => Some(Self::Svg),
            "toml" => Some(Self::Toml),
            "topology-bus" => Some(Self::TopologyBus),
            "topology-complex" => Some(Self::TopologyComplex),
            "topology-full" => Some(Self::TopologyFull),
            "topology-full-hierarchy" => Some(Self::TopologyFullHierarchy),
            "topology-ring" => Some(Self::TopologyRing),
            "topology-ring-2" => Some(Self::TopologyRing2),
            "topology-ring-3" => Some(Self::TopologyRing3),
            "topology-star" => Some(Self::TopologyStar),
            "topology-star-2" => Some(Self::TopologyStar2),
            "topology-star-3" => Some(Self::TopologyStar3),
            "topology-star-ring" => Some(Self::TopologyStarRing),
            "topology-star-ring-2" => Some(Self::TopologyStarRing2),
            "topology-star-ring-3" => Some(Self::TopologyStarRing3),
            "txt" => Some(Self::Txt),
            "viewport-narrow" => Some(Self::ViewportNarrow),
            "viewport-wide" => Some(Self::ViewportWide),
            "vinyl" => Some(Self::Vinyl),
            "wash-machine" => Some(Self::WashMachine),
            "wifi" => Some(Self::Wifi),
            "wifi-0" => Some(Self::Wifi0),
            "wifi-1" => Some(Self::Wifi1),
            "wifi-2" => Some(Self::Wifi2),
            "wifi-off" => Some(Self::WifiOff),
            "xbox-a" => Some(Self::XboxA),
            "xbox-b" => Some(Self::XboxB),
            "xbox-x" => Some(Self::XboxX),
            "xbox-y" => Some(Self::XboxY),
            "zip" => Some(Self::Zip),
            _ => None,
        }
    }
}

impl TablerIconData for DevicesIcon {
    fn name(&self) -> &'static str {
        match self {
            Self::AccessPoint => "access-point",
            Self::AccessPointOff => "access-point-off",
            Self::AirConditioning => "air-conditioning",
            Self::AirConditioningDisabled => "air-conditioning-disabled",
            Self::AlarmSmoke => "alarm-smoke",
            Self::Antenna => "antenna",
            Self::AntennaBars1 => "antenna-bars-1",
            Self::AntennaBars2 => "antenna-bars-2",
            Self::AntennaBars3 => "antenna-bars-3",
            Self::AntennaBars4 => "antenna-bars-4",
            Self::AntennaBars5 => "antenna-bars-5",
            Self::AntennaBarsOff => "antenna-bars-off",
            Self::AntennaOff => "antenna-off",
            Self::Battery => "battery",
            Self::Battery1 => "battery-1",
            Self::Battery2 => "battery-2",
            Self::Battery3 => "battery-3",
            Self::Battery4 => "battery-4",
            Self::BatteryCharging => "battery-charging",
            Self::BatteryCharging2 => "battery-charging-2",
            Self::BatteryEco => "battery-eco",
            Self::BatteryExclamation => "battery-exclamation",
            Self::BatteryOff => "battery-off",
            Self::BatterySpark => "battery-spark",
            Self::BatteryVertical => "battery-vertical",
            Self::BatteryVertical1 => "battery-vertical-1",
            Self::BatteryVertical2 => "battery-vertical-2",
            Self::BatteryVertical3 => "battery-vertical-3",
            Self::BatteryVertical4 => "battery-vertical-4",
            Self::BatteryVerticalCharging => "battery-vertical-charging",
            Self::BatteryVerticalCharging2 => "battery-vertical-charging-2",
            Self::BatteryVerticalEco => "battery-vertical-eco",
            Self::BatteryVerticalExclamation => "battery-vertical-exclamation",
            Self::BatteryVerticalOff => "battery-vertical-off",
            Self::Binary => "binary",
            Self::BinaryOff => "binary-off",
            Self::BinaryTree => "binary-tree",
            Self::BinaryTree2 => "binary-tree-2",
            Self::Blender => "blender",
            Self::Bluetooth => "bluetooth",
            Self::BluetoothConnected => "bluetooth-connected",
            Self::BluetoothOff => "bluetooth-off",
            Self::BluetoothX => "bluetooth-x",
            Self::Bmp => "bmp",
            Self::Broadcast => "broadcast",
            Self::BroadcastOff => "broadcast-off",
            Self::Browser => "browser",
            Self::BrowserCheck => "browser-check",
            Self::BrowserMaximize => "browser-maximize",
            Self::BrowserMinus => "browser-minus",
            Self::BrowserOff => "browser-off",
            Self::BrowserPlus => "browser-plus",
            Self::BrowserShare => "browser-share",
            Self::BrowserX => "browser-x",
            Self::Calculator => "calculator",
            Self::CalculatorOff => "calculator-off",
            Self::Cardboards => "cardboards",
            Self::CardboardsOff => "cardboards-off",
            Self::CellSignal1 => "cell-signal-1",
            Self::CellSignal2 => "cell-signal-2",
            Self::CellSignal3 => "cell-signal-3",
            Self::CellSignal4 => "cell-signal-4",
            Self::CellSignal5 => "cell-signal-5",
            Self::CellSignalOff => "cell-signal-off",
            Self::CircuitAmmeter => "circuit-ammeter",
            Self::CircuitBattery => "circuit-battery",
            Self::CircuitBulb => "circuit-bulb",
            Self::CircuitCapacitor => "circuit-capacitor",
            Self::CircuitCapacitorPolarized => "circuit-capacitor-polarized",
            Self::CircuitCell => "circuit-cell",
            Self::CircuitCellPlus => "circuit-cell-plus",
            Self::CircuitChangeover => "circuit-changeover",
            Self::CircuitDiode => "circuit-diode",
            Self::CircuitDiodeZener => "circuit-diode-zener",
            Self::CircuitGround => "circuit-ground",
            Self::CircuitGroundDigital => "circuit-ground-digital",
            Self::CircuitInductor => "circuit-inductor",
            Self::CircuitMotor => "circuit-motor",
            Self::CircuitPushbutton => "circuit-pushbutton",
            Self::CircuitResistor => "circuit-resistor",
            Self::CircuitSwitchClosed => "circuit-switch-closed",
            Self::CircuitSwitchOpen => "circuit-switch-open",
            Self::CircuitVoltmeter => "circuit-voltmeter",
            Self::CloudComputing => "cloud-computing",
            Self::CloudDataConnection => "cloud-data-connection",
            Self::Cpu => "cpu",
            Self::Cpu2 => "cpu-2",
            Self::CpuOff => "cpu-off",
            Self::Csv => "csv",
            Self::Device3dCamera => "device-3d-camera",
            Self::Device3dLens => "device-3d-lens",
            Self::DeviceAirpods => "device-airpods",
            Self::DeviceAirpodsCase => "device-airpods-case",
            Self::DeviceAirtag => "device-airtag",
            Self::DeviceAnalytics => "device-analytics",
            Self::DeviceAudioTape => "device-audio-tape",
            Self::DeviceCameraPhone => "device-camera-phone",
            Self::DeviceCctv => "device-cctv",
            Self::DeviceCctvOff => "device-cctv-off",
            Self::DeviceComputerCamera => "device-computer-camera",
            Self::DeviceComputerCamera2 => "device-computer-camera-2",
            Self::DeviceComputerCameraOff => "device-computer-camera-off",
            Self::DeviceDesktop => "device-desktop",
            Self::DeviceDesktopAnalytics => "device-desktop-analytics",
            Self::DeviceDesktopBolt => "device-desktop-bolt",
            Self::DeviceDesktopCancel => "device-desktop-cancel",
            Self::DeviceDesktopCheck => "device-desktop-check",
            Self::DeviceDesktopCode => "device-desktop-code",
            Self::DeviceDesktopCog => "device-desktop-cog",
            Self::DeviceDesktopDollar => "device-desktop-dollar",
            Self::DeviceDesktopDown => "device-desktop-down",
            Self::DeviceDesktopExclamation => "device-desktop-exclamation",
            Self::DeviceDesktopHeart => "device-desktop-heart",
            Self::DeviceDesktopMinus => "device-desktop-minus",
            Self::DeviceDesktopOff => "device-desktop-off",
            Self::DeviceDesktopPause => "device-desktop-pause",
            Self::DeviceDesktopPin => "device-desktop-pin",
            Self::DeviceDesktopPlus => "device-desktop-plus",
            Self::DeviceDesktopQuestion => "device-desktop-question",
            Self::DeviceDesktopSearch => "device-desktop-search",
            Self::DeviceDesktopShare => "device-desktop-share",
            Self::DeviceDesktopStar => "device-desktop-star",
            Self::DeviceDesktopUp => "device-desktop-up",
            Self::DeviceDesktopX => "device-desktop-x",
            Self::DeviceFloppy => "device-floppy",
            Self::DeviceGamepad => "device-gamepad",
            Self::DeviceGamepad2 => "device-gamepad-2",
            Self::DeviceGamepad3 => "device-gamepad-3",
            Self::DeviceHeartMonitor => "device-heart-monitor",
            Self::DeviceImac => "device-imac",
            Self::DeviceImacBolt => "device-imac-bolt",
            Self::DeviceImacCancel => "device-imac-cancel",
            Self::DeviceImacCheck => "device-imac-check",
            Self::DeviceImacCode => "device-imac-code",
            Self::DeviceImacCog => "device-imac-cog",
            Self::DeviceImacDollar => "device-imac-dollar",
            Self::DeviceImacDown => "device-imac-down",
            Self::DeviceImacExclamation => "device-imac-exclamation",
            Self::DeviceImacHeart => "device-imac-heart",
            Self::DeviceImacMinus => "device-imac-minus",
            Self::DeviceImacOff => "device-imac-off",
            Self::DeviceImacPause => "device-imac-pause",
            Self::DeviceImacPin => "device-imac-pin",
            Self::DeviceImacPlus => "device-imac-plus",
            Self::DeviceImacQuestion => "device-imac-question",
            Self::DeviceImacSearch => "device-imac-search",
            Self::DeviceImacShare => "device-imac-share",
            Self::DeviceImacStar => "device-imac-star",
            Self::DeviceImacUp => "device-imac-up",
            Self::DeviceImacX => "device-imac-x",
            Self::DeviceIpad => "device-ipad",
            Self::DeviceIpadBolt => "device-ipad-bolt",
            Self::DeviceIpadCancel => "device-ipad-cancel",
            Self::DeviceIpadCheck => "device-ipad-check",
            Self::DeviceIpadCode => "device-ipad-code",
            Self::DeviceIpadCog => "device-ipad-cog",
            Self::DeviceIpadDollar => "device-ipad-dollar",
            Self::DeviceIpadDown => "device-ipad-down",
            Self::DeviceIpadExclamation => "device-ipad-exclamation",
            Self::DeviceIpadHeart => "device-ipad-heart",
            Self::DeviceIpadHorizontal => "device-ipad-horizontal",
            Self::DeviceIpadHorizontalBolt => "device-ipad-horizontal-bolt",
            Self::DeviceIpadHorizontalCancel => "device-ipad-horizontal-cancel",
            Self::DeviceIpadHorizontalCheck => "device-ipad-horizontal-check",
            Self::DeviceIpadHorizontalCode => "device-ipad-horizontal-code",
            Self::DeviceIpadHorizontalCog => "device-ipad-horizontal-cog",
            Self::DeviceIpadHorizontalDollar => "device-ipad-horizontal-dollar",
            Self::DeviceIpadHorizontalDown => "device-ipad-horizontal-down",
            Self::DeviceIpadHorizontalExclamation => "device-ipad-horizontal-exclamation",
            Self::DeviceIpadHorizontalHeart => "device-ipad-horizontal-heart",
            Self::DeviceIpadHorizontalMinus => "device-ipad-horizontal-minus",
            Self::DeviceIpadHorizontalOff => "device-ipad-horizontal-off",
            Self::DeviceIpadHorizontalPause => "device-ipad-horizontal-pause",
            Self::DeviceIpadHorizontalPin => "device-ipad-horizontal-pin",
            Self::DeviceIpadHorizontalPlus => "device-ipad-horizontal-plus",
            Self::DeviceIpadHorizontalQuestion => "device-ipad-horizontal-question",
            Self::DeviceIpadHorizontalSearch => "device-ipad-horizontal-search",
            Self::DeviceIpadHorizontalShare => "device-ipad-horizontal-share",
            Self::DeviceIpadHorizontalStar => "device-ipad-horizontal-star",
            Self::DeviceIpadHorizontalUp => "device-ipad-horizontal-up",
            Self::DeviceIpadHorizontalX => "device-ipad-horizontal-x",
            Self::DeviceIpadMinus => "device-ipad-minus",
            Self::DeviceIpadOff => "device-ipad-off",
            Self::DeviceIpadPause => "device-ipad-pause",
            Self::DeviceIpadPin => "device-ipad-pin",
            Self::DeviceIpadPlus => "device-ipad-plus",
            Self::DeviceIpadQuestion => "device-ipad-question",
            Self::DeviceIpadSearch => "device-ipad-search",
            Self::DeviceIpadShare => "device-ipad-share",
            Self::DeviceIpadStar => "device-ipad-star",
            Self::DeviceIpadUp => "device-ipad-up",
            Self::DeviceIpadX => "device-ipad-x",
            Self::DeviceLandlinePhone => "device-landline-phone",
            Self::DeviceLaptop => "device-laptop",
            Self::DeviceLaptopOff => "device-laptop-off",
            Self::DeviceMobile => "device-mobile",
            Self::DeviceMobileBolt => "device-mobile-bolt",
            Self::DeviceMobileCancel => "device-mobile-cancel",
            Self::DeviceMobileCharging => "device-mobile-charging",
            Self::DeviceMobileCheck => "device-mobile-check",
            Self::DeviceMobileCode => "device-mobile-code",
            Self::DeviceMobileCog => "device-mobile-cog",
            Self::DeviceMobileDollar => "device-mobile-dollar",
            Self::DeviceMobileDown => "device-mobile-down",
            Self::DeviceMobileExclamation => "device-mobile-exclamation",
            Self::DeviceMobileHeart => "device-mobile-heart",
            Self::DeviceMobileMessage => "device-mobile-message",
            Self::DeviceMobileMinus => "device-mobile-minus",
            Self::DeviceMobileOff => "device-mobile-off",
            Self::DeviceMobilePause => "device-mobile-pause",
            Self::DeviceMobilePin => "device-mobile-pin",
            Self::DeviceMobilePlus => "device-mobile-plus",
            Self::DeviceMobileQuestion => "device-mobile-question",
            Self::DeviceMobileRotated => "device-mobile-rotated",
            Self::DeviceMobileSearch => "device-mobile-search",
            Self::DeviceMobileShare => "device-mobile-share",
            Self::DeviceMobileStar => "device-mobile-star",
            Self::DeviceMobileUp => "device-mobile-up",
            Self::DeviceMobileVibration => "device-mobile-vibration",
            Self::DeviceMobileX => "device-mobile-x",
            Self::DeviceNintendo => "device-nintendo",
            Self::DeviceNintendoOff => "device-nintendo-off",
            Self::DeviceRemote => "device-remote",
            Self::DeviceScreen => "device-screen",
            Self::DeviceSdCard => "device-sd-card",
            Self::DeviceSim => "device-sim",
            Self::DeviceSim1 => "device-sim-1",
            Self::DeviceSim2 => "device-sim-2",
            Self::DeviceSim3 => "device-sim-3",
            Self::DeviceSpeaker => "device-speaker",
            Self::DeviceSpeakerOff => "device-speaker-off",
            Self::DeviceTablet => "device-tablet",
            Self::DeviceTabletBolt => "device-tablet-bolt",
            Self::DeviceTabletCancel => "device-tablet-cancel",
            Self::DeviceTabletCheck => "device-tablet-check",
            Self::DeviceTabletCode => "device-tablet-code",
            Self::DeviceTabletCog => "device-tablet-cog",
            Self::DeviceTabletDollar => "device-tablet-dollar",
            Self::DeviceTabletDown => "device-tablet-down",
            Self::DeviceTabletExclamation => "device-tablet-exclamation",
            Self::DeviceTabletHeart => "device-tablet-heart",
            Self::DeviceTabletMinus => "device-tablet-minus",
            Self::DeviceTabletOff => "device-tablet-off",
            Self::DeviceTabletPause => "device-tablet-pause",
            Self::DeviceTabletPin => "device-tablet-pin",
            Self::DeviceTabletPlus => "device-tablet-plus",
            Self::DeviceTabletQuestion => "device-tablet-question",
            Self::DeviceTabletSearch => "device-tablet-search",
            Self::DeviceTabletShare => "device-tablet-share",
            Self::DeviceTabletStar => "device-tablet-star",
            Self::DeviceTabletUp => "device-tablet-up",
            Self::DeviceTabletX => "device-tablet-x",
            Self::DeviceTv => "device-tv",
            Self::DeviceTvOff => "device-tv-off",
            Self::DeviceTvOld => "device-tv-old",
            Self::DeviceUsb => "device-usb",
            Self::DeviceVisionPro => "device-vision-pro",
            Self::DeviceWatch => "device-watch",
            Self::DeviceWatchBolt => "device-watch-bolt",
            Self::DeviceWatchCancel => "device-watch-cancel",
            Self::DeviceWatchCheck => "device-watch-check",
            Self::DeviceWatchCode => "device-watch-code",
            Self::DeviceWatchCog => "device-watch-cog",
            Self::DeviceWatchDollar => "device-watch-dollar",
            Self::DeviceWatchDown => "device-watch-down",
            Self::DeviceWatchExclamation => "device-watch-exclamation",
            Self::DeviceWatchHeart => "device-watch-heart",
            Self::DeviceWatchMinus => "device-watch-minus",
            Self::DeviceWatchOff => "device-watch-off",
            Self::DeviceWatchPause => "device-watch-pause",
            Self::DeviceWatchPin => "device-watch-pin",
            Self::DeviceWatchPlus => "device-watch-plus",
            Self::DeviceWatchQuestion => "device-watch-question",
            Self::DeviceWatchSearch => "device-watch-search",
            Self::DeviceWatchShare => "device-watch-share",
            Self::DeviceWatchStar => "device-watch-star",
            Self::DeviceWatchStats => "device-watch-stats",
            Self::DeviceWatchStats2 => "device-watch-stats-2",
            Self::DeviceWatchUp => "device-watch-up",
            Self::DeviceWatchX => "device-watch-x",
            Self::Devices => "devices",
            Self::Devices2 => "devices-2",
            Self::DevicesBolt => "devices-bolt",
            Self::DevicesCancel => "devices-cancel",
            Self::DevicesCheck => "devices-check",
            Self::DevicesCode => "devices-code",
            Self::DevicesCog => "devices-cog",
            Self::DevicesDollar => "devices-dollar",
            Self::DevicesDown => "devices-down",
            Self::DevicesExclamation => "devices-exclamation",
            Self::DevicesHeart => "devices-heart",
            Self::DevicesMinus => "devices-minus",
            Self::DevicesOff => "devices-off",
            Self::DevicesPause => "devices-pause",
            Self::DevicesPc => "devices-pc",
            Self::DevicesPcOff => "devices-pc-off",
            Self::DevicesPin => "devices-pin",
            Self::DevicesPlus => "devices-plus",
            Self::DevicesQuestion => "devices-question",
            Self::DevicesSearch => "devices-search",
            Self::DevicesShare => "devices-share",
            Self::DevicesStar => "devices-star",
            Self::DevicesUp => "devices-up",
            Self::DevicesX => "devices-x",
            Self::Dialpad => "dialpad",
            Self::DialpadOff => "dialpad-off",
            Self::Disc => "disc",
            Self::DiscOff => "disc-off",
            Self::DualScreen => "dual-screen",
            Self::EarphoneBluetooth => "earphone-bluetooth",
            Self::Error404 => "error-404",
            Self::Error404Off => "error-404-off",
            Self::Fridge => "fridge",
            Self::FridgeOff => "fridge-off",
            Self::Gif => "gif",
            Self::GitBranch => "git-branch",
            Self::GitBranchDeleted => "git-branch-deleted",
            Self::GitCherryPick => "git-cherry-pick",
            Self::GitCommit => "git-commit",
            Self::GitCompare => "git-compare",
            Self::GitFork => "git-fork",
            Self::GitMerge => "git-merge",
            Self::GitPullRequest => "git-pull-request",
            Self::GitPullRequestClosed => "git-pull-request-closed",
            Self::GitPullRequestConflict => "git-pull-request-conflict",
            Self::GitPullRequestDraft => "git-pull-request-draft",
            Self::HammerDrill => "hammer-drill",
            Self::Html => "html",
            Self::HttpConnect => "http-connect",
            Self::HttpConnectOff => "http-connect-off",
            Self::HttpDelete => "http-delete",
            Self::HttpDeleteOff => "http-delete-off",
            Self::HttpGet => "http-get",
            Self::HttpGetOff => "http-get-off",
            Self::HttpHead => "http-head",
            Self::HttpHeadOff => "http-head-off",
            Self::HttpOptions => "http-options",
            Self::HttpOptionsOff => "http-options-off",
            Self::HttpPatch => "http-patch",
            Self::HttpPatchOff => "http-patch-off",
            Self::HttpPost => "http-post",
            Self::HttpPostOff => "http-post-off",
            Self::HttpPut => "http-put",
            Self::HttpPutOff => "http-put-off",
            Self::HttpQue => "http-que",
            Self::HttpQueOff => "http-que-off",
            Self::HttpTrace => "http-trace",
            Self::HttpTraceOff => "http-trace-off",
            Self::Jpg => "jpg",
            Self::Json => "json",
            Self::Keyboard => "keyboard",
            Self::KeyboardHide => "keyboard-hide",
            Self::KeyboardOff => "keyboard-off",
            Self::KeyboardShow => "keyboard-show",
            Self::LawnMower => "lawn-mower",
            Self::LoadBalancer => "load-balancer",
            Self::Mouse => "mouse",
            Self::Mouse2 => "mouse-2",
            Self::MouseOff => "mouse-off",
            Self::Network => "network",
            Self::NetworkOff => "network-off",
            Self::Nfc => "nfc",
            Self::NfcOff => "nfc-off",
            Self::Pdf => "pdf",
            Self::Phone => "phone",
            Self::PhoneCall => "phone-call",
            Self::PhoneCalling => "phone-calling",
            Self::PhoneCheck => "phone-check",
            Self::PhoneIncoming => "phone-incoming",
            Self::PhoneOff => "phone-off",
            Self::PhoneOutgoing => "phone-outgoing",
            Self::PhonePause => "phone-pause",
            Self::PhonePlus => "phone-plus",
            Self::PhoneSpark => "phone-spark",
            Self::PhoneX => "phone-x",
            Self::PlaystationCircle => "playstation-circle",
            Self::PlaystationSquare => "playstation-square",
            Self::PlaystationTriangle => "playstation-triangle",
            Self::PlaystationX => "playstation-x",
            Self::Plug => "plug",
            Self::PlugConnected => "plug-connected",
            Self::PlugConnectedX => "plug-connected-x",
            Self::PlugOff => "plug-off",
            Self::PlugX => "plug-x",
            Self::Png => "png",
            Self::Power => "power",
            Self::Printer => "printer",
            Self::PrinterOff => "printer-off",
            Self::Qrcode => "qrcode",
            Self::QrcodeOff => "qrcode-off",
            Self::Router => "router",
            Self::RouterOff => "router-off",
            Self::ScreenShare => "screen-share",
            Self::ScreenShareOff => "screen-share-off",
            Self::Server => "server",
            Self::Server2 => "server-2",
            Self::ServerBolt => "server-bolt",
            Self::ServerCog => "server-cog",
            Self::ServerOff => "server-off",
            Self::ServerSpark => "server-spark",
            Self::Shredder => "shredder",
            Self::Signal2g => "signal-2g",
            Self::Signal3g => "signal-3g",
            Self::Signal4g => "signal-4g",
            Self::Signal4gPlus => "signal-4g-plus",
            Self::Signal5g => "signal-5g",
            Self::Signal6g => "signal-6g",
            Self::SignalE => "signal-e",
            Self::SignalG => "signal-g",
            Self::SignalH => "signal-h",
            Self::SignalHPlus => "signal-h-plus",
            Self::SignalLte => "signal-lte",
            Self::Sql => "sql",
            Self::Svg => "svg",
            Self::Toml => "toml",
            Self::TopologyBus => "topology-bus",
            Self::TopologyComplex => "topology-complex",
            Self::TopologyFull => "topology-full",
            Self::TopologyFullHierarchy => "topology-full-hierarchy",
            Self::TopologyRing => "topology-ring",
            Self::TopologyRing2 => "topology-ring-2",
            Self::TopologyRing3 => "topology-ring-3",
            Self::TopologyStar => "topology-star",
            Self::TopologyStar2 => "topology-star-2",
            Self::TopologyStar3 => "topology-star-3",
            Self::TopologyStarRing => "topology-star-ring",
            Self::TopologyStarRing2 => "topology-star-ring-2",
            Self::TopologyStarRing3 => "topology-star-ring-3",
            Self::Txt => "txt",
            Self::ViewportNarrow => "viewport-narrow",
            Self::ViewportWide => "viewport-wide",
            Self::Vinyl => "vinyl",
            Self::WashMachine => "wash-machine",
            Self::Wifi => "wifi",
            Self::Wifi0 => "wifi-0",
            Self::Wifi1 => "wifi-1",
            Self::Wifi2 => "wifi-2",
            Self::WifiOff => "wifi-off",
            Self::XboxA => "xbox-a",
            Self::XboxB => "xbox-b",
            Self::XboxX => "xbox-x",
            Self::XboxY => "xbox-y",
            Self::Zip => "zip",
        }
    }

    fn outline_svg(&self) -> &'static str {
        match self {
            Self::AccessPoint => ACCESS_POINT_SVG,
            Self::AccessPointOff => ACCESS_POINT_OFF_SVG,
            Self::AirConditioning => AIR_CONDITIONING_SVG,
            Self::AirConditioningDisabled => AIR_CONDITIONING_DISABLED_SVG,
            Self::AlarmSmoke => ALARM_SMOKE_SVG,
            Self::Antenna => ANTENNA_SVG,
            Self::AntennaBars1 => ANTENNA_BARS_1_SVG,
            Self::AntennaBars2 => ANTENNA_BARS_2_SVG,
            Self::AntennaBars3 => ANTENNA_BARS_3_SVG,
            Self::AntennaBars4 => ANTENNA_BARS_4_SVG,
            Self::AntennaBars5 => ANTENNA_BARS_5_SVG,
            Self::AntennaBarsOff => ANTENNA_BARS_OFF_SVG,
            Self::AntennaOff => ANTENNA_OFF_SVG,
            Self::Battery => BATTERY_SVG,
            Self::Battery1 => BATTERY_1_SVG,
            Self::Battery2 => BATTERY_2_SVG,
            Self::Battery3 => BATTERY_3_SVG,
            Self::Battery4 => BATTERY_4_SVG,
            Self::BatteryCharging => BATTERY_CHARGING_SVG,
            Self::BatteryCharging2 => BATTERY_CHARGING_2_SVG,
            Self::BatteryEco => BATTERY_ECO_SVG,
            Self::BatteryExclamation => BATTERY_EXCLAMATION_SVG,
            Self::BatteryOff => BATTERY_OFF_SVG,
            Self::BatterySpark => BATTERY_SPARK_SVG,
            Self::BatteryVertical => BATTERY_VERTICAL_SVG,
            Self::BatteryVertical1 => BATTERY_VERTICAL_1_SVG,
            Self::BatteryVertical2 => BATTERY_VERTICAL_2_SVG,
            Self::BatteryVertical3 => BATTERY_VERTICAL_3_SVG,
            Self::BatteryVertical4 => BATTERY_VERTICAL_4_SVG,
            Self::BatteryVerticalCharging => BATTERY_VERTICAL_CHARGING_SVG,
            Self::BatteryVerticalCharging2 => BATTERY_VERTICAL_CHARGING_2_SVG,
            Self::BatteryVerticalEco => BATTERY_VERTICAL_ECO_SVG,
            Self::BatteryVerticalExclamation => BATTERY_VERTICAL_EXCLAMATION_SVG,
            Self::BatteryVerticalOff => BATTERY_VERTICAL_OFF_SVG,
            Self::Binary => BINARY_SVG,
            Self::BinaryOff => BINARY_OFF_SVG,
            Self::BinaryTree => BINARY_TREE_SVG,
            Self::BinaryTree2 => BINARY_TREE_2_SVG,
            Self::Blender => BLENDER_SVG,
            Self::Bluetooth => BLUETOOTH_SVG,
            Self::BluetoothConnected => BLUETOOTH_CONNECTED_SVG,
            Self::BluetoothOff => BLUETOOTH_OFF_SVG,
            Self::BluetoothX => BLUETOOTH_X_SVG,
            Self::Bmp => BMP_SVG,
            Self::Broadcast => BROADCAST_SVG,
            Self::BroadcastOff => BROADCAST_OFF_SVG,
            Self::Browser => BROWSER_SVG,
            Self::BrowserCheck => BROWSER_CHECK_SVG,
            Self::BrowserMaximize => BROWSER_MAXIMIZE_SVG,
            Self::BrowserMinus => BROWSER_MINUS_SVG,
            Self::BrowserOff => BROWSER_OFF_SVG,
            Self::BrowserPlus => BROWSER_PLUS_SVG,
            Self::BrowserShare => BROWSER_SHARE_SVG,
            Self::BrowserX => BROWSER_X_SVG,
            Self::Calculator => CALCULATOR_SVG,
            Self::CalculatorOff => CALCULATOR_OFF_SVG,
            Self::Cardboards => CARDBOARDS_SVG,
            Self::CardboardsOff => CARDBOARDS_OFF_SVG,
            Self::CellSignal1 => CELL_SIGNAL_1_SVG,
            Self::CellSignal2 => CELL_SIGNAL_2_SVG,
            Self::CellSignal3 => CELL_SIGNAL_3_SVG,
            Self::CellSignal4 => CELL_SIGNAL_4_SVG,
            Self::CellSignal5 => CELL_SIGNAL_5_SVG,
            Self::CellSignalOff => CELL_SIGNAL_OFF_SVG,
            Self::CircuitAmmeter => CIRCUIT_AMMETER_SVG,
            Self::CircuitBattery => CIRCUIT_BATTERY_SVG,
            Self::CircuitBulb => CIRCUIT_BULB_SVG,
            Self::CircuitCapacitor => CIRCUIT_CAPACITOR_SVG,
            Self::CircuitCapacitorPolarized => CIRCUIT_CAPACITOR_POLARIZED_SVG,
            Self::CircuitCell => CIRCUIT_CELL_SVG,
            Self::CircuitCellPlus => CIRCUIT_CELL_PLUS_SVG,
            Self::CircuitChangeover => CIRCUIT_CHANGEOVER_SVG,
            Self::CircuitDiode => CIRCUIT_DIODE_SVG,
            Self::CircuitDiodeZener => CIRCUIT_DIODE_ZENER_SVG,
            Self::CircuitGround => CIRCUIT_GROUND_SVG,
            Self::CircuitGroundDigital => CIRCUIT_GROUND_DIGITAL_SVG,
            Self::CircuitInductor => CIRCUIT_INDUCTOR_SVG,
            Self::CircuitMotor => CIRCUIT_MOTOR_SVG,
            Self::CircuitPushbutton => CIRCUIT_PUSHBUTTON_SVG,
            Self::CircuitResistor => CIRCUIT_RESISTOR_SVG,
            Self::CircuitSwitchClosed => CIRCUIT_SWITCH_CLOSED_SVG,
            Self::CircuitSwitchOpen => CIRCUIT_SWITCH_OPEN_SVG,
            Self::CircuitVoltmeter => CIRCUIT_VOLTMETER_SVG,
            Self::CloudComputing => CLOUD_COMPUTING_SVG,
            Self::CloudDataConnection => CLOUD_DATA_CONNECTION_SVG,
            Self::Cpu => CPU_SVG,
            Self::Cpu2 => CPU_2_SVG,
            Self::CpuOff => CPU_OFF_SVG,
            Self::Csv => CSV_SVG,
            Self::Device3dCamera => DEVICE_3D_CAMERA_SVG,
            Self::Device3dLens => DEVICE_3D_LENS_SVG,
            Self::DeviceAirpods => DEVICE_AIRPODS_SVG,
            Self::DeviceAirpodsCase => DEVICE_AIRPODS_CASE_SVG,
            Self::DeviceAirtag => DEVICE_AIRTAG_SVG,
            Self::DeviceAnalytics => DEVICE_ANALYTICS_SVG,
            Self::DeviceAudioTape => DEVICE_AUDIO_TAPE_SVG,
            Self::DeviceCameraPhone => DEVICE_CAMERA_PHONE_SVG,
            Self::DeviceCctv => DEVICE_CCTV_SVG,
            Self::DeviceCctvOff => DEVICE_CCTV_OFF_SVG,
            Self::DeviceComputerCamera => DEVICE_COMPUTER_CAMERA_SVG,
            Self::DeviceComputerCamera2 => DEVICE_COMPUTER_CAMERA_2_SVG,
            Self::DeviceComputerCameraOff => DEVICE_COMPUTER_CAMERA_OFF_SVG,
            Self::DeviceDesktop => DEVICE_DESKTOP_SVG,
            Self::DeviceDesktopAnalytics => DEVICE_DESKTOP_ANALYTICS_SVG,
            Self::DeviceDesktopBolt => DEVICE_DESKTOP_BOLT_SVG,
            Self::DeviceDesktopCancel => DEVICE_DESKTOP_CANCEL_SVG,
            Self::DeviceDesktopCheck => DEVICE_DESKTOP_CHECK_SVG,
            Self::DeviceDesktopCode => DEVICE_DESKTOP_CODE_SVG,
            Self::DeviceDesktopCog => DEVICE_DESKTOP_COG_SVG,
            Self::DeviceDesktopDollar => DEVICE_DESKTOP_DOLLAR_SVG,
            Self::DeviceDesktopDown => DEVICE_DESKTOP_DOWN_SVG,
            Self::DeviceDesktopExclamation => DEVICE_DESKTOP_EXCLAMATION_SVG,
            Self::DeviceDesktopHeart => DEVICE_DESKTOP_HEART_SVG,
            Self::DeviceDesktopMinus => DEVICE_DESKTOP_MINUS_SVG,
            Self::DeviceDesktopOff => DEVICE_DESKTOP_OFF_SVG,
            Self::DeviceDesktopPause => DEVICE_DESKTOP_PAUSE_SVG,
            Self::DeviceDesktopPin => DEVICE_DESKTOP_PIN_SVG,
            Self::DeviceDesktopPlus => DEVICE_DESKTOP_PLUS_SVG,
            Self::DeviceDesktopQuestion => DEVICE_DESKTOP_QUESTION_SVG,
            Self::DeviceDesktopSearch => DEVICE_DESKTOP_SEARCH_SVG,
            Self::DeviceDesktopShare => DEVICE_DESKTOP_SHARE_SVG,
            Self::DeviceDesktopStar => DEVICE_DESKTOP_STAR_SVG,
            Self::DeviceDesktopUp => DEVICE_DESKTOP_UP_SVG,
            Self::DeviceDesktopX => DEVICE_DESKTOP_X_SVG,
            Self::DeviceFloppy => DEVICE_FLOPPY_SVG,
            Self::DeviceGamepad => DEVICE_GAMEPAD_SVG,
            Self::DeviceGamepad2 => DEVICE_GAMEPAD_2_SVG,
            Self::DeviceGamepad3 => DEVICE_GAMEPAD_3_SVG,
            Self::DeviceHeartMonitor => DEVICE_HEART_MONITOR_SVG,
            Self::DeviceImac => DEVICE_IMAC_SVG,
            Self::DeviceImacBolt => DEVICE_IMAC_BOLT_SVG,
            Self::DeviceImacCancel => DEVICE_IMAC_CANCEL_SVG,
            Self::DeviceImacCheck => DEVICE_IMAC_CHECK_SVG,
            Self::DeviceImacCode => DEVICE_IMAC_CODE_SVG,
            Self::DeviceImacCog => DEVICE_IMAC_COG_SVG,
            Self::DeviceImacDollar => DEVICE_IMAC_DOLLAR_SVG,
            Self::DeviceImacDown => DEVICE_IMAC_DOWN_SVG,
            Self::DeviceImacExclamation => DEVICE_IMAC_EXCLAMATION_SVG,
            Self::DeviceImacHeart => DEVICE_IMAC_HEART_SVG,
            Self::DeviceImacMinus => DEVICE_IMAC_MINUS_SVG,
            Self::DeviceImacOff => DEVICE_IMAC_OFF_SVG,
            Self::DeviceImacPause => DEVICE_IMAC_PAUSE_SVG,
            Self::DeviceImacPin => DEVICE_IMAC_PIN_SVG,
            Self::DeviceImacPlus => DEVICE_IMAC_PLUS_SVG,
            Self::DeviceImacQuestion => DEVICE_IMAC_QUESTION_SVG,
            Self::DeviceImacSearch => DEVICE_IMAC_SEARCH_SVG,
            Self::DeviceImacShare => DEVICE_IMAC_SHARE_SVG,
            Self::DeviceImacStar => DEVICE_IMAC_STAR_SVG,
            Self::DeviceImacUp => DEVICE_IMAC_UP_SVG,
            Self::DeviceImacX => DEVICE_IMAC_X_SVG,
            Self::DeviceIpad => DEVICE_IPAD_SVG,
            Self::DeviceIpadBolt => DEVICE_IPAD_BOLT_SVG,
            Self::DeviceIpadCancel => DEVICE_IPAD_CANCEL_SVG,
            Self::DeviceIpadCheck => DEVICE_IPAD_CHECK_SVG,
            Self::DeviceIpadCode => DEVICE_IPAD_CODE_SVG,
            Self::DeviceIpadCog => DEVICE_IPAD_COG_SVG,
            Self::DeviceIpadDollar => DEVICE_IPAD_DOLLAR_SVG,
            Self::DeviceIpadDown => DEVICE_IPAD_DOWN_SVG,
            Self::DeviceIpadExclamation => DEVICE_IPAD_EXCLAMATION_SVG,
            Self::DeviceIpadHeart => DEVICE_IPAD_HEART_SVG,
            Self::DeviceIpadHorizontal => DEVICE_IPAD_HORIZONTAL_SVG,
            Self::DeviceIpadHorizontalBolt => DEVICE_IPAD_HORIZONTAL_BOLT_SVG,
            Self::DeviceIpadHorizontalCancel => DEVICE_IPAD_HORIZONTAL_CANCEL_SVG,
            Self::DeviceIpadHorizontalCheck => DEVICE_IPAD_HORIZONTAL_CHECK_SVG,
            Self::DeviceIpadHorizontalCode => DEVICE_IPAD_HORIZONTAL_CODE_SVG,
            Self::DeviceIpadHorizontalCog => DEVICE_IPAD_HORIZONTAL_COG_SVG,
            Self::DeviceIpadHorizontalDollar => DEVICE_IPAD_HORIZONTAL_DOLLAR_SVG,
            Self::DeviceIpadHorizontalDown => DEVICE_IPAD_HORIZONTAL_DOWN_SVG,
            Self::DeviceIpadHorizontalExclamation => DEVICE_IPAD_HORIZONTAL_EXCLAMATION_SVG,
            Self::DeviceIpadHorizontalHeart => DEVICE_IPAD_HORIZONTAL_HEART_SVG,
            Self::DeviceIpadHorizontalMinus => DEVICE_IPAD_HORIZONTAL_MINUS_SVG,
            Self::DeviceIpadHorizontalOff => DEVICE_IPAD_HORIZONTAL_OFF_SVG,
            Self::DeviceIpadHorizontalPause => DEVICE_IPAD_HORIZONTAL_PAUSE_SVG,
            Self::DeviceIpadHorizontalPin => DEVICE_IPAD_HORIZONTAL_PIN_SVG,
            Self::DeviceIpadHorizontalPlus => DEVICE_IPAD_HORIZONTAL_PLUS_SVG,
            Self::DeviceIpadHorizontalQuestion => DEVICE_IPAD_HORIZONTAL_QUESTION_SVG,
            Self::DeviceIpadHorizontalSearch => DEVICE_IPAD_HORIZONTAL_SEARCH_SVG,
            Self::DeviceIpadHorizontalShare => DEVICE_IPAD_HORIZONTAL_SHARE_SVG,
            Self::DeviceIpadHorizontalStar => DEVICE_IPAD_HORIZONTAL_STAR_SVG,
            Self::DeviceIpadHorizontalUp => DEVICE_IPAD_HORIZONTAL_UP_SVG,
            Self::DeviceIpadHorizontalX => DEVICE_IPAD_HORIZONTAL_X_SVG,
            Self::DeviceIpadMinus => DEVICE_IPAD_MINUS_SVG,
            Self::DeviceIpadOff => DEVICE_IPAD_OFF_SVG,
            Self::DeviceIpadPause => DEVICE_IPAD_PAUSE_SVG,
            Self::DeviceIpadPin => DEVICE_IPAD_PIN_SVG,
            Self::DeviceIpadPlus => DEVICE_IPAD_PLUS_SVG,
            Self::DeviceIpadQuestion => DEVICE_IPAD_QUESTION_SVG,
            Self::DeviceIpadSearch => DEVICE_IPAD_SEARCH_SVG,
            Self::DeviceIpadShare => DEVICE_IPAD_SHARE_SVG,
            Self::DeviceIpadStar => DEVICE_IPAD_STAR_SVG,
            Self::DeviceIpadUp => DEVICE_IPAD_UP_SVG,
            Self::DeviceIpadX => DEVICE_IPAD_X_SVG,
            Self::DeviceLandlinePhone => DEVICE_LANDLINE_PHONE_SVG,
            Self::DeviceLaptop => DEVICE_LAPTOP_SVG,
            Self::DeviceLaptopOff => DEVICE_LAPTOP_OFF_SVG,
            Self::DeviceMobile => DEVICE_MOBILE_SVG,
            Self::DeviceMobileBolt => DEVICE_MOBILE_BOLT_SVG,
            Self::DeviceMobileCancel => DEVICE_MOBILE_CANCEL_SVG,
            Self::DeviceMobileCharging => DEVICE_MOBILE_CHARGING_SVG,
            Self::DeviceMobileCheck => DEVICE_MOBILE_CHECK_SVG,
            Self::DeviceMobileCode => DEVICE_MOBILE_CODE_SVG,
            Self::DeviceMobileCog => DEVICE_MOBILE_COG_SVG,
            Self::DeviceMobileDollar => DEVICE_MOBILE_DOLLAR_SVG,
            Self::DeviceMobileDown => DEVICE_MOBILE_DOWN_SVG,
            Self::DeviceMobileExclamation => DEVICE_MOBILE_EXCLAMATION_SVG,
            Self::DeviceMobileHeart => DEVICE_MOBILE_HEART_SVG,
            Self::DeviceMobileMessage => DEVICE_MOBILE_MESSAGE_SVG,
            Self::DeviceMobileMinus => DEVICE_MOBILE_MINUS_SVG,
            Self::DeviceMobileOff => DEVICE_MOBILE_OFF_SVG,
            Self::DeviceMobilePause => DEVICE_MOBILE_PAUSE_SVG,
            Self::DeviceMobilePin => DEVICE_MOBILE_PIN_SVG,
            Self::DeviceMobilePlus => DEVICE_MOBILE_PLUS_SVG,
            Self::DeviceMobileQuestion => DEVICE_MOBILE_QUESTION_SVG,
            Self::DeviceMobileRotated => DEVICE_MOBILE_ROTATED_SVG,
            Self::DeviceMobileSearch => DEVICE_MOBILE_SEARCH_SVG,
            Self::DeviceMobileShare => DEVICE_MOBILE_SHARE_SVG,
            Self::DeviceMobileStar => DEVICE_MOBILE_STAR_SVG,
            Self::DeviceMobileUp => DEVICE_MOBILE_UP_SVG,
            Self::DeviceMobileVibration => DEVICE_MOBILE_VIBRATION_SVG,
            Self::DeviceMobileX => DEVICE_MOBILE_X_SVG,
            Self::DeviceNintendo => DEVICE_NINTENDO_SVG,
            Self::DeviceNintendoOff => DEVICE_NINTENDO_OFF_SVG,
            Self::DeviceRemote => DEVICE_REMOTE_SVG,
            Self::DeviceScreen => DEVICE_SCREEN_SVG,
            Self::DeviceSdCard => DEVICE_SD_CARD_SVG,
            Self::DeviceSim => DEVICE_SIM_SVG,
            Self::DeviceSim1 => DEVICE_SIM_1_SVG,
            Self::DeviceSim2 => DEVICE_SIM_2_SVG,
            Self::DeviceSim3 => DEVICE_SIM_3_SVG,
            Self::DeviceSpeaker => DEVICE_SPEAKER_SVG,
            Self::DeviceSpeakerOff => DEVICE_SPEAKER_OFF_SVG,
            Self::DeviceTablet => DEVICE_TABLET_SVG,
            Self::DeviceTabletBolt => DEVICE_TABLET_BOLT_SVG,
            Self::DeviceTabletCancel => DEVICE_TABLET_CANCEL_SVG,
            Self::DeviceTabletCheck => DEVICE_TABLET_CHECK_SVG,
            Self::DeviceTabletCode => DEVICE_TABLET_CODE_SVG,
            Self::DeviceTabletCog => DEVICE_TABLET_COG_SVG,
            Self::DeviceTabletDollar => DEVICE_TABLET_DOLLAR_SVG,
            Self::DeviceTabletDown => DEVICE_TABLET_DOWN_SVG,
            Self::DeviceTabletExclamation => DEVICE_TABLET_EXCLAMATION_SVG,
            Self::DeviceTabletHeart => DEVICE_TABLET_HEART_SVG,
            Self::DeviceTabletMinus => DEVICE_TABLET_MINUS_SVG,
            Self::DeviceTabletOff => DEVICE_TABLET_OFF_SVG,
            Self::DeviceTabletPause => DEVICE_TABLET_PAUSE_SVG,
            Self::DeviceTabletPin => DEVICE_TABLET_PIN_SVG,
            Self::DeviceTabletPlus => DEVICE_TABLET_PLUS_SVG,
            Self::DeviceTabletQuestion => DEVICE_TABLET_QUESTION_SVG,
            Self::DeviceTabletSearch => DEVICE_TABLET_SEARCH_SVG,
            Self::DeviceTabletShare => DEVICE_TABLET_SHARE_SVG,
            Self::DeviceTabletStar => DEVICE_TABLET_STAR_SVG,
            Self::DeviceTabletUp => DEVICE_TABLET_UP_SVG,
            Self::DeviceTabletX => DEVICE_TABLET_X_SVG,
            Self::DeviceTv => DEVICE_TV_SVG,
            Self::DeviceTvOff => DEVICE_TV_OFF_SVG,
            Self::DeviceTvOld => DEVICE_TV_OLD_SVG,
            Self::DeviceUsb => DEVICE_USB_SVG,
            Self::DeviceVisionPro => DEVICE_VISION_PRO_SVG,
            Self::DeviceWatch => DEVICE_WATCH_SVG,
            Self::DeviceWatchBolt => DEVICE_WATCH_BOLT_SVG,
            Self::DeviceWatchCancel => DEVICE_WATCH_CANCEL_SVG,
            Self::DeviceWatchCheck => DEVICE_WATCH_CHECK_SVG,
            Self::DeviceWatchCode => DEVICE_WATCH_CODE_SVG,
            Self::DeviceWatchCog => DEVICE_WATCH_COG_SVG,
            Self::DeviceWatchDollar => DEVICE_WATCH_DOLLAR_SVG,
            Self::DeviceWatchDown => DEVICE_WATCH_DOWN_SVG,
            Self::DeviceWatchExclamation => DEVICE_WATCH_EXCLAMATION_SVG,
            Self::DeviceWatchHeart => DEVICE_WATCH_HEART_SVG,
            Self::DeviceWatchMinus => DEVICE_WATCH_MINUS_SVG,
            Self::DeviceWatchOff => DEVICE_WATCH_OFF_SVG,
            Self::DeviceWatchPause => DEVICE_WATCH_PAUSE_SVG,
            Self::DeviceWatchPin => DEVICE_WATCH_PIN_SVG,
            Self::DeviceWatchPlus => DEVICE_WATCH_PLUS_SVG,
            Self::DeviceWatchQuestion => DEVICE_WATCH_QUESTION_SVG,
            Self::DeviceWatchSearch => DEVICE_WATCH_SEARCH_SVG,
            Self::DeviceWatchShare => DEVICE_WATCH_SHARE_SVG,
            Self::DeviceWatchStar => DEVICE_WATCH_STAR_SVG,
            Self::DeviceWatchStats => DEVICE_WATCH_STATS_SVG,
            Self::DeviceWatchStats2 => DEVICE_WATCH_STATS_2_SVG,
            Self::DeviceWatchUp => DEVICE_WATCH_UP_SVG,
            Self::DeviceWatchX => DEVICE_WATCH_X_SVG,
            Self::Devices => DEVICES_SVG,
            Self::Devices2 => DEVICES_2_SVG,
            Self::DevicesBolt => DEVICES_BOLT_SVG,
            Self::DevicesCancel => DEVICES_CANCEL_SVG,
            Self::DevicesCheck => DEVICES_CHECK_SVG,
            Self::DevicesCode => DEVICES_CODE_SVG,
            Self::DevicesCog => DEVICES_COG_SVG,
            Self::DevicesDollar => DEVICES_DOLLAR_SVG,
            Self::DevicesDown => DEVICES_DOWN_SVG,
            Self::DevicesExclamation => DEVICES_EXCLAMATION_SVG,
            Self::DevicesHeart => DEVICES_HEART_SVG,
            Self::DevicesMinus => DEVICES_MINUS_SVG,
            Self::DevicesOff => DEVICES_OFF_SVG,
            Self::DevicesPause => DEVICES_PAUSE_SVG,
            Self::DevicesPc => DEVICES_PC_SVG,
            Self::DevicesPcOff => DEVICES_PC_OFF_SVG,
            Self::DevicesPin => DEVICES_PIN_SVG,
            Self::DevicesPlus => DEVICES_PLUS_SVG,
            Self::DevicesQuestion => DEVICES_QUESTION_SVG,
            Self::DevicesSearch => DEVICES_SEARCH_SVG,
            Self::DevicesShare => DEVICES_SHARE_SVG,
            Self::DevicesStar => DEVICES_STAR_SVG,
            Self::DevicesUp => DEVICES_UP_SVG,
            Self::DevicesX => DEVICES_X_SVG,
            Self::Dialpad => DIALPAD_SVG,
            Self::DialpadOff => DIALPAD_OFF_SVG,
            Self::Disc => DISC_SVG,
            Self::DiscOff => DISC_OFF_SVG,
            Self::DualScreen => DUAL_SCREEN_SVG,
            Self::EarphoneBluetooth => EARPHONE_BLUETOOTH_SVG,
            Self::Error404 => ERROR_404_SVG,
            Self::Error404Off => ERROR_404_OFF_SVG,
            Self::Fridge => FRIDGE_SVG,
            Self::FridgeOff => FRIDGE_OFF_SVG,
            Self::Gif => GIF_SVG,
            Self::GitBranch => GIT_BRANCH_SVG,
            Self::GitBranchDeleted => GIT_BRANCH_DELETED_SVG,
            Self::GitCherryPick => GIT_CHERRY_PICK_SVG,
            Self::GitCommit => GIT_COMMIT_SVG,
            Self::GitCompare => GIT_COMPARE_SVG,
            Self::GitFork => GIT_FORK_SVG,
            Self::GitMerge => GIT_MERGE_SVG,
            Self::GitPullRequest => GIT_PULL_REQUEST_SVG,
            Self::GitPullRequestClosed => GIT_PULL_REQUEST_CLOSED_SVG,
            Self::GitPullRequestConflict => GIT_PULL_REQUEST_CONFLICT_SVG,
            Self::GitPullRequestDraft => GIT_PULL_REQUEST_DRAFT_SVG,
            Self::HammerDrill => HAMMER_DRILL_SVG,
            Self::Html => HTML_SVG,
            Self::HttpConnect => HTTP_CONNECT_SVG,
            Self::HttpConnectOff => HTTP_CONNECT_OFF_SVG,
            Self::HttpDelete => HTTP_DELETE_SVG,
            Self::HttpDeleteOff => HTTP_DELETE_OFF_SVG,
            Self::HttpGet => HTTP_GET_SVG,
            Self::HttpGetOff => HTTP_GET_OFF_SVG,
            Self::HttpHead => HTTP_HEAD_SVG,
            Self::HttpHeadOff => HTTP_HEAD_OFF_SVG,
            Self::HttpOptions => HTTP_OPTIONS_SVG,
            Self::HttpOptionsOff => HTTP_OPTIONS_OFF_SVG,
            Self::HttpPatch => HTTP_PATCH_SVG,
            Self::HttpPatchOff => HTTP_PATCH_OFF_SVG,
            Self::HttpPost => HTTP_POST_SVG,
            Self::HttpPostOff => HTTP_POST_OFF_SVG,
            Self::HttpPut => HTTP_PUT_SVG,
            Self::HttpPutOff => HTTP_PUT_OFF_SVG,
            Self::HttpQue => HTTP_QUE_SVG,
            Self::HttpQueOff => HTTP_QUE_OFF_SVG,
            Self::HttpTrace => HTTP_TRACE_SVG,
            Self::HttpTraceOff => HTTP_TRACE_OFF_SVG,
            Self::Jpg => JPG_SVG,
            Self::Json => JSON_SVG,
            Self::Keyboard => KEYBOARD_SVG,
            Self::KeyboardHide => KEYBOARD_HIDE_SVG,
            Self::KeyboardOff => KEYBOARD_OFF_SVG,
            Self::KeyboardShow => KEYBOARD_SHOW_SVG,
            Self::LawnMower => LAWN_MOWER_SVG,
            Self::LoadBalancer => LOAD_BALANCER_SVG,
            Self::Mouse => MOUSE_SVG,
            Self::Mouse2 => MOUSE_2_SVG,
            Self::MouseOff => MOUSE_OFF_SVG,
            Self::Network => NETWORK_SVG,
            Self::NetworkOff => NETWORK_OFF_SVG,
            Self::Nfc => NFC_SVG,
            Self::NfcOff => NFC_OFF_SVG,
            Self::Pdf => PDF_SVG,
            Self::Phone => PHONE_SVG,
            Self::PhoneCall => PHONE_CALL_SVG,
            Self::PhoneCalling => PHONE_CALLING_SVG,
            Self::PhoneCheck => PHONE_CHECK_SVG,
            Self::PhoneIncoming => PHONE_INCOMING_SVG,
            Self::PhoneOff => PHONE_OFF_SVG,
            Self::PhoneOutgoing => PHONE_OUTGOING_SVG,
            Self::PhonePause => PHONE_PAUSE_SVG,
            Self::PhonePlus => PHONE_PLUS_SVG,
            Self::PhoneSpark => PHONE_SPARK_SVG,
            Self::PhoneX => PHONE_X_SVG,
            Self::PlaystationCircle => PLAYSTATION_CIRCLE_SVG,
            Self::PlaystationSquare => PLAYSTATION_SQUARE_SVG,
            Self::PlaystationTriangle => PLAYSTATION_TRIANGLE_SVG,
            Self::PlaystationX => PLAYSTATION_X_SVG,
            Self::Plug => PLUG_SVG,
            Self::PlugConnected => PLUG_CONNECTED_SVG,
            Self::PlugConnectedX => PLUG_CONNECTED_X_SVG,
            Self::PlugOff => PLUG_OFF_SVG,
            Self::PlugX => PLUG_X_SVG,
            Self::Png => PNG_SVG,
            Self::Power => POWER_SVG,
            Self::Printer => PRINTER_SVG,
            Self::PrinterOff => PRINTER_OFF_SVG,
            Self::Qrcode => QRCODE_SVG,
            Self::QrcodeOff => QRCODE_OFF_SVG,
            Self::Router => ROUTER_SVG,
            Self::RouterOff => ROUTER_OFF_SVG,
            Self::ScreenShare => SCREEN_SHARE_SVG,
            Self::ScreenShareOff => SCREEN_SHARE_OFF_SVG,
            Self::Server => SERVER_SVG,
            Self::Server2 => SERVER_2_SVG,
            Self::ServerBolt => SERVER_BOLT_SVG,
            Self::ServerCog => SERVER_COG_SVG,
            Self::ServerOff => SERVER_OFF_SVG,
            Self::ServerSpark => SERVER_SPARK_SVG,
            Self::Shredder => SHREDDER_SVG,
            Self::Signal2g => SIGNAL_2G_SVG,
            Self::Signal3g => SIGNAL_3G_SVG,
            Self::Signal4g => SIGNAL_4G_SVG,
            Self::Signal4gPlus => SIGNAL_4G_PLUS_SVG,
            Self::Signal5g => SIGNAL_5G_SVG,
            Self::Signal6g => SIGNAL_6G_SVG,
            Self::SignalE => SIGNAL_E_SVG,
            Self::SignalG => SIGNAL_G_SVG,
            Self::SignalH => SIGNAL_H_SVG,
            Self::SignalHPlus => SIGNAL_H_PLUS_SVG,
            Self::SignalLte => SIGNAL_LTE_SVG,
            Self::Sql => SQL_SVG,
            Self::Svg => SVG_SVG,
            Self::Toml => TOML_SVG,
            Self::TopologyBus => TOPOLOGY_BUS_SVG,
            Self::TopologyComplex => TOPOLOGY_COMPLEX_SVG,
            Self::TopologyFull => TOPOLOGY_FULL_SVG,
            Self::TopologyFullHierarchy => TOPOLOGY_FULL_HIERARCHY_SVG,
            Self::TopologyRing => TOPOLOGY_RING_SVG,
            Self::TopologyRing2 => TOPOLOGY_RING_2_SVG,
            Self::TopologyRing3 => TOPOLOGY_RING_3_SVG,
            Self::TopologyStar => TOPOLOGY_STAR_SVG,
            Self::TopologyStar2 => TOPOLOGY_STAR_2_SVG,
            Self::TopologyStar3 => TOPOLOGY_STAR_3_SVG,
            Self::TopologyStarRing => TOPOLOGY_STAR_RING_SVG,
            Self::TopologyStarRing2 => TOPOLOGY_STAR_RING_2_SVG,
            Self::TopologyStarRing3 => TOPOLOGY_STAR_RING_3_SVG,
            Self::Txt => TXT_SVG,
            Self::ViewportNarrow => VIEWPORT_NARROW_SVG,
            Self::ViewportWide => VIEWPORT_WIDE_SVG,
            Self::Vinyl => VINYL_SVG,
            Self::WashMachine => WASH_MACHINE_SVG,
            Self::Wifi => WIFI_SVG,
            Self::Wifi0 => WIFI_0_SVG,
            Self::Wifi1 => WIFI_1_SVG,
            Self::Wifi2 => WIFI_2_SVG,
            Self::WifiOff => WIFI_OFF_SVG,
            Self::XboxA => XBOX_A_SVG,
            Self::XboxB => XBOX_B_SVG,
            Self::XboxX => XBOX_X_SVG,
            Self::XboxY => XBOX_Y_SVG,
            Self::Zip => ZIP_SVG,
        }
    }

    fn filled_svg(&self) -> Option<&'static str> {
        // Filled variants would be added here
        None
    }
}
