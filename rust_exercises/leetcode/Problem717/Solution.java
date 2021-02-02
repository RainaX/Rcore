public class Solution {
    public boolean isOneBitCharacter(int[] bits) {
        int i = bits.length - 2;
        while (i >= 0 && bits[i] == 1) {
            i -= 1;
        }
        return (bits.length - i) % 2 == 0;
    }
}
