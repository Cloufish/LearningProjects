import subprocess


def main():

    while True:
        d = select_file("corpus/*")
        d = mutate(d)
        execute(d):
            write_sample(d)
        sys.stdout.write(".")
        sys.stdout.flush()

if __name__ == "__main__":
    main()
