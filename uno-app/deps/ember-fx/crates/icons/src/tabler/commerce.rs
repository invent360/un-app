//! Commerce icons from Tabler Icons.
//!
//! This module contains 362 icons.

use crate::tabler::TablerIconData;

// SVG Constants
const APPLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 11.319c0 3.102 .444 5.319 2.222 7.978c1.351 1.797 3.156 2.247 5.08 .988c.426 -.268 .97 -.268 1.397 0c1.923 1.26 3.728 .809 5.079 -.988c1.778 -2.66 2.222 -4.876 2.222 -7.977c0 -2.661 -1.99 -5.32 -4.444 -5.32c-1.267 0 -2.41 .693 -3.22 1.44a.5 .5 0 0 1 -.672 0c-.809 -.746 -1.953 -1.44 -3.22 -1.44c-2.454 0 -4.444 2.66 -4.444 5.319" /> <path d="M7 12c0 -1.47 .454 -2.34 1.5 -3" /> <path d="M12 7c0 -1.2 .867 -4 3 -4" /> </svg>"##;
const AVOCADO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.8 14.04a3.905 3.905 0 0 1 1.337 -2.075c1.195 -.985 1.816 -2.285 1.863 -3.902c-.047 -1.43 -.54 -2.626 -1.477 -3.586c-.96 -.938 -2.156 -1.43 -3.585 -1.477c-1.618 .047 -2.918 .668 -3.903 1.863c-.562 .68 -1.254 1.125 -2.074 1.336c-.938 .188 -1.828 .48 -2.672 .88c-.844 .398 -1.559 .878 -2.144 1.44c-1.43 1.501 -2.145 3.224 -2.145 5.169c0 1.946 .715 3.668 2.145 5.168c1.5 1.429 3.222 2.144 5.168 2.144c1.945 0 3.667 -.715 5.167 -2.145c.563 -.585 1.055 -1.3 1.477 -2.144c.398 -.844 .68 -1.723 .844 -2.637v-.035l-.001 .001" /> <path d="M10.87 10.036c-.942 .112 -1.794 .538 -2.556 1.278c-.74 .762 -1.166 1.614 -1.278 2.556c-.135 .92 .112 1.704 .74 2.354c.65 .628 1.435 .875 2.354 .74c.942 -.112 1.794 -.538 2.556 -1.278c.74 -.762 1.166 -1.614 1.278 -2.556c.135 -.92 -.112 -1.704 -.74 -2.354c-.65 -.628 -1.435 -.875 -2.354 -.74" /> </svg>"##;
const BACKPACK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 18v-6a6 6 0 0 1 6 -6h2a6 6 0 0 1 6 6v6a3 3 0 0 1 -3 3h-8a3 3 0 0 1 -3 -3" /> <path d="M10 6v-1a2 2 0 1 1 4 0v1" /> <path d="M9 21v-4a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v4" /> <path d="M11 10h2" /> </svg>"##;
const BACKPACK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 6h3a6 6 0 0 1 6 6v3m-.129 3.872a3 3 0 0 1 -2.871 2.128h-8a3 3 0 0 1 -3 -3v-6a5.99 5.99 0 0 1 2.285 -4.712" /> <path d="M10 6v-1a2 2 0 1 1 4 0v1" /> <path d="M9 21v-4a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v4" /> <path d="M3 3l18 18" /> </svg>"##;
const BAGUETTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.628 11.283l5.644 -5.637c2.665 -2.663 5.924 -3.747 8.663 -1.205l.188 .181a2.987 2.987 0 0 1 0 4.228l-11.287 11.274a3 3 0 0 1 -4.089 .135l-.143 -.135c-2.728 -2.724 -1.704 -6.117 1.024 -8.841" /> <path d="M9.5 7.5l1.5 3.5" /> <path d="M6.5 10.5l1.5 3.5" /> <path d="M12.5 4.5l1.5 3.5" /> </svg>"##;
const BANANA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 6v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2a9.09 9.09 0 0 1 -4 8.08c-2 1.31 -5 1.57 -7 1.59a2 2 0 0 0 -2 2a2 2 0 0 0 1.16 1.81c2.69 1.2 9.46 3.44 14.35 -1.66c4.49 -4.74 1.49 -11.82 1.49 -11.82" /> </svg>"##;
const BASKET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M5.001 8h13.999a2 2 0 0 1 1.977 2.304l-1.255 7.152a3 3 0 0 1 -2.966 2.544h-9.512a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304" /> <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> </svg>"##;
const BASKET_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M13 20h-5.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.358 2.04" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const BASKET_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M12 20h-4.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.3 1.713" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const BASKET_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M11 20h-3.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.479 2.729" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const BASKET_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M11 20h-3.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304c-.21 1.202 -.37 2.104 -.475 2.705" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const BASKET_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M12 20h-4.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.267 1.522" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const BASKET_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M12.5 20h-5.256a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.394 2.248" /> <path d="M13.856 13.254a2 2 0 1 0 -1.856 2.746" /> <path d="M16 21l5 -5" /> <path d="M21 21v.01" /> <path d="M16 16v.01" /> </svg>"##;
const BASKET_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M13 20h-5.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const BASKET_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M12 20h-4.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.349 1.989" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const BASKET_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M15 20h-7.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.36 2.055" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const BASKET_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M10.5 20h-3.256a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.143 .817" /> <path d="M12.602 12.092a2 2 0 0 0 -2.233 3.066" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const BASKET_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M12 20h-4.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.833 4.75" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M16 19h6" /> </svg>"##;
const BASKET_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l.75 -2.252m1.001 -3.002l.249 -.746" /> <path d="M12 8h7a2 2 0 0 1 1.977 2.304c-.442 2.516 -.756 4.438 -.977 5.696m-1.01 3.003a2.997 2.997 0 0 1 -2.234 .997h-9.512a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h2.999" /> <path d="M12 12a2 2 0 1 0 2 2" /> <path d="M3 3l18 18" /> </svg>"##;
const BASKET_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M13 20h-5.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.478 2.725" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const BASKET_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M12 20h-4.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.161 .92" /> <path d="M13.866 13.28a2 2 0 1 0 -1.866 2.72" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const BASKET_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M12 20h-4.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.359 2.043" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const BASKET_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M15 20h-7.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.161 .918" /> <path d="M12 16a2 2 0 1 0 0 -4a2 2 0 0 0 0 4" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const BASKET_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M11 20h-3.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.215 1.227" /> <path d="M13.483 12.658a2 2 0 1 0 -2.162 3.224" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const BASKET_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M12.5 20h-5.256a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.478 2.723" /> <path d="M14 14a2 2 0 1 0 -2 2" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const BASKET_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M10.5 20h-3.256a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.133 .757" /> <path d="M13.596 12.794a2 2 0 0 0 -3.377 2.116" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const BASKET_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M12 20h-4.756a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.358 2.04" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const BASKET_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10l-2 -6" /> <path d="M7 10l2 -6" /> <path d="M13.5 20h-6.256a3 3 0 0 1 -2.965 -2.544l-1.255 -7.152a2 2 0 0 1 1.977 -2.304h13.999a2 2 0 0 1 1.977 2.304l-.532 3.03" /> <path d="M10 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const BEER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 21h6a1 1 0 0 0 1 -1v-3.625c0 -1.397 .29 -2.775 .845 -4.025l.31 -.7c.556 -1.25 .845 -2.253 .845 -3.65v-4a1 1 0 0 0 -1 -1h-10a1 1 0 0 0 -1 1v4c0 1.397 .29 2.4 .845 3.65l.31 .7a9.931 9.931 0 0 1 .845 4.025v3.625a1 1 0 0 0 1 1" /> <path d="M6 8h12" /> </svg>"##;
const BEER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 7v1.111c0 1.242 .29 2.467 .845 3.578l.31 .622a8 8 0 0 1 .845 3.578v4.111h6v-4.111a8 8 0 0 1 .045 -.85m.953 -3.035l.157 -.315a8 8 0 0 0 .845 -3.578v-4.111h-9" /> <path d="M7 8h1m4 0h5" /> <path d="M3 3l18 18" /> </svg>"##;
const BLEACH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 19h14a2 2 0 0 0 1.84 -2.75l-7.1 -12.25a2 2 0 0 0 -3.5 0l-7.1 12.25a2 2 0 0 0 1.75 2.75" /> </svg>"##;
const BLEACH_CHLORINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 19h14a2 2 0 0 0 1.84 -2.75l-7.1 -12.25a2 2 0 0 0 -3.5 0l-7.1 12.25a2 2 0 0 0 1.75 2.75" /> <path d="M11 12h-1a2 2 0 1 0 0 4h1" /> <path d="M14 12v4h2" /> </svg>"##;
const BLEACH_NO_CHLORINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 19h14a2 2 0 0 0 1.84 -2.75l-7.1 -12.25a2 2 0 0 0 -3.5 0l-7.1 12.25a2 2 0 0 0 1.75 2.75" /> <path d="M6.576 19l7.907 -13.733" /> <path d="M11.719 19.014l5.346 -9.284" /> </svg>"##;
const BLEACH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 19h14m1.986 -1.977a2 2 0 0 0 -.146 -.773l-7.1 -12.25a2 2 0 0 0 -3.5 0l-.815 1.405m-1.488 2.568l-4.797 8.277a2 2 0 0 0 1.75 2.75" /> <path d="M3 3l18 18" /> </svg>"##;
const BONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 3a3 3 0 0 1 3 3a3 3 0 1 1 -2.12 5.122l-4.758 4.758a3 3 0 1 1 -5.117 2.297l0 -.177l-.176 0a3 3 0 1 1 2.298 -5.115l4.758 -4.758a3 3 0 0 1 2.12 -5.122l-.005 -.005" /> </svg>"##;
const BONE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 8.502l.38 -.38a3 3 0 1 1 5.12 -2.122a3 3 0 1 1 -2.12 5.122l-.372 .372m-2.008 2.008l-2.378 2.378a3 3 0 1 1 -5.117 2.297l0 -.177l-.176 0a3 3 0 1 1 2.298 -5.115l2.378 -2.378" /> <path d="M3 3l18 18" /> </svg>"##;
const BOTTLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5h4v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2" /> <path d="M14 3.5c0 1.626 .507 3.212 1.45 4.537l.05 .07a8.093 8.093 0 0 1 1.5 4.694v6.199a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2v-6.2c0 -1.682 .524 -3.322 1.5 -4.693l.05 -.07a7.823 7.823 0 0 0 1.45 -4.537" /> <path d="M7 14.803a2.4 2.4 0 0 0 1 -.803a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 1 -.805" /> </svg>"##;
const BOTTLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5h4v-2a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1v2" /> <path d="M14 3.5c0 1.626 .507 3.212 1.45 4.537l.05 .07a8.093 8.093 0 0 1 1.5 4.694v.199m0 4v2a2 2 0 0 1 -2 2h-6a2 2 0 0 1 -2 -2v-6.2a8.09 8.09 0 0 1 1.35 -4.474m1.336 -2.63a7.822 7.822 0 0 0 .314 -2.196" /> <path d="M7 14.803a2.4 2.4 0 0 0 1 -.803a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 .866 -.142" /> <path d="M3 3l18 18" /> </svg>"##;
const BOWL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 8h16a1 1 0 0 1 1 1v.5c0 1.5 -2.517 5.573 -4 6.5v1a1 1 0 0 1 -1 1h-8a1 1 0 0 1 -1 -1v-1c-1.687 -1.054 -4 -5 -4 -6.5v-.5a1 1 0 0 1 1 -1" /> </svg>"##;
const BOWL_CHOPSTICKS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 11h16a1 1 0 0 1 1 1v.5c0 1.5 -2.517 5.573 -4 6.5v1a1 1 0 0 1 -1 1h-8a1 1 0 0 1 -1 -1v-1c-1.687 -1.054 -4 -5 -4 -6.5v-.5a1 1 0 0 1 1 -1" /> <path d="M19 7l-14 1" /> <path d="M19 2l-14 3" /> </svg>"##;
const BOWL_SPOON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 11h16a1 1 0 0 1 1 1v.5c0 1.5 -2.517 5.573 -4 6.5v1a1 1 0 0 1 -1 1h-8a1 1 0 0 1 -1 -1v-1c-1.687 -1.054 -4 -5 -4 -6.5v-.5a1 1 0 0 1 1 -1" /> <path d="M8 7c1.657 0 3 -.895 3 -2s-1.343 -2 -3 -2s-3 .895 -3 2s1.343 2 3 2" /> <path d="M11 5h9" /> </svg>"##;
const BREAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 4a3 3 0 0 1 2 5.235v8.765a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-8.764a3 3 0 0 1 1.824 -5.231h12.176v-.005" /> </svg>"##;
const BREAD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 4l10 .005v-.005a3 3 0 0 1 2 5.235v6.765m-.59 3.418c-.36 .36 -.86 .582 -1.41 .582h-12a2 2 0 0 1 -2 -2v-8.764a3 3 0 0 1 .418 -4.785" /> <path d="M3 3l18 18" /> </svg>"##;
const BUBBLE_TEA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.95 9l-1.478 8.69c-.25 1.463 -.374 2.195 -.936 2.631c-1.2 .931 -6.039 .88 -7.172 0c-.562 -.436 -.687 -1.168 -.936 -2.632l-1.478 -8.689" /> <path d="M6 9l.514 -1.286a5.908 5.908 0 0 1 10.972 0l.514 1.286" /> <path d="M5 9h14" /> <path d="M12 9l4 -7" /> <path d="M10.01 14h.01" /> <path d="M11.02 18h.01" /> <path d="M13.02 16h.01" /> </svg>"##;
const BUBBLE_TEA_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.95 9l-1.478 8.69c-.25 1.463 -.374 2.195 -.936 2.631c-1.2 .931 -6.039 .88 -7.172 0c-.562 -.436 -.687 -1.168 -.936 -2.632l-1.478 -8.689" /> <path d="M6 9l.514 -1.286a5.908 5.908 0 0 1 10.972 0l.514 1.286" /> <path d="M5 9h14" /> <path d="M12 9l4 -7" /> <path d="M7 14c.593 .642 1.484 1.017 2.5 1c1.016 .017 1.907 -.358 2.5 -1s1.484 -1.017 2.5 -1c1.016 -.017 1.907 .358 2.5 1" /> </svg>"##;
const BURGER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 15h16a4 4 0 0 1 -4 4h-8a4 4 0 0 1 -4 -4" /> <path d="M12 4c3.783 0 6.953 2.133 7.786 5h-15.572c.833 -2.867 4.003 -5 7.786 -5" /> <path d="M5 12h14" /> </svg>"##;
const BUSINESSPLAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 6a5 3 0 1 0 10 0a5 3 0 1 0 -10 0" /> <path d="M11 6v4c0 1.657 2.239 3 5 3s5 -1.343 5 -3v-4" /> <path d="M11 10v4c0 1.657 2.239 3 5 3s5 -1.343 5 -3v-4" /> <path d="M11 14v4c0 1.657 2.239 3 5 3s5 -1.343 5 -3v-4" /> <path d="M7 9h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M5 15v1m0 -8v1" /> </svg>"##;
const CAKE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 20h18v-8a3 3 0 0 0 -3 -3h-12a3 3 0 0 0 -3 3v8" /> <path d="M3 14.803c.312 .135 .654 .204 1 .197a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1c.35 .007 .692 -.062 1 -.197" /> <path d="M12 4l1.465 1.638a2 2 0 1 1 -3.015 .099l1.55 -1.737" /> </svg>"##;
const CAKE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 17v-5a3 3 0 0 0 -3 -3h-5m-4 0h-3a3 3 0 0 0 -3 3v8h17" /> <path d="M3 14.803c.312 .135 .654 .204 1 .197a2.4 2.4 0 0 0 2 -1a2.4 2.4 0 0 1 2 -1a2.4 2.4 0 0 1 2 1a2.4 2.4 0 0 0 2 1a2.4 2.4 0 0 0 2 -1m4 0a2.4 2.4 0 0 0 2 1c.35 .007 .692 -.062 1 -.197" /> <path d="M10.172 6.188c.07 -.158 .163 -.31 .278 -.451l1.55 -1.737l1.465 1.638a2 2 0 0 1 -.65 3.19" /> <path d="M3 3l18 18" /> </svg>"##;
const CAKE_ROLL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 15c-4.97 0 -9 -2.462 -9 -5.5s4.03 -5.5 9 -5.5s9 2.462 9 5.5s-4.03 5.5 -9 5.5" /> <path d="M12 6.97c3 0 4 1.036 4 1.979c0 2.805 -8 2.969 -8 -.99c0 -2.11 1.5 -3.959 4 -3.959" /> <path d="M21 9.333v5.334c0 2.945 -4.03 5.333 -9 5.333c-4.97 0 -9 -2.388 -9 -5.333v-5.334" /> </svg>"##;
const CANDY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.05 11.293l4.243 -4.243a2 2 0 0 1 2.828 0l2.829 2.83a2 2 0 0 1 0 2.828l-4.243 4.243a2 2 0 0 1 -2.828 0l-2.829 -2.831a2 2 0 0 1 0 -2.828" /> <path d="M16.243 9.172l3.086 -.772a1.5 1.5 0 0 0 .697 -2.516l-2.216 -2.217a1.5 1.5 0 0 0 -2.44 .47l-1.248 2.913" /> <path d="M9.172 16.243l-.772 3.086a1.5 1.5 0 0 1 -2.516 .697l-2.217 -2.216a1.5 1.5 0 0 1 .47 -2.44l2.913 -1.248" /> </svg>"##;
const CANDY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.174 7.17l.119 -.12a2 2 0 0 1 2.828 0l2.829 2.83a2 2 0 0 1 0 2.828l-.124 .124m-2 2l-2.123 2.123a2 2 0 0 1 -2.828 0l-2.829 -2.831a2 2 0 0 1 0 -2.828l2.113 -2.112" /> <path d="M16.243 9.172l3.086 -.772a1.5 1.5 0 0 0 .697 -2.516l-2.216 -2.217a1.5 1.5 0 0 0 -2.44 .47l-1.248 2.913" /> <path d="M9.172 16.243l-.772 3.086a1.5 1.5 0 0 1 -2.516 .697l-2.217 -2.216a1.5 1.5 0 0 1 .47 -2.44l2.913 -1.248" /> <path d="M3 3l18 18" /> </svg>"##;
const CARAMBOLA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.286 21.09q -1.69 .001 -5.288 -2.615q -3.596 2.617 -5.288 2.616q -2.726 0 -.495 -6.8q -9.389 -6.775 2.135 -6.775h.076q 1.785 -5.516 3.574 -5.516q 1.785 0 3.574 5.516h.076q 11.525 0 2.133 6.774q 2.23 6.802 -.497 6.8" /> </svg>"##;
const CARROT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 21s9.834 -3.489 12.684 -6.34a4.487 4.487 0 0 0 0 -6.344a4.483 4.483 0 0 0 -6.342 0c-2.86 2.861 -6.347 12.689 -6.347 12.689l.005 -.005" /> <path d="M9 13l-1.5 -1.5" /> <path d="M16 14l-2 -2" /> <path d="M22 8s-1.14 -2 -3 -2c-1.406 0 -3 2 -3 2s1.14 2 3 2s3 -2 3 -2" /> <path d="M16 2s-2 1.14 -2 3s2 3 2 3s2 -1.577 2 -3c0 -1.86 -2 -3 -2 -3" /> </svg>"##;
const CARROT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.868 8.846c-2.756 3.382 -5.868 12.154 -5.868 12.154s8.75 -3.104 12.134 -5.85m1.667 -2.342a4.486 4.486 0 0 0 -5.589 -5.615" /> <path d="M9 13l-1.5 -1.5" /> <path d="M22 8s-1.14 -2 -3 -2c-1.406 0 -3 2 -3 2s1.14 2 3 2s3 -2 3 -2" /> <path d="M16 2s-2 1.14 -2 3s2 3 2 3s2 -1.577 2 -3c0 -1.86 -2 -3 -2 -3" /> <path d="M3 3l18 18" /> </svg>"##;
const CASH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 15h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v3" /> <path d="M7 10a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1l0 -8" /> <path d="M12 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> </svg>"##;
const CASH_BANKNOTE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M3 8a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2l0 -8" /> <path d="M18 12h.01" /> <path d="M6 12h.01" /> </svg>"##;
const CASH_BANKNOTE_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 18h-6a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v3" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M6 12h.01" /> <path d="M18.42 15.61a2.1 2.1 0 1 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const CASH_BANKNOTE_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.872 11.13a3.001 3.001 0 1 0 -4.29 3.514" /> <path d="M10 18h-5a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v3" /> <path d="M6 12h.01" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.24 2.24 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.24 2.24 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const CASH_BANKNOTE_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M12 18h-7a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v7" /> <path d="M18 12h.01" /> <path d="M6 12h.01" /> <path d="M16 19h6" /> </svg>"##;
const CASH_BANKNOTE_MOVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M12 18h-7a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4.5" /> <path d="M18 12h.01" /> <path d="M6 12h.01" /> <path d="M16 19h6" /> <path d="M19 16l3 3l-3 3" /> </svg>"##;
const CASH_BANKNOTE_MOVE_BACK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M12 18h-7a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4.5" /> <path d="M18 12h.01" /> <path d="M6 12h.01" /> <path d="M16 19h6" /> <path d="M19 16l-3 3l3 3" /> </svg>"##;
const CASH_BANKNOTE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.88 9.878a3 3 0 1 0 4.242 4.243m.58 -3.425a3.012 3.012 0 0 0 -1.412 -1.405" /> <path d="M10 6h9a2 2 0 0 1 2 2v8c0 .294 -.064 .574 -.178 .825m-2.822 1.175h-13a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h1" /> <path d="M18 12l.01 0" /> <path d="M6 12l.01 0" /> <path d="M3 3l18 18" /> </svg>"##;
const CASH_BANKNOTE_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /> <path d="M12.25 18h-7.25a2 2 0 0 1 -2 -2v-8a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4.5" /> <path d="M18 12h.01" /> <path d="M6 12h.01" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const CASH_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 15h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v3" /> <path d="M11 19h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v1.25" /> <path d="M18.42 15.61a2.1 2.1 0 1 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const CASH_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 15h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v3" /> <path d="M10.25 19h-2.25a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v1.25" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.24 2.24 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.24 2.24 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const CASH_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 15h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v3" /> <path d="M12 19h-4a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v5" /> <path d="M12 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M16 19h6" /> </svg>"##;
const CASH_MOVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 15h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v3" /> <path d="M12 19h-4a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v2.5" /> <path d="M15.92 13.437a2 2 0 1 0 -2.472 2.486" /> <path d="M16 19h6" /> <path d="M19 16l3 3l-3 3" /> </svg>"##;
const CASH_MOVE_BACK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 15h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v3" /> <path d="M12 19h-4a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v2.5" /> <path d="M15.914 13.417a2 2 0 1 0 -2.447 2.511" /> <path d="M16 19h6" /> <path d="M19 16l-3 3l3 3" /> </svg>"##;
const CASH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 9h6a2 2 0 0 1 2 2v6m-2 2h-10a2 2 0 0 1 -2 -2v-6a2 2 0 0 1 2 -2" /> <path d="M12.582 12.59a2 2 0 0 0 2.83 2.826" /> <path d="M17 9v-2a2 2 0 0 0 -2 -2h-6m-4 0a2 2 0 0 0 -2 2v6a2 2 0 0 0 2 2h2" /> <path d="M3 3l18 18" /> </svg>"##;
const CASH_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 15h-3a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v3" /> <path d="M12 19h-4a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1h12a1 1 0 0 1 1 1v2.5" /> <path d="M12 14a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const CASH_REGISTER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 15h-2.5c-.398 0 -.779 .158 -1.061 .439c-.281 .281 -.439 .663 -.439 1.061c0 .398 .158 .779 .439 1.061c.281 .281 .663 .439 1.061 .439h1c.398 0 .779 .158 1.061 .439c.281 .281 .439 .663 .439 1.061c0 .398 -.158 .779 -.439 1.061c-.281 .281 -.663 .439 -1.061 .439h-2.5" /> <path d="M19 21v1m0 -8v1" /> <path d="M13 21h-7c-.53 0 -1.039 -.211 -1.414 -.586c-.375 -.375 -.586 -.884 -.586 -1.414v-10c0 -.53 .211 -1.039 .586 -1.414c.375 -.375 .884 -.586 1.414 -.586h2m12 3.12v-1.12c0 -.53 -.211 -1.039 -.586 -1.414c-.375 -.375 -.884 -.586 -1.414 -.586h-2" /> <path d="M16 10v-6c0 -.53 -.211 -1.039 -.586 -1.414c-.375 -.375 -.884 -.586 -1.414 -.586h-4c-.53 0 -1.039 .211 -1.414 .586c-.375 .375 -.586 .884 -.586 1.414v6m8 0h-8m8 0h1m-9 0h-1" /> <path d="M8 14v.01" /> <path d="M8 17v.01" /> <path d="M12 13.99v.01" /> <path d="M12 17v.01" /> </svg>"##;
const CHEESE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.519 20.008l16.481 -.008v-3.5a2 2 0 1 1 0 -4v-3.5h-16.722" /> <path d="M21 9l-9.385 -4.992c-2.512 .12 -4.758 1.42 -6.327 3.425c-1.423 1.82 -2.288 4.221 -2.288 6.854c0 2.117 .56 4.085 1.519 5.721" /> <path d="M15 13v.01" /> <path d="M8 13v.01" /> <path d="M11 16v.01" /> </svg>"##;
const CHEF_HAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3c1.918 0 3.52 1.35 3.91 3.151a4 4 0 0 1 2.09 7.723l0 7.126h-12v-7.126a4 4 0 1 1 2.092 -7.723a4 4 0 0 1 3.908 -3.151" /> <path d="M6.161 17.009l11.839 -.009" /> </svg>"##;
const CHEF_HAT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.72 4.712a4 4 0 0 1 7.19 1.439a4 4 0 0 1 2.09 7.723v.126m0 4v3h-12v-7.126a4 4 0 0 1 .081 -7.796" /> <path d="M6.161 17.009l10.839 -.009" /> <path d="M3 3l18 18" /> </svg>"##;
const CHOCOLATE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21v-16" /> <path d="M6 15h12" /> <path d="M6 9h10.5" /> <path d="M10.05 3a2.5 2.5 0 0 0 3.987 1.47a3 3 0 0 0 2.047 2.387a2.504 2.504 0 0 0 1.916 3.093v9.05a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h2.05" /> </svg>"##;
const CLOTHES_RACK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M12 7v14" /> <path d="M9 21h6" /> <path d="M7.757 9.243a6 6 0 0 0 8.486 0" /> </svg>"##;
const CLOTHES_RACK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M12 7v1m0 4v9" /> <path d="M9 21h6" /> <path d="M7.757 9.243a6 6 0 0 0 3.129 1.653m3.578 -.424a6 6 0 0 0 1.779 -1.229" /> <path d="M3 3l18 18" /> </svg>"##;
const COFFEE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 14c.83 .642 2.077 1.017 3.5 1c1.423 .017 2.67 -.358 3.5 -1c.83 -.642 2.077 -1.017 3.5 -1c1.423 -.017 2.67 .358 3.5 1" /> <path d="M8 3a2.4 2.4 0 0 0 -1 2a2.4 2.4 0 0 0 1 2" /> <path d="M12 3a2.4 2.4 0 0 0 -1 2a2.4 2.4 0 0 0 1 2" /> <path d="M3 10h14v5a6 6 0 0 1 -6 6h-2a6 6 0 0 1 -6 -6v-5" /> <path d="M16.746 16.726a3 3 0 1 0 .252 -5.555" /> </svg>"##;
const COFFEE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 14c.83 .642 2.077 1.017 3.5 1c1.423 .017 2.67 -.358 3.5 -1c.73 -.565 1.783 -.923 3 -.99" /> <path d="M8 3c-.194 .14 -.364 .305 -.506 .49" /> <path d="M12 3a2.4 2.4 0 0 0 -1 2a2.4 2.4 0 0 0 1 2" /> <path d="M14 10h3v3m-.257 3.743a6 6 0 0 1 -5.743 4.257h-2a6 6 0 0 1 -6 -6v-5h7" /> <path d="M20.116 16.124a3 3 0 0 0 -3.118 -4.953" /> <path d="M3 3l18 18" /> </svg>"##;
const COIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14.8 9a2 2 0 0 0 -1.8 -1h-2a2 2 0 1 0 0 4h2a2 2 0 1 1 0 4h-2a2 2 0 0 1 -1.8 -1" /> <path d="M12 7v10" /> </svg>"##;
const COIN_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 8h4.09c1.055 0 1.91 .895 1.91 2s-.855 2 -1.91 2c1.055 0 1.91 .895 1.91 2s-.855 2 -1.91 2h-4.09" /> <path d="M10 12h4" /> <path d="M10 7v10v-9" /> <path d="M13 7v1" /> <path d="M13 16v1" /> </svg>"##;
const COIN_EURO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M14.401 8c-.669 -.628 -1.5 -1 -2.401 -1c-2.21 0 -4 2.239 -4 5s1.79 5 4 5c.9 0 1.731 -.372 2.4 -1" /> <path d="M7 10.5h4" /> <path d="M7 13.5h4" /> </svg>"##;
const COIN_MONERO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M4 16h4v-7l4 4l4 -4v7h4" /> </svg>"##;
const COIN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.8 9a2 2 0 0 0 -1.8 -1h-1m-2.82 1.171a2 2 0 0 0 1.82 2.829h1m2.824 2.822a2 2 0 0 1 -1.824 1.178h-2a2 2 0 0 1 -1.8 -1" /> <path d="M20.042 16.045a9 9 0 0 0 -12.087 -12.087m-2.318 1.677a9 9 0 1 0 12.725 12.73" /> <path d="M12 6v2m0 8v2" /> <path d="M3 3l18 18" /> </svg>"##;
const COIN_POUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M15 9a2 2 0 1 0 -4 0v5a2 2 0 0 1 -2 2h6" /> <path d="M9 12h4" /> </svg>"##;
const COIN_RUPEE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M15 8h-6h1a3 3 0 0 1 0 6h-1l3 3" /> <path d="M9 11h6" /> </svg>"##;
const COIN_TAKA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8l.553 -.276a1 1 0 0 1 1.447 .894v6.382a2 2 0 0 0 2 2h.5a2.5 2.5 0 0 0 2.5 -2.5v-.5h-1" /> <path d="M8 11h7" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /> </svg>"##;
const COIN_YEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 12h6" /> <path d="M9 15h6" /> <path d="M9 8l3 4.5" /> <path d="M15 8l-3 4.5v4.5" /> </svg>"##;
const COIN_YUAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 13h6" /> <path d="M9 8l3 4.5" /> <path d="M15 8l-3 4.5v4.5" /> </svg>"##;
const COINS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 14c0 1.657 2.686 3 6 3s6 -1.343 6 -3s-2.686 -3 -6 -3s-6 1.343 -6 3" /> <path d="M9 14v4c0 1.656 2.686 3 6 3s6 -1.344 6 -3v-4" /> <path d="M3 6c0 1.072 1.144 2.062 3 2.598s4.144 .536 6 0c1.856 -.536 3 -1.526 3 -2.598c0 -1.072 -1.144 -2.062 -3 -2.598s-4.144 -.536 -6 0c-1.856 .536 -3 1.526 -3 2.598" /> <path d="M3 6v10c0 .888 .772 1.45 2 2" /> <path d="M3 11c0 .888 .772 1.45 2 2" /> </svg>"##;
const COOKER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 7h.01" /> <path d="M15 7h.01" /> <path d="M9 7h.01" /> <path d="M5 5a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -14" /> <path d="M9 15h6" /> <path d="M5 11h14" /> </svg>"##;
const COOKIE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13v.01" /> <path d="M12 17v.01" /> <path d="M12 12v.01" /> <path d="M16 14v.01" /> <path d="M11 8v.01" /> <path d="M13.148 3.476l2.667 1.104a4 4 0 0 0 4.656 6.14l.053 .132a3 3 0 0 1 0 2.296q -.745 1.18 -1.024 1.852q -.283 .684 -.66 2.216a3 3 0 0 1 -1.624 1.623q -1.572 .394 -2.216 .661q -.712 .295 -1.852 1.024a3 3 0 0 1 -2.296 0q -1.203 -.754 -1.852 -1.024q -.707 -.292 -2.216 -.66a3 3 0 0 1 -1.623 -1.624q -.397 -1.577 -.661 -2.216q -.298 -.718 -1.024 -1.852a3 3 0 0 1 0 -2.296q .719 -1.116 1.024 -1.852q .257 -.62 .66 -2.216a3 3 0 0 1 1.624 -1.623q 1.547 -.384 2.216 -.661q .687 -.285 1.852 -1.024a3 3 0 0 1 2.296 0" /> </svg>"##;
const COOKIE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 13v.01" /> <path d="M12 17v.01" /> <path d="M12 12v.01" /> <path d="M18.192 18.187a3 3 0 0 1 -.976 .652c-1.048 .263 -1.787 .483 -2.216 .661c-.475 .197 -1.092 .538 -1.852 1.024a3 3 0 0 1 -2.296 0c-.802 -.503 -1.419 -.844 -1.852 -1.024c-.471 -.195 -1.21 -.415 -2.216 -.66a3 3 0 0 1 -1.623 -1.624c-.265 -1.052 -.485 -1.79 -.661 -2.216c-.198 -.479 -.54 -1.096 -1.024 -1.852a3 3 0 0 1 0 -2.296c.48 -.744 .82 -1.361 1.024 -1.852c.171 -.413 .391 -1.152 .66 -2.216a3 3 0 0 1 .649 -.971m2.821 -1.174c.14 -.049 .263 -.095 .37 -.139c.458 -.19 1.075 -.531 1.852 -1.024a3 3 0 0 1 2.296 0l2.667 1.104a4 4 0 0 0 4.656 6.14l.053 .132a3 3 0 0 1 0 2.296c-.497 .786 -.838 1.404 -1.024 1.852a6.579 6.579 0 0 0 -.135 .36" /> <path d="M3 3l18 18" /> </svg>"##;
const CREDIT_CARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3l0 -8" /> <path d="M3 10l18 0" /> <path d="M7 15l.01 0" /> <path d="M11 15l2 0" /> </svg>"##;
const CREDIT_CARD_HAND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 4h9.914a3 3 0 0 1 1.92 .695l5.166 4.305" /> <path d="M11.15 9h8.85a2 2 0 0 1 2 2v7a2 2 0 0 1 -2 2h-13a2 2 0 0 1 -2 -2v-8.7" /> <path d="M3 8l7.2 4.7a1.803 1.803 0 0 0 2 -3l-4.2 -2.7" /> <path d="M5 16h17" /> </svg>"##;
const CREDIT_CARD_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M9 5h9a3 3 0 0 1 3 3v8a3 3 0 0 1 -.128 .87" /> <path d="M18.87 18.872a3 3 0 0 1 -.87 .128h-12a3 3 0 0 1 -3 -3v-8c0 -1.352 .894 -2.495 2.124 -2.87" /> <path d="M3 11l8 0" /> <path d="M15 11l6 0" /> <path d="M7 15l.01 0" /> <path d="M11 15l2 0" /> </svg>"##;
const CREDIT_CARD_PAY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-6a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4.5" /> <path d="M3 10h18" /> <path d="M16 19h6" /> <path d="M19 16l3 3l-3 3" /> <path d="M7.005 15h.005" /> <path d="M11 15h2" /> </svg>"##;
const CREDIT_CARD_REFUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19h-6a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v4.5" /> <path d="M3 10h18" /> <path d="M7 15h.01" /> <path d="M11 15h2" /> <path d="M16 19h6" /> <path d="M19 16l-3 3l3 3" /> </svg>"##;
const CUP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 11h14v-3h-14l0 3" /> <path d="M17.5 11l-1.5 10h-8l-1.5 -10" /> <path d="M6 8v-1a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v1" /> <path d="M15 5v-2" /> </svg>"##;
const CUP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h-3v3h6m4 0h4v-3h-7" /> <path d="M17.5 11l-.323 2.154m-.525 3.497l-.652 4.349h-8l-1.5 -10" /> <path d="M6 8v-1c0 -.296 .064 -.577 .18 -.83m2.82 -1.17h7a2 2 0 0 1 2 2v1" /> <path d="M15 5v-2" /> <path d="M3 3l18 18" /> </svg>"##;
const CURRENCY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M4 4l3 3" /> <path d="M20 4l-3 3" /> <path d="M4 20l3 -3" /> <path d="M20 20l-3 -3" /> </svg>"##;
const CURRENCY_AFGHANI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 13h-3.5a3.5 3.5 0 1 1 3.5 -3.5v6.5h-7" /> <path d="M12 3v.01" /> <path d="M12 19v2" /> </svg>"##;
const CURRENCY_BAHRAINI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 10v1a4 4 0 0 0 4 4h2a2 2 0 0 0 2 -2v-3" /> <path d="M7 19.01v-.01" /> <path d="M14 15.01v-.01" /> <path d="M17 15h2a2 2 0 0 0 1.649 -3.131l-2.653 -3.869" /> </svg>"##;
const CURRENCY_BAHT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 6h5a3 3 0 0 1 3 3v.143a2.857 2.857 0 0 1 -2.857 2.857h-5.143" /> <path d="M8 12h5a3 3 0 0 1 3 3v.143a2.857 2.857 0 0 1 -2.857 2.857h-5.143" /> <path d="M8 6v12" /> <path d="M11 4v2" /> <path d="M11 18v2" /> </svg>"##;
const CURRENCY_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 6h8a3 3 0 0 1 0 6a3 3 0 0 1 0 6h-8" /> <path d="M8 6l0 12" /> <path d="M8 12l6 0" /> <path d="M9 3l0 3" /> <path d="M13 3l0 3" /> <path d="M9 18l0 3" /> <path d="M13 18l0 3" /> </svg>"##;
const CURRENCY_CENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.007 7.54a5.965 5.965 0 0 0 -4.008 -1.54a6 6 0 0 0 -5.992 6c0 3.314 2.682 6 5.992 6a5.965 5.965 0 0 0 4 -1.536" /> <path d="M12 20v-2" /> <path d="M12 6v-2" /> </svg>"##;
const CURRENCY_DINAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 20.01v-.01" /> <path d="M6 13l2.386 -.9a1 1 0 0 0 -.095 -1.902l-1.514 -.404a1 1 0 0 1 -.102 -1.9l2.325 -.894" /> <path d="M3 14v1a3 3 0 0 0 3 3h4.161a3 3 0 0 0 2.983 -3.32l-1.144 -10.68" /> <path d="M16 17l1 1h2a2 2 0 0 0 1.649 -3.131l-2.653 -3.869" /> </svg>"##;
const CURRENCY_DIRHAM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.5 19h-3.5" /> <path d="M8.599 16.479a1.5 1.5 0 1 0 -1.099 2.521" /> <path d="M7 4v9" /> <path d="M15 13h1.888a1.5 1.5 0 0 0 1.296 -2.256l-2.184 -3.744" /> <path d="M11 13.01v-.01" /> </svg>"##;
const CURRENCY_DOGECOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 12h6" /> <path d="M9 6v12" /> <path d="M6 18h6a6 6 0 1 0 0 -12h-6" /> </svg>"##;
const CURRENCY_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.7 8a3 3 0 0 0 -2.7 -2h-4a3 3 0 0 0 0 6h4a3 3 0 0 1 0 6h-4a3 3 0 0 1 -2.7 -2" /> <path d="M12 3v3m0 12v3" /> </svg>"##;
const CURRENCY_DOLLAR_AUSTRALIAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 18l3.279 -11.476a.75 .75 0 0 1 1.442 0l3.279 11.476" /> <path d="M21 6h-4a3 3 0 0 0 0 6h1a3 3 0 0 1 0 6h-4" /> <path d="M17 20v-2" /> <path d="M18 6v-2" /> <path d="M4.5 14h5" /> </svg>"##;
const CURRENCY_DOLLAR_BRUNEI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 6h-4a3 3 0 0 0 0 6h1a3 3 0 0 1 0 6h-4" /> <path d="M17 20v-2" /> <path d="M18 6v-2" /> <path d="M3 6v12h4a3 3 0 0 0 0 -6h-4h4a3 3 0 0 0 0 -6h-4" /> </svg>"##;
const CURRENCY_DOLLAR_CANADIAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 6h-4a3 3 0 0 0 0 6h1a3 3 0 0 1 0 6h-4" /> <path d="M10 18h-1a6 6 0 1 1 0 -12h1" /> <path d="M17 20v-2" /> <path d="M18 6v-2" /> </svg>"##;
const CURRENCY_DOLLAR_GUYANESE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 6h-4a3 3 0 0 0 0 6h1a3 3 0 0 1 0 6h-4" /> <path d="M10 6h-3a4 4 0 0 0 -4 4v4a4 4 0 0 0 4 4h3v-6h-2" /> <path d="M17 20v-2" /> <path d="M18 6v-2" /> </svg>"##;
const CURRENCY_DOLLAR_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.7 8a3 3 0 0 0 -2.7 -2h-4m-2.557 1.431a3 3 0 0 0 2.557 4.569h2m4.564 4.558a3 3 0 0 1 -2.564 1.442h-4a3 3 0 0 1 -2.7 -2" /> <path d="M12 3v3m0 12v3" /> <path d="M3 3l18 18" /> </svg>"##;
const CURRENCY_DOLLAR_SINGAPORE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 6h-4a3 3 0 0 0 0 6h1a3 3 0 0 1 0 6h-4" /> <path d="M10 6h-4a3 3 0 1 0 0 6h1a3 3 0 0 1 0 6h-4" /> <path d="M17 20v-2" /> <path d="M18 6v-2" /> </svg>"##;
const CURRENCY_DOLLAR_ZIMBABWEAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 6h-4a3 3 0 0 0 0 6h1a3 3 0 0 1 0 6h-4" /> <path d="M17 20v-2" /> <path d="M18 6v-2" /> <path d="M3 6h7l-7 12h7" /> </svg>"##;
const CURRENCY_DONG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 20h8" /> <path d="M15 13a3 3 0 0 1 -3 3a3 3 0 0 1 -3 -3a3 3 0 0 1 3 -3a3 3 0 0 1 3 3" /> <path d="M15 4v12" /> <path d="M13 6h4" /> </svg>"##;
const CURRENCY_DRAM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 10a6 6 0 1 1 12 0v10" /> <path d="M12 16h8" /> <path d="M12 12h8" /> </svg>"##;
const CURRENCY_ETHEREUM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 12l6 -9l6 9l-6 9l-6 -9" /> <path d="M6 12l6 -3l6 3l-6 2l-6 -2" /> </svg>"##;
const CURRENCY_EURO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.2 7a6 7 0 1 0 0 10" /> <path d="M13 10h-8m0 4h8" /> </svg>"##;
const CURRENCY_EURO_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.2 7c-1.977 -2.26 -4.954 -2.602 -7.234 -1.04m-1.913 2.079c-1.604 2.72 -1.374 6.469 .69 8.894c2.292 2.691 6 2.758 8.356 .18" /> <path d="M10 10h-5m0 4h8" /> <path d="M3 3l18 18" /> </svg>"##;
const CURRENCY_FLORIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 12h8" /> <path d="M7 19c1.213 0 2.31 -.723 2.788 -1.838l4.424 -10.324a3.033 3.033 0 0 1 2.788 -1.838" /> </svg>"##;
const CURRENCY_FORINT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 4h-4a3 3 0 0 0 -3 3v12" /> <path d="M10 11h-6" /> <path d="M16 4v13a2 2 0 0 0 2 2h2" /> <path d="M19 9h-5" /> </svg>"##;
const CURRENCY_FRANK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 5h-6a2 2 0 0 0 -2 2v12" /> <path d="M7 15h4" /> <path d="M9 11h7" /> </svg>"##;
const CURRENCY_GUARANI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.007 7.54a5.965 5.965 0 0 0 -4.008 -1.54a6 6 0 0 0 -5.992 6c0 3.314 2.682 6 5.992 6a5.965 5.965 0 0 0 4 -1.536c.732 -.66 1.064 -2.148 1 -4.464h-5" /> <path d="M12 20v-16" /> </svg>"##;
const CURRENCY_HRYVNIA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 7a2.64 2.64 0 0 1 2.562 -2h3.376a2.64 2.64 0 0 1 2.562 2a2.57 2.57 0 0 1 -1.344 2.922l-5.876 2.938a3.338 3.338 0 0 0 -1.78 3.64a3.11 3.11 0 0 0 3.05 2.5h2.888a2.64 2.64 0 0 0 2.562 -2" /> <path d="M6 10h12" /> <path d="M6 14h12" /> </svg>"##;
const CURRENCY_HUSD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 20v-5" /> <path d="M17 9v-5" /> <path d="M17 20v-6a2 2 0 0 0 -2 -2h-6a2 2 0 0 1 -2 -2v-6" /> </svg>"##;
const CURRENCY_IRANIAN_RIAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 4v9a2 2 0 0 1 -2 2h-1a3 3 0 0 1 -3 -3v-1" /> <path d="M12 5v8a1 1 0 0 0 1 1h1a2 2 0 0 0 2 -2v-1" /> <path d="M21 14v1.096a5 5 0 0 1 -3.787 4.85l-.213 .054" /> <path d="M11 18h.01" /> <path d="M14 18h.01" /> </svg>"##;
const CURRENCY_KIP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 12h12" /> <path d="M9 5v14" /> <path d="M16 19a7 7 0 0 0 -7 -7a7 7 0 0 0 7 -7" /> </svg>"##;
const CURRENCY_KRONE_CZECH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 6v12" /> <path d="M5 12c3.5 0 6 -3 6 -6" /> <path d="M5 12c3.5 0 6 3 6 6" /> <path d="M19 6l-2 2l-2 -2" /> <path d="M19 12h-2a3 3 0 0 0 0 6h2" /> </svg>"##;
const CURRENCY_KRONE_DANISH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 6v12" /> <path d="M5 12c3.5 0 6 -3 6 -6" /> <path d="M5 12c3.5 0 6 3 6 6" /> <path d="M15 10v8" /> <path d="M19 10a4 4 0 0 0 -4 4" /> <path d="M20 18.01v-.01" /> </svg>"##;
const CURRENCY_KRONE_SWEDISH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 6v12" /> <path d="M5 12c3.5 0 6 -3 6 -6" /> <path d="M5 12c3.5 0 6 3 6 6" /> <path d="M15 10v8" /> <path d="M19 10a4 4 0 0 0 -4 4" /> </svg>"##;
const CURRENCY_LARI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 13a6 6 0 1 0 -6 6" /> <path d="M6 19h12" /> <path d="M10 5v7" /> <path d="M14 12v-7" /> </svg>"##;
const CURRENCY_LEU_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 18h-7a3 3 0 0 1 -3 -3v-10" /> </svg>"##;
const CURRENCY_LIRA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 5v15a7 7 0 0 0 7 -7" /> <path d="M6 15l8 -4" /> <path d="M14 7l-8 4" /> </svg>"##;
const CURRENCY_LITECOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 19h-8.194a2 2 0 0 1 -1.98 -2.283l1.674 -11.717" /> <path d="M14 9l-9 4" /> </svg>"##;
const CURRENCY_LYD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 15h.01" /> <path d="M21 5v10a2 2 0 0 1 -2 2h-2.764a2 2 0 0 1 -1.789 -1.106l-.447 -.894" /> <path d="M5 8l2.773 4.687c.427 .697 .234 1.626 -.43 2.075a1.38 1.38 0 0 1 -.773 .238h-2.224a.93 .93 0 0 1 -.673 -.293l-.673 -.707" /> </svg>"##;
const CURRENCY_MANAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 19v-7a5 5 0 1 1 10 0v7" /> <path d="M12 5v14" /> </svg>"##;
const CURRENCY_MONERO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 18h3v-11l6 7l6 -7v11h3" /> </svg>"##;
const CURRENCY_NAIRA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 18v-10.948a1.05 1.05 0 0 1 1.968 -.51l6.064 10.916a1.05 1.05 0 0 0 1.968 -.51v-10.948" /> <path d="M5 10h14" /> <path d="M5 14h14" /> </svg>"##;
const CURRENCY_NANO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 20l10 -16" /> <path d="M7 12h10" /> <path d="M7 16h10" /> <path d="M17 20l-10 -16" /> </svg>"##;
const CURRENCY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18.531 14.524a7 7 0 0 0 -9.06 -9.053m-2.422 1.582a7 7 0 0 0 9.903 9.896" /> <path d="M4 4l3 3" /> <path d="M20 4l-3 3" /> <path d="M4 20l3 -3" /> <path d="M20 20l-3 -3" /> <path d="M3 3l18 18" /> </svg>"##;
const CURRENCY_PAANGA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 6h-4a3 3 0 0 0 0 6h1a3 3 0 0 1 0 6h-4" /> <path d="M17 20v-2" /> <path d="M18 6v-2" /> <path d="M3 6h8" /> <path d="M7 6v12" /> </svg>"##;
const CURRENCY_PESO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 19v-14h3.5a4.5 4.5 0 1 1 0 9h-3.5" /> <path d="M18 8h-12" /> <path d="M18 11h-12" /> </svg>"##;
const CURRENCY_POUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 18.5a6 6 0 0 1 -5 0a6 6 0 0 0 -5 .5a3 3 0 0 0 2 -2.5v-7.5a4 4 0 0 1 7.45 -2m-2.55 6h-7" /> </svg>"##;
const CURRENCY_POUND_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 18.5a6 6 0 0 1 -5 0a6 6 0 0 0 -5 .5a3 3 0 0 0 2 -2.5v-7.5m1.192 -2.825a4 4 0 0 1 6.258 .825m-3.45 6h-6" /> <path d="M3 3l18 18" /> </svg>"##;
const CURRENCY_QUETZAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 12a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" /> <path d="M13 13l5 5" /> </svg>"##;
const CURRENCY_REAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 6h-4a3 3 0 0 0 0 6h1a3 3 0 0 1 0 6h-4" /> <path d="M4 18v-12h3a3 3 0 1 1 0 6h-3c5.5 0 5 4 6 6" /> <path d="M18 6v-2" /> <path d="M17 20v-2" /> </svg>"##;
const CURRENCY_RENMINBI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 9v8a2 2 0 1 0 4 0" /> <path d="M19 9h-14" /> <path d="M19 5h-14" /> <path d="M9 9v4c0 2.5 -.667 4 -2 6" /> </svg>"##;
const CURRENCY_RIPPLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M14 7a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M14 17a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M10 12h3l2 -2.5" /> <path d="M15 14.5l-2 -2.5" /> </svg>"##;
const CURRENCY_RIYAL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 9v2a2 2 0 1 1 -4 0v-1v1a2 2 0 1 1 -4 0v-1v4a2 2 0 1 1 -4 0v-2" /> <path d="M18 12.01v-.01" /> <path d="M22 10v1a5 5 0 0 1 -5 5" /> </svg>"##;
const CURRENCY_RUBEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 19v-14h6a3 3 0 0 1 0 6h-8" /> <path d="M14 15h-8" /> </svg>"##;
const CURRENCY_RUFIYAA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 16h.01" /> <path d="M4 16c9.5 -4 11.5 -8 14 -9" /> <path d="M12 8l5 3" /> </svg>"##;
const CURRENCY_RUPEE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 5h-11h3a4 4 0 0 1 0 8h-3l6 6" /> <path d="M7 9l11 0" /> </svg>"##;
const CURRENCY_RUPEE_NEPALESE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 5h-11h3a4 4 0 1 1 0 8h-3l6 6" /> <path d="M21 17l-4.586 -4.414a2 2 0 0 0 -2.828 2.828l.707 .707" /> </svg>"##;
const CURRENCY_SHEKEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 18v-12h4a4 4 0 0 1 4 4v4" /> <path d="M18 6v12h-4a4 4 0 0 1 -4 -4v-4" /> </svg>"##;
const CURRENCY_SOLANA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18h12l4 -4h-12l-4 4" /> <path d="M8 14l-4 -4h12l4 4" /> <path d="M16 10l4 -4h-12l-4 4" /> </svg>"##;
const CURRENCY_SOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 18v-12h-5v10a2 2 0 0 1 -2 2" /> <path d="M14 6v12h4a3 3 0 0 0 0 -6h-4h4a3 3 0 0 0 0 -6h-4" /> </svg>"##;
const CURRENCY_TAKA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15.5 15.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M7 7a2 2 0 1 1 4 0v9a3 3 0 0 0 6 0v-.5" /> <path d="M8 11h6" /> </svg>"##;
const CURRENCY_TENGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 5h12" /> <path d="M6 9h12" /> <path d="M12 9v10" /> </svg>"##;
const CURRENCY_TETHER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 11a8 2 0 1 0 16 0a8 2 0 1 0 -16 0" /> <path d="M12 20v-16" /> <path d="M4 4h16" /> </svg>"##;
const CURRENCY_TUGRIK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 6h10" /> <path d="M12 6v13" /> <path d="M8 17l8 -3" /> <path d="M16 10l-8 3" /> </svg>"##;
const CURRENCY_WON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6l3.245 11.358a.85 .85 0 0 0 1.624 .035l3.131 -9.393l3.131 9.393a.85 .85 0 0 0 1.624 -.035l3.245 -11.358" /> <path d="M21 10h-18" /> <path d="M21 14h-18" /> </svg>"##;
const CURRENCY_XRP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5l3.585 3.585a4.83 4.83 0 0 0 6.83 0l3.585 -3.585" /> <path d="M5 19l3.585 -3.585a4.83 4.83 0 0 1 6.83 0l3.585 3.584" /> </svg>"##;
const CURRENCY_YEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19v-7l-5 -7m10 0l-5 7" /> <path d="M8 17l8 0" /> <path d="M8 13l8 0" /> </svg>"##;
const CURRENCY_YEN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19v-7m5 -7l-3.328 4.66" /> <path d="M8 17h8" /> <path d="M8 13h5" /> <path d="M3 3l18 18" /> </svg>"##;
const CURRENCY_YUAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19v-7l-5 -7" /> <path d="M17 5l-5 7" /> <path d="M8 13h8" /> </svg>"##;
const CURRENCY_ZCASH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 6h10l-10 12h10" /> <path d="M12 4v2" /> <path d="M12 18v2" /> </svg>"##;
const CURRENCY_ZLOTY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 18h-7l7 -7h-7" /> <path d="M17 18v-13" /> <path d="M14 14.5l6 -3.5" /> </svg>"##;
const DIAPER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8.323c0 -.579 0 -.868 .044 -1.11a2.7 2.7 0 0 1 2.17 -2.169c.239 -.044 .529 -.044 1.109 -.044h11.353c.579 0 .868 0 1.11 .044a2.7 2.7 0 0 1 2.169 2.17c.044 .24 .044 .53 .044 1.11v2.676a9 9 0 0 1 -18 0l.001 -2.677" /> <path d="M17 9h4" /> <path d="M3 9h4" /> <path d="M14.25 19.7v-1.4a6.3 6.3 0 0 1 6.3 -6.3" /> <path d="M9.75 19.7v-1.4a6.3 6.3 0 0 0 -6.3 -6.3" /> </svg>"##;
const DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l6 -6" /> <path d="M9 9.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M14 14.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const DISCOUNT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l3 -3m2 -2l1 -1" /> <path d="M9.148 9.145a.498 .498 0 0 0 .352 .855a.5 .5 0 0 0 .35 -.142" /> <path d="M14.148 14.145a.498 .498 0 0 0 .352 .855a.5 .5 0 0 0 .35 -.142" /> <path d="M5.641 5.631a9 9 0 1 0 12.719 12.738m1.68 -2.318a9 9 0 0 0 -12.074 -12.098" /> <path d="M3 3l18 18" /> </svg>"##;
const DOG_BOWL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15l5.586 -5.585a2 2 0 1 1 3.414 -1.415a2 2 0 1 1 -1.413 3.414l-3.587 3.586" /> <path d="M12 13l-3.586 -3.585a2 2 0 1 0 -3.414 -1.415a2 2 0 1 0 1.413 3.414l3.587 3.586" /> <path d="M3 20h18c-.175 -1.671 -.046 -3.345 -2 -5h-14c-1.333 1 -2 2.667 -2 5" /> </svg>"##;
const DUMPLING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.532 5.532a2.53 2.53 0 0 1 2.56 -.623a2.532 2.532 0 0 1 4.604 -.717q .146 -.24 .356 -.45a2.532 2.532 0 0 1 4.318 1.637a2.53 2.53 0 0 1 2.844 .511l.358 .358c1.384 1.385 -.7 5.713 -4.655 9.669c-3.956 3.955 -8.284 6.04 -9.669 4.655l-.358 -.358l-.114 -.122a2.53 2.53 0 0 1 -.398 -2.724a2.532 2.532 0 0 1 -1.186 -4.675a2.532 2.532 0 0 1 .718 -4.603a2.53 2.53 0 0 1 .622 -2.558" /> </svg>"##;
const E_PASSPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 7a2 2 0 0 1 2 -2h16a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-16a2 2 0 0 1 -2 -2l0 -10" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M9 12h-7" /> <path d="M15 12h7" /> </svg>"##;
const EGG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 14.083c0 4.154 -2.966 6.74 -7 6.917c-4.2 0 -7 -2.763 -7 -6.917c0 -5.538 3.5 -11.09 7 -11.083c3.5 .007 7 5.545 7 11.083" /> </svg>"##;
const EGG_CRACKED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 14.083c0 4.154 -2.966 6.74 -7 6.917c-4.2 0 -7 -2.763 -7 -6.917c0 -5.538 3.5 -11.09 7 -11.083c3.5 .007 7 5.545 7 11.083" /> <path d="M12 3l-1.5 5l3.5 2.5l-2 3.5" /> </svg>"##;
const EGG_FRIED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M14 3a5 5 0 0 1 4.872 6.13a3 3 0 0 1 .178 5.681a3 3 0 1 1 -4.684 3.626a5 5 0 1 1 -8.662 -5a5 5 0 1 1 4.645 -8.856a4.982 4.982 0 0 1 3.651 -1.585l0 .004" /> </svg>"##;
const EGG_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.927 17.934c-1.211 1.858 -3.351 2.953 -5.927 3.066c-4.2 0 -7 -2.763 -7 -6.917c0 -2.568 .753 -5.14 1.91 -7.158" /> <path d="M8.642 4.628c1.034 -1.02 2.196 -1.63 3.358 -1.628c3.5 .007 7 5.545 7 11.083c0 .298 -.015 .587 -.045 .868" /> <path d="M3 3l18 18" /> </svg>"##;
const EGGS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 22c-3 0 -4.868 -2.118 -5 -5c0 -3 2 -5 5 -5c4 0 8.01 2.5 8 5c0 2.5 -4 5 -8 5" /> <path d="M8 18c-3.03 -.196 -5 -2.309 -5 -5.38c0 -4.307 2.75 -8.625 5.5 -8.62c2.614 0 5.248 3.915 5.5 8" /> </svg>"##;
const FLIP_FLOPS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 4c2.21 0 4 1.682 4 3.758c0 .078 0 .156 -.008 .234l-.6 9.014c-.11 1.683 -1.596 3 -3.392 3s-3.28 -1.311 -3.392 -3l-.6 -9.014c-.138 -2.071 1.538 -3.855 3.743 -3.985a4.15 4.15 0 0 1 .25 -.007l-.001 0" /> <path d="M14.5 14c1 -3.333 2.167 -5 3.5 -5c1.333 0 2.5 1.667 3.5 5" /> <path d="M18 16v1" /> <path d="M6 4c2.21 0 4 1.682 4 3.758c0 .078 0 .156 -.008 .234l-.6 9.014c-.11 1.683 -1.596 3 -3.392 3s-3.28 -1.311 -3.392 -3l-.6 -9.014c-.138 -2.071 1.538 -3.855 3.742 -3.985c.084 0 .167 -.007 .25 -.007" /> <path d="M2.5 14c1 -3.333 2.167 -5 3.5 -5c1.333 0 2.5 1.667 3.5 5" /> <path d="M6 16v1" /> </svg>"##;
const GIFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 9a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1l0 -2" /> <path d="M12 8l0 13" /> <path d="M19 12v7a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-7" /> <path d="M7.5 8a2.5 2.5 0 0 1 0 -5a4.8 8 0 0 1 4.5 5a4.8 8 0 0 1 4.5 -5a2.5 2.5 0 0 1 0 5" /> </svg>"##;
const GIFT_CARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3l0 -8" /> <path d="M7 16l3 -3l3 3" /> <path d="M8 13c-.789 0 -2 -.672 -2 -1.5s.711 -1.5 1.5 -1.5c1.128 -.02 2.077 1.17 2.5 3c.423 -1.83 1.372 -3.02 2.5 -3c.789 0 1.5 .672 1.5 1.5s-1.211 1.5 -2 1.5h-4" /> </svg>"##;
const GIFT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8h8a1 1 0 0 1 1 1v2a1 1 0 0 1 -1 1h-4m-4 0h-8a1 1 0 0 1 -1 -1v-2a1 1 0 0 1 1 -1h4" /> <path d="M12 12v9" /> <path d="M19 12v3m0 4a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-7" /> <path d="M7.5 8a2.5 2.5 0 0 1 -2.457 -2.963m2.023 -2c.14 -.023 .286 -.037 .434 -.037c1.974 -.034 3.76 1.95 4.5 5c.74 -3.05 2.526 -5.034 4.5 -5a2.5 2.5 0 1 1 0 5" /> <path d="M3 3l18 18" /> </svg>"##;
const GLASS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 21h8" /> <path d="M12 16v5" /> <path d="M17 5l1 6c0 3.012 -2.686 5 -6 5s-6 -1.988 -6 -5l1 -6" /> <path d="M7 5a5 2 0 1 0 10 0a5 2 0 1 0 -10 0" /> </svg>"##;
const GLASS_CHAMPAGNE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 21h6" /> <path d="M12 16v5" /> <path d="M8 5a4 2 0 1 0 8 0a4 2 0 1 0 -8 0" /> <path d="M8 5c0 6.075 1.79 11 4 11s4 -4.925 4 -11" /> </svg>"##;
const GLASS_COCKTAIL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 21h8" /> <path d="M12 15v6" /> <path d="M5 5a7 2 0 1 0 14 0a7 2 0 1 0 -14 0" /> <path d="M5 5v.388c0 .432 .126 .853 .362 1.206l5 7.509c.633 .951 1.88 1.183 2.785 .517c.191 -.141 .358 -.316 .491 -.517l5 -7.509c.236 -.353 .362 -.774 .362 -1.206v-.388" /> </svg>"##;
const GLASS_FULL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 21l8 0" /> <path d="M12 15l0 6" /> <path d="M17 3l1 7c0 3.012 -2.686 5 -6 5s-6 -1.988 -6 -5l1 -7h10" /> <path d="M6 10a5 5 0 0 1 6 0a5 5 0 0 0 6 0" /> </svg>"##;
const GLASS_GIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 21h8" /> <path d="M12 15v6" /> <path d="M5.5 5a6.5 2 0 1 0 13 0a6.5 2 0 1 0 -13 0" /> <path d="M5.75 4.5c-.612 .75 -.75 2 -.75 3.5a7 7 0 0 0 14 0c0 -1.5 -.094 -2.75 -.75 -3.5" /> </svg>"##;
const GLASS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 21h8" /> <path d="M12 16v5" /> <path d="M17 5l1 6c0 .887 -.233 1.685 -.646 2.37m-2.083 1.886c-.941 .48 -2.064 .744 -3.271 .744c-3.314 0 -6 -1.988 -6 -5l.711 -4.268" /> <path d="M10.983 6.959c.329 .027 .669 .041 1.017 .041c2.761 0 5 -.895 5 -2s-2.239 -2 -5 -2c-1.716 0 -3.23 .346 -4.13 .872" /> <path d="M3 3l18 18" /> </svg>"##;
const GRAPE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 3a14.5 14.5 0 0 0 -1 6" /> <path d="M12 8.9s-2.77 .52 -4.1 -.8s-.8 -4 -.8 -4s2.57 -.53 3.88 .8s1.02 4 1.02 4" /> <path d="M14 19a2 2 0 1 0 -4 0a2 2 0 0 0 4 0" /> <path d="M14 17a2 2 0 1 1 0 -4a2 2 0 0 1 0 4" /> <path d="M10 17a2 2 0 1 1 0 -4a2 2 0 0 1 0 4" /> <path d="M12 13a2 2 0 1 1 0 -4a2 2 0 0 1 0 4" /> <path d="M16 13a2 2 0 1 1 0 -4a2 2 0 0 1 0 4" /> <path d="M8 13a2 2 0 1 1 0 -4a2 2 0 0 1 0 4" /> </svg>"##;
const GRILL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 8h-14a6 6 0 0 0 6 6h2a6 6 0 0 0 6 -5.775l0 -.225" /> <path d="M17 20a2 2 0 1 1 0 -4a2 2 0 0 1 0 4" /> <path d="M15 14l1 2" /> <path d="M9 14l-3 6" /> <path d="M15 18h-8" /> <path d="M15 5v-1" /> <path d="M12 5v-1" /> <path d="M9 5v-1" /> </svg>"##;
const GRILL_FORK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 5l11.5 11.5" /> <path d="M19.347 16.575l1.08 1.079a1.96 1.96 0 0 1 -2.773 2.772l-1.08 -1.079a1.96 1.96 0 0 1 2.773 -2.772" /> <path d="M3 7l3.05 3.15a2.9 2.9 0 0 0 4.1 -4.1l-3.15 -3.05" /> </svg>"##;
const GRILL_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 8h-3a6 6 0 0 0 6 6h2c.315 0 .624 -.024 .926 -.071m2.786 -1.214a5.99 5.99 0 0 0 2.284 -4.49l0 -.225h-7" /> <path d="M18.827 18.815a2 2 0 1 1 -2.663 -2.633" /> <path d="M9 14l-3 6" /> <path d="M15 18h-8" /> <path d="M15 5v-1" /> <path d="M12 5v-1" /> <path d="M9 5v-1" /> <path d="M3 3l18 18" /> </svg>"##;
const GRILL_SPATULA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.2 10.2l6.3 6.3" /> <path d="M19.347 16.575l1.08 1.079a1.96 1.96 0 0 1 -2.773 2.772l-1.08 -1.079a1.96 1.96 0 0 1 2.773 -2.772" /> <path d="M3 7l3.05 3.15a2.9 2.9 0 0 0 4.1 -4.1l-3.15 -3.05l-4 4" /> </svg>"##;
const HANGER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 6a2 2 0 1 0 -4 0c0 1.667 .67 3 2 4h-.008l7.971 4.428a2 2 0 0 1 1.029 1.749v.823a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-.823a2 2 0 0 1 1.029 -1.749l7.971 -4.428" /> </svg>"##;
const HANGER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14 6a2 2 0 1 0 -4 0m6.506 6.506l3.461 1.922a2 2 0 0 1 1.029 1.749v.823m-2 2h-14a2 2 0 0 1 -2 -2v-.823a2 2 0 0 1 1.029 -1.749l6.673 -3.707" /> <path d="M3 3l18 18" /> </svg>"##;
const ICE_CREAM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21.5v-4.5" /> <path d="M8 17h8v-10a4 4 0 1 0 -8 0v10" /> <path d="M8 10.5l8 -3.5" /> <path d="M8 14.5l8 -3.5" /> </svg>"##;
const ICE_CREAM_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.657 11a6 6 0 1 0 -11.315 0" /> <path d="M6.342 11l5.658 11l5.657 -11l-11.315 0" /> </svg>"##;
const ICE_CREAM_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21.5v-4.5" /> <path d="M8 8v9h8v-1m0 -4v-5a4 4 0 0 0 -7.277 -2.294" /> <path d="M8 10.5l1.74 -.76m2.79 -1.222l3.47 -1.518" /> <path d="M8 14.5l4.488 -1.964" /> <path d="M3 3l18 18" /> </svg>"##;
const IRONING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 6h7.459a3 3 0 0 1 2.959 2.507l.577 3.464l.81 4.865a1 1 0 0 1 -.985 1.164h-16.82a7 7 0 0 1 7 -7h9.8" /> </svg>"##;
const IRONING_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 6h7.459a3 3 0 0 1 2.959 2.507l.577 3.464l.81 4.865a1 1 0 0 1 -.985 1.164h-16.82a7 7 0 0 1 7 -7h9.8" /> <path d="M12 15h.01" /> </svg>"##;
const IRONING_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15h.01" /> <path d="M9 6h7.459a3 3 0 0 1 2.959 2.507l.577 3.464l.81 4.865a1 1 0 0 1 -.985 1.164h-16.82a7 7 0 0 1 7 -7h9.8" /> <path d="M14 15h.01" /> </svg>"##;
const IRONING_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 15h.01" /> <path d="M9 6h7.459a3 3 0 0 1 2.959 2.507l.577 3.464l.81 4.865a1 1 0 0 1 -.985 1.164h-16.82a7 7 0 0 1 7 -7h9.8" /> <path d="M9 15h.01" /> <path d="M15 15h.01" /> </svg>"##;
const IRONING_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 6h6.459a3 3 0 0 1 2.959 2.507l.577 3.464l.804 4.821l.007 .044m-2.806 1.164h-15a7 7 0 0 1 7 -7h1m4 0h4.8" /> <path d="M3 3l18 18" /> </svg>"##;
const IRONING_STEAM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 19v2" /> <path d="M9 4h7.459a3 3 0 0 1 2.959 2.507l.577 3.464l.81 4.865a1 1 0 0 1 -.985 1.164h-16.82a7 7 0 0 1 7 -7h9.8" /> <path d="M8 19l-1 2" /> <path d="M16 19l1 2" /> </svg>"##;
const IRONING_STEAM_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 4h7.459a3 3 0 0 1 2.959 2.507l.577 3.464l.81 4.865a1 1 0 0 1 -.821 1.15" /> <path d="M16 16h-13a7 7 0 0 1 6.056 -6.937" /> <path d="M13 9h6.8" /> <path d="M12 19v2" /> <path d="M8 19l-1 2" /> <path d="M16 19l1 2" /> <path d="M3 3l18 18" /> </svg>"##;
const JACKET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 3l-4 5l-4 -5" /> <path d="M12 19a2 2 0 0 1 -2 2h-4a2 2 0 0 1 -2 -2v-8.172a2 2 0 0 1 .586 -1.414l.828 -.828a2 2 0 0 0 .586 -1.414v-2.172a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v2.172a2 2 0 0 0 .586 1.414l.828 .828a2 2 0 0 1 .586 1.414v8.172a2 2 0 0 1 -2 2h-4a2 2 0 0 1 -2 -2" /> <path d="M20 13h-3a1 1 0 0 0 -1 1v2a1 1 0 0 0 1 1h3" /> <path d="M4 17h3a1 1 0 0 0 1 -1v-2a1 1 0 0 0 -1 -1h-3" /> <path d="M12 19v-11" /> </svg>"##;
const LEMON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17.536 3.393c3.905 3.906 3.905 10.237 0 14.143c-3.906 3.905 -10.237 3.905 -14.143 0l14.143 -14.143" /> <path d="M5.868 15.06a6.5 6.5 0 0 0 9.193 -9.192" /> <path d="M10.464 10.464l4.597 4.597" /> <path d="M10.464 10.464v6.364" /> <path d="M10.464 10.464h6.364" /> </svg>"##;
const LEMON_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 4a2 2 0 0 1 1.185 3.611c1.55 2.94 .873 6.917 -1.892 9.682c-2.765 2.765 -6.743 3.442 -9.682 1.892a2 2 0 1 1 -2.796 -2.796c-1.55 -2.94 -.873 -6.917 1.892 -9.682c2.765 -2.765 6.743 -3.442 9.682 -1.892a2 2 0 0 1 1.611 -.815" /> </svg>"##;
const LOLLIPOP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /> <path d="M21 10a3.5 3.5 0 0 0 -7 0" /> <path d="M14 10a3.5 3.5 0 0 1 -7 0" /> <path d="M14 17a3.5 3.5 0 0 0 0 -7" /> <path d="M14 3a3.5 3.5 0 0 0 0 7" /> <path d="M3 21l6 -6" /> </svg>"##;
const LOLLIPOP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.462 7.493a7 7 0 0 0 9.06 9.039m2.416 -1.57a7 7 0 1 0 -9.884 -9.915" /> <path d="M21 10a3.5 3.5 0 0 0 -7 0" /> <path d="M12.71 12.715a3.5 3.5 0 0 1 -5.71 -2.715" /> <path d="M14 17c.838 0 1.607 -.294 2.209 -.785m1.291 -2.715a3.5 3.5 0 0 0 -3.5 -3.5" /> <path d="M14 3a3.5 3.5 0 0 0 -3.5 3.5" /> <path d="M3 21l6 -6" /> <path d="M3 3l18 18" /> </svg>"##;
const MEAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.62 8.382l1.966 -1.967a2 2 0 1 1 3.414 -1.415a2 2 0 1 1 -1.413 3.414l-1.82 1.821" /> <path d="M5.904 18.596c2.733 2.734 5.9 4 7.07 2.829c1.172 -1.172 -.094 -4.338 -2.828 -7.071c-2.733 -2.734 -5.9 -4 -7.07 -2.829c-1.172 1.172 .094 4.338 2.828 7.071" /> <path d="M7.5 16l1 1" /> <path d="M12.975 21.425c3.905 -3.906 4.855 -9.288 2.121 -12.021c-2.733 -2.734 -8.115 -1.784 -12.02 2.121" /> </svg>"##;
const MEAT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.62 8.382l1.966 -1.967a2 2 0 1 1 3.414 -1.415a2 2 0 1 1 -1.413 3.414l-1.82 1.821" /> <path d="M5.904 18.596c2.733 2.734 5.9 4 7.07 2.829c1.172 -1.172 -.094 -4.338 -2.828 -7.071c-2.733 -2.734 -5.9 -4 -7.07 -2.829c-1.172 1.172 .094 4.338 2.828 7.071" /> <path d="M7.5 16l1 1" /> <path d="M12.975 21.425c1.582 -1.582 2.679 -3.407 3.242 -5.2" /> <path d="M16.6 12.6c-.16 -1.238 -.653 -2.345 -1.504 -3.195c-.85 -.85 -1.955 -1.344 -3.192 -1.503" /> <path d="M8.274 8.284c-1.792 .563 -3.616 1.66 -5.198 3.242" /> <path d="M3 3l18 18" /> </svg>"##;
const MELON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 10c0 5.523 -4.477 10 -10 10a9.967 9.967 0 0 1 -6.984 -2.842l4.343 -4.153a4 4 0 0 0 5.76 -5.51l4.342 -4.153a9.963 9.963 0 0 1 2.539 6.658" /> </svg>"##;
const MICHELIN_BIB_GOURMAND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.97 20c-2.395 -1.947 -4.763 -5.245 -1.005 -8c-.52 -4 3.442 -7.5 5.524 -7.5c.347 -1 1.499 -1.5 2.54 -1.5c1.04 0 2.135 .5 2.482 1.5c2.082 0 6.044 3.5 5.524 7.5c3.758 2.755 1.39 6.053 -1.005 8" /> <path d="M8 11a1 2 0 1 0 2 0a1 2 0 1 0 -2 0" /> <path d="M14 11a1 2 0 1 0 2 0a1 2 0 1 0 -2 0" /> <path d="M8 17.085c3.5 2.712 6.5 2.712 9 -1.085" /> <path d="M13 18.5c.815 -2.337 1.881 -1.472 2 -.55" /> </svg>"##;
const MICHELIN_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M14.792 17.063c0 .337 .057 .618 .057 .9c0 1.8 -1.238 3.037 -2.982 3.037c-1.8 0 -2.98 -1.238 -2.98 -3.206v-.731c-.957 .675 -1.576 .9 -2.42 .9c-1.518 0 -2.925 -1.463 -2.925 -3.094c0 -1.181 .844 -2.194 2.082 -2.756l.28 -.113c-1.574 -.787 -2.362 -1.688 -2.362 -2.925c0 -1.687 1.294 -3.094 2.925 -3.094c.675 0 1.52 .338 2.138 .788l.281 .112c0 -.337 -.056 -.619 -.056 -.844c0 -1.8 1.237 -3.037 2.98 -3.037c1.8 0 2.981 1.237 2.981 3.206v.394l-.056 .281c.956 -.675 1.575 -.9 2.419 -.9c1.519 0 2.925 1.463 2.925 3.094c0 1.181 -.844 2.194 -2.081 2.756l-.282 .169c1.575 .787 2.363 1.688 2.363 2.925c0 1.688 -1.294 3.094 -2.925 3.094c-.675 0 -1.575 -.281 -2.138 -.788l-.225 -.169l.001 .001" /> </svg>"##;
const MICHELIN_STAR_GREEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.432 17.949c.863 1.544 2.589 1.976 4.13 1.112c1.54 -.865 1.972 -2.594 1.048 -4.138c-.185 -.309 -.309 -.556 -.494 -.74c.247 .06 .555 .06 .925 .06c1.726 0 2.959 -1.234 2.959 -2.963c0 -1.73 -1.233 -2.965 -3.02 -2.965c-.37 0 -.617 0 -.925 .062c.185 -.185 .308 -.432 .493 -.74c.863 -1.545 .431 -3.274 -1.048 -4.138c-1.541 -.865 -3.205 -.433 -4.13 1.111c-.185 .309 -.308 .556 -.432 .803c-.123 -.247 -.246 -.494 -.431 -.803c-.802 -1.605 -2.528 -2.038 -4.007 -1.173c-1.541 .865 -1.973 2.594 -1.048 4.137c.185 .31 .308 .556 .493 .741c-.246 -.061 -.555 -.061 -.924 -.061c-1.788 0 -3.021 1.235 -3.021 2.964c0 1.729 1.233 2.964 3.02 2.964" /> <path d="M4.073 21c4.286 -2.756 5.9 -5.254 7.927 -9" /> </svg>"##;
const MICROWAVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 7a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1l0 -10" /> <path d="M15 6v12" /> <path d="M18 12h.01" /> <path d="M18 15h.01" /> <path d="M18 9h.01" /> <path d="M6.5 10.5c1 -.667 1.5 -.667 2.5 0c.833 .347 1.667 .926 2.5 0" /> <path d="M6.5 13.5c1 -.667 1.5 -.667 2.5 0c.833 .347 1.667 .926 2.5 0" /> </svg>"##;
const MICROWAVE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M18 18h-14a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h2m4 0h10a1 1 0 0 1 1 1v10" /> <path d="M15 6v5m0 4v3" /> <path d="M18 12h.01" /> <path d="M18 9h.01" /> <path d="M6.5 10.5c1 -.667 1.5 -.667 2.5 0c.636 .265 1.272 .665 1.907 .428" /> <path d="M6.5 13.5c1 -.667 1.5 -.667 2.5 0c.833 .347 1.667 .926 2.5 0" /> <path d="M3 3l18 18" /> </svg>"##;
const MILK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 6h8v-2a1 1 0 0 0 -1 -1h-6a1 1 0 0 0 -1 1v2" /> <path d="M16 6l1.094 1.759a6 6 0 0 1 .906 3.17v8.071a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-8.071a6 6 0 0 1 .906 -3.17l1.094 -1.759" /> <path d="M10 16a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M10 10h4" /> </svg>"##;
const MILK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 6h6v-2a1 1 0 0 0 -1 -1h-6a1 1 0 0 0 -1 1" /> <path d="M16 6l1.094 1.759a6 6 0 0 1 .906 3.17v3.071m0 4v1a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-8.071a6 6 0 0 1 .906 -3.17l.327 -.525" /> <path d="M10 16a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M3 3l18 18" /> </svg>"##;
const MILKSHAKE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 10a5 5 0 0 0 -10 0" /> <path d="M6 11a1 1 0 0 1 1 -1h10a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1l0 -1" /> <path d="M7 13l1.81 7.243a1 1 0 0 0 .97 .757h4.44a1 1 0 0 0 .97 -.757l1.81 -7.243" /> <path d="M12 5v-2" /> </svg>"##;
const MONEYBAG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 3h5a1.5 1.5 0 0 1 1.5 1.5a3.5 3.5 0 0 1 -3.5 3.5h-1a3.5 3.5 0 0 1 -3.5 -3.5a1.5 1.5 0 0 1 1.5 -1.5" /> <path d="M4 17v-1a8 8 0 1 1 16 0v1a4 4 0 0 1 -4 4h-8a4 4 0 0 1 -4 -4" /> </svg>"##;
const MONEYBAG_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 3h5a1.5 1.5 0 0 1 1.5 1.5a3.5 3.5 0 0 1 -3.5 3.5h-1a3.5 3.5 0 0 1 -3.5 -3.5a1.5 1.5 0 0 1 1.5 -1.5" /> <path d="M11 21h-3a4 4 0 0 1 -4 -4v-1a8 8 0 0 1 14.376 -4.833" /> <path d="M18.42 15.61a2.1 2.1 0 1 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const MONEYBAG_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 3h5a1.5 1.5 0 0 1 1.5 1.5a3.5 3.5 0 0 1 -3.5 3.5h-1a3.5 3.5 0 0 1 -3.5 -3.5a1.5 1.5 0 0 1 1.5 -1.5" /> <path d="M11.5 21h-3.5a4 4 0 0 1 -4 -4v-1a8 8 0 0 1 14.376 -4.833" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.24 2.24 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.24 2.24 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const MONEYBAG_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 3h5a1.5 1.5 0 0 1 1.5 1.5a3.5 3.5 0 0 1 -3.5 3.5h-1a3.5 3.5 0 0 1 -3.5 -3.5a1.5 1.5 0 0 1 1.5 -1.5" /> <path d="M12.5 21h-4.5a4 4 0 0 1 -4 -4v-1a8 8 0 0 1 15.943 -.958" /> <path d="M16 19h6" /> </svg>"##;
const MONEYBAG_MOVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 3h5a1.5 1.5 0 0 1 1.5 1.5a3.5 3.5 0 0 1 -3.5 3.5h-1a3.5 3.5 0 0 1 -3.5 -3.5a1.5 1.5 0 0 1 1.5 -1.5" /> <path d="M12.5 21h-4.5a4 4 0 0 1 -4 -4v-1a8 8 0 0 1 14.946 -3.971" /> <path d="M16 19h6" /> <path d="M19 16l3 3l-3 3" /> </svg>"##;
const MONEYBAG_MOVE_BACK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 3h5a1.5 1.5 0 0 1 1.5 1.5a3.5 3.5 0 0 1 -3.5 3.5h-1a3.5 3.5 0 0 1 -3.5 -3.5a1.5 1.5 0 0 1 1.5 -1.5" /> <path d="M12.5 21h-4.5a4 4 0 0 1 -4 -4v-1a8 8 0 0 1 14.953 -3.959" /> <path d="M16 19h6" /> <path d="M19 16l-3 3l3 3" /> </svg>"##;
const MONEYBAG_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.5 3h5a1.5 1.5 0 0 1 1.5 1.5a3.5 3.5 0 0 1 -3.5 3.5h-1a3.5 3.5 0 0 1 -3.5 -3.5a1.5 1.5 0 0 1 1.5 -1.5" /> <path d="M12.5 21h-4.5a4 4 0 0 1 -4 -4v-1a8 8 0 0 1 14.935 -3.991" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const MUG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4.083 5h10.834a1.08 1.08 0 0 1 1.083 1.077v8.615c0 2.38 -1.94 4.308 -4.333 4.308h-4.334c-2.393 0 -4.333 -1.929 -4.333 -4.308v-8.615a1.08 1.08 0 0 1 1.083 -1.077" /> <path d="M16 8h2.5c1.38 0 2.5 1.045 2.5 2.333v2.334c0 1.288 -1.12 2.333 -2.5 2.333h-2.5" /> </svg>"##;
const MUG_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 5h5.917a1.08 1.08 0 0 1 1.083 1.077v5.923m-.167 3.88a4.33 4.33 0 0 1 -4.166 3.12h-4.334c-2.393 0 -4.333 -1.929 -4.333 -4.308v-8.615a1.08 1.08 0 0 1 1.083 -1.077h.917" /> <path d="M16 8h2.5c1.38 0 2.5 1.045 2.5 2.333v2.334c0 1.148 -.89 2.103 -2.06 2.297" /> <path d="M3 3l18 18" /> </svg>"##;
const MUSHROOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 11.1c0 -4.474 -3.582 -8.1 -8 -8.1s-8 3.626 -8 8.1a.9 .9 0 0 0 .9 .9h14.2a.9 .9 0 0 0 .9 -.9" /> <path d="M10 12v7a2 2 0 1 0 4 0v-7" /> </svg>"##;
const MUSHROOM_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.874 5.89a8.128 8.128 0 0 0 -1.874 5.21a.9 .9 0 0 0 .9 .9h7.1m4 0h3.1a.9 .9 0 0 0 .9 -.9c0 -4.474 -3.582 -8.1 -8 -8.1c-1.43 0 -2.774 .38 -3.936 1.047" /> <path d="M10 12v7a2 2 0 1 0 4 0v-5" /> <path d="M3 3l18 18" /> </svg>"##;
const NUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 6.84a2.007 2.007 0 0 1 1 1.754v6.555c0 .728 -.394 1.4 -1.03 1.753l-6 3.844a1.995 1.995 0 0 1 -1.94 0l-6 -3.844a2.006 2.006 0 0 1 -1.03 -1.752v-6.557c0 -.728 .394 -1.399 1.03 -1.753l6 -3.582a2.049 2.049 0 0 1 2 0l6 3.582h-.03" /> <path d="M9 12a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> </svg>"##;
const PACKAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 3l8 4.5l0 9l-8 4.5l-8 -4.5l0 -9l8 -4.5" /> <path d="M12 12l8 -4.5" /> <path d="M12 12l0 9" /> <path d="M12 12l-8 -4.5" /> <path d="M16 5.25l-8 4.5" /> </svg>"##;
const PACKAGE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.812 4.793l3.188 -1.793l8 4.5v8.5m-2.282 1.784l-5.718 3.216l-8 -4.5v-9l2.223 -1.25" /> <path d="M14.543 10.57l5.457 -3.07" /> <path d="M12 12v9" /> <path d="M12 12l-8 -4.5" /> <path d="M16 5.25l-4.35 2.447m-2.564 1.442l-1.086 .611" /> <path d="M3 3l18 18" /> </svg>"##;
const PAPER_BAG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 3h8a2 2 0 0 1 2 2v1.82a5 5 0 0 0 .528 2.236l.944 1.888a5 5 0 0 1 .528 2.236v5.82a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-5.82a5 5 0 0 1 .528 -2.236l1.472 -2.944v-3a2 2 0 0 1 2 -2" /> <path d="M12 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M6 21a2 2 0 0 0 2 -2v-5.82a5 5 0 0 0 -.528 -2.236l-1.472 -2.944" /> <path d="M11 7h2" /> </svg>"##;
const PAPER_BAG_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.158 3.185c.256 -.119 .542 -.185 .842 -.185h8a2 2 0 0 1 2 2v1.82a5 5 0 0 0 .528 2.236l.944 1.888a5 5 0 0 1 .528 2.236v2.82m-.177 3.824a2 2 0 0 1 -1.823 1.176h-12a2 2 0 0 1 -2 -2v-5.82a5 5 0 0 1 .528 -2.236l1.472 -2.944v-2" /> <path d="M13.185 13.173a2 2 0 1 0 2.64 2.647" /> <path d="M6 21a2 2 0 0 0 2 -2v-5.82a5 5 0 0 0 -.528 -2.236l-1.472 -2.944" /> <path d="M11 7h2" /> <path d="M3 3l18 18" /> </svg>"##;
const PEPPER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 11c0 2.21 -2.239 4 -5 4s-5 -1.79 -5 -4a8 8 0 1 0 16 0a3 3 0 0 0 -6 0" /> <path d="M16 8c0 -2 2 -4 4 -4" /> </svg>"##;
const PEPPER_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.59 12.59c-.77 1.418 -2.535 2.41 -4.59 2.41c-2.761 0 -5 -1.79 -5 -4a8 8 0 0 0 13.643 5.67m1.64 -2.357a7.97 7.97 0 0 0 .717 -3.313a3 3 0 0 0 -5.545 -1.59" /> <path d="M16 8c0 -2 2 -4 4 -4" /> <path d="M3 3l18 18" /> </svg>"##;
const PERFUME_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 6v3" /> <path d="M14 6v3" /> <path d="M5 11a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2l0 -8" /> <path d="M10 15a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M9 3h6v3h-6l0 -3" /> </svg>"##;
const PIZZA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 21.5c-3.04 0 -5.952 -.714 -8.5 -1.983l8.5 -16.517l8.5 16.517a19.09 19.09 0 0 1 -8.5 1.983" /> <path d="M5.38 15.866a14.94 14.94 0 0 0 6.815 1.634a14.944 14.944 0 0 0 6.502 -1.479" /> <path d="M13 11.01v-.01" /> <path d="M11 14v-.01" /> </svg>"##;
const PIZZA_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.313 6.277l1.687 -3.277l5.34 10.376m2.477 6.463a19.093 19.093 0 0 1 -7.817 1.661c-3.04 0 -5.952 -.714 -8.5 -1.983l5.434 -10.559" /> <path d="M5.38 15.866a14.94 14.94 0 0 0 6.815 1.634c1.56 0 3.105 -.24 4.582 -.713" /> <path d="M11 14v-.01" /> <path d="M3 3l18 18" /> </svg>"##;
const RECEIPT_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16l-3 -2l-2 2l-2 -2l-2 2l-2 -2l-3 2" /> <path d="M9 7h4.09c1.055 0 1.91 .895 1.91 2s-.855 2 -1.91 2c1.055 0 1.91 .895 1.91 2s-.855 2 -1.91 2h-4.09" /> <path d="M10 11h4" /> <path d="M10 6v10v-9" /> <path d="M13 6v1" /> <path d="M13 15v1" /> </svg>"##;
const RECEIPT_EURO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16l-3 -2l-2 2l-2 -2l-2 2l-2 -2l-3 2" /> <path d="M15 7.8c-.523 -.502 -1.172 -.8 -1.875 -.8c-1.727 0 -3.125 1.791 -3.125 4s1.398 4 3.125 4c.703 0 1.352 -.298 1.874 -.8" /> <path d="M9 11h4" /> </svg>"##;
const RECEIPT_POUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16l-3 -2l-2 2l-2 -2l-2 2l-2 -2l-3 2" /> <path d="M15 9a2 2 0 1 0 -4 0v4a2 2 0 0 1 -2 2h6" /> <path d="M9 12h4" /> </svg>"##;
const RECEIPT_RUPEE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16l-3 -2l-2 2l-2 -2l-2 2l-2 -2l-3 2" /> <path d="M15 7h-6h1a3 3 0 0 1 0 6h-1l3 3" /> <path d="M9 10h6" /> </svg>"##;
const RECEIPT_YEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16l-3 -2l-2 2l-2 -2l-2 2l-2 -2l-3 2" /> <path d="M9 11h6" /> <path d="M9 14h6" /> <path d="M9 7l3 4.5" /> <path d="M15 7l-3 4.5v4.5" /> </svg>"##;
const RECEIPT_YUAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 21v-16a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v16l-3 -2l-2 2l-2 -2l-2 2l-2 -2l-3 2" /> <path d="M9 12h6" /> <path d="M9 7l3 4.5" /> <path d="M15 7l-3 4.5v4.5" /> </svg>"##;
const ROSETTE_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l6 -6" /> <path d="M9 9.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M14 14.5a.5 .5 0 1 0 1 0a.5 .5 0 1 0 -1 0" fill="currentColor" /> <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7a2.2 2.2 0 0 0 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1a2.2 2.2 0 0 0 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1" /> </svg>"##;
const ROSETTE_DISCOUNT_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 7.2a2.2 2.2 0 0 1 2.2 -2.2h1a2.2 2.2 0 0 0 1.55 -.64l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.64 1.55v1a2.2 2.2 0 0 1 -2.2 2.2h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1" /> <path d="M9 12l2 2l4 -4" /> </svg>"##;
const ROSETTE_DISCOUNT_CHECK_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 12l2 2l1.5 -1.5m2 -2l.5 -.5" /> <path d="M8.887 4.89a2.2 2.2 0 0 0 .863 -.53l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.528 .858m-.757 3.248a2.193 2.193 0 0 1 -1.555 .644h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1c0 -.604 .244 -1.152 .638 -1.55" /> <path d="M3 3l18 18" /> </svg>"##;
const ROSETTE_DISCOUNT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15l3 -3m2 -2l1 -1" /> <path d="M9.148 9.145a.498 .498 0 0 0 .352 .855a.5 .5 0 0 0 .35 -.142" /> <path d="M14.148 14.145a.498 .498 0 0 0 .352 .855a.5 .5 0 0 0 .35 -.142" /> <path d="M8.887 4.89a2.2 2.2 0 0 0 .863 -.53l.7 -.7a2.2 2.2 0 0 1 3.12 0l.7 .7c.412 .41 .97 .64 1.55 .64h1a2.2 2.2 0 0 1 2.2 2.2v1c0 .58 .23 1.138 .64 1.55l.7 .7a2.2 2.2 0 0 1 0 3.12l-.7 .7a2.2 2.2 0 0 0 -.528 .858m-.757 3.248a2.193 2.193 0 0 1 -1.555 .644h-1a2.2 2.2 0 0 0 -1.55 .64l-.7 .7a2.2 2.2 0 0 1 -3.12 0l-.7 -.7a2.2 2.2 0 0 0 -1.55 -.64h-1a2.2 2.2 0 0 1 -2.2 -2.2v-1a2.2 2.2 0 0 0 -.64 -1.55l-.7 -.7a2.2 2.2 0 0 1 0 -3.12l.7 -.7a2.2 2.2 0 0 0 .64 -1.55v-1c0 -.604 .244 -1.152 .638 -1.55" /> <path d="M3 3l18 18" /> </svg>"##;
const SALAD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 11h16a1 1 0 0 1 1 1v.5c0 1.5 -2.517 5.573 -4 6.5v1a1 1 0 0 1 -1 1h-8a1 1 0 0 1 -1 -1v-1c-1.687 -1.054 -4 -5 -4 -6.5v-.5a1 1 0 0 1 1 -1" /> <path d="M18.5 11c.351 -1.017 .426 -2.236 .5 -3.714v-1.286h-2.256c-2.83 0 -4.616 .804 -5.64 2.076" /> <path d="M5.255 11.008a12.204 12.204 0 0 1 -.255 -2.008v-1h1.755c.98 0 1.801 .124 2.479 .35" /> <path d="M8 8l1 -4l4 2.5" /> <path d="M13 11v-.5a2.5 2.5 0 1 0 -5 0v.5" /> </svg>"##;
const SALT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 13v.01" /> <path d="M10 16v.01" /> <path d="M14 16v.01" /> <path d="M7.5 8h9l-.281 -2.248a2 2 0 0 0 -1.985 -1.752h-4.468a2 2 0 0 0 -1.986 1.752l-.28 2.248" /> <path d="M7.5 8l-1.612 9.671a2 2 0 0 0 1.973 2.329h8.278a2 2 0 0 0 1.973 -2.329l-1.612 -9.671" /> </svg>"##;
const SAUSAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5.5 5.5a2.5 2.5 0 0 0 -2.5 2.5c0 7.18 5.82 13 13 13a2.5 2.5 0 1 0 0 -5a8 8 0 0 1 -8 -8a2.5 2.5 0 0 0 -2.5 -2.5" /> <path d="M5.195 5.519l-1.243 -1.989a1 1 0 0 1 .848 -1.53h1.392a1 1 0 0 1 .848 1.53l-1.245 1.99" /> <path d="M18.482 18.225l1.989 -1.243a1 1 0 0 1 1.53 .848v1.392a1 1 0 0 1 -1.53 .848l-1.991 -1.245" /> </svg>"##;
const SHIRT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 4l6 2v5h-3v8a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1v-8h-3v-5l6 -2a3 3 0 0 0 6 0" /> </svg>"##;
const SHIRT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.243 4.252l.757 -.252c0 .43 .09 .837 .252 1.206m1.395 1.472a3 3 0 0 0 4.353 -2.678l6 2v5h-3v3m0 4v1a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1v-8h-3v-5l2.26 -.753" /> <path d="M3 3l18 18" /> </svg>"##;
const SHIRT_SPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 4l6 2v5h-3v8a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1v-8h-3v-5l6 -2a3 3 0 0 0 6 0" /> <path d="M10.5 11h2.5l-1.5 5" /> </svg>"##;
const SHOE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 6h5.426a1 1 0 0 1 .863 .496l1.064 1.823a3 3 0 0 0 1.896 1.407l4.677 1.114a4 4 0 0 1 3.074 3.89v2.27a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1" /> <path d="M14 13l1 -2" /> <path d="M8 18v-1a4 4 0 0 0 -4 -4h-1" /> <path d="M10 12l1.5 -3" /> </svg>"##;
const SHOE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13.846 9.868l4.08 .972a4 4 0 0 1 3.074 3.89v2.27m-3 1h-14a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h2" /> <path d="M8 18v-1a4 4 0 0 0 -4 -4h-1" /> <path d="M10 12l.663 -1.327" /> <path d="M3 3l18 18" /> </svg>"##;
const SHOPPING_BAG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.331 8h11.339a2 2 0 0 1 1.977 2.304l-1.255 8.152a3 3 0 0 1 -2.966 2.544h-6.852a3 3 0 0 1 -2.965 -2.544l-1.255 -8.152a2 2 0 0 1 1.977 -2.304" /> <path d="M9 11v-5a3 3 0 0 1 6 0v5" /> </svg>"##;
const SHOPPING_BAG_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-2.926a3 3 0 0 1 -2.965 -2.544l-1.255 -8.152a2 2 0 0 1 1.977 -2.304h11.339a2 2 0 0 1 1.977 2.304l-.5 3.248" /> <path d="M9 11v-5a3 3 0 0 1 6 0v5" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const SHOPPING_BAG_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-3.926a3 3 0 0 1 -2.965 -2.544l-1.255 -8.152a2 2 0 0 1 1.977 -2.304h11.339a2 2 0 0 1 1.977 2.304l-.416 2.7" /> <path d="M9 11v-5a3 3 0 0 1 6 0v5" /> <path d="M16 21l5 -5" /> <path d="M21 21v.01" /> <path d="M16 16v.01" /> </svg>"##;
const SHOPPING_BAG_EDIT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11 21h-2.426a3 3 0 0 1 -2.965 -2.544l-1.255 -8.152a2 2 0 0 1 1.977 -2.304h11.339a2 2 0 0 1 1.977 2.304l-.109 .707" /> <path d="M9 11v-5a3 3 0 0 1 6 0v5" /> <path d="M18.42 15.61a2.1 2.1 0 0 1 2.97 2.97l-3.39 3.42h-3v-3l3.42 -3.39" /> </svg>"##;
const SHOPPING_BAG_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 21h-6.426a3 3 0 0 1 -2.965 -2.544l-1.255 -8.152a2 2 0 0 1 1.977 -2.304h11.339a2 2 0 0 1 1.977 2.304l-.258 1.678" /> <path d="M9 11v-5a3 3 0 0 1 6 0v5" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const SHOPPING_BAG_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-2.926a3 3 0 0 1 -2.965 -2.544l-1.255 -8.152a2 2 0 0 1 1.977 -2.304h11.339a2 2 0 0 1 1.977 2.304c-.057 .368 -.1 .644 -.127 .828" /> <path d="M9 11v-5a3 3 0 0 1 6 0v5" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const SHOPPING_BAG_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-3.926a3 3 0 0 1 -2.965 -2.544l-1.255 -8.152a2 2 0 0 1 1.977 -2.304h11.339a2 2 0 0 1 1.977 2.304l-.73 4.744" /> <path d="M9 11v-5a3 3 0 0 1 6 0v5" /> <path d="M16 19h6" /> </svg>"##;
const SHOPPING_BAG_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.5 21h-3.926a3 3 0 0 1 -2.965 -2.544l-1.255 -8.152a2 2 0 0 1 1.977 -2.304h11.339a2 2 0 0 1 1.977 2.304l-.263 1.708" /> <path d="M16 19h6" /> <path d="M19 16v6" /> <path d="M9 11v-5a3 3 0 0 1 6 0v5" /> </svg>"##;
const SHOPPING_BAG_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M11.5 21h-2.926a3 3 0 0 1 -2.965 -2.544l-1.255 -8.152a2 2 0 0 1 1.977 -2.304h11.339a2 2 0 0 1 1.977 2.304l-.117 .761" /> <path d="M9 11v-5a3 3 0 0 1 6 0v5" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const SHOPPING_BAG_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 21h-4.426a3 3 0 0 1 -2.965 -2.544l-1.255 -8.152a2 2 0 0 1 1.977 -2.304h11.339a2 2 0 0 1 1.977 2.304l-.506 3.287" /> <path d="M9 11v-5a3 3 0 0 1 6 0v5" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const SHOPPING_CART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 17h-11v-14h-2" /> <path d="M6 5l14 1l-1 7h-13" /> </svg>"##;
const SHOPPING_CART_BOLT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M13.5 17h-7.5v-14h-2" /> <path d="M6 5l14 1l-.858 6.004m-2.642 .996h-10.5" /> <path d="M19 16l-2 3h4l-2 3" /> </svg>"##;
const SHOPPING_CART_CANCEL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 17h-6v-14h-2" /> <path d="M6 5l14 1l-.857 5.998m-3.643 1.002h-9.5" /> <path d="M16 19a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M17 21l4 -4" /> </svg>"##;
const SHOPPING_CART_CHECK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M11.5 17h-5.5v-14h-2" /> <path d="M6 5l14 1l-1 7h-13" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const SHOPPING_CART_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M11.5 17h-5.5v-14h-2" /> <path d="M6 5l14 1l-1 7h-13" /> <path d="M20 21l2 -2l-2 -2" /> <path d="M17 17l-2 2l2 2" /> </svg>"##;
const SHOPPING_CART_COG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 17h-6v-14h-2" /> <path d="M6 5l14 1l-.79 5.526m-3.21 1.474h-10" /> <path d="M17.001 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M19.001 15.5v1.5" /> <path d="M19.001 21v1.5" /> <path d="M22.032 17.25l-1.299 .75" /> <path d="M17.27 20l-1.3 .75" /> <path d="M15.97 17.25l1.3 .75" /> <path d="M20.733 20l1.3 .75" /> </svg>"##;
const SHOPPING_CART_COPY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M11.5 17h-5.5v-14h-2" /> <path d="M6 5l14 1l-1 7h-13" /> <path d="M15 19l2 2l4 -4" /> </svg>"##;
const SHOPPING_CART_DISCOUNT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12.5 17h-6.5v-14h-2" /> <path d="M6 5l14 1l-.859 6.011m-6.141 .989h-7" /> <path d="M16 21l5 -5" /> <path d="M21 21v.01" /> <path d="M16 16v.01" /> </svg>"##;
const SHOPPING_CART_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M13 17h-7v-14h-2" /> <path d="M6 5l14 1l-.575 4.022m-4.925 2.978h-8.5" /> <path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M19 21v1m0 -8v1" /> </svg>"##;
const SHOPPING_CART_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12.5 17h-6.5v-14h-2" /> <path d="M6 5l14 1l-.859 6.011m-2.641 .989h-10.5" /> <path d="M19 16v6" /> <path d="M22 19l-3 3l-3 -3" /> </svg>"##;
const SHOPPING_CART_EXCLAMATION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M15 17h-9v-14h-2" /> <path d="M6 5l14 1l-.854 5.976m-2.646 1.024h-10.5" /> <path d="M19 16v3" /> <path d="M19 22v.01" /> </svg>"##;
const SHOPPING_CART_HEART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M10 17h-4v-14h-2" /> <path d="M6 5l14 1l-.717 5.016m-7.783 1.984h-5.5" /> <path d="M18 22l3.35 -3.284a2.143 2.143 0 0 0 .005 -3.071a2.242 2.242 0 0 0 -3.129 -.006l-.224 .22l-.223 -.22a2.242 2.242 0 0 0 -3.128 -.006a2.143 2.143 0 0 0 -.006 3.071l3.355 3.296" /> </svg>"##;
const SHOPPING_CART_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12.5 17h-6.5v-14h-2" /> <path d="M6 5l14 1l-1 7h-13" /> <path d="M16 19h6" /> </svg>"##;
const SHOPPING_CART_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M17 17a2 2 0 1 0 2 2" /> <path d="M17 17h-11v-11" /> <path d="M9.239 5.231l10.761 .769l-1 7h-2m-4 0h-7" /> <path d="M3 3l18 18" /> </svg>"##;
const SHOPPING_CART_PAUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M13 17h-7v-14h-2" /> <path d="M6 5l14 1l-1 7h-13" /> <path d="M17 17v5" /> <path d="M21 17v5" /> </svg>"##;
const SHOPPING_CART_PIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12 17h-6v-14h-2" /> <path d="M6 5l14 1l-.716 5.011m-5.284 1.989h-8" /> <path d="M21.121 20.121a3 3 0 1 0 -4.242 0c.418 .419 1.125 1.045 2.121 1.879c1.051 -.89 1.759 -1.516 2.121 -1.879" /> <path d="M19 18v.01" /> </svg>"##;
const SHOPPING_CART_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12.5 17h-6.5v-14h-2" /> <path d="M6 5l14 1l-.86 6.017m-2.64 .983h-10.5" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const SHOPPING_CART_QUESTION_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M13.5 17h-7.5v-14h-2" /> <path d="M6 5l14 1l-.714 5m-4.786 2h-8.5" /> <path d="M19 22v.01" /> <path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> </svg>"##;
const SHOPPING_CART_SEARCH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M11 17h-5v-14h-2" /> <path d="M6 5l14 1l-.718 5.023m-6.282 1.977h-7" /> <path d="M15 18a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /> <path d="M20.2 20.2l1.8 1.8" /> </svg>"##;
const SHOPPING_CART_SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12.5 17h-6.5v-14h-2" /> <path d="M6 5l14 1l-1 7h-13" /> <path d="M16 22l5 -5" /> <path d="M21 21.5v-4.5h-4.5" /> </svg>"##;
const SHOPPING_CART_STAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M9.5 17h-3.5v-14h-2" /> <path d="M6 5l14 1l-.615 4.302m-6.885 2.698h-6.5" /> <path d="M17.8 20.817l-2.172 1.138a.392 .392 0 0 1 -.568 -.41l.415 -2.411l-1.757 -1.707a.389 .389 0 0 1 .217 -.665l2.428 -.352l1.086 -2.193a.392 .392 0 0 1 .702 0l1.086 2.193l2.428 .352a.39 .39 0 0 1 .217 .665l-1.757 1.707l.414 2.41a.39 .39 0 0 1 -.567 .411l-2.172 -1.138" /> </svg>"##;
const SHOPPING_CART_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M12.5 17h-6.5v-14h-2" /> <path d="M6 5l14 1l-.854 5.977m-2.646 1.023h-10.5" /> <path d="M19 22v-6" /> <path d="M22 19l-3 -3l-3 3" /> </svg>"##;
const SHOPPING_CART_X_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /> <path d="M13 17h-7v-14h-2" /> <path d="M6 5l14 1l-1 7h-13" /> <path d="M22 22l-5 -5" /> <path d="M17 22l5 -5" /> </svg>"##;
const SOCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M13 3v6l4.798 5.142a4 4 0 0 1 -5.441 5.86l-6.736 -6.41a2 2 0 0 1 -.621 -1.451v-9.141h8" /> <path d="M7.895 15.768c.708 -.721 1.105 -1.677 1.105 -2.768a4 4 0 0 0 -4 -4" /> </svg>"##;
const SOUP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 11h16a1 1 0 0 1 1 1v.5c0 1.5 -2.517 5.573 -4 6.5v1a1 1 0 0 1 -1 1h-8a1 1 0 0 1 -1 -1v-1c-1.687 -1.054 -4 -5 -4 -6.5v-.5a1 1 0 0 1 1 -1" /> <path d="M12 4a2.4 2.4 0 0 0 -1 2a2.4 2.4 0 0 0 1 2" /> <path d="M16 4a2.4 2.4 0 0 0 -1 2a2.4 2.4 0 0 0 1 2" /> <path d="M8 4a2.4 2.4 0 0 0 -1 2a2.4 2.4 0 0 0 1 2" /> </svg>"##;
const SOUP_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 19h16" /> <path d="M15 11h6c0 1.691 -.525 3.26 -1.42 4.552m-2.034 2.032a7.963 7.963 0 0 1 -4.546 1.416h-2a8 8 0 0 1 -8 -8h8" /> <path d="M12 5v3" /> <path d="M15 5v3" /> <path d="M3 3l18 18" /> </svg>"##;
const TAG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.5 7.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3 6v5.172a2 2 0 0 0 .586 1.414l7.71 7.71a2.41 2.41 0 0 0 3.408 0l5.592 -5.592a2.41 2.41 0 0 0 0 -3.408l-7.71 -7.71a2 2 0 0 0 -1.414 -.586h-5.172a3 3 0 0 0 -3 3" /> </svg>"##;
const TAG_MINUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.5 7.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M18.898 16.102l.699 -.699l.699 -.699c.941 -.941 .941 -2.467 0 -3.408l-7.71 -7.71c-.375 -.375 -.884 -.586 -1.414 -.586h-5.172c-1.657 0 -3 1.343 -3 3v5.172c0 .53 .211 1.039 .586 1.414l7.71 7.71c.471 .47 1.087 .706 1.704 .706" /> <path d="M16 19h6" /> </svg>"##;
const TAG_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7.149 7.144a.498 .498 0 0 0 .351 .856a.498 .498 0 0 0 .341 -.135" /> <path d="M3.883 3.875a2.99 2.99 0 0 0 -.883 2.125v5.172a2 2 0 0 0 .586 1.414l7.71 7.71a2.41 2.41 0 0 0 3.408 0l2.796 -2.796m2.005 -2.005l.79 -.79a2.41 2.41 0 0 0 0 -3.41l-7.71 -7.71a2 2 0 0 0 -1.412 -.585h-4.173" /> <path d="M3 3l18 18" /> </svg>"##;
const TAG_PLUS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.5 7.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M21.002 13c0 -.617 -.235 -1.233 -.706 -1.704l-7.71 -7.71c-.375 -.375 -.884 -.586 -1.414 -.586h-5.172c-1.657 0 -3 1.343 -3 3v5.172c0 .53 .211 1.039 .586 1.414l7.71 7.71c.471 .47 1.087 .706 1.704 .706" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"##;
const TAG_STARRED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6.5 7.5a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <path d="M3 6v5.172a2 2 0 0 0 .586 1.414l7.71 7.71a2.41 2.41 0 0 0 3.408 0l5.592 -5.592a2.41 2.41 0 0 0 0 -3.408l-7.71 -7.71a2 2 0 0 0 -1.414 -.586h-5.172a3 3 0 0 0 -3 3" /> <path d="M12.5 13.847l-1.5 1.153l.532 -1.857l-1.532 -1.143h1.902l.598 -1.8l.598 1.8h1.902l-1.532 1.143l.532 1.857l-1.5 -1.153" /> </svg>"##;
const TAGS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 8v4.172a2 2 0 0 0 .586 1.414l5.71 5.71a2.41 2.41 0 0 0 3.408 0l3.592 -3.592a2.41 2.41 0 0 0 0 -3.408l-5.71 -5.71a2 2 0 0 0 -1.414 -.586h-4.172a2 2 0 0 0 -2 2" /> <path d="M18 19l1.592 -1.592a4.82 4.82 0 0 0 0 -6.816l-4.592 -4.592" /> <path d="M7 10h-.01" /> </svg>"##;
const TAGS_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16.296 12.296l-5.71 -5.71" /> <path d="M6 6h-1a2 2 0 0 0 -2 2v4.172a2 2 0 0 0 .586 1.414l5.71 5.71a2.41 2.41 0 0 0 3.408 0l3.278 -3.278" /> <path d="M18 19l.496 -.496" /> <path d="M20.384 16.367a4.822 4.822 0 0 0 -.792 -5.775l-4.592 -4.592" /> <path d="M7 10h-.01" /> <path d="M3 3l18 18" /> </svg>"##;
const TAIWAN_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 19a4 4 0 0 0 4 -4v-7" /> <path d="M14 8v10a1 1 0 0 0 1.45 .89l2.55 -1.27" /> <path d="M6 5h12" /> <path d="M6 8h12" /> </svg>"##;
const TAX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.487 21h7.026a4 4 0 0 0 3.808 -5.224l-1.706 -5.306a5 5 0 0 0 -4.76 -3.47h-1.71a5 5 0 0 0 -4.76 3.47l-1.706 5.306a4 4 0 0 0 3.808 5.224" /> <path d="M15 3q -1 4 -3 4t -3 -4l6 0" /> <path d="M14 11h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /> <path d="M12 10v1" /> <path d="M12 17v1" /> </svg>"##;
const TAX_EURO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.487 21h7.026a4 4 0 0 0 3.808 -5.224l-1.706 -5.306a5 5 0 0 0 -4.76 -3.47h-1.71a5 5 0 0 0 -4.76 3.47l-1.706 5.306a4 4 0 0 0 3.808 5.224" /> <path d="M15 3q -1 4 -3 4t -3 -4l6 0" /> <path d="M12 14h-3" /> <path d="M14 11.172a3 3 0 1 0 0 5.656" /> </svg>"##;
const TAX_POUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8.487 21h7.026a4 4 0 0 0 3.808 -5.224l-1.706 -5.306a5 5 0 0 0 -4.76 -3.47h-1.71a5 5 0 0 0 -4.76 3.47l-1.706 5.306a4 4 0 0 0 3.808 5.224" /> <path d="M15 3q -1 4 -3 4t -3 -4l6 0" /> <path d="M14 11h-1a2 2 0 0 0 -2 2v2c0 1.105 -.395 2 -1.5 2h4.5" /> <path d="M10 14h3" /> </svg>"##;
const TEAPOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.29 3h3.42a2 2 0 0 1 1.988 1.78l1.555 14a2 2 0 0 1 -1.988 2.22h-6.53a2 2 0 0 1 -1.988 -2.22l1.555 -14a2 2 0 0 1 1.988 -1.78" /> <path d="M7.47 12.5l-4.257 -5.019a.899 .899 0 0 1 .69 -1.481h13.09a3 3 0 0 1 3.007 3v3c0 1.657 -1.346 3 -3.007 3" /> <path d="M7 17h10" /> </svg>"##;
const TIE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 22l4 -4l-2.5 -11l.993 -2.649a1 1 0 0 0 -.936 -1.351h-3.114a1 1 0 0 0 -.936 1.351l.993 2.649l-2.5 11l4 4" /> <path d="M10.5 7h3l5 5.5" /> </svg>"##;
const TOOLS_KITCHEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 3h8l-1 9h-6l-1 -9" /> <path d="M7 18h2v3h-2l0 -3" /> <path d="M20 3v12h-5c-.023 -3.681 .184 -7.406 5 -12" /> <path d="M20 15v6h-1v-3" /> <path d="M8 12l0 6" /> </svg>"##;
const TOOLS_KITCHEN_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 3v12h-5c-.023 -3.681 .184 -7.406 5 -12m0 12v6h-1v-3m-10 -14v17m-3 -17v3a3 3 0 1 0 6 0v-3" /> </svg>"##;
const TOOLS_KITCHEN_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M7 4v17m-3 -17v3a3 3 0 1 0 6 0v-3" /> <path d="M14 8a3 4 0 1 0 6 0a3 4 0 1 0 -6 0" /> <path d="M17 12v9" /> </svg>"##;
const TRANSACTION_BITCOIN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 12h4.09c1.055 0 1.91 .895 1.91 2s-.855 2 -1.91 2c1.055 0 1.91 .895 1.91 2s-.855 2 -1.91 2h-4.09" /> <path d="M16 16h4" /> <path d="M16 11v10v-9" /> <path d="M19 11v1" /> <path d="M19 20v1" /> <path d="M3 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 5h8" /> <path d="M7 5v8a3 3 0 0 0 3 3h1" /> </svg>"##;
const TRANSACTION_DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.8 13a2 2 0 0 0 -1.8 -1h-2a2 2 0 1 0 0 4h2a2 2 0 1 1 0 4h-2a2 2 0 0 1 -1.8 -1" /> <path d="M18 11v10" /> <path d="M3 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 5h8" /> <path d="M7 5v8a3 3 0 0 0 3 3h1" /> </svg>"##;
const TRANSACTION_EURO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12.8c-.523 -.502 -1.172 -.8 -1.875 -.8c-1.727 0 -3.125 1.791 -3.125 4s1.398 4 3.125 4c.703 0 1.352 -.298 1.874 -.8" /> <path d="M15 16h4" /> <path d="M3 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 5h8" /> <path d="M7 5v8a3 3 0 0 0 3 3h1" /> </svg>"##;
const TRANSACTION_POUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 14a2 2 0 1 0 -4 0v4a2 2 0 0 1 -2 2h6" /> <path d="M15 17h4" /> <path d="M3 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 5h8" /> <path d="M7 5v8a3 3 0 0 0 3 3h1" /> </svg>"##;
const TRANSACTION_RUPEE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21 12h-6h1a3 3 0 0 1 0 6h-1l3 3" /> <path d="M15 15h6" /> <path d="M3 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 5h8" /> <path d="M7 5v8a3 3 0 0 0 3 3h1" /> </svg>"##;
const TRANSACTION_YEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 16h6" /> <path d="M15 12l3 4.5" /> <path d="M21 12l-3 4.5v4.5" /> <path d="M15 19h6" /> <path d="M3 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 5h8" /> <path d="M7 5v8a3 3 0 0 0 3 3h1" /> </svg>"##;
const TRANSACTION_YUAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M15 17h6" /> <path d="M15 12l3 4.5" /> <path d="M21 12l-3 4.5v4.5" /> <path d="M3 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 5a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M7 5h8" /> <path d="M7 5v8a3 3 0 0 0 3 3h1" /> </svg>"##;
const TRANSFER_IN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 18v3h16v-14l-8 -4l-8 4v3" /> <path d="M4 14h9" /> <path d="M10 11l3 3l-3 3" /> </svg>"##;
const TRANSFER_OUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 19v2h16v-14l-8 -4l-8 4v2" /> <path d="M13 14h-9" /> <path d="M7 11l-3 3l3 3" /> </svg>"##;
const TRUCK_DELIVERY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 17h-2v-4m-1 -8h11v12m-4 0h6m4 0h2v-6h-8m0 -5h5l3 5" /> <path d="M3 9l4 0" /> </svg>"##;
const TRUCK_LOADING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 3h1a2 2 0 0 1 2 2v10a2 2 0 0 0 2 2h15" /> <path d="M9 9a3 3 0 0 1 3 -3h4a3 3 0 0 1 3 3v2a3 3 0 0 1 -3 3h-4a3 3 0 0 1 -3 -3l0 -2" /> <path d="M7 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M16 19a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> </svg>"##;
const TRUCK_RETURN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M15 17a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <path d="M5 17h-2v-11a1 1 0 0 1 1 -1h9v6h-5l2 2m0 -4l-2 2" /> <path d="M9 17l6 0" /> <path d="M13 6h5l3 5v6h-2" /> </svg>"##;
const WALLET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 8v-3a1 1 0 0 0 -1 -1h-10a2 2 0 0 0 0 4h12a1 1 0 0 1 1 1v3m0 4v3a1 1 0 0 1 -1 1h-12a2 2 0 0 1 -2 -2v-12" /> <path d="M20 12v4h-4a2 2 0 0 1 0 -4h4" /> </svg>"##;
const WALLET_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M17 8v-3a1 1 0 0 0 -1 -1h-8m-3.413 .584a2 2 0 0 0 1.413 3.416h2m4 0h6a1 1 0 0 1 1 1v3" /> <path d="M19 19a1 1 0 0 1 -1 1h-12a2 2 0 0 1 -2 -2v-12" /> <path d="M16 12h4v4m-4 0a2 2 0 0 1 -2 -2" /> <path d="M3 3l18 18" /> </svg>"##;
const WASH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.486 8.965c.168 .02 .34 .033 .514 .035c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.17 0 .339 -.014 .503 -.034" /> <path d="M3 6l1.721 10.329a2 2 0 0 0 1.973 1.671h10.612a2 2 0 0 0 1.973 -1.671l1.721 -10.329" /> </svg>"##;
const WASH_DRY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-12" /> </svg>"##;
const WASH_DRY_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-12" /> <path d="M6 12a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" /> <path d="M12 12h.01" /> </svg>"##;
const WASH_DRY_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-12" /> <path d="M6 12a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" /> <path d="M10 12h.01" /> <path d="M14 12h.01" /> </svg>"##;
const WASH_DRY_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-12" /> <path d="M6 12a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" /> <path d="M12 12h.01" /> <path d="M9 12h.01" /> <path d="M15 12h.01" /> </svg>"##;
const WASH_DRY_A_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M9 16v-4.8c0 -1.657 1.343 -3.2 3 -3.2s3 1.543 3 3.2v4.8" /> <path d="M15 13h-6" /> </svg>"##;
const WASH_DRY_DIP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-12" /> <path d="M12 7v10" /> <path d="M16 7v10" /> <path d="M8 7v10" /> </svg>"##;
const WASH_DRY_F_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 16v-8h4" /> <path d="M13 12h-3" /> </svg>"##;
const WASH_DRY_FLAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-12" /> <path d="M7 12h10" /> </svg>"##;
const WASH_DRY_HANG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-12" /> <path d="M4 4.01c5.333 5.323 10.667 5.32 16 -.01" /> </svg>"##;
const WASH_DRY_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.116 20.127a2.99 2.99 0 0 1 -2.116 .873h-12a3 3 0 0 1 -3 -3v-12c0 -.827 .335 -1.576 .877 -2.12m3.123 -.88h11a3 3 0 0 1 3 3v11" /> <path d="M3 3l18 18" /> </svg>"##;
const WASH_DRY_P_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M10 16v-8h2.5a2.5 2.5 0 1 1 0 5h-2.5" /> </svg>"##;
const WASH_DRY_SHADE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-12" /> <path d="M3 11l8 -8" /> <path d="M3 17l14 -14" /> </svg>"##;
const WASH_DRY_W_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> <path d="M8 8l1.5 8h1l1.5 -6l1.5 6h1l1.5 -8" /> </svg>"##;
const WASH_DRYCLEAN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /> </svg>"##;
const WASH_DRYCLEAN_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.048 16.033a9 9 0 0 0 -12.094 -12.075m-2.321 1.682a9 9 0 0 0 12.733 12.723" /> <path d="M3 3l18 18" /> </svg>"##;
const WASH_ECO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6l1.721 10.329a2 2 0 0 0 1.973 1.671h5.306m8.162 -6.972l.838 -5.028" /> <path d="M3.486 8.965c.168 .02 .34 .033 .514 .035c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.17 0 .339 -.014 .503 -.034" /> <path d="M16 22s0 -2 3 -4" /> <path d="M19 21a3 3 0 0 1 0 -6h3v3a3 3 0 0 1 -3 3" /> </svg>"##;
const WASH_GENTLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.486 5.965c.168 .02 .34 .033 .514 .035c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.17 0 .339 -.014 .503 -.034" /> <path d="M3 3l1.721 10.329a2 2 0 0 0 1.973 1.671h10.612a2 2 0 0 0 1.973 -1.671l1.721 -10.329" /> <path d="M5 18h14" /> <path d="M5 21h14" /> </svg>"##;
const WASH_HAND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.486 8.965c.168 .02 .34 .033 .514 .035c.79 .009 1.539 -.178 2 -.5c.426 -.296 .777 -.5 1.5 -.5h1" /> <path d="M16 8l.615 .034c.552 .067 1.046 .23 1.385 .466c.461 .322 1.21 .509 2 .5c.17 0 .339 -.014 .503 -.034" /> <path d="M14 10.5l.586 .578a1.516 1.516 0 0 0 2 0c.476 -.433 .55 -1.112 .176 -1.622l-1.762 -2.456c-.37 -.506 -1.331 -1 -2 -1h-3.117a1 1 0 0 0 -.992 .876l-.499 3.986a3.857 3.857 0 0 0 2.608 4.138a2.28 2.28 0 0 0 3 -2.162v-2.338" /> <path d="M3 6l1.721 10.329a2 2 0 0 0 1.973 1.671h10.612a2 2 0 0 0 1.973 -1.671l1.721 -10.329" /> </svg>"##;
const WASH_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6l1.721 10.329a2 2 0 0 0 1.973 1.671h10.612c.208 0 .41 -.032 .6 -.092m1.521 -2.472l1.573 -9.436" /> <path d="M3.486 8.965c.168 .02 .34 .033 .514 .035c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5m4.92 .919c.428 -.083 .805 -.227 1.08 -.418c.461 -.322 1.21 -.508 2 -.5c.79 -.008 1.539 .178 2 .5c.461 .32 1.21 .508 2 .5c.17 0 .339 -.015 .503 -.035" /> <path d="M3 3l18 18" /> </svg>"##;
const WASH_PRESS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.486 7.965c.168 .02 .34 .033 .514 .035c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.17 0 .339 -.014 .503 -.034" /> <path d="M3 5l1.721 10.329a2 2 0 0 0 1.973 1.671h10.612a2 2 0 0 0 1.973 -1.671l1.721 -10.329" /> <path d="M5 20h14" /> </svg>"##;
const WASH_TEMPERATURE_1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6l1.721 10.329a2 2 0 0 0 1.973 1.671h10.612a2 2 0 0 0 1.973 -1.671l1.721 -10.329" /> <path d="M3.486 8.965c.168 .02 .34 .033 .514 .035c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.17 0 .339 -.014 .503 -.034" /> <path d="M12 13h.01" /> </svg>"##;
const WASH_TEMPERATURE_2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.486 8.965c.168 .02 .34 .033 .514 .035c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.17 0 .339 -.014 .503 -.034" /> <path d="M3 6l1.721 10.329a2 2 0 0 0 1.973 1.671h10.612a2 2 0 0 0 1.973 -1.671l1.721 -10.329" /> <path d="M14 13h.01" /> <path d="M10 13h.01" /> </svg>"##;
const WASH_TEMPERATURE_3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.486 8.965c.168 .02 .34 .033 .514 .035c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.17 0 .339 -.014 .503 -.034" /> <path d="M3 6l1.721 10.329a2 2 0 0 0 1.973 1.671h10.612a2 2 0 0 0 1.973 -1.671l1.721 -10.329" /> <path d="M12 13h.01" /> <path d="M15 13h.01" /> <path d="M9 13h.01" /> </svg>"##;
const WASH_TEMPERATURE_4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3.486 8.965c.168 .02 .34 .033 .514 .035c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.17 0 .339 -.014 .503 -.034" /> <path d="M3 6l1.721 10.329a2 2 0 0 0 1.973 1.671h10.612a2 2 0 0 0 1.973 -1.671l1.721 -10.329" /> <path d="M10 15h.01" /> <path d="M14 15h.01" /> <path d="M14 12h.01" /> <path d="M10 12h.01" /> </svg>"##;
const WASH_TEMPERATURE_5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 15h.01" /> <path d="M3 6l1.721 10.329a2 2 0 0 0 1.973 1.671h10.612a2 2 0 0 0 1.973 -1.671l1.721 -10.329" /> <path d="M14 15h.01" /> <path d="M15 12h.01" /> <path d="M12 12h.01" /> <path d="M9 12h.01" /> <path d="M3.486 8.965c.168 .02 .34 .033 .514 .035c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.17 0 .339 -.014 .503 -.034" /> </svg>"##;
const WASH_TEMPERATURE_6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9 15h.01" /> <path d="M3 6l1.721 10.329a2 2 0 0 0 1.973 1.671h10.612a2 2 0 0 0 1.973 -1.671l1.721 -10.329" /> <path d="M12 15h.01" /> <path d="M15 15h.01" /> <path d="M15 12h.01" /> <path d="M12 12h.01" /> <path d="M9 12h.01" /> <path d="M3.486 8.965c.168 .02 .34 .033 .514 .035c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.79 .009 1.539 -.178 2 -.5c.461 -.32 1.21 -.507 2 -.5c.79 -.007 1.539 .18 2 .5c.461 .322 1.21 .509 2 .5c.17 0 .339 -.014 .503 -.034" /> </svg>"##;
const WASH_TUMBLE_DRY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-12" /> <path d="M6 12a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" /> </svg>"##;
const WASH_TUMBLE_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20.116 20.127a2.99 2.99 0 0 1 -2.116 .873h-12a3 3 0 0 1 -3 -3v-12c0 -.827 .335 -1.576 .877 -2.12m3.123 -.88h11a3 3 0 0 1 3 3v11" /> <path d="M17.744 13.74a6 6 0 0 0 -7.486 -7.482m-2.499 1.497a6 6 0 1 0 8.48 8.49" /> <path d="M3 3l18 18" /> </svg>"##;
const WHEAT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.014 21.514v-3.75" /> <path d="M5.93 9.504l-.43 1.604c-.712 2.659 .866 5.391 3.524 6.105c.997 .268 1.993 .535 2.99 .801v-3.44c-.164 -2.105 -1.637 -3.879 -3.676 -4.426l-2.408 -.644" /> <path d="M13.744 11.164c.454 -.454 .815 -.994 1.061 -1.587c.246 -.594 .372 -1.23 .372 -1.873c0 -.643 -.126 -1.279 -.372 -1.872c-.246 -.594 -.606 -1.133 -1.061 -1.588l-1.73 -1.73l-1.73 1.73c-.454 .454 -.815 .994 -1.06 1.588c-.246 .594 -.372 1.23 -.373 1.872c0 .643 .127 1.279 .373 1.873c.246 .594 .606 1.133 1.06 1.587" /> <path d="M18.099 9.504l.43 1.604c.712 2.659 -.866 5.391 -3.525 6.105c-.997 .268 -1.994 .535 -2.99 .801v-3.44c.164 -2.105 1.637 -3.879 3.677 -4.426l2.408 -.644" /> </svg>"##;
const WHEAT_OFF_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 3l18 18" /> <path d="M12 21.5v-3.75" /> <path d="M5.916 9.49l-.43 1.604c-.712 2.659 .866 5.392 3.524 6.104c.997 .268 1.994 .535 2.99 .802v-3.44c-.164 -2.105 -1.637 -3.879 -3.677 -4.426l-2.407 -.644" /> <path d="M10.249 4.251c.007 -.007 .014 -.014 .021 -.021l1.73 -1.73" /> <path d="M10.27 11.15c-.589 -.589 -1.017 -1.318 -1.246 -2.118" /> <path d="M14.988 8.988c.229 -.834 .234 -1.713 .013 -2.549c-.221 -.836 -.659 -1.598 -1.271 -2.209l-1.73 -1.73" /> <path d="M16.038 10.037l2.046 -.547l.431 1.604c.142 .53 .193 1.063 .162 1.583" /> <path d="M16.506 16.505c-.45 .307 -.959 .544 -1.516 .694c-.997 .268 -1.994 .535 -2.99 .801v-3.44c.055 -.708 .259 -1.379 .582 -1.978" /> </svg>"##;
const WHISK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M21.015 3.035l-16.515 16.465" /> <path d="M3.173 17.619a4.63 4.63 0 0 0 3.284 3.26a4.67 4.67 0 0 0 4.487 -1.194c1.85 -1.836 4.07 -10.65 4.07 -10.65s-8.88 2.296 -10.639 4.132a4.59 4.59 0 0 0 -1.202 4.452" /> </svg>"##;

