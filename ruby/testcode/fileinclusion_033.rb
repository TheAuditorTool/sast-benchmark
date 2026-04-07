require_relative 'shared'

# Stub Zeitwerk module for syntax validity
module Zeitwerk
  class Loader
    def push_dir(dir); end
    def setup; end
    def self.new; super; end
  end
end

# vuln-code-snippet start ruby_fi_zeitwerk_eager
def handler(req)
  Zeitwerk::Loader.new.tap { |l| l.push_dir(req.param('dir')) }.setup # vuln-code-snippet vuln-line ruby_fi_zeitwerk_eager
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_zeitwerk_eager
