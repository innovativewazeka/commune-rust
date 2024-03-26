import rust_thread_executor

def sum_fn():
    sum_value = 0
    for i in range(1, 100000001):
        sum_value += i
    print(sum_value)

A = rust_thread_executor.create_thread(sum_fn)
print("here => ", A)