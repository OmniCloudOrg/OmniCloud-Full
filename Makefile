# OmniCloud PaaS Makefile
# A unified build system for OmniCloud Platform components

# Determine OS and set appropriate commands
ifeq ($(OS),Windows_NT)
    DETECTED_OS := Windows
    # Use cmd.exe as shell even on PowerShell to ensure consistency
    SHELL := cmd.exe
    # Windows commands
    RM := del /Q
    RM_DIR := rmdir /S /Q
    MKDIR := mkdir
    CP := copy
    TOUCH := type nul >
    NULL_DEVICE := nul
    # Path separator
    SEP := \\
    # Command separator
    CMD_SEP := &
    # Make directory if it doesn't exist
    MKDIR_P = if not exist "$1" mkdir "$1"
    # Windows executables
    CARGO := cargo
    NPM := npm
    NODE := node
    DOCKER := docker
    DOCKER_COMPOSE := docker-compose
    # Add .exe extension
    EXE := .exe
else
    DETECTED_OS := $(shell uname -s)
    # Unix commands
    RM := rm -f
    RM_DIR := rm -rf
    MKDIR := mkdir -p
    CP := cp
    TOUCH := touch
    NULL_DEVICE := /dev/null
    # Path separator
    SEP := /
    # Command separator
    CMD_SEP := ;
    # Make directory if it doesn't exist
    MKDIR_P = mkdir -p "$1"
    # Unix executables
    CARGO := cargo
    NPM := npm
    NODE := node
    DOCKER := docker
    DOCKER_COMPOSE := docker-compose
    # No extension
    EXE :=
endif

# Main directories with OS-independent paths
CRATES_DIR := crates
SERVICES_DIR := services
DOCKER_DIR := docker
ROOT_DIR := .

# Define paths using platform-agnostic method
define path
$(subst /,$(SEP),$1)
endef

# Core crates
LODESTONE := $(call path,$(CRATES_DIR)/Lodestone)
OMNI_AGENT := $(call path,$(CRATES_DIR)/OmniAgent)
OMNI_CLI := $(call path,$(CRATES_DIR)/OmniCLI)
OMNI_DIRECTOR := $(call path,$(CRATES_DIR)/OmniDirector)
OMNI_FORGE := $(call path,$(CRATES_DIR)/OmniForge)
OMNI_ORCHESTRATOR := $(call path,$(CRATES_DIR)/OmniOrchestrator)

# Service directories
OMNI_COSMOS := $(call path,$(SERVICES_DIR)/OmniCosmos)
OMNI_EDITOR := $(call path,$(SERVICES_DIR)/OmniEditor)

# List of all crates and services
ALL_CRATES := $(LODESTONE) $(OMNI_AGENT) $(OMNI_CLI) $(OMNI_DIRECTOR) $(OMNI_FORGE) $(OMNI_ORCHESTRATOR)
ALL_SERVICES := $(OMNI_COSMOS) $(OMNI_EDITOR)

# Default target
.PHONY: all
all: build

# Show help
.PHONY: help
help:
	@echo OmniCloud PaaS Build System
	@echo ============================
	@echo.
	@echo Main targets:
	@echo   all                 - Build all crates (default)
	@echo   build               - Build all crates in debug mode
	@echo   release             - Build all crates in release mode
	@echo   test                - Run tests for all crates
	@echo   clean               - Remove build artifacts
	@echo   docs                - Generate documentation
	@echo   docker-build        - Build all Docker images
	@echo   docker-up           - Start all services with Docker Compose
	@echo   docker-down         - Stop all Docker Compose services
	@echo.
	@echo Individual crate targets:
	@echo   build-lodestone     - Build the Lodestone crate
	@echo   build-agent         - Build the OmniAgent crate
	@echo   build-cli           - Build the OmniCLI crate
	@echo   build-director      - Build the OmniDirector crate
	@echo   build-forge         - Build the OmniForge crate
	@echo   build-orchestrator  - Build the OmniOrchestrator crate
	@echo.
	@echo Frontend service targets:
	@echo   build-cosmos        - Build the OmniCosmos frontend
	@echo   build-editor        - Build the OmniEditor frontend
	@echo.
	@echo Development targets:
	@echo   lint                - Run linter on all crates
	@echo   format              - Format all code with rustfmt
	@echo   dev-setup           - Set up development environment
	@echo   update-deps         - Update dependencies for all crates
	@echo.
	@echo Deployment targets:
	@echo   deploy-dev          - Deploy to development environment
	@echo   deploy-staging      - Deploy to staging environment
	@echo   deploy-prod         - Deploy to production environment
	@echo.
	@echo Utility targets:
	@echo   check-env           - Check if development environment is properly set up
	@echo   db-init             - Initialize database schema
	@echo   backup              - Backup data and configuration
	@echo   help                - Show this help information

