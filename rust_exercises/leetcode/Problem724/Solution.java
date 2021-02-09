public class Solution {
    public int pivotIndex(int[] nums) {
        int leftSum = 0;
        int rightSum = 0;
        for (int i = 0; i < nums.length; i += 1) {
            rightSum += nums[i];
        }
        
        int pivot = 0;
        while (pivot < nums.length) {
            rightSum -= nums[pivot];
            if (leftSum == rightSum) {
                return pivot;
            }
            leftSum += nums[pivot];
            pivot += 1;
        }
        
        return -1;
    }
}
