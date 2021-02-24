fn main() {

   let mut arr: [i32; 5] = [10,20,30,40,50];
   let mut search=20;
   let mut start=0;
   let mut end=arr.len()-1;
   let mut value=0;
   println!("Question 1 Answer ");
   BinaryRecursion(search, end as i32, start, arr);
   linearsearch(&mut arr, search, value);
   leapyear();
}
fn BinaryRecursion(search:i32,end:i32,start:i32,mut arr: [i32;5]) {


   if start > end

   {
      return;
   }
   let mut mid=(start+end)/2;

   if arr[mid as usize] == search
   {

      println!("{ }",mid);
      return;
   }
   else if search > arr[mid as usize ]
   {
      BinaryRecursion(search,end,mid+1,arr);
      return;

      println!("{}",mid);
   }
   else {
      BinaryRecursion(search,mid-1,start,arr);
      return;

      println!("{ }",mid);
   }


}

pub fn linearsearch(arr:&[i32] ,search: i32,value: i32)
{
   if arr.len() == value as usize
   {
      println!("Number is not present");
      return;
   }
   if arr[value as usize] == search
   {
      println!("{}" , value);
      return ;
   }
   linearsearch(arr,  search , value+1);
}

fn leapyear() {

   println!("Question 3 answer");
   let tuple=(5,6,2021);
   let mut count = 0;

if tuple.2%4==0
      {
         count +=1;
      }

   println!("{}",count);
}



