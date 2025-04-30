# AI 模块 API 文档（完整版）

## 接口说明

### 1. 纯文本分析接口

![text](text.png)

- **请求示例**：
```bash
curl -X POST http://localhost:8080/ai/text -H "Content-Type: application/json" -d "{\"input\":{\"type\":\"Text\",\"content\":\"你最擅长做什么？\"},\"model\":\"qwen-turbo\"}"
```
- **响应示例**：
```json
{"content":"我最擅长处理各种文本相关的任务...","confidence":null,"raw_response":{"output":{"finish_reason":"stop","text":"我最擅长处理各种文本相关的任务..."},"usage":{"total_tokens":53,"output_tokens":40,"input_tokens":13}}}
```

### 2. 图片分析接口

![image](image.png)

- **请求示例**：
```bash
curl -X POST http://localhost:8080/ai/image -F "image=@image.png" -F "prompt=请分析这张图片的内容" -F "model=qwen-vl-max"
```
- **响应示例**：
```json
{"content":"这张图片展示了一杯水果饮品...","confidence":null,"raw_response":{"output":{"choices":[{"finish_reason":"stop","message":{"role":"assistant","content":[{"text":"这张图片展示了一杯水果饮品..."}]}}]},"usage":{"input_tokens_details":{"text_tokens":24,"image_tokens":128},"total_tokens":258}}}
```

## 响应字段说明
- `content`: AI生成的主要内容
- `confidence`: 置信度评分（当前版本为null）
- `raw_response`: 原始API响应数据（含token用量等元数据）

## 环境配置
```bash
export AI_TONGYI_API_KEY="your_api_key"
```

## 注意事项
1. 图片文件需为PNG/JPG格式
2. 文本接口最大支持8000字符
3. 图片大小建议不超过5MB
4. 错误响应会返回4xx/5xx状态码