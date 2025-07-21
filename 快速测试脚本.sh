#!/bin/bash
# Rosetta Connect 快速测试脚本

set -e

echo "🚀 Rosetta Connect 快速功能测试"
echo "================================"

BINARY="$(pwd)/target/release/rosetta-connect"
TEST_DIR="test-demo-$(date +%s)"

# 检查二进制文件是否存在
if [ ! -f "$BINARY" ]; then
    echo "❌ 二进制文件不存在，请先运行 'cargo build --release'"
    exit 1
fi

echo "📁 创建测试目录: $TEST_DIR"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo ""
echo "🔧 测试 1: 项目初始化"
echo "----------------------"
"$BINARY" init --bundle-id com.test.quickdemo

echo ""
echo "📥 测试 2: 拉取内容"
echo "-------------------"
"$BINARY" pull

echo ""
echo "🤖 测试 3: AI 翻译"
echo "------------------"
"$BINARY" translate --locales zh-Hans,fr-FR

echo ""
echo "📊 测试 4: 差异对比"
echo "-------------------"
"$BINARY" diff

echo ""
echo "💰 测试 5: 成本估算"
echo "-------------------"
"$BINARY" cost --detailed

echo ""
echo "👀 测试 6: 内容预览"
echo "-------------------"
"$BINARY" preview --locale zh-Hans

echo ""
echo "✅ 测试 7: 内容验证"
echo "-------------------"
"$BINARY" validate

echo ""
echo "📤 测试 8: 自动推送"
echo "-------------------"
"$BINARY" push 1.0.0 --yes

echo ""
echo "📋 测试 9: 模板管理"
echo "-------------------"
"$BINARY" template list

echo ""
echo "🎉 所有测试完成！"
echo "================"
echo "✅ 所有功能正常工作"
echo "📂 测试文件保存在: $(pwd)"
echo "🗑️  可以删除测试目录: rm -rf $(pwd)"

cd ..