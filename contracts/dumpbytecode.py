import json
import sys
with open(sys.argv[1], "r") as infile:
  indata = json.load(infile)

# for some reason begins with 0x
bytecode = indata["bytecode"][2:].strip()
abi = indata["abi"]

with open(sys.argv[2], "w") as outfile:
  outfile.write(bytecode)

with open(sys.argv[3], "w") as outfile:
  json.dump(abi, outfile)