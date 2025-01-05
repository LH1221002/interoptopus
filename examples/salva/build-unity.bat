REM Step 1: Build everything except cs_generator
cargo build --release || goto :error

REM Step 2: Build cs_generator
cd cs_generator || goto :error
cargo build -vv --release --features build-script || goto :error

echo Build completed successfully.
exit /b 0

:error
echo Build failed.
exit /b 1