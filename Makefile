VERSION := $(shell cargo pkgid | awk -F'@' '{print $$NF}')
export VERSION

.PHONY:
install:
	cargo install --path .

.PHONY:
format:
	cargo clippy --fix --allow-dirty
	cargo fmt

.PHONY:
release:
	git tag -a v$(VERSION) -m "release v$(VERSION)"
	git push origin $(VERSION)
	cargo publish