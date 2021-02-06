public class Solution {
    public List<List<String>> accountsMerge(List<List<String>> accounts) {
        Map<String, Integer> map = new HashMap<>();
        Node[] nodes = new Node[accounts.size()];
        for (int i = 0; i < accounts.size(); i += 1) {
            nodes[i] = new Node(i);
            List<String> account = accounts.get(i);
            for (int j = 1; j < account.size(); j += 1) {
                String email = account.get(j);
                
                if (map.containsKey(email)) {
                    Node.union(nodes[map.get(email)], nodes[i]);
                } else {
                    map.put(email, i);
                }
            }
        }
        
        Map<Integer, List<String>> resultMap = new HashMap<>();
        for (Map.Entry<String, Integer> entry: map.entrySet()) {
            int rootId = nodes[entry.getValue()].getRootId();
            if (!resultMap.containsKey(rootId)) {
                resultMap.put(rootId, new ArrayList<>());
            }
            resultMap.get(rootId).add(entry.getKey());
        }
        
        List<List<String>>result = new ArrayList<>();
        for (Map.Entry<Integer, List<String>> entry: resultMap.entrySet()) {
            Collections.sort(entry.getValue());
            List<String> account = new ArrayList<>();
            result.add(account);
            account.add(accounts.get(entry.getKey()).get(0));
            account.addAll(entry.getValue());
        }
        
        return result;
    }
}

class Node {
    int id;
    Node root;
    int size;
    
    public Node (int id) {
        this.id = id;
        this.root = this;
        this.size = 1;
    }
    
    public static void union(Node n1, Node n2) {
        Node root1 = n1;
        while (root1.root != root1) {
            root1 = root1.root;
        }
        Node root2 = n2;
        while (root2.root != root2) {
            root2 = root2.root;
        }
        
        if (root1 == root2) {
            return;
        }
        
        if (root1.size > root2.size) {
            root2.root = root1;
            root1.size += root2.size;
        } else {
            root1.root = root2;
            root2.size += root1.size;
        }
    }
    
    public int getRootId() {
        Node r = this;
        while (r.root != r) {
            r = r.root;
        }
        return r.id;
    }
}
