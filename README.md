# mixmag_axum
small webapp to retrieve data from music website

## Getting Started

### Prerequisites
- Rust (install from https://rustup.rs/)
- MySQL/MariaDB running locally
- `.env` file with database configuration

### Setup

1. **Install dependencies:**
   ```bash
   cargo build
   ```

2. **Configure database:**
   Create a `.env` file in the project root:
   ```
   DATABASE_URL=mysql://username:password@localhost/mixmag_axum
   ```

3. **Run migrations (if applicable):**
   ```bash
   diesel migration run
   ```

4. **Start the server:**
   ```bash
   cargo run
   ```

The server will start at `http://0.0.0.0:3000`

### API Endpoints

**Important:** Do not use trailing slashes in URLs. Axum requires exact path matching.

- `http://localhost:3000/api` - API root
- `http://localhost:3000/api/health` - Health check
- `http://localhost:3000/api/articles` - Articles list
- `http://localhost:3000/api/news` - News list
- `http://localhost:3000/api/cd_reviews` - CD Reviews list

**Note:** URLs with trailing slashes (e.g., `/api/health/`) will return 404. Always use paths without trailing slashes.

### Testing

```bash
curl http://localhost:3000/api
curl http://localhost:3000/api/health
curl http://localhost:3000/api/cd_reviews
```

### Development

For automatic server restart on code changes:
```bash
cargo install cargo-watch
cargo watch -x run
```

### Deployment

Build Docker image:
```bash
./deploy.sh
```
