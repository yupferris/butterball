; 3D Library for Blitz Basic 2D

; (C) 2005 Jake Taylor [Thygrion]

Type point
	Field x#
	Field y#
	Field z#
	Field u#
	Field v#
	Field nx#
	Field ny#
	Field nz#
	Field rx#
	Field ry#
	Field rz#
	Field sx#
	Field sy#
End Type

Type face
	Field v[3]
	Field visible
End Type

Type entity
	Field x#
	Field y#
	Field z#
	Field xan#
	Field yan#
	Field zan#
	Field r#
	Field g#
	Field b#
	Field t
	Field parent
	Field hidden
	Field nl
	Field bfc
	Field rd
	Field alpha#
	Field sx#
	Field sy#
	Field sz#
	Field points
	Field faces
	Field p.point[10000]
	Field f.face[10000]
End Type

Type texture
	Field image
	Field masked
	Field srm
	Field r#[65536]
	Field g#[65536]
	Field b#[65536]
End Type

Const maxentities = 256

Const maxtextures = 12

Dim e.entity(maxentities)
Dim ee(maxentities)

Dim t.texture(maxtextures)
Dim te(maxtextures)

Dim screen#(1,1,2)
Dim screen2(1,1)
Dim background(1,1)

Global width = 0
Global height = 0

Global lightx# = 0
Global lighty# = 0
Global lightz# = 0
Global lightr# = 1
Global lightg# = 1
Global lightb# = 1

Global ambientr# = .5
Global ambientg# = .5
Global ambientb# = .5

Global wl# = 1

Global useshadows = False

Global bgimage

Global distance# = 0
Global size# = 0

Function graphics3d(w,h,d,m)
width = w
height = h
Dim screen#(width,height,2)
Dim screen2(width,height)
Dim background(width,height)
bgimage = CreateImage(width,height)
For y = 0 To height - 1
	For x = 0 To width - 1
		screen#(x,y,2) = 10000
	Next
Next
Graphics w,h,d,m
End Function

Function shadows(bool)
useshadows = bool
End Function

Function background3d(fname$)
bgimage = LoadImage(fname$)
ResizeImage bgimage,width,height
LockBuffer ImageBuffer(bgimage)
For y = 0 To height - 1
	For x = 0 To width - 1
		background(x,y) = ReadPixelFast(x,y,ImageBuffer(bgimage)) And $FFFFFF
		screen(x,y,1) = background(x,y)
	Next
Next
UnlockBuffer ImageBuffer(bgimage)
End Function

Function clearbackground3d()
LockBuffer ImageBuffer(bgimage)
For y = 0 To height - 1
	For x = 0 To width - 1
		WritePixelFast x,y,$000000,ImageBuffer(bgimage)
		background(x,y) = $000000
		screen(x,y,1) = $000000
	Next
Next
UnlockBuffer ImageBuffer(bgimage)
End Function

