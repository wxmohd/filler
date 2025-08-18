# Filler AI Audit Test Script
Write-Host "=== Filler AI Audit Test Suite ===" -ForegroundColor Green

# Build the AI
Write-Host "Building AI..." -ForegroundColor Yellow
cargo build --release --bin filler_ai
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

# Copy to solution directory as expected by audit
if (!(Test-Path "solution")) { New-Item -ItemType Directory -Path "solution" }
Copy-Item "target\release\filler_ai.exe" "solution\filler_ai.exe"
Write-Host "AI built and copied to solution/" -ForegroundColor Green

# Test the AI with basic input
Write-Host "Testing AI functionality..." -ForegroundColor Yellow

$testInput = @'
$$$ exec p1 : [solution/filler_ai.exe]
Anfield 5 5:
000 .....
001 .....
002 ..O..
003 .....
004 .....
Piece 2 1:
OO
'@

try {
    $output = $testInput | .\solution\filler_ai.exe
    Write-Host "AI Output: $output" -ForegroundColor Cyan
    
    if ($output -match "^\d+ \d+$") {
        Write-Host "Output format is correct" -ForegroundColor Green
        
        Write-Host ""
        Write-Host "=== Audit Commands Ready ===" -ForegroundColor Green
        Write-Host "Your AI is ready for audit testing." -ForegroundColor White
        Write-Host ""
        Write-Host "To test with Docker (when available):" -ForegroundColor Cyan
        Write-Host "1. docker build -t filler-game ." -ForegroundColor White
        Write-Host "2. docker run -it filler-game" -ForegroundColor White
        Write-Host "3. ./game_engine -f maps/map00 -p1 solution/filler_ai -p2 robots/wall_e" -ForegroundColor White
        Write-Host "4. ./game_engine -f maps/map01 -p1 solution/filler_ai -p2 robots/h2_d2" -ForegroundColor White
        Write-Host "5. ./game_engine -f maps/map02 -p1 solution/filler_ai -p2 robots/bender" -ForegroundColor White
        Write-Host ""
        Write-Host "AI should win at least 4/5 games against each opponent." -ForegroundColor Yellow
        
    } else {
        Write-Host "Invalid output format" -ForegroundColor Red
    }
} catch {
    Write-Host "Error testing AI: $_" -ForegroundColor Red
}

Write-Host "Test Complete" -ForegroundColor Green
