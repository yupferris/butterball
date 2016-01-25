AppTitle "Sweet Fireworks"

Const width = 640
Const height = 480

Graphics width,height,32,2
SetBuffer BackBuffer()

SeedRnd MilliSecs()

Type rocket
        Field x#
        Field y#
        Field z#
        Field xv#
        Field yv#
        Field zv#
        Field argb
        Field time
        Field timer
End Type

Type frag
        Field x#
        Field y#
        Field z#
        Field xv#
        Field yv#
        Field zv#
        Field r
        Field g
        Field b
        Field f#
        Field fv#
End Type

Dim screen(width,height)

Global camx#
Global camy#
Global camz# = -500.0

Global grav# = .01
Global damp# = .999

createrocket(camx#,camy#,camz# - 20.0)

While Not KeyDown(1)
Cls

LockBuffer
fireworks()
UnlockBuffer

If KeyDown(203) Then camx# = camx# - 20.0
If KeyDown(205) Then camx# = camx# + 20.0
If KeyDown(200) Then camy# = camy# + 20.0
If KeyDown(208) Then camy# = camy# - 20.0
If KeyDown(30) Then camz# = camz# + 20.0
If KeyDown(44) Then camz# = camz# - 20.0
If KeyHit(57) Then createrocket(camx#,camy#,camz# - 20.0)
If KeyHit(28) Then createfrags(camx#,camy#,camz# + 20.0,Rand(-$FF000,$FFFFFF))

Flip
Wend
Delete Each rocket
Delete Each frag
End

Function createrocket(x#,y#,z#)
r.rocket = New rocket
r\x# = x#
r\y# = y#
r\z# = z#
txan# = Rnd(-5.0,15.0)
tyan# = Rnd(-10.0,10.0)
tzan# = 0;Rnd(360.0)
cx# = Cos(txan#)
sx# = Sin(txan#)
cy# = Cos(tyan#)
sy# = Sin(tyan#)
cz# = Cos(tzan#)
sz# = Sin(tzan#)
cxsy# = Float(cx# * sy#)
m20# = Float(sx# * sz#) - Float(cxsy# * cz#)
m21# = Float(sx# * cz#) + Float(cxsy# * sz#)
m22# = Float(cx# * cy#)
tz# = 6.0
r\xv# = Float(tz# * m20#)
r\yv# = Float(tz# * m21#)
r\zv# = Float(tz# * m22#)
r\argb = Rand(-$FF000,$FFFFFF)
r\timer = Rand(100,2000)
r\time = MilliSecs()
End Function

Function createfrags(x#,y#,z#,argb)
If argb > 0 And argb < $686868 Then argb = $686868
For i = 0 To 699
        f.frag = New frag
        f\x# = x#
        f\y# = y#
        f\z# = z#
        txan# = Rnd(360.0)
        tyan# = Rnd(360.0)
        tzan# = Rnd(360.0)
        cx# = Cos(txan#)
        sx# = Sin(txan#)
        cy# = Cos(tyan#)
        sy# = Sin(tyan#)
        cz# = Cos(tzan#)
        sz# = Sin(tzan#)
        cxsy# = Float(cx# * sy#)
        m20# = Float(sx# * sz#) - Float(cxsy# * cz#)
        m21# = Float(sx# * cz#) + Float(cxsy# * sz#)
        m22# = Float(cx# * cy#)
        tz# = Rnd(.5,1.0)
        f\xv# = Float(tz# * m20#)
        f\yv# = Float(tz# * m21#)
        f\zv# = Float(tz# * m22#)
        If argb < 0
                targb = Rand($686868,$FFFFFF)
        Else
                targb = argb
        EndIf
        f\r = (targb Shr 16) And %11111111
        f\g = (targb Shr 8) And %11111111
        f\b = targb And %11111111
        f\f# = 1.0
        f\fv# = Rnd(.0025,.025)
Next
End Function

Function fireworks()
For r.rocket = Each rocket
        r\x# = r\x# + r\xv#
        r\y# = r\y# + r\yv#
        r\z# = r\z# + r\zv#
        r\xv# = r\xv# * damp#
        r\yv# = r\yv# * damp#
        r\zv# = r\zv# * damp#
;        r\yv# = r\yv# + grav#
        rx# = r\x# - camx#
        ry# = r\y# - camy#
        rz# = r\z# - camz#
        If rz# > 1.0
                sx# = Float(Float(rx# / rz#) * 300.0) + Float(width Shr 1)
                sy# = Float(-Float(ry# / rz#) * 300.0) + Float(height Shr 1)
                wupixel(sx#,sy#,128)
                wupixel(sx# - 1.0,sy#,128)
                wupixel(sx# + 1.0,sy#,128)
                wupixel(sx#,sy# - 1.0,128)
                wupixel(sx#,sy# + 1.0,128)
        EndIf
        If MilliSecs() >= r\time + r\timer
                createfrags(r\x#,r\y#,r\z#,r\argb)
                Delete r
        EndIf
Next
For f.frag = Each frag
        f\x# = f\x# + f\xv#
        f\y# = f\y# + f\yv#
        f\z# = f\z# + f\zv#
        f\xv# = f\xv# * damp#
        f\yv# = f\yv# * damp#
        f\zv# = f\zv# * damp#
        f\yv# = f\yv# - grav#
        fx# = f\x# - camx#
        fy# = f\y# - camy#
        fz# = f\z# - camz#
        If fz# > 1.0
                sx# = Float(Float(fx# / fz#) * 300.0) + Float(width Shr 1)
                sy# = Float(-Float(fy# / fz#) * 300.0) + Float(height Shr 1)
                wupixel2(sx#,sy#,((f\r * f\f#) Shl 16) + ((f\g * f\f#) Shl 8) + (f\b * f\f#))
        EndIf
        f\f# = f\f# - f\fv#
        If f\f# <= 0 Then Delete f
Next
For y = 0 To height - 1
        For x = 0 To width - 1
                argb = screen(x,y)
                If argb > $000000 Then WritePixelFast x,y,argb : screen(x,y) = 0
        Next
Next
End Function

Function wupixel(wx#,wy#,c)
Local x = Floor(wx#)
Local y = Floor(wy#)
Local xd# = Float(wx# - Float(x))
Local yd# = Float(wy# - Float(y))
Local c1# = Float(Float(1.0 - xd#) * Float(1.0 - yd#))
Local c2# = Float(xd# * Float(1.0 - yd#))
Local c3# = Float(Float(1.0 - xd#) * yd#)
Local c4# = Float(xd# * yd#)
pixel x,y,c1# * c
pixel x + 1,y,c2# * c
pixel x,y + 1,c3# * c
pixel x + 1,y + 1,c4# * c
End Function

Function wupixel2(wx#,wy#,argb)
Local x = Floor(wx#)
Local y = Floor(wy#)
Local xd# = Float(wx# - Float(x))
Local yd# = Float(wy# - Float(y))
Local c1# = Float(Float(1.0 - xd#) * Float(1.0 - yd#))
Local c2# = Float(xd# * Float(1.0 - yd#))
Local c3# = Float(Float(1.0 - xd#) * yd#)
Local c4# = Float(xd# * yd#)
Local r = (argb Shr 16) And %11111111
Local g = (argb Shr 8) And %11111111
Local b = argb And %11111111
Local r1 = r * c1#
Local g1 = g * c1#
Local b1 = b * c1#
Local r2 = r * c2#
Local g2 = g * c2#
Local b2 = b * c2#
Local r3 = r * c3#
Local g3 = g * c3#
Local b3 = b * c3#
Local r4 = r * c4#
Local g4 = g * c4#
Local b4 = b * c4#
pixel2 x,y,(r1 Shl 16) + (g1 Shl 8) + b1
pixel2 x + 1,y,(r2 Shl 16) + (g2 Shl 8) + b2
pixel2 x,y + 1,(r3 Shl 16) + (g3 Shl 8) + b3
pixel2 x + 1,y + 1,(r4 Shl 16) + (g4 Shl 8) + b4
End Function

Function pixel(x,y,c)
If x < 0 Or x >= width Or y < 0 Or y >= height Or c <= 0 Then Return
;bi = stab(x,y)
argb = screen(x,y);PeekInt(sbank,bi)
If argb > 0
        r = ((argb Shr 16) And %11111111) + c
        g = ((argb Shr 8) And %11111111) + c
        b = (argb And %11111111) + c
        If r > 255 Then r = 255
        If g > 255 Then g = 255
        If b > 255 Then b = 255
        argb = (r Shl 16) + (g Shl 8) + b
Else
        If c > 255 Then c = 255
        argb = (c Shl 16) + (c Shl 8) + c
EndIf
If argb > $000000 Then screen(x,y) = argb;PokeInt sbank,bi,argb
End Function

Function pixel2(x,y,argb)
If x < 0 Or x >= width Or y < 0 Or y >= height Or argb <= $000000 Then Return
r = (argb Shr 16) And %11111111
g = (argb Shr 8) And %11111111
b = argb And %11111111
;bi = stab(x,y)
argb = screen(x,y);PeekInt(sbank,bi)
If argb > 0
        r = ((argb Shr 16) And %11111111) + r
        g = ((argb Shr 8) And %11111111) + g
        b = (argb And %11111111) + b
EndIf
If r > 255 Then r = 255
If g > 255 Then g = 255
If b > 255 Then b = 255
argb = (r Shl 16) + (g Shl 8) + b
If argb > $000000 Then screen(x,y) = argb;PokeInt sbank,bi,argb
End Function
