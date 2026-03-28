@u32_u0 = fixed<-32, dn>;
@u64_u0 = fixed<-64, dn>;
@u128_u0 = fixed<-128, dn>;
@u32_u2 = fixed<-30, dn>;
@u64_u2 = fixed<-62, dn>;
@u128_u2 = fixed<-128, dn>;

exact = sqrt(m_u2);

# r0_u0;
s0_u2 = m_u2;

# Unused for f64
r1_u0 = r0_u0;
s1_u2 = s0_u2;

r2_u0 = r1_u0;
s2_u2 = s1_u2;

gs2_i0_u_u0 = r2_u0;
gs2_i0_s_u2 = u32_u2((u64_u2(s2_u2) * u64_u0(gs2_i0_u_u0)) / 0x100000000);
gs2_i0_d_u2 = u32_u2((u64_u2(gs2_i0_s_u2) * u64_u0(r2_u0)) / 0x100000000);
gs2_i1_u_u0 u32_u2= 3 - gs2_i0_d_u2;
gs2_i1_r_u0 = u32_u2((u64_u2(r2_u0) * u64_u0(gs2_i1_u_u0)) / 0x100000000);

gs2_i1_s_u2 = u32_u2((u64_u2(gs2_i0_s_u2) * u64_u0(gs2_i1_u_u0)) / 0x100000000);
gs2_i1_d_u2 = u32_u2((u64_u2(gs2_i1_s_u2) * u64_u0(gs2_i1_r_u0)) / 0x100000000);
gs2_i2_u_u0 u32_u2= 3 - gs2_i1_d_u2;
gs2_i2_r_u0 = u32_u2((u64_u2(gs2_i1_r_u0) * u64_u0(gs2_i2_u_u0)) / 0x100000000);

#gs2_i2_s_u2 = u32_u2((u64_u2(gs2_i1_s_u2) * u64_u0(gs2_i2_u_u0)) / 0x100000000);
#gs2_i2_d_u2 = u32_u2((u64_u2(gs2_i2_s_u2) * u64_u0(gs2_i2_r_u0)) / 0x100000000);
#gs2_i3_u_u0 u32_u2= 3 - gs2_i2_d_u2;
#gs2_i3_r_u0 = u32_u2((u64_u2(gs2_i2_r_u0) * u64_u0(gs2_i3_u_u0)) / 0x100000000);

r_u0 = gs2_i2_r_u0;
s_u2 = gs2_i1_s_u2;

{
  @FIX(m_u2,-30) /\ m_u2 in [1,4]
  /\ @FIX(r0_u0,-16)
  /\ r0_u0 - exact in [-1b-16,1b-16]
  -> r_u0 - exact in ?
}



# r0 * (2 - d * r0) - R -> (r0 - R) * (r0 - R) * -d   { d <> 0 };
# r1 * (2 - d * r1) - R -> (r1 - R) * (r1 - R) * -d   { d <> 0 };


# R = 1 / d;

# r1 fixed<-14,dn>= r0 * (2 - fixed<-16,dn>(d) * r0);
# r2 fixed<-30,dn>= r1 * (2 - d * r1);

# { @FIX(d,-24) /\ d in [0.5,1] /\
#   @FIX(r0,-8) /\ r0 - R in [-1b-8,1b-8] ->
#   r2 - R in ?
# }

# r0 * (2 - d * r0) - R -> (r0 - R) * (r0 - R) * -d   { d <> 0 };
# r1 * (2 - d * r1) - R -> (r1 - R) * (r1 - R) * -d   { d <> 0 };



# @rnd = float<ieee_64,ne>;
# x = rnd(xx); # x is an f64 version of any real xx
# y = sqrt(x);
# err = y - sqrt(x);


# { x in [0,1] -> err in ? }

# # # some notations
# @rnd = float<ieee_32, ne>;


# x = rnd(xxuij);                           # x is a floating-point number
# y rnd= x * (1 - x);                    # equivalent to y = rnd(x * rnd(1 - x))
# z = x * (1 - x);                       # the value we want to approximate

# # the logical proposition
# { x in [0,1] -> y in [0,0.25] /\ y - z in [-3b-27,3b-27] }

# # hints
# z -> 0.25 - (x - 0.5) * (x - 0.5);     # x * (1 - x) == 1/4 - (x - 1/2)^2



# y = x * (10 - x);
# { x in [4,6] -> y in [24.5, 24.5] }
# # { x in [4,6] -> y in ? }
# x * (10 - x) -> 24.5;

# @floor = int<dn>;
# @ceil = int<up>;

# Recip08 = (0x1p71 / y);
# recip16 = floor(((recip08 * (0x1p24 - (recip08 * floor((y / 0x1p48))))) / 0x1p14));
# Recip16 = (0x1p80 / y);
# recip32 = floor(((recip16 * (0x1p48 - (recip16 * floor((y / 0x1p33))))) / 0x1p32));
# Recip32 = (0x1p95 / y);
# recip64 = floor(((recip32 * (0x1p64 - ceil((((recip32 * y) + 1) / 0x1p32)))) / 0x1p32));
# Recip64 = (0x1p126 / y);
# xh = floor((x / 0x1p32));
# xl = ((x / 0x1p32) - xh);
# rh = floor((recip64 / 0x1p32));
# rl = ((recip64 / 0x1p32) - rh);
# fullquot = ((x * recip64) / 0x1p64);
# mul_error = (((-(xl) * rl) - ((xh * rl) - floor((xh * rl)))) - ((xl * rh) - floor((xl * rh))));
# quot = (((xh * rh) + floor((xh * rl))) + floor((xl * rh)));
# Quot = ((x * Recip64) / 0x1p64);
# nquot = (quot * 0x1p1);
# NQuot = (Quot * 0x1p1);

# { y in [0x8000000000000000,0x80FFFFFFFFFFFFFF] /\
#   recip08 = 0xFF /\
#   x in [0x8000000000000000, 0xFFFFFFFFFFFFFFFF] /\ x/y <= 1 ->
#   nquot - NQuot in [-0.5000,0] }

# (recip08 * (0x1p24 - recip08 * (y/0x1p48))) / 0x1p14 - Recip16 ->
#   (-(recip08 - Recip08) * (recip08 - Recip08) * y / 0x1p62)
#   { y <> 0 };

# (recip16 * (0x1p48 - recip16 * (y/0x1p33))) / 0x1p32 - Recip32 ->
#   (-(recip16 - Recip16) * (recip16 - Recip16) * y / 0x1p65)
#   { y <> 0 };

# (recip32 * ((0x1p64 - (recip32 * y + 1) / 0x1p32)) / 0x1p32) - Recip64 ->
#   (-(recip32 - Recip32) * (recip32 - Recip32) * y / 0x1p64) - recip32/0x1p64
#   { y <> 0 };

# quot - Quot -> fullquot - Quot + mul_error { y <> 0 };
