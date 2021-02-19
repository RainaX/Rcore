class MyCalendarThree {
    TreeMap<Integer, int[]> map;
    int k;

    public MyCalendarThree() {
        map = new TreeMap<>();
        k = 0;
    }
    
    public int book(int start, int end) {
        Integer prev = map.floorKey(start);
        
        Map<Integer, int[]> sub = map.subMap(prev == null? start: prev, end);
        TreeMap<Integer, int[]> temp = new TreeMap<>();
        for (Map.Entry<Integer, int[]> entry: sub.entrySet()) {
            temp.put(entry.getKey(), new int[]{entry.getValue()[0], entry.getValue()[1]});
        }
        
        
        List<int[]> covered = new ArrayList<>();
        for (Map.Entry<Integer, int[]> entry: temp.entrySet()) {
            int s = entry.getKey();
            int e = entry.getValue()[0];
            int count = entry.getValue()[1];
            
            if (e <= start) {
                continue;
            }
            k = Math.max(k, count + 1);
            if (s < start && e < end) {
                covered.add(new int[]{start, e});
                map.replace(s, new int[]{start, count});
                map.put(start, new int[]{e, count + 1});
            } else if (e > end && s > start) {
                covered.add(new int[]{s, end});
                map.replace(s, new int[]{end, count + 1});
                map.put(end, new int[]{e, count});
            } else if (s <= start && e >= end) {
                covered.add(new int[]{start, end});
                if (start > s) {
                    map.replace(s, new int[]{start, count});
                } else {
                    map.remove(s);
                }
                
                if (end < e) {
                    map.put(end, new int[]{e, count});
                }
                map.put(start, new int[]{end, count + 1});
            } else {
                covered.add(new int[]{s, e});
                map.replace(s, new int[]{e, count + 1});
            }
        }
        
        int time = start;
        for (int i = 0; i < covered.size(); i += 1) {
            if (time < covered.get(i)[0]) {
                map.put(time, new int[]{covered.get(i)[0], 1});
            }
            time = covered.get(i)[1];
        }
        if (time < end) {
            map.put(time, new int[]{end, 1});
        }
        k = Math.max(k, 1);
        
        
        
        
        return k;
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * MyCalendarThree obj = new MyCalendarThree();
 * int param_1 = obj.book(start,end);
 */
