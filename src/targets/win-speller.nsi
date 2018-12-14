Name "{app_name}"
Unicode true
SetCompressor /SOLID lzma

!define MULTIUSER_EXECUTIONLEVEL Highest
!define MULTIUSER_MUI
!define MULTIUSER_INSTALLMODE_COMMANDLINE
!define MULTIUSER_USE_PROGRAMFILES64

!define MULTIUSER_INSTALLMODE_INSTDIR WinDivvun\Spellers\{bcp47code}

!include MultiUser.nsh
!include MUI2.nsh

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

    OutFile installer.exe

    !finalize '{sign_tool}'
!endif

Function .onInit
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

  ; grant access to application packages
  nsExec::Exec 'icacls "$INSTDIR" /grant "ALL APPLICATION PACKAGES":R /T'

  !ifndef INNER
    File "uninstall.exe"
  !endif
SectionEnd

!ifdef INNER

Function un.onInit
  !insertmacro MULTIUSER_UNINIT
FunctionEnd

Section un.UninstallSection
  Delete $INSTDIR\uninstall.exe
  Delete $INSTDIR\{bcp47code}.zhfst
  RMDir $INSTDIR
  RMDir $INSTDIR\..
  RMDir $INSTDIR\..\..
SectionEnd

!endif
