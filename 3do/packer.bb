; File Packer/Unpacker Functions

packfolder "data","F_06449.dat"

Function packfolder(dname$,fname$)
file = WriteFile(fname$)
dir = ReadDir(dname$)
Repeat

nfile$ = NextFile$(dir)
If nfile$ = "" Then Exit
If FileType(dname$ + "\" + nfile$) <> 2
	tfile = ReadFile(dname$ + "\" + nfile$)
	fsize = FileSize(dname$ + "\" + nfile$)
	WriteString file,nfile$
	WriteInt file,fsize
	For i = 0 To fsize - 1
		WriteByte file,ReadByte(tfile)
	Next
	CloseFile tfile
EndIf

Forever
CloseDir dir
CloseFile file
End Function

Function unpackfolder(fname$,dname$)
If FileType(dname$) = 0 Then CreateDir dname$
file = ReadFile(fname$)
While Not Eof(file)

tname$ = ReadString$(file)
tfile = WriteFile(dname$ + "\" + tname$)
fsize = ReadInt(file)
For i = 0 To fsize - 1
	WriteByte tfile,ReadByte(file)
Next
CloseFile tfile

Wend
CloseFile file
End Function

Function cleardir(dname$)
dir = ReadDir(dname$)
Repeat

fname$ = NextFile$(dir)
If fname$ = "" Then Exit
DeleteFile dname$ + "\" + fname$

Forever
CloseDir dir
DeleteDir dname$
End Function