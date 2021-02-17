class MyCalendarTwo {
    TreeMap<Integer, Integer> once;
    TreeMap<Integer, Integer> twice;
    public MyCalendarTwo() {
        once = new TreeMap<>();
        twice = new TreeMap<>();
    }
    
    public boolean book(int start, int end) {
        Integer twice_left = twice.floorKey(start);
        Integer twice_right = twice.ceilingKey(start);
        
        
        if ((twice_left != null && twice.get(twice_left) > start) || (twice_right != null && end > twice_right)) {
            return false;
        }
        
        
        Integer once_left = once.floorKey(start);
        TreeMap<Integer, Integer> temp = new TreeMap<>(once.subMap(once_left == null? start: once_left, end));
        
        
        
        List<Integer> twice_parts = new ArrayList<>();        
        for (Map.Entry<Integer, Integer> entry: temp.entrySet()) {
            if (entry.getValue() <= start) {
                continue;
            }
            if (entry.getKey() < start && entry.getValue() < end) {
                twice_parts.add(start);
                twice_parts.add(entry.getValue());
                twice.put(start, entry.getValue());
                once.replace(entry.getKey(), start);
            } else if (entry.getValue() > end && entry.getKey() > start) {
                twice_parts.add(entry.getKey());
                twice_parts.add(end);
                twice.put(entry.getKey(), end);
                once.put(end, entry.getValue());
                once.remove(entry.getKey());
            } else if (entry.getKey() <= start && entry.getValue() >= end) {
                twice_parts.add(start);
                twice_parts.add(end);
                twice.put(start, end);
                if (start > entry.getKey()) {
                    once.replace(entry.getKey(), start);
                } else {
                    once.remove(entry.getKey());
                }
                
                if (end < entry.getValue()) {
                    once.put(end, entry.getValue());
                }
            } else {
                twice_parts.add(entry.getKey());
                twice_parts.add(entry.getValue());
                twice.put(entry.getKey(), entry.getValue());
                once.remove(entry.getKey());
            }
        }
        
        int time = start;
        for (int i = 0; i < twice_parts.size(); i += 2) {
            if (time < twice_parts.get(i)) {
                once.put(time, twice_parts.get(i));
            }
            time = twice_parts.get(i + 1);
        }
        
        if (time < end) {
            once.put(time, end);
        }
        
        
        return true;
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * MyCalendarTwo obj = new MyCalendarTwo();
 * boolean param_1 = obj.book(start,end);
 */
