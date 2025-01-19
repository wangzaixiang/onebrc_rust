
```asm

__ZN11onebrc_rust5ver2010FileReader9scan_loop17h0a63cab6d9465a81E:
Lfunc_begin147:
	.loc	167 171 0
	.cfi_startproc
	stp	d11, d10, [sp, #-128]!
	.cfi_def_cfa_offset 128
	stp	d9, d8, [sp, #16]
	stp	x28, x27, [sp, #32]
	stp	x26, x25, [sp, #48]
	stp	x24, x23, [sp, #64]
	stp	x22, x21, [sp, #80]
	stp	x20, x19, [sp, #96]
	stp	x29, x30, [sp, #112]
	add	x29, sp, #112
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	.cfi_offset w27, -88
	.cfi_offset w28, -96
	.cfi_offset b8, -104
	.cfi_offset b9, -112
	.cfi_offset b10, -120
	.cfi_offset b11, -128
	.cfi_remember_state
	sub	sp, sp, #416
	str	x3, [sp, #16]
	stur	x0, [x29, #-176]
Ltmp15306:
	.loc	167 231 15 prologue_end
	cbz	x1, LBB147_66
Ltmp15307:
	.loc	167 0 15 is_stmt 0
	mov	x19, #0
	mov	x23, #0
	mov	x14, #0
	ldur	x8, [x29, #-176]
Ltmp15308:
	.loc	16 4390 14 is_stmt 1
	ldp	q2, q1, [x8, #32]
	sub	x15, x8, #8
	movi.16b	v6, #10
	mov	x16, #-2
	ldp	q0, q3, [x8]
	stp	q1, q3, [sp, #144]					; [sp, #112]: q0, q2, q1, q3 共64字节， 顺序 q0, q3, q2, q1
Ltmp15309:
	.loc	16 0 14 is_stmt 0
	stp	q2, q0, [sp, #112]
	movi.2d	v7, #0xffffffffff000000
Lloh1018:
	adrp	x8, lCPI147_0@PAGE
Lloh1019:
	ldr	q0, [x8, lCPI147_0@PAGEOFF]
	str	q0, [sp, #32]
	movi.16b	v16, #45
	movi.4h	v8, #1
	movi.16b	v17, #208
	movi.16b	v18, #48
Lloh1020:
	adrp	x8, lCPI147_1@PAGE
Lloh1021:
	ldr	d9, [x8, lCPI147_1@PAGEOFF]
Lloh1022:
	adrp	x8, lCPI147_2@PAGE
Lloh1023:
	ldr	q19, [x8, lCPI147_2@PAGEOFF]
Lloh1024:
	adrp	x8, lCPI147_3@PAGE
Lloh1025:
	ldr	q20, [x8, lCPI147_3@PAGEOFF]
	mov	w8, #1
	str	x8, [sp, #8]
	movi.8b	v10, #45
	adrp	x17, lCPI147_4@PAGE
	movi.4h	v11, #47
	adrp	x0, lCPI147_5@PAGE
	adrp	x3, lCPI147_6@PAGE
	stp	x15, x2, [sp, #256]
	stp	q20, q19, [sp, #224]
	str	x1, [sp, #24]
	b	LBB147_3
Ltmp15310:
LBB147_2:
	.loc	167 365 20 is_stmt 1
	cmp	x14, #0
	csel	x14, x21, x14, eq
Ltmp15311:
	.loc	167 0 20 is_stmt 0
	ldr	x8, [sp, #104]
Ltmp15312:
	mov	x19, x8
	ldr	x1, [sp, #24]
Ltmp15313:
	.loc	167 231 15 is_stmt 1
	cmp	x8, x1
Ltmp15314:
	b.hs	LBB147_66
Ltmp15315:
LBB147_3:
	.loc	167 232 16
	add	x11, x19, #64
	cmp	x11, x1
	b.hi	LBB147_65
Ltmp15316:
	.loc	167 0 16 is_stmt 0
	movi.16b	v2, #59
	ldp	q22, q21, [sp, #112]					; 从栈上 load q22, q21, q5, q4 
Ltmp15317:
	.loc	144 33 51 is_stmt 1
	cmeq.16b	v0, v22, v2
	ldr	q3, [sp, #32]
	and.16b	v0, v0, v3
	ext.16b	v1, v0, v0, #8
	zip1.16b	v0, v0, v1
	addv.8h	h0, v0
	fmov	w8, s0
	ldp	q5, q4, [sp, #144]
Ltmp15318:
	cmeq.16b	v0, v5, v2
	and.16b	v0, v0, v3
	ext.16b	v1, v0, v0, #8
	zip1.16b	v0, v0, v1
	addv.8h	h0, v0
	fmov	w9, s0
	bfi	w8, w9, #16, #16
Ltmp15319:
	cmeq.16b	v0, v21, v2
	and.16b	v0, v0, v3
	ext.16b	v1, v0, v0, #8
	zip1.16b	v0, v0, v1
	addv.8h	h0, v0
	fmov	w9, s0
Ltmp15320:
	cmeq.16b	v0, v4, v2
	and.16b	v0, v0, v3
	ext.16b	v1, v0, v0, #8
	zip1.16b	v0, v0, v1
	addv.8h	h0, v0
	fmov	w10, s0
	bfi	w9, w10, #16, #16
	orr	x21, x9, x8, lsl #32
Ltmp15321:
	.loc	144 33 51 is_stmt 0
	cmeq.16b	v0, v22, v6
	and.16b	v0, v0, v3
	ext.16b	v1, v0, v0, #8
	zip1.16b	v0, v0, v1
	addv.8h	h0, v0
	fmov	w8, s0
	cmeq.16b	v0, v5, v6
	and.16b	v0, v0, v3
	ext.16b	v1, v0, v0, #8
	zip1.16b	v0, v0, v1
	addv.8h	h0, v0
	fmov	w9, s0
	bfi	w8, w9, #16, #16
	cmeq.16b	v0, v21, v6
	and.16b	v0, v0, v3
	ext.16b	v1, v0, v0, #8
	zip1.16b	v0, v0, v1
	addv.8h	h0, v0
	fmov	w9, s0
	cmeq.16b	v0, v4, v6
	and.16b	v0, v0, v3
	ext.16b	v1, v0, v0, #8
	zip1.16b	v0, v0, v1
	addv.8h	h0, v0
	fmov	w10, s0
	bfi	w9, w10, #16, #16
	orr	x20, x9, x8, lsl #32
Ltmp15322:
	.loc	144 0 51
	ldur	x8, [x29, #-176]
	str	x11, [sp, #104]
Ltmp15323:
	.loc	30 967 18 is_stmt 1
	add	x8, x8, x11
Ltmp15324:
	.loc	16 4390 14
	ldp	q2, q1, [x8, #32]
	ldp	q0, q3, [x8]
	stp	q1, q3, [sp, #144]				; 保存到 [sp, #112..#178] 
Ltmp15325:
	.loc	16 0 14 is_stmt 0
	stp	q2, q0, [sp, #112]
Ltmp15326:
	.loc	56 85 20 is_stmt 1
	fmov	d0, x20
	cnt.8b	v0, v0
	uaddlv.8b	h0, v0
	fmov	w26, s0
Ltmp15327:
	.loc	16 1506 8
	cmp	x26, #4
	b.lo	LBB147_51
Ltmp15328:
	.loc	16 0 8 is_stmt 0
	sub	x1, x19, #64
	stur	x1, [x29, #-256]
Ltmp15329:
LBB147_6:
	.loc	56 162 20 is_stmt 1
	rbit	x8, x21
	clz	x8, x8
Ltmp15330:
	.loc	167 165 17
	lsl	x9, x16, x8
	.loc	167 165 9 is_stmt 0
	and	x9, x9, x21
Ltmp15331:
	.loc	167 245 114 is_stmt 1
	add	x8, x8, x19
Ltmp15332:
	.loc	56 162 20
	rbit	x10, x14
	clz	x10, x10
Ltmp15333:
	.loc	167 165 17
	lsl	x11, x16, x10
	.loc	167 165 9 is_stmt 0
	and	x11, x11, x14
Ltmp15334:
	.loc	167 245 55 is_stmt 1
	orr	x10, x1, x10
Ltmp15335:
	.loc	167 245 38 is_stmt 0
	cmp	x14, #0
	csel	x14, xzr, x11, eq
Ltmp15336:
	csel	x21, x9, x21, eq
Ltmp15337:
	csel	x8, x8, x10, eq
Ltmp15338:
	.loc	56 162 20 is_stmt 1
	rbit	x9, x20
	clz	x9, x9
Ltmp15339:
	.loc	167 165 17
	lsl	x10, x16, x9
	.loc	167 165 9 is_stmt 0
	and	x10, x10, x20
Ltmp15340:
	.loc	56 162 20 is_stmt 1
	rbit	x11, x10
	clz	x11, x11
Ltmp15341:
	.loc	167 165 17
	lsl	x12, x16, x11
	.loc	167 165 9 is_stmt 0
	and	x10, x12, x10
Ltmp15342:
	.loc	56 162 20 is_stmt 1
	rbit	x12, x10
	clz	x12, x12
Ltmp15343:
	.loc	167 165 17
	lsl	x13, x16, x12
	.loc	167 165 9 is_stmt 0
	and	x4, x13, x10
Ltmp15344:
	.loc	56 162 20 is_stmt 1
	rbit	x10, x4
	clz	x6, x10
Ltmp15345:
	.loc	56 0 20 is_stmt 0
	ldur	x13, [x29, #-176]
Ltmp15346:
	.loc	16 4390 14 is_stmt 1
	ldr	q0, [x13, x23]
Ltmp15347:
	.loc	167 246 35
	add	x20, x9, x19
Ltmp15348:
	.loc	30 967 18
	add	x9, x13, x20
Ltmp15349:
	.loc	16 4390 14
	ldur	q23, [x9, #1]
Ltmp15350:
	.loc	167 250 35
	add	x28, x11, x19
Ltmp15351:
	.loc	30 967 18
	add	x10, x13, x28
Ltmp15352:
	.loc	16 4390 14
	ldur	q22, [x10, #1]
Ltmp15353:
	.loc	167 256 35
	add	x27, x12, x19
Ltmp15354:
	.loc	30 967 18
	add	x11, x13, x27
Ltmp15355:
	.loc	16 4390 14
	ldur	q21, [x11, #1]
Ltmp15356:
	.loc	16 4390 14 is_stmt 0
	ldur	x9, [x9, #-8]
Ltmp15357:
	.loc	30 967 18 is_stmt 1
	sub	x10, x10, #8
Ltmp15358:
	.loc	16 4390 14
	ldur	x11, [x11, #-8]
Ltmp15359:
	.loc	167 259 35
	add	x7, x6, x19
Ltmp15360:
	.loc	30 967 18
	add	x12, x15, x7
Ltmp15361:
	.loc	167 269 50
	fmov	d1, x11
Ltmp15362:
	ld1.d	{ v1 }[1], [x12]
Ltmp15363:
	fmov	d2, x9
Ltmp15364:
	ld1.d	{ v2 }[1], [x10]
Ltmp15365:
	and.16b	v3, v1, v7
	and.16b	v4, v2, v7
Ltmp15366:
	.loc	144 33 51
	cmeq.16b	v3, v3, v16
	cmeq.16b	v4, v4, v16
Ltmp15367:
	.loc	157 90 13
	ext.16b	v5, v4, v3, #1
	ext.16b	v3, v3, v4, #1
Ltmp15368:
	.loc	144 33 51
	cmeq.2d	v3, v3, #0
	cmeq.2d	v4, v5, #0
	uzp1.4s	v3, v4, v3
	xtn.4h	v3, v3
Ltmp15369:
	.loc	163 31 18
	and.8b	v4, v3, v8
	orn.8b	v3, v4, v3
Ltmp15370:
	.loc	167 289 68
	uzp2.4s	v1, v2, v1
Ltmp15371:
	.loc	145 314 23
	add.16b	v2, v1, v17
	cmhi.16b	v2, v6, v2
Ltmp15372:
	.loc	163 31 18
	bif.16b	v1, v18, v2
Ltmp15373:
	.loc	166 240 26
	ext.16b	v2, v1, v1, #8
Ltmp15374:
	.loc	150 40 18
	mov.16b	v4, v19
	smlal.8h	v4, v1, v9
	mov.16b	v5, v19
	smlal.8h	v5, v2, v9
Ltmp15375:
	.loc	157 90 13
	ext.16b	v1, v5, v4, #12
Ltmp15376:
	.loc	150 40 18
	add.8h	v1, v1, v4
Ltmp15377:
	.loc	157 90 13
	rev32.8h	v2, v1
Ltmp15378:
	.loc	150 40 18
	add.8h	v2, v2, v1
Ltmp15379:
	.loc	167 299 26
	umov.h	w9, v3[0]
	.loc	167 299 38 is_stmt 0
	umov.h	w10, v2[3]
Ltmp15380:
	.loc	167 299 26
	mul	w25, w10, w9
Ltmp15381:
	.loc	167 304 62 is_stmt 1
	sub	x24, x8, x23
Ltmp15382:
	.loc	157 90 13
	dup.16b	v1, w24
Ltmp15383:
	.loc	156 59 51
	cmhi.16b	v1, v1, v20
Ltmp15384:
	.loc	163 31 18
	and.16b	v0, v0, v1
Ltmp15385:
	.loc	167 476 23
	fmov	x9, d0
	.loc	167 476 32 is_stmt 0
	mov.d	x10, v0[1]
Ltmp15386:
	.loc	167 480 22 is_stmt 1
	lsr	x11, x9, #20
Ltmp15387:
	.loc	167 481 22
	lsr	x12, x10, #20
Ltmp15388:
	.loc	167 484 13
	eor	x13, x9, x10
	lsr	x13, x13, #40
	eor	w11, w11, w12
Ltmp15389:
	eor	w11, w13, w11
	eor	w9, w9, w10
Ltmp15390:
	eor	w9, w11, w9
Ltmp15391:
	.loc	167 487 32
	and	x5, x9, #0xfffff
Ltmp15392:
	.loc	20 97 14
	add	x22, x2, x5, lsl #6
Ltmp15393:
	.loc	143 904 49
	ldr	q1, [x22]
Ltmp15394:
	.loc	145 289 18
	cmeq.2d	v1, v1, v0
	mvn.16b	v1, v1
	xtn.2s	v1, v1
	umaxp.2s	v1, v1, v1
	fmov	w9, s1
Ltmp15395:
	.loc	16 1506 8
	tbnz	w9, #0, LBB147_15
Ltmp15396:
	.loc	167 447 18
	ldp	w8, w9, [x22, #16]
Ltmp15397:
	.loc	167 494 30
	sxth	w10, w25
Ltmp15398:
	.loc	17 0 0 is_stmt 0
	cmp	w8, w10
	csel	w8, w8, w10, lt
Ltmp15399:
	cmp	w9, w10
	csel	w9, w9, w10, gt
Ltmp15400:
	.loc	167 497 13 is_stmt 1
	stp	w8, w9, [x22, #16]
Ltmp15401:
	.loc	167 447 18
	ldr	d0, [x22, #24]
Ltmp15402:
	.loc	167 493 13
	movi.2s	v1, #1
	mov.s	v1[1], w10
	add.2s	v0, v0, v1
	.loc	167 497 13
	str	d0, [x22, #24]
Ltmp15403:
LBB147_8:
	.loc	167 0 0 is_stmt 0
	rbit	x8, x21
	umov.h	w9, v3[1]
	clz	x22, x8
Ltmp15404:
	umov.h	w8, v2[7]
	mul	w25, w8, w9
Ltmp15405:
	mvn	x8, x20
	add	x9, x19, x22
Ltmp15406:
	add	x24, x9, x8
Ltmp15407:
	dup.16b	v0, w24
Ltmp15408:
	cmhi.16b	v0, v0, v20
Ltmp15409:
	and.16b	v0, v23, v0
Ltmp15410:
	.loc	167 476 23 is_stmt 1
	fmov	x8, d0
	.loc	167 476 32 is_stmt 0
	mov.d	x9, v0[1]
Ltmp15411:
	.loc	167 480 22 is_stmt 1
	lsr	x10, x8, #20
Ltmp15412:
	.loc	167 481 22
	lsr	x11, x9, #20
Ltmp15413:
	.loc	167 484 13
	eor	x12, x8, x9
	lsr	x12, x12, #40
	eor	w10, w10, w11
Ltmp15414:
	eor	w10, w12, w10
	eor	w8, w8, w9
Ltmp15415:
	eor	w8, w10, w8
Ltmp15416:
	.loc	167 487 32
	and	x5, x8, #0xfffff
Ltmp15417:
	.loc	20 97 14
	add	x20, x2, x5, lsl #6
Ltmp15418:
	.loc	143 904 49
	ldr	q1, [x20]
Ltmp15419:
	.loc	145 289 18
	cmeq.2d	v1, v1, v0
	mvn.16b	v1, v1
	xtn.2s	v1, v1
	umaxp.2s	v1, v1, v1
	fmov	w8, s1
Ltmp15420:
	.loc	16 1506 8
	tbnz	w8, #0, LBB147_19
Ltmp15421:
	.loc	167 447 18
	ldp	w8, w9, [x20, #16]
Ltmp15422:
	.loc	167 494 30
	sxth	w10, w25
Ltmp15423:
	.loc	17 0 0 is_stmt 0
	cmp	w8, w10
	csel	w8, w8, w10, lt
Ltmp15424:
	cmp	w9, w10
	csel	w9, w9, w10, gt
Ltmp15425:
	.loc	167 497 13 is_stmt 1
	stp	w8, w9, [x20, #16]
Ltmp15426:
	.loc	167 447 18
	ldr	d0, [x20, #24]
Ltmp15427:
	.loc	167 493 13
	movi.2s	v1, #1
	mov.s	v1[1], w10
	add.2s	v0, v0, v1
	.loc	167 497 13
	str	d0, [x20, #24]
Ltmp15428:
LBB147_10:
	.loc	167 0 0 is_stmt 0
	lsl	x8, x16, x22
	and	x20, x8, x21
Ltmp15429:
	rbit	x8, x20
	clz	x21, x8
Ltmp15430:
	ext.16b	v0, v4, v5, #12
	add.8h	v0, v0, v5
	rev32.8h	v1, v0
	umov.h	w8, v3[2]
	add.8h	v2, v1, v0
	umov.h	w9, v2[3]
	mul	w25, w9, w8
Ltmp15431:
	mvn	x8, x28
	add	x9, x19, x21
Ltmp15432:
	add	x24, x9, x8
Ltmp15433:
	dup.16b	v0, w24
Ltmp15434:
	cmhi.16b	v0, v0, v20
Ltmp15435:
	and.16b	v0, v22, v0
Ltmp15436:
	.loc	167 476 23 is_stmt 1
	fmov	x8, d0
	.loc	167 476 32 is_stmt 0
	mov.d	x9, v0[1]
Ltmp15437:
	.loc	167 480 22 is_stmt 1
	lsr	x10, x8, #20
Ltmp15438:
	.loc	167 481 22
	lsr	x11, x9, #20
Ltmp15439:
	.loc	167 484 13
	eor	x12, x8, x9
	lsr	x12, x12, #40
	eor	w10, w10, w11
Ltmp15440:
	eor	w10, w12, w10
	eor	w8, w8, w9
Ltmp15441:
	eor	w8, w10, w8
Ltmp15442:
	.loc	167 487 32
	and	x5, x8, #0xfffff
Ltmp15443:
	.loc	20 97 14
	add	x22, x2, x5, lsl #6
Ltmp15444:
	.loc	143 904 49
	ldr	q1, [x22]
Ltmp15445:
	.loc	145 289 18
	cmeq.2d	v1, v1, v0
	mvn.16b	v1, v1
	xtn.2s	v1, v1
	umaxp.2s	v1, v1, v1
	fmov	w8, s1
Ltmp15446:
	.loc	16 1506 8
	tbnz	w8, #0, LBB147_23
Ltmp15447:
	.loc	167 447 18
	ldp	w8, w9, [x22, #16]
Ltmp15448:
	.loc	167 494 30
	sxth	w10, w25
Ltmp15449:
	.loc	17 0 0 is_stmt 0
	cmp	w8, w10
	csel	w8, w8, w10, lt
Ltmp15450:
	cmp	w9, w10
	csel	w9, w9, w10, gt
Ltmp15451:
	.loc	167 497 13 is_stmt 1
	stp	w8, w9, [x22, #16]
Ltmp15452:
	.loc	167 447 18
	ldr	d0, [x22, #24]
Ltmp15453:
	.loc	167 493 13
	movi.2s	v1, #1
	mov.s	v1[1], w10
	add.2s	v0, v0, v1
	.loc	167 497 13
	str	d0, [x22, #24]
Ltmp15454:
LBB147_12:
	.loc	167 0 0 is_stmt 0
	lsl	x8, x16, x21
	and	x20, x8, x20
Ltmp15455:
	rbit	x8, x20
	umov.h	w9, v3[3]
	clz	x21, x8
Ltmp15456:
	umov.h	w8, v2[7]
	mul	w25, w8, w9
Ltmp15457:
	mvn	x8, x27
	add	x9, x19, x21
Ltmp15458:
	add	x24, x9, x8
Ltmp15459:
	dup.16b	v0, w24
Ltmp15460:
	cmhi.16b	v0, v0, v20
Ltmp15461:
	and.16b	v0, v21, v0
Ltmp15462:
	.loc	167 476 23 is_stmt 1
	fmov	x8, d0
	.loc	167 476 32 is_stmt 0
	mov.d	x9, v0[1]
Ltmp15463:
	.loc	167 480 22 is_stmt 1
	lsr	x10, x8, #20
Ltmp15464:
	.loc	167 481 22
	lsr	x11, x9, #20
Ltmp15465:
	.loc	167 484 13
	eor	x12, x8, x9
	lsr	x12, x12, #40
	eor	w10, w10, w11
Ltmp15466:
	eor	w10, w12, w10
	eor	w8, w8, w9
Ltmp15467:
	eor	w8, w10, w8
Ltmp15468:
	.loc	167 487 32
	and	x5, x8, #0xfffff
Ltmp15469:
	.loc	20 97 14
	add	x22, x2, x5, lsl #6
Ltmp15470:
	.loc	143 904 49
	ldr	q1, [x22]
Ltmp15471:
	.loc	145 289 18
	cmeq.2d	v1, v1, v0
	mvn.16b	v1, v1
	xtn.2s	v1, v1
	umaxp.2s	v1, v1, v1
	fmov	w8, s1
Ltmp15472:
	.loc	16 1506 8
	tbnz	w8, #0, LBB147_27
Ltmp15473:
	.loc	167 447 18
	ldp	w8, w9, [x22, #16]
Ltmp15474:
	.loc	167 494 30
	sxth	w10, w25
Ltmp15475:
	.loc	17 0 0 is_stmt 0
	cmp	w8, w10
	csel	w8, w8, w10, lt
Ltmp15476:
	cmp	w9, w10
	csel	w9, w9, w10, gt
Ltmp15477:
	.loc	167 497 13 is_stmt 1
	stp	w8, w9, [x22, #16]
Ltmp15478:
	.loc	167 447 18
	ldr	d0, [x22, #24]
Ltmp15479:
	.loc	167 493 13
	movi.2s	v1, #1
	mov.s	v1[1], w10
	add.2s	v0, v0, v1
	.loc	167 497 13
	str	d0, [x22, #24]
Ltmp15480:
LBB147_14:
	.loc	167 0 0 is_stmt 0
	lsl	x8, x16, x21
	and	x21, x8, x20
Ltmp15481:
	lsl	x8, x16, x6
	and	x20, x8, x4
Ltmp15482:
	.loc	167 335 21 is_stmt 1
	sub	w26, w26, #4
Ltmp15483:
	.loc	167 336 21
	add	x23, x7, #1
Ltmp15484:
	.loc	16 1506 8
	cmp	w26, #3
	b.hi	LBB147_6
	b	LBB147_51
Ltmp15485:
LBB147_15:
	.loc	9 2683 19
	ldr	x9, [x22, #48]
Ltmp15486:
	.loc	9 0 19 is_stmt 0
	stur	x4, [x29, #-248]
	stp	q3, q21, [x29, #-240]
	stp	q4, q22, [sp, #192]
	str	q5, [sp, #176]
	stp	q2, q23, [sp, #64]
Ltmp15487:
	.loc	16 1506 8 is_stmt 1
	cbnz	x9, LBB147_47
Ltmp15488:
	.loc	167 501 13
	str	q0, [x22]
Ltmp15489:
	.loc	26 376 16
	tbnz	x24, #63, LBB147_67
Ltmp15490:
	.loc	26 0 16 is_stmt 0
	stp	x7, x6, [x29, #-192]
	stur	x14, [x29, #-200]
Ltmp15491:
	.loc	11 465 12 is_stmt 1
	cmp	x8, x23
	b.ne	LBB147_31
Ltmp15492:
	.loc	11 0 12 is_stmt 0
	mov	w0, #1
	.loc	11 465 12
	b	LBB147_32
Ltmp15493:
LBB147_19:
	.loc	9 2683 19 is_stmt 1
	ldr	x8, [x20, #48]
Ltmp15494:
	.loc	9 0 19 is_stmt 0
	stur	x4, [x29, #-248]
	stp	q3, q21, [x29, #-240]
	stp	q4, q22, [sp, #192]
	str	q5, [sp, #176]
Ltmp15495:
	.loc	16 1506 8 is_stmt 1
	cbnz	x8, LBB147_48
Ltmp15496:
	.loc	167 501 13
	str	q0, [x20]
Ltmp15497:
	.loc	26 376 16
	tbnz	x24, #63, LBB147_67
Ltmp15498:
	.loc	26 0 16 is_stmt 0
	stp	x7, x6, [x29, #-192]
	stur	x14, [x29, #-200]
Ltmp15499:
	.loc	11 465 12 is_stmt 1
	cbz	x24, LBB147_35
Ltmp15500:
	.loc	5 1757 9
Lloh1026:
	adrp	x8, ___rust_no_alloc_shim_is_unstable@GOTPAGE
Lloh1027:
	ldr	x8, [x8, ___rust_no_alloc_shim_is_unstable@GOTPAGEOFF]
Ltmp15501:
	ldrb	wzr, [x8]
Ltmp15502:
	.loc	24 96 9
	mov	x0, x24
	mov	w1, #1
	bl	___rust_alloc
Ltmp15503:
	.loc	24 0 9 is_stmt 0
	cbnz	x0, LBB147_36
	b	LBB147_68
Ltmp15504:
LBB147_23:
	.loc	9 2683 19 is_stmt 1
	ldr	x8, [x22, #48]
Ltmp15505:
	.loc	9 0 19 is_stmt 0
	stp	q3, q21, [x29, #-240]
	str	q2, [sp, #192]
Ltmp15506:
	.loc	16 1506 8 is_stmt 1
	cbnz	x8, LBB147_49
Ltmp15507:
	.loc	167 501 13
	str	q0, [x22]
Ltmp15508:
	.loc	26 376 16
	tbnz	x24, #63, LBB147_67
Ltmp15509:
	.loc	26 0 16 is_stmt 0
	stp	x7, x6, [x29, #-192]
	stur	x4, [x29, #-248]
	stur	x14, [x29, #-200]
Ltmp15510:
	.loc	11 465 12 is_stmt 1
	cbz	x24, LBB147_39
Ltmp15511:
	.loc	5 1757 9
Lloh1028:
	adrp	x8, ___rust_no_alloc_shim_is_unstable@GOTPAGE
Lloh1029:
	ldr	x8, [x8, ___rust_no_alloc_shim_is_unstable@GOTPAGEOFF]
Ltmp15512:
	ldrb	wzr, [x8]
Ltmp15513:
	.loc	24 96 9
	mov	x0, x24
	mov	w1, #1
	bl	___rust_alloc
Ltmp15514:
	.loc	24 0 9 is_stmt 0
	cbnz	x0, LBB147_40
	b	LBB147_68
Ltmp15515:
LBB147_27:
	.loc	9 2683 19 is_stmt 1
	ldr	x8, [x22, #48]
Ltmp15516:
	.loc	16 1506 8
	cbnz	x8, LBB147_50
Ltmp15517:
	.loc	167 501 13
	str	q0, [x22]
Ltmp15518:
	.loc	26 376 16
	tbnz	x24, #63, LBB147_67
Ltmp15519:
	.loc	26 0 16 is_stmt 0
	stp	x7, x6, [x29, #-192]
	mov	x28, x4
	mov	x27, x14
Ltmp15520:
	.loc	11 465 12 is_stmt 1
	cbz	x24, LBB147_43
Ltmp15521:
	.loc	5 1757 9
Lloh1030:
	adrp	x8, ___rust_no_alloc_shim_is_unstable@GOTPAGE
Lloh1031:
	ldr	x8, [x8, ___rust_no_alloc_shim_is_unstable@GOTPAGEOFF]
Ltmp15522:
	ldrb	wzr, [x8]
Ltmp15523:
	.loc	24 96 9
	mov	x0, x24
	mov	w1, #1
	bl	___rust_alloc
Ltmp15524:
	.loc	24 0 9 is_stmt 0
	cbnz	x0, LBB147_44
	b	LBB147_68
Ltmp15525:
LBB147_31:
	.loc	5 1757 9 is_stmt 1
Lloh1032:
	adrp	x8, ___rust_no_alloc_shim_is_unstable@GOTPAGE
Ltmp15526:
Lloh1033:
	ldr	x8, [x8, ___rust_no_alloc_shim_is_unstable@GOTPAGEOFF]
Ltmp15527:
	ldrb	wzr, [x8]
Ltmp15528:
	.loc	24 96 9
	mov	x0, x24
	mov	w1, #1
	bl	___rust_alloc
Ltmp15529:
	.loc	11 478 19
	cbz	x0, LBB147_68
Ltmp15530:
LBB147_32:
	.loc	11 0 19 is_stmt 0
	ldur	x8, [x29, #-176]
Ltmp15531:
	.loc	30 967 18 is_stmt 1
	add	x1, x8, x23
	str	x0, [sp, #56]
Ltmp15532:
	.loc	16 4390 14
	mov	x2, x24
	bl	_memcpy
Ltmp15533:
	.loc	167 502 13
	ldr	x1, [x22, #32]
Ltmp15534:
	.loc	11 521 12
	cbz	x1, LBB147_34
Ltmp15535:
	.loc	167 502 13
	ldr	x0, [x22, #40]
Ltmp15536:
	.loc	24 116 14
	mov	w2, #1
	bl	___rust_dealloc
Ltmp15537:
LBB147_34:
	.loc	24 0 14 is_stmt 0
	ldr	x8, [sp, #56]
	.loc	167 502 13 is_stmt 1
	stp	x24, x8, [x22, #32]
	str	x24, [x22, #48]
	.loc	167 503 53
	sxth	w8, w25
	.loc	167 503 13 is_stmt 0
	stp	w8, w8, [x22, #16]
	mov	w9, #1
	stp	w9, w8, [x22, #24]
	ldp	x15, x2, [sp, #256]
	ldp	x14, x7, [x29, #-200]
	movi.16b	v6, #10
	mov	x16, #-2
	movi.2d	v7, #0xffffffffff000000
	movi.16b	v16, #45
	movi.16b	v17, #208
	movi.16b	v18, #48
	ldp	q20, q19, [sp, #224]
	adrp	x17, lCPI147_4@PAGE
	adrp	x0, lCPI147_5@PAGE
	adrp	x3, lCPI147_6@PAGE
	ldp	x1, x4, [x29, #-256]
	ldur	x6, [x29, #-184]
	ldp	q3, q21, [x29, #-240]
	ldp	q4, q22, [sp, #192]
	ldr	q5, [sp, #176]
	ldp	q2, q23, [sp, #64]
	b	LBB147_8
Ltmp15538:
LBB147_35:
	.loc	167 0 13
	mov	w0, #1
Ltmp15539:
LBB147_36:
	ldur	x8, [x29, #-176]
Ltmp15540:
	.loc	30 967 18 is_stmt 1
	add	x1, x8, x23
	str	x0, [sp, #80]
Ltmp15541:
	.loc	16 4390 14
	mov	x2, x24
	bl	_memcpy
Ltmp15542:
	.loc	167 502 13
	ldr	x1, [x20, #32]
Ltmp15543:
	.loc	11 521 12
	cbz	x1, LBB147_38
Ltmp15544:
	.loc	167 502 13
	ldr	x0, [x20, #40]
Ltmp15545:
	.loc	24 116 14
	mov	w2, #1
	bl	___rust_dealloc
Ltmp15546:
LBB147_38:
	.loc	24 0 14 is_stmt 0
	ldr	x8, [sp, #80]
	.loc	167 502 13 is_stmt 1
	stp	x24, x8, [x20, #32]
	str	x24, [x20, #48]
	.loc	167 503 53
	sxth	w8, w25
	.loc	167 503 13 is_stmt 0
	stp	w8, w8, [x20, #16]
	mov	w9, #1
	stp	w9, w8, [x20, #24]
	ldp	x15, x2, [sp, #256]
	ldp	x14, x7, [x29, #-200]
	movi.16b	v6, #10
	mov	x16, #-2
	movi.2d	v7, #0xffffffffff000000
	movi.16b	v16, #45
	movi.16b	v17, #208
	movi.16b	v18, #48
	ldp	q20, q19, [sp, #224]
	adrp	x17, lCPI147_4@PAGE
	adrp	x0, lCPI147_5@PAGE
	adrp	x3, lCPI147_6@PAGE
	ldp	x1, x4, [x29, #-256]
	ldur	x6, [x29, #-184]
	ldp	q3, q21, [x29, #-240]
	ldp	q4, q22, [sp, #192]
	ldr	q5, [sp, #176]
	b	LBB147_10
Ltmp15547:
LBB147_39:
	.loc	167 0 13
	mov	w0, #1
Ltmp15548:
LBB147_40:
	ldur	x8, [x29, #-176]
Ltmp15549:
	.loc	30 967 18 is_stmt 1
	add	x1, x8, x23
	mov	x28, x0
Ltmp15550:
	.loc	16 4390 14
	mov	x2, x24
	bl	_memcpy
Ltmp15551:
	.loc	167 502 13
	ldr	x1, [x22, #32]
Ltmp15552:
	.loc	11 521 12
	cbz	x1, LBB147_42
Ltmp15553:
	.loc	167 502 13
	ldr	x0, [x22, #40]
Ltmp15554:
	.loc	24 116 14
	mov	w2, #1
	bl	___rust_dealloc
Ltmp15555:
LBB147_42:
	.loc	167 502 13
	stp	x24, x28, [x22, #32]
	str	x24, [x22, #48]
	.loc	167 503 53
	sxth	w8, w25
	.loc	167 503 13 is_stmt 0
	stp	w8, w8, [x22, #16]
	mov	w9, #1
	stp	w9, w8, [x22, #24]
	ldp	x15, x2, [sp, #256]
	ldp	x14, x7, [x29, #-200]
	movi.16b	v6, #10
	mov	x16, #-2
	movi.2d	v7, #0xffffffffff000000
	movi.16b	v16, #45
	movi.16b	v17, #208
	movi.16b	v18, #48
	ldp	q20, q19, [sp, #224]
	adrp	x17, lCPI147_4@PAGE
	adrp	x0, lCPI147_5@PAGE
	adrp	x3, lCPI147_6@PAGE
	ldp	x1, x4, [x29, #-256]
	ldur	x6, [x29, #-184]
	ldp	q3, q21, [x29, #-240]
	ldr	q2, [sp, #192]
	b	LBB147_12
Ltmp15556:
LBB147_43:
	.loc	167 0 13
	mov	w0, #1
Ltmp15557:
LBB147_44:
	ldur	x8, [x29, #-176]
Ltmp15558:
	.loc	30 967 18 is_stmt 1
	add	x1, x8, x23
	mov	x23, x0
Ltmp15559:
	.loc	16 4390 14
	mov	x2, x24
	bl	_memcpy
Ltmp15560:
	.loc	167 502 13
	ldr	x1, [x22, #32]
Ltmp15561:
	.loc	11 521 12
	cbz	x1, LBB147_46
Ltmp15562:
	.loc	167 502 13
	ldr	x0, [x22, #40]
Ltmp15563:
	.loc	24 116 14
	mov	w2, #1
	bl	___rust_dealloc
Ltmp15564:
LBB147_46:
	.loc	167 502 13
	stp	x24, x23, [x22, #32]
	str	x24, [x22, #48]
	.loc	167 503 53
	sxth	w8, w25
	.loc	167 503 13 is_stmt 0
	stp	w8, w8, [x22, #16]
	mov	w9, #1
	stp	w9, w8, [x22, #24]
	ldp	x15, x2, [sp, #256]
	mov	x14, x27
	movi.16b	v6, #10
	mov	x16, #-2
	movi.2d	v7, #0xffffffffff000000
	movi.16b	v16, #45
	movi.16b	v17, #208
	movi.16b	v18, #48
	ldp	q20, q19, [sp, #224]
	adrp	x17, lCPI147_4@PAGE
	adrp	x0, lCPI147_5@PAGE
	adrp	x3, lCPI147_6@PAGE
	ldur	x1, [x29, #-256]
	mov	x4, x28
	ldp	x7, x6, [x29, #-192]
	b	LBB147_14
Ltmp15565:
LBB147_47:
	.loc	167 0 13
	mov	x0, x2
	ldur	x8, [x29, #-176]
Ltmp15566:
	.loc	30 967 18 is_stmt 1
	add	x2, x8, x23
	ldr	x1, [sp, #16]
Ltmp15567:
	.loc	167 507 13
	mov	x3, x24
	mov	x4, x25
	mov	x22, x14
Ltmp15568:
	.loc	167 0 13 is_stmt 0
	mov	x25, x6
Ltmp15569:
	mov	x24, x7
Ltmp15570:
	.loc	167 507 13
	bl	__ZN11onebrc_rust5ver208AggrInfo9slow_save17h4b7cc44bbce66fadE
Ltmp15571:
	.loc	167 0 13
	ldp	q2, q23, [sp, #64]
	ldp	q5, q4, [sp, #176]
	ldp	q22, q20, [sp, #208]
	ldp	q3, q21, [x29, #-240]
	mov	x7, x24
	mov	x6, x25
	ldp	x1, x4, [x29, #-256]
	adrp	x3, lCPI147_6@PAGE
	adrp	x0, lCPI147_5@PAGE
	adrp	x17, lCPI147_4@PAGE
	ldr	q19, [sp, #240]
	movi.16b	v18, #48
	movi.16b	v17, #208
	movi.16b	v16, #45
	movi.2d	v7, #0xffffffffff000000
	mov	x16, #-2
	movi.16b	v6, #10
	ldp	x15, x2, [sp, #256]
	mov	x14, x22
Ltmp15572:
	.loc	167 509 6 is_stmt 1
	b	LBB147_8
Ltmp15573:
LBB147_48:
	.loc	167 0 6 is_stmt 0
	mov	x0, x2
	ldur	x8, [x29, #-176]
Ltmp15574:
	.loc	30 967 18 is_stmt 1
	add	x2, x8, x23
	ldr	x1, [sp, #16]
Ltmp15575:
	.loc	167 507 13
	mov	x3, x24
	mov	x4, x25
	mov	x20, x14
Ltmp15576:
	.loc	167 0 13 is_stmt 0
	mov	x25, x6
Ltmp15577:
	mov	x24, x7
Ltmp15578:
	.loc	167 507 13
	bl	__ZN11onebrc_rust5ver208AggrInfo9slow_save17h4b7cc44bbce66fadE
Ltmp15579:
	.loc	167 0 13
	ldp	q5, q4, [sp, #176]
	ldp	q22, q20, [sp, #208]
	ldp	q3, q21, [x29, #-240]
	mov	x7, x24
	mov	x6, x25
	ldp	x1, x4, [x29, #-256]
	adrp	x3, lCPI147_6@PAGE
	adrp	x0, lCPI147_5@PAGE
	adrp	x17, lCPI147_4@PAGE
	ldr	q19, [sp, #240]
	movi.16b	v18, #48
	movi.16b	v17, #208
	movi.16b	v16, #45
	movi.2d	v7, #0xffffffffff000000
	mov	x16, #-2
	movi.16b	v6, #10
	ldp	x15, x2, [sp, #256]
	mov	x14, x20
Ltmp15580:
	.loc	167 509 6 is_stmt 1
	b	LBB147_10
Ltmp15581:
LBB147_49:
	.loc	167 0 6 is_stmt 0
	mov	x0, x2
	ldur	x8, [x29, #-176]
Ltmp15582:
	.loc	30 967 18 is_stmt 1
	add	x2, x8, x23
	ldr	x1, [sp, #16]
Ltmp15583:
	.loc	167 507 13
	mov	x3, x24
	mov	x24, x4
Ltmp15584:
	mov	x4, x25
	mov	x22, x14
Ltmp15585:
	.loc	167 0 13 is_stmt 0
	mov	x25, x6
Ltmp15586:
	mov	x28, x7
Ltmp15587:
	.loc	167 507 13
	bl	__ZN11onebrc_rust5ver208AggrInfo9slow_save17h4b7cc44bbce66fadE
Ltmp15588:
	.loc	167 0 13
	ldr	q2, [sp, #192]
	ldp	q3, q21, [x29, #-240]
	mov	x7, x28
	mov	x6, x25
	mov	x4, x24
	ldur	x1, [x29, #-256]
	adrp	x3, lCPI147_6@PAGE
	adrp	x0, lCPI147_5@PAGE
	adrp	x17, lCPI147_4@PAGE
	ldp	q20, q19, [sp, #224]
	movi.16b	v18, #48
	movi.16b	v17, #208
	movi.16b	v16, #45
	movi.2d	v7, #0xffffffffff000000
	mov	x16, #-2
	movi.16b	v6, #10
	ldp	x15, x2, [sp, #256]
	mov	x14, x22
Ltmp15589:
	.loc	167 509 6 is_stmt 1
	b	LBB147_12
Ltmp15590:
LBB147_50:
	.loc	167 0 6 is_stmt 0
	mov	x0, x2
	ldur	x8, [x29, #-176]
Ltmp15591:
	.loc	30 967 18 is_stmt 1
	add	x2, x8, x23
	ldr	x1, [sp, #16]
Ltmp15592:
	.loc	167 507 13
	mov	x3, x24
	mov	x23, x4
Ltmp15593:
	mov	x4, x25
	mov	x22, x14
Ltmp15594:
	.loc	167 0 13 is_stmt 0
	mov	x24, x6
Ltmp15595:
	mov	x25, x7
Ltmp15596:
	.loc	167 507 13
	bl	__ZN11onebrc_rust5ver208AggrInfo9slow_save17h4b7cc44bbce66fadE
Ltmp15597:
	.loc	167 0 13
	mov	x7, x25
	mov	x6, x24
	mov	x4, x23
	ldur	x1, [x29, #-256]
	adrp	x3, lCPI147_6@PAGE
	adrp	x0, lCPI147_5@PAGE
	adrp	x17, lCPI147_4@PAGE
	ldp	q20, q19, [sp, #224]
	movi.16b	v18, #48
	movi.16b	v17, #208
	movi.16b	v16, #45
	movi.2d	v7, #0xffffffffff000000
	mov	x16, #-2
	movi.16b	v6, #10
	ldp	x15, x2, [sp, #256]
	mov	x14, x22
Ltmp15598:
	.loc	167 509 6 is_stmt 1
	b	LBB147_14
Ltmp15599:
LBB147_51:
	.loc	167 340 23
	cbz	w26, LBB147_2
Ltmp15600:
	.loc	167 0 23 is_stmt 0
	sub	x1, x19, #64
Ltmp15601:
LBB147_53:
	.loc	56 162 20 is_stmt 1
	rbit	x8, x14
	clz	x8, x8
Ltmp15602:
	.loc	167 165 17
	lsl	x9, x16, x8
	.loc	167 165 9 is_stmt 0
	and	x9, x9, x14
Ltmp15603:
	.loc	167 343 55 is_stmt 1
	orr	x8, x1, x8
Ltmp15604:
	.loc	56 162 20
	rbit	x10, x21
	clz	x10, x10
Ltmp15605:
	.loc	167 165 17
	lsl	x11, x16, x10
	.loc	167 165 9 is_stmt 0
	and	x11, x11, x21
Ltmp15606:
	.loc	167 343 114 is_stmt 1
	add	x10, x10, x19
Ltmp15607:
	.loc	167 343 38 is_stmt 0
	cmp	x14, #0
	csel	x14, x9, xzr, ne
Ltmp15608:
	csel	x21, x21, x11, ne
Ltmp15609:
	csel	x8, x8, x10, ne
Ltmp15610:
	.loc	56 162 20 is_stmt 1
	rbit	x9, x20
	clz	x28, x9
Ltmp15611:
	.loc	56 0 20 is_stmt 0
	ldur	x9, [x29, #-176]
Ltmp15612:
	.loc	16 4390 14 is_stmt 1
	ldr	q0, [x9, x23]
Ltmp15613:
	.loc	167 344 35
	add	x27, x28, x19
Ltmp15614:
	.loc	16 4390 14
	ldr	d1, [x15, x27]
Ltmp15615:
	.loc	144 33 51
	cmeq.8b	v2, v1, v10
	ldr	d3, [x17, lCPI147_4@PAGEOFF]
	and.8b	v2, v2, v3
	addv.8b	b2, v2
Ltmp15616:
	.loc	151 120 26
	ushll.8h	v1, v1, #0
Ltmp15617:
	.loc	167 58 33
	ext.16b	v1, v1, v1, #8
Ltmp15618:
	.loc	150 40 18
	ldr	d3, [x0, lCPI147_5@PAGEOFF]
Ltmp15619:
	ldr	d4, [x3, lCPI147_6@PAGEOFF]
	mla.4h	v4, v1, v3
Ltmp15620:
	.loc	156 80 51
	cmhi.4h	v1, v1, v11
Ltmp15621:
	.loc	150 40 18
	and.8b	v1, v4, v1
Ltmp15622:
	.loc	157 90 13
	ext.8b	v3, v1, v1, #4
Ltmp15623:
	.loc	150 40 18
	add.4h	v1, v3, v1
Ltmp15624:
	.loc	157 90 13
	dup.4h	v3, v1[2]
Ltmp15625:
	.loc	144 33 51
	fmov	w9, s2
Ltmp15626:
	.loc	150 40 18
	add.4h	v1, v3, v1
Ltmp15627:
	.loc	167 70 5
	umov.h	w10, v1[3]
Ltmp15628:
	.loc	167 353 36
	tst	w9, #0x3f
	cneg	w25, w10, ne
Ltmp15629:
	.loc	167 357 70
	sub	x24, x8, x23
Ltmp15630:
	.loc	157 90 13
	dup.16b	v1, w24
Ltmp15631:
	.loc	156 59 51
	cmhi.16b	v1, v1, v20
Ltmp15632:
	.loc	163 31 18
	and.16b	v0, v0, v1
Ltmp15633:
	.loc	167 476 23
	fmov	x9, d0
	.loc	167 476 32 is_stmt 0
	mov.d	x10, v0[1]
Ltmp15634:
	.loc	167 480 22 is_stmt 1
	lsr	x11, x9, #20
Ltmp15635:
	.loc	167 481 22
	lsr	x12, x10, #20
Ltmp15636:
	.loc	167 484 13
	eor	x13, x9, x10
	lsr	x13, x13, #40
	eor	w11, w11, w12
Ltmp15637:
	eor	w11, w13, w11
	eor	w9, w9, w10
Ltmp15638:
	eor	w9, w11, w9
Ltmp15639:
	.loc	167 487 32
	and	x5, x9, #0xfffff
Ltmp15640:
	.loc	20 97 14
	add	x22, x2, x5, lsl #6
Ltmp15641:
	.loc	143 904 49
	ldr	q1, [x22]
Ltmp15642:
	.loc	145 289 18
	cmeq.2d	v1, v1, v0
	mvn.16b	v1, v1
	xtn.2s	v1, v1
	umaxp.2s	v1, v1, v1
	fmov	w9, s1
Ltmp15643:
	.loc	16 1506 8
	tbnz	w9, #0, LBB147_56
Ltmp15644:
	.loc	167 447 18
	ldp	w8, w9, [x22, #16]
Ltmp15645:
	.loc	167 494 30
	sxth	w10, w25
Ltmp15646:
	.loc	17 0 0 is_stmt 0
	cmp	w8, w10
	csel	w8, w8, w10, lt
Ltmp15647:
	cmp	w9, w10
	csel	w9, w9, w10, gt
Ltmp15648:
	.loc	167 497 13 is_stmt 1
	stp	w8, w9, [x22, #16]
Ltmp15649:
	.loc	167 447 18
	ldr	d0, [x22, #24]
Ltmp15650:
	.loc	167 493 13
	movi.2s	v1, #1
	mov.s	v1[1], w10
	add.2s	v0, v0, v1
	.loc	167 497 13
	str	d0, [x22, #24]
Ltmp15651:
LBB147_55:
	.loc	167 0 0 is_stmt 0
	lsl	x8, x16, x28
	and	x20, x8, x20
Ltmp15652:
	.loc	167 362 21 is_stmt 1
	add	x23, x27, #1
Ltmp15653:
	.loc	167 361 21
	subs	w26, w26, #1
Ltmp15654:
	.loc	167 0 21 is_stmt 0
	b.ne	LBB147_53
	b	LBB147_2
Ltmp15655:
LBB147_56:
	.loc	9 2683 19 is_stmt 1
	ldr	x9, [x22, #48]
Ltmp15656:
	.loc	16 1506 8
	cbnz	x9, LBB147_64
Ltmp15657:
	.loc	167 501 13
	str	q0, [x22]
Ltmp15658:
	.loc	26 376 16
	tbnz	x24, #63, LBB147_67
Ltmp15659:
	.loc	26 0 16 is_stmt 0
	stur	x1, [x29, #-184]
	stur	x14, [x29, #-200]
Ltmp15660:
	.loc	11 465 12 is_stmt 1
	cmp	x8, x23
	b.ne	LBB147_60
Ltmp15661:
	.loc	11 0 12 is_stmt 0
	mov	w0, #1
	.loc	11 465 12
	b	LBB147_61
Ltmp15662:
LBB147_60:
	.loc	5 1757 9 is_stmt 1
Lloh1034:
	adrp	x8, ___rust_no_alloc_shim_is_unstable@GOTPAGE
Ltmp15663:
Lloh1035:
	ldr	x8, [x8, ___rust_no_alloc_shim_is_unstable@GOTPAGEOFF]
Ltmp15664:
	ldrb	wzr, [x8]
Ltmp15665:
	.loc	24 96 9
	mov	x0, x24
	mov	w1, #1
	bl	___rust_alloc
Ltmp15666:
	.loc	11 478 19
	cbz	x0, LBB147_68
Ltmp15667:
LBB147_61:
	.loc	11 0 19 is_stmt 0
	ldur	x8, [x29, #-176]
Ltmp15668:
	.loc	30 967 18 is_stmt 1
	add	x1, x8, x23
	mov	x23, x0
Ltmp15669:
	.loc	16 4390 14
	mov	x2, x24
	bl	_memcpy
Ltmp15670:
	.loc	167 502 13
	ldr	x1, [x22, #32]
Ltmp15671:
	.loc	11 521 12
	cbz	x1, LBB147_63
Ltmp15672:
	.loc	167 502 13
	ldr	x0, [x22, #40]
Ltmp15673:
	.loc	24 116 14
	mov	w2, #1
	bl	___rust_dealloc
Ltmp15674:
LBB147_63:
	.loc	167 502 13
	stp	x24, x23, [x22, #32]
	str	x24, [x22, #48]
	.loc	167 503 53
	sxth	w8, w25
	.loc	167 503 13 is_stmt 0
	stp	w8, w8, [x22, #16]
	mov	w9, #1
	stp	w9, w8, [x22, #24]
	ldp	x15, x2, [sp, #256]
	ldur	x14, [x29, #-200]
	movi.16b	v6, #10
	mov	x16, #-2
	movi.2d	v7, #0xffffffffff000000
	movi.16b	v16, #45
	movi.16b	v17, #208
	movi.16b	v18, #48
	ldp	q20, q19, [sp, #224]
	adrp	x17, lCPI147_4@PAGE
	adrp	x0, lCPI147_5@PAGE
	adrp	x3, lCPI147_6@PAGE
	ldur	x1, [x29, #-184]
	b	LBB147_55
Ltmp15675:
LBB147_64:
	.loc	167 0 13
	mov	x0, x2
	ldur	x8, [x29, #-176]
Ltmp15676:
	.loc	30 967 18 is_stmt 1
	add	x2, x8, x23
	mov	x23, x1
Ltmp15677:
	.loc	30 0 18 is_stmt 0
	ldr	x1, [sp, #16]
Ltmp15678:
	.loc	167 507 13 is_stmt 1
	mov	x3, x24
	mov	x4, x25
	mov	x22, x14
Ltmp15679:
	bl	__ZN11onebrc_rust5ver208AggrInfo9slow_save17h4b7cc44bbce66fadE
Ltmp15680:
	.loc	167 0 13 is_stmt 0
	mov	x1, x23
	adrp	x3, lCPI147_6@PAGE
	adrp	x0, lCPI147_5@PAGE
	adrp	x17, lCPI147_4@PAGE
	ldp	q20, q19, [sp, #224]
	movi.16b	v18, #48
	movi.16b	v17, #208
	movi.16b	v16, #45
	movi.2d	v7, #0xffffffffff000000
	mov	x16, #-2
	movi.16b	v6, #10
	ldp	x15, x2, [sp, #256]
	mov	x14, x22
Ltmp15681:
	.loc	167 509 6 is_stmt 1
	b	LBB147_55
Ltmp15682:
LBB147_65:
	.loc	63 593 9
Lloh1036:
	adrp	x8, l___unnamed_170@PAGE
Lloh1037:
	add	x8, x8, l___unnamed_170@PAGEOFF
	mov	w9, #1
	stp	x8, x9, [x29, #-168]
	mov	w8, #8
	stp	xzr, xzr, [x29, #-144]
	stur	x8, [x29, #-152]
Ltmp15683:
	.loc	167 374 17
	sub	x0, x29, #168
	bl	__ZN3std2io5stdio6_print17h33097c8113d0a5f1E
Ltmp15684:
LBB147_66:
	.loc	167 380 6 epilogue_begin
	add	sp, sp, #416
	.cfi_def_cfa wsp, 128
	ldp	x29, x30, [sp, #112]
	ldp	x20, x19, [sp, #96]
	ldp	x22, x21, [sp, #80]
	ldp	x24, x23, [sp, #64]
	ldp	x26, x25, [sp, #48]
	ldp	x28, x27, [sp, #32]
	ldp	d9, d8, [sp, #16]
	ldp	d11, d10, [sp], #128
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	.cfi_restore b8
	.cfi_restore b9
	.cfi_restore b10
	.cfi_restore b11
	ret
LBB147_67:
	.cfi_restore_state
	.loc	167 0 6 is_stmt 0
	str	xzr, [sp, #8]
LBB147_68:
Ltmp15685:
	.loc	11 428 25 is_stmt 1
Lloh1038:
	adrp	x2, l___unnamed_16@PAGE
Lloh1039:
	add	x2, x2, l___unnamed_16@PAGEOFF
	ldr	x0, [sp, #8]
	mov	x1, x24
	bl	__ZN5alloc7raw_vec12handle_error17h6b07f4a4f8598f15E
Ltmp15686:
	.loh AdrpLdr	Lloh1024, Lloh1025
	.loh AdrpAdrp	Lloh1022, Lloh1024
	.loh AdrpLdr	Lloh1022, Lloh1023
	.loh AdrpAdrp	Lloh1020, Lloh1022
	.loh AdrpLdr	Lloh1020, Lloh1021
	.loh AdrpAdrp	Lloh1018, Lloh1020
	.loh AdrpLdr	Lloh1018, Lloh1019
	.loh AdrpLdrGot	Lloh1026, Lloh1027
	.loh AdrpLdrGot	Lloh1028, Lloh1029
	.loh AdrpLdrGot	Lloh1030, Lloh1031
	.loh AdrpLdrGot	Lloh1032, Lloh1033
	.loh AdrpLdrGot	Lloh1034, Lloh1035
	.loh AdrpAdd	Lloh1036, Lloh1037
	.loh AdrpAdd	Lloh1038, Lloh1039
Lfunc_end147:
	.cfi_endproc

```