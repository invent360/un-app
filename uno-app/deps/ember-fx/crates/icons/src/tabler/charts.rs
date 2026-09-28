//! Charts icons from Tabler Icons.
//!
//! This module contains 39 icons.

use crate::tabler::TablerIconData;

// SVG Constants
const CHART_ARCS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M16.924 11.132a5 5 0 1 0 -4.056 5.792" /> <path d="M3 12a9 9 0 1 0 9 -9" /> </svg>"##;
const CHART_ARCS_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 12a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M7 12a5 5 0 1 0 5 -5" /> <path d="M6.29 18.957a9 9 0 1 0 5.71 -15.957" /> </svg>"##;
const CHART_AREA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19l16 0" /> <path d="M4 15l4 -6l4 2l4 -5l4 4l0 5l-16 0" /> </svg>"##;
const CHART_AREA_LINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19l4 -6l4 2l4 -5l4 4l0 5l-16 0" /> <path d="M4 12l3 -4l4 2l5 -6l4 4" /> </svg>"##;
const CHART_ARROWS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 18l14 0" /> <path d="M9 9l3 3l-3 3" /> <path d="M14 15l3 3l-3 3" /> <path d="M3 3l0 18" /> <path d="M3 12l9 0" /> <path d="M18 3l3 3l-3 3" /> <path d="M3 6l18 0" /> </svg>"##;
const CHART_ARROWS_VERTICAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 21v-14" /> <path d="M9 15l3 -3l3 3" /> <path d="M15 10l3 -3l3 3" /> <path d="M3 21l18 0" /> <path d="M12 21l0 -9" /> <path d="M3 6l3 -3l3 3" /> <path d="M6 21v-18" /> </svg>"##;
const CHART_BAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 13a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -6" /> <path d="M15 9a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -10" /> <path d="M9 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -14" /> <path d="M4 20h14" /> </svg>"##;
const CHART_BAR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 13a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -6" /> <path d="M12 8h2a1 1 0 0 1 1 1v2m0 4v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1v-10" /> <path d="M15 11v-6a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v12m-1 3h-4a1 1 0 0 1 -1 -1v-4" /> <path d="M4 20h14" /> <path d="M3 3l18 18" /> </svg>"##;
const CHART_BAR_POPULAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 13a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -6" /> <path d="M9 9a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -10" /> <path d="M15 5a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v14a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1l0 -14" /> <path d="M4 20h14" /> </svg>"##;
const CHART_BUBBLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 16a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M14 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 7.5a4.5 4.5 0 1 0 9 0a4.5 4.5 0 1 0 -9 0" /> </svg>"##;
const CHART_CANDLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 7a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -3" /> <path d="M6 4l0 2" /> <path d="M6 11l0 9" /> <path d="M10 15a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v3a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -3" /> <path d="M12 4l0 10" /> <path d="M12 19l0 1" /> <path d="M16 6a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1l0 -4" /> <path d="M18 4l0 1" /> <path d="M18 11l0 9" /> </svg>"##;
const CHART_CIRCLES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 9.5a5.5 5.5 0 1 0 11 0a5.5 5.5 0 1 0 -11 0" /> <path d="M9 14.5a5.5 5.5 0 1 0 11 0a5.5 5.5 0 1 0 -11 0" /> </svg>"##;
const CHART_COHORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9h18v-6h-18v18h6v-18" /> <path d="M3 15h12v-12" /> </svg>"##;
const CHART_COLUMN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 20h3" /> <path d="M17 20h3" /> <path d="M10.5 20h3" /> <path d="M4 16h3" /> <path d="M17 16h3" /> <path d="M10.5 16h3" /> <path d="M4 12h3" /> <path d="M17 12h3" /> <path d="M10.5 12h3" /> <path d="M4 8h3" /> <path d="M17 8h3" /> <path d="M4 4h3" /> </svg>"##;
const CHART_COVARIATE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 11h.009" /> <path d="M14 15h.009" /> <path d="M12 6h.009" /> <path d="M8 10h.009" /> <path d="M3 21l17 -17" /> <path d="M3 3v18h18" /> </svg>"##;
const CHART_DONUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 3.2a9 9 0 1 0 10.8 10.8a1 1 0 0 0 -1 -1h-3.8a4.1 4.1 0 1 1 -5 -5v-4a.9 .9 0 0 0 -1 -.8" /> <path d="M15 3.5a9 9 0 0 1 5.5 5.5h-4.5a9 9 0 0 0 -1 -1v-4.5" /> </svg>"##;
const CHART_DONUT_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3v5m4 4h5" /> <path d="M8 12a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CHART_DONUT_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3v5m4 4h5" /> <path d="M8.929 14.582l-3.429 2.918" /> <path d="M8 12a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CHART_DONUT_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.848 14.667l-3.348 2.833" /> <path d="M12 3v5m4 4h5" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14.219 15.328l2.781 4.172" /> <path d="M8 12a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> </svg>"##;
const CHART_DOTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3v18h18" /> <path d="M7 9a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 7a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M12 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10.16 10.62l2.34 2.88" /> <path d="M15.088 13.328l2.837 -4.586" /> </svg>"##;
const CHART_DOTS_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3v18h18" /> <path d="M7 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M11 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M21 3l-6 1.5" /> <path d="M14.113 6.65l2.771 3.695" /> <path d="M16 12.5l-5 2" /> </svg>"##;
const CHART_DOTS_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M14 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 6a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M3 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M9 17l5 -1.5" /> <path d="M6.5 8.5l7.81 5.37" /> <path d="M7 7l8 -1" /> </svg>"##;
const CHART_FUNNEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.387 3h15.226a1 1 0 0 1 .948 1.316l-5.105 15.316a2 2 0 0 1 -1.898 1.368h-3.116a2 2 0 0 1 -1.898 -1.368l-5.104 -15.316a1 1 0 0 1 .947 -1.316" /> <path d="M5 9h14" /> <path d="M7 15h10" /> </svg>"##;
const CHART_GRID_DOTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 6a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 12a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M4 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 18a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M8 18h8" /> <path d="M18 20v1" /> <path d="M18 3v1" /> <path d="M6 20v1" /> <path d="M6 10v-7" /> <path d="M12 3v18" /> <path d="M18 8v8" /> <path d="M8 12h13" /> <path d="M21 6h-1" /> <path d="M16 6h-13" /> <path d="M3 12h1" /> <path d="M20 18h1" /> <path d="M3 18h1" /> <path d="M6 14v2" /> </svg>"##;
const CHART_HISTOGRAM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3v18h18" /> <path d="M20 18v3" /> <path d="M16 16v5" /> <path d="M12 13v8" /> <path d="M8 16v5" /> <path d="M3 11c6 0 5 -5 9 -5s3 5 9 5" /> </svg>"##;
const CHART_INFOGRAPHIC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /> <path d="M7 3v4h4" /> <path d="M9 17l0 4" /> <path d="M17 14l0 7" /> <path d="M13 13l0 8" /> <path d="M21 12l0 9" /> </svg>"##;
const CHART_LINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19l16 0" /> <path d="M4 15l4 -6l4 2l4 -5l4 4" /> </svg>"##;
const CHART_PIE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 3.2a9 9 0 1 0 10.8 10.8a1 1 0 0 0 -1 -1h-6.8a2 2 0 0 1 -2 -2v-7a.9 .9 0 0 0 -1 -.8" /> <path d="M15 3.5a9 9 0 0 1 5.5 5.5h-4.5a1 1 0 0 1 -1 -1v-4.5" /> </svg>"##;
const CHART_PIE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3v9h9" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CHART_PIE_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12l-6.5 5.5" /> <path d="M12 3v9h9" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const CHART_PIE_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 12l-6.5 5.5" /> <path d="M12 3v9h9" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M12 12l5 7.5" /> </svg>"##;
const CHART_PIE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.63 5.643a9 9 0 0 0 12.742 12.715m1.674 -2.29a9.03 9.03 0 0 0 .754 -2.068a1 1 0 0 0 -1 -1h-2.8m-4 0a2 2 0 0 1 -2 -2m0 -4v-3a.9 .9 0 0 0 -1 -.8a9 9 0 0 0 -2.057 .749" /> <path d="M15 3.5a9 9 0 0 1 5.5 5.5h-4.5a1 1 0 0 1 -1 -1v-4.5" /> <path d="M3 3l18 18" /> </svg>"##;
const CHART_PPF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 17c0 -6.075 -5.373 -11 -12 -11" /> <path d="M3 3v18h18" /> </svg>"##;
const CHART_RADAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3l9.5 7l-3.5 11h-12l-3.5 -11l9.5 -7" /> <path d="M12 7.5l5.5 4l-2.5 5.5h-6.5l-2 -5.5l5.5 -4" /> <path d="M2.5 10l9.5 3l9.5 -3" /> <path d="M12 3v10l6 8" /> <path d="M6 21l6 -8" /> </svg>"##;
const CHART_SANKEY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6c6.944 0 9.056 8 16 8" /> <path d="M4 12c6.37 0 9.63 6 16 6" /> <path d="M20 6c-7.526 0 -7.905 12 -16 12" /> </svg>"##;
const CHART_SCATTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3v18h18" /> <path d="M8 15.015v.015" /> <path d="M16 16.015v.015" /> <path d="M8 7.03v.015" /> <path d="M12 11.03v.015" /> <path d="M19 11.03v.015" /> </svg>"##;
const CHART_SCATTER_3D_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 20l9 -7" /> <path d="M12 3v10l9 7" /> <path d="M17 12v.015" /> <path d="M17 4.015v.015" /> <path d="M21 8.015v.015" /> <path d="M12 19.015v.015" /> <path d="M3 12.015v.015" /> <path d="M7 8.015v.015" /> <path d="M3 4.015v.015" /> </svg>"##;
const CHART_TREEMAP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2l0 -12" /> <path d="M12 4v16" /> <path d="M4 15h8" /> <path d="M12 12h8" /> <path d="M16 12v8" /> <path d="M16 16h4" /> </svg>"##;
const TRENDING_UP_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 14l6 -6l4 4l9 -9" /> <path d="M15 3h6v6" /> <path d="M15 21h6v-6" /> <path d="M21 21l-6 -6" /> </svg>"##;

