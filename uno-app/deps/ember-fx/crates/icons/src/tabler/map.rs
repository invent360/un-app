//! Map icons from Tabler Icons.
//!
//! This module contains 488 icons.

use crate::tabler::TablerIconData;

// SVG Constants
const ACORN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 10l-.45 4.1a8.36 8.36 0 0 1 -5.18 6.83a1 1 0 0 1 -.74 0a8.36 8.36 0 0 1 -5.18 -6.83l-.45 -4.1" /> <path d="M13 3a4.9 4.9 0 0 0 -1 3" /> <path d="M8 6h8a3 3 0 0 1 3 3a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1a3 3 0 0 1 3 -3" /> </svg>"##;
const AERIAL_LIFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 5l16 -2" /> <path d="M12 4v10" /> <path d="M6.894 8h10.306c2.45 3 2.45 9 -.2 12h-10.106c-2.544 -3 -2.544 -9 0 -12" /> <path d="M5 14h14" /> </svg>"##;
const AIR_BALLOON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 21v-3h6v3a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1" /> <path d="M9 18c-2.347 -2.169 -5 -5.226 -5 -8a8 8 0 1 1 16 0c0 2.774 -2.653 5.831 -5 8" /> <path d="M5.5 14h13" /> <path d="M10 14c-1.69 -4.712 -.924 -8.197 0 -11.602" /> <path d="M14 14c1.469 -3.867 1.19 -7.735 0 -11.602" /> </svg>"##;
const AIR_TRAFFIC_CONTROL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 3h2" /> <path d="M12 3v3" /> <path d="M5.998 6h12.004a2 2 0 0 1 1.916 2.575l-1.8 6a2 2 0 0 1 -1.916 1.425h-8.404a2 2 0 0 1 -1.916 -1.425l-1.8 -6a2 2 0 0 1 1.916 -2.575" /> <path d="M8.5 6l1.5 10v5" /> <path d="M15.5 6l-1.5 10v5" /> </svg>"##;
const ALIEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 17a2.5 2.5 0 0 0 2 0" /> <path d="M12 3c-4.664 0 -7.396 2.331 -7.862 5.595a11.816 11.816 0 0 0 2 8.592a10.777 10.777 0 0 0 3.199 3.064c1.666 1 3.664 1 5.33 0a10.777 10.777 0 0 0 3.199 -3.064a11.89 11.89 0 0 0 2 -8.592c-.466 -3.265 -3.198 -5.595 -7.862 -5.595l-.004 0" /> <path d="M8 11l2 2" /> <path d="M16 11l-2 2" /> </svg>"##;
const AMBULANCE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 17h-2v-11a1 1 0 0 1 1 -1h9v12m-4 0h6m4 0h2v-6h-8m0 -5h5l3 5" /> <path d="M6 10h4m-2 -2v4" /> </svg>"##;
const ANCHOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 9v12m-8 -8a8 8 0 0 0 16 0m1 0h-2m-14 0h-2" /> <path d="M9 6a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const ANCHOR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12v9" /> <path d="M4 13a8 8 0 0 0 14.138 5.13m1.44 -2.56a7.99 7.99 0 0 0 .422 -2.57" /> <path d="M21 13h-2" /> <path d="M5 13h-2" /> <path d="M12.866 8.873a3 3 0 1 0 -3.737 -3.747" /> <path d="M3 3l18 18" /> </svg>"##;
const ATOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12v.01" /> <path d="M19.071 4.929c-1.562 -1.562 -6 .337 -9.9 4.243c-3.905 3.905 -5.804 8.337 -4.242 9.9c1.562 1.561 6 -.338 9.9 -4.244c3.905 -3.905 5.804 -8.337 4.242 -9.9" /> <path d="M4.929 4.929c-1.562 1.562 .337 6 4.243 9.9c3.905 3.905 8.337 5.804 9.9 4.242c1.561 -1.562 -.338 -6 -4.244 -9.9c-3.905 -3.905 -8.337 -5.804 -9.9 -4.242" /> </svg>"##;
const ATOM_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M12 21l0 .01" /> <path d="M3 9l0 .01" /> <path d="M21 9l0 .01" /> <path d="M8 20.1a9 9 0 0 1 -5 -7.1" /> <path d="M16 20.1a9 9 0 0 0 5 -7.1" /> <path d="M6.2 5a9 9 0 0 1 11.4 0" /> </svg>"##;
const ATOM_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12v.01" /> <path d="M9.172 9.172c-3.906 3.905 -5.805 8.337 -4.243 9.9c1.562 1.561 6 -.338 9.9 -4.244m1.884 -2.113c2.587 -3.277 3.642 -6.502 2.358 -7.786c-1.284 -1.284 -4.508 -.23 -7.784 2.357" /> <path d="M4.929 4.929c-1.562 1.562 .337 6 4.243 9.9c3.905 3.905 8.337 5.804 9.9 4.242m-.072 -4.071c-.767 -1.794 -2.215 -3.872 -4.172 -5.828c-1.944 -1.945 -4.041 -3.402 -5.828 -4.172" /> <path d="M3 3l18 18" /> </svg>"##;
const AUTOMATIC_GEARBOX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 17v4h1a2 2 0 1 0 0 -4h-1" /> <path d="M17 11h1.5a1.5 1.5 0 0 0 0 -3h-1.5v5" /> <path d="M3 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 7v3a1 1 0 0 0 1 1h3v7a1 1 0 0 0 1 1h3" /> <path d="M9 11h4" /> </svg>"##;
const BACKHOE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M11 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M13 19l-9 0" /> <path d="M4 15l9 0" /> <path d="M8 12v-5h2a3 3 0 0 1 3 3v5" /> <path d="M5 15v-2a1 1 0 0 1 1 -1h7" /> <path d="M21.12 9.88l-3.12 -4.88l-5 5" /> <path d="M21.12 9.88a3 3 0 0 1 -2.12 5.12a3 3 0 0 1 -2.12 -.88l4.24 -4.24" /> </svg>"##;
const BALLOON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8a2 2 0 0 0 -2 -2" /> <path d="M6 8a6 6 0 1 1 12 0c0 4.97 -2.686 9 -6 9s-6 -4.03 -6 -9" /> <path d="M12 17v1a2 2 0 0 1 -2 2h-3a2 2 0 0 0 -2 2" /> </svg>"##;
const BALLOON_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 8a2 2 0 0 0 -2 -2" /> <path d="M7.762 3.753a6 6 0 0 1 10.238 4.247c0 1.847 -.37 3.564 -1.007 4.993m-1.59 2.42c-.967 1 -2.14 1.587 -3.403 1.587c-3.314 0 -6 -4.03 -6 -9c0 -.593 .086 -1.166 .246 -1.707" /> <path d="M12 17v1a2 2 0 0 1 -2 2h-3a2 2 0 0 0 -2 2" /> <path d="M3 3l18 18" /> </svg>"##;
const BAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 16c.74 -2.286 2.778 -3.762 5 -3c-.173 -2.595 .13 -5.314 -2 -7.5c-1.708 2.648 -3.358 2.557 -5 2.5v-4l-3 2l-3 -2v4c-1.642 .057 -3.292 .148 -5 -2.5c-2.13 2.186 -1.827 4.905 -2 7.5c2.222 -.762 4.26 .714 5 3c2.593 0 3.889 .952 5 4c1.111 -3.048 2.407 -4 5 -4" /> <path d="M9 8a3 3 0 0 0 6 0" /> </svg>"##;
const BATTERY_AUTOMOTIVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -10" /> <path d="M6 5v-2" /> <path d="M18 3v2" /> <path d="M6.5 12h3" /> <path d="M14.5 12h3" /> <path d="M16 10.5v3" /> </svg>"##;
const BEACH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.553 16.75a7.5 7.5 0 0 0 -10.606 0" /> <path d="M18 3.804a6 6 0 0 0 -8.196 2.196l10.392 6a6 6 0 0 0 -2.196 -8.196" /> <path d="M16.732 10c1.658 -2.87 2.225 -5.644 1.268 -6.196c-.957 -.552 -3.075 1.326 -4.732 4.196" /> <path d="M15 9l-3 5.196" /> <path d="M3 19.25a2.4 2.4 0 0 1 1 -.25a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 1 .25" /> </svg>"##;
const BEACH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.071 15.102a7.502 7.502 0 0 0 -8.124 1.648" /> <path d="M10.27 6.269l9.926 5.731a6 6 0 0 0 -10.32 -6.123" /> <path d="M16.732 10c1.658 -2.87 2.225 -5.644 1.268 -6.196c-.957 -.552 -3.075 1.326 -4.732 4.196" /> <path d="M15 9l-.739 1.279" /> <path d="M12.794 12.82l-.794 1.376" /> <path d="M3 19.25a2.4 2.4 0 0 1 1 -.25a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 1.135 -.858" /> <path d="M3 3l18 18" /> </svg>"##;
const BED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 9a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M22 17v-3h-20" /> <path d="M2 8v9" /> <path d="M12 14h10v-2a3 3 0 0 0 -3 -3h-7v5" /> </svg>"##;
const BED_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7a2 2 0 1 0 2 2" /> <path d="M22 17v-3h-4m-4 0h-12" /> <path d="M2 8v9" /> <path d="M12 12v2h2m4 0h4v-2a3 3 0 0 0 -3 -3h-6" /> <path d="M3 3l18 18" /> </svg>"##;
const BIKE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 18a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M16 18a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M12 19v-4l-3 -3l5 -4l2 3h3" /> <path d="M13.007 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const BIKE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M16.437 16.44a3 3 0 0 0 4.123 4.123m1.44 -2.563a3 3 0 0 0 -3 -3" /> <path d="M12 19v-4l-3 -3l1.665 -1.332m2.215 -1.772l1.12 -.896l2 3h3" /> <path d="M16 5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3 3l18 18" /> </svg>"##;
const BULLDOZER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 17a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 17a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M19 13v4a2 2 0 0 0 2 2h1" /> <path d="M14 19h-10" /> <path d="M4 15h10" /> <path d="M9 11v-5h2a3 3 0 0 1 3 3v6" /> <path d="M5 15v-3a1 1 0 0 1 1 -1h8" /> <path d="M19 17h-3" /> </svg>"##;
const BUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 17h-2v-11a1 1 0 0 1 1 -1h14a5 7 0 0 1 5 7v5h-2m-4 0h-8" /> <path d="M16 5l1.5 7l4.5 0" /> <path d="M2 10l15 0" /> <path d="M7 5l0 5" /> <path d="M12 5l0 5" /> </svg>"##;
const BUS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16.18 16.172a2 2 0 0 0 2.652 2.648" /> <path d="M4 17h-2v-11a1 1 0 0 1 1 -1h2m4 0h8c2.761 0 5 3.134 5 7v5h-1m-5 0h-8" /> <path d="M16 5l1.5 7h4.5" /> <path d="M2 10h8m4 0h3" /> <path d="M7 7v3" /> <path d="M12 5v3" /> <path d="M3 3l18 18" /> </svg>"##;
const BUS_STOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -4" /> <path d="M16 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 5h7c2.761 0 5 3.134 5 7v5h-2" /> <path d="M16 17h-8" /> <path d="M16 5l1.5 7h4.5" /> <path d="M9.5 10h7.5" /> <path d="M12 5v5" /> <path d="M5 9v11" /> </svg>"##;
const BUTTERFLY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.176a3 3 0 1 1 -4.953 -2.449l-.025 .023a4.502 4.502 0 0 1 1.483 -8.75c1.414 0 2.675 .652 3.5 1.671a4.5 4.5 0 1 1 4.983 7.079a3 3 0 1 1 -4.983 2.25l-.005 .176" /> <path d="M12 19v-10" /> <path d="M9 3l3 2l3 -2" /> </svg>"##;
const CACTUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 9v1a3 3 0 0 0 3 3h1" /> <path d="M18 8v5a3 3 0 0 1 -3 3h-1" /> <path d="M10 21v-16a2 2 0 1 1 4 0v16" /> <path d="M7 21h10" /> </svg>"##;
const CACTUS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 9v1a3 3 0 0 0 3 3h1" /> <path d="M18 8v5a3 3 0 0 1 -.129 .872m-2.014 2a3 3 0 0 1 -.857 .124h-1" /> <path d="M10 21v-11m0 -4v-1a2 2 0 1 1 4 0v5m0 4v7" /> <path d="M7 21h10" /> <path d="M3 3l18 18" /> </svg>"##;
const CAMPER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 18a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M15 18a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M5 18h-1a1 1 0 0 1 -1 -1v-11a2 2 0 0 1 2 -2h12a4 4 0 0 1 4 4h-18" /> <path d="M9 18h6" /> <path d="M19 18h1a1 1 0 0 0 1 -1v-4l-3 -5" /> <path d="M21 13h-7" /> <path d="M14 8v10" /> </svg>"##;
const CAMPFIRE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 21l16 -4" /> <path d="M20 21l-16 -4" /> <path d="M12 15a4 4 0 0 0 4 -4c0 -3 -2 -3 -2 -8c-4 2 -6 5 -6 8a4 4 0 0 0 4 4" /> </svg>"##;
const CANNABIS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 20s0 -2 1 -3.5c-1.5 0 -2 -.5 -4 -1.5c0 0 1.839 -1.38 5 -1c-1.789 -.97 -3.279 -2.03 -5 -6c0 0 3.98 -.3 6.5 3.5c-2.284 -4.9 1.5 -9.5 1.5 -9.5c2.734 5.47 2.389 7.5 1.5 9.5c2.531 -3.77 6.5 -3.5 6.5 -3.5c-1.721 3.97 -3.211 5.03 -5 6c3.161 -.38 5 1 5 1c-2 1 -2.5 1.5 -4 1.5c1 1.5 1 3.5 1 3.5c-2 0 -4.438 -2.22 -5 -3c-.563 .78 -3 3 -5 3" /> <path d="M12 22v-5" /> </svg>"##;
const CAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 17h-2v-6l2 -5h9l4 5h1a2 2 0 0 1 2 2v4h-2m-4 0h-6m-6 -6h15m-6 0v-5" /> </svg>"##;
const CAR_4WD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a2 2 0 0 1 2 -2a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2a2 2 0 0 1 -2 -2l0 -2" /> <path d="M5 17a2 2 0 0 1 2 -2a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2a2 2 0 0 1 -2 -2l0 -2" /> <path d="M15 5a2 2 0 0 1 2 -2a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2a2 2 0 0 1 -2 -2l0 -2" /> <path d="M15 17a2 2 0 0 1 2 -2a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2a2 2 0 0 1 -2 -2l0 -2" /> <path d="M9 18h6" /> <path d="M9 6h6" /> <path d="M12 6.5v-.5v12" /> </svg>"##;
const CAR_CRANE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 18h8m4 0h2v-6a5 5 0 0 0 -5 -5h-1l1.5 5h4.5" /> <path d="M12 18v-11h3" /> <path d="M3 17v-5h9" /> <path d="M4 12v-6l18 -3v2" /> <path d="M8 12v-4l-4 -2" /> </svg>"##;
const CAR_CRASH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 6l4 5h1a2 2 0 0 1 2 2v4h-2m-4 0h-5m0 -6h8m-6 0v-5m2 0h-4" /> <path d="M14 8v-2" /> <path d="M19 12h2" /> <path d="M17.5 15.5l1.5 1.5" /> <path d="M17.5 8.5l1.5 -1.5" /> </svg>"##;
const CAR_DOOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 14h2" /> <path d="M19 10h-16" /> <path d="M6.7 3.45l-3.7 5.55v3.08a1 1 0 0 0 .85 1a6 6 0 0 1 5.15 5.92v1a1 1 0 0 0 1 1h8a1 1 0 0 0 1 -1v-16a1 1 0 0 0 -1 -1h-10.46a1 1 0 0 0 -.84 .45" /> </svg>"##;
const CAR_FAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12v-9l4.912 1.914a1.7 1.7 0 0 1 .428 2.925l-5.34 4.161" /> <path d="M12 12h9l-1.914 4.912a1.7 1.7 0 0 1 -2.925 .428l-4.161 -5.34" /> <path d="M12 12h-9l1.914 -4.912a1.7 1.7 0 0 1 2.925 -.428l4.161 5.34" /> <path d="M12 12v9l-4.912 -1.914a1.7 1.7 0 0 1 -.428 -2.925l5.34 -4.161" /> </svg>"##;
const CAR_FAN_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12v-9l4.912 1.914a1.7 1.7 0 0 1 .428 2.925l-5.34 4.161" /> <path d="M14.424 15.03l-2.424 -3.03h6" /> <path d="M12 12h-9l1.914 -4.912a1.7 1.7 0 0 1 2.925 -.428l4.161 5.34" /> <path d="M12 12v9l-4.912 -1.914a1.7 1.7 0 0 1 -.428 -2.925l5.34 -4.161" /> <path d="M18 17l2 -2v6" /> </svg>"##;
const CAR_FAN_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12v-9l4.912 1.914a1.7 1.7 0 0 1 .428 2.925l-5.34 4.161" /> <path d="M14.044 14.624l-2.044 -2.624h4" /> <path d="M12 12h-9l1.914 -4.912a1.7 1.7 0 0 1 2.925 -.428l4.161 5.34" /> <path d="M12 12v9l-4.912 -1.914a1.7 1.7 0 0 1 -.428 -2.925l5.34 -4.161" /> <path d="M18 15h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2" /> </svg>"##;
const CAR_FAN_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12v-9l4.912 1.914a1.7 1.7 0 0 1 .428 2.925l-5.34 4.161" /> <path d="M14.044 14.624l-2.044 -2.624h4" /> <path d="M12 12h-9l1.914 -4.912a1.7 1.7 0 0 1 2.925 -.428l4.161 5.34" /> <path d="M12 12v9l-4.912 -1.914a1.7 1.7 0 0 1 -.428 -2.925l5.34 -4.161" /> <path d="M18 15.5a.5 .5 0 0 1 .5 -.5h1a1.5 1.5 0 0 1 0 3h-.5h.5a1.5 1.5 0 0 1 0 3h-1a.5 .5 0 0 1 -.5 -.5" /> </svg>"##;
const CAR_FAN_AUTO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12v-9l4.912 1.914a1.7 1.7 0 0 1 .428 2.925l-5.34 4.161" /> <path d="M14.044 14.624l-2.044 -2.624h4" /> <path d="M12 12h-9l1.914 -4.912a1.7 1.7 0 0 1 2.925 -.428l4.161 5.34" /> <path d="M12 12v9l-4.912 -1.914a1.7 1.7 0 0 1 -.428 -2.925l5.34 -4.161" /> <path d="M17 21v-4a2 2 0 1 1 4 0v4" /> <path d="M17 19h4" /> </svg>"##;
const CAR_LIFTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 21l10 -7l-10 -7" /> <path d="M17 7l-10 7l10 7" /> <path d="M20 7h-16a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1" /> <path d="M3 21h18" /> </svg>"##;
const CAR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15.584 15.588a2 2 0 0 0 2.828 2.83" /> <path d="M5 17h-2v-6l2 -5h1m4 0h4l4 5h1a2 2 0 0 1 2 2v4m-6 0h-6m-6 -6h8m4 0h3m-6 -3v-2" /> <path d="M3 3l18 18" /> </svg>"##;
const CAR_OFF_ROAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 17h6" /> <path d="M9 17a2 2 0 1 1 -4 0a2 2 0 0 1 4 0" /> <path d="M19 17a2 2 0 1 1 -4 0a2 2 0 0 1 4 0" /> <path d="M17 10l-2 -3" /> <path d="M19 17h2v-5a2 2 0 0 0 -2 -2h-5v2h-2.586a1 1 0 0 1 -.707 -.293l-1.121 -1.121a2 2 0 0 0 -1.414 -.586h-4.172a1 1 0 0 0 -1 1v6h2" /> </svg>"##;
const CAR_SUSPENSION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 22a3 3 0 1 1 0 -6a3 3 0 0 1 0 6" /> <path d="M12 16v-12" /> <path d="M13 2h-2v2h2v-2" /> <path d="M9 11l6 -1" /> <path d="M9 14l6 -1" /> <path d="M9 8l6 -1" /> </svg>"##;
const CAR_SUV_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 17a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M16 17a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M5 9l2 -4h7.438a2 2 0 0 1 1.94 1.515l.622 2.485h3a2 2 0 0 1 2 2v3" /> <path d="M10 9v-4" /> <path d="M2 7v4" /> <path d="M22.001 14.001a4.992 4.992 0 0 0 -4.001 -2.001a4.992 4.992 0 0 0 -4 2h-3a4.998 4.998 0 0 0 -8.003 .003" /> <path d="M5 12v-3h13" /> </svg>"##;
const CAR_TURBINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 13a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M18.86 11c.088 .66 .14 1.512 .14 2a8 8 0 1 1 -8 -8h6" /> <path d="M11 9c2.489 .108 4.489 .108 6 0" /> <path d="M17 4a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -6" /> <path d="M11 13l-3.5 -1.5" /> <path d="M11 13l2.5 3" /> <path d="M8.5 16l2.5 -3" /> <path d="M11 13l3.5 -1.5" /> <path d="M11 9v4" /> </svg>"##;
const CARAVAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M11 18h7a2 2 0 0 0 2 -2v-7a2 2 0 0 0 -2 -2h-9.5a5.5 5.5 0 0 0 -5.5 5.5v3.5a2 2 0 0 0 2 2h2" /> <path d="M8 7l7 -3l1 3" /> <path d="M13 11.5a.5 .5 0 0 1 .5 -.5h2a.5 .5 0 0 1 .5 .5v2a.5 .5 0 0 1 -.5 .5h-2a.5 .5 0 0 1 -.5 -.5l0 -2" /> <path d="M20 16h2" /> </svg>"##;
const CAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 3v10a8 8 0 1 1 -16 0v-10l3.432 3.432a7.963 7.963 0 0 1 4.568 -1.432c1.769 0 3.403 .574 4.728 1.546l3.272 -3.546" /> <path d="M2 16h5l-4 4" /> <path d="M22 16h-5l4 4" /> <path d="M11 16a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M9 11v.01" /> <path d="M15 11v.01" /> </svg>"##;
const CHARGING_PILE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 7l-1 1" /> <path d="M14 11h1a2 2 0 0 1 2 2v3a1.5 1.5 0 0 0 3 0v-7l-3 -3" /> <path d="M4 20v-14a2 2 0 0 1 2 -2h6a2 2 0 0 1 2 2v14" /> <path d="M9 11.5l-1.5 2.5h3l-1.5 2.5" /> <path d="M3 20l12 0" /> <path d="M4 8l10 0" /> </svg>"##;
const CHERRY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 16.5a3.5 3.5 0 1 0 7 0a3.5 3.5 0 1 0 -7 0" /> <path d="M14 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M9 13c.366 -2 1.866 -3.873 4.5 -5.6" /> <path d="M17 15c-1.333 -2.333 -2.333 -5.333 -1 -9" /> <path d="M5 6c3.667 -2.667 7.333 -2.667 11 0c-3.667 2.667 -7.333 2.667 -11 0" /> </svg>"##;
const CHRISTMAS_TREE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3l4 4l-2 1l4 4l-3 1l4 4h-14l4 -4l-3 -1l4 -4l-2 -1l4 -4" /> <path d="M14 17v3a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-3" /> </svg>"##;
const CHRISTMAS_TREE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 5.5l2.5 -2.5l4 4l-2 1l4 4l-1.5 .5m.5 4.5h-12l4 -4l-3 -1l3 -3" /> <path d="M14 17v3a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-3" /> <path d="M3 3l18 18" /> </svg>"##;
const CLOUD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.657 18c-2.572 0 -4.657 -2.007 -4.657 -4.483c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.913 0 3.464 1.56 3.464 3.486c0 1.927 -1.551 3.487 -3.465 3.487h-11.878" /> </svg>"##;
const CLOUD_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 18.004h-6.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.396 0 2.6 .831 3.148 2.03" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const CLOUD_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.004h-5.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99a3.45 3.45 0 0 1 2.756 1.373" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const CLOUD_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 18.004h-4.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.388 0 2.585 .82 3.138 2.007" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const CLOUD_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 18.004h-4.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99a3.468 3.468 0 0 1 3.307 2.444" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const CLOUD_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.004h-5.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c.956 0 1.822 .39 2.449 1.02" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const CLOUD_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 18.004h-6.843c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.28 1.023 1.957 2.51 1.873 4.027" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const CLOUD_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.004h-5.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.38 0 2.573 .813 3.13 1.99" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const CLOUD_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 18.004h-8.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.374 0 2.562 .805 3.121 1.972" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const CLOUD_FOG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 16a4.6 4.4 0 0 1 0 -9a5 4.5 0 0 1 11 2h1a3.5 3.5 0 0 1 0 7h-12" /> <path d="M5 20l14 0" /> </svg>"##;
const CLOUD_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 18.004h-3.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const CLOUD_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.004h-5.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.913 0 3.464 1.56 3.464 3.486c0 .186 -.015 .37 -.042 .548" /> <path d="M16 19h6" /> </svg>"##;
const CLOUD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.58 5.548c.24 -.11 .492 -.207 .752 -.286c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.913 0 3.464 1.56 3.464 3.486c0 .957 -.383 1.824 -1.003 2.454m-2.997 1.033h-11.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.13 -.582 .37 -1.128 .7 -1.62" /> <path d="M3 3l18 18" /> </svg>"##;
const CLOUD_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 18.004h-6.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.406 0 2.617 .843 3.16 2.055" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const CLOUD_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.004h-5.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const CLOUD_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.004h-5.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99a3.46 3.46 0 0 1 3.085 1.9" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const CLOUD_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.5 18.004h-7.843c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const CLOUD_RAIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18a4.6 4.4 0 0 1 0 -9a5 4.5 0 0 1 11 2h1a3.5 3.5 0 0 1 0 7" /> <path d="M11 13v2m0 3v2m4 -5v2m0 3v2" /> </svg>"##;
const CLOUD_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 18.004h-4.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const CLOUD_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 18.004h-5.843c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.41 0 2.624 .848 3.164 2.065" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const CLOUD_SNOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18a4.6 4.4 0 0 1 0 -9a5 4.5 0 0 1 11 2h1a3.5 3.5 0 0 1 0 7" /> <path d="M11 15v.01m0 3v.01m0 3v.01m4 -4v.01m0 3v.01" /> </svg>"##;
const CLOUD_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 18.004h-2.843c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.209 .967 1.88 2.347 1.88 3.776" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const CLOUD_STORM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18a4.6 4.4 0 0 1 0 -9a5 4.5 0 0 1 11 2h1a3.5 3.5 0 0 1 0 7h-1" /> <path d="M13 14l-2 4l3 0l-2 4" /> </svg>"##;
const CLOUD_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.004h-5.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.38 0 2.57 .811 3.128 1.986" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const CLOUD_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 18.004h-6.343c-2.572 -.004 -4.657 -2.011 -4.657 -4.487c0 -2.475 2.085 -4.482 4.657 -4.482c.393 -1.762 1.794 -3.2 3.675 -3.773c1.88 -.572 3.956 -.193 5.444 1c1.488 1.19 2.162 3.007 1.77 4.769h.99c1.37 0 2.556 .8 3.117 1.964" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const CLOVER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 10l-3.397 -3.44a2.104 2.104 0 0 1 0 -2.95a2.04 2.04 0 0 1 2.912 0l.485 .39l.485 -.39a2.04 2.04 0 0 1 2.912 0a2.104 2.104 0 0 1 0 2.95l-3.397 3.44" /> <path d="M12 14l-3.397 3.44a2.104 2.104 0 0 0 0 2.95a2.04 2.04 0 0 0 2.912 0l.485 -.39l.485 .39a2.04 2.04 0 0 0 2.912 0a2.104 2.104 0 0 0 0 -2.95l-3.397 -3.44" /> <path d="M14 12l3.44 -3.397a2.104 2.104 0 0 1 2.95 0a2.04 2.04 0 0 1 0 2.912l-.39 .485l.39 .485a2.04 2.04 0 0 1 0 2.912a2.104 2.104 0 0 1 -2.95 0l-3.44 -3.397" /> <path d="M10 12l-3.44 -3.397a2.104 2.104 0 0 0 -2.95 0a2.04 2.04 0 0 0 0 2.912l.39 .485l-.39 .485a2.04 2.04 0 0 0 0 2.912a2.104 2.104 0 0 0 2.95 0l3.44 -3.397" /> </svg>"##;
const CLOVER_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 11l-3.397 -3.44a2.104 2.104 0 0 1 0 -2.95a2.04 2.04 0 0 1 2.912 0l.485 .39l.485 -.39a2.04 2.04 0 0 1 2.912 0a2.104 2.104 0 0 1 0 2.95l-3.397 3.44" /> <path d="M11 11l-3.397 3.44a2.104 2.104 0 0 0 0 2.95a2.04 2.04 0 0 0 2.912 0l.485 -.39l.485 .39a2.04 2.04 0 0 0 2.912 0a2.104 2.104 0 0 0 0 -2.95l-3.397 -3.44" /> <path d="M14.44 7.603a2.104 2.104 0 0 1 2.95 0a2.04 2.04 0 0 1 0 2.912l-.39 .485l.39 .485a2.04 2.04 0 0 1 0 2.912a2.104 2.104 0 0 1 -2.95 0" /> <path d="M7.56 7.603a2.104 2.104 0 0 0 -2.95 0a2.04 2.04 0 0 0 0 2.912l.39 .485l-.39 .485a2.04 2.04 0 0 0 0 2.912a2.104 2.104 0 0 0 2.95 0" /> <path d="M15 15l6 6" /> </svg>"##;
const COMET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.5 18.5l-3 1.5l.5 -3.5l-2 -2l3 -.5l1.5 -3l1.5 3l3 .5l-2 2l.5 3.5l-3 -1.5" /> <path d="M4 4l7 7" /> <path d="M9 4l3.5 3.5" /> <path d="M4 9l3.5 3.5" /> </svg>"##;
const COMPASS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 16l2 -6l6 -2l-2 6l-6 2" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 3l0 2" /> <path d="M12 19l0 2" /> <path d="M3 12l2 0" /> <path d="M19 12l2 0" /> </svg>"##;
const COMPASS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 9l3 -1l-1 3m-1 3l-6 2l2 -6" /> <path d="M20.042 16.045a9 9 0 0 0 -12.087 -12.087m-2.318 1.677a9 9 0 1 0 12.725 12.73" /> <path d="M12 3v2" /> <path d="M12 19v2" /> <path d="M3 12h2" /> <path d="M19 12h2" /> <path d="M3 3l18 18" /> </svg>"##;
const CRANE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 21h6" /> <path d="M9 21v-18l-6 6h18" /> <path d="M9 3l10 6" /> <path d="M17 9v4a2 2 0 1 1 -2 2" /> </svg>"##;
const CRANE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 21h6" /> <path d="M9 21v-12" /> <path d="M9 5v-2l-1 1" /> <path d="M6 6l-3 3h6" /> <path d="M13 9h8" /> <path d="M9 3l10 6" /> <path d="M17 9v4a2 2 0 0 1 2 2m-2 2a2 2 0 0 1 -2 -2" /> <path d="M3 3l18 18" /> </svg>"##;
const CRYSTAL_BALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.73 17.018a8 8 0 1 1 10.54 0" /> <path d="M5 19a2 2 0 0 0 2 2h10a2 2 0 1 0 0 -4h-10a2 2 0 0 0 -2 2" /> <path d="M11 7a3 3 0 0 0 -3 3" /> </svg>"##;
const CURRENT_LOCATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M4 12a8 8 0 1 0 16 0a8 8 0 1 0 -16 0" /> <path d="M12 2l0 2" /> <path d="M12 20l0 2" /> <path d="M20 12l2 0" /> <path d="M2 12l2 0" /> </svg>"##;
const CURRENT_LOCATION_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.685 10.661c-.3 -.6 -.795 -1.086 -1.402 -1.374m-3.397 .584a3 3 0 1 0 4.24 4.245" /> <path d="M6.357 6.33a8 8 0 1 0 11.301 11.326m1.642 -2.378a8 8 0 0 0 -10.597 -10.569" /> <path d="M12 2v2" /> <path d="M12 20v2" /> <path d="M20 12h2" /> <path d="M2 12h2" /> <path d="M3 3l18 18" /> </svg>"##;
const DEER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3c0 2 1 3 4 3c2 0 3 1 3 3" /> <path d="M21 3c0 2 -1 3 -4 3c-2 0 -3 .333 -3 3" /> <path d="M12 18c-1 0 -4 -3 -4 -6c0 -2 1.333 -3 4 -3s4 1 4 3c0 3 -3 6 -4 6" /> <path d="M15.185 14.889l.095 -.18a4 4 0 1 1 -6.56 0" /> <path d="M17 3c0 1.333 -.333 2.333 -1 3" /> <path d="M7 3c0 1.333 .333 2.333 1 3" /> <path d="M7 6c-2.667 .667 -4.333 1.667 -5 3" /> <path d="M17 6c2.667 .667 4.333 1.667 5 3" /> <path d="M8.5 10l-1.5 -1" /> <path d="M15.5 10l1.5 -1" /> <path d="M12 15h.01" /> </svg>"##;
const DIRECTIONS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21v-4" /> <path d="M12 13v-4" /> <path d="M12 5v-2" /> <path d="M10 21h4" /> <path d="M8 5v4h11l2 -2l-2 -2l-11 0" /> <path d="M14 13v4h-8l-2 -2l2 -2l8 0" /> </svg>"##;
const DIRECTIONS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21v-4" /> <path d="M12 13v-1" /> <path d="M12 5v-2" /> <path d="M10 21h4" /> <path d="M8 8v1h1m4 0h6l2 -2l-2 -2h-10" /> <path d="M14 14v3h-8l-2 -2l2 -2h7" /> <path d="M3 3l18 18" /> </svg>"##;
const DOG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 5h2" /> <path d="M19 12c-.667 5.333 -2.333 8 -5 8h-4c-2.667 0 -4.333 -2.667 -5 -8" /> <path d="M11 16c0 .667 .333 1 1 1s1 -.333 1 -1h-2" /> <path d="M12 18v2" /> <path d="M10 11v.01" /> <path d="M14 11v.01" /> <path d="M5 4l6 .97l-6.238 6.688a1.021 1.021 0 0 1 -1.41 .111a.953 .953 0 0 1 -.327 -.954l1.975 -6.815" /> <path d="M19 4l-6 .97l6.238 6.688c.358 .408 .989 .458 1.41 .111a.953 .953 0 0 0 .327 -.954l-1.975 -6.815" /> </svg>"##;
const DRONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 10h4v4h-4l0 -4" /> <path d="M10 10l-3.5 -3.5" /> <path d="M9.96 6a3.5 3.5 0 1 0 -3.96 3.96" /> <path d="M14 10l3.5 -3.5" /> <path d="M18 9.96a3.5 3.5 0 1 0 -3.96 -3.96" /> <path d="M14 14l3.5 3.5" /> <path d="M14.04 18a3.5 3.5 0 1 0 3.96 -3.96" /> <path d="M10 14l-3.5 3.5" /> <path d="M6 14.04a3.5 3.5 0 1 0 3.96 3.96" /> </svg>"##;
const DRONE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 14h-4v-4" /> <path d="M10 10l-3.5 -3.5" /> <path d="M9.957 5.95a3.503 3.503 0 0 0 -2.917 -2.91m-3.02 .989a3.5 3.5 0 0 0 1.98 5.936" /> <path d="M14 10l3.5 -3.5" /> <path d="M18 9.965a3.5 3.5 0 1 0 -3.966 -3.965" /> <path d="M14 14l3.5 3.5" /> <path d="M14.035 18a3.5 3.5 0 0 0 5.936 1.98m.987 -3.026a3.503 3.503 0 0 0 -2.918 -2.913" /> <path d="M10 14l-3.5 3.5" /> <path d="M6 14.035a3.5 3.5 0 1 0 3.966 3.965" /> <path d="M3 3l18 18" /> </svg>"##;
const DROP_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.07 15.34c1.115 .88 2.74 .88 3.855 0c1.115 -.88 1.398 -2.388 .671 -3.575l-2.596 -3.765l-2.602 3.765c-.726 1.187 -.443 2.694 .672 3.575" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const DROPLETS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.072 20.3a2.999 2.999 0 0 0 3.856 0a3.002 3.002 0 0 0 .67 -3.798l-2.095 -3.227a.6 .6 0 0 0 -1.005 0l-2.098 3.227a3.003 3.003 0 0 0 .671 3.798" /> <path d="M16.072 20.3a2.999 2.999 0 0 0 3.856 0a3.002 3.002 0 0 0 .67 -3.798l-2.095 -3.227a.6 .6 0 0 0 -1.005 0l-2.098 3.227a3.003 3.003 0 0 0 .671 3.798" /> <path d="M10.072 10.3a2.999 2.999 0 0 0 3.856 0a3.002 3.002 0 0 0 .67 -3.798l-2.095 -3.227a.6 .6 0 0 0 -1.005 0l-2.098 3.227a3.003 3.003 0 0 0 .671 3.798l.001 0" /> </svg>"##;
const ENGINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10v6" /> <path d="M12 5v3" /> <path d="M10 5h4" /> <path d="M5 13h-2" /> <path d="M6 10h2l2 -2h3.382a1 1 0 0 1 .894 .553l1.448 2.894a1 1 0 0 0 .894 .553h1.382v-2h2a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-2v-2h-3v2a1 1 0 0 1 -1 1h-3.465a1 1 0 0 1 -.832 -.445l-1.703 -2.555h-2v-6" /> </svg>"##;
const ENGINE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10v6" /> <path d="M12 5v3" /> <path d="M10 5h4" /> <path d="M5 13h-2" /> <path d="M16 16h-1v2a1 1 0 0 1 -1 1h-3.465a1 1 0 0 1 -.832 -.445l-1.703 -2.555h-2v-6h2l.99 -.99m3.01 -1.01h1.382a1 1 0 0 1 .894 .553l1.448 2.894a1 1 0 0 0 .894 .553h1.382v-2h2a1 1 0 0 1 1 1v6" /> <path d="M3 3l18 18" /> </svg>"##;
const ESCALATOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.5 5h-2.672a2 2 0 0 0 -1.414 .586l-8.414 8.414h-2.5a2.5 2.5 0 1 0 0 5h3.672a2 2 0 0 0 1.414 -.586l8.414 -8.414h1.5a2.5 2.5 0 0 0 0 -5" /> </svg>"##;
const ESCALATOR_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.5 7h2.733a2 2 0 0 1 1.337 .513l9.43 8.487h1.5a2.5 2.5 0 1 1 0 5h-2.733a2 2 0 0 1 -1.337 -.513l-9.43 -8.487h-1.5a2.5 2.5 0 1 1 0 -5" /> <path d="M18 3v7" /> <path d="M15 7l3 3l3 -3" /> </svg>"##;
const ESCALATOR_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.5 7h-2.672a2 2 0 0 0 -1.414 .586l-8.414 8.414h-2.5a2.5 2.5 0 1 0 0 5h3.672a2 2 0 0 0 1.414 -.586l8.414 -8.414h1.5a2.5 2.5 0 1 0 0 -5" /> <path d="M6 10v-7" /> <path d="M3 6l3 -3l3 3" /> </svg>"##;
const FEATHER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20l10 -10m0 -5v5h5m-9 -1v5h5m-9 -1v5h5m-5 -5l4 -4l4 -4" /> <path d="M19 10c.638 -.636 1 -1.515 1 -2.486a3.515 3.515 0 0 0 -3.517 -3.514c-.97 0 -1.847 .367 -2.483 1m-3 13l4 -4l4 -4" /> </svg>"##;
const FEATHER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20l8 -8" /> <path d="M14 5v5h5" /> <path d="M9 11v4h4" /> <path d="M6 13v5h5" /> <path d="M6 13l3.502 -3.502m2.023 -2.023l2.475 -2.475" /> <path d="M19 10c.638 -.636 1 -1.515 1 -2.486a3.515 3.515 0 0 0 -3.517 -3.514c-.97 0 -1.847 .367 -2.483 1" /> <path d="M11 18l3.499 -3.499m2.008 -2.008l2.493 -2.493" /> <path d="M3 3l18 18" /> </svg>"##;
const FERRY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 18h15.293c1.02 0 1.972 -.503 2.536 -1.34l2.171 -3.66h-18.479l-1.521 5" /> <path d="M14 8l-1 -2" /> <path d="M6.107 12.675l1.384 -4.675h8l2.675 4.598" /> </svg>"##;
const FIRE_HYDRANT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21h14" /> <path d="M17 21v-5h1a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-1v-4a5 5 0 0 0 -10 0v4h-1a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h1v5" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6 8h12" /> </svg>"##;
const FIRE_HYDRANT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21h14" /> <path d="M17 21v-4m2 -2v-2a1 1 0 0 0 -1 -1h-1v-4a5 5 0 0 0 -8.533 -3.538m-1.387 2.638a5.03 5.03 0 0 0 -.08 .9v4h-1a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h1v5" /> <path d="M12 12a2 2 0 1 0 2 2" /> <path d="M6 8h2m4 0h6" /> <path d="M3 3l18 18" /> </svg>"##;
const FIRETRUCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 18h8m4 0h2v-6a5 5 0 0 0 -5 -5h-1l1.5 5h4.5" /> <path d="M12 18v-11h3" /> <path d="M3 17l0 -5l9 0" /> <path d="M3 9l18 -6" /> <path d="M6 12l0 -4" /> </svg>"##;
const FISH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.69 7.44a6.973 6.973 0 0 0 -1.69 4.56c0 1.747 .64 3.345 1.699 4.571" /> <path d="M2 9.504c7.715 8.647 14.75 10.265 20 2.498c-5.25 -7.761 -12.285 -6.142 -20 2.504" /> <path d="M18 11v.01" /> <path d="M11.5 10.5c-.667 1 -.667 2 0 3" /> </svg>"##;
const FISH_BONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.69 7.44a6.973 6.973 0 0 0 -1.69 4.56a6.97 6.97 0 0 0 1.699 4.571c1.914 -.684 3.691 -2.183 5.301 -4.565c-1.613 -2.384 -3.394 -3.883 -5.312 -4.565" /> <path d="M2 9.504a40.73 40.73 0 0 0 2.422 2.504a39.679 39.679 0 0 0 -2.422 2.498" /> <path d="M18 11v.01" /> <path d="M4.422 12h10.578" /> <path d="M7 10v4" /> <path d="M11 8v8" /> </svg>"##;
const FISH_HOOK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 9v6a5 5 0 0 1 -10 0v-4l3 3" /> <path d="M14 7a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 5v-2" /> </svg>"##;
const FISH_HOOK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 9v3m-.085 3.924a5 5 0 0 1 -9.915 -.924v-4l3 3" /> <path d="M14 7a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 5v-2" /> <path d="M3 3l18 18" /> </svg>"##;
const FISH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.69 7.44a6.973 6.973 0 0 0 -1.63 3.635" /> <path d="M2 9.504c5.307 5.948 10.293 8.57 14.597 7.1m2.583 -1.449c.988 -.788 1.93 -1.836 2.82 -3.153c-3 -4.443 -6.596 -5.812 -10.564 -4.548m-2.764 1.266c-2.145 1.266 -4.378 3.215 -6.672 5.786" /> <path d="M18 11v.01" /> <path d="M11.153 11.169c-.287 .777 -.171 1.554 .347 2.331" /> <path d="M3 3l18 18" /> </svg>"##;
const FLAG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5a5 5 0 0 1 7 0a5 5 0 0 0 7 0v9a5 5 0 0 1 -7 0a5 5 0 0 0 -7 0v-9" /> <path d="M5 21v-7" /> </svg>"##;
const FLAG_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 14h14v-9h-14v16" /> </svg>"##;
const FLAG_2_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 14h9m4 0h1v-9h-10m-4 0v16" /> <path d="M3 3l18 18" /> </svg>"##;
const FLAG_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 14h14l-4.5 -4.5l4.5 -4.5h-14v16" /> </svg>"##;
const FLAG_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.673 15.36a4.978 4.978 0 0 1 -2.673 -1.36a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v7" /> <path d="M5 21v-7" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const FLAG_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.342 14.941a4.993 4.993 0 0 1 -1.342 -.941a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v7" /> <path d="M5 21v-7" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const FLAG_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.767 15.12a4.983 4.983 0 0 1 -1.767 -1.12a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v8.5" /> <path d="M5 21v-7" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const FLAG_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.41 14.973a4.991 4.991 0 0 1 -1.41 -.973a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v8" /> <path d="M5 21v-7" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const FLAG_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.901 14.702a5.014 5.014 0 0 1 -.901 -.702a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v6.5" /> <path d="M5 21v-7" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const FLAG_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.222 14.882a4.998 4.998 0 0 1 -1.222 -.882a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v5" /> <path d="M5 21v-7" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const FLAG_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.434 15.315a4.978 4.978 0 0 1 -2.434 -1.315a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v7" /> <path d="M5 21v-7" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const FLAG_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.035 15.408a4.98 4.98 0 0 1 -3.035 -1.408a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v7" /> <path d="M5 21v-7" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const FLAG_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.33 13.447a5.001 5.001 0 0 0 -6.33 .553v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v6" /> <path d="M5 21v-7" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const FLAG_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.373 15.301a4.978 4.978 0 0 1 -2.373 -1.301a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v9" /> <path d="M5 21v-7" /> <path d="M16 19h6" /> </svg>"##;
const FLAG_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5v16" /> <path d="M19 5v9" /> <path d="M7.641 3.645a5 5 0 0 1 4.359 1.355a5 5 0 0 0 7 0" /> <path d="M5 14a5 5 0 0 1 7 0a4.984 4.984 0 0 0 3.437 1.429m3.019 -.966c.19 -.14 .371 -.294 .544 -.463" /> <path d="M3 3l18 18" /> </svg>"##;
const FLAG_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.536 15.029a4.987 4.987 0 0 1 -1.536 -1.029a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v8.5" /> <path d="M5 21v-7" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const FLAG_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.857 14.675a5.016 5.016 0 0 1 -.857 -.675a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v6" /> <path d="M5 21v-7" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const FLAG_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.433 15.315a4.978 4.978 0 0 1 -2.433 -1.315a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v7" /> <path d="M5 21v-7" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const FLAG_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 15a4.914 4.914 0 0 1 -1.5 -1a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v6" /> <path d="M5 21v-7" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const FLAG_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 14a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v6" /> <path d="M5 21v-7" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const FLAG_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.13 14.833a5.002 5.002 0 0 1 -1.13 -.833a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v8" /> <path d="M5 21v-7" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const FLAG_SPARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.165 15.249a5 5 0 0 1 -2.165 -1.249a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v6.5" /> <path d="M5 21v-7" /> <path d="M19 22.5a4.75 4.75 0 0 1 3.5 -3.5a4.75 4.75 0 0 1 -3.5 -3.5a4.75 4.75 0 0 1 -3.5 3.5a4.75 4.75 0 0 1 3.5 3.5" /> </svg>"##;
const FLAG_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.475 13.551a5.001 5.001 0 0 0 -6.475 .449v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v5" /> <path d="M5 21v-7" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const FLAG_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.138 15.241a4.979 4.979 0 0 1 -2.138 -1.241a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v7" /> <path d="M5 21v-7" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const FLAG_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.533 15.028a4.988 4.988 0 0 1 -1.533 -1.028a5 5 0 0 0 -7 0v-9a5 5 0 0 1 7 0a5 5 0 0 0 7 0v8.5" /> <path d="M5 21v-7" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const FLAME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 10.941c2.333 -3.308 .167 -7.823 -1 -8.941c0 3.395 -2.235 5.299 -3.667 6.706c-1.43 1.408 -2.333 3.294 -2.333 5.588c0 3.704 3.134 6.706 7 6.706c3.866 0 7 -3.002 7 -6.706c0 -1.712 -1.232 -4.403 -2.333 -5.588c-2.084 3.353 -3.257 3.353 -4.667 2.235" /> </svg>"##;
const FLAME_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.973 8.974c-.335 .378 -.67 .716 -.973 1.026c-1.226 1.26 -2 3.24 -2 5a6 6 0 0 0 11.472 2.466m.383 -3.597c-.32 -1.409 -1.122 -3.045 -1.855 -3.869c-.281 .472 -.543 .87 -.79 1.202m-2.358 -2.35c-.068 -2.157 -1.182 -4.184 -1.852 -4.852c0 .968 -.18 1.801 -.465 2.527" /> <path d="M3 3l18 18" /> </svg>"##;
const FLARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3l3 6l6 3l-6 3l-3 6l-3 -6l-6 -3l6 -3l3 -6" /> </svg>"##;
const FLOOD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10l9 -7l9 7" /> <path d="M6 7.75v4.25m12 0v-4.25" /> <path d="M3 20.75a2.4 2.4 0 0 0 1 .25a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 1 -.25" /> <path d="M3 16.75a2.4 2.4 0 0 0 1 .25a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 1 -.25" /> </svg>"##;
const FLOWER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M12 2a3 3 0 0 1 3 3c0 .562 -.259 1.442 -.776 2.64l-.724 1.36l1.76 -1.893c.499 -.6 .922 -1 1.27 -1.205a2.968 2.968 0 0 1 4.07 1.099a3.011 3.011 0 0 1 -1.09 4.098c-.374 .217 -.99 .396 -1.846 .535l-2.664 .366l2.4 .326c1 .145 1.698 .337 2.11 .576a3.011 3.011 0 0 1 1.09 4.098a2.968 2.968 0 0 1 -4.07 1.098c-.348 -.202 -.771 -.604 -1.27 -1.205l-1.76 -1.893l.724 1.36c.516 1.199 .776 2.079 .776 2.64a3 3 0 0 1 -6 0c0 -.562 .259 -1.442 .776 -2.64l.724 -1.36l-1.76 1.893c-.499 .601 -.922 1 -1.27 1.205a2.968 2.968 0 0 1 -4.07 -1.098a3.011 3.011 0 0 1 1.09 -4.098c.374 -.218 .99 -.396 1.846 -.536l2.664 -.366l-2.4 -.325c-1 -.145 -1.698 -.337 -2.11 -.576a3.011 3.011 0 0 1 -1.09 -4.099a2.968 2.968 0 0 1 4.07 -1.099c.348 .203 .771 .604 1.27 1.205l1.76 1.894c-1 -2.292 -1.5 -3.625 -1.5 -4a3 3 0 0 1 3 -3" /> </svg>"##;
const FLOWER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.875 9.882a3 3 0 0 0 4.247 4.238m.581 -3.423a3.012 3.012 0 0 0 -1.418 -1.409" /> <path d="M9 5a3 3 0 0 1 6 0c0 .562 -.259 1.442 -.776 2.64l-.724 1.36l1.76 -1.893c.499 -.6 .922 -1 1.27 -1.205a2.968 2.968 0 0 1 4.07 1.099a3.011 3.011 0 0 1 -1.09 4.098c-.374 .217 -.99 .396 -1.846 .535l-1.779 .244m.292 .282l1.223 .166c1 .145 1.698 .337 2.11 .576a3.011 3.011 0 0 1 1.226 3.832m-2.277 1.733a2.968 2.968 0 0 1 -1.929 -.369c-.348 -.202 -.771 -.604 -1.27 -1.205l-1.76 -1.893l.724 1.36c.516 1.199 .776 2.079 .776 2.64a3 3 0 0 1 -6 0c0 -.562 .259 -1.442 .776 -2.64l.724 -1.36l-1.76 1.893c-.499 .601 -.922 1 -1.27 1.205a2.968 2.968 0 0 1 -4.07 -1.098a3.011 3.011 0 0 1 1.09 -4.098c.374 -.218 .99 -.396 1.846 -.536l2.664 -.366l-2.4 -.325c-1 -.145 -1.698 -.337 -2.11 -.576a3.011 3.011 0 0 1 -1.09 -4.099a2.968 2.968 0 0 1 2.134 -1.467" /> <path d="M3 3l18 18" /> </svg>"##;
const FORKLIFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M12 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 17l5 0" /> <path d="M3 17v-6h13v6" /> <path d="M5 11v-4h4" /> <path d="M9 11v-6h4l3 6" /> <path d="M22 15h-3v-10" /> <path d="M16 13l3 0" /> </svg>"##;
const FOUNTAIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 16v-5a2 2 0 1 0 -4 0" /> <path d="M15 16v-5a2 2 0 1 1 4 0" /> <path d="M12 16v-10a3 3 0 0 1 6 0" /> <path d="M6 6a3 3 0 0 1 6 0" /> <path d="M3 16h18v2a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-2" /> </svg>"##;
const FOUNTAIN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 16v-5a2 2 0 1 0 -4 0" /> <path d="M15 16v-1m0 -4a2 2 0 1 1 4 0" /> <path d="M12 16v-4m0 -4v-2a3 3 0 0 1 6 0" /> <path d="M7.451 3.43a3 3 0 0 1 4.549 2.57" /> <path d="M20 16h1v1m-.871 3.114a2.99 2.99 0 0 1 -2.129 .886h-12a3 3 0 0 1 -3 -3v-2h13" /> <path d="M3 3l18 18" /> </svg>"##;
const GARDEN_CART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 17.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> <path d="M6 8v11a1 1 0 0 0 1.806 .591l3.694 -5.091v.055" /> <path d="M6 8h15l-3.5 7l-7.1 -.747a4 4 0 0 1 -3.296 -2.493l-2.853 -7.13a1 1 0 0 0 -.928 -.63h-1.323" /> </svg>"##;
const GARDEN_CART_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.733 15.732a2.5 2.5 0 1 0 3.544 3.527" /> <path d="M6 8v11a1 1 0 0 0 1.806 .591l3.694 -5.091v.055" /> <path d="M6 8h2m4 0h9l-3 6.01m-3.319 .693l-4.276 -.45a4 4 0 0 1 -3.296 -2.493l-2.853 -7.13a1 1 0 0 0 -.928 -.63h-1.323" /> <path d="M3 3l18 18" /> </svg>"##;
const GAS_STATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 11h1a2 2 0 0 1 2 2v3a1.5 1.5 0 0 0 3 0v-7l-3 -3" /> <path d="M4 20v-14a2 2 0 0 1 2 -2h6a2 2 0 0 1 2 2v14" /> <path d="M3 20l12 0" /> <path d="M18 7v1a1 1 0 0 0 1 1h1" /> <path d="M4 11l10 0" /> </svg>"##;
const GAS_STATION_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11a2 2 0 0 1 2 2m3 3v-7l-3 -3" /> <path d="M4 20v-14c0 -.548 .22 -1.044 .577 -1.405m3.423 -.595h4a2 2 0 0 1 2 2v4m0 4v6" /> <path d="M3 20h12" /> <path d="M18 7v1a1 1 0 0 0 1 1h1" /> <path d="M4 11h7" /> <path d="M3 3l18 18" /> </svg>"##;
const GEOMETRY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 21l4 -12m2 0l1.48 4.439m.949 2.847l1.571 4.714" /> <path d="M10 7a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 12c1.526 2.955 4.588 5 8 5c3.41 0 6.473 -2.048 8 -5" /> <path d="M12 5v-2" /> </svg>"##;
const GLOBE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 9a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" /> <path d="M5.75 15a8.015 8.015 0 1 0 9.25 -13" /> <path d="M11 17v4" /> <path d="M7 21h8" /> </svg>"##;
const GLOBE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.353 7.355a4 4 0 0 0 5.29 5.293m2.007 -2.009a4 4 0 0 0 -5.3 -5.284" /> <path d="M5.75 15a8.015 8.015 0 0 0 9.792 .557m2.02 -1.998a8.015 8.015 0 0 0 -2.562 -11.559" /> <path d="M11 17v4" /> <path d="M7 21h8" /> <path d="M3 3l18 18" /> </svg>"##;
const GPS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 17l-1 -4l-4 -1l9 -4l-4 9" /> </svg>"##;
const GRAVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21v-2a3 3 0 0 1 3 -3h8a3 3 0 0 1 3 3v2h-14" /> <path d="M10 16v-5h-4v-4h4v-4h4v4h4v4h-4v5" /> </svg>"##;
const GRAVE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 16.17v-9.17a3 3 0 0 1 3 -3h4a3 3 0 0 1 3 3v9.171" /> <path d="M12 7v5" /> <path d="M10 9h4" /> <path d="M5 21v-2a3 3 0 0 1 3 -3h8a3 3 0 0 1 3 3v2h-14" /> </svg>"##;
const GROWTH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.5 15a4.5 4.5 0 0 0 -4.5 4.5m4.5 -8.5a4.5 4.5 0 0 0 -4.5 4.5m4.5 -8.5a4.5 4.5 0 0 0 -4.5 4.5m-4 3.5c2.21 0 4 2.015 4 4.5m-4 -8.5c2.21 0 4 2.015 4 4.5m-4 -8.5c2.21 0 4 2.015 4 4.5m0 -7.5v6" /> </svg>"##;
const HAZE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h1" /> <path d="M12 3v1" /> <path d="M20 12h1" /> <path d="M5.6 5.6l.7 .7" /> <path d="M18.4 5.6l-.7 .7" /> <path d="M8 12a4 4 0 1 1 8 0" /> <path d="M3 16h18" /> <path d="M3 20h18" /> </svg>"##;
const HELICOPTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10l1 2h6" /> <path d="M12 9a2 2 0 0 0 -2 2v3c0 1.1 .9 2 2 2h7a2 2 0 0 0 2 -2c0 -3.31 -3.13 -5 -7 -5h-2" /> <path d="M13 9l0 -3" /> <path d="M5 6l15 0" /> <path d="M15 9.1v3.9h5.5" /> <path d="M15 19l0 -3" /> <path d="M19 19l-8 0" /> </svg>"##;
const HELICOPTER_LANDING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M9 8l0 8" /> <path d="M9 12l6 0" /> <path d="M15 8l0 8" /> </svg>"##;
const HORSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 10l-.85 8.507a1.357 1.357 0 0 0 1.35 1.493h.146a2 2 0 0 0 1.857 -1.257l.994 -2.486a2 2 0 0 1 1.857 -1.257h1.292a2 2 0 0 1 1.857 1.257l.994 2.486a2 2 0 0 0 1.857 1.257h.146a1.37 1.37 0 0 0 1.364 -1.494l-.864 -9.506h-8c0 -3 -3 -5 -6 -5l-3 6l2 2l3 -2" /> <path d="M22 14v-2a3 3 0 0 0 -3 -3" /> </svg>"##;
const HORSESHOE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 17c.5 -1.242 2 -2 2 -5s-1 -9 -9 -9s-9 6 -9 9s1.495 3.749 2 5l-2 1l2 3l2.406 -1.147c1.25 -.714 1.778 -2.08 1.203 -3.363c-1.078 -2.407 -1.609 -8.49 3.391 -8.49s4.469 6.083 3.39 8.49c-.574 1.284 -.045 2.649 1.204 3.363l2.406 1.147l2 -3l-2 -1" /> </svg>"##;
const ICEBERG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 10l-2 9l-4 3l-3 -5l-3 -1l-2 -6l2 -5l3 -2l3 3l4 1l2 3" /> <path d="M3 10h18" /> </svg>"##;
const JETSKI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5h1.4a1 1 0 0 1 .882 .53l1.718 3.22" /> <path d="M3.485 16.94l.136 .545a2 2 0 0 0 1.94 1.515h7.439a10 10 0 0 0 8 -4c0 -6 -5 -8 -5 -8c-1.889 2.518 -5.852 4 -9 4h-2a2 2 0 0 0 -2 2c0 1.328 .163 2.652 .485 3.94" /> <path d="M3.25 15h17.75" /> </svg>"##;
const LEAF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21c.5 -4.5 2.5 -8 7 -10" /> <path d="M9 18c6.218 0 10.5 -3.288 11 -12v-2h-4.014c-9 0 -11.986 4 -12 9c0 1 0 3 2 5h3l.014 0" /> </svg>"##;
const LEAF_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21c.5 -4.5 2.5 -8 7 -10" /> <path d="M7.5 15q -3.5 0 -4.5 -6a8.4 8.4 0 0 1 3.438 .402a12 12 0 0 1 -.052 -.793c0 -3.606 3.204 -5.609 3.204 -5.609s2.003 1.252 2.842 3.557q 2.568 -1.557 6.568 -1.557q .396 3.775 -1.557 6.568c2.305 .839 3.557 2.842 3.557 2.842s-3 2.59 -7 2.59c0 1 0 1 .5 3q -6 0 -7 -5" /> </svg>"##;
const LEAF_MAPLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21c.5 -4.5 2.5 -8 7 -10" /> <path d="M13 19c-2.733 0 -4.16 -3.11 -5 -5c-1.892 -.84 -4 -1.826 -4 -4.556c1.014 -.644 2.816 -.649 4 -.444c-.312 -2.071 -.37 -4.414 1 -6c2.364 .369 3 4 3 4c1.463 -1.368 4 -2 6 -2c0 2 -.63 4.538 -2 6q 3.687 .996 4 3c-1.586 1.36 -3.933 1.311 -6 1q .19 1.098 -1 4" /> </svg>"##;
const LEAF_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21c.475 -4.27 2.3 -7.64 6.331 -9.683" /> <path d="M6.618 6.623c-1.874 1.625 -2.625 3.877 -2.632 6.377c0 1 0 3 2 5h3.014c2.733 0 5.092 -.635 6.92 -2.087m1.899 -2.099c1.224 -1.872 1.987 -4.434 2.181 -7.814v-2h-4.014c-2.863 0 -5.118 .405 -6.861 1.118" /> <path d="M3 3l18 18" /> </svg>"##;
const LIVE_VIEW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8v-2a2 2 0 0 1 2 -2h2" /> <path d="M4 16v2a2 2 0 0 0 2 2h2" /> <path d="M16 4h2a2 2 0 0 1 2 2v2" /> <path d="M16 20h2a2 2 0 0 0 2 -2v-2" /> <path d="M12 11l0 .01" /> <path d="M12 18l-3.5 -5a4 4 0 1 1 7 0l-3.5 5" /> </svg>"##;
const LOCATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 3l-6.5 18a.55 .55 0 0 1 -1 0l-3.5 -7l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5" /> </svg>"##;
const LOCATION_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.05 20.1l-3.05 -6.1l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.312 9.173" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const LOCATION_BROKEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.896 19.792l-2.896 -5.792l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.487 9.657" /> <path d="M21.5 21.5l-5 -5" /> <path d="M16.5 21.5l5 -5" /> </svg>"##;
const LOCATION_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l-2 -4l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.305 9.151" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const LOCATION_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.512 17.023l-1.512 -3.023l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-4.45 12.324" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const LOCATION_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.505 17.01l-1.505 -3.01l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.677 10.184" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const LOCATION_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l-2 -4l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.14 8.697" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const LOCATION_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.08 20.162l-3.08 -6.162l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-2.55 7.063" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const LOCATION_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l-2 -4l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.328 9.217" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const LOCATION_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.024 19.55l-.524 1.45a.55 .55 0 0 1 -1 0l-3.5 -7l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.317 9.186" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const LOCATION_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.365 14.73l-.365 -.73l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.024 8.373" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const LOCATION_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l-2 -4l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-4.347 12.038" /> <path d="M16 19h6" /> </svg>"##;
const LOCATION_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.72 6.712l10.28 -3.712l-3.724 10.313m-1.056 2.925l-1.72 4.762a.55 .55 0 0 1 -1 0l-3.5 -7l-7 -3.5a.55 .55 0 0 1 0 -1l4.775 -1.724" /> <path d="M3 3l18 18" /> </svg>"##;
const LOCATION_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.02 20.04l-3.02 -6.04l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.634 10.062" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const LOCATION_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l-2 -4l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-2.901 8.034" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const LOCATION_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l-2 -4l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.361 9.308" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const LOCATION_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.5 21a.55 .55 0 0 1 -1 0l-3.5 -7l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-2.967 8.215" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const LOCATION_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 16l-1 -2l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-2.916 8.076" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const LOCATION_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l-2 -4l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.616 10.015" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const LOCATION_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.336 14.672l-.336 -.672l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-2.565 7.104" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const LOCATION_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18l-2 -4l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.251 9.003" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const LOCATION_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 21l-.224 -.448l-3.276 -6.552l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5l-3.622 10.03" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const MAP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7l6 -3l6 3l6 -3v13l-6 3l-6 -3l-6 3v-13" /> <path d="M9 4v13" /> <path d="M15 7v13" /> </svg>"##;
const MAP_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.5l-3 -1.5l-6 3v-13l6 -3l6 3l6 -3v7.5" /> <path d="M9 4v13" /> <path d="M15 7v5.5" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const MAP_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19l-4 -2l-6 3v-13l6 -3l6 3l6 -3v8.5" /> <path d="M9 4v13" /> <path d="M15 7v7.5" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const MAP_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.5l-3 -1.5l-6 3v-13l6 -3l6 3l6 -3v8" /> <path d="M9 4v13" /> <path d="M15 7v6" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const MAP_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 18l-2 -1l-6 3v-13l6 -3l6 3l6 -3v9" /> <path d="M9 4v13" /> <path d="M15 7v8" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const MAP_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 18l-2 -1l-6 3v-13l6 -3l6 3l6 -3v9" /> <path d="M9 4v13" /> <path d="M15 7v6.5" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const MAP_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.5l-3 -1.5l-6 3v-13l6 -3l6 3l6 -3v8" /> <path d="M9 4v13" /> <path d="M15 7v6.5" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const MAP_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19l-4 -2l-6 3v-13l6 -3l6 3l6 -3v8.5" /> <path d="M9 4v13" /> <path d="M15 7v5.5" /> <path d="M16 21l5 -5" /> <path d="M21 21v.01" /> <path d="M16 16v.01" /> </svg>"##;
const MAP_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19l-4 -2l-6 3v-13l6 -3l6 3l6 -3v6.5" /> <path d="M9 4v13" /> <path d="M15 7v5" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const MAP_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.5l-3 -1.5l-6 3v-13l6 -3l6 3l6 -3v8.5" /> <path d="M9 4v13" /> <path d="M15 7v8" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const MAP_EAST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14 9h-4v6h4" /> <path d="M10 12h2.5" /> </svg>"##;
const MAP_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 20l-6 -3l-6 3v-13l6 -3l6 3l6 -3v8.5" /> <path d="M9 4v13" /> <path d="M15 7v13" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const MAP_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 17.5l-1 -.5l-6 3v-13l6 -3l6 3l6 -3v7" /> <path d="M9 4v13" /> <path d="M15 7v4" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const MAP_LOCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M13.004 21.216a2 2 0 0 1 -2.417 -.316l-4.244 -4.243a8 8 0 0 1 11.314 -11.314a7.93 7.93 0 0 1 2.343 5.657" /> <path d="M17 19a1 1 0 0 1 1 -1h3a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-3a1 1 0 0 1 -1 -1v-2" /> <path d="M18 18v-1.5a1.5 1.5 0 1 1 3 0v1.5" /> </svg>"##;
const MAP_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.5l-3 -1.5l-6 3v-13l6 -3l6 3l6 -3v11" /> <path d="M9 4v13" /> <path d="M15 7v8" /> <path d="M16 19h6" /> </svg>"##;
const MAP_NORTH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 15v-6l4 6v-6" /> </svg>"##;
const MAP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.32 4.34l.68 -.34l6 3l6 -3v13m-2.67 1.335l-3.33 1.665l-6 -3l-6 3v-13l2.665 -1.333" /> <path d="M9 4v1m0 4v8" /> <path d="M15 7v4m0 4v5" /> <path d="M3 3l18 18" /> </svg>"##;
const MAP_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19l-4 -2l-6 3v-13l6 -3l6 3l6 -3v9" /> <path d="M9 4v13" /> <path d="M15 7v6.5" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const MAP_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M17.657 16.657l-4.243 4.243a2 2 0 0 1 -2.827 0l-4.244 -4.243a8 8 0 1 1 11.314 0" /> </svg>"##;
const MAP_PIN_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.5l-3 -1.5l-6 3v-13l6 -3l6 3l6 -3v7" /> <path d="M9 4v13" /> <path d="M15 7v5" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const MAP_PIN_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M13.414 20.9a2 2 0 0 1 -2.827 0l-4.244 -4.243a8 8 0 1 1 13.591 -4.629" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const MAP_PIN_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M12.463 21.431a1.999 1.999 0 0 1 -1.876 -.531l-4.244 -4.243a8 8 0 1 1 13.594 -4.655" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const MAP_PIN_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M11.87 21.48a1.992 1.992 0 0 1 -1.283 -.58l-4.244 -4.243a8 8 0 1 1 13.355 -3.474" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const MAP_PIN_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M11.85 21.48a1.992 1.992 0 0 1 -1.263 -.58l-4.244 -4.243a8 8 0 1 1 13.385 -3.585" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const MAP_PIN_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M12.005 21.485a1.994 1.994 0 0 1 -1.418 -.585l-4.244 -4.243a8 8 0 1 1 13.634 -5.05" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const MAP_PIN_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M13.02 21.206a2 2 0 0 1 -2.433 -.306l-4.244 -4.243a8 8 0 1 1 13.607 -6.555" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const MAP_PIN_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M12.736 21.345a2 2 0 0 1 -2.149 -.445l-4.244 -4.243a8 8 0 1 1 13.59 -4.624" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const MAP_PIN_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M15.005 19.31l-1.591 1.59a2 2 0 0 1 -2.827 0l-4.244 -4.243a8 8 0 1 1 13.592 -4.638" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const MAP_PIN_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11a3 3 0 1 0 -3.973 2.839" /> <path d="M11.76 21.47a1.991 1.991 0 0 1 -1.173 -.57l-4.244 -4.243a8 8 0 1 1 13.657 -5.588" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const MAP_PIN_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M12.758 21.337a2 2 0 0 1 -2.171 -.437l-4.244 -4.243a8 8 0 1 1 12.585 -1.652" /> <path d="M16 19h6" /> </svg>"##;
const MAP_PIN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.442 9.432a3 3 0 0 0 4.113 4.134m1.445 -2.566a3 3 0 0 0 -3 -3" /> <path d="M17.152 17.162l-3.738 3.738a2 2 0 0 1 -2.827 0l-4.244 -4.243a8 8 0 0 1 -.476 -10.794m2.18 -1.82a8.003 8.003 0 0 1 10.91 10.912" /> <path d="M3 3l18 18" /> </svg>"##;
const MAP_PIN_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M13.414 20.9a2 2 0 0 1 -2.827 0l-4.244 -4.243a8 8 0 1 1 13.337 -3.413" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const MAP_PIN_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M12.783 21.326a2 2 0 0 1 -2.196 -.426l-4.244 -4.243a8 8 0 1 1 13.657 -5.62" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const MAP_PIN_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M12.794 21.322a2 2 0 0 1 -2.207 -.422l-4.244 -4.243a8 8 0 1 1 13.59 -4.616" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const MAP_PIN_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M14.997 19.317l-1.583 1.583a2 2 0 0 1 -2.827 0l-4.244 -4.243a8 8 0 1 1 13.657 -5.584" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const MAP_PIN_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.916 11.707a3 3 0 1 0 -2.916 2.293" /> <path d="M11.991 21.485a1.994 1.994 0 0 1 -1.404 -.585l-4.244 -4.243a8 8 0 1 1 13.651 -5.351" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const MAP_PIN_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M12.02 21.485a1.996 1.996 0 0 1 -1.433 -.585l-4.244 -4.243a8 8 0 1 1 13.403 -3.651" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const MAP_PIN_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11a3 3 0 1 0 -3.908 2.86" /> <path d="M11.059 21.25a2 2 0 0 1 -.472 -.35l-4.244 -4.243a8 8 0 1 1 13.646 -6.079" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const MAP_PIN_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M12.789 21.324a2 2 0 0 1 -2.202 -.424l-4.244 -4.243a8 8 0 1 1 13.59 -4.626" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const MAP_PIN_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 11a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M13.024 21.204a2 2 0 0 1 -2.437 -.304l-4.244 -4.243a8 8 0 1 1 13.119 -2.766" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const MAP_PINS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.828 9.828a4 4 0 1 0 -5.656 0l2.828 2.829l2.828 -2.829" /> <path d="M8 7l0 .01" /> <path d="M18.828 17.828a4 4 0 1 0 -5.656 0l2.828 2.829l2.828 -2.829" /> <path d="M16 15l0 .01" /> </svg>"##;
const MAP_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.5l-3 -1.5l-6 3v-13l6 -3l6 3l6 -3v8.5" /> <path d="M9 4v13" /> <path d="M15 7v8" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const MAP_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 20l-6 -3l-6 3v-13l6 -3l6 3l6 -3v7.5" /> <path d="M9 4v13" /> <path d="M15 7v5.5" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const MAP_ROUTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7l6 -3l6 3l6 -3v13l-6 3l-6 -3l-6 3v-13" /> <path d="M9 12v.01" /> <path d="M6 13v.01" /> <path d="M17 15l-4 -4" /> <path d="M13 15l4 -4" /> </svg>"##;
const MAP_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 18l-2 -1l-6 3v-13l6 -3l6 3l6 -3v7.5" /> <path d="M9 4v13" /> <path d="M15 7v5" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const MAP_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 19l-4 -2l-6 3v-13l6 -3l6 3l6 -3v9" /> <path d="M9 4v13" /> <path d="M15 7v6.5" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const MAP_SHIELD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11a3 3 0 1 0 -3 3" /> <path d="M12.249 21.47a2 2 0 0 1 -1.662 -.57l-4.244 -4.243a8 8 0 1 1 13.655 -5.828" /> <path d="M22 16c0 4 -2.5 6 -3.5 6s-3.5 -2 -3.5 -6c1 0 2.5 -.5 3.5 -1.5c1 1 2.5 1.5 3.5 1.5" /> </svg>"##;
const MAP_SOUTH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 14.25c0 .414 .336 .75 .75 .75h2.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h2.25a.75 .75 0 0 1 .75 .75" /> </svg>"##;
const MAP_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.718 17.359l-.718 -.359l-6 3v-13l6 -3l6 3l6 -3v7.5" /> <path d="M9 4v13" /> <path d="M15 7v4" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const MAP_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.5l-3 -1.5l-6 3v-13l6 -3l6 3l6 -3v8.5" /> <path d="M9 4v13" /> <path d="M15 7v7.5" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const MAP_WEST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 9l1 6l2 -3.75l2 3.75l1 -6" /> </svg>"##;
const MAP_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 19.5l-5 -2.5l-6 3v-13l6 -3l6 3l6 -3v9" /> <path d="M9 4v13" /> <path d="M15 7v6.5" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const MASKS_THEATER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.192 9h6.616a2 2 0 0 1 1.992 2.183l-.567 6.182a4 4 0 0 1 -3.983 3.635h-1.5a4 4 0 0 1 -3.983 -3.635l-.567 -6.182a2 2 0 0 1 1.992 -2.183" /> <path d="M15 13h.01" /> <path d="M18 13h.01" /> <path d="M15 16.5c1 .667 2 .667 3 0" /> <path d="M8.632 15.982a4.037 4.037 0 0 1 -.382 .018h-1.5a4 4 0 0 1 -3.983 -3.635l-.567 -6.182a2 2 0 0 1 1.992 -2.183h6.616a2 2 0 0 1 2 2" /> <path d="M6 8h.01" /> <path d="M9 8h.01" /> <path d="M6 12c.764 -.51 1.528 -.63 2.291 -.36" /> </svg>"##;
const MASKS_THEATER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 9c.058 0 .133 0 .192 0h6.616a2 2 0 0 1 1.992 2.183l-.554 6.041m-1.286 2.718a3.99 3.99 0 0 1 -2.71 1.058h-1.5a4 4 0 0 1 -3.983 -3.635l-.567 -6.182" /> <path d="M18 13h.01" /> <path d="M15 16.5c.657 .438 1.313 .588 1.97 .451" /> <path d="M8.632 15.982a4.05 4.05 0 0 1 -.382 .018h-1.5a4 4 0 0 1 -3.983 -3.635l-.567 -6.182a2 2 0 0 1 .514 -1.531a1.99 1.99 0 0 1 1.286 -.652m4 0h2.808a2 2 0 0 1 2 2" /> <path d="M6 8h.01" /> <path d="M6 12c.764 -.51 1.528 -.63 2.291 -.36" /> <path d="M3 3l18 18" /> </svg>"##;
const MEDICAL_CROSS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 3a1 1 0 0 1 1 1v4.535l3.928 -2.267a1 1 0 0 1 1.366 .366l1 1.732a1 1 0 0 1 -.366 1.366l-3.927 2.268l3.927 2.269a1 1 0 0 1 .366 1.366l-1 1.732a1 1 0 0 1 -1.366 .366l-3.928 -2.269v4.536a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-4.536l-3.928 2.268a1 1 0 0 1 -1.366 -.366l-1 -1.732a1 1 0 0 1 .366 -1.366l3.927 -2.268l-3.927 -2.268a1 1 0 0 1 -.366 -1.366l1 -1.732a1 1 0 0 1 1.366 -.366l3.928 2.267v-4.535a1 1 0 0 1 1 -1h2" /> </svg>"##;
const MEDICAL_CROSS_CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M12 8v8" /> <path d="M15.5 10l-7 4" /> <path d="M15.5 14l-7 -4" /> </svg>"##;
const MEDICAL_CROSS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.928 17.733l-.574 -.331l-3.354 -1.938v4.536a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-4.536l-3.928 2.268a1 1 0 0 1 -1.366 -.366l-1 -1.732a1 1 0 0 1 .366 -1.366l3.927 -2.268l-3.927 -2.268a1 1 0 0 1 -.366 -1.366l1 -1.732a1 1 0 0 1 1.366 -.366l.333 .192m3.595 -.46v-2a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v4.535l3.928 -2.267a1 1 0 0 1 1.366 .366l1 1.732a1 1 0 0 1 -.366 1.366l-3.927 2.268l3.927 2.269a1 1 0 0 1 .366 1.366l-.24 .416" /> <path d="M3 3l18 18" /> </svg>"##;
const METEOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 3l-5 9h5l-6.891 7.086a6.5 6.5 0 1 1 -8.855 -9.506l7.746 -6.58l-1 5l9 -5" /> <path d="M7 14.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> </svg>"##;
const METEOR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.75 5.761l3.25 -2.761l-1 5l9 -5l-5 9h5l-2.467 2.536m-1.983 2.04l-2.441 2.51a6.5 6.5 0 1 1 -8.855 -9.506l2.322 -1.972" /> <path d="M7 14.5a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /> <path d="M3 3l18 18" /> </svg>"##;
const MIST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5h3m4 0h9" /> <path d="M3 10h11m4 0h1" /> <path d="M5 15h5m4 0h7" /> <path d="M3 20h9m4 0h3" /> </svg>"##;
const MIST_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5h9" /> <path d="M3 10h7" /> <path d="M18 10h1" /> <path d="M5 15h5" /> <path d="M14 15h1m4 0h2" /> <path d="M3 20h9m4 0h3" /> <path d="M3 3l18 18" /> </svg>"##;
const MONKEYBAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21v-15l5 -3l5 3v15" /> <path d="M8 21v-7" /> <path d="M3 14h10" /> <path d="M6 10a2 2 0 1 1 4 0" /> <path d="M13 13c6 0 3 8 8 8" /> </svg>"##;
const MOON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3c.132 0 .263 0 .393 0a7.5 7.5 0 0 0 7.92 12.446a9 9 0 1 1 -8.313 -12.454l0 .008" /> </svg>"##;
const MOON_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.418 4.157a8 8 0 0 0 0 15.686" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const MOON_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.962 3.949a8.97 8.97 0 0 1 4.038 -.957v.008h.393a7.478 7.478 0 0 0 -2.07 3.308m-.141 3.84c.186 .823 .514 1.626 .989 2.373a7.49 7.49 0 0 0 4.586 3.268m3.893 -.11c.223 -.067 .444 -.144 .663 -.233a9.088 9.088 0 0 1 -.274 .597m-1.695 2.337a9 9 0 0 1 -12.71 -12.749" /> <path d="M3 3l18 18" /> </svg>"##;
const MOON_STARS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3c.132 0 .263 0 .393 0a7.5 7.5 0 0 0 7.92 12.446a9 9 0 1 1 -8.313 -12.454l0 .008" /> <path d="M17 4a2 2 0 0 0 2 2a2 2 0 0 0 -2 2a2 2 0 0 0 -2 -2a2 2 0 0 0 2 -2" /> <path d="M19 11h2m-1 -1v2" /> </svg>"##;
const MOPED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 16v1a2 2 0 0 0 4 0v-5h-3a3 3 0 0 0 -3 3v1h10a6 6 0 0 1 5 -4v-5a2 2 0 0 0 -2 -2h-1" /> <path d="M6 9l3 0" /> </svg>"##;
const MOTORBIKE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 16a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M16 16a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M7.5 14h5l4 -4h-10.5m1.5 4l4 -4" /> <path d="M13 6h2l1.5 3l2 4" /> </svg>"##;
const MOUNTAIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 20h18l-6.921 -14.612a2.3 2.3 0 0 0 -4.158 0l-6.921 14.612" /> <path d="M7.5 11l2 2.5l2.5 -2.5l2 3l2.5 -2" /> </svg>"##;
const MOUNTAIN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.281 14.26l-4.201 -8.872a2.3 2.3 0 0 0 -4.158 0l-.165 .349m-1.289 2.719l-5.468 11.544h17" /> <path d="M7.5 11l2 2.5l2 -2" /> <path d="M3 3l18 18" /> </svg>"##;
const NAVIGATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18.5l7.265 2.463c.196 .077 .42 .032 .57 -.116a.548 .548 0 0 0 .134 -.572l-7.969 -17.275l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463" /> </svg>"##;
const NAVIGATION_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.559 12.882l-4.559 -9.882l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463l1.036 .351" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const NAVIGATION_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.371 12.476l-4.371 -9.476l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const NAVIGATION_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.487 14.894l-5.487 -11.894l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l6.275 -2.127" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const NAVIGATION_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.653 13.086l-4.653 -10.086l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l6.246 -2.117" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const NAVIGATION_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.387 12.51l-4.387 -9.51l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const NAVIGATION_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.43 12.603l-4.43 -9.603l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463l1.272 .431" /> <path d="M16 21l5 -5" /> <path d="M21 21v.01" /> <path d="M16 16v.01" /> </svg>"##;
const NAVIGATION_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.945 11.551l-3.945 -8.551l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463l1.594 .54" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const NAVIGATION_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.528 12.815l-4.528 -9.815l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const NAVIGATION_EAST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 3h-4v6h4" /> <path d="M10 6h2.5" /> <path d="M16 21l-4 -8l-4 8l4 -2l4 2" /> </svg>"##;
const NAVIGATION_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.535 12.832l-4.535 -9.832l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463c1.38 .468 2.416 .82 3.107 1.053" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const NAVIGATION_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.721 11.067l-3.721 -8.067l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l5.614 -1.903" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const NAVIGATION_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.5 15c-1.232 -2.67 -3.065 -6.67 -5.5 -12l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463" /> <path d="M16 19h6" /> </svg>"##;
const NAVIGATION_NORTH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 21l-4 -8l-4 8l4 -2l4 2" /> <path d="M10 9v-6l4 6v-6" /> </svg>"##;
const NAVIGATION_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.28 12.28c-.95 -2.064 -2.377 -5.157 -4.28 -9.28c-.7 1.515 -1.223 2.652 -1.573 3.41m-1.27 2.75c-.882 1.913 -2.59 5.618 -5.127 11.115c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463l7.265 2.463c.196 .077 .42 .032 .57 -.116a.548 .548 0 0 0 .134 -.572l-.26 -.563" /> <path d="M3 3l18 18" /> </svg>"##;
const NAVIGATION_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.666 13.114l-4.666 -10.114l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463l1.056 .358" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const NAVIGATION_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.002 11.676l-4.002 -8.676l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const NAVIGATION_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.573 12.914l-4.573 -9.914l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const NAVIGATION_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.081 11.847l-4.081 -8.847l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463c1.35 .458 2.362 .8 3.037 1.03" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const NAVIGATION_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.876 11.403l-3.876 -8.403l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l6.29 -2.132" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const NAVIGATION_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.633 13.043l-4.633 -10.043l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463l.955 .324" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const NAVIGATION_SOUTH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 8.25c0 .414 .336 .75 .75 .75h2.25a1 1 0 0 0 1 -1v-1a1 1 0 0 0 -1 -1h-2a1 1 0 0 1 -1 -1v-1a1 1 0 0 1 1 -1h2.25a.75 .75 0 0 1 .75 .75" /> <path d="M16 21l-4 -8l-4 8l4 -2l4 2" /> </svg>"##;
const NAVIGATION_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.574 10.747l-3.574 -7.747l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l5.454 -1.85" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const NAVIGATION_TOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.54 19.977a.34 .34 0 0 0 .357 -.07a.33 .33 0 0 0 .084 -.35l-4.981 -10.557l-4.982 10.557a.33 .33 0 0 0 .084 .35a.34 .34 0 0 0 .357 .07l4.541 -1.477l4.54 1.477" /> <path d="M12 3v2" /> </svg>"##;
const NAVIGATION_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.54 12.843l-4.54 -9.843l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const NAVIGATION_WEST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 3l1 6l2 -3.75l2 3.75l1 -6" /> <path d="M16 21l-4 -8l-4 8l4 -2l4 2" /> </svg>"##;
const NAVIGATION_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.622 13.02l-4.622 -10.02l-7.97 17.275c-.07 .2 -.017 .424 .135 .572c.15 .148 .374 .193 .57 .116l7.265 -2.463l1.563 .53" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const NORTH_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h18" /> <path d="M12 21v-18" /> <path d="M7.5 7.5l9 9" /> <path d="M7.5 16.5l9 -9" /> </svg>"##;
const PARACHUTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12a10 10 0 1 0 -20 0" /> <path d="M22 12c0 -1.66 -1.46 -3 -3.25 -3c-1.8 0 -3.25 1.34 -3.25 3c0 -1.66 -1.57 -3 -3.5 -3s-3.5 1.34 -3.5 3c0 -1.66 -1.46 -3 -3.25 -3c-1.8 0 -3.25 1.34 -3.25 3" /> <path d="M2 12l10 10l-3.5 -10" /> <path d="M15.5 12l-3.5 10l10 -10" /> </svg>"##;
const PARACHUTE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 12c0 -5.523 -4.477 -10 -10 -10c-1.737 0 -3.37 .443 -4.794 1.222m-2.28 1.71a9.969 9.969 0 0 0 -2.926 7.068" /> <path d="M22 12c0 -1.66 -1.46 -3 -3.25 -3c-1.63 0 -2.973 1.099 -3.212 2.54m-.097 -.09c-.23 -1.067 -1.12 -1.935 -2.29 -2.284m-3.445 .568c-.739 .55 -1.206 1.36 -1.206 2.266c0 -1.66 -1.46 -3 -3.25 -3c-1.8 0 -3.25 1.34 -3.25 3" /> <path d="M2 12l10 10l-3.5 -10" /> <path d="M14.582 14.624l-2.582 7.376l4.992 -4.992m2.014 -2.014l3 -3" /> <path d="M3 3l18 18" /> </svg>"##;
const PARKING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14" /> <path d="M10 16v-8h2.667c.736 0 1.333 .895 1.333 2s-.597 2 -1.333 2h-2.667" /> </svg>"##;
const PARKING_METER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 9a3 3 0 0 0 -6 0" /> <path d="M12 19v3" /> <path d="M10.938 19h2.122a4.04 4.04 0 0 0 3.868 -2.82l1.775 -5.68c1.082 -3.463 -.882 -7.138 -4.386 -8.208a6.7 6.7 0 0 0 -1.96 -.292h-.716c-3.668 0 -6.641 2.939 -6.641 6.563c0 .657 .1 1.31 .296 1.937l1.775 5.68a4.04 4.04 0 0 0 3.867 2.82" /> <path d="M11 12h2" /> <path d="M12 12v3" /> </svg>"##;
const PARKING_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4h10a2 2 0 0 1 2 2v10m-.582 3.41c-.362 .365 -.864 .59 -1.418 .59h-12a2 2 0 0 1 -2 -2v-12c0 -.554 .225 -1.056 .59 -1.418" /> <path d="M9 16v-7m3 -1h1a2 2 0 0 1 1.817 2.836m-2.817 1.164h-3" /> <path d="M3 3l18 18" /> </svg>"##;
const PAW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.7 13.5c-1.1 -2 -1.441 -2.5 -2.7 -2.5c-1.259 0 -1.736 .755 -2.836 2.747c-.942 1.703 -2.846 1.845 -3.321 3.291c-.097 .265 -.145 .677 -.143 .962c0 1.176 .787 2 1.8 2c1.259 0 3 -1 4.5 -1s3.241 1 4.5 1c1.013 0 1.8 -.823 1.8 -2c0 -.285 -.049 -.697 -.146 -.962c-.475 -1.451 -2.512 -1.835 -3.454 -3.538" /> <path d="M20.188 8.082a1.039 1.039 0 0 0 -.406 -.082h-.015c-.735 .012 -1.56 .75 -1.993 1.866c-.519 1.335 -.28 2.7 .538 3.052c.129 .055 .267 .082 .406 .082c.739 0 1.575 -.742 2.011 -1.866c.516 -1.335 .273 -2.7 -.54 -3.052l-.001 0" /> <path d="M9.474 9c.055 0 .109 0 .163 -.011c.944 -.128 1.533 -1.346 1.32 -2.722c-.203 -1.297 -1.047 -2.267 -1.932 -2.267c-.055 0 -.109 0 -.163 .011c-.944 .128 -1.533 1.346 -1.32 2.722c.204 1.293 1.048 2.267 1.933 2.267" /> <path d="M16.456 6.733c.214 -1.376 -.375 -2.594 -1.32 -2.722a1.164 1.164 0 0 0 -.162 -.011c-.885 0 -1.728 .97 -1.93 2.267c-.214 1.376 .375 2.594 1.32 2.722c.054 .007 .108 .011 .162 .011c.885 0 1.73 -.974 1.93 -2.267" /> <path d="M5.69 12.918c.816 -.352 1.054 -1.719 .536 -3.052c-.436 -1.124 -1.271 -1.866 -2.009 -1.866c-.14 0 -.277 .027 -.407 .082c-.816 .352 -1.054 1.719 -.536 3.052c.436 1.124 1.271 1.866 2.009 1.866c.14 0 .277 -.027 .407 -.082" /> </svg>"##;
const PAW_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.168 11.154c-.71 .31 -1.184 1.107 -2 2.593c-.942 1.703 -2.846 1.845 -3.321 3.291c-.097 .265 -.145 .677 -.143 .962c0 1.176 .787 2 1.8 2c1.259 0 3 -1 4.5 -1s3.241 1 4.5 1c.927 0 1.664 -.689 1.783 -1.708" /> <path d="M20.188 8.082a1.039 1.039 0 0 0 -.406 -.082h-.015c-.735 .012 -1.56 .75 -1.993 1.866c-.519 1.335 -.28 2.7 .538 3.052c.129 .055 .267 .082 .406 .082c.739 0 1.575 -.742 2.011 -1.866c.516 -1.335 .273 -2.7 -.54 -3.052l-.001 0" /> <path d="M11 6.992a3.608 3.608 0 0 0 -.04 -.725c-.203 -1.297 -1.047 -2.267 -1.932 -2.267a1.237 1.237 0 0 0 -.758 .265" /> <path d="M16.456 6.733c.214 -1.376 -.375 -2.594 -1.32 -2.722a1.164 1.164 0 0 0 -.162 -.011c-.885 0 -1.728 .97 -1.93 2.267c-.214 1.376 .375 2.594 1.32 2.722c.054 .007 .108 .011 .162 .011c.885 0 1.73 -.974 1.93 -2.267" /> <path d="M5.69 12.918c.816 -.352 1.054 -1.719 .536 -3.052c-.436 -1.124 -1.271 -1.866 -2.009 -1.866c-.14 0 -.277 .027 -.407 .082c-.816 .352 -1.054 1.719 -.536 3.052c.436 1.124 1.271 1.866 2.009 1.866c.14 0 .277 -.027 .407 -.082" /> <path d="M3 3l18 18" /> </svg>"##;
const PENNANT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 21l4 0" /> <path d="M10 21l0 -18" /> <path d="M10 4l9 4l-9 4" /> </svg>"##;
const PENNANT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 21h-4" /> <path d="M14 21v-18" /> <path d="M14 4l-9 4l9 4" /> </svg>"##;
const PENNANT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 21h4" /> <path d="M10 21v-11m0 -4v-3" /> <path d="M10 4l9 4l-4.858 2.16m-2.764 1.227l-1.378 .613" /> <path d="M3 3l18 18" /> </svg>"##;
const PICNIC_TABLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 7l2 9m-10 -9l-2 9m-1 -9h14m2 5h-18" /> </svg>"##;
const PIG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11v.01" /> <path d="M16 3l0 3.803a6.019 6.019 0 0 1 2.658 3.197h1.341a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-1.342a6.008 6.008 0 0 1 -1.658 2.473v2.027a1.5 1.5 0 0 1 -3 0v-.583a6.04 6.04 0 0 1 -1 .083h-4a6.04 6.04 0 0 1 -1 -.083v.583a1.5 1.5 0 0 1 -3 0v-2l0 -.027a6 6 0 0 1 4 -10.473h2.5l4.5 -3" /> </svg>"##;
const PIG_MONEY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11v.01" /> <path d="M5.173 8.378a3 3 0 1 1 4.656 -1.377" /> <path d="M16 4v3.803a6.019 6.019 0 0 1 2.658 3.197h1.341a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-1.342c-.336 .95 -.907 1.8 -1.658 2.473v2.027a1.5 1.5 0 0 1 -3 0v-.583a6.04 6.04 0 0 1 -1 .083h-4a6.04 6.04 0 0 1 -1 -.083v.583a1.5 1.5 0 0 1 -3 0v-2l0 -.027a6 6 0 0 1 4 -10.473h2.5l4.5 -3" /> </svg>"##;
const PIG_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11v.01" /> <path d="M10 6h1.499l4.5 -3l0 3.803a6.019 6.019 0 0 1 2.658 3.197h1.341a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-1.342c-.057 .16 -.12 .318 -.19 .472m-1.467 2.528v1.5a1.5 1.5 0 0 1 -3 0v-.583a6.04 6.04 0 0 1 -1 .083h-4a6.04 6.04 0 0 1 -1 -.083v.583a1.5 1.5 0 0 1 -3 0v-2l0 -.027a6 6 0 0 1 1.5 -9.928" /> <path d="M3 3l18 18" /> </svg>"##;
const PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 4.5l-4 4l-4 1.5l-1.5 1.5l7 7l1.5 -1.5l1.5 -4l4 -4" /> <path d="M9 15l-4.5 4.5" /> <path d="M14.5 4l5.5 5.5" /> </svg>"##;
const PINNED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 4v6l-2 4v2h10v-2l-2 -4v-6" /> <path d="M12 16l0 5" /> <path d="M8 4l8 0" /> </svg>"##;
const PINNED_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M15 4.5l-3.249 3.249m-2.57 1.433l-2.181 .818l-1.5 1.5l7 7l1.5 -1.5l.82 -2.186m1.43 -2.563l3.25 -3.251" /> <path d="M9 15l-4.5 4.5" /> <path d="M14.5 4l5.5 5.5" /> </svg>"##;
const PLANE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 10h4a2 2 0 0 1 0 4h-4l-4 7h-3l2 -7h-4l-2 2h-3l2 -4l-2 -4h3l2 2h4l-2 -7h3l4 7" /> </svg>"##;
const PLANE_ARRIVAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.157 11.81l4.83 1.295a2 2 0 1 1 -1.036 3.863l-14.489 -3.882l-1.345 -6.572l2.898 .776l1.414 2.45l2.898 .776l-.12 -7.279l2.898 .777l2.052 7.797" /> <path d="M3 21h18" /> </svg>"##;
const PLANE_DEPARTURE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.639 10.258l4.83 -1.294a2 2 0 1 1 1.035 3.863l-14.489 3.883l-4.45 -5.02l2.897 -.776l2.45 1.414l2.897 -.776l-3.743 -6.244l2.898 -.777l5.675 5.727" /> <path d="M3 21h18" /> </svg>"##;
const PLANE_INFLIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 11.085h5a2 2 0 1 1 0 4h-15l-3 -6h3l2 2h3l-2 -7h3l4 7" /> <path d="M3 21h18" /> </svg>"##;
const PLANE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.788 5.758l-.788 -2.758h3l4 7h4a2 2 0 1 1 0 4h-2m-2.718 1.256l-3.282 5.744h-3l2 -7h-4l-2 2h-3l2 -4l-2 -4h3l2 2h3" /> <path d="M3 3l18 18" /> </svg>"##;
const PLANE_TILT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.5 6.5l3 -2.9a2.05 2.05 0 0 1 2.9 2.9l-2.9 3l2.5 7.5l-2.5 2.55l-3.5 -6.55l-3 3v3l-2 2l-1.5 -4.5l-4.5 -1.5l2 -2h3l3 -3l-6.5 -3.5l2.5 -2.5l7.5 2.5" /> </svg>"##;
const PLANET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.816 13.58c2.292 2.138 3.546 4 3.092 4.9c-.745 1.46 -5.783 -.259 -11.255 -3.838c-5.47 -3.579 -9.304 -7.664 -8.56 -9.123c.464 -.91 2.926 -.444 5.803 .805" /> <path d="M5 12a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> </svg>"##;
const PLANET_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.042 7.059a7 7 0 0 0 9.908 9.89m1.581 -2.425a7 7 0 0 0 -9.057 -9.054" /> <path d="M3 3l18 18" /> </svg>"##;
const PLANT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 15h10v4a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2v-4" /> <path d="M12 9a6 6 0 0 0 -6 -6h-3v2a6 6 0 0 0 6 6h3" /> <path d="M12 11a6 6 0 0 1 6 -6h3v1a6 6 0 0 1 -6 6h-3" /> <path d="M12 15l0 -6" /> </svg>"##;
const PLANT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 9a10 10 0 1 0 20 0" /> <path d="M12 19a10 10 0 0 1 10 -10" /> <path d="M2 9a10 10 0 0 1 10 10" /> <path d="M12 4a9.7 9.7 0 0 1 2.99 7.5" /> <path d="M9.01 11.5a9.7 9.7 0 0 1 2.99 -7.5" /> </svg>"##;
const PLANT_2_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 9c0 5.523 4.477 10 10 10a9.953 9.953 0 0 0 5.418 -1.593m2.137 -1.855a9.961 9.961 0 0 0 2.445 -6.552" /> <path d="M12 19c0 -1.988 .58 -3.84 1.58 -5.397m1.878 -2.167a9.961 9.961 0 0 1 6.542 -2.436" /> <path d="M2 9a10 10 0 0 1 10 10" /> <path d="M12 4a9.7 9.7 0 0 1 3 7.013" /> <path d="M9.01 11.5a9.696 9.696 0 0 1 .163 -2.318m1.082 -2.942a9.696 9.696 0 0 1 1.745 -2.24" /> <path d="M3 3l18 18" /> </svg>"##;
const PLANT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 17v2a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2v-4h8" /> <path d="M11.9 7.908a6 6 0 0 0 -4.79 -4.806m-4.11 -.102v2a6 6 0 0 0 6 6h2" /> <path d="M12.531 8.528a6 6 0 0 1 5.469 -3.528h3v1a6 6 0 0 1 -5.037 5.923" /> <path d="M12 15v-3" /> <path d="M3 3l18 18" /> </svg>"##;
const POKEBALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M3 12h6" /> <path d="M15 12h6" /> </svg>"##;
const POKEBALL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.04 16.048a9 9 0 0 0 -12.083 -12.09m-2.32 1.678a9 9 0 1 0 12.737 12.719" /> <path d="M9.884 9.874a3 3 0 1 0 4.24 4.246m.57 -3.441a3.012 3.012 0 0 0 -1.41 -1.39" /> <path d="M3 12h6m7 0h5" /> <path d="M3 3l18 18" /> </svg>"##;
const PRISON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 4v16" /> <path d="M14 4v16" /> <path d="M6 4v5" /> <path d="M6 15v5" /> <path d="M10 4v5" /> <path d="M11 9h-6v6h6l0 -6" /> <path d="M10 15v5" /> <path d="M8 12h-.01" /> </svg>"##;
const RADAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12h-8a1 1 0 1 0 -1 1v8a9 9 0 0 0 9 -9" /> <path d="M16 9a5 5 0 1 0 -7 7" /> <path d="M20.486 9a9 9 0 1 0 -11.482 11.495" /> </svg>"##;
const RADAR_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M15.51 15.56a5 5 0 1 0 -3.51 1.44" /> <path d="M18.832 17.86a9 9 0 1 0 -6.832 3.14" /> <path d="M12 12v9" /> </svg>"##;
const RADAR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.291 11.295a1 1 0 0 0 .709 1.705v8c2.488 0 4.74 -1.01 6.37 -2.642m1.675 -2.319a8.962 8.962 0 0 0 .955 -4.039h-5" /> <path d="M16 9a5 5 0 0 0 -5.063 -1.88m-2.466 1.347a5 5 0 0 0 .53 7.535" /> <path d="M20.486 9a9 9 0 0 0 -12.525 -5.032m-2.317 1.675a9 9 0 0 0 3.36 14.852" /> <path d="M3 3l18 18" /> </svg>"##;
const RAINBOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 17c0 -5.523 -4.477 -10 -10 -10c-5.523 0 -10 4.477 -10 10" /> <path d="M18 17a6 6 0 1 0 -12 0" /> <path d="M14 17a2 2 0 1 0 -4 0" /> </svg>"##;
const RAINBOW_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 17c0 -5.523 -4.477 -10 -10 -10c-.308 0 -.613 .014 -.914 .041m-3.208 .845a10 10 0 0 0 -5.878 9.114" /> <path d="M11.088 11.069a6 6 0 0 0 -5.088 5.931" /> <path d="M14 17a2 2 0 1 0 -4 0" /> <path d="M3 3l18 18" /> </svg>"##;
const RIPPLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7c3 -2 6 -2 9 0s6 2 9 0" /> <path d="M3 17c3 -2 6 -2 9 0s6 2 9 0" /> <path d="M3 12c3 -2 6 -2 9 0s6 2 9 0" /> </svg>"##;
const RIPPLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7c.915 -.61 1.83 -1.034 2.746 -1.272m4.212 .22c.68 .247 1.361 .598 2.042 1.052c3 2 6 2 9 0" /> <path d="M3 17c3 -2 6 -2 9 0c2.092 1.395 4.184 1.817 6.276 1.266" /> <path d="M3 12c3 -2 6 -2 9 0m5.482 1.429c1.173 -.171 2.345 -.647 3.518 -1.429" /> <path d="M3 3l18 18" /> </svg>"##;
const ROAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19l4 -14" /> <path d="M16 5l4 14" /> <path d="M12 8v-2" /> <path d="M12 13v-2" /> <path d="M12 18v-2" /> </svg>"##;
const ROAD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19l3.332 -11.661" /> <path d="M16 5l2.806 9.823" /> <path d="M12 8v-2" /> <path d="M12 13v-1" /> <path d="M12 18v-2" /> <path d="M3 3l18 18" /> </svg>"##;
const ROAD_SIGN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.446 2.6l7.955 7.954a2.045 2.045 0 0 1 0 2.892l-7.955 7.955a2.045 2.045 0 0 1 -2.892 0l-7.955 -7.955a2.045 2.045 0 0 1 0 -2.892l7.955 -7.955a2.045 2.045 0 0 1 2.892 0" /> <path d="M9 14v-2c0 -.59 .414 -1 1 -1h5" /> <path d="M13 9l2 2l-2 2" /> </svg>"##;
const ROCKET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 13a8 8 0 0 1 7 7a6 6 0 0 0 3 -5a9 9 0 0 0 6 -8a3 3 0 0 0 -3 -3a9 9 0 0 0 -8 6a6 6 0 0 0 -5 3" /> <path d="M7 14a6 6 0 0 0 -3 6a6 6 0 0 0 6 -3" /> <path d="M14 9a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const ROCKET_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.29 9.275a9.03 9.03 0 0 0 -.29 .725a6 6 0 0 0 -5 3a8 8 0 0 1 7 7a6 6 0 0 0 3 -5c.241 -.085 .478 -.18 .708 -.283m2.428 -1.61a9 9 0 0 0 2.864 -6.107a3 3 0 0 0 -3 -3a9 9 0 0 0 -6.107 2.864" /> <path d="M7 14a6 6 0 0 0 -3 6a6 6 0 0 0 6 -3" /> <path d="M14 9a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3 3l18 18" /> </svg>"##;
const ROLLERCOASTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21a5.55 5.55 0 0 0 5.265 -3.795l.735 -2.205a8.775 8.775 0 0 1 8.325 -6h3.675" /> <path d="M20 9v12" /> <path d="M8 21v-3" /> <path d="M12 21v-10" /> <path d="M16 9.5v11.5" /> <path d="M15 3h5v3h-5l0 -3" /> <path d="M6 8l4 -3l2 2.5l-4 3l-1.8 -.5l-.2 -2" /> </svg>"##;
const ROLLERCOASTER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21a5.55 5.55 0 0 0 5.265 -3.795l.735 -2.205a8.759 8.759 0 0 1 2.35 -3.652m2.403 -1.589a8.76 8.76 0 0 1 3.572 -.759h3.675" /> <path d="M20 9v7m0 4v1" /> <path d="M8 21v-3" /> <path d="M12 21v-9" /> <path d="M16 9.5v2.5m0 4v5" /> <path d="M15 3h5v3h-5l0 -3" /> <path d="M9.446 5.415l.554 -.415l2 2.5l-.285 .213m-2.268 1.702l-1.447 1.085l-1.8 -.5l-.2 -2l1.139 -.854" /> <path d="M3 3l18 18" /> </svg>"##;
const ROUTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M19 7a2 2 0 1 0 0 -4a2 2 0 0 0 0 4" /> <path d="M11 19h5.5a3.5 3.5 0 0 0 0 -7h-8a3.5 3.5 0 0 1 0 -7h4.5" /> </svg>"##;
const ROUTE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M19 7a2 2 0 1 0 0 -4a2 2 0 0 0 0 4" /> <path d="M14 5a2 2 0 0 0 -2 2v10a2 2 0 0 1 -2 2" /> </svg>"##;
const ROUTE_ALT_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 3h-5v5" /> <path d="M16 3h5v5" /> <path d="M3 3l7.536 7.536a5 5 0 0 1 1.464 3.534v6.93" /> <path d="M18 6.01v-.01" /> <path d="M16 8.02v-.01" /> <path d="M14 10v.01" /> </svg>"##;
const ROUTE_ALT_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 3h5v5" /> <path d="M8 3h-5v5" /> <path d="M21 3l-7.536 7.536a5 5 0 0 0 -1.464 3.534v6.93" /> <path d="M6 6.01v-.01" /> <path d="M8 8.02v-.01" /> <path d="M10 10v.01" /> </svg>"##;
const ROUTE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M12 19h4.5c.71 0 1.372 -.212 1.924 -.576m1.545 -2.459a3.5 3.5 0 0 0 -3.469 -3.965h-.499m-4 0h-3.501a3.5 3.5 0 0 1 -2.477 -5.972m2.477 -1.028h3.5" /> <path d="M3 3l18 18" /> </svg>"##;
const ROUTE_SQUARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17h4v4h-4l0 -4" /> <path d="M17 3h4v4h-4l0 -4" /> <path d="M11 19h5.5a3.5 3.5 0 0 0 0 -7h-8a3.5 3.5 0 0 1 0 -7h4.5" /> </svg>"##;
const ROUTE_SQUARE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 5a2 2 0 0 0 -2 2v10a2 2 0 0 1 -2 2" /> <path d="M3 17h4v4h-4l0 -4" /> <path d="M17 3h4v4h-4l0 -4" /> </svg>"##;
const ROUTE_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17l4 4" /> <path d="M7 17l-4 4" /> <path d="M17 3l4 4" /> <path d="M21 3l-4 4" /> <path d="M11 19h5.5a3.5 3.5 0 0 0 0 -7h-8a3.5 3.5 0 0 1 0 -7h4.5" /> </svg>"##;
const ROUTE_X_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17l4 4" /> <path d="M7 17l-4 4" /> <path d="M17 3l4 4" /> <path d="M21 3l-4 4" /> <path d="M14 5a2 2 0 0 0 -2 2v10a2 2 0 0 1 -2 2" /> </svg>"##;
const SAILBOAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 20a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1" /> <path d="M4 18l-1 -3h18l-1 3" /> <path d="M11 12h7l-7 -9v9" /> <path d="M8 7l-2 5" /> </svg>"##;
const SAILBOAT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 20a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1" /> <path d="M4 18l-1 -3h18l-1 3" /> <path d="M12 11v4" /> <path d="M7 3c1.333 2.667 1.333 5.333 0 8h10c1.333 -2.667 1.333 -5.333 0 -8" /> <path d="M6 3h12" /> </svg>"##;
const SAILBOAT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 20a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1" /> <path d="M4 18l-1 -3h12m4 0h2l-.506 1.517" /> <path d="M11 11v1h1m4 0h2l-7 -9v4" /> <path d="M7.713 7.718l-1.713 4.282" /> <path d="M3 3l18 18" /> </svg>"##;
const SATELLITE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.707 6.293l2.586 -2.586a1 1 0 0 1 1.414 0l5.586 5.586a1 1 0 0 1 0 1.414l-2.586 2.586a1 1 0 0 1 -1.414 0l-5.586 -5.586a1 1 0 0 1 0 -1.414" /> <path d="M6 10l-3 3l3 3l3 -3" /> <path d="M10 6l3 -3l3 3l-3 3" /> <path d="M12 12l1.5 1.5" /> <path d="M14.5 17a2.5 2.5 0 0 0 2.5 -2.5" /> <path d="M15 21a6 6 0 0 0 6 -6" /> </svg>"##;
const SATELLITE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.707 3.707l5.586 5.586m-1.293 2.707l-1.293 1.293a1 1 0 0 1 -1.414 0l-5.586 -5.586a1 1 0 0 1 0 -1.414l1.293 -1.293" /> <path d="M6 10l-3 3l3 3l3 -3" /> <path d="M10 6l3 -3l3 3l-3 3" /> <path d="M12 12l1.5 1.5" /> <path d="M14.5 17c.69 0 1.316 -.28 1.769 -.733" /> <path d="M15 21c1.654 0 3.151 -.67 4.237 -1.752m1.507 -2.507a6 6 0 0 0 .256 -1.741" /> <path d="M3 3l18 18" /> </svg>"##;
const SCHOOL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 9l-10 -4l-10 4l10 4l10 -4v6" /> <path d="M6 10.6v5.4a6 3 0 0 0 12 0v-5.4" /> </svg>"##;
const SCHOOL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 9l-10 -4l-2.136 .854m-2.864 1.146l-5 2l10 4l.697 -.279m2.878 -1.151l6.425 -2.57v6" /> <path d="M6 10.6v5.4c0 1.657 2.686 3 6 3c2.334 0 4.357 -.666 5.35 -1.64m.65 -3.36v-3.4" /> <path d="M3 3l18 18" /> </svg>"##;
const SCOOTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M8 17h5a6 6 0 0 1 5 -5v-5a2 2 0 0 0 -2 -2h-1" /> </svg>"##;
const SCOOTER_ELECTRIC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M8 17h5a6 6 0 0 1 5 -5v-5a2 2 0 0 0 -2 -2h-1" /> <path d="M10 4l-2 4h3l-2 4" /> </svg>"##;
const SEEDLING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 10a6 6 0 0 0 -6 -6h-3v2a6 6 0 0 0 6 6h3" /> <path d="M12 14a6 6 0 0 1 6 -6h3v1a6 6 0 0 1 -6 6h-3" /> <path d="M12 20l0 -10" /> </svg>"##;
const SEEDLING_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.412 7.407a6.025 6.025 0 0 0 -2.82 -2.82m-4.592 -.587h-1v2a6 6 0 0 0 6 6h3" /> <path d="M12 14a6 6 0 0 1 .255 -1.736m1.51 -2.514a5.981 5.981 0 0 1 4.235 -1.75h3v1c0 2.158 -1.14 4.05 -2.85 5.107m-3.15 .893h-3" /> <path d="M12 20v-8" /> <path d="M3 3l18 18" /> </svg>"##;
const SEGWAY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 3h3q -2.25 5 .75 11" /> <path d="M8 17a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M12 17.01v.01" /> </svg>"##;
const SHIP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 20a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1" /> <path d="M4 18l-1 -5h18l-2 4" /> <path d="M5 13v-6h8l4 6" /> <path d="M7 7v-4h-1" /> </svg>"##;
const SHIP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 20a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1" /> <path d="M4 18l-1 -5h10m4 0h4l-1.334 2.668" /> <path d="M5 13v-6h2m4 0h2l4 6" /> <path d="M3 3l18 18" /> </svg>"##;
const SIGN_LEFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 21h-4" /> <path d="M14 21v-10" /> <path d="M14 6v-3" /> <path d="M18 6h-10l-2 2.5l2 2.5h10l0 -5" /> </svg>"##;
const SIGN_RIGHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 21h4" /> <path d="M10 21v-10" /> <path d="M10 6v-3" /> <path d="M6 6h10l2 2.5l-2 2.5h-10l0 -5" /> </svg>"##;
const SKATEBOARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 9a2 1 0 0 0 2 1h14a2 1 0 0 0 2 -1" /> </svg>"##;
const SKATEBOARD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 15a2 2 0 0 0 2 2m2 -2a2 2 0 0 0 -2 -2" /> <path d="M3 9c0 .552 .895 1 2 1h5m4 0h5c1.105 0 2 -.448 2 -1" /> <path d="M3 3l18 18" /> </svg>"##;
const SLEIGH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19h15a4 4 0 0 0 4 -4" /> <path d="M16 15h-9a4 4 0 0 1 -4 -4v-6l1.243 1.243a6 6 0 0 0 4.242 1.757h3.515v2a2 2 0 0 0 2 2h.5a1.5 1.5 0 0 0 1.5 -1.5a1.5 1.5 0 0 1 3 0v1.5a3 3 0 0 1 -3 3" /> <path d="M15 15v4" /> <path d="M7 15v4" /> </svg>"##;
const SNOWFLAKE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 4l2 1l2 -1" /> <path d="M12 2v6.5l3 1.72" /> <path d="M17.928 6.268l.134 2.232l1.866 1.232" /> <path d="M20.66 7l-5.629 3.25l.01 3.458" /> <path d="M19.928 14.268l-1.866 1.232l-.134 2.232" /> <path d="M20.66 17l-5.629 -3.25l-2.99 1.738" /> <path d="M14 20l-2 -1l-2 1" /> <path d="M12 22v-6.5l-3 -1.72" /> <path d="M6.072 17.732l-.134 -2.232l-1.866 -1.232" /> <path d="M3.34 17l5.629 -3.25l-.01 -3.458" /> <path d="M4.072 9.732l1.866 -1.232l.134 -2.232" /> <path d="M3.34 7l5.629 3.25l2.99 -1.738" /> </svg>"##;
const SNOWFLAKE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 4l2 1l2 -1" /> <path d="M12 2v6m1.196 1.186l1.804 1.034" /> <path d="M17.928 6.268l.134 2.232l1.866 1.232" /> <path d="M20.66 7l-5.629 3.25l-.031 .75" /> <path d="M19.928 14.268l-1.015 .67" /> <path d="M14.212 14.226l-2.171 1.262" /> <path d="M14 20l-2 -1l-2 1" /> <path d="M12 22v-6.5l-3 -1.72" /> <path d="M6.072 17.732l-.134 -2.232l-1.866 -1.232" /> <path d="M3.34 17l5.629 -3.25l-.01 -3.458" /> <path d="M4.072 9.732l1.866 -1.232l.134 -2.232" /> <path d="M3.34 7l5.629 3.25l.802 -.466" /> <path d="M3 3l18 18" /> </svg>"##;
const SNOWMAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3a4 4 0 0 1 2.906 6.75a6 6 0 1 1 -5.81 0a4 4 0 0 1 2.904 -6.75" /> <path d="M17.5 11.5l2.5 -1.5" /> <path d="M6.5 11.5l-2.5 -1.5" /> <path d="M12 13h.01" /> <path d="M12 16h.01" /> </svg>"##;
const SPEEDBOAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 17h14.4a3 3 0 0 0 2.5 -1.34l3.1 -4.66h-6.23a4 4 0 0 0 -1.49 .29l-3.56 1.42a4 4 0 0 1 -1.49 .29h-5.73l-1.5 4" /> <path d="M6 13l1.5 -5" /> <path d="M6 8h8l2 3" /> </svg>"##;
const SPIDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 4v2l5 5" /> <path d="M2.5 9.5l1.5 1.5h6" /> <path d="M4 19v-2l6 -6" /> <path d="M19 4v2l-5 5" /> <path d="M21.5 9.5l-1.5 1.5h-6" /> <path d="M20 19v-2l-6 -6" /> <path d="M8 15a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M10 9a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const STAIRS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 5h-5v5h-5v5h-5v5h-5" /> </svg>"##;
const STAIRS_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 21h-5v-5h-5v-5h-5v-5h-5" /> <path d="M18 3v7" /> <path d="M15 7l3 3l3 -3" /> </svg>"##;
const STAIRS_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 6h-5v5h-5v5h-5v5h-5" /> <path d="M6 10v-7" /> <path d="M3 6l3 -3l3 3" /> </svg>"##;
const STEERING_WHEEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M12 14l0 7" /> <path d="M10 12l-6.75 -2" /> <path d="M14 12l6.75 -2" /> </svg>"##;
const STEERING_WHEEL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.04 16.048a9 9 0 0 0 -12.083 -12.09m-2.32 1.678a9 9 0 1 0 12.737 12.719" /> <path d="M10.595 10.576a2 2 0 1 0 2.827 2.83" /> <path d="M12 14v7" /> <path d="M10 12l-6.75 -2" /> <path d="M15.542 11.543l5.208 -1.543" /> <path d="M3 3l18 18" /> </svg>"##;
const STORM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M5 12a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M5.369 14.236c-1.839 -3.929 -1.561 -7.616 -.704 -11.236" /> <path d="M18.63 9.76c1.837 3.928 1.561 7.615 .703 11.236" /> </svg>"##;
const STORM_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.884 9.874a3 3 0 1 0 4.24 4.246m.57 -3.441a3.012 3.012 0 0 0 -1.41 -1.39" /> <path d="M7.037 7.063a7 7 0 0 0 9.907 9.892m1.585 -2.426a7 7 0 0 0 -9.058 -9.059" /> <path d="M5.369 14.236c-1.605 -3.428 -1.597 -6.673 -1 -9.849" /> <path d="M18.63 9.76a14.323 14.323 0 0 1 1.368 6.251m-.37 3.608c-.087 .46 -.187 .92 -.295 1.377" /> <path d="M3 3l18 18" /> </svg>"##;
const SUBMARINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 11v6h2l1 -1.5l3 1.5h10a3 3 0 0 0 0 -6h-10l-3 1.5l-1 -1.5h-2" /> <path d="M17 11l-1 -3h-5l-1 3" /> <path d="M13 8v-2a1 1 0 0 1 1 -1h1" /> </svg>"##;
const SUN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M3 12h1m8 -9v1m8 8h1m-9 8v1m-6.4 -15.4l.7 .7m12.1 -.7l-.7 .7m0 11.4l.7 .7m-12.1 -.7l-.7 .7" /> </svg>"##;
const SUN_ELECTRICITY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12a4 4 0 0 0 4 4m0 -8a4 4 0 0 0 -4 4" /> <path d="M3 12h1" /> <path d="M12 3v1" /> <path d="M12 20v1" /> <path d="M5.6 5.6l.7 .7" /> <path d="M6.3 17.7l-.7 .7" /> <path d="M20 7l-3 5h4l-3 5" /> </svg>"##;
const SUN_HIGH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.828 14.828a4 4 0 1 0 -5.656 -5.656a4 4 0 0 0 5.656 5.656" /> <path d="M6.343 17.657l-1.414 1.414" /> <path d="M6.343 6.343l-1.414 -1.414" /> <path d="M17.657 6.343l1.414 -1.414" /> <path d="M17.657 17.657l1.414 1.414" /> <path d="M4 12h-2" /> <path d="M12 4v-2" /> <path d="M20 12h2" /> <path d="M12 20v2" /> </svg>"##;
const SUN_LOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M4 12h.01" /> <path d="M12 4v.01" /> <path d="M20 12h.01" /> <path d="M12 20v.01" /> <path d="M6.31 6.31l-.01 -.01" /> <path d="M17.71 6.31l-.01 -.01" /> <path d="M17.7 17.7l.01 .01" /> <path d="M6.3 17.7l.01 .01" /> </svg>"##;
const SUN_MOON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.173 14.83a4 4 0 1 1 5.657 -5.657" /> <path d="M11.294 12.707l.174 .247a7.5 7.5 0 0 0 8.845 2.492a9 9 0 0 1 -14.671 2.914" /> <path d="M3 12h1" /> <path d="M12 3v1" /> <path d="M5.6 5.6l.7 .7" /> <path d="M3 21l18 -18" /> </svg>"##;
const SUN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M16 12a4 4 0 0 0 -4 -4m-2.834 1.177a4 4 0 0 0 5.66 5.654" /> <path d="M3 12h1m8 -9v1m8 8h1m-9 8v1m-6.4 -15.4l.7 .7m12.1 -.7l-.7 .7m0 11.4l.7 .7m-12.1 -.7l-.7 .7" /> </svg>"##;
const SUN_WIND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.468 10a4 4 0 1 0 -5.466 5.46" /> <path d="M2 12h1" /> <path d="M11 3v1" /> <path d="M11 20v1" /> <path d="M4.6 5.6l.7 .7" /> <path d="M17.4 5.6l-.7 .7" /> <path d="M5.3 17.7l-.7 .7" /> <path d="M15 13h5a2 2 0 1 0 0 -4" /> <path d="M12 16h5.714l.253 0a2 2 0 0 1 2.033 2a2 2 0 0 1 -2 2h-.286" /> </svg>"##;
const SUNRISE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17h1m16 0h1m-15.4 -6.4l.7 .7m12.1 -.7l-.7 .7m-9.7 5.7a4 4 0 0 1 8 0" /> <path d="M3 21l18 0" /> <path d="M12 9v-6l3 3m-6 0l3 -3" /> </svg>"##;
const SUNSET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17h1m16 0h1m-15.4 -6.4l.7 .7m12.1 -.7l-.7 .7m-9.7 5.7a4 4 0 0 1 8 0" /> <path d="M3 21l18 0" /> <path d="M12 3v6l3 -3m-6 0l3 3" /> </svg>"##;
const SUNSET_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 13h1" /> <path d="M20 13h1" /> <path d="M5.6 6.6l.7 .7" /> <path d="M18.4 6.6l-.7 .7" /> <path d="M8 13a4 4 0 1 1 8 0" /> <path d="M3 17h18" /> <path d="M7 20h5" /> <path d="M16 20h1" /> <path d="M12 5v-1" /> </svg>"##;
const TANK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 15a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3" /> <path d="M6 12l1 -5h5l3 5" /> <path d="M21 9l-7.8 0" /> </svg>"##;
const TARGET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M7 12a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const TARGET_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.286 11.3a1 1 0 0 0 1.41 1.419" /> <path d="M8.44 8.49a5 5 0 0 0 7.098 7.044m1.377 -2.611a5 5 0 0 0 -5.846 -5.836" /> <path d="M5.649 5.623a9 9 0 1 0 12.698 12.758m1.683 -2.313a9 9 0 0 0 -12.076 -12.11" /> <path d="M3 3l18 18" /> </svg>"##;
const TEMPERATURE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 13.5a4 4 0 1 0 4 0v-8.5a2 2 0 0 0 -4 0v8.5" /> <path d="M10 9l4 0" /> </svg>"##;
const TEMPERATURE_CELSIUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M20 9a3 3 0 0 0 -3 -3h-1a3 3 0 0 0 -3 3v6a3 3 0 0 0 3 3h1a3 3 0 0 0 3 -3" /> </svg>"##;
const TEMPERATURE_FAHRENHEIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M13 12l5 0" /> <path d="M20 6h-6a1 1 0 0 0 -1 1v11" /> </svg>"##;
const TEMPERATURE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13.5a4 4 0 1 0 4 0v-8.5a2 2 0 0 0 -4 0v8.5" /> <path d="M8 9l4 0" /> <path d="M16 9l6 0" /> </svg>"##;
const TEMPERATURE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 10v3.5a4 4 0 1 0 5.836 2.33m-1.836 -5.83v-5a2 2 0 1 0 -4 0v1" /> <path d="M13 9h1" /> <path d="M3 3l18 18" /> </svg>"##;
const TEMPERATURE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13.5a4 4 0 1 0 4 0v-8.5a2 2 0 0 0 -4 0v8.5" /> <path d="M8 9l4 0" /> <path d="M16 9l6 0" /> <path d="M19 6l0 6" /> </svg>"##;
const TENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 14l4 6h6l-9 -16l-9 16h6l4 -6" /> </svg>"##;
const TENT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 14l4 6h5m-2.863 -6.868l-5.137 -9.132l-1.44 2.559m-1.44 2.563l-6.12 10.878h6l4 -6" /> <path d="M3 3l18 18" /> </svg>"##;
const THEATER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h16" /> <path d="M20 16v-10a2 2 0 0 0 -2 -2h-12a2 2 0 0 0 -2 2v10l4 -6c2.667 1.333 5.333 1.333 8 0l4 6" /> </svg>"##;
const TIR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 18h8m4 0h2v-6a5 7 0 0 0 -5 -7h-1l1.5 7h4.5" /> <path d="M12 18v-13h3" /> <path d="M3 17l0 -5l9 0" /> </svg>"##;
const TOOLS_KITCHEN_2_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.386 10.409c.53 -2.28 1.766 -4.692 4.614 -7.409v12m-4 0h-1c0 -.313 0 -.627 0 -.941" /> <path d="M19 19v2h-1v-3" /> <path d="M8 8v13" /> <path d="M5 5v2a3 3 0 0 0 4.546 2.572m1.454 -2.572v-3" /> <path d="M3 3l18 18" /> </svg>"##;
const TOOLS_KITCHEN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 3h5l-.5 4.5m-.4 3.595l-.1 .905h-6l-.875 -7.874" /> <path d="M7 18h2v3h-2v-3" /> <path d="M15.225 11.216c.42 -2.518 1.589 -5.177 4.775 -8.216v12h-1" /> <path d="M20 15v1m0 4v1h-1v-2" /> <path d="M8 12v6" /> <path d="M3 3l18 18" /> </svg>"##;
const TORNADO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 4l-18 0" /> <path d="M13 16l-6 0" /> <path d="M11 20l4 0" /> <path d="M6 8l14 0" /> <path d="M4 12l12 0" /> </svg>"##;
const TRACK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 15l11 -11m5 5l-11 11m-4 -8l7 7m-3.5 -10.5l7 7m-3.5 -10.5l7 7" /> </svg>"##;
const TRACTOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 15a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M7 15l0 .01" /> <path d="M17 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10.5 17l6.5 0" /> <path d="M20 15.2v-4.2a1 1 0 0 0 -1 -1h-6l-2 -5h-6v6.5" /> <path d="M18 5h-1a1 1 0 0 0 -1 1v4" /> </svg>"##;
const TRAFFIC_CONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20l16 0" /> <path d="M9.4 10l5.2 0" /> <path d="M7.8 15l8.4 0" /> <path d="M6 20l5 -15h2l5 15" /> </svg>"##;
const TRAFFIC_CONE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h16" /> <path d="M9.4 10h.6m4 0h.6" /> <path d="M7.8 15h7.2" /> <path d="M6 20l3.5 -10.5" /> <path d="M10.5 6.5l.5 -1.5h2l2 6m2 6l1 3" /> <path d="M3 3l18 18" /> </svg>"##;
const TRAFFIC_LIGHTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7a5 5 0 0 1 5 -5a5 5 0 0 1 5 5v10a5 5 0 0 1 -5 5a5 5 0 0 1 -5 -5l0 -10" /> <path d="M11 7a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M11 17a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> </svg>"##;
const TRAFFIC_LIGHTS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4c.912 -1.219 2.36 -2 4 -2a5 5 0 0 1 5 5v6m0 4a5 5 0 0 1 -10 0v-10" /> <path d="M12 8a1 1 0 1 0 -1 -1" /> <path d="M11.291 11.295a1 1 0 0 0 1.418 1.41" /> <path d="M11 17a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3 3l18 18" /> </svg>"##;
const TRAIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 13c0 -3.87 -3.37 -7 -10 -7h-8" /> <path d="M3 15h16a2 2 0 0 0 2 -2" /> <path d="M3 6v5h17.5" /> <path d="M3 11v4" /> <path d="M8 11v-5" /> <path d="M13 11v-4.5" /> <path d="M3 19h18" /> </svg>"##;
const TREE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 13l-2 -2" /> <path d="M12 12l2 -2" /> <path d="M12 21v-13" /> <path d="M9.824 16a3 3 0 0 1 -2.743 -3.69a3 3 0 0 1 .304 -4.833a3 3 0 0 1 4.615 -3.707a3 3 0 0 1 4.614 3.707a3 3 0 0 1 .305 4.833a3 3 0 0 1 -2.919 3.695h-4l-.176 -.005" /> </svg>"##;
const TREES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 5l3 3l-2 1l4 4l-3 1l4 4h-9" /> <path d="M15 21l0 -3" /> <path d="M8 13l-2 -2" /> <path d="M8 12l2 -2" /> <path d="M8 21v-13" /> <path d="M5.824 16a3 3 0 0 1 -2.743 -3.69a3 3 0 0 1 .304 -4.833a3 3 0 0 1 4.615 -3.707a3 3 0 0 1 4.614 3.707a3 3 0 0 1 .305 4.833a3 3 0 0 1 -2.919 3.695h-4l-.176 -.005" /> </svg>"##;
const TROLLEY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6 16l3 2" /> <path d="M12 17l8 -12" /> <path d="M17 10l2 1" /> <path d="M9.592 4.695l3.306 2.104a1.3 1.3 0 0 1 .396 1.8l-3.094 4.811a1.3 1.3 0 0 1 -1.792 .394l-3.306 -2.104a1.3 1.3 0 0 1 -.396 -1.8l3.094 -4.81a1.3 1.3 0 0 1 1.792 -.394" /> </svg>"##;
const TRUCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 17h-2v-11a1 1 0 0 1 1 -1h9v12m-4 0h6m4 0h2v-6h-8m0 -5h5l3 5" /> </svg>"##;
const TRUCK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15.585 15.586a2 2 0 0 0 2.826 2.831" /> <path d="M5 17h-2v-11a1 1 0 0 1 1 -1h1m3.96 0h4.04v4m0 4v4m-4 0h6m6 0v-6h-6m-2 -5h5l3 5" /> <path d="M3 3l18 18" /> </svg>"##;
const UNICYCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 16a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M12 16v-11" /> <path d="M8 3q 2 2 7 2" /> </svg>"##;
const UV_INDEX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12h1m16 0h1m-15.4 -6.4l.7 .7m12.1 -.7l-.7 .7m-9.7 5.7a4 4 0 1 1 8 0" /> <path d="M12 4v-1" /> <path d="M13 16l2 5h1l2 -5" /> <path d="M6 16v3a2 2 0 1 0 4 0v-3" /> </svg>"##;
const VIEWFINDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 3l0 4" /> <path d="M12 21l0 -3" /> <path d="M3 12l4 0" /> <path d="M21 12l-3 0" /> <path d="M12 12l0 .01" /> </svg>"##;
const VIEWFINDER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.65 5.623a9 9 0 1 0 12.71 12.745m1.684 -2.328a9 9 0 0 0 -12.094 -12.08" /> <path d="M12 3v4" /> <path d="M12 21v-3" /> <path d="M3 12h4" /> <path d="M21 12h-3" /> <path d="M12 12v.01" /> <path d="M3 3l18 18" /> </svg>"##;
const VOLCANO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 8v-1a2 2 0 1 0 -4 0" /> <path d="M15 8v-1a2 2 0 1 1 4 0" /> <path d="M4 20l3.472 -7.812a2 2 0 0 1 1.828 -1.188h5.4a2 2 0 0 1 1.828 1.188l3.472 7.812" /> <path d="M6.192 15.064a2.14 2.14 0 0 1 .475 -.064c.527 -.009 1.026 .178 1.333 .5c.307 .32 .806 .507 1.333 .5c.527 .007 1.026 -.18 1.334 -.5c.307 -.322 .806 -.509 1.333 -.5c.527 -.009 1.026 .178 1.333 .5c.308 .32 .807 .507 1.334 .5c.527 .007 1.026 -.18 1.333 -.5c.307 -.322 .806 -.509 1.333 -.5c.161 .003 .32 .025 .472 .064" /> <path d="M12 8v-4" /> </svg>"##;
const WHEELCHAIR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 16a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M17 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19 17a3 3 0 0 0 -3 -3h-3.4" /> <path d="M3 3h1a2 2 0 0 1 2 2v6" /> <path d="M6 8h11" /> <path d="M15 8v6" /> </svg>"##;
const WHEELCHAIR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 16a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M17.582 17.59a2 2 0 0 0 2.833 2.824" /> <path d="M14 14h-1.4" /> <path d="M6 6v5" /> <path d="M6 8h2m4 0h5" /> <path d="M15 8v3" /> <path d="M3 3l18 18" /> </svg>"##;
const WHIRL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M12 21c-3.314 0 -6 -2.462 -6 -5.5s2.686 -5.5 6 -5.5" /> <path d="M21 12c0 3.314 -2.462 6 -5.5 6s-5.5 -2.686 -5.5 -6" /> <path d="M12 14c3.314 0 6 -2.462 6 -5.5s-2.686 -5.5 -6 -5.5" /> <path d="M14 12c0 -3.314 -2.462 -6 -5.5 -6s-5.5 2.686 -5.5 6" /> </svg>"##;
const WIND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 8h8.5a2.5 2.5 0 1 0 -2.34 -3.24" /> <path d="M3 12h15.5a2.5 2.5 0 1 1 -2.34 3.24" /> <path d="M4 16h5.5a2.5 2.5 0 1 1 -2.34 3.24" /> </svg>"##;
const WIND_ELECTRICITY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 7l-3 5h4l-3 5" /> <path d="M3 16h4a2 2 0 1 1 0 4" /> <path d="M3 12h8a2 2 0 1 0 0 -4" /> <path d="M3 8h3a2 2 0 1 0 0 -4" /> </svg>"##;
const WIND_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 8h3m4 0h1.5a2.5 2.5 0 1 0 -2.34 -3.24" /> <path d="M3 12h9" /> <path d="M16 12h2.5a2.5 2.5 0 0 1 1.801 4.282" /> <path d="M4 16h5.5a2.5 2.5 0 1 1 -2.34 3.24" /> <path d="M3 3l18 18" /> </svg>"##;
const WINDMILL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12c2.76 0 5 -2.01 5 -4.5s-2.24 -4.5 -5 -4.5v9" /> <path d="M12 12c0 2.76 2.01 5 4.5 5s4.5 -2.24 4.5 -5h-9" /> <path d="M12 12c-2.76 0 -5 2.01 -5 4.5s2.24 4.5 5 4.5v-9" /> <path d="M12 12c0 -2.76 -2.01 -5 -4.5 -5s-4.5 2.24 -4.5 5h9" /> </svg>"##;
const WINDMILL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.061 11.06c1.18 -.824 1.939 -2.11 1.939 -3.56c0 -2.49 -2.24 -4.5 -5 -4.5v5" /> <path d="M12 12c0 2.76 2.01 5 4.5 5c.166 0 .33 -.01 .49 -.03m2.624 -1.36c.856 -.91 1.386 -2.19 1.386 -3.61h-5" /> <path d="M12 12c-2.76 0 -5 2.01 -5 4.5s2.24 4.5 5 4.5v-9" /> <path d="M6.981 7.033c-2.244 .285 -3.981 2.402 -3.981 4.967h9" /> <path d="M3 3l18 18" /> </svg>"##;
const WINDSOCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 3v18" /> <path d="M6 11l12 -1v-4l-12 -1" /> <path d="M10 5.5v5" /> <path d="M14 6v4" /> <path d="M4 21h4" /> </svg>"##;
const WIPER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 18a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3 9l5.5 5.5a5 5 0 0 1 7 0l5.5 -5.5a12 12 0 0 0 -18 0" /> <path d="M12 18l-2.2 -12.8" /> </svg>"##;
const WIPER_WASH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 20a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3 11l5.5 5.5a5 5 0 0 1 7 0l5.5 -5.5a12 12 0 0 0 -18 0" /> <path d="M12 20l0 -14" /> <path d="M4 6a4 4 0 0 1 .4 -1.8" /> <path d="M7 2.1a4 4 0 0 1 2 0" /> <path d="M12 6a4 4 0 0 0 -.4 -1.8" /> <path d="M12 6a4 4 0 0 1 .4 -1.8" /> <path d="M15 2.1a4 4 0 0 1 2 0" /> <path d="M20 6a4 4 0 0 0 -.4 -1.8" /> </svg>"##;
const WORLD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h16.8" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a17 17 0 0 1 0 18" /> </svg>"##;
const WORLD_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.985 12.52a9 9 0 1 0 -7.52 8.36" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h10.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3c2.313 3.706 3.07 7.856 2.27 12" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const WORLD_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -8.985 9" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h9.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.991 16.991 0 0 1 2.53 10.275" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const WORLD_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.946 12.99a9 9 0 1 0 -9.46 7.995" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h13.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.997 16.997 0 0 1 2.311 12.001" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const WORLD_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.942 13.02a9 9 0 1 0 -9.47 7.964" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h9.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3c2 3.206 2.837 6.913 2.508 10.537" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const WORLD_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -8.979 9" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h8.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.992 16.992 0 0 1 2.522 10.376" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const WORLD_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.876 10.51a9 9 0 1 0 -7.839 10.43" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h9.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.986 16.986 0 0 1 2.578 9.02" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const WORLD_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.986 12.509a9 9 0 1 0 -8.455 8.476" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h10.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3c2.313 3.706 3.07 7.857 2.27 12" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const WORLD_DOWNLOAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -9 9" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h8.4" /> <path d="M11.578 3a17 17 0 0 0 0 18" /> <path d="M12.5 3c1.719 2.755 2.5 5.876 2.5 9" /> <path d="M18 14v7m-3 -3l3 3l3 -3" /> </svg>"##;
const WORLD_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.986 12.51a9 9 0 1 0 -5.71 7.873" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h10.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a17 17 0 0 1 0 18" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const WORLD_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -9.679 8.974" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h6.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.983 16.983 0 0 1 2.556 8.136" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const WORLD_LATITUDE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M4.6 7l14.8 0" /> <path d="M3 12l18 0" /> <path d="M4.6 17l14.8 0" /> </svg>"##;
const WORLD_LONGITUDE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M11.5 3a11.2 11.2 0 0 0 0 18" /> <path d="M12.5 3a11.2 11.2 0 0 1 0 18" /> <path d="M12 3l0 18" /> </svg>"##;
const WORLD_MAP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 8h-2a2 2 0 0 0 -2 2a2 2 0 1 1 -4 0v-1a2 2 0 0 0 -2 -2h-1a2 2 0 0 1 -2 -2v-.5" /> <path d="M3 12h3a2 2 0 0 1 2 2v.5a1.5 1.5 0 0 0 1.5 1.5a1.5 1.5 0 0 1 1.5 1.5v3.25" /> <path d="M15 20.5v-3.5a2 2 0 0 1 2 -2h3.5" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const WORLD_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.483 15.006a9 9 0 1 0 -7.958 5.978" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h16.8" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.94 16.94 0 0 1 2.307 12" /> <path d="M16 19h6" /> </svg>"##;
const WORLD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.657 5.615a9 9 0 1 0 12.717 12.739m1.672 -2.322a9 9 0 0 0 -12.066 -12.084" /> <path d="M3.6 9h5.4m4 0h7.4" /> <path d="M3.6 15h11.4m4 0h1.4" /> <path d="M11.5 3a17.001 17.001 0 0 0 -1.493 3.022m-.847 3.145c-.68 4.027 .1 8.244 2.34 11.833" /> <path d="M12.5 3a16.982 16.982 0 0 1 2.549 8.005m-.207 3.818a16.979 16.979 0 0 1 -2.342 6.177" /> <path d="M3 3l18 18" /> </svg>"##;
const WORLD_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.945 12.997a9 9 0 1 0 -7.928 7.945" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h9.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.992 16.992 0 0 1 2.51 10.526" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const WORLD_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.972 11.291a9 9 0 1 0 -8.322 9.686" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h8.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.986 16.986 0 0 1 2.578 9.018" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const WORLD_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.985 12.518a9 9 0 1 0 -8.45 8.466" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h11.4" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.998 16.998 0 0 1 2.283 12.157" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const WORLD_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.975 11.33a9 9 0 1 0 -5.673 9.043" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h9.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.988 16.988 0 0 1 2.57 9.518m-1.056 5.403a17 17 0 0 1 -1.514 3.079" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const WORLD_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -9 9" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h7.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.984 16.984 0 0 1 2.574 8.62" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const WORLD_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.94 13.045a9 9 0 1 0 -8.953 7.955" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h9.4" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.991 16.991 0 0 1 2.529 10.294" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const WORLD_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -9.968 8.948" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h6.4" /> <path d="M11.5 3a17.001 17.001 0 0 0 -1.886 13.802" /> <path d="M12.5 3a16.982 16.982 0 0 1 2.549 8.01" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const WORLD_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.985 12.52a9 9 0 1 0 -8.451 8.463" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h10.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.996 16.996 0 0 1 2.391 11.512" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const WORLD_UPLOAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12a9 9 0 1 0 -9 9" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h8.4" /> <path d="M11.578 3a17 17 0 0 0 0 18" /> <path d="M12.5 3c1.719 2.755 2.5 5.876 2.5 9" /> <path d="M18 21v-7m3 3l-3 -3l-3 3" /> </svg>"##;
const WORLD_WWW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19.5 7a9 9 0 0 0 -7.5 -4a8.991 8.991 0 0 0 -7.484 4" /> <path d="M11.5 3a16.989 16.989 0 0 0 -1.826 4" /> <path d="M12.5 3a16.989 16.989 0 0 1 1.828 4" /> <path d="M19.5 17a9 9 0 0 1 -7.5 4a8.991 8.991 0 0 1 -7.484 -4" /> <path d="M11.5 21a16.989 16.989 0 0 1 -1.826 -4" /> <path d="M12.5 21a16.989 16.989 0 0 0 1.828 -4" /> <path d="M2 10l1 4l1.5 -4l1.5 4l1 -4" /> <path d="M17 10l1 4l1.5 -4l1.5 4l1 -4" /> <path d="M9.5 10l1 4l1.5 -4l1.5 4l1 -4" /> </svg>"##;
const WORLD_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.929 13.131a9 9 0 1 0 -8.931 7.869" /> <path d="M3.6 9h16.8" /> <path d="M3.6 15h9.9" /> <path d="M11.5 3a17 17 0 0 0 0 18" /> <path d="M12.5 3a16.992 16.992 0 0 1 2.505 10.573" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const WRECKING_BALL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 13a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M2 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M11 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M13 19l-9 0" /> <path d="M4 15l9 0" /> <path d="M8 12v-5h2a3 3 0 0 1 3 3v5" /> <path d="M5 15v-2a1 1 0 0 1 1 -1h7" /> <path d="M19 11v-7l-6 7" /> </svg>"##;
const ZEPPELIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.5 4c4.694 0 8.5 2.686 8.5 6s-3.806 6 -8.5 6c-2.13 0 -4.584 -.926 -7.364 -2.777l-2.136 1.777v-3.33a46.07 46.07 0 0 1 -2 -1.67a46.07 46.07 0 0 1 2 -1.67v-3.33l2.135 1.778c2.78 -1.852 5.235 -2.778 7.365 -2.778" /> <path d="M10 15.5v4.5h6v-4" /> </svg>"##;
const ZEPPELIN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.773 15.783c-.723 .141 -1.486 .217 -2.273 .217c-2.13 0 -4.584 -.926 -7.364 -2.777l-2.136 1.777v-3.33a46.07 46.07 0 0 1 -2 -1.67a46.07 46.07 0 0 1 2 -1.67v-3.33l2.135 1.778c.13 -.087 .261 -.172 .39 -.256m2.564 -1.42c1.601 -.735 3.071 -1.102 4.411 -1.102c4.694 0 8.5 2.686 8.5 6c0 1.919 -1.276 3.627 -3.261 4.725" /> <path d="M10 15.5v4.5h6v-4" /> <path d="M3 3l18 18" /> </svg>"##;
const ZODIAC_AQUARIUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10l3 -3l3 3l3 -3l3 3l3 -3l3 3" /> <path d="M3 17l3 -3l3 3l3 -3l3 3l3 -3l3 3" /> </svg>"##;
const ZODIAC_ARIES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 5a5 5 0 1 0 -4 8" /> <path d="M16 13a5 5 0 1 0 -4 -8" /> <path d="M12 21l0 -16" /> </svg>"##;
const ZODIAC_CANCER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M15 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M3 12a10 6.5 0 0 1 14 -6.5" /> <path d="M21 12a10 6.5 0 0 1 -14 6.5" /> </svg>"##;
const ZODIAC_CAPRICORN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 4a3 3 0 0 1 3 3v9" /> <path d="M7 7a3 3 0 0 1 6 0v11a3 3 0 0 1 -3 3" /> <path d="M13 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const ZODIAC_GEMINI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3a21 21 0 0 0 18 0" /> <path d="M3 21a21 21 0 0 1 18 0" /> <path d="M7 4.5l0 15" /> <path d="M17 4.5l0 15" /> </svg>"##;
const ZODIAC_LEO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 17a4 4 0 1 0 8 0" /> <path d="M3 16a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M7 7a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M7 7c0 3 2 5 2 9" /> <path d="M15 7c0 4 -2 6 -2 10" /> </svg>"##;
const ZODIAC_LIBRA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 20l14 0" /> <path d="M5 17h5v-.3a7 7 0 1 1 4 0v.3h5" /> </svg>"##;
const ZODIAC_PISCES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 3a21 21 0 0 1 0 18" /> <path d="M19 3a21 21 0 0 0 0 18" /> <path d="M5 12l14 0" /> </svg>"##;
const ZODIAC_SAGITTARIUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20l16 -16" /> <path d="M13 4h7v7" /> <path d="M6.5 12.5l5 5" /> </svg>"##;
const ZODIAC_SCORPIO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a2 2 0 0 1 2 2v9" /> <path d="M5 6a2 2 0 0 1 4 0v9" /> <path d="M9 6a2 2 0 0 1 4 0v10a3 3 0 0 0 3 3h5l-3 -3m0 6l3 -3" /> </svg>"##;
const ZODIAC_TAURUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 3a6 6 0 0 0 12 0" /> <path d="M6 15a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" /> </svg>"##;
const ZODIAC_VIRGO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 4a2 2 0 0 1 2 2v9" /> <path d="M5 6a2 2 0 0 1 4 0v9" /> <path d="M9 6a2 2 0 0 1 4 0v10a7 5 0 0 0 7 5" /> <path d="M12 21a7 5 0 0 0 7 -5v-2a3 3 0 0 0 -6 0" /> </svg>"##;
const ZOOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M21 21l-6 -6" /> </svg>"##;
const ZOOM_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M8 8l4 4" /> <path d="M12 8l-4 4" /> <path d="M21 21l-6 -6" /> </svg>"##;
const ZOOM_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M21 21l-6 -6" /> <path d="M7 10l2 2l4 -4" /> </svg>"##;
const ZOOM_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M21 21l-6 -6" /> <path d="M8 8l-2 2l2 2" /> <path d="M12 8l2 2l-2 2" /> </svg>"##;
const ZOOM_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M21 21l-6 -6" /> <path d="M10 13v.01" /> <path d="M10 7v3" /> </svg>"##;
const ZOOM_IN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M7 10l6 0" /> <path d="M10 7l0 6" /> <path d="M21 21l-6 -6" /> </svg>"##;
const ZOOM_IN_AREA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 13v4" /> <path d="M13 15h4" /> <path d="M10 15a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M22 22l-3 -3" /> <path d="M6 18h-1a2 2 0 0 1 -2 -2v-1" /> <path d="M3 11v-1" /> <path d="M3 6v-1a2 2 0 0 1 2 -2h1" /> <path d="M10 3h1" /> <path d="M15 3h1a2 2 0 0 1 2 2v1" /> </svg>"##;
const ZOOM_MONEY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M21 21l-6 -6" /> <path d="M12 7h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M10 13v1m0 -8v1" /> </svg>"##;
const ZOOM_OUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M7 10l6 0" /> <path d="M21 21l-6 -6" /> </svg>"##;
const ZOOM_OUT_AREA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 15h4" /> <path d="M10 15a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /> <path d="M22 22l-3 -3" /> <path d="M6 18h-1a2 2 0 0 1 -2 -2v-1" /> <path d="M3 11v-1" /> <path d="M3 6v-1a2 2 0 0 1 2 -2h1" /> <path d="M10 3h1" /> <path d="M15 3h1a2 2 0 0 1 2 2v1" /> </svg>"##;
const ZOOM_PAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M17 17l-2.5 -2.5" /> <path d="M10 4l2 -2l2 2" /> <path d="M20 10l2 2l-2 2" /> <path d="M4 10l-2 2l2 2" /> <path d="M10 20l2 2l2 -2" /> </svg>"##;
const ZOOM_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M21 21l-6 -6" /> <path d="M10 13l0 .01" /> <path d="M10 10a1.5 1.5 0 1 0 -1.14 -2.474" /> </svg>"##;
const ZOOM_REPLACE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 21l-6 -6" /> <path d="M3.291 8a7 7 0 0 1 5.077 -4.806a7.021 7.021 0 0 1 8.242 4.403" /> <path d="M17 4v4h-4" /> <path d="M16.705 12a7 7 0 0 1 -5.074 4.798a7.021 7.021 0 0 1 -8.241 -4.403" /> <path d="M3 16v-4h4" /> </svg>"##;
const ZOOM_RESET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 21l-6 -6" /> <path d="M3.268 12.043a7.017 7.017 0 0 0 6.634 4.957a7.012 7.012 0 0 0 7.043 -6.131a7 7 0 0 0 -5.314 -7.672a7.021 7.021 0 0 0 -8.241 4.403" /> <path d="M3 4v4h4" /> </svg>"##;