# Build targets
.PHONY: build build-lodestone build-agent build-cli build-director build-forge build-orchestrator build-cosmos build-editor

build: build-lodestone build-agent build-cli build-director build-forge build-orchestrator build-cosmos build-editor

build-lodestone:
	@echo Building Lodestone...
	@cd $(LODESTONE) $(CMD_SEP) $(CARGO) build

build-agent:
	@echo Building OmniAgent...
	@cd $(OMNI_AGENT) $(CMD_SEP) $(CARGO) build

build-cli:
	@echo Building OmniCLI...
	@cd $(OMNI_CLI) $(CMD_SEP) $(CARGO) build

build-director:
	@echo Building OmniDirector...
	@cd $(OMNI_DIRECTOR) $(CMD_SEP) $(CARGO) build

build-forge:
	@echo Building OmniForge...
	@cd $(OMNI_FORGE) $(CMD_SEP) $(CARGO) build

build-orchestrator:
	@echo Building OmniOrchestrator...
	@cd $(OMNI_ORCHESTRATOR) $(CMD_SEP) $(CARGO) build

build-cosmos:
	@echo Building OmniCosmos...
	@cd $(OMNI_COSMOS) $(CMD_SEP) $(NPM) install $(CMD_SEP) $(NPM) run build

build-editor:
	@echo Building OmniEditor...
	@cd $(OMNI_EDITOR) $(CMD_SEP) $(NPM) install $(CMD_SEP) $(NPM) run build

# Windows-specific batch command for foreach loop
ifeq ($(OS),Windows_NT)
# Release build - Windows version
.PHONY: release
release:
	@echo Building all crates in release mode...
	@for %%c in ($(subst $(SEP),/,$(subst $(SPACE),$(COMMA),$(ALL_CRATES)))) do ( \
		echo Building %%c in release mode... && \
		cd %%c && $(CARGO) build --release && cd $(ROOT_DIR) \
	)
	@for %%s in ($(subst $(SEP),/,$(subst $(SPACE),$(COMMA),$(ALL_SERVICES)))) do ( \
		echo Building %%s in production mode... && \
		cd %%s && $(NPM) install && $(NPM) run build && cd $(ROOT_DIR) \
	)
else
# Release build - Unix version
.PHONY: release
release:
	@echo Building all crates in release mode...
	@for crate in $(ALL_CRATES); do \
		echo "Building $$crate in release mode..."; \
		cd $$crate && $(CARGO) build --release && cd $(ROOT_DIR); \
	done
	@for service in $(ALL_SERVICES); do \
		echo "Building $$service in production mode..."; \
		cd $$service && $(NPM) install && $(NPM) run build && cd $(ROOT_DIR); \
	done
endif

# Test targets
.PHONY: test test-lodestone test-agent test-cli test-director test-forge test-orchestrator test-full

test: test-lodestone test-agent test-cli test-director test-forge test-orchestrator

test-lodestone:
	@echo Testing Lodestone...
	@cd $(LODESTONE) $(CMD_SEP) $(CARGO) test

test-agent:
	@echo Testing OmniAgent...
	@cd $(OMNI_AGENT) $(CMD_SEP) $(CARGO) test

test-cli:
	@echo Testing OmniCLI...
	@cd $(OMNI_CLI) $(CMD_SEP) $(CARGO) test

test-director:
	@echo Testing OmniDirector...
	@cd $(OMNI_DIRECTOR) $(CMD_SEP) $(CARGO) test

