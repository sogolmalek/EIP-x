; ModuleID = 'probe4.85613129bdceffa7-cgu.0'
source_filename = "probe4.85613129bdceffa7-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

@alloc_15bbdde4b3731b612a9fc16a3484a41b = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/core/src/num/mod.rs" }>, align 1
@alloc_2c589cfdc6391a59dee52bbb016e4dad = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_15bbdde4b3731b612a9fc16a3484a41b, [16 x i8] c"K\00\00\00\00\00\00\00@\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17ha4489cd29965d551E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hb082cadb2b50aea4E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h1042ba0fc6819e49E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_2c589cfdc6391a59dee52bbb016e4dad) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hb082cadb2b50aea4E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h1042ba0fc6819e49E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 8, !"PIC Level", i32 2}
