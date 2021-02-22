fn main() {
    println!("Question No 2-Solution");
    println!("hello World Programming");
    fibonacci();
}
fn fibonacci(){
    let mut First_term=0;let mut count =0;
    let mut Second_term=1;
    let mut next_term;
    println!("Question No 3-Solution");
    println!("The fibonacci Series is:");
    while count<10{

        next_term=First_term+Second_term;
        print!(" { }",next_term);
        First_term=Second_term;
        Second_term=next_term;
        count+=1;

    }
}
