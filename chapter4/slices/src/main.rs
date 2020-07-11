// Slices let you reference a contiguous sequence of elements
// in a collection rather than the whole collection


fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    
    s.clear();

    let s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..11];

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
