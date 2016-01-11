; 3D OVERLOAD!

; (C) 2005 Jake Taylor [Thygrion]

AppTitle "[T/C] 3D OVERLOAD [T/C]"

Include "3DLib.bb"

unpackfolder "F_06449.dat","data"

Global cl$ = CommandLine$()
If cl$ = "-w"
	graphics3d 320,240,32,2
Else
	graphics3d 320,240,32,1
EndIf

SetBuffer BackBuffer()

SeedRnd MilliSecs()

setdistance 100
setsize 300

ambientlight 0,0,0
positionlight getdistance#() / 2,getdistance#() / 2,0
worldlight 0

shadows True

background3d "data\bg.png"

Global tex1 = loadtexture("data\WALL.png")
Global tex2 = loadtexture("data\spheremap.bmp")
Global tex3 = loadtexture("data\Face.jpg")
Global tex4 = loadtexture("data\fin.png")
texturesrm tex2
masktexture tex4

Global torus = loadascii("data\torus.txt")
scaleentity torus,20,20,20
entityalpha torus,0
hideentity torus

Global cube = loadascii("data\cube.txt")
scaleentity cube,20,20,20
entityalpha cube,0
entitytexture cube,tex1
hideentity cube

Global teapot = loadascii("data\teapot.txt")
scaleentity teapot,0,0,0
entitynolight teapot
entitytexture teapot,tex2
hideentity teapot

Global head = loadascii("data\head.txt")
scaleentity head,80,80,80
positionentity head,0,-20,0
entityalpha head,0
entitytexture head,tex3
hideentity head

Global equad = loadascii("data\quad.txt")
scaleentity equad,30,20,1
entityalpha equad,0
entitytexture equad,tex4
;entitynolight equad
entitynobfc equad
hideentity equad

Global font = LoadAnimImage("data\font.png",16,16,0,64)

Global stxt$ = "Pretty sweet, huh? This demo is AWESOME - It can open Milkshape 3D ASCII models, "
stxt$ = stxt$ + "it has per-pixel lighting, shadows, perspective-correct texture mapping, light-ray "
stxt$ = stxt$ + "refraction for alpha blending, and spherical reflection mapping! This is the "
stxt$ = stxt$ + "coolest demo I've ever done. It was coded ENTIRELY with Blitz 2D, no Blitz 3D or "
stxt$ = stxt$ + "Blitz Plus!                     Credits - (Code: Thygrion)-(GFX: Thygrion, Clyde "
stxt$ = stxt$ + "Radcliffe, Unknown)-(Music: Unknown)                    Greets go out to: "
stxt$ = stxt$ + "Shockwave/JackDaniels-Clyde-Fash-Roly-Wham-Blitz Amateur-Turkwoyz-Kiwee-Joncom2000-"
stxt$ = stxt$ + "Zawran-Recharge-CKarl                    Hit F to toggle FPS info."
stxt$ = stxt$ + "                    - End of Text -                    "
stxt$ = Upper$(stxt$)
Global scrollx = width
Global scrolly = height * .9
Global scrollan# = 0

Global enddemo = False

Global part = 1

Global a# = 0
Global s# = 1

Global an# = 0

Global filenum = 1

Global fps = 0
Global fpsc = 0
Global fpss = False

Global fpstime = MilliSecs()
Global fpstimer = 1000

Global audio = PlayMusic("data\COWMAN.it")

;cleardir "data"

Global ptime = MilliSecs()
Global ptimer = 45000

While Not enddemo
Cls

Select part
	Case 1
		part1()
	Case 2
		part2()
	Case 3
		part3()
	Case 4
		part4()
	Case 5
		part5()
	Case 6
		part6()
	Case 7
		part7()
	Case 8
		part8()
	Case 9
		part9()
	Case 10
		part10()
	Case 11
		part11()
	Case 12
		part12()
End Select

renderworld

If part > 1 Then updatescroll()

If KeyHit(33) Then fpss = Not fpss
If fpss Then calcfps()

If KeyHit(1) And part > 1 And part < 11 Then enddemo = True

If cl$ = "-s" And KeyHit(31) Then SaveBuffer FrontBuffer(),"3DOverloadSS" + filenum + ".bmp" : filenum = filenum + 1

Flip False
Wend
For i = 100 To 0 Step -4
	Cls
	
	a# = Float(i / 100.0)
	ChannelVolume audio,a# * .5
	worldlight a#
	renderworld
	scrolly = scrolly + 2
	updatescroll()
	
	Flip
Next
StopChannel audio
clearworld
FreeImage font
End

Function part1()
a# = a# + .025
worldlight a#
If a# > 1
	a# = 0
	worldlight 1
	unhideentity torus
	part = 2
	ptime = MilliSecs()
EndIf
End Function

Function part2()
If a# < 1
	a# = a# + .025
	If a# > 1 Then a# = 1
	entityalpha torus,a#
