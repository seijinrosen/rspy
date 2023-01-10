test:
	cargo check
	cargo test -- --nocapture --color=always
	cargo clippy

after-develop-merged:
	git switch main
	git pull --prune
	git branch --delete develop
	git switch --create develop

bump-version:
	cargo run --example bump_version

clean:
	rm -r node_modules/
	rm -r target/

coverage:
	cargo tarpaulin

doc:
	cargo doc -Zunstable-options -Zrustdoc-scrape-examples --open

init:
	poetry install
	pre-commit install
	direnv allow

lint:
	pre-commit run --all-files

open:
	gh repo view --web

switch:
	git switch --create develop