/// Commerce icon variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommerceIcon {
    Apple,
    Avocado,
    Backpack,
    BackpackOff,
    Baguette,
    Banana,
    Basket,
    BasketBolt,
    BasketCancel,
    BasketCheck,
    BasketCode,
    BasketCog,
    BasketDiscount,
    BasketDollar,
    BasketDown,
    BasketExclamation,
    BasketHeart,
    BasketMinus,
    BasketOff,
    BasketPause,
    BasketPin,
    BasketPlus,
    BasketQuestion,
    BasketSearch,
    BasketShare,
    BasketStar,
    BasketUp,
    BasketX,
    Beer,
    BeerOff,
    Bleach,
    BleachChlorine,
    BleachNoChlorine,
    BleachOff,
    Bone,
    BoneOff,
    Bottle,
    BottleOff,
    Bowl,
    BowlChopsticks,
    BowlSpoon,
    Bread,
    BreadOff,
    BubbleTea,
    BubbleTea2,
    Burger,
    Businessplan,
    Cake,
    CakeOff,
    CakeRoll,
    Candy,
    CandyOff,
    Carambola,
    Carrot,
    CarrotOff,
    Cash,
    CashBanknote,
    CashBanknoteEdit,
    CashBanknoteHeart,
    CashBanknoteMinus,
    CashBanknoteMove,
    CashBanknoteMoveBack,
    CashBanknoteOff,
    CashBanknotePlus,
    CashEdit,
    CashHeart,
    CashMinus,
    CashMove,
    CashMoveBack,
    CashOff,
    CashPlus,
    CashRegister,
    Cheese,
    ChefHat,
    ChefHatOff,
    Chocolate,
    ClothesRack,
    ClothesRackOff,
    Coffee,
    CoffeeOff,
    Coin,
    CoinBitcoin,
    CoinEuro,
    CoinMonero,
    CoinOff,
    CoinPound,
    CoinRupee,
    CoinTaka,
    CoinYen,
    CoinYuan,
    Coins,
    Cooker,
    Cookie,
    CookieOff,
    CreditCard,
    CreditCardHand,
    CreditCardOff,
    CreditCardPay,
    CreditCardRefund,
    Cup,
    CupOff,
    Currency,
    CurrencyAfghani,
    CurrencyBahraini,
    CurrencyBaht,
    CurrencyBitcoin,
    CurrencyCent,
    CurrencyDinar,
    CurrencyDirham,
    CurrencyDogecoin,
    CurrencyDollar,
    CurrencyDollarAustralian,
    CurrencyDollarBrunei,
    CurrencyDollarCanadian,
    CurrencyDollarGuyanese,
    CurrencyDollarOff,
    CurrencyDollarSingapore,
    CurrencyDollarZimbabwean,
    CurrencyDong,
    CurrencyDram,
    CurrencyEthereum,
    CurrencyEuro,
    CurrencyEuroOff,
    CurrencyFlorin,
    CurrencyForint,
    CurrencyFrank,
    CurrencyGuarani,
    CurrencyHryvnia,
    CurrencyHusd,
    CurrencyIranianRial,
    CurrencyKip,
    CurrencyKroneCzech,
    CurrencyKroneDanish,
    CurrencyKroneSwedish,
    CurrencyLari,
    CurrencyLeu,
    CurrencyLira,
    CurrencyLitecoin,
    CurrencyLyd,
    CurrencyManat,
    CurrencyMonero,
    CurrencyNaira,
    CurrencyNano,
    CurrencyOff,
    CurrencyPaanga,
    CurrencyPeso,
    CurrencyPound,
    CurrencyPoundOff,
    CurrencyQuetzal,
    CurrencyReal,
    CurrencyRenminbi,
    CurrencyRipple,
    CurrencyRiyal,
    CurrencyRubel,
    CurrencyRufiyaa,
    CurrencyRupee,
    CurrencyRupeeNepalese,
    CurrencyShekel,
    CurrencySolana,
    CurrencySom,
    CurrencyTaka,
    CurrencyTenge,
    CurrencyTether,
    CurrencyTugrik,
    CurrencyWon,
    CurrencyXrp,
    CurrencyYen,
    CurrencyYenOff,
    CurrencyYuan,
    CurrencyZcash,
    CurrencyZloty,
    Diaper,
    Discount,
    DiscountOff,
    DogBowl,
    Dumpling,
    EPassport,
    Egg,
    EggCracked,
    EggFried,
    EggOff,
    Eggs,
    FlipFlops,
    Gift,
    GiftCard,
    GiftOff,
    Glass,
    GlassChampagne,
    GlassCocktail,
    GlassFull,
    GlassGin,
    GlassOff,
    Grape,
    Grill,
    GrillFork,
    GrillOff,
    GrillSpatula,
    Hanger,
    HangerOff,
    IceCream,
    IceCream2,
    IceCreamOff,
    Ironing,
    Ironing1,
    Ironing2,
    Ironing3,
    IroningOff,
    IroningSteam,
    IroningSteamOff,
    Jacket,
    Lemon,
    Lemon2,
    Lollipop,
    LollipopOff,
    Meat,
    MeatOff,
    Melon,
    MichelinBibGourmand,
    MichelinStar,
    MichelinStarGreen,
    Microwave,
    MicrowaveOff,
    Milk,
    MilkOff,
    Milkshake,
    Moneybag,
    MoneybagEdit,
    MoneybagHeart,
    MoneybagMinus,
    MoneybagMove,
    MoneybagMoveBack,
    MoneybagPlus,
    Mug,
    MugOff,
    Mushroom,
    MushroomOff,
    Nut,
    Package,
    PackageOff,
    PaperBag,
    PaperBagOff,
    Pepper,
    PepperOff,
    Perfume,
    Pizza,
    PizzaOff,
    ReceiptBitcoin,
    ReceiptEuro,
    ReceiptPound,
    ReceiptRupee,
    ReceiptYen,
    ReceiptYuan,
    RosetteDiscount,
    RosetteDiscountCheck,
    RosetteDiscountCheckOff,
    RosetteDiscountOff,
    Salad,
    Salt,
    Sausage,
    Shirt,
    ShirtOff,
    ShirtSport,
    Shoe,
    ShoeOff,
    ShoppingBag,
    ShoppingBagCheck,
    ShoppingBagDiscount,
    ShoppingBagEdit,
    ShoppingBagExclamation,
    ShoppingBagHeart,
    ShoppingBagMinus,
    ShoppingBagPlus,
    ShoppingBagSearch,
    ShoppingBagX,
    ShoppingCart,
    ShoppingCartBolt,
    ShoppingCartCancel,
    ShoppingCartCheck,
    ShoppingCartCode,
    ShoppingCartCog,
    ShoppingCartCopy,
    ShoppingCartDiscount,
    ShoppingCartDollar,
    ShoppingCartDown,
    ShoppingCartExclamation,
    ShoppingCartHeart,
    ShoppingCartMinus,
    ShoppingCartOff,
    ShoppingCartPause,
    ShoppingCartPin,
    ShoppingCartPlus,
    ShoppingCartQuestion,
    ShoppingCartSearch,
    ShoppingCartShare,
    ShoppingCartStar,
    ShoppingCartUp,
    ShoppingCartX,
    Sock,
    Soup,
    SoupOff,
    Tag,
    TagMinus,
    TagOff,
    TagPlus,
    TagStarred,
    Tags,
    TagsOff,
    TaiwanDollar,
    Tax,
    TaxEuro,
    TaxPound,
    Teapot,
    Tie,
    ToolsKitchen,
    ToolsKitchen2,
    ToolsKitchen3,
    TransactionBitcoin,
    TransactionDollar,
    TransactionEuro,
    TransactionPound,
    TransactionRupee,
    TransactionYen,
    TransactionYuan,
    TransferIn,
    TransferOut,
    TruckDelivery,
    TruckLoading,
    TruckReturn,
    Wallet,
    WalletOff,
    Wash,
    WashDry,
    WashDry1,
    WashDry2,
    WashDry3,
    WashDryA,
    WashDryDip,
    WashDryF,
    WashDryFlat,
    WashDryHang,
    WashDryOff,
    WashDryP,
    WashDryShade,
    WashDryW,
    WashDryclean,
    WashDrycleanOff,
    WashEco,
    WashGentle,
    WashHand,
    WashOff,
    WashPress,
    WashTemperature1,
    WashTemperature2,
    WashTemperature3,
    WashTemperature4,
    WashTemperature5,
    WashTemperature6,
    WashTumbleDry,
    WashTumbleOff,
    Wheat,
    WheatOff,
    Whisk,
}

