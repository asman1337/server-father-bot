# Server Father

A Telegram bot written in Rust that watches over your servers like a caring father. Get instant alerts when servers go down and organize them in groups for easier management.

## Core Features

### Server Management
- Create server groups (e.g., "Production", "Development", "Databases")
- Add/remove servers to groups
- View all servers and their groups

### Health Monitoring
- Basic health check:
  - Server up/down status (ping/TCP)

### Monitoring Methods
1. Manual Checks
   - Check single server status
   - Check entire group status
   - Refresh button for quick rechecks

2. Automated Monitoring
   - Periodic checks (configurable interval)
   - Instant alerts on server down
   - Group-based notifications

### Bot Commands
- `/start` - Start the bot
- `/addserver` - Add new server
- `/removeserver` - Remove server
- `/creategroup` - Create server group
- `/check` - Manual status check
- `/setinterval` - Set check interval
- `/status` - View all servers status
- `/checkserver <server>` - Detailed check of specific server
- `/configure <server>` - Configure server-specific settings

## Tech Stack
- Rust
- teloxide (Telegram Bot Framework)
- Sea-ORM (Database ORM)
  - Support for SQLite, PostgreSQL, MySQL
  - Easy database migrations
  - Async query support
- tokio (async runtime)

## Roadmap
### Phase 1 (Current) - Server Availability [🚧 In Progress]
- [ ] Basic server up/down monitoring
- [ ] Group management
  - [ ] Create/delete groups
  - [ ] Add/remove servers
  - [ ] List groups and servers
- [ ] Simple alerts
  - [ ] Down alerts
  - [ ] Recovery notifications
- [ ] Basic UI/Commands
  - [ ] Server management commands
  - [ ] Manual check functionality
  - [ ] Periodic check setup

### Phase 2 - Enhanced Monitoring [📅 Planned]
- [ ] Server-specific monitoring
  - [ ] CPU usage
  - [ ] Memory usage
  - [ ] Disk space
- [ ] Custom ports checking
- [ ] SSH integration

### Phase 3 - Advanced Features [📅 Planned]
- [ ] Detailed server metrics
- [ ] Service status monitoring
- [ ] Process monitoring
- [ ] Network connection status
- [ ] Custom alert thresholds
- [ ] Per-server monitoring intervals

## Progress Legend
- 🚧 In Progress
- ✅ Completed
- 📅 Planned
- ⏸️ On Hold

## Development
Requires Rust 1.84+ and Cargo