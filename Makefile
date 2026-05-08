.PHONY: help start stop status restart

help:
	@echo "AI 技术应用平台 - 服务管理"
	@echo ""
	@echo "  make start      启动所有服务"
	@echo "  make stop      停止所有服务"
	@echo "  make status    查看服务状态"
	@echo "  make restart   重启所有服务"
	@echo ""
	@echo "服务端口:"
	@echo "  professor:   http://localhost:8080"
	@echo "  wayfind:     http://localhost:8081"
	@echo "  show-web:    http://localhost:5173"

start: ## 启动所有服务
	@echo "启动所有服务..."
	@cd professor && nohup python3 server/unified_server.py > /dev/null 2>&1 &
	@cd way_find/backend && go build -o wayfind-api . > /dev/null 2>&1 && ./wayfind-api -port 8081 > /dev/null 2>&1 &
	@cd show-web && npm run dev -- --host > /dev/null 2>&1 &
	@sleep 4
	@echo ""
	@echo "所有服务已启动:"
	@echo "  professor:  http://localhost:8080"
	@echo "  wayfind:    http://localhost:8081"
	@echo "  show-web:   http://localhost:5173"

stop: ## 停止所有服务
	@fuser -k 8080/tcp 2>/dev/null; echo ""
	@fuser -k 8081/tcp 2>/dev/null; echo ""
	@fuser -k 5173/tcp 2>/dev/null; echo ""
	@echo "所有服务已停止"

status: ## 查看服务状态
	@echo "服务状态:"
	@echo ""
	@(lsof -i :8080 >/dev/null 2>&1 && echo "  [RUNNING] professor  :8080" || echo "  [STOPPED] professor  :8080")
	@(lsof -i :8081 >/dev/null 2>&1 && echo "  [RUNNING] wayfind   :8081" || echo "  [STOPPED] wayfind   :8081")
	@(lsof -i :5173 >/dev/null 2>&1 && echo "  [RUNNING] show-web :5173" || echo "  [STOPPED] show-web :5173")

restart: stop && sleep 1 && start ## 重启所有服务