/// Charts icon variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChartsIcon {
    ChartArcs,
    ChartArcs3,
    ChartArea,
    ChartAreaLine,
    ChartArrows,
    ChartArrowsVertical,
    ChartBar,
    ChartBarOff,
    ChartBarPopular,
    ChartBubble,
    ChartCandle,
    ChartCircles,
    ChartCohort,
    ChartColumn,
    ChartCovariate,
    ChartDonut,
    ChartDonut2,
    ChartDonut3,
    ChartDonut4,
    ChartDots,
    ChartDots2,
    ChartDots3,
    ChartFunnel,
    ChartGridDots,
    ChartHistogram,
    ChartInfographic,
    ChartLine,
    ChartPie,
    ChartPie2,
    ChartPie3,
    ChartPie4,
    ChartPieOff,
    ChartPpf,
    ChartRadar,
    ChartSankey,
    ChartScatter,
    ChartScatter3d,
    ChartTreemap,
    TrendingUpDown,
}

impl ChartsIcon {
    /// Returns all available icons in this category.
    pub fn all() -> &'static [Self] {
        &[Self::ChartArcs, Self::ChartArcs3, Self::ChartArea, Self::ChartAreaLine, Self::ChartArrows, Self::ChartArrowsVertical, Self::ChartBar, Self::ChartBarOff, Self::ChartBarPopular, Self::ChartBubble, Self::ChartCandle, Self::ChartCircles, Self::ChartCohort, Self::ChartColumn, Self::ChartCovariate, Self::ChartDonut, Self::ChartDonut2, Self::ChartDonut3, Self::ChartDonut4, Self::ChartDots, Self::ChartDots2, Self::ChartDots3, Self::ChartFunnel, Self::ChartGridDots, Self::ChartHistogram, Self::ChartInfographic, Self::ChartLine, Self::ChartPie, Self::ChartPie2, Self::ChartPie3, Self::ChartPie4, Self::ChartPieOff, Self::ChartPpf, Self::ChartRadar, Self::ChartSankey, Self::ChartScatter, Self::ChartScatter3d, Self::ChartTreemap, Self::TrendingUpDown]
    }

    /// Returns the icon count.
    pub fn count() -> usize {
        39
    }

    /// Creates an icon from its kebab-case name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "chart-arcs" => Some(Self::ChartArcs),
            "chart-arcs-3" => Some(Self::ChartArcs3),
            "chart-area" => Some(Self::ChartArea),
            "chart-area-line" => Some(Self::ChartAreaLine),
            "chart-arrows" => Some(Self::ChartArrows),
            "chart-arrows-vertical" => Some(Self::ChartArrowsVertical),
            "chart-bar" => Some(Self::ChartBar),
            "chart-bar-off" => Some(Self::ChartBarOff),
            "chart-bar-popular" => Some(Self::ChartBarPopular),
            "chart-bubble" => Some(Self::ChartBubble),
            "chart-candle" => Some(Self::ChartCandle),
            "chart-circles" => Some(Self::ChartCircles),
            "chart-cohort" => Some(Self::ChartCohort),
            "chart-column" => Some(Self::ChartColumn),
            "chart-covariate" => Some(Self::ChartCovariate),
            "chart-donut" => Some(Self::ChartDonut),
            "chart-donut-2" => Some(Self::ChartDonut2),
            "chart-donut-3" => Some(Self::ChartDonut3),
            "chart-donut-4" => Some(Self::ChartDonut4),
            "chart-dots" => Some(Self::ChartDots),
            "chart-dots-2" => Some(Self::ChartDots2),
            "chart-dots-3" => Some(Self::ChartDots3),
            "chart-funnel" => Some(Self::ChartFunnel),
            "chart-grid-dots" => Some(Self::ChartGridDots),
            "chart-histogram" => Some(Self::ChartHistogram),
            "chart-infographic" => Some(Self::ChartInfographic),
            "chart-line" => Some(Self::ChartLine),
            "chart-pie" => Some(Self::ChartPie),
            "chart-pie-2" => Some(Self::ChartPie2),
            "chart-pie-3" => Some(Self::ChartPie3),
            "chart-pie-4" => Some(Self::ChartPie4),
            "chart-pie-off" => Some(Self::ChartPieOff),
            "chart-ppf" => Some(Self::ChartPpf),
            "chart-radar" => Some(Self::ChartRadar),
            "chart-sankey" => Some(Self::ChartSankey),
            "chart-scatter" => Some(Self::ChartScatter),
            "chart-scatter-3d" => Some(Self::ChartScatter3d),
            "chart-treemap" => Some(Self::ChartTreemap),
            "trending-up-down" => Some(Self::TrendingUpDown),
            _ => None,
        }
    }
}

