; Fire

AppTitle "Fire"

Const width = 320
Const height = 240

Graphics width,height,32,1

SetBuffer BackBuffer()

SeedRnd MilliSecs()

Dim fire(width,height)
Dim pallette(255)

createpallette()

Global cooling = 3

While Not KeyDown(1)
Cls

If MouseDown(1) And MouseX() > 1 And MouseX() < width - 1 And MouseY() > 1 And MouseY() < height - 1 Then createfire(MouseX(),MouseY())

updatefire()

LockBuffer BackBuffer()
WritePixelFast MouseX(),MouseY(),$FFFFFF,BackBuffer()
UnlockBuffer BackBuffer()

Flip
Wend
End

Function createpallette()
For i = 0 To 85
        r = i * 3
        g = 0
        b = 0
        pallette(i) = (r Shl 16) + (g Shl 8) + b
Next
For i = 0 To 85
        r = 255
        g = i * 3
        b = 0
        pallette(85 + i) = (r Shl 16) + (g Shl 8) + b
Next
For i = 0 To 85
        r = 255
        g = 255
        b = i * 3
        pallette(170 + i) = (r Shl 16) + (g Shl 8) + b
Next
End Function

Function createfire(x,y)
For yy = y - 3 To y + 3
        For xx = x - 3 To x + 3
                If xx >= 0 And xx <= width And yy >= 0 And yy <= height Then fire(xx,yy) = 255
        Next
Next
End Function

Function updatefire()
LockBuffer BackBuffer()

For y = 2 To height - 4
        For x = 2 To width - 4
                l = fire(x - 1,y)
                r = fire(x + 1,y)
                t = fire(x,y - 1)
                b = fire(x,y + 1)
                nc = ((l + r + t + b) / 4) - cooling
                If nc > 0
                        nx = x + Rand(-1,1)
                        ny = y - Rand(2)
                        fire(nx,ny) = nc
                        WritePixelFast x,y,pallette(fire(nx,ny)),BackBuffer()
                EndIf
        Next
Next

UnlockBuffer BackBuffer()
End Function