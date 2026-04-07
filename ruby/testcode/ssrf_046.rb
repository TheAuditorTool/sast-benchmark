require_relative 'shared'

module MiniMagick
  class Image
    def self.open(path); new; end
  end
end

UPLOAD_DIR = '/var/app/uploads'.freeze

# vuln-code-snippet start ruby_ssrf_image_local_only
def process_local_image(req)
  path = UPLOAD_DIR + '/' + File.basename(req.param('filename'))
  MiniMagick::Image.open(path) # vuln-code-snippet safe-line ruby_ssrf_image_local_only
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_image_local_only
