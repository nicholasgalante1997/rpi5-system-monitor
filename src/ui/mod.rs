use crate::models::data_objects::disk_info::DiskReportInfo;
use crate::models::data_objects::system_info::SystemReportInfo;
use crate::utils;

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

pub struct HttpViews;

impl HttpViews {
    pub fn get_overview_view(
        system_info: &SystemReportInfo,
        disks_info: &Vec<DiskReportInfo>,
    ) -> String {
        format!(
            r#"
            <div id="overview" class="tab-content active">
                <div class="section-header">
                    <h2 class="section-title">System Overview</h2>
                </div>

                <div class="card-grid">
                    <div class="card">
                        <div class="card-header">
                            <div class="card-title">
                                <div class="card-icon"></div>
                                CPU Usage
                            </div>
                        </div>
                        <div class="card-value" id="cpu-usage">{:.2}%</div>
                        <div class="progress-container">
                            <div class="progress-bar">
                                <div class="progress-fill" id="cpu-progress" style="width: {:.2}%"></div>
                            </div>
                            <div class="progress-stats">
                                <span>0%</span>
                                <span>100%</span>
                            </div>
                        </div>
                        <div class="card-description">Current CPU utilization across all cores</div>
                    </div>

                    <div class="card">
                        <div class="card-header">
                            <div class="card-title">
                                <div class="card-icon"></div>
                                Memory Usage
                            </div>
                        </div>
                        <div class="card-value" id="memory-usage">{:.2} GB</div>
                        <div class="progress-container">
                            <div class="progress-bar">
                                <div class="progress-fill" id="memory-progress" style="width: {}%"></div>
                            </div>
                            <div class="progress-stats">
                                <span>0 GB</span>
                                <span id="total-memory">{} GB</span>
                            </div>
                        </div>
                        <div class="card-description">Physical memory currently in use</div>
                    </div>

                    <div class="card">
                        <div class="card-header">
                            <div class="card-title">
                                <div class="card-icon"></div>
                                Disk Usage
                            </div>
                        </div>
                        <div class="card-value" id="disk-usage">{:.2} GB</div>
                        <div class="progress-container">
                            <div class="progress-bar">
                                <div class="progress-fill" id="disk-progress" style="width: {:.1}%"></div>
                            </div>
                            <div class="progress-stats">
                                <span>0 GB</span>
                                <span id="total-disk">{:.2} GB</span>
                            </div>
                        </div>
                        <div class="card-description">Primary storage utilization</div>
                    </div>
                </div>

                <div class="card">
                    <div class="card-header">
                        <div class="card-title">
                            <div class="card-icon"></div>
                            System Information
                        </div>
                    </div>
                    <div>
                        <div class="detail-row">
                            <div class="detail-label">Hostname</div>
                            <div class="detail-value" id="hostname">{}</div>
                        </div>
                        <div class="detail-row">
                            <div class="detail-label">Kernel Version</div>
                            <div class="detail-value" id="kernel-version">{}</div>
                        </div>
                        <div class="detail-row">
                            <div class="detail-label">Uptime</div>
                            <div class="detail-value" id="uptime">{}</div>
                        </div>
                        <div class="detail-row">
                            <div class="detail-label">OS</div>
                            <div class="detail-value" id="os-info">{}</div>
                        </div>
                    </div>
                </div>
            </div>
            "#,
            system_info.total_cpu_usage,
            system_info.total_cpu_usage,
            utils::convert_bytes_to_gbs(system_info.used_memory),
            utils::convert_to_percent(system_info.used_memory, system_info.total_memory),
            utils::convert_bytes_to_gbs(system_info.total_memory),
            utils::convert_bytes_to_gbs(utils::get_total_disk_usage_across_all_disks(disks_info)),
            utils::convert_to_percent(
                utils::get_total_disk_usage_across_all_disks(disks_info),
                utils::get_total_disk_space_across_all_disks(disks_info)
            ),
            utils::convert_bytes_to_gbs(utils::get_total_disk_space_across_all_disks(disks_info)),
            system_info.system_host_name,
            system_info.system_kernal_version,
            utils::format_uptime(system_info.uptime_in_seconds),
            system_info.system_os_version,
        )
    }
}
