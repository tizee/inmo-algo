/*
 * minimum_difficulty_of_a_job_schedule
 * You want to schedule a list of jobs in d days. Jobs are dependent (i.e To work on the i-th job, you have to finish all the jobs j where 0 <= j < i).
 * You have to finish at least one task every day. The difficulty of a job schedule is the sum of difficulties of each day of the d days. The difficulty of a day is the maximum difficulty of a job done in that day.
 * Given an array of integers jobDifficulty and an integer d. The difficulty of the i-th job is jobDifficulty[i].
 * Return the minimum difficulty of a job schedule. If you cannot find a schedule for the jobs return -1.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/16/untitled.png" style="width: 365px; height: 230px;" />
 * Input: jobDifficulty = [6,5,4,3,2,1], d = 2
 * Output: 7
 * Explanation: First day you can finish the first 5 jobs, total difficulty = 6.
 * Second day you can finish the last job, total difficulty = 1.
 * The difficulty of the schedule = 6 + 1 = 7
 *
 * Example 2:
 *
 * Input: jobDifficulty = [9,9,9], d = 4
 * Output: -1
 * Explanation: If you finish a job per day you will still have a free day. you cannot find a schedule for the given jobs.
 *
 * Example 3:
 *
 * Input: jobDifficulty = [1,1,1], d = 3
 * Output: 3
 * Explanation: The schedule is one job per day. total difficulty will be 3.
 *
 * Example 4:
 *
 * Input: jobDifficulty = [7,1,7,1,7,1], d = 3
 * Output: 15
 *
 * Example 5:
 *
 * Input: jobDifficulty = [11,111,22,222,33,333,44,444], d = 6
 * Output: 843
 *
 *
 * Constraints:
 *
 * 	1 <= jobDifficulty.length <= 300
 * 	0 <= jobDifficulty[i] <= 1000
 * 	1 <= d <= 10
 *
 *
 * problem link: https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/
 * discussion link: https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/discuss/?currentPage=1&orderBy=most_votes&query=
 *
 * */

struct Solution1 {}

impl Solution1 {
    // straight forward dfs witch cache
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let N = job_difficulty.len() as i32;
        if d > N {
            return -1;
        }
        let mut cache: Vec<Vec<i32>> = Vec::with_capacity(N as usize);
        // init cache
        for _ in 0..N {
            let mut row: Vec<i32> = Vec::with_capacity((d + 1) as usize);
            for _ in 0..d + 1 {
                row.push(-1);
            }
            cache.push(row);
        }
        return Solution1::dfs(d, 0, &mut cache, &job_difficulty);
    }

    fn dfs(
        cur_day: i32,
        cur_len: i32,
        cache: &mut Vec<Vec<i32>>,
        job_difficulty: &Vec<i32>,
    ) -> i32 {
        let N = job_difficulty.len() as i32;
        if cur_len == N && cur_day == 0 {
            return 0;
        };
        if cur_len == N || cur_day == 0 {
            return i32::MAX;
        }
        let val = cache[cur_len as usize][cur_day as usize];
        if val != -1 {
            return val;
        }
        let mut min_val = i32::MAX;
        let mut cur_max = job_difficulty[cur_len as usize];
        for i in cur_len..N {
            // local maximum
            cur_max = std::cmp::max(cur_max, job_difficulty[i as usize]);
            // local minimum
            let temp_min = Solution1::dfs(cur_day - 1, i + 1, cache, job_difficulty);
            if temp_min != i32::MAX {
                // global
                min_val = std::cmp::min(min_val, temp_min + cur_max);
            }
        }
        cache[cur_len as usize][cur_day as usize] = min_val;
        return cache[cur_len as usize][cur_day as usize];
    }
}

struct Solution2 {}

// bottom up 1d dp
impl Solution2 {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let N = job_difficulty.len() as i32;
        if d > N {
            return -1;
        }
        let mut dp: Vec<i32> = Vec::with_capacity((N + 1) as usize);
        for _ in 0..N + 1 {
            dp.push(i32::MAX);
        }
        dp[N as usize] = 0;
        for day in 1..d + 1 {
            for i in 0..(N - day + 1) {
                let mut cur_max = 0;
                dp[i as usize] = i32::MAX;
                for j in i..(N - day + 1) {
                    cur_max = std::cmp::max(cur_max, job_difficulty[j as usize]);
                    // update if could reduce
                    dp[i as usize] =
                        std::cmp::min(dp[i as usize], cur_max.saturating_add(dp[(j + 1) as usize]));
                }
            }
        }
        return dp[0];
    }
}

struct Solution3 {}

// 1d dp, optimization with monotonic stack
impl Solution3 {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let days = d as usize;
        let n = job_difficulty.len();
        if n < days {
            return -1;
        }
        let max_job_difficulty = job_difficulty.iter().max().unwrap();

        // current day result
        let mut new_dp = vec![std::i32::MAX - max_job_difficulty; n];
        // the day before result
        let mut old_dp = vec![0; n];

        // monotonic/minimum stack
        let mut stack = vec![];

        for d in 0..days {
            stack.clear();
            std::mem::swap(&mut old_dp, &mut new_dp);

            for i in d..n {
                new_dp[i] = if i > 0 {
                    old_dp[i - 1] + job_difficulty[i]
                } else {
                    job_difficulty[i]
                };
                while let Some(&j) = stack.last() {
                    if job_difficulty[j] > job_difficulty[i] {
                        break;
                    }
                    stack.pop();
                    new_dp[i] =
                        std::cmp::min(new_dp[i], new_dp[j] - job_difficulty[j] + job_difficulty[i]);
                }

                if let Some(&j) = stack.last() {
                    new_dp[i] = std::cmp::min(new_dp[i], new_dp[j]);
                }
                stack.push(i);
            }
        }
        new_dp[n - 1]
    }
}

#[cfg(test)]
mod test_1335 {
    use super::*;
    #[test]
    fn test_1335_solution1() {
        assert_eq!(
            Solution1::min_difficulty([7, 1, 7, 1, 7, 1].to_vec(), 3),
            15
        );
        assert_eq!(
            Solution1::min_difficulty([11, 111, 22, 222, 33, 333, 44, 444].to_vec(), 6),
            843
        );
        assert_eq!(
            Solution1::min_difficulty([11, 111, 22, 222, 44, 444, 333, 33].to_vec(), 6),
            843
        );
        assert_eq!(Solution1::min_difficulty([1, 1, 1].to_vec(), 3), 3);
        assert_eq!(Solution1::min_difficulty([9, 9, 9].to_vec(), 4), -1);
    }

    #[test]
    fn test_1335_solution2() {
        assert_eq!(
            Solution2::min_difficulty([7, 1, 7, 1, 7, 1].to_vec(), 3),
            15
        );
        assert_eq!(
            Solution2::min_difficulty([11, 111, 22, 222, 33, 333, 44, 444].to_vec(), 6),
            843
        );
        assert_eq!(
            Solution2::min_difficulty([11, 111, 22, 222, 44, 444, 333, 33].to_vec(), 6),
            843
        );
        assert_eq!(Solution2::min_difficulty([1, 1, 1].to_vec(), 3), 3);
        assert_eq!(Solution2::min_difficulty([9, 9, 9].to_vec(), 4), -1);
    }
}
