; Fake 3D Track Test

AppTitle "Fake 3D Track Test"

Const width = 640
Const height = 480

Graphics width,height,0,2
SetBuffer BackBuffer()

Type seg
        Field x#
        Field y#
        Field z#
        Field cx#
        Field cy#
        Field cz#
        Field an#
        Field can#
        Field sx1#
        Field sx2#
        Field sy#
        Field lw#
        Field c
End Type

Dim track.seg(1)

Global segments

;Global bgimage = LoadImage("bg1.bmp")
Global trackimage = CreateImage(256,256)
;If ImageWidth(bgimage) <> width Or ImageHeight(bgimage) <> height Then ResizeImage bgimage,width,height

Global segwidth# = Float(Float(Float(width) / 640.0) * 400.0)
Global linewidth# = Float(Float(segwidth# / 500.0) * 22.0)

Global lex#
Global grz#

Global maxvel# = .6;.97
Global gforce# = .7

Global bgx

Global px#
Global py# = .4
Global pz#
Global pan#
Global pvel#
Global pseg
Global psegs#
Global plan#

;Global audio = PlayMusic("race.xm")
;ChannelVolume audio,.6

setuptrack(30)
setuptrackimage()

While Not KeyDown(1)
Cls

;TileBlock bgimage,bgx,0
drawtrack()
DrawImage trackimage,7,7

;pseg = pseg + 1
psegs# = psegs# + pvel#
If psegs# >= 1.0
        psegs# = psegs# - 1.0
        pseg = pseg + 1
