class Solution {
    public List<Integer> selfDividingNumbers(int left, int right) {
        List<Integer> result = new ArrayList<>();
        
        for (int x = left; x <= right; x += 1) {
            int temp = x;
            while (temp > 0) {
                int digit = temp % 10;
                if (digit == 0 || x % digit != 0) {
                    break;
                }
                temp /= 10;
            }
            if (temp == 0) {
                result.add(x);
            }
        }
        
        return result;
    }
}
