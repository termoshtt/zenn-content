---
title: "C API で確保されたメモリを Fortran の配列にする"
emoji: "🐕"
type: "tech"
topics: ["Fortran"]
published: true
---

C の API でメモリが確保されて、アドレスだけ帰ってくる関数があるとします。

```c
#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>

int* myalloc() {
  int* ptr = (int*) malloc(4 * sizeof(int));
  ptr[0] = 1;
  ptr[1] = 2;
  ptr[2] = 3;
  ptr[3] = 4;
  return ptr;
}
```

例えばこのような `myalloc` で確保されたメモリを Fortran から配列として使うには次の様にします

```fortran
program f1
  use iso_c_binding
  interface
    function myalloc() bind(c)
      import c_ptr
      type(c_ptr) myalloc
    end function
  end interface

  type(c_ptr) addr
  integer, pointer :: array(:)

  addr = myalloc()
  call c_f_pointer(addr, array, [4])

  print *, array ! [1, 2, 3, 4]
end
```

`c_f_pointer` は `type(c_ptr)` のアドレス値を Fortran におけるポインタに変換するための関数です。第三引数に配列の shape を取ることが可能です。

参考
-----
- [(gcc) 9.53 C_F_POINTER — Convert C into Fortran pointer](https://gcc.gnu.org/onlinedocs/gfortran/C_005fF_005fPOINTER.html)
- [(nag) Fortran Tip集: C との相互利用可能性の例](https://www.nag-j.co.jp/fortran/tips/tips_InteroperabilityWithC.html#CfRaPtr)
