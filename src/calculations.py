import argparse


def calc_positive_values(arr):
    return sum(x for x in arr if x > 0)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-f", "--file", required=True)

    args = parser.parse_args()
    print(2)

    with open("C:/Users/Pro10/CLionProjects/untitled/" + args.file, 'rb') as f:
        data = f.read()
        print(calc_positive_values(data))


if __name__ == "__main__":
    main()
