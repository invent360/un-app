-- FAQ seed data
-- Run with: psql -d uno_app -f seeds/faq_data.sql

INSERT INTO faq_items (category, display_order, question_en, answer_en, is_featured, is_active)
VALUES
    ('general', 1, 'What is UNO?', 'UNO is a platform that allows you to earn passive income by sharing your unused internet bandwidth. Our app runs in the background and performs small network tasks that generate earnings for you.', true, true),
    ('general', 2, 'What devices are supported?', 'UNO works on Android phones and WiFi routers. iOS support is coming soon. For best results, keep the app running on a device with a stable WiFi connection.', false, true),
    ('general', 3, 'Can I use multiple devices?', 'Yes! You can run UNO on multiple devices with different licenses. Each device earns independently based on its connection quality and uptime.', false, true),
    ('earnings', 1, 'How much can I earn?', 'Earnings vary based on your device type, internet connection quality, and uptime. On average, users earn between $3-15 per month per device.', true, true),
    ('earnings', 2, 'How do I get paid?', 'Earnings accumulate in your account and can be withdrawn once you reach the minimum threshold. We support various payment methods including cryptocurrency and traditional bank transfers.', false, true),
    ('setup', 1, 'How do I install the app?', 'Download the UNO app from the Google Play Store, create an account, and register your license key. The app will start running tasks automatically.', false, true),
    ('setup', 2, 'What is a license key?', 'A license key is a unique code that activates your UNO account. You can get a free license from our website. Each license determines your earning split with the network operator.', false, true),
    ('security', 1, 'Is my data safe?', 'Yes! We only use your internet bandwidth for legitimate network tasks. We never access, collect, or transmit your personal data or browsing history.', true, true)
ON CONFLICT DO NOTHING;
