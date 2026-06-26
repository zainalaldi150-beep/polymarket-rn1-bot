# Polymarket RN2 Trading Bot v2.1

## Deskripsi

Polymarket RN2 Bot adalah **versi improved** dari RN1 Bot dengan performa dan stabilitas yang lebih baik. Bot ini dirancang khusus untuk trading di platform Polymarket dengan multiple strategies canggih.

## Fitur Utama

- ✅ **6 Trading Strategies**
  - Mispricing Detection (Arbitrage & EV)
  - Market Making (Liquidity Provider)
  - HFT (High Frequency Trading)
  - Hedging (Risk Management)
  - Hold/Zombie (Hold to Settlement)
  - Farming (Accumulation Strategy)

- ✅ **Multi Data Source**
  - Polymarket CLOB API
  - Polymarket Gamma API
  - Sportmonks API (Optional)
  - SportsDataIO API (Optional)
  - Sportradar API (Optional)
  - PMXT WebSocket Pool (5 connections)

- ✅ **Trading Modes**
  - Paper Trading (Simulasi tanpa risiko)
  - Live Trading (Dengan API key asli)
  - Backtesting (Dengan data historis)

- ✅ **AI Integration**
  - GLM-5.2 untuk prediksi probabilitas
  - Analisis pasar real-time

- ✅ **Monitoring & Logging**
  - File-based logging (CSV)
  - Real-time price updates via WebSocket
  - Trade history tracking

## Persyaratan Sistem

- **OS**: Linux (Rekomendasi: Ubuntu 20.04/22.04)
- **Rust**: 1.70+
- **RAM**: 2GB minimum (4GB rekomendasi)
- **CPU**: 2 cores minimum
- **Disk**: 10GB SSD
- **Koneksi**: Internet stabil (low latency rekomendasi)

## Quick Start

### 1. Clone Repository
```bash
git clone https://github.com/zainalaldi150-beep/polymarket-rn2-bot.git
cd polymarket-rn2-bot
```

### 2. Build Project
```bash
# Development build (untuk testing)
cargo build

# Release build (untuk production - REKOMENDASI)
cargo build --release
```

Binary akan berada di `target/release/polymarket-rn2-bot` (ukuran ~8.5MB)

### 3. Setup Environment

Buat file `.env`:
```bash
# Polymarket API (WAJIB untuk live trading)
POLYMARKET_API_KEY=your_api_key_here
POLYMARKET_API_SECRET=your_api_secret_here
POLYMARKET_API_PASSPHRASE=your_passphrase_here
POLYMARKET_PRIVATE_KEY=your_private_key_here

# AI GLM-5.2 (OPSIONAL)
GLM_API_KEY=your_glm_api_key
GLM_BASE_URL=https://api.z.ai/api/v1
GLM_MODEL=glm-5.2

# Google Sheets Logging (OPSIONAL)
GOOGLE_CREDENTIALS=path/to/credentials.json
GOOGLE_SHEET_ID=your_sheet_id

# Sports Data APIs (OPSIONAL)
SPORTMONKS_API_TOKEN=your_token
SPORTSDATAIO_API_KEY=your_key
SPORTRADAR_API_KEY=your_key
```

### 4. Konfigurasi

Edit `config.json` sesuai kebutuhan:
```json
{
  "polymarket": {
    "clob_api_url": "https://clob.polymarket.com/api",
    "gamma_api_url": "https://gamma-api.polymarket.com",
    "ws_url": "wss://ws-subscriptions-clob.polymarket.com/ws/market"
  },
  "trading": {
    "max_markets": 10,
    "scan_interval_ms": 1000,
    "min_liquidity": 1000,
    "mispricing_low_threshold": 0.05,
    "mispricing_high_threshold": 0.15,
    "min_ev_threshold": 0.5,
    "maker_spread": 0.02,
    "max_orders_per_market": 5,
    "order_size": 10,
    "hold_to_settlement": true,
    "max_active_positions": 20,
    "paper_mode": true,
    "paper_balance": 10000
  }
}
```

## Usage

### Paper Trading (Testing)
```bash
# Jalankan dengan paper mode
./target/release/polymarket-rn2-bot --paper --config config.json

# Output:
# Starting RN1 Trading Engine v2.0
# PAPER MODE ACTIVATED
# Starting Paper Balance: $10000.00
# Strategies: Mispricing | MarketMaking | HFT | Hedging | Hold | Farming
```

### Live Trading
```bash
# Pastikan API key sudah di-set di .env
./target/release/polymarket-rn2-bot --config config.json

# Output:
# LIVE MODE ACTIVATED
# Connecting to WebSocket: wss://ws-subscriptions-clob.polymarket.com/ws/market
# PMXT WebSocket Pool with 5 connections created
```

### Backtesting
```bash
# Siapkan file CSV dengan data historis
./target/release/polymarket-rn2-bot --backtest --config config.json --backtest-file data/markets.csv
```

### CLI Options
```
Usage: polymarket-rn2-bot [OPTIONS]

Options:
  -p, --config <FILE>        Path ke file konfigurasi [default: config.json]
      --paper                Aktifkan paper trading mode
      --backtest             Jalankan mode backtest
      --backtest-file <FILE> Path ke file data historis [default: data/historical.csv]
  -h, --help                 Tampilkan help
  -V, --version              Tampilkan version
```

## Deploy ke VPS

### Metode 1: Manual Deploy

```bash
# Build di local
cargo build --release

# Upload ke VPS
scp target/release/polymarket-rn2-bot user@your-vps:/opt/bot/
scp config.json user@your-vps:/opt/bot/
scp .env user@your-vps:/opt/bot/

# Di VPS
cd /opt/bot
chmod +x polymarket-rn2-bot
./polymarket-rn2-bot --paper --config config.json
```

