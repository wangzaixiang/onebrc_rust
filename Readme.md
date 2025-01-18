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
3. 01-14
   - 调试汇编
   - `pos1:[usize;64], pos2: [usize;64]` 保存最多 64 个位置（可管理最大 512 bytes）
   - `index1，index2` 下一个位置
   - `end1，end2` 结束位置
   - 每次最多读取 256 字节，并更新 pos1, pos2, end1, end2
   - 当 (end2 - index2 + 64) % 64 < 4 时，表示不足 4 行，读取下一个 256 字节, 否则每次处理4行
- 其他
  - 检查 u128 的 popcnt

01-16:
1.  save_item 2.25s
    - 调整 Item 结构，改为读写一个 u128，减少读写
    - why p0 ^ p1 ... 1.23s
2.  计算 str_to_hash 向量化， len -> (len_a, len_b) 位移 key_a << n >> n。0.9s
    - 长度计算耗时太多，调整为 一组 SIMD 计算？
3. load_current_64 目前约20G/s, 是否可以更高? 13G / 0.6 = 21G/s.  1.2s
   - from_slice 增加2个？最多 256 字节？需要2个 v1: ARM 对 u128 的支持是通过2个 u64 来完成的，性能不佳？
   - pos1, pos2 
4. process_line 减少计算 - 144ms
5. parse_value_3 ～460ms 对-号采用向量操作

01-17 ver20
1. 重新整理处理流程，最大化减少内存访问
   - scan_loop read 64 bytes        ~548ms
   - for each line: read key, value(2)   ~410ms + 410ms
   - for each line, access aggr     ~ 1380  aggr 4 个字段合并，hash 读取合并
2. 同时处理最多4个行，提高指令并行度，尽量复用寄存器
    - 读内存操作尽量提前
    - 循环内避免对 stack 的内存访问，全部转寄存器，需要查看生成的asm
3. 位操作优化
4. key, value parse 再次评估是否合并向量化

01-18:
1. truncate key 从 normal 版本优化到 simd 版本， 从 797 -> 235ms。