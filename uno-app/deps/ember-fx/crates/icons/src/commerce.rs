//! Commerce icons.
//!
//! This module provides SVG icons for shopping and e-commerce.
//! Enable the `commerce` feature to use these icons.

use leptos::prelude::*;

/// Commerce icon identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommerceIcon {
    // Shopping
    /// Shopping cart
    Cart,
    /// Shopping bag
    ShoppingBag,
    /// Basket
    Basket,
    /// Store/Shop
    Store,
    /// Storefront
    Storefront,
    /// Tag/Price tag
    Tag,
    /// Tags
    Tags,
    /// Barcode
    Barcode,
    /// QR Code
    QrCode,

    // Products
    /// Package/Box
    Package,
    /// Gift
    Gift,
    /// Gift card
    GiftCard,
    /// Product
    Product,
    /// Inventory
    Inventory,

    // Shipping
    /// Truck/Delivery
    Truck,
    /// Shipping box
    Shipping,
    /// Returns
    Returns,
    /// Warehouse
    Warehouse,

    // Actions
    /// Add to cart
    AddToCart,
    /// Remove from cart
    RemoveFromCart,
    /// Wishlist
    Wishlist,
    /// Compare
    Compare,
    /// Review/Rating
    Review,

    // Commerce
    /// Sale/Discount
    Sale,
    /// Coupon
    Coupon,
    /// Checkout
    Checkout,
    /// Order
    Order,
    /// Refund
    Refund,
}

impl CommerceIcon {
    /// Returns the icon name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Cart => "Cart",
            Self::ShoppingBag => "Shopping Bag",
            Self::Basket => "Basket",
            Self::Store => "Store",
            Self::Storefront => "Storefront",
            Self::Tag => "Tag",
            Self::Tags => "Tags",
            Self::Barcode => "Barcode",
            Self::QrCode => "QR Code",
            Self::Package => "Package",
            Self::Gift => "Gift",
            Self::GiftCard => "Gift Card",
            Self::Product => "Product",
            Self::Inventory => "Inventory",
            Self::Truck => "Truck",
            Self::Shipping => "Shipping",
            Self::Returns => "Returns",
            Self::Warehouse => "Warehouse",
            Self::AddToCart => "Add to Cart",
            Self::RemoveFromCart => "Remove from Cart",
            Self::Wishlist => "Wishlist",
            Self::Compare => "Compare",
            Self::Review => "Review",
            Self::Sale => "Sale",
            Self::Coupon => "Coupon",
            Self::Checkout => "Checkout",
            Self::Order => "Order",
            Self::Refund => "Refund",
        }
    }

    /// Parse a commerce icon from its name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "cart" | "shopping cart" => Some(Self::Cart),
            "shoppingbag" | "shopping-bag" | "shopping bag" => Some(Self::ShoppingBag),
            "basket" => Some(Self::Basket),
            "store" | "shop" => Some(Self::Store),
            "storefront" => Some(Self::Storefront),
            "tag" | "price tag" => Some(Self::Tag),
            "tags" => Some(Self::Tags),
            "barcode" => Some(Self::Barcode),
            "qrcode" | "qr-code" | "qr code" => Some(Self::QrCode),
            "package" | "box" => Some(Self::Package),
            "gift" => Some(Self::Gift),
            "giftcard" | "gift-card" | "gift card" => Some(Self::GiftCard),
            "product" => Some(Self::Product),
            "inventory" | "stock" => Some(Self::Inventory),
            "truck" | "delivery" => Some(Self::Truck),
            "shipping" => Some(Self::Shipping),
            "returns" | "return" => Some(Self::Returns),
            "warehouse" => Some(Self::Warehouse),
            "addtocart" | "add-to-cart" | "add to cart" => Some(Self::AddToCart),
            "removefromcart" | "remove-from-cart" | "remove from cart" => Some(Self::RemoveFromCart),
            "wishlist" | "favorites" => Some(Self::Wishlist),
            "compare" => Some(Self::Compare),
            "review" | "rating" => Some(Self::Review),
            "sale" | "discount" => Some(Self::Sale),
            "coupon" => Some(Self::Coupon),
            "checkout" => Some(Self::Checkout),
            "order" => Some(Self::Order),
            "refund" => Some(Self::Refund),
            _ => None,
        }
    }

    /// Returns all available commerce icons.
    pub fn all() -> &'static [CommerceIcon] {
        &[
            Self::Cart, Self::ShoppingBag, Self::Basket, Self::Store, Self::Storefront, Self::Tag, Self::Tags, Self::Barcode, Self::QrCode,
            Self::Package, Self::Gift, Self::GiftCard, Self::Product, Self::Inventory,
            Self::Truck, Self::Shipping, Self::Returns, Self::Warehouse,
            Self::AddToCart, Self::RemoveFromCart, Self::Wishlist, Self::Compare, Self::Review,
            Self::Sale, Self::Coupon, Self::Checkout, Self::Order, Self::Refund,
        ]
    }
}

