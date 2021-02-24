class Solution {
    public int networkDelayTime(int[][] times, int n, int k) {
        Map<Integer, List<int[]>> edges = new HashMap<>();
        Map<Integer, Integer> delay = new HashMap<>();
        
        for (int[] time: times) {
            if (!edges.containsKey(time[0])) {
                edges.put(time[0], new ArrayList<>());
            }
            edges.get(time[0]).add(new int[]{time[1], time[2]});
        }
        delay.put(k, 0);
        
        PriorityQueue<Node> pq = new PriorityQueue<>();
        pq.offer(new Node(k, 0));
        
        while (!pq.isEmpty()) {
            Node closest = pq.poll();
            if (!edges.containsKey(closest.id)) {
                continue;
            }
            for (int[] pair: edges.get(closest.id)) {
                int dest = pair[0];
                int time = closest.time + pair[1];
                if (!delay.containsKey(dest)) {
                    delay.put(dest, time);
                    pq.offer(new Node(dest, time));
                } else {
                    if (time < delay.get(dest)) {
                        delay.replace(dest, time);
                        pq.offer(new Node(dest, time));
                    }
                }
            }
        }
        
        if (delay.size() < n) {
            return -1;
        }
        
        int result = 0;
        for (Map.Entry<Integer, Integer> entry: delay.entrySet()) {
            result = Math.max(result, entry.getValue());
        }
        
        return result;
    }
}

class Node implements Comparable<Node> {
    int id;
    int time;
    
    public Node(int id, int time) {
        this.id = id;
        this.time = time;
    }
    
    @Override
    public int compareTo(Node n) {
        return this.time - n.time;
    }
}
