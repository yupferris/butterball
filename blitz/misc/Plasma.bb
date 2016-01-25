; Plasma

AppTitle "Plasma"

Const width = 320
Const height = 240

Graphics width,height,32,1
SetBuffer BackBuffer()

Dim ctable#(1080)
Dim stable#(1080)

setuptables()

Dim palette(255)

setuppalette()

Global wave1# = 0
Global wave2# = 0

While Not KeyDown(1)
Cls

updateplasma()

Flip
Wend
End

Function setuptables()
For i = 0 To 1080
        ctable#(i) = Cos(i)
        stable#(i) = Sin(i)
Next
End Function

Function setuppalette()
For i = 0 To 85
        r = i * 3
        g = i * 1.5
        b = 0
        palette(i) = (r Shl 16) + (g Shl 8) + b
Next
For i = 1 To 85
        r = 255
        g = 127 + (i * 1.5)
        b = i * 1.5
        palette(i + 85) = (r Shl 16) + (g Shl 8) + b
Next
For i = 1 To 85
        r = 255
        g = 255
        b = 127 + (i * 1.5)
        palette(i + 170) = (r Shl 16) + (g Shl 8) + b
Next
End Function

Function updateplasma()
LockBuffer

wave1# = wave1# + 1
wave2# = wave2# + 6
If wave1# >= 360 Then wave1# = wave1# - 360
If wave2# >= 360 Then wave2# = wave2# - 360

a# = wave1#
b# = wave2#
For y = 0 To height - 1
        For x = 0 To width - 1
                c = Abs((ctable#(a + x) + stable#(b) + ctable#(x + y)) * 85)
                argb = palette(c)
                WritePixelFast x,y,argb
                a# = a# + .0075
                b# = b# + .00375
                If a# >= 360 Then a# = a# - 360
                If b# >= 360 Then b# = b# - 360
        Next
Next

UnlockBuffer
End Function