EndIf
pseg2 = pseg + 1
If pseg < 0 Then pseg = pseg + segments
If pseg >= segments Then pseg = pseg - segments
If pseg2 >= segments Then pseg2 = pseg2 - segments
ps# = Float(1.0 - psegs#)
psx# = Float(track(pseg)\x# * ps#) + Float(track(pseg2)\x# * psegs#)
psz# = Float(track(pseg)\z# * ps#) + Float(track(pseg2)\z# * psegs#)
Color 200,200,200
Oval Int(Float(psx# - lex#) * 2.0) + 7,Int(-Float(psz# - grz#) * 2.0) + 7,5,5
Color 0,0,0
Oval Int(Float(psx# - lex#) * 2.0) + 7,Int(-Float(psz# - grz#) * 2.0) + 7,5,5,0
aan# = track(pseg)\an#
ban# = track(pseg2)\an#
If aan# >= 270.0 And ban# <= 0 Then aan# = aan# - 360.0
plan# = pan#
pan# = Float(aan# * Float(1.0 - psegs#)) + Float(ban# * psegs#)
If plan# >= 270.0 And pan# <= 0 Then plan# = plan# - 360.0
v# = Float(pvel# * .06)
If KeyDown(203) Then px# = px# - v#
If KeyDown(205) Then px# = px# + v#
px# = px# + Float(Float(Float(plan# - pan#) * v#) * gforce#)
If px# < -1.0 Then px# = -1.0
If px# > 1.0 Then px# = 1.0
If KeyDown(200)
        pvel# = pvel# + .01
        If pvel# > maxvel# Then pvel# = maxvel#
ElseIf pvel# > 0
        pvel# = pvel# - .006
        If pvel# < 0 Then pvel# = 0
EndIf

bgx = bgx + Int(Float(Float(plan# - pan#) * v#) * width)

Color 255,255,255
Text 2,2,pseg + "/" + segments

Flip
Wend
For i = 0 To segments - 1
        Delete track(i)
Next
;FreeImage bgimage
FreeImage trackimage
End

Function setuptrack(div)
Restore trackdata
Read segs
segments = segs * div
Dim track.seg(segments)
For i = 0 To segs - 1
        j = i * div
        track.seg(j) = New seg
        Read track(j)\x#
        Read track(j)\y#
        Read track(j)\z#
        Read track(j)\an#
        Read track(j)\cx#
        Read track(j)\cy#
        Read track(j)\cz#
        Read track(j)\can#
        track(j)\x# = Float(track(j)\x# * 10.0)
        track(j)\y# = Float(track(j)\y# * 10.0)
        track(j)\z# = Float(track(j)\z# * 10.0)
        track(j)\cx# = Float(track(j)\cx# * 10.0)
        track(j)\cy# = Float(track(j)\cy# * 10.0)
        track(j)\cz# = Float(track(j)\cz# * 10.0)
Next
c = False
For i = 0 To segs - 1
        j = i * div
        ax# = track(j)\x#
        ay# = track(j)\y#
        az# = track(j)\z#
        aan# = track(j)\an#
        bx# = track(j)\cx#
        by# = track(j)\cy#
        bz# = track(j)\cz#
        ban# = track(j)\can#
        j = i + 1
        If j >= segs Then j = j - segs
        j = j * div
        cx# = track(j)\x#
        cy# = track(j)\y#
        cz# = track(j)\z#
        can# = track(j)\an#
        If aan# >= 270.0 And can# <= 0 Then aan# = aan# - 360.0
;        bx# = Float(Float(ax# + cx#) * .5)
;        by# = Float(Float(ay# + cy#) * .5)
;        bz# = Float(Float(az# + cz#) * .5)
        ban# = Float(Float(aan# + can#) * .5)
        For j = 0 To div - 1
                k = (i * div) + j
                b# = Float(Float(j) / div)
                a# = Float(1.0 - b#)
                as# = Float(a# * a#)
                bs# = Float(b# * b#)
                track.seg(k) = New seg
                track(k)\x# = Float(ax# * as#) + Float(bx# * 2.0 * a# * b#) + Float(cx# * bs#)
                track(k)\y# = Float(ay# * as#) + Float(by# * 2.0 * a# * b#) + Float(cy# * bs#)
                track(k)\z# = Float(az# * as#) + Float(bz# * 2.0 * a# * b#) + Float(cz# * bs#)
                track(k)\an# = Float(aan# * as#) + Float(ban# * 2.0 * a# * b#) + Float(can# * bs#)
                track(k)\c = c
                c = Not c
        Next
Next
End Function

Function drawtrack()
m00# = Cos(pan#)
m01# = -Sin(pan#)
m10# = -m01#
m11# = m00#
ps# = Float(1.0 - psegs#)
pseg2 = pseg + 1
If pseg2 >= segments Then pseg2 = pseg2 - segments
psx# = Float(track(pseg)\x# * ps#) + Float(track(pseg2)\x# * psegs#)
psy# = Float(track(pseg)\y# * ps#) + Float(track(pseg2)\y# * psegs#)
psz# = Float(track(pseg)\z# * ps#) + Float(track(pseg2)\z# * psegs#)
For i = 0 To segments - 1
        tx# = track(i)\x# - psx#
        ty# = track(i)\y# - psy#
        tz# = track(i)\z# - psz#
        rx# = Float(tx# * m00#) + Float(tz# * m01#) - px#
        ry# = ty# - py#
        rz# = Float(tx# * m10#) + Float(tz# * m11#) - pz#
        w# = Float(segwidth# / rz#)
        sx = Int(Float(rx# / rz#) * 500.0) + (width Shr 1)
        track(i)\sy# = -Float(Float(ry# / rz#) * 500.0) + Float(height Shr 1)
        track(i)\sx1# = sx - Float(w# * .5)
        track(i)\sx2# = sx + Float(w# * .5)
        track(i)\lw# = Float(linewidth# / rz#)
Next
For i = pseg + 30 To pseg + 1 Step -1
        j = i
        If j >= segments Then j = j - segments
        k = j + 1
        If k >= segments Then k = k - segments
        If track(k)\c
                Color 0,125,0;200,110,140
        Else
                Color 0,150,0;200,160,200
        EndIf
        Rect 0,Int(track(k)\sy#),width,Int(track(j)\sy) - Int(track(k)\sy)
Next
LockBuffer
For i = pseg + 30 To pseg + 1 Step -1
        j = i
        If j >= segments Then j = j - segments
        k = j + 1
        If k >= segments Then k = k - segments
        If track(k)\c
                argb = $808080
        Else
                argb = $606060
        EndIf
        sx1# = Float(Float(track(k)\sx1# + track(j)\sx1#) * .5)
        sx2# = Float(Float(track(k)\sx2# + track(j)\sx2#) * .5)
        sy1# = track(k)\sy#
        sy2# = track(j)\sy#
        sy3# = Float(Float(sy1# + sy2#) * .5)
        lw1# = Float(track(k)\lw# * .5)
        lw2# = Float(track(j)\lw# * .5)
        lw3# = Float(Float(lw1# + lw2#) * .5)
        drawquad track(k)\sx1#,track(k)\sx2#,sy1#,track(j)\sx1#,track(j)\sx2#,sy2#,argb
        drawquad track(k)\sx1# - lw1#,track(k)\sx1#,sy1#,sx1# - lw3#,sx1#,sy3#,$FF0000
        drawquad track(k)\sx2#,track(k)\sx2# + lw1#,sy1#,sx2#,sx2# + lw3#,sy3#,$FF0000
        drawquad sx1# - lw3#,sx1#,sy3#,track(j)\sx1# - lw2#,track(j)\sx1#,sy2#,$FFFFFF
        drawquad sx2#,sx2# + lw3#,sy3#,track(j)\sx2#,track(j)\sx2# + lw2#,sy2#,$FFFFFF
Next
UnlockBuffer
End Function

Function setuptrackimage()
For i = 0 To segments - 1
        If track(i)\x# < lex# Then lex# = track(i)\x#
        If track(i)\z# > grz# Then grz# = track(i)\z#
Next
SetBuffer ImageBuffer(trackimage)
For i = 0 To segments - 1
        x = Int(Float(track(i)\x# - lex#) * 2.0)
        y = Int(-Float(track(i)\z# - grz#) * 2.0)
        Color 60,60,60
        Rect x + 3,y + 3,5,5
Next
For i = 0 To segments - 1
        x = Int(Float(track(i)\x# - lex#) * 2.0)
        y = Int(-Float(track(i)\z# - grz#) * 2.0)
        Color 130,130,130
        Rect x,y,5,5
Next
SetBuffer BackBuffer()
End Function

Function drawquad(x1#,x2#,y1#,x3#,x4#,y2#,argb)
sy1 = Int(y1#)
sy2 = Int(y2#)
h = sy2 - sy1
If h < 1 Then Return
sx1# = x1#
sx2# = x2#
sxi1# = Float(Float(x3# - x1#) / h)
sxi2# = Float(Float(x4# - x2#) / h)
If sy1 < 0 Then sy1 = 0
If sy2 >= height Then sy2 = height - 1
For y = sy1 To sy2
        lx = Int(sx1#)
        rx = Int(sx2#)
        If lx < 0 Then lx = 0
        If rx >= width Then rx = width - 1
        For x = lx To rx
                WritePixelFast x,y,argb
        Next
        sx1# = sx1# + sxi1#
        sx2# = sx2# + sxi2#
Next
End Function

.trackdata ; Basic Oval
Data 8
Data 0,0,0,0 ; 0
Data 0,0,.5,0
Data 0,0,1.0,0 ; 1
Data 0,0,2.0,45.0
Data 1.0,0,2.0,90.0 ; 2
Data 1.5,0,2.0,90.0
Data 2.0,0,2.0,90.0 ; 3
Data 3.0,0,2.0,135.0
Data 3.0,0,1.0,180.0 ; 4
Data 3.0,0,.5,180.0
Data 3.0,0,0,180.0 ; 5
Data 3.0,0,-1.0,225.0
Data 2.0,0,-1.0,270.0 ; 6
Data 1.5,0,-1.0,270.0
Data 1.0,0,-1.0,270.0 ; 7
Data 0,0,-1.0,-45.0