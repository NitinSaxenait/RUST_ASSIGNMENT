//function to find repeated character in a given string
pub fn repeated_char(str : &str, i : usize, j : usize)
{
    //using vector to collect characters
    let mut char_vector : Vec<char> = str.chars().collect();
    if i < str.len() {
        let mut count = 1;

        if j < str.len() {
            //if character is same then count +1
            //same character must not count
            if char_vector[i] == char_vector[j] && char_vector[i]!=' ' {
                count += 1;
                char_vector[j] = '0';
            }
            let j = j + 1;
            repeated_char(&str,i,j);
        }
        //if found duplicate character then print that character

        let i = i+1;

        repeated_char(&str,i,j);
        if count > 1 && char_vector[i] != '0' {
            print!("{ } ",char_vector[i]);
        }
    }
}