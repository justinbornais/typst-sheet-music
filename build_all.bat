@echo off
setlocal enabledelayedexpansion

for %%I in ("%~dp0.") do set "ROOT=%%~fI"
cd /d "%ROOT%"

echo Building WASM plugin...
pushd wasm
cargo build --target wasm32-unknown-unknown --release
if errorlevel 1 (
  echo Failed to build WASM plugin
  popd
  exit /b 1
)
popd
copy /Y "wasm\target\wasm32-unknown-unknown\release\scorify_wasm.wasm" "scorify_wasm.wasm" >nul

for %%D in (examples tests) do (
  pushd "%%D"
  for /r %%F in (*.typ) do (
    echo Compiling %%F
    typst compile "%%F" --font-path "%ROOT%\fonts" --root "%ROOT%"
    if errorlevel 1 (
      echo Failed to compile %%F
      popd
      exit /b 1
    )
  )
  popd
)

echo All Typst files compiled successfully.
exit /b 0
