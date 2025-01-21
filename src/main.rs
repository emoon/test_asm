use core::arch::x86_64::*;

#[inline(never)]
fn mulh(a: __m128i, b: __m128i) -> __m128i {
    unsafe { _mm_mulhrs_epi16(a, b) }
}

fn main() {
    unsafe {
        let a = _mm_set1_epi16(0x1234);
        let b = _mm_set1_epi16(0x5678);
        core::hint::black_box(&a);
        let c = mulh(a, b);
        println!("{:?}", c);
    }
}
