!define MULTIUSER_EXECUTIONLEVEL Highest
!define MULTIUSER_MUI
!define MULTIUSER_INSTALLMODE_COMMANDLINE
!define MULTIUSER_USE_PROGRAMFILES64

!define MULTIUSER_INSTALLMODE_INSTDIR WinDivvun\Spellers\{bcp47code}

!include MultiUser.nsh
!include MUI2.nsh

Name "{app_name}"
Outfile installer.exe

!insertmacro MUI_PAGE_WELCOME
!insertmacro MULTIUSER_PAGE_INSTALLMODE
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE English

Function .onInit
  !insertmacro MULTIUSER_INIT
FunctionEnd

Section "Installer Section"
  SetOutPath $INSTDIR

  ; copy spellchecker
  File /oname={bcp47code}.zhfst speller.zhfst

  ; grant access to application packages
  Exec 'icacls "$INSTDIR" /grant "ALL APPLICATION PACKAGES":R /T'

  writeUninstaller "$INSTDIR\uninstall.exe"
SectionEnd

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

!finalize '{sign_tool}'
