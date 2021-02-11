class Solution {
    public String countOfAtoms(String formula) {
        Map<Integer, Integer> open2Time = new HashMap<>();
        Map<Integer, Integer> close2Skip = new HashMap<>();
        
        int i = 0;
        Stack<Integer> temp = new Stack<>();
        while (i < formula.length()) {
            char c = formula.charAt(i);
            
            i += 1;
            
            if (c == '(') {
                temp.push(i - 1);
            } else if (c == ')') {
                int time = 0;
                int j = i - 1;
                while (i < formula.length() && Character.isDigit(formula.charAt(i))) {
                    time = time * 10 + (int) (formula.charAt(i) - '0');
                    i += 1;
                }
                time = Math.max(1, time);
                open2Time.put(temp.pop(), time);
                close2Skip.put(j, i);
            }
        }
        
        Map<String, Integer> map = new TreeMap<>();
        int count = 0;
        int time = 1;
        StringBuilder sb = new StringBuilder();
        
        i = 0;
        while (i < formula.length()) {
            char c = formula.charAt(i);
            
            if (Character.isUpperCase(c) || c == '(' || c == ')') {
                if (sb.length() > 0) {
                    String element = sb.toString();
                    count = Math.max(count, 1);
                    map.put(element, map.getOrDefault(element, 0) + count * time);
                    
                    sb = new StringBuilder();
                    count = 0;
                }
            }
            
            if (c == ')') {
                time /= temp.pop();
                i = close2Skip.get(i);
            } else {
                if (c == '(') {
                    temp.push(open2Time.get(i));
                    time *= temp.peek();
                } else if (Character.isAlphabetic(c)) {
                    sb.append(c);
                } else if (Character.isDigit(c)) {
                    count = count * 10 + (int) (c - '0');
                }
                i += 1;
            }
        }
        
        if (sb.length() > 0) {
            String element = sb.toString();
            count = Math.max(count, 1);
            map.put(element, map.getOrDefault(element, 0) + count * time);
            sb = new StringBuilder();
        }
        
        for (Map.Entry<String, Integer> entry: map.entrySet()) {
            sb.append(entry.getKey());
            if (entry.getValue() > 1) {
                sb.append(Integer.toString(entry.getValue()));
            }
        }
        
        return sb.toString();
    }
}
