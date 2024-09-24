from operator import le
import string
words = open("src/words.csv")
letter_count = dict.fromkeys(string.ascii_lowercase, 0)
total = 0
for word in words:
    for letter in word.strip("\n"):
        total+=1
        letter_count[letter] = letter_count[letter]+ 1

print(letter_count)

for letter in letter_count:
    print('map.insert("{}".to_string(), {:.2f});'.format(letter, (letter_count[letter]/total)*100))