impl CommerceIcon {
    /// Returns all available icons in this category.
    pub fn all() -> &'static [Self] {
        &[Self::Apple, Self::Avocado, Self::Backpack, Self::BackpackOff, Self::Baguette, Self::Banana, Self::Basket, Self::BasketBolt, Self::BasketCancel, Self::BasketCheck, Self::BasketCode, Self::BasketCog, Self::BasketDiscount, Self::BasketDollar, Self::BasketDown, Self::BasketExclamation, Self::BasketHeart, Self::BasketMinus, Self::BasketOff, Self::BasketPause, Self::BasketPin, Self::BasketPlus, Self::BasketQuestion, Self::BasketSearch, Self::BasketShare, Self::BasketStar, Self::BasketUp, Self::BasketX, Self::Beer, Self::BeerOff, Self::Bleach, Self::BleachChlorine, Self::BleachNoChlorine, Self::BleachOff, Self::Bone, Self::BoneOff, Self::Bottle, Self::BottleOff, Self::Bowl, Self::BowlChopsticks, Self::BowlSpoon, Self::Bread, Self::BreadOff, Self::BubbleTea, Self::BubbleTea2, Self::Burger, Self::Businessplan, Self::Cake, Self::CakeOff, Self::CakeRoll, Self::Candy, Self::CandyOff, Self::Carambola, Self::Carrot, Self::CarrotOff, Self::Cash, Self::CashBanknote, Self::CashBanknoteEdit, Self::CashBanknoteHeart, Self::CashBanknoteMinus, Self::CashBanknoteMove, Self::CashBanknoteMoveBack, Self::CashBanknoteOff, Self::CashBanknotePlus, Self::CashEdit, Self::CashHeart, Self::CashMinus, Self::CashMove, Self::CashMoveBack, Self::CashOff, Self::CashPlus, Self::CashRegister, Self::Cheese, Self::ChefHat, Self::ChefHatOff, Self::Chocolate, Self::ClothesRack, Self::ClothesRackOff, Self::Coffee, Self::CoffeeOff, Self::Coin, Self::CoinBitcoin, Self::CoinEuro, Self::CoinMonero, Self::CoinOff, Self::CoinPound, Self::CoinRupee, Self::CoinTaka, Self::CoinYen, Self::CoinYuan, Self::Coins, Self::Cooker, Self::Cookie, Self::CookieOff, Self::CreditCard, Self::CreditCardHand, Self::CreditCardOff, Self::CreditCardPay, Self::CreditCardRefund, Self::Cup, Self::CupOff, Self::Currency, Self::CurrencyAfghani, Self::CurrencyBahraini, Self::CurrencyBaht, Self::CurrencyBitcoin, Self::CurrencyCent, Self::CurrencyDinar, Self::CurrencyDirham, Self::CurrencyDogecoin, Self::CurrencyDollar, Self::CurrencyDollarAustralian, Self::CurrencyDollarBrunei, Self::CurrencyDollarCanadian, Self::CurrencyDollarGuyanese, Self::CurrencyDollarOff, Self::CurrencyDollarSingapore, Self::CurrencyDollarZimbabwean, Self::CurrencyDong, Self::CurrencyDram, Self::CurrencyEthereum, Self::CurrencyEuro, Self::CurrencyEuroOff, Self::CurrencyFlorin, Self::CurrencyForint, Self::CurrencyFrank, Self::CurrencyGuarani, Self::CurrencyHryvnia, Self::CurrencyHusd, Self::CurrencyIranianRial, Self::CurrencyKip, Self::CurrencyKroneCzech, Self::CurrencyKroneDanish, Self::CurrencyKroneSwedish, Self::CurrencyLari, Self::CurrencyLeu, Self::CurrencyLira, Self::CurrencyLitecoin, Self::CurrencyLyd, Self::CurrencyManat, Self::CurrencyMonero, Self::CurrencyNaira, Self::CurrencyNano, Self::CurrencyOff, Self::CurrencyPaanga, Self::CurrencyPeso, Self::CurrencyPound, Self::CurrencyPoundOff, Self::CurrencyQuetzal, Self::CurrencyReal, Self::CurrencyRenminbi, Self::CurrencyRipple, Self::CurrencyRiyal, Self::CurrencyRubel, Self::CurrencyRufiyaa, Self::CurrencyRupee, Self::CurrencyRupeeNepalese, Self::CurrencyShekel, Self::CurrencySolana, Self::CurrencySom, Self::CurrencyTaka, Self::CurrencyTenge, Self::CurrencyTether, Self::CurrencyTugrik, Self::CurrencyWon, Self::CurrencyXrp, Self::CurrencyYen, Self::CurrencyYenOff, Self::CurrencyYuan, Self::CurrencyZcash, Self::CurrencyZloty, Self::Diaper, Self::Discount, Self::DiscountOff, Self::DogBowl, Self::Dumpling, Self::EPassport, Self::Egg, Self::EggCracked, Self::EggFried, Self::EggOff, Self::Eggs, Self::FlipFlops, Self::Gift, Self::GiftCard, Self::GiftOff, Self::Glass, Self::GlassChampagne, Self::GlassCocktail, Self::GlassFull, Self::GlassGin, Self::GlassOff, Self::Grape, Self::Grill, Self::GrillFork, Self::GrillOff, Self::GrillSpatula, Self::Hanger, Self::HangerOff, Self::IceCream, Self::IceCream2, Self::IceCreamOff, Self::Ironing, Self::Ironing1, Self::Ironing2, Self::Ironing3, Self::IroningOff, Self::IroningSteam, Self::IroningSteamOff, Self::Jacket, Self::Lemon, Self::Lemon2, Self::Lollipop, Self::LollipopOff, Self::Meat, Self::MeatOff, Self::Melon, Self::MichelinBibGourmand, Self::MichelinStar, Self::MichelinStarGreen, Self::Microwave, Self::MicrowaveOff, Self::Milk, Self::MilkOff, Self::Milkshake, Self::Moneybag, Self::MoneybagEdit, Self::MoneybagHeart, Self::MoneybagMinus, Self::MoneybagMove, Self::MoneybagMoveBack, Self::MoneybagPlus, Self::Mug, Self::MugOff, Self::Mushroom, Self::MushroomOff, Self::Nut, Self::Package, Self::PackageOff, Self::PaperBag, Self::PaperBagOff, Self::Pepper, Self::PepperOff, Self::Perfume, Self::Pizza, Self::PizzaOff, Self::ReceiptBitcoin, Self::ReceiptEuro, Self::ReceiptPound, Self::ReceiptRupee, Self::ReceiptYen, Self::ReceiptYuan, Self::RosetteDiscount, Self::RosetteDiscountCheck, Self::RosetteDiscountCheckOff, Self::RosetteDiscountOff, Self::Salad, Self::Salt, Self::Sausage, Self::Shirt, Self::ShirtOff, Self::ShirtSport, Self::Shoe, Self::ShoeOff, Self::ShoppingBag, Self::ShoppingBagCheck, Self::ShoppingBagDiscount, Self::ShoppingBagEdit, Self::ShoppingBagExclamation, Self::ShoppingBagHeart, Self::ShoppingBagMinus, Self::ShoppingBagPlus, Self::ShoppingBagSearch, Self::ShoppingBagX, Self::ShoppingCart, Self::ShoppingCartBolt, Self::ShoppingCartCancel, Self::ShoppingCartCheck, Self::ShoppingCartCode, Self::ShoppingCartCog, Self::ShoppingCartCopy, Self::ShoppingCartDiscount, Self::ShoppingCartDollar, Self::ShoppingCartDown, Self::ShoppingCartExclamation, Self::ShoppingCartHeart, Self::ShoppingCartMinus, Self::ShoppingCartOff, Self::ShoppingCartPause, Self::ShoppingCartPin, Self::ShoppingCartPlus, Self::ShoppingCartQuestion, Self::ShoppingCartSearch, Self::ShoppingCartShare, Self::ShoppingCartStar, Self::ShoppingCartUp, Self::ShoppingCartX, Self::Sock, Self::Soup, Self::SoupOff, Self::Tag, Self::TagMinus, Self::TagOff, Self::TagPlus, Self::TagStarred, Self::Tags, Self::TagsOff, Self::TaiwanDollar, Self::Tax, Self::TaxEuro, Self::TaxPound, Self::Teapot, Self::Tie, Self::ToolsKitchen, Self::ToolsKitchen2, Self::ToolsKitchen3, Self::TransactionBitcoin, Self::TransactionDollar, Self::TransactionEuro, Self::TransactionPound, Self::TransactionRupee, Self::TransactionYen, Self::TransactionYuan, Self::TransferIn, Self::TransferOut, Self::TruckDelivery, Self::TruckLoading, Self::TruckReturn, Self::Wallet, Self::WalletOff, Self::Wash, Self::WashDry, Self::WashDry1, Self::WashDry2, Self::WashDry3, Self::WashDryA, Self::WashDryDip, Self::WashDryF, Self::WashDryFlat, Self::WashDryHang, Self::WashDryOff, Self::WashDryP, Self::WashDryShade, Self::WashDryW, Self::WashDryclean, Self::WashDrycleanOff, Self::WashEco, Self::WashGentle, Self::WashHand, Self::WashOff, Self::WashPress, Self::WashTemperature1, Self::WashTemperature2, Self::WashTemperature3, Self::WashTemperature4, Self::WashTemperature5, Self::WashTemperature6, Self::WashTumbleDry, Self::WashTumbleOff, Self::Wheat, Self::WheatOff, Self::Whisk]
    }

    /// Returns the icon count.
    pub fn count() -> usize {
        362
    }

    /// Creates an icon from its kebab-case name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "apple" => Some(Self::Apple),
            "avocado" => Some(Self::Avocado),
            "backpack" => Some(Self::Backpack),
            "backpack-off" => Some(Self::BackpackOff),
            "baguette" => Some(Self::Baguette),
            "banana" => Some(Self::Banana),
            "basket" => Some(Self::Basket),
            "basket-bolt" => Some(Self::BasketBolt),
            "basket-cancel" => Some(Self::BasketCancel),
            "basket-check" => Some(Self::BasketCheck),
            "basket-code" => Some(Self::BasketCode),
            "basket-cog" => Some(Self::BasketCog),
            "basket-discount" => Some(Self::BasketDiscount),
            "basket-dollar" => Some(Self::BasketDollar),
            "basket-down" => Some(Self::BasketDown),
            "basket-exclamation" => Some(Self::BasketExclamation),
            "basket-heart" => Some(Self::BasketHeart),
            "basket-minus" => Some(Self::BasketMinus),
            "basket-off" => Some(Self::BasketOff),
            "basket-pause" => Some(Self::BasketPause),
            "basket-pin" => Some(Self::BasketPin),
            "basket-plus" => Some(Self::BasketPlus),
            "basket-question" => Some(Self::BasketQuestion),
            "basket-search" => Some(Self::BasketSearch),
            "basket-share" => Some(Self::BasketShare),
            "basket-star" => Some(Self::BasketStar),
            "basket-up" => Some(Self::BasketUp),
            "basket-x" => Some(Self::BasketX),
            "beer" => Some(Self::Beer),
            "beer-off" => Some(Self::BeerOff),
            "bleach" => Some(Self::Bleach),
            "bleach-chlorine" => Some(Self::BleachChlorine),
            "bleach-no-chlorine" => Some(Self::BleachNoChlorine),
            "bleach-off" => Some(Self::BleachOff),
            "bone" => Some(Self::Bone),
            "bone-off" => Some(Self::BoneOff),
            "bottle" => Some(Self::Bottle),
            "bottle-off" => Some(Self::BottleOff),
            "bowl" => Some(Self::Bowl),
            "bowl-chopsticks" => Some(Self::BowlChopsticks),
            "bowl-spoon" => Some(Self::BowlSpoon),
            "bread" => Some(Self::Bread),
            "bread-off" => Some(Self::BreadOff),
            "bubble-tea" => Some(Self::BubbleTea),
            "bubble-tea-2" => Some(Self::BubbleTea2),
            "burger" => Some(Self::Burger),
            "businessplan" => Some(Self::Businessplan),
            "cake" => Some(Self::Cake),
            "cake-off" => Some(Self::CakeOff),
            "cake-roll" => Some(Self::CakeRoll),
            "candy" => Some(Self::Candy),
            "candy-off" => Some(Self::CandyOff),
            "carambola" => Some(Self::Carambola),
            "carrot" => Some(Self::Carrot),
            "carrot-off" => Some(Self::CarrotOff),
            "cash" => Some(Self::Cash),
            "cash-banknote" => Some(Self::CashBanknote),
            "cash-banknote-edit" => Some(Self::CashBanknoteEdit),
            "cash-banknote-heart" => Some(Self::CashBanknoteHeart),
            "cash-banknote-minus" => Some(Self::CashBanknoteMinus),
            "cash-banknote-move" => Some(Self::CashBanknoteMove),
            "cash-banknote-move-back" => Some(Self::CashBanknoteMoveBack),
            "cash-banknote-off" => Some(Self::CashBanknoteOff),
            "cash-banknote-plus" => Some(Self::CashBanknotePlus),
            "cash-edit" => Some(Self::CashEdit),
            "cash-heart" => Some(Self::CashHeart),
            "cash-minus" => Some(Self::CashMinus),
            "cash-move" => Some(Self::CashMove),
            "cash-move-back" => Some(Self::CashMoveBack),
            "cash-off" => Some(Self::CashOff),
            "cash-plus" => Some(Self::CashPlus),
            "cash-register" => Some(Self::CashRegister),
            "cheese" => Some(Self::Cheese),
            "chef-hat" => Some(Self::ChefHat),
            "chef-hat-off" => Some(Self::ChefHatOff),
            "chocolate" => Some(Self::Chocolate),
            "clothes-rack" => Some(Self::ClothesRack),
            "clothes-rack-off" => Some(Self::ClothesRackOff),
            "coffee" => Some(Self::Coffee),
            "coffee-off" => Some(Self::CoffeeOff),
            "coin" => Some(Self::Coin),
            "coin-bitcoin" => Some(Self::CoinBitcoin),
            "coin-euro" => Some(Self::CoinEuro),
            "coin-monero" => Some(Self::CoinMonero),
            "coin-off" => Some(Self::CoinOff),
            "coin-pound" => Some(Self::CoinPound),
            "coin-rupee" => Some(Self::CoinRupee),
            "coin-taka" => Some(Self::CoinTaka),
            "coin-yen" => Some(Self::CoinYen),
            "coin-yuan" => Some(Self::CoinYuan),
            "coins" => Some(Self::Coins),
            "cooker" => Some(Self::Cooker),
            "cookie" => Some(Self::Cookie),
            "cookie-off" => Some(Self::CookieOff),
            "credit-card" => Some(Self::CreditCard),
            "credit-card-hand" => Some(Self::CreditCardHand),
            "credit-card-off" => Some(Self::CreditCardOff),
            "credit-card-pay" => Some(Self::CreditCardPay),
            "credit-card-refund" => Some(Self::CreditCardRefund),
            "cup" => Some(Self::Cup),
            "cup-off" => Some(Self::CupOff),
            "currency" => Some(Self::Currency),
            "currency-afghani" => Some(Self::CurrencyAfghani),
            "currency-bahraini" => Some(Self::CurrencyBahraini),
            "currency-baht" => Some(Self::CurrencyBaht),
            "currency-bitcoin" => Some(Self::CurrencyBitcoin),
            "currency-cent" => Some(Self::CurrencyCent),
            "currency-dinar" => Some(Self::CurrencyDinar),
            "currency-dirham" => Some(Self::CurrencyDirham),
            "currency-dogecoin" => Some(Self::CurrencyDogecoin),
            "currency-dollar" => Some(Self::CurrencyDollar),
            "currency-dollar-australian" => Some(Self::CurrencyDollarAustralian),
            "currency-dollar-brunei" => Some(Self::CurrencyDollarBrunei),
            "currency-dollar-canadian" => Some(Self::CurrencyDollarCanadian),
            "currency-dollar-guyanese" => Some(Self::CurrencyDollarGuyanese),
            "currency-dollar-off" => Some(Self::CurrencyDollarOff),
            "currency-dollar-singapore" => Some(Self::CurrencyDollarSingapore),
            "currency-dollar-zimbabwean" => Some(Self::CurrencyDollarZimbabwean),
            "currency-dong" => Some(Self::CurrencyDong),
            "currency-dram" => Some(Self::CurrencyDram),
            "currency-ethereum" => Some(Self::CurrencyEthereum),
            "currency-euro" => Some(Self::CurrencyEuro),
            "currency-euro-off" => Some(Self::CurrencyEuroOff),
            "currency-florin" => Some(Self::CurrencyFlorin),
            "currency-forint" => Some(Self::CurrencyForint),
            "currency-frank" => Some(Self::CurrencyFrank),
            "currency-guarani" => Some(Self::CurrencyGuarani),
            "currency-hryvnia" => Some(Self::CurrencyHryvnia),
            "currency-husd" => Some(Self::CurrencyHusd),
            "currency-iranian-rial" => Some(Self::CurrencyIranianRial),
            "currency-kip" => Some(Self::CurrencyKip),
            "currency-krone-czech" => Some(Self::CurrencyKroneCzech),
            "currency-krone-danish" => Some(Self::CurrencyKroneDanish),
            "currency-krone-swedish" => Some(Self::CurrencyKroneSwedish),
            "currency-lari" => Some(Self::CurrencyLari),
            "currency-leu" => Some(Self::CurrencyLeu),
            "currency-lira" => Some(Self::CurrencyLira),
            "currency-litecoin" => Some(Self::CurrencyLitecoin),
            "currency-lyd" => Some(Self::CurrencyLyd),
            "currency-manat" => Some(Self::CurrencyManat),
            "currency-monero" => Some(Self::CurrencyMonero),
            "currency-naira" => Some(Self::CurrencyNaira),
            "currency-nano" => Some(Self::CurrencyNano),
            "currency-off" => Some(Self::CurrencyOff),
            "currency-paanga" => Some(Self::CurrencyPaanga),
            "currency-peso" => Some(Self::CurrencyPeso),
            "currency-pound" => Some(Self::CurrencyPound),
            "currency-pound-off" => Some(Self::CurrencyPoundOff),
            "currency-quetzal" => Some(Self::CurrencyQuetzal),
            "currency-real" => Some(Self::CurrencyReal),
            "currency-renminbi" => Some(Self::CurrencyRenminbi),
            "currency-ripple" => Some(Self::CurrencyRipple),
            "currency-riyal" => Some(Self::CurrencyRiyal),
            "currency-rubel" => Some(Self::CurrencyRubel),
            "currency-rufiyaa" => Some(Self::CurrencyRufiyaa),
            "currency-rupee" => Some(Self::CurrencyRupee),
            "currency-rupee-nepalese" => Some(Self::CurrencyRupeeNepalese),
            "currency-shekel" => Some(Self::CurrencyShekel),
            "currency-solana" => Some(Self::CurrencySolana),
            "currency-som" => Some(Self::CurrencySom),
            "currency-taka" => Some(Self::CurrencyTaka),
            "currency-tenge" => Some(Self::CurrencyTenge),
            "currency-tether" => Some(Self::CurrencyTether),
            "currency-tugrik" => Some(Self::CurrencyTugrik),
            "currency-won" => Some(Self::CurrencyWon),
            "currency-xrp" => Some(Self::CurrencyXrp),
            "currency-yen" => Some(Self::CurrencyYen),
            "currency-yen-off" => Some(Self::CurrencyYenOff),
            "currency-yuan" => Some(Self::CurrencyYuan),
            "currency-zcash" => Some(Self::CurrencyZcash),
            "currency-zloty" => Some(Self::CurrencyZloty),
            "diaper" => Some(Self::Diaper),
            "discount" => Some(Self::Discount),
            "discount-off" => Some(Self::DiscountOff),
            "dog-bowl" => Some(Self::DogBowl),
            "dumpling" => Some(Self::Dumpling),
            "e-passport" => Some(Self::EPassport),
            "egg" => Some(Self::Egg),
            "egg-cracked" => Some(Self::EggCracked),
            "egg-fried" => Some(Self::EggFried),
            "egg-off" => Some(Self::EggOff),
            "eggs" => Some(Self::Eggs),
            "flip-flops" => Some(Self::FlipFlops),
            "gift" => Some(Self::Gift),
            "gift-card" => Some(Self::GiftCard),
            "gift-off" => Some(Self::GiftOff),
            "glass" => Some(Self::Glass),
            "glass-champagne" => Some(Self::GlassChampagne),
            "glass-cocktail" => Some(Self::GlassCocktail),
            "glass-full" => Some(Self::GlassFull),
            "glass-gin" => Some(Self::GlassGin),
            "glass-off" => Some(Self::GlassOff),
            "grape" => Some(Self::Grape),
            "grill" => Some(Self::Grill),
            "grill-fork" => Some(Self::GrillFork),
            "grill-off" => Some(Self::GrillOff),
            "grill-spatula" => Some(Self::GrillSpatula),
            "hanger" => Some(Self::Hanger),
            "hanger-off" => Some(Self::HangerOff),
            "ice-cream" => Some(Self::IceCream),
            "ice-cream-2" => Some(Self::IceCream2),
            "ice-cream-off" => Some(Self::IceCreamOff),
            "ironing" => Some(Self::Ironing),
            "ironing-1" => Some(Self::Ironing1),
            "ironing-2" => Some(Self::Ironing2),
            "ironing-3" => Some(Self::Ironing3),
            "ironing-off" => Some(Self::IroningOff),
            "ironing-steam" => Some(Self::IroningSteam),
            "ironing-steam-off" => Some(Self::IroningSteamOff),
            "jacket" => Some(Self::Jacket),
            "lemon" => Some(Self::Lemon),
            "lemon-2" => Some(Self::Lemon2),
            "lollipop" => Some(Self::Lollipop),
            "lollipop-off" => Some(Self::LollipopOff),
            "meat" => Some(Self::Meat),
            "meat-off" => Some(Self::MeatOff),
            "melon" => Some(Self::Melon),
            "michelin-bib-gourmand" => Some(Self::MichelinBibGourmand),
            "michelin-star" => Some(Self::MichelinStar),
            "michelin-star-green" => Some(Self::MichelinStarGreen),
            "microwave" => Some(Self::Microwave),
            "microwave-off" => Some(Self::MicrowaveOff),
            "milk" => Some(Self::Milk),
            "milk-off" => Some(Self::MilkOff),
            "milkshake" => Some(Self::Milkshake),
            "moneybag" => Some(Self::Moneybag),
            "moneybag-edit" => Some(Self::MoneybagEdit),
            "moneybag-heart" => Some(Self::MoneybagHeart),
            "moneybag-minus" => Some(Self::MoneybagMinus),
            "moneybag-move" => Some(Self::MoneybagMove),
            "moneybag-move-back" => Some(Self::MoneybagMoveBack),
            "moneybag-plus" => Some(Self::MoneybagPlus),
            "mug" => Some(Self::Mug),
            "mug-off" => Some(Self::MugOff),
            "mushroom" => Some(Self::Mushroom),
            "mushroom-off" => Some(Self::MushroomOff),
            "nut" => Some(Self::Nut),
            "package" => Some(Self::Package),
            "package-off" => Some(Self::PackageOff),
            "paper-bag" => Some(Self::PaperBag),
            "paper-bag-off" => Some(Self::PaperBagOff),
            "pepper" => Some(Self::Pepper),
            "pepper-off" => Some(Self::PepperOff),
            "perfume" => Some(Self::Perfume),
            "pizza" => Some(Self::Pizza),
            "pizza-off" => Some(Self::PizzaOff),
            "receipt-bitcoin" => Some(Self::ReceiptBitcoin),
            "receipt-euro" => Some(Self::ReceiptEuro),
            "receipt-pound" => Some(Self::ReceiptPound),
            "receipt-rupee" => Some(Self::ReceiptRupee),
            "receipt-yen" => Some(Self::ReceiptYen),
            "receipt-yuan" => Some(Self::ReceiptYuan),
            "rosette-discount" => Some(Self::RosetteDiscount),
            "rosette-discount-check" => Some(Self::RosetteDiscountCheck),
            "rosette-discount-check-off" => Some(Self::RosetteDiscountCheckOff),
            "rosette-discount-off" => Some(Self::RosetteDiscountOff),
            "salad" => Some(Self::Salad),
            "salt" => Some(Self::Salt),
            "sausage" => Some(Self::Sausage),
            "shirt" => Some(Self::Shirt),
            "shirt-off" => Some(Self::ShirtOff),
            "shirt-sport" => Some(Self::ShirtSport),
            "shoe" => Some(Self::Shoe),
            "shoe-off" => Some(Self::ShoeOff),
            "shopping-bag" => Some(Self::ShoppingBag),
            "shopping-bag-check" => Some(Self::ShoppingBagCheck),
            "shopping-bag-discount" => Some(Self::ShoppingBagDiscount),
            "shopping-bag-edit" => Some(Self::ShoppingBagEdit),
            "shopping-bag-exclamation" => Some(Self::ShoppingBagExclamation),
            "shopping-bag-heart" => Some(Self::ShoppingBagHeart),
            "shopping-bag-minus" => Some(Self::ShoppingBagMinus),
            "shopping-bag-plus" => Some(Self::ShoppingBagPlus),
            "shopping-bag-search" => Some(Self::ShoppingBagSearch),
            "shopping-bag-x" => Some(Self::ShoppingBagX),
            "shopping-cart" => Some(Self::ShoppingCart),
            "shopping-cart-bolt" => Some(Self::ShoppingCartBolt),
            "shopping-cart-cancel" => Some(Self::ShoppingCartCancel),
            "shopping-cart-check" => Some(Self::ShoppingCartCheck),
            "shopping-cart-code" => Some(Self::ShoppingCartCode),
            "shopping-cart-cog" => Some(Self::ShoppingCartCog),
            "shopping-cart-copy" => Some(Self::ShoppingCartCopy),
            "shopping-cart-discount" => Some(Self::ShoppingCartDiscount),
            "shopping-cart-dollar" => Some(Self::ShoppingCartDollar),
            "shopping-cart-down" => Some(Self::ShoppingCartDown),
            "shopping-cart-exclamation" => Some(Self::ShoppingCartExclamation),
            "shopping-cart-heart" => Some(Self::ShoppingCartHeart),
            "shopping-cart-minus" => Some(Self::ShoppingCartMinus),
            "shopping-cart-off" => Some(Self::ShoppingCartOff),
            "shopping-cart-pause" => Some(Self::ShoppingCartPause),
            "shopping-cart-pin" => Some(Self::ShoppingCartPin),
            "shopping-cart-plus" => Some(Self::ShoppingCartPlus),
            "shopping-cart-question" => Some(Self::ShoppingCartQuestion),
            "shopping-cart-search" => Some(Self::ShoppingCartSearch),
            "shopping-cart-share" => Some(Self::ShoppingCartShare),
            "shopping-cart-star" => Some(Self::ShoppingCartStar),
            "shopping-cart-up" => Some(Self::ShoppingCartUp),
            "shopping-cart-x" => Some(Self::ShoppingCartX),
            "sock" => Some(Self::Sock),
            "soup" => Some(Self::Soup),
            "soup-off" => Some(Self::SoupOff),
            "tag" => Some(Self::Tag),
            "tag-minus" => Some(Self::TagMinus),
            "tag-off" => Some(Self::TagOff),
            "tag-plus" => Some(Self::TagPlus),
            "tag-starred" => Some(Self::TagStarred),
            "tags" => Some(Self::Tags),
            "tags-off" => Some(Self::TagsOff),
            "taiwan-dollar" => Some(Self::TaiwanDollar),
            "tax" => Some(Self::Tax),
            "tax-euro" => Some(Self::TaxEuro),
            "tax-pound" => Some(Self::TaxPound),
            "teapot" => Some(Self::Teapot),
            "tie" => Some(Self::Tie),
            "tools-kitchen" => Some(Self::ToolsKitchen),
            "tools-kitchen-2" => Some(Self::ToolsKitchen2),
            "tools-kitchen-3" => Some(Self::ToolsKitchen3),
            "transaction-bitcoin" => Some(Self::TransactionBitcoin),
            "transaction-dollar" => Some(Self::TransactionDollar),
            "transaction-euro" => Some(Self::TransactionEuro),
            "transaction-pound" => Some(Self::TransactionPound),
            "transaction-rupee" => Some(Self::TransactionRupee),
            "transaction-yen" => Some(Self::TransactionYen),
            "transaction-yuan" => Some(Self::TransactionYuan),
            "transfer-in" => Some(Self::TransferIn),
            "transfer-out" => Some(Self::TransferOut),
            "truck-delivery" => Some(Self::TruckDelivery),
            "truck-loading" => Some(Self::TruckLoading),
            "truck-return" => Some(Self::TruckReturn),
            "wallet" => Some(Self::Wallet),
            "wallet-off" => Some(Self::WalletOff),
            "wash" => Some(Self::Wash),
            "wash-dry" => Some(Self::WashDry),
            "wash-dry-1" => Some(Self::WashDry1),
            "wash-dry-2" => Some(Self::WashDry2),
            "wash-dry-3" => Some(Self::WashDry3),
            "wash-dry-a" => Some(Self::WashDryA),
            "wash-dry-dip" => Some(Self::WashDryDip),
            "wash-dry-f" => Some(Self::WashDryF),
            "wash-dry-flat" => Some(Self::WashDryFlat),
            "wash-dry-hang" => Some(Self::WashDryHang),
            "wash-dry-off" => Some(Self::WashDryOff),
            "wash-dry-p" => Some(Self::WashDryP),
            "wash-dry-shade" => Some(Self::WashDryShade),
            "wash-dry-w" => Some(Self::WashDryW),
            "wash-dryclean" => Some(Self::WashDryclean),
            "wash-dryclean-off" => Some(Self::WashDrycleanOff),
            "wash-eco" => Some(Self::WashEco),
            "wash-gentle" => Some(Self::WashGentle),
            "wash-hand" => Some(Self::WashHand),
            "wash-off" => Some(Self::WashOff),
            "wash-press" => Some(Self::WashPress),
            "wash-temperature-1" => Some(Self::WashTemperature1),
            "wash-temperature-2" => Some(Self::WashTemperature2),
            "wash-temperature-3" => Some(Self::WashTemperature3),
            "wash-temperature-4" => Some(Self::WashTemperature4),
            "wash-temperature-5" => Some(Self::WashTemperature5),
            "wash-temperature-6" => Some(Self::WashTemperature6),
            "wash-tumble-dry" => Some(Self::WashTumbleDry),
            "wash-tumble-off" => Some(Self::WashTumbleOff),
            "wheat" => Some(Self::Wheat),
            "wheat-off" => Some(Self::WheatOff),
            "whisk" => Some(Self::Whisk),
            _ => None,
        }
    }
}

