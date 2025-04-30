# rs-auth-ai
A Rust-based template project by [Erio-Harrison](https://github.com/Erio-Harrison) for building applications with user authentication and AI-powered features. It offers a modular, extensible architecture with JWT-based authentication, OAuth (Google and Facebook), and AI integration (currently supporting Tongyi Qianwen, with easy expansion to other providers). The project uses MongoDB for data storage and Redis for caching/session management, designed for rapid development and scalability.

## API Documentation
Detailed API documentation is available in:
- [`src/auth/README.md`](./src/auth/README.md): Covers authentication endpoints (register, login, profile, OAuth).
- [`src/ai/README.md`](./src/ai/README.md): Covers AI endpoints (text and image analysis).

## Project Structure
```
rs-auth-ai/
├── src/
│   ├── main.rs                # Application entry point
│   ├── errors.rs              # Error handling
│   ├── config.rs              # Configuration management
│   ├── db.rs                  # MongoDB connection and operations
│   ├── models/                # Data models
│   │   ├── user.rs           # User and OAuth models
│   │   └── ai.rs             # AI request/response models
│   ├── service/               # Core services
│   │   ├── mod.rs            # Service module entry
│   │   └── redis_service.rs  # Redis service
│   ├── auth/                  # Authentication module
│   │   ├── utils.rs          # JWT utilities
│   │   ├── routes.rs         # Authentication routes
│   │   ├── handlers.rs       # Authentication logic
│   │   ├── oauth/            # OAuth providers
│   │   │   ├── models.rs     # OAuth data models
│   │   │   ├── google.rs     # Google OAuth
│   │   │   └── facebook.rs   # Facebook OAuth
│   │   └── README.md         # Authentication API docs
│   └── ai/                    # AI module
│       ├── mod.rs            # AI module entry
│       ├── routes.rs         # AI routes
│       ├── handlers.rs       # AI request handlers
│       ├── service.rs        # AI service logic
│       ├── providers/        # AI provider implementations
│       │   ├── mod.rs        # Provider module entry
│       │   └── tongyi.rs     # Tongyi Qianwen provider
│       └── README.md         # AI API docs
└── Cargo.toml                # Project dependencies
```

## Features

### Authentication:
- JWT-based registration and login with Argon2 password hashing.
- OAuth 2.0 for Google and Facebook, extensible to other providers.
- User profile management with avatar support.

### AI Integration:
- Text and image analysis via Tongyi Qianwen.
- Modular provider interface for adding AI services (e.g., OpenAI, Claude).

### Database and Caching:
- MongoDB for flexible storage of user and AI data.
- Redis for efficient caching and session management.

### Configuration:
- Environment-based setup via .env files.

### Error Handling:
- Custom error types for consistent API responses.

## Getting Started

### Prerequisites
- Rust (stable, latest version)
- Cargo
- MongoDB
- Redis
- API keys for Google and Facebook OAuth (optional)
- Tongyi Qianwen API key (for AI features)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/Erio-Harrison/rs-auth-ai.git
cd rs-auth-ai
```

2. Configure environment variables in a .env file:
```env
DATABASE_URL=Your DATABASE_URL
REDIS_URL=redis://localhost:6379
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
JWT_SECRET=your_strong_secret_key
JWT_EXPIRATION=604800
GOOGLE_CLIENT_ID=your_google_client_id
GOOGLE_CLIENT_SECRET=your_google_client_secret
FACEBOOK_APP_ID=your_facebook_app_id
FACEBOOK_APP_SECRET=your_facebook_app_secret
AI_TONGYI_API_KEY=your_tongyi_api_key
```

3. Install dependencies:
```bash
cargo build
```

4. Run the application:
```bash
cargo run
```

## Design Highlights

### Extensible AI Providers:
- Uses a Provider trait for seamless integration of new AI services.
- Supports text and image inputs, with Tongyi Qianwen as the default provider.

### Robust Authentication:
- Secure JWT with configurable expiration (default: 1 week).
- OAuth support for third-party logins, with MongoDB storage for user data.

### Data Management:
- MongoDB stores user profiles (with avatars, OAuth providers) and AI results.
- Custom serialization for MongoDB datetime fields.

### Performance:
- Actix-Web for high-performance, asynchronous request handling.
- Redis for fast caching and session management.

## Extending the Project

### Adding AI Providers
1. Create a new provider in `src/ai/providers/` (e.g., openai.rs).
2. Implement the Provider trait for text/image processing and API calls.
3. Update `src/ai/service.rs` to select the provider based on configuration.

### Adding OAuth Providers
1. Add a new file in `src/auth/oauth/` (e.g., twitter.rs).
2. Implement the OAuth flow, similar to google.rs or facebook.rs.
3. Update `src/auth/oauth/models.rs` and `src/auth/routes.rs`.

### Custom Models
1. Define new models in `src/models/`.
2. Update `src/db.rs` for MongoDB operations.

## Dependencies
Key dependencies (see Cargo.toml for details):
- actix-web: Web framework
- mongodb: MongoDB driver
- redis: Redis client
- jsonwebtoken: JWT handling
- argon2: Password hashing
- reqwest: HTTP client for AI APIs
- serde: Data serialization

## Contributing
1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Commit changes (`git commit -m 'Add your feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a pull request.

## License
MIT License. See LICENSE file for details.

## Contact
For issues or feedback, open an issue on GitHub or contact [Erio-Harrison](https://github.com/Erio-Harrison).