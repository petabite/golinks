MODULE_PATH=github.com/petabite/golinks

.PHONY: all
all: golinks

.PHONY: golinks
golinks:
	go build -v  -o ./tmp/golinks ./cmd/golinks

.PHONY: run
run:
	go run $(MODULE_PATH)/cmd/golinks 

.PHONY: dev
dev:
	air

.PHONY: format
format:
	gofmt -l -s -w .
