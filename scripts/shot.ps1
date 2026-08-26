param(
    [string]$Out = "shot.png",
    [int]$Settle = 2500,
    [string[]]$AppArgs = @()
)

$ErrorActionPreference = "Stop"

Add-Type -AssemblyName System.Drawing, System.Windows.Forms
Add-Type @'
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

public class Win {
    [DllImport("shcore.dll")] public static extern int SetProcessDpiAwareness(int v);
    [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int cmd);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
    [DllImport("user32.dll")] public static extern int GetWindowTextLength(IntPtr h);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)]
    public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
    [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc cb, IntPtr p);
    public delegate bool EnumProc(IntPtr h, IntPtr p);

    public static List<IntPtr> VisibleTitled(uint pid, string title) {
        var hits = new List<IntPtr>();
        EnumWindows((h, p) => {
            uint wp; GetWindowThreadProcessId(h, out wp);
            if (wp != pid || !IsWindowVisible(h)) return true;
            int n = GetWindowTextLength(h);
            if (n == 0) return true;
            var sb = new StringBuilder(n + 1);
            GetWindowText(h, sb, sb.Capacity);
            if (sb.ToString() == title) hits.Add(h);
            return true;
        }, IntPtr.Zero);
        return hits;
    }
}
'@

[void][Win]::SetProcessDpiAwareness(2)

Get-Process canvas-app -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Milliseconds 400

$exe = Join-Path $PSScriptRoot "..\target\debug\canvas-app.exe"
if (-not (Test-Path $exe)) { throw "build it first: cargo build -p canvas-app" }

$start = @{ FilePath = $exe; PassThru = $true }
if ($AppArgs.Count -gt 0) { $start.ArgumentList = $AppArgs }
$proc = Start-Process @start
Start-Sleep -Milliseconds $Settle

$shell = New-Object -ComObject Shell.Application
try {
    $wins = [Win]::VisibleTitled($proc.Id, "Canvas")
    if ($wins.Count -eq 0) { throw "no visible window titled Canvas for pid $($proc.Id)" }

    $shell.MinimizeAll()
    Start-Sleep -Milliseconds 900
    foreach ($h in $wins) { [void][Win]::ShowWindow($h, 9); [void][Win]::ShowWindow($h, 5) }
    Start-Sleep -Milliseconds 700

    $vs = [System.Windows.Forms.SystemInformation]::VirtualScreen
    $bmp = New-Object System.Drawing.Bitmap $vs.Width, $vs.Height
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.CopyFromScreen($vs.X, $vs.Y, 0, 0, $bmp.Size)
    $g.Dispose()
    $path = if ([System.IO.Path]::IsPathRooted($Out)) { $Out } else { Join-Path (Get-Location) $Out }
    $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
    Write-Output "$path  $($vs.Width)x$($vs.Height)  windows=$($wins.Count)"
}
finally {
    $shell.UndoMinimizeALL()
    Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
}
