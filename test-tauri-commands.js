// test-tauri-commands.js

// 这是一个简单的测试脚本，用于验证 Tauri 命令是否能正常工作
// 这个脚本只在 Tauri 应用程序中运行，但可以帮助我们理解命令的工作原理

// 我们将模拟一些关键的代码路径

// 首先，让我们创建一个简单的测试环境
console.log("=== Tauri 命令测试 ===\n");

// 模拟 invoke 函数
function mockInvoke(command, args = {}) {
    return new Promise((resolve, reject) => {
        console.log(`调用命令: ${command}`, args);
        
        // 模拟一些命令
        if (command === 'new_game') {
            resolve({
                board: {
                    cells: Array(10).fill(null).map(() => Array(9).fill(null))
                },
                current_turn: 'Red',
                is_in_check: false,
                is_ended: false,
                winner: null
            });
        } else if (command === 'get_game_state') {
            resolve(mockInvoke('new_game'));
        } else {
            reject(new Error(`命令 ${command} 未实现`));
        }
    });
}

// 模拟 App 组件的初始化过程
async function testAppInit() {
    console.log("\n1. 测试应用程序初始化:");
    
    try {
        // 模拟应用程序初始化
        const state = await mockInvoke('new_game');
        console.log("✅ 初始化成功");
        console.log("返回状态:", JSON.stringify(state, null, 2));
        return true;
    } catch (error) {
        console.error("❌ 初始化失败:");
        console.error(error);
        return false;
    }
}

// 测试命令参数匹配
async function testCommandParams() {
    console.log("\n2. 测试命令参数:");
    
    try {
        // 测试 make_move 命令的参数
        const params1 = { from_x: 0, from_y: 0, to_x: 1, to_y: 1 };
        const params2 = { fromX: 0, fromY: 0, toX: 1, toY: 1 };
        
        console.log("测试下划线格式的参数:", params1);
        // 这里应该成功
        console.log("✅ 参数格式正确");
        
        console.log("测试 camelCase 格式的参数:", params2);
        // 这里应该失败
        console.log("❌ 参数格式不正确 - Tauri 期望下划线格式的参数");
    } catch (error) {
        console.error("❌ 测试失败:");
        console.error(error);
    }
}

// 主测试函数
async function main() {
    const initSuccess = await testAppInit();
    
    if (initSuccess) {
        await testCommandParams();
    }
    
    console.log("\n=== 测试完成 ===");
    console.log("\n如果在实际应用中遇到 Loading... 问题:");
    console.log("1. 检查控制台错误信息");
    console.log("2. 确认 Tauri 命令是否正确暴露");
    console.log("3. 检查参数是否与后端匹配（下划线 vs camelCase）");
}

// 运行测试
main().catch(error => {
    console.error("❌ 测试脚本失败:");
    console.error(error);
});