pub struct Solution;

impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let area_a = i64::from(ax2 - ax1) * i64::from(ay2 - ay1);
        let area_b = i64::from(bx2 - bx1) * i64::from(by2 - by1);
        let ix1 = ax1.max(bx1);
        let iy1 = ay1.max(by1);
        let ix2 = ax2.min(bx2);
        let iy2 = ay2.min(by2);
        let overlap = if ix1 < ix2 && iy1 < iy2 {
            i64::from(ix2 - ix1) * i64::from(iy2 - iy1)
        } else {
            0
        };
        (area_a + area_b - overlap) as i32
    }
}
