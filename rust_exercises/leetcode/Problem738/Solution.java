class Solution {
    public int monotoneIncreasingDigits(int N) {
        List<Integer> digits = new ArrayList<>();
        while (N > 0) {
            digits.add(N % 10);
            N /= 10;
        }
        
        int lo = 0;
        int hi = 1;
        while (hi < digits.size()) {
            if (digits.get(hi) > digits.get(hi - 1)) {
                digits.set(hi, digits.get(hi) - 1);
                for (int i = lo; i < hi; i += 1) {
                    digits.set(i, 9);
                }
                lo = hi;
            }
            hi += 1;
        }
        
        int result = 0;
        for (int i = digits.size() - 1; i >= 0; i -= 1) {
            result = 10 * result + digits.get(i);
        }
        return result;
    }
}
