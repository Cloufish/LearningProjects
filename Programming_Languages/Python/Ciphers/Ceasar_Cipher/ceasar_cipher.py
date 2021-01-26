
message = input('Enter message that needs to be encrypted: ')
key = int(input('Enter a encryption,decryption key (the shift value): '))

symbols = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890 !?'

translated = ''

for symbol in message:
    if symbol in symbols:
        symbolIndex = symbols.find(symbol)
        translatedIndex = symbolIndex + key

        if translatedIndex >= len(symbols):
            translatedIndex = translatedIndex - len(symbols)
        elif translatedIndex < 0:
            translatedIndex = translatedIndex + len(symbols)

        translated = translated + symbols[translatedIndex]
    else:
        translated = translated + symbol

print(translated)
