class Solution {
    public int deleteAndEarn(int[] nums) {
        TreeMap<Integer, Integer> map = new TreeMap<>();
        for (int num: nums) {
            Integer prev = map.get(num);
            if (prev == null) {
                map.put(num, num);
            } else {
                map.put(num, prev.intValue() + num);
            }
        }
        
        int prev = 0;
        int sum0 = 0;
        int sum1 = 0;
        int count = 0;
        for (Map.Entry<Integer, Integer> entry: map.entrySet()) {
            if (count > 0 && entry.getKey() == prev + 1) {
                int temp0 = sum0;
                sum0 = Math.max(sum0, sum1);
                sum1 = temp0 + entry.getValue();
            } else {
                sum0 = Math.max(sum0, sum1);
                sum1 = sum0 + entry.getValue();
            }
            count += 1;
            prev = entry.getKey();
        }
        
        return Math.max(sum0, sum1);
    }
}
