class Solution {
    public int evaluate(String expression) {
        Map<Integer, Integer> paren = new HashMap<>();
        Stack<Integer> temp = new Stack<>();
        
        for (int i = 0; i < expression.length(); i += 1) {
            char c = expression.charAt(i);
            
            if (c == '(') {
                temp.push(i);
            } else if (c == ')') {
                paren.put(temp.pop(), i);
            }
        }
        
        return evaluate(expression, 0, expression.length() - 1, new HashMap<>(), paren);
    }
    
    private int evaluate(String expression, int l, int r, Map<String, Stack<Integer>> values, Map<Integer, Integer> paren) {
        if (expression.charAt(l) == '(') {
            return evaluate(expression, l + 1, r - 1, values, paren);
        }
        
        if (r - l >= 3 && expression.substring(l, l + 4).equals("let ")) {
            Stack<String> new_assigned = new Stack<>();
            int pos = l + 4;
            
            while (true) {
                
                int vr = get_right(expression, pos, r, paren);
                if (vr == r) {
                    break;
                }
                String v = expression.substring(pos, vr + 1);
                pos = vr + 2;
                
                int er = get_right(expression, pos, r, paren);
                int e = evaluate(expression, pos, er, values, paren);
                pos = er + 2;
                
                new_assigned.push(v);
                if (!values.containsKey(v)) {
                    values.put(v, new Stack<>());
                }
                values.get(v).push(e);
            }
            
            int ret = evaluate(expression, pos, r, values, paren);
            while (!new_assigned.isEmpty()) {
                String v = new_assigned.pop();
                values.get(v).pop();
                if (values.get(v).isEmpty()) {
                    values.remove(v);
                }
            }
            return ret;
        }
        
        if (r - l >= 3 && expression.substring(l, l + 4).equals("add ")) {
            int pos = l + 4;
            
            int e1r = get_right(expression, pos, r, paren);
            int e1 = evaluate(expression, pos, e1r, values, paren);
            pos = e1r + 2;
            
            int e2 = evaluate(expression, pos, r, values, paren);
            
            return e1 + e2;
        }
        
        if (r - l >= 4 && expression.substring(l, l + 5).equals("mult ")) {
            int pos = l + 5;
            
            int e1r = get_right(expression, pos, r, paren);
            int e1 = evaluate(expression, pos, e1r, values, paren);
            pos = e1r + 2;
            
            int e2 = evaluate(expression, pos, r, values, paren);
            
            return e1 * e2;
        }
        
        
        if (Character.isLowerCase(expression.charAt(l))) {
            return values.get(expression.substring(l, r + 1)).peek();
        }
        
        return Integer.parseInt(expression.substring(l, r + 1));
    }
    
    private int get_right(String expression, int l, int r, Map<Integer, Integer> paren) {
        if (expression.charAt(l) == '(') {
            return paren.get(l);
        }
        
        int right = l;
        while (right <= r && expression.charAt(right) != ' ') {
            right += 1;
        }
        return right - 1;
    }
}
