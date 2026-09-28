# UNO App Makefile
# Operational commands for building, deploying, and managing the application

# =============================================================================
# CONFIGURATION
# =============================================================================

# Docker
DOCKER_REGISTRY ?= katson360
IMAGE_NAME ?= uno-app
IMAGE_TAG ?= latest
DOCKER_IMAGE = $(DOCKER_REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG)

# Kubernetes
NAMESPACE ?= uno-app
HELM_RELEASE ?= uno-app
HELM_CHART_PATH = infra/kubernetes/charts/uno-app

# Environment
ENV ?= local

# =============================================================================
# HELP
# =============================================================================

.PHONY: help
help: ## Show this help message
	@echo "UNO App - Operational Commands"
	@echo ""
	@echo "Usage: make [target] [VAR=value]"
	@echo ""
	@echo "Variables:"
	@echo "  DOCKER_REGISTRY  Docker registry (default: katson360)"
	@echo "  IMAGE_TAG        Image tag (default: latest)"
	@echo "  NAMESPACE        K8s namespace (default: uno-app)"
	@echo "  ENV              Environment: local, europe, india, americas (default: local)"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

# =============================================================================
# DOCKER - BUILD
# =============================================================================

.PHONY: docker-build
docker-build: ## Build Docker image
	docker build -t $(DOCKER_IMAGE) .

.PHONY: docker-build-local
docker-build-local: ## Build Docker image for local development
	docker build -f Dockerfile.local -t $(DOCKER_IMAGE) .

.PHONY: docker-build-minikube
docker-build-minikube: ## Build Docker image in minikube's Docker environment
	eval $$(minikube docker-env) && docker build -t $(DOCKER_IMAGE) .

# =============================================================================
# DOCKER - PUSH
# =============================================================================

.PHONY: docker-push
docker-push: ## Push Docker image to registry
	docker push $(DOCKER_IMAGE)

.PHONY: docker-login
docker-login: ## Login to Docker registry
	docker login

.PHONY: docker-build-push
docker-build-push: docker-build docker-push ## Build and push Docker image

# =============================================================================
# DOCKER - RUN
# =============================================================================

.PHONY: docker-run
docker-run: ## Run Docker container locally
	docker run --rm -p 3000:3000 --env-file .env $(DOCKER_IMAGE)

.PHONY: docker-shell
docker-shell: ## Shell into Docker container
	docker run --rm -it --entrypoint /bin/sh $(DOCKER_IMAGE)

# =============================================================================
# MINIKUBE
# =============================================================================

.PHONY: minikube-start
minikube-start: ## Start minikube cluster
	minikube start --cpus=4 --memory=8192 --driver=docker

.PHONY: minikube-stop
minikube-stop: ## Stop minikube cluster
	minikube stop

.PHONY: minikube-delete
minikube-delete: ## Delete minikube cluster
	minikube delete

.PHONY: minikube-dashboard
minikube-dashboard: ## Open minikube dashboard
	minikube dashboard

.PHONY: minikube-service
minikube-service: ## Open uno-app service in browser (minikube)
	minikube service $(HELM_RELEASE) -n $(NAMESPACE)

.PHONY: minikube-tunnel
minikube-tunnel: ## Start minikube tunnel for LoadBalancer services
	minikube tunnel

# =============================================================================
# HELM - SETUP
# =============================================================================

.PHONY: helm-deps
helm-deps: ## Update Helm chart dependencies
	cd $(HELM_CHART_PATH) && helm dependency update

.PHONY: helm-lint
helm-lint: ## Lint Helm chart
	helm lint $(HELM_CHART_PATH)

.PHONY: helm-template
helm-template: ## Render Helm templates (dry-run)
	helm template $(HELM_RELEASE) $(HELM_CHART_PATH) \
		-f $(HELM_CHART_PATH)/values.yaml \
		-f $(HELM_CHART_PATH)/values-strategy-$(ENV).yaml \
		-n $(NAMESPACE)

# =============================================================================
# HELM - DEPLOY
# =============================================================================

.PHONY: helm-deploy
helm-deploy: ## Deploy Helm chart (ENV=local|europe|india|americas)
	helm upgrade --install $(HELM_RELEASE) $(HELM_CHART_PATH) \
		-f $(HELM_CHART_PATH)/values.yaml \
		-f $(HELM_CHART_PATH)/values-strategy-$(ENV).yaml \
		-n $(NAMESPACE) \
		--create-namespace \
		--wait --timeout 5m

.PHONY: helm-deploy-local
helm-deploy-local: ## Deploy to local minikube with integrated PostgreSQL
	$(MAKE) helm-deploy ENV=local

.PHONY: helm-deploy-europe
helm-deploy-europe: ## Deploy Europe strategy (production)
	$(MAKE) helm-deploy ENV=europe

.PHONY: helm-deploy-india
helm-deploy-india: ## Deploy India strategy (production)
	$(MAKE) helm-deploy ENV=india

