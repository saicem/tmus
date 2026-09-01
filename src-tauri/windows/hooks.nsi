!macro NSIS_HOOK_POSTINSTALL
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "Tmus" '$INSTDIR\tmus.exe nw'

  ; Add the install directory to the user PATH so the bundled CLI is available
  ; from any terminal. Skip the write when it is already present, compared
  ; case-insensitively because Windows paths are case-insensitive.
  ReadRegStr $0 HKCU "Environment" "Path"
  ${StrCase} $1 $0 "L"
  ${StrCase} $2 $INSTDIR "L"
  ${StrLoc} $3 $1 $2 ">"
  ${If} $3 == ""
    ${If} $0 == ""
      StrCpy $0 "$INSTDIR"
    ${Else}
      StrCpy $0 "$0;$INSTDIR"
    ${EndIf}
    WriteRegExpandStr HKCU "Environment" "Path" "$0"
    ; Notify the system so running processes see the updated PATH immediately.
    System::Call "user32::SendMessageTimeout(i 0xFFFF, i 0x1A, i 0, t 'Environment', i 0, i 5000, *i r4)"
  ${EndIf}
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "Tmus"

  ; Remove the install directory from the user PATH and collapse leftover
  ; separators; delete the value entirely when no entries remain.
  ReadRegStr $0 HKCU "Environment" "Path"
  ${If} $0 != ""
    StrCpy $1 "$0"
    StrCpy $2 ""
    ${DoWhile} $1 != ""
      ; Locate the first separator in the remaining path.
      StrLen $6 $1
      StrCpy $4 ""
      StrCpy $5 0
      ${DoWhile} $5 < $6
        StrCpy $7 $1 1 $5
        ${If} $7 == ";"
          StrCpy $4 $5
          ${Break}
        ${EndIf}
        IntOp $5 $5 + 1
      ${Loop}
      ${If} $4 == ""
        StrCpy $8 "$1"
        StrCpy $1 ""
      ${Else}
        StrCpy $8 "$1" $4
        IntOp $5 $4 + 1
        StrCpy $1 "$1" "" $5
      ${EndIf}
      ${If} $8 != $INSTDIR
        ${If} $2 == ""
          StrCpy $2 "$8"
        ${Else}
          StrCpy $2 "$2;$8"
        ${EndIf}
      ${EndIf}
    ${Loop}
    ${If} $2 != $0
      ${If} $2 == ""
        DeleteRegValue HKCU "Environment" "Path"
      ${Else}
        WriteRegExpandStr HKCU "Environment" "Path" "$2"
      ${EndIf}
      System::Call "user32::SendMessageTimeout(i 0xFFFF, i 0x1A, i 0, t 'Environment', i 0, i 5000, *i r4)"
    ${EndIf}
  ${EndIf}

  ${If} $DeleteAppDataCheckboxState = 1
  ${AndIf} $UpdateMode <> 1
    RmDir /r "$APPDATA\tmus"
    RmDir /r "$LOCALAPPDATA\tmus"
  ${EndIf}
!macroend
