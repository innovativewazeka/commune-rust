import rust_thread_executor
import time

def fn():
    print("start test")
    start_time = time.time()
    sum_value = 0
    for i in range(1000000001):
        sum_value += i
    print("end test")
    elapsed_time = time.time() - start_time
    print("elapsed time is", sum_value, elapsed_time)

print(rust_thread_executor.thread_executor(fn))