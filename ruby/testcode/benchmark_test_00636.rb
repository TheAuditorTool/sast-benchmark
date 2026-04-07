require_relative 'shared'

module MiniMagick
  class Image
    def self.open(url); new; end
  end
end

def proxy_image(req)
  MiniMagick::Image.open(req.param('url'))
  BenchmarkResponse.json({ ok: true })
end
