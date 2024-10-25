pull:
	@git pull

build:
	@go build -o bin/booklyst-core

run: build
	@./bin/booklyst-core

status:
	@sudo systemctl status booklyst-core

migrate:
	@~/go/bin/goose sqlite3 -dir sql/migrations app.db up

serve: pull build migrate
	@sudo systemctl restart booklyst-core nginx
	@sleep 1
	@$(MAKE) status
