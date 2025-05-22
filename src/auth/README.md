# Authentication API Documentation

## API Endpoints Overview

### Authentication Routes (`/auth/`)
- `POST /auth/register` - User registration
- `POST /auth/login` - User login
- `POST /auth/oauth` - OAuth login

### User Management Routes (`/user/`)
- `GET /user/profile` - Get user profile (requires authentication)
- `PUT /user/update_avatar` - Update user avatar (requires authentication)

## Authentication Flow Testing

### 1. User Registration
**Request:**
```bash
curl -X POST http://localhost:8080/auth/register -H "Content-Type: application/json" -d "{\"email\": \"harrisontest3@example.com\", \"username\": \"harrison3\", \"password\": \"password123\"}"
```

**Response:**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2MTM4YjQwNC0wNzRjLTQ2YzktOWUwNC1lYjJmODBlMzE0NmIiLCJleHAiOjE3NDg1MjkwMTQsImlhdCI6MTc0NzkyNDIxNH0.1O_jarmimg-EK-i4jDJupQd7lLiyS1I7qutUf-d1Nic",
  "user": {
    "id": "6138b404-074c-46c9-9e04-eb2f80e3146b",
    "email": "harrisontest3@example.com",
    "username": "harrison3",
    "avatar": "/default-avatar.png"
  }
}
```

### 2. Duplicate Registration (Error Case)
**Request:**
```bash
curl -X POST http://localhost:8080/auth/register -H "Content-Type: application/json" -d "{\"email\": \"harrisontest3@example.com\", \"username\": \"harrison4\", \"password\": \"anotherpassword\"}"
```

**Response:**
```json
{
  "error": "验证错误: 邮箱已注册"
}
```

### 3. User Login (Correct Password)
**Request:**
```bash
curl -X POST http://localhost:8080/auth/login -H "Content-Type: application/json" -d "{\"email\": \"harrisontest3@example.com\", \"password\": \"password123\"}"
```

**Response:**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2MTM4YjQwNC0wNzRjLTQ2YzktOWUwNC1lYjJmODBlMzE0NmIiLCJleHAiOjE3NDg1MjkwMjUsImlhdCI6MTc0NzkyNDIyNX0.ogxKiGSElHXcjkzd51Ek5cvFknwryPbcZGHKebfCqfY",
  "user": {
    "id": "6138b404-074c-46c9-9e04-eb2f80e3146b",
    "email": "harrisontest3@example.com",
    "username": "harrison3",
    "avatar": "/default-avatar.png"
  }
}
```

### 4. User Login (Wrong Password)
**Request:**
```bash
curl -X POST http://localhost:8080/auth/login -H "Content-Type: application/json" -d "{\"email\": \"harrisontest3@example.com\", \"password\": \"wrongpassword\"}"
```

**Response:**
```json
{
  "error": "认证失败: 用户名或密码错误"
}
```

### 5. Get User Profile (Authenticated)
**Request:**
```bash
curl -X GET http://localhost:8080/user/profile -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2MTM4YjQwNC0wNzRjLTQ2YzktOWUwNC1lYjJmODBlMzE0NmIiLCJleHAiOjE3NDg1MjkwMjUsImlhdCI6MTc0NzkyNDIyNX0.ogxKiGSElHXcjkzd51Ek5cvFknwryPbcZGHKebfCqfY"
```

**Response:**
```json
{
  "id": "6138b404-074c-46c9-9e04-eb2f80e3146b",
  "email": "harrisontest3@example.com",
  "username": "harrison3",
  "avatar": "/default-avatar.png"
}
```

### 6. Update User Avatar
**Request:**
```bash
curl -X PUT http://localhost:8080/user/update_avatar -H "Content-Type: application/json" -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2MTM4YjQwNC0wNzRjLTQ2YzktOWUwNC1lYjJmODBlMzE0NmIiLCJleHAiOjE3NDg1MjkwMjUsImlhdCI6MTc0NzkyNDIyNX0.ogxKiGSElHXcjkzd51Ek5cvFknwryPbcZGHKebfCqfY" -d "\"/uploads/new-avatar.png\""
```

**Response:**
```json
{
  "avatar": "/uploads/new-avatar.png"
}
```

### 7. Unauthenticated Access (Error Case)
**Request:**
```bash
curl -X GET http://localhost:8080/user/profile
```

**Response:**
```json
{
  "error": "认证失败: 缺少认证Token"
}
```

### 8. Password Length Validation (Error Case)
**Request:**
```bash
curl -X POST http://localhost:8080/auth/register -H "Content-Type: application/json" -d "{\"email\": \"short@example.com\", \"username\": \"shortuser\", \"password\": \"123\"}"
```

**Response:**
```json
{
  "error": "验证错误: 密码长度必须至少为8位"
}
```

## Request/Response Format

### Registration Request Fields
- `email` (string, required): User email address
- `username` (string, required): Unique username  
- `password` (string, required): Password (minimum 8 characters)

### Login Request Fields
- `email` (string, required): User email address
- `password` (string, required): User password

### Successful Response Format
- `token` (string): JWT authentication token
- `user` (object): User information
  - `id` (UUID): User unique identifier
  - `email` (string): User email
  - `username` (string): User username
  - `avatar` (string): Avatar URL path

### Error Response Format
- `error` (string): Error message describing what went wrong

## Authentication Notes

1. **JWT Token**: Valid for 7 days (604800 seconds)
2. **Authorization Header**: Use `Authorization: Bearer <token>` for authenticated requests
3. **Password Requirements**: Minimum 8 characters
4. **Email Validation**: Must be valid email format
5. **Username Uniqueness**: Usernames must be unique across the system