# Server Father Bot 🤖

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

## Setup

### Prerequisites
- Rust (latest stable version)
- SQLite
- Telegram Bot Token (get it from [@BotFather](https://t.me/botfather))

### Environment Variables
Create a `.env` file in the project root with:
```env
TELOXIDE_TOKEN=your_telegram_bot_token
DATABASE_URL=sqlite:./server_father.db
CHECK_INTERVAL=300  # Server check interval in seconds
```

### Build and Run
```bash
# Clone the repository
git clone https://github.com/asman1337/server-father-bot
cd server-father-bot

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

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.