/// Map icon variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MapIcon {
    Acorn,
    AerialLift,
    AirBalloon,
    AirTrafficControl,
    Alien,
    Ambulance,
    Anchor,
    AnchorOff,
    Atom,
    Atom2,
    AtomOff,
    AutomaticGearbox,
    Backhoe,
    Balloon,
    BalloonOff,
    Bat,
    BatteryAutomotive,
    Beach,
    BeachOff,
    Bed,
    BedOff,
    Bike,
    BikeOff,
    Bulldozer,
    Bus,
    BusOff,
    BusStop,
    Butterfly,
    Cactus,
    CactusOff,
    Camper,
    Campfire,
    Cannabis,
    Car,
    Car4wd,
    CarCrane,
    CarCrash,
    CarDoor,
    CarFan,
    CarFan1,
    CarFan2,
    CarFan3,
    CarFanAuto,
    CarLifter,
    CarOff,
    CarOffRoad,
    CarSuspension,
    CarSuv,
    CarTurbine,
    Caravan,
    Cat,
    ChargingPile,
    Cherry,
    ChristmasTree,
    ChristmasTreeOff,
    Cloud,
    CloudBolt,
    CloudCancel,
    CloudCheck,
    CloudCode,
    CloudCog,
    CloudDollar,
    CloudDown,
    CloudExclamation,
    CloudFog,
    CloudHeart,
    CloudMinus,
    CloudOff,
    CloudPause,
    CloudPin,
    CloudPlus,
    CloudQuestion,
    CloudRain,
    CloudSearch,
    CloudShare,
    CloudSnow,
    CloudStar,
    CloudStorm,
    CloudUp,
    CloudX,
    Clover,
    Clover2,
    Comet,
    Compass,
    CompassOff,
    Crane,
    CraneOff,
    CrystalBall,
    CurrentLocation,
    CurrentLocationOff,
    Deer,
    Directions,
    DirectionsOff,
    Dog,
    Drone,
    DroneOff,
    DropCircle,
    Droplets,
    Engine,
    EngineOff,
    Escalator,
    EscalatorDown,
    EscalatorUp,
    Feather,
    FeatherOff,
    Ferry,
    FireHydrant,
    FireHydrantOff,
    Firetruck,
    Fish,
    FishBone,
    FishHook,
    FishHookOff,
    FishOff,
    Flag,
    Flag2,
    Flag2Off,
    Flag3,
    FlagBolt,
    FlagCancel,
    FlagCheck,
    FlagCode,
    FlagCog,
    FlagDollar,
    FlagDown,
    FlagExclamation,
    FlagHeart,
    FlagMinus,
    FlagOff,
    FlagPause,
    FlagPin,
    FlagPlus,
    FlagQuestion,
    FlagSearch,
    FlagShare,
    FlagSpark,
    FlagStar,
    FlagUp,
    FlagX,
    Flame,
    FlameOff,
    Flare,
    Flood,
    Flower,
    FlowerOff,
    Forklift,
    Fountain,
    FountainOff,
    GardenCart,
    GardenCartOff,
    GasStation,
    GasStationOff,
    Geometry,
    Globe,
    GlobeOff,
    Gps,
    Grave,
    Grave2,
    Growth,
    Haze,
    Helicopter,
    HelicopterLanding,
    Horse,
    Horseshoe,
    Iceberg,
    Jetski,
    Leaf,
    Leaf2,
    LeafMaple,
    LeafOff,
    LiveView,
    Location,
    LocationBolt,
    LocationBroken,
    LocationCancel,
    LocationCheck,
    LocationCode,
    LocationCog,
    LocationDollar,
    LocationDown,
    LocationExclamation,
    LocationHeart,
    LocationMinus,
    LocationOff,
    LocationPause,
    LocationPin,
    LocationPlus,
    LocationQuestion,
    LocationSearch,
    LocationShare,
    LocationStar,
    LocationUp,
    LocationX,
    Map,
    Map2,
    MapBolt,
    MapCancel,
    MapCheck,
    MapCode,
    MapCog,
    MapDiscount,
    MapDollar,
    MapDown,
    MapEast,
    MapExclamation,
    MapHeart,
    MapLock,
    MapMinus,
    MapNorth,
    MapOff,
    MapPause,
    MapPin,
    MapPin2,
    MapPinBolt,
    MapPinCancel,
    MapPinCheck,
    MapPinCode,
    MapPinCog,
    MapPinDollar,
    MapPinDown,
    MapPinExclamation,
    MapPinHeart,
    MapPinMinus,
    MapPinOff,
    MapPinPause,
    MapPinPin,
    MapPinPlus,
    MapPinQuestion,
    MapPinSearch,
    MapPinShare,
    MapPinStar,
    MapPinUp,
    MapPinX,
    MapPins,
    MapPlus,
    MapQuestion,
    MapRoute,
    MapSearch,
    MapShare,
    MapShield,
    MapSouth,
    MapStar,
    MapUp,
    MapWest,
    MapX,
    MasksTheater,
    MasksTheaterOff,
    MedicalCross,
    MedicalCrossCircle,
    MedicalCrossOff,
    Meteor,
    MeteorOff,
    Mist,
    MistOff,
    Monkeybar,
    Moon,
    Moon2,
    MoonOff,
    MoonStars,
    Moped,
    Motorbike,
    Mountain,
    MountainOff,
    Navigation,
    NavigationBolt,
    NavigationCancel,
    NavigationCheck,
    NavigationCode,
    NavigationCog,
    NavigationDiscount,
    NavigationDollar,
    NavigationDown,
    NavigationEast,
    NavigationExclamation,
    NavigationHeart,
    NavigationMinus,
    NavigationNorth,
    NavigationOff,
    NavigationPause,
    NavigationPin,
    NavigationPlus,
    NavigationQuestion,
    NavigationSearch,
    NavigationShare,
    NavigationSouth,
    NavigationStar,
    NavigationTop,
    NavigationUp,
    NavigationWest,
    NavigationX,
    NorthStar,
    Parachute,
    ParachuteOff,
    Parking,
    ParkingMeter,
    ParkingOff,
    Paw,
    PawOff,
    Pennant,
    Pennant2,
    PennantOff,
    PicnicTable,
    Pig,
    PigMoney,
    PigOff,
    Pin,
    Pinned,
    PinnedOff,
    Plane,
    PlaneArrival,
    PlaneDeparture,
    PlaneInflight,
    PlaneOff,
    PlaneTilt,
    Planet,
    PlanetOff,
    Plant,
    Plant2,
    Plant2Off,
    PlantOff,
    Pokeball,
    PokeballOff,
    Prison,
    Radar,
    Radar2,
    RadarOff,
    Rainbow,
    RainbowOff,
    Ripple,
    RippleOff,
    Road,
    RoadOff,
    RoadSign,
    Rocket,
    RocketOff,
    Rollercoaster,
    RollercoasterOff,
    Route,
    Route2,
    RouteAltLeft,
    RouteAltRight,
    RouteOff,
    RouteSquare,
    RouteSquare2,
    RouteX,
    RouteX2,
    Sailboat,
    Sailboat2,
    SailboatOff,
    Satellite,
    SatelliteOff,
    School,
    SchoolOff,
    Scooter,
    ScooterElectric,
    Seedling,
    SeedlingOff,
    Segway,
    Ship,
    ShipOff,
    SignLeft,
    SignRight,
    Skateboard,
    SkateboardOff,
    Sleigh,
    Snowflake,
    SnowflakeOff,
    Snowman,
    Speedboat,
    Spider,
    Stairs,
    StairsDown,
    StairsUp,
    SteeringWheel,
    SteeringWheelOff,
    Storm,
    StormOff,
    Submarine,
    Sun,
    SunElectricity,
    SunHigh,
    SunLow,
    SunMoon,
    SunOff,
    SunWind,
    Sunrise,
    Sunset,
    Sunset2,
    Tank,
    Target,
    TargetOff,
    Temperature,
    TemperatureCelsius,
    TemperatureFahrenheit,
    TemperatureMinus,
    TemperatureOff,
    TemperaturePlus,
    Tent,
    TentOff,
    Theater,
    Tir,
    ToolsKitchen2Off,
    ToolsKitchenOff,
    Tornado,
    Track,
    Tractor,
    TrafficCone,
    TrafficConeOff,
    TrafficLights,
    TrafficLightsOff,
    Train,
    Tree,
    Trees,
    Trolley,
    Truck,
    TruckOff,
    Unicycle,
    UvIndex,
    Viewfinder,
    ViewfinderOff,
    Volcano,
    Wheelchair,
    WheelchairOff,
    Whirl,
    Wind,
    WindElectricity,
    WindOff,
    Windmill,
    WindmillOff,
    Windsock,
    Wiper,
    WiperWash,
    World,
    WorldBolt,
    WorldCancel,
    WorldCheck,
    WorldCode,
    WorldCog,
    WorldDollar,
    WorldDown,
    WorldDownload,
    WorldExclamation,
    WorldHeart,
    WorldLatitude,
    WorldLongitude,
    WorldMap,
    WorldMinus,
    WorldOff,
    WorldPause,
    WorldPin,
    WorldPlus,
    WorldQuestion,
    WorldSearch,
    WorldShare,
    WorldStar,
    WorldUp,
    WorldUpload,
    WorldWww,
    WorldX,
    WreckingBall,
    Zeppelin,
    ZeppelinOff,
    ZodiacAquarius,
    ZodiacAries,
    ZodiacCancer,
    ZodiacCapricorn,
    ZodiacGemini,
    ZodiacLeo,
    ZodiacLibra,
    ZodiacPisces,
    ZodiacSagittarius,
    ZodiacScorpio,
    ZodiacTaurus,
    ZodiacVirgo,
    Zoom,
    ZoomCancel,
    ZoomCheck,
    ZoomCode,
    ZoomExclamation,
    ZoomIn,
    ZoomInArea,
    ZoomMoney,
    ZoomOut,
    ZoomOutArea,
    ZoomPan,
    ZoomQuestion,
    ZoomReplace,
    ZoomReset,
}

