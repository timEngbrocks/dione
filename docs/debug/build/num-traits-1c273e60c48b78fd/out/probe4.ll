; ModuleID = 'probe4.ccbd5c6a8a6bc402-cgu.0'
source_filename = "probe4.ccbd5c6a8a6bc402-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-gnu"

@alloc_f1424eaf621bbd6837b07d6a38498610 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/1c05d50c8403c56d9a8b6fb871f15aaa26fb5d07\\library\\core\\src\\num\\mod.rs" }>, align 1
@alloc_e68d458b66951b29f767672eaef2c521 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_f1424eaf621bbd6837b07d6a38498610, [16 x i8] c"K\00\00\00\00\00\00\00v\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17hdc05faa8e883e116E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hc03b1014f0dfd4cbE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17he04f3fee4cd4b0bbE(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_e68d458b66951b29f767672eaef2c521) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hc03b1014f0dfd4cbE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17he04f3fee4cd4b0bbE(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.75.0-nightly (1c05d50c8 2023-10-21)"}
