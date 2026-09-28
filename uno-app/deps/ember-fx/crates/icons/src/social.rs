//! Social icons.
//!
//! This module provides SVG icons for social media and communication.
//! Enable the `social` feature to use these icons.

use leptos::prelude::*;

/// Social icon identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SocialIcon {
    // Communication
    /// Message/Chat bubble
    Message,
    /// Messages (multiple)
    Messages,
    /// Comment
    Comment,
    /// Send
    Send,
    /// Inbox
    Inbox,
    /// At sign (@)
    AtSign,
    /// Phone
    Phone,
    /// Video call
    Video,

    // Social actions
    /// Like/Thumbs up
    Like,
    /// Dislike/Thumbs down
    Dislike,
    /// Share
    Share,
    /// Bookmark
    Bookmark,
    /// Follow/Add user
    Follow,
    /// Unfollow/Remove user
    Unfollow,
    /// Repost/Retweet
    Repost,

    // Social platforms (generic representations)
    /// Globe/World (public)
    Globe,
    /// Network/Connections
    Network,
    /// Community/Group
    Community,
    /// Hashtag
    Hashtag,
    /// Verified badge
    Verified,

    // Media
    /// Camera
    Camera,
    /// Photo/Image
    Photo,
    /// Gallery
    Gallery,
    /// Story (circle)
    Story,
    /// Live streaming
    Live,

    // Profile
    /// Profile
    Profile,
    /// Friends
    Friends,
    /// Followers
    Followers,
    /// Block
    Block,
    /// Report
    Report,
}

impl SocialIcon {
    /// Returns the icon name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Message => "Message",
            Self::Messages => "Messages",
            Self::Comment => "Comment",
            Self::Send => "Send",
            Self::Inbox => "Inbox",
            Self::AtSign => "At",
            Self::Phone => "Phone",
            Self::Video => "Video",
            Self::Like => "Like",
            Self::Dislike => "Dislike",
            Self::Share => "Share",
            Self::Bookmark => "Bookmark",
            Self::Follow => "Follow",
            Self::Unfollow => "Unfollow",
            Self::Repost => "Repost",
            Self::Globe => "Globe",
            Self::Network => "Network",
            Self::Community => "Community",
            Self::Hashtag => "Hashtag",
            Self::Verified => "Verified",
            Self::Camera => "Camera",
            Self::Photo => "Photo",
            Self::Gallery => "Gallery",
            Self::Story => "Story",
            Self::Live => "Live",
            Self::Profile => "Profile",
            Self::Friends => "Friends",
            Self::Followers => "Followers",
            Self::Block => "Block",
            Self::Report => "Report",
        }
    }

    /// Parse a social icon from its name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "message" | "chat" => Some(Self::Message),
            "messages" => Some(Self::Messages),
            "comment" => Some(Self::Comment),
            "send" => Some(Self::Send),
            "inbox" => Some(Self::Inbox),
            "at" | "atsign" | "at-sign" | "@" => Some(Self::AtSign),
            "phone" | "call" => Some(Self::Phone),
            "video" | "videocall" | "video-call" => Some(Self::Video),
            "like" | "thumbsup" | "thumbs-up" => Some(Self::Like),
            "dislike" | "thumbsdown" | "thumbs-down" => Some(Self::Dislike),
            "share" => Some(Self::Share),
            "bookmark" | "save" => Some(Self::Bookmark),
            "follow" | "adduser" | "add-user" => Some(Self::Follow),
            "unfollow" | "removeuser" | "remove-user" => Some(Self::Unfollow),
            "repost" | "retweet" => Some(Self::Repost),
            "globe" | "world" | "public" => Some(Self::Globe),
            "network" | "connections" => Some(Self::Network),
            "community" | "group" => Some(Self::Community),
            "hashtag" | "tag" => Some(Self::Hashtag),
            "verified" | "badge" => Some(Self::Verified),
            "camera" => Some(Self::Camera),
            "photo" | "image" => Some(Self::Photo),
            "gallery" | "photos" => Some(Self::Gallery),
            "story" | "stories" => Some(Self::Story),
            "live" | "streaming" => Some(Self::Live),
            "profile" => Some(Self::Profile),
            "friends" => Some(Self::Friends),
            "followers" => Some(Self::Followers),
            "block" => Some(Self::Block),
            "report" | "flag" => Some(Self::Report),
            _ => None,
        }
    }

    /// Returns all available social icons.
    pub fn all() -> &'static [SocialIcon] {
        &[
            Self::Message, Self::Messages, Self::Comment, Self::Send, Self::Inbox, Self::AtSign, Self::Phone, Self::Video,
            Self::Like, Self::Dislike, Self::Share, Self::Bookmark, Self::Follow, Self::Unfollow, Self::Repost,
            Self::Globe, Self::Network, Self::Community, Self::Hashtag, Self::Verified,
            Self::Camera, Self::Photo, Self::Gallery, Self::Story, Self::Live,
            Self::Profile, Self::Friends, Self::Followers, Self::Block, Self::Report,
        ]
    }
}

// Social SVG constants
const MESSAGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>"##;

const MESSAGES_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"/></svg>"##;

const COMMENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/><path d="M8 10h8"/><path d="M8 14h4"/></svg>"##;

const SEND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><line x1="22" y1="2" x2="11" y2="13"/><polygon points="22 2 15 22 11 13 2 9 22 2"/></svg>"##;

const INBOX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/></svg>"##;

const AT_SIGN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4"/><path d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94"/></svg>"##;

const PHONE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"/></svg>"##;

const VIDEO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polygon points="23 7 16 12 23 17 23 7"/><rect x="1" y="5" width="15" height="14" rx="2" ry="2"/></svg>"##;

