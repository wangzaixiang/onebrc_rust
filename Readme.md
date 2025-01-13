1. mmap
2. SIMD
3. &str -> i64
4. read measurements
   - value has -2.0
   - name max length is 26
   - distinct name count is 413
   - using u128 as name
5. parse numbers

Next Steps:
1. batch
   - 1 batch process 4 lines
   - 4 * u128 - for name
   - 4 value 
     -  4 * ( u8x8 , clear top bits ) -> u8x32  ( u8x8x4)
     -  u8x8x4 eq '-' -> Mask<i32x4> -> i32x4
     -  mask = (u8x8x4 >= '0' && u8x8x4 <= '9') select (u8x8x4 - '0')
     -  select (mask, uxx8x4 - '0', 0) * scale
     -  shuffle B4, B3, B1
     -  (B4 + B3 + B1) * Sign
     -  最后结果得出 4 value
2. Hash
   - 413 station
   - `[ u128: key + u32: idx1; 1M ]`, index = hash(u128) 虽然有 20M，但是是极度稀疏的，最终在 cache 中只占用 413 * 128 = 50K
   - `stations[413]` each (i16, i16, u32, i32)  12*413=5K 很小
   - 基于 u128 key 计算一个 hashCode, 使得几乎不会冲突： 0..20 ^ 20..40 ^ 40..60 ^ 60..80 ^ 80..100 ^ 100..120 ^ 120..128
   将 L1 Cache 发挥到淋漓尽致。