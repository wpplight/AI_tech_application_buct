.PHONY: help start stop status restart dev clean

# 服务配置
PROFESSOR_PORT := 8080
WAYFIND_PORT := 8081
SHOWWEB_PORT := 5173

PROFESSOR_DIR := professor
PROFESSOR_START := python3 $(PROFESSOR_DIR)/server/unified_server.py
WAYFIND_DIR := way_find/backend
WAYFIND_BUILD := go build -o wayfind-api .
WAYFIND_START := ./wayfind-api -port $(WAYFIND_PORT)
SHOWWEB_DIR := show-web

help: ## 显示帮助信息
	@echo "AI 技术应用平台 - Makefile"
	@echo ""
	@echo "用法:"
	@echo "  make start      启动所有服务"
	@echo "  make stop       停止所有服务"
	@echo "  make status     查看服务状态"
	@echo "  make restart    重启所有服务"
	@echo ""
	@echo "独立控制:"
	@echo "  make professor.start    启动专家系统 (端口 $(PROFESSOR_PORT))"
	@echo "  make wayfind.start     启动寻路算法 (端口 $(WAYFIND_PORT))"
	@echo "  make showweb.start     启动前端 (端口 $(SHOWWEB_PORT))"
	@echo "  make professor.stop     停止专家系统"
	@echo "  make wayfind.stop      停止寻路算法"
	@echo "  make showweb.stop      停止前端"
	@echo ""
	@echo "服务端口:"
	@echo "  专家系统:  http://localhost:$(PROFESSOR_PORT)"
	@echo "  寻路算法:  http://localhost:$(WAYFIND_PORT)"
	@echo "  前端界面:  http://localhost:$(SHOWWEB_PORT)"

start: ## 启动所有服务
	@echo "启动所有服务..."
	@make professor.start &
	@make wayfind.start &
	@make showweb.start &
	@echo ""
	@echo "所有服务已启动:"
	@echo "  专家系统:  http://localhost:$(PROFESSOR_PORT)"
	@echo "  寻路算法:  http://localhost:$(WAYFIND_PORT)"
	@echo "  前端界面:  http://localhost:$(SHOWWEB_PORT)"

stop: ## 停止所有服务
	@echo "停止所有服务..."
	@make professor.stop
	@make wayfind.stop
	@make showweb.stop
	@echo "所有服务已停止"

status: ## 查看服务状态
	@echo "服务状态:"
	@echo ""
	@(lsof -i :$(PROFESSOR_PORT) >/dev/null 2>&1 && echo "  [RUNNING] professor     :$(PROFESSOR_PORT)" || echo "  [STOPPED] professor     :$(PROFESSOR_PORT)")
	@(lsof -i :$(WAYFIND_PORT) >/dev/null 2>&1 && echo "  [RUNNING] wayfind      :$(WAYFIND_PORT)" || echo "  [STOPPED] wayfind      :$(WAYFIND_PORT)")
	@(lsof -i :$(SHOWWEB_PORT) >/dev/null 2>&1 && echo "  [RUNNING] show-web     :$(SHOWWEB_PORT)" || echo "  [STOPPED] show-web     :$(SHOWWEB_PORT)")

restart: stop start ## 重启所有服务

# --- 专家系统 (端口 8080) ---
professor.start:
	@echo "启动专家系统 (端口 $(PROFESSOR_PORT))..."
	@cd $(PROFESSOR_DIR) && $(PROFESSOR_START) > /dev/null 2>&1 &
	@sleep 1
	@lsof -i :$(PROFESSOR_PORT) >/dev/null 2>&1 && echo "  ✓ professor 已启动" || echo "  ✗ professor 启动失败"

professor.stop:
	@echo "停止专家系统..."
	@fuser -k $(PROFESSOR_PORT)/tcp 2>/dev/null && echo "  ✓ professor 已停止" || echo "  professor 未运行"

# --- 寻路算法 (端口 8081) ---
wayfind.start:
	@echo "启动寻路算法服务 (端口 $(WAYFIND_PORT))..."
	@cd $(WAYFIND_DIR) && $(WAYFIND_BUILD) > /dev/null 2>&1 && $(WAYFIND_START) > /dev/null 2>&1 &
	@sleep 2
	@lsof -i :$(WAYFIND_PORT) >/dev/null 2>&1 && echo "  ✓ wayfind 已启动" || echo "  ✗ wayfind 启动失败"

wayfind.stop:
	@echo "停止寻路算法..."
	@fuser -k $(WAYFIND_PORT)/tcp 2>/dev/null && echo "  ✓ wayfind 已停止" || echo "  wayfind 未运行"

# --- 前端 (端口 5173) ---
showweb.start:
	@echo "启动前端 (端口 $(SHOWWEB_PORT))..."
	@cd $(SHOWWEB_DIR) && npm run dev -- --host > /dev/null 2>&1 &
	@sleep 3
	@lsof -i :$(SHOWWEB_PORT) >/dev/null 2>&1 && echo "  ✓ show-web 已启动" || echo "  ✗ show-web 启动失败"

showweb.stop:
	@echo "停止前端..."
	@fuser -k $(SHOWWEB_PORT)/tcp 2>/dev/null && echo "  ✓ show-web 已停止" || echo "  show-web 未运行"

clean: stop ## 停止所有服务并清理进程
	@echo "清理完成"
