public class Solution {
    public List<String> removeComments(String[] source) {
        List<String> result = new ArrayList<>();
        boolean block = false;
        int i = 0;
        int pos = 0;
        StringBuilder sb = new StringBuilder();
        while (i < source.length) {
            if (block) {
                int pos_end = source[i].substring(pos).indexOf("*/");
                if (pos_end < 0) {
                    i += 1;
                    pos = 0;
                } else {
                    pos += pos_end + 2;
                    block = false;
                    if (pos >= source[i].length()) {
                        i += 1;
                        pos = 0;
                        if (sb.length() > 0) {
                            result.add(sb.toString());
                            sb = new StringBuilder();
                        }
                    }
                }
            } else {
                String remain = source[i].substring(pos);
                int pos_line = remain.indexOf("//");
                int pos_block = remain.indexOf("/*");
                if (pos_line < 0 && pos_block < 0) {
                    sb.append(remain);
                    i += 1;
                    pos = 0;
                    result.add(sb.toString());
                    sb = new StringBuilder();
                } else if ((pos_line >= 0 && pos_block < 0) || (pos_line >= 0 && pos_block >= 0 && pos_line < pos_block)) {
                    sb.append(remain.substring(0, pos_line));
                    i += 1;
                    pos = 0;
                    if (sb.length() > 0) {
                        result.add(sb.toString());
                        sb = new StringBuilder();
                    }
                } else {
                    sb.append(remain.substring(0, pos_block));
                    pos += pos_block + 2;
                    block = true;
                    if (pos >= source[i].length()) {
                        i += 1;
                        pos = 0;
                    }
                }
            }
        }
        
        return result;
    }
}
