param(
  [string]$TargetFile = "tests/test-render-basic.typ"
)

$ErrorActionPreference = "Stop"

function Measure-Step {
  param(
    [string]$Label,
    [scriptblock]$Action
  )

  $result = Measure-Command $Action
  [pscustomobject]@{
    Step = $Label
    Milliseconds = [math]::Round($result.TotalMilliseconds, 3)
    Seconds = [math]::Round($result.TotalSeconds, 3)
  }
}

$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Push-Location $repoRoot

try {
  $results = @(
    Measure-Step -Label "build_all.bat" -Action { & .\build_all.bat | Out-Null }
    Measure-Step -Label $TargetFile -Action { & typst compile $TargetFile --font-path fonts --root . | Out-Null }
  )

  $results | Format-Table -AutoSize
}
finally {
  Pop-Location
}