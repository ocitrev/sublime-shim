cargo install --path "%~dp0smerge"
@if errorlevel 1 exit /b %errorlevel%

cargo install --path "%~dp0subl"
@if errorlevel 1 exit /b %errorlevel%
