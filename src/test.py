message = "a5%i0%s08" % (8, "95")
message = message.ljust(24, "0")
message
message_bytes = bytearray.fromhex(message)
message_bytes += bytearray([sum(message_bytes) & 0xFF])
print ''.join('{:02x}'.format(x) for x in message_bytes)
# a58095080000000000000000c2


message = "a5%i0%s08" % (8, "90")
message = message.ljust(24, "0")
message
message_bytes = bytearray.fromhex(message)
message_bytes += bytearray([sum(message_bytes) & 0xFF])
print ''.join('{:02x}'.format(x) for x in message_bytes)
# a58090080000000000000000bd