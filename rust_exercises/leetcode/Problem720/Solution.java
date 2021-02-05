public class Solution {
    public String longestWord(String[] words) {
        Map<String, Integer> map = new HashMap<>();
        for (String word: words) {
            if (word.length() == 1) {
                map.put(word, 2);
            } else {
                map.put(word, 0);
            }
        }
        
        String longest = null;
        
        for (String word: map.keySet()) {
            if (isValid(word, map)) {
                if (longest == null || word.length() > longest.length() || (word.length() == longest.length() && word.compareTo(longest) < 0)) {
                    longest = word;
                }
            }
        }
        
        return longest;
    }
    
    private boolean isValid(String word, Map<String, Integer> map) {
        if (!map.containsKey(word)) {
            return false;
        }
        int state = map.get(word);
        switch (state) {
            case 2:
                return true;
            case 1:
                return false;
            default:
                if (isValid(word.substring(0, word.length() - 1), map)) {
                    map.replace(word, 2);
                    return true;
                }
                map.replace(word, 1);
                return false;
        }
    }
}
