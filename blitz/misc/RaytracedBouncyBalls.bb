;
; Raytraced Bouncy Balls
;

AppTitle "Raytraced Bouncy Balls"

Const width = 320
Const height = 240

Graphics width,height,32,2
SetBuffer BackBuffer()

SeedRnd MilliSecs()

Type plane
        Field nx#
        Field ny#
        Field nz#
        Field dis#
        Field argb
End Type

Type sphere
        Field x#
        Field y#
        Field z#
        Field xv#
        Field yv#
        Field zv#
        Field argb
End Type

Dim p.plane(1)

Dim s.sphere(1)

Dim normalx#(width,height)
Dim normaly#(width,height)
Dim normalz#(width,height)

Global planes = 1

Global spheres = 3

Global sphererad# = 100.0
Global sphererad2# = Float(sphererad# * sphererad#)

Global camerax#
Global cameray#
Global cameraz# = -800.0

Global lightx#
Global lighty# = 800.0
Global lightz#

Global spec# = 10.0

Global grav# = .3
Global fric# = .25

setupplanes()

setupspheres()
s(0)\argb = $FF0000
s(1)\argb = $00FF00
s(2)\argb = $0000FF

setupnormals()

While Not KeyDown(1)
Cls

LockBuffer
raytrace()
UnlockBuffer

For i = 0 To spheres - 1
        s(i)\x# = s(i)\x# + s(i)\xv#
        s(i)\y# = s(i)\y# + s(i)\yv#
        s(i)\z# = s(i)\z# + s(i)\zv#
        s(i)\yv# = s(i)\yv# - grav#
        If s(i)\y# - sphererad# < -p(0)\dis#
                s(i)\y# = -p(0)\dis# + sphererad#
                s(i)\xv# = Float(s(i)\xv# * fric#)
                s(i)\yv# = Float(-s(i)\yv# * fric#)
                s(i)\zv# = Float(s(i)\zv# * fric#)
                If s(i)\xv# > -fric# Or s(i)\xv# < fric# Then s(i)\xv# = 0
                If s(i)\zv# > -fric# Or s(i)\zv# < fric# Then s(i)\zv# = 0
        EndIf
Next

If KeyDown(203) Then camerax# = camerax# - 50.0
If KeyDown(205) Then camerax# = camerax# + 50.0
If KeyDown(200) Then cameraz# = cameraz# + 50.0
If KeyDown(208) Then cameraz# = cameraz# - 50.0

Flip
Wend
For i = 0 To planes - 1
        Delete p(i)
Next
For i = 0 To spheres - 1
        Delete s(i)
Next
End

Function setupplanes()
Dim p.plane(planes)
For i = 0 To planes - 1
        p.plane(i) = New plane
        p(i)\ny# = 1.0
        p(i)\dis# = Float(sphererad# * 4.0)
        p(i)\argb = $686868;Rand($686868,$FFFFFF)
Next
End Function

Function setupspheres()
Dim s.sphere(spheres)
Local sr# = Float(sphererad# * 2.5)
;Local an# = Rnd(360.0)
;Local ani# = Float(360.0 / spheres)
For i = 0 To spheres - 1
        s.sphere(i) = New sphere
        s(i)\x# = Rnd(-sr#,sr#)
        s(i)\y# = Rnd(0,sphererad#)
        s(i)\z# = Rnd(-sr#,sr#)
        s(i)\xv# = Rnd(-5.0,5.0)
        s(i)\yv# = Rnd(-2.0,7.0)
        s(i)\zv# = Rnd(-5.0,5.0)
        s(i)\argb = Rand($686868,$FFFFFF)
;        an# = an# + ani#
Next
End Function

