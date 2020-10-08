
import random, sys, transposition_cipher_encrypt, transposition_cipher_decrypt

def main():

    random.seed(42)  # Setting random seed.

    for i in range(20): # We'll run 20 tests
        # Generating random messages to test.

        # The message will have a random length:
        message = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'* random.randint(4, 40)

        # Convert the message string to a list ot shuffle
        message = list(message)
        random.shuffle(message)
        message = ''.join(message) #Convert list to a string

        print('Test #%s: "%s..."' % (i + 1, message[:50]))

        #Check all possible keys for each message:
        for key in range(1, int(len(message)/2)):
            encrypted = transposition_cipher_encrypt.encryptMessage(key, message)
            decrypted = transposition_cipher_decrypt.decryptMessage(key, encrypted)

            #If the decryption doesn't match the original message, display
            # an error message and quit:
            if message != decrypted:
                print('Mismatch with key %s and message %s.' % (key, message))
                print('Decrypted as: ' + decrypted)
                sys.exit()
    print('Transposition cipher test passed')

#If Cipher_Fuzzer is run (instead of imported as a module) call
# the main() function:
if __name__ == '__main__':
    main()
