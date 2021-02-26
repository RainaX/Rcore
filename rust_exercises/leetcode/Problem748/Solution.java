class Solution {
    public String shortestCompletingWord(String licensePlate, String[] words) {
        int[] licenseMap = new int[26];
        for (int i = 0; i < licensePlate.length(); i += 1) {
            char ch = licensePlate.charAt(i);
            if (Character.isAlphabetic(ch)) {
                int index = (int) (Character.toLowerCase(ch) - 'a');
                licenseMap[index] += 1;
            }
        }
        
        String shortest = null;
        for (String word: words) {
            int[] wordMap = new int[26];
            for (int i = 0; i < word.length(); i += 1) {
                char ch = word.charAt(i);
                int index = (int) (ch - 'a');
                wordMap[index] += 1;
            }
            int i;
            for (i = 0; i < 26; i += 1) {
                if (wordMap[i] < licenseMap[i]) {
                    break;
                }
            }
            if (i >= 26 && (shortest == null || word.length() < shortest.length())) {
                shortest = word;
            }
        }
        
        return shortest;
    }
}
