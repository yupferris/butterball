; 3D Starfield with rotating camera

AppTitle "3D Starfield with rotating camera"

Const width = 640
Const height = 480

Graphics width,height,32,1
SetBuffer BackBuffer()

SeedRnd MilliSecs()

Type star
        Field x#
        Field y#
        Field z#
        Field zv#
End Type

Global numstars = 5000 ; Mess with this!!!

Dim s.star(numstars)

initstars()

Global move# = .05
Global damping# = .99
Global zoom# = 3
Global camx# = 0
Global camy# = 0
Global camz# = 0
Global camxv# = 0
Global camyv# = 0
Global camzv# = 0
Global camxa# = 0
Global camya# = 0
Global camza# = 0

While Not KeyDown(1)
Cls

LockBuffer
updatestars()
UnlockBuffer
updatecamera()

Flip
Wend
Delete Each star
End

Function initstars()
For i = 1 To numstars
        s.star(i) = New star
        initstar(i)
Next
End Function

Function initstar(i)
s(i)\x# = Rnd(-1000,1000)
s(i)\y# = Rnd(-1000,1000)
s(i)\z# = Rnd(-1000,1000)
s(i)\zv# = Rnd(1.0,5.0)
End Function

Function updatestars()
cx# = Cos(camx#)
sx# = Sin(camx#)
cy# = Cos(camy#)
sy# = Sin(camy#)
cz# = Cos(camz#)
sz# = Sin(camz#)
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
For i = 1 To numstars
        s(i)\z# = s(i)\z# - s(i)\zv#
        tx# = s(i)\x#
        ty# = s(i)\y#
        tz# = s(i)\z#
        rx# = Float(tx# * m00#) + Float(ty# * m10#) + Float(tz# * m20#)
        ry# = Float(tx# * m01#) + Float(ty# * m11#) + Float(tz# * m21#)
        rz# = Float(tx# * m02#) + Float(ty# * m12#) + Float(tz# * m22#)
        nx = ((Float(rx# / rz#) * 100) * zoom#) + (width Shr 1)
        ny = ((-Float(ry# / rz#) * 100) * zoom#) + (height Shr 1)
        If s(i)\z# <= -1000
                initstar(i)
                s(i)\z# = 1000
        ElseIf nx > 1 And nx < width - 2 And ny > 1 And ny < height - 2 And rz# > 0 And rz# <= 1000
                c = 255 * Float(1.0 - Float(rz# / 1000.0))
                c2 = c * .66
                c3 = c * .33
                argb2 = (c2 Shl 16) + (c2 Shl 8) + c2
                argb3 = (c3 Shl 16) + (c3 Shl 8) + c3
                WritePixelFast nx,ny,(c Shl 16) + (c Shl 8) + c
                WritePixelFast nx - 1,ny,argb2
                WritePixelFast nx + 1,ny,argb2
                WritePixelFast nx,ny - 1,argb2
                WritePixelFast nx,ny + 1,argb2
                WritePixelFast nx - 2,ny,argb3
                WritePixelFast nx + 2,ny,argb3
                WritePixelFast nx,ny - 2,argb3
                WritePixelFast nx,ny + 2,argb3
        EndIf
Next
End Function

Function updatecamera()
;camxa# = Rnd(-move#,move#) * damping#
;camya# = Rnd(-move#,move#) * damping#
;camza# = Rnd(-move#,move#) * damping#
;camxv# = camxv# + camxa#
;camyv# = camyv# + camya#
;camzv# = camzv# + camza#
;camxv# = camxv# * damping#
;camyv# = camyv# * damping#
;camzv# = camzv# * damping#
camx# = camx# + .25;camxv#
camy# = camy# + .5;camyv#
camz# = camz# + .25;camzv#
End Function