impl TablerIconData for CommerceIcon {
    fn name(&self) -> &'static str {
        match self {
            Self::Apple => "apple",
            Self::Avocado => "avocado",
            Self::Backpack => "backpack",
            Self::BackpackOff => "backpack-off",
            Self::Baguette => "baguette",
            Self::Banana => "banana",
            Self::Basket => "basket",
            Self::BasketBolt => "basket-bolt",
            Self::BasketCancel => "basket-cancel",
            Self::BasketCheck => "basket-check",
            Self::BasketCode => "basket-code",
            Self::BasketCog => "basket-cog",
            Self::BasketDiscount => "basket-discount",
            Self::BasketDollar => "basket-dollar",
            Self::BasketDown => "basket-down",
            Self::BasketExclamation => "basket-exclamation",
            Self::BasketHeart => "basket-heart",
            Self::BasketMinus => "basket-minus",
            Self::BasketOff => "basket-off",
            Self::BasketPause => "basket-pause",
            Self::BasketPin => "basket-pin",
            Self::BasketPlus => "basket-plus",
            Self::BasketQuestion => "basket-question",
            Self::BasketSearch => "basket-search",
            Self::BasketShare => "basket-share",
            Self::BasketStar => "basket-star",
            Self::BasketUp => "basket-up",
            Self::BasketX => "basket-x",
            Self::Beer => "beer",
            Self::BeerOff => "beer-off",
            Self::Bleach => "bleach",
            Self::BleachChlorine => "bleach-chlorine",
            Self::BleachNoChlorine => "bleach-no-chlorine",
            Self::BleachOff => "bleach-off",
            Self::Bone => "bone",
            Self::BoneOff => "bone-off",
            Self::Bottle => "bottle",
            Self::BottleOff => "bottle-off",
            Self::Bowl => "bowl",
            Self::BowlChopsticks => "bowl-chopsticks",
            Self::BowlSpoon => "bowl-spoon",
            Self::Bread => "bread",
            Self::BreadOff => "bread-off",
            Self::BubbleTea => "bubble-tea",
            Self::BubbleTea2 => "bubble-tea-2",
            Self::Burger => "burger",
            Self::Businessplan => "businessplan",
            Self::Cake => "cake",
            Self::CakeOff => "cake-off",
            Self::CakeRoll => "cake-roll",
            Self::Candy => "candy",
            Self::CandyOff => "candy-off",
            Self::Carambola => "carambola",
            Self::Carrot => "carrot",
            Self::CarrotOff => "carrot-off",
            Self::Cash => "cash",
            Self::CashBanknote => "cash-banknote",
            Self::CashBanknoteEdit => "cash-banknote-edit",
            Self::CashBanknoteHeart => "cash-banknote-heart",
            Self::CashBanknoteMinus => "cash-banknote-minus",
            Self::CashBanknoteMove => "cash-banknote-move",
            Self::CashBanknoteMoveBack => "cash-banknote-move-back",
            Self::CashBanknoteOff => "cash-banknote-off",
            Self::CashBanknotePlus => "cash-banknote-plus",
            Self::CashEdit => "cash-edit",
            Self::CashHeart => "cash-heart",
            Self::CashMinus => "cash-minus",
            Self::CashMove => "cash-move",
            Self::CashMoveBack => "cash-move-back",
            Self::CashOff => "cash-off",
            Self::CashPlus => "cash-plus",
            Self::CashRegister => "cash-register",
            Self::Cheese => "cheese",
            Self::ChefHat => "chef-hat",
            Self::ChefHatOff => "chef-hat-off",
            Self::Chocolate => "chocolate",
            Self::ClothesRack => "clothes-rack",
            Self::ClothesRackOff => "clothes-rack-off",
            Self::Coffee => "coffee",
            Self::CoffeeOff => "coffee-off",
            Self::Coin => "coin",
            Self::CoinBitcoin => "coin-bitcoin",
            Self::CoinEuro => "coin-euro",
            Self::CoinMonero => "coin-monero",
            Self::CoinOff => "coin-off",
            Self::CoinPound => "coin-pound",
            Self::CoinRupee => "coin-rupee",
            Self::CoinTaka => "coin-taka",
            Self::CoinYen => "coin-yen",
            Self::CoinYuan => "coin-yuan",
            Self::Coins => "coins",
            Self::Cooker => "cooker",
            Self::Cookie => "cookie",
            Self::CookieOff => "cookie-off",
            Self::CreditCard => "credit-card",
            Self::CreditCardHand => "credit-card-hand",
            Self::CreditCardOff => "credit-card-off",
            Self::CreditCardPay => "credit-card-pay",
            Self::CreditCardRefund => "credit-card-refund",
            Self::Cup => "cup",
            Self::CupOff => "cup-off",
            Self::Currency => "currency",
            Self::CurrencyAfghani => "currency-afghani",
            Self::CurrencyBahraini => "currency-bahraini",
            Self::CurrencyBaht => "currency-baht",
            Self::CurrencyBitcoin => "currency-bitcoin",
            Self::CurrencyCent => "currency-cent",
            Self::CurrencyDinar => "currency-dinar",
            Self::CurrencyDirham => "currency-dirham",
            Self::CurrencyDogecoin => "currency-dogecoin",
            Self::CurrencyDollar => "currency-dollar",
            Self::CurrencyDollarAustralian => "currency-dollar-australian",
            Self::CurrencyDollarBrunei => "currency-dollar-brunei",
            Self::CurrencyDollarCanadian => "currency-dollar-canadian",
            Self::CurrencyDollarGuyanese => "currency-dollar-guyanese",
            Self::CurrencyDollarOff => "currency-dollar-off",
            Self::CurrencyDollarSingapore => "currency-dollar-singapore",
            Self::CurrencyDollarZimbabwean => "currency-dollar-zimbabwean",
            Self::CurrencyDong => "currency-dong",
            Self::CurrencyDram => "currency-dram",
            Self::CurrencyEthereum => "currency-ethereum",
            Self::CurrencyEuro => "currency-euro",
            Self::CurrencyEuroOff => "currency-euro-off",
            Self::CurrencyFlorin => "currency-florin",
            Self::CurrencyForint => "currency-forint",
            Self::CurrencyFrank => "currency-frank",
            Self::CurrencyGuarani => "currency-guarani",
            Self::CurrencyHryvnia => "currency-hryvnia",
            Self::CurrencyHusd => "currency-husd",
            Self::CurrencyIranianRial => "currency-iranian-rial",
            Self::CurrencyKip => "currency-kip",
            Self::CurrencyKroneCzech => "currency-krone-czech",
            Self::CurrencyKroneDanish => "currency-krone-danish",
            Self::CurrencyKroneSwedish => "currency-krone-swedish",
            Self::CurrencyLari => "currency-lari",
            Self::CurrencyLeu => "currency-leu",
            Self::CurrencyLira => "currency-lira",
            Self::CurrencyLitecoin => "currency-litecoin",
            Self::CurrencyLyd => "currency-lyd",
            Self::CurrencyManat => "currency-manat",
            Self::CurrencyMonero => "currency-monero",
            Self::CurrencyNaira => "currency-naira",
            Self::CurrencyNano => "currency-nano",
            Self::CurrencyOff => "currency-off",
            Self::CurrencyPaanga => "currency-paanga",
            Self::CurrencyPeso => "currency-peso",
            Self::CurrencyPound => "currency-pound",
            Self::CurrencyPoundOff => "currency-pound-off",
            Self::CurrencyQuetzal => "currency-quetzal",
            Self::CurrencyReal => "currency-real",
            Self::CurrencyRenminbi => "currency-renminbi",
            Self::CurrencyRipple => "currency-ripple",
            Self::CurrencyRiyal => "currency-riyal",
            Self::CurrencyRubel => "currency-rubel",
            Self::CurrencyRufiyaa => "currency-rufiyaa",
            Self::CurrencyRupee => "currency-rupee",
            Self::CurrencyRupeeNepalese => "currency-rupee-nepalese",
            Self::CurrencyShekel => "currency-shekel",
            Self::CurrencySolana => "currency-solana",
            Self::CurrencySom => "currency-som",
            Self::CurrencyTaka => "currency-taka",
            Self::CurrencyTenge => "currency-tenge",
            Self::CurrencyTether => "currency-tether",
            Self::CurrencyTugrik => "currency-tugrik",
            Self::CurrencyWon => "currency-won",
            Self::CurrencyXrp => "currency-xrp",
            Self::CurrencyYen => "currency-yen",
            Self::CurrencyYenOff => "currency-yen-off",
            Self::CurrencyYuan => "currency-yuan",
            Self::CurrencyZcash => "currency-zcash",
            Self::CurrencyZloty => "currency-zloty",
            Self::Diaper => "diaper",
            Self::Discount => "discount",
            Self::DiscountOff => "discount-off",
            Self::DogBowl => "dog-bowl",
            Self::Dumpling => "dumpling",
            Self::EPassport => "e-passport",
            Self::Egg => "egg",
            Self::EggCracked => "egg-cracked",
            Self::EggFried => "egg-fried",
            Self::EggOff => "egg-off",
            Self::Eggs => "eggs",
            Self::FlipFlops => "flip-flops",
            Self::Gift => "gift",
            Self::GiftCard => "gift-card",
            Self::GiftOff => "gift-off",
            Self::Glass => "glass",
            Self::GlassChampagne => "glass-champagne",
            Self::GlassCocktail => "glass-cocktail",
            Self::GlassFull => "glass-full",
            Self::GlassGin => "glass-gin",
            Self::GlassOff => "glass-off",
            Self::Grape => "grape",
            Self::Grill => "grill",
            Self::GrillFork => "grill-fork",
            Self::GrillOff => "grill-off",
            Self::GrillSpatula => "grill-spatula",
            Self::Hanger => "hanger",
            Self::HangerOff => "hanger-off",
            Self::IceCream => "ice-cream",
            Self::IceCream2 => "ice-cream-2",
            Self::IceCreamOff => "ice-cream-off",
            Self::Ironing => "ironing",
            Self::Ironing1 => "ironing-1",
            Self::Ironing2 => "ironing-2",
            Self::Ironing3 => "ironing-3",
            Self::IroningOff => "ironing-off",
            Self::IroningSteam => "ironing-steam",
            Self::IroningSteamOff => "ironing-steam-off",
            Self::Jacket => "jacket",
            Self::Lemon => "lemon",
            Self::Lemon2 => "lemon-2",
            Self::Lollipop => "lollipop",
            Self::LollipopOff => "lollipop-off",
            Self::Meat => "meat",
            Self::MeatOff => "meat-off",
            Self::Melon => "melon",
            Self::MichelinBibGourmand => "michelin-bib-gourmand",
            Self::MichelinStar => "michelin-star",
            Self::MichelinStarGreen => "michelin-star-green",
            Self::Microwave => "microwave",
            Self::MicrowaveOff => "microwave-off",
            Self::Milk => "milk",
            Self::MilkOff => "milk-off",
            Self::Milkshake => "milkshake",
            Self::Moneybag => "moneybag",
            Self::MoneybagEdit => "moneybag-edit",
            Self::MoneybagHeart => "moneybag-heart",
            Self::MoneybagMinus => "moneybag-minus",
            Self::MoneybagMove => "moneybag-move",
            Self::MoneybagMoveBack => "moneybag-move-back",
            Self::MoneybagPlus => "moneybag-plus",
            Self::Mug => "mug",
            Self::MugOff => "mug-off",
            Self::Mushroom => "mushroom",
            Self::MushroomOff => "mushroom-off",
            Self::Nut => "nut",
            Self::Package => "package",
            Self::PackageOff => "package-off",
            Self::PaperBag => "paper-bag",
            Self::PaperBagOff => "paper-bag-off",
            Self::Pepper => "pepper",
            Self::PepperOff => "pepper-off",
            Self::Perfume => "perfume",
            Self::Pizza => "pizza",
            Self::PizzaOff => "pizza-off",
            Self::ReceiptBitcoin => "receipt-bitcoin",
            Self::ReceiptEuro => "receipt-euro",
            Self::ReceiptPound => "receipt-pound",
            Self::ReceiptRupee => "receipt-rupee",
            Self::ReceiptYen => "receipt-yen",
            Self::ReceiptYuan => "receipt-yuan",
            Self::RosetteDiscount => "rosette-discount",
            Self::RosetteDiscountCheck => "rosette-discount-check",
            Self::RosetteDiscountCheckOff => "rosette-discount-check-off",
            Self::RosetteDiscountOff => "rosette-discount-off",
            Self::Salad => "salad",
            Self::Salt => "salt",
            Self::Sausage => "sausage",
            Self::Shirt => "shirt",
            Self::ShirtOff => "shirt-off",
            Self::ShirtSport => "shirt-sport",
            Self::Shoe => "shoe",
            Self::ShoeOff => "shoe-off",
            Self::ShoppingBag => "shopping-bag",
            Self::ShoppingBagCheck => "shopping-bag-check",
            Self::ShoppingBagDiscount => "shopping-bag-discount",
            Self::ShoppingBagEdit => "shopping-bag-edit",
            Self::ShoppingBagExclamation => "shopping-bag-exclamation",
            Self::ShoppingBagHeart => "shopping-bag-heart",
            Self::ShoppingBagMinus => "shopping-bag-minus",
            Self::ShoppingBagPlus => "shopping-bag-plus",
            Self::ShoppingBagSearch => "shopping-bag-search",
            Self::ShoppingBagX => "shopping-bag-x",
            Self::ShoppingCart => "shopping-cart",
            Self::ShoppingCartBolt => "shopping-cart-bolt",
            Self::ShoppingCartCancel => "shopping-cart-cancel",
            Self::ShoppingCartCheck => "shopping-cart-check",
            Self::ShoppingCartCode => "shopping-cart-code",
            Self::ShoppingCartCog => "shopping-cart-cog",
            Self::ShoppingCartCopy => "shopping-cart-copy",
            Self::ShoppingCartDiscount => "shopping-cart-discount",
            Self::ShoppingCartDollar => "shopping-cart-dollar",
            Self::ShoppingCartDown => "shopping-cart-down",
            Self::ShoppingCartExclamation => "shopping-cart-exclamation",
            Self::ShoppingCartHeart => "shopping-cart-heart",
            Self::ShoppingCartMinus => "shopping-cart-minus",
            Self::ShoppingCartOff => "shopping-cart-off",
            Self::ShoppingCartPause => "shopping-cart-pause",
            Self::ShoppingCartPin => "shopping-cart-pin",
            Self::ShoppingCartPlus => "shopping-cart-plus",
            Self::ShoppingCartQuestion => "shopping-cart-question",
            Self::ShoppingCartSearch => "shopping-cart-search",
            Self::ShoppingCartShare => "shopping-cart-share",
            Self::ShoppingCartStar => "shopping-cart-star",
            Self::ShoppingCartUp => "shopping-cart-up",
            Self::ShoppingCartX => "shopping-cart-x",
            Self::Sock => "sock",
            Self::Soup => "soup",
            Self::SoupOff => "soup-off",
            Self::Tag => "tag",
            Self::TagMinus => "tag-minus",
            Self::TagOff => "tag-off",
            Self::TagPlus => "tag-plus",
            Self::TagStarred => "tag-starred",
            Self::Tags => "tags",
            Self::TagsOff => "tags-off",
            Self::TaiwanDollar => "taiwan-dollar",
            Self::Tax => "tax",
            Self::TaxEuro => "tax-euro",
            Self::TaxPound => "tax-pound",
            Self::Teapot => "teapot",
            Self::Tie => "tie",
            Self::ToolsKitchen => "tools-kitchen",
            Self::ToolsKitchen2 => "tools-kitchen-2",
            Self::ToolsKitchen3 => "tools-kitchen-3",
            Self::TransactionBitcoin => "transaction-bitcoin",
            Self::TransactionDollar => "transaction-dollar",
            Self::TransactionEuro => "transaction-euro",
            Self::TransactionPound => "transaction-pound",
            Self::TransactionRupee => "transaction-rupee",
            Self::TransactionYen => "transaction-yen",
            Self::TransactionYuan => "transaction-yuan",
            Self::TransferIn => "transfer-in",
            Self::TransferOut => "transfer-out",
            Self::TruckDelivery => "truck-delivery",
            Self::TruckLoading => "truck-loading",
            Self::TruckReturn => "truck-return",
            Self::Wallet => "wallet",
            Self::WalletOff => "wallet-off",
            Self::Wash => "wash",
            Self::WashDry => "wash-dry",
            Self::WashDry1 => "wash-dry-1",
            Self::WashDry2 => "wash-dry-2",
            Self::WashDry3 => "wash-dry-3",
            Self::WashDryA => "wash-dry-a",
            Self::WashDryDip => "wash-dry-dip",
            Self::WashDryF => "wash-dry-f",
            Self::WashDryFlat => "wash-dry-flat",
            Self::WashDryHang => "wash-dry-hang",
            Self::WashDryOff => "wash-dry-off",
            Self::WashDryP => "wash-dry-p",
            Self::WashDryShade => "wash-dry-shade",
            Self::WashDryW => "wash-dry-w",
            Self::WashDryclean => "wash-dryclean",
            Self::WashDrycleanOff => "wash-dryclean-off",
            Self::WashEco => "wash-eco",
            Self::WashGentle => "wash-gentle",
            Self::WashHand => "wash-hand",
            Self::WashOff => "wash-off",
            Self::WashPress => "wash-press",
            Self::WashTemperature1 => "wash-temperature-1",
            Self::WashTemperature2 => "wash-temperature-2",
            Self::WashTemperature3 => "wash-temperature-3",
            Self::WashTemperature4 => "wash-temperature-4",
            Self::WashTemperature5 => "wash-temperature-5",
            Self::WashTemperature6 => "wash-temperature-6",
            Self::WashTumbleDry => "wash-tumble-dry",
            Self::WashTumbleOff => "wash-tumble-off",
            Self::Wheat => "wheat",
            Self::WheatOff => "wheat-off",
            Self::Whisk => "whisk",
        }
    }

    fn outline_svg(&self) -> &'static str {
        match self {
            Self::Apple => APPLE_SVG,
            Self::Avocado => AVOCADO_SVG,
            Self::Backpack => BACKPACK_SVG,
            Self::BackpackOff => BACKPACK_OFF_SVG,
            Self::Baguette => BAGUETTE_SVG,
            Self::Banana => BANANA_SVG,
            Self::Basket => BASKET_SVG,
            Self::BasketBolt => BASKET_BOLT_SVG,
            Self::BasketCancel => BASKET_CANCEL_SVG,
            Self::BasketCheck => BASKET_CHECK_SVG,
            Self::BasketCode => BASKET_CODE_SVG,
            Self::BasketCog => BASKET_COG_SVG,
            Self::BasketDiscount => BASKET_DISCOUNT_SVG,
            Self::BasketDollar => BASKET_DOLLAR_SVG,
            Self::BasketDown => BASKET_DOWN_SVG,
            Self::BasketExclamation => BASKET_EXCLAMATION_SVG,
            Self::BasketHeart => BASKET_HEART_SVG,
            Self::BasketMinus => BASKET_MINUS_SVG,
            Self::BasketOff => BASKET_OFF_SVG,
            Self::BasketPause => BASKET_PAUSE_SVG,
            Self::BasketPin => BASKET_PIN_SVG,
            Self::BasketPlus => BASKET_PLUS_SVG,
            Self::BasketQuestion => BASKET_QUESTION_SVG,
            Self::BasketSearch => BASKET_SEARCH_SVG,
            Self::BasketShare => BASKET_SHARE_SVG,
            Self::BasketStar => BASKET_STAR_SVG,
            Self::BasketUp => BASKET_UP_SVG,
            Self::BasketX => BASKET_X_SVG,
            Self::Beer => BEER_SVG,
            Self::BeerOff => BEER_OFF_SVG,
            Self::Bleach => BLEACH_SVG,
            Self::BleachChlorine => BLEACH_CHLORINE_SVG,
            Self::BleachNoChlorine => BLEACH_NO_CHLORINE_SVG,
            Self::BleachOff => BLEACH_OFF_SVG,
            Self::Bone => BONE_SVG,
            Self::BoneOff => BONE_OFF_SVG,
            Self::Bottle => BOTTLE_SVG,
            Self::BottleOff => BOTTLE_OFF_SVG,
            Self::Bowl => BOWL_SVG,
            Self::BowlChopsticks => BOWL_CHOPSTICKS_SVG,
            Self::BowlSpoon => BOWL_SPOON_SVG,
            Self::Bread => BREAD_SVG,
            Self::BreadOff => BREAD_OFF_SVG,
            Self::BubbleTea => BUBBLE_TEA_SVG,
            Self::BubbleTea2 => BUBBLE_TEA_2_SVG,
            Self::Burger => BURGER_SVG,
            Self::Businessplan => BUSINESSPLAN_SVG,
            Self::Cake => CAKE_SVG,
            Self::CakeOff => CAKE_OFF_SVG,
            Self::CakeRoll => CAKE_ROLL_SVG,
            Self::Candy => CANDY_SVG,
            Self::CandyOff => CANDY_OFF_SVG,
            Self::Carambola => CARAMBOLA_SVG,
            Self::Carrot => CARROT_SVG,
            Self::CarrotOff => CARROT_OFF_SVG,
            Self::Cash => CASH_SVG,
            Self::CashBanknote => CASH_BANKNOTE_SVG,
            Self::CashBanknoteEdit => CASH_BANKNOTE_EDIT_SVG,
            Self::CashBanknoteHeart => CASH_BANKNOTE_HEART_SVG,
            Self::CashBanknoteMinus => CASH_BANKNOTE_MINUS_SVG,
            Self::CashBanknoteMove => CASH_BANKNOTE_MOVE_SVG,
            Self::CashBanknoteMoveBack => CASH_BANKNOTE_MOVE_BACK_SVG,
            Self::CashBanknoteOff => CASH_BANKNOTE_OFF_SVG,
            Self::CashBanknotePlus => CASH_BANKNOTE_PLUS_SVG,
            Self::CashEdit => CASH_EDIT_SVG,
            Self::CashHeart => CASH_HEART_SVG,
            Self::CashMinus => CASH_MINUS_SVG,
            Self::CashMove => CASH_MOVE_SVG,
            Self::CashMoveBack => CASH_MOVE_BACK_SVG,
            Self::CashOff => CASH_OFF_SVG,
            Self::CashPlus => CASH_PLUS_SVG,
            Self::CashRegister => CASH_REGISTER_SVG,
            Self::Cheese => CHEESE_SVG,
            Self::ChefHat => CHEF_HAT_SVG,
            Self::ChefHatOff => CHEF_HAT_OFF_SVG,
            Self::Chocolate => CHOCOLATE_SVG,
            Self::ClothesRack => CLOTHES_RACK_SVG,
            Self::ClothesRackOff => CLOTHES_RACK_OFF_SVG,
            Self::Coffee => COFFEE_SVG,
            Self::CoffeeOff => COFFEE_OFF_SVG,
            Self::Coin => COIN_SVG,
            Self::CoinBitcoin => COIN_BITCOIN_SVG,
            Self::CoinEuro => COIN_EURO_SVG,
            Self::CoinMonero => COIN_MONERO_SVG,
            Self::CoinOff => COIN_OFF_SVG,
            Self::CoinPound => COIN_POUND_SVG,
            Self::CoinRupee => COIN_RUPEE_SVG,
            Self::CoinTaka => COIN_TAKA_SVG,
            Self::CoinYen => COIN_YEN_SVG,
            Self::CoinYuan => COIN_YUAN_SVG,
            Self::Coins => COINS_SVG,
            Self::Cooker => COOKER_SVG,
            Self::Cookie => COOKIE_SVG,
            Self::CookieOff => COOKIE_OFF_SVG,
            Self::CreditCard => CREDIT_CARD_SVG,
            Self::CreditCardHand => CREDIT_CARD_HAND_SVG,
            Self::CreditCardOff => CREDIT_CARD_OFF_SVG,
            Self::CreditCardPay => CREDIT_CARD_PAY_SVG,
            Self::CreditCardRefund => CREDIT_CARD_REFUND_SVG,
            Self::Cup => CUP_SVG,
            Self::CupOff => CUP_OFF_SVG,
            Self::Currency => CURRENCY_SVG,
            Self::CurrencyAfghani => CURRENCY_AFGHANI_SVG,
            Self::CurrencyBahraini => CURRENCY_BAHRAINI_SVG,
            Self::CurrencyBaht => CURRENCY_BAHT_SVG,
            Self::CurrencyBitcoin => CURRENCY_BITCOIN_SVG,
            Self::CurrencyCent => CURRENCY_CENT_SVG,
            Self::CurrencyDinar => CURRENCY_DINAR_SVG,
            Self::CurrencyDirham => CURRENCY_DIRHAM_SVG,
            Self::CurrencyDogecoin => CURRENCY_DOGECOIN_SVG,
            Self::CurrencyDollar => CURRENCY_DOLLAR_SVG,
            Self::CurrencyDollarAustralian => CURRENCY_DOLLAR_AUSTRALIAN_SVG,
            Self::CurrencyDollarBrunei => CURRENCY_DOLLAR_BRUNEI_SVG,
            Self::CurrencyDollarCanadian => CURRENCY_DOLLAR_CANADIAN_SVG,
            Self::CurrencyDollarGuyanese => CURRENCY_DOLLAR_GUYANESE_SVG,
            Self::CurrencyDollarOff => CURRENCY_DOLLAR_OFF_SVG,
            Self::CurrencyDollarSingapore => CURRENCY_DOLLAR_SINGAPORE_SVG,
            Self::CurrencyDollarZimbabwean => CURRENCY_DOLLAR_ZIMBABWEAN_SVG,
            Self::CurrencyDong => CURRENCY_DONG_SVG,
            Self::CurrencyDram => CURRENCY_DRAM_SVG,
            Self::CurrencyEthereum => CURRENCY_ETHEREUM_SVG,
            Self::CurrencyEuro => CURRENCY_EURO_SVG,
            Self::CurrencyEuroOff => CURRENCY_EURO_OFF_SVG,
            Self::CurrencyFlorin => CURRENCY_FLORIN_SVG,
            Self::CurrencyForint => CURRENCY_FORINT_SVG,
            Self::CurrencyFrank => CURRENCY_FRANK_SVG,
            Self::CurrencyGuarani => CURRENCY_GUARANI_SVG,
            Self::CurrencyHryvnia => CURRENCY_HRYVNIA_SVG,
            Self::CurrencyHusd => CURRENCY_HUSD_SVG,
            Self::CurrencyIranianRial => CURRENCY_IRANIAN_RIAL_SVG,
            Self::CurrencyKip => CURRENCY_KIP_SVG,
            Self::CurrencyKroneCzech => CURRENCY_KRONE_CZECH_SVG,
            Self::CurrencyKroneDanish => CURRENCY_KRONE_DANISH_SVG,
            Self::CurrencyKroneSwedish => CURRENCY_KRONE_SWEDISH_SVG,
            Self::CurrencyLari => CURRENCY_LARI_SVG,
            Self::CurrencyLeu => CURRENCY_LEU_SVG,
            Self::CurrencyLira => CURRENCY_LIRA_SVG,
            Self::CurrencyLitecoin => CURRENCY_LITECOIN_SVG,
            Self::CurrencyLyd => CURRENCY_LYD_SVG,
            Self::CurrencyManat => CURRENCY_MANAT_SVG,
            Self::CurrencyMonero => CURRENCY_MONERO_SVG,
            Self::CurrencyNaira => CURRENCY_NAIRA_SVG,
            Self::CurrencyNano => CURRENCY_NANO_SVG,
            Self::CurrencyOff => CURRENCY_OFF_SVG,
            Self::CurrencyPaanga => CURRENCY_PAANGA_SVG,
            Self::CurrencyPeso => CURRENCY_PESO_SVG,
            Self::CurrencyPound => CURRENCY_POUND_SVG,
            Self::CurrencyPoundOff => CURRENCY_POUND_OFF_SVG,
            Self::CurrencyQuetzal => CURRENCY_QUETZAL_SVG,
            Self::CurrencyReal => CURRENCY_REAL_SVG,
            Self::CurrencyRenminbi => CURRENCY_RENMINBI_SVG,
            Self::CurrencyRipple => CURRENCY_RIPPLE_SVG,
            Self::CurrencyRiyal => CURRENCY_RIYAL_SVG,
            Self::CurrencyRubel => CURRENCY_RUBEL_SVG,
            Self::CurrencyRufiyaa => CURRENCY_RUFIYAA_SVG,
            Self::CurrencyRupee => CURRENCY_RUPEE_SVG,
            Self::CurrencyRupeeNepalese => CURRENCY_RUPEE_NEPALESE_SVG,
            Self::CurrencyShekel => CURRENCY_SHEKEL_SVG,
            Self::CurrencySolana => CURRENCY_SOLANA_SVG,
            Self::CurrencySom => CURRENCY_SOM_SVG,
            Self::CurrencyTaka => CURRENCY_TAKA_SVG,
            Self::CurrencyTenge => CURRENCY_TENGE_SVG,
            Self::CurrencyTether => CURRENCY_TETHER_SVG,
            Self::CurrencyTugrik => CURRENCY_TUGRIK_SVG,
            Self::CurrencyWon => CURRENCY_WON_SVG,
            Self::CurrencyXrp => CURRENCY_XRP_SVG,
            Self::CurrencyYen => CURRENCY_YEN_SVG,
            Self::CurrencyYenOff => CURRENCY_YEN_OFF_SVG,
            Self::CurrencyYuan => CURRENCY_YUAN_SVG,
            Self::CurrencyZcash => CURRENCY_ZCASH_SVG,
            Self::CurrencyZloty => CURRENCY_ZLOTY_SVG,
            Self::Diaper => DIAPER_SVG,
            Self::Discount => DISCOUNT_SVG,
            Self::DiscountOff => DISCOUNT_OFF_SVG,
            Self::DogBowl => DOG_BOWL_SVG,
            Self::Dumpling => DUMPLING_SVG,
            Self::EPassport => E_PASSPORT_SVG,
            Self::Egg => EGG_SVG,
            Self::EggCracked => EGG_CRACKED_SVG,
            Self::EggFried => EGG_FRIED_SVG,
            Self::EggOff => EGG_OFF_SVG,
            Self::Eggs => EGGS_SVG,
            Self::FlipFlops => FLIP_FLOPS_SVG,
            Self::Gift => GIFT_SVG,
            Self::GiftCard => GIFT_CARD_SVG,
            Self::GiftOff => GIFT_OFF_SVG,
            Self::Glass => GLASS_SVG,
            Self::GlassChampagne => GLASS_CHAMPAGNE_SVG,
            Self::GlassCocktail => GLASS_COCKTAIL_SVG,
            Self::GlassFull => GLASS_FULL_SVG,
            Self::GlassGin => GLASS_GIN_SVG,
            Self::GlassOff => GLASS_OFF_SVG,
            Self::Grape => GRAPE_SVG,
            Self::Grill => GRILL_SVG,
            Self::GrillFork => GRILL_FORK_SVG,
            Self::GrillOff => GRILL_OFF_SVG,
            Self::GrillSpatula => GRILL_SPATULA_SVG,
            Self::Hanger => HANGER_SVG,
            Self::HangerOff => HANGER_OFF_SVG,
            Self::IceCream => ICE_CREAM_SVG,
            Self::IceCream2 => ICE_CREAM_2_SVG,
            Self::IceCreamOff => ICE_CREAM_OFF_SVG,
            Self::Ironing => IRONING_SVG,
            Self::Ironing1 => IRONING_1_SVG,
            Self::Ironing2 => IRONING_2_SVG,
            Self::Ironing3 => IRONING_3_SVG,
            Self::IroningOff => IRONING_OFF_SVG,
            Self::IroningSteam => IRONING_STEAM_SVG,
            Self::IroningSteamOff => IRONING_STEAM_OFF_SVG,
            Self::Jacket => JACKET_SVG,
            Self::Lemon => LEMON_SVG,
            Self::Lemon2 => LEMON_2_SVG,
            Self::Lollipop => LOLLIPOP_SVG,
            Self::LollipopOff => LOLLIPOP_OFF_SVG,
            Self::Meat => MEAT_SVG,
            Self::MeatOff => MEAT_OFF_SVG,
            Self::Melon => MELON_SVG,
            Self::MichelinBibGourmand => MICHELIN_BIB_GOURMAND_SVG,
            Self::MichelinStar => MICHELIN_STAR_SVG,
            Self::MichelinStarGreen => MICHELIN_STAR_GREEN_SVG,
            Self::Microwave => MICROWAVE_SVG,
            Self::MicrowaveOff => MICROWAVE_OFF_SVG,
            Self::Milk => MILK_SVG,
            Self::MilkOff => MILK_OFF_SVG,
            Self::Milkshake => MILKSHAKE_SVG,
            Self::Moneybag => MONEYBAG_SVG,
            Self::MoneybagEdit => MONEYBAG_EDIT_SVG,
            Self::MoneybagHeart => MONEYBAG_HEART_SVG,
            Self::MoneybagMinus => MONEYBAG_MINUS_SVG,
            Self::MoneybagMove => MONEYBAG_MOVE_SVG,
            Self::MoneybagMoveBack => MONEYBAG_MOVE_BACK_SVG,
            Self::MoneybagPlus => MONEYBAG_PLUS_SVG,
            Self::Mug => MUG_SVG,
            Self::MugOff => MUG_OFF_SVG,
            Self::Mushroom => MUSHROOM_SVG,
            Self::MushroomOff => MUSHROOM_OFF_SVG,
            Self::Nut => NUT_SVG,
            Self::Package => PACKAGE_SVG,
            Self::PackageOff => PACKAGE_OFF_SVG,
            Self::PaperBag => PAPER_BAG_SVG,
            Self::PaperBagOff => PAPER_BAG_OFF_SVG,
            Self::Pepper => PEPPER_SVG,
            Self::PepperOff => PEPPER_OFF_SVG,
            Self::Perfume => PERFUME_SVG,
            Self::Pizza => PIZZA_SVG,
            Self::PizzaOff => PIZZA_OFF_SVG,
            Self::ReceiptBitcoin => RECEIPT_BITCOIN_SVG,
            Self::ReceiptEuro => RECEIPT_EURO_SVG,
            Self::ReceiptPound => RECEIPT_POUND_SVG,
            Self::ReceiptRupee => RECEIPT_RUPEE_SVG,
            Self::ReceiptYen => RECEIPT_YEN_SVG,
            Self::ReceiptYuan => RECEIPT_YUAN_SVG,
            Self::RosetteDiscount => ROSETTE_DISCOUNT_SVG,
            Self::RosetteDiscountCheck => ROSETTE_DISCOUNT_CHECK_SVG,
            Self::RosetteDiscountCheckOff => ROSETTE_DISCOUNT_CHECK_OFF_SVG,
            Self::RosetteDiscountOff => ROSETTE_DISCOUNT_OFF_SVG,
            Self::Salad => SALAD_SVG,
            Self::Salt => SALT_SVG,
            Self::Sausage => SAUSAGE_SVG,
            Self::Shirt => SHIRT_SVG,
            Self::ShirtOff => SHIRT_OFF_SVG,
            Self::ShirtSport => SHIRT_SPORT_SVG,
            Self::Shoe => SHOE_SVG,
            Self::ShoeOff => SHOE_OFF_SVG,
            Self::ShoppingBag => SHOPPING_BAG_SVG,
            Self::ShoppingBagCheck => SHOPPING_BAG_CHECK_SVG,
            Self::ShoppingBagDiscount => SHOPPING_BAG_DISCOUNT_SVG,
            Self::ShoppingBagEdit => SHOPPING_BAG_EDIT_SVG,
            Self::ShoppingBagExclamation => SHOPPING_BAG_EXCLAMATION_SVG,
            Self::ShoppingBagHeart => SHOPPING_BAG_HEART_SVG,
            Self::ShoppingBagMinus => SHOPPING_BAG_MINUS_SVG,
            Self::ShoppingBagPlus => SHOPPING_BAG_PLUS_SVG,
            Self::ShoppingBagSearch => SHOPPING_BAG_SEARCH_SVG,
            Self::ShoppingBagX => SHOPPING_BAG_X_SVG,
            Self::ShoppingCart => SHOPPING_CART_SVG,
            Self::ShoppingCartBolt => SHOPPING_CART_BOLT_SVG,
            Self::ShoppingCartCancel => SHOPPING_CART_CANCEL_SVG,
            Self::ShoppingCartCheck => SHOPPING_CART_CHECK_SVG,
            Self::ShoppingCartCode => SHOPPING_CART_CODE_SVG,
            Self::ShoppingCartCog => SHOPPING_CART_COG_SVG,
            Self::ShoppingCartCopy => SHOPPING_CART_COPY_SVG,
            Self::ShoppingCartDiscount => SHOPPING_CART_DISCOUNT_SVG,
            Self::ShoppingCartDollar => SHOPPING_CART_DOLLAR_SVG,
            Self::ShoppingCartDown => SHOPPING_CART_DOWN_SVG,
            Self::ShoppingCartExclamation => SHOPPING_CART_EXCLAMATION_SVG,
            Self::ShoppingCartHeart => SHOPPING_CART_HEART_SVG,
            Self::ShoppingCartMinus => SHOPPING_CART_MINUS_SVG,
            Self::ShoppingCartOff => SHOPPING_CART_OFF_SVG,
            Self::ShoppingCartPause => SHOPPING_CART_PAUSE_SVG,
            Self::ShoppingCartPin => SHOPPING_CART_PIN_SVG,
            Self::ShoppingCartPlus => SHOPPING_CART_PLUS_SVG,
            Self::ShoppingCartQuestion => SHOPPING_CART_QUESTION_SVG,
            Self::ShoppingCartSearch => SHOPPING_CART_SEARCH_SVG,
            Self::ShoppingCartShare => SHOPPING_CART_SHARE_SVG,
            Self::ShoppingCartStar => SHOPPING_CART_STAR_SVG,
            Self::ShoppingCartUp => SHOPPING_CART_UP_SVG,
            Self::ShoppingCartX => SHOPPING_CART_X_SVG,
            Self::Sock => SOCK_SVG,
            Self::Soup => SOUP_SVG,
            Self::SoupOff => SOUP_OFF_SVG,
            Self::Tag => TAG_SVG,
            Self::TagMinus => TAG_MINUS_SVG,
            Self::TagOff => TAG_OFF_SVG,
            Self::TagPlus => TAG_PLUS_SVG,
            Self::TagStarred => TAG_STARRED_SVG,
            Self::Tags => TAGS_SVG,
            Self::TagsOff => TAGS_OFF_SVG,
            Self::TaiwanDollar => TAIWAN_DOLLAR_SVG,
            Self::Tax => TAX_SVG,
            Self::TaxEuro => TAX_EURO_SVG,
            Self::TaxPound => TAX_POUND_SVG,
            Self::Teapot => TEAPOT_SVG,
            Self::Tie => TIE_SVG,
            Self::ToolsKitchen => TOOLS_KITCHEN_SVG,
            Self::ToolsKitchen2 => TOOLS_KITCHEN_2_SVG,
            Self::ToolsKitchen3 => TOOLS_KITCHEN_3_SVG,
            Self::TransactionBitcoin => TRANSACTION_BITCOIN_SVG,
            Self::TransactionDollar => TRANSACTION_DOLLAR_SVG,
            Self::TransactionEuro => TRANSACTION_EURO_SVG,
            Self::TransactionPound => TRANSACTION_POUND_SVG,
            Self::TransactionRupee => TRANSACTION_RUPEE_SVG,
            Self::TransactionYen => TRANSACTION_YEN_SVG,
            Self::TransactionYuan => TRANSACTION_YUAN_SVG,
            Self::TransferIn => TRANSFER_IN_SVG,
            Self::TransferOut => TRANSFER_OUT_SVG,
            Self::TruckDelivery => TRUCK_DELIVERY_SVG,
            Self::TruckLoading => TRUCK_LOADING_SVG,
            Self::TruckReturn => TRUCK_RETURN_SVG,
            Self::Wallet => WALLET_SVG,
            Self::WalletOff => WALLET_OFF_SVG,
            Self::Wash => WASH_SVG,
            Self::WashDry => WASH_DRY_SVG,
            Self::WashDry1 => WASH_DRY_1_SVG,
            Self::WashDry2 => WASH_DRY_2_SVG,
            Self::WashDry3 => WASH_DRY_3_SVG,
            Self::WashDryA => WASH_DRY_A_SVG,
            Self::WashDryDip => WASH_DRY_DIP_SVG,
            Self::WashDryF => WASH_DRY_F_SVG,
            Self::WashDryFlat => WASH_DRY_FLAT_SVG,
            Self::WashDryHang => WASH_DRY_HANG_SVG,
            Self::WashDryOff => WASH_DRY_OFF_SVG,
            Self::WashDryP => WASH_DRY_P_SVG,
            Self::WashDryShade => WASH_DRY_SHADE_SVG,
            Self::WashDryW => WASH_DRY_W_SVG,
            Self::WashDryclean => WASH_DRYCLEAN_SVG,
            Self::WashDrycleanOff => WASH_DRYCLEAN_OFF_SVG,
            Self::WashEco => WASH_ECO_SVG,
            Self::WashGentle => WASH_GENTLE_SVG,
            Self::WashHand => WASH_HAND_SVG,
            Self::WashOff => WASH_OFF_SVG,
            Self::WashPress => WASH_PRESS_SVG,
            Self::WashTemperature1 => WASH_TEMPERATURE_1_SVG,
            Self::WashTemperature2 => WASH_TEMPERATURE_2_SVG,
            Self::WashTemperature3 => WASH_TEMPERATURE_3_SVG,
            Self::WashTemperature4 => WASH_TEMPERATURE_4_SVG,
            Self::WashTemperature5 => WASH_TEMPERATURE_5_SVG,
            Self::WashTemperature6 => WASH_TEMPERATURE_6_SVG,
            Self::WashTumbleDry => WASH_TUMBLE_DRY_SVG,
            Self::WashTumbleOff => WASH_TUMBLE_OFF_SVG,
            Self::Wheat => WHEAT_SVG,
            Self::WheatOff => WHEAT_OFF_SVG,
            Self::Whisk => WHISK_SVG,
        }
    }

    fn filled_svg(&self) -> Option<&'static str> {
        // Filled variants would be added here
        None
    }
}
