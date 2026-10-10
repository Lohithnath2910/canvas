param(
    [string]$Out = "hotkey.png",
    [string]$Type = "",
    [switch]$Enter,
    [int]$Settle = 2500
)

$ErrorActionPreference = "Stop"
Add-Type -AssemblyName System.Drawing, System.Windows.Forms
Add-Type @'
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
public class Hk {
    [DllImport("shcore.dll")] public static extern int SetProcessDpiAwareness(int v);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
    [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
    [DllImport("user32.dll")] public static extern int GetWindowTextLength(IntPtr h);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)]
    public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
    [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc cb, IntPtr p);
    public delegate bool EnumProc(IntPtr h, IntPtr p);

    public static IntPtr FirstOther(uint skipPid) {
        IntPtr hit = IntPtr.Zero;
        EnumWindows((h, p) => {
            uint wp; GetWindowThreadProcessId(h, out wp);
            if (wp == skipPid || !IsWindowVisible(h)) return true;
            int n = GetWindowTextLength(h);
            if (n < 3) return true;
            var sb = new StringBuilder(n + 1);
            GetWindowText(h, sb, sb.Capacity);
            if (sb.ToString() == "Canvas") return true;
            hit = h;
            return false;
        }, IntPtr.Zero);
        return hit;
    }
}
'@
[void][Hk]::SetProcessDpiAwareness(2)

Get-Process canvas-app -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Milliseconds 400

$exe = Join-Path $PSScriptRoot "..\target\debug\canvas-app.exe"
if (-not (Test-Path $exe)) { throw "build it first" }
$app = Start-Process -FilePath $exe -PassThru
Start-Sleep -Milliseconds $Settle

$other = [Hk]::FirstOther([uint32]$app.Id)
if ($other -eq [IntPtr]::Zero) { throw "no other visible window to hand the foreground to" }
[void][Hk]::SetForegroundWindow($other)
Start-Sleep -Milliseconds 900
if ([Hk]::GetForegroundWindow() -eq [IntPtr]::Zero) { throw "no foreground window at all" }
$before = [Hk]::GetForegroundWindow()
Write-Output "foreground before Ctrl+K: $before (canvas pid $($app.Id))"

try {
    [System.Windows.Forms.SendKeys]::SendWait("^k")
    Start-Sleep -Milliseconds 1200
    if ($Type -ne "") {
        [System.Windows.Forms.SendKeys]::SendWait($Type)
        Start-Sleep -Milliseconds 800
    }
    if ($Enter) {
        [System.Windows.Forms.SendKeys]::SendWait("{ENTER}")
        Start-Sleep -Milliseconds 1200
    }

    $vs = [System.Windows.Forms.SystemInformation]::VirtualScreen
    $bmp = New-Object System.Drawing.Bitmap $vs.Width, $vs.Height
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.CopyFromScreen($vs.X, $vs.Y, 0, 0, $bmp.Size)
    $g.Dispose()
    $path = if ([System.IO.Path]::IsPathRooted($Out)) { $Out } else { Join-Path (Get-Location) $Out }
    $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
    Write-Output "$path  $($vs.Width)x$($vs.Height)"
}
finally {
    Stop-Process -Id $app.Id -Force -ErrorAction SilentlyContinue
}
