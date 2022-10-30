use leetcode::flood_fill::Solution;

fn main() {

    let image = vec![vec![1,1,1], vec![1,1,0], vec![1,0,1]];
    let sr = 1;
    let sc = 1;
    let new_color = 2;
    let result = Solution::flood_fill(image, sr, sc, new_color);
    println!("2 result = {:#?}", result);

    let image = vec![vec![0,0,0], vec![0,0,0]];
    let sr = 0;
    let sc = 0;
    let new_color = 0;
    let result = Solution::flood_fill(image, sr, sc, new_color);
    println!("0 result = {:#?}", result);
}
