/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public ListNode[] splitListToParts(ListNode root, int k) {
        int len = 0;
        ListNode curr = root;
        while (curr != null) {
            len += 1;
            curr = curr.next;
        }
        
        int average = len / k;
        int extra = len % k;
        
        ListNode[] result = new ListNode[k];
        curr = root;
        for (int i = 0; i < k; i += 1) {
            result[i] = curr;
            for (int j = 0; j < (i < extra? average: average - 1); j += 1) {
                curr = curr.next;
            }
            if (curr != null) {
                ListNode next = curr.next;
                curr.next = null;
                curr = next;
            }
        }
        
        return result;
    }
}
