require_relative 'shared'
require 'zlib'

# vuln-code-snippet start ruby_weakhash_crc32
def check_integrity(req)
  api_key = req.header('X-Api-Key')
  checksum = Zlib.crc32(api_key) # vuln-code-snippet vuln-line ruby_weakhash_crc32
  BenchmarkResponse.ok(checksum.to_s)
end
# vuln-code-snippet end ruby_weakhash_crc32
