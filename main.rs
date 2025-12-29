pub fn first_non_repeating(s: &str) -> Option<char> 
{   
    let mut ans: Option<char> = None;
    if s.is_empty() { return ans; }
    
    let chars1: Vec<char> = s.chars().collect();
    let chars2: Vec<char> = s.chars().collect();
    let chars3: Vec<char> = s.chars().collect();
    
    
    'outer: for i in 0..chars1.len() 
    {
         ans = Some(chars1[i]);
         //println!("i:{:?}", ans);
         
         'inner: for j in i+1..chars2.len() 
         {
            //println!("j:{:?}", j);
            //println!("i:{:?}", ans);
             if ans.is_some() && ans.unwrap().to_ascii_lowercase() == chars2[j].to_ascii_lowercase()
             {
                 ans = None;
             }
         }
         
         if i>0
         {
            'inner2: for k in (0..=i-1).rev()
            {
                if ans.is_some() && ans.unwrap().to_ascii_lowercase() == chars3[k].to_ascii_lowercase()
                {
                    ans = None;
                }
            }
         }

         
         if ans.is_some()
         {
             break 'outer;
         }
    }
    
    ans
}

fn main ()
{
    println!("{:?}", first_non_repeating("moonmen"));
}
