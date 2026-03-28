@u32_u0 = fixed<-32, dn>;
@u64_u0 = fixed<-64, dn>;
@u128_u0 = fixed<-128, dn>;
@u32_u2 = fixed<-30, dn>;
@u64_u2 = fixed<-62, dn>;
@u128_u2 = fixed<-128, dn>;

exact = sqrt(m);

s0 = u32_u2((u64_u2(m) * u64_u0(r0)) / 0x100000000);
d0 = u32_u2((u64_u2(s0) * u64_u0(r0)) / 0x100000000);
u0 = 3 - d0;
r1 = u32_u2(u32_u2((u64_u2(r0) * u64_u0(u0)) / 0x100000000) * 2);

s1 = u32_u2((u64_u2(s0) * u64_u0(u0)) / 0x100000000) * 2;
d1 = u32_u2((u64_u2(s1) * u64_u0(r1)) / 0x100000000);
u1 = 3 - d1;

s2 = u32_u2((u64_u2(s1) * u64_u0(u1)) / 0x100000000);

{
  # 24 mantissa bits
  @FIX(m,-22) /\ m in [1, 4]
  /\ @FIX(r0,-16)
  /\ 1 - (r0 / exact) in [-1b-16,1b-16]
  -> r0 * exact in ?
  /\ |r0 * exact - 1| in ?
  /\ |r1 * exact - 1| in ?
  /\ |s0 / exact - 1| in ?
  /\ |s1 / exact - 1| in ?
  /\ |s2 / exact - 1| in ?
}

#r0 * (3 - (s0 * r0)) - exact -> 3*r0