const LIKE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 9V5a3 3 0 0 0-3-3l-4 9v11h11.28a2 2 0 0 0 2-1.7l1.38-9a2 2 0 0 0-2-2.3zM7 22H4a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h3"/></svg>"##;

const DISLIKE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M10 15v4a3 3 0 0 0 3 3l4-9V2H5.72a2 2 0 0 0-2 1.7l-1.38 9a2 2 0 0 0 2 2.3zm7-13h2.67A2.31 2.31 0 0 1 22 4v7a2.31 2.31 0 0 1-2.33 2H17"/></svg>"##;

const SHARE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/><circle cx="18" cy="19" r="3"/><line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/><line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/></svg>"##;

const BOOKMARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z"/></svg>"##;

const FOLLOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="8.5" cy="7" r="4"/><line x1="20" y1="8" x2="20" y2="14"/><line x1="23" y1="11" x2="17" y2="11"/></svg>"##;

const UNFOLLOW_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="8.5" cy="7" r="4"/><line x1="23" y1="11" x2="17" y2="11"/></svg>"##;

const REPOST_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="17 1 21 5 17 9"/><path d="M3 11V9a4 4 0 0 1 4-4h14"/><polyline points="7 23 3 19 7 15"/><path d="M21 13v2a4 4 0 0 1-4 4H3"/></svg>"##;

const GLOBE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>"##;

const NETWORK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="5" r="3"/><circle cx="5" cy="19" r="3"/><circle cx="19" cy="19" r="3"/><line x1="12" y1="8" x2="5" y2="16"/><line x1="12" y1="8" x2="19" y2="16"/></svg>"##;

const COMMUNITY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>"##;

const HASHTAG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><line x1="4" y1="9" x2="20" y2="9"/><line x1="4" y1="15" x2="20" y2="15"/><line x1="10" y1="3" x2="8" y2="21"/><line x1="16" y1="3" x2="14" y2="21"/></svg>"##;

const VERIFIED_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/><path d="M9 12l2 2 4-4"/></svg>"##;

const CAMERA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"/><circle cx="12" cy="13" r="4"/></svg>"##;

const PHOTO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><circle cx="8.5" cy="8.5" r="1.5"/><polyline points="21 15 16 10 5 21"/></svg>"##;

const GALLERY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="8.5" cy="8.5" r="1.5"/><path d="M21 15l-5-5L5 21"/><rect x="6" y="6" width="12" height="12" rx="1"/></svg>"##;

const STORY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="6"/><path d="M12 2a10 10 0 0 1 0 20"/></svg>"##;

const LIVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M5.64 5.64a9 9 0 0 0 0 12.73"/><path d="M18.36 5.64a9 9 0 0 1 0 12.73"/><path d="M8.11 8.11a5 5 0 0 0 0 7.78"/><path d="M15.89 8.11a5 5 0 0 1 0 7.78"/></svg>"##;

const PROFILE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"##;

const FRIENDS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>"##;

const FOLLOWERS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="9" cy="7" r="4"/><path d="M3 21v-2a4 4 0 0 1 4-4h4a4 4 0 0 1 4 4v2"/><path d="M16 11l2 2 4-4"/></svg>"##;

const BLOCK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="4.93" y1="4.93" x2="19.07" y2="19.07"/></svg>"##;

const REPORT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z"/><line x1="4" y1="22" x2="4" y2="15"/></svg>"##;

/// Get the SVG content for a social icon.
#[must_use]
pub fn get_social_svg(icon: SocialIcon) -> &'static str {
    match icon {
        SocialIcon::Message => MESSAGE_SVG,
        SocialIcon::Messages => MESSAGES_SVG,
        SocialIcon::Comment => COMMENT_SVG,
        SocialIcon::Send => SEND_SVG,
        SocialIcon::Inbox => INBOX_SVG,
        SocialIcon::AtSign => AT_SIGN_SVG,
        SocialIcon::Phone => PHONE_SVG,
        SocialIcon::Video => VIDEO_SVG,
        SocialIcon::Like => LIKE_SVG,
        SocialIcon::Dislike => DISLIKE_SVG,
        SocialIcon::Share => SHARE_SVG,
        SocialIcon::Bookmark => BOOKMARK_SVG,
        SocialIcon::Follow => FOLLOW_SVG,
        SocialIcon::Unfollow => UNFOLLOW_SVG,
        SocialIcon::Repost => REPOST_SVG,
        SocialIcon::Globe => GLOBE_SVG,
        SocialIcon::Network => NETWORK_SVG,
        SocialIcon::Community => COMMUNITY_SVG,
        SocialIcon::Hashtag => HASHTAG_SVG,
        SocialIcon::Verified => VERIFIED_SVG,
        SocialIcon::Camera => CAMERA_SVG,
        SocialIcon::Photo => PHOTO_SVG,
        SocialIcon::Gallery => GALLERY_SVG,
        SocialIcon::Story => STORY_SVG,
        SocialIcon::Live => LIVE_SVG,
        SocialIcon::Profile => PROFILE_SVG,
        SocialIcon::Friends => FRIENDS_SVG,
        SocialIcon::Followers => FOLLOWERS_SVG,
        SocialIcon::Block => BLOCK_SVG,
        SocialIcon::Report => REPORT_SVG,
    }
}

/// Social icon component.
#[component]
pub fn Social(
    /// The social icon to display.
    icon: SocialIcon,
    /// Optional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
    /// Optional aria-label for accessibility.
    #[prop(optional, into)]
    aria_label: Option<String>,
) -> impl IntoView {
    let svg = get_social_svg(icon);
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
