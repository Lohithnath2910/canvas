param(
    [string]$Out = "shot.png",
    [int]$Settle = 2500,
    [string[]]$AppArgs = @(),
    [string]$Title = "Canvas",
    [switch]$Arrange
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
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] public static extern bool MoveWindow(IntPtr h, int x, int y, int w, int t, bool repaint);
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
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
    $wins = [Win]::VisibleTitled($proc.Id, $Title)
    if ($wins.Count -eq 0) { throw "no visible window titled $Title for pid $($proc.Id)" }

    $shell.MinimizeAll()
    Start-Sleep -Milliseconds 900
    foreach ($h in $wins) { [void][Win]::ShowWindow($h, 9); [void][Win]::ShowWindow($h, 5) }
    Start-Sleep -Milliseconds 700

    if ($Arrange) {
        $x = 60
        foreach ($h in $wins) {
            $r = New-Object Win+RECT
            [void][Win]::GetWindowRect($h, [ref]$r)
            [void][Win]::MoveWindow($h, $x, 80, $r.R - $r.L, $r.B - $r.T, $true)
            $x += ($r.R - $r.L) + 40
        }
        Start-Sleep -Milliseconds 500
    }

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
