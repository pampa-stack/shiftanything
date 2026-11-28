@echo off
setlocal
call C:\VSCommunity\VC\Auxiliary\Build\vcvars64.bat
if errorlevel 1 exit /b %errorlevel%
set "CARGO_HOME=C:\sq\cargo"
set "RUSTUP_HOME=C:\sq\rustup"
set "CARGO_TARGET_DIR=C:\sq\target"
set "CARGO_BUILD_JOBS=6"
set "CARGO_NET_RETRY=2"
set "PATH=C:\sq\cargo\bin;C:\sq\tauri;%PATH%"
cd /d C:\sq\src-bbfecd2-soundcraft
cargo.exe test --offline --locked --target x86_64-pc-windows-msvc -p artcraft-launcher-core --lib
if errorlevel 1 exit /b %errorlevel%
cargo.exe test --offline --locked --target x86_64-pc-windows-msvc -p artcraft-launcher-native --lib
if errorlevel 1 exit /b %errorlevel%
cargo.exe test --offline --locked --release --target x86_64-pc-windows-msvc -p shiftanything-tauri-proof --features windows-lifecycle installation_evidence_is_scoped_to_catalog_app_and_target
if errorlevel 1 exit /b %errorlevel%
cargo.exe build --offline --locked --release --target x86_64-pc-windows-msvc -p shiftanything-tauri-proof --features windows-lifecycle
if errorlevel 1 exit /b %errorlevel%
echo CANDIDATE_BUILD_PASS
