require_relative 'shared'

module MiniMagick
  class Image
    def self.open(url); new; end
  end
end

# vuln-code-snippet start ruby_ssrf_image_proxy
def proxy_image(req)
  MiniMagick::Image.open(req.param('url')) # vuln-code-snippet vuln-line ruby_ssrf_image_proxy
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_image_proxy
