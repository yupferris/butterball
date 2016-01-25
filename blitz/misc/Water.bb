; Water Effect

; Simulates how water refracts light when disturbed

; (C) 2005 Jake Taylor [Thygrion]

AppTitle "Water Effect"

Const width = 320
Const height = 240

Graphics width,height,32,2

SeedRnd MilliSecs()

Dim water(width,height,2)
Dim wimage(width,height)

timage = LoadImage("sky.png") ; ANY image as big as the screen (320 by 240)
ResizeImage timage,width,height
SetBuffer ImageBuffer(timage)
LockBufferFor y = 0 To height - 1
        For x = 0 To width - 1
                wimage(x,y) = ReadPixelFast(x,y,ImageBuffer(timage)) And $FFFFFF
        Next
Next
UnlockBuffer
SetBuffer BackBuffer()
FreeImage timage

Global wdamping = 10

While Not KeyDown(1)
Cls

LockBuffer

updatewater()

WritePixelFast MouseX(),MouseY(),$FFFFFF

UnlockBuffer

Flip
Wend
End

Function updatewater()
If MouseDown(1) Then water(MouseX(),MouseY(),1) = water(MouseX(),MouseY(),1) - 500
If MouseHit(2) Then water(MouseX(),MouseY(),1) = water(MouseX(),MouseY(),1) - 50000
If KeyDown(57)
        For i = 1 To 5
                nx = Rand(1,width - 2)
                ny = Rand(1,height - 2)
                water(nx,ny,1) = water(nx,ny,1) - 500
        Next
EndIf
For y = 1 To height - 2
        For x = 1 To width - 2
                tw = ((water(x - 1,y,1) + water(x + 1,y,1) + water(x,y - 1,1) + water(x,y + 1,1)) / 2) - water(x,y,2)
                water(x,y,2) = tw - (tw / wdamping)
        Next
Next
For y = 1 To height - 2
        For x = 1 To width - 2
                xdis = ((water(x,y + 1,2) - water(x,y - 1,2)) - water(x,y,2)) / 8
                ydis = ((water(x + 1,y,2) - water(x - 1,y,2)) - water(x,y,2)) / 8
                wx = x + xdis
                wy = y + ydis
                If wx <= 0 Then wx = 1
                If wx >= width Then wx = width - 1
                If wy <= 0 Then wy = 1
                If wy >= height Then wy = height - 1
                argb = wimage(wx,wy)
;                If water(x,y,2) < 0 Then argb = $FF0000
;                If water(x,y,2) > 0 Then argb = $0000FF
                If argb <> $000000 Then WritePixelFast x,y,argb
;                If wx <> x Or wy <> y Then WritePixelFast x,y,$00FF00
                tw = water(x,y,2)
                water(x,y,2) = water(x,y,1)
                water(x,y,1) = tw
        Next
        water(0,y,1) = 0
        water(0,y,2) = 0
        water(width - 1,y,1) = 0
        water(width - 1,y,2) = 0
Next
For x = 0 To width - 1
        water(x,0,1) = 0
        water(x,0,2) = 0
        water(x,height - 1,1) = 0
        water(x,height - 1,2) = 0
Next
End Function