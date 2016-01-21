; Anti-Aliased Vectorbobs

AppTitle "Anti-Aliased Vectorbobs"

Const width = 640
Const height = 480

Const width2 = width / 2
Const height2 = height / 2

Graphics width,height,32,1
SetBuffer BackBuffer()

HidePointer

Dim screen(width2,height2,2)
Dim palette(255)
Dim obj#(1,3)
Dim bob(1,1)

Global points
Global distance# = 6
Global size# = 150
Global xan#
Global yan#
Global zan#
Global xani# = .25
Global yani# = .5
Global zani# = .25

Global bobox = width2 / 2
Global boboy = height2 / 2
Global bobw
Global bobh

Global fps = 0
Global fpsc = 0
Global fpss = False
Global fpstime = MilliSecs()
Global fpstimer = 1000

Global intensity# = 3

setuppalette()
loadobj()
loadbob()

While Not KeyDown(1)
Cls

updateobj()
If MouseDown(1) Then drawbob MouseX() Shr 1,MouseY() Shr 1
LockBuffer
aalias()
UnlockBuffer
calcfps()

Flip False
Wend
End

Function setuppalette()
For i = 0 To 85
        r = 0
        g = i * 1.5
        b = i * 3
        palette(i) = (r Shl 16) + (g Shl 8) + b
Next
For i = 1 To 85
        r = i * 1.5
        g = 127 + (i * 1.5)
        b = 255
        palette(85 + i) = (r Shl 16) + (g Shl 8) + b
Next
For i = 1 To 85
        r = 127 + (i * 1.5)
        g = 255
        b = 255
        palette(170 + i) = (r Shl 16) + (g Shl 8) + b
Next
End Function

Function loadobj()
points = 125
Dim obj#(points,3)
For z = 0 To 4
        nz# = Float(z - 2)
        For y = 0 To 4
                ny# = -Float(y - 2)
                For x = 0 To 4
                        nx# = Float(x - 2)
                        i = (z * 25) + (y * 5) + x
                        obj#(i,0) = nx#
                        obj#(i,1) = ny#
                        obj#(i,2) = nz#
                Next
        Next
Next
End Function

Function updateobj()
cx# = Cos(xan#)
sx# = Sin(xan#)
cy# = Cos(yan#)
sy# = Sin(yan#)
cz# = Cos(zan#)
sz# = Sin(zan#)
sxsy# = Float(sx# * sy#)
cxsy# = Float(cx# * sy#)
m00# = Float(cy# * cz#)
m01# = Float(-cy# * sz#)
m02# = sy#
m10# = Float(cx# * sz#) + Float(sxsy# * cz#)
m11# = Float(cx# * cz#) - Float(sxsy# * sz#)
m12# = Float(-sx# * cy#)
m20# = Float(sx# * sz#) - Float(cxsy# * cz#)
m21# = Float(sx# * cz#) + Float(cxsy# * sz#)
m22# = Float(cx# * cy#)
For i = 0 To points - 1
        tx# = obj#(i,0)
        ty# = obj#(i,1)
        tz# = obj#(i,2)
        rx# = Float(tx# * m00#) + Float(ty# * m10#) + Float(tz# * m20#)
        ry# = Float(tx# * m01#) + Float(ty# * m11#) + Float(tz# * m21#)
        rz# = Float(tx# * m02#) + Float(ty# * m12#) + Float(tz# * m22#)
        rz# = Float(distance# + rz#)
        If rz# > 1
                nx = (Float(rx# / rz#) * size#) + bobox
                ny = (-Float(ry# / rz#) * size#) + boboy
                drawbob nx,ny
        EndIf
Next
xan# = xan# + xani#
yan# = yan# + yani#
zan# = zan# + zani#
End Function

Function loadbob()
Restore bobdata
Read bobw
Read bobh
Dim bob(bobw,bobh)
For y = 0 To bobh - 1
        For x = 0 To bobw - 1
                Read bob(x,y)
        Next
Next
End Function

Function drawbob(ox,oy)
For y = 0 To bobh - 1
        ny = (oy - (bobh / 2)) + y
        For x = 0 To bobw - 1
                nx = (ox - (bobw / 2)) + x
                If nx > 0 And nx < width2 - 1 And ny > 0 And ny < height2 - 1
                        c = screen(nx,ny,1) + (bob(x,y) * intensity#)
;                        c = bob(x,y) * 55;intensity#
                        If c > 255 Then c = 255
                        screen(nx,ny,1) = c
                EndIf
        Next
Next
End Function

Function aalias()
For y = 1 To height2 - 2
        For x = 1 To width2 - 2
                screen(x,y,2) = (screen(x - 1,y,1) + screen(x + 1,y,1) + screen(x,y - 1,1) + screen(x,y + 1,1)) Shr 2
        Next
Next
For y = 1 To height2 - 1
        For x = 1 To width2 - 1
                c = screen(x,y,2)
                If c > 0
                        nx = x Shl 1
                        ny = y Shl 1
                        argb = palette(c)
                        WritePixelFast nx,ny,palette(c)
                        WritePixelFast nx + 1,ny,argb
                        WritePixelFast nx,ny + 1,argb
                        WritePixelFast nx + 1,ny + 1,argb
                EndIf
                screen(x,y,1) = c * .9
        Next
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
Color 255,255,255
Text 2,2,fps + " FPS"
End Function

.bobdata

Data 16,16

Data 0,0,0,0,0,2,2,2,2,2,2,0,0,0,0,0
Data 0,0,0,2,2,2,2,2,2,2,2,2,2,0,0,0
Data 0,0,2,1,2,2,2,2,2,2,2,2,2,2,0,0
Data 0,3,3,2,2,2,2,2,3,3,3,3,2,2,2,0
Data 0,3,1,2,2,2,2,3,3,5,5,3,3,2,2,0
Data 3,3,1,2,2,2,2,3,5,5,5,5,3,2,2,2
Data 3,1,1,2,2,2,2,3,5,5,5,5,3,2,2,2
Data 3,1,1,2,2,2,2,3,3,5,5,3,3,2,2,2
Data 3,1,1,2,2,2,2,2,3,3,3,3,2,2,2,2
Data 3,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2
Data 3,3,1,1,2,2,2,2,2,2,2,2,2,2,2,2
Data 0,3,1,1,1,2,2,2,2,2,2,2,2,2,2,0
Data 0,3,3,1,1,1,1,2,2,2,2,2,2,3,3,0
Data 0,0,3,3,1,1,1,1,1,1,1,1,3,3,0,0
Data 0,0,0,3,3,3,1,1,1,1,3,3,3,0,0,0
Data 0,0,0,0,0,3,3,3,3,3,3,0,0,0,0,0
