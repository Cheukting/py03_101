import pyo3_101 as p1

sum = p1.sum_as_string(1,2)
print(f"{sum} as type {type(sum)}")

# test say_hello
print(p1.say_hello(name = "John"))

# test check_reg
print(p1.check_reg("reg_list.txt", "John"))
