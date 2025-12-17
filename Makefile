FMT_TITLE='\\033[7\;1m'
FMT_PRIMARY='\\033[36m'
FMT_END='\\033[0m'

COMMON_DEPS := Cargo.toml Cargo.lock

.PHONY: help
help:
	@printf -- "                              ASCACOU\n"
	@printf -- "----------------------------------------------------------------------\n"
	@printf "Usage: make \033[36m<target>\033[0m\n"
	@printf "\n"
	@awk ' \
			BEGIN {FS = ":.*##"} \
			/^[a-zA-Z0-9_-]+:.*?##/ { printf "  $(FMT_PRIMARY)%-30s$(FMT_END) %s\n", $$1, $$2 } \
			/^##@/ { printf "\n$(FMT_TITLE) %s $(FMT_END)\n", substr($$0, 5) } \
	' $(MAKEFILE_LIST)

##@ Development

.PHONY: bitfiddle
bitfiddle: ## Run bitfiddle server for interactive development
	open http://localhost:8000/bitfiddle.html
	python -m http.server 8000

##@ Publishing

.PHONY: wasm
wasm: ## Generate wasm package under pkg/
	wasm-pack build wasm --target web --out-file minicou --out-dir ../pkg

npm: wasm ## Publish to npm
	wasm-pack publish

pypi: ## Publish to pypi
	cd py && maturin publish
