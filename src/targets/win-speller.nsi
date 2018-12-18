!define COMPANY_NAME "Divvun"
!define APP_ID "{app_id}"
!define APP_URL "http://divvun.no/"
!define APP_NAME "{app_name}"
!define VERSION "{version}"

Name "${{APP_NAME}}"
Unicode true
SetCompressor /SOLID lzma

!define MULTIUSER_EXECUTIONLEVEL Highest
!define MULTIUSER_MUI
!define MULTIUSER_INSTALLMODE_COMMANDLINE
!define MULTIUSER_USE_PROGRAMFILES64

!define MULTIUSER_INSTALLMODE_INSTDIR WinDivvun\Spellers\{bcp47code}

!include MultiUser.nsh
!include MUI2.nsh
!include x64.nsh

!insertmacro MUI_PAGE_WELCOME
!insertmacro MULTIUSER_PAGE_INSTALLMODE
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE English

!ifdef INNER
    !echo "Generating Uninstaller"
    OutFile "tempinstaller.exe"
    SetCompress off
!else
    ; create uninstall
    !makensis '/DINNER "${{__FILE__}}"' = 0

    !system "tempinstaller.exe" = 2

    ; sign the uninstaller
    !system '{sign_tool_uninstaller}' = 0

    OutFile install.exe

    !finalize '{sign_tool}'
!endif

Function .onInit
  ${{If}} ${{RunningX64}}
    SetRegView 64
  ${{EndIf}}
  !insertmacro MULTIUSER_INIT

  !ifdef INNER
    System::Call "kernel32::GetCurrentDirectory(i ${{NSIS_MAX_STRLEN}}, t .r0)"
    WriteUninstaller "$0\uninstall.exe"
    Quit
  !endif
FunctionEnd

Section "Installer Section"
  SetOutPath $INSTDIR

  ; copy spellchecker
  File /oname={bcp47code}.zhfst speller.zhfst

  !ifndef INNER
    ; grant access to application packages
    nsExec::Exec 'icacls "$INSTDIR" /grant "ALL APPLICATION PACKAGES":R /T'

    ; update uninstall information
    # Registry information for add/remove programs
    WriteRegStr SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "DisplayName" "${{APP_NAME}}"
    WriteRegStr SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "UninstallString" "$\"$INSTDIR\uninstall.exe$\""
    WriteRegStr SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "QuietUninstallString" "$\"$INSTDIR\uninstall.exe$\" /S"
    WriteRegStr SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "InstallLocation" "$\"$INSTDIR$\""
    # WriteRegStr SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "DisplayIcon" "$\"$INSTDIR\logo.ico$\""
    WriteRegStr SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "Publisher" "${{COMPANY_NAME}}"
    WriteRegStr SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "HelpLink" "${{APP_URL}}"
    # WriteRegStr SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "URLUpdateInfo" "$\"${{UPDATEURL}}$\""
    # WriteRegStr SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "URLInfoAbout" "$\"${{ABOUTURL}}$\""
    WriteRegStr SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "DisplayVersion" "${{VERSION}}"
    # WriteRegDWORD SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "VersionMajor" ${{VERSIONMAJOR}}
    # WriteRegDWORD SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "VersionMinor" ${{VERSIONMINOR}}
    # There is no option for modifying or repairing the install
    WriteRegDWORD SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "NoModify" 1
    WriteRegDWORD SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "NoRepair" 1
    # Set the INSTALLSIZE constant (!defined at the top of this script) so Add/Remove Programs can accurately report the size
    # WriteRegDWORD SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}" "EstimatedSize" ${{INSTALLSIZE}}
  
    File "uninstall.exe"
  !endif
SectionEnd

!ifdef INNER

Function un.onInit
  ${{If}} ${{RunningX64}}
    SetRegView 64
  ${{EndIf}}
  !insertmacro MULTIUSER_UNINIT
FunctionEnd

Section un.UninstallSection
  Delete /REBOOTOK $INSTDIR\{bcp47code}.zhfst
  Delete /REBOOTOK $INSTDIR\uninstall.exe
  RMDir $INSTDIR

  DeleteRegKey SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}"
SectionEnd

!endif
