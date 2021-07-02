import sys
ascii = "Hello"
mem_size = sys.getsizeof(ascii)
print(f"'{ascii}': length: {len(ascii)} chars: {len(ascii)} mem_size: {mem_size}")

# make bytes
print(ascii.encode('utf-8'))
print("after e:", ascii[2:])

#
print("---")

uni = "Héllö"
mem_size = sys.getsizeof(uni)
print(f"'{uni}': length: {len(uni)} chars: {len(uni)} mem_size: {mem_size}")

# make bytes
#
#
print(uni.encode('utf-8'))
print("after e:", uni[2:])
