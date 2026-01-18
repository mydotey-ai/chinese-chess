import http from 'http';
import fs from 'fs';
import path from 'path';

const PORT = 8080;

const server = http.createServer((req, res) => {
    let filePath = '.' + req.url;
    
    console.log(`${req.method} ${req.url}`);
    
    if (filePath === './') {
        filePath = './debug-app.html';
    }
    
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
    
    fs.readFile(filePath, (error, content) => {
        if (error) {
            if(error.code === 'ENOENT'){
                res.writeHead(404, { 'Content-Type': 'text/html' });
                res.end('<h1>404 - 文件未找到</h1>', 'utf-8');
                console.log(`未找到文件: ${filePath}`);
            }
            else {
                res.writeHead(500);
                res.end(`服务器错误: ${error.code} ..\n`);
                console.error(`服务器错误: ${error.code}`);
            }
        }
        else {
            res.writeHead(200, { 'Content-Type': contentType });
            res.end(content, 'utf-8');
            console.log(`成功返回: ${filePath}`);
        }
    });
});

server.listen(PORT, () => {
    console.log(`服务器运行在 http://localhost:${PORT}/`);
    console.log(`调试页面地址: http://localhost:${PORT}/debug-app.html`);
    console.log(`测试页面地址: http://localhost:${PORT}/simple-app.html`);
    console.log('按 Ctrl+C 停止服务器');
});

process.on('SIGINT', () => {
    console.log('\n正在停止服务器...');
    server.close(() => {
        console.log('服务器已停止');
        process.exit(0);
    });
});

process.on('uncaughtException', (err) => {
    console.error('未捕获的异常:', err);
    process.exit(1);
});