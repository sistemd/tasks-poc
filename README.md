Running tests spawning a large number of tasks an measuring memory usage.

Test ran with 10 million, 1 million, and 100 thousands tasks.

10 million, causes some task congestion to the point where sometimes it takes 5 seconds
for the "main" task to re-run:

![](plot-tasks-10-million.svg)

1 million, no congestion:

![](plot-tasks-1-million.svg)

100 thousand:

![](plot-tasks-100-000.svg)
