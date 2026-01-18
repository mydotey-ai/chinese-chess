@echo off
chcp 65001 >nul

echo.
echo 中国象棋应用程序构建和启动脚本
echo.

setlocal enabledelayedexpansion

:: 检查是否使用了参数
if "%1"=="clean" (
    call :cleanup
    goto :end
) else if "%1"=="build" (
    call :build_release
    goto :end
) else if "%1"=="run" (
    call :run_app
    goto :end
) else if "%1"=="help" (
    call :show_help
    goto :end
) else (
    call :full_process
    goto :end
)

:: 完整流程：检查依赖、清理、构建、启动
:full_process
echo [INFO] 开始构建和启动中国象棋应用程序

:: 检查依赖项
call :check_dependencies
if errorlevel 1 goto :error

:: 清理现有进程
call :cleanup_processes

:: 构建前端
call :build_frontend
if errorlevel 1 goto :error

:: 构建后端
call :build_backend
if errorlevel 1 goto :error

:: 启动开发服务器
call :start_dev_server

:: 启动 Tauri 应用程序
call :start_tauri_app

goto :end

:: 清理所有构建产物和进程
:cleanup
echo [INFO] 进入清理模式...

:: 停止现有进程
call :cleanup_processes

:: 删除前端构建产物
if exist "frontend\dist" (
    echo [INFO] 正在删除前端构建产物...
    rmdir /s /q "frontend\dist"
)

:: 删除 Rust 构建产物
if exist "target" (
    echo [INFO] 正在删除 Rust 构建产物...
    cargo clean
)

:: 删除 node_modules
if exist "frontend\node_modules" (
    echo [INFO] 正在删除前端依赖...
    rmdir /s /q "frontend\node_modules"
)

echo [SUCCESS] 清理完成
goto :end

:: 构建发布版本
:build_release
echo [INFO] 进入只构建模式...

call :check_dependencies
if errorlevel 1 goto :error

call :cleanup_processes

call :build_frontend
if errorlevel 1 goto :error

echo [INFO] 正在构建发布版本的 Rust 后端...
cargo build --release
if errorlevel 1 goto :error

echo [SUCCESS] 发布版本构建成功
echo [INFO] 可执行文件位置: %cd%\target\release\chinese-chess.exe
goto :end

:: 只运行应用程序
:run_app
if not exist "target\debug\chinese-chess.exe" (
    echo [WARNING] 未找到 debug 版本，正在构建...
    
    call :check_dependencies
    if errorlevel 1 goto :error
    
    call :cleanup_processes
    
    call :build_frontend
    if errorlevel 1 goto :error
    
    call :build_backend
    if errorlevel 1 goto :error
)

echo [INFO] 正在启动 Tauri 应用程序...
cargo run

goto :end

:: 显示帮助信息
:show_help
echo 中国象棋应用程序构建和启动脚本
echo.
echo 用法: %0 [命令]
echo.
echo 命令:
echo   help    显示此帮助信息
echo   clean   清理所有构建产物和进程
echo   build   构建发布版本
echo   run     运行应用程序（如果需要会先构建）
echo   (无参数) 执行完整流程：检查依赖、清理、构建、启动
echo.
echo 示例:
echo   %0              # 执行完整流程
echo   %0 build        # 只构建发布版本
echo   %0 clean        # 清理所有构建产物
echo   %0 help         # 显示帮助信息
goto :end

:: 检查依赖项
:check_dependencies
echo [INFO] 正在检查依赖项...

:: 检查 npm 是否可用
npm --version >nul 2>&1
if errorlevel 1 (
    echo [ERROR] npm 未找到，请安装 Node.js 和 npm
    exit /b 1
)

:: 检查 cargo 是否可用
cargo --version >nul 2>&1
if errorlevel 1 (
    echo [ERROR] cargo 未找到，请安装 Rust
    exit /b 1
)

echo [SUCCESS] 所有依赖项已满足
exit /b 0

:: 清理现有进程
:cleanup_processes
echo [INFO] 正在清理现有进程...
taskkill /f /im vite.exe >nul 2>&1 || true
taskkill /f /im chinese-chess.exe >nul 2>&1 || true
timeout /t 2 /nobreak >nul

exit /b 0

:: 构建前端
:build_frontend
echo [INFO] 正在构建前端...

cd frontend

:: 检查 node_modules 是否存在
if not exist "node_modules" (
    echo [INFO] node_modules 不存在，正在安装依赖...
    npm install
    if errorlevel 1 (
        echo [ERROR] 依赖安装失败
        cd ..
        exit /b 1
    )
)

:: 构建前端
npm run build
if errorlevel 1 (
    echo [ERROR] 前端构建失败
    cd ..
    exit /b 1
)

cd ..
echo [SUCCESS] 前端构建成功

exit /b 0

:: 构建后端
:build_backend
echo [INFO] 正在构建 Rust 后端...

cargo build
if errorlevel 1 (
    echo [ERROR] Rust 后端构建失败
    exit /b 1
)

echo [SUCCESS] Rust 后端构建成功

exit /b 0

:: 启动开发服务器
:start_dev_server
echo [INFO] 正在启动开发服务器...

:: 启动 Vite 开发服务器（后台运行）
cd frontend
start "" npm run dev
cd ..

:: 等待服务器启动
echo [INFO] 正在等待 Vite 服务器启动...
timeout /t 5 /nobreak >nul

exit /b 0

:: 启动 Tauri 应用程序
:start_tauri_app
echo [INFO] 正在启动 Tauri 应用程序...

cargo run

exit /b 0

:: 错误处理
:error
echo [ERROR] 脚本执行失败
goto :end

:end
endlocal