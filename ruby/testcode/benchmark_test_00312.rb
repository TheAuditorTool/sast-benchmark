require_relative 'shared'

module Zeitwerk
  class Loader
    def push_dir(dir); end
    def setup; end
    def self.new; super; end
  end
end

def handler(req)
  Zeitwerk::Loader.new.tap { |l| l.push_dir(req.param('dir')) }.setup
  BenchmarkResponse.json({ ok: true })
end
