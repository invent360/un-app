-- Guides Seed Data
-- Run with: psql -d uno_app -f seeds/guides_data.sql
-- Or through sqlx after demo_data.sql

-- Clear existing guides (optional - comment out if you want to keep existing data)
-- DELETE FROM page_contents WHERE content_type = 'guide';

-- Insert Guide Content
INSERT INTO page_contents (
    content_type,
    slug,
    status,
    content,
    translations,
    translation_status,
    display_order,
    is_featured,
    is_active,
    version,
    published_version,
    published_at
) VALUES

-- 1. Installation Guide (NEW)
(
    'guide',
    'installation-guide',
    'published',
    '{
        "title": "Installation Guide",
        "description": "Learn how to download and install the Unetwork app on your Android or iOS device to start earning network incentives.",
        "thumbnail": "/assets/guides/install.png",
        "duration_minutes": 3,
        "difficulty": "easy",
        "direction": "ltr",
        "steps": [
            {
                "order": 1,
                "title": "Download the App",
                "description": "Visit the Google Play Store (Android) or Apple App Store (iOS) and search for \"Unetwork\". Tap the Install/Get button to download the app to your device.",
                "image": null
            },
            {
                "order": 2,
                "title": "Open the App",
                "description": "Once installed, tap the Unetwork icon on your home screen or app drawer to launch the application.",
                "image": null
            },
            {
                "order": 3,
                "title": "Accept Permissions",
                "description": "The app may request certain permissions for optimal functionality. Review and accept the necessary permissions to enable all features.",
                "image": null
            },
            {
                "order": 4,
                "title": "Ready to Set Up",
                "description": "Installation complete! You are now ready to set up your account and start earning network incentives. Proceed to the App Setup Guide for the next steps.",
                "image": null
            }
        ]
    }'::jsonb,
    '{}'::jsonb,
    '{}'::jsonb,
    1,
    true,
    true,
    1,
    1,
    NOW()
),

-- 2. App Setup Guide
(
    'guide',
    'app-setup-guide',
    'published',
    '{
        "title": "App Setup Guide",
        "description": "Complete guide to setting up Unetwork on your mobile device and start earning network incentives.",
        "thumbnail": "/assets/guides/setup.png",
        "duration_minutes": 5,
        "difficulty": "easy",
        "direction": "ltr",
        "steps": [
            {
                "order": 1,
                "title": "Welcome to Unetwork",
                "description": "Open the Unetwork app to see the welcome screen. Generate Network Incentives by Verifying the Telecom Grid from Your Mobile Phone. Click ''GET STARTED'' to proceed.",
                "image": null
            },
            {
                "order": 2,
                "title": "Choose Your Role",
                "description": "Select how you want to use Unetwork:\n\n- **Node Operator**: Manage nodes and licenses you own on-chain.\n- **License Operator**: Operate a single or multiple licenses.\n- **Claim a License**: Explore the marketplace to choose and claim yours.",
                "image": null
            },
            {
                "order": 3,
                "title": "Access Your Leased License",
                "description": "If you selected ''License Operator'', enter your lease code to continue. If you don''t have a lease code, tap ''Skip'' to continue without one.",
                "image": null
            },
            {
                "order": 4,
                "title": "Choose Sign-In Method",
                "description": "Select how you want to access Unetwork:\n\n- **Sign in with Web3**: Use your crypto wallet (MetaMask, WalletConnect, or Coinbase) to manage licenses.\n- **Sign in with Email**: Use your email address to manage licenses.",
                "image": null
            },
            {
                "order": 5,
                "title": "Sign In with Email",
                "description": "To manage your leased license, sign in with your email. Enter your email address and we''ll send you a one-time code to continue. Click ''CONTINUE'' after entering your email.",
                "image": null
            },
            {
                "order": 6,
                "title": "Verify Your Code",
                "description": "We''ve sent a one-time code to your email. Enter the OTP code to confirm your identity and continue to your account. Click ''VERIFY'' to complete sign-in.",
                "image": null
            },
            {
                "order": 7,
                "title": "License Claimed Successfully",
                "description": "Congratulations! You have successfully claimed this License. Click ''CONTINUE'' to proceed to your dashboard.",
                "image": null
            },
            {
                "order": 8,
                "title": "Dashboard Overview",
                "description": "Welcome to your Unetwork dashboard! Here you can see:\n\n- **Total Network Incentives**: Your cumulative earnings\n- **Earnings**: Today''s earnings and Last 7 Days summary\n- **Proof of Work Status**: Service Status, Switch Node, Validation Node connection states\n- **License Details**: Your license ID, activation status, lease duration, and required uptime (97% daily)",
                "image": null
            }
        ]
    }'::jsonb,
    '{
        "es": {
            "title": "Guía de Configuración de la App",
            "description": "Guía completa para configurar Unetwork en tu dispositivo móvil y comenzar a ganar incentivos de red.",
            "direction": "ltr"
        },
        "ar": {
            "title": "دليل إعداد التطبيق",
            "description": "دليل شامل لإعداد Unetwork على جهازك المحمول والبدء في كسب حوافز الشبكة.",
            "direction": "rtl"
        }
    }'::jsonb,
    '{"es": "partial", "ar": "partial"}'::jsonb,
    2,
    true,
    true,
    1,
    1,
    NOW()
),

