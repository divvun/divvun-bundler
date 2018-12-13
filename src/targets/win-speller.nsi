!define CLSID {{E45885BF-50CB-4F8F-9B19-95767EAF0F5C}}
!define DLL_NAME windivvun.dll

; General
Unicode true
RequestExecutionLevel admin

!define MULTIUSER_EXECUTIONLEVEL Admin
!define MULTIUSER_MUI
!define MULTIUSER_INSTALLMODE_COMMANDLINE
!define MULTIUSER_INSTALLMODE_INSTDIR WinDivvun\

!include MultiUser.nsh
!include MUI2.nsh
!include x64.nsh

!define MUI_FINISHPAGE_NOAUTOCLOSE
!define MUI_UNFINISHPAGE_NOAUTOCLOSE


Name "{app_name}"
Outfile installer.exe

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE English

Section "Installer Section"
  SetOutPath $INSTDIR

  ; copy spellchecker
  File /oname=${{DLL_NAME}} ${{DLL_NAME}}

  ; create folder for spellers
  CreateDirectory $INSTDIR\Spellers

  ; update registry
  WriteRegStr HKLM "SOFTWARE\Microsoft\Spelling\Spellers\Divvun" "CLSID" "${{CLSID}}"
  WriteRegStr HKLM "SOFTWARE\Classes\CLSID\${{CLSID}}" "" "WinDivvun Spell Checking Service"
  WriteRegStr HKLM "SOFTWARE\Classes\CLSID\${{CLSID}}" "AppId" "${{CLSID}}"
  WriteRegStr HKLM "SOFTWARE\Classes\CLSID\${{CLSID}}\InProcServer32" "" "$INSTDIR\${{DLL_NAME}}"
  WriteRegStr HKLM "SOFTWARE\Classes\CLSID\${{CLSID}}\InProcServer32" "ThreadingModel" "Both"
  WriteRegStr HKLM "SOFTWARE\Classes\CLSID\${{CLSID}}\Version" "" "{version}.{build}"

  ; grant access to application packages
  Exec 'icacls "$INSTDIR" /grant "ALL APPLICATION PACKAGES":R /T'

  writeUninstaller "$INSTDIR\uninstall.exe"
SectionEnd

Section un.UninstallSection
  DeleteRegKey HKLM "SOFTWARE\Microsoft\Spelling\Spellers\Divvun"
  DeleteRegKey HKLM "SOFTWARE\Classes\CLSID\${{CLSID}}"
  Delete $INSTDIR\${{DLL_NAME}}
  Delete $INSTDIR\uninstall.exe
SectionEnd

!finalize '{sign_tool}'

Function .onInit
  !insertmacro MULTIUSER_INIT
  ${{If}} ${{RunningX64}}
      SetRegView 64
  ${{EndIf}}
FunctionEnd

Function un.onInit
  !insertmacro MULTIUSER_UNINIT
FunctionEnd
