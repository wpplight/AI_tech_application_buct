.PHONY: dev build clean

APP_NAME := wayfind
ROOT_DIR := $(shell pwd)
BACKEND_DIR := $(ROOT_DIR)/backend
FRONTEND_DIR := $(ROOT_DIR)/frontend
BIN_DIR := $(BACKEND_DIR)/bin

dev:
	@cd $(BACKEND_DIR) && wails3 dev

build:
	@echo "Building frontend..."
	@cd $(FRONTEND_DIR) && npm run build
	@echo "Copying frontend assets..."
	@rm -rf $(BACKEND_DIR)/frontend/dist
	@cp -r $(FRONTEND_DIR)/dist $(BACKEND_DIR)/frontend/
	@echo "Building backend..."
	@cd $(BACKEND_DIR) && go build -tags desktop -o $(BIN_DIR)/$(APP_NAME)
	@echo "Build complete: $(BIN_DIR)/$(APP_NAME)"

clean:
	@rm -rf $(BACKEND_DIR)/frontend/dist
	@rm -rf $(BIN_DIR)/$(APP_NAME)
	@rm -f $(BACKEND_DIR)/backend
	@echo "Clean complete"
