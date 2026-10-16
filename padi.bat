@echo off
if "%~1"=="" (
    cargo run --quiet -- help
) else (
    cargo run --quiet -- %*
)
