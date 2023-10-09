use std::io::Read;

fn join_vecs(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32>{
    return vec1.into_iter().chain(vec2).collect::<Vec<i32>>();
}

// fn multiply_vec(key: &Vec<i32>, n: i32) -> Vec<i32> {
//     let res = key.clone();
//
//     let res = (0..=n).map(|x| vec!())
//         .reduce(|acc, _i| join_vecs(acc, key.clone()))
//         .expect("this should never happen");
//
//     return res;
//     // return res.clone();
//
//     // let res = (0..=n).map(|x| "".to_string())
//     //     .reduce(|acc, _i| [acc, key.clone()].join(""));
//     // return res.expect("this should never happen");
// }
//
// fn get_key_with_length(key: &Vec<i32>, n: i32) -> Vec<i32> {
//     let multiply_times = n / key.len() as i32;
//     let new_key = multiply_vec(key, multiply_times + 1).into_iter()
//         .take(n as usize).collect::<Vec<i32>>();
//         // .expect("this should never happen");
//
//     return new_key;
// }
//

fn split_text_recursive(text: &String, n: usize, res: Vec<String>) -> Vec<String> {
    if text.len() == 0 {return res;}
    if text.len() <= n {
        let new_res = res.into_iter().chain(vec!(text.clone()).into_iter()).collect::<Vec<String>>();
        return new_res;
    }

    let head = text.chars().take(n)
        .map(|c| c.to_string())
        .reduce(|acc, c| [acc, c].join(""))
        .expect("this should never happen");

    let tail = text.chars().skip(n)
        .map(|c| c.to_string())
        .reduce(|acc, c| [acc, c].join(""))
        .expect("this should never happen");

    let new_res = res.into_iter().chain(vec!(head).into_iter()).collect::<Vec<String>>();
    return split_text_recursive(&tail, n, new_res);

}

fn split_text(text: &String, n: usize) -> Vec<String> {
    return split_text_recursive(text, n, vec!());
    // return vec!();
}

fn permutate_text(text: &String, key: &Vec<i32>) -> String{

    let mut zipped = text.chars().into_iter().zip(key.clone())
        .collect::<Vec<(char, i32)>>();

    zipped.sort_by(|(c1, x1), (c2, x2)| x1.cmp(x2));



    let res = zipped.into_iter().map(|(c, x)| c)
        .map(|c| c.to_string())
        .reduce(|acc, c| [acc, c].join(""))
        .expect("this should never happen");
    return res;
    // return text.clone();
}

fn en_de_crypt(text: &String, key: &Vec<i32>) -> String {
    // let new_key = get_key_with_length(key, text.len() as i32);
    let splits = split_text(text, key.len());
    let decrypted = splits.iter()
        .map(|x| permutate_text(x, key))
        .reduce(|acc, x| [acc, x].join(""))
        .expect("this should never happen");

    return decrypted;
    "".to_string()
}


fn z3(){

    let en_permutation = vec!(2,4,6,1,8,3,5,7);
    let de_permutation = vec!(4,1,6,2,7,3,8,5);
    let cyphertext = "TGEEMNELNNTDROEOAAHDOETCSHAEIRLM".to_string();

    println!("Decryption");
    let decyphered = en_de_crypt(&cyphertext, &de_permutation);
    println!("Plaintext: {}", decyphered.to_lowercase());
    println!("Cyphertext: {cyphertext}");

    println!("Encryption");
    println!("Plaintext: {}", decyphered.to_lowercase());

    let encrypted = en_de_crypt(&decyphered, &en_permutation);
    println!("Cyphertext: {encrypted}");

}

fn main() {
    z3()
}
