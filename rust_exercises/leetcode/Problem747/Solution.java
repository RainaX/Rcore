class Solution {
    public int dominantIndex(int[] nums) {
        if (nums.length == 1) {
            return 0;
        }
        
        int first_id = -1;
        int second_id = -1;
        
        for (int i = 0; i < nums.length; i += 1) {
            if (first_id ==-1 || nums[i] > nums[first_id]) {
                second_id = first_id;
                first_id = i;
            } else if (second_id == -1 || nums[i] > nums[second_id]) {
                second_id = i;
            }
        }
        
        if (nums[first_id] >= 2 * nums[second_id]) {
            return first_id;
        }
        return -1;
    }
}