Function setupnormals()
Local nx#
Local ny#
Local nz# = 200.0
Local dis#
For y = 0 To height - 1
        ny# = Float(height Shr 1) - Float(y)
        For x = 0 To width - 1
                nx# = Float(width Shr 1) - Float(x)
                dis# = Sqr(Float(nx# * nx#) + Float(ny# * ny#) + Float(nz# * nz#))
                normalx#(x,y) = -Float(nx# / dis#)
                normaly#(x,y) = Float(ny# / dis#)
                normalz#(x,y) = Float(nz# / dis#)
        Next
Next
End Function

Function raytrace()
Local argb
For y = 0 To height - 1
        For x = 0 To width - 1
                argb = ray(camerax#,cameray#,cameraz#,normalx#(x,y),normaly#(x,y),normalz#(x,y),256)
                If argb <> $000000 Then WritePixelFast x,y,argb
        Next
Next
End Function

Function ray(ex#,ey#,ez#,evx#,evy#,evz#,c)
If c <= 32 Then Return
Local plane.plane
Local sphere.sphere
Local z# = 10000
Local svx#
Local svy#
Local svz#
Local ix#
Local iy#
Local iz#
Local nx#
Local ny#
Local nz#
Local rnx#
Local rny#
Local rnz#
Local lvx#
Local lvy#
Local lvz#
Local dxis#
Local ydis#
Local zdis#
Local dxis2#
Local ydis2#
Local zdis2#
Local dis#
Local dis2#
Local l#
Local c1
Local c2
Local c3
Local r
Local g
Local b
For i = 0 To planes - 1
        plane.plane = p(i)
        nx# = plane\nx#
        ny# = plane\ny#
        nz# = plane\nz#
        dis# = Float(Float(nx# * evx#) + Float(ny# * evy#) + Float(nz# * evz#))
        If dis# < 0
                dis2# = Float(-Float(plane\dis# + Float(Float(nx# * ex#) + Float(ny# * ey#) + Float(nz# * ez#))) / dis#)
                ix# = ex# + Float(evx# * dis2#)
                iy# = ey# + Float(evy# * dis2#)
                iz# = ez# + Float(evz# * dis2#)
                lvx# = Float(lightx# - ix#)
                lvy# = Float(lighty# - iy#)
                lvz# = Float(lightz# - iz#)
                dis# = Sqr(Float(lvx# * lvx#) + Float(lvy# * lvy#) + Float(lvz# * lvz#))
                lvx# = Float(lvx# / dis#)
                lvy# = Float(lvy# / dis#)
                lvz# = Float(lvz# / dis#)
                l# = Float(Float(nx# * lvx#) + Float(ny# * lvy#) + Float(nz# * lvz#))
                dis# = Float(Float(Float(nx# * evx#) + Float(ny# * evy#) + Float(nz# * evz#)) * 2.0)
                rnx# = evx# - Float(nx# * dis#)
                rny# = evy# - Float(ny# * dis#)
                rnz# = evz# - Float(nz# * dis#)
                c1 = Int(l# * 256.0)
                c2 = Int(Float(Float(rnx# * lvx#) + Float(rny# * lvy#) + Float(rnz# * lvz#))^spec# * 256.0)
                argb = plane\argb
                z# = dis2#
        EndIf
Next
For i = 0 To spheres - 1
        sphere.sphere = s(i)
        svx# = sphere\x# - ex#
        svy# = sphere\y# - ey#
        svz# = sphere\z# - ez#
        dis2# = Float(Float(svx# * evx#) + Float(svy# * evy#) + Float(svz# * evz#))
        dis# = Float(Float(svx# * svx#) + Float(svy# * svy#) + Float(svz# * svz#)) - Float(dis2# * dis2#)
        If dis# <= sphererad2#
                dis2# = dis2# - Sqr(sphererad2# - dis#)
                If dis2# > 0 And dis2# < z#
                        ix# = ex# + Float(evx# * dis2#)
                        iy# = ey# + Float(evy# * dis2#)
                        iz# = ez# + Float(evz# * dis2#)
                        nx# = Float(Float(ix# - sphere\x#) / sphererad#)
                        ny# = Float(Float(iy# - sphere\y#) / sphererad#)
                        nz# = Float(Float(iz# - sphere\z#) / sphererad#)
                        lvx# = Float(lightx# - ix#)
                        lvy# = Float(lighty# - iy#)
                        lvz# = Float(lightz# - iz#)
                        dis# = Sqr(Float(lvx# * lvx#) + Float(lvy# * lvy#) + Float(lvz# * lvz#))
                        lvx# = Float(lvx# / dis#)
                        lvy# = Float(lvy# / dis#)
                        lvz# = Float(lvz# / dis#)
                        l# = Float(Float(nx# * lvx#) + Float(ny# * lvy#) + Float(nz# * lvz#))
                        dis# = Float(Float(Float(nx# * evx#) + Float(ny# * evy#) + Float(nz# * evz#)) * 2.0)
                        rnx# = evx# - Float(nx# * dis#)
                        rny# = evy# - Float(ny# * dis#)
                        rnz# = evz# - Float(nz# * dis#)
                        c1 = Int(l# * 256.0)
                        c2 = Int(Float(Float(rnx# * lvx#) + Float(rny# * lvy#) + Float(rnz# * lvz#))^spec# * 256.0)
                        argb = sphere\argb
                        z# = dis2#
                EndIf
        EndIf
Next
If shadowed(ix#,iy#,iz#)
        c1 = 30
        c2 = 0
Else
        If c1 < 30 Then c1 = 30
        If c2 < 0 Then c2 = 0
EndIf
c3 = (c1 * c) Shr 8
r = ((argb And $FF0000) * c3) Shr 8
g = ((argb And $00FF00) * c3) Shr 8
b = ((argb And $0000FF) * c3) Shr 8
r = r + (c2 Shl 16)
g = g + (c2 Shl 8)
b = b + c2
If argb <> $000000 Then argb = ray(ix#,iy#,iz#,rnx#,rny#,rnz#,c - 32)
r = r + (argb And $FF0000)
g = g + (argb And $00FF00)
b = b + (argb And $0000FF)
If r > $FF0000
        r = $FF0000
Else
        r = r And $FF0000
EndIf
If g > $00FF00
        g = $00FF00
Else
        g = g And $00FF00
EndIf
If b > $0000FF
        b = $0000FF
Else
        b = b And $0000FF
EndIf
Return r Or g Or b
End Function

Function shadowed(x#,y#,z#)
Local plane.plane
Local sphere.sphere
Local shad = False
Local evx# = Float(lightx# - x#)
Local evy# = Float(lighty# - y#)
Local evz# = Float(lightz# - z#)
Local dis# = Sqr(Float(evx# * evx#) + Float(evy# * evy#) + Float(evz# * evz#))
Local dis2#
evx# = Float(evx# / dis#)
evy# = Float(evy# / dis#)
evz# = Float(evz# / dis#)
For i = 0 To planes - 1
        plane.plane = p(i)
        nx# = plane\nx#
        ny# = plane\ny#
        nz# = plane\nz#
        dis# = Float(Float(nx# * evx#) + Float(ny# * evy#) + Float(nz# * evz#))
        If dis# < 0
                shad = True
                i = planes - 1
        EndIf
Next
If shad = False
        For i = 0 To spheres - 1
                sphere.sphere = s(i)
                svx# = sphere\x# - x#
                svy# = sphere\y# - y#
                svz# = sphere\z# - z#
                dis2# = Float(Float(svx# * evx#) + Float(svy# * evy#) + Float(svz# * evz#))
                dis# = Float(Float(svx# * svx#) + Float(svy# * svy#) + Float(svz# * svz#)) - Float(dis2# * dis2#)
                If dis2# > 0 And dis# < sphererad2#
                        shad = True
                        i = spheres - 1
                EndIf
        Next
EndIf
Return shad
End Function