// Commerce SVG constants
const CART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="9" cy="21" r="1"/><circle cx="20" cy="21" r="1"/><path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"/></svg>"##;

const SHOPPING_BAG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z"/><path d="M3 6h18"/><path d="M16 10a4 4 0 0 1-8 0"/></svg>"##;

const BASKET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M5 7h14l-1.5 12H6.5L5 7z"/><path d="M2 7h20"/><path d="M16 7l-4-5-4 5"/><path d="M9 11v5"/><path d="M12 11v5"/><path d="M15 11v5"/></svg>"##;

const STORE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>"##;

const STOREFRONT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3h18v6H3z"/><path d="M3 9v12h18V9"/><path d="M3 9c0 1.5 1.5 3 3 3s3-1.5 3-3"/><path d="M9 9c0 1.5 1.5 3 3 3s3-1.5 3-3"/><path d="M15 9c0 1.5 1.5 3 3 3s3-1.5 3-3"/><path d="M9 21v-6h6v6"/></svg>"##;

const TAG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12.586 2.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8 8a2 2 0 0 0 2.828 0l7.172-7.172a2 2 0 0 0 0-2.828l-8-8z"/><circle cx="7" cy="7" r="1"/></svg>"##;

const TAGS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M9 5H2v7l6.29 6.29c.94.94 2.48.94 3.42 0l3.58-3.58c.94-.94.94-2.48 0-3.42L9 5z"/><path d="M6 9.01V9"/><path d="M15 5h2v7l-6.29 6.29c-.62.63-.62 1.64 0 2.26l.71.71"/></svg>"##;

const BARCODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 5v14"/><path d="M6 5v14"/><path d="M9 5v14"/><path d="M13 5v14"/><path d="M16 5v14"/><path d="M19 5v14"/><path d="M21 5v14"/></svg>"##;

const QR_CODE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/><path d="M14 14h3v3h-3z"/><path d="M18 18h3v3h-3z"/><path d="M14 18h1"/><path d="M18 14v1"/><path d="M21 14h-1"/><path d="M14 21v-1"/></svg>"##;

const PACKAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M16.5 9.4l-9-5.19"/><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/></svg>"##;

const GIFT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 12 20 22 4 22 4 12"/><rect x="2" y="7" width="20" height="5"/><line x1="12" y1="22" x2="12" y2="7"/><path d="M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z"/><path d="M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z"/></svg>"##;

const GIFT_CARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="5" width="20" height="14" rx="2"/><path d="M12 5v14"/><path d="M2 12h10"/><path d="M12 12h10"/><path d="M8 9a2 2 0 1 0-2-2"/><path d="M16 9a2 2 0 1 1 2-2"/></svg>"##;

const PRODUCT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/><line x1="7" y1="7" x2="7.01" y2="7"/></svg>"##;

const INVENTORY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M4 4h16v16H4z"/><path d="M4 9h16"/><path d="M4 14h16"/><path d="M9 4v16"/><path d="M14 4v16"/></svg>"##;

const TRUCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="1" y="3" width="15" height="13"/><polygon points="16 8 20 8 23 11 23 16 16 16 16 8"/><circle cx="5.5" cy="18.5" r="2.5"/><circle cx="18.5" cy="18.5" r="2.5"/></svg>"##;

const SHIPPING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M16.5 9.4l-9-5.19"/><path d="M21 16V8l-9-5.19L3 8v8l9 5.19L21 16z"/><path d="M3.27 6.96L12 12.01l8.73-5.05"/><path d="M12 22.08V12"/><path d="M9 10l3 2 3-2"/></svg>"##;

const RETURNS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M8 16H3v5"/></svg>"##;

const WAREHOUSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 21h18"/><path d="M5 21V7l7-4 7 4v14"/><path d="M9 21v-6h6v6"/><path d="M10 10h4"/></svg>"##;

const ADD_TO_CART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="9" cy="21" r="1"/><circle cx="20" cy="21" r="1"/><path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"/><path d="M12 9v6"/><path d="M9 12h6"/></svg>"##;

const REMOVE_FROM_CART_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="9" cy="21" r="1"/><circle cx="20" cy="21" r="1"/><path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"/><path d="M9 12h6"/></svg>"##;

const WISHLIST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/></svg>"##;

const COMPARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="18"/><rect x="14" y="3" width="7" height="18"/><path d="M6.5 8h1"/><path d="M17.5 8h1"/><path d="M6.5 12h1"/><path d="M17.5 12h1"/><path d="M6.5 16h1"/><path d="M17.5 16h1"/></svg>"##;

const REVIEW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>"##;

const SALE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2L2 7v10l10 5 10-5V7L12 2z"/><path d="M8 10l8 8"/><circle cx="9" cy="9" r="1.5"/><circle cx="15" cy="15" r="1.5"/></svg>"##;

const COUPON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M2 9a3 3 0 0 1 3 3 3 3 0 0 1-3 3v4h20v-4a3 3 0 0 1-3-3 3 3 0 0 1 3-3V5H2v4z"/><path d="M13 5v2"/><path d="M13 11v2"/><path d="M13 17v2"/></svg>"##;

const CHECKOUT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 4H3"/><path d="M18 8H6"/><path d="M15 12H9"/><path d="M12 16l-3 4h6l-3-4z"/></svg>"##;

const ORDER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><path d="M14 2v6h6"/><path d="M9 13h6"/><path d="M9 17h3"/><path d="M9 9h1"/></svg>"##;

const REFUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><circle cx="12" cy="14" r="4"/><path d="M12 12v4"/><path d="M10 14h4"/></svg>"##;

/// Get the SVG content for a commerce icon.
#[must_use]
pub fn get_commerce_svg(icon: CommerceIcon) -> &'static str {
    match icon {
        CommerceIcon::Cart => CART_SVG,
        CommerceIcon::ShoppingBag => SHOPPING_BAG_SVG,
        CommerceIcon::Basket => BASKET_SVG,
        CommerceIcon::Store => STORE_SVG,
        CommerceIcon::Storefront => STOREFRONT_SVG,
        CommerceIcon::Tag => TAG_SVG,
        CommerceIcon::Tags => TAGS_SVG,
        CommerceIcon::Barcode => BARCODE_SVG,
        CommerceIcon::QrCode => QR_CODE_SVG,
        CommerceIcon::Package => PACKAGE_SVG,
        CommerceIcon::Gift => GIFT_SVG,
        CommerceIcon::GiftCard => GIFT_CARD_SVG,
        CommerceIcon::Product => PRODUCT_SVG,
        CommerceIcon::Inventory => INVENTORY_SVG,
        CommerceIcon::Truck => TRUCK_SVG,
        CommerceIcon::Shipping => SHIPPING_SVG,
        CommerceIcon::Returns => RETURNS_SVG,
        CommerceIcon::Warehouse => WAREHOUSE_SVG,
        CommerceIcon::AddToCart => ADD_TO_CART_SVG,
        CommerceIcon::RemoveFromCart => REMOVE_FROM_CART_SVG,
        CommerceIcon::Wishlist => WISHLIST_SVG,
        CommerceIcon::Compare => COMPARE_SVG,
        CommerceIcon::Review => REVIEW_SVG,
        CommerceIcon::Sale => SALE_SVG,
        CommerceIcon::Coupon => COUPON_SVG,
        CommerceIcon::Checkout => CHECKOUT_SVG,
        CommerceIcon::Order => ORDER_SVG,
        CommerceIcon::Refund => REFUND_SVG,
    }
}

/// Commerce icon component.
#[component]
pub fn Commerce(
    /// The commerce icon to display.
    icon: CommerceIcon,
    /// Optional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
    /// Optional aria-label for accessibility.
    #[prop(optional, into)]
    aria_label: Option<String>,
) -> impl IntoView {
    let svg = get_commerce_svg(icon);
    let label = aria_label.unwrap_or_else(|| icon.name().to_string());

    view! {
        <span
            class=class
            role="img"
            aria-label=label
            inner_html=svg
        />
    }
}
