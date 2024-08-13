#! /usr/bin/bash
cd frontend/ && npx prettier --write "**/*.astro" && npx astro build && cd .. && cargo run