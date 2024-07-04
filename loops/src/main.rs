fn main() {
    // let is_even = true;
    // if(is_even) {
    //     println!("no. is even");
    // }else{
    //     println!("no. is not even");
    // }

    // for i in 0..100{
    //     print!("{}", i);
    // }

    let sentence = String ::from("my name is");
    let first_word =  get_first_word(sentence);
    
    let n = 100;
    for i in 0..n{
        println!("Heelo {}", i);
    }
    print!("First word is : {}", first_word);
}

fn get_first_word(sentence: String)->String{
    let mut ans  = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }
    return ans;
}

