require_relative 'shared'
require 'zlib'

# vuln-code-snippet start ruby_weakhash_crc32_integrity
def check_integrity(req)
  data = req.body_str
  checksum = Zlib.crc32(data) # vuln-code-snippet vuln-line ruby_weakhash_crc32_integrity
  BenchmarkResponse.ok("checksum: #{checksum}")
end
# vuln-code-snippet end ruby_weakhash_crc32_integrity