test-forge:
	@echo Testing OmniForge...
	@cd $(OMNI_FORGE) $(CMD_SEP) $(CARGO) test

test-orchestrator:
	@echo Testing OmniOrchestrator...
	@cd $(OMNI_ORCHESTRATOR) $(CMD_SEP) $(CARGO) test

test-full:
	@echo Running full integration tests...
ifeq ($(OS),Windows_NT)
	@if exist "tests\full-test.sh" ( \
		bash tests/full-test.sh \
	) else ( \
		echo Full test script not found. \
	)
else
	@if [ -f "tests/full-test.sh" ]; then \
		bash tests/full-test.sh; \
	else \
		echo "Full test script not found."; \
	fi
endif

# Clean targets - conditional for Windows/Unix
.PHONY: clean clean-all

ifeq ($(OS),Windows_NT)
clean:
	@echo Cleaning build artifacts...
	@for %%c in ($(subst $(SEP),/,$(subst $(SPACE),$(COMMA),$(ALL_CRATES)))) do ( \
		echo Cleaning %%c... && \
		cd %%c && $(CARGO) clean && cd $(ROOT_DIR) \
	)
	@for %%s in ($(subst $(SEP),/,$(subst $(SPACE),$(COMMA),$(ALL_SERVICES)))) do ( \
		echo Cleaning %%s... && \
		cd %%s && if exist node_modules $(RM_DIR) node_modules && if exist .next $(RM_DIR) .next && cd $(ROOT_DIR) \
	)

clean-all: clean
	@echo Deep cleaning everything...
	@if exist target $(RM_DIR) target
	@if exist Cargo.lock $(RM) Cargo.lock
	@for %%c in ($(subst $(SEP),/,$(subst $(SPACE),$(COMMA),$(ALL_CRATES)))) do ( \
		echo Deep cleaning %%c... && \
		cd %%c && if exist Cargo.lock $(RM) Cargo.lock && cd $(ROOT_DIR) \
	)
else
clean:
	@echo Cleaning build artifacts...
	@for crate in $(ALL_CRATES); do \
		echo "Cleaning $$crate..."; \
		cd $$crate && $(CARGO) clean && cd $(ROOT_DIR); \
	done
	@for service in $(ALL_SERVICES); do \
		echo "Cleaning $$service..."; \
		cd $$service && $(RM_DIR) node_modules .next 2>/dev/null || true && cd $(ROOT_DIR); \
	done

clean-all: clean
	@echo Deep cleaning everything...
	@$(RM_DIR) target 2>/dev/null || true
	@$(RM) Cargo.lock 2>/dev/null || true
	@for crate in $(ALL_CRATES); do \
		echo "Deep cleaning $$crate..."; \
		cd $$crate && $(RM) Cargo.lock 2>/dev/null || true && cd $(ROOT_DIR); \
	done
endif

# Documentation
.PHONY: docs

docs:
	@echo Generating documentation...
	@$(CARGO) doc --no-deps
	@echo Documentation available at target/doc/index.html

# Docker targets
.PHONY: docker-build docker-up docker-down

docker-build:
	@echo Building Docker images...
	@cd $(DOCKER_DIR) $(CMD_SEP) $(DOCKER_COMPOSE) build

docker-up:
	@echo Starting Docker Compose services...
	@cd $(DOCKER_DIR) $(CMD_SEP) $(DOCKER_COMPOSE) up -d

docker-down:
	@echo Stopping Docker Compose services...
	@cd $(DOCKER_DIR) $(CMD_SEP) $(DOCKER_COMPOSE) down

# Development tools - conditional for Windows/Unix
.PHONY: lint format dev-setup update-deps

ifeq ($(OS),Windows_NT)
lint:
	@echo Running linter on all crates...
	@for %%c in ($(subst $(SEP),/,$(subst $(SPACE),$(COMMA),$(ALL_CRATES)))) do ( \
		echo Linting %%c... && \
		cd %%c && $(CARGO) clippy && cd $(ROOT_DIR) \
	)

