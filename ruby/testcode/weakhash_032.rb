require_relative 'shared'
require 'zlib'

# vuln-code-snippet start ruby_weakhash_crc32_checksum
def compute_checksum(req)
  data     = req.body_str
  checksum = Zlib.crc32(data) # vuln-code-snippet vuln-line ruby_weakhash_crc32_checksum
  BenchmarkResponse.json({ checksum: checksum })
end
# vuln-code-snippet end ruby_weakhash_crc32_checksum
