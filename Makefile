.PHONY: backend frontend deploy all

# Build and update backend
backend:
	@echo "=== Backend: Building and generating IDL with Anchor ==="
	cd backend/idl/src && cargo clean && anchor build --skip-lint
	cd backend/idl && make idl
	@echo "=== Copying generated IDL from backend to frontend ==="
	cp -f backend/idl/code_vm.json frontend/src/idl/code_vm.json

# Build and run the frontend
frontend:
	@echo "=== Frontend: Installing packages, building, and starting ==="
	cd frontend && npm install
	cd frontend && npm run build
	cd frontend && npm start

# Deploy the backend (smart contract) to the desired Solana cluster
deploy:
	@echo "=== Deploying backend smart contract ==="
	cd backend && anchor deploy
	@echo "Deploy your frontend via Vercel or your chosen hosting provider."

# "all" builds both backend and frontend
all: backend frontend