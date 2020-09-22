message = input('Enter message that needs to be decrypted: ')

symbols = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890 !?'

translated = ''

for key in range(len(symbols)):
	translated = ''
	for symbol in message:
	    if symbol in symbols:
	        symbolIndex = symbols.find(symbol)
	        translatedIndex = symbolIndex - key

	        if translatedIndex >= len(symbols):
	            translatedIndex = translatedIndex - len(symbols)
	        elif translatedIndex < 0:
	            translatedIndex = translatedIndex + len(symbols)

	        translated = translated + symbols[translatedIndex]
	    else:
	        translated = translated + symbol

	print('Key #%s: %s' % (key, translated))