-- 3. Telemetry Activation Guide
(
    'guide',
    'telemetry-activation-guide',
    'published',
    '{
        "title": "Telemetry Activation Guide",
        "description": "Learn how to enable telemetry collection on your device to contribute to network verification and earn incentives.",
        "thumbnail": "/assets/guides/telemetry.png",
        "duration_minutes": 2,
        "difficulty": "easy",
        "direction": "ltr",
        "steps": [
            {
                "order": 1,
                "title": "Open the Unetwork App",
                "description": "Launch the Unetwork app on your mobile device. You''ll see the main dashboard with your current status and available tasks.",
                "image": null
            },
            {
                "order": 2,
                "title": "Navigate to Tasks",
                "description": "Tap on the Tasks icon in the bottom navigation bar to view all available earning opportunities.",
                "image": null
            },
            {
                "order": 3,
                "title": "Telemetry is Always Active",
                "description": "Connection to Unetwork is enabled by default and keeps your device continuously connected to participate in ongoing Unetwork tasks. No additional setup required!",
                "image": null
            }
        ]
    }'::jsonb,
    '{}'::jsonb,
    '{}'::jsonb,
    3,
    false,
    true,
    1,
    1,
    NOW()
),

-- 4. Connectivity Verification Activation Guide
(
    'guide',
    'connectivity-verification-guide',
    'published',
    '{
        "title": "Connectivity Verification Activation Guide",
        "description": "Learn how to enable connectivity verification on your device to help map network coverage and earn rewards.",
        "thumbnail": "/assets/guides/scout_runner.png",
        "duration_minutes": 2,
        "difficulty": "easy",
        "direction": "ltr",
        "steps": [
            {
                "order": 1,
                "title": "Open Unetwork Tasks",
                "description": "Launch the Unetwork app and navigate to the Tasks section using the bottom navigation bar.",
                "image": null
            },
            {
                "order": 2,
                "title": "Select Connectivity Verification",
                "description": "Tap on ''Connectivity Verification'' from the list of available tasks to view details.",
                "image": null
            },
            {
                "order": 3,
                "title": "Enable the Task",
                "description": "Review the requirements and tap ''Enable'' to start connectivity verification. The task will run automatically in the background.",
                "image": null
            },
            {
                "order": 4,
                "title": "Optional: Grant Location Permission",
                "description": "For more accurate coverage mapping, grant location permission when prompted. This helps associate connectivity data with geographic areas.",
                "image": null
            }
        ]
    }'::jsonb,
    '{}'::jsonb,
    '{}'::jsonb,
    4,
    false,
    true,
    1,
    1,
    NOW()
),

