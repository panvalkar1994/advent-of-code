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


// --- Part Two ---
// Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.

// Now, a nice string is one with all of the following properties:

// It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
// It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
// For example:

// qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
// xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
// uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
// ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.

pub fn get_nice_string_count_v2(input:String)->i32{
    let mut count = 0;
    for line in input.lines() {
        if is_repeated_pair_present(line) && is_one_skip_same_letter(line) {
            count += 1;
        }
    }
    count
}

fn is_repeated_pair_present(input:&str) -> bool {
    let v = input.as_bytes();
    for i in 0..v.len()-1 {
        for j in i+2..v.len()-1{
            if v[i] == v[j] && v[i+1] == v[j+1] {
                return true;
            }
        }
    }
    false
}

fn is_one_skip_same_letter(input:&str) -> bool {
    let v = input.as_bytes();
    for i in 0..v.len()-2{
        if v[i] == v[i+2] {
            return true;
        }
    }
    false
}