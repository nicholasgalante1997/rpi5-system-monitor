pub struct Html {
    content: String,
}

impl Html {
    pub fn new(content: String) -> Self {
        Self { content }
    }


    pub fn into_page(&self) -> String {
        let page = format!(
            r#"
            <!doctype html>
            <html>
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Rs-pberry Pi System Monitor</title>
                    {}
                </head>
                <body>
                    <div class="container">
                        <div class="hero">
                            <img src="https://cdn.iconscout.com/icon/free/png-256/free-raspberry-pi-3-569254.png" alt="Raspberry Pi Logo">
                            <h1>System Monitor</h1>
                            <p class="subtitle">Real-time hardware information dashboard</p>
                        </div>
                        {}
                    </div>
                </body>
                <script async="async" type="module">
                    setTimeout(() => window.reload(), 1000 * 60 * 60);
                </script>
                <script src="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.4/js/all.min.js"></script>
            </html>
            "#,
            Html::get_page_styles(),
            self.content
        );

        page
    }

    fn get_page_styles() -> String {
        r#"
        <style>
            :root {
                --primary-gradient: linear-gradient(135deg, #ff416c, #ff4b2b);
                --secondary-gradient: linear-gradient(135deg, #654ea3, #eaafc8);
                --glass-bg: rgba(255, 255, 255, 0.15);
                --glass-border: rgba(255, 255, 255, 0.18);
                --text-color: #f8f9fa;
                --muted-text: rgba(248, 249, 250, 0.7);
                --card-radius: 16px;
                --shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.2);
            }

            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
                font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            }

            body {
                background: linear-gradient(135deg, #121212, #2d3436);
                color: var(--text-color);
                min-height: 100vh;
                line-height: 1.6;
                padding: 20px;
            }

            .container {
                max-width: 700px;
                margin: 0 auto;
            }

            .hero {
                display: flex;
                flex-direction: column;
                align-items: center;
                text-align: center;
                margin-bottom: 30px;
                padding: 20px;
                position: relative;
            }

            .hero img {
                width: 120px;
                height: auto;
                margin-bottom: 15px;
                filter: drop-shadow(0 5px 10px rgba(0, 0, 0, 0.3));
            }

            h1 {
                font-size: 36px;
                font-weight: 800;
                margin-bottom: 10px;
                background: var(--primary-gradient);
                -webkit-background-clip: text;
                background-clip: text;
                -webkit-text-fill-color: transparent;
            }

            .subtitle {
                font-size: 16px;
                color: var(--muted-text);
                margin-bottom: 15px;
            }

            .refresh-time {
                font-size: 12px;
                color: var(--muted-text);
                margin-top: 15px;
            }

            .cards-container {
                display: grid;
                grid-template-columns: 1fr;
                gap: 20px;
            }

            .card {
                background: var(--glass-bg);
                backdrop-filter: blur(12px);
                -webkit-backdrop-filter: blur(12px);
                border-radius: var(--card-radius);
                padding: 25px;
                box-shadow: var(--shadow);
                border: 1px solid var(--glass-border);
                overflow: hidden;
                position: relative;
                transition: transform 0.3s ease;
            }

            .card:hover {
                transform: translateY(-5px);
            }

            .card::before {
                content: '';
                position: absolute;
                top: 0;
                left: 0;
                right: 0;
                height: 4px;
                background: var(--secondary-gradient);
                border-radius: var(--card-radius) var(--card-radius) 0 0;
            }

            .card-title {
                display: flex;
                align-items: center;
                margin-bottom: 15px;
                font-weight: 600;
            }

            .card-title i {
                margin-right: 10px;
                font-size: 18px;
                background: var(--primary-gradient);
                -webkit-background-clip: text;
                background-clip: text;
                -webkit-text-fill-color: transparent;
            }

            .info-row {
                display: flex;
                justify-content: space-between;
                padding: 8px 0;
                border-bottom: 1px solid rgba(255, 255, 255, 0.08);
            }

            .info-row:last-child {
                border-bottom: none;
            }

            .info-label {
                font-size: 14px;
                color: var(--muted-text);
            }

            .info-value {
                font-size: 14px;
                font-weight: 500;
            }

            .progress-container {
                margin-top: 10px;
                margin-bottom: 5px;
                height: 8px;
                background: rgba(255, 255, 255, 0.1);
                border-radius: 4px;
                overflow: hidden;
            }

            .progress-bar {
                height: 100%;
                background: var(--primary-gradient);
                width: 75%;
                border-radius: 4px;
            }

            .temperature-indicator {
                display: flex;
                align-items: center;
                justify-content: space-between;
            }

            .temperature-value {
                font-weight: 600;
            }

            .normal {
                color: #4cd964;
            }

            .warning {
                color: #ffcc00;
            }

            .danger {
                color: #ff3b30;
            }

            .footer {
                text-align: center;
                margin-top: 40px;
                color: var(--muted-text);
                font-size: 12px;
            }

            @media (max-width: 540px) {
                .card {
                    padding: 20px;
                }
                h1 {
                    font-size: 30px;
                }
            }

            /* Custom animation for the refresh label */
            @keyframes pulse {
                0% { opacity: 0.6; }
                50% { opacity: 1; }
                100% { opacity: 0.6; }
            }

            .pulse {
                animation: pulse 2s infinite ease-in-out;
            }
        </style>
        "#.to_string()
    }
}