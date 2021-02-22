/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
class Solution {
    public int findClosestLeaf(TreeNode root, int k) {
        Map<Integer, TreeNode> map = new HashMap<>();
        map.put(root.val, null);
        recordParent(root, map);
        
        Queue<Integer> queue = new LinkedList<>();
        Set<Integer> visited = new HashSet<>();
        
        queue.offer(k);
        visited.add(k);
        
        while (true) {
            int val = queue.poll();
            TreeNode parent = map.get(val);
            TreeNode curr;
            if (parent != null) {
                if (parent.left != null && parent.left.val == val) {
                    curr = parent.left;
                } else {
                    curr = parent.right;
                }
            } else {
                curr = root;
            }
            
            if (curr.left == null && curr.right == null) {
                return curr.val;
            }
            
            if (parent != null && !visited.contains(parent.val)) {
                visited.add(parent.val);
                queue.offer(parent.val);
            }
            if (curr.left != null && !visited.contains(curr.left.val)) {
                visited.add(curr.left.val);
                queue.offer(curr.left.val);
            }
            if (curr.right != null && !visited.contains(curr.right.val)) {
                visited.add(curr.right.val);
                queue.offer(curr.right.val);
            }
        }
        
    }
    
    private void recordParent(TreeNode node, Map<Integer, TreeNode> map) {
        if (node.left != null) {
            map.put(node.left.val, node);
            recordParent(node.left, map);
        }
        if (node.right != null) {
            map.put(node.right.val, node);
            recordParent(node.right, map);
        }
    }
}
