
```asm
onebrc_rust::ver20::FileReader::ver20_scan_loop:
	stp    d11, d10, [sp, #-0x80]!    ; save callee's d8-d11, x19-x30 
	stp    d9, d8, [sp, #0x10]      
	stp    x28, x27, [sp, #0x20]    
	stp    x26, x25, [sp, #0x30]    
	stp    x24, x23, [sp, #0x40]    
	stp    x22, x21, [sp, #0x50]    
	stp    x20, x19, [sp, #0x60]    
	stp    x29, x30, [sp, #0x70]    
	add    x29, sp, #0x70           
	sub    sp, sp, #0x1c0           
	str    x3, [sp, #0x10]          
	cbz    x1, 0x1000109cc            ; x0 = self.mmap.buffer   x1 = self.mmap.length   ; if length == 0, return
	mov    x17, #0x0                  ; x17 = cursor
	mov    x23, #0x0                  ; =0
	mov    x14, #0x0                  ; =0
	ldp    q2, q1, [x0, #0x20]        ; q0, q3, q2, q1 = buffer[0..64]
	sub    x15, x0, #0x8            
	movi.16b v6, #0xa               
	mov    x16, #-0x2                 ; =-2
	ldp    q0, q3, [x0]             
	stp    q1, q3, [sp, #0xa0]        ; [sp, #0xa0] = (v_3, v_1)
	stp    q2, q0, [sp, #0x80]        ; [sp, #0x80] = (v_2, v_0)
	movi.2d v7, #0xffffffffff000000    ; 0xFFFF_FFFF_FF00_0000 value mask
	adrp   x8, 167                  
	ldr    q0, [x8, #0x1a0]           ; q0 = 0x8040_2010_0804_0201_8040_2010_0804_0201        
	str    q0, [sp, #0x20]          
	movi.16b v16, #0x2d             
	movi.4h v8, #0x1                
	movi.16b v17, #0xd0             
	movi.16b v18, #0x30             
	adrp   x8, 167                  
	ldr    d9, [x8, #0x7b0]         ; d9 = 0x0000_0000_0000_0000_0100_0a64_0100_0a64        
	adrp   x8, 167                  
	ldr    q19, [x8, #0x790]        ; q19 = 0xffd0_0000_fe20_ed40_ffd0_0000_fe20_ed40
	adrp   x8, 167                  
	ldr    q20, [x8, #0x7a0]       ; q20 = 0x0f0e_0d0c_0b0a_0908_0706_0504_0302_0100 
	mov    w8, #0x1                   ; =1
	str    x8, [sp, #0x8]           
	movi.8b v10, #0x2d              
	adrp   x3, 167                  
	movi.4h v11, #0x2f              
	adrp   x4, 167                  
	adrp   x6, 167                  
	str    x2, [sp, #0x128]         ; x2 = aggr     
	stur   x0, [x29, #-0x100]       ; buffer 
	str    x15, [sp, #0x120]        
	stp    q20, q19, [sp, #0x100]   
	str    x1, [sp, #0x18]          
	b      0x10000fc98                ; <+224> at ver20.rs:232:16
	cmp    x14, #0x0                
	csel   x14, x22, x14, eq        
	ldr    x8, [sp, #0x70]          
	mov    x17, x8                  
	ldr    x1, [sp, #0x18]          
	cmp    x8, x1                   
	b.hs   0x1000109cc                ; <+3604> at ver20.rs:380:6
fc98:	
	add    x11, x17, #0x40           ; cursor + 0x40 < length          
	cmp    x11, x1                  
	b.hi   0x1000109a8                ; <+3568> [inlined] core::fmt::Arguments::new_const at mod.rs:593:9
	movi.16b v2, #0x3b              ; cursor + 0x40 < length
	ldp    q22, q21, [sp, #0x80]    ; 从局部变量中读 (v_2, v_0)
	cmeq.16b v0, v22, v2            ; v_2 == 0x3b ';'
	ldr    q3, [sp, #0x20]          ; q3 = 0x8040_2010_0804_0201_8040_2010_0804_0201
	and.16b v0, v0, v3              ; v0 = (v_2 == 0x3b) & 0x8040_2010_0804_0201 
	ext.16b v1, v0, v0, #0x8        
	zip1.16b v0, v0, v1             
	addv.8h h0, v0                  
	fmov   w8, s0                   
	ldp    q5, q4, [sp, #0xa0]     ; 从局部变量中读 (v_3, v_1)   ; simd_eq to_bit_mask 的计算过程有多次 load 操作，理论上是多余的？
	cmeq.16b v0, v5, v2             
	and.16b v0, v0, v3              
	ext.16b v1, v0, v0, #0x8        
	zip1.16b v0, v0, v1             
	addv.8h h0, v0                  
	fmov   w9, s0                   
	bfi    w8, w9, #16, #16         
	cmeq.16b v0, v21, v2            
	and.16b v0, v0, v3              
	ext.16b v1, v0, v0, #0x8        
	zip1.16b v0, v0, v1             
	addv.8h h0, v0                  
	fmov   w9, s0                   
	cmeq.16b v0, v4, v2             
	and.16b v0, v0, v3              
	ext.16b v1, v0, v0, #0x8        
	zip1.16b v0, v0, v1             
	addv.8h h0, v0                  
	fmov   w10, s0                  
	bfi    w9, w10, #16, #16        
	orr    x22, x9, x8, lsl #32     
	cmeq.16b v0, v22, v6            
	and.16b v0, v0, v3              
	ext.16b v1, v0, v0, #0x8        
	zip1.16b v0, v0, v1             
	addv.8h h0, v0                  
	fmov   w8, s0                   
	cmeq.16b v0, v5, v6             
	and.16b v0, v0, v3              
	ext.16b v1, v0, v0, #0x8        
	zip1.16b v0, v0, v1             
	addv.8h h0, v0                  
	fmov   w9, s0                   
	bfi    w8, w9, #16, #16         
	cmeq.16b v0, v21, v6            
	and.16b v0, v0, v3              
	ext.16b v1, v0, v0, #0x8        
	zip1.16b v0, v0, v1             
	addv.8h h0, v0                  
	fmov   w9, s0                   
	cmeq.16b v0, v4, v6             
	and.16b v0, v0, v3              
	ext.16b v1, v0, v0, #0x8        
	zip1.16b v0, v0, v1             
	addv.8h h0, v0                  
	fmov   w10, s0                  
	bfi    w9, w10, #16, #16        
	orr    x24, x9, x8, lsl #32     
	str    x11, [sp, #0x70]         
	add    x8, x0, x11              
	ldp    q2, q1, [x8, #0x20]      
	ldp    q0, q3, [x8]             
	stp    q1, q3, [sp, #0xa0]      
	stp    q2, q0, [sp, #0x80]      
	fmov   d0, x24                  
	cnt.8b v0, v0                   
	uaddlv.8b h0, v0                
	fmov   w21, s0                  
	cmp    x21, #0x4                
	stur   x17, [x29, #-0xf0]       
	b.lo   0x100010754                ; <+2972> at ver20.rs:340:23
	sub    x1, x17, #0x40           
	stur   x1, [x29, #-0xf8]        
	rbit   x8, x22                  
	clz    x8, x8                   
	lsl    x9, x16, x8              
	and    x9, x9, x22              
	add    x8, x8, x17              
	rbit   x10, x14                 
	clz    x10, x10                 
	lsl    x11, x16, x10            
	and    x11, x11, x14            
	orr    x10, x10, x1             
	cmp    x14, #0x0                
	csel   x14, xzr, x11, eq        
	csel   x28, x9, x22, eq         
	csel   x8, x8, x10, eq          
	rbit   x9, x24                  
	clz    x9, x9                   
	lsl    x10, x16, x9             
	and    x10, x10, x24            
	rbit   x11, x10                 
	clz    x11, x11                 
	lsl    x12, x16, x11            
	and    x10, x12, x10            
	rbit   x12, x10                 
	clz    x12, x12                 
	lsl    x13, x16, x12            
	and    x13, x13, x10            
	rbit   x10, x13                 
	clz    x7, x10                  
	ldr    q0, [x0, x23]            
	add    x27, x9, x17             
	add    x22, x0, x27             
	ldr    q23, [x22, #0x1]!        
	add    x24, x11, x17            
	add    x20, x0, x24             
	ldr    q22, [x20, #0x1]!        
	add    x9, x12, x17             
	stur   x9, [x29, #-0xb0]        
	add    x11, x0, x9              
	ldr    q21, [x11, #0x1]!        
	ldur   x9, [x22, #-0x9]         
	sub    x10, x20, #0x9           
	stur   x11, [x29, #-0xb8]       
	ldur   x11, [x11, #-0x9]        
	add    x30, x7, x17             
	add    x12, x15, x30            
	fmov   d1, x11                  
	ld1.d  { v1 }[1], [x12]         
	and.16b v2, v1, v7              
	fmov   d3, x9                   
	ld1.d  { v3 }[1], [x10]         
	and.16b v4, v3, v7              
	cmeq.16b v2, v2, v16            
	cmeq.16b v4, v4, v16            
	ext.16b v5, v4, v2, #0x1        
	ext.16b v2, v2, v4, #0x1        
	cmeq.2d v2, v2, #0              
	cmeq.2d v4, v5, #0              
	uzp1.4s v2, v4, v2              
	xtn.4h v2, v2                   
	and.8b v4, v2, v8               
	orn.8b v4, v4, v2               
	uzp2.4s v1, v3, v1              
	add.16b v2, v1, v17             
	cmhi.16b v2, v6, v2             
	bif.16b v1, v18, v2             
	ext.16b v2, v1, v1, #0x8        
	mov.16b v3, v19                 
	smlal.8h v3, v1, v9             
	mov.16b v5, v19                 
	smlal.8h v5, v2, v9             
	ext.16b v1, v5, v3, #0xc        
	add.8h v1, v1, v3               
	rev32.8h v2, v1                 
	add.8h v2, v2, v1               
	umov.h w9, v4[0]                
	umov.h w10, v2[3]               
	mul    w19, w10, w9             
	sub    x25, x8, x23             
	dup.16b v1, w25                 
	cmhi.16b v1, v1, v20            
	and.16b v0, v0, v1              
	fmov   x8, d0                   
	mov.d  x9, v0[1]                
	eor    x10, x8, x9              
	lsr    x11, x10, #40            
	lsr    x10, x10, #20            
	eor    w10, w10, w11            
	eor    w8, w8, w9               
	eor    w8, w10, w8              
	and    x5, x8, #0xfffff         
	add    x26, x2, x5, lsl #6      
	ldr    q1, [x26]                
	cmeq.2d v1, v1, v0              
	mvn.16b v1, v1                  
	xtn.2s v1, v1                   
	umaxp.2s v1, v1, v1             
	fmov   w8, s1                   
	tbnz   w8, #0x0, 0x1000101a8      ; <+1520> [inlined] alloc::vec::Vec<T,A>::len at mod.rs:2683:19
	ldp    w8, w9, [x26, #0x10]     
	sxth   w10, w19                 
	cmp    w8, w10                  
	csel   w8, w8, w10, lt          
	cmp    w9, w10                  
	csel   w9, w9, w10, gt          
	stp    w8, w9, [x26, #0x10]     
	ldr    d0, [x26, #0x18]         
	movi.2s v1, #0x1                
	mov.s  v1[1], w10               
	add.2s v0, v0, v1               
	str    d0, [x26, #0x18]         
	rbit   x8, x28                  
	clz    x23, x8                  
	umov.h w8, v4[1]                
	umov.h w9, v2[7]                
	mul    w19, w9, w8              
	mvn    x8, x27                  
	add    x9, x17, x23             
	add    x25, x9, x8              
	dup.16b v0, w25                 
	cmhi.16b v0, v0, v20            
	and.16b v0, v23, v0             
	fmov   x8, d0                   
	mov.d  x9, v0[1]                
	eor    x10, x8, x9              
	lsr    x11, x10, #40            
	lsr    x10, x10, #20            
	eor    w10, w10, w11            
	eor    w8, w8, w9               
	eor    w8, w10, w8              
	and    x5, x8, #0xfffff         
	add    x26, x2, x5, lsl #6      
	ldr    q1, [x26]                
	cmeq.2d v1, v1, v0              
	mvn.16b v1, v1                  
	xtn.2s v1, v1                   
	umaxp.2s v1, v1, v1             
	fmov   w8, s1                   
	tbnz   w8, #0x0, 0x100010200      ; <+1608> [inlined] alloc::vec::Vec<T,A>::len at mod.rs:2683:19
	ldp    w8, w9, [x26, #0x10]     
	sxth   w10, w19                 
	cmp    w8, w10                  
	csel   w8, w8, w10, lt          
	cmp    w9, w10                  
	csel   w9, w9, w10, gt          
	stp    w8, w9, [x26, #0x10]     
	ldr    d0, [x26, #0x18]         
	movi.2s v1, #0x1                
	mov.s  v1[1], w10               
	add.2s v0, v0, v1               
	str    d0, [x26, #0x18]         
	lsl    x8, x16, x23             
	and    x23, x8, x28             
	rbit   x8, x23                  
	clz    x25, x8                  
	ext.16b v0, v3, v5, #0xc        
	add.8h v0, v0, v5               
	rev32.8h v1, v0                 
	add.8h v2, v1, v0               
	umov.h w8, v4[2]                
	umov.h w9, v2[3]                
	mul    w19, w9, w8              
	mvn    x8, x24                  
	add    x9, x17, x25             
	add    x22, x9, x8              
	dup.16b v0, w22                 
	cmhi.16b v0, v0, v20            
	and.16b v0, v22, v0             
	fmov   x8, d0                   
	mov.d  x9, v0[1]                
	eor    x10, x8, x9              
	lsr    x11, x10, #40            
	lsr    x10, x10, #20            
	eor    w10, w10, w11            
	eor    w8, w8, w9               
	eor    w8, w10, w8              
	and    x5, x8, #0xfffff         
	add    x24, x2, x5, lsl #6      
	ldr    q1, [x24]                
	cmeq.2d v1, v1, v0              
	mvn.16b v1, v1                  
	xtn.2s v1, v1                   
	umaxp.2s v1, v1, v1             
	fmov   w8, s1                   
	tbnz   w8, #0x0, 0x1000102d4      ; <+1820> [inlined] alloc::vec::Vec<T,A>::len at mod.rs:2683:19
	ldp    w8, w9, [x24, #0x10]     
	sxth   w10, w19                 
	cmp    w8, w10                  
	csel   w8, w8, w10, lt          
	cmp    w9, w10                  
	csel   w9, w9, w10, gt          
	stp    w8, w9, [x24, #0x10]     
	ldr    d0, [x24, #0x18]         
	movi.2s v1, #0x1                
	mov.s  v1[1], w10               
	add.2s v0, v0, v1               
	str    d0, [x24, #0x18]         
	ldur   x10, [x29, #-0xb0]       
	lsl    x8, x16, x25             
	and    x22, x8, x23             
	rbit   x8, x22                  
	clz    x23, x8                  
	umov.h w8, v4[3]                
	umov.h w9, v2[7]                
	mul    w19, w9, w8              
	mvn    x8, x10                  
	add    x9, x17, x23             
	add    x20, x9, x8              
	dup.16b v0, w20                 
	cmhi.16b v0, v0, v20            
	and.16b v0, v21, v0             
	fmov   x8, d0                   
	mov.d  x9, v0[1]                
	eor    x10, x8, x9              
	lsr    x11, x10, #40            
	lsr    x10, x10, #20            
	eor    w10, w10, w11            
	eor    w8, w8, w9               
	eor    w8, w10, w8              
	and    x5, x8, #0xfffff         
	add    x24, x2, x5, lsl #6      
	ldr    q1, [x24]                
	cmeq.2d v1, v1, v0              
	mvn.16b v1, v1                  
	xtn.2s v1, v1                   
	umaxp.2s v1, v1, v1             
	fmov   w8, s1                   
	tbnz   w8, #0x0, 0x1000103ac      ; <+2036> [inlined] alloc::vec::Vec<T,A>::len at mod.rs:2683:19
	ldp    w8, w9, [x24, #0x10]     
	sxth   w10, w19                 
	cmp    w8, w10                  
	csel   w8, w8, w10, lt          
	cmp    w9, w10                  
	csel   w9, w9, w10, gt          
	stp    w8, w9, [x24, #0x10]     
	ldr    d0, [x24, #0x18]         
	movi.2s v1, #0x1                
	mov.s  v1[1], w10               
	add.2s v0, v0, v1               
	str    d0, [x24, #0x18]         
	lsl    x8, x16, x23             
	and    x22, x8, x22             
	lsl    x8, x16, x7              
	and    x24, x8, x13             
	sub    w21, w21, #0x4           
	add    x23, x30, #0x1           
	cmp    w21, #0x3                
	b.hi   0x10000fdc8                ; <+528> [inlined] core::num::<impl u64>::trailing_zeros at uint_macros.rs:162:20
	b      0x100010754                ; <+2972> at ver20.rs:340:23
	ldr    x8, [x26, #0x30]         
	stp    q4, q21, [x29, #-0xe0]   
	stp    q3, q22, [sp, #0xe0]     
	str    q5, [sp, #0xd0]          
	stp    q2, q23, [sp, #0x50]     
	cbnz   x8, 0x10001056c            ; <+2484> at raw_vec.rs
	str    q0, [x26]                
	tbnz   x25, #0x3f, 0x100010a0c    ; <+3668> at raw_vec.rs
	stur   x30, [x29, #-0xe8]       
	stp    x7, x13, [sp, #0xc0]     
	str    x14, [sp, #0x78]         
	cbz    x25, 0x100010474           ; <+2236> at main.rs
	adrp   x8, 216                  
	add    x8, x8, #0xd29             ; __rust_no_alloc_shim_is_unstable
	ldrb   wzr, [x8]                
	mov    x0, x25                  
	mov    w1, #0x1                   ; =1
	bl     0x1000260b4                ; __rust_alloc
	cbz    x0, 0x100010a6c            ; <+3764> at raw_vec.rs
	mov    x8, x0                   
	ldur   x0, [x29, #-0x100]       
	b      0x100010478                ; <+2240> [inlined] core::ptr::const_ptr::<impl *const T>::add at const_ptr.rs:967:18
	ldr    x8, [x26, #0x30]         
	stp    q4, q21, [x29, #-0xe0]   
	stp    q3, q22, [sp, #0xe0]     
	str    q5, [sp, #0xd0]          
	cbnz   x8, 0x1000105f0            ; <+2616> [inlined] onebrc_rust::ver20::AggrInfo::save_item_u64x2 at ver20.rs:507:13
	str    q0, [x26]                
	tbnz   x25, #0x3f, 0x1000109f4    ; <+3644> at ver20.rs
	stur   x30, [x29, #-0xe8]       
	stp    x7, x13, [sp, #0xc0]     
	mov    x27, x14                 
	cbz    x25, 0x10001050c           ; <+2388> at main.rs
	adrp   x8, 216                  
	add    x8, x8, #0xd29             ; __rust_no_alloc_shim_is_unstable
	ldrb   wzr, [x8]                
	mov    x0, x25                  
	mov    w1, #0x1                   ; =1
	bl     0x1000260b4                ; __rust_alloc
	cbz    x0, 0x100010a9c            ; <+3812> at raw_vec.rs
	str    x0, [sp, #0x48]          
	mov    x1, x22                  
	mov    x2, x25                  
	bl     0x1000b0020                ; symbol stub for: memcpy
	ldr    x1, [x26, #0x20]         
	cbz    x1, 0x10001026c            ; <+1716> [inlined] onebrc_rust::ver20::AggrInfo::save_item_u64x2 + 108 at alloc.rs
	ldr    x0, [x26, #0x28]         
	mov    w2, #0x1                   ; =1
	bl     0x1000260b8                ; __rust_dealloc
	ldr    x8, [sp, #0x48]          
	stp    x25, x8, [x26, #0x20]    
	str    x25, [x26, #0x30]        
	sxth   w8, w19                  
	stp    w8, w8, [x26, #0x10]     
	mov    w9, #0x1                   ; =1
	stp    w9, w8, [x26, #0x18]     
	ldp    x15, x2, [sp, #0x120]    
	ldp    x0, x1, [x29, #-0x100]   
	mov    x14, x27                 
	movi.16b v6, #0xa               
	mov    x16, #-0x2                 ; =-2
	movi.2d v7, #0xffffffffff000000 
	movi.16b v16, #0x2d             
	ldp    x17, x30, [x29, #-0xf0]  
	movi.16b v17, #0xd0             
	movi.16b v18, #0x30             
	ldp    q20, q19, [sp, #0x100]   
	adrp   x3, 166                  
	adrp   x4, 166                  
	adrp   x6, 166                  
	ldp    x7, x13, [sp, #0xc0]     
	ldp    q4, q21, [x29, #-0xe0]   
	ldp    q3, q22, [sp, #0xe0]     
	ldr    q5, [sp, #0xd0]          
	b      0x100010020                ; <+1128> at ver20.rs
	ldr    x8, [x24, #0x30]         
	stp    q4, q21, [x29, #-0xe0]   
	str    q2, [sp, #0xe0]          
	cbnz   x8, 0x10001066c            ; <+2740> [inlined] onebrc_rust::ver20::AggrInfo::save_item_u64x2 at ver20.rs:507:13
	str    q0, [x24]                
	tbnz   x22, #0x3f, 0x100010a3c    ; <+3716> at raw_vec.rs
	stur   x30, [x29, #-0xe8]       
	mov    x28, x7                  
	mov    x27, x13                 
	mov    x26, x14                 
	cbz    x22, 0x10001052c           ; <+2420> at raw_vec.rs
	adrp   x8, 216                  
	add    x8, x8, #0xd29             ; __rust_no_alloc_shim_is_unstable
	ldrb   wzr, [x8]                
	mov    x0, x22                  
	mov    w1, #0x1                   ; =1
	bl     0x1000260b4                ; __rust_alloc
	cbz    x0, 0x100010a54            ; <+3740> [inlined] alloc::raw_vec::RawVecInner<A>::with_capacity_in + 20 at raw_vec.rs
	str    x0, [sp, #0x38]          
	mov    x1, x20                  
	mov    x2, x22                  
	bl     0x1000b0020                ; symbol stub for: memcpy
	ldr    x1, [x24, #0x20]         
	cbz    x1, 0x100010340            ; <+1928> [inlined] onebrc_rust::ver20::AggrInfo::save_item_u64x2 + 108 at alloc.rs
	ldr    x0, [x24, #0x28]         
	mov    w2, #0x1                   ; =1
	bl     0x1000260b8                ; __rust_dealloc
	ldr    x8, [sp, #0x38]          
	stp    x22, x8, [x24, #0x20]    
	str    x22, [x24, #0x30]        
	sxth   w8, w19                  
	stp    w8, w8, [x24, #0x10]     
	mov    w9, #0x1                   ; =1
	stp    w9, w8, [x24, #0x18]     
	ldp    x15, x2, [sp, #0x120]    
	ldp    x0, x1, [x29, #-0x100]   
	mov    x14, x26                 
	movi.16b v6, #0xa               
	mov    x16, #-0x2                 ; =-2
	movi.2d v7, #0xffffffffff000000 
	movi.16b v16, #0x2d             
	ldp    x17, x30, [x29, #-0xf0]  
	movi.16b v17, #0xd0             
	movi.16b v18, #0x30             
	ldp    q20, q19, [sp, #0x100]   
	adrp   x3, 166                  
	adrp   x4, 166                  
	adrp   x6, 166                  
	mov    x13, x27                 
	mov    x7, x28                  
	ldp    q4, q21, [x29, #-0xe0]   
	ldur   x10, [x29, #-0xb0]       
	ldr    q2, [sp, #0xe0]          
	b      0x1000100dc                ; <+1316> at ver20.rs
	ldr    x8, [x24, #0x30]         
	cbnz   x8, 0x1000106e4            ; <+2860> [inlined] onebrc_rust::ver20::AggrInfo::save_item_u64x2 at ver20.rs:507:13
	str    q0, [x24]                
	tbnz   x20, #0x3f, 0x100010a24    ; <+3692> at raw_vec.rs
	mov    x28, x30                 
	mov    x27, x7                  
	mov    x26, x13                 
	mov    x25, x14                 
	cbz    x20, 0x10001054c           ; <+2452> at raw_vec.rs
	adrp   x8, 216                  
	add    x8, x8, #0xd29             ; __rust_no_alloc_shim_is_unstable
	ldrb   wzr, [x8]                
	mov    x0, x20                  
	mov    w1, #0x1                   ; =1
	bl     0x1000260b4                ; __rust_alloc
	cbz    x0, 0x100010a84            ; <+3788> at raw_vec.rs
	ldur   x1, [x29, #-0xb8]        
	str    x0, [sp, #0x30]          
	mov    x2, x20                  
	bl     0x1000b0020                ; symbol stub for: memcpy
	ldr    x1, [x24, #0x20]         
	cbz    x1, 0x100010410            ; <+2136> [inlined] onebrc_rust::ver20::AggrInfo::save_item_u64x2 + 100 at alloc.rs
	ldr    x0, [x24, #0x28]         
	mov    w2, #0x1                   ; =1
	bl     0x1000260b8                ; __rust_dealloc
	ldr    x8, [sp, #0x30]          
	stp    x20, x8, [x24, #0x20]    
	str    x20, [x24, #0x30]        
	sxth   w8, w19                  
	stp    w8, w8, [x24, #0x10]     
	mov    w9, #0x1                   ; =1
	stp    w9, w8, [x24, #0x18]     
	ldp    x15, x2, [sp, #0x120]    
	ldp    x0, x1, [x29, #-0x100]   
	mov    x14, x25                 
	movi.16b v6, #0xa               
	mov    x16, #-0x2                 ; =-2
	movi.2d v7, #0xffffffffff000000 
	movi.16b v16, #0x2d             
	ldur   x17, [x29, #-0xf0]       
	movi.16b v17, #0xd0             
	movi.16b v18, #0x30             
	ldp    q20, q19, [sp, #0x100]   
	adrp   x3, 166                  
	adrp   x4, 166                  
	adrp   x6, 166                  
	mov    x13, x26                 
	mov    x7, x27                  
	mov    x30, x28                 
	b      0x100010184                ; <+1484> at ver20.rs
	mov    w8, #0x1                   ; =1
	add    x1, x0, x23              
	str    x8, [sp, #0x40]          
	mov    x0, x8                   
	mov    x2, x25                  
	bl     0x1000b0020                ; symbol stub for: memcpy
	ldr    x1, [x26, #0x20]         
	cbz    x1, 0x1000104a0            ; <+2280> [inlined] onebrc_rust::ver20::AggrInfo::save_item_u64x2 + 32 at alloc.rs
	ldr    x0, [x26, #0x28]         
	mov    w2, #0x1                   ; =1
	bl     0x1000260b8                ; __rust_dealloc
	ldr    x8, [sp, #0x40]          
	stp    x25, x8, [x26, #0x20]    
	str    x25, [x26, #0x30]        
	sxth   w8, w19                  
	stp    w8, w8, [x26, #0x10]     
	mov    w9, #0x1                   ; =1
	stp    w9, w8, [x26, #0x18]     
	ldp    x15, x2, [sp, #0x120]    
	ldp    x0, x1, [x29, #-0x100]   
	ldr    x14, [sp, #0x78]         
	movi.16b v6, #0xa               
	mov    x16, #-0x2                 ; =-2
	movi.2d v7, #0xffffffffff000000 
	movi.16b v16, #0x2d             
	ldp    x17, x30, [x29, #-0xf0]  
	movi.16b v17, #0xd0             
	movi.16b v18, #0x30             
	ldp    q20, q19, [sp, #0x100]   
	adrp   x3, 166                  
	adrp   x4, 166                  
	adrp   x6, 166                  
	ldp    x7, x13, [sp, #0xc0]     
	ldp    q4, q21, [x29, #-0xe0]   
	ldp    q3, q22, [sp, #0xe0]     
	ldr    q5, [sp, #0xd0]          
	ldp    q2, q23, [sp, #0x50]     
	b      0x10000ff80                ; <+968> at ver20.rs
	mov    w0, #0x1                   ; =1
	str    x0, [sp, #0x48]          
	mov    x1, x22                  
	mov    x2, x25                  
	bl     0x1000b0020                ; symbol stub for: memcpy
	ldr    x1, [x26, #0x20]         
	cbnz   x1, 0x100010260            ; <+1704> [inlined] <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop + 8 at raw_vec.rs:404:18
	b      0x10001026c                ; <+1716> [inlined] onebrc_rust::ver20::AggrInfo::save_item_u64x2 + 108 at alloc.rs
	mov    w0, #0x1                   ; =1
	str    x0, [sp, #0x38]          
	mov    x1, x20                  
	mov    x2, x22                  
	bl     0x1000b0020                ; symbol stub for: memcpy
	ldr    x1, [x24, #0x20]         
	cbnz   x1, 0x100010334            ; <+1916> [inlined] <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop + 8 at raw_vec.rs:404:18
	b      0x100010340                ; <+1928> [inlined] onebrc_rust::ver20::AggrInfo::save_item_u64x2 + 108 at alloc.rs
	mov    w0, #0x1                   ; =1
	ldur   x1, [x29, #-0xb8]        
	str    x0, [sp, #0x30]          
	mov    x2, x20                  
	bl     0x1000b0020                ; symbol stub for: memcpy
	ldr    x1, [x24, #0x20]         
	cbnz   x1, 0x100010404            ; <+2124> [inlined] <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop + 8 at raw_vec.rs:404:18
	b      0x100010410                ; <+2136> [inlined] onebrc_rust::ver20::AggrInfo::save_item_u64x2 + 100 at alloc.rs
	mov    x8, x0                   
	mov    x0, x2                   
	add    x2, x8, x23              
	ldr    x1, [sp, #0x10]          
	mov    x3, x25                  
	mov    x4, x19                  
	mov    x19, x14                 
	mov    x23, x13                 
	mov    x25, x7                  
	mov    x26, x30                 
	bl     0x100010adc                ; onebrc_rust::ver20::AggrInfo::slow_save at ver20.rs:589
	ldp    q2, q23, [sp, #0x50]     
	ldp    q5, q3, [sp, #0xd0]      
	ldp    q22, q20, [sp, #0xf0]    
	ldp    q4, q21, [x29, #-0xe0]   
	mov    x30, x26                 
	mov    x7, x25                  
	mov    x13, x23                 
	ldp    x1, x17, [x29, #-0xf8]   
	adrp   x6, 166                  
	adrp   x4, 166                  
	adrp   x3, 166                  
	ldr    q19, [sp, #0x110]        
	movi.16b v18, #0x30             
	movi.16b v17, #0xd0             
	movi.16b v16, #0x2d             
	movi.2d v7, #0xffffffffff000000 
	mov    x16, #-0x2                 ; =-2
	movi.16b v6, #0xa               
	ldp    x15, x2, [sp, #0x120]    
	mov    x14, x19                 
	ldur   x0, [x29, #-0x100]       
	b      0x10000ff80                ; <+968> at ver20.rs
	mov    x0, x2                   
	ldr    x1, [sp, #0x10]          
	mov    x2, x22                  
	mov    x3, x25                  
	mov    x4, x19                  
	mov    x19, x14                 
	mov    x22, x13                 
	mov    x25, x7                  
	mov    x26, x30                 
	bl     0x100010adc                ; onebrc_rust::ver20::AggrInfo::slow_save at ver20.rs:589
	ldp    q5, q3, [sp, #0xd0]      
	ldp    q22, q20, [sp, #0xf0]    
	ldp    q4, q21, [x29, #-0xe0]   
	mov    x30, x26                 
	mov    x7, x25                  
	mov    x13, x22                 
	ldp    x1, x17, [x29, #-0xf8]   
	adrp   x6, 166                  
	adrp   x4, 166                  
	adrp   x3, 166                  
	ldr    q19, [sp, #0x110]        
	movi.16b v18, #0x30             
	movi.16b v17, #0xd0             
	movi.16b v16, #0x2d             
	movi.2d v7, #0xffffffffff000000 
	mov    x16, #-0x2                 ; =-2
	movi.16b v6, #0xa               
	ldp    x15, x2, [sp, #0x120]    
	mov    x14, x19                 
	ldur   x0, [x29, #-0x100]       
	b      0x100010020                ; <+1128> at ver20.rs
	mov    x0, x2                   
	ldr    x1, [sp, #0x10]          
	mov    x2, x20                  
	mov    x3, x22                  
	mov    x4, x19                  
	mov    x19, x14                 
	mov    x20, x13                 
	mov    x22, x7                  
	mov    x24, x30                 
	bl     0x100010adc                ; onebrc_rust::ver20::AggrInfo::slow_save at ver20.rs:589
	ldr    q2, [sp, #0xe0]          
	ldp    q4, q21, [x29, #-0xe0]   
	mov    x30, x24                 
	mov    x7, x22                  
	mov    x13, x20                 
	ldp    x1, x17, [x29, #-0xf8]   
	adrp   x6, 166                  
	adrp   x4, 166                  
	adrp   x3, 166                  
	ldp    q20, q19, [sp, #0x100]   
	movi.16b v18, #0x30             
	movi.16b v17, #0xd0             
	movi.16b v16, #0x2d             
	movi.2d v7, #0xffffffffff000000 
	mov    x16, #-0x2                 ; =-2
	movi.16b v6, #0xa               
	ldp    x15, x2, [sp, #0x120]    
	mov    x14, x19                 
	ldur   x0, [x29, #-0x100]       
	b      0x1000100d8                ; <+1312> at ver20.rs
	mov    x0, x2                   
	ldr    x1, [sp, #0x10]          
	ldur   x2, [x29, #-0xb8]        
	mov    x3, x20                  
	mov    x4, x19                  
	mov    x19, x14                 
	mov    x20, x13                 
	mov    x24, x7                  
	mov    x25, x30                 
	bl     0x100010adc                ; onebrc_rust::ver20::AggrInfo::slow_save at ver20.rs:589
	mov    x30, x25                 
	mov    x7, x24                  
	mov    x13, x20                 
	ldp    x1, x17, [x29, #-0xf8]   
	adrp   x6, 166                  
	adrp   x4, 166                  
	adrp   x3, 166                  
	ldp    q20, q19, [sp, #0x100]   
	movi.16b v18, #0x30             
	movi.16b v17, #0xd0             
	movi.16b v16, #0x2d             
	movi.2d v7, #0xffffffffff000000 
	mov    x16, #-0x2                 ; =-2
	movi.16b v6, #0xa               
	ldp    x15, x2, [sp, #0x120]    
	mov    x14, x19                 
	ldur   x0, [x29, #-0x100]       
	b      0x100010184                ; <+1484> at ver20.rs
	cbz    w21, 0x10000fc7c           ; <+196> at ver20.rs:365:20
	sub    x26, x17, #0x40          
	rbit   x8, x14                  
	clz    x8, x8                   
	lsl    x9, x16, x8              
	and    x9, x9, x14              
	orr    x8, x8, x26              
	rbit   x10, x22                 
	clz    x10, x10                 
	lsl    x11, x16, x10            
	and    x11, x11, x22            
	add    x10, x10, x17            
	cmp    x14, #0x0                
	csel   x14, x9, xzr, ne         
	csel   x22, x22, x11, ne        
	csel   x8, x8, x10, ne          
	rbit   x9, x24                  
	clz    x28, x9                  
	add    x27, x28, x17            
	ldr    q0, [x0, x23]            
	ldr    d1, [x15, x27]           
	cmeq.8b v2, v1, v10             
	ldr    d3, [x3, #0x7b8]         
	and.8b v2, v2, v3               
	addv.8b b2, v2                  
	fmov   w9, s2                   
	ushll.8h v1, v1, #0x0           
	ext.16b v1, v1, v1, #0x8        
	cmhi.4h v2, v1, v11             
	ldr    d3, [x4, #0x7c0]         
	ldr    d4, [x6, #0x7c8]         
	mla.4h v4, v1, v3               
	and.8b v1, v4, v2               
	ext.8b v2, v1, v1, #0x4         
	add.4h v1, v2, v1               
	dup.4h v2, v1[2]                
	add.4h v1, v2, v1               
	umov.h w10, v1[3]               
	tst    w9, #0x3f                
	cneg   w19, w10, ne             
	sub    x20, x8, x23             
	dup.16b v1, w20                 
	cmhi.16b v1, v1, v20            
	and.16b v0, v0, v1              
	fmov   x8, d0                   
	mov.d  x9, v0[1]                
	eor    x10, x8, x9              
	lsr    x11, x10, #40            
	lsr    x10, x10, #20            
	eor    w10, w10, w11            
	eor    w8, w8, w9               
	eor    w8, w10, w8              
	and    x5, x8, #0xfffff         
	add    x25, x2, x5, lsl #6      
	ldr    q1, [x25]                
	cmeq.2d v1, v1, v0              
	mvn.16b v1, v1                  
	xtn.2s v1, v1                   
	umaxp.2s v1, v1, v1             
	fmov   w8, s1                   
	tbnz   w8, #0x0, 0x100010890      ; <+3288> [inlined] alloc::vec::Vec<T,A>::len at mod.rs:2683:19
	ldp    w8, w9, [x25, #0x10]     
	sxth   w10, w19                 
	cmp    w8, w10                  
	csel   w8, w8, w10, lt          
	cmp    w9, w10                  
	csel   w9, w9, w10, gt          
	stp    w8, w9, [x25, #0x10]     
	ldr    d0, [x25, #0x18]         
	movi.2s v1, #0x1                
	mov.s  v1[1], w10               
	add.2s v0, v0, v1               
	str    d0, [x25, #0x18]         
	lsl    x8, x16, x28             
	and    x24, x8, x24             
	add    x23, x27, #0x1           
	subs   w21, w21, #0x1           
	b.ne   0x10001075c                ; <+2980> [inlined] core::num::<impl u64>::trailing_zeros at uint_macros.rs:162:20
	b      0x10000fc7c                ; <+196> at ver20.rs:365:20
	ldr    x8, [x25, #0x30]         
	cbnz   x8, 0x10001094c            ; <+3476> at main.rs
	str    q0, [x25]                
	tbnz   x20, #0x3f, 0x100010ab4    ; <+3836> at raw_vec.rs
	str    x14, [sp, #0x78]         
	cbz    x20, 0x1000108c8           ; <+3344> at alloc.rs
	adrp   x8, 216                  
	add    x8, x8, #0xd29             ; __rust_no_alloc_shim_is_unstable
	ldrb   wzr, [x8]                
	mov    x0, x20                  
	mov    w1, #0x1                   ; =1
	bl     0x1000260b4                ; __rust_alloc
	cbnz   x0, 0x1000108cc            ; <+3348> at alloc.rs
	b      0x100010ac8                ; <+3856> [inlined] alloc::raw_vec::RawVecInner<A>::with_capacity_in + 16 at raw_vec.rs
	mov    w0, #0x1                   ; =1
	ldur   x8, [x29, #-0x100]       
	add    x1, x8, x23              
	str    x0, [sp]                 
	mov    x2, x20                  
	bl     0x1000b0020                ; symbol stub for: memcpy
	ldr    x1, [x25, #0x20]         
	cbz    x1, 0x1000108f4            ; <+3388> [inlined] onebrc_rust::ver20::AggrInfo::save_item_u64x2 + 28 at alloc.rs
	ldr    x0, [x25, #0x28]         
	mov    w2, #0x1                   ; =1
	bl     0x1000260b8                ; __rust_dealloc
	ldr    x8, [sp]                 
	stp    x20, x8, [x25, #0x20]    
	str    x20, [x25, #0x30]        
	sxth   w8, w19                  
	stp    w8, w8, [x25, #0x10]     
	mov    w9, #0x1                   ; =1
	stp    w9, w8, [x25, #0x18]     
	ldp    x15, x2, [sp, #0x120]    
	ldur   x0, [x29, #-0x100]       
	ldr    x14, [sp, #0x78]         
	movi.16b v6, #0xa               
	mov    x16, #-0x2                 ; =-2
	movi.2d v7, #0xffffffffff000000 
	movi.16b v16, #0x2d             
	ldur   x17, [x29, #-0xf0]       
	movi.16b v17, #0xd0             
	movi.16b v18, #0x30             
	ldp    q20, q19, [sp, #0x100]   
	adrp   x3, 166                  
	adrp   x4, 166                  
	adrp   x6, 166                  
	b      0x100010878                ; <+3264> at ver20.rs
	mov    x8, x0                   
	mov    x0, x2                   
	add    x2, x8, x23              
	ldr    x1, [sp, #0x10]          
	mov    x3, x20                  
	mov    x4, x19                  
	mov    x19, x14                 
	bl     0x100010adc                ; onebrc_rust::ver20::AggrInfo::slow_save at ver20.rs:589
	adrp   x6, 166                  
	adrp   x4, 166                  
	adrp   x3, 166                  
	ldp    q20, q19, [sp, #0x100]   
	movi.16b v18, #0x30             
	movi.16b v17, #0xd0             
	ldur   x17, [x29, #-0xf0]       
	movi.16b v16, #0x2d             
	movi.2d v7, #0xffffffffff000000 
	mov    x16, #-0x2                 ; =-2
	movi.16b v6, #0xa               
	ldp    x15, x2, [sp, #0x120]    
	mov    x14, x19                 
	ldur   x0, [x29, #-0x100]       
	b      0x100010878                ; <+3264> at ver20.rs
	adrp   x8, 208                  
	add    x8, x8, #0xb58             ; onebrc_rust::ver5::ver5 + 18446744073709531748 at raw_vec.rs
	mov    w9, #0x1                   ; =1
	stp    x8, x9, [x29, #-0xa8]    
	mov    w8, #0x8                   ; =8
	stp    xzr, xzr, [x29, #-0x90]  
	stur   x8, [x29, #-0x98]        
	sub    x0, x29, #0xa8           
	bl     0x10008b884                ; std::io::stdio::_print at stdio.rs:1232
	add    sp, sp, #0x1c0           
	ldp    x29, x30, [sp, #0x70]    
	ldp    x20, x19, [sp, #0x60]    
	ldp    x22, x21, [sp, #0x50]    
	ldp    x24, x23, [sp, #0x40]    
	ldp    x26, x25, [sp, #0x30]    
	ldp    x28, x27, [sp, #0x20]    
	ldp    d9, d8, [sp, #0x10]      
	ldp    d11, d10, [sp], #0x80    
	ret                             
	str    xzr, [sp, #0x8]          
	adrp   x2, 208                  
	add    x2, x2, #0xb20             ; onebrc_rust::ver5::ver5 + 18446744073709531692 at mod.rs:1757:9
	mov    x0, xzr                  
	ldr    x1, [sp, #0x48]          
	bl     0x1000af8bc                ; alloc::raw_vec::handle_error at raw_vec.rs:789
	str    xzr, [sp, #0x8]          
	adrp   x2, 208                  
	add    x2, x2, #0xb20             ; onebrc_rust::ver5::ver5 + 18446744073709531692 at mod.rs:1757:9
	mov    x0, xzr                  
	ldr    x1, [sp, #0x40]          
	bl     0x1000af8bc                ; alloc::raw_vec::handle_error at raw_vec.rs:789
	str    xzr, [sp, #0x8]          
	adrp   x2, 208                  
	add    x2, x2, #0xb20             ; onebrc_rust::ver5::ver5 + 18446744073709531692 at mod.rs:1757:9
	mov    x0, xzr                  
	ldr    x1, [sp, #0x30]          
	bl     0x1000af8bc                ; alloc::raw_vec::handle_error at raw_vec.rs:789
	str    xzr, [sp, #0x8]          
	adrp   x2, 208                  
	add    x2, x2, #0xb20             ; onebrc_rust::ver5::ver5 + 18446744073709531692 at mod.rs:1757:9
	mov    x0, xzr                  
	ldr    x1, [sp, #0x38]          
	bl     0x1000af8bc                ; alloc::raw_vec::handle_error at raw_vec.rs:789
	str    x22, [sp, #0x38]         
	adrp   x2, 208                  
	add    x2, x2, #0xb20             ; onebrc_rust::ver5::ver5 + 18446744073709531692 at mod.rs:1757:9
	ldr    x0, [sp, #0x8]           
	mov    x1, x22                  
	bl     0x1000af8bc                ; alloc::raw_vec::handle_error at raw_vec.rs:789
	str    x25, [sp, #0x40]         
	adrp   x2, 208                  
	add    x2, x2, #0xb20             ; onebrc_rust::ver5::ver5 + 18446744073709531692 at mod.rs:1757:9
	ldr    x0, [sp, #0x8]           
	mov    x1, x25                  
	bl     0x1000af8bc                ; alloc::raw_vec::handle_error at raw_vec.rs:789
	str    x20, [sp, #0x30]         
	adrp   x2, 208                  
	add    x2, x2, #0xb20             ; onebrc_rust::ver5::ver5 + 18446744073709531692 at mod.rs:1757:9
	ldr    x0, [sp, #0x8]           
	mov    x1, x20                  
	bl     0x1000af8bc                ; alloc::raw_vec::handle_error at raw_vec.rs:789
	str    x25, [sp, #0x48]         
	adrp   x2, 208                  
	add    x2, x2, #0xb20             ; onebrc_rust::ver5::ver5 + 18446744073709531692 at mod.rs:1757:9
	ldr    x0, [sp, #0x8]           
	mov    x1, x25                  
	bl     0x1000af8bc                ; alloc::raw_vec::handle_error at raw_vec.rs:789
	str    xzr, [sp, #0x8]          
	adrp   x2, 208                  
	add    x2, x2, #0xb20             ; onebrc_rust::ver5::ver5 + 18446744073709531692 at mod.rs:1757:9
	ldp    x1, x0, [sp]             
	bl     0x1000af8bc                ; alloc::raw_vec::handle_error at raw_vec.rs:789
	str    x20, [sp]                
	adrp   x2, 208                  
	add    x2, x2, #0xb20             ; onebrc_rust::ver5::ver5 + 18446744073709531692 at mod.rs:1757:9
	ldp    x1, x0, [sp]             
	bl     0x1000af8bc                ; alloc::raw_vec::handle_error at raw_vec.rs:789



```