// server.js

const http = require('http');
const fs = require('fs');
const path = require('path');

// 创建简单的 HTTP 服务器
const PORT = 8080;

const server = http.createServer((req, res) => {
    let filePath = '.' + req.url;
    
    console.log(`${req.method} ${req.url}`);
    
    // 处理根路径
    if (filePath === './') {
        filePath = './simple-app.html';
    }
    
    // 确定文件扩展名和内容类型
    const extname = String(path.extname(filePath)).toLowerCase();
    const contentType = {
        '.html': 'text/html',
        '.js': 'text/javascript',
        '.css': 'text/css',
        '.json': 'application/json',
        '.png': 'image/png',
        '.jpg': 'image/jpg',
        '.gif': 'image/gif',
        '.svg': 'image/svg+xml',
        '.ico': 'image/x-icon'
    }[extname] || 'application/octet-stream';
    
    // 读取并返回文件
    fs.readFile(filePath, (error, content) => {
        if (error) {
            if(error.code === 'ENOENT'){
                // 404 错误
                res.writeHead(404, { 'Content-Type': 'text/html' });
                res.end('<h1>404 - 文件未找到</h1>', 'utf-8');
                console.log(`未找到文件: ${filePath}`);
            }
            else {
                // 服务器内部错误
                res.writeHead(500);
                res.end(`服务器错误: ${error.code} ..\n`);
                console.error(`服务器错误: ${error.code}`);
            }
        }
        else {
            // 成功
            res.writeHead(200, { 'Content-Type': contentType });
            res.end(content, 'utf-8');
            console.log(`成功返回: ${filePath}`);
        }
    });
});

server.listen(PORT, () => {
    console.log(`服务器运行在 http://localhost:${PORT}/`);
    console.log(`测试页面地址: http://localhost:${PORT}/simple-app.html`);
    console.log(`前端构建地址: http://localhost:${PORT}/frontend/dist/index.html`);
    console.log('按 Ctrl+C 停止服务器');
});

// 处理退出信号
process.on('SIGINT', () => {
    console.log('\n正在停止服务器...');
    server.close(() => {
        console.log('服务器已停止');
        process.exit(0);
    });
});

// 处理未捕获的异常
process.on('uncaughtException', (err) => {
    console.error('未捕获的异常:', err);
    process.exit(1);
});