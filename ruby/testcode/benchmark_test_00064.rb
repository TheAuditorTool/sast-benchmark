require_relative 'shared'
require 'zlib'

def check_integrity(req)
  data = req.body_str
  checksum = Zlib.crc32(data)
  BenchmarkResponse.ok("checksum: #{checksum}")
end
