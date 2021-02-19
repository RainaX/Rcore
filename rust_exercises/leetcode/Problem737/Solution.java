class Solution {
    public boolean areSentencesSimilarTwo(String[] words1, String[] words2, List<List<String>> pairs) {
        if (words1.length != words2.length) {
            return false;
        }
        
        Map<String, Node> map = new HashMap<>();
        for (List<String> pair: pairs) {
            if (!map.containsKey(pair.get(0))) {
                map.put(pair.get(0), new Node(pair.get(0)));
            }
            if (!map.containsKey(pair.get(1))) {
                map.put(pair.get(1), new Node(pair.get(1)));
            }
            
            Node.union(map.get(pair.get(0)), map.get(pair.get(1)));
        }
        
        for (int i = 0; i < words1.length; i += 1) {
            String w1 = words1[i];
            String w2 = words2[i];
            
            if (w1.equals(w2)) {
                continue;
            }
            
            if (map.containsKey(w1) && map.containsKey(w2) && Node.similar(map.get(w1), map.get(w2))) {
                continue;
            }
            
            return false;
        }
        return true;
    }
}


class Node {
    String word;
    Node parent;
    int count;
    
    public Node(String word) {
        this.word = word;
        this.parent = this;
        this.count = 1;
    }
    
    static void union(Node n1, Node n2) {
        Node root1 = n1.getRoot();
        Node root2 = n2.getRoot();
        if (root1 == root2) {
            return;
        }
        
        if (root1.count > root2.count) {
            root2.parent = root1;
            root1.count += root2.count;
        } else {
            root1.parent = root2;
            root2.count += root1.count;
        }
    }
    
    static boolean similar(Node n1, Node n2) {
        return n1.getRoot() == n2.getRoot();
    }
    
    private Node getRoot() {
        Node curr = this;
        while (curr.parent != curr) {
            curr = curr.parent;
        }
        return curr;
    }
}
