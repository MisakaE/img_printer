use std::fmt::Display;
pub fn s_to_u32<T:Display>(st:T)->u32{
    let mut st = st.to_string();
    let mut cnt:u32= 1;
    let mut sum:u32 = 0;
    while let Some(i) = st.pop() {
        sum += (i as u32 - '0' as u32)*cnt;
        cnt *= 10;
    }
    sum
}