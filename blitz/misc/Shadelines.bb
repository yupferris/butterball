; Shadelines

AppTitle "Shadelines"

Const width = 800
Const height = 600

Graphics width,height,32,1
SetBuffer BackBuffer()

SeedRnd MilliSecs()

Global liwidth = width
Global liheight = height

Global limage = CreateImage(liwidth,liheight)
MidHandle limage

Dim lines(liwidth,liheight)
Dim palette(6,255)

setuppalette()

Global paletteindex = Rand(1,6)

Global xan1# = Rnd(360.0)
Global yan1# = Rnd(360.0)
Global xan2# = Rnd(360.0)
Global yan2# = Rnd(360.0)
Global xani1# = Rnd(1.0,3.0)
Global yani1# = Rnd(1.0,3.0)
Global xani2# = Rnd(1.0,3.0)
Global yani2# = Rnd(1.0,3.0)

Global sx1# = (liwidth / 2) + (Cos(xan1#) * (liwidth / 2))
Global sy1# = (liheight / 2) + (Sin(yan1#) * (liheight / 2))
Global sx2# = (liwidth / 2) + (Cos(xan2#) * (liwidth / 2))
Global sy2# = (liheight / 2) + (Sin(yan2#) * (liheight / 2))

Global ltime = MilliSecs()
Global ltimer = Rand(3000,10000)

While Not KeyDown(1)
Cls

updatelines()

Flip
Wend
FreeImage limage
End

Function setuppalette()
For i = 0 To 127
        c = (i * 2) + 1
        r1 = c
        g1 = 0
        b1 = 0
        r2 = 0
        g2 = c
        b2 = 0
        r3 = 0
        g3 = 0
        b3 = c
        r4 = c
        g4 = c
        b4 = 0
        r5 = c
        g5 = 0
        b5 = c
        r6 = 0
        g6 = c
        b6 = c
        palette(1,i) = (r1 Shl 16) + (g1 Shl 8) + b1
        palette(2,i) = (r2 Shl 16) + (g2 Shl 8) + b2
        palette(3,i) = (r3 Shl 16) + (g3 Shl 8) + b3
        palette(4,i) = (r4 Shl 16) + (g4 Shl 8) + b4
        palette(5,i) = (r5 Shl 16) + (g5 Shl 8) + b5
        palette(6,i) = (r6 Shl 16) + (g6 Shl 8) + b6
Next
For i = 0 To 127
        c = (i * 2) + 1
        r1 = 255
        g1 = c
        b1 = c
        r2 = c
        g2 = 255
        b2 = c
        r3 = c
        g3 = c
        b3 = 255
        r4 = 255
        g4 = 255
        b4 = c
        r5 = 255
        g5 = c
        b5 = 255
        r6 = c
        g6 = 255
        b6 = 255
        palette(1,128 + i) = (r1 Shl 16) + (g1 Shl 8) + b1
        palette(2,128 + i) = (r2 Shl 16) + (g2 Shl 8) + b2
        palette(3,128 + i) = (r3 Shl 16) + (g3 Shl 8) + b3
        palette(4,128 + i) = (r4 Shl 16) + (g4 Shl 8) + b4
        palette(5,128 + i) = (r5 Shl 16) + (g5 Shl 8) + b5
        palette(6,128 + i) = (r6 Shl 16) + (g6 Shl 8) + b6
Next
End Function

Function updatelines()
LockBuffer ImageBuffer(limage)
sline sx1#,sy1#,sx2#,sy2#
UnlockBuffer ImageBuffer(limage)

DrawBlock limage,width / 2,height / 2

If MilliSecs() >= ltime + ltimer
        clearlines()
        xan1# = Rnd(360.0)
        yan1# = Rnd(360.0)
        xan2# = Rnd(360.0)
        yan2# = Rnd(360.0)
        xani1# = Rnd(1.0,3.0)
        yani1# = Rnd(1.0,3.0)
        xani2# = Rnd(1.0,3.0)
        yani2# = Rnd(1.0,3.0)
        ltime = MilliSecs()
        ltimer = Rand(3000,10000)
EndIf

xan1# = xan1# + xani1#
yan1# = yan1# + yani1#
xan2# = xan2# + xani2#
yan2# = yan2# + yani2#

sx1# = (liwidth / 2) + (Cos(xan1#) * ((liwidth - 1) / 2))
sy1# = (liheight / 2) + (Sin(yan1#) * ((liheight - 1) / 2))
sx2# = (liwidth / 2) + (Cos(xan2#) * ((liwidth - 1) / 2))
sy2# = (liheight / 2) + (Sin(yan2#) * ((liheight - 1) / 2))
End Function

Function clearlines()
SetBuffer ImageBuffer(limage)
Cls
SetBuffer BackBuffer()
For y = 0 To liheight
        For x = 0 To liwidth
                lines(x,y) = 0
        Next
Next
paletteindex = Rand(1,6)
End Function

Function sline(x1,y1,x2,y2)
x# = x1
y# = y1
xdis# = Float(x2 - x1)
ydis# = Float(y2 - y1)
dis# = Sqr(Float(xdis# * xdis#) + Float(ydis# * ydis#))
xv# = 0
yv# = 0
If dis# > 0
        xv# = Float(x2 - x1) / dis#
        yv# = Float(y2 - y1) / dis#
        For i = 1 To dis#
                If x >= 0 And x < liwidth And y >= 0 And y < liheight
                        c = lines(x,y) + 63
                        If c > 255 Then c = 255
                        lines(x,y) = c
                        WritePixelFast x,y,palette(paletteindex,c),ImageBuffer(limage)
                EndIf
                x# = x# + xv#
                y# = y# + yv#
        Next
EndIf
End Function