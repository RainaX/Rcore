public class Solution {
    public int smallestDistancePair(int[] nums, int k) {
        Arrays.sort(nums);
        
        int lo = 0;
        int hi = nums[nums.length - 1] - nums[0];
        while (lo < hi) {
            int mi = (lo + hi) / 2;
            int count = 0;
            int left = 0;
            int right = 0;
            while (left < nums.length) {
                while (right < nums.length && nums[right] - nums[left] <= mi) {
                    right += 1;
                }
                count += right - left - 1;
                left += 1;
            }
            if (count >= k) {
                hi = mi;
            } else {
                lo = mi + 1;
            }
        }
        
        return lo;
    }
}