impl MapIcon {
    /// Returns all available icons in this category.
    pub fn all() -> &'static [Self] {
        &[Self::Acorn, Self::AerialLift, Self::AirBalloon, Self::AirTrafficControl, Self::Alien, Self::Ambulance, Self::Anchor, Self::AnchorOff, Self::Atom, Self::Atom2, Self::AtomOff, Self::AutomaticGearbox, Self::Backhoe, Self::Balloon, Self::BalloonOff, Self::Bat, Self::BatteryAutomotive, Self::Beach, Self::BeachOff, Self::Bed, Self::BedOff, Self::Bike, Self::BikeOff, Self::Bulldozer, Self::Bus, Self::BusOff, Self::BusStop, Self::Butterfly, Self::Cactus, Self::CactusOff, Self::Camper, Self::Campfire, Self::Cannabis, Self::Car, Self::Car4wd, Self::CarCrane, Self::CarCrash, Self::CarDoor, Self::CarFan, Self::CarFan1, Self::CarFan2, Self::CarFan3, Self::CarFanAuto, Self::CarLifter, Self::CarOff, Self::CarOffRoad, Self::CarSuspension, Self::CarSuv, Self::CarTurbine, Self::Caravan, Self::Cat, Self::ChargingPile, Self::Cherry, Self::ChristmasTree, Self::ChristmasTreeOff, Self::Cloud, Self::CloudBolt, Self::CloudCancel, Self::CloudCheck, Self::CloudCode, Self::CloudCog, Self::CloudDollar, Self::CloudDown, Self::CloudExclamation, Self::CloudFog, Self::CloudHeart, Self::CloudMinus, Self::CloudOff, Self::CloudPause, Self::CloudPin, Self::CloudPlus, Self::CloudQuestion, Self::CloudRain, Self::CloudSearch, Self::CloudShare, Self::CloudSnow, Self::CloudStar, Self::CloudStorm, Self::CloudUp, Self::CloudX, Self::Clover, Self::Clover2, Self::Comet, Self::Compass, Self::CompassOff, Self::Crane, Self::CraneOff, Self::CrystalBall, Self::CurrentLocation, Self::CurrentLocationOff, Self::Deer, Self::Directions, Self::DirectionsOff, Self::Dog, Self::Drone, Self::DroneOff, Self::DropCircle, Self::Droplets, Self::Engine, Self::EngineOff, Self::Escalator, Self::EscalatorDown, Self::EscalatorUp, Self::Feather, Self::FeatherOff, Self::Ferry, Self::FireHydrant, Self::FireHydrantOff, Self::Firetruck, Self::Fish, Self::FishBone, Self::FishHook, Self::FishHookOff, Self::FishOff, Self::Flag, Self::Flag2, Self::Flag2Off, Self::Flag3, Self::FlagBolt, Self::FlagCancel, Self::FlagCheck, Self::FlagCode, Self::FlagCog, Self::FlagDollar, Self::FlagDown, Self::FlagExclamation, Self::FlagHeart, Self::FlagMinus, Self::FlagOff, Self::FlagPause, Self::FlagPin, Self::FlagPlus, Self::FlagQuestion, Self::FlagSearch, Self::FlagShare, Self::FlagSpark, Self::FlagStar, Self::FlagUp, Self::FlagX, Self::Flame, Self::FlameOff, Self::Flare, Self::Flood, Self::Flower, Self::FlowerOff, Self::Forklift, Self::Fountain, Self::FountainOff, Self::GardenCart, Self::GardenCartOff, Self::GasStation, Self::GasStationOff, Self::Geometry, Self::Globe, Self::GlobeOff, Self::Gps, Self::Grave, Self::Grave2, Self::Growth, Self::Haze, Self::Helicopter, Self::HelicopterLanding, Self::Horse, Self::Horseshoe, Self::Iceberg, Self::Jetski, Self::Leaf, Self::Leaf2, Self::LeafMaple, Self::LeafOff, Self::LiveView, Self::Location, Self::LocationBolt, Self::LocationBroken, Self::LocationCancel, Self::LocationCheck, Self::LocationCode, Self::LocationCog, Self::LocationDollar, Self::LocationDown, Self::LocationExclamation, Self::LocationHeart, Self::LocationMinus, Self::LocationOff, Self::LocationPause, Self::LocationPin, Self::LocationPlus, Self::LocationQuestion, Self::LocationSearch, Self::LocationShare, Self::LocationStar, Self::LocationUp, Self::LocationX, Self::Map, Self::Map2, Self::MapBolt, Self::MapCancel, Self::MapCheck, Self::MapCode, Self::MapCog, Self::MapDiscount, Self::MapDollar, Self::MapDown, Self::MapEast, Self::MapExclamation, Self::MapHeart, Self::MapLock, Self::MapMinus, Self::MapNorth, Self::MapOff, Self::MapPause, Self::MapPin, Self::MapPin2, Self::MapPinBolt, Self::MapPinCancel, Self::MapPinCheck, Self::MapPinCode, Self::MapPinCog, Self::MapPinDollar, Self::MapPinDown, Self::MapPinExclamation, Self::MapPinHeart, Self::MapPinMinus, Self::MapPinOff, Self::MapPinPause, Self::MapPinPin, Self::MapPinPlus, Self::MapPinQuestion, Self::MapPinSearch, Self::MapPinShare, Self::MapPinStar, Self::MapPinUp, Self::MapPinX, Self::MapPins, Self::MapPlus, Self::MapQuestion, Self::MapRoute, Self::MapSearch, Self::MapShare, Self::MapShield, Self::MapSouth, Self::MapStar, Self::MapUp, Self::MapWest, Self::MapX, Self::MasksTheater, Self::MasksTheaterOff, Self::MedicalCross, Self::MedicalCrossCircle, Self::MedicalCrossOff, Self::Meteor, Self::MeteorOff, Self::Mist, Self::MistOff, Self::Monkeybar, Self::Moon, Self::Moon2, Self::MoonOff, Self::MoonStars, Self::Moped, Self::Motorbike, Self::Mountain, Self::MountainOff, Self::Navigation, Self::NavigationBolt, Self::NavigationCancel, Self::NavigationCheck, Self::NavigationCode, Self::NavigationCog, Self::NavigationDiscount, Self::NavigationDollar, Self::NavigationDown, Self::NavigationEast, Self::NavigationExclamation, Self::NavigationHeart, Self::NavigationMinus, Self::NavigationNorth, Self::NavigationOff, Self::NavigationPause, Self::NavigationPin, Self::NavigationPlus, Self::NavigationQuestion, Self::NavigationSearch, Self::NavigationShare, Self::NavigationSouth, Self::NavigationStar, Self::NavigationTop, Self::NavigationUp, Self::NavigationWest, Self::NavigationX, Self::NorthStar, Self::Parachute, Self::ParachuteOff, Self::Parking, Self::ParkingMeter, Self::ParkingOff, Self::Paw, Self::PawOff, Self::Pennant, Self::Pennant2, Self::PennantOff, Self::PicnicTable, Self::Pig, Self::PigMoney, Self::PigOff, Self::Pin, Self::Pinned, Self::PinnedOff, Self::Plane, Self::PlaneArrival, Self::PlaneDeparture, Self::PlaneInflight, Self::PlaneOff, Self::PlaneTilt, Self::Planet, Self::PlanetOff, Self::Plant, Self::Plant2, Self::Plant2Off, Self::PlantOff, Self::Pokeball, Self::PokeballOff, Self::Prison, Self::Radar, Self::Radar2, Self::RadarOff, Self::Rainbow, Self::RainbowOff, Self::Ripple, Self::RippleOff, Self::Road, Self::RoadOff, Self::RoadSign, Self::Rocket, Self::RocketOff, Self::Rollercoaster, Self::RollercoasterOff, Self::Route, Self::Route2, Self::RouteAltLeft, Self::RouteAltRight, Self::RouteOff, Self::RouteSquare, Self::RouteSquare2, Self::RouteX, Self::RouteX2, Self::Sailboat, Self::Sailboat2, Self::SailboatOff, Self::Satellite, Self::SatelliteOff, Self::School, Self::SchoolOff, Self::Scooter, Self::ScooterElectric, Self::Seedling, Self::SeedlingOff, Self::Segway, Self::Ship, Self::ShipOff, Self::SignLeft, Self::SignRight, Self::Skateboard, Self::SkateboardOff, Self::Sleigh, Self::Snowflake, Self::SnowflakeOff, Self::Snowman, Self::Speedboat, Self::Spider, Self::Stairs, Self::StairsDown, Self::StairsUp, Self::SteeringWheel, Self::SteeringWheelOff, Self::Storm, Self::StormOff, Self::Submarine, Self::Sun, Self::SunElectricity, Self::SunHigh, Self::SunLow, Self::SunMoon, Self::SunOff, Self::SunWind, Self::Sunrise, Self::Sunset, Self::Sunset2, Self::Tank, Self::Target, Self::TargetOff, Self::Temperature, Self::TemperatureCelsius, Self::TemperatureFahrenheit, Self::TemperatureMinus, Self::TemperatureOff, Self::TemperaturePlus, Self::Tent, Self::TentOff, Self::Theater, Self::Tir, Self::ToolsKitchen2Off, Self::ToolsKitchenOff, Self::Tornado, Self::Track, Self::Tractor, Self::TrafficCone, Self::TrafficConeOff, Self::TrafficLights, Self::TrafficLightsOff, Self::Train, Self::Tree, Self::Trees, Self::Trolley, Self::Truck, Self::TruckOff, Self::Unicycle, Self::UvIndex, Self::Viewfinder, Self::ViewfinderOff, Self::Volcano, Self::Wheelchair, Self::WheelchairOff, Self::Whirl, Self::Wind, Self::WindElectricity, Self::WindOff, Self::Windmill, Self::WindmillOff, Self::Windsock, Self::Wiper, Self::WiperWash, Self::World, Self::WorldBolt, Self::WorldCancel, Self::WorldCheck, Self::WorldCode, Self::WorldCog, Self::WorldDollar, Self::WorldDown, Self::WorldDownload, Self::WorldExclamation, Self::WorldHeart, Self::WorldLatitude, Self::WorldLongitude, Self::WorldMap, Self::WorldMinus, Self::WorldOff, Self::WorldPause, Self::WorldPin, Self::WorldPlus, Self::WorldQuestion, Self::WorldSearch, Self::WorldShare, Self::WorldStar, Self::WorldUp, Self::WorldUpload, Self::WorldWww, Self::WorldX, Self::WreckingBall, Self::Zeppelin, Self::ZeppelinOff, Self::ZodiacAquarius, Self::ZodiacAries, Self::ZodiacCancer, Self::ZodiacCapricorn, Self::ZodiacGemini, Self::ZodiacLeo, Self::ZodiacLibra, Self::ZodiacPisces, Self::ZodiacSagittarius, Self::ZodiacScorpio, Self::ZodiacTaurus, Self::ZodiacVirgo, Self::Zoom, Self::ZoomCancel, Self::ZoomCheck, Self::ZoomCode, Self::ZoomExclamation, Self::ZoomIn, Self::ZoomInArea, Self::ZoomMoney, Self::ZoomOut, Self::ZoomOutArea, Self::ZoomPan, Self::ZoomQuestion, Self::ZoomReplace, Self::ZoomReset]
    }

    /// Returns the icon count.
    pub fn count() -> usize {
        488
    }

    /// Creates an icon from its kebab-case name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "acorn" => Some(Self::Acorn),
            "aerial-lift" => Some(Self::AerialLift),
            "air-balloon" => Some(Self::AirBalloon),
            "air-traffic-control" => Some(Self::AirTrafficControl),
            "alien" => Some(Self::Alien),
            "ambulance" => Some(Self::Ambulance),
            "anchor" => Some(Self::Anchor),
            "anchor-off" => Some(Self::AnchorOff),
            "atom" => Some(Self::Atom),
            "atom-2" => Some(Self::Atom2),
            "atom-off" => Some(Self::AtomOff),
            "automatic-gearbox" => Some(Self::AutomaticGearbox),
            "backhoe" => Some(Self::Backhoe),
            "balloon" => Some(Self::Balloon),
            "balloon-off" => Some(Self::BalloonOff),
            "bat" => Some(Self::Bat),
            "battery-automotive" => Some(Self::BatteryAutomotive),
            "beach" => Some(Self::Beach),
            "beach-off" => Some(Self::BeachOff),
            "bed" => Some(Self::Bed),
            "bed-off" => Some(Self::BedOff),
            "bike" => Some(Self::Bike),
            "bike-off" => Some(Self::BikeOff),
            "bulldozer" => Some(Self::Bulldozer),
            "bus" => Some(Self::Bus),
            "bus-off" => Some(Self::BusOff),
            "bus-stop" => Some(Self::BusStop),
            "butterfly" => Some(Self::Butterfly),
            "cactus" => Some(Self::Cactus),
            "cactus-off" => Some(Self::CactusOff),
            "camper" => Some(Self::Camper),
            "campfire" => Some(Self::Campfire),
            "cannabis" => Some(Self::Cannabis),
            "car" => Some(Self::Car),
            "car-4wd" => Some(Self::Car4wd),
            "car-crane" => Some(Self::CarCrane),
            "car-crash" => Some(Self::CarCrash),
            "car-door" => Some(Self::CarDoor),
            "car-fan" => Some(Self::CarFan),
            "car-fan-1" => Some(Self::CarFan1),
            "car-fan-2" => Some(Self::CarFan2),
            "car-fan-3" => Some(Self::CarFan3),
            "car-fan-auto" => Some(Self::CarFanAuto),
            "car-lifter" => Some(Self::CarLifter),
            "car-off" => Some(Self::CarOff),
            "car-off-road" => Some(Self::CarOffRoad),
            "car-suspension" => Some(Self::CarSuspension),
            "car-suv" => Some(Self::CarSuv),
            "car-turbine" => Some(Self::CarTurbine),
            "caravan" => Some(Self::Caravan),
            "cat" => Some(Self::Cat),
            "charging-pile" => Some(Self::ChargingPile),
            "cherry" => Some(Self::Cherry),
            "christmas-tree" => Some(Self::ChristmasTree),
            "christmas-tree-off" => Some(Self::ChristmasTreeOff),
            "cloud" => Some(Self::Cloud),
            "cloud-bolt" => Some(Self::CloudBolt),
            "cloud-cancel" => Some(Self::CloudCancel),
            "cloud-check" => Some(Self::CloudCheck),
            "cloud-code" => Some(Self::CloudCode),
            "cloud-cog" => Some(Self::CloudCog),
            "cloud-dollar" => Some(Self::CloudDollar),
            "cloud-down" => Some(Self::CloudDown),
            "cloud-exclamation" => Some(Self::CloudExclamation),
            "cloud-fog" => Some(Self::CloudFog),
            "cloud-heart" => Some(Self::CloudHeart),
            "cloud-minus" => Some(Self::CloudMinus),
            "cloud-off" => Some(Self::CloudOff),
            "cloud-pause" => Some(Self::CloudPause),
            "cloud-pin" => Some(Self::CloudPin),
            "cloud-plus" => Some(Self::CloudPlus),
            "cloud-question" => Some(Self::CloudQuestion),
            "cloud-rain" => Some(Self::CloudRain),
            "cloud-search" => Some(Self::CloudSearch),
            "cloud-share" => Some(Self::CloudShare),
            "cloud-snow" => Some(Self::CloudSnow),
            "cloud-star" => Some(Self::CloudStar),
            "cloud-storm" => Some(Self::CloudStorm),
            "cloud-up" => Some(Self::CloudUp),
            "cloud-x" => Some(Self::CloudX),
            "clover" => Some(Self::Clover),
            "clover-2" => Some(Self::Clover2),
            "comet" => Some(Self::Comet),
            "compass" => Some(Self::Compass),
            "compass-off" => Some(Self::CompassOff),
            "crane" => Some(Self::Crane),
            "crane-off" => Some(Self::CraneOff),
            "crystal-ball" => Some(Self::CrystalBall),
            "current-location" => Some(Self::CurrentLocation),
            "current-location-off" => Some(Self::CurrentLocationOff),
            "deer" => Some(Self::Deer),
            "directions" => Some(Self::Directions),
            "directions-off" => Some(Self::DirectionsOff),
            "dog" => Some(Self::Dog),
            "drone" => Some(Self::Drone),
            "drone-off" => Some(Self::DroneOff),
            "drop-circle" => Some(Self::DropCircle),
            "droplets" => Some(Self::Droplets),
            "engine" => Some(Self::Engine),
            "engine-off" => Some(Self::EngineOff),
            "escalator" => Some(Self::Escalator),
            "escalator-down" => Some(Self::EscalatorDown),
            "escalator-up" => Some(Self::EscalatorUp),
            "feather" => Some(Self::Feather),
            "feather-off" => Some(Self::FeatherOff),
            "ferry" => Some(Self::Ferry),
            "fire-hydrant" => Some(Self::FireHydrant),
            "fire-hydrant-off" => Some(Self::FireHydrantOff),
            "firetruck" => Some(Self::Firetruck),
            "fish" => Some(Self::Fish),
            "fish-bone" => Some(Self::FishBone),
            "fish-hook" => Some(Self::FishHook),
            "fish-hook-off" => Some(Self::FishHookOff),
            "fish-off" => Some(Self::FishOff),
            "flag" => Some(Self::Flag),
            "flag-2" => Some(Self::Flag2),
            "flag-2-off" => Some(Self::Flag2Off),
            "flag-3" => Some(Self::Flag3),
            "flag-bolt" => Some(Self::FlagBolt),
            "flag-cancel" => Some(Self::FlagCancel),
            "flag-check" => Some(Self::FlagCheck),
            "flag-code" => Some(Self::FlagCode),
            "flag-cog" => Some(Self::FlagCog),
            "flag-dollar" => Some(Self::FlagDollar),
            "flag-down" => Some(Self::FlagDown),
            "flag-exclamation" => Some(Self::FlagExclamation),
            "flag-heart" => Some(Self::FlagHeart),
            "flag-minus" => Some(Self::FlagMinus),
            "flag-off" => Some(Self::FlagOff),
            "flag-pause" => Some(Self::FlagPause),
            "flag-pin" => Some(Self::FlagPin),
            "flag-plus" => Some(Self::FlagPlus),
            "flag-question" => Some(Self::FlagQuestion),
            "flag-search" => Some(Self::FlagSearch),
            "flag-share" => Some(Self::FlagShare),
            "flag-spark" => Some(Self::FlagSpark),
            "flag-star" => Some(Self::FlagStar),
            "flag-up" => Some(Self::FlagUp),
            "flag-x" => Some(Self::FlagX),
            "flame" => Some(Self::Flame),
            "flame-off" => Some(Self::FlameOff),
            "flare" => Some(Self::Flare),
            "flood" => Some(Self::Flood),
            "flower" => Some(Self::Flower),
            "flower-off" => Some(Self::FlowerOff),
            "forklift" => Some(Self::Forklift),
            "fountain" => Some(Self::Fountain),
            "fountain-off" => Some(Self::FountainOff),
            "garden-cart" => Some(Self::GardenCart),
            "garden-cart-off" => Some(Self::GardenCartOff),
            "gas-station" => Some(Self::GasStation),
            "gas-station-off" => Some(Self::GasStationOff),
            "geometry" => Some(Self::Geometry),
            "globe" => Some(Self::Globe),
            "globe-off" => Some(Self::GlobeOff),
            "gps" => Some(Self::Gps),
            "grave" => Some(Self::Grave),
            "grave-2" => Some(Self::Grave2),
            "growth" => Some(Self::Growth),
            "haze" => Some(Self::Haze),
            "helicopter" => Some(Self::Helicopter),
            "helicopter-landing" => Some(Self::HelicopterLanding),
            "horse" => Some(Self::Horse),
            "horseshoe" => Some(Self::Horseshoe),
            "iceberg" => Some(Self::Iceberg),
            "jetski" => Some(Self::Jetski),
            "leaf" => Some(Self::Leaf),
            "leaf-2" => Some(Self::Leaf2),
            "leaf-maple" => Some(Self::LeafMaple),
            "leaf-off" => Some(Self::LeafOff),
            "live-view" => Some(Self::LiveView),
            "location" => Some(Self::Location),
            "location-bolt" => Some(Self::LocationBolt),
            "location-broken" => Some(Self::LocationBroken),
            "location-cancel" => Some(Self::LocationCancel),
            "location-check" => Some(Self::LocationCheck),
            "location-code" => Some(Self::LocationCode),
            "location-cog" => Some(Self::LocationCog),
            "location-dollar" => Some(Self::LocationDollar),
            "location-down" => Some(Self::LocationDown),
            "location-exclamation" => Some(Self::LocationExclamation),
            "location-heart" => Some(Self::LocationHeart),
            "location-minus" => Some(Self::LocationMinus),
            "location-off" => Some(Self::LocationOff),
            "location-pause" => Some(Self::LocationPause),
            "location-pin" => Some(Self::LocationPin),
            "location-plus" => Some(Self::LocationPlus),
            "location-question" => Some(Self::LocationQuestion),
            "location-search" => Some(Self::LocationSearch),
            "location-share" => Some(Self::LocationShare),
            "location-star" => Some(Self::LocationStar),
            "location-up" => Some(Self::LocationUp),
            "location-x" => Some(Self::LocationX),
            "map" => Some(Self::Map),
            "map-2" => Some(Self::Map2),
            "map-bolt" => Some(Self::MapBolt),
            "map-cancel" => Some(Self::MapCancel),
            "map-check" => Some(Self::MapCheck),
            "map-code" => Some(Self::MapCode),
            "map-cog" => Some(Self::MapCog),
            "map-discount" => Some(Self::MapDiscount),
            "map-dollar" => Some(Self::MapDollar),
            "map-down" => Some(Self::MapDown),
            "map-east" => Some(Self::MapEast),
            "map-exclamation" => Some(Self::MapExclamation),
            "map-heart" => Some(Self::MapHeart),
            "map-lock" => Some(Self::MapLock),
            "map-minus" => Some(Self::MapMinus),
            "map-north" => Some(Self::MapNorth),
            "map-off" => Some(Self::MapOff),
            "map-pause" => Some(Self::MapPause),
            "map-pin" => Some(Self::MapPin),
            "map-pin-2" => Some(Self::MapPin2),
            "map-pin-bolt" => Some(Self::MapPinBolt),
            "map-pin-cancel" => Some(Self::MapPinCancel),
            "map-pin-check" => Some(Self::MapPinCheck),
            "map-pin-code" => Some(Self::MapPinCode),
            "map-pin-cog" => Some(Self::MapPinCog),
            "map-pin-dollar" => Some(Self::MapPinDollar),
            "map-pin-down" => Some(Self::MapPinDown),
            "map-pin-exclamation" => Some(Self::MapPinExclamation),
            "map-pin-heart" => Some(Self::MapPinHeart),
            "map-pin-minus" => Some(Self::MapPinMinus),
            "map-pin-off" => Some(Self::MapPinOff),
            "map-pin-pause" => Some(Self::MapPinPause),
            "map-pin-pin" => Some(Self::MapPinPin),
            "map-pin-plus" => Some(Self::MapPinPlus),
            "map-pin-question" => Some(Self::MapPinQuestion),
            "map-pin-search" => Some(Self::MapPinSearch),
            "map-pin-share" => Some(Self::MapPinShare),
            "map-pin-star" => Some(Self::MapPinStar),
            "map-pin-up" => Some(Self::MapPinUp),
            "map-pin-x" => Some(Self::MapPinX),
            "map-pins" => Some(Self::MapPins),
            "map-plus" => Some(Self::MapPlus),
            "map-question" => Some(Self::MapQuestion),
            "map-route" => Some(Self::MapRoute),
            "map-search" => Some(Self::MapSearch),
            "map-share" => Some(Self::MapShare),
            "map-shield" => Some(Self::MapShield),
            "map-south" => Some(Self::MapSouth),
            "map-star" => Some(Self::MapStar),
            "map-up" => Some(Self::MapUp),
            "map-west" => Some(Self::MapWest),
            "map-x" => Some(Self::MapX),
            "masks-theater" => Some(Self::MasksTheater),
            "masks-theater-off" => Some(Self::MasksTheaterOff),
            "medical-cross" => Some(Self::MedicalCross),
            "medical-cross-circle" => Some(Self::MedicalCrossCircle),
            "medical-cross-off" => Some(Self::MedicalCrossOff),
            "meteor" => Some(Self::Meteor),
            "meteor-off" => Some(Self::MeteorOff),
            "mist" => Some(Self::Mist),
            "mist-off" => Some(Self::MistOff),
            "monkeybar" => Some(Self::Monkeybar),
            "moon" => Some(Self::Moon),
            "moon-2" => Some(Self::Moon2),
            "moon-off" => Some(Self::MoonOff),
            "moon-stars" => Some(Self::MoonStars),
            "moped" => Some(Self::Moped),
            "motorbike" => Some(Self::Motorbike),
            "mountain" => Some(Self::Mountain),
            "mountain-off" => Some(Self::MountainOff),
            "navigation" => Some(Self::Navigation),
            "navigation-bolt" => Some(Self::NavigationBolt),
            "navigation-cancel" => Some(Self::NavigationCancel),
            "navigation-check" => Some(Self::NavigationCheck),
            "navigation-code" => Some(Self::NavigationCode),
            "navigation-cog" => Some(Self::NavigationCog),
            "navigation-discount" => Some(Self::NavigationDiscount),
            "navigation-dollar" => Some(Self::NavigationDollar),
            "navigation-down" => Some(Self::NavigationDown),
            "navigation-east" => Some(Self::NavigationEast),
            "navigation-exclamation" => Some(Self::NavigationExclamation),
            "navigation-heart" => Some(Self::NavigationHeart),
            "navigation-minus" => Some(Self::NavigationMinus),
            "navigation-north" => Some(Self::NavigationNorth),
            "navigation-off" => Some(Self::NavigationOff),
            "navigation-pause" => Some(Self::NavigationPause),
            "navigation-pin" => Some(Self::NavigationPin),
            "navigation-plus" => Some(Self::NavigationPlus),
            "navigation-question" => Some(Self::NavigationQuestion),
            "navigation-search" => Some(Self::NavigationSearch),
            "navigation-share" => Some(Self::NavigationShare),
            "navigation-south" => Some(Self::NavigationSouth),
            "navigation-star" => Some(Self::NavigationStar),
            "navigation-top" => Some(Self::NavigationTop),
            "navigation-up" => Some(Self::NavigationUp),
            "navigation-west" => Some(Self::NavigationWest),
            "navigation-x" => Some(Self::NavigationX),
            "north-star" => Some(Self::NorthStar),
            "parachute" => Some(Self::Parachute),
            "parachute-off" => Some(Self::ParachuteOff),
            "parking" => Some(Self::Parking),
            "parking-meter" => Some(Self::ParkingMeter),
            "parking-off" => Some(Self::ParkingOff),
            "paw" => Some(Self::Paw),
            "paw-off" => Some(Self::PawOff),
            "pennant" => Some(Self::Pennant),
            "pennant-2" => Some(Self::Pennant2),
            "pennant-off" => Some(Self::PennantOff),
            "picnic-table" => Some(Self::PicnicTable),
            "pig" => Some(Self::Pig),
            "pig-money" => Some(Self::PigMoney),
            "pig-off" => Some(Self::PigOff),
            "pin" => Some(Self::Pin),
            "pinned" => Some(Self::Pinned),
            "pinned-off" => Some(Self::PinnedOff),
            "plane" => Some(Self::Plane),
            "plane-arrival" => Some(Self::PlaneArrival),
            "plane-departure" => Some(Self::PlaneDeparture),
            "plane-inflight" => Some(Self::PlaneInflight),
            "plane-off" => Some(Self::PlaneOff),
            "plane-tilt" => Some(Self::PlaneTilt),
            "planet" => Some(Self::Planet),
            "planet-off" => Some(Self::PlanetOff),
            "plant" => Some(Self::Plant),
            "plant-2" => Some(Self::Plant2),
            "plant-2-off" => Some(Self::Plant2Off),
            "plant-off" => Some(Self::PlantOff),
            "pokeball" => Some(Self::Pokeball),
            "pokeball-off" => Some(Self::PokeballOff),
            "prison" => Some(Self::Prison),
            "radar" => Some(Self::Radar),
            "radar-2" => Some(Self::Radar2),
            "radar-off" => Some(Self::RadarOff),
            "rainbow" => Some(Self::Rainbow),
            "rainbow-off" => Some(Self::RainbowOff),
            "ripple" => Some(Self::Ripple),
            "ripple-off" => Some(Self::RippleOff),
            "road" => Some(Self::Road),
            "road-off" => Some(Self::RoadOff),
            "road-sign" => Some(Self::RoadSign),
            "rocket" => Some(Self::Rocket),
            "rocket-off" => Some(Self::RocketOff),
            "rollercoaster" => Some(Self::Rollercoaster),
            "rollercoaster-off" => Some(Self::RollercoasterOff),
            "route" => Some(Self::Route),
            "route-2" => Some(Self::Route2),
            "route-alt-left" => Some(Self::RouteAltLeft),
            "route-alt-right" => Some(Self::RouteAltRight),
            "route-off" => Some(Self::RouteOff),
            "route-square" => Some(Self::RouteSquare),
            "route-square-2" => Some(Self::RouteSquare2),
            "route-x" => Some(Self::RouteX),
            "route-x-2" => Some(Self::RouteX2),
            "sailboat" => Some(Self::Sailboat),
            "sailboat-2" => Some(Self::Sailboat2),
            "sailboat-off" => Some(Self::SailboatOff),
            "satellite" => Some(Self::Satellite),
            "satellite-off" => Some(Self::SatelliteOff),
            "school" => Some(Self::School),
            "school-off" => Some(Self::SchoolOff),
            "scooter" => Some(Self::Scooter),
            "scooter-electric" => Some(Self::ScooterElectric),
            "seedling" => Some(Self::Seedling),
            "seedling-off" => Some(Self::SeedlingOff),
            "segway" => Some(Self::Segway),
            "ship" => Some(Self::Ship),
            "ship-off" => Some(Self::ShipOff),
            "sign-left" => Some(Self::SignLeft),
            "sign-right" => Some(Self::SignRight),
            "skateboard" => Some(Self::Skateboard),
            "skateboard-off" => Some(Self::SkateboardOff),
            "sleigh" => Some(Self::Sleigh),
            "snowflake" => Some(Self::Snowflake),
            "snowflake-off" => Some(Self::SnowflakeOff),
            "snowman" => Some(Self::Snowman),
            "speedboat" => Some(Self::Speedboat),
            "spider" => Some(Self::Spider),
            "stairs" => Some(Self::Stairs),
            "stairs-down" => Some(Self::StairsDown),
            "stairs-up" => Some(Self::StairsUp),
            "steering-wheel" => Some(Self::SteeringWheel),
            "steering-wheel-off" => Some(Self::SteeringWheelOff),
            "storm" => Some(Self::Storm),
            "storm-off" => Some(Self::StormOff),
            "submarine" => Some(Self::Submarine),
            "sun" => Some(Self::Sun),
            "sun-electricity" => Some(Self::SunElectricity),
            "sun-high" => Some(Self::SunHigh),
            "sun-low" => Some(Self::SunLow),
            "sun-moon" => Some(Self::SunMoon),
            "sun-off" => Some(Self::SunOff),
            "sun-wind" => Some(Self::SunWind),
            "sunrise" => Some(Self::Sunrise),
            "sunset" => Some(Self::Sunset),
            "sunset-2" => Some(Self::Sunset2),
            "tank" => Some(Self::Tank),
            "target" => Some(Self::Target),
            "target-off" => Some(Self::TargetOff),
            "temperature" => Some(Self::Temperature),
            "temperature-celsius" => Some(Self::TemperatureCelsius),
            "temperature-fahrenheit" => Some(Self::TemperatureFahrenheit),
            "temperature-minus" => Some(Self::TemperatureMinus),
            "temperature-off" => Some(Self::TemperatureOff),
            "temperature-plus" => Some(Self::TemperaturePlus),
            "tent" => Some(Self::Tent),
            "tent-off" => Some(Self::TentOff),
            "theater" => Some(Self::Theater),
            "tir" => Some(Self::Tir),
            "tools-kitchen-2-off" => Some(Self::ToolsKitchen2Off),
            "tools-kitchen-off" => Some(Self::ToolsKitchenOff),
            "tornado" => Some(Self::Tornado),
            "track" => Some(Self::Track),
            "tractor" => Some(Self::Tractor),
            "traffic-cone" => Some(Self::TrafficCone),
            "traffic-cone-off" => Some(Self::TrafficConeOff),
            "traffic-lights" => Some(Self::TrafficLights),
            "traffic-lights-off" => Some(Self::TrafficLightsOff),
            "train" => Some(Self::Train),
            "tree" => Some(Self::Tree),
            "trees" => Some(Self::Trees),
            "trolley" => Some(Self::Trolley),
            "truck" => Some(Self::Truck),
            "truck-off" => Some(Self::TruckOff),
            "unicycle" => Some(Self::Unicycle),
            "uv-index" => Some(Self::UvIndex),
            "viewfinder" => Some(Self::Viewfinder),
            "viewfinder-off" => Some(Self::ViewfinderOff),
            "volcano" => Some(Self::Volcano),
            "wheelchair" => Some(Self::Wheelchair),
            "wheelchair-off" => Some(Self::WheelchairOff),
            "whirl" => Some(Self::Whirl),
            "wind" => Some(Self::Wind),
            "wind-electricity" => Some(Self::WindElectricity),
            "wind-off" => Some(Self::WindOff),
            "windmill" => Some(Self::Windmill),
            "windmill-off" => Some(Self::WindmillOff),
            "windsock" => Some(Self::Windsock),
            "wiper" => Some(Self::Wiper),
            "wiper-wash" => Some(Self::WiperWash),
            "world" => Some(Self::World),
            "world-bolt" => Some(Self::WorldBolt),
            "world-cancel" => Some(Self::WorldCancel),
            "world-check" => Some(Self::WorldCheck),
            "world-code" => Some(Self::WorldCode),
            "world-cog" => Some(Self::WorldCog),
            "world-dollar" => Some(Self::WorldDollar),
            "world-down" => Some(Self::WorldDown),
            "world-download" => Some(Self::WorldDownload),
            "world-exclamation" => Some(Self::WorldExclamation),
            "world-heart" => Some(Self::WorldHeart),
            "world-latitude" => Some(Self::WorldLatitude),
            "world-longitude" => Some(Self::WorldLongitude),
            "world-map" => Some(Self::WorldMap),
            "world-minus" => Some(Self::WorldMinus),
            "world-off" => Some(Self::WorldOff),
            "world-pause" => Some(Self::WorldPause),
            "world-pin" => Some(Self::WorldPin),
            "world-plus" => Some(Self::WorldPlus),
            "world-question" => Some(Self::WorldQuestion),
            "world-search" => Some(Self::WorldSearch),
            "world-share" => Some(Self::WorldShare),
            "world-star" => Some(Self::WorldStar),
            "world-up" => Some(Self::WorldUp),
            "world-upload" => Some(Self::WorldUpload),
            "world-www" => Some(Self::WorldWww),
            "world-x" => Some(Self::WorldX),
            "wrecking-ball" => Some(Self::WreckingBall),
            "zeppelin" => Some(Self::Zeppelin),
            "zeppelin-off" => Some(Self::ZeppelinOff),
            "zodiac-aquarius" => Some(Self::ZodiacAquarius),
            "zodiac-aries" => Some(Self::ZodiacAries),
            "zodiac-cancer" => Some(Self::ZodiacCancer),
            "zodiac-capricorn" => Some(Self::ZodiacCapricorn),
            "zodiac-gemini" => Some(Self::ZodiacGemini),
            "zodiac-leo" => Some(Self::ZodiacLeo),
            "zodiac-libra" => Some(Self::ZodiacLibra),
            "zodiac-pisces" => Some(Self::ZodiacPisces),
            "zodiac-sagittarius" => Some(Self::ZodiacSagittarius),
            "zodiac-scorpio" => Some(Self::ZodiacScorpio),
            "zodiac-taurus" => Some(Self::ZodiacTaurus),
            "zodiac-virgo" => Some(Self::ZodiacVirgo),
            "zoom" => Some(Self::Zoom),
            "zoom-cancel" => Some(Self::ZoomCancel),
            "zoom-check" => Some(Self::ZoomCheck),
            "zoom-code" => Some(Self::ZoomCode),
            "zoom-exclamation" => Some(Self::ZoomExclamation),
            "zoom-in" => Some(Self::ZoomIn),
            "zoom-in-area" => Some(Self::ZoomInArea),
            "zoom-money" => Some(Self::ZoomMoney),
            "zoom-out" => Some(Self::ZoomOut),
            "zoom-out-area" => Some(Self::ZoomOutArea),
            "zoom-pan" => Some(Self::ZoomPan),
            "zoom-question" => Some(Self::ZoomQuestion),
            "zoom-replace" => Some(Self::ZoomReplace),
            "zoom-reset" => Some(Self::ZoomReset),
            _ => None,
        }
    }
}

