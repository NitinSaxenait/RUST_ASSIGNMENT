fn main() {

   let mut arr: [i32; 5] = [10,20,30,40,50];
   let mut search=20;
   let mut start=0;
   let mut end=arr.len()-1;
   let mut value=0;
   println!("Question 1 Answer ");
   binaryRecursion(search, end as i32, start, arr);
   linearsearch(&mut arr, search, value);
   leapyear();
}
// function showing binary recursion
fn binaryRecursion(search:i32,end:i32,start:i32,mut arr: [i32;5]) {

//if array is empty return from function
   if start > end

   {
      return;
   }
   // dividing array into two parts
   let mut mid=(start+end)/2;
//if mid value value is equal to search value then return mid value
   if arr[mid as usize] == search
   {

      println!("{ }",mid);
      return;
   }
    // if search is greater then mid value then array became mid+1
   else if search > arr[mid as usize ]
   {
      binaryRecursion(search,end,mid+1,arr);
      return;

      println!("{}",mid);
   }
   else { 
      // if search is less  then mid value then array became mid-1
      
      binaryRecursion(search,mid-1,start,arr);
      return;

      println!("{ }",mid);
   }


}
//program to find linear search
pub fn linearsearch(arr:&[i32] ,search: i32,value: i32)
{
   //if array us empty then return statement
   if arr.len() == value as usize
   {
      println!("Number is not present");
      return;
   }
   //if array value is equal to search value,we will return search value as output
   if arr[value as usize] == search
   {
      println!("{}" , value);
      return ;
   }
   linearsearch(arr,  search , value+1);
}
//program to count no of leap year in tuple of array
fn leapyear() {

   println!("Question 3 answer");
   //tuple containing dates,month,year
   let tuple=(25,02,2021);
   let mut count = 0;
// if year is divided by 4 then it will be counted in count
if tuple.2%4==0
      {
         count +=1;
      }

   println!("{}",count);
}

//merge sort 
/*fn mergesort(arr:i32){
   let i=0;
   if arr.len<1
   {
      return;
   }
   let midpoint=arr.len/2;
   let mut left_arr: [i32]=midpoint;
   let mut right_arr: [i32];
   if arr.len % 2==0
   {
      right_arr=midpoint;
   }
   else {
      right_arr=midpoint+1;
   }
   while i<midpoint {
      left_arr= arr: [i32];

   }

   while i<midpoint {

   }
   left_arr=mergesort(left_arr);
   
   
   
}
*/


