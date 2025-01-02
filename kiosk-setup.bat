@echo off
:: Requires administrative privileges
if not "%~1"=="" (
    echo Script already elevated.
) else (
    :: Relaunch script with administrative privileges
    echo Requesting administrative privileges...
    powershell -Command "Start-Process '%~f0' -ArgumentList 'elevated' -Verb runAs"
    exit
)

:: Disable desktop icons
echo Disabling desktop icons...
reg add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v HideIcons /t REG_DWORD /d 1 /f

:: Set wallpaper to solid black
echo Setting wallpaper to solid black...
reg add "HKEY_CURRENT_USER\Control Panel\Desktop" /v Wallpaper /t REG_SZ /d "" /f
reg add "HKEY_CURRENT_USER\Control Panel\Colors" /v Background /t REG_SZ /d "0 0 0" /f
RUNDLL32.EXE user32.dll,UpdatePerUserSystemParameters

:: Disable visual effects
echo Disabling visual effects...
reg add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects" /v VisualFXSetting /t REG_DWORD /d 2 /f
reg add "HKEY_CURRENT_USER\Control Panel\Desktop\WindowMetrics" /v MinAnimate /t REG_SZ /d 0 /f

:: Unpin all items from taskbar
echo Unpinning all items from taskbar...
powershell -Command "& {$taskbarItems = ((New-Object -ComObject Shell.Application).NameSpace(0).Items() | Where-Object {$_.IsShortcut -and $_.Path -like '*Taskbar*'}).InvokeVerb('Unpin from taskbar')}"

:: Set taskbar to auto-hide
echo Setting taskbar to auto-hide...
reg add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3" /v Settings /t REG_BINARY /d 0000000028010000000000001000000003010000 /f
taskkill /im explorer.exe /f & start explorer.exe

:: Remove taskbar search input and widgets
echo Removing taskbar search input and widgets...
reg add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Search" /v SearchboxTaskbarMode /t REG_DWORD /d 0 /f
reg add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v ShowTaskViewButton /t REG_DWORD /d 0 /f
taskkill /im explorer.exe /f & start explorer.exe

:: Set up AutoLogon
echo Configuring AutoLogon...
reg add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon" /v AutoAdminLogon /t REG_SZ /d 1 /f
reg add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon" /v DefaultUserName /t REG_SZ /d "Mardens" /f
reg add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon" /v DefaultPassword /t REG_SZ /d "mardens" /f

:: Downlaod the Kiosk Application]
echo Downloading Kiosk Application...
curl -o "%USERPROFILE%\Desktop\KioskApp.exe" "https://github.com/Mardens-Inc/Kiosk/releases/latest/download/service-desk-kiosk.exe"


:: Inform the user
echo Script completed successfully. The computer will now restart.
shutdown /r /t 0
pause