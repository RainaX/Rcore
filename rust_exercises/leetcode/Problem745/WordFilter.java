class WordFilter {
    SuffixNode suffixRoot;

    public WordFilter(String[] words) {
        suffixRoot = new SuffixNode();
        
        for (int i = 0; i < words.length; i += 1) {
            suffixRoot.insert(words[i], i);
        }
    }
    
    public int f(String prefix, String suffix) {
        return suffixRoot.search(prefix, suffix);
    }
}

class SuffixNode {
    Map<Character, SuffixNode> children;
    PrefixNode prefixRoot;
    
    public SuffixNode() {
        children = new HashMap<>();
        prefixRoot = new PrefixNode();
    }
    
    public void insert(String word, int index) {
        SuffixNode curr = this;
        for (int i = word.length() - 1; i >= 0; i -= 1) {
            char ch = word.charAt(i);
            if (!curr.children.containsKey(ch)) {
                curr.children.put(ch, new SuffixNode());
            }
            curr = curr.children.get(ch);
            curr.prefixRoot.insert(word, index);
        }
    }
    
    public int search(String prefix, String suffix) {
        SuffixNode curr = this;
        for (int i = suffix.length() - 1; i >= 0; i -= 1) {
            char ch = suffix.charAt(i);
            if (!curr.children.containsKey(ch)) {
                return -1;
            }
            curr = curr.children.get(ch);
        }
        
        return curr.prefixRoot.search(prefix);
        
    }
}

class PrefixNode {
    Map<Character, PrefixNode> children;
    int maxIndex;
    
    public PrefixNode() {
        children = new HashMap<>();
        maxIndex = -1;
    }
    
    public void insert(String word, int index) {
        PrefixNode curr = this;
        for (int i = 0; i < word.length(); i += 1) {
            curr.maxIndex = Math.max(curr.maxIndex, index);
            char ch = word.charAt(i);
            if (!curr.children.containsKey(ch)) {
                curr.children.put(ch, new PrefixNode());
            }
            curr = curr.children.get(ch);
        }
        curr.maxIndex = Math.max(curr.maxIndex, index);
    }
    
    public int search(String word) {
        PrefixNode curr = this;
        for (int i = 0; i < word.length(); i += 1) {
            char ch = word.charAt(i);
            if (!curr.children.containsKey(ch)) {
                return -1;
            }
            curr = curr.children.get(ch);
        }
        
        return curr.maxIndex;
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * WordFilter obj = new WordFilter(words);
 * int param_1 = obj.f(prefix,suffix);
 */