impl TablerIconData for MapIcon {
    fn name(&self) -> &'static str {
        match self {
            Self::Acorn => "acorn",
            Self::AerialLift => "aerial-lift",
            Self::AirBalloon => "air-balloon",
            Self::AirTrafficControl => "air-traffic-control",
            Self::Alien => "alien",
            Self::Ambulance => "ambulance",
            Self::Anchor => "anchor",
            Self::AnchorOff => "anchor-off",
            Self::Atom => "atom",
            Self::Atom2 => "atom-2",
            Self::AtomOff => "atom-off",
            Self::AutomaticGearbox => "automatic-gearbox",
            Self::Backhoe => "backhoe",
            Self::Balloon => "balloon",
            Self::BalloonOff => "balloon-off",
            Self::Bat => "bat",
            Self::BatteryAutomotive => "battery-automotive",
            Self::Beach => "beach",
            Self::BeachOff => "beach-off",
            Self::Bed => "bed",
            Self::BedOff => "bed-off",
            Self::Bike => "bike",
            Self::BikeOff => "bike-off",
            Self::Bulldozer => "bulldozer",
            Self::Bus => "bus",
            Self::BusOff => "bus-off",
            Self::BusStop => "bus-stop",
            Self::Butterfly => "butterfly",
            Self::Cactus => "cactus",
            Self::CactusOff => "cactus-off",
            Self::Camper => "camper",
            Self::Campfire => "campfire",
            Self::Cannabis => "cannabis",
            Self::Car => "car",
            Self::Car4wd => "car-4wd",
            Self::CarCrane => "car-crane",
            Self::CarCrash => "car-crash",
            Self::CarDoor => "car-door",
            Self::CarFan => "car-fan",
            Self::CarFan1 => "car-fan-1",
            Self::CarFan2 => "car-fan-2",
            Self::CarFan3 => "car-fan-3",
            Self::CarFanAuto => "car-fan-auto",
            Self::CarLifter => "car-lifter",
            Self::CarOff => "car-off",
            Self::CarOffRoad => "car-off-road",
            Self::CarSuspension => "car-suspension",
            Self::CarSuv => "car-suv",
            Self::CarTurbine => "car-turbine",
            Self::Caravan => "caravan",
            Self::Cat => "cat",
            Self::ChargingPile => "charging-pile",
            Self::Cherry => "cherry",
            Self::ChristmasTree => "christmas-tree",
            Self::ChristmasTreeOff => "christmas-tree-off",
            Self::Cloud => "cloud",
            Self::CloudBolt => "cloud-bolt",
            Self::CloudCancel => "cloud-cancel",
            Self::CloudCheck => "cloud-check",
            Self::CloudCode => "cloud-code",
            Self::CloudCog => "cloud-cog",
            Self::CloudDollar => "cloud-dollar",
            Self::CloudDown => "cloud-down",
            Self::CloudExclamation => "cloud-exclamation",
            Self::CloudFog => "cloud-fog",
            Self::CloudHeart => "cloud-heart",
            Self::CloudMinus => "cloud-minus",
            Self::CloudOff => "cloud-off",
            Self::CloudPause => "cloud-pause",
            Self::CloudPin => "cloud-pin",
            Self::CloudPlus => "cloud-plus",
            Self::CloudQuestion => "cloud-question",
            Self::CloudRain => "cloud-rain",
            Self::CloudSearch => "cloud-search",
            Self::CloudShare => "cloud-share",
            Self::CloudSnow => "cloud-snow",
            Self::CloudStar => "cloud-star",
            Self::CloudStorm => "cloud-storm",
            Self::CloudUp => "cloud-up",
            Self::CloudX => "cloud-x",
            Self::Clover => "clover",
            Self::Clover2 => "clover-2",
            Self::Comet => "comet",
            Self::Compass => "compass",
            Self::CompassOff => "compass-off",
            Self::Crane => "crane",
            Self::CraneOff => "crane-off",
            Self::CrystalBall => "crystal-ball",
            Self::CurrentLocation => "current-location",
            Self::CurrentLocationOff => "current-location-off",
            Self::Deer => "deer",
            Self::Directions => "directions",
            Self::DirectionsOff => "directions-off",
            Self::Dog => "dog",
            Self::Drone => "drone",
            Self::DroneOff => "drone-off",
            Self::DropCircle => "drop-circle",
            Self::Droplets => "droplets",
            Self::Engine => "engine",
            Self::EngineOff => "engine-off",
            Self::Escalator => "escalator",
            Self::EscalatorDown => "escalator-down",
            Self::EscalatorUp => "escalator-up",
            Self::Feather => "feather",
            Self::FeatherOff => "feather-off",
            Self::Ferry => "ferry",
            Self::FireHydrant => "fire-hydrant",
            Self::FireHydrantOff => "fire-hydrant-off",
            Self::Firetruck => "firetruck",
            Self::Fish => "fish",
            Self::FishBone => "fish-bone",
            Self::FishHook => "fish-hook",
            Self::FishHookOff => "fish-hook-off",
            Self::FishOff => "fish-off",
            Self::Flag => "flag",
            Self::Flag2 => "flag-2",
            Self::Flag2Off => "flag-2-off",
            Self::Flag3 => "flag-3",
            Self::FlagBolt => "flag-bolt",
            Self::FlagCancel => "flag-cancel",
            Self::FlagCheck => "flag-check",
            Self::FlagCode => "flag-code",
            Self::FlagCog => "flag-cog",
            Self::FlagDollar => "flag-dollar",
            Self::FlagDown => "flag-down",
            Self::FlagExclamation => "flag-exclamation",
            Self::FlagHeart => "flag-heart",
            Self::FlagMinus => "flag-minus",
            Self::FlagOff => "flag-off",
            Self::FlagPause => "flag-pause",
            Self::FlagPin => "flag-pin",
            Self::FlagPlus => "flag-plus",
            Self::FlagQuestion => "flag-question",
            Self::FlagSearch => "flag-search",
            Self::FlagShare => "flag-share",
            Self::FlagSpark => "flag-spark",
            Self::FlagStar => "flag-star",
            Self::FlagUp => "flag-up",
            Self::FlagX => "flag-x",
            Self::Flame => "flame",
            Self::FlameOff => "flame-off",
            Self::Flare => "flare",
            Self::Flood => "flood",
            Self::Flower => "flower",
            Self::FlowerOff => "flower-off",
            Self::Forklift => "forklift",
            Self::Fountain => "fountain",
            Self::FountainOff => "fountain-off",
            Self::GardenCart => "garden-cart",
            Self::GardenCartOff => "garden-cart-off",
            Self::GasStation => "gas-station",
            Self::GasStationOff => "gas-station-off",
            Self::Geometry => "geometry",
            Self::Globe => "globe",
            Self::GlobeOff => "globe-off",
            Self::Gps => "gps",
            Self::Grave => "grave",
            Self::Grave2 => "grave-2",
            Self::Growth => "growth",
            Self::Haze => "haze",
            Self::Helicopter => "helicopter",
            Self::HelicopterLanding => "helicopter-landing",
            Self::Horse => "horse",
            Self::Horseshoe => "horseshoe",
            Self::Iceberg => "iceberg",
            Self::Jetski => "jetski",
            Self::Leaf => "leaf",
            Self::Leaf2 => "leaf-2",
            Self::LeafMaple => "leaf-maple",
            Self::LeafOff => "leaf-off",
            Self::LiveView => "live-view",
            Self::Location => "location",
            Self::LocationBolt => "location-bolt",
            Self::LocationBroken => "location-broken",
            Self::LocationCancel => "location-cancel",
            Self::LocationCheck => "location-check",
            Self::LocationCode => "location-code",
            Self::LocationCog => "location-cog",
            Self::LocationDollar => "location-dollar",
            Self::LocationDown => "location-down",
            Self::LocationExclamation => "location-exclamation",
            Self::LocationHeart => "location-heart",
            Self::LocationMinus => "location-minus",
            Self::LocationOff => "location-off",
            Self::LocationPause => "location-pause",
            Self::LocationPin => "location-pin",
            Self::LocationPlus => "location-plus",
            Self::LocationQuestion => "location-question",
            Self::LocationSearch => "location-search",
            Self::LocationShare => "location-share",
            Self::LocationStar => "location-star",
            Self::LocationUp => "location-up",
            Self::LocationX => "location-x",
            Self::Map => "map",
            Self::Map2 => "map-2",
            Self::MapBolt => "map-bolt",
            Self::MapCancel => "map-cancel",
            Self::MapCheck => "map-check",
            Self::MapCode => "map-code",
            Self::MapCog => "map-cog",
            Self::MapDiscount => "map-discount",
            Self::MapDollar => "map-dollar",
            Self::MapDown => "map-down",
            Self::MapEast => "map-east",
            Self::MapExclamation => "map-exclamation",
            Self::MapHeart => "map-heart",
            Self::MapLock => "map-lock",
            Self::MapMinus => "map-minus",
            Self::MapNorth => "map-north",
            Self::MapOff => "map-off",
            Self::MapPause => "map-pause",
            Self::MapPin => "map-pin",
            Self::MapPin2 => "map-pin-2",
            Self::MapPinBolt => "map-pin-bolt",
            Self::MapPinCancel => "map-pin-cancel",
            Self::MapPinCheck => "map-pin-check",
            Self::MapPinCode => "map-pin-code",
            Self::MapPinCog => "map-pin-cog",
            Self::MapPinDollar => "map-pin-dollar",
            Self::MapPinDown => "map-pin-down",
            Self::MapPinExclamation => "map-pin-exclamation",
            Self::MapPinHeart => "map-pin-heart",
            Self::MapPinMinus => "map-pin-minus",
            Self::MapPinOff => "map-pin-off",
            Self::MapPinPause => "map-pin-pause",
            Self::MapPinPin => "map-pin-pin",
            Self::MapPinPlus => "map-pin-plus",
            Self::MapPinQuestion => "map-pin-question",
            Self::MapPinSearch => "map-pin-search",
            Self::MapPinShare => "map-pin-share",
            Self::MapPinStar => "map-pin-star",
            Self::MapPinUp => "map-pin-up",
            Self::MapPinX => "map-pin-x",
            Self::MapPins => "map-pins",
            Self::MapPlus => "map-plus",
            Self::MapQuestion => "map-question",
            Self::MapRoute => "map-route",
            Self::MapSearch => "map-search",
            Self::MapShare => "map-share",
            Self::MapShield => "map-shield",
            Self::MapSouth => "map-south",
            Self::MapStar => "map-star",
            Self::MapUp => "map-up",
            Self::MapWest => "map-west",
            Self::MapX => "map-x",
            Self::MasksTheater => "masks-theater",
            Self::MasksTheaterOff => "masks-theater-off",
            Self::MedicalCross => "medical-cross",
            Self::MedicalCrossCircle => "medical-cross-circle",
            Self::MedicalCrossOff => "medical-cross-off",
            Self::Meteor => "meteor",
            Self::MeteorOff => "meteor-off",
            Self::Mist => "mist",
            Self::MistOff => "mist-off",
            Self::Monkeybar => "monkeybar",
            Self::Moon => "moon",
            Self::Moon2 => "moon-2",
            Self::MoonOff => "moon-off",
            Self::MoonStars => "moon-stars",
            Self::Moped => "moped",
            Self::Motorbike => "motorbike",
            Self::Mountain => "mountain",
            Self::MountainOff => "mountain-off",
            Self::Navigation => "navigation",
            Self::NavigationBolt => "navigation-bolt",
            Self::NavigationCancel => "navigation-cancel",
            Self::NavigationCheck => "navigation-check",
            Self::NavigationCode => "navigation-code",
            Self::NavigationCog => "navigation-cog",
            Self::NavigationDiscount => "navigation-discount",
            Self::NavigationDollar => "navigation-dollar",
            Self::NavigationDown => "navigation-down",
            Self::NavigationEast => "navigation-east",
            Self::NavigationExclamation => "navigation-exclamation",
            Self::NavigationHeart => "navigation-heart",
            Self::NavigationMinus => "navigation-minus",
            Self::NavigationNorth => "navigation-north",
            Self::NavigationOff => "navigation-off",
            Self::NavigationPause => "navigation-pause",
            Self::NavigationPin => "navigation-pin",
            Self::NavigationPlus => "navigation-plus",
            Self::NavigationQuestion => "navigation-question",
            Self::NavigationSearch => "navigation-search",
            Self::NavigationShare => "navigation-share",
            Self::NavigationSouth => "navigation-south",
            Self::NavigationStar => "navigation-star",
            Self::NavigationTop => "navigation-top",
            Self::NavigationUp => "navigation-up",
            Self::NavigationWest => "navigation-west",
            Self::NavigationX => "navigation-x",
            Self::NorthStar => "north-star",
            Self::Parachute => "parachute",
            Self::ParachuteOff => "parachute-off",
            Self::Parking => "parking",
            Self::ParkingMeter => "parking-meter",
            Self::ParkingOff => "parking-off",
            Self::Paw => "paw",
            Self::PawOff => "paw-off",
            Self::Pennant => "pennant",
            Self::Pennant2 => "pennant-2",
            Self::PennantOff => "pennant-off",
            Self::PicnicTable => "picnic-table",
            Self::Pig => "pig",
            Self::PigMoney => "pig-money",
            Self::PigOff => "pig-off",
            Self::Pin => "pin",
            Self::Pinned => "pinned",
            Self::PinnedOff => "pinned-off",
            Self::Plane => "plane",
            Self::PlaneArrival => "plane-arrival",
            Self::PlaneDeparture => "plane-departure",
            Self::PlaneInflight => "plane-inflight",
            Self::PlaneOff => "plane-off",
            Self::PlaneTilt => "plane-tilt",
            Self::Planet => "planet",
            Self::PlanetOff => "planet-off",
            Self::Plant => "plant",
            Self::Plant2 => "plant-2",
            Self::Plant2Off => "plant-2-off",
            Self::PlantOff => "plant-off",
            Self::Pokeball => "pokeball",
            Self::PokeballOff => "pokeball-off",
            Self::Prison => "prison",
            Self::Radar => "radar",
            Self::Radar2 => "radar-2",
            Self::RadarOff => "radar-off",
            Self::Rainbow => "rainbow",
            Self::RainbowOff => "rainbow-off",
            Self::Ripple => "ripple",
            Self::RippleOff => "ripple-off",
            Self::Road => "road",
            Self::RoadOff => "road-off",
            Self::RoadSign => "road-sign",
            Self::Rocket => "rocket",
            Self::RocketOff => "rocket-off",
            Self::Rollercoaster => "rollercoaster",
            Self::RollercoasterOff => "rollercoaster-off",
            Self::Route => "route",
            Self::Route2 => "route-2",
            Self::RouteAltLeft => "route-alt-left",
            Self::RouteAltRight => "route-alt-right",
            Self::RouteOff => "route-off",
            Self::RouteSquare => "route-square",
            Self::RouteSquare2 => "route-square-2",
            Self::RouteX => "route-x",
            Self::RouteX2 => "route-x-2",
            Self::Sailboat => "sailboat",
            Self::Sailboat2 => "sailboat-2",
            Self::SailboatOff => "sailboat-off",
            Self::Satellite => "satellite",
            Self::SatelliteOff => "satellite-off",
            Self::School => "school",
            Self::SchoolOff => "school-off",
            Self::Scooter => "scooter",
            Self::ScooterElectric => "scooter-electric",
            Self::Seedling => "seedling",
            Self::SeedlingOff => "seedling-off",
            Self::Segway => "segway",
            Self::Ship => "ship",
            Self::ShipOff => "ship-off",
            Self::SignLeft => "sign-left",
            Self::SignRight => "sign-right",
            Self::Skateboard => "skateboard",
            Self::SkateboardOff => "skateboard-off",
            Self::Sleigh => "sleigh",
            Self::Snowflake => "snowflake",
            Self::SnowflakeOff => "snowflake-off",
            Self::Snowman => "snowman",
            Self::Speedboat => "speedboat",
            Self::Spider => "spider",
            Self::Stairs => "stairs",
            Self::StairsDown => "stairs-down",
            Self::StairsUp => "stairs-up",
            Self::SteeringWheel => "steering-wheel",
            Self::SteeringWheelOff => "steering-wheel-off",
            Self::Storm => "storm",
            Self::StormOff => "storm-off",
            Self::Submarine => "submarine",
            Self::Sun => "sun",
            Self::SunElectricity => "sun-electricity",
            Self::SunHigh => "sun-high",
            Self::SunLow => "sun-low",
            Self::SunMoon => "sun-moon",
            Self::SunOff => "sun-off",
            Self::SunWind => "sun-wind",
            Self::Sunrise => "sunrise",
            Self::Sunset => "sunset",
            Self::Sunset2 => "sunset-2",
            Self::Tank => "tank",
            Self::Target => "target",
            Self::TargetOff => "target-off",
            Self::Temperature => "temperature",
            Self::TemperatureCelsius => "temperature-celsius",
            Self::TemperatureFahrenheit => "temperature-fahrenheit",
            Self::TemperatureMinus => "temperature-minus",
            Self::TemperatureOff => "temperature-off",
            Self::TemperaturePlus => "temperature-plus",
            Self::Tent => "tent",
            Self::TentOff => "tent-off",
            Self::Theater => "theater",
            Self::Tir => "tir",
            Self::ToolsKitchen2Off => "tools-kitchen-2-off",
            Self::ToolsKitchenOff => "tools-kitchen-off",
            Self::Tornado => "tornado",
            Self::Track => "track",
            Self::Tractor => "tractor",
            Self::TrafficCone => "traffic-cone",
            Self::TrafficConeOff => "traffic-cone-off",
            Self::TrafficLights => "traffic-lights",
            Self::TrafficLightsOff => "traffic-lights-off",
            Self::Train => "train",
            Self::Tree => "tree",
            Self::Trees => "trees",
            Self::Trolley => "trolley",
            Self::Truck => "truck",
            Self::TruckOff => "truck-off",
            Self::Unicycle => "unicycle",
            Self::UvIndex => "uv-index",
            Self::Viewfinder => "viewfinder",
            Self::ViewfinderOff => "viewfinder-off",
            Self::Volcano => "volcano",
            Self::Wheelchair => "wheelchair",
            Self::WheelchairOff => "wheelchair-off",
            Self::Whirl => "whirl",
            Self::Wind => "wind",
            Self::WindElectricity => "wind-electricity",
            Self::WindOff => "wind-off",
            Self::Windmill => "windmill",
            Self::WindmillOff => "windmill-off",
            Self::Windsock => "windsock",
            Self::Wiper => "wiper",
            Self::WiperWash => "wiper-wash",
            Self::World => "world",
            Self::WorldBolt => "world-bolt",
            Self::WorldCancel => "world-cancel",
            Self::WorldCheck => "world-check",
            Self::WorldCode => "world-code",
            Self::WorldCog => "world-cog",
            Self::WorldDollar => "world-dollar",
            Self::WorldDown => "world-down",
            Self::WorldDownload => "world-download",
            Self::WorldExclamation => "world-exclamation",
            Self::WorldHeart => "world-heart",
            Self::WorldLatitude => "world-latitude",
            Self::WorldLongitude => "world-longitude",
            Self::WorldMap => "world-map",
            Self::WorldMinus => "world-minus",
            Self::WorldOff => "world-off",
            Self::WorldPause => "world-pause",
            Self::WorldPin => "world-pin",
            Self::WorldPlus => "world-plus",
            Self::WorldQuestion => "world-question",
            Self::WorldSearch => "world-search",
            Self::WorldShare => "world-share",
            Self::WorldStar => "world-star",
            Self::WorldUp => "world-up",
            Self::WorldUpload => "world-upload",
            Self::WorldWww => "world-www",
            Self::WorldX => "world-x",
            Self::WreckingBall => "wrecking-ball",
            Self::Zeppelin => "zeppelin",
            Self::ZeppelinOff => "zeppelin-off",
            Self::ZodiacAquarius => "zodiac-aquarius",
            Self::ZodiacAries => "zodiac-aries",
            Self::ZodiacCancer => "zodiac-cancer",
            Self::ZodiacCapricorn => "zodiac-capricorn",
            Self::ZodiacGemini => "zodiac-gemini",
            Self::ZodiacLeo => "zodiac-leo",
            Self::ZodiacLibra => "zodiac-libra",
            Self::ZodiacPisces => "zodiac-pisces",
            Self::ZodiacSagittarius => "zodiac-sagittarius",
            Self::ZodiacScorpio => "zodiac-scorpio",
            Self::ZodiacTaurus => "zodiac-taurus",
            Self::ZodiacVirgo => "zodiac-virgo",
            Self::Zoom => "zoom",
            Self::ZoomCancel => "zoom-cancel",
            Self::ZoomCheck => "zoom-check",
            Self::ZoomCode => "zoom-code",
            Self::ZoomExclamation => "zoom-exclamation",
            Self::ZoomIn => "zoom-in",
            Self::ZoomInArea => "zoom-in-area",
            Self::ZoomMoney => "zoom-money",
            Self::ZoomOut => "zoom-out",
            Self::ZoomOutArea => "zoom-out-area",
            Self::ZoomPan => "zoom-pan",
            Self::ZoomQuestion => "zoom-question",
            Self::ZoomReplace => "zoom-replace",
            Self::ZoomReset => "zoom-reset",
        }
    }

    fn outline_svg(&self) -> &'static str {
        match self {
            Self::Acorn => ACORN_SVG,
            Self::AerialLift => AERIAL_LIFT_SVG,
            Self::AirBalloon => AIR_BALLOON_SVG,
            Self::AirTrafficControl => AIR_TRAFFIC_CONTROL_SVG,
            Self::Alien => ALIEN_SVG,
            Self::Ambulance => AMBULANCE_SVG,
            Self::Anchor => ANCHOR_SVG,
            Self::AnchorOff => ANCHOR_OFF_SVG,
            Self::Atom => ATOM_SVG,
            Self::Atom2 => ATOM_2_SVG,
            Self::AtomOff => ATOM_OFF_SVG,
            Self::AutomaticGearbox => AUTOMATIC_GEARBOX_SVG,
            Self::Backhoe => BACKHOE_SVG,
            Self::Balloon => BALLOON_SVG,
            Self::BalloonOff => BALLOON_OFF_SVG,
            Self::Bat => BAT_SVG,
            Self::BatteryAutomotive => BATTERY_AUTOMOTIVE_SVG,
            Self::Beach => BEACH_SVG,
            Self::BeachOff => BEACH_OFF_SVG,
            Self::Bed => BED_SVG,
            Self::BedOff => BED_OFF_SVG,
            Self::Bike => BIKE_SVG,
            Self::BikeOff => BIKE_OFF_SVG,
            Self::Bulldozer => BULLDOZER_SVG,
            Self::Bus => BUS_SVG,
            Self::BusOff => BUS_OFF_SVG,
            Self::BusStop => BUS_STOP_SVG,
            Self::Butterfly => BUTTERFLY_SVG,
            Self::Cactus => CACTUS_SVG,
            Self::CactusOff => CACTUS_OFF_SVG,
            Self::Camper => CAMPER_SVG,
            Self::Campfire => CAMPFIRE_SVG,
            Self::Cannabis => CANNABIS_SVG,
            Self::Car => CAR_SVG,
            Self::Car4wd => CAR_4WD_SVG,
            Self::CarCrane => CAR_CRANE_SVG,
            Self::CarCrash => CAR_CRASH_SVG,
            Self::CarDoor => CAR_DOOR_SVG,
            Self::CarFan => CAR_FAN_SVG,
            Self::CarFan1 => CAR_FAN_1_SVG,
            Self::CarFan2 => CAR_FAN_2_SVG,
            Self::CarFan3 => CAR_FAN_3_SVG,
            Self::CarFanAuto => CAR_FAN_AUTO_SVG,
            Self::CarLifter => CAR_LIFTER_SVG,
            Self::CarOff => CAR_OFF_SVG,
            Self::CarOffRoad => CAR_OFF_ROAD_SVG,
            Self::CarSuspension => CAR_SUSPENSION_SVG,
            Self::CarSuv => CAR_SUV_SVG,
            Self::CarTurbine => CAR_TURBINE_SVG,
            Self::Caravan => CARAVAN_SVG,
            Self::Cat => CAT_SVG,
            Self::ChargingPile => CHARGING_PILE_SVG,
            Self::Cherry => CHERRY_SVG,
            Self::ChristmasTree => CHRISTMAS_TREE_SVG,
            Self::ChristmasTreeOff => CHRISTMAS_TREE_OFF_SVG,
            Self::Cloud => CLOUD_SVG,
            Self::CloudBolt => CLOUD_BOLT_SVG,
            Self::CloudCancel => CLOUD_CANCEL_SVG,
            Self::CloudCheck => CLOUD_CHECK_SVG,
            Self::CloudCode => CLOUD_CODE_SVG,
            Self::CloudCog => CLOUD_COG_SVG,
            Self::CloudDollar => CLOUD_DOLLAR_SVG,
            Self::CloudDown => CLOUD_DOWN_SVG,
            Self::CloudExclamation => CLOUD_EXCLAMATION_SVG,
            Self::CloudFog => CLOUD_FOG_SVG,
            Self::CloudHeart => CLOUD_HEART_SVG,
            Self::CloudMinus => CLOUD_MINUS_SVG,
            Self::CloudOff => CLOUD_OFF_SVG,
            Self::CloudPause => CLOUD_PAUSE_SVG,
            Self::CloudPin => CLOUD_PIN_SVG,
            Self::CloudPlus => CLOUD_PLUS_SVG,
            Self::CloudQuestion => CLOUD_QUESTION_SVG,
            Self::CloudRain => CLOUD_RAIN_SVG,
            Self::CloudSearch => CLOUD_SEARCH_SVG,
            Self::CloudShare => CLOUD_SHARE_SVG,
            Self::CloudSnow => CLOUD_SNOW_SVG,
            Self::CloudStar => CLOUD_STAR_SVG,
            Self::CloudStorm => CLOUD_STORM_SVG,
            Self::CloudUp => CLOUD_UP_SVG,
            Self::CloudX => CLOUD_X_SVG,
            Self::Clover => CLOVER_SVG,
            Self::Clover2 => CLOVER_2_SVG,
            Self::Comet => COMET_SVG,
            Self::Compass => COMPASS_SVG,
            Self::CompassOff => COMPASS_OFF_SVG,
            Self::Crane => CRANE_SVG,
            Self::CraneOff => CRANE_OFF_SVG,
            Self::CrystalBall => CRYSTAL_BALL_SVG,
            Self::CurrentLocation => CURRENT_LOCATION_SVG,
            Self::CurrentLocationOff => CURRENT_LOCATION_OFF_SVG,
            Self::Deer => DEER_SVG,
            Self::Directions => DIRECTIONS_SVG,
            Self::DirectionsOff => DIRECTIONS_OFF_SVG,
            Self::Dog => DOG_SVG,
            Self::Drone => DRONE_SVG,
            Self::DroneOff => DRONE_OFF_SVG,
            Self::DropCircle => DROP_CIRCLE_SVG,
            Self::Droplets => DROPLETS_SVG,
            Self::Engine => ENGINE_SVG,
            Self::EngineOff => ENGINE_OFF_SVG,
            Self::Escalator => ESCALATOR_SVG,
            Self::EscalatorDown => ESCALATOR_DOWN_SVG,
            Self::EscalatorUp => ESCALATOR_UP_SVG,
            Self::Feather => FEATHER_SVG,
            Self::FeatherOff => FEATHER_OFF_SVG,
            Self::Ferry => FERRY_SVG,
            Self::FireHydrant => FIRE_HYDRANT_SVG,
            Self::FireHydrantOff => FIRE_HYDRANT_OFF_SVG,
            Self::Firetruck => FIRETRUCK_SVG,
            Self::Fish => FISH_SVG,
            Self::FishBone => FISH_BONE_SVG,
            Self::FishHook => FISH_HOOK_SVG,
            Self::FishHookOff => FISH_HOOK_OFF_SVG,
            Self::FishOff => FISH_OFF_SVG,
            Self::Flag => FLAG_SVG,
            Self::Flag2 => FLAG_2_SVG,
            Self::Flag2Off => FLAG_2_OFF_SVG,
            Self::Flag3 => FLAG_3_SVG,
            Self::FlagBolt => FLAG_BOLT_SVG,
            Self::FlagCancel => FLAG_CANCEL_SVG,
            Self::FlagCheck => FLAG_CHECK_SVG,
            Self::FlagCode => FLAG_CODE_SVG,
            Self::FlagCog => FLAG_COG_SVG,
            Self::FlagDollar => FLAG_DOLLAR_SVG,
            Self::FlagDown => FLAG_DOWN_SVG,
            Self::FlagExclamation => FLAG_EXCLAMATION_SVG,
            Self::FlagHeart => FLAG_HEART_SVG,
            Self::FlagMinus => FLAG_MINUS_SVG,
            Self::FlagOff => FLAG_OFF_SVG,
            Self::FlagPause => FLAG_PAUSE_SVG,
            Self::FlagPin => FLAG_PIN_SVG,
            Self::FlagPlus => FLAG_PLUS_SVG,
            Self::FlagQuestion => FLAG_QUESTION_SVG,
            Self::FlagSearch => FLAG_SEARCH_SVG,
            Self::FlagShare => FLAG_SHARE_SVG,
            Self::FlagSpark => FLAG_SPARK_SVG,
            Self::FlagStar => FLAG_STAR_SVG,
            Self::FlagUp => FLAG_UP_SVG,
            Self::FlagX => FLAG_X_SVG,
            Self::Flame => FLAME_SVG,
            Self::FlameOff => FLAME_OFF_SVG,
            Self::Flare => FLARE_SVG,
            Self::Flood => FLOOD_SVG,
            Self::Flower => FLOWER_SVG,
            Self::FlowerOff => FLOWER_OFF_SVG,
            Self::Forklift => FORKLIFT_SVG,
            Self::Fountain => FOUNTAIN_SVG,
            Self::FountainOff => FOUNTAIN_OFF_SVG,
            Self::GardenCart => GARDEN_CART_SVG,
            Self::GardenCartOff => GARDEN_CART_OFF_SVG,
            Self::GasStation => GAS_STATION_SVG,
            Self::GasStationOff => GAS_STATION_OFF_SVG,
            Self::Geometry => GEOMETRY_SVG,
            Self::Globe => GLOBE_SVG,
            Self::GlobeOff => GLOBE_OFF_SVG,
            Self::Gps => GPS_SVG,
            Self::Grave => GRAVE_SVG,
            Self::Grave2 => GRAVE_2_SVG,
            Self::Growth => GROWTH_SVG,
            Self::Haze => HAZE_SVG,
            Self::Helicopter => HELICOPTER_SVG,
            Self::HelicopterLanding => HELICOPTER_LANDING_SVG,
            Self::Horse => HORSE_SVG,
            Self::Horseshoe => HORSESHOE_SVG,
            Self::Iceberg => ICEBERG_SVG,
            Self::Jetski => JETSKI_SVG,
            Self::Leaf => LEAF_SVG,
            Self::Leaf2 => LEAF_2_SVG,
            Self::LeafMaple => LEAF_MAPLE_SVG,
            Self::LeafOff => LEAF_OFF_SVG,
            Self::LiveView => LIVE_VIEW_SVG,
            Self::Location => LOCATION_SVG,
            Self::LocationBolt => LOCATION_BOLT_SVG,
            Self::LocationBroken => LOCATION_BROKEN_SVG,
            Self::LocationCancel => LOCATION_CANCEL_SVG,
            Self::LocationCheck => LOCATION_CHECK_SVG,
            Self::LocationCode => LOCATION_CODE_SVG,
            Self::LocationCog => LOCATION_COG_SVG,
            Self::LocationDollar => LOCATION_DOLLAR_SVG,
            Self::LocationDown => LOCATION_DOWN_SVG,
            Self::LocationExclamation => LOCATION_EXCLAMATION_SVG,
            Self::LocationHeart => LOCATION_HEART_SVG,
            Self::LocationMinus => LOCATION_MINUS_SVG,
            Self::LocationOff => LOCATION_OFF_SVG,
            Self::LocationPause => LOCATION_PAUSE_SVG,
            Self::LocationPin => LOCATION_PIN_SVG,
            Self::LocationPlus => LOCATION_PLUS_SVG,
            Self::LocationQuestion => LOCATION_QUESTION_SVG,
            Self::LocationSearch => LOCATION_SEARCH_SVG,
            Self::LocationShare => LOCATION_SHARE_SVG,
            Self::LocationStar => LOCATION_STAR_SVG,
            Self::LocationUp => LOCATION_UP_SVG,
            Self::LocationX => LOCATION_X_SVG,
            Self::Map => MAP_SVG,
            Self::Map2 => MAP_2_SVG,
            Self::MapBolt => MAP_BOLT_SVG,
            Self::MapCancel => MAP_CANCEL_SVG,
            Self::MapCheck => MAP_CHECK_SVG,
            Self::MapCode => MAP_CODE_SVG,
            Self::MapCog => MAP_COG_SVG,
            Self::MapDiscount => MAP_DISCOUNT_SVG,
            Self::MapDollar => MAP_DOLLAR_SVG,
            Self::MapDown => MAP_DOWN_SVG,
            Self::MapEast => MAP_EAST_SVG,
            Self::MapExclamation => MAP_EXCLAMATION_SVG,
            Self::MapHeart => MAP_HEART_SVG,
            Self::MapLock => MAP_LOCK_SVG,
            Self::MapMinus => MAP_MINUS_SVG,
            Self::MapNorth => MAP_NORTH_SVG,
            Self::MapOff => MAP_OFF_SVG,
            Self::MapPause => MAP_PAUSE_SVG,
            Self::MapPin => MAP_PIN_SVG,
            Self::MapPin2 => MAP_PIN_2_SVG,
            Self::MapPinBolt => MAP_PIN_BOLT_SVG,
            Self::MapPinCancel => MAP_PIN_CANCEL_SVG,
            Self::MapPinCheck => MAP_PIN_CHECK_SVG,
            Self::MapPinCode => MAP_PIN_CODE_SVG,
            Self::MapPinCog => MAP_PIN_COG_SVG,
            Self::MapPinDollar => MAP_PIN_DOLLAR_SVG,
            Self::MapPinDown => MAP_PIN_DOWN_SVG,
            Self::MapPinExclamation => MAP_PIN_EXCLAMATION_SVG,
            Self::MapPinHeart => MAP_PIN_HEART_SVG,
            Self::MapPinMinus => MAP_PIN_MINUS_SVG,
            Self::MapPinOff => MAP_PIN_OFF_SVG,
            Self::MapPinPause => MAP_PIN_PAUSE_SVG,
            Self::MapPinPin => MAP_PIN_PIN_SVG,
            Self::MapPinPlus => MAP_PIN_PLUS_SVG,
            Self::MapPinQuestion => MAP_PIN_QUESTION_SVG,
            Self::MapPinSearch => MAP_PIN_SEARCH_SVG,
            Self::MapPinShare => MAP_PIN_SHARE_SVG,
            Self::MapPinStar => MAP_PIN_STAR_SVG,
            Self::MapPinUp => MAP_PIN_UP_SVG,
            Self::MapPinX => MAP_PIN_X_SVG,
            Self::MapPins => MAP_PINS_SVG,
            Self::MapPlus => MAP_PLUS_SVG,
            Self::MapQuestion => MAP_QUESTION_SVG,
            Self::MapRoute => MAP_ROUTE_SVG,
            Self::MapSearch => MAP_SEARCH_SVG,
            Self::MapShare => MAP_SHARE_SVG,
            Self::MapShield => MAP_SHIELD_SVG,
            Self::MapSouth => MAP_SOUTH_SVG,
            Self::MapStar => MAP_STAR_SVG,
            Self::MapUp => MAP_UP_SVG,
            Self::MapWest => MAP_WEST_SVG,
            Self::MapX => MAP_X_SVG,
            Self::MasksTheater => MASKS_THEATER_SVG,
            Self::MasksTheaterOff => MASKS_THEATER_OFF_SVG,
            Self::MedicalCross => MEDICAL_CROSS_SVG,
            Self::MedicalCrossCircle => MEDICAL_CROSS_CIRCLE_SVG,
            Self::MedicalCrossOff => MEDICAL_CROSS_OFF_SVG,
            Self::Meteor => METEOR_SVG,
            Self::MeteorOff => METEOR_OFF_SVG,
            Self::Mist => MIST_SVG,
            Self::MistOff => MIST_OFF_SVG,
            Self::Monkeybar => MONKEYBAR_SVG,
            Self::Moon => MOON_SVG,
            Self::Moon2 => MOON_2_SVG,
            Self::MoonOff => MOON_OFF_SVG,
            Self::MoonStars => MOON_STARS_SVG,
            Self::Moped => MOPED_SVG,
            Self::Motorbike => MOTORBIKE_SVG,
            Self::Mountain => MOUNTAIN_SVG,
            Self::MountainOff => MOUNTAIN_OFF_SVG,
            Self::Navigation => NAVIGATION_SVG,
            Self::NavigationBolt => NAVIGATION_BOLT_SVG,
            Self::NavigationCancel => NAVIGATION_CANCEL_SVG,
            Self::NavigationCheck => NAVIGATION_CHECK_SVG,
            Self::NavigationCode => NAVIGATION_CODE_SVG,
            Self::NavigationCog => NAVIGATION_COG_SVG,
            Self::NavigationDiscount => NAVIGATION_DISCOUNT_SVG,
            Self::NavigationDollar => NAVIGATION_DOLLAR_SVG,
            Self::NavigationDown => NAVIGATION_DOWN_SVG,
            Self::NavigationEast => NAVIGATION_EAST_SVG,
            Self::NavigationExclamation => NAVIGATION_EXCLAMATION_SVG,
            Self::NavigationHeart => NAVIGATION_HEART_SVG,
            Self::NavigationMinus => NAVIGATION_MINUS_SVG,
            Self::NavigationNorth => NAVIGATION_NORTH_SVG,
            Self::NavigationOff => NAVIGATION_OFF_SVG,
            Self::NavigationPause => NAVIGATION_PAUSE_SVG,
            Self::NavigationPin => NAVIGATION_PIN_SVG,
            Self::NavigationPlus => NAVIGATION_PLUS_SVG,
            Self::NavigationQuestion => NAVIGATION_QUESTION_SVG,
            Self::NavigationSearch => NAVIGATION_SEARCH_SVG,
            Self::NavigationShare => NAVIGATION_SHARE_SVG,
            Self::NavigationSouth => NAVIGATION_SOUTH_SVG,
            Self::NavigationStar => NAVIGATION_STAR_SVG,
            Self::NavigationTop => NAVIGATION_TOP_SVG,
            Self::NavigationUp => NAVIGATION_UP_SVG,
            Self::NavigationWest => NAVIGATION_WEST_SVG,
            Self::NavigationX => NAVIGATION_X_SVG,
            Self::NorthStar => NORTH_STAR_SVG,
            Self::Parachute => PARACHUTE_SVG,
            Self::ParachuteOff => PARACHUTE_OFF_SVG,
            Self::Parking => PARKING_SVG,
            Self::ParkingMeter => PARKING_METER_SVG,
            Self::ParkingOff => PARKING_OFF_SVG,
            Self::Paw => PAW_SVG,
            Self::PawOff => PAW_OFF_SVG,
            Self::Pennant => PENNANT_SVG,
            Self::Pennant2 => PENNANT_2_SVG,
            Self::PennantOff => PENNANT_OFF_SVG,
            Self::PicnicTable => PICNIC_TABLE_SVG,
            Self::Pig => PIG_SVG,
            Self::PigMoney => PIG_MONEY_SVG,
            Self::PigOff => PIG_OFF_SVG,
            Self::Pin => PIN_SVG,
            Self::Pinned => PINNED_SVG,
            Self::PinnedOff => PINNED_OFF_SVG,
            Self::Plane => PLANE_SVG,
            Self::PlaneArrival => PLANE_ARRIVAL_SVG,
            Self::PlaneDeparture => PLANE_DEPARTURE_SVG,
            Self::PlaneInflight => PLANE_INFLIGHT_SVG,
            Self::PlaneOff => PLANE_OFF_SVG,
            Self::PlaneTilt => PLANE_TILT_SVG,
            Self::Planet => PLANET_SVG,
            Self::PlanetOff => PLANET_OFF_SVG,
            Self::Plant => PLANT_SVG,
            Self::Plant2 => PLANT_2_SVG,
            Self::Plant2Off => PLANT_2_OFF_SVG,
            Self::PlantOff => PLANT_OFF_SVG,
            Self::Pokeball => POKEBALL_SVG,
            Self::PokeballOff => POKEBALL_OFF_SVG,
            Self::Prison => PRISON_SVG,
            Self::Radar => RADAR_SVG,
            Self::Radar2 => RADAR_2_SVG,
            Self::RadarOff => RADAR_OFF_SVG,
            Self::Rainbow => RAINBOW_SVG,
            Self::RainbowOff => RAINBOW_OFF_SVG,
            Self::Ripple => RIPPLE_SVG,
            Self::RippleOff => RIPPLE_OFF_SVG,
            Self::Road => ROAD_SVG,
            Self::RoadOff => ROAD_OFF_SVG,
            Self::RoadSign => ROAD_SIGN_SVG,
            Self::Rocket => ROCKET_SVG,
            Self::RocketOff => ROCKET_OFF_SVG,
            Self::Rollercoaster => ROLLERCOASTER_SVG,
            Self::RollercoasterOff => ROLLERCOASTER_OFF_SVG,
            Self::Route => ROUTE_SVG,
            Self::Route2 => ROUTE_2_SVG,
            Self::RouteAltLeft => ROUTE_ALT_LEFT_SVG,
            Self::RouteAltRight => ROUTE_ALT_RIGHT_SVG,
            Self::RouteOff => ROUTE_OFF_SVG,
            Self::RouteSquare => ROUTE_SQUARE_SVG,
            Self::RouteSquare2 => ROUTE_SQUARE_2_SVG,
            Self::RouteX => ROUTE_X_SVG,
            Self::RouteX2 => ROUTE_X_2_SVG,
            Self::Sailboat => SAILBOAT_SVG,
            Self::Sailboat2 => SAILBOAT_2_SVG,
            Self::SailboatOff => SAILBOAT_OFF_SVG,
            Self::Satellite => SATELLITE_SVG,
            Self::SatelliteOff => SATELLITE_OFF_SVG,
            Self::School => SCHOOL_SVG,
            Self::SchoolOff => SCHOOL_OFF_SVG,
            Self::Scooter => SCOOTER_SVG,
            Self::ScooterElectric => SCOOTER_ELECTRIC_SVG,
            Self::Seedling => SEEDLING_SVG,
            Self::SeedlingOff => SEEDLING_OFF_SVG,
            Self::Segway => SEGWAY_SVG,
            Self::Ship => SHIP_SVG,
            Self::ShipOff => SHIP_OFF_SVG,
            Self::SignLeft => SIGN_LEFT_SVG,
            Self::SignRight => SIGN_RIGHT_SVG,
            Self::Skateboard => SKATEBOARD_SVG,
            Self::SkateboardOff => SKATEBOARD_OFF_SVG,
            Self::Sleigh => SLEIGH_SVG,
            Self::Snowflake => SNOWFLAKE_SVG,
            Self::SnowflakeOff => SNOWFLAKE_OFF_SVG,
            Self::Snowman => SNOWMAN_SVG,
            Self::Speedboat => SPEEDBOAT_SVG,
            Self::Spider => SPIDER_SVG,
            Self::Stairs => STAIRS_SVG,
            Self::StairsDown => STAIRS_DOWN_SVG,
            Self::StairsUp => STAIRS_UP_SVG,
            Self::SteeringWheel => STEERING_WHEEL_SVG,
            Self::SteeringWheelOff => STEERING_WHEEL_OFF_SVG,
            Self::Storm => STORM_SVG,
            Self::StormOff => STORM_OFF_SVG,
            Self::Submarine => SUBMARINE_SVG,
            Self::Sun => SUN_SVG,
            Self::SunElectricity => SUN_ELECTRICITY_SVG,
            Self::SunHigh => SUN_HIGH_SVG,
            Self::SunLow => SUN_LOW_SVG,
            Self::SunMoon => SUN_MOON_SVG,
            Self::SunOff => SUN_OFF_SVG,
            Self::SunWind => SUN_WIND_SVG,
            Self::Sunrise => SUNRISE_SVG,
            Self::Sunset => SUNSET_SVG,
            Self::Sunset2 => SUNSET_2_SVG,
            Self::Tank => TANK_SVG,
            Self::Target => TARGET_SVG,
            Self::TargetOff => TARGET_OFF_SVG,
            Self::Temperature => TEMPERATURE_SVG,
            Self::TemperatureCelsius => TEMPERATURE_CELSIUS_SVG,
            Self::TemperatureFahrenheit => TEMPERATURE_FAHRENHEIT_SVG,
            Self::TemperatureMinus => TEMPERATURE_MINUS_SVG,
            Self::TemperatureOff => TEMPERATURE_OFF_SVG,
            Self::TemperaturePlus => TEMPERATURE_PLUS_SVG,
            Self::Tent => TENT_SVG,
            Self::TentOff => TENT_OFF_SVG,
            Self::Theater => THEATER_SVG,
            Self::Tir => TIR_SVG,
            Self::ToolsKitchen2Off => TOOLS_KITCHEN_2_OFF_SVG,
            Self::ToolsKitchenOff => TOOLS_KITCHEN_OFF_SVG,
            Self::Tornado => TORNADO_SVG,
            Self::Track => TRACK_SVG,
            Self::Tractor => TRACTOR_SVG,
            Self::TrafficCone => TRAFFIC_CONE_SVG,
            Self::TrafficConeOff => TRAFFIC_CONE_OFF_SVG,
            Self::TrafficLights => TRAFFIC_LIGHTS_SVG,
            Self::TrafficLightsOff => TRAFFIC_LIGHTS_OFF_SVG,
            Self::Train => TRAIN_SVG,
            Self::Tree => TREE_SVG,
            Self::Trees => TREES_SVG,
            Self::Trolley => TROLLEY_SVG,
            Self::Truck => TRUCK_SVG,
            Self::TruckOff => TRUCK_OFF_SVG,
            Self::Unicycle => UNICYCLE_SVG,
            Self::UvIndex => UV_INDEX_SVG,
            Self::Viewfinder => VIEWFINDER_SVG,
            Self::ViewfinderOff => VIEWFINDER_OFF_SVG,
            Self::Volcano => VOLCANO_SVG,
            Self::Wheelchair => WHEELCHAIR_SVG,
            Self::WheelchairOff => WHEELCHAIR_OFF_SVG,
            Self::Whirl => WHIRL_SVG,
            Self::Wind => WIND_SVG,
            Self::WindElectricity => WIND_ELECTRICITY_SVG,
            Self::WindOff => WIND_OFF_SVG,
            Self::Windmill => WINDMILL_SVG,
            Self::WindmillOff => WINDMILL_OFF_SVG,
            Self::Windsock => WINDSOCK_SVG,
            Self::Wiper => WIPER_SVG,
            Self::WiperWash => WIPER_WASH_SVG,
            Self::World => WORLD_SVG,
            Self::WorldBolt => WORLD_BOLT_SVG,
            Self::WorldCancel => WORLD_CANCEL_SVG,
            Self::WorldCheck => WORLD_CHECK_SVG,
            Self::WorldCode => WORLD_CODE_SVG,
            Self::WorldCog => WORLD_COG_SVG,
            Self::WorldDollar => WORLD_DOLLAR_SVG,
            Self::WorldDown => WORLD_DOWN_SVG,
            Self::WorldDownload => WORLD_DOWNLOAD_SVG,
            Self::WorldExclamation => WORLD_EXCLAMATION_SVG,
            Self::WorldHeart => WORLD_HEART_SVG,
            Self::WorldLatitude => WORLD_LATITUDE_SVG,
            Self::WorldLongitude => WORLD_LONGITUDE_SVG,
            Self::WorldMap => WORLD_MAP_SVG,
            Self::WorldMinus => WORLD_MINUS_SVG,
            Self::WorldOff => WORLD_OFF_SVG,
            Self::WorldPause => WORLD_PAUSE_SVG,
            Self::WorldPin => WORLD_PIN_SVG,
            Self::WorldPlus => WORLD_PLUS_SVG,
            Self::WorldQuestion => WORLD_QUESTION_SVG,
            Self::WorldSearch => WORLD_SEARCH_SVG,
            Self::WorldShare => WORLD_SHARE_SVG,
            Self::WorldStar => WORLD_STAR_SVG,
            Self::WorldUp => WORLD_UP_SVG,
            Self::WorldUpload => WORLD_UPLOAD_SVG,
            Self::WorldWww => WORLD_WWW_SVG,
            Self::WorldX => WORLD_X_SVG,
            Self::WreckingBall => WRECKING_BALL_SVG,
            Self::Zeppelin => ZEPPELIN_SVG,
            Self::ZeppelinOff => ZEPPELIN_OFF_SVG,
            Self::ZodiacAquarius => ZODIAC_AQUARIUS_SVG,
            Self::ZodiacAries => ZODIAC_ARIES_SVG,
            Self::ZodiacCancer => ZODIAC_CANCER_SVG,
            Self::ZodiacCapricorn => ZODIAC_CAPRICORN_SVG,
            Self::ZodiacGemini => ZODIAC_GEMINI_SVG,
            Self::ZodiacLeo => ZODIAC_LEO_SVG,
            Self::ZodiacLibra => ZODIAC_LIBRA_SVG,
            Self::ZodiacPisces => ZODIAC_PISCES_SVG,
            Self::ZodiacSagittarius => ZODIAC_SAGITTARIUS_SVG,
            Self::ZodiacScorpio => ZODIAC_SCORPIO_SVG,
            Self::ZodiacTaurus => ZODIAC_TAURUS_SVG,
            Self::ZodiacVirgo => ZODIAC_VIRGO_SVG,
            Self::Zoom => ZOOM_SVG,
            Self::ZoomCancel => ZOOM_CANCEL_SVG,
            Self::ZoomCheck => ZOOM_CHECK_SVG,
            Self::ZoomCode => ZOOM_CODE_SVG,
            Self::ZoomExclamation => ZOOM_EXCLAMATION_SVG,
            Self::ZoomIn => ZOOM_IN_SVG,
            Self::ZoomInArea => ZOOM_IN_AREA_SVG,
            Self::ZoomMoney => ZOOM_MONEY_SVG,
            Self::ZoomOut => ZOOM_OUT_SVG,
            Self::ZoomOutArea => ZOOM_OUT_AREA_SVG,
            Self::ZoomPan => ZOOM_PAN_SVG,
            Self::ZoomQuestion => ZOOM_QUESTION_SVG,
            Self::ZoomReplace => ZOOM_REPLACE_SVG,
            Self::ZoomReset => ZOOM_RESET_SVG,
        }
    }

    fn filled_svg(&self) -> Option<&'static str> {
        // Filled variants would be added here
        None
    }
}
