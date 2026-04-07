require_relative 'shared'
require 'zlib'

def check_integrity(req)
  api_key = req.header('X-Api-Key')
  checksum = Zlib.crc32(api_key)
  BenchmarkResponse.ok(checksum.to_s)
end
