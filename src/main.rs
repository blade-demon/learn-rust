fn main() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好！";
    let english = "World, hello!";
    let regions = [southern_germany, chinese, english];
    for region in regions {
        println!("{}", region);
    }
}