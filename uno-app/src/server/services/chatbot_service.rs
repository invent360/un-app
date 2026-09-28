//! Chatbot service for intent matching and responses

/// Intent types the chatbot can recognize
#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    Greeting,
    HowToEarn,
    HowToGetLicense,
    WhatIsUno,
    HowToInstall,
    PaymentInfo,
    DeviceSupport,
    Earnings,
    Security,
    ContactSupport,
    Unknown,
}

/// Chatbot service for handling user messages
pub struct ChatbotService;

impl ChatbotService {
    pub fn new() -> Self {
        Self
    }

    /// Match user input to an intent
    fn match_intent(&self, message: &str) -> Intent {
        let lower = message.to_lowercase();
        
        // Greeting patterns
        if lower.contains("hello") || lower.contains("hi") || lower.contains("hey") 
            || lower.contains("good morning") || lower.contains("good evening") {
            return Intent::Greeting;
        }

        // What is UNO
        if lower.contains("what is uno") || lower.contains("what's uno") 
            || lower.contains("explain uno") || lower.contains("about uno") {
            return Intent::WhatIsUno;
        }

        // How to earn
        if lower.contains("how") && (lower.contains("earn") || lower.contains("money") || lower.contains("income")) {
            return Intent::HowToEarn;
        }

        // Earnings questions
        if lower.contains("how much") || lower.contains("earning") || lower.contains("payment") 
            || lower.contains("payout") || lower.contains("withdraw") {
            return Intent::Earnings;
        }

        // License questions
        if lower.contains("license") || lower.contains("claim") || lower.contains("get started") {
            return Intent::HowToGetLicense;
        }

        // Installation
        if lower.contains("install") || lower.contains("download") || lower.contains("setup") 
            || lower.contains("set up") {
            return Intent::HowToInstall;
        }

        // Device support
        if lower.contains("device") || lower.contains("phone") || lower.contains("router") 
            || lower.contains("android") || lower.contains("ios") || lower.contains("iphone") {
            return Intent::DeviceSupport;
        }

        // Security
        if lower.contains("safe") || lower.contains("secure") || lower.contains("privacy") 
            || lower.contains("data") || lower.contains("trust") {
            return Intent::Security;
        }

        // Payment info
        if lower.contains("pay") || lower.contains("crypto") || lower.contains("bank") 
            || lower.contains("withdrawal") {
            return Intent::PaymentInfo;
        }

        // Contact support
        if lower.contains("support") || lower.contains("help") || lower.contains("contact") 
            || lower.contains("human") || lower.contains("agent") || lower.contains("talk to") {
            return Intent::ContactSupport;
        }

        Intent::Unknown
    }

    /// Get a response based on the matched intent
    pub fn get_response(&self, message: &str) -> String {
        let intent = self.match_intent(message);

        match intent {
            Intent::Greeting => {
                "Hello! Welcome to UNO Support. How can I help you today? You can ask me about:\n\
                • How to earn money\n\
                • Getting a license\n\
                • Installing the app\n\
                • Payment methods".to_string()
            }
            Intent::WhatIsUno => {
                "UNO is a platform that lets you earn passive income by sharing your unused internet bandwidth. \
                Our app runs quietly in the background on your device and performs small network tasks that generate earnings for you. \
                It's completely safe and doesn't affect your browsing or use your personal data.".to_string()
            }
            Intent::HowToEarn => {
                "Earning with UNO is simple:\n\
                1. Get a free license from our website\n\
                2. Download and install the UNO app\n\
                3. Register your license in the app\n\
                4. Keep the app running - you'll earn automatically!\n\n\
                Earnings depend on your device type and connection quality. Most users earn $3-15 per month per device.".to_string()
            }
            Intent::Earnings => {
                "Earnings vary based on several factors:\n\
                • WiFi Routers: $5-15/month\n\
                • Phones on WiFi: $3-10/month\n\
                • Phones on Mobile Data: $1-5/month\n\n\
                Your license split also affects earnings - a 70:30 split means you keep 70% of what your device generates. \
                Minimum withdrawal is typically reached within 1-2 months.".to_string()
            }
            Intent::HowToGetLicense => {
                "Getting a license is easy and free!\n\n\
                1. Go to the Licenses page on our website\n\
                2. Choose a split variant (70:30 gives you the highest share)\n\
                3. Click 'Claim' to receive your unique license key\n\
                4. Save your license key - you'll need it for the app\n\n\
                Would you like me to help you navigate to the licenses page?".to_string()
            }
            Intent::HowToInstall => {
                "To install UNO:\n\n\
                1. Download the UNO app from Google Play Store\n\
                2. Open the app and create an account\n\
                3. Enter your license key when prompted\n\
                4. Grant necessary permissions\n\
                5. That's it! The app will start earning automatically\n\n\
                For routers, we provide separate firmware - contact support for details.".to_string()
            }
            Intent::DeviceSupport => {
                "Currently supported devices:\n\
                • Android phones (Android 7.0+)\n\
                • WiFi routers (select models with custom firmware)\n\n\
                Coming soon:\n\
                • iOS devices\n\
                • Windows desktop app\n\n\
                For best results, use a device with a stable WiFi connection and keep the app running 24/7.".to_string()
            }
            Intent::Security => {
                "Your security is our priority:\n\n\
                • We NEVER access your personal data or browsing history\n\
                • We only use your spare bandwidth for legitimate network tasks\n\
                • All data transfers are encrypted\n\
                • The app is open-source and audited\n\
                • You control when the app runs\n\n\
                Our network is used for things like website testing, content delivery, and market research - all legal and ethical activities.".to_string()
            }
            Intent::PaymentInfo => {
                "Payment options:\n\
                • Cryptocurrency (USDT, BTC, ETH)\n\
                • Bank transfer (available in select countries)\n\
                • PayPal (coming soon)\n\n\
                Minimum withdrawal: $10\n\
                Processing time: 1-3 business days\n\n\
                Earnings accumulate in your account and you can request withdrawal anytime after reaching the minimum.".to_string()
            }
            Intent::ContactSupport => {
                "Need to talk to a human? No problem!\n\n\
                • Email: support@uno-network.com\n\
                • In-app chat: Available 24/7 in the UNO app\n\
                • Response time: Usually within 24 hours\n\n\
                For urgent issues, the in-app support is fastest. Is there anything specific I can help you with first?".to_string()
            }
            Intent::Unknown => {
                "I'm not sure I understand that question. Here are some things I can help with:\n\n\
                • \"What is UNO?\" - Learn about our platform\n\
                • \"How do I earn money?\" - Earning guide\n\
                • \"How do I get a license?\" - License info\n\
                • \"Is it safe?\" - Security information\n\
                • \"Contact support\" - Talk to a human\n\n\
                Or just ask your question in a different way!".to_string()
            }
        }
    }
}

impl Default for ChatbotService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting_intent() {
        let service = ChatbotService::new();
        assert_eq!(service.match_intent("Hello!"), Intent::Greeting);
        assert_eq!(service.match_intent("Hi there"), Intent::Greeting);
        assert_eq!(service.match_intent("hey"), Intent::Greeting);
    }

    #[test]
    fn test_earning_intents() {
        let service = ChatbotService::new();
        assert_eq!(service.match_intent("How do I earn money?"), Intent::HowToEarn);
        assert_eq!(service.match_intent("How much can I make?"), Intent::Earnings);
    }

    #[test]
    fn test_unknown_intent() {
        let service = ChatbotService::new();
        assert_eq!(service.match_intent("asdfasdf"), Intent::Unknown);
    }
}
