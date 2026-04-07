require_relative 'shared'

module Zeitwerk
  class Loader
    def push_dir(dir); end
    def setup; end
    def self.new; super; end
  end
end

ZEITWERK_DIR = '/app/lib'.freeze

def handler(req)
  loader = Zeitwerk::Loader.new
  loader.push_dir(ZEITWERK_DIR)
  loader.setup
  BenchmarkResponse.json({ ok: true })
end
