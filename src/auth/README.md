## 认证流程完整测试

### 1. 用户注册
请求：
```cmd
curl -X POST http://localhost:8080/auth/register -H "Content-Type: application/json" -d "{\"email\": \"harrisontest2@example.com\", \"password\": \"password123\"}"
```
响应：
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "user": {
    "id": "680dc12d18851f4183ecd1e0",
    "email": "harrisontest2@example.com",
    "avatar": "/default-avatar.png"
  }
}
```

### 2. 重复注册（失败测试）
请求：
```cmd
curl -X POST http://localhost:8080/auth/register -H "Content-Type: application/json" -d "{\"email\": \"harrisontest2@example.com\", \"password\": \"anotherpassword\"}"
```
响应：
```json
{
  "error": "验证错误: 邮箱已注册"
}
```

### 3. 用户登录（正确密码）
请求：
```cmd
curl -X POST http://localhost:8080/auth/login -H "Content-Type: application/json" -d "{\"email\": \"harrisontest2@example.com\", \"password\": \"password123\"}"
```
响应：
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "user": {
    "id": "680dc12d18851f4183ecd1e0",
    "email": "harrisontest2@example.com",
    "avatar": "/default-avatar.png"
  }
}
```

### 4. 用户登录（错误密码）
请求：
```cmd
curl -X POST http://localhost:8080/auth/login -H "Content-Type: application/json" -d "{\"email\": \"harrisontest2@example.com\", \"password\": \"wrongpassword\"}"
```
响应：
```json
{
  "error": "认证失败: 用户名或密码错误"
}
```

### 5. 获取用户资料（需要认证）
请求：
```cmd
curl -X GET http://localhost:8080/user/profile -H "Authorization: Bearer YOUR_TOKEN"
```
响应：
```json
{
  "id": "680dc12d18851f4183ecd1e0",
  "email": "harrisontest2@example.com",
  "avatar": "/default-avatar.png"
}
```

### 6. 更新用户头像
请求：
```cmd
curl -X PUT http://localhost:8080/user/update_avatar -H "Content-Type: application/json" -H "Authorization: Bearer YOUR_TOKEN" -d "\"/uploads/new-avatar.png\""
```
预期响应：
```json
{
  "avatar": "/uploads/new-avatar.png"
}
```

### 7. 未认证访问资料（失败测试）
请求：
```cmd
curl -X GET http://localhost:8080/user/profile
```
响应：
```json
{
  "error": "认证失败: 缺少认证Token"
}
```

注意事项：
1. 需要将 `YOUR_TOKEN` 替换为实际的 token（从登录或注册响应中获取）
2. token 有效期是一周，如果过期需要重新登录获取
3. 所有需要认证的接口都需要在 header 中加入 `Authorization: Bearer YOUR_TOKEN`
4. 注意 Windows 下 JSON 字符串中的引号需要转义 `\"`