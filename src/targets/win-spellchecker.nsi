!define CLSID {{E45885BF-50CB-4F8F-9B19-95767EAF0F5C}}
!define DLL_NAME windivvun.dll

; General
Name "{app_name}"
Unicode true
SetCompressor /SOLID lzma

RequestExecutionLevel admin

!define MULTIUSER_EXECUTIONLEVEL Admin
!define MULTIUSER_MUI
!define MULTIUSER_INSTALLMODE_COMMANDLINE
!define MULTIUSER_USE_PROGRAMFILES64
!define MULTIUSER_INSTALLMODE_INSTDIR WinDivvun\

!include MultiUser.nsh
!include MUI2.nsh
!include x64.nsh

!define MUI_FINISHPAGE_NOAUTOCLOSE
!define MUI_UNFINISHPAGE_NOAUTOCLOSE

!insertmacro MUI_PAGE_WELCOME
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
  !ifdef INNER
    System::Call "kernel32::GetCurrentDirectory(i ${{NSIS_MAX_STRLEN}}, t .r0)"
    WriteUninstaller "$0\uninstall.exe"
    Quit
  !else
    ${{If}} ${{RunningX64}}
      SetRegView 64
    ${{EndIf}}

    !insertmacro MULTIUSER_INIT
  !endif
FunctionEnd

Section "Installer Section"
  SetOutPath $INSTDIR

  ; copy spellchecker
  File /oname=${{DLL_NAME}} ${{DLL_NAME}}

  ; create folder for spellers
  CreateDirectory $INSTDIR\Spellers

  ; update registry
  WriteRegStr SHCTX "SOFTWARE\Microsoft\Spelling\Spellers\Divvun" "CLSID" "${{CLSID}}"
  WriteRegStr SHCTX "SOFTWARE\Classes\CLSID\${{CLSID}}" "" "WinDivvun Spell Checking Service"
  WriteRegStr SHCTX "SOFTWARE\Classes\CLSID\${{CLSID}}" "AppId" "${{CLSID}}"
  WriteRegStr SHCTX "SOFTWARE\Classes\CLSID\${{CLSID}}\InProcServer32" "" "$INSTDIR\${{DLL_NAME}}"
  WriteRegStr SHCTX "SOFTWARE\Classes\CLSID\${{CLSID}}\InProcServer32" "ThreadingModel" "Both"
  WriteRegStr SHCTX "SOFTWARE\Classes\CLSID\${{CLSID}}\Version" "" "{version}.{build}"

  ; grant access to application packages
  Exec 'icacls "$INSTDIR" /grant "ALL APPLICATION PACKAGES":R /T'

  !ifndef INNER
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
  DeleteRegKey SHCTX "SOFTWARE\Microsoft\Spelling\Spellers\Divvun"
  DeleteRegKey SHCTX "SOFTWARE\Classes\CLSID\${{CLSID}}"
  Delete /REBOOTOK $INSTDIR\${{DLL_NAME}}
  Delete $INSTDIR\uninstall.exe
  RMDir $INSTDIR\Spellers
  RMDir $INSTDIR
SectionEnd

!endif
