Require Import Gappa.Gappa_library.
Section Generated_by_Gappa.
Variable _x : R.
Definition r4 := Float1 (10).
Definition r3 := (r4 - _x)%R.
Definition _y := (_x * r3)%R.
Definition r5 := float10R (Float10 (245) (-1)).
Hypothesis a1 : _y = r5.
Lemma b1 : _y = r5.
 apply a1.
Qed.
Definition f1 := Float2 (1) (2).
Definition f2 := Float2 (3) (1).
Definition i1 := makepairF f1 f2.
Definition p1 := BND _x i1. (* BND(x, [4, 6]) *)
Definition f3 := Float2 (49) (-1).
Definition i2 := makepairF f3 f3.
Definition p2 := BND _y i2. (* BND(y, [24.5, 24.5]) *)
Definition s2 := not p2.
Definition s1 := p1 /\ s2.
Lemma l2 : s1 -> s2.
Proof.
 intros h0.
 assert (h1 := h0).
 exact (proj2 h1).
Qed.
Definition p3 := _y = r5. (* EQL(y, 245e-1) *)
Lemma t1 : p3.
Proof.
 refine (b1) ; finalize.
Qed.
Lemma l4 : s1 -> p3 (* EQL(y, 245e-1) *).
Proof.
 intros h0.
 apply t1.
Qed.
Definition p4 := BND r5 i2. (* BND(245e-1, [24.5, 24.5]) *)
Lemma t2 : p4.
Proof.
 refine (constant10 _ i2 _) ; finalize.
Qed.
Lemma l5 : s1 -> p4 (* BND(245e-1, [24.5, 24.5]) *).
Proof.
 intros h0.
 apply t2.
Qed.
Lemma t3 : p3 -> p4 -> p2.
Proof.
 intros h0 h1.
 refine (bnd_rewrite _y r5 i2 h0 h1) ; finalize.
Qed.
Lemma l3 : s1 -> p2 (* BND(y, [24.5, 24.5]) *).
Proof.
 intros h0.
 assert (h1 := l4 h0).
 assert (h2 := l5 h0).
 apply t3. exact h1. exact h2.
Qed.
Lemma l1 : s1 -> False.
Proof.
 intros h0.
 assert (h1 := l2 h0).
 assert (h2 := l3 h0).
 refine (simplify (Tatom false (Abnd 0%nat i2)) Tfalse (Abnd 0%nat i2) (List.cons _y List.nil) h2 h1 _) ; finalize.
Qed.
End Generated_by_Gappa.