Function setdistance(d#)
distance# = d#
End Function

Function getdistance#()
Return distance#
End Function

Function setsize(s#)
size# = s#
End Function

Function renderworld()
DrawBlock bgimage,0,0
For eid = 1 To maxentities
	If ee(eid)
		If Not e(eid)\hidden
			cx# = Cos(e(eid)\xan#)
			sx# = Sin(e(eid)\xan#)
			cy# = Cos(e(eid)\yan#)
			sy# = Sin(e(eid)\yan#)
			cz# = Cos(e(eid)\zan#)
			sz# = Sin(e(eid)\zan#)
			If e(eid)\parent > 0
				pid = e(eid)\parent
				pcx# = Cos(e(pid)\xan#)
				psx# = Sin(e(pid)\xan#)
				pcy# = Cos(e(pid)\yan#)
				psy# = Sin(e(pid)\yan#)
				pcz# = Cos(e(pid)\zan#)
				psz# = Sin(e(pid)\zan#)
			EndIf
			For i = 1 To e(eid)\points
				x3d# = Float(e(eid)\p[i]\x# * e(eid)\sx#)
				y3d# = Float(e(eid)\p[i]\y# * e(eid)\sy#)
				z3d# = Float(e(eid)\p[i]\z# * e(eid)\sz#)
				tx# = Float(x3d# * cy#) + Float(z3d# * sy#)
				tz# = Float(z3d# * cy#) - Float(x3d# * sy#)
				ty# = Float(y3d# * cx#) + Float(tz# * sx#)
				tz# = Float(tz# * cx#) - Float(y3d# * sx#)
				tx2# = tx#
				tx# = e(eid)\x# + Float(tx# * cz#) + Float(ty# * sz#)
				ty# = e(eid)\y# + Float(ty# * cz#) - Float(tx2# * sz#)
				tz# = e(eid)\z# + tz#
				If e(eid)\parent > 0
					x3d# = tx#
					y3d# = ty#
					z3d# = tz#
					pid = e(eid)\parent
					tx# = Float(x3d# * pcy#) + Float(z3d# * psy#)
					tz# = Float(z3d# * pcy#) - Float(x3d# * psy#)
					ty# = Float(y3d# * pcx#) + Float(tz# * psx#)
					tz# = Float(tz# * pcx#) - Float(y3d# * psx#)
					tx2# = tx#
					tx# = e(pid)\x# + Float(tx# * pcz#) + Float(ty# * psz#)
					ty# = e(pid)\y# + Float(ty# * pcz#) - Float(tx2# * psz#)
					tz# = e(pid)\z# + tz#
				EndIf
				tz# = distance# + tz#
				e(eid)\p[i]\rx# = tx#
				e(eid)\p[i]\ry# = ty#
				e(eid)\p[i]\rz# = tz#
				e(eid)\p[i]\sx# = ((tx# / tz#) * size#) + (width / 2)
				e(eid)\p[i]\sy# = ((-ty# / tz#) * size#) + (height / 2)
				e(eid)\p[i]\nx# = 2
				e(eid)\p[i]\ny# = 0
				e(eid)\p[i]\nz# = 0
			Next
			For i = 1 To e(eid)\faces
				x1# = e(eid)\p[e(eid)\f[i]\v[1]]\rx#
				y1# = e(eid)\p[e(eid)\f[i]\v[1]]\ry#
				z1# = e(eid)\p[e(eid)\f[i]\v[1]]\rz#
				x2# = e(eid)\p[e(eid)\f[i]\v[2]]\rx#
				y2# = e(eid)\p[e(eid)\f[i]\v[2]]\ry#
				z2# = e(eid)\p[e(eid)\f[i]\v[2]]\rz#
				x3# = e(eid)\p[e(eid)\f[i]\v[3]]\rx#
				y3# = e(eid)\p[e(eid)\f[i]\v[3]]\ry#
				z3# = e(eid)\p[e(eid)\f[i]\v[3]]\rz#
				vx1# = Float(x1# - x2#)
				vy1# = Float(y1# - y2#)
				vz1# = Float(z1# - z2#)
				vx2# = Float(x2# - x3#)
				vy2# = Float(y2# - y3#)
				vz2# = Float(z2# - z3#)
				mag1# = Sqr(Float(vx1# * vx1#) + Float(vy1# * vy1#) + Float(vz1# * vz1#))
				mag2# = Sqr(Float(vx2# * vx2#) + Float(vy2# * vy2#) + Float(vz2# * vz2#))
				vx1# = Float(vx1# / mag1#)
				vy1# = Float(vy1# / mag1#)
				vz1# = Float(vz1# / mag1#)
				vx2# = Float(vx2# / mag2#)
				vy2# = Float(vy2# / mag2#)
				vz2# = Float(vz2# / mag2#)
				nx# = Float(vy1# * vz2#) - Float(vz1# * vy2#)
				ny# = Float(vz1# * vx2#) - Float(vx1# * vz2#)
				nz# = Float(vx1# * vy2#) - Float(vy1# * vx2#)
				vx# = Float(x1# + x2# + x3#) / 3.0
				vy# = Float(y1# + y2# + y3#) / 3.0
				vz# = Float(z1# + z2# + z3#) / 3.0
				mag# = Sqr(Float(vx# * vx#) + Float(vy# * vy#) + Float(vz# * vz#))
				vx# = Float(vx# / mag#)
				vy# = Float(vy# / mag#)
				vz# = Float(vz# / mag#)
				cull# = Float(nx# * vx#) + Float(ny# * vy#) + Float(nz# * vz#)
				If cull# > 0
					e(eid)\f[i]\visible = False
				Else
					e(eid)\f[i]\visible = True
				EndIf
				For j = 1 To 3
					If e(eid)\p[e(eid)\f[i]\v[j]]\nx# = 2
						e(eid)\p[e(eid)\f[i]\v[j]]\nx# = nx#
						e(eid)\p[e(eid)\f[i]\v[j]]\ny# = ny#
						e(eid)\p[e(eid)\f[i]\v[j]]\nz# = nz#
					Else
						e(eid)\p[e(eid)\f[i]\v[j]]\nx# = Float(e(eid)\p[e(eid)\f[i]\v[j]]\nx# + nx#) / 2
						e(eid)\p[e(eid)\f[i]\v[j]]\ny# = Float(e(eid)\p[e(eid)\f[i]\v[j]]\ny# + ny#) / 2
						e(eid)\p[e(eid)\f[i]\v[j]]\nz# = Float(e(eid)\p[e(eid)\f[i]\v[j]]\nz# + nz#) / 2
					EndIf
				Next
			Next
			For i = 1 To e(eid)\faces
				If e(eid)\f[i]\visible Or e(eid)\bfc = False
					z1# = e(eid)\p[e(eid)\f[i]\v[1]]\rz#
					z2# = e(eid)\p[e(eid)\f[i]\v[2]]\rz#
					z3# = e(eid)\p[e(eid)\f[i]\v[3]]\rz#
					x1# = Float(e(eid)\p[e(eid)\f[i]\v[1]]\rx# / z1#)
					y1# = Float(e(eid)\p[e(eid)\f[i]\v[1]]\ry# / z1#)
					x2# = Float(e(eid)\p[e(eid)\f[i]\v[2]]\rx# / z2#)
					y2# = Float(e(eid)\p[e(eid)\f[i]\v[2]]\ry# / z2#)
					x3# = Float(e(eid)\p[e(eid)\f[i]\v[3]]\rx# / z3#)
					y3# = Float(e(eid)\p[e(eid)\f[i]\v[3]]\ry# / z3#)
					nx1# = Float(e(eid)\p[e(eid)\f[i]\v[1]]\nx# / z1#)
					ny1# = Float(e(eid)\p[e(eid)\f[i]\v[1]]\ny# / z1#)
					nz1# = Float(e(eid)\p[e(eid)\f[i]\v[1]]\nz# / z1#)
					nx2# = Float(e(eid)\p[e(eid)\f[i]\v[2]]\nx# / z2#)
					ny2# = Float(e(eid)\p[e(eid)\f[i]\v[2]]\ny# / z2#)
					nz2# = Float(e(eid)\p[e(eid)\f[i]\v[2]]\nz# / z2#)
					nx3# = Float(e(eid)\p[e(eid)\f[i]\v[3]]\nx# / z3#)
					ny3# = Float(e(eid)\p[e(eid)\f[i]\v[3]]\ny# / z3#)
					nz3# = Float(e(eid)\p[e(eid)\f[i]\v[3]]\nz# / z3#)
					sx1 = e(eid)\p[e(eid)\f[i]\v[1]]\sx
					sy1 = e(eid)\p[e(eid)\f[i]\v[1]]\sy
					sx2 = e(eid)\p[e(eid)\f[i]\v[2]]\sx
					sy2 = e(eid)\p[e(eid)\f[i]\v[2]]\sy
					sx3 = e(eid)\p[e(eid)\f[i]\v[3]]\sx
					sy3 = e(eid)\p[e(eid)\f[i]\v[3]]\sy
					u1# = Float(e(eid)\p[e(eid)\f[i]\v[1]]\u# / z1#)
					v1# = Float(e(eid)\p[e(eid)\f[i]\v[1]]\v# / z1#)
					u2# = Float(e(eid)\p[e(eid)\f[i]\v[2]]\u# / z2#)
					v2# = Float(e(eid)\p[e(eid)\f[i]\v[2]]\v# / z2#)
					u3# = Float(e(eid)\p[e(eid)\f[i]\v[3]]\u# / z3#)
					v3# = Float(e(eid)\p[e(eid)\f[i]\v[3]]\v# / z3#)
					z1# = Float(1.0 / z1#)
					z2# = Float(1.0 / z2#)
					z3# = Float(1.0 / z3#)
					alpha# = e(eid)\alpha#
					If sx1 >= 0 Or sy1 >= 0 Or sx1 < width Or sy1 < height Or sx2 >= 0 Or sy2 >= 0 Or sx2 < width Or sy2 < height Or sx3 >= 0 Or sy3 >= 0 Or sx3 < width Or sy3 < height
						If e(eid)\t > 0
							textriangle sx1,sy1,sx2,sy2,sx3,sy3,x1#,y1#,z1#,x2#,y2#,z2#,x3#,y3#,z3#,nx1#,ny1#,nz1#,nx2#,ny2#,nz2#,nx3#,ny3#,nz3#,u1#,v1#,u2#,v2#,u3#,v3#,eid,alpha#
						Else
							fr# = e(eid)\r#
							fg# = e(eid)\g#
							fb# = e(eid)\b#
							triangle sx1,sy1,sx2,sy2,sx3,sy3,x1#,y1#,z1#,x2#,y2#,z2#,x3#,y3#,z3#,nx1#,ny1#,nz1#,nx2#,ny2#,nz2#,nx3#,ny3#,nz3#,fr#,fg#,fb#,eid,alpha#
						EndIf
					EndIf
				EndIf
			Next
			For y = 0 To height - 1
				For x = 0 To width - 1
					If screen2(x,y) <> $000000 Then screen(x,y,1) = screen2(x,y) : screen2(x,y) = $000000
				Next
			Next
		EndIf
	EndIf
Next
If useshadows
	shadowz# = 9990
	For eid = 1 To maxentities
		If ee(eid)
			If e(eid)\hidden = False And e(eid)\alpha# > 0
				alpha# = e(eid)\alpha#
				ox = Float(e(eid)\x# - lightx#) * Float(Float(shadowz# - e(eid)\z#) / 10000)
				oy = -Float(e(eid)\y# - lighty#) * Float(Float(shadowz# - e(eid)\z#) / 10000)
				For i = 1 To e(eid)\faces
					If e(eid)\f[i]\visible Or e(eid)\bfc = False
						sx1 = ox + e(eid)\p[e(eid)\f[i]\v[1]]\sx
						sy1 = oy + e(eid)\p[e(eid)\f[i]\v[1]]\sy
						sx2 = ox + e(eid)\p[e(eid)\f[i]\v[2]]\sx
						sy2 = oy + e(eid)\p[e(eid)\f[i]\v[2]]\sy
						sx3 = ox + e(eid)\p[e(eid)\f[i]\v[3]]\sx
						sy3 = oy + e(eid)\p[e(eid)\f[i]\v[3]]\sy
						striangle sx1,sy1,sx2,sy2,sx3,sy3,shadowz#,alpha#
					EndIf
				Next
				For y = 0 To height - 1
					For x = 0 To width - 1
						If screen2(x,y) <> $000000 Then screen(x,y,1) = screen2(x,y) : screen2(x,y) = $000000
					Next
				Next
			EndIf
		EndIf
	Next
EndIf
LockBuffer
For y = 0 To height - 1
	For x = 0 To width - 1
		argb = screen(x,y,1)
		If argb <> $000000
			r = ((argb Shr 16) And 255) * wl#
			g = ((argb Shr 8) And 255) * wl#
			b = (argb And 255) * wl#
			argb = (r Shl 16) + (g Shl 8) + b
		EndIf
		If argb <> background(x,y) Then WritePixelFast x,y,argb
		screen(x,y,1) = background(x,y)
		screen#(x,y,2) = 10000
	Next
Next
UnlockBuffer
End Function

Function clearworld()
For i = 1 To maxentities
	If ee(i)
		For j = 1 To e(i)\points
			Delete e(i)\p[j]
		Next
		For j = 1 To e(i)\faces
			Delete e(i)\f[j]
		Next
		Delete e(i)
	EndIf
Next
For i = 1 To maxtextures
	If te(i)
		FreeImage t(i)\image
		Delete t(i)
	EndIf
Next
FreeImage bgimage
End Function

Function positionlight(x#,y#,z#)
lightx# = x#
lighty# = y#
lightz# = z#
End Function

Function lightcolor(r#,g#,b#)
If r# < 0 Then r# = 0
If g# < 0 Then g# = 0
If b# < 0 Then b# = 0
If r# > 255 Then r# = 255
If g# > 255 Then g# = 255
If b# > 255 Then b# = 255
lightr# = Float(r# / 255.0)
lightg# = Float(g# / 255.0)
lightb# = Float(b# / 255.0)
End Function

Function ambientlight(r#,g#,b#)
If r# < 0 Then r# = 0
If g# < 0 Then g# = 0
If b# < 0 Then b# = 0
If r# > 255 Then r# = 255
If g# > 255 Then g# = 255
If b# > 255 Then b# = 255
ambientr# = Float(r# / 255.0)
ambientg# = Float(g# / 255.0)
ambientb# = Float(b# / 255.0)
End Function

Function worldlight(l#)
wl# = l#
End Function

Function positionentity(eid,x#,y#,z#)
e(eid)\x# = x#
e(eid)\y# = y#
e(eid)\z# = z#
End Function

Function entityx#(eid)
Return e(eid)\x#
End Function

Function entityy#(eid)
Return e(eid)\y#
End Function

Function entityz#(eid)
Return e(eid)\z#
End Function

Function rotateentity(eid,x#,y#,z#)
e(eid)\xan# = x#
e(eid)\yan# = y#
e(eid)\zan# = z#
End Function

Function turnentity(eid,x#,y#,z#)
e(eid)\xan# = e(eid)\xan# + x#
e(eid)\yan# = e(eid)\yan# + y#
e(eid)\zan# = e(eid)\zan# + z#
End Function

Function scaleentity(eid,x#,y#,z#)
e(eid)\sx# = x#
e(eid)\sy# = y#
e(eid)\sz# = z#
End Function

Function flipmesh(eid)
For i = 1 To e(eid)\faces
	tv = e(eid)\f[i]\v[1]
	e(eid)\f[i]\v[1] = e(eid)\f[i]\v[3]
	e(eid)\f[i]\v[3] = tv
Next
End Function

Function entitycolor(eid,r#,g#,b#)
If r# < 0 Then r# = 0
If g# < 0 Then g# = 0
If b# < 0 Then b# = 0
If r# > 255 Then r# = 255
If g# > 255 Then g# = 255
If b# > 255 Then b# = 255
e(eid)\r# = Float(r# / 255.0)
e(eid)\g# = Float(g# / 255.0)
e(eid)\b# = Float(b# / 255.0)
End Function

Function entitytexture(eid,tid)
e(eid)\t = tid
End Function

Function entityparent(eid,pid)
e(eid)\parent = pid
End Function

Function entitynolight(eid)
e(eid)\nl = True
End Function

Function entitynobfc(eid)
e(eid)\bfc = False
End Function

Function entityrd(eid,rd)
e(eid)\rd = rd
End Function

Function hideentity(eid)
e(eid)\hidden = True
End Function

Function unhideentity(eid)
e(eid)\hidden = False
End Function

Function entityalpha(eid,alpha#)
e(eid)\alpha# = alpha#
End Function

Function loadtexture(fname$)
image = LoadImage(fname$)
If ImageWidth(image) <> 256 Or ImageHeight(image) <> 256 Then ResizeImage image,256,256
tid = 0
For i = 1 To maxtextures
	If Not te(i) Then tid = i : Exit
Next
te(i) = True
t.texture(tid) = New texture
t(tid)\image = image
LockBuffer ImageBuffer(image)
For y = 0 To 255
	For x = 0 To 255
		argb = ReadPixelFast(x,y,ImageBuffer(image))
		r = (argb Shr 16) And 255
		g = (argb Shr 8) And 255
		b = argb And 255
		t(tid)\r#[(y * 256) + x] = Float(r / 255.0)
		t(tid)\g#[(y * 256) + x] = Float(g / 255.0)
		t(tid)\b#[(y * 256) + x] = Float(b / 255.0)
	Next
Next
UnlockBuffer ImageBuffer(image)
FreeImage image
Return tid
End Function

Function masktexture(tid)
t(tid)\masked = True
End Function

Function texturesrm(tid)
t(tid)\srm = True
End Function

Function loadascii(fname$)
eid = 0
For i = 1 To maxentities
	If Not ee(i) Then eid = i : Exit
Next
ee(eid) = True
e.entity(eid) = New entity
e(eid)\bfc = True
e(eid)\rd = 0
e(eid)\alpha# = 1
e(eid)\sx# = 1
e(eid)\sy# = 1
e(eid)\sz# = 1
e(eid)\r# = 1
e(eid)\g# = 1
e(eid)\b# = 1
file = ReadFile(fname$)
crap$ = ReadLine$(file)
crap$ = ReadLine$(file)
l$ = ReadLine$(file)
surfaces = Int(Right$(l$,Len(l$) - 8))
For i = 1 To surfaces
	crap$ = ReadLine$(file)
	points = Int(ReadLine$(file))
	e(eid)\points = points
	For j = 1 To points
		e(eid)\p.point[j] = New point
		l$ = ReadLine$(file)
		value$ = ""
		a = 1
		For k = 1 To Len(l$) - 4
			c$ = Mid$(l$,k + 2,1)
			If c$ = " "
				If a = 1
					e(eid)\p[j]\x# = Float(v$)
				ElseIf a = 2
					e(eid)\p[j]\y# = Float(v$)
				ElseIf a = 3
					e(eid)\p[j]\z# = Float(v$)
				ElseIf a = 4
					e(eid)\p[j]\u# = Float(v$)
				ElseIf a = 5
					e(eid)\p[j]\v# = Float(v$)
				EndIf
				a = a + 1
				v$ = ""
			Else
				v$ = v$ + c$
			EndIf
		Next
	Next
	normals = Int(ReadLine$(file))
	For j = 1 To normals
		crap$ = ReadLine$(file)
	Next
	faces = Int(ReadLine$(file))
	e(eid)\faces = faces
	For j = 1 To faces
		e(eid)\f.face[j] = New face
		l$ = ReadLine$(file)
		v$ = ""
		a = 1
		For k = 1 To Len(l$) - 5
			c$ = Mid$(l$,k + 2,1)
			If c$ = " "
				If a = 1
					e(eid)\f[j]\v[1] = Int(v$) + 1
				ElseIf a = 2
					e(eid)\f[j]\v[2] = Int(v$) + 1
				ElseIf a = 3
					e(eid)\f[j]\v[3] = Int(v$) + 1
				EndIf
				a = a + 1
				v$ = ""
			Else
				v$ = v$ + c$
			EndIf
		Next
	Next
Next
CloseFile file
Return eid
End Function

Function triangle(sx1,sy1,sx2,sy2,sx3,sy3,x1#,y1#,z1#,x2#,y2#,z2#,x3#,y3#,z3#,nx1#,ny1#,nz1#,nx2#,ny2#,nz2#,nx3#,ny3#,nz3#,fr#,fg#,fb#,eid,alpha#)
a2# = Float(1.0 - alpha#)
If sy2 < sy1
	tsx = sx1
	tsy = sy1
	tx# = x1#
	ty# = y1#
	tz# = z1#
	tnx# = nx1#
	tny# = ny1#
	tnz# = nz1#
	sx1 = sx2
	sy1 = sy2
	x1# = x2#
	y1# = y2#
	z1# = z2#
	nx1# = nx2#
	ny1# = ny2#
	nz1# = nz2#
	sx2 = tsx
	sy2 = tsy
	x2# = tx#
	y2# = ty#
	z2# = tz#
	nx2# = tnx#
	ny2# = tny#
	nz2# = tnz#
EndIf
If sy3 < sy1
	tsx = sx1
	tsy = sy1
	tx# = x1#
	ty# = y1#
	tz# = z1#
	tnx# = nx1#
	tny# = ny1#
	tnz# = nz1#
	sx1 = sx3
	sy1 = sy3
	x1# = x3#
	y1# = y3#
	z1# = z3#
	nx1# = nx3#
	ny1# = ny3#
	nz1# = nz3#
	sx3 = tsx
	sy3 = tsy
	x3# = tx#
	y3# = ty#
	z3# = tz#
	nx3# = tnx#
	ny3# = tny#
	nz3# = tnz#
EndIf
If sy3 < sy2
	tsx = sx2
	tsy = sy2
	tx# = x2#
	ty# = y2#
	tz# = z2#
	tnx# = nx2#
	tny# = ny2#
	tnz# = nz2#
	sx2 = sx3
	sy2 = sy3
	x2# = x3#
	y2# = y3#
	z2# = z3#
	nx2# = nx3#
	ny2# = ny3#
	nz2# = nz3#
	sx3 = tsx
	sy3 = tsy
	x3# = tx#
	y3# = ty#
	z3# = tz#
	nx3# = tnx#
	ny3# = tny#
	nz3# = tnz#
EndIf
ratio# = Float(sy2 - sy1) / Float(sy3 - sy1)
sx4 = Float((sx3 - sx1) * ratio#) + sx1
sy4 = sy2
x4# = Float(Float(x3# - x1#) * ratio#) + x1#
y4# = y2#
z4# = Float(Float(z3# - z1#) * ratio#) + z1#
nx4# = Float(Float(nx3# - nx1#) * ratio#) + nx1#
ny4# = Float(Float(ny3# - ny1#) * ratio#) + ny1#
nz4# = Float(Float(nz3# - nz1#) * ratio#) + nz1#
If sx4 < sx2
	tsx = sx2
	tx# = x2#
	ty# = y2#
	tz# = z2#
	tnx# = nx2#
	tny# = ny2#
	tnz# = nz2#
	sx2 = sx4
	x2# = x4#
	y2# = y4#
	z2# = z4#
	nx2# = nx4#
	ny2# = ny4#
	nz2# = nz4#
	sx4 = tsx
	x4# = tx#
	y4# = ty#
	z4# = tz#
	nx4# = tnx#
	ny4# = tny#
	nz4# = tnz#
EndIf

h# = Float(sy2 - sy1)
If h# > 0
	sl# = sx1
	sr# = sx1
	lx# = x1#
	ly# = y1#
	lz# = z1#
	rx# = x1#
	ry# = y1#
	rz# = z1#
	lnx# = nx1#
	lny# = ny1#
	lnz# = nz1#
	rnx# = nx1#
	rny# = ny1#
	rnz# = nz1#
	lsv# = Float(sx2 - sx1) / h#
	rsv# = Float(sx4 - sx1) / h#
	lxv# = Float(x2# - x1#) / h#
	lyv# = Float(y2# - y1#) / h#
	lzv# = Float(z2# - z1#) / h#
	rxv# = Float(x4# - x1#) / h#
	ryv# = Float(y4# - y1#) / h#
	rzv# = Float(z4# - z1#) / h#
	lnxv# = Float(nx2# - nx1#) / h#
	lnyv# = Float(ny2# - ny1#) / h#
	lnzv# = Float(nz2# - nz1#) / h#
	rnxv# = Float(nx4# - nx1#) / h#
	rnyv# = Float(ny4# - ny1#) / h#
	rnzv# = Float(nz4# - nz1#) / h#
	For i = sy1 To sy2
		hline sl,sr,i,lx#,rx#,ly#,ry#,lz#,rz#,lnx#,rnx#,lny#,rny#,lnz#,rnz#,fr#,fg#,fb#,eid,alpha#,a2#
		sl# = sl# + lsv#
		sr# = sr# + rsv#
		lx# = lx# + lxv#
		ly# = ly# + lyv#
		lz# = lz# + lzv#
		rx# = rx# + rxv#
		ry# = ry# + ryv#
		rz# = rz# + rzv#
		lnx# = lnx# + lnxv#
		lny# = lny# + lnyv#
		lnz# = lnz# + lnzv#
		rnx# = rnx# + rnxv#
		rny# = rny# + rnyv#
		rnz# = rnz# + rnzv#
	Next
EndIf

h# = Float(sy3 - sy2)
If h# > 0
	sl# = sx3
	sr# = sx3
	lx# = x3#
	ly# = y3#
	lz# = z3#
	rx# = x3#
	ry# = y3#
	rz# = z3#
	lnx# = nx3#
	lny# = ny3#
	lnz# = nz3#
	rnx# = nx3#
	rny# = ny3#
	rnz# = nz3#
	lsv# = Float(sx2 - sx3) / h#
	rsv# = Float(sx4 - sx3) / h#
	lxv# = Float(x2# - x3#) / h#
	lyv# = Float(y2# - y3#) / h#
	lzv# = Float(z2# - z3#) / h#
	rxv# = Float(x4# - x3#) / h#
	ryv# = Float(y4# - y3#) / h#
	rzv# = Float(z4# - z3#) / h#
	lnxv# = Float(nx2# - nx3#) / h#
	lnyv# = Float(ny2# - ny3#) / h#
	lnzv# = Float(nz2# - nz3#) / h#
	rnxv# = Float(nx4# - nx3#) / h#
	rnyv# = Float(ny4# - ny3#) / h#
	rnzv# = Float(nz4# - nz3#) / h#
	For i = sy3 To sy2 + 1 Step -1
		hline sl,sr,i,lx#,rx#,ly#,ry#,lz#,rz#,lnx#,rnx#,lny#,rny#,lnz#,rnz#,fr#,fg#,fb#,eid,alpha#,a2#
		sl# = sl# + lsv#
		sr# = sr# + rsv#
		lx# = lx# + lxv#
		ly# = ly# + lyv#
		lz# = lz# + lzv#
		rx# = rx# + rxv#
		ry# = ry# + ryv#
		rz# = rz# + rzv#
		lnx# = lnx# + lnxv#
		lny# = lny# + lnyv#
		lnz# = lnz# + lnzv#
		rnx# = rnx# + rnxv#
		rny# = rny# + rnyv#
		rnz# = rnz# + rnzv#
	Next
EndIf
End Function

Function hline(fx,tx,yy,lx#,rx#,ly#,ry#,lz#,rz#,lnx#,rnx#,lny#,rny#,lnz#,rnz#,fr#,fg#,fb#,eid,a1#,a2#)
rd = e(eid)\rd
diffuser# = fr#
diffuseg# = fg#
diffuseb# = fb#
w# = Float(tx - fx)
xv# = Float(rx# - lx#) / w#
yv# = Float(ry# - ly#) / w#
zv# = Float(rz# - lz#) / w#
nxv# = Float(rnx# - lnx#) / w#
nyv# = Float(rny# - lny#) / w#
nzv# = Float(rnz# - lnz#) / w#
cx# = lx#
cy# = ly#
cz# = lz#
cnx# = lnx#
cny# = lny#
cnz# = lnz#
For xx = fx To tx
	If xx >= 0 And xx < width And yy >= 0 And yy < height
		cz2# = Float(1.0 / cz#)
		If cz2# >= 2 And cz2# <= screen#(xx,yy,2)
			cx2# = Float(cx# * cz2#)
			cy2# = Float(cy# * cz2#)
			cnx2# = Float(cnx# * cz2#)
			cny2# = Float(cny# * cz2#)
			cnz2# = Float(cnz# * cz2#)
			If e(eid)\nl
				nr# = diffuser# * 255
				ng# = diffuseg# * 255
				nb# = diffuseb# * 255
			Else
				lvx# = Float(cx2# - lightx#)
				lvy# = Float(cy2# - lighty#)
				lvz# = Float(cz2# - lightz#)
				lmag# = Sqr(Float(lvx# * lvx#) + Float(lvy# * lvy#) + Float(lvz# * lvz#))
				lvx# = Float(lvx# / lmag#)
				lvy# = Float(lvy# / lmag#)
				lvz# = Float(lvz# / lmag#)
				dot1# = Float(cnx2# * lvx#) + Float(cny2# * lvy#) + Float(cnz2# * lvz#)
				If dot1# < 0
					dot1# = -dot1#
				Else
					dot1# = 0
				EndIf
				nar# = Float(ambientr# * diffuser#)
				nag# = Float(ambientg# * diffuseg#)
				nab# = Float(ambientb# * diffuseb#)
				ndr# = Float(diffuser# * dot1#)
				ndg# = Float(diffuseg# * dot1#)
				ndb# = Float(diffuseb# * dot1#)
				nr# = Float(nar# + Float(lightr# * ndr#)) * 255.0
				ng# = Float(nag# + Float(lightg# * ndg#)) * 255.0
				nb# = Float(nab# + Float(lightb# * ndb#)) * 255.0
			EndIf
			If nr# < 5 Then nr# = 5
			If ng# < 5 Then ng# = 5
			If nb# < 5 Then nb# = 5
			If nr# > 255 Then nr# = 255
			If ng# > 255 Then ng# = 255
			If nb# > 255 Then nb# = 255
			nr# = nr# * a1#
			ng# = ng# * a1#
			nb# = nb# * a1#
			dx = xx - (cnx2# * rd)
			dy = yy + (cny2# * rd)
			If dx < 0 Then dx = 0
			If dx > width - 1 Then dx = width - 1
			If dy < 0 Then dy = 0
			If dy > height - 1 Then dy = height - 1
			argb = screen(dx,dy,1)
			nr# = nr# + (((argb Shr 16) And 255) * a2#)
			ng# = ng# + (((argb Shr 8) And 255) * a2#)
			nb# = nb# + ((argb And 255) * a2#)
			screen2(xx,yy) = (nr# Shl 16) + (ng# Shl 8) + nb#
			screen#(xx,yy,2) = cz2#
		EndIf
	EndIf
	cx# = cx# + xv#
	cy# = cy# + yv#
	cz# = cz# + zv#
	cnx# = cnx# + nxv#
	cny# = cny# + nyv#
	cnz# = cnz# + nzv#
Next
End Function

Function textriangle(sx1,sy1,sx2,sy2,sx3,sy3,x1#,y1#,z1#,x2#,y2#,z2#,x3#,y3#,z3#,nx1#,ny1#,nz1#,nx2#,ny2#,nz2#,nx3#,ny3#,nz3#,u1#,v1#,u2#,v2#,u3#,v3#,eid,alpha#)
a2# = Float(1.0 - alpha#)
If sy2 < sy1
	tsx = sx1
	tsy = sy1
	tx# = x1#
	ty# = y1#
	tz# = z1#
	tu# = u1#
	tv# = v1#
	tnx# = nx1#
	tny# = ny1#
	tnz# = nz1#
	sx1 = sx2
	sy1 = sy2
	x1# = x2#
	y1# = y2#
	z1# = z2#
	u1# = u2#
	v1# = v2#
	nx1# = nx2#
	ny1# = ny2#
	nz1# = nz2#
	sx2 = tsx
	sy2 = tsy
	x2# = tx#
	y2# = ty#
	z2# = tz#
	u2# = tu#
	v2# = tv#
	nx2# = tnx#
	ny2# = tny#
	nz2# = tnz#
EndIf
If sy3 < sy1
	tsx = sx1
	tsy = sy1
	tx# = x1#
	ty# = y1#
	tz# = z1#
	tu# = u1#
	tv# = v1#
	tnx# = nx1#
	tny# = ny1#
	tnz# = nz1#
	sx1 = sx3
	sy1 = sy3
	x1# = x3#
	y1# = y3#
	z1# = z3#
	u1# = u3#
	v1# = v3#
	nx1# = nx3#
	ny1# = ny3#
	nz1# = nz3#
	sx3 = tsx
	sy3 = tsy
	x3# = tx#
	y3# = ty#
	z3# = tz#
	u3# = tu#
	v3# = tv#
	nx3# = tnx#
	ny3# = tny#
	nz3# = tnz#
EndIf
If sy3 < sy2
	tsx = sx2
	tsy = sy2
	tx# = x2#
	ty# = y2#
	tz# = z2#
	tu# = u2#
	tv# = v2#
	tnx# = nx2#
	tny# = ny2#
	tnz# = nz2#
	sx2 = sx3
	sy2 = sy3
	x2# = x3#
	y2# = y3#
	z2# = z3#
	u2# = u3#
	v2# = v3#
	nx2# = nx3#
	ny2# = ny3#
	nz2# = nz3#
	sx3 = tsx
	sy3 = tsy
	x3# = tx#
	y3# = ty#
	z3# = tz#
	u3# = tu#
	v3# = tv#
	nx3# = tnx#
	ny3# = tny#
	nz3# = tnz#
EndIf
ratio# = Float(sy2 - sy1) / Float(sy3 - sy1)
sx4 = Float((sx3 - sx1) * ratio#) + sx1
sy4 = sy2
x4# = Float(Float(x3# - x1#) * ratio#) + x1#
y4# = y2#
z4# = Float(Float(z3# - z1#) * ratio#) + z1#
u4# = Float(Float(u3# - u1#) * ratio#) + u1#
v4# = Float(Float(v3# - v1#) * ratio#) + v1#
nx4# = Float(Float(nx3# - nx1#) * ratio#) + nx1#
ny4# = Float(Float(ny3# - ny1#) * ratio#) + ny1#
nz4# = Float(Float(nz3# - nz1#) * ratio#) + nz1#
If sx4 < sx2
	tsx = sx2
	tx# = x2#
	ty# = y2#
	tz# = z2#
	tu# = u2#
	tv# = v2#
	tnx# = nx2#
	tny# = ny2#
	tnz# = nz2#
	sx2 = sx4
	x2# = x4#
	y2# = y4#
	z2# = z4#
	u2# = u4#
	v2# = v4#
	nx2# = nx4#
	ny2# = ny4#
	nz2# = nz4#
	sx4 = tsx
	x4# = tx#
	y4# = ty#
	z4# = tz#
	u4# = tu#
	v4# = tv#
	nx4# = tnx#
	ny4# = tny#
	nz4# = tnz#
EndIf

h# = Float(sy2 - sy1)
If h# > 0
	sl# = sx1
	sr# = sx1
	lx# = x1#
	ly# = y1#
	lz# = z1#
	rx# = x1#
	ry# = y1#
	rz# = z1#
	lu# = u1#
	lv# = v1#
	ru# = u1#
	rv# = v1#
	lnx# = nx1#
	lny# = ny1#
	lnz# = nz1#
	rnx# = nx1#
	rny# = ny1#
	rnz# = nz1#
	lsv# = Float(sx2 - sx1) / h#
	rsv# = Float(sx4 - sx1) / h#
	lxv# = Float(x2# - x1#) / h#
	lyv# = Float(y2# - y1#) / h#
	lzv# = Float(z2# - z1#) / h#
	rxv# = Float(x4# - x1#) / h#
	ryv# = Float(y4# - y1#) / h#
	rzv# = Float(z4# - z1#) / h#
	luv# = Float(u2# - u1#) / h#
	lvv# = Float(v2# - v1#) / h#
	ruv# = Float(u4# - u1#) / h#
	rvv# = Float(v4# - v1#) / h#
	lnxv# = Float(nx2# - nx1#) / h#
	lnyv# = Float(ny2# - ny1#) / h#
	lnzv# = Float(nz2# - nz1#) / h#
	rnxv# = Float(nx4# - nx1#) / h#
	rnyv# = Float(ny4# - ny1#) / h#
	rnzv# = Float(nz4# - nz1#) / h#
	For i = sy1 To sy2
		texhline sl,sr,i,lx#,rx#,ly#,ry#,lz#,rz#,lnx#,rnx#,lny#,rny#,lnz#,rnz#,lu#,ru#,lv#,rv#,eid,alpha#,a2#
		sl# = sl# + lsv#
		sr# = sr# + rsv#
		lx# = lx# + lxv#
		ly# = ly# + lyv#
		lz# = lz# + lzv#
		rx# = rx# + rxv#
		ry# = ry# + ryv#
		rz# = rz# + rzv#
		lu# = lu# + luv#
		lv# = lv# + lvv#
		ru# = ru# + ruv#
		rv# = rv# + rvv#
		lnx# = lnx# + lnxv#
		lny# = lny# + lnyv#
		lnz# = lnz# + lnzv#
		rnx# = rnx# + rnxv#
		rny# = rny# + rnyv#
		rnz# = rnz# + rnzv#
	Next
EndIf

h# = Float(sy3 - sy2)
If h# > 0
	sl# = sx3
	sr# = sx3
	lx# = x3#
	ly# = y3#
	lz# = z3#
	rx# = x3#
	ry# = y3#
	rz# = z3#
	lu# = u3#
	lv# = v3#
	ru# = u3#
	rv# = v3#
	lnx# = nx3#
	lny# = ny3#
	lnz# = nz3#
	rnx# = nx3#
	rny# = ny3#
	rnz# = nz3#
	lsv# = Float(sx2 - sx3) / h#
	rsv# = Float(sx4 - sx3) / h#
	lxv# = Float(x2# - x3#) / h#
	lyv# = Float(y2# - y3#) / h#
	lzv# = Float(z2# - z3#) / h#
	rxv# = Float(x4# - x3#) / h#
	ryv# = Float(y4# - y3#) / h#
	rzv# = Float(z4# - z3#) / h#
	luv# = Float(u2# - u3#) / h#
	lvv# = Float(v2# - v3#) / h#
	ruv# = Float(u4# - u3#) / h#
	rvv# = Float(v4# - v3#) / h#
	lnxv# = Float(nx2# - nx3#) / h#
	lnyv# = Float(ny2# - ny3#) / h#
	lnzv# = Float(nz2# - nz3#) / h#
	rnxv# = Float(nx4# - nx3#) / h#
	rnyv# = Float(ny4# - ny3#) / h#
	rnzv# = Float(nz4# - nz3#) / h#
	For i = sy3 To sy2 + 1 Step -1
		texhline sl,sr,i,lx#,rx#,ly#,ry#,lz#,rz#,lnx#,rnx#,lny#,rny#,lnz#,rnz#,lu#,ru#,lv#,rv#,eid,alpha#,a2#
		sl# = sl# + lsv#
		sr# = sr# + rsv#
		lx# = lx# + lxv#
		ly# = ly# + lyv#
		lz# = lz# + lzv#
		rx# = rx# + rxv#
		ry# = ry# + ryv#
		rz# = rz# + rzv#
		lu# = lu# + luv#
		lv# = lv# + lvv#
		ru# = ru# + ruv#
		rv# = rv# + rvv#
		lnx# = lnx# + lnxv#
		lny# = lny# + lnyv#
		lnz# = lnz# + lnzv#
		rnx# = rnx# + rnxv#
		rny# = rny# + rnyv#
		rnz# = rnz# + rnzv#
	Next
EndIf
End Function

Function texhline(fx,tx,yy,lx#,rx#,ly#,ry#,lz#,rz#,lnx#,rnx#,lny#,rny#,lnz#,rnz#,lu#,ru#,lv#,rv#,eid,a1#,a2#)
tid = e(eid)\t
rd = e(eid)\rd
w# = Float(tx - fx)
xv# = Float(rx# - lx#) / w#
yv# = Float(ry# - ly#) / w#
zv# = Float(rz# - lz#) / w#
uv# = Float(ru# - lu#) / w#
vv# = Float(rv# - lv#) / w#
nxv# = Float(rnx# - lnx#) / w#
nyv# = Float(rny# - lny#) / w#
nzv# = Float(rnz# - lnz#) / w#
cx# = lx#
cy# = ly#
cz# = lz#
cu# = lu#
cv# = lv#
cnx# = lnx#
cny# = lny#
cnz# = lnz#
For xx = fx To tx
	If xx >= 0 And xx < width And yy >= 0 And yy < height
		cz2# = Float(1.0 / cz#)
		If cz2# >= 2 And cz2# <= screen#(xx,yy,2)
			cx2# = Float(cx# * cz2#)
			cy2# = Float(cy# * cz2#)
			cnx2# = Float(cnx# * cz2#)
			cny2# = Float(cny# * cz2#)
			cnz2# = Float(cnz# * cz2#)
			If t(tid)\srm
				tu = (cnx2# * 127) + 128
				tv = (-cny2# * 127) + 128
			Else
				tu = Int(Float(cu# * cz2#) * 255)
				tv = Int(Float(cv# * cz2#) * 255)
			EndIf
			diffuser# = t(tid)\r#[(tv * 256) + tu]
			diffuseg# = t(tid)\g#[(tv * 256) + tu]
			diffuseb# = t(tid)\b#[(tv * 256) + tu]
			If diffuser# <> 0 Or diffuseg# <> 0 Or diffuseb# <> 0 Or t(tid)\masked = False
				If e(eid)\nl
					nr# = diffuser# * 255
					ng# = diffuseg# * 255
					nb# = diffuseb# * 255
				Else
					lvx# = Float(cx2# - lightx#)
					lvy# = Float(cy2# - lighty#)
					lvz# = Float(cz2# - lightz#)
					lmag# = Sqr(Float(lvx# * lvx#) + Float(lvy# * lvy#) + Float(lvz# * lvz#))
					lvx# = Float(lvx# / lmag#)
					lvy# = Float(lvy# / lmag#)
					lvz# = Float(lvz# / lmag#)
					dot1# = Float(cnx2# * lvx#) + Float(cny2# * lvy#) + Float(cnz2# * lvz#)
					If dot1# < 0
						dot1# = -dot1#
					Else
						dot1# = 0
					EndIf
					nar# = Float(ambientr# * diffuser#)
					nag# = Float(ambientg# * diffuseg#)
					nab# = Float(ambientb# * diffuseb#)
					ndr# = Float(diffuser# * dot1#)
					ndg# = Float(diffuseg# * dot1#)
					ndb# = Float(diffuseb# * dot1#)
					nr# = Float(nar# + Float(lightr# * ndr#)) * 255.0
					ng# = Float(nag# + Float(lightg# * ndg#)) * 255.0
					nb# = Float(nab# + Float(lightb# * ndb#)) * 255.0
				EndIf
				If nr# < 5 Then nr# = 5
				If ng# < 5 Then ng# = 5
				If nb# < 5 Then nb# = 5
				If nr# > 255 Then nr# = 255
				If ng# > 255 Then ng# = 255
				If nb# > 255 Then nb# = 255
				nr# = nr# * a1#
				ng# = ng# * a1#
				nb# = nb# * a1#
				dx = xx - (cnx2# * rd)
				dy = yy + (cny2# * rd)
				If dx < 0 Then dx = 0
				If dx > width - 1 Then dx = width - 1
				If dy < 0 Then dy = 0
				If dy > height - 1 Then dy = height - 1
				argb = screen(dx,dy,1)
				nr# = nr# + (((argb Shr 16) And 255) * a2#)
				ng# = ng# + (((argb Shr 8) And 255) * a2#)
				nb# = nb# + ((argb And 255) * a2#)
				screen2(xx,yy) = (nr# Shl 16) + (ng# Shl 8) + nb#
				screen#(xx,yy,2) = cz2#
			EndIf
		EndIf
	EndIf
	cx# = cx# + xv#
	cy# = cy# + yv#
	cz# = cz# + zv#
	cu# = cu# + uv#
	cv# = cv# + vv#
	cnx# = cnx# + nxv#
	cny# = cny# + nyv#
	cnz# = cnz# + nzv#
Next
End Function

Function striangle(sx1,sy1,sx2,sy2,sx3,sy3,z#,alpha#)
alpha = Float(Float(1.0 - alpha#) / 2.0) + .5
If sy2 < sy1
	tsx = sx1
	tsy = sy1
	sx1 = sx2
	sy1 = sy2
	sx2 = tsx
	sy2 = tsy
EndIf
If sy3 < sy1
	tsx = sx1
	tsy = sy1
	sx1 = sx3
	sy1 = sy3
	sx3 = tsx
	sy3 = tsy
EndIf
If sy3 < sy2
	tsx = sx2
	tsy = sy2
	sx2 = sx3
	sy2 = sy3
	sx3 = tsx
	sy3 = tsy
EndIf
ratio# = Float(sy2 - sy1) / Float(sy3 - sy1)
sx4 = ((sx3 - sx1) * ratio#) + sx1
sy4 = sy2
If sx4 < sx2
	tsx = sx4
	sx4 = sx2
	sx2 = tsx
EndIf

h# = Float(sy2 - sy1)
If h# > 0
	l# = sx1
	r# = sx1
	lv# = Float(sx2 - sx1) / h#
	rv# = Float(sx4 - sx1) / h#
	For i = sy1 To sy2
		If i >= 0 And i < height Then shline l,r,i,z#,alpha#
		l# = l# + lv#
		r# = r# + rv#
	Next
EndIf

h# = Float(sy3 - sy2)
If h# > 0
	l# = sx3
	r# = sx3
	lv# = Float(sx2 - sx3) / h#
	rv# = Float(sx4 - sx3) / h#
	For i = sy3 To sy2 Step -1
		If i >= 0 And i < height Then shline l,r,i,z#,alpha#
		l# = l# + lv#
		r# = r# + rv#
	Next
EndIf
End Function

Function shline(fx,tx,yy,z#,alpha#)
For xx = fx To tx
	If xx >= 0 And xx < width And yy >= 0 And yy < height
		If z# <= screen#(xx,yy,2)
			argb = screen(xx,yy,1)
			r = ((argb Shr 16) And 255) * alpha#
			g = ((argb Shr 8) And 255) * alpha#
			b = (argb And 255) * alpha#
			screen2(xx,yy) = (r Shl 16) + (g Shl 8) + b
		EndIf
	EndIf
Next
End Function