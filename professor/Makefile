.PHONY: help start start-all start-unified start-fullscan start-incremental start-rete start-frontend stop status check health clean

# 定义颜色输出
GREEN  := \033[32m
YELLOW := \033[33m
BLUE   := \033[34m
RED    := \033[31m
RESET  := \033[0m

# Python 命令
PYTHON := python3

# 服务器配置
UNIFIED_PORT     := 8080
FULLSCAN_PORT    := 8081
INCREMENTAL_PORT := 8082
RETE_PORT        := 8083

# 服务器目录
FULLSCAN_DIR     := fullscan_py
INCREMENTAL_DIR  := incremental_py
RETE_DIR         := rete_py
SERVER_DIR       := server
FRONTEND_DIR     := ../show-web
FRONTEND_PORT    := 5173

# PID 文件目录
PID_DIR := .pids

help:
	@echo "$(BLUE)动物识别专家系统 - 服务器管理工具$(RESET)"
	@echo ""
	@echo "$(YELLOW)统一服务器（推荐）：$(RESET)"
	@echo "  make start              $(GREEN)# 启动后端 + 前端 (端口 8080 + 5173)$(RESET)"
	@echo "  make start-unified     $(GREEN)# 仅启动后端统一服务器 (端口 8080)$(RESET)"
	@echo "  make start-frontend    $(GREEN)# 仅启动前端开发服务器 (端口 5173)$(RESET)"
	@echo ""
	@echo "$(YELLOW)独立服务器（不推荐）：$(RESET)"
	@echo "  make start-all         $(GREEN)# 启动所有独立服务器 (端口 8081-8083)$(RESET)"
	@echo "  make start-fullscan    $(GREEN)# 启动全扫描算法服务器 (端口 8081)$(RESET)"
	@echo "  make start-incremental $(GREEN)# 启动增量触发算法服务器 (端口 8082)$(RESET)"
	@echo "  make start-rete        $(GREEN)# 启动Rete网络算法服务器 (端口 8083)$(RESET)"
	@echo ""
	@echo "$(YELLOW)服务器控制：$(RESET)"
	@echo "  make stop              $(YELLOW)# 停止所有服务器$(RESET)"
	@echo "  make status            $(YELLOW)# 查看服务器状态$(RESET)"
	@echo "  make health            $(YELLOW)# 健康检查$(RESET)"
	@echo "  make check             $(YELLOW)# 检查端口占用$(RESET)"
	@echo "  make clean             $(YELLOW)# 清理PID文件和日志$(RESET)"
	@echo ""
	@echo "$(YELLOW)API 使用示例：$(RESET)"
	@echo "  # 使用全扫描算法"
	@echo "  curl 'http://localhost:8080/api/rules?algo=fullscan'"
	@echo ""
	@echo "  # 使用增量触发算法"
	@echo "  curl 'http://localhost:8080/api/rules?algo=incremental'"
	@echo ""
	@echo "  # 使用Rete网络算法"
	@echo "  curl 'http://localhost:8080/api/rules?algo=rete'"
	@echo ""

# ==================== 统一服务器 ====================

start: start-unified start-frontend
	@echo ""
	@echo "$(GREEN)✓ 所有服务已启动！$(RESET)"
	@echo "$(BLUE)后端: http://localhost:$(UNIFIED_PORT)$(RESET)"
	@echo "$(BLUE)前端: http://localhost:$(FRONTEND_PORT)$(RESET)"

start-unified:
	@if lsof -Pi :$(UNIFIED_PORT) -sTCP:LISTEN -t >/dev/null 2>&1; then \
		echo "$(YELLOW)⚠ 统一服务器已在运行 (端口 $(UNIFIED_PORT))$(RESET)"; \
	else \
		mkdir -p $(PID_DIR); \
		cd $(SERVER_DIR) && nohup $(PYTHON) __main__.py > ../$(PID_DIR)/unified.log 2>&1 & \
		echo $$! > $(PID_DIR)/unified.pid; \
		sleep 1; \
		if lsof -Pi :$(UNIFIED_PORT) -sTCP:LISTEN -t >/dev/null 2>&1; then \
			echo "$(GREEN)✓ 统一服务器已启动 (端口 $(UNIFIED_PORT))$(RESET)"; \
			echo "$(BLUE)支持的算法: fullscan, incremental, rete$(RESET)"; \
			echo "$(BLUE)使用 ?algo=<algorithm> 参数切换算法$(RESET)"; \
		else \
			echo "$(RED)✗ 统一服务器启动失败，请检查日志 $(PID_DIR)/unified.log$(RESET)"; \
		fi \
	fi

# ==================== 前端服务器 ====================

start-frontend:
	@if lsof -Pi :$(FRONTEND_PORT) -sTCP:LISTEN -t >/dev/null 2>&1; then \
		echo "$(YELLOW)⚠ 前端服务器已在运行 (端口 $(FRONTEND_PORT))$(RESET)"; \
	else \
		mkdir -p $(PID_DIR); \
		cd $(FRONTEND_DIR) && nohup npm run dev > ../professor/$(PID_DIR)/frontend.log 2>&1 & \
		echo $$! > $(PID_DIR)/frontend.pid; \
		sleep 3; \
		if lsof -Pi :$(FRONTEND_PORT) -sTCP:LISTEN -t >/dev/null 2>&1; then \
			echo "$(GREEN)✓ 前端服务器已启动 (端口 $(FRONTEND_PORT))$(RESET)"; \
		else \
			echo "$(YELLOW)⚠ 前端服务器可能需要更长时间启动，请检查日志 $(PID_DIR)/frontend.log$(RESET)"; \
		fi \
	fi

# ==================== 独立服务器 ====================

start-all: start-fullscan start-incremental start-rete
	@echo ""
	@echo "$(GREEN)✓ 所有独立服务器已启动！$(RESET)"
	@echo "$(BLUE)访问地址：$(RESET)"
	@echo "  - 全扫描算法:   http://localhost:$(FULLSCAN_PORT)"
	@echo "  - 增量触发算法: http://localhost:$(INCREMENTAL_PORT)"
	@echo "  - Rete网络算法: http://localhost:$(RETE_PORT)"
	@echo ""
	@echo "$(YELLOW)提示：推荐使用统一服务器 'make start-unified'$(RESET)"

start-fullscan:
	@mkdir -p $(PID_DIR)
	@if lsof -Pi :$(FULLSCAN_PORT) -sTCP:LISTEN -t >/dev/null 2>&1; then \
		echo "$(YELLOW)⚠ 全扫描服务器已在运行 (端口 $(FULLSCAN_PORT))$(RESET)"; \
	else \
		cd $(FULLSCAN_DIR) && nohup $(PYTHON) web_server.py > $(PID_DIR)/fullscan.log 2>&1 & \
		echo $$! > $(PID_DIR)/fullscan.pid; \
		sleep 1; \
		if lsof -Pi :$(FULLSCAN_PORT) -sTCP:LISTEN -t >/dev/null 2>&1; then \
			echo "$(GREEN)✓ 全扫描算法服务器已启动 (端口 $(FULLSCAN_PORT))$(RESET)"; \
		else \
			echo "$(RED)✗ 全扫描服务器启动失败，请检查日志 $(PID_DIR)/fullscan.log$(RESET)"; \
		fi \
	fi

start-incremental:
	@mkdir -p $(PID_DIR)
	@if lsof -Pi :$(INCREMENTAL_PORT) -sTCP:LISTEN -t >/dev/null 2>&1; then \
		echo "$(YELLOW)⚠ 增量触发服务器已在运行 (端口 $(INCREMENTAL_PORT))$(RESET)"; \
	else \
		cd $(INCREMENTAL_DIR) && nohup $(PYTHON) web_server.py > $(PID_DIR)/incremental.log 2>&1 & \
		echo $$! > $(PID_DIR)/incremental.pid; \
		sleep 1; \
		if lsof -Pi :$(INCREMENTAL_PORT) -sTCP:LISTEN -t >/dev/null 2>&1; then \
			echo "$(GREEN)✓ 增量触发算法服务器已启动 (端口 $(INCREMENTAL_PORT))$(RESET)"; \
		else \
			echo "$(RED)✗ 增量触发服务器启动失败，请检查日志 $(PID_DIR)/incremental.log$(RESET)"; \
		fi \
	fi

start-rete:
	@mkdir -p $(PID_DIR)
	@if lsof -Pi :$(RETE_PORT) -sTCP:LISTEN -t >/dev/null 2>&1; then \
		echo "$(YELLOW)⚠ Rete服务器已在运行 (端口 $(RETE_PORT))$(RESET)"; \
	else \
		cd $(RETE_DIR) && nohup $(PYTHON) web_server.py > $(PID_DIR)/rete.log 2>&1 & \
		echo $$! > $(PID_DIR)/rete.pid; \
		sleep 1; \
		if lsof -Pi :$(RETE_PORT) -sTCP:LISTEN -t >/dev/null 2>&1; then \
			echo "$(GREEN)✓ Rete网络算法服务器已启动 (端口 $(RETE_PORT))$(RESET)"; \
		else \
			echo "$(RED)✗ Rete服务器启动失败，请检查日志 $(PID_DIR)/rete.log$(RESET)"; \
		fi \
	fi

# ==================== 服务器控制 ====================

stop:
	@echo "$(YELLOW)正在停止服务器...$(RESET)"
	@for service in unified fullscan incremental rete frontend; do \
		if [ -f $(PID_DIR)/$$service.pid ]; then \
			pid=$$(cat $(PID_DIR)/$$service.pid); \
			if kill -0 $$pid 2>/dev/null; then \
				kill $$pid 2>/dev/null; \
				sleep 0.5; \
				if kill -0 $$pid 2>/dev/null; then \
					kill -9 $$pid 2>/dev/null; \
				fi; \
				echo "$(GREEN)✓ $$service 服务器已停止$(RESET)"; \
			else \
				echo "$(YELLOW)⚠ $$service 服务器未运行$(RESET)"; \
			fi; \
			rm -f $(PID_DIR)/$$service.pid; \
		else \
			echo "$(YELLOW)⚠ $$service PID文件不存在$(RESET)"; \
		fi \
	done
	@echo "$(GREEN)✓ 所有服务器已停止$(RESET)"

status:
	@echo "$(BLUE)服务器状态：$(RESET)"
	@echo ""
	@echo "┌─────────────────┬───────────┬──────────────┐"
	@echo "│ 服务器          │ 端口      │ 状态         │"
	@echo "├─────────────────┼───────────┼──────────────┤"
	@for service in unified; do \
		port=$(UNIFIED_PORT); \
		if lsof -Pi :$$port -sTCP:LISTEN -t >/dev/null 2>&1; then \
			status="$(GREEN)● 运行中$(RESET)"; \
		else \
			status="$(RED)○ 已停止$(RESET)"; \
		fi; \
		printf "│ %-15s │ %-9s │ %-12s │\n" "统一服务器" "$$port" "$$status"; \
	done
	@echo "├─────────────────┼───────────┼──────────────┤"
	@for service in fullscan incremental rete; do \
		port=$(FULLSCAN_PORT); \
		if [ "$$service" = "incremental" ]; then port=$(INCREMENTAL_PORT); fi; \
		if [ "$$service" = "rete" ]; then port=$(RETE_PORT); fi; \
		algo=""; \
		if [ "$$service" = "fullscan" ]; then algo="全扫描"; fi; \
		if [ "$$service" = "incremental" ]; then algo="增量触发"; fi; \
		if [ "$$service" = "rete" ]; then algo="Rete"; fi; \
		if lsof -Pi :$$port -sTCP:LISTEN -t >/dev/null 2>&1; then \
			status="$(GREEN)● 运行中$(RESET)"; \
		else \
			status="$(RED)○ 已停止$(RESET)"; \
		fi; \
		printf "│ %-15s │ %-9s │ %-12s │\n" "$$algo" "$$port" "$$status"; \
	done
	@echo "└─────────────────┴───────────┴──────────────┘"
	@echo ""
	@echo "$(BLUE)前端服务器：$(RESET)"
	@echo "┌─────────────────┬───────────┬──────────────┐"
	@if lsof -Pi :$(FRONTEND_PORT) -sTCP:LISTEN -t >/dev/null 2>&1; then \
		printf "│ %-15s │ %-9s │ $(GREEN)● 运行中$(RESET)   │\n" "Vue前端" "$(FRONTEND_PORT)"; \
	else \
		printf "│ %-15s │ %-9s │ $(RED)○ 已停止$(RESET)   │\n" "Vue前端" "$(FRONTEND_PORT)"; \
	fi
	@echo "└─────────────────┴───────────┴──────────────┘"

health:
	@echo "$(BLUE)健康检查：$(RESET)"
	@for service in unified fullscan incremental rete; do \
		port=$(UNIFIED_PORT); name="统一服务器"; \
		if [ "$$service" = "fullscan" ]; then port=$(FULLSCAN_PORT); name="全扫描"; fi; \
		if [ "$$service" = "incremental" ]; then port=$(INCREMENTAL_PORT); name="增量触发"; fi; \
		if [ "$$service" = "rete" ]; then port=$(RETE_PORT); name="Rete"; fi; \
		if lsof -Pi :$$port -sTCP:LISTEN -t >/dev/null 2>&1; then \
			response=$$(curl -s -o /dev/null -w "%{http_code}" http://localhost:$$port/ 2>/dev/null); \
			if [ "$$response" = "200" ]; then \
				echo "$(GREEN)✓ $$name (端口 $$port) - 健康$(RESET)"; \
			else \
				echo "$(YELLOW)⚠ $$name (端口 $$port) - 响应异常 (HTTP $$response)$(RESET)"; \
			fi \
		else \
			echo "$(RED)✗ $$name (端口 $$port) - 未运行$(RESET)"; \
		fi \
	done

check:
	@echo "$(BLUE)端口占用检查：$(RESET)"
	@echo ""
	@echo "统一服务器端口: $(UNIFIED_PORT)"
	@lsof -i :$(UNIFIED_PORT) 2>/dev/null || echo "  (未占用)"
	@echo ""
	@echo "独立服务器端口:"
	@for port in $(FULLSCAN_PORT) $(INCREMENTAL_PORT) $(RETE_PORT); do \
		echo "端口 $${port}:"; \
		lsof -i :$$port 2>/dev/null || echo "  (未占用)"; \
		echo ""; \
	done

clean:
	@echo "$(YELLOW)清理PID文件和日志...$(RESET)"
	@rm -rf $(PID_DIR)
	@echo "$(GREEN)✓ 清理完成$(RESET)"

# ==================== 开发辅助 ====================

test-api:
	@echo "$(BLUE)测试统一服务器 API：$(RESET)"
	@echo ""
	@echo "获取规则 (fullscan):"
	@curl -s 'http://localhost:8080/api/rules?algo=fullscan' | head -c 200
	@echo ""
	@echo ""
	@echo "获取规则 (incremental):"
	@curl -s 'http://localhost:8080/api/rules?algo=incremental' | head -c 200
	@echo ""
	@echo ""
	@echo "获取规则 (rete):"
	@curl -s 'http://localhost:8080/api/rules?algo=rete' | head -c 200
	@echo ""

logs:
	@echo "$(BLUE)查看服务器日志：$(RESET)"
	@for service in unified fullscan incremental rete; do \
		if [ -f $(PID_DIR)/$$service.log ]; then \
			echo ""; \
			echo "$(YELLOW)=== $$service ===$(RESET)"; \
			tail -20 $(PID_DIR)/$$service.log; \
		fi; \
	done

restart:
	@$(MAKE) stop
	@sleep 1
	@$(MAKE) start
