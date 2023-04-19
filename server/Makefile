MODULE_PATH=github.com/petabite/golinks

.PHONY: all
all: golinks

.PHONY: golinks
golinks:
	go build -v  -o ./tmp/golinks ./cmd/golinks

.PHONY: run
run:
	ENV=dev go run $(MODULE_PATH)/cmd/golinks 

.PHONY: install
install: build-client
	go install $(MODULE_PATH)/cmd/golinks

.PHONY: build-client
build-client:
	make -C ../client build
	rm -rf ./internal/server/static
	cp -r ../client/dist ./internal/server/static

.PHONY: dev
dev:
	air

.PHONY: format
format:
	gofmt -l -s -w .
