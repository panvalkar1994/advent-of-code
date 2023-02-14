// --- Day 5: Doesn't He Have Intern-Elves For This? ---
// Santa needs help figuring out which strings in his text file are naughty or nice.

// A nice string is one with all of the following properties:

// It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
// It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
// It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
// For example:

// ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
// aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
// jchzalrnumimnmhp is naughty because it has no double letter.
// haegwjzuvuyypxyu is naughty because it contains the string xy.
// dvszwmarrgswjxmb is naughty because it contains only one vowel.
// How many strings are nice?

pub fn get_nice_string_count(input:String) -> i32{
    let mut count = 0;
    let disallowed = vec!["ab", "cd", "pq","xy"];
    for line in input.lines() {
        if !disallowed_str_present(line, &disallowed) && is_repeated_at_least_once(line) && is_more_vowels(line, 3) {
            count += 1;
        }
    }
    count
}


fn is_repeated_at_least_once(s:&str) -> bool {
    let n = s.len();
    let cs = s.as_bytes();
    for i in 1..n{
        match cs[i] == cs[i-1] {
            true => return  true,
            false => continue,
        }
    }
    false
}

fn is_more_vowels(s:&str, count:i32) -> bool {
    let vowels = "aeiou".as_bytes();
    let mut v = Vec::new();
    s.bytes().for_each(|x| match vowels.contains(&x) {
        true=>{
            v.push(x);
        },
        false=>{}
    });
    v.len() >= count as usize
}

fn disallowed_str_present(s:&str, disallowed:&Vec<&str>)-> bool{
    for val in disallowed {
        if s.contains(val) {
            return true;
        }
    }
    false
}