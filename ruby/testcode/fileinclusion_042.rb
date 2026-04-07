require_relative 'shared'

# Stub Zeitwerk module for syntax validity
module Zeitwerk
  class Loader
    def push_dir(dir); end
    def setup; end
    def self.new; super; end
  end
end

ZEITWERK_DIR = '/app/lib'.freeze

# vuln-code-snippet start ruby_fi_zeitwerk_safe
def handler(req)
  loader = Zeitwerk::Loader.new
  loader.push_dir(ZEITWERK_DIR) # vuln-code-snippet safe-line ruby_fi_zeitwerk_safe
  loader.setup
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_zeitwerk_safe