.PHONY: helm-deploy-americas
helm-deploy-americas: ## Deploy Americas strategy (production)
	$(MAKE) helm-deploy ENV=americas

.PHONY: helm-deploy-ha
helm-deploy-ha: ## Deploy with HA PostgreSQL (ENV required)
	helm upgrade --install $(HELM_RELEASE) $(HELM_CHART_PATH) \
		-f $(HELM_CHART_PATH)/values.yaml \
		-f $(HELM_CHART_PATH)/values-strategy-$(ENV).yaml \
		-f $(HELM_CHART_PATH)/values-database-ha.yaml \
		-n $(NAMESPACE) \
		--create-namespace \
		--wait --timeout 5m

# =============================================================================
# HELM - MANAGE
# =============================================================================

.PHONY: helm-status
helm-status: ## Show Helm release status
	helm status $(HELM_RELEASE) -n $(NAMESPACE)

.PHONY: helm-history
helm-history: ## Show Helm release history
	helm history $(HELM_RELEASE) -n $(NAMESPACE)

.PHONY: helm-rollback
helm-rollback: ## Rollback to previous Helm release
	helm rollback $(HELM_RELEASE) -n $(NAMESPACE)

.PHONY: helm-uninstall
helm-uninstall: ## Uninstall Helm release
	helm uninstall $(HELM_RELEASE) -n $(NAMESPACE)

.PHONY: helm-values
helm-values: ## Show deployed Helm values
	helm get values $(HELM_RELEASE) -n $(NAMESPACE)

# =============================================================================
# KUBERNETES - NAMESPACE & SECRETS
# =============================================================================

.PHONY: k8s-namespace
k8s-namespace: ## Create namespace
	kubectl create namespace $(NAMESPACE) --dry-run=client -o yaml | kubectl apply -f -

.PHONY: k8s-secrets-local
k8s-secrets-local: ## Create secrets for local development
	@echo "Creating local development secrets..."
	kubectl create secret generic admin-credentials \
		--namespace $(NAMESPACE) \
		--from-literal=api_key="local-dev-key" \
		--from-literal=client_id="local-admin" \
		--from-literal=secret_key="local-secret" \
		--dry-run=client -o yaml | kubectl apply -f -
	kubectl create secret generic r2-credentials \
		--namespace $(NAMESPACE) \
		--from-literal=access_key_id="local" \
		--from-literal=secret_access_key="local" \
		--from-literal=endpoint="http://localhost:9000" \
		--from-literal=bucket_name="local" \
		--dry-run=client -o yaml | kubectl apply -f -

.PHONY: k8s-secrets-prod
k8s-secrets-prod: ## Create secrets for production (requires env vars)
	@echo "Creating production secrets..."
	@test -n "$(ADMIN_API_KEY)" || (echo "ADMIN_API_KEY not set" && exit 1)
	@test -n "$(DATABASE_URL)" || (echo "DATABASE_URL not set" && exit 1)
	kubectl create secret generic admin-credentials \
		--namespace $(NAMESPACE) \
		--from-literal=api_key="$(ADMIN_API_KEY)" \
		--from-literal=client_id="$(ADMIN_CLIENT_ID)" \
		--from-literal=secret_key="$(ADMIN_SECRET_KEY)" \
		--dry-run=client -o yaml | kubectl apply -f -
	kubectl create secret generic db-credentials \
		--namespace $(NAMESPACE) \
		--from-literal=url="$(DATABASE_URL)" \
		--dry-run=client -o yaml | kubectl apply -f -
	kubectl create secret generic r2-credentials \
		--namespace $(NAMESPACE) \
		--from-literal=access_key_id="$(R2_ACCESS_KEY)" \
		--from-literal=secret_access_key="$(R2_SECRET_KEY)" \
		--from-literal=endpoint="$(R2_ENDPOINT)" \
		--from-literal=bucket_name="$(R2_BUCKET)" \
		--dry-run=client -o yaml | kubectl apply -f -

# =============================================================================
# KUBERNETES - STATUS
# =============================================================================

.PHONY: k8s-pods
k8s-pods: ## List pods
	kubectl get pods -n $(NAMESPACE)

.PHONY: k8s-services
k8s-services: ## List services
	kubectl get svc -n $(NAMESPACE)

.PHONY: k8s-all
k8s-all: ## List all resources
	kubectl get all -n $(NAMESPACE)

.PHONY: k8s-describe
k8s-describe: ## Describe all pods
	kubectl describe pods -n $(NAMESPACE)

.PHONY: k8s-events
k8s-events: ## Show recent events
	kubectl get events -n $(NAMESPACE) --sort-by='.lastTimestamp' | tail -20

# =============================================================================
# KUBERNETES - LOGS
# =============================================================================

.PHONY: k8s-logs
k8s-logs: ## Show uno-app logs
	kubectl logs -n $(NAMESPACE) -l app=uno-app --tail=100

.PHONY: k8s-logs-follow
k8s-logs-follow: ## Follow uno-app logs
	kubectl logs -n $(NAMESPACE) -l app=uno-app -f

