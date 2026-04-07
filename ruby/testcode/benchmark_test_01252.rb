require_relative 'shared'

module MiniMagick
  class Image
    def self.open(path); new; end
  end
end

UPLOAD_DIR = '/var/app/uploads'.freeze

def process_local_image(req)
  path = UPLOAD_DIR + '/' + File.basename(req.param('filename'))
  MiniMagick::Image.open(path)
  BenchmarkResponse.json({ ok: true })
end
