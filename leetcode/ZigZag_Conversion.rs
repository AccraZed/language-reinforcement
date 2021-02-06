pub fn convert(s: String, num_rows: i32) -> String {
    let s_len = s.len();
    let s = &s[..];
    let mut index: usize = 0;
    let mut flip: bool = true;
    let mut ret: String = String::new();

    if num_rows == 1 {
        return s.to_string();
    }

    for row in 0..num_rows {
        index = row as usize;
        flip = true;

        while (index < s_len) {
            ret.push(s.chars().nth(index).unwrap());

            if flip {
                index += 2 * (num_rows - row - 1) as usize;
                flip = false;
            } else {
                index += 2 * row as usize;

                if row == 0 || row == num_rows - 1 {
                    ret.pop();
                }

                flip = true;
            }
        }
    }

    ret
}