import pyo3_101 as p1

sum = p1.sum_as_string(1,2)
print(f"{sum} as type {type(sum)}")

# test say_hello
print(p1.say_hello(name = "John"))

# test check_reg
# print(p1.check_reg("reg_list.txt", "John"))

# test count_att
print(p1.count_att(["John", "Susan", "Peter", "Judy"]))

# test count_att
budget_dict = {
"John": 850,
"Susan": 790,
"Peter":1030,
"Judy": 540,
}
print(p1.travel_avg(budget_dict))

# test Attendee
print(f"Number of attendees are {p1.Attendee.cur_reg_num}")

me = p1.Attendee('Cheuk', True)
print(me.name, me.speaker, me.reg_num)
print(f"Number of attendees are {p1.Attendee.cur_reg_num}")

keynote = p1.Attendee('John', True)
print(keynote.name, keynote.speaker, keynote.reg_num)
keynote.name = 'Jon'
print(keynote.name, keynote.speaker, keynote.reg_num)

print(f"Number of attendees are {p1.Attendee.cur_reg_num}")

# test Fibonacci
# fib = p1.Fibonacci()
# for _ in range(10):
#     print(next(fib))

# for num in p1.Fibonacci():
#     print(num)

# test SaveLog
@p1.SaveLog
def say_hello(name=None):
    if name is None:
        print(f"calling say_hello with no name")
        return "hello"
    else:
        print(f"calling say_hello to {name}")
        return f"hello {name}"

say_hello()
say_hello("John")
print("==== check the log ====")
print(say_hello.log)
