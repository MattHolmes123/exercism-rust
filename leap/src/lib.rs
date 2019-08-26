pub fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0) && !(year % 100 == 0 && year % 400 != 0)
}

//pub fn is_leap_year_v1(year: u64) -> bool {
//    let is_leap = if year % 4 == 0 {
//        if year % 100 == 0 && year % 400 != 0 {
//            false
//        } else {
//            true
//        }
//    } else {
//        false
//    };
//
//    is_leap
//}
//
//pub fn is_leap_year_v2(year: u64) -> bool {
//    let is_leap = if year % 4 == 0 {
//        !(year % 100 == 0 && year % 400 != 0)
//    } else {
//        false
//    };
//
//    is_leap
//}
//
//pub fn is_leap_year_v3(year: u64) -> bool {
//    if year % 4 == 0 {
//        !(year % 100 == 0 && year % 400 != 0)
//    } else {
//        false
//    }
//}
