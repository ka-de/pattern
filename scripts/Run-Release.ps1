# Define a function to colorize each line based on the log level
function Colorize-Output {
	param([string]$Text)

	if ($Text -match "DEBUG") {
		Write-Host $Text -ForegroundColor Magenta
	}
	elseif ($Text -match "INFO") {
		Write-Host $Text -ForegroundColor Blue
	}
	elseif ($Text -match "ERROR") {
		Write-Host $Text -ForegroundColor Red
	}
	elseif ($Text -match "WARN") {
		Write-Host $Text -ForegroundColor Yellow
	}
	elseif ($Text -match "TRACE") {
		Write-Host $Text -ForegroundColor Cyan
	}
	else {
		Write-Host $Text
	}
}

# Run the command, filter the output, and pipe it to the colorizing function
cargo run --release 2>&1 | Out-String -Stream | Where-Object { $_ -notmatch "ID3D12Device::CreateCommittedResource:" -and $_ -notmatch "Live Object at" } | ForEach-Object {
	Colorize-Output $_
}
