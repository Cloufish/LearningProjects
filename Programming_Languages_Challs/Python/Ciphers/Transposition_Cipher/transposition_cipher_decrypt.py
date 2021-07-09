import math

def main():
    myMessage = input("Input your secret MESSAGE that you've got!: ")
    myKey = int(input("Input your KEY in integer that you've got!: "))

    plaintext = decryptMessage(myKey, myMessage)

    #Print with a | (called "pipe" character) after it in case there are spaces at the end of the decryoted message:
    print(plaintext + '|')

def decryptMessage(key, message):
    # The transposition decrypt function will simulate the "colums" and "rows" of the grid that the plaintext is written on by using a list of string. First, we need to calclate few values."

    #The number of "columns" in our transposition grid:
    numOfColumns = int(math.ceil(len(message) / float(key)))
    #The number of "rows" in our grid:
    numOfRows = key
    #The number of "shaded boxes" in the last "column" of the grid:
    numOfShadedBoxes = (numOfColumns * numOfRows) - len(message)

    # Each string in plaintext represents a column in the grid:
    plaintext = [''] * numOfColumns

    #The column and row variables point to where the grid the next character in the encrypted message will go:

    column = 0
    row = 0

    for symbol in message:
        plaintext[column] += symbol
        column += 1 # Point o the next column.

        #If there are no more columns OR we're at a shaded box, go back to the first column and the next row:
        if (column == numOfColumns) or (column == numOfColumns - 1 and row         >= numOfRows - numOfShadedBoxes):
            column = 0
            row += 1
    return ''.join(plaintext)

if __name__ == '__main__':
    main()