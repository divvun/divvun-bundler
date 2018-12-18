!define CLSID {{E45885BF-50CB-4F8F-9B19-95767EAF0F5C}}
!define DLL_NAME windivvun.dll
!define DLL_NAME32 windivvun32.dll
!define DLL_NAME64 windivvun64.dll
!define COMPANY_NAME "Divvun"
!define APP_NAME "{app_name}"
!define APP_ID "{app_id}"
!define APP_URL "http://divvun.no/"
!define VERSION "{version}"

Name "${{APP_NAME}}"
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
UninstPage Custom un.LockedListShow
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

Section "64bit" Section1
  SectionIn RO  

  SetOutPath $INSTDIR
  File /oname=${{DLL_NAME}} ${{DLL_NAME64}}
SectionEnd

Section "32bit" Section2
  SectionIn RO  

  SetOutPath $INSTDIR
  File /oname=${{DLL_NAME}} ${{DLL_NAME32}}
SectionEnd

Function .onInit
  ${{If}} ${{RunningX64}}
    SetRegView 64
  ${{EndIf}}
  !insertmacro MULTIUSER_INIT

  IntOp $0 ${{SF_SELECTED}} | ${{SF_RO}}
  ${{If}} ${{RunningX64}}
    SectionSetFlags ${{Section1}} $0
    SectionSetFlags ${{Section2}} ${{SECTION_OFF}}
  ${{Else}}
    SectionSetFlags ${{Section2}} ${{SECTION_OFF}} 
    SectionSetFlags ${{Section1}} $0
  ${{EndIf}}
  
  !ifdef INNER
    System::Call "kernel32::GetCurrentDirectory(i ${{NSIS_MAX_STRLEN}}, t .r0)"
    WriteUninstaller "$0\uninstall.exe"
    Quit
  !endif
FunctionEnd

Section "Installer Section"
  SetOutPath $INSTDIR

  !ifndef INNER
    ; create folder for spellers
    CreateDirectory $INSTDIR\Spellers

    ; grant access to application packages
    nsExec::Exec 'icacls "$INSTDIR" /grant "ALL APPLICATION PACKAGES":R /T'

    ; update registry
    WriteRegStr SHCTX "SOFTWARE\Microsoft\Spelling\Spellers\Divvun" "CLSID" "${{CLSID}}"
    WriteRegStr SHCTX "SOFTWARE\Classes\CLSID\${{CLSID}}" "" "WinDivvun Spell Checking Service"
    WriteRegStr SHCTX "SOFTWARE\Classes\CLSID\${{CLSID}}" "AppId" "${{CLSID}}"
    WriteRegStr SHCTX "SOFTWARE\Classes\CLSID\${{CLSID}}\InProcServer32" "" "$INSTDIR\${{DLL_NAME}}"
    WriteRegStr SHCTX "SOFTWARE\Classes\CLSID\${{CLSID}}\InProcServer32" "ThreadingModel" "Both"
    WriteRegStr SHCTX "SOFTWARE\Classes\CLSID\${{CLSID}}\Version" "" "${{VERSION}}"

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

Function un.LockedListShow
  # !insertmacro MUI_HEADER_TEXT `LockedList Test` `Using AddModule and notepad.exe`
  ${{If}} ${{RunningX64}}
    File /oname=$PLUGINSDIR\LockedList64.dll `${{NSISDIR}}\Plugins\LockedList64.dll`
  ${{Else}}
    File /oname=$PLUGINSDIR\LockedList.dll `${{NSISDIR}}\Plugins\LockedList.dll`
  ${{EndIf}}
  LockedList::AddModule "${{DLL_NAME}}" 
  LockedList::Dialog /autoclose `` `` `` `Close All`
  Pop $R0
FunctionEnd

Section un.UninstallSection
  DeleteRegKey SHCTX "SOFTWARE\Microsoft\Spelling\Spellers\Divvun"
  DeleteRegKey SHCTX "SOFTWARE\Classes\CLSID\${{CLSID}}"
  Delete /REBOOTOK $INSTDIR\${{DLL_NAME}}
  Delete $INSTDIR\uninstall.exe
  RMDir $INSTDIR\Spellers
  RMDir $INSTDIR

  DeleteRegKey SHCTX "Software\Microsoft\Windows\CurrentVersion\Uninstall\${{APP_ID}}"
SectionEnd

!endif
