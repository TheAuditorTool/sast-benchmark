require_relative 'shared'
require 'zlib'

def compute_checksum(req)
  data     = req.body_str
  checksum = Zlib.crc32(data)
  BenchmarkResponse.json({ checksum: checksum })
end
