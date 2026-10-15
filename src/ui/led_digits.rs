/// The text of a classic 3-digit LED counter: zero-padded, and clamped to what fits
/// (999 at most, and -99 at least, because the minus sign takes one of the three digits).
pub fn led_digits(value: isize) -> String {
    if value < 0 {
        format!("-{:02}", value.unsigned_abs().min(99))
    } else {
        format!("{:03}", value.min(999))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_three_characters_zero_padded_and_clamped() {
        let cases = [
            (0, "000"),
            (7, "007"),
            (10, "010"),
            (999, "999"),
            (1234, "999"),
            (-1, "-01"),
            (-42, "-42"),
            (-150, "-99"),
            (isize::MIN, "-99"), // `abs()` would overflow here; `unsigned_abs()` can't
        ];
        for (value, expected) in cases {
            assert_eq!(led_digits(value), expected, "for {value}");
        }
    }
}