format:
	@echo Formatting code...
	@for %%c in ($(subst $(SEP),/,$(subst $(SPACE),$(COMMA),$(ALL_CRATES)))) do ( \
		echo Formatting %%c... && \
		cd %%c && $(CARGO) fmt && cd $(ROOT_DIR) \
	)

dev-setup:
	@echo Setting up development environment...
	@if exist "$(OMNI_DIRECTOR)\dev-setup.sh" ( \
		bash $(OMNI_DIRECTOR)/dev-setup.sh \
	) else if exist "$(OMNI_ORCHESTRATOR)\dev-setup.sh" ( \
		bash $(OMNI_ORCHESTRATOR)/dev-setup.sh \
	) else ( \
		echo Dev setup script not found \
	)

update-deps:
	@echo Updating dependencies for all crates...
	@for %%c in ($(subst $(SEP),/,$(subst $(SPACE),$(COMMA),$(ALL_CRATES)))) do ( \
		echo Updating dependencies for %%c... && \
		cd %%c && $(CARGO) update && cd $(ROOT_DIR) \
	)
else
lint:
	@echo Running linter on all crates...
	@for crate in $(ALL_CRATES); do \
		echo "Linting $$crate..."; \
		cd $$crate && $(CARGO) clippy && cd $(ROOT_DIR); \
	done

format:
	@echo Formatting code...
	@for crate in $(ALL_CRATES); do \
		echo "Formatting $$crate..."; \
		cd $$crate && $(CARGO) fmt && cd $(ROOT_DIR); \
	done

dev-setup:
	@echo Setting up development environment...
	@if [ -f "$(OMNI_DIRECTOR)/dev-setup.sh" ]; then \
		bash $(OMNI_DIRECTOR)/dev-setup.sh; \
	elif [ -f "$(OMNI_ORCHESTRATOR)/dev-setup.sh" ]; then \
		bash $(OMNI_ORCHESTRATOR)/dev-setup.sh; \
	else \
		echo "Dev setup script not found"; \
	fi

update-deps:
	@echo Updating dependencies for all crates...
	@for crate in $(ALL_CRATES); do \
		echo "Updating dependencies for $$crate..."; \
		cd $$crate && $(CARGO) update && cd $(ROOT_DIR); \
	done
endif

# Deployment targets
.PHONY: deploy-dev deploy-staging deploy-prod

deploy-dev:
	@echo Deploying to development environment...
	@# Add deployment commands here

deploy-staging:
	@echo Deploying to staging environment...
	@# Add deployment commands here

deploy-prod:
	@echo Deploying to production environment...
	@# Add deployment commands here

# Utility targets
.PHONY: check-env db-init backup

check-env:
	@echo Checking development environment...
	@echo Rust version: $(shell $(CARGO) --version)
	@echo Cargo version: $(shell $(CARGO) --version)
	@echo Node version: $(shell $(NODE) --version)
	@echo NPM version: $(shell $(NPM) --version)
	@echo Docker version: $(shell $(DOCKER) --version)
	@echo Docker Compose version: $(shell $(DOCKER_COMPOSE) --version)
	@echo OS: $(DETECTED_OS)

ifeq ($(OS),Windows_NT)
db-init:
	@echo Initializing database schema...
	@if exist "$(OMNI_ORCHESTRATOR)\sql\db_init.sql" ( \
		echo Found database initialization script \
		@rem Add commands to execute the SQL script here \
	) else ( \
		echo Database initialization script not found \
	)

backup:
	@echo Backing up data and configuration...
	@if not exist backups\config mkdir backups\config
	@if not exist backups\data mkdir backups\data
	@copy config.json backups\config\
	@rem Add more backup commands as needed
else
db-init:
	@echo Initializing database schema...
	@if [ -f "$(OMNI_ORCHESTRATOR)/sql/db_init.sql" ]; then \
		echo "Found database initialization script"; \
		# Add commands to execute the SQL script here \
	else \
		echo "Database initialization script not found"; \
	fi

backup:
	@echo Backing up data and configuration...
	@mkdir -p backups/config backups/data
	@cp config.json backups/config/
	@# Add more backup commands as needed
endif