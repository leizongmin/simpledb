## Benchmark

```
map_put                         count=100000    spent=1374ms    qps=72780
map_get                         count=100000    spent=252ms     qps=396825
map_count                       count=100000    spent=113ms     qps=884955
map_delete                      count=100000    spent=1356ms    qps=73746

set_add                         count=100000    spent=1392ms    qps=71839
set_count                       count=100000    spent=109ms     qps=917431
set_is_member                   count=100000    spent=238ms     qps=420168
set_delete                      count=100000    spent=1340ms    qps=74626

list_left_push                  count=100000    spent=1298ms    qps=77041
list_count                      count=100000    spent=185ms     qps=540540
list_left_pop                   count=100000    spent=1517ms    qps=65919
list_right_push                 count=100000    spent=1280ms    qps=78125
list_count                      count=100000    spent=154ms     qps=649350
list_right_pop                  count=100000    spent=1545ms    qps=64724

sorted_list_add                 count=10000     spent=140ms     qps=71428
sorted_list_count               count=10000     spent=20ms      qps=500000
sorted_list_left_pop            count=10000     spent=662ms     qps=15105
sorted_list_add                 count=10000     spent=132ms     qps=75757
sorted_list_right_pop           count=10000     spent=1211ms    qps=8257
```
