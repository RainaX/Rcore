class Solution {
    public boolean areSentencesSimilar(String[] sentence1, String[] sentence2, List<List<String>> similarPairs) {
        if (sentence1.length != sentence2.length) {
            return false;
        }
        
        Map<String, Set<String>> map = new HashMap<>();
        for (List<String> pair: similarPairs) {
            String w0 = pair.get(0);
            String w1 = pair.get(1);
            
            if (!map.containsKey(w0)) {
                map.put(w0, new HashSet<>());
            }
            map.get(w0).add(w1);
        }
        
        for (int i = 0; i < sentence1.length; i += 1) {
            String w1 = sentence1[i];
            String w2 = sentence2[i];
            
            if (!w1.equals(w2) && !(map.containsKey(w1) && map.get(w1).contains(w2)) && !(map.containsKey(w2) && map.get(w2).contains(w1))) {
                return false;
            }
        }
        
        return true;
    }
}
