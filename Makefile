FMT_TITLE='\\033[7\;1m'
FMT_PRIMARY='\\033[36m'
FMT_END='\\033[0m'

.PHONY: help
help:
	@printf -- "                              ASCACOU\n"
	@printf -- "----------------------------------------------------------------------\n"
	@printf "Usage: make \033[36m<target>\033[0m\n"
	@printf "\n"
	@printf "\033[7;1m %s \033[0m\n" "OPTIONAL VARIABLES"
	@awk ' \
		BEGIN {FS = " \\?=.*?## "} \
		/^[a-zA-Z_-]+ \?=.*?## .*$$/ {printf "  $(FMT_PRIMARY)%-30s$(FMT_END) %s\n", $$1, $$2} \
	' $(MAKEFILE_LIST)
	@awk ' \
			BEGIN {FS = ":.*##"} \
			/^[a-zA-Z0-9_-]+:.*?##/ { printf "  $(FMT_PRIMARY)%-30s$(FMT_END) %s\n", $$1, $$2 } \
			/^##@/ { printf "\n$(FMT_TITLE) %s $(FMT_END)\n", substr($$0, 5) } \
	' $(MAKEFILE_LIST)

.PHONY: wasm
wasm: ## Generate wasm package under pkg/
	wasm-pack build wasm --out-dir ../pkg

npm: wasm ## Publish to npm
	wasm-pack publish