-- 5. Entropy Generation Activation Guide
(
    'guide',
    'entropy-generation-guide',
    'published',
    '{
        "title": "Entropy Generation Activation Guide",
        "description": "Learn how entropy generation works and how to participate in creating verifiable randomness for blockchain and security applications.",
        "thumbnail": "/assets/guides/entropy.png",
        "duration_minutes": 3,
        "difficulty": "easy",
        "direction": "ltr",
        "steps": [
            {
                "order": 1,
                "title": "What is Entropy?",
                "description": "Entropy means unpredictability - real randomness. It powers cryptographic keys, fair lotteries and games, blockchain VRFs, and finance & security systems. It helps make security stronger and outcomes fair.",
                "image": "/assets/tasks/05_entropy_generation/1.png"
            },
            {
                "order": 2,
                "title": "What Entropy is Used For",
                "description": "Entropy powers many important things: crypto keys, lotteries & gambling, VRFs for blockchains, random selection, fintech & custody, and security & defense. Anything that needs fair, safe, and unpredictable outcomes.",
                "image": "/assets/tasks/05_entropy_generation/2.png"
            },
            {
                "order": 3,
                "title": "Why Software Randomness Isn''t Enough",
                "description": "Computers can make pseudorandom numbers, but they need real, unpredictable input to be truly random. High-assurance customers need proof: independent sources, auditability, provenance, and bias-resistance.",
                "image": "/assets/tasks/05_entropy_generation/3.png"
            },
            {
                "order": 4,
                "title": "One Device vs. Global Mesh",
                "description": "A single device provides one source, one place, and one trust point. Unetwork''s global mesh uses many phones, many places, making it harder to predict and providing stronger trust through decentralization.",
                "image": "/assets/tasks/05_entropy_generation/4.png"
            },
            {
                "order": 5,
                "title": "How Entropy Works on Unetwork",
                "description": "The process: 1) Real phones collect tiny random signals, 2) Each phone locks in its piece, 3) They are combined securely, 4) The network creates random output with proof. Transparent, verifiable, and hard to cheat.",
                "image": "/assets/tasks/05_entropy_generation/5.png"
            },
            {
                "order": 6,
                "title": "VRF & Post-Quantum Security",
                "description": "VRF (Verifiable Random Function) provides cryptographic randomness with proof. Unetwork adds a real-world entropy layer. As we move toward post-quantum security, strong entropy becomes even more important.",
                "image": "/assets/tasks/05_entropy_generation/6.png"
            },
            {
                "order": 7,
                "title": "Why Regulated Markets Care",
                "description": "Industries want proof that randomness is real, audit trails, independent sources, and assurance that no single party controlled it. Used in gambling, fintech, security, defense, and compliance. Not just random - provable.",
                "image": "/assets/tasks/05_entropy_generation/7.png"
            },
            {
                "order": 8,
                "title": "What This Looks Like For You",
                "description": "Run the app on a real phone, your phone sends small entropy samples, the network checks and combines them, you earn UP for valid contributions, and fake or bad devices are filtered out. Runs in the background - you help, you earn.",
                "image": "/assets/tasks/05_entropy_generation/8.png"
            }
        ]
    }'::jsonb,
    '{}'::jsonb,
    '{}'::jsonb,
    5,
    false,
    true,
    1,
    1,
    NOW()
),

-- 6. Incentive Withdrawal Guide via Bank
(
    'guide',
    'incentive-withdrawal-bank',
    'published',
    '{
        "title": "Incentive Withdrawal Guide via Bank",
        "description": "Learn how to withdraw your generated Network Incentives directly to your bank account using the Unetwork web management panel.",
        "thumbnail": "/assets/guides/bank.png",
        "duration_minutes": 5,
        "difficulty": "easy",
        "direction": "ltr",
        "steps": [
            {
                "order": 1,
                "title": "View Your Dashboard",
                "description": "From your Unetwork dashboard, you can see your Total Network Incentives and earnings summary. This shows your cumulative rewards, today''s earnings, and last 7 days performance. Navigate to the Withdrawals section from the sidebar menu to proceed with your withdrawal.",
                "image": null
            },
            {
                "order": 2,
                "title": "Access Withdrawals",
                "description": "In the Withdrawals section, you can view your Redeemable Incentives (available to withdraw) and Total Incentives earned. The page shows your Previous Withdrawals history with transaction details and status.\n\n**Note:** The minimum withdrawal amount is $5.00 USD (1 UP = $1.00 USD).",
                "image": null
            },
            {
                "order": 3,
                "title": "Select Bank Transfer",
                "description": "Click on the ''Bank Account Registration'' tab to set up or use your bank account for withdrawals. This option allows direct transfer to your local bank account.",
                "image": null
            },
            {
                "order": 4,
                "title": "Select Your Country",
                "description": "Select your Country of Residence from the dropdown menu. Available regions include:\n\n- **Asia**: India, Singapore, Malaysia, Thailand, Indonesia, Philippines, Vietnam\n- **Africa**: Nigeria, Kenya, South Africa, Ghana, Egypt\n- **South America**: Brazil\n\n**Note:** Changing your country will reset the form.",
                "image": null
            },
            {
                "order": 5,
                "title": "Enter Banking Details",
                "description": "Complete the Bank Account Registration form with your details:\n\n- **Full Legal Name**: Enter your name exactly as it appears on your bank account\n- **Account Currency**: Select your local currency\n- **Account Type**: Choose Personal or Business\n- **Banking Details**: Enter your bank name, account number, and routing codes",
                "image": null
            },
            {
                "order": 6,
                "title": "Complete Withdrawal",
                "description": "After registering your bank account, enter the amount you wish to withdraw and confirm the transaction. The funds will be transferred to your bank account within the processing time for your region.",
                "image": null
            }
        ]
    }'::jsonb,
    '{
        "es": {
            "title": "Guía de Retiro de Incentivos via Banco",
            "description": "Aprende como retirar tus Incentivos de Red generados directamente a tu cuenta bancaria usando el panel de gestión web de Unetwork.",
            "direction": "ltr"
        },
        "ar": {
            "title": "دليل سحب الحوافز عبر البنك",
            "description": "تعلم كيفية سحب حوافز الشبكة المكتسبة مباشرة إلى حسابك المصرفي باستخدام لوحة إدارة الويب Unetwork.",
            "direction": "rtl"
        }
    }'::jsonb,
    '{"es": "partial", "ar": "partial"}'::jsonb,
    6,
    true,
    true,
    1,
    1,
    NOW()
),

