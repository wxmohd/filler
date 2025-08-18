# PowerShell script to test the Filler AI
Write-Host "=== Testing Filler AI ===" -ForegroundColor Green

# Build the AI
Write-Host "Building AI..." -ForegroundColor Yellow
cargo build --release --bin filler_ai
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "Build successful" -ForegroundColor Green

# Test with the existing test input
Write-Host "Testing with test_input.txt..." -ForegroundColor Yellow
$output = Get-Content test_input.txt | .\target\release\filler_ai.exe
Write-Host "AI Output: $output" -ForegroundColor Cyan

# Validate output format
if ($output -match "^\d+ \d+$") {
    Write-Host "Output format is correct" -ForegroundColor Green
} else {
    Write-Host "Invalid output format" -ForegroundColor Red
}

Write-Host "Test Complete" -ForegroundColor Green
