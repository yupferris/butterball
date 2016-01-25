; LS Vectorballs

AppTitle "LS Vectorballs"

Const width = 640
Const height = 480

Graphics width,height,32,1

Type ball
        Field x#
        Field y#
        Field z#
        Field nx#
        Field ny#
        Field nz#
        Field c
End Type

Global limage = CreateImage(32,32)
SetBuffer ImageBuffer(limage)
Color 255,0,255
Rect 0,0,ImageWidth(limage),ImageHeight(limage),1
For i = 0 To 15
        Color i * 17,i * 17,i * 17
        Oval i,i,ImageWidth(limage) - (i * 2),ImageHeight(limage) - (i * 2)
Next
MaskImage limage,255,0,255
MidHandle limage

Global bimage = CreateImage(96,96,5)
MaskImage bimage,255,0,255
MidHandle bimage
SetBuffer ImageBuffer(bimage,4)
For i = 0 To 48
        Color (i * 5) + 15,(i * 5) + 15,(i * 5) + 15
        Oval i,i,ImageWidth(bimage) - (i * 2),ImageHeight(bimage) - (i * 2),1
Next
SetBuffer BackBuffer()

Global mask = CreateImage(ImageWidth(bimage),ImageHeight(bimage))
MaskImage mask,0,0,0
SetBuffer ImageBuffer(mask)
Color 255,0,255
Rect 0,0,ImageWidth(mask),ImageHeight(mask),1
Color 0,0,0
Oval 0,0,ImageWidth(mask),ImageHeight(mask),1
SetBuffer BackBuffer()

Global numballs = 27
Dim b.ball(numballs)

loadobj()

Global xan# = 0
Global yan# = 0
Global zan# = 0
Global lxan# = 90
Global lyan# = 0
Global lx# = (width / 2) + (Cos(lxan#) * (width / 3))
Global ly# = (height / 2) + (Sin(lyan#) * (height / 3))

Global distance = 300
Global size# = 275

Global ox# = width / 3
Global oy# = height / 2
;MoveMouse ox#,oy#

While Not KeyDown(1)
Cls

Color 128,128,128
Rect 0,0,width - 1,height - 1,1

updateobj()
updatelight()

Flip
Wend

FreeImage limage
FreeImage bimage
FreeImage mask
For i = 1 To numballs
        Delete b(i)
Next
End

Function loadobj()
Restore obj
For i = 1 To numballs
        b.ball(i) = New ball
        Read b(i)\x#
        Read b(i)\y#
        Read b(i)\z#
        Read b(i)\c
Next
End Function

Function updateobj()
cx# = Cos(xan#)
sx# = Sin(xan#)
cy# = Cos(yan#)
sy# = Sin(yan#)
cz# = Cos(zan#)
sz# = Sin(zan#)
For i = 1 To numballs
        x3d# = b(i)\x#
        y3d# = b(i)\y#
        z3d# = b(i)\z#
        ty# = ((y3d# * cx#) - (z3d# * sx#))
        tz# = ((y3d# * sx#) + (z3d# * cx#))
        tx# = ((x3d# * cy#) - (tz# * sy#))
        tz# = ((x3d# * sy#) + (tz# * cy#))
        x2# = tx#
        tx# = ((tx# * cz#) - (ty# * sz#))
        ty# = ((x2# * sz#) + (ty# * cz#))
        nz# = distance - ((tz# / 3) + 1)
        b(i)\nx# = ((tx# * size#) / nz#) + ox#
        b(i)\ny# = ((-ty# * size#) / nz#) + oy#
        b(i)\nz# = nz#
Next

For i = 1 To numballs
        For i2 = 1 To numballs
                If b(i)\nz# > b(i2)\nz#
                        tx# = b(i)\x#
                        ty# = b(i)\y#
                        tz# = b(i)\z#
                        tnx# = b(i)\nx#
                        tny# = b(i)\ny#
                        tnz# = b(i)\nz#
                        tc# = b(i)\c
                        b(i)\x# = b(i2)\x#
                        b(i)\y# = b(i2)\y#
                        b(i)\z# = b(i2)\z#
                        b(i)\nx# = b(i2)\nx#
                        b(i)\ny# = b(i2)\ny#
                        b(i)\nz# = b(i2)\nz#
                        b(i)\c = b(i2)\c
                        b(i2)\x# = tx#
                        b(i2)\y# = ty#
                        b(i2)\z# = tz#
                        b(i2)\nx# = tnx#
                        b(i2)\ny# = tny#
                        b(i2)\nz# = tnz#
                        b(i2)\c = tc
                EndIf
        Next
Next

For i = 1 To numballs
        DrawImage bimage,b(i)\nx#,b(i)\ny#,b(i)\c - 1
Next

xan# = xan# + 1
yan# = yan# + 2
zan# = zan# + 1
End Function

Function updatelight()
xrad# = (lx# - ox#) / 4
yrad# = (ly# - oy#) / 4
ix# = (ImageWidth(bimage) / 2) + xrad#
iy# = (ImageHeight(bimage) / 2) + yrad#
SetBuffer ImageBuffer(bimage,3)
Cls
DrawImage bimage,ix#,iy#,4
For i = 0 To 4
        LockBuffer ImageBuffer(bimage,i)
Next
For y = 0 To ImageHeight(bimage) - 1
        For x = 0 To ImageWidth(bimage) - 1
                argb = ReadPixelFast(x,y,ImageBuffer(bimage,3))
                red = ((argb Shr 16) And 255) Shl 16
                grn = ((argb Shr 8) And 255) Shl 8
                blu = argb And 255
                WritePixelFast x,y,grn,ImageBuffer(bimage,0)
                WritePixelFast x,y,blu,ImageBuffer(bimage,1)
                WritePixelFast x,y,red,ImageBuffer(bimage,2)
        Next
Next
For i = 0 To 3
        UnlockBuffer ImageBuffer(bimage,i)
        SetBuffer ImageBuffer(bimage,i)
        DrawImage mask,0,0
Next
UnlockBuffer ImageBuffer(bimage,4)
SetBuffer BackBuffer()
lx# = ox# + (Cos(lxan#) * (ox# * .8))
ly# = oy# + (Sin(lyan#) * (oy# * .8))
If MouseDown(1)
        lx# = MouseX()
        ly# = MouseY()
EndIf
DrawImage limage,lx#,ly#
lxan# = lxan# + 4
lyan# = lyan# + 3
End Function

.obj

Data -96,96,96,1
Data 0,96,96,1
Data 96,96,96,1
Data -96,0,96,1
Data 0,0,96,1
Data 96,0,96,1
Data -96,-96,96,1
Data 0,-96,96,1
Data 96,-96,96,1
Data -96,96,0,2
Data 0,96,0,2
Data 96,96,0,2
Data -96,0,0,2
Data 0,0,0,2
Data 96,0,0,2
Data -96,-96,0,2
Data 0,-96,0,2
Data 96,-96,0,2
Data -96,96,-96,3
Data 0,96,-96,3
Data 96,96,-96,3
Data -96,0,-96,3
Data 0,0,-96,3
Data 96,0,-96,3
Data -96,-96,-96,3
Data 0,-96,-96,3
Data 96,-96,-96,3