EndIf
turnentity torus,1,2,1
If KeyHit(57) Or MilliSecs() >= ptime + ptimer
	entityrd torus,10
	part = 3
	ptime = MilliSecs()
EndIf
End Function

Function part3()
If a# > .4
	a# = a# - .025
	If a# < .4 Then a# = .4
	entityalpha torus,a#
EndIf
turnentity torus,1,2,1
If KeyHit(57) Or MilliSecs() >= ptime + ptimer
	part = 4
EndIf
End Function

Function part4()
a# = a# - .025
entityalpha torus,a#
turnentity torus,1,2,1
positionentity torus,0,entityy(torus) + 3,0
If a# < 0
	a# = 0
	hideentity torus
	unhideentity cube
	part = 5
	ptime = MilliSecs()
EndIf
End Function

Function part5()
If a# < 1
	a# = a# + .025
	If a# > 1 Then a# = 1
	entityalpha cube,a#
EndIf
turnentity cube,-2,-4,-2
If KeyHit(57) Or MilliSecs() >= ptime + ptimer
	entityrd cube,10
	part = 6
	ptime = MilliSecs()
EndIf
End Function

Function part6()
s# = s# - .025
entityalpha cube,s#
scaleentity cube,20 * s#,20 * s#,20 * s#
turnentity cube,2,4,2
If s# < 0
	hideentity cube
	unhideentity teapot
	part = 7
	ptime = MilliSecs()
EndIf
End Function

Function part7()
If s# < 1
	s# = s# + .05
	If s# > 1 Then s# = 1
	scaleentity teapot,s# * 20,s# * 20,s# * 20
EndIf
turnentity teapot,1,2,1
If KeyHit(57) Or MilliSecs() >= ptime + ptimer
	a# = 1
	part = 8
	ptime = MilliSecs()
EndIf
End Function

Function part8()
turnentity teapot,1,2,1
a# = a# - .05
entityalpha teapot,a#
If a# < 0
	a# = 0
	ambientlight 64,64,64
	hideentity teapot
	unhideentity head
	part = 9
	ptime = MilliSecs()
EndIf
End Function

Function part9()
If a# < 1
	a# = a# + .025
	If a# > 1 Then a# = 1
	entityalpha head,a#
EndIf
rotateentity head,Sin(an#) * 9,Sin(an# / 5) * 25,0
an# = an# + 20
If KeyHit(57) Or MilliSecs() >= ptime + ptimer
	entityrd head,10
	part = 10
EndIf
End Function

Function part10()
If a# > 0
	a# = a# - .025
	If a# < 0 Then a# = 0
	entityalpha head,a#
	lightcolor 255,255 * a#,255 * a#
	worldlight a#
EndIf
rotateentity head,Sin(an#) * 9,Sin(an# / 5) * 25,0
an# = an# + 20
If a# <= 0
	a# = 0
	hideentity head
	unhideentity equad
	ambientlight 0,0,0
	shadows False
	lightcolor 255,255,255
	clearbackground3d
	worldlight 1
	an# = 0
	part = 11
EndIf
End Function

Function part11()
If a# < 1
	a# = a# + .025
	If a# > 1 Then a# = 1
	entityalpha equad,a#
EndIf
rotateentity equad,0,Sin(an#) * 30,0
an# = an# + 5
positionlight Cos(an#) * 12,Sin(an#) * 12,getdistance#() - 12
If KeyHit(57) Or KeyHit(1)
	s# = 1
	part = 12
EndIf
End Function

Function part12()
turnentity equad,0,20,0
s# = s# - .025
scaleentity equad,30 * s#,20 * s#,s#
an# = an# + 5
positionlight Cos(an#) * 12,Sin(an#) * 12,getdistance#() - 12
If s# < 0 Then enddemo = True
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

Function updatescroll()
ctext scrollx,scrolly,stxt$,False,True,font,scrollan#,3,10
scrollan# = scrollan# + 10
scrollx = scrollx - 6
If scrollx < -(Len(stxt$) * ImageWidth(font)) Then scrollx = width
End Function

Function ctext(x,y,txt$,xc,yc,font,san#,freq#,amp,spacing = 0)
l = Len(txt$)
w = ImageWidth(font)
h = ImageHeight(font)
If xc
	x = x - (((l * w) + ((l - 1) * spacing)) / 2)
EndIf
If yc
	y = y - (h / 2)
EndIf
san2# = san#
For i = 1 To l
	frame = Asc(Mid$(txt$,i,1)) - 32
	For j = 0 To w - 1
		If x + j > -w And x + j < width + w Then DrawImageRect font,x + j,y + (Sin(san2#) * amp),j,0,1,h - 1,frame
		san2# = san2# + freq#
	Next
	x = x + (w + spacing)
Next
End Function

Function calcfps()
If MilliSecs() >= fpstime + fpstimer
	fps = fpsc
	fpsc = 0
	fpstime = MilliSecs()
Else
	fpsc = fpsc + 1
EndIf
Color 128,191,255
Text 2,2,fps + " FPS"
End Function