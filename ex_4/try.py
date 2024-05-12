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
me = p1.Attendee('Cheuk', True)
print(me.name, me.speaker)

keynote = p1.Attendee('John', True)
print(keynote.name, keynote.speaker)
keynote.name = 'Jon'
print(keynote.name, keynote.speaker)
