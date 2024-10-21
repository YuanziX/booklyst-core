pull:
	@git pull

build:
	@go build -o bin/booklyst-core

run: build
	@./bin/booklyst-core

status:
	@sudo systemctl status booklyst-core

serve: pull build
	@sudo systemctl restart booklyst-core nginx
	@sleep 1
	@$(MAKE) status