### Metode 2: Systemd Service (Rekomendasi)

1. Upload semua file ke `/opt/polymarket-bot/`

2. Buat service file:
```bash
sudo nano /etc/systemd/system/polymarket-bot.service
```

3. Isi dengan:
```ini
[Unit]
Description=Polymarket RN2 Trading Bot
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
User=your_user
WorkingDirectory=/opt/polymarket-bot
EnvironmentFile=/opt/polymarket-bot/.env
ExecStart=/opt/polymarket-bot/polymarket-rn2-bot --config /opt/polymarket-bot/config.json
Restart=always
RestartSec=30
StandardOutput=syslog
StandardError=syslog

[Install]
WantedBy=multi-user.target
```

4. Aktifkan service:
```bash
sudo systemctl daemon-reload
sudo systemctl enable polymarket-bot
sudo systemctl start polymarket-bot
sudo systemctl status polymarket-bot
```

5. Cek logs:
```bash
journalctl -u polymarket-bot -f
```

### Metode 3: Docker (Coming Soon)

## Konfigurasi Strategi

### Mispricing Strategy
Mendeteksi peluang arbitrase dan expected value:
- `mispricing_low_threshold`: 0.05 (5%)
- `mispricing_high_threshold`: 0.15 (15%)
- `min_ev_threshold`: 0.5 (50%)

### Market Making Strategy
Menyediakan likuiditas:
- `maker_spread`: 0.02 (2%)
- `order_size`: 10
- `max_orders_per_market`: 5

### HFT Strategy
High Frequency Trading:
- `arbitrage_threshold`: 0.01 (1%)
- `latency_ms`: 10
- `max_arb_size`: 50

### Hedging Strategy
Manajemen risiko:
- `max_correlation`: 0.8
- `hedge_ratio`: 0.5

### Zombie/Hold Strategy
Hold hingga settlement:
- `profit_take_threshold`: 0.1 (10%)
- `ignore_losses`: true

### Farming Strategy
Akumulasi posisi:
- `min_price`: 0.1
- `max_price`: 0.9
- `farm_size`: 100
- `expiry_window_minutes`: 60

## Monitoring & Logging

### Log Files
- Trade logs: `logs/trades_*.csv`
- Format: `timestamp,market_id,side,price,size,status`

### Real-time Monitoring
```bash
# Lihat logs systemd
tail -f /var/log/syslog | grep polymarket

# Atau
journalctl -u polymarket-bot -f
```

## Troubleshooting

### Error: "POLY_API_KEY not set"
**Solusi**: Pastikan environment variables sudah di-set di file `.env`

### Error: "Connection refused"
**Solusi**: 
- Periksa koneksi internet
- Pastikan Polymarket API accessible
- Coba `curl https://clob.polymarket.com/api`

### Error: "No markets found"
**Solusi**: 
- Periksa `min_liquidity` di config
- Pastikan ada pasar aktif di Polymarket
- Coba kurangi `min_liquidity`

### Error: "Insufficient balance"
**Solusi**: 
- Tambah `paper_balance` untuk paper mode
- Deposit USDC ke akun Polymarket untuk live mode

### Bot tidak menjalankan trade
**Solusi**:
- Aktifkan strategi di config
- Periksa threshold values
- Cek logs untuk detail

## Performance Optimization

### VPS Requirements
- **Minimal**: 2 vCPU, 2GB RAM, 10GB SSD
- **Rekomendasi**: 4 vCPU, 4GB RAM, 20GB SSD
- **High Frequency**: 8+ vCPU, 8GB+ RAM, NVMe SSD

### Latency
- Gunakan VPS di region yang dekat dengan Polymarket server
- Rekomendasi: AWS US-East, DigitalOcean NYC, Linode Newark
- Test latency: `ping clob.polymarket.com`

### Connection Pool
- PMXT WebSocket Pool: 5 connections (sudah optimal)
- Untuk HFT: pertimbangkan menambah connection

## Security

### API Keys
- JANGAN PERNAH commit API keys ke repository
- Gunakan environment variables
- Rotate API keys secara berkala

### Firewall
- Buka port 443 (HTTPS)
- Buka port 80 (HTTP redirect)
- Tutup semua port lain yang tidak diperlukan

### SSH
- Gunakan SSH key authentication
- Disable password authentication
- Ubah default SSH port (22 -> 2222)

## Update

```bash
# Pull latest changes
git pull origin main

# Rebuild
cargo build --release

# Restart service
sudo systemctl restart polymarket-bot
```

## Kontribusi

1. Fork repository
2. Buat branch (`git checkout -b feature/nama-fitur`)
3. Commit (`git commit -m 'Add fitur baru'`)
4. Push (`git push origin feature/nama-fitur`)
5. Buat Pull Request

## Lisensi

MIT License - Copyright (c) 2024 zainalaldi150-beep

## Kontak

- **GitHub**: [zainalaldi150-beep](https://github.com/zainalaldi150-beep)
- **Repository**: [polymarket-rn2-bot](https://github.com/zainalaldi150-beep/polymarket-rn2-bot)

---

## Peringatan ⚠️

> **DISCLAIMER**: Bot ini untuk tujuan edukasi dan penelitian. Trading cryptocurrency dan prediction markets memiliki risiko tinggi. Developer tidak bertanggung jawab atas kerugian apapun. Gunakan dengan risiko Anda sendiri.

> **NOT FINANCIAL ADVICE**: Ini bukan saran keuangan. Lakukan riset Anda sendiri sebelum trading.

> **TEST THOROUGHLY**: Selalu test dengan paper trading sebelum menggunakan live mode.

---

**Happy Trading!** 🚀
