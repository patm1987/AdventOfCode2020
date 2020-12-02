--- Part Two ---
The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to 2020?

-- idea --
load in all numbers then sort them.
Add three iterators: smallest, smallest+1, largest
a, b, c
while a < b < c
    while a + b + c > 2020 then c --
    while a + b + c < 2020 then b ++
    if a + b + c == 2020 then return
    else a++, b = a+1

wait no:
a = smallest
b = a + 1
c = largest
while a < b < c
    while a + b + c > 2020
        c--
    b = (a + c) / 2
    while b > 2020
        b --
    while b < 2020
        b ++
    
    if a + b + c == 2020
        return true

    else
        a ++
        b = a + 1
return false
