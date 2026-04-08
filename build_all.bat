@echo off
setlocal enabledelayedexpansion

for %%I in ("%~dp0.") do set "ROOT=%%~fI"
cd /d "%ROOT%"

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
