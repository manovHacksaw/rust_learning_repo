struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u128
}

fn main() {

    let user1 :User= User{
        active: true,
        username: String::from("ig_raaz"),
        email: String::from("rajhcodes@gmail.com"),
        sign_in_count: 678
    };

    


    
    let sentence:String  = String::from("Entschudigung, milch bitte!");
    let first_word: String = get_first_word(sentence);
    println!("The first word is: {}", first_word);

    let mut a:i32 = 1;
    let mut b: i32 = 5;

    println!("A: {}, B: {}",a, b );

    
    let  c: i32 = a;
  
    a = b;
    b = c;

    println!("A: {}, B: {}",a, b );

    
    
}


fn get_first_word(sentence: String) -> String {
    let mut ans = String::new();

    for character in sentence.chars() {
        if character == ' ' {
            break;
        }
        ans.push(character);
    }

    ans
}