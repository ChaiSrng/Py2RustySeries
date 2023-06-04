fn main(){
    let alice = "I like dogs";
    let bob = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says:{}", bob);
    println!("Alice says again:{}", alice);

    let pangram = "the quick brown fox jumps over the lazy";
    println!("Pangram: {}", pangram);

    println!("Words in reverse");
    for word in pangram.split(' ').rev(){
        println!("> {}", word);
    }

    let mut chars:Vec<char> = pangram.chars().collect(); //[c for c in pangram]
    chars.sort();
    chars.dedup();


    let mut string = String::new();
    for c in chars{
        string.push(c);
        string.push_str(", ");
    }

    //taking slice of characters that we want to get rid off
    let chars_to_trim:&[char] = &[' ',','];
    let trimmed_str:&str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", string);
    println!("Trimmed characters: {}", trimmed_str);
}