-- 7. Incentive Withdrawal Guide via Crypto (NEW)
(
    'guide',
    'incentive-withdrawal-crypto',
    'published',
    '{
        "title": "Incentive Withdrawal Guide via Crypto",
        "description": "Learn how to withdraw your generated Network Incentives as cryptocurrency to your wallet using the Unetwork web management panel.",
        "thumbnail": "/assets/guides/crypto.png",
        "duration_minutes": 5,
        "difficulty": "easy",
        "direction": "ltr",
        "steps": [
            {
                "order": 1,
                "title": "View Your Dashboard",
                "description": "From your Unetwork dashboard, you can see your Total Network Incentives and earnings summary. This shows your cumulative rewards, today''s earnings, and last 7 days performance. Navigate to the Withdrawals section from the sidebar menu to proceed with your withdrawal.",
                "image": null
            },
            {
                "order": 2,
                "title": "Access Withdrawals",
                "description": "In the Withdrawals section, you can view your Redeemable Incentives (available to withdraw) and Total Incentives earned. The page shows your Previous Withdrawals history with transaction details and status. Click the ''Withdraw Incentives'' button to start.\n\n**Note:** The minimum withdrawal amount is $5.00 USD (1 UP = $1.00 USD).",
                "image": null
            },
            {
                "order": 3,
                "title": "Enter Withdrawal Details",
                "description": "On the Claim Incentive page, fill in your withdrawal details:\n\n- **Amount**: Enter the amount you wish to redeem (or click ''Max'' for full balance)\n- **Wallet Address**: Enter your destination crypto wallet address\n- **Payout Type**: Select your preferred blockchain network (Ethereum, Binance Chain, Solana, Ripple XRP, or Cardano)\n- **Asset**: Choose the cryptocurrency asset (BNB, USDT, or USDC)\n\n**Important:** Enter a valid crypto address. Crypto sent to the wrong address cannot be recovered.",
                "image": null
            },
            {
                "order": 4,
                "title": "Review Quote",
                "description": "Review the exchange quote details:\n\n- **Quote expires in**: Time remaining to confirm (the exchange rate is locked for this duration)\n- **Amount**: The USD amount being withdrawn\n- **Fee**: Transaction fee (if any)\n- **Asset**: The cryptocurrency you selected\n- **Asset Amount**: The exact amount of crypto you will receive",
                "image": null
            },
            {
                "order": 5,
                "title": "Confirm Withdrawal",
                "description": "Click ''Confirm'' to complete your withdrawal. The transaction will be processed and sent to your wallet address. You can track the status in your Previous Withdrawals list.\n\n**Tip:** Always double-check your wallet address and network selection before confirming.",
                "image": null
            }
        ]
    }'::jsonb,
    '{
        "es": {
            "title": "Guía de Retiro de Incentivos via Cripto",
            "description": "Aprende como retirar tus Incentivos de Red generados como criptomoneda a tu billetera usando el panel de gestión web de Unetwork.",
            "direction": "ltr"
        },
        "ar": {
            "title": "دليل سحب الحوافز عبر العملات المشفرة",
            "description": "تعلم كيفية سحب حوافز الشبكة المكتسبة كعملة مشفرة إلى محفظتك باستخدام لوحة إدارة الويب Unetwork.",
            "direction": "rtl"
        }
    }'::jsonb,
    '{"es": "partial", "ar": "partial"}'::jsonb,
    7,
    true,
    true,
    1,
    1,
    NOW()
)

ON CONFLICT (content_type, slug) DO UPDATE SET
    status = EXCLUDED.status,
    content = EXCLUDED.content,
    translations = EXCLUDED.translations,
    translation_status = EXCLUDED.translation_status,
    display_order = EXCLUDED.display_order,
    is_featured = EXCLUDED.is_featured,
    is_active = EXCLUDED.is_active,
    published_version = EXCLUDED.published_version,
    published_at = EXCLUDED.published_at,
    updated_at = NOW();

-- Verify the data
SELECT 'Guides inserted/updated:' as info;
SELECT slug,
       content->>'title' as title,
       content->>'thumbnail' as thumbnail,
       display_order,
       is_featured,
       status
FROM page_contents
WHERE content_type = 'guide'
ORDER BY display_order;