.PHONY: k8s-logs-db
k8s-logs-db: ## Show PostgreSQL logs
	kubectl logs -n $(NAMESPACE) uno-db-0 --tail=100

# =============================================================================
# KUBERNETES - DEBUG
# =============================================================================

.PHONY: k8s-shell
k8s-shell: ## Shell into uno-app pod
	kubectl exec -it -n $(NAMESPACE) deploy/uno-app -- /bin/sh

.PHONY: k8s-shell-db
k8s-shell-db: ## Shell into PostgreSQL pod
	kubectl exec -it -n $(NAMESPACE) uno-db-0 -- bash

.PHONY: k8s-psql
k8s-psql: ## Connect to PostgreSQL
	kubectl exec -it -n $(NAMESPACE) uno-db-0 -- bash -c 'PGPASSWORD="$$POSTGRES_PASSWORD" psql -U postgres -d uno'

.PHONY: k8s-port-forward
k8s-port-forward: ## Port forward uno-app to localhost:3000
	kubectl port-forward -n $(NAMESPACE) svc/uno-app 3000:3000

.PHONY: k8s-port-forward-db
k8s-port-forward-db: ## Port forward PostgreSQL to localhost:5432
	kubectl port-forward -n $(NAMESPACE) svc/uno-db 5432:5432

# =============================================================================
# KUBERNETES - RESTART
# =============================================================================

.PHONY: k8s-restart
k8s-restart: ## Restart uno-app deployment
	kubectl rollout restart deployment/uno-app -n $(NAMESPACE)

.PHONY: k8s-restart-db
k8s-restart-db: ## Restart PostgreSQL (StatefulSet)
	kubectl delete pod -n $(NAMESPACE) uno-db-0

# =============================================================================
# HEALTH CHECKS
# =============================================================================

.PHONY: health
health: ## Check application health
	kubectl exec -n $(NAMESPACE) deploy/uno-app -- curl -s http://localhost:3000/api/v1/health

.PHONY: health-db
health-db: ## Check database connection
	kubectl exec -n $(NAMESPACE) uno-db-0 -- bash -c 'PGPASSWORD="$$POSTGRES_PASSWORD" psql -U postgres -d uno -c "SELECT 1;"'

# =============================================================================
# CLEANUP
# =============================================================================

.PHONY: clean
clean: ## Clean up local development resources
	-helm uninstall $(HELM_RELEASE) -n $(NAMESPACE)
	-kubectl delete pvc --all -n $(NAMESPACE)
	-kubectl delete namespace $(NAMESPACE)

.PHONY: clean-docker
clean-docker: ## Remove local Docker images
	-docker rmi $(DOCKER_IMAGE)
	-docker system prune -f

# =============================================================================
# DEVELOPMENT WORKFLOWS
# =============================================================================

.PHONY: dev-setup
dev-setup: minikube-start helm-deps k8s-namespace k8s-secrets-local ## Setup local development environment
	@echo "Local development environment ready!"
	@echo "Run 'make dev-deploy' to deploy the application"

.PHONY: dev-deploy
dev-deploy: helm-deploy-local ## Deploy to local development
	@echo "Waiting for pods to be ready..."
	@sleep 10
	@$(MAKE) k8s-pods
	@$(MAKE) health

.PHONY: dev-rebuild
dev-rebuild: docker-build-minikube k8s-restart ## Rebuild image and restart pods (minikube)
	@echo "Waiting for rollout..."
	kubectl rollout status deployment/uno-app -n $(NAMESPACE)

.PHONY: dev-logs
dev-logs: k8s-logs-follow ## Follow development logs

.PHONY: dev-clean
dev-clean: clean minikube-stop ## Clean up development environment

# =============================================================================
# PRODUCTION WORKFLOWS
# =============================================================================

.PHONY: prod-deploy
prod-deploy: ## Deploy to production (requires ENV and secrets)
	@test -n "$(ENV)" || (echo "ENV not set (europe|india|americas)" && exit 1)
	@test "$(ENV)" != "local" || (echo "Use 'make dev-deploy' for local" && exit 1)
	$(MAKE) helm-deploy ENV=$(ENV)

.PHONY: prod-status
prod-status: k8s-pods k8s-services health ## Check production status

# =============================================================================
# CI/CD
# =============================================================================

.PHONY: ci-build
ci-build: docker-build docker-push ## CI: Build and push image

.PHONY: ci-deploy
ci-deploy: helm-deps helm-deploy ## CI: Deploy application

# =============================================================================
# SHORTCUTS
# =============================================================================

.PHONY: up
up: dev-setup dev-deploy ## Quick start: setup and deploy locally

.PHONY: down
down: dev-clean ## Quick stop: clean everything

.PHONY: logs
logs: k8s-logs-follow ## Shortcut: follow logs

.PHONY: ps
ps: k8s-pods ## Shortcut: list pods

.PHONY: status
status: k8s-all health ## Shortcut: show status
