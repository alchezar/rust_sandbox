// IKinder

/// # Coddy Users
/// ## Easy
/// Each week, Coddy has more users than the previous week. The
/// recursion function that represents Coddy's weekly users goes
/// like this:\
///  If the previous week had `x` users, and this week had `y`
/// users, Coddy is going to have `x + 3y` users in the next week.\
/// Coddy had **0** users in the first week and **1** users in the
/// second week.
fn get_users(n: i32) -> i32 {
    if n <= 2 {
        return n - 1;
    }
    get_users(n - 2) + 3 * get_users(n - 1)
}

/// # Coddy Users Longer
/// ## Medium
/// Each week, Coddy has more users than the previous week. The
/// recursion function that represents Coddy's weekly users goes
/// like this: \
/// If the previous week had `x` users, and this week had `y` users,
/// Coddy is going to have `x + 3y` users in the next week.\
/// Coddy had **0** users in the first week and **1** user in the
/// second week.
/// The amount of **active users** Coddy has in the `n`'th week `modulo 109 + 7`.
fn get_users_longer(n: i32) -> i32 {
    const MOD: u32 = 1_000_000_007;
    let mut prev = 0;
    let mut curr = 1;
    let mut next = 0;

    for _ in 2..n {
        next = (prev + 3 * curr) % MOD;
        prev = curr;
        curr = next;
    }
    next as i32
}

/// # Coddy Active Users
/// ## Hard
/// Not all of Coddy's registered users are active.
/// The recursion function that represents Coddy's weekly active
/// users is a calculation which involves both previous registered
/// users (from last challenge) and also previous active users:\
/// If previous week had **x1** users and **x2** active users, and this
/// week had **y1** users and **y2** active users, Coddy is going to have
/// **x1+x2+y1+y2** active users next week.\
/// Coddy had `0` active users in the first week and `1` active user
/// in the second week.
/// The amount of **active users** Coddy has in the `n`'th week
/// `modulo 109 + 7`.
fn get_active_users(n: i32) -> i32 {
    const MOD: u32 = 1_000_000_007;
    let mut all_prev = 0;
    let mut all_curr = 1;
    let mut _all_next = 0;

    let mut active_prev = 0;
    let mut active_curr = 1;
    let mut active_next = 1;

    for _ in 3..=n {
        _all_next = (all_prev + 3 * all_curr) % MOD;
        active_next = (all_prev + active_prev + all_curr + active_curr) % MOD;

        all_prev = all_curr;
        all_curr = _all_next;

        active_prev = active_curr;
        active_curr = active_next;
    }
    active_next as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_users_1() {
        let data = (3, 3);
        assert_eq!(get_users(data.0), data.1);
    }
    #[test]
    fn get_users_2() {
        let data = (5, 33);
        assert_eq!(get_users(data.0), data.1);
    }
    #[test]
    fn get_users_3() {
        let data = (10, 12970);
        assert_eq!(get_users(data.0), data.1);
    }
    #[test]
    fn get_users_4() {
        let data = (12, 141481);
        assert_eq!(get_users(data.0), data.1);
    }
    #[test]
    fn get_users_5() {
        let data = (19, 606_529_080);
        assert_eq!(get_users(data.0), data.1);
    }

    #[test]
    fn get_users_longer_1() {
        let data = (5, 33);
        assert_eq!(get_users_longer(data.0), data.1);
    }
    #[test]
    fn get_users_longer_2() {
        let data = (23, 171_862_773);
        assert_eq!(get_users_longer(data.0), data.1);
    }
    #[test]
    fn get_users_longer_3() {
        let data = (35, 741_606_231);
        assert_eq!(get_users_longer(data.0), data.1);
    }
    #[test]
    fn get_users_longer_4() {
        let data = (87, 250_490_968);
        assert_eq!(get_users_longer(data.0), data.1);
    }

    #[test]
    fn get_active_users_1() {
        let data = (2, 1);
        assert_eq!(get_active_users(data.0), data.1);
    }
    #[test]
    fn get_active_users_2() {
        let data = (5, 22);
        assert_eq!(get_active_users(data.0), data.1);
    }
    #[test]
    fn get_active_users_3() {
        let data = (4, 7);
        assert_eq!(get_active_users(data.0), data.1);
    }
    #[test]
    fn get_active_users_4() {
        let data = (12, 92_176);
        assert_eq!(get_active_users(data.0), data.1);
    }
    #[test]
    fn get_active_users_5() {
        let data = (32, 196_303_248);
        assert_eq!(get_active_users(data.0), data.1);
    }
    #[test]
    fn get_active_users_6() {
        let data = (89, 637_410_251);
        assert_eq!(get_active_users(data.0), data.1);
    }
    #[test]
    fn get_active_users_7() {
        let data = (99, 877_827_748);
        assert_eq!(get_active_users(data.0), data.1);
    }
    #[test]
    fn get_active_users_8() {
        let data = (65, 103_708_346);
        assert_eq!(get_active_users(data.0), data.1);
    }
}
