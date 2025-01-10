# Server Father Bot 🤖

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Docker Build](https://github.com/asman1337/server-father-bot/actions/workflows/docker-publish.yml/badge.svg)](https://github.com/asman1337/server-father-bot/actions/workflows/docker-publish.yml)

A Telegram bot that monitors your servers' health status, organized in groups. Get instant alerts when servers go down and check server status manually or automatically.

## Features

✅ Implemented:
- Server Management
  - Add servers with host, port, and name
  - Remove servers
  - List all servers with their status
  - Check individual server status
  - Monitor servers automatically
  - Get instant notifications when server status changes

- Group Management
  - Create server groups
  - Add servers to groups
  - Remove groups
  - List all groups
  - Check group status (all servers in a group)

- Monitoring
  - Automatic server status checking
  - Configurable check intervals
  - Real-time status notifications
  - Support for multiple chat monitoring

## Installation

### Option 1: Using Docker (Recommended) 🐳

#### A. Using Pre-built Image
1. Pull the image from GitHub Container Registry:
   ```bash
   docker pull ghcr.io/asman1337/server-father-bot:latest
   ```

2. Create a `.env` file with your configuration:
   ```env
   TELOXIDE_TOKEN=your_telegram_bot_token
   CHECK_INTERVAL=300  # Server check interval in seconds
   ```

3. Create a data directory for SQLite:
   ```bash
   mkdir data
   ```

4. Create a docker-compose.yml file:
   ```yaml
   version: '3.8'
   services:
     bot:
       image: ghcr.io/asman1337/server-father-bot:latest
       restart: unless-stopped
       env_file: .env
       volumes:
         - ./data:/usr/local/bin/data
   ```

5. Start the bot:
   ```bash
   docker compose up -d
   ```

#### B. Building Locally

#### Prerequisites
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)
- Telegram Bot Token (get it from [@BotFather](https://t.me/botfather))

#### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/asman1337/server-father-bot
   cd server-father-bot
   ```

2. Create a `.env` file with your configuration:
   ```env
   TELOXIDE_TOKEN=your_telegram_bot_token
   CHECK_INTERVAL=300  # Server check interval in seconds
   ```

3. Create a data directory for SQLite:
   ```bash
   mkdir data
   ```

4. Build and start the bot:
   ```bash
   docker compose up -d
   ```

The bot will run in a container with:
- SQLite database persisted in `./data` directory
- Automatic restarts if it crashes
- Environment variables from your `.env` file

#### Docker Commands
```bash
# View logs
docker compose logs -f

# Stop the bot
docker compose down

# Rebuild and restart
docker compose up -d --build
```

### Option 2: Manual Installation

#### Prerequisites
- Rust (latest stable version)
- SQLite
- OpenSSL development libraries (`libssl-dev` on Ubuntu/Debian, `openssl-devel` on Fedora)
- pkg-config
- Telegram Bot Token (get it from [@BotFather](https://t.me/botfather))

#### Steps
1. Install SQLite:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install sqlite3
   
   # macOS
   brew install sqlite3
   
   # Windows
   # Download from https://www.sqlite.org/download.html
   ```

2. Create the SQLite database:
   ```bash
   # Create an empty database file
   sqlite3 server_father.db ".databases"

   # Or on Windows PowerShell
   New-Item -ItemType File -Path "server_father.db"
   ```

3. Create a `.env` file in the project root with:
   ```env
   TELOXIDE_TOKEN=your_telegram_bot_token
   DATABASE_URL=sqlite:./server_father.db
   CHECK_INTERVAL=300  # Server check interval in seconds
   ```

4. Build and run:
   ```bash
   # Build the project
   cargo build --release

   # Run the bot
   cargo run --release
   ```

## Commands

- `/start` - Start the bot
- `/addserver` - Add a new server
- `/removeserver` - Remove a server
- `/status` - View all servers status
- `/check <server_id>` - Check specific server status
- `/monitor` - Start monitoring servers
- `/creategroup` - Create a new server group
- `/groups` - List all groups
- `/addtogroup` - Add server to group
- `/removegroup` - Remove a group
- `/checkgroup <group_id>` - Check group status

## Technical Details

- Built with Rust 🦀
- Uses [teloxide](https://github.com/teloxide/teloxide) for Telegram Bot API
- [SeaORM](https://github.com/SeaQL/sea-orm) for database operations
- SQLite for data storage
- Asynchronous architecture with tokio
- Clean code structure with proper error handling

## Project Structure
```
src/
├── bot/        # Bot core functionality
├── commands/   # Command handlers
├── config/     # Configuration management
├── db/         # Database models and migrations
├── error/      # Error types and handling
├── monitor/    # Server monitoring logic
└── services/   # Business logic services
```

## Database Schema
```sql
-- Servers table
CREATE TABLE server (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    host VARCHAR NOT NULL,
    port INTEGER NOT NULL,
    group_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (group_id) REFERENCES server_group(id)
);

-- Groups table
CREATE TABLE server_group (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

## Contributing

We love your input! We want to make contributing to Server Father Bot as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

### Development Process

1. Fork the repo and create your branch from `master`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code lints
6. Issue that pull request!

### Any contributions you make will be under the MIT Software License

In short, when you submit code changes, your submissions are understood to be under the same [MIT License](LICENSE) that covers the project. Feel free to contact the maintainers if that's a concern.

### Report bugs using Github's [issue tracker](https://github.com/asman1337/server-father-bot/issues)

We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/asman1337/server-father-bot/issues/new); it's that easy!

### Write bug reports with detail, background, and sample code

**Great Bug Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce
  - Be specific!
  - Give sample code if you can
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening, or stuff you tried that didn't work)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2024 Asman Mirza