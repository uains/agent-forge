# 第5课

## 问题

1、为什么不直接使用Request发起请求，还要单独创建客户端？

在hyper官方网站的Client案例中使用handshake，与Client有什么区别

## Hyper错误处理

1、错误来源

| 错误来源        | 原因                    |
| --------------- | ----------------------- |
| URI Parsing     | URL格式错误             |
| Request Builder | Header或Request构建错误 |
