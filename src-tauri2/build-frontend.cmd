@echo off
cd /d "%~dp0"
if exist websrc rmdir /s /q websrc
mkdir websrc
cd /d "%~dp0..\cube_shuffle-wasm"
trunk build --release --dist "%~dp0websrc"
