## Benchmark

```
map_put                         count=100000    spent=951ms     qps=105152
map_get                         count=100000    spent=151ms     qps=662251
map_count                       count=100000    spent=73ms      qps=1369863
map_delete                      count=100000    spent=1001ms    qps=99900

set_add                         count=100000    spent=914ms     qps=109409
set_count                       count=100000    spent=61ms      qps=1639344
set_is_member                   count=100000    spent=134ms     qps=746268
set_delete                      count=100000    spent=951ms     qps=105152

list_left_push                  count=100000    spent=859ms     qps=116414
list_count                      count=100000    spent=88ms      qps=1136363
list_left_pop                   count=100000    spent=1038ms    qps=96339
list_right_push                 count=100000    spent=868ms     qps=115207
list_count                      count=100000    spent=84ms      qps=1190476
list_right_pop                  count=100000    spent=1015ms    qps=98522

sorted_list_add                 count=10000     spent=82ms      qps=121951
sorted_list_count               count=10000     spent=6ms       qps=1666666
sorted_list_left_pop            count=10000     spent=498ms     qps=20080
sorted_list_add                 count=10000     spent=81ms      qps=123456
sorted_list_right_pop           count=10000     spent=1011ms    qps=9891

sorted_set_add                  count=10000     spent=120ms     qps=83333
sorted_set_is_member            count=10000     spent=15ms      qps=666666
sorted_set_count                count=10000     spent=7ms       qps=1428571
sorted_set_left                 count=10000     spent=249ms     qps=40160
sorted_set_right                count=10000     spent=725ms     qps=13793
sorted_set_delete               count=10000     spent=363ms     qps=27548
```
