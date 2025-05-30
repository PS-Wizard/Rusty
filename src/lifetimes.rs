use std::fmt::Display;

fn longest_with_announcements<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Annoucement {}", ann);
    if x.len() > y.len() {
        return x;
    }
    y
}
pub fn lifetimes() {

}