impl TablerIconData for ChartsIcon {
    fn name(&self) -> &'static str {
        match self {
            Self::ChartArcs => "chart-arcs",
            Self::ChartArcs3 => "chart-arcs-3",
            Self::ChartArea => "chart-area",
            Self::ChartAreaLine => "chart-area-line",
            Self::ChartArrows => "chart-arrows",
            Self::ChartArrowsVertical => "chart-arrows-vertical",
            Self::ChartBar => "chart-bar",
            Self::ChartBarOff => "chart-bar-off",
            Self::ChartBarPopular => "chart-bar-popular",
            Self::ChartBubble => "chart-bubble",
            Self::ChartCandle => "chart-candle",
            Self::ChartCircles => "chart-circles",
            Self::ChartCohort => "chart-cohort",
            Self::ChartColumn => "chart-column",
            Self::ChartCovariate => "chart-covariate",
            Self::ChartDonut => "chart-donut",
            Self::ChartDonut2 => "chart-donut-2",
            Self::ChartDonut3 => "chart-donut-3",
            Self::ChartDonut4 => "chart-donut-4",
            Self::ChartDots => "chart-dots",
            Self::ChartDots2 => "chart-dots-2",
            Self::ChartDots3 => "chart-dots-3",
            Self::ChartFunnel => "chart-funnel",
            Self::ChartGridDots => "chart-grid-dots",
            Self::ChartHistogram => "chart-histogram",
            Self::ChartInfographic => "chart-infographic",
            Self::ChartLine => "chart-line",
            Self::ChartPie => "chart-pie",
            Self::ChartPie2 => "chart-pie-2",
            Self::ChartPie3 => "chart-pie-3",
            Self::ChartPie4 => "chart-pie-4",
            Self::ChartPieOff => "chart-pie-off",
            Self::ChartPpf => "chart-ppf",
            Self::ChartRadar => "chart-radar",
            Self::ChartSankey => "chart-sankey",
            Self::ChartScatter => "chart-scatter",
            Self::ChartScatter3d => "chart-scatter-3d",
            Self::ChartTreemap => "chart-treemap",
            Self::TrendingUpDown => "trending-up-down",
        }
    }

    fn outline_svg(&self) -> &'static str {
        match self {
            Self::ChartArcs => CHART_ARCS_SVG,
            Self::ChartArcs3 => CHART_ARCS_3_SVG,
            Self::ChartArea => CHART_AREA_SVG,
            Self::ChartAreaLine => CHART_AREA_LINE_SVG,
            Self::ChartArrows => CHART_ARROWS_SVG,
            Self::ChartArrowsVertical => CHART_ARROWS_VERTICAL_SVG,
            Self::ChartBar => CHART_BAR_SVG,
            Self::ChartBarOff => CHART_BAR_OFF_SVG,
            Self::ChartBarPopular => CHART_BAR_POPULAR_SVG,
            Self::ChartBubble => CHART_BUBBLE_SVG,
            Self::ChartCandle => CHART_CANDLE_SVG,
            Self::ChartCircles => CHART_CIRCLES_SVG,
            Self::ChartCohort => CHART_COHORT_SVG,
            Self::ChartColumn => CHART_COLUMN_SVG,
            Self::ChartCovariate => CHART_COVARIATE_SVG,
            Self::ChartDonut => CHART_DONUT_SVG,
            Self::ChartDonut2 => CHART_DONUT_2_SVG,
            Self::ChartDonut3 => CHART_DONUT_3_SVG,
            Self::ChartDonut4 => CHART_DONUT_4_SVG,
            Self::ChartDots => CHART_DOTS_SVG,
            Self::ChartDots2 => CHART_DOTS_2_SVG,
            Self::ChartDots3 => CHART_DOTS_3_SVG,
            Self::ChartFunnel => CHART_FUNNEL_SVG,
            Self::ChartGridDots => CHART_GRID_DOTS_SVG,
            Self::ChartHistogram => CHART_HISTOGRAM_SVG,
            Self::ChartInfographic => CHART_INFOGRAPHIC_SVG,
            Self::ChartLine => CHART_LINE_SVG,
            Self::ChartPie => CHART_PIE_SVG,
            Self::ChartPie2 => CHART_PIE_2_SVG,
            Self::ChartPie3 => CHART_PIE_3_SVG,
            Self::ChartPie4 => CHART_PIE_4_SVG,
            Self::ChartPieOff => CHART_PIE_OFF_SVG,
            Self::ChartPpf => CHART_PPF_SVG,
            Self::ChartRadar => CHART_RADAR_SVG,
            Self::ChartSankey => CHART_SANKEY_SVG,
            Self::ChartScatter => CHART_SCATTER_SVG,
            Self::ChartScatter3d => CHART_SCATTER_3D_SVG,
            Self::ChartTreemap => CHART_TREEMAP_SVG,
            Self::TrendingUpDown => TRENDING_UP_DOWN_SVG,
        }
    }

    fn filled_svg(&self) -> Option<&'static str> {
        // Filled variants would be added here
        None
    }
}
