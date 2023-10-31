use core::{cmp::Ordering, ops::AddAssign};

// static BIG_STRIDE: u8 = 255;
static HALF_BIG_STRIDE: u8 = 127;

#[derive(Clone, Copy)]
pub struct Stride(pub u8);

// impl Stride {
//     pub fn test_stride_ord() {
//         assert_eq!(Stride(125) < Stride(255), false);
//         assert_eq!(Stride(126) < Stride(255), false);
//         assert_eq!(Stride(127) < Stride(255), false);
//         assert_eq!(Stride(128) < Stride(255), true);
//         assert_eq!(Stride(129) < Stride(255), true);

//         assert_eq!(Stride(116) > Stride(245), true);
//         assert_eq!(Stride(117) > Stride(245), true);
//         assert_eq!(Stride(118) > Stride(245), false);
//     }
// }

impl PartialOrd for Stride {
    //STRIDE_MAX – STRIDE_MIN <= BigStride / 2
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 >= other.0 && self.0 - other.0 <= HALF_BIG_STRIDE
            || self.0 < other.0 && other.0 - self.0 > HALF_BIG_STRIDE{
            Some(Ordering::Greater)
        }else{
            Some(Ordering::Less)
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        let _ = other;
        false
    }
}

impl AddAssign<u8> for Stride{
    fn add_assign(&mut self, other: u8) {
        self.0 += other;
    }
}
















