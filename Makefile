dev:
	@echo 👀 watching your app
	cargo watch -q -c -w src/ -x "run"
	@echo dev mode closed successfully.

build:
	@echo building your app for production
	cargo build --release
	@echo successfully build your app.