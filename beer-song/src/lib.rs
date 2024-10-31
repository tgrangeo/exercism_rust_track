pub fn verse(n: u32) -> String {
    if n == 0 {
        return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    } else {   
        return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.",n,n,n-1);
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut res: String = String::from("");
    let mut counter = start;
    while counter < end {
       res.push_str(&verse(start));
       counter -= 1;
    }
    return res;
}
