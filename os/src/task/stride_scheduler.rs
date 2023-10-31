use core::{cmp::Ordering, ops::AddAssign};
use alloc::sync::Arc;
use super::TaskControlBlock;

pub static BIG_STRIDE: u8 = 255;
pub static HALF_BIG_STRIDE: u8 = 127;

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
    //而且需要大小翻转，以每次都pop最小值
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 >= other.0 && self.0 - other.0 <= HALF_BIG_STRIDE
            || self.0 < other.0 && other.0 - self.0 > HALF_BIG_STRIDE{
            Some(Ordering::Less)
        }else{
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        let _ = other;
        false
    }
}

impl Eq for Stride{}

impl Ord for Stride{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl AddAssign<u8> for Stride{
    fn add_assign(&mut self, other: u8) {
        self.0 += other;
    }
}

impl From<Stride> for u8{
    fn from(value: Stride) -> Self {
        value.0
    }
}

pub struct StrideWrapTcb(pub Stride, pub Arc<TaskControlBlock>);

// impl StrideWrapTcb {
//     pub fn wrap_tcb(task: Arc<TaskControlBlock>) -> StrideWrapTcb{
//         StrideWrapTcb(task.get_stride(), task.clone())
//     }
// }

impl PartialOrd for StrideWrapTcb{
    //要进行大小翻转让heap每次弹出最小值
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = self.0.partial_cmp(&other.0);
        match res {
            Some(Ordering::Less) => Some(Ordering::Greater),
            _ => Some(Ordering::Less)
        }
    }
}

impl PartialEq for StrideWrapTcb{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}









