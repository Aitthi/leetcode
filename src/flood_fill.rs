

pub struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let old_color = image[sr as usize][sc as usize] as usize;
        if old_color == color as usize {
            return image;
        }
        struct Dfs {
            f: fn(&mut Dfs, &mut Vec<Vec<i32>>, usize, usize, usize, usize),
        }
        let mut dfs = Dfs {
            f: |dfs, image, r, c, old_color, color| {
                image[r][c] = color as i32;
                if r > 0 && image[r - 1][c] as usize == old_color {
                    (dfs.f)(dfs, image, r - 1, c, old_color, color);
                }
                if r < image.len() - 1 && image[r + 1][c] as usize == old_color {
                    (dfs.f)(dfs, image, r + 1, c, old_color, color);
                }
                if c > 0 && image[r][c - 1] as usize == old_color {
                    (dfs.f)(dfs, image, r, c - 1, old_color, color);
                }
                if c < image[r].len() - 1 && image[r][c + 1] as usize == old_color {
                    (dfs.f)(dfs, image, r, c + 1, old_color, color);
                }
            },
        };
        (dfs.f)(&mut dfs, &mut image, sr as usize, sc as usize,old_color,color as usize);
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // [[1,1,1],[1,1,0],[1,0,1]]
    // 1
    // 1
    // 2
    #[test]
    fn target_2() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let sr = 1;
        let sc = 1;
        let new_color = 2;
        let result = Solution::flood_fill(image, sr, sc, new_color);
        assert_eq!(result, vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]);
    }

    // [[0,0,0],[0,0,0]]
    // 0
    // 0
    // 0
    #[test]
    fn target_0() {
        let image = vec![vec![0, 0, 0], vec![0, 0, 0]];
        let sr = 0;
        let sc = 0;
        let new_color = 0;
        let result = Solution::flood_fill(image, sr, sc, new_color);
        assert_eq!(result, vec![vec![0, 0, 0], vec![0, 0, 0]]